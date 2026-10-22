//! plaa's own Milan entity: one stream output of two channels that sends
//! what plaa plays, on gPTP time, once a listener on the network is ready
//! for it, and one stream input that plays the stream a controller binds
//! it to on the sound output. MAAP claims the stream's multicast address;
//! MSRP advertises it and hears listeners, and declares the input's
//! listener, MVRP joins its VLAN, as triib's endpoints do, in triib's
//! entity service's participant where it runs on the port. There, while
//! the port runs AVB Lite, the service declares them by CVU SRP in the
//! entity's name, and the stream goes as a copy to each listener it
//! admits, as it says.

mod lite;

use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;
use std::time::Duration;

use atdecc::descriptor::DescriptorType;
use atdecc::entity::{
    EndpointModel, Entity, EntityEvent, InputReservation, OutputReservation, ProbedStream,
    StreamModel, UNADDRESSED,
};
use atdecc::lite::{FallbackReason, LiteFlags, LiteStatus, PtpProfile};
use atdecc::maap::{self, Maap, MaapEvent};
use atdecc::stream_format::StreamFormat;
use atdecc::{ClockIdentity, EntityId, EntityModelId, Instant, MacAddress, StreamId};
use avb_mrp::msrp::{ListenerState, TalkerDeclaration};
use avb_mrp::sharing::{Mode, Notice};
use avb_stream::audio::{SharedBuffer, Sink, Source};
use avb_stream::media::Media;
use avb_stream::{Listener, ListenerConfig, MediaClock, Talker, TalkerConfig, TalkerStats};

use crate::reservation::{self, Heard, Reservation, VLAN};
use crate::runtime::Input;
use crate::sound::{CHANNELS, Sound};
use crate::{Config, Ptp, TalkerState, gptp, ptp};
use lite::OwnLite;

/// The middle of plaa's entity ID, after the interface address's first
/// half: triib's endpoints count up from 0 there.
const INSTANCE: u8 = 0x70;

/// plaa's entity model, under Scramble Tools' MA-S block, as triib's. Its
/// revision, the last octet, goes up as the model changes, so controllers
/// that keep models by their ID read it again: 2 brought the logos, 3 the
/// stream input.
const ENTITY_MODEL: EntityModelId = EntityModelId(0x8c1f_6436_c020_0004);

/// plaa's app icon, which controllers show the entity with.
const LOGO: &[u8] = include_bytes!("../../../data/icons/hicolor/128x128/apps/run.plaa.png");

/// Scramble Tools' logo, its vendor's, as triib's entities carry it.
const VENDOR_LOGO: &[u8] = include_bytes!("../../../data/icons/scramble-vendor.png");

/// What the stream sends, and the input takes, as triib's entities do:
/// the AVB community's Basic and Extended Audio profiles' AAF, in SR class
/// A and B, and AM824, at 48, 96 and 192 kHz, of two channels or eight,
/// the sound in the first two. Milan's stereo stream first, its default.
fn formats() -> Vec<StreamFormat> {
    avb_stream::formats::offered(CHANNELS, false)
}

/// The priority of class A streams.
const PRIORITY: u8 = 3;

/// The priority of class B streams.
const CLASS_B_PRIORITY: u8 = 2;

/// The priority of AVB Lite's streams (AVB Lite profile, 7).
const LITE_PRIORITY: u8 = 5;

/// A talker's destination in AVB Lite while its stream comes unicast, to
/// each listener's own address (profile 6).
const UNICAST: MacAddress = MacAddress([0; 6]);

/// The latency the talker adds to the stream's path, in nanoseconds.
const TALKER_LATENCY: u32 = 125_000;

/// How long after withdrawing the stream it is declared again, so a
/// bridge has let the old declaration go.
const REDECLARE: Duration = Duration::from_secs(2);

/// How often the stream's counters go to the entity, as Milan allows its
/// notifications of them at most.
const COUNT_EVERY: Duration = Duration::from_secs(1);

/// How long ptp4l may take to answer once its service starts.
const PTP_PATIENCE: Duration = Duration::from_secs(15);

/// How often it looks whether another program serves the port's
/// participant, to share it, while it runs its own or none.
const LOOK: Duration = Duration::from_secs(2);

/// How long plaa waits for a program that served the port's participant
/// to come back before it runs its own: triib's app starts its entity
/// service again within seconds of it stopping, and plaa's participants
/// running meanwhile would be two on the port as the service opens.
const SERVICE_GRACE: Duration = Duration::from_secs(15);

/// ptp4l's read-only management socket.
const PTP4L_SOCKET: &str = "/var/run/ptp4lro";

/// plaa's entity ID on an interface with address `mac`.
pub(crate) fn entity_id(mac: MacAddress) -> EntityId {
    let [a, b, c, d, e, f] = mac.0;
    EntityId(u64::from_be_bytes([a, b, c, 0xff, INSTANCE, d, e, f]))
}

/// What the stream sent over the entity's life, for its counters: the
/// talkers that ran, and with [`Sent::counters`] the one running.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Sent {
    starts: u32,
    stops: u32,
    frames: u64,
    media_resets: u64,
}

impl Sent {
    /// The talker that stopped, with what it sent.
    fn stopped(&mut self, stats: TalkerStats) {
        self.stops = self.stops.wrapping_add(1);
        self.frames += stats.frames_sent;
        self.media_resets += stats.media_resets;
    }

    /// The stream output's counters in GET_COUNTERS' places (IEEE
    /// 1722.1-2021, 7.4.42.2; Milan 1.3, 5.4.2.25), with `running` counted
    /// in. Each wraps as the counters do.
    fn counters(self, running: Option<TalkerStats>) -> [u32; 32] {
        let frames = self.frames + running.map_or(0, |stats| stats.frames_sent);
        let resets = self.media_resets + running.map_or(0, |stats| stats.media_resets);
        let mut counters = [0; 32];
        counters[0] = self.starts;
        counters[1] = self.stops;
        counters[3] = resets as u32;
        // Every frame carries a valid timestamp.
        counters[5] = frames as u32;
        counters[7] = frames as u32;
        counters
    }
}

/// What the stream input listens to.
#[derive(Default)]
struct Listening {
    /// The stream of the talker's output it is bound to, once the probe
    /// found it.
    stream: Option<ProbedStream>,
    /// Its talker's declaration, as the port registered it: none until
    /// heard, or while no talker declares it.
    talker: Option<TalkerDeclaration>,
    /// The participant plaa shares was asked to listen to the stream.
    asked: bool,
    /// The listener plaa's own participants declare for it.
    declared: Option<ListenerState>,
    /// The participant plaa shares cannot declare listeners now, as in
    /// AVB Lite.
    refused: bool,
    receiver: Option<Listener>,
    /// The stream it takes, and where it comes to.
    receiving: Option<(u64, Vec<MacAddress>)>,
    /// The buffer a player drains, which the stream plays into; the sound
    /// output where none.
    into: Option<SharedBuffer>,
    /// The input's counters, kept as the receivers come and go.
    counters: [u32; 32],
}

