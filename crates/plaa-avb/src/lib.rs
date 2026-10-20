//! plaa on an AVB network: a controller that finds the talkers and
//! listeners there and connects them, and on Linux plaa's own Milan
//! entity, whose one stream output sends what plaa plays once a listener
//! is ready for it.
//!
//! One thread runs it all on the interface, as triib's endpoint daemon
//! does: frames from raw sockets, the controller's and the entity's state
//! machines from the avb crates, MAAP for the stream's address, MSRP and
//! MVRP for its reservation, and gPTP from ptp4l. The entity cannot share
//! an interface with another program declaring streams there, such as
//! triib's endpoint daemon: two MRP participants on one port withdraw each
//! other's declarations.

#[cfg(target_os = "linux")]
mod gptp;
#[cfg(target_os = "linux")]
mod own;
mod runtime;
mod sound;

use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::JoinHandle;

pub use atdecc::EntityId;
pub use atdecc::stream_format::StreamFormat;
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
}

/// An entity on the network, as the controller read it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Found {
    pub id: EntityId,
    pub name: String,
    /// plaa's own.
    pub own: bool,
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

/// What the network looks like now.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Snapshot {
    /// The entities found, by name, plaa's own among them.
    pub entities: Vec<Found>,
    pub talker: TalkerState,
    /// The last command that failed, and why.
    pub failure: Option<String>,
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

/// An interface AVB can run on: wired, and on Linux with a PTP hardware
/// clock for the stream's times.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interface {
    pub name: String,
    pub up: bool,
    /// Whether it can send a stream: it has a PTP hardware clock.
    pub clock: bool,
}

/// The computer's wired interfaces, those with a PTP hardware clock first.
pub fn interfaces() -> Vec<Interface> {
    let mut found: Vec<Interface> = avb_net::interfaces()
        .into_iter()
        .filter(|interface| interface.physical && !interface.wireless)
        .map(|interface| Interface {
            name: interface.name,
            up: interface.up,
            clock: interface.hardware_clock.is_some(),
        })
        .collect();
    found.sort_by_key(|interface| (!interface.clock, interface.name.clone()));
    found
}
