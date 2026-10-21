//! plaa on an AVB network: a controller that finds the talkers and
//! listeners there and connects them, and a talker sending what plaa
//! plays once a listener is ready for it: on Linux plaa's own Milan
//! entity, whose one stream output sends it and whose one stream input
//! plays what a controller binds it to, and on a Mac the Mac's own
//! entity, which macOS runs and plaa plays into (`host`).
//!
//! One thread runs it all on the interface, as triib's endpoint daemon
//! does: frames from raw sockets, the controller's and the entity's state
//! machines from the avb crates, MAAP for the stream's address, MSRP and
//! MVRP for its reservation, and gPTP from ptp4l. Two MRP participants on
//! one port withdraw each other's declarations, so where triib's endpoint
//! daemon runs on the interface, the entity reserves its stream in the
//! daemon's participant, which it shares.

#[cfg(target_os = "linux")]
mod gptp;
#[cfg(target_os = "macos")]
mod host;
#[cfg(target_os = "linux")]
mod own;
#[cfg(target_os = "linux")]
mod ptp;
#[cfg(target_os = "linux")]
mod reservation;
mod runtime;
mod sound;

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::JoinHandle;

pub use atdecc::aem::{StreamInfo, StreamInfoFlags, StreamInputCounters};
pub use atdecc::stream_format::StreamFormat;
pub use atdecc::{EntityId, MacAddress};
pub use sound::Sound;

use runtime::Input;

/// What to run on which interface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub interface: String,
    /// Send what plaa plays from an entity of its own.
    pub talker: bool,
    /// The entity's name, as controllers show it.
    pub name: String,
    /// Where plaa keeps its stream's address between runs, to send to it
    /// again at once while MAAP claims it anew.
    pub address_file: Option<std::path::PathBuf>,
}

/// A stream output or input of an entity on the network.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stream {
    pub index: u16,
    pub name: String,
    pub format: StreamFormat,
    /// The formats it can take.
    pub formats: Vec<StreamFormat>,
    /// For an input, the talker's output it is bound to.
    pub bound: Option<(EntityId, u16)>,
    /// What the entity last said the stream is doing, once read.
    pub info: Option<StreamInfo>,
    /// For an input, how it has taken its streams, once read.
    pub counters: Option<StreamInputCounters>,
}

/// An entity on the network, as the controller read it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Found {
    pub id: EntityId,
    pub name: String,
    /// plaa's own.
    pub own: bool,
    /// On this computer, as triib's entities are.
    pub here: bool,
    /// Its audio stream outputs and inputs, media clocks left out.
    pub outputs: Vec<Stream>,
    pub inputs: Vec<Stream>,
}

/// How plaa's own talker stands.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum TalkerState {
    #[default]
    Off,
    /// The entity cannot run on this interface, for this reason.
    Blocked(String),
    /// Advertised, and waiting for its stream's address or a listener.
    Waiting,
    /// Sending, a listener being ready.
    Streaming,
}

/// Why plaa's stream input cannot reserve the stream it listens to, and
/// takes only what comes to the port anyway.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListenBlocked {
    /// triib's entity service, whose participant plaa shares, is too old
    /// to listen for it.
    OldService,
    /// The port runs AVB Lite, which reserves no streams.
    AvbLite,
}

/// How ptp4l, which keeps the time plaa's stream is sent by, stands on
/// the interface.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Ptp {
    /// Not known yet, or not plaa's to keep: it sends no stream of its
    /// own, or the Mac keeps the time.
    #[default]
    Unknown,
    /// ptp4l answers for the interface.
    Running,
    /// ptp4l answers, for another interface.
    Elsewhere,
    /// plaa started its ptp4l service, which has yet to answer.
    Starting,
    /// No ptp4l runs on the interface: plaa's service is not installed,
    /// or could not start, and why.
    Missing(Option<String>),
}

/// What the network looks like now.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Snapshot {
    /// The entities found, by name, plaa's own among them.
    pub entities: Vec<Found>,
    pub talker: TalkerState,
    /// The last command that failed, and why.
    pub failure: Option<String>,
    /// Listeners on this computer get no stream from plaa: the
    /// participant plaa shares tells it only the network's listeners, as
    /// triib's entity service did before it said hello.
    pub local_unserved: bool,
    pub ptp: Ptp,
    pub listen_blocked: Option<ListenBlocked>,
}

