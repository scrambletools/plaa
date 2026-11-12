//! plaa's talker and listener on Linux: this computer's default entity,
//! which avb-endpointd runs on the interface, the daemon in the person's
//! session that runs every entity on this computer, whichever app made it.
//! plaa starts and enables the daemon's unit, plays what it plays into the
//! default entity's playback device, as any app can, and plays its
//! recording device while its input takes a stream; the daemon's status
//! file says how the entity and gptpd stand. plaa's controller reaches the
//! entity on the network as it reaches any other.

use std::path::PathBuf;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex, OnceLock, PoisonError};
use std::time::{Duration, Instant};

use atdecc::controller::Controller;
use atdecc::{EntityId, MacAddress};
use avb_endpointd::config::{self, Direction};
use avb_endpointd::status::{self, Blocked, DaemonStatus, EndpointStatus, Setup};
use avb_endpointd::unit::{self, UnitState};
use avb_stream::audio::{SharedBuffer, Sink, Source};
use cpal::traits::{DeviceTrait, HostTrait};

use crate::feed::{Feed, bound_input};
use crate::sound::{CHANNELS, Sound};
use crate::{Clock, ListenBlocked, Ptp, TalkerState};

/// How often it reads the daemon's status, and looks for its devices
/// while they are not open.
const LOOK: Duration = Duration::from_secs(1);

/// Whether a package, or a dev install, put the daemon's unit in.
fn installed() -> bool {
    let home = std::env::var_os("HOME").map(PathBuf::from);
    let user = home.map(|home| home.join(".config/systemd/user"));
    [
        Some(PathBuf::from("/etc/systemd/user")),
        user,
        Some(PathBuf::from("/usr/local/lib/systemd/user")),
        Some(PathBuf::from("/usr/lib/systemd/user")),
    ]
    .into_iter()
    .flatten()
    .any(|folder| folder.join("avb-endpointd@.service").exists())
}

/// Starts avb-endpointd on `interface` and enables it, so this computer is
/// on the network there from then on, at each login, with or without
/// plaa. Done on a thread of its own, one interface at a time in the order
/// asked, as systemctl can take a while. Turning plaa's sending off leaves
/// it running: its entities may be another app's too.
pub fn enable(interface: &str) {
    static WORKER: OnceLock<Mutex<Sender<String>>> = OnceLock::new();
    let worker = WORKER.get_or_init(|| {
        let (sender, asked) = mpsc::channel::<String>();
        let _ = std::thread::Builder::new()
            .name("plaa-daemon-unit".into())
            .spawn(move || {
                for interface in asked {
                    if !installed() {
                        continue;
                    }
                    if let Err(error) = unit::start(&interface) {
                        eprintln!("plaa: could not start avb-endpointd on {interface}: {error}");
                    }
                }
            });
        Mutex::new(sender)
    });
    let _ = worker
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .send(interface.to_owned());
}

/// A device's name as audio programs show it, by PipeWire's node name,
/// which the daemon makes of the entity's ID: its playback device's for
/// an output, its recording device's for an input.
fn device_named(node: &str, input: bool) -> Option<String> {
    let host = cpal::default_host();
    let devices = match input {
        true => host.input_devices().ok()?,
        false => host.output_devices().ok()?,
    };
    devices
        .into_iter()
        .find(|device| device.id().is_ok_and(|id| id.id() == node))
        .and_then(|device| Some(device.description().ok()?.name().to_owned()))
}

pub(crate) struct Daemon {
    interface: String,
    /// The default entity: at its usual instance until the daemon says
    /// where.
    entity: EntityId,
    sound: Arc<Sound>,
    /// What the daemon said last, while it runs.
    status: Option<DaemonStatus>,
    /// Why the daemon does not run, where it cannot.
    blocked: Option<String>,
    /// The entity's playback and recording devices, while the daemon
    /// gives them.
    playback: Option<String>,
    recording: Option<String>,
    feed: Option<Feed>,
    /// Plays the stream the entity's input takes, while it takes one,
    /// into `into`.
    listen: Option<Feed>,
    /// Where that stream plays: a player's buffer, or the sound output
    /// where none.
    into: Option<SharedBuffer>,
    /// The device the listening feed would not open on, not tried again
    /// until it changes.
    listen_failed: Option<String>,
    looked: Option<Instant>,
}