pub(crate) struct Own {
    entity: Entity,
    listening: Listening,
    maap: Maap,
    /// Where the stream is reserved; none while plaa waits for a program
    /// that went to come back, or cannot.
    reservation: Option<Reservation>,
    /// Counts the shared participants, to tell whose connection closed.
    generation: u64,
    /// When it next looks for a program serving the port's participant.
    look_at: Duration,
    /// When the program whose participant plaa shared went, for plaa to
    /// wait a while for it to come back.
    gone_at: Option<Duration>,
    sender: Sender<Input>,
    clock: Arc<MediaClock>,
    sound: Arc<Sound>,
    talker: Option<Talker>,
    /// The VLAN tag the running talker sends with, priority and VLAN.
    talker_tag: u16,
    /// The port's mode, as the program sharing its participant says it.
    mode: Option<Mode>,
    /// Where the stream goes while the port runs AVB Lite, and with which
    /// tag, as that program says: none until it says, and the stream goes
    /// nowhere meanwhile.
    copies: Option<(u16, Vec<MacAddress>)>,
    /// The link's speed in Mb/s, as of the last mode, for GET_LITE_STATUS.
    link_speed: u32,
    /// What ptp4l last said of the interface.
    gptp: Option<gptp::Status>,
    /// The interface's link and grandmaster, counted for controllers.
    link: Link,
    /// AVB Lite on plaa's own participants: weighing whether to fall
    /// back, and CVU SRP once it does.
    own_lite: Option<OwnLite>,
    /// Where PTP's messages come in, for the fallback and each
    /// grandmaster's link speed.
    ptp_socket: Option<Arc<avb_net::Socket>>,
    /// The link speed each grandmaster announces, and the media clock's
    /// correction for it in AVB Lite (profile 5).
    grandmaster_links: HashMap<u64, u32>,
    correction: i64,
    interface: String,
    mac: MacAddress,
    own_clock: ClockIdentity,
    /// Whether a listener on the network is ready for the stream.
    ready: bool,
    /// The stream is advertised.
    declared: bool,
    /// When it is next declared, once it has an address.
    declare_at: Option<Duration>,
    withdrawn_at: Option<Duration>,
    /// Why it stopped: another program declares streams on the port, and
    /// does not share its participant.
    blocked: Option<String>,
    /// Why the program sharing its participant would not declare the
    /// stream, until it does.
    refused: Option<String>,
    /// What the stream sent, for its counters, and when they last went
    /// to the entity.
    sent: Sent,
    counted_at: Duration,
    /// Where the stream's address is kept between runs, and the one kept,
    /// in use until MAAP has claimed the stream's address anew.
    address_file: Option<std::path::PathBuf>,
    kept: Option<MacAddress>,
    /// How ptp4l stands on the interface, and whether plaa tried to start
    /// its service there, which it does once.
    ptp: Ptp,
    ptp_tried: bool,
    ptp_started: Option<std::time::Instant>,
}

impl Own {
    /// Sets the entity up on `config`'s interface, its frames read into
    /// `sender`; an error says why it cannot run there.
    pub(crate) fn new(
        config: &Config,
        mac: MacAddress,
        sender: &Sender<Input>,
        stop: &Arc<AtomicBool>,
        sound: Arc<Sound>,
    ) -> io::Result<Self> {
        let interface = config.interface.clone();
        let found = avb_net::interfaces()
            .into_iter()
            .find(|found| found.name == interface)
            .ok_or_else(|| io::Error::other(format!("no interface named {interface}")))?;
        let Some(phc) = found.hardware_clock else {
            return Err(io::Error::other(format!(
                "{interface} has no PTP hardware clock to time a stream by"
            )));
        };
        let clock = Arc::new(MediaClock::open(Some(phc))?);
        let generation = 1;
        let mut blocked = None;
        let reservation = match Reservation::share(
            &interface,
            sender,
            generation,
            Duration::ZERO,
            entity_id(mac).0,
        ) {
            Some(shared) => Some(shared),
            None => match reservation::taken(&interface) {
                Some(why) => {
                    blocked = Some(why);
                    None
                }
                None => Some(Reservation::own(&interface, mac, sender)?),
            },
        };
        gptp::watch(
            PTP4L_SOCKET.to_owned(),
            stop.clone(),
            sender.clone(),
            Input::Gptp,
        );
        let own_clock = ClockIdentity(u64::from_be_bytes([
            mac.0[0], mac.0[1], mac.0[2], 0xff, 0xfe, mac.0[3], mac.0[4], mac.0[5],
        ]));
        let model = EndpointModel {
            entity_id: entity_id(mac),
            entity_model_id: ENTITY_MODEL,
            entity_name: config.name.clone(),
            group_name: String::new(),
            vendor_name: "Scramble Tools".into(),
            model_name: "plaa".into(),
            firmware_version: env!("CARGO_PKG_VERSION").into(),
            serial_number: String::new(),
            mac,
            interface_name: interface.clone(),
            clock_identity: own_clock,
            sampling_rates: avb_stream::formats::RATES.to_vec(),
            current_sampling_rate: avb_stream::formats::RATES[0],
            outputs: vec![StreamModel {
                name: "Playback".into(),
                formats: formats(),
                current_format: formats()[0],
                channels: CHANNELS,
            }],
            inputs: vec![StreamModel {
                name: "Listen".into(),
                formats: formats(),
                current_format: formats()[0],
                channels: CHANNELS,
            }],
            clock_source: 0,
            clock_source_inputs: Vec::new(),
            logo: Some(LOGO.to_vec()),
            vendor_logo: Some(VENDOR_LOGO.to_vec()),
        };
        let stream = StreamId::new(mac, u16::from(INSTANCE) << 8);
        let ptp_socket = lite::open_ptp(&interface, sender, stop)
            .inspect_err(|error| eprintln!("plaa: PTP's messages go unheard: {error}"))
            .ok();
        let alone = reservation
            .as_ref()
            .is_some_and(|reservation| !reservation.is_shared());
        let mut own = Self {
            entity: Entity::new(model, &[(stream, UNADDRESSED)]),
            listening: Listening::default(),
            maap: Maap::new(mac, 1),
            reservation,
            generation,
            look_at: LOOK,
            gone_at: None,
            sender: sender.clone(),
            clock,
            sound,
            talker: None,
            talker_tag: 0,
            mode: None,
            copies: None,
            link_speed: found.speed.unwrap_or(0),
            gptp: None,
            link: Link::default(),
            own_lite: None,
            ptp_socket,
            grandmaster_links: HashMap::new(),
            correction: 0,
            interface,
            mac,
            own_clock,
            ready: false,
            declared: false,
            declare_at: None,
            withdrawn_at: None,
            blocked,
            refused: None,
            sent: Sent::default(),
            counted_at: Duration::ZERO,
            kept: config
                .address_file
                .as_deref()
                .and_then(|file| kept_address(file, &config.interface)),
            address_file: config.address_file.clone(),
            ptp: Ptp::Unknown,
            ptp_tried: false,
            ptp_started: None,
        };
        if alone {
            own.arm_lite(Duration::ZERO);
        }
        Ok(own)
    }

    pub(crate) fn entity_id(&self) -> EntityId {
        self.entity.entity_id()
    }

    /// Whether listeners on this computer go without plaa's stream, as
    /// the participant plaa shares tells only the network's.
    pub(crate) fn local_unserved(&self) -> bool {
        self.reservation
            .as_ref()
            .is_some_and(|reservation| !reservation.reaches_local())
    }

