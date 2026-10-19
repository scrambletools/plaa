//! What plays, told to the desktop through MPRIS
//! (`org.mpris.MediaPlayer2.plaa`), and the media keys, panels and
//! `playerctl` controlling it through the same.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Duration;

use zbus::blocking::Connection;
use zbus::interface;
use zbus::zvariant::{ObjectPath, OwnedValue, Value};

const PATH: &str = "/org/mpris/MediaPlayer2";
const PLAYER: &str = "org.mpris.MediaPlayer2.Player";

/// What the desktop asks of the player.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    PlayPause,
    Play,
    Pause,
    Stop,
    Next,
    Previous,
    /// Moves by this many seconds, back when negative.
    SeekBy(f64),
    /// Goes to this many seconds.
    SeekTo(f64),
    Raise,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Playback {
    Playing,
    Paused,
    #[default]
    Stopped,
}

/// What the desktop is told plays.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NowPlaying {
    pub playback: Playback,
    /// Changes with each file, so the desktop knows a new one plays.
    pub track: u64,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub length: Option<Duration>,
    /// The file's address, and its cover's.
    pub url: Option<String>,
    pub art: Option<String>,
    pub position: Duration,
    pub rate: f64,
    pub can_previous: bool,
    pub can_next: bool,
    pub can_seek: bool,
}

type Shared = Arc<Mutex<NowPlaying>>;
type Commands = Arc<dyn Fn(Command) + Send + Sync>;

/// The running service. Dropping it leaves the bus.
pub struct Service {
    connection: Connection,
    shared: Shared,
}

impl Service {
    /// Joins the session bus as plaa's MPRIS player, sending what is asked
    /// of it to `commands`. None when there is no session bus.
    pub fn start(name: &str, commands: impl Fn(Command) + Send + Sync + 'static) -> Option<Self> {
        let shared: Shared = Arc::default();
        let send: Commands = Arc::new(commands);
        let connection = zbus::blocking::connection::Builder::session()
            .ok()?
            .serve_at(PATH, Root { send: send.clone() })
            .ok()?
            .serve_at(
                PATH,
                Player {
                    shared: shared.clone(),
                    send,
                },
            )
            .ok()?
            .build()
            .ok()?;
        let name = format!("org.mpris.MediaPlayer2.{name}");
        // A second plaa takes a name of its own, as MPRIS asks.
        let own = format!("{name}.instance{}", std::process::id());
        if connection.request_name(name.as_str()).is_err() {
            connection.request_name(own.as_str()).ok()?;
        }
        Some(Self { connection, shared })
    }

    /// Tells the desktop what plays now, sending what changed.
    pub fn update(&self, now: NowPlaying) {
        let before = {
            let mut shared = self.shared.lock().unwrap_or_else(PoisonError::into_inner);
            std::mem::replace(&mut *shared, now.clone())
        };
        let mut changed: HashMap<&str, Value> = HashMap::new();
        if before.playback != now.playback {
            changed.insert("PlaybackStatus", Value::from(status(now.playback)));
        }
        if before.track != now.track
            || before.title != now.title
            || before.artist != now.artist
            || before.album != now.album
            || before.length != now.length
            || before.art != now.art
        {
            changed.insert("Metadata", Value::from(metadata(&now)));
        }
        if before.rate != now.rate {
            changed.insert("Rate", Value::from(now.rate));
        }
        for (name, old, new) in [
            ("CanGoPrevious", before.can_previous, now.can_previous),
            ("CanGoNext", before.can_next, now.can_next),
            ("CanSeek", before.can_seek, now.can_seek),
        ] {
            if old != new {
                changed.insert(name, Value::from(new));
            }
        }
        // A jump in position, as a seek, rather than the time going on.
        let expected = before.position.as_secs_f64() + 0.5 * before.rate.max(1.0);
        if before.track == now.track
            && (now.position.as_secs_f64() < before.position.as_secs_f64() - 0.5
                || now.position.as_secs_f64() > expected + 1.0)
        {
            let micros = now.position.as_micros() as i64;
            let _ = self
                .connection
                .emit_signal(None::<&str>, PATH, PLAYER, "Seeked", &(micros,));
        }
        if changed.is_empty() {
            return;
        }
        let _ = self.connection.emit_signal(
            None::<&str>,
            PATH,
            "org.freedesktop.DBus.Properties",
            "PropertiesChanged",
            &(PLAYER, changed, Vec::<&str>::new()),
        );
    }
}

fn status(playback: Playback) -> &'static str {
    match playback {
        Playback::Playing => "Playing",
        Playback::Paused => "Paused",
        Playback::Stopped => "Stopped",
    }
}

