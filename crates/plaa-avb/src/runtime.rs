//! The thread plaa's network side runs on: frames from the sockets'
//! reader threads and commands from the app come in on one channel; the
//! controller, and on Linux plaa's entity, run their timers and send what
//! they queue; and what the controller has read is laid out for the app.

use std::collections::HashMap;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::JoinHandle;
use std::time::Duration;

use atdecc::controller::{self, CommandId, Controller, Event, Outcome};
use atdecc::descriptor::DescriptorType;
use atdecc::model::EntityModel;
use atdecc::{ADP_ACMP_MULTICAST, EntityId, Instant, MacAddress};
use avb_net::Socket;

#[cfg(target_os = "linux")]
use crate::gptp;
#[cfg(target_os = "linux")]
use crate::own::{self, Own};
use crate::sound::Sound;
use crate::{Config, Found, Snapshot, Stream, TalkerState};

/// The middle of plaa's controller's ID, after the interface address's
/// first half; triib's controller has 0xfd there.
const CONTROLLER_INSTANCE: u8 = 0xfc;

/// A stream's end: an entity, and its stream output's or input's index.
type End = (EntityId, u16);

/// The longest the thread waits without looking at its timers.
const MOST_WAIT: Duration = Duration::from_millis(250);

pub(crate) enum Input {
    /// An ATDECC or MAAP frame, and who sent it.
    Avtp(MacAddress, Vec<u8>),
    #[cfg(target_os = "linux")]
    Msrp(MacAddress, Vec<u8>),
    #[cfg(target_os = "linux")]
    Mvrp(MacAddress, Vec<u8>),
    #[cfg(target_os = "linux")]
    Gptp(Option<gptp::Status>),
    Connect((EntityId, u16), (EntityId, u16)),
    Disconnect((EntityId, u16)),
    /// What plays arrives at another rate.
    SoundRate,
    Stop,
}

/// Reads `socket`'s frames into `sender` on a thread of its own, until
/// `stop`.
pub(crate) fn read_into(
    socket: Arc<Socket>,
    stop: Arc<AtomicBool>,
    sender: Sender<Input>,
    name: &str,
    wrap: fn(MacAddress, Vec<u8>) -> Input,
) -> io::Result<()> {
    std::thread::Builder::new()
        .name(name.into())
        .spawn(move || {
            let mut buffer = vec![0u8; 1522];
            while !stop.load(Ordering::Relaxed) {
                match socket.receive(&mut buffer, Some(Duration::from_millis(200))) {
                    Ok(Some(received)) => {
                        let bytes = buffer[..received.length].to_vec();
                        if sender.send(wrap(received.source, bytes)).is_err() {
                            return;
                        }
                    }
                    Ok(None) => {}
                    Err(_) => std::thread::sleep(Duration::from_millis(200)),
                }
            }
        })?;
    Ok(())
}

