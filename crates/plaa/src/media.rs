//! What plays, told to the system's media controls: MPRIS on Linux, Now
//! Playing on macOS, the System Media Transport Controls on Windows. Media
//! keys and the system's panels control plaa through the same.

use std::path::PathBuf;
use std::time::Duration;

#[cfg(target_os = "linux")]
pub use crate::mpris::Service;
#[cfg(target_os = "macos")]
pub use crate::now_playing_macos::Service;
#[cfg(windows)]
pub use crate::smtc_windows::Service;

/// What the system asks of the player.
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
    #[cfg(target_os = "linux")]
    Raise,
    #[cfg(target_os = "linux")]
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Playback {
    Playing,
    Paused,
    #[default]
    Stopped,
}

/// What the system is told plays.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NowPlaying {
    pub playback: Playback,
    /// Changes with each file, so the system knows a new one plays.
    pub track: u64,
    pub title: String,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub length: Option<Duration>,
    /// The file's or stream's address.
    #[cfg(target_os = "linux")]
    pub url: Option<String>,
    /// The file's cover, saved as a picture.
    pub art: Option<PathBuf>,
    pub position: Duration,
    pub rate: f64,
    pub can_previous: bool,
    pub can_next: bool,
    pub can_seek: bool,
}

impl NowPlaying {
    /// Whether another item plays than `before`, or the same one is known
    /// better, so the system needs telling what it is.
    pub fn item_changed(&self, before: &NowPlaying) -> bool {
        self.track != before.track
            || self.title != before.title
            || self.artist != before.artist
            || self.album != before.album
            || self.length != before.length
            || self.art != before.art
    }

    /// Whether the position jumped since `before`, as a seek does, rather
    /// than the time going on.
    pub fn jumped(&self, before: &NowPlaying) -> bool {
        let position = self.position.as_secs_f64();
        let earlier = before.position.as_secs_f64();
        let expected = earlier + 0.5 * before.rate.max(1.0);
        self.track == before.track && (position < earlier - 0.5 || position > expected + 1.0)
    }
}
