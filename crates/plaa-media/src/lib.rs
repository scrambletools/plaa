//! plaa's media engine: FFmpeg reads and decodes files and streams of
//! nearly any format, sound plays through the system's output device, and
//! pictures are handed to the interface when the clock reaches them.

mod ass;
pub mod audio;
pub mod capture;
pub mod clock;
mod decode;
pub mod edit;
pub mod export;
mod ffi;
pub mod frame;
pub mod info;
mod inputs;
mod lanes;
mod mix;
pub mod player;
mod queue;
pub mod read;
pub mod record;
pub mod spectrum;
pub mod subtitle;
pub mod thumbnails;
pub mod waveform;
pub mod web;

pub use audio::{OutputDevice, output_devices};
pub use decode::{Picture, same_language};
pub use edit::{Clip, Edit, Feed, OWN, Other, Track, Transform};
pub use frame::VideoFrame;
pub use info::MediaInfo;
pub use player::{Event, LivePull, LiveSound, Options, Player, Quality, Source, Status};
pub use subtitle::SubtitlePicture;
pub use thumbnails::{Thumbnail, Thumbnails};

/// Sets FFmpeg up once: its network support, and only errors in its log.
pub fn init() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        let _ = ffmpeg_next::init();
        ffmpeg_next::format::network::init();
        let level = if std::env::var_os("PLAA_FFMPEG_LOG").is_some() {
            ffmpeg_next::util::log::Level::Verbose
        } else {
            ffmpeg_next::util::log::Level::Error
        };
        ffmpeg_next::util::log::set_level(level);
    });
}

/// libass's version, such as "0.17.3", where it is there to draw ASS
/// subtitles, for `--version`.
pub fn libass_version() -> Option<String> {
    ass::version()
}

/// FFmpeg's version, such as "9.0.1", for Settings and `--version`.
pub fn ffmpeg_version() -> String {
    let release = ffi::version();
    release.strip_prefix('n').unwrap_or(&release).to_owned()
}
