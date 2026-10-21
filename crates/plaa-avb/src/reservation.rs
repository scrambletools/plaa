//! How plaa's talker reserves its stream on the port, and its listener
//! the stream it listens to: with MRP participants of its own, MSRP and
//! MVRP, where no other program here declares on the port; else through
//! the one that does, which shares its participant (`avb_mrp::sharing`),
//! as two on one port withdraw each other's declarations. triib's entity
//! service shares its own, and keeps plaa's declarations while the port
//! runs AVB Lite, telling plaa where to send each copy of its stream.

use std::io;
use std::os::unix::net::UnixStream;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::time::Duration;

use avb_mrp::msrp::{self, Domain, ListenerState, TalkerDeclaration};
use avb_mrp::sharing::{self, Hello, Mode, Notice, Refusal, Request, feature};
use avb_mrp::{Participant, Registration, mvrp};
use avb_net::{MacAddress, Socket};

use crate::runtime::{Input, read_into};

/// The VLAN class A and B's streams run on.
pub(crate) const VLAN: u16 = 2;

/// What the port says of plaa's stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Heard {
    /// The listeners for it: their state, or none once they left.
    Listener(Option<ListenerState>),
    /// The program sharing its participant cannot declare the stream now,
    /// or can again.
    Unavailable,
    Available,
    /// It declared the stream.
    Declared,
    /// Another talker on this computer declares a stream of its ID, as
    /// while the program has yet to see an earlier connection of plaa's
    /// close: plaa declares it again in a moment.
    Taken,
    /// It could not declare the stream, for a reason plaa cannot mend.
    Refused(Refusal),
    /// The talker of the stream plaa listens to, as it declares it, or
    /// none while no talker does.
    Talker(Option<TalkerDeclaration>),
    /// The program sharing its participant cannot declare listeners now,
    /// as while the port runs AVB Lite.
    ListenRefused,
    /// The port's mode, AVB or AVB Lite, as the program sharing its
    /// participant says it once plaa named its entity, and as it changes.
    Mode(Mode),
    /// Where plaa's stream goes while the port runs AVB Lite, a copy to
    /// each address, and with which VLAN tag.
    Copies {
        tag: u16,
        destinations: Vec<MacAddress>,
    },
}

/// Why plaa cannot declare on `interface`: triib's entity service runs on
/// it, too old to share its participant. It claims each interface it runs
/// on with an abstract socket name.
pub(crate) fn taken(interface: &str) -> Option<String> {
    let name = format!(" @triib-endpointd/{interface}");
    std::fs::read_to_string("/proc/net/unix")
        .is_ok_and(|sockets| sockets.lines().any(|line| line.ends_with(&name)))
        .then(|| {
            format!(
                "triib's entity service on {interface} is too old to share its reservations; \
                 update triib"
            )
        })
}

/// The program serving `interface`'s participant, connected, if one does.
fn connect(interface: &str) -> Option<UnixStream> {
    UnixStream::connect(sharing::socket_path(interface)?).ok()
}

pub(crate) enum Reservation {
    Own(Box<OwnParticipants>),
    Shared(SharedParticipant),
}

pub(crate) struct OwnParticipants {
    msrp_socket: Arc<Socket>,
    mvrp_socket: Arc<Socket>,
    msrp: Participant,
    mvrp: Participant,
    /// Ends the sockets' reader threads as these go.
    stop: Arc<AtomicBool>,
}

pub(crate) struct SharedParticipant {
    writer: UnixStream,
    /// Whether it can declare the stream now.
    available: bool,
    /// Tells its reader's closing from an earlier connection's.
    generation: u64,
    /// What it said of itself as plaa connected: none while plaa waits,
    /// and version 0, which says nothing, once plaa waited in vain.
    hello: Option<Hello>,
    connected: Duration,
    /// It said Unavailable before its hello, as it does to a program that
    /// connects while the port runs AVB Lite, not knowing yet whether the
    /// program takes it: plaa holds that until the hello says.
    held: bool,
    /// What plaa heard as it gave up waiting for the hello.
    pending: Option<Heard>,
}