/// Opens the sockets and starts the thread.
pub(crate) fn start(
    config: Config,
    snapshot: Arc<Mutex<Snapshot>>,
    sound: Arc<Sound>,
    stop: Arc<AtomicBool>,
    changed: Box<dyn Fn() + Send>,
) -> io::Result<(Sender<Input>, JoinHandle<()>)> {
    let avtp = Arc::new(Socket::open(&config.interface, atdecc::ETHERTYPE_AVTP)?);
    avtp.join_multicast(ADP_ACMP_MULTICAST)?;
    let (sender, inputs) = mpsc::channel();
    sound.tell(sender.clone());
    read_into(
        avtp.clone(),
        stop.clone(),
        sender.clone(),
        "plaa-avtp",
        Input::Avtp,
    )?;
    let mac = avtp.mac();
    let [a, b, c, d, e, f] = mac.0;
    let controller_id = EntityId(u64::from_be_bytes([
        a,
        b,
        c,
        0xff,
        CONTROLLER_INSTANCE,
        d,
        e,
        f,
    ]));
    let mut controller_config = controller::Config::new(controller_id);
    // Names, streams, formats and bindings are all plaa shows.
    controller_config.network_info = false;
    controller_config.read_mappings = false;
    controller_config.media_clock_info = false;
    controller_config.read_counters = false;
    controller_config.read_transit_times = false;
    controller_config.read_configurations = false;
    controller_config.read_talker_state = false;
    controller_config.lite_status = false;
    controller_config.wireless_status = false;
    let mut runtime = Runtime {
        start: std::time::Instant::now(),
        mac,
        avtp,
        controller: Controller::new(controller_config),
        pending: HashMap::new(),
        #[cfg(target_os = "linux")]
        own: None,
        talker: TalkerState::Off,
        failure: None,
        inputs,
        snapshot,
        changed,
        stop: stop.clone(),
    };
    #[cfg(target_os = "linux")]
    if config.talker {
        // MAAP, for the stream's address.
        runtime.avtp.join_multicast(own::MAAP_DESTINATION)?;
        match Own::new(&config, mac, &sender, &stop, sound) {
            Ok(own) => runtime.own = Some(own),
            Err(error) => runtime.talker = TalkerState::Blocked(error.to_string()),
        }
    }
    #[cfg(not(target_os = "linux"))]
    if config.talker {
        let _ = sound;
        runtime.talker = TalkerState::Blocked("plaa sends streams on Linux only, for now".into());
    }
    let thread = std::thread::Builder::new()
        .name("plaa-avb".into())
        .spawn(move || runtime.run())?;
    Ok((sender, thread))
}

struct Runtime {
    start: std::time::Instant,
    mac: MacAddress,
    avtp: Arc<Socket>,
    controller: Controller,
    /// Bindings waiting for the listener's format to be set first.
    pending: HashMap<CommandId, (End, End)>,
    #[cfg(target_os = "linux")]
    own: Option<Own>,
    /// The talker's state where there is no entity to ask.
    talker: TalkerState,
    failure: Option<String>,
    inputs: Receiver<Input>,
    snapshot: Arc<Mutex<Snapshot>>,
    changed: Box<dyn Fn() + Send>,
    stop: Arc<AtomicBool>,
}

impl Runtime {
    fn since(&self) -> Duration {
        self.start.elapsed()
    }

    fn now(&self) -> Instant {
        Instant::from_nanos(self.since().as_nanos() as u64)
    }

    fn run(mut self) {
        #[cfg(target_os = "linux")]
        {
            let (now, since) = (self.now(), self.since());
            if let Some(own) = &mut self.own {
                own.begin(now, since);
            }
        }
        self.controller.discover(None);
        self.publish();
        while !self.stop.load(Ordering::Relaxed) {
            let wait = self.next_wait();
            match self.inputs.recv_timeout(wait) {
                Ok(Input::Stop) | Err(RecvTimeoutError::Disconnected) => break,
                Ok(input) => self.handle(input),
                Err(RecvTimeoutError::Timeout) => {}
            }
            self.turn();
        }
        #[cfg(target_os = "linux")]
        {
            let since = self.since();
            if let Some(own) = &mut self.own {
                own.end(since);
            }
        }
        self.controller.depart();
        self.flush();
    }

    fn next_wait(&self) -> Duration {
        let now = self.now();
        let controller = self
            .controller
            .poll_timeout()
            .map(|at| at.saturating_duration_since(now));
        #[cfg(target_os = "linux")]
        let own = self
            .own
            .as_ref()
            .and_then(|own| own.poll_timeout(now, self.since()));
        #[cfg(not(target_os = "linux"))]
        let own = None;
        [controller, own, Some(MOST_WAIT)]
            .into_iter()
            .flatten()
            .min()
            .unwrap_or(MOST_WAIT)
    }

