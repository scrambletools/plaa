//! Opening what plays: a file or a stream, through FFmpeg, or the streams
//! yt-dlp finds on a web page, whose picture and sound may come apart.

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use ffmpeg::format::context::Input;
use ffmpeg_next as ffmpeg;

use crate::decode::{self, Picture};
use crate::info::MediaInfo;
use crate::player::{Event, Options, Shared, Source};
use crate::web::{self, Found, Page, WebError};

/// Where the sound input's streams are numbered from among the source's,
/// past any the picture's input has.
pub(crate) const SOUND_TRACKS: usize = 1 << 21;

/// How long a web page's thumbnail may take to arrive.
const THUMBNAIL_PATIENCE: Duration = Duration::from_secs(4);

/// What the source is read from: one input, or for a web page that keeps
/// its sound apart, the picture's and the sound's.
pub(crate) struct Inputs {
    pub main: Input,
    /// Where the main input's timestamps start, in seconds.
    pub start: f64,
    /// The sound's input, and where its timestamps start.
    pub sound: Option<(Input, f64)>,
    /// Each input has been read to its end since the last seek.
    pub main_read: bool,
    pub sound_read: bool,
}

impl Inputs {
    /// The input stream `index` of the source is in, its number there, and
    /// where that input's timestamps start.
    pub fn locate(&self, index: usize) -> (&Input, usize, f64) {
        match (&self.sound, index.checked_sub(SOUND_TRACKS)) {
            (Some((sound, start)), Some(local)) => (sound, local, *start),
            _ => (&self.main, index, self.start),
        }
    }

    /// Every input, with the number its streams start from and where its
    /// timestamps start.
    pub fn each_mut(&mut self) -> impl Iterator<Item = (&mut Input, usize, f64)> {
        std::iter::once((&mut self.main, 0, self.start)).chain(
            self.sound
                .as_mut()
                .map(|(sound, start)| (sound, SOUND_TRACKS, *start)),
        )
    }

    /// Moves every input to `position`, seconds from the start.
    pub fn seek(&mut self, position: f64) {
        self.main_read = false;
        self.sound_read = false;
        for (input, _, start) in self.each_mut() {
            let timestamp = ((position + start) * f64::from(ffmpeg::ffi::AV_TIME_BASE)) as i64;
            if let Err(error) = input.seek(timestamp, ..timestamp) {
                eprintln!("plaa: seeking: {error}");
            }
        }
    }
}

/// An opened source: its inputs, the web page they came from, and the
/// page's picture.
pub(crate) struct Opened {
    pub inputs: Inputs,
    pub page: Option<Arc<Page>>,
    pub thumbnail: Option<Picture>,
}

/// Opens `source`: through yt-dlp first when it is a web page. Tells
/// `events` what went wrong, or of a list of pages, and returns none then.
pub(crate) fn open(
    source: &Source,
    options: &Options,
    shared: &Shared,
    events: &dyn Fn(Event),
) -> Option<Opened> {
    let stop = &shared.stop;
    let mut page = options.page.clone();
    let mut no_yt_dlp = false;
    if page.is_none()
        && options.web
        && let Source::Url(url) = source
        && web::is_page(url)
    {
        match web::read(url, stop) {
            Ok(Found::Page(found)) => page = Some(Arc::new(found)),
            Ok(Found::List(title, entries)) => {
                events(Event::List(title, entries));
                return None;
            }
            // Perhaps a stream FFmpeg reads.
            Err(WebError::Unsupported) => {}
            Err(WebError::Missing) => no_yt_dlp = true,
            Err(WebError::Stopped) => return None,
            Err(WebError::Failed(why)) => {
                events(Event::Failed(why));
                return None;
            }
        }
    }
    let Some(page) = page else {
        let main = match open_input(source.path(), source.is_url(), &[], stop) {
            Ok(main) => main,
            Err(error) => {
                if !stop.load(Ordering::Relaxed) {
                    events(if no_yt_dlp && error == ffmpeg::Error::InvalidData {
                        Event::NeedsYtDlp
                    } else {
                        Event::Failed(describe(&error))
                    });
                }
                return None;
            }
        };
        return Some(Opened {
            inputs: Inputs {
                start: decode::start_seconds(&main),
                main,
                sound: None,
                main_read: false,
                sound_read: false,
            },
            page: None,
            thumbnail: None,
        });
    };
    events(Event::Page(page.clone()));
    let Some((picture, sound)) = page.choose(shared.quality()) else {
        events(Event::Failed(
            "nothing on this page can be played".to_owned(),
        ));
        return None;
    };
    // The sound and the thumbnail arrive while the picture opens.
    let (main, sound, thumbnail) = std::thread::scope(|scope| {
        let sound = sound.map(|sound| {
            scope.spawn(|| open_input(Path::new(&sound.url), true, &sound.headers, stop))
        });
        let thumbnail = page.thumbnail.as_deref().map(|address| {
            scope.spawn(move || {
                let stop = stop.clone();
                let started = Instant::now();
                decode::picture_at(address, move || {
                    stop.load(Ordering::Relaxed) || started.elapsed() > THUMBNAIL_PATIENCE
                })
            })
        });
        let main = open_input(Path::new(&picture.url), true, &picture.headers, stop);
        (
            main,
            sound.and_then(|sound| sound.join().ok()),
            thumbnail.and_then(|thumbnail| thumbnail.join().ok().flatten()),
        )
    });
    let main = match main {
        Ok(main) => main,
        Err(error) => {
            if !stop.load(Ordering::Relaxed) {
                events(Event::Failed(describe(&error)));
            }
            return None;
        }
    };
    // Without its sound, the picture plays silently.
    let sound = match sound {
        Some(Ok(sound)) => {
            let start = decode::start_seconds(&sound);
            Some((sound, start))
        }
        Some(Err(error)) => {
            events(Event::NoSound(describe(&error)));
            None
        }
        None => None,
    };
    Some(Opened {
        inputs: Inputs {
            start: decode::start_seconds(&main),
            main,
            sound,
            main_read: false,
            sound_read: false,
        },
        page: Some(page),
        thumbnail,
    })
}