    /// Starts advertising, joins VLAN 2 and class A and B's domains, and
    /// claims the stream's address.
    pub(crate) fn begin(&mut self, now: Instant, since: Duration) {
        if self.blocked.is_some() {
            return;
        }
        self.entity.start(now);
        if let Some(reservation) = &mut self.reservation {
            reservation.begin(since);
        }
        self.maap.start(now);
        // The address the stream had last time, plaa's moments ago as a
        // rule, goes out at once rather than after MAAP's three probes:
        // listeners hear plaa again within a fraction of a second. MAAP
        // claims an address meanwhile, and the stream moves to it if it
        // is another.
        if let Some(address) = self.kept {
            self.entity.set_output_destination(0, address);
            self.declare_at = Some(self.redeclare_at(since));
        }
    }

    /// Whether the stream has an address: MAAP's, or the one kept from
    /// the last run until MAAP has claimed one.
    fn addressed(&self) -> bool {
        self.maap.address(0).is_some() || self.kept.is_some()
    }

    pub(crate) fn state(&self) -> TalkerState {
        match (&self.blocked, &self.reservation, &self.talker) {
            (Some(why), _, _) => TalkerState::Blocked(why.clone()),
            (None, Some(reservation), _) if !reservation.available() => {
                TalkerState::Blocked(format!(
                    "triib's entity service runs {} as AVB Lite, which reserves no streams",
                    self.interface
                ))
            }
            (None, _, _) if self.refused.is_some() => {
                TalkerState::Blocked(self.refused.clone().unwrap_or_default())
            }
            (None, _, Some(_)) => TalkerState::Streaming,
            (None, _, None) => TalkerState::Waiting,
        }
    }

    /// An ATDECC or MAAP frame from the network, or from plaa's own
    /// controller, sent to a group address or not: CVU SRP's commands
    /// among them in AVB Lite.
    pub(crate) fn handle_avtp(
        &mut self,
        now: Instant,
        since: Duration,
        source: MacAddress,
        group: bool,
        bytes: &[u8],
    ) {
        if self.blocked.is_some() {
            return;
        }
        if bytes.first() == Some(&atdecc::avtp::subtype::MAAP) {
            self.maap.handle_frame(now, source, bytes);
        } else {
            self.cvu_heard(since, source, group, bytes);
            self.entity.handle_frame(now, source, bytes);
        }
    }

    pub(crate) fn handle_msrp(
        &mut self,
        now: Instant,
        since: Duration,
        source: MacAddress,
        bytes: &[u8],
    ) {
        if source == self.mac {
            self.foreign(now, since);
        } else if let Some(reservation) = &mut self.reservation {
            reservation.handle_msrp(since, bytes);
        }
    }

    pub(crate) fn handle_mvrp(
        &mut self,
        now: Instant,
        since: Duration,
        source: MacAddress,
        bytes: &[u8],
    ) {
        if source == self.mac {
            self.foreign(now, since);
        } else if let Some(reservation) = &mut self.reservation {
            reservation.handle_mvrp(since, bytes);
        }
    }

    /// MRP from this computer's own address that plaa's sockets did not
    /// send: another participant on the port, whose declarations and
    /// plaa's would withdraw each other. plaa shares it where it is
    /// served, else steps aside.
    fn foreign(&mut self, now: Instant, since: Duration) {
        let own = self
            .reservation
            .as_ref()
            .is_some_and(|reservation| !reservation.is_shared());
        if !own || self.share(now, since) {
            return;
        }
        self.blocked = Some(format!(
            "another program declares streams on {}, and does not share them",
            self.interface
        ));
        self.disarm_lite();
        self.reservation = None;
        self.stop_talker();
        self.declared = false;
        self.ready = false;
        self.declare_at = None;
        self.lost_participant(since);
        self.entity.depart();
    }

    /// Shares the participant of a program serving the port's, if one
    /// does: the stream is declared in it, or the entity starts again
    /// where it had stepped aside. Whether it does.
    fn share(&mut self, now: Instant, since: Duration) -> bool {
        let generation = self.generation + 1;
        let entity = self.entity.entity_id().0;
        let Some(shared) =
            Reservation::share(&self.interface, &self.sender, generation, since, entity)
        else {
            return false;
        };
        self.generation = generation;
        self.gone_at = None;
        self.refused = None;
        self.disarm_lite();
        // plaa's own participants go without a word, as the stream they
        // declared is declared again in the shared one.
        self.reservation = Some(shared);
        self.lost_participant(since);
        if self.blocked.take().is_some() {
            self.begin(now, since);
        } else if self.declared {
            self.declare(since);
        } else if self.addressed() && self.declare_at.is_none() {
            self.declare_at = Some(self.redeclare_at(since));
        }
        true
    }

    /// What the program sharing its participant said.
    pub(crate) fn handle_shared(&mut self, since: Duration, notice: Notice) {
        let ours = self.entity.output_stream(0).map(|(id, _)| id.0);
        let listened = self.listened();
        // It said whether it takes Listen.
        let hello = matches!(notice, Notice::Hello(_));
        if let Some(heard) = self
            .reservation
            .as_mut()
            .and_then(|reservation| reservation.handle_notice(notice, ours, listened))
        {
            self.heard(since, heard);
        }
        if hello {
            self.ask_to_listen(since);
        }
    }

    /// The program sharing its participant went, as its connection
    /// `generation` closed: it withdrew the stream. plaa waits a moment
    /// for it to come back, then runs its own.
    pub(crate) fn shared_closed(&mut self, since: Duration, generation: u64) {
        let ours = self.reservation.as_ref().and_then(Reservation::generation) == Some(generation);
        if !ours {
            return;
        }
        self.reservation = None;
        self.stop_talker();
        if self.declared {
            self.declared = false;
            self.withdrawn_at = Some(since);
        }
        self.ready = false;
        self.declare_at = None;
        self.look_at = since + REDECLARE;
        self.gone_at = Some(since);
        self.update_talker();
        self.lost_participant(since);
    }

    /// The participant plaa declared its listener in went, and with it
    /// what it registered: plaa listens again once it has another.
    fn lost_participant(&mut self, since: Duration) {
        self.mode = None;
        self.copies = None;
        self.update_lite_status();
        self.listening.talker = None;
        self.listening.asked = false;
        self.listening.declared = None;
        self.listening.refused = false;
        self.update_listener(since);
    }

    /// Looks for a program serving the port's participant, to share it;
    /// where none does and plaa has no participant, it runs its own.
    fn look(&mut self, now: Instant, since: Duration) {
        self.look_at = since + LOOK;
        let shared = self
            .reservation
            .as_ref()
            .is_some_and(Reservation::is_shared);
        if shared || self.share(now, since) || self.reservation.is_some() || self.blocked.is_some()
        {
            return;
        }
        match alone(since, self.gone_at, reservation::taken(&self.interface)) {
            Alone::Wait => return,
            Alone::Blocked(why) => {
                self.blocked = Some(why);
                self.entity.depart();
                return;
            }
            Alone::Own => {}
        }
        match Reservation::own(&self.interface, self.mac, &self.sender) {
            Ok(mut own) => {
                own.begin(since);
                self.reservation = Some(own);
                self.arm_lite(since);
                if self.addressed() {
                    self.declare_at = Some(self.redeclare_at(since));
                }
                self.ask_to_listen(since);
            }
            Err(error) => {
                self.blocked = Some(error.to_string());
                self.entity.depart();
            }
        }
    }

