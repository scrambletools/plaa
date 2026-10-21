//! The thread plaa's network side runs on: frames from the sockets'
//! reader threads and commands from the app come in on one channel; the
//! controller, and on Linux plaa's entity, run their timers and send what
//! they queue; and what the controller has read is laid out for the app.
//! On a Mac, commands to the Mac's own entity, plaa's talker there, go
//! through Apple's AVB framework, which answers them.

use std::collections::{HashMap, VecDeque};
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::JoinHandle;
use std::time::Duration;

use atdecc::controller::{self, CommandId, Controller, Event, Outcome};
use atdecc::descriptor::DescriptorType;
use atdecc::model::EntityModel;
use atdecc::stream_format::StreamFormat;
use atdecc::{ADP_ACMP_MULTICAST, EntityId, Instant, MacAddress};
use avb_net::Socket;

#[cfg(target_os = "linux")]
use crate::gptp;
#[cfg(target_os = "macos")]
use crate::host::Host;
#[cfg(target_os = "linux")]
use crate::own::{self, Own};
use crate::sound::Sound;
use crate::{Config, Found, Snapshot, Stream, TalkerState};

/// The middle of plaa's controller's ID, after the interface address's
/// first half; triib's controller has 0xfd there.
const CONTROLLER_INSTANCE: u8 = 0xfc;

/// A stream's end: an entity, and its stream output's or input's index.
type End = (EntityId, u16);

/// A step in connecting: a stream's new format, then the binding.
#[derive(Debug, Clone, Copy)]
enum Step {
    Format(EntityId, DescriptorType, u16, StreamFormat),
    Bind(End, End),
}

/// The longest the thread waits without looking at its timers.
const MOST_WAIT: Duration = Duration::from_millis(250);

/// How often the inputs the windows detail are asked how they stand.
const CHOSEN_EVERY: Duration = Duration::from_secs(1);

/// How often the next listener in turn is read again, while the windows
/// list them.
const LISTED_EVERY: Duration = Duration::from_millis(500);

/// The longest wait while the Mac's own entity owes answers, which come
/// through the framework, not the socket.
#[cfg(target_os = "macos")]
const LOCAL_WAIT: Duration = Duration::from_millis(2);

/// Apple's framework on the interface, opened and used on the runtime's
/// thread alone.
#[cfg(target_os = "macos")]
struct OnThread(Option<atdecc::blocking::Local>);

// SAFETY: it holds nothing while the runtime moves to its thread, which
// alone opens the framework and uses it.
#[cfg(target_os = "macos")]
#[allow(unsafe_code)]
unsafe impl Send for OnThread {}

