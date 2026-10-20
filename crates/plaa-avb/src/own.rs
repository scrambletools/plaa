//! plaa's own Milan entity: one stream output of two channels that sends
//! what plaa plays, on gPTP time, once a listener on the network is ready
//! for it. MAAP claims the stream's multicast address; MSRP advertises it
//! and hears listeners, MVRP joins its VLAN, as triib's endpoints do.

use std::io;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;
use std::time::Duration;

use atdecc::descriptor::DescriptorType;
use atdecc::entity::{
    EndpointModel, Entity, EntityEvent, OutputReservation, StreamModel, UNADDRESSED,
};
use atdecc::maap::{self, Maap, MaapEvent};
use atdecc::stream_format::StreamFormat;
use atdecc::{ClockIdentity, EntityId, EntityModelId, Instant, MacAddress, StreamId};
use avb_mrp::msrp::{self, Domain, ListenerState, TalkerDeclaration};
use avb_mrp::{Participant, Registration, mvrp};
use avb_net::Socket;
use avb_stream::audio::Source;
use avb_stream::media::Media;
use avb_stream::{MediaClock, Talker, TalkerConfig};

use crate::runtime::{Input, read_into};
use crate::sound::{CHANNELS, Sound};
use crate::{Config, TalkerState, gptp};

/// The middle of plaa's entity ID, after the interface address's first
/// half: triib's endpoints count up from 0 there.
const INSTANCE: u8 = 0x70;

/// plaa's entity model, under Scramble Tools' MA-S block, as triib's.
const ENTITY_MODEL: EntityModelId = EntityModelId(0x8c1f_6436_c020_0001);

/// What the stream sends: AAF at 48 kHz, six samples a frame, in 32 bits
/// or 24 in 32, of two channels or eight, the sizes Milan's Basic Audio
/// profile has, the sound in the first two of eight.
const FORMATS: [StreamFormat; 4] = [
    StreamFormat(0x0205_0220_0080_6000),
    StreamFormat(0x0205_0218_0080_6000),
    StreamFormat(0x0205_0220_0200_6000),
    StreamFormat(0x0205_0218_0200_6000),
];

/// The VLAN and priority of class A streams.
const VLAN: u16 = 2;
const PRIORITY: u8 = 3;

/// The latency the talker adds to the stream's path, in nanoseconds.
const TALKER_LATENCY: u32 = 125_000;

/// How long after withdrawing the stream it is declared again, so a
/// bridge has let the old declaration go.
const REDECLARE: Duration = Duration::from_secs(2);

/// ptp4l's read-only management socket.
const PTP4L_SOCKET: &str = "/var/run/ptp4lro";

/// plaa's entity ID on an interface with address `mac`.
pub(crate) fn entity_id(mac: MacAddress) -> EntityId {
    let [a, b, c, d, e, f] = mac.0;
    EntityId(u64::from_be_bytes([a, b, c, 0xff, INSTANCE, d, e, f]))
}

/// Why plaa's entity cannot run on `interface`, if another program
/// declares streams there: triib's endpoint daemon claims each interface
/// it runs on with an abstract socket name.
fn taken(interface: &str) -> Option<String> {
    let name = format!(" @triib-endpointd/{interface}");
    std::fs::read_to_string("/proc/net/unix")
        .is_ok_and(|sockets| sockets.lines().any(|line| line.ends_with(&name)))
        .then(|| format!("triib's entity service runs on {interface}"))
}