    /// When the stream can be declared again, a moment after it was
    /// withdrawn.
    fn redeclare_at(&self, since: Duration) -> Duration {
        self.withdrawn_at
            .map_or(since, |withdrawn| withdrawn + REDECLARE)
            .max(since)
    }

    /// Acts on what the port says of the stream; in AVB Lite on plaa's own
    /// participants CVU SRP says it, and MSRP's word goes unheeded.
    fn heard(&mut self, since: Duration, heard: Heard) {
        if self.cvu().is_some() && matches!(heard, Heard::Listener(_) | Heard::Talker(_)) {
            return;
        }
        match heard {
            Heard::Listener(state) => {
                self.ready = state.is_some_and(ListenerState::ready);
            }
            Heard::Unavailable => {
                // Withdrawn by the program sharing its participant, or
                // not declared; its listener too.
                self.stop_talker();
                self.declare_at = None;
                if self.declared {
                    self.declared = false;
                    self.withdrawn_at = Some(since);
                }
                self.ready = false;
                self.listening.talker = None;
                self.listening.asked = false;
                self.listening.refused = true;
                self.update_listener(since);
            }
            Heard::Available => {
                if self.addressed() && !self.declared {
                    self.declare_at = Some(self.redeclare_at(since));
                }
                self.listening.refused = false;
                self.ask_to_listen(since);
            }
            Heard::Talker(talker) => {
                self.listening.talker = talker;
                self.listening.refused = false;
                self.update_listener(since);
                return;
            }
            Heard::ListenRefused => {
                self.listening.talker = None;
                self.listening.refused = true;
                self.update_listener(since);
                return;
            }
            Heard::Mode(mode) => {
                // Into AVB Lite, the stream waits to be told where to go;
                // back in AVB, it goes as declared.
                let lite = mode.lite.is_some();
                if self.mode.map(|before| before.lite.is_some()) != Some(lite) {
                    self.copies = None;
                }
                self.mode = Some(mode);
                if let Some(found) = avb_net::interfaces()
                    .into_iter()
                    .find(|found| found.name == self.interface)
                {
                    self.link_speed = found.speed.unwrap_or(0);
                }
                self.follow_copies();
                self.update_lite_status();
                self.update_listener(since);
            }
            Heard::Copies { tag, destinations } => {
                self.copies = Some((tag, destinations));
                self.follow_copies();
                self.update_lite_status();
            }
            Heard::Declared => self.refused = None,
            Heard::Taken => {
                // Another talker here has its ID, as while the program has
                // yet to see plaa's earlier connection close.
                self.stop_talker();
                self.ready = false;
                if self.declared {
                    self.declared = false;
                    self.withdrawn_at = Some(since);
                }
                self.declare_at = Some(self.redeclare_at(since));
            }
            Heard::Refused(refusal) => {
                eprintln!(
                    "plaa: the program sharing {}'s participant refused the stream: {refusal:?}",
                    self.interface
                );
                self.stop_talker();
                self.ready = false;
                self.declared = false;
                self.declare_at = None;
                self.refused = Some(format!(
                    "triib's entity service on {} could not declare plaa's stream; update \
                     plaa and triib",
                    self.interface
                ));
            }
        }
        self.update_talker();
    }

    pub(crate) fn handle_gptp(
        &mut self,
        now: Instant,
        since: Duration,
        status: Option<gptp::Status>,
    ) {
        let answered = status.is_some();
        // ptp4l on another interface says nothing of this one.
        let ours = status.filter(|status| status.own_clock == self.own_clock);
        self.ptp = match (&ours, answered) {
            (Some(_), _) => Ptp::Running,
            (None, true) => Ptp::Elsewhere,
            (None, false) => self.missing_ptp(),
        };
        let gptp = ours
            .as_ref()
            .map(|status| status.gptp.clone())
            .unwrap_or_default();
        self.entity.set_gptp(now, gptp);
        if let Some(status) = &ours {
            self.link.grandmaster(status.gptp.grandmaster);
        }
        self.gptp = ours;
        self.lite_ptp4l(since);
        self.update_lite_status();
    }

    /// No ptp4l answers: plaa starts its service, once, where it is
    /// installed and no ptp4l runs on the interface that might answer
    /// in a moment.
    fn missing_ptp(&mut self) -> Ptp {
        if self.ptp_tried {
            return match &self.ptp {
                Ptp::Starting | Ptp::Missing(_) => self.ptp.clone(),
                _ => Ptp::Missing(None),
            };
        }
        if ptp::running_on(&self.interface) {
            return Ptp::Starting;
        }
        if !ptp::installed() {
            return Ptp::Missing(None);
        }
        self.ptp_tried = true;
        match ptp::start(&self.interface) {
            Ok(()) => {
                self.ptp_started = Some(std::time::Instant::now());
                Ptp::Starting
            }
            Err(why) => Ptp::Missing(Some(why)),
        }
    }

    /// How ptp4l stands on the interface.
    pub(crate) fn ptp(&self) -> Ptp {
        self.ptp.clone()
    }

    /// The output's rate changed: the stream takes the new buffer.
    pub(crate) fn sound_changed(&mut self) {
        if self.talker.is_some() {
            self.start_talker();
        }
    }