    fn handle(&mut self, input: Input) {
        let now = self.now();
        match input {
            Input::Avtp(source, bytes) => self.heard(now, source, &bytes),
            #[cfg(target_os = "linux")]
            Input::Msrp(source, bytes) => {
                let since = self.since();
                if let Some(own) = &mut self.own {
                    own.handle_msrp(since, source, &bytes);
                }
            }
            #[cfg(target_os = "linux")]
            Input::Mvrp(source, bytes) => {
                let since = self.since();
                if let Some(own) = &mut self.own {
                    own.handle_mvrp(since, source, &bytes);
                }
            }
            #[cfg(target_os = "linux")]
            Input::Gptp(status) => {
                if let Some(own) = &mut self.own {
                    own.handle_gptp(now, status);
                }
            }
            Input::Connect(talker, listener) => self.connect(now, talker, listener),
            Input::Disconnect(listener) => {
                self.failure = None;
                self.controller.disconnect(now, listener);
            }
            Input::SoundRate =>
            {
                #[cfg(target_os = "linux")]
                if let Some(own) = &mut self.own {
                    own.sound_changed();
                }
            }
            Input::Stop => {}
        }
    }

    /// A frame from the network, or from plaa's own entity or controller,
    /// which the socket does not bring back: each hears the other.
    fn heard(&mut self, now: Instant, source: MacAddress, bytes: &[u8]) {
        if bytes.first() != Some(&atdecc::avtp::subtype::MAAP) {
            let _ = self.controller.handle_frame(now, source, bytes);
        }
        #[cfg(target_os = "linux")]
        if let Some(own) = &mut self.own {
            own.handle_avtp(now, source, bytes);
        }
    }

    /// Binds `listener` to `talker`, setting the listener's format to the
    /// talker's first where they differ and the listener can take it.
    fn connect(&mut self, now: Instant, talker: (EntityId, u16), listener: (EntityId, u16)) {
        self.failure = None;
        let format = |entity: EntityId, input: bool, index: u16| {
            let model = self.controller.model(entity)?;
            model
                .streams(input)
                .find(|stream| stream.index == index)
                .map(|stream| (stream.current_format, stream.formats().collect::<Vec<_>>()))
        };
        let wanted = format(talker.0, false, talker.1).map(|(format, _)| format);
        let have = format(listener.0, true, listener.1);
        match (wanted, have) {
            (Some(wanted), Some((current, formats)))
                if wanted != current && formats.contains(&wanted) =>
            {
                let command = self.controller.set_stream_format(
                    now,
                    listener.0,
                    DescriptorType::STREAM_INPUT,
                    listener.1,
                    wanted,
                );
                self.pending.insert(command, (talker, listener));
            }
            _ => {
                self.controller.connect(now, talker, listener);
            }
        }
    }

    /// Runs the timers, acts on what happened, sends what is queued, and
    /// lays the network out again when it changed.
    fn turn(&mut self) {
        let now = self.now();
        let since = self.since();
        self.controller.handle_timeout(now);
        #[cfg(target_os = "linux")]
        let talker_before = self.talker_state();
        #[cfg(target_os = "linux")]
        if let Some(own) = &mut self.own {
            own.turn(now, since);
        }
        let _ = since;
        let mut changed = false;
        loop {
            while let Some(event) = self.controller.poll_event() {
                changed |= self.controller_event(now, event);
            }
            if !self.flush() {
                break;
            }
        }
        #[cfg(target_os = "linux")]
        {
            changed |= self.talker_state() != talker_before;
        }
        if changed {
            self.publish();
        }
    }

    /// Acts on what the controller says; whether the network changed.
    fn controller_event(&mut self, now: Instant, event: Event) -> bool {
        match event {
            Event::CommandFinished(command, outcome) => {
                if let Some((talker, listener)) = self.pending.remove(&command) {
                    // The format is set, or could not be: bound either way,
                    // as the listener may still take the stream.
                    self.controller.connect(now, talker, listener);
                    return false;
                }
                if outcome != Outcome::Done {
                    self.failure = Some(match outcome {
                        Outcome::Refused(refusal) => format!("refused: {refusal:?}"),
                        Outcome::NoResponse => "no response".into(),
                        Outcome::NotPossible | Outcome::Done => "not possible".into(),
                    });
                }
                true
            }
            Event::EnumerationStarted(_) => false,
            _ => true,
        }
    }