pub(crate) struct Own {
    entity: Entity,
    maap: Maap,
    msrp_socket: Arc<Socket>,
    mvrp_socket: Arc<Socket>,
    msrp: Participant,
    mvrp: Participant,
    clock: Arc<MediaClock>,
    sound: Arc<Sound>,
    talker: Option<Talker>,
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
    /// Why it stopped: another program declares streams on the port.
    blocked: Option<String>,
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
        if let Some(why) = taken(&interface) {
            return Err(io::Error::other(why));
        }
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
        let msrp_socket = Arc::new(Socket::open(&interface, avb_mrp::ETHERTYPE_MSRP)?);
        msrp_socket.join_multicast(avb_mrp::MSRP_DESTINATION)?;
        let mvrp_socket = Arc::new(Socket::open(&interface, avb_mrp::ETHERTYPE_MVRP)?);
        mvrp_socket.join_multicast(avb_mrp::MVRP_DESTINATION)?;
        read_into(
            msrp_socket.clone(),
            stop.clone(),
            sender.clone(),
            "plaa-msrp",
            Input::Msrp,
        )?;
        read_into(
            mvrp_socket.clone(),
            stop.clone(),
            sender.clone(),
            "plaa-mvrp",
            Input::Mvrp,
        )?;
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
            sampling_rates: vec![48_000],
            current_sampling_rate: 48_000,
            outputs: vec![StreamModel {
                name: "Playback".into(),
                formats: FORMATS.to_vec(),
                current_format: FORMATS[0],
                channels: CHANNELS,
            }],
            inputs: Vec::new(),
            clock_source: 0,
            clock_source_inputs: Vec::new(),
        };
        let stream = StreamId::new(mac, u16::from(INSTANCE) << 8);
        let seed = mac
            .0
            .iter()
            .fold(0u64, |seed, octet| (seed << 8) | u64::from(*octet));
        Ok(Self {
            entity: Entity::new(model, &[(stream, UNADDRESSED)]),
            maap: Maap::new(mac, 1),
            msrp_socket,
            mvrp_socket,
            msrp: Participant::new(msrp::FORMAT, Duration::ZERO, seed ^ INSTANCE as u64),
            mvrp: Participant::new(mvrp::FORMAT, Duration::ZERO, seed ^ 0x5555),
            clock,
            sound,
            talker: None,
            interface,
            mac,
            own_clock,
            ready: false,
            declared: false,
            declare_at: None,
            withdrawn_at: None,
            blocked: None,
        })
    }

    pub(crate) fn entity_id(&self) -> EntityId {
        self.entity.entity_id()
    }

    /// Starts advertising, joins VLAN 2 and class A and B's domains, and
    /// claims the stream's address.
    pub(crate) fn begin(&mut self, now: Instant, since: Duration) {
        self.entity.start(now);
        self.mvrp.declare(since, mvrp::VID, mvrp::value(VLAN), None);
        for domain in [Domain::CLASS_A, Domain::CLASS_B] {
            self.msrp
                .declare(since, msrp::attribute::DOMAIN, domain.value(), None);
        }
        self.maap.start(now);
    }

    pub(crate) fn state(&self) -> TalkerState {
        match (&self.blocked, &self.talker) {
            (Some(why), _) => TalkerState::Blocked(why.clone()),
            (None, Some(_)) => TalkerState::Streaming,
            (None, None) => TalkerState::Waiting,
        }
    }

    /// An ATDECC or MAAP frame from the network, or from plaa's own
    /// controller.
    pub(crate) fn handle_avtp(&mut self, now: Instant, source: MacAddress, bytes: &[u8]) {
        if self.blocked.is_some() {
            return;
        }
        if bytes.first() == Some(&atdecc::avtp::subtype::MAAP) {
            self.maap.handle_frame(now, source, bytes);
        } else {
            self.entity.handle_frame(now, source, bytes);
        }
    }

    pub(crate) fn handle_msrp(&mut self, since: Duration, source: MacAddress, bytes: &[u8]) {
        if source == self.mac {
            self.foreign();
        } else if self.blocked.is_none() {
            self.msrp.handle_pdu(since, bytes);
        }
    }

    pub(crate) fn handle_mvrp(&mut self, since: Duration, source: MacAddress, bytes: &[u8]) {
        if source == self.mac {
            self.foreign();
        } else if self.blocked.is_none() {
            self.mvrp.handle_pdu(since, bytes);
        }
    }

    /// MRP from this computer's own address that these sockets did not
    /// send: another participant on the port, whose declarations and ours
    /// would withdraw each other. The entity steps aside.
    fn foreign(&mut self) {
        if self.blocked.is_some() {
            return;
        }
        self.blocked = Some(format!(
            "another program declares streams on {}, such as triib's entity service",
            self.interface
        ));
        self.talker = None;
        self.entity.depart();
    }

    pub(crate) fn handle_gptp(&mut self, now: Instant, status: Option<gptp::Status>) {
        // ptp4l on another interface says nothing of this one.
        let gptp = status
            .filter(|status| status.own_clock == self.own_clock)
            .map(|status| status.gptp)
            .unwrap_or_default();
        self.entity.set_gptp(now, gptp);
    }

    /// The output's rate changed: the stream takes the new buffer.
    pub(crate) fn sound_changed(&mut self) {
        if self.talker.is_some() {
            self.start_talker();
        }
    }

    /// Runs the timers, then acts on what the entity, MAAP and MSRP say.
    pub(crate) fn turn(&mut self, now: Instant, since: Duration) {
        if self.blocked.is_some() {
            return;
        }
        self.entity.handle_timeout(now);
        self.msrp.handle_timeout(since);
        self.mvrp.handle_timeout(since);
        self.maap.handle_timeout(now);
        while let Some(event) = self.maap.poll_event() {
            match event {
                MaapEvent::Acquired(_) => {
                    if let Some(address) = self.maap.address(0) {
                        self.entity.set_output_destination(0, address);
                        let at = self
                            .withdrawn_at
                            .map_or(since, |withdrawn| withdrawn + REDECLARE)
                            .max(since);
                        self.declare_at = Some(at);
                    }
                }
                MaapEvent::Lost => {
                    self.talker = None;
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
                        self.talker = None;
                        self.withdraw(since);
                        self.declare_at = Some(since + REDECLARE);
                    }
                }
                EntityEvent::MaxTransitTimeChanged { .. } if self.talker.is_some() => {
                    self.start_talker();
                }
                _ => {}
            }
        }
        while let Some(registration) = self.msrp.poll_event() {
            let ours = self.entity.output_stream(0).map(|(id, _)| id.0);
            match registration {
                Registration::Joined {
                    attribute_type: msrp::attribute::LISTENER,
                    value,
                    four_packed,
                } if stream_of(&value) == ours => {
                    self.ready = ListenerState::from_number(four_packed.unwrap_or(0)).ready();
                    self.update_talker();
                }
                Registration::Left {
                    attribute_type: msrp::attribute::LISTENER,
                    value,
                } if stream_of(&value) == ours => {
                    self.ready = false;
                    self.update_talker();
                }
                _ => {}
            }
        }
        while self.mvrp.poll_event().is_some() {}
    }

    /// When the entity, MAAP or MRP next need their timers run.
    pub(crate) fn poll_timeout(&self, now: Instant, since: Duration) -> Option<Duration> {
        if self.blocked.is_some() {
            return None;
        }
        let instants = [self.entity.poll_timeout(), self.maap.poll_timeout()]
            .into_iter()
            .flatten()
            .map(|at| at.saturating_duration_since(now));
        let durations = [
            Some(self.msrp.poll_timeout()),
            Some(self.mvrp.poll_timeout()),
            self.declare_at,
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

    /// Sends what MSRP and MVRP queued.
    pub(crate) fn send_mrp(&mut self) {
        while let Some(pdu) = self.msrp.poll_transmit() {
            let _ = self.msrp_socket.send(avb_mrp::MSRP_DESTINATION, &pdu);
        }
        while let Some(pdu) = self.mvrp.poll_transmit() {
            let _ = self.mvrp_socket.send(avb_mrp::MVRP_DESTINATION, &pdu);
        }
    }

    /// Says goodbye: the stream stops and is withdrawn, and the entity
    /// departs.
    pub(crate) fn end(&mut self, since: Duration) {
        if self.blocked.is_some() {
            return;
        }
        self.talker = None;
        self.withdraw(since);
        self.msrp.handle_timeout(since);
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
            max_interval_frames: 1,
            priority: PRIORITY,
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
        self.msrp.declare(
            since,
            msrp::attribute::TALKER_ADVERTISE,
            msrp::talker_value(&declaration),
            None,
        );
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
        self.msrp.withdraw(
            since,
            msrp::attribute::TALKER_ADVERTISE,
            &stream_id.0.to_be_bytes(),
        );
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
            self.talker = None;
        }
    }

    fn start_talker(&mut self) {
        self.talker = None;
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
        let config = TalkerConfig {
            interface: self.interface.clone(),
            mac: self.mac,
            stream_id: stream_id.0,
            destinations: vec![destination],
            vlan_id: VLAN,
            priority: PRIORITY,
            format,
            max_transit_time: Duration::from_nanos(u64::from(transit)),
            source: Source::Shared(self.sound.buffer()),
        };
        match Talker::start(config, self.clock.clone()) {
            Ok(talker) => self.talker = Some(talker),
            Err(error) => eprintln!("plaa: the AVB stream did not start: {error}"),
        }
    }
}

/// The stream ID a listener or talker attribute starts with.
fn stream_of(value: &[u8]) -> Option<u64> {
    Some(u64::from_be_bytes(value.get(..8)?.try_into().ok()?))
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
        assert_eq!(stream_of(&0x1234u64.to_be_bytes()), Some(0x1234));
        assert_eq!(stream_of(&[1, 2]), None);
    }

    #[test]
    fn its_formats_are_milans_sizes_and_fit() {
        for format in FORMATS {
            assert!(matches!(format.channels(), Some(2 | 8)));
            assert_eq!(format.sample_rate(), Some(48_000));
            assert!(Media::of(format).is_some_and(|media| media.pdu_length() <= 1500));
        }
    }

    #[test]
    fn triibs_claim_is_seen() {
        assert!(taken("no-such-interface-here").is_none());
    }
}