    /// Runs the timers, then acts on what the entity, MAAP and MSRP say.
    pub(crate) fn turn(&mut self, now: Instant, since: Duration) {
        if since >= self.look_at {
            self.look(now, since);
        }
        if self.ptp == Ptp::Starting
            && self
                .ptp_started
                .is_some_and(|started| started.elapsed() >= PTP_PATIENCE)
        {
            self.ptp = Ptp::Missing(Some(
                "its service started, but ptp4l does not answer".into(),
            ));
        }
        if self.blocked.is_some() {
            return;
        }
        self.entity.handle_timeout(now);
        if let Some(reservation) = &mut self.reservation {
            reservation.handle_timeout(since);
        }
        if since >= self.counted_at + COUNT_EVERY {
            self.counted_at = since;
            let counters = self.sent.counters(self.talker.as_ref().map(Talker::stats));
            self.entity
                .set_counters(DescriptorType::STREAM_OUTPUT, 0, counters);
            self.count_received();
            let up = link_up(&self.interface);
            self.link.link(up);
            self.entity
                .set_counters(DescriptorType::AVB_INTERFACE, 0, self.link.counters());
            self.lite_link(since, up);
        }
        self.maap.handle_timeout(now);
        while let Some(event) = self.maap.poll_event() {
            match event {
                MaapEvent::Acquired(_) => {
                    let Some(address) = self.maap.address(0) else {
                        continue;
                    };
                    if let Some(file) = &self.address_file {
                        keep_address(file, &self.interface, address);
                    }
                    let kept = self.kept.take();
                    if kept == Some(address) {
                        // In use already.
                        continue;
                    }
                    if kept.is_some() {
                        // The kept one was not to be had: the stream moves.
                        self.stop_talker();
                        self.withdraw(since);
                    }
                    self.entity.set_output_destination(0, address);
                    self.declare_at = Some(self.redeclare_at(since));
                }
                MaapEvent::Lost => {
                    self.stop_talker();
                    self.declare_at = None;
                    self.withdraw(since);
                }
            }
        }
        if self.declare_at.is_some_and(|at| at <= since) {
            self.declare_at = None;
            self.declare(since);
        }
        while let Some(event) = self.entity.poll_event() {
            match event {
                EntityEvent::StreamFormatChanged {
                    descriptor_type: DescriptorType::STREAM_OUTPUT,
                    ..
                } => {
                    // Its frames' size changed: declared again, and sent
                    // anew.
                    if self.declared {
                        self.stop_talker();
                        self.withdraw(since);
                        self.declare_at = Some(since + REDECLARE);
                    }
                }
                EntityEvent::MaxTransitTimeChanged { .. } if self.talker.is_some() => {
                    self.start_talker();
                }
                EntityEvent::LiteConfigChanged { escalation_allowed } => {
                    // The port's to decide, for every talker on it: the
                    // program sharing its participant takes it, and says
                    // the Mode that follows; on plaa's own, plaa does.
                    if self.own_lite.is_some() {
                        self.lite_escalation(since, escalation_allowed);
                    } else if let Some(reservation) = &mut self.reservation {
                        reservation.escalate(escalation_allowed);
                    }
                }
                EntityEvent::InputSettled { index: 0, stream } => self.settled(since, stream),
                EntityEvent::InputUnsettled { index: 0 }
                | EntityEvent::InputUnbound { index: 0 } => {
                    self.stop_listening(since);
                }
                EntityEvent::StreamFormatChanged {
                    descriptor_type: DescriptorType::STREAM_INPUT,
                    ..
                } => {
                    // Taken anew in its new format.
                    self.stop_receiver();
                    self.update_listener(since);
                }
                _ => {}
            }
        }
        let ours = self.entity.output_stream(0).map(|(id, _)| id.0);
        let listened = self.listened();
        while let Some(heard) = self
            .reservation
            .as_mut()
            .and_then(|reservation| reservation.poll_heard(ours, listened))
        {
            self.heard(since, heard);
        }
        self.lite_turn(since);
    }

    /// Why the input cannot reserve its stream, where it cannot.
    pub(crate) fn listen_blocked(&self) -> Option<crate::ListenBlocked> {
        self.listening.stream?;
        if self.listening.refused {
            return Some(crate::ListenBlocked::AvbLite);
        }
        self.reservation
            .as_ref()
            .filter(|reservation| reservation.listens() == Some(false))
            .map(|_| crate::ListenBlocked::OldService)
    }

    /// The stream the input listens to, once found.
    fn listened(&self) -> Option<u64> {
        self.listening.stream.map(|stream| stream.stream_id.0)
    }

    /// The probe found the bound talker's stream: plaa listens to it, or
    /// to where it moved.
    fn settled(&mut self, since: Duration, stream: ProbedStream) {
        if self.listened() != Some(stream.stream_id.0) {
            self.stop_listening(since);
        }
        self.listening.stream = Some(stream);
        self.ask_to_listen(since);
        self.update_listener(since);
    }

    /// Asks the participant to listen to the input's stream, where it can
    /// and has yet to be asked; plaa's own participants answer at once
    /// with its talker.
    fn ask_to_listen(&mut self, since: Duration) {
        let Some(stream_id) = self.listened() else {
            return;
        };
        if self.listening.asked {
            return;
        }
        let entity = self.entity.entity_id();
        if let Some(participant) = self.cvu() {
            let talker = participant.listen(since, stream_id, Some(entity));
            self.listening.asked = true;
            self.listening.talker = talker;
            self.update_listener(since);
            return;
        }
        let Some(reservation) = self
            .reservation
            .as_mut()
            .filter(|reservation| reservation.listens() == Some(true))
        else {
            return;
        };
        self.listening.asked = true;
        if let Some(heard) = reservation.listen(stream_id) {
            self.heard(since, heard);
        }
    }

    /// The input lost its stream: plaa's listener is withdrawn, and the
    /// stream stops playing.
    fn stop_listening(&mut self, since: Duration) {
        let listened = self.listened();
        if let (Some(stream_id), Some(participant)) = (listened, self.cvu()) {
            participant.listen(since, stream_id, None);
        } else if let (Some(stream_id), Some(reservation)) = (listened, &mut self.reservation)
            && (self.listening.asked || self.listening.declared.is_some())
        {
            reservation.unlisten(since, stream_id);
        }
        self.stop_receiver();
        self.listening.stream = None;
        self.listening.talker = None;
        self.listening.asked = false;
        self.listening.declared = None;
        self.entity
            .set_input_reservation(0, InputReservation::default());
    }

    /// Declares plaa's listener as the stream's talker is registered, tells
    /// controllers how it stands, and takes the stream once it is
    /// reserved, or, beside a participant that cannot listen for plaa,
    /// takes what comes.
    fn update_listener(&mut self, since: Duration) {
        let Some(stream) = self.listening.stream else {
            self.stop_receiver();
            return;
        };
        let stream_id = stream.stream_id.0;
        let talker = self.listening.talker;
        let state = talker.map(|talker| {
            if talker.failure.is_some() {
                ListenerState::AskingFailed
            } else {
                ListenerState::Ready
            }
        });
        // In AVB Lite on plaa's own participants CVU SRP declares it.
        if self.listening.declared != state
            && self.cvu().is_none()
            && let Some(reservation) = &mut self.reservation
        {
            reservation.declare_listener(since, stream_id, state);
            self.listening.declared = state;
        }
        self.entity.set_input_reservation(
            0,
            InputReservation {
                talker_registered: talker.is_some_and(|talker| talker.failure.is_none()),
                failure: talker
                    .and_then(|talker| talker.failure)
                    .map(|failure| (failure.code, failure.bridge_id)),
                accumulated_latency: talker.map_or(0, |talker| talker.accumulated_latency),
                registering: state.is_some(),
            },
        );
        let cannot_listen = self
            .reservation
            .as_ref()
            .is_some_and(|reservation| reservation.listens() == Some(false));
        // Where the stream goes, as its talker declares it now: the probe
        // can have answered before the talker moved it.
        let destination = talker.map_or(stream.destination, |talker| talker.destination);
        let lite = self.mode.is_some_and(|mode| mode.lite.is_some());
        let destinations = receive_at(lite, self.mac, destination, stream.destination);
        if state == Some(ListenerState::Ready) || cannot_listen {
            let receiving = Some((stream_id, destinations));
            if self.listening.receiving != receiving
                && let Some((stream_id, destinations)) = receiving
            {
                self.start_receiver(stream_id, destinations);
            }
        } else {
            self.stop_receiver();
        }
    }