impl Daemon {
    pub(crate) fn new(interface: &str, mac: MacAddress, sound: Arc<Sound>) -> Self {
        Self {
            interface: interface.to_owned(),
            entity: avb_endpointd::entity_id(mac, config::DEFAULT_INSTANCE),
            sound,
            status: None,
            blocked: None,
            playback: None,
            recording: None,
            feed: None,
            listen: None,
            into: None,
            listen_failed: None,
            looked: None,
        }
    }

    pub(crate) fn entity_id(&self) -> EntityId {
        self.entity
    }

    /// What the daemon says of its default entity, while it runs.
    fn listed(&self) -> Option<&EndpointStatus> {
        self.status
            .as_ref()?
            .endpoints
            .iter()
            .find(|entity| entity.default)
    }

    pub(crate) fn state(&self) -> TalkerState {
        if let Some(why) = &self.blocked {
            return TalkerState::Blocked(why.clone());
        }
        let Some(entity) = self.listed() else {
            return TalkerState::Waiting;
        };
        let interface = &self.interface;
        if entity.held {
            return TalkerState::Blocked(format!(
                "another program, such as an older plaa, runs an entity with this computer's ID on {interface}"
            ));
        }
        let output = entity
            .streams
            .iter()
            .find(|stream| stream.direction == Direction::Out && !stream.clock);
        let Some(output) = output else {
            return TalkerState::Waiting;
        };
        match output.blocked {
            _ if output.state == "streaming" => TalkerState::Streaming,
            Some(Blocked::NoClock) => TalkerState::Blocked(format!(
                "{interface} has no PTP hardware clock to time a stream by"
            )),
            Some(Blocked::NoDevice) => TalkerState::Blocked(
                "avb-endpointd could not make the entity's playback device: PipeWire may not run"
                    .to_owned(),
            ),
            _ => TalkerState::Waiting,
        }
    }

    /// How gptpd stands on the interface, as the daemon says.
    pub(crate) fn ptp(&self) -> Ptp {
        let Some(status) = &self.status else {
            return Ptp::Unknown;
        };
        match (status.setup, &status.clock) {
            (Some(Setup::Gptpd), _) => Ptp::Starting,
            (Some(Setup::NoGptpd), _) => Ptp::Missing(None),
            (Some(Setup::Refused), _) => {
                Ptp::Missing(Some(format!("another PTP daemon holds {}", self.interface)))
            }
            (_, Some(clock)) => Ptp::Running(Clock {
                priority1: clock.priority1.unwrap_or(u8::MAX),
                grandmaster: clock.grandmaster_is_us,
                config: format!("/etc/gptpd/{}.conf", self.interface),
            }),
            _ => Ptp::Unknown,
        }
    }

    /// Why plaa's input takes only what comes to the port: AVB Lite
    /// reserves no streams.
    pub(crate) fn listen_blocked(&self) -> Option<ListenBlocked> {
        let clock = self.status.as_ref()?.clock.as_ref()?;
        clock.end_to_end.then_some(ListenBlocked::AvbLite)
    }