impl OwnParticipants {
    /// The talker the port registered for `stream_id`: its Talker
    /// Advertise, else its Talker Failed, as one gives way to the other.
    fn talker(&self, stream_id: u64) -> Option<TalkerDeclaration> {
        let find = |wanted: u8| {
            self.msrp
                .registered()
                .find(|(attribute_type, value, _)| {
                    *attribute_type == wanted && stream_of(value) == Some(stream_id)
                })
                .and_then(|(attribute_type, value, _)| {
                    msrp::decode_talker(attribute_type, value, avb_mrp::Event::New)
                })
        };
        find(msrp::attribute::TALKER_ADVERTISE).or_else(|| find(msrp::attribute::TALKER_FAILED))
    }
}

impl SharedParticipant {
    /// The participant said hello: the Unavailable it said before stands
    /// where it takes no AVB Lite. One that does says Available once plaa
    /// names its entity, so that one was a moment's.
    fn hello_heard(&mut self) -> Option<Heard> {
        let lite = self.hello.is_some_and(|hello| hello.has(feature::AVB_LITE));
        (std::mem::take(&mut self.held) && !lite).then(|| {
            self.available = false;
            Heard::Unavailable
        })
    }
}

impl Drop for OwnParticipants {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

impl Drop for SharedParticipant {
    fn drop(&mut self) {
        // Its reader stops as the connection closes.
        let _ = self.writer.shutdown(std::net::Shutdown::Both);
    }
}

impl Reservation {
    /// The participant of the program serving `interface`'s, shared, if
    /// one does, its notices read into `sender` as of `generation`, with
    /// plaa's entity `entity`, which AVB Lite declares in the name of.
    pub(crate) fn share(
        interface: &str,
        sender: &Sender<Input>,
        generation: u64,
        since: Duration,
        entity: u64,
    ) -> Option<Self> {
        shared(connect(interface)?, sender, generation, since, entity).ok()
    }

    /// plaa's own participants.
    pub(crate) fn own(
        interface: &str,
        mac: MacAddress,
        sender: &Sender<Input>,
    ) -> io::Result<Self> {
        own(interface, mac, sender)
    }

    /// Whether another program's participant is shared.
    pub(crate) fn is_shared(&self) -> bool {
        matches!(self, Self::Shared(_))
    }

    pub(crate) fn generation(&self) -> Option<u64> {
        match self {
            Self::Shared(shared) => Some(shared.generation),
            Self::Own(_) => None,
        }
    }

    /// Whether the stream can be declared now.
    pub(crate) fn available(&self) -> bool {
        match self {
            Self::Shared(shared) => shared.available,
            Self::Own(_) => true,
        }
    }

    /// Whether plaa can listen through it: always through its own; through
    /// a program's, where it says it takes Listen, `None` while plaa waits
    /// for it to say.
    pub(crate) fn listens(&self) -> Option<bool> {
        match self {
            Self::Shared(shared) => shared.hello.map(|hello| hello.has(feature::LISTENERS)),
            Self::Own(_) => Some(true),
        }
    }

    /// Starts listening to `stream_id`: a program sharing its participant
    /// is asked to, and answers with its talker; plaa's own participants
    /// look for its talker among what they registered.
    pub(crate) fn listen(&mut self, stream_id: u64) -> Option<Heard> {
        match self {
            Self::Own(own) => Some(Heard::Talker(own.talker(stream_id))),
            Self::Shared(shared) => {
                let _ = sharing::write_request(&mut shared.writer, &Request::Listen(stream_id));
                None
            }
        }
    }

    /// Declares plaa's listener for `stream_id` in `state` on its own
    /// participants, or withdraws it for none; a program sharing its
    /// participant does that itself, as the talker comes and goes.
    pub(crate) fn declare_listener(
        &mut self,
        since: Duration,
        stream_id: u64,
        state: Option<ListenerState>,
    ) {
        let Self::Own(own) = self else {
            return;
        };
        let key = stream_id.to_be_bytes().to_vec();
        match state {
            Some(state) => {
                own.msrp
                    .declare(since, msrp::attribute::LISTENER, key, Some(state.number()));
            }
            None => own.msrp.withdraw(since, msrp::attribute::LISTENER, &key),
        }
    }