    /// Plays the stream `stream_id`, sent to `destinations`, on the sound
    /// output, or nowhere where there is none.
    fn start_receiver(&mut self, stream_id: u64, destinations: Vec<MacAddress>) {
        self.stop_receiver();
        let Some(format) = self
            .entity
            .model()
            .inputs
            .first()
            .map(|stream| stream.current_format)
        else {
            return;
        };
        let config = ListenerConfig {
            interface: self.interface.clone(),
            stream_id,
            destinations: destinations.clone(),
            format,
            sink: match &self.listening.into {
                Some(buffer) => Sink::Shared(buffer.clone()),
                None => Sink::Device {
                    name: None,
                    first_channel: 0,
                },
            },
            recovered: None,
            // The first two of eight channels, as plaa sends its own.
            channel_map: format
                .channels()
                .is_some_and(|channels| channels != CHANNELS)
                .then(|| (0..CHANNELS).map(Some).collect()),
        };
        let started = Listener::start(config.clone(), self.clock.clone()).or_else(|error| {
            eprintln!("plaa: the AVB stream plays nowhere: {error}");
            Listener::start(
                ListenerConfig {
                    sink: Sink::Discard,
                    ..config
                },
                self.clock.clone(),
            )
        });
        match started {
            Ok(receiver) => {
                self.listening.receiver = Some(receiver);
                self.listening.receiving = Some((stream_id, destinations));
            }
            Err(error) => eprintln!("plaa: could not listen to the AVB stream: {error}"),
        }
    }

    /// Plays the stream into `buffer` from now on, or on the sound output
    /// where none, starting again where it plays already.
    pub(crate) fn listen_into(&mut self, buffer: Option<SharedBuffer>) {
        if self.listening.into == buffer {
            return;
        }
        self.listening.into = buffer;
        if let Some((stream_id, destinations)) = self.listening.receiving.clone() {
            self.start_receiver(stream_id, destinations);
        }
    }

    fn stop_receiver(&mut self) {
        self.listening.receiver = None;
        self.listening.receiving = None;
    }

    /// The input's counters, as Milan's GET_COUNTERS places them, from
    /// what the receiver counted; one that stopped while locked unlocked
    /// as it stopped.
    fn count_received(&mut self) {
        let counters = &mut self.listening.counters;
        match &self.listening.receiver {
            Some(receiver) => {
                let stats = receiver.stats();
                let low = |count: u64| count as u32;
                *counters = [0; 32];
                counters[0] = low(stats.media_locked);
                counters[1] = low(stats.media_unlocked);
                counters[2] = low(stats.interrupted);
                counters[3] = low(stats.sequence_mismatches);
                counters[6] = low(stats.frames_received);
                counters[8] = low(stats.unsupported_formats);
                counters[9] = low(stats.late);
                counters[10] = low(stats.early);
                counters[11] = low(stats.frames_received);
            }
            None if counters[0] > counters[1] => counters[1] += 1,
            None => return,
        }
        self.entity
            .set_counters(DescriptorType::STREAM_INPUT, 0, *counters);
    }

    /// When the entity, MAAP or MRP next need their timers run.
    pub(crate) fn poll_timeout(&self, now: Instant, since: Duration) -> Option<Duration> {
        if self.blocked.is_some() {
            return Some(self.look_at.saturating_sub(since));
        }
        let instants = [self.entity.poll_timeout(), self.maap.poll_timeout()]
            .into_iter()
            .flatten()
            .map(|at| at.saturating_duration_since(now));
        let durations = [
            self.reservation
                .as_ref()
                .and_then(Reservation::poll_timeout),
            self.declare_at,
            Some(self.look_at),
            self.lite_timeout(),
        ]
        .into_iter()
        .flatten()
        .map(|at| at.saturating_sub(since));
        instants.chain(durations).min()
    }

    /// The next ATDECC or MAAP frame to send, and where.
    pub(crate) fn poll_transmit(&mut self) -> Option<(MacAddress, Vec<u8>)> {
        self.entity.poll_transmit().or_else(|| {
            self.maap
                .poll_transmit()
                .map(|(destination, pdu)| (destination, pdu.to_vec()))
        })
    }

    /// Sends what plaa's own MSRP and MVRP queued.
    pub(crate) fn send_mrp(&mut self) {
        if let Some(reservation) = &mut self.reservation {
            reservation.send();
        }
    }

    /// Says goodbye: the stream stops and is withdrawn, and the entity
    /// departs.
    pub(crate) fn end(&mut self, since: Duration) {
        if self.blocked.is_some() {
            return;
        }
        self.stop_talker();
        self.withdraw(since);
        self.stop_listening(since);
        self.disarm_lite();
        if let Some(reservation) = &mut self.reservation {
            reservation.handle_timeout(since);
        }
        self.entity.depart();
    }

    /// The Talker Advertise for the stream, from its current format.
    fn declaration(&self) -> Option<TalkerDeclaration> {
        let (stream_id, destination) = self.entity.output_stream(0)?;
        let format = self.entity.model().outputs.first()?.current_format;
        let media = Media::of(format)?;
        Some(TalkerDeclaration {
            stream_id: stream_id.0,
            destination,
            vlan_id: VLAN,
            max_frame_size: media.pdu_length() as u16,
            // One frame each interval of its class.
            max_interval_frames: 1,
            priority: self.priority(),
            rank: true,
            accumulated_latency: TALKER_LATENCY,
            failure: None,
            event: avb_mrp::Event::New,
        })
    }

    fn declare(&mut self, since: Duration) {
        let Some(declaration) = self.declaration() else {
            return;
        };
        let entity = self.entity.entity_id();
        if let Some(participant) = self.cvu() {
            participant.declare_talker(since, entity, declaration);
            self.declared = true;
            self.update_talker();
            return;
        }
        let Some(reservation) = self
            .reservation
            .as_mut()
            .filter(|reservation| reservation.available())
        else {
            return;
        };
        reservation.declare(since, &declaration);
        self.declared = true;
        self.update_talker();
    }

    fn withdraw(&mut self, since: Duration) {
        let Some((stream_id, _)) = self.entity.output_stream(0) else {
            return;
        };
        if !self.declared {
            return;
        }
        if let Some(participant) = self.cvu() {
            participant.withdraw_talker(since, stream_id.0);
        } else if let Some(reservation) = &mut self.reservation {
            reservation.withdraw(since, stream_id.0);
        }
        self.declared = false;
        self.ready = false;
        self.withdrawn_at = Some(since);
        self.update_talker();
    }

    /// Tells controllers how the stream stands, and starts or stops it as
    /// a listener is ready or not.
    fn update_talker(&mut self) {
        self.entity.set_output_reservation(
            0,
            OutputReservation {
                ready_listeners: u16::from(self.ready),
                registering: self.declared,
            },
        );
        if self.ready && self.declared && self.talker.is_none() {
            self.start_talker();
        } else if !(self.ready && self.declared) {
            self.stop_talker();
        }
    }

    /// Stops the stream, counting what it sent.
    fn stop_talker(&mut self) {
        if let Some(talker) = self.talker.take() {
            self.sent.stopped(talker.stats());
        }
    }

    /// Where the stream goes now, and the VLAN tag it goes with: as
    /// declared, or while the port runs AVB Lite where its copies go,
    /// nowhere until plaa is told.
    fn sending(&self, destination: MacAddress) -> (Vec<MacAddress>, u16) {
        let tag = |priority: u8, vlan: u16| (u16::from(priority) << 13) | vlan;
        match (self.mode, &self.copies) {
            (Some(Mode { lite: Some(_), .. }), Some((copies_tag, copies))) => {
                (copies.clone(), *copies_tag)
            }
            (Some(mode @ Mode { lite: Some(_), .. }), None) => {
                (Vec::new(), tag(LITE_PRIORITY, mode.media_vlan))
            }
            _ => (vec![destination], tag(self.priority(), VLAN)),
        }
    }