/// Opens `path` with FFmpeg, cut short once `stop` is set. Addresses
/// reconnect when the connection drops, and send the site's headers.
fn open_input(
    path: &Path,
    network: bool,
    headers: &[(String, String)],
    stop: &Arc<AtomicBool>,
) -> Result<Input, ffmpeg::Error> {
    let mut dictionary = ffmpeg::Dictionary::new();
    if network {
        dictionary.set("reconnect", "1");
        dictionary.set("reconnect_streamed", "1");
        dictionary.set("rw_timeout", "15000000");
        let agent = headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("user-agent"))
            .map_or(concat!("plaa/", env!("CARGO_PKG_VERSION")), |(_, agent)| {
                agent.as_str()
            });
        dictionary.set("user_agent", agent);
        let others: String = headers
            .iter()
            .filter(|(name, _)| !name.eq_ignore_ascii_case("user-agent"))
            .map(|(name, value)| format!("{name}: {value}\r\n"))
            .collect();
        if !others.is_empty() {
            dictionary.set("headers", &others);
        }
    }
    let stop = stop.clone();
    ffmpeg::format::input_with_interrupt_and_dictionary(
        path,
        move || stop.load(Ordering::Relaxed),
        dictionary,
    )
}

/// Adds the sound input's tracks to `info`, numbered past the picture's,
/// and what the web page says of itself.
pub(crate) fn merge(info: &mut MediaInfo, sound: Option<MediaInfo>, page: Option<&Page>) {
    if let Some(sound) = sound {
        info.audio.extend(sound.audio.into_iter().map(|mut track| {
            track.track.index += SOUND_TRACKS;
            track
        }));
        info.seekable &= sound.seekable;
        info.duration = info.duration.max(sound.duration);
    }
    let Some(page) = page else {
        return;
    };
    info.title = page.title.clone().or(info.title.take());
    info.artist = page.artist.clone().or(info.artist.take());
    info.album = page.album.clone().or(info.album.take());
    if page.live {
        info.duration = None;
    } else if info.duration.is_none() {
        info.duration = page.duration;
    }
    if info.chapters.is_empty() {
        info.chapters = page.chapters.clone();
    }
}

pub(crate) fn describe(error: &ffmpeg::Error) -> String {
    match error {
        ffmpeg::Error::InvalidData => "this is not a file plaa can read".to_owned(),
        ffmpeg::Error::Other { errno } if *errno == libc_enoent() => {
            "the file does not exist".to_owned()
        }
        error => error.to_string(),
    }
}

/// ENOENT, the same on every system plaa runs on.
const fn libc_enoent() -> i32 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::info::{AudioTrack, Track};

    #[test]
    fn a_page_and_its_sound_fill_in_the_picture() {
        let mut info = MediaInfo {
            seekable: true,
            duration: Some(Duration::from_secs(60)),
            ..MediaInfo::default()
        };
        let sound = MediaInfo {
            audio: vec![AudioTrack {
                track: Track {
                    index: 0,
                    ..Track::default()
                },
                ..AudioTrack::default()
            }],
            seekable: true,
            duration: Some(Duration::from_secs(61)),
            ..MediaInfo::default()
        };
        let page = Page {
            title: Some("A film".to_owned()),
            artist: Some("Blender".to_owned()),
            ..Page::default()
        };
        merge(&mut info, Some(sound), Some(&page));
        assert_eq!(info.audio[0].track.index, SOUND_TRACKS);
        assert_eq!(info.duration, Some(Duration::from_secs(61)));
        assert_eq!(info.title.as_deref(), Some("A film"));
        assert_eq!(info.artist.as_deref(), Some("Blender"));
    }
}