    pub(crate) fn unlisten(&mut self, since: Duration, stream_id: u64) {
        match self {
            Self::Own(own) => {
                own.msrp
                    .withdraw(since, msrp::attribute::LISTENER, &stream_id.to_be_bytes());
            }
            Self::Shared(shared) => {
                let _ = sharing::write_request(&mut shared.writer, &Request::Unlisten(stream_id));
            }
        }
    }

    /// Whether listeners on this computer, such as triib's entities, can
    /// take plaa's stream: not through a participant that tells only the
    /// network's listeners, which takes theirs for none.
    pub(crate) fn reaches_local(&self) -> bool {
        match self {
            Self::Shared(shared) => shared
                .hello
                .is_none_or(|hello| hello.has(feature::LOCAL_LISTENERS)),
            Self::Own(_) => true,
        }
    }

    /// Tells the program sharing its participant whether streams may
    /// escalate to multicast in AVB Lite, as a controller set it on plaa's
    /// entity: it decides that for every talker on the port.
    pub(crate) fn escalate(&mut self, allowed: bool) {
        if let Self::Shared(shared) = self
            && shared
                .hello
                .is_some_and(|hello| hello.has(feature::AVB_LITE))
        {
            let _ = sharing::write_request(&mut shared.writer, &Request::Escalation(allowed));
        }
    }

    /// Joins VLAN 2 and class A and B's domains, which a program sharing
    /// its participant has done already.
    pub(crate) fn begin(&mut self, since: Duration) {
        if let Self::Own(own) = self {
            own.mvrp.declare(since, mvrp::VID, mvrp::value(VLAN), None);
            for domain in [Domain::CLASS_A, Domain::CLASS_B] {
                own.msrp
                    .declare(since, msrp::attribute::DOMAIN, domain.value(), None);
            }
        }
    }

    pub(crate) fn declare(&mut self, since: Duration, declaration: &TalkerDeclaration) {
        match self {
            Self::Own(own) => own.msrp.declare(
                since,
                msrp::attribute::TALKER_ADVERTISE,
                msrp::talker_value(declaration),
                None,
            ),
            Self::Shared(shared) => {
                let _ = sharing::write_request(&mut shared.writer, &Request::Declare(*declaration));
            }
        }
    }

    pub(crate) fn withdraw(&mut self, since: Duration, stream_id: u64) {
        match self {
            Self::Own(own) => own.msrp.withdraw(
                since,
                msrp::attribute::TALKER_ADVERTISE,
                &stream_id.to_be_bytes(),
            ),
            Self::Shared(shared) => {
                let _ = sharing::write_request(&mut shared.writer, &Request::Withdraw(stream_id));
            }
        }
    }

    pub(crate) fn handle_msrp(&mut self, since: Duration, bytes: &[u8]) {
        if let Self::Own(own) = self {
            own.msrp.handle_pdu(since, bytes);
        }
    }

    pub(crate) fn handle_mvrp(&mut self, since: Duration, bytes: &[u8]) {
        if let Self::Own(own) = self {
            own.mvrp.handle_pdu(since, bytes);
        }
    }

