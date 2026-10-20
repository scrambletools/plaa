//! How plaa's talker reserves its stream on the port: with MRP
//! participants of its own, MSRP and MVRP, where no other program here
//! declares on the port; else through the one that does, which shares its
//! participant (`avb_mrp::sharing`), as two on one port withdraw each
//! other's declarations. triib's entity service shares its own.

use std::io;
use std::os::unix::net::UnixStream;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::time::Duration;

use avb_mrp::msrp::{self, Domain, ListenerState, TalkerDeclaration};
use avb_mrp::sharing::{self, Notice, Request};
use avb_mrp::{Participant, Registration, mvrp};
use avb_net::{MacAddress, Socket};

use crate::runtime::{Input, read_into};

/// The VLAN class A and B's streams run on.
pub(crate) const VLAN: u16 = 2;

/// What the port says of plaa's stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Heard {
    /// The listeners for it: their state, or none once they left.
    Listener(Option<ListenerState>),
    /// The program sharing its participant cannot declare the stream now,
    /// or can again.
    Unavailable,
    Available,
}

/// Why plaa cannot declare on `interface`: triib's entity service runs on
/// it, too old to share its participant. It claims each interface it runs
/// on with an abstract socket name.
pub(crate) fn taken(interface: &str) -> Option<String> {
    let name = format!(" @triib-endpointd/{interface}");
    std::fs::read_to_string("/proc/net/unix")
        .is_ok_and(|sockets| sockets.lines().any(|line| line.ends_with(&name)))
        .then(|| format!("triib's entity service runs on {interface} and does not share it"))
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
    /// one does, its notices read into `sender` as of `generation`.
    pub(crate) fn share(interface: &str, sender: &Sender<Input>, generation: u64) -> Option<Self> {
        shared(connect(interface)?, sender, generation).ok()
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

    /// What the program sharing its participant said, of `ours` or of the
    /// port.
    pub(crate) fn handle_notice(&mut self, notice: Notice, ours: Option<u64>) -> Option<Heard> {
        let Self::Shared(shared) = self else {
            return None;
        };
        match notice {
            Notice::Listener { stream_id, state } if Some(stream_id) == ours => {
                Some(Heard::Listener(state))
            }
            Notice::Listener { .. } => None,
            Notice::Unavailable => {
                shared.available = false;
                Some(Heard::Unavailable)
            }
            Notice::Available => {
                shared.available = true;
                Some(Heard::Available)
            }
        }
    }

    pub(crate) fn handle_timeout(&mut self, since: Duration) {
        if let Self::Own(own) = self {
            own.msrp.handle_timeout(since);
            own.mvrp.handle_timeout(since);
        }
    }

    /// What plaa's own participants registered for `ours`.
    pub(crate) fn poll_heard(&mut self, ours: Option<u64>) -> Option<Heard> {
        let Self::Own(own) = self else {
            return None;
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
        Input::Msrp,
    )?;
    read_into(
        mvrp_socket.clone(),
        stop.clone(),
        sender.clone(),
        "plaa-mvrp",
        Input::Mvrp,
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
/// on a thread of their own.
fn shared(stream: UnixStream, sender: &Sender<Input>, generation: u64) -> io::Result<Reservation> {
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
    }))
}

/// The stream ID a listener or talker attribute starts with.
fn stream_of(value: &[u8]) -> Option<u64> {
    Some(u64::from_be_bytes(value.get(..8)?.try_into().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let mut reservation = shared(ours, &sender, 1).unwrap();
        assert!(reservation.is_shared());
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
            reservation.handle_notice(heard, Some(0x77)),
            Some(Heard::Listener(Some(ListenerState::Ready)))
        );
        // Another stream's listeners are not plaa's.
        assert_eq!(reservation.handle_notice(heard, Some(0x78)), None);
        drop(theirs);
        assert!(matches!(
            receiver.recv_timeout(Duration::from_secs(2)),
            Ok(Input::SharedClosed(1))
        ));
    }
}