    /// Sends what the controller and the entity queued, handing each the
    /// other's frames, as the socket does not bring its own back. Says
    /// whether any were handed over.
    fn flush(&mut self) -> bool {
        let now = self.now();
        // Only plaa's entity, on Linux, hands frames over.
        #[cfg_attr(not(target_os = "linux"), expect(unused_mut))]
        let mut handed = false;
        let mut out = [0u8; 1500];
        let mut from_controller = Vec::new();
        while let Ok(Some(transmit)) = self.controller.poll_transmit(&mut out) {
            let bytes = &out[..transmit.length];
            let _ = self.avtp.send(transmit.destination, bytes);
            if transmit.destination == ADP_ACMP_MULTICAST || transmit.destination == self.mac {
                from_controller.push(bytes.to_vec());
            }
        }
        #[cfg(target_os = "linux")]
        if let Some(own) = &mut self.own {
            for bytes in &from_controller {
                own.handle_avtp(now, self.mac, bytes);
                handed = true;
            }
            let mut from_entity = Vec::new();
            while let Some((destination, bytes)) = own.poll_transmit() {
                let _ = self.avtp.send(destination, &bytes);
                if destination == ADP_ACMP_MULTICAST || destination == self.mac {
                    from_entity.push(bytes);
                }
            }
            own.send_mrp();
            for bytes in from_entity {
                let _ = self.controller.handle_frame(now, self.mac, &bytes);
                handed = true;
            }
        }
        let _ = (now, from_controller);
        handed
    }

    fn talker_state(&self) -> TalkerState {
        #[cfg(target_os = "linux")]
        if let Some(own) = &self.own {
            return own.state();
        }
        self.talker.clone()
    }

    /// Lays out what the controller read for the app, and tells it.
    fn publish(&mut self) {
        #[cfg(target_os = "linux")]
        let own = self.own.as_ref().map(Own::entity_id);
        #[cfg(not(target_os = "linux"))]
        let own: Option<EntityId> = None;
        let mut entities: Vec<Found> = self
            .controller
            .entities()
            .map(|discovered| discovered.entity_id())
            .filter_map(|id| {
                let model = self.controller.model(id)?;
                Some(found(id, model, own == Some(id)))
            })
            .collect();
        entities.sort_by(|left, right| {
            (!left.own, left.name.to_lowercase()).cmp(&(!right.own, right.name.to_lowercase()))
        });
        let snapshot = Snapshot {
            entities,
            talker: self.talker_state(),
            failure: self.failure.clone(),
        };
        let mut shared = self.snapshot.lock().unwrap_or_else(PoisonError::into_inner);
        if *shared != snapshot {
            *shared = snapshot;
            drop(shared);
            (self.changed)();
        }
    }
}

/// An entity as the app shows it, from what the controller read: its
/// audio streams, media clocks left out.
fn found(id: EntityId, model: &EntityModel, own: bool) -> Found {
    let streams = |input: bool| {
        model
            .streams(input)
            .filter(|stream| !stream.is_media_clock())
            .map(|stream| {
                let descriptor_type = if input {
                    DescriptorType::STREAM_INPUT
                } else {
                    DescriptorType::STREAM_OUTPUT
                };
                let name = model
                    .name_of(descriptor_type, stream.index)
                    .filter(|name| !name.is_empty())
                    .map_or_else(|| format!("{}", stream.index + 1), str::to_owned);
                Stream {
                    index: stream.index,
                    name,
                    format: stream.current_format,
                    formats: stream.formats().collect(),
                    bound: input
                        .then(|| model.binding(stream.index)?.talker_stream())
                        .flatten(),
                }
            })
            .collect::<Vec<_>>()
    };
    Found {
        id,
        name: model
            .entity_name()
            .filter(|name| !name.is_empty())
            .map_or_else(|| format!("{id}"), str::to_owned),
        own,
        outputs: streams(false),
        inputs: streams(true),
    }
}
