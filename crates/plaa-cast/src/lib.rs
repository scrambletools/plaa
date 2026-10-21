//! plaa casting what a window plays to a renderer on the network: DLNA and
//! UPnP AV media renderers, such as TVs, speakers and Kodi, found by SSDP,
//! and Chromecasts, found by mDNS. A file is served to the renderer over
//! HTTP from this computer while it plays there; a stream's address is
//! handed over as it is.
//!
//! A [`Session`] runs a cast on a thread of its own: it gives the
//! renderer the media, plays it from where the window was, carries out
//! play, pause, seek and volume, and tells how the renderer stands every
//! second until it is dropped, which stops the renderer.

mod castv2;
mod discovery;
mod http;
mod mdns;
mod serve;
mod upnp;

use std::path::PathBuf;
use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::time::{Duration, Instant};

pub use discovery::{Renderer, discover};
pub use upnp::State;

/// How often the renderer is asked how it stands.
const POLL: Duration = Duration::from_secs(1);

/// How long the renderer may take to start playing before a position to
/// start from is sought, as most take no seek before they play.
const START_WAIT: Duration = Duration::from_secs(10);

/// What to cast.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Media {
    /// A file, served to the renderer.
    File(PathBuf),
    /// A stream's address, which the renderer reads itself.
    Url(String),
}

/// How the renderer stands.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Status {
    pub state: State,
    /// Seconds into the media, where it says.
    pub position: Option<f64>,
    pub duration: Option<f64>,
}

/// What a session tells.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// The renderer took the media and was asked to play it.
    Started,
    Status(Status),
    /// It stopped at the end, having played.
    Ended,
    /// It could not, for this reason; the session is over.
    Failed(String),
}

enum Command {
    Play,
    Pause,
    Seek(f64),
    Volume(u8),
}

/// A cast running. Dropping it stops the renderer and the serving.
pub struct Session {
    commands: Option<Sender<Command>>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Session {
    /// Casts `media`, titled `title`, to `renderer` from `start` seconds,
    /// at `volume` (0 to 100) where the renderer takes one, telling
    /// `events` from the session's thread.
    pub fn start(
        renderer: Renderer,
        media: Media,
        title: String,
        start: f64,
        volume: Option<u8>,
        events: impl Fn(Event) + Send + 'static,
    ) -> Session {
        let (commands, received) = mpsc::channel();
        let thread = std::thread::Builder::new()
            .name("plaa-cast".into())
            .spawn(move || {
                if let Err(why) = run(&renderer, media, &title, start, volume, received, &events) {
                    events(Event::Failed(why));
                }
                // The renderer stops as the cast ends, as it would otherwise
                // go on with a file no longer served; a Chromecast's session
                // stops itself.
                if let discovery::Kind::Dlna(dlna) = &renderer.kind {
                    let _ = upnp::stop(dlna);
                }
            })
            .ok();
        Session {
            commands: Some(commands),
            thread,
        }
    }

    pub fn play(&self) {
        self.send(Command::Play);
    }

    pub fn pause(&self) {
        self.send(Command::Pause);
    }

    pub fn seek(&self, seconds: f64) {
        self.send(Command::Seek(seconds));
    }

    pub fn set_volume(&self, volume: u8) {
        self.send(Command::Volume(volume));
    }

    fn send(&self, command: Command) {
        if let Some(commands) = &self.commands {
            let _ = commands.send(command);
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        // The thread sees the channel close, and stops the renderer.
        self.commands = None;
        if let Some(thread) = self.thread.take() {
            // Not waited for: a renderer that does not answer would hold up
            // the window.
            drop(thread);
        }
    }
}

fn run(
    renderer: &Renderer,
    media: Media,
    title: &str,
    start: f64,
    volume: Option<u8>,
    commands: mpsc::Receiver<Command>,
    events: &impl Fn(Event),
) -> Result<(), String> {
    let failed = |error: std::io::Error| error.to_string();
    // Held while the cast lasts.
    let (_served, url, mime) = match media {
        Media::File(path) => {
            let mime = serve::mime(&path);
            let ip = serve::address_toward(&renderer.host()).map_err(failed)?;
            let served = serve::serve(path, mime, ip).map_err(failed)?;
            let url = served.url().to_owned();
            (Some(served), url, mime)
        }
        Media::Url(url) => {
            let mime = stream_mime(&url);
            (None, url, mime)
        }
    };
    let renderer = match &renderer.kind {
        discovery::Kind::Dlna(dlna) => dlna,
        discovery::Kind::Chromecast(address) => {
            return castv2::run(
                *address, &url, mime, title, start, volume, &commands, events,
            );
        }
    };
    let metadata = upnp::metadata(&url, title, mime);
    // A renderer playing something else stops first, as some refuse a new
    // address while they play.
    let _ = upnp::stop(renderer);
    upnp::set_uri(renderer, &url, &metadata).map_err(failed)?;
    if let Some(volume) = volume {
        let _ = upnp::set_volume(renderer, volume);
    }
    upnp::play(renderer).map_err(failed)?;
    events(Event::Started);
    let mut seek_to = (start > 1.0).then_some(start);
    let asked = Instant::now();
    let mut played = false;
    loop {
        match commands.recv_timeout(POLL) {
            Ok(command) => {
                let done = match command {
                    Command::Play => upnp::play(renderer),
                    Command::Pause => upnp::pause(renderer),
                    Command::Seek(seconds) => {
                        seek_to = None;
                        upnp::seek(renderer, seconds)
                    }
                    Command::Volume(volume) => upnp::set_volume(renderer, volume),
                };
                if let Err(error) = done {
                    eprintln!("plaa: the renderer: {error}");
                }
                continue;
            }
            Err(RecvTimeoutError::Disconnected) => return Ok(()),
            Err(RecvTimeoutError::Timeout) => {}
        }
        let state = upnp::state(renderer).map_err(failed)?;
        let (position, duration) = upnp::position(renderer).unwrap_or((None, None));
        if state == State::Playing {
            played = true;
            // Where the window was, once it plays.
            if let Some(seconds) = seek_to.take() {
                let _ = upnp::seek(renderer, seconds);
            }
        } else if seek_to.is_some() && asked.elapsed() > START_WAIT {
            seek_to = None;
        }
        events(Event::Status(Status {
            state,
            position,
            duration,
        }));
        if played && matches!(state, State::Stopped | State::NoMedia) {
            events(Event::Ended);
            return Ok(());
        }
    }
}

/// The type of a stream at `url`, by its address: HLS's and DASH's
/// playlists, else MP4, which renderers take for most streams.
fn stream_mime(url: &str) -> &'static str {
    let path = url.split(['?', '#']).next().unwrap_or(url).to_lowercase();
    if path.ends_with(".m3u8") {
        "application/x-mpegurl"
    } else if path.ends_with(".mpd") {
        "application/dash+xml"
    } else {
        "video/mp4"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn streams_are_typed_by_their_address() {
        assert_eq!(
            stream_mime("https://a.example/live/index.m3u8?token=1"),
            "application/x-mpegurl"
        );
        assert_eq!(
            stream_mime("http://a.example/manifest.MPD"),
            "application/dash+xml"
        );
        assert_eq!(stream_mime("rtsp://camera/stream"), "video/mp4");
    }
}
