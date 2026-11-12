//! plaa on an AVB network: a controller that finds the talkers and
//! listeners there and connects them, and a talker sending what plaa
//! plays once a listener is ready for it, from an entity another program
//! runs, which plaa plays into as any app can: on Linux this computer's
//! default entity, which avb-endpointd runs with every other entity on
//! the computer (`daemon`), whose first stream output sends what plays
//! into its playback device and whose first stream input plays what a
//! controller binds it to from its recording device; on a Mac the Mac's
//! own entity, which macOS runs (`host`).
//!
//! One thread runs the controller on the interface: frames from a raw
//! socket, and the controller's state machine from atdecc.

#[cfg(target_os = "linux")]
mod daemon;
#[cfg(any(target_os = "linux", target_os = "macos"))]
mod feed;
#[cfg(target_os = "macos")]
mod host;
mod runtime;
mod sound;

#[cfg(target_os = "linux")]
pub use daemon::enable as enable_daemon;

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
    /// Send what plaa plays, from this computer's entity.
    pub talker: bool,
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
    /// Under another gPTP grandmaster than plaa's entity, so keeping
    /// another time: no stream passes between them.
    pub apart: bool,
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
    /// The port runs AVB Lite, which reserves no streams.
    AvbLite,
}

/// How gptpd, which keeps the time plaa's stream is sent by, stands on
/// the interface.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Ptp {
    /// Not known yet, or not plaa's to keep: it sends no stream of its
    /// own, or the Mac keeps the time.
    #[default]
    Unknown,
    /// gptpd answers for the interface, with this computer's clock.
    Running(Clock),
    /// gptpd's service started, and gptpd has yet to answer.
    Starting,
    /// gptpd does not run on the interface: it is not installed, or
    /// refused to run there, or could not start, and why.
    Missing(Option<String>),
}

/// This computer's clock on the network, as gptpd ranks it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clock {
    /// Its priority1: 255 never makes it the grandmaster, and lower
    /// ranks it higher.
    pub priority1: u8,
    /// It is the network's grandmaster now.
    pub grandmaster: bool,
    /// gptpd's configuration for the interface, which sets priority1.
    pub config: String,
}

/// What the network looks like now.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Snapshot {
    /// The entities found, by name, plaa's own among them.
    pub entities: Vec<Found>,
    pub talker: TalkerState,
    /// The last command that failed, and why.
    pub failure: Option<String>,
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
    /// each entity's name and the formats of its streams in turn, and
    /// every second how the `chosen` inputs take their streams.
    pub fn watch(&self, listed: bool, chosen: Vec<(EntityId, u16)>) {
        let _ = self.inputs.send(Input::Watch { listed, chosen });
    }

    /// Where what plays goes, to be sent.
    pub fn sound(&self) -> Arc<Sound> {
        self.sound.clone()
    }

    /// What a player takes the stream plaa listens to from.
    pub fn listened(&self) -> Listened {
        Listened {
            inputs: self.inputs.clone(),
        }
    }

    /// The stream plaa listens to plays on the sound output again, as
    /// no player takes it.
    pub fn listen_on_output(&self) {
        let _ = self.inputs.send(Input::ListenInto(None));
    }
}

/// What drains the stream plaa listens to: two channels a chunk at a
/// time, silence where none came.
pub type Pull = Box<dyn FnMut(&mut [f32]) + Send>;

/// The stream plaa's input takes, for a player to play.
#[derive(Clone)]
pub struct Listened {
    inputs: std::sync::mpsc::Sender<Input>,
}

impl Listened {
    /// Has the stream play into a buffer at `rate` from now on, and gives
    /// what drains it, two channels a chunk at a time, as a sound device
    /// does, so the stream keeps to the device's clock.
    pub fn open(&self, rate: u32) -> Pull {
        let buffer = Arc::new(avb_stream::audio::FrameBuffer::new(
            sound::CHANNELS,
            rate as usize / 100,
        ));
        let _ = self
            .inputs
            .send(Input::ListenInto(Some(avb_stream::audio::SharedBuffer {
                buffer: buffer.clone(),
                channels: sound::CHANNELS,
                rate,
            })));
        Box::new(move |out: &mut [f32]| buffer.device_pull(out, None))
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

/// The helper that opens raw sockets for plaa's controller, the one file
/// granted CAP_NET_RAW: avb-endpointd's, which opens the daemon's too.
#[cfg(target_os = "linux")]
pub const HELPER: &str = "avb-netraw";

/// Has every raw socket come from the helper, started once: plaa itself
/// holds no capability, so the desktop's portals keep answering it.
#[cfg(target_os = "linux")]
fn use_helper() -> io::Result<()> {
    let folders =
        ["/usr/lib/avb-endpointd", "/usr/libexec/avb-endpointd"].map(std::path::Path::new);
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