/// plaa on one interface. Dropping it says goodbye to the network: the
/// entity departs and its stream is withdrawn.
pub struct Network {
    inputs: Sender<Input>,
    snapshot: Arc<Mutex<Snapshot>>,
    sound: Arc<Sound>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Network {
    /// Starts on `config`'s interface, calling `changed` from its thread
    /// whenever the snapshot changes. Fails when raw Ethernet is out of
    /// reach, as when the helper lacks CAP_NET_RAW, saying how to grant
    /// it.
    pub fn start(config: Config, changed: impl Fn() + Send + 'static) -> io::Result<Self> {
        #[cfg(target_os = "linux")]
        use_helper()?;
        let snapshot = Arc::new(Mutex::new(Snapshot::default()));
        let sound = Arc::new(Sound::new());
        let stop = Arc::new(AtomicBool::new(false));
        let (inputs, thread) = runtime::start(
            config,
            snapshot.clone(),
            sound.clone(),
            stop.clone(),
            Box::new(changed),
        )?;
        Ok(Self {
            inputs,
            snapshot,
            sound,
            stop,
            thread: Some(thread),
        })
    }

    pub fn snapshot(&self) -> Snapshot {
        self.snapshot
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    /// Binds a listener's input to a talker's output, in `format` or the
    /// talker's: each is set to it first where it is in another and can
    /// take it.
    pub fn connect(
        &self,
        talker: (EntityId, u16),
        listener: (EntityId, u16),
        format: Option<StreamFormat>,
    ) {
        let _ = self.inputs.send(Input::Connect(talker, listener, format));
    }

    pub fn disconnect(&self, listener: (EntityId, u16)) {
        let _ = self.inputs.send(Input::Disconnect(listener));
    }

    /// Keeps what the windows show of the network up to date by asking
    /// again, as entities that send no notifications need: while `listed`,
    /// each listener's name and formats in turn, and every second how the
    /// `chosen` inputs take their streams.
    pub fn watch(&self, listed: bool, chosen: Vec<(EntityId, u16)>) {
        let _ = self.inputs.send(Input::Watch { listed, chosen });
    }

    /// Where what plays goes, to be sent.
    pub fn sound(&self) -> Arc<Sound> {
        self.sound.clone()
    }
}

impl Drop for Network {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        let _ = self.inputs.send(Input::Stop);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// The helper that opens raw sockets for plaa, the one file granted
/// CAP_NET_RAW, as packages and builds put it.
#[cfg(target_os = "linux")]
pub const HELPER: &str = "plaa-netraw";

/// Has every raw socket come from the helper, started once: plaa itself
/// holds no capability, so the desktop's portals keep answering it.
#[cfg(target_os = "linux")]
fn use_helper() -> io::Result<()> {
    let folders = ["/usr/lib/plaa", "/usr/libexec/plaa"].map(std::path::Path::new);
    avb_net::broker::use_helper(HELPER, &folders)
}

/// Whether plaa may send and receive raw Ethernet here, through its
/// helper on Linux: an error saying why not.
pub fn check_access() -> io::Result<()> {
    #[cfg(target_os = "linux")]
    use_helper()?;
    avb_net::check_access()
}

/// An interface AVB can run on: wired.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interface {
    pub name: String,
    pub up: bool,
    /// Whether plaa can send a stream on it: on Linux it has a PTP
    /// hardware clock for the stream's times, on a Mac AVB is on for it.
    pub can_send: bool,
}

/// The computer's wired interfaces, those plaa can send on first.
pub fn interfaces() -> Vec<Interface> {
    let mut found: Vec<Interface> = avb_net::interfaces()
        .into_iter()
        .filter(|interface| interface.physical && !interface.wireless)
        .map(|interface| Interface {
            can_send: can_send(&interface),
            name: interface.name,
            up: interface.up,
        })
        .collect();
    found.sort_by_key(|interface| (!interface.can_send, interface.name.clone()));
    found
}

#[cfg(not(target_os = "macos"))]
fn can_send(interface: &avb_net::Interface) -> bool {
    cfg!(target_os = "linux") && interface.hardware_clock.is_some()
}

/// On a Mac, the Mac's own entity sends, once AVB is on for the interface.
#[cfg(target_os = "macos")]
fn can_send(interface: &avb_net::Interface) -> bool {
    atdecc::blocking::avb_enabled(&interface.name) == Some(true)
}