    /// Whether the stream goes in SR class B, as its format decides.
    fn class_b(&self) -> bool {
        self.entity
            .model()
            .outputs
            .first()
            .is_some_and(|output| output.class_b())
    }

    /// The priority its frames go out on outside AVB Lite: its class's.
    fn priority(&self) -> u8 {
        if self.class_b() {
            CLASS_B_PRIORITY
        } else {
            PRIORITY
        }
    }

    /// Sends the running stream where it goes now: to its new
    /// destinations from its next frame, or started again with its new
    /// tag.
    fn follow_copies(&mut self) {
        let (Some(talker), Some((_, destination))) = (&self.talker, self.entity.output_stream(0))
        else {
            return;
        };
        let (destinations, tag) = self.sending(destination);
        if tag == self.talker_tag {
            talker.set_destinations(destinations);
        } else {
            self.start_talker();
        }
    }

    /// What GET_LITE_STATUS answers, as triib's entities answer it: once
    /// the program sharing its participant said the port's mode, which
    /// plaa's own participants never do, as they run AVB alone.
    fn update_lite_status(&mut self) {
        if self.mode.is_none() && self.entity.lite_status().is_none() {
            return;
        }
        let ptp = self.gptp.as_ref();
        let offset = ptp.and_then(|status| status.offset);
        let lite = self.mode.and_then(|mode| mode.lite);
        let mut flags = LiteFlags::empty();
        if let Some(mode) = self.mode {
            flags = LiteFlags::CAPABLE | LiteFlags::EGRESS_VALID;
            flags.set(LiteFlags::ACTIVE, lite.is_some());
            flags.set(LiteFlags::ESCALATION_ALLOWED, mode.escalation);
        }
        flags.set(LiteFlags::OFFSET_VALID, offset.is_some());
        // kb/s, each copy counted, a frame each interval of its class.
        let intervals = if self.class_b() { 4000 } else { 8000 };
        let egress = match (&self.copies, self.declaration()) {
            (Some((_, copies)), Some(declaration)) if lite.is_some() => {
                copies.len() as u64 * declaration.bandwidth(intervals) / 1000
            }
            _ => 0,
        };
        self.entity.set_lite_status(LiteStatus {
            interface: 0,
            flags,
            fallback_reason: FallbackReason(lite.unwrap_or(0)),
            // ptp4l still in gPTP keeps the profile gPTP, fallen back or
            // not.
            ptp_profile: if lite.is_some() && ptp.is_some_and(|status| status.end_to_end) {
                PtpProfile::AVB_LITE_PTP
            } else {
                PtpProfile::GPTP
            },
            ptp_domain: ptp.map_or(0, |status| status.gptp.domain),
            media_vlan_id: self.mode.map_or(0, |mode| mode.media_vlan),
            unicast_fanout_limit: self.mode.map_or(0, |mode| mode.unicast_fanout),
            link_speed: self.link_speed,
            committed_egress: egress.min(u64::from(u32::MAX)) as u32,
            grandmaster: ptp.map_or(ClockIdentity(0), |status| status.gptp.grandmaster),
            offset_from_grandmaster: offset
                .unwrap_or(0)
                .clamp(i64::from(i32::MIN), i64::from(i32::MAX))
                as i32,
        });
    }

    fn start_talker(&mut self) {
        self.stop_talker();
        let Some((stream_id, destination)) = self.entity.output_stream(0) else {
            return;
        };
        let Some(format) = self
            .entity
            .model()
            .outputs
            .first()
            .map(|stream| stream.current_format)
        else {
            return;
        };
        let transit = self.entity.max_transit_time(0).unwrap_or(2_000_000);
        // The sound in the stream's first two channels, the rest silent.
        self.sound
            .set_channels(format.channels().unwrap_or(CHANNELS));
        let (destinations, tag) = self.sending(destination);
        let config = TalkerConfig {
            interface: self.interface.clone(),
            mac: self.mac,
            stream_id: stream_id.0,
            destinations,
            vlan_id: tag & 0x0fff,
            priority: (tag >> 13) as u8,
            format,
            max_transit_time: Duration::from_nanos(u64::from(transit)),
            source: Source::Shared(self.sound.buffer()),
        };
        match Talker::start(config, self.clock.clone()) {
            Ok(talker) => {
                self.talker = Some(talker);
                self.talker_tag = tag;
                self.sent.starts = self.sent.starts.wrapping_add(1);
            }
            Err(error) => eprintln!("plaa: the AVB stream did not start: {error}"),
        }
    }
}

/// What the interface's counters count (Milan 1.3, 5.4.2.25): its link
/// going up and down, the first look at an up link counted as one going
/// up, and its grandmaster changing.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Link {
    up: Option<bool>,
    ups: u32,
    downs: u32,
    grandmaster: Option<ClockIdentity>,
    grandmaster_changes: u32,
}

impl Link {
    fn link(&mut self, up: bool) {
        match (self.up, up) {
            (Some(before), _) if before == up => return,
            (_, true) => self.ups = self.ups.wrapping_add(1),
            (Some(_), false) => self.downs = self.downs.wrapping_add(1),
            (None, false) => {}
        }
        self.up = Some(up);
    }

    fn grandmaster(&mut self, grandmaster: ClockIdentity) {
        if self.grandmaster.is_some_and(|before| before != grandmaster) {
            self.grandmaster_changes = self.grandmaster_changes.wrapping_add(1);
        }
        self.grandmaster = Some(grandmaster);
    }

    /// In GET_COUNTERS' places: LINK_UP, LINK_DOWN and GPTP_GM_CHANGED.
    fn counters(self) -> [u32; 32] {
        let mut counters = [0; 32];
        counters[0] = self.ups;
        counters[1] = self.downs;
        counters[5] = self.grandmaster_changes;
        counters
    }
}

/// Whether `interface`'s link is up: it has a carrier.
fn link_up(interface: &str) -> bool {
    std::fs::read_to_string(format!("/sys/class/net/{interface}/carrier"))
        .is_ok_and(|carrier| carrier.trim() == "1")
}

/// What plaa does where no program serves the port's participant.
#[derive(Debug, PartialEq, Eq)]
enum Alone {
    /// Waits for the one that went to come back.
    Wait,
    /// Steps aside for one that runs on the port and does not serve it,
    /// as an old triib entity service, for this reason.
    Blocked(String),
    /// Runs its own participants.
    Own,
}

/// What plaa does at `since`, finding no program serving the port's
/// participant: the one whose plaa shared went at `gone_at`, and
/// `taken` says why another holds the port without serving it, as
/// triib's entity service does a moment as it starts, and an old one
/// always.
fn alone(since: Duration, gone_at: Option<Duration>, taken: Option<String>) -> Alone {
    if gone_at.is_some_and(|gone| since < gone + SERVICE_GRACE) {
        return Alone::Wait;
    }
    match taken {
        Some(why) => Alone::Blocked(why),
        None => Alone::Own,
    }
}