    /// What the program sharing its participant said, of `ours`, of the
    /// stream plaa listens to, `listened`, or of the port.
    pub(crate) fn handle_notice(
        &mut self,
        notice: Notice,
        ours: Option<u64>,
        listened: Option<u64>,
    ) -> Option<Heard> {
        let Self::Shared(shared) = self else {
            return None;
        };
        match notice {
            Notice::Hello(hello) => {
                shared.hello = Some(hello);
                shared.hello_heard()
            }
            Notice::Unavailable if shared.hello.is_none() => {
                shared.held = true;
                None
            }
            Notice::Mode(mode) => Some(Heard::Mode(mode)),
            Notice::Copies {
                stream_id,
                tag,
                destinations,
            } if Some(stream_id) == ours => Some(Heard::Copies { tag, destinations }),
            Notice::Talker { stream_id, talker } if Some(stream_id) == listened => {
                Some(Heard::Talker(talker))
            }
            Notice::Refused {
                stream_id,
                refusal: Refusal::Unavailable,
            } if Some(stream_id) == listened && Some(stream_id) != ours => {
                Some(Heard::ListenRefused)
            }
            Notice::Listener { stream_id, state } if Some(stream_id) == ours => {
                Some(Heard::Listener(state))
            }
            Notice::Unavailable => {
                shared.available = false;
                Some(Heard::Unavailable)
            }
            Notice::Available => {
                shared.held = false;
                shared.available = true;
                Some(Heard::Available)
            }
            Notice::Declared(stream_id) if Some(stream_id) == ours => Some(Heard::Declared),
            // A Declare whose talker it could not read names no stream.
            Notice::Refused { stream_id, refusal } if Some(stream_id) == ours || stream_id == 0 => {
                Some(match refusal {
                    Refusal::Taken => Heard::Taken,
                    Refusal::Unavailable => {
                        shared.available = false;
                        Heard::Unavailable
                    }
                    refusal => Heard::Refused(refusal),
                })
            }
            _ => None,
        }
    }

    pub(crate) fn handle_timeout(&mut self, since: Duration) {
        match self {
            Self::Own(own) => {
                own.msrp.handle_timeout(since);
                own.mvrp.handle_timeout(since);
            }
            Self::Shared(shared) => {
                if shared.hello.is_none() && since >= shared.connected + sharing::HELLO_WAIT {
                    shared.hello = Some(Hello {
                        version: 0,
                        features: 0,
                    });
                    shared.pending = shared.hello_heard();
                }
            }
        }
    }

    /// What plaa's own participants registered for `ours`, and for the
    /// stream plaa listens to, `listened`: its talker as it is now. Of a
    /// shared one, what plaa took as it gave up waiting for its hello.
    pub(crate) fn poll_heard(&mut self, ours: Option<u64>, listened: Option<u64>) -> Option<Heard> {
        let own = match self {
            Self::Own(own) => own,
            Self::Shared(shared) => return shared.pending.take(),
        };
        while own.mvrp.poll_event().is_some() {}
        while let Some(registration) = own.msrp.poll_event() {
            match registration {
                Registration::Joined {
                    attribute_type: msrp::attribute::LISTENER,
                    value,
                    four_packed,
                } if stream_of(&value) == ours => {
                    let state = ListenerState::from_number(four_packed.unwrap_or(0));
                    return Some(Heard::Listener(Some(state)));
                }
                Registration::Left {
                    attribute_type: msrp::attribute::LISTENER,
                    value,
                } if stream_of(&value) == ours => return Some(Heard::Listener(None)),
                Registration::Joined {
                    attribute_type:
                        msrp::attribute::TALKER_ADVERTISE | msrp::attribute::TALKER_FAILED,
                    value,
                    ..
                }
                | Registration::Left {
                    attribute_type:
                        msrp::attribute::TALKER_ADVERTISE | msrp::attribute::TALKER_FAILED,
                    value,
                } if listened.is_some() && stream_of(&value) == listened => {
                    let stream_id = listened.unwrap_or_default();
                    return Some(Heard::Talker(own.talker(stream_id)));
                }
                _ => {}
            }
        }
        None
    }

    pub(crate) fn poll_timeout(&self) -> Option<Duration> {
        match self {
            Self::Own(own) => Some(own.msrp.poll_timeout().min(own.mvrp.poll_timeout())),
            Self::Shared(_) => None,
        }
    }