    /// Looks again, each second, how the daemon stands, and for the
    /// entity's devices: feeds the playback device what plaa plays while
    /// it is there, and plays the recording device while the entity's
    /// input takes a stream.
    pub(crate) fn turn(&mut self, controller: &Controller) {
        if self.looked.is_some_and(|looked| looked.elapsed() < LOOK) {
            return;
        }
        self.looked = Some(Instant::now());
        self.status = status::read(&self.interface);
        self.blocked = match self.status {
            Some(_) => None,
            None => self.why_not_running(),
        };
        let listed = self.listed().filter(|entity| !entity.held).map(|entity| {
            u64::from_str_radix(entity.entity_id.trim_start_matches("0x"), 16).map(EntityId)
        });
        let Some(Ok(entity)) = listed else {
            self.playback = None;
            self.recording = None;
            self.stop();
            return;
        };
        if entity != self.entity {
            self.entity = entity;
            self.playback = None;
            self.recording = None;
            self.stop();
        }
        let id = entity.0;
        if self.feed.is_none() {
            self.playback = device_named(&format!("avb-{id:016x}-out-1"), false);
        }
        if self.feed.is_none()
            && let Some(device) = &self.playback
        {
            self.sound.set_channels(CHANNELS);
            let source = Source::Shared(self.sound.buffer());
            let sink = Sink::Device {
                name: Some(device.clone()),
                first_channel: 0,
            };
            match Feed::start("plaa-daemon", source, sink) {
                Ok(feed) => self.feed = Some(feed),
                Err(error) => eprintln!("plaa: could not play into {device}: {error}"),
            }
        }
        if bound_input(controller, entity).is_none() {
            self.listen = None;
            self.listen_failed = None;
            return;
        }
        if self.listen.is_none() {
            self.recording = device_named(&format!("avb-{id:016x}-in-1"), true);
        }
        if self.listen.is_none()
            && let Some(device) = &self.recording
            && self.listen_failed.as_ref() != Some(device)
        {
            let source = Source::Device {
                name: Some(device.clone()),
                first_channel: 0,
            };
            let sink = match &self.into {
                Some(buffer) => Sink::Shared(buffer.clone()),
                None => Sink::Device {
                    name: None,
                    first_channel: 0,
                },
            };
            match Feed::start("plaa-daemon-listen", source, sink) {
                Ok(feed) => {
                    self.listen = Some(feed);
                    self.listen_failed = None;
                }
                Err(error) => {
                    eprintln!("plaa: could not listen to {device}: {error}");
                    self.listen_failed = Some(device.clone());
                }
            }
        }
    }

    /// Why the daemon does not run on the interface, where it cannot, as
    /// its unit says; none while it starts.
    fn why_not_running(&self) -> Option<String> {
        if !installed() {
            return Some(
                "avb-endpointd, which runs this computer's entity, is not installed".to_owned(),
            );
        }
        let interface = &self.interface;
        match unit::state(interface).ok()? {
            UnitState::Active | UnitState::Inactive => None,
            UnitState::Refused => Some(format!(
                "another person's avb-endpointd runs the entities on {interface}"
            )),
            UnitState::NoInterface => Some(format!("avb-endpointd finds no interface {interface}")),
            UnitState::BadConfig => {
                let path = config::path(interface).unwrap_or_default();
                Some(format!("avb-endpointd cannot read {}", path.display()))
            }
            UnitState::Failed => Some(format!(
                "avb-endpointd stopped on {interface}: see journalctl --user -u {}",
                unit::name(interface).unwrap_or_default()
            )),
        }
    }

    /// Plays the stream the entity's input takes into `buffer` from now
    /// on, or on the sound output where none, starting again where it
    /// plays already.
    pub(crate) fn listen_into(&mut self, buffer: Option<SharedBuffer>) {
        if self.into == buffer {
            return;
        }
        self.into = buffer;
        self.listen = None;
        self.listen_failed = None;
        self.looked = None;
    }

    /// What plays arrives at another rate: the feed takes the new buffer.
    pub(crate) fn sound_changed(&mut self) {
        if self.feed.is_some() {
            self.feed = None;
            self.looked = None;
        }
    }

    fn stop(&mut self) {
        self.feed = None;
        self.listen = None;
        self.listen_failed = None;
    }
}