/// Where the input takes its stream: where its talker sends it, as it
/// declares, `declared`, or as the probe found, `probed`. In AVB Lite
/// that is this computer's own address, `own`, while the stream comes
/// unicast, and the multicast address once it escalates: plaa takes both,
/// as the talker moves between them before it says.
fn receive_at(
    lite: bool,
    own: MacAddress,
    declared: MacAddress,
    probed: MacAddress,
) -> Vec<MacAddress> {
    if !lite && declared != UNICAST {
        return vec![declared];
    }
    let multicast = [declared, probed]
        .into_iter()
        .find(|address| address.0[0] & 1 == 1);
    std::iter::once(own).chain(multicast).collect()
}

/// The stream's address kept in `file` for `interface`, where it is one
/// of MAAP's.
fn kept_address(file: &std::path::Path, interface: &str) -> Option<MacAddress> {
    let text = std::fs::read_to_string(file).ok()?;
    let (kept_for, address) = text.trim().split_once(' ')?;
    let address: MacAddress = address.parse().ok()?;
    (kept_for == interface && address.0[..4] == [0x91, 0xe0, 0xf0, 0x00]).then_some(address)
}

/// Keeps the stream's address in `file` for the next run on `interface`.
fn keep_address(file: &std::path::Path, interface: &str, address: MacAddress) {
    if let Some(folder) = file.parent() {
        let _ = std::fs::create_dir_all(folder);
    }
    let _ = std::fs::write(file, format!("{interface} {address}\n"));
}

/// The MAAP block's destination, to listen for MAAP on.
pub(crate) const MAAP_DESTINATION: MacAddress = maap::DESTINATION;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn its_ids_are_its_own() {
        let mac = MacAddress([0xa0, 0x10, 0xa3, 0xa5, 0x17, 0x26]);
        // Beside triib's first endpoint on the same interface,
        // 0xa010a3ff00a51726.
        assert_eq!(entity_id(mac), EntityId(0xa010_a3ff_70a5_1726));
    }

    /// The stream's address is kept between runs, for its interface alone,
    /// and only one of MAAP's.
    #[test]
    fn its_address_is_kept_between_runs() {
        let folder = std::env::temp_dir().join(format!("plaa-address-{}", std::process::id()));
        let file = folder.join("avb-address");
        let address = MacAddress([0x91, 0xe0, 0xf0, 0x00, 0x63, 0x26]);
        assert_eq!(kept_address(&file, "enp2s0"), None);
        keep_address(&file, "enp2s0", address);
        assert_eq!(kept_address(&file, "enp2s0"), Some(address));
        assert_eq!(kept_address(&file, "eth1"), None);
        std::fs::write(&file, "enp2s0 01:80:c2:00:00:0e\n").unwrap();
        assert_eq!(kept_address(&file, "enp2s0"), None);
        let _ = std::fs::remove_dir_all(&folder);
    }

    /// Its counters add up every talker that ran, the running one too.
    #[test]
    fn its_counters_count_every_run() {
        let ran = |frames_sent, media_resets| TalkerStats {
            frames_sent,
            media_resets,
            ..TalkerStats::default()
        };
        let mut sent = Sent {
            starts: 2,
            ..Sent::default()
        };
        sent.stopped(ran(8000, 1));
        let counters = sent.counters(Some(ran(400, 0)));
        assert_eq!(&counters[..8], &[2, 1, 0, 1, 0, 8400, 0, 8400]);
        assert_eq!(sent.counters(None)[7], 8000);
        // They wrap, as counters do.
        sent.stopped(ran(u64::from(u32::MAX), 0));
        assert_eq!(sent.counters(None)[7], 7999);
    }

    /// Its logo is the app's icon and its vendor's Scramble Tools', PNGs
    /// small enough for controllers to read in a few dozen Address Access
    /// responses.
    #[test]
    fn its_logos_are_pngs_of_128_pixels() {
        for logo in [LOGO, VENDOR_LOGO] {
            assert!(logo.starts_with(b"\x89PNG\r\n\x1a\n"));
            // IHDR's width and height.
            assert_eq!(&logo[16..24], &[0, 0, 0, 128, 0, 0, 0, 128]);
            assert!(logo.len() < 64 * 1024);
        }
    }

    /// The link counts each time it goes up, the first look too, and down;
    /// the grandmaster each time it changes.
    #[test]
    fn its_interface_counts_links_and_grandmasters() {
        let mut link = Link::default();
        link.link(true);
        link.link(true);
        link.link(false);
        link.link(true);
        let first = ClockIdentity(0x0001_f2ff_feff_3b14);
        link.grandmaster(first);
        link.grandmaster(first);
        link.grandmaster(ClockIdentity(0xe8f6_0aff_fee0_9220));
        assert_eq!(&link.counters()[..6], &[2, 1, 0, 0, 0, 1]);
        // A link down at the first look went down for no one.
        let mut down = Link::default();
        down.link(false);
        assert_eq!(&down.counters()[..2], &[0, 0]);
        assert!(!link_up("no-such-interface-here"));
    }

    /// plaa runs its own participants only once a program that served the
    /// port's has not come back for a while, and steps aside for one that
    /// holds the port without serving it.
    #[test]
    fn its_own_participants_wait_for_the_service() {
        let gone = Some(Duration::from_secs(100));
        let second = Duration::from_secs;
        assert_eq!(alone(second(103), gone, None), Alone::Wait);
        assert_eq!(alone(second(103), gone, Some("old".into())), Alone::Wait);
        assert_eq!(alone(second(115), gone, None), Alone::Own);
        assert_eq!(
            alone(second(115), gone, Some("old".into())),
            Alone::Blocked("old".into())
        );
        assert_eq!(alone(second(3), None, None), Alone::Own);
    }

    /// In AVB Lite the input takes its stream at this computer's address
    /// and, once known, at the multicast one it escalates to.
    #[test]
    fn avb_lite_streams_come_here_or_to_their_group() {
        let own = MacAddress([0xa0, 0x10, 0xa3, 0xa5, 0x17, 0x26]);
        let group = MacAddress([0x91, 0xe0, 0xf0, 0x00, 0x63, 0x26]);
        let other = MacAddress([0x91, 0xe0, 0xf0, 0x00, 0x63, 0x27]);
        assert_eq!(receive_at(false, own, group, other), vec![group]);
        assert_eq!(receive_at(true, own, UNICAST, group), vec![own, group]);
        assert_eq!(receive_at(true, own, other, group), vec![own, other]);
        // A talker unicast to a probe that found none.
        assert_eq!(receive_at(true, own, UNICAST, UNICAST), vec![own]);
        assert_eq!(receive_at(false, own, UNICAST, group), vec![own, group]);
    }

    #[test]
    fn its_formats_are_the_profiles_and_fit() {
        let formats = formats();
        assert_eq!(formats[0], StreamFormat(0x0205_0220_0080_6000));
        for format in &formats {
            assert!(matches!(format.channels(), Some(2 | 8)));
            let rate = format.sample_rate().unwrap();
            assert!(avb_stream::formats::RATES.contains(&rate));
            assert!(Media::of(*format).is_some_and(|media| media.pdu_length() <= 1500));
        }
        // Class B's 12 samples at 48 kHz, as the ESP boards send, Milan's
        // 24 of 32 bits at 192 kHz, and AM824 as the MOTU 8D declares it.
        for format in [
            0x0205_0220_0080_c000,
            0x0209_0218_0201_8000,
            0x00a0_0208_6000_0800,
        ] {
            assert!(formats.contains(&StreamFormat(format)), "{format:#x}");
        }
    }
}