    /// Sends what plaa's own participants queued.
    pub(crate) fn send(&mut self) {
        if let Self::Own(own) = self {
            while let Some(pdu) = own.msrp.poll_transmit() {
                let _ = own.msrp_socket.send(avb_mrp::MSRP_DESTINATION, &pdu);
            }
            while let Some(pdu) = own.mvrp.poll_transmit() {
                let _ = own.mvrp_socket.send(avb_mrp::MVRP_DESTINATION, &pdu);
            }
        }
    }
}

fn own(interface: &str, mac: MacAddress, sender: &Sender<Input>) -> io::Result<Reservation> {
    let msrp_socket = Arc::new(Socket::open(interface, avb_mrp::ETHERTYPE_MSRP)?);
    msrp_socket.join_multicast(avb_mrp::MSRP_DESTINATION)?;
    let mvrp_socket = Arc::new(Socket::open(interface, avb_mrp::ETHERTYPE_MVRP)?);
    mvrp_socket.join_multicast(avb_mrp::MVRP_DESTINATION)?;
    let stop = Arc::new(AtomicBool::new(false));
    read_into(
        msrp_socket.clone(),
        stop.clone(),
        sender.clone(),
        "plaa-msrp",
        |source, _, bytes| Input::Msrp(source, bytes),
    )?;
    read_into(
        mvrp_socket.clone(),
        stop.clone(),
        sender.clone(),
        "plaa-mvrp",
        |source, _, bytes| Input::Mvrp(source, bytes),
    )?;
    let seed = mac
        .0
        .iter()
        .fold(0u64, |seed, octet| (seed << 8) | u64::from(*octet));
    Ok(Reservation::Own(Box::new(OwnParticipants {
        msrp_socket,
        mvrp_socket,
        msrp: Participant::new(msrp::FORMAT, Duration::ZERO, seed ^ 0x70),
        mvrp: Participant::new(mvrp::FORMAT, Duration::ZERO, seed ^ 0x5555),
        stop,
    })))
}

/// Shares the participant `stream` reaches, its notices read into `sender`
/// on a thread of their own, saying hello first, then naming plaa's
/// entity, `entity`, before anything is declared.
fn shared(
    mut stream: UnixStream,
    sender: &Sender<Input>,
    generation: u64,
    since: Duration,
    entity: u64,
) -> io::Result<Reservation> {
    let hello = Hello {
        version: sharing::VERSION,
        features: feature::AVB_LITE,
    };
    sharing::write_request(&mut stream, &Request::Hello(hello))?;
    // A participant without AVB Lite skips it.
    sharing::write_request(&mut stream, &Request::Entity(entity))?;
    let mut reader = stream.try_clone()?;
    let sender = sender.clone();
    std::thread::Builder::new()
        .name("plaa-sharing".into())
        .spawn(move || {
            while let Ok(notice) = sharing::read_notice(&mut reader) {
                if sender.send(Input::Shared(notice)).is_err() {
                    return;
                }
            }
            let _ = sender.send(Input::SharedClosed(generation));
        })?;
    Ok(Reservation::Shared(SharedParticipant {
        writer: stream,
        available: true,
        generation,
        hello: None,
        connected: since,
        held: false,
        pending: None,
    }))
}

/// The stream ID a listener or talker attribute starts with.
fn stream_of(value: &[u8]) -> Option<u64> {
    Some(u64::from_be_bytes(value.get(..8)?.try_into().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// plaa's entity, as it names it to the participant.
    const ENTITY: u64 = 0xa010_a3ff_70a5_1726;

    /// The hello plaa says and the entity it names, as it connects.
    fn connected(theirs: &mut UnixStream) {
        assert_eq!(
            sharing::read_request(theirs).unwrap(),
            Request::Hello(Hello {
                version: sharing::VERSION,
                features: feature::AVB_LITE,
            })
        );
        assert_eq!(
            sharing::read_request(theirs).unwrap(),
            Request::Entity(ENTITY)
        );
    }

    #[test]
    fn stream_ids_are_read_from_attributes() {
        assert_eq!(stream_of(&0x1234u64.to_be_bytes()), Some(0x1234));
        assert_eq!(stream_of(&[1, 2]), None);
        assert!(taken("no-such-interface-here").is_none());
    }

    /// A program serving its participant gets plaa's declarations, and
    /// plaa hears its notices, through a socket pair.
    #[test]
    fn a_shared_participant_declares_and_tells() {
        let (ours, mut theirs) = UnixStream::pair().unwrap();
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut reservation = shared(ours, &sender, 1, Duration::ZERO, ENTITY).unwrap();
        assert!(reservation.is_shared());
        connected(&mut theirs);
        let declaration = TalkerDeclaration {
            stream_id: 0x77,
            destination: MacAddress([0x91, 0xe0, 0xf0, 0, 0, 1]),
            vlan_id: VLAN,
            max_frame_size: 56,
            max_interval_frames: 1,
            priority: 3,
            rank: true,
            accumulated_latency: 125_000,
            failure: None,
            event: avb_mrp::Event::New,
        };
        reservation.declare(Duration::ZERO, &declaration);
        assert_eq!(
            sharing::read_request(&mut theirs).unwrap(),
            Request::Declare(declaration)
        );
        let notice = Notice::Listener {
            stream_id: 0x77,
            state: Some(ListenerState::Ready),
        };
        sharing::write_notice(&mut theirs, &notice).unwrap();
        let Ok(Input::Shared(heard)) = receiver.recv_timeout(Duration::from_secs(2)) else {
            panic!("no notice");
        };
        assert_eq!(
            reservation.handle_notice(heard.clone(), Some(0x77), None),
            Some(Heard::Listener(Some(ListenerState::Ready)))
        );
        // Another stream's listeners are not plaa's.
        assert_eq!(reservation.handle_notice(heard, Some(0x78), None), None);
        drop(theirs);
        assert!(matches!(
            receiver.recv_timeout(Duration::from_secs(2)),
            Ok(Input::SharedClosed(1))
        ));
    }

    /// What a participant of version 1 says back, and what plaa takes of
    /// one that says nothing.
    #[test]
    fn a_shared_participant_says_hello_and_answers() {
        let (ours, mut theirs) = UnixStream::pair().unwrap();
        let (sender, _receiver) = std::sync::mpsc::channel();
        let mut reservation = shared(ours, &sender, 1, Duration::ZERO, ENTITY).unwrap();
        connected(&mut theirs);
        // Waiting for its hello, plaa takes it for one that serves this
        // computer's listeners.
        assert!(reservation.reaches_local());
        let hello = Hello {
            version: 1,
            features: feature::LOCAL_LISTENERS,
        };
        assert_eq!(
            reservation.handle_notice(Notice::Hello(hello), Some(7), None),
            None
        );
        reservation.handle_timeout(Duration::from_secs(5));
        assert!(reservation.reaches_local());
        assert_eq!(
            reservation.handle_notice(Notice::Declared(7), Some(7), None),
            Some(Heard::Declared)
        );
        let refused = |refusal, stream_id| Notice::Refused { stream_id, refusal };
        assert_eq!(
            reservation.handle_notice(refused(Refusal::Taken, 7), Some(7), None),
            Some(Heard::Taken)
        );
        assert_eq!(
            reservation.handle_notice(refused(Refusal::Taken, 8), Some(7), None),
            None
        );
        assert_eq!(
            reservation.handle_notice(refused(Refusal::Unreadable, 0), Some(7), None),
            Some(Heard::Refused(Refusal::Unreadable))
        );
        assert_eq!(
            reservation.handle_notice(refused(Refusal::Unavailable, 7), Some(7), None),
            Some(Heard::Unavailable)
        );
        assert!(!reservation.available());

        // One that says no hello in time is of version 0, which counts
        // only the network's listeners.
        let (ours, mut theirs) = UnixStream::pair().unwrap();
        let mut old = shared(ours, &sender, 2, Duration::from_secs(10), ENTITY).unwrap();
        connected(&mut theirs);
        old.handle_timeout(Duration::from_secs(10) + sharing::HELLO_WAIT / 2);
        assert!(old.reaches_local());
        old.handle_timeout(Duration::from_secs(10) + sharing::HELLO_WAIT);
        assert!(!old.reaches_local());
        // A version 1 one without the feature, too.
        let (ours, _theirs) = UnixStream::pair().unwrap();
        let mut plain = shared(ours, &sender, 3, Duration::ZERO, ENTITY).unwrap();
        let hello = Hello {
            version: 1,
            features: 0,
        };
        plain.handle_notice(Notice::Hello(hello), Some(7), None);
        assert!(!plain.reaches_local());
    }

    /// plaa listens through a participant that says it takes Listen: it
    /// asks, hears the stream's talker, and hears when it cannot.
    #[test]
    fn a_shared_participant_listens_for_plaa() {
        let (ours, mut theirs) = UnixStream::pair().unwrap();
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut reservation = shared(ours, &sender, 1, Duration::ZERO, ENTITY).unwrap();
        connected(&mut theirs);
        // Not known until it says hello.
        assert_eq!(reservation.listens(), None);
        let hello = Hello {
            version: 2,
            features: feature::LOCAL_LISTENERS | feature::LISTENERS,
        };
        reservation.handle_notice(Notice::Hello(hello), Some(7), None);
        assert_eq!(reservation.listens(), Some(true));
        assert_eq!(reservation.listen(0x99), None, "it answers by notice");
        assert_eq!(
            sharing::read_request(&mut theirs).unwrap(),
            Request::Listen(0x99)
        );
        let talker = TalkerDeclaration {
            stream_id: 0x99,
            destination: MacAddress([0x91, 0xe0, 0xf0, 0, 0, 9]),
            vlan_id: VLAN,
            max_frame_size: 56,
            max_interval_frames: 1,
            priority: 3,
            rank: true,
            accumulated_latency: 250_000,
            failure: None,
            event: avb_mrp::Event::New,
        };
        let notice = Notice::Talker {
            stream_id: 0x99,
            talker: Some(talker),
        };
        sharing::write_notice(&mut theirs, &notice).unwrap();
        let Ok(Input::Shared(heard)) = receiver.recv_timeout(Duration::from_secs(2)) else {
            panic!("no notice");
        };
        assert_eq!(
            reservation.handle_notice(heard.clone(), Some(7), Some(0x99)),
            Some(Heard::Talker(Some(talker)))
        );
        // Another stream's talker is not the one plaa listens to.
        assert_eq!(reservation.handle_notice(heard, Some(7), Some(0x98)), None);
        let refused = Notice::Refused {
            stream_id: 0x99,
            refusal: Refusal::Unavailable,
        };
        assert_eq!(
            reservation.handle_notice(refused, Some(7), Some(0x99)),
            Some(Heard::ListenRefused)
        );
        reservation.unlisten(Duration::ZERO, 0x99);
        assert_eq!(
            sharing::read_request(&mut theirs).unwrap(),
            Request::Unlisten(0x99)
        );
        // One of version 1 cannot listen for plaa.
        let (ours, _theirs) = UnixStream::pair().unwrap();
        let mut old = shared(ours, &sender, 2, Duration::ZERO, ENTITY).unwrap();
        let hello = Hello {
            version: 1,
            features: feature::LOCAL_LISTENERS,
        };
        old.handle_notice(Notice::Hello(hello), Some(7), None);
        assert_eq!(old.listens(), Some(false));
    }

    /// Connecting while the port runs AVB Lite, plaa hears Unavailable
    /// before the participant's hello: one that takes AVB Lite says Mode
    /// and Available next, so plaa holds it; one that does not, or says
    /// no hello, leaves it standing.
    #[test]
    fn an_unavailable_before_the_hello_waits_for_it() {
        let (sender, _receiver) = std::sync::mpsc::channel();
        let lite = Hello {
            version: 2,
            features: feature::LOCAL_LISTENERS | feature::LISTENERS | feature::AVB_LITE,
        };
        let (ours, _theirs) = UnixStream::pair().unwrap();
        let mut reservation = shared(ours, &sender, 1, Duration::ZERO, ENTITY).unwrap();
        assert_eq!(
            reservation.handle_notice(Notice::Unavailable, Some(7), None),
            None
        );
        assert_eq!(
            reservation.handle_notice(Notice::Hello(lite), Some(7), None),
            None
        );
        assert!(reservation.available());
        assert_eq!(
            reservation.handle_notice(Notice::Available, Some(7), None),
            Some(Heard::Available)
        );

        let (ours, _theirs) = UnixStream::pair().unwrap();
        let mut plain = shared(ours, &sender, 2, Duration::ZERO, ENTITY).unwrap();
        plain.handle_notice(Notice::Unavailable, Some(7), None);
        let hello = Hello {
            version: 2,
            features: feature::LOCAL_LISTENERS | feature::LISTENERS,
        };
        assert_eq!(
            plain.handle_notice(Notice::Hello(hello), Some(7), None),
            Some(Heard::Unavailable)
        );
        assert!(!plain.available());
        // Once it said hello, Unavailable is heard at once.
        let (ours, _theirs) = UnixStream::pair().unwrap();
        let mut later = shared(ours, &sender, 3, Duration::ZERO, ENTITY).unwrap();
        later.handle_notice(Notice::Hello(lite), Some(7), None);
        assert_eq!(
            later.handle_notice(Notice::Unavailable, Some(7), None),
            Some(Heard::Unavailable)
        );

        let (ours, _theirs) = UnixStream::pair().unwrap();
        let mut silent = shared(ours, &sender, 4, Duration::ZERO, ENTITY).unwrap();
        silent.handle_notice(Notice::Unavailable, Some(7), None);
        assert_eq!(silent.poll_heard(Some(7), None), None);
        silent.handle_timeout(sharing::HELLO_WAIT);
        assert_eq!(silent.poll_heard(Some(7), None), Some(Heard::Unavailable));
        assert_eq!(silent.poll_heard(Some(7), None), None);
        assert!(!silent.available());
    }

    /// In AVB Lite plaa hears the port's mode and where its stream's
    /// copies go, and passes a controller's escalation on.
    #[test]
    fn a_shared_participant_runs_avb_lite() {
        let (ours, mut theirs) = UnixStream::pair().unwrap();
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut reservation = shared(ours, &sender, 1, Duration::ZERO, ENTITY).unwrap();
        connected(&mut theirs);
        // Not before the participant says it takes AVB Lite.
        reservation.escalate(true);
        let hello = Hello {
            version: 2,
            features: feature::LISTENERS | feature::AVB_LITE,
        };
        reservation.handle_notice(Notice::Hello(hello), Some(7), None);
        reservation.escalate(false);
        assert_eq!(
            sharing::read_request(&mut theirs).unwrap(),
            Request::Escalation(false)
        );
        let mode = sharing::Mode {
            lite: Some(2),
            media_vlan: 2,
            unicast_fanout: 2,
            escalation: false,
        };
        let listener = MacAddress([0xe8, 0xf6, 0x0a, 0xe0, 0x92, 0x20]);
        for notice in [
            Notice::Mode(mode),
            Notice::Copies {
                stream_id: 8,
                tag: (5 << 13) | 2,
                destinations: vec![listener],
            },
            Notice::Copies {
                stream_id: 7,
                tag: (5 << 13) | 2,
                destinations: vec![listener],
            },
        ] {
            sharing::write_notice(&mut theirs, &notice).unwrap();
        }
        let mut heard = Vec::new();
        for _ in 0..3 {
            let Ok(Input::Shared(notice)) = receiver.recv_timeout(Duration::from_secs(2)) else {
                panic!("no notice");
            };
            heard.extend(reservation.handle_notice(notice, Some(7), None));
        }
        // Another stream's copies are not plaa's.
        assert_eq!(
            heard,
            [
                Heard::Mode(mode),
                Heard::Copies {
                    tag: (5 << 13) | 2,
                    destinations: vec![listener],
                },
            ]
        );
    }
}