pub(crate) enum Input {
    /// An ATDECC or MAAP frame, and who sent it.
    Avtp(MacAddress, Vec<u8>),
    #[cfg(target_os = "linux")]
    Msrp(MacAddress, Vec<u8>),
    #[cfg(target_os = "linux")]
    Mvrp(MacAddress, Vec<u8>),
    #[cfg(target_os = "linux")]
    Gptp(Option<gptp::Status>),
    /// What the program sharing its port's participant says.
    #[cfg(target_os = "linux")]
    Shared(avb_mrp::sharing::Notice),
    /// Its connection of this generation closed.
    #[cfg(target_os = "linux")]
    SharedClosed(u64),
    /// Binds a listener's input to a talker's output, both in a format
    /// given, or the talker's.
    Connect(End, End, Option<StreamFormat>),
    Disconnect((EntityId, u16)),
    /// What the windows show: the listeners, and the inputs detailed.
    Watch {
        listed: bool,
        chosen: Vec<End>,
    },
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
    // Names, streams, formats, bindings and how the streams fare, by
    // their info and counters, are all plaa shows.
    controller_config.network_info = false;
    controller_config.read_mappings = false;
    controller_config.media_clock_info = false;
    controller_config.read_transit_times = false;
    controller_config.read_configurations = false;
    controller_config.read_talker_state = false;
    controller_config.lite_status = false;
    controller_config.wireless_status = false;
    // Without Apple's framework the Mac's own entity is out of reach, so
    // the controller leaves what is advertised from this address alone,
    // until AVB is on and the runtime's thread opens the framework.
    #[cfg(target_os = "macos")]
    if atdecc::blocking::avb_enabled(&config.interface) != Some(true) {
        controller_config.own_mac = Some(mac);
    }
    let mut runtime = Runtime {
        start: std::time::Instant::now(),
        mac,
        avtp,
        controller: Controller::new(controller_config),
        pending: HashMap::new(),
        #[cfg(target_os = "linux")]
        own: None,
        #[cfg(target_os = "macos")]
        interface: config.interface.clone(),
        #[cfg(target_os = "macos")]
        controller_config,
        #[cfg(target_os = "macos")]
        local: OnThread(None),
        #[cfg(target_os = "macos")]
        host: None,
        talker: TalkerState::Off,
        failure: None,
        watch: Watch::default(),
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
    #[cfg(target_os = "macos")]
    if config.talker {
        runtime.host = Some(Host::new(&config.interface, mac, sound));
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    if config.talker {
        let _ = sound;
        runtime.talker =
            TalkerState::Blocked("plaa sends streams on Linux and macOS only, for now".into());
    }
    let thread = std::thread::Builder::new()
        .name("plaa-avb".into())
        .spawn(move || runtime.run())?;
    Ok((sender, thread))
}

/// What the windows show of the network, asked for again as it can
/// change unannounced: some entities send no notifications, and changes
/// other controllers make then reach plaa no other way.
#[derive(Default)]
struct Watch {
    listed: bool,
    chosen: Vec<End>,
    /// When the chosen inputs, and the last listener in turn, were asked.
    chosen_at: Option<Duration>,
    listed_at: Option<Duration>,
    /// Counts the listeners asked, to take the next in turn.
    turn: usize,
}

struct Runtime {
    start: std::time::Instant,
    mac: MacAddress,
    avtp: Arc<Socket>,
    controller: Controller,
    /// What is left of connections waiting for a format to be set.
    pending: HashMap<CommandId, VecDeque<Step>>,
    #[cfg(target_os = "linux")]
    own: Option<Own>,
    #[cfg(target_os = "macos")]
    interface: String,
    /// The controller's, to start it again once the framework opens.
    #[cfg(target_os = "macos")]
    controller_config: controller::Config,
    /// The Mac's own entities, through Apple's framework, opened on the
    /// runtime's thread, which alone uses it.
    #[cfg(target_os = "macos")]
    local: OnThread,
    /// The Mac's own entity, as plaa's talker.
    #[cfg(target_os = "macos")]
    host: Option<Host>,
    /// The talker's state where there is no entity to ask.
    talker: TalkerState,
    failure: Option<String>,
    watch: Watch,
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
        #[cfg(target_os = "macos")]
        self.open_local();
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
        #[cfg(target_os = "macos")]
        let own = self
            .local
            .0
            .as_ref()
            .is_some_and(atdecc::blocking::Local::waiting)
            .then_some(LOCAL_WAIT)
            .or(own);
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
                    own.handle_msrp(now, since, source, &bytes);
                }
            }
            #[cfg(target_os = "linux")]
            Input::Mvrp(source, bytes) => {
                let since = self.since();
                if let Some(own) = &mut self.own {
                    own.handle_mvrp(now, since, source, &bytes);
                }
            }
            #[cfg(target_os = "linux")]
            Input::Gptp(status) => {
                if let Some(own) = &mut self.own {
                    own.handle_gptp(now, status);
                }
            }
            #[cfg(target_os = "linux")]
            Input::Shared(notice) => {
                let since = self.since();
                if let Some(own) = &mut self.own {
                    own.handle_shared(since, notice);
                }
            }
            #[cfg(target_os = "linux")]
            Input::SharedClosed(generation) => {
                let since = self.since();
                if let Some(own) = &mut self.own {
                    own.shared_closed(since, generation);
                }
            }
            Input::Connect(talker, listener, format) => {
                self.connect(now, talker, listener, format);
            }
            Input::Disconnect(listener) => {
                self.failure = None;
                self.controller.disconnect(now, listener);
            }
            Input::Watch { listed, chosen } => {
                // Newly chosen inputs are asked at once.
                if chosen != self.watch.chosen {
                    self.watch.chosen_at = None;
                }
                self.watch.listed = listed;
                self.watch.chosen = chosen;
            }
            Input::SoundRate => {
                #[cfg(target_os = "linux")]
                if let Some(own) = &mut self.own {
                    own.sound_changed();
                }
                #[cfg(target_os = "macos")]
                if let Some(host) = &mut self.host {
                    host.sound_changed();
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

    /// Binds `listener` to `talker` in `format`, or in the talker's: the
    /// talker's output, then the listener's input, set to it first where
    /// they are in another and can take it, as Milan listeners take a
    /// stream only in their own format.
    fn connect(&mut self, now: Instant, talker: End, listener: End, format: Option<StreamFormat>) {
        self.failure = None;
        let stream = |entity: EntityId, input: bool, index: u16| {
            let model = self.controller.model(entity)?;
            model
                .streams(input)
                .find(|stream| stream.index == index)
                .map(|stream| (stream.current_format, stream.formats().collect::<Vec<_>>()))
        };
        let mut steps = VecDeque::new();
        let sent = stream(talker.0, false, talker.1);
        let wanted = format.or(sent.as_ref().map(|(current, _)| *current));
        if let (Some(wanted), Some((current, formats))) = (wanted, &sent)
            && wanted != *current
            && formats.contains(&wanted)
        {
            steps.push_back(Step::Format(
                talker.0,
                DescriptorType::STREAM_OUTPUT,
                talker.1,
                wanted,
            ));
        }
        if let (Some(wanted), Some((current, formats))) =
            (wanted, stream(listener.0, true, listener.1))
            && wanted != current
            && formats.contains(&wanted)
        {
            steps.push_back(Step::Format(
                listener.0,
                DescriptorType::STREAM_INPUT,
                listener.1,
                wanted,
            ));
        }
        steps.push_back(Step::Bind(talker, listener));
        self.next_step(now, steps);
    }

    /// Sends the first of `steps`, keeping the rest for when it is done.
    fn next_step(&mut self, now: Instant, mut steps: VecDeque<Step>) {
        let Some(step) = steps.pop_front() else {
            return;
        };
        let command = match step {
            Step::Format(entity, descriptor_type, index, format) => self
                .controller
                .set_stream_format(now, entity, descriptor_type, index, format),
            Step::Bind(talker, listener) => self.controller.connect(now, talker, listener),
        };
        if !steps.is_empty() {
            self.pending.insert(command, steps);
        }
    }

    /// Runs the timers, acts on what happened, sends what is queued, and
    /// lays the network out again when it changed.
    fn turn(&mut self) {
        let now = self.now();
        let since = self.since();
        #[cfg(target_os = "macos")]
        self.take_local();
        self.controller.handle_timeout(now);
        self.refresh(now, since);
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        let talker_before = (self.talker_state(), self.local_unserved());
        #[cfg(target_os = "linux")]
        if let Some(own) = &mut self.own {
            own.turn(now, since);
        }
        #[cfg(target_os = "macos")]
        if let Some(host) = &mut self.host {
            host.turn(&self.controller);
            if host.avb_on() && self.local.0.is_none() {
                self.open_local();
            }
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
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            changed |= (self.talker_state(), self.local_unserved()) != talker_before;
        }
        if changed {
            self.publish();
        }
    }

    /// Acts on what the controller says; whether the network changed.
    fn controller_event(&mut self, now: Instant, event: Event) -> bool {
        match event {
            Event::CommandFinished(command, outcome) => {
                if let Some(steps) = self.pending.remove(&command) {
                    // A format is set, or could not be: on either way, as
                    // the listener may still take the stream.
                    self.next_step(now, steps);
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
            #[cfg(target_os = "macos")]
            if self.send_local(bytes) {
                continue;
            }
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

    /// Opens Apple's framework on the interface once AVB is on for it: a
    /// controller that left the Mac's own entity alone starts again,
    /// reaching it.
    #[cfg(target_os = "macos")]
    fn open_local(&mut self) {
        if self.local.0.is_some() {
            return;
        }
        let entity_id = self.controller_config.entity_id;
        let Some(local) = atdecc::blocking::Local::open(&self.interface, entity_id) else {
            return;
        };
        self.local.0 = Some(local);
        if self.controller_config.own_mac.take().is_some() {
            self.controller = Controller::new(self.controller_config);
            self.controller.discover(None);
        }
    }

    /// Hands the controller what the Mac's own entity answered through
    /// the framework, as frames from it.
    #[cfg(target_os = "macos")]
    fn take_local(&mut self) {
        let now = self.now();
        while let Some(bytes) = self
            .local
            .0
            .as_ref()
            .and_then(atdecc::blocking::Local::poll)
        {
            let _ = self.controller.handle_frame(now, self.mac, &bytes);
        }
    }

    /// Sends a command to the Mac's own entity through the framework,
    /// returning whether it did.
    #[cfg(target_os = "macos")]
    fn send_local(&self, bytes: &[u8]) -> bool {
        let Some(local) = &self.local.0 else {
            return false;
        };
        local.send(bytes, |entity_id| {
            self.controller
                .entity(entity_id)
                .is_some_and(|entity| entity.mac == self.mac)
        })
    }

    /// Whether listeners on this computer go without plaa's stream.
    fn local_unserved(&self) -> bool {
        #[cfg(target_os = "linux")]
        if let Some(own) = &self.own {
            return own.local_unserved();
        }
        false
    }

    fn talker_state(&self) -> TalkerState {
        #[cfg(target_os = "linux")]
        if let Some(own) = &self.own {
            return own.state();
        }
        #[cfg(target_os = "macos")]
        if let Some(host) = &self.host {
            return host.state();
        }
        self.talker.clone()
    }

    /// Asks again for what the windows show: every second how the chosen
    /// inputs take their streams, and in turn one listener's name and its
    /// inputs' formats.
    fn refresh(&mut self, now: Instant, since: Duration) {
        let due = |at: Option<Duration>, every: Duration| at.is_none_or(|at| since >= at + every);
        if !self.watch.chosen.is_empty() && due(self.watch.chosen_at, CHOSEN_EVERY) {
            self.watch.chosen_at = Some(since);
            for &(entity, input) in &self.watch.chosen {
                let stream = (entity, DescriptorType::STREAM_INPUT, input);
                self.controller
                    .refresh_stream(now, stream.0, stream.1, stream.2);
                self.controller
                    .refresh_descriptor(now, stream.0, stream.1, stream.2);
            }
        }
        if !self.watch.listed || !due(self.watch.listed_at, LISTED_EVERY) {
            return;
        }
        self.watch.listed_at = Some(since);
        let own = self.own_entity();
        let listeners: Vec<(EntityId, Vec<u16>)> = self
            .controller
            .entities()
            .map(|discovered| discovered.entity_id())
            .filter(|&id| Some(id) != own)
            .filter_map(|id| {
                let inputs: Vec<u16> = self
                    .controller
                    .model(id)?
                    .streams(true)
                    .filter(|stream| !stream.is_media_clock())
                    .map(|stream| stream.index)
                    .collect();
                (!inputs.is_empty()).then_some((id, inputs))
            })
            .collect();
        if listeners.is_empty() {
            return;
        }
        let (entity, inputs) = &listeners[self.watch.turn % listeners.len()];
        self.watch.turn = self.watch.turn.wrapping_add(1);
        self.controller
            .refresh_descriptor(now, *entity, DescriptorType::ENTITY, 0);
        for &input in inputs {
            self.controller
                .refresh_descriptor(now, *entity, DescriptorType::STREAM_INPUT, input);
        }
    }

    /// plaa's own entity, where it has one.
    fn own_entity(&self) -> Option<EntityId> {
        #[cfg(target_os = "linux")]
        let own = self.own.as_ref().map(Own::entity_id);
        #[cfg(target_os = "macos")]
        let own = self.host.as_ref().and_then(Host::entity_id);
        #[cfg(not(any(target_os = "linux", target_os = "macos")))]
        let own: Option<EntityId> = None;
        own
    }

    /// Lays out what the controller read for the app, and tells it.
    fn publish(&mut self) {
        let own = self.own_entity();
        let mut entities: Vec<Found> = self
            .controller
            .entities()
            .map(|discovered| discovered.entity_id())
            .filter_map(|id| {
                let model = self.controller.model(id)?;
                let here = self
                    .controller
                    .entity(id)
                    .is_some_and(|entity| entity.mac == self.mac);
                Some(found(id, model, own == Some(id), here))
            })
            .collect();
        entities.sort_by(|left, right| {
            (!left.own, left.name.to_lowercase()).cmp(&(!right.own, right.name.to_lowercase()))
        });
        let snapshot = Snapshot {
            entities,
            talker: self.talker_state(),
            failure: self.failure.clone(),
            local_unserved: self.local_unserved(),
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
fn found(id: EntityId, model: &EntityModel, own: bool, here: bool) -> Found {
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
                    info: model.stream_info(descriptor_type, stream.index).copied(),
                    counters: input
                        .then(|| model.stream_input_counters(stream.index))
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
        here,
        outputs: streams(false),
        inputs: streams(true),
    }
}