fn metadata(now: &NowPlaying) -> HashMap<String, OwnedValue> {
    let mut values: HashMap<String, Value> = HashMap::new();
    let track = format!("/io/github/scrambletools/plaa/track/{}", now.track);
    if let Ok(path) = ObjectPath::try_from(track) {
        values.insert("mpris:trackid".into(), Value::from(path));
    }
    values.insert("xesam:title".into(), Value::from(now.title.clone()));
    if let Some(artist) = &now.artist {
        values.insert("xesam:artist".into(), Value::from(vec![artist.clone()]));
    }
    if let Some(album) = &now.album {
        values.insert("xesam:album".into(), Value::from(album.clone()));
    }
    if let Some(length) = now.length {
        values.insert(
            "mpris:length".into(),
            Value::from(length.as_micros() as i64),
        );
    }
    if let Some(url) = &now.url {
        values.insert("xesam:url".into(), Value::from(url.clone()));
    }
    if let Some(art) = &now.art {
        values.insert("mpris:artUrl".into(), Value::from(art.clone()));
    }
    values
        .into_iter()
        .filter_map(|(key, value)| Some((key, OwnedValue::try_from(value).ok()?)))
        .collect()
}

struct Root {
    send: Commands,
}

#[interface(name = "org.mpris.MediaPlayer2")]
impl Root {
    fn raise(&self) {
        (self.send)(Command::Raise);
    }

    fn quit(&self) {
        (self.send)(Command::Quit);
    }

    #[zbus(property)]
    fn can_quit(&self) -> bool {
        true
    }

    #[zbus(property)]
    fn can_raise(&self) -> bool {
        true
    }

    #[zbus(property)]
    fn has_track_list(&self) -> bool {
        false
    }

    #[zbus(property)]
    fn identity(&self) -> &str {
        "plaa"
    }

    #[zbus(property)]
    fn desktop_entry(&self) -> &str {
        crate::app::APP_ID
    }

    #[zbus(property)]
    fn supported_uri_schemes(&self) -> Vec<String> {
        Vec::new()
    }

    #[zbus(property)]
    fn supported_mime_types(&self) -> Vec<String> {
        Vec::new()
    }
}

struct Player {
    shared: Shared,
    send: Commands,
}

impl Player {
    fn now(&self) -> NowPlaying {
        self.shared
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

#[interface(name = "org.mpris.MediaPlayer2.Player")]
impl Player {
    fn next(&self) {
        (self.send)(Command::Next);
    }

    fn previous(&self) {
        (self.send)(Command::Previous);
    }

    fn pause(&self) {
        (self.send)(Command::Pause);
    }

    fn play_pause(&self) {
        (self.send)(Command::PlayPause);
    }

    fn stop(&self) {
        (self.send)(Command::Stop);
    }

    fn play(&self) {
        (self.send)(Command::Play);
    }

    /// Moves by `offset` microseconds.
    fn seek(&self, offset: i64) {
        (self.send)(Command::SeekBy(offset as f64 / 1_000_000.0));
    }

    /// Goes to `position` microseconds, if `track` still plays.
    fn set_position(&self, track: ObjectPath<'_>, position: i64) {
        let playing = format!("/io/github/scrambletools/plaa/track/{}", self.now().track);
        if track.as_str() == playing && position >= 0 {
            (self.send)(Command::SeekTo(position as f64 / 1_000_000.0));
        }
    }

    #[zbus(property)]
    fn playback_status(&self) -> &str {
        status(self.now().playback)
    }

    #[zbus(property)]
    fn rate(&self) -> f64 {
        self.now().rate
    }

    #[zbus(property)]
    fn minimum_rate(&self) -> f64 {
        0.25
    }

    #[zbus(property)]
    fn maximum_rate(&self) -> f64 {
        3.0
    }

    #[zbus(property)]
    fn metadata(&self) -> HashMap<String, OwnedValue> {
        metadata(&self.now())
    }

    #[zbus(property)]
    fn position(&self) -> i64 {
        self.now().position.as_micros() as i64
    }

    #[zbus(property)]
    fn can_go_next(&self) -> bool {
        self.now().can_next
    }

    #[zbus(property)]
    fn can_go_previous(&self) -> bool {
        self.now().can_previous
    }

    #[zbus(property)]
    fn can_play(&self) -> bool {
        self.now().playback != Playback::Stopped || !self.now().title.is_empty()
    }

    #[zbus(property)]
    fn can_pause(&self) -> bool {
        self.now().playback == Playback::Playing
    }

    #[zbus(property)]
    fn can_seek(&self) -> bool {
        self.now().can_seek
    }

    #[zbus(property)]
    fn can_control(&self) -> bool {
        true
    }
}
