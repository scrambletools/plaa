//! Web pages, played through yt-dlp when it is installed: it reads a page,
//! such as a YouTube video's, for its streams and what it is called, and
//! plaa plays the streams.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use serde::Deserialize;

use crate::info::Chapter;
use crate::player::Quality;

/// What yt-dlp found on a page with something to play.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Page {
    pub title: Option<String>,
    /// Who made it: the artist, or the channel or uploader.
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<Duration>,
    pub chapters: Vec<Chapter>,
    /// The address of a picture for it.
    pub thumbnail: Option<String>,
    pub live: bool,
    /// What it can be played from, worst first, as yt-dlp ranks them.
    pub formats: Vec<Format>,
}

/// One stream of a page: a picture, sound, or both.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Format {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fps: Option<f64>,
    pub video: bool,
    pub audio: bool,
    /// The HTTP headers the site wants with it.
    pub headers: Vec<(String, String)>,
}

/// A page in a list, such as a playlist's video.
#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub url: String,
    pub title: Option<String>,
}

/// What an address turned out to be.
#[derive(Debug, Clone, PartialEq)]
pub enum Found {
    Page(Page),
    /// A list of pages, such as a playlist.
    List(Vec<Entry>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum WebError {
    /// yt-dlp is not installed.
    Missing,
    /// yt-dlp knows no way to read the page, which may be a stream
    /// FFmpeg reads itself.
    Unsupported,
    /// Stopped before yt-dlp was done.
    Stopped,
    /// yt-dlp could not read the page, saying why.
    Failed(String),
}

/// File endings of what FFmpeg plays straight from an address, rather
/// than through yt-dlp.
const STREAMS: &[&str] = &[
    "3gp", "aac", "ac3", "aif", "aiff", "avi", "f4v", "flac", "flv", "m2ts", "m3u", "m3u8", "m4a",
    "m4b", "m4v", "mka", "mkv", "mov", "mp2", "mp3", "mp4", "mpd", "mpeg", "mpg", "mts", "oga",
    "ogg", "ogv", "opus", "pls", "ts", "wav", "weba", "webm", "wma", "wmv",
];

/// How long yt-dlp may take to read a page.
const PATIENCE: Duration = Duration::from_secs(60);

/// Whether `url` is a web page to read with yt-dlp: an HTTP address that
/// does not end in a stream's file ending.
pub fn is_page(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    let Some(rest) = lower
        .strip_prefix("https://")
        .or_else(|| lower.strip_prefix("http://"))
    else {
        return false;
    };
    let path = rest.split(['?', '#']).next().unwrap_or(rest);
    let last = path.rsplit('/').next().unwrap_or(path);
    let ending = last.rsplit_once('.').map(|(_, ending)| ending);
    !ending.is_some_and(|ending| STREAMS.contains(&ending))
}

/// yt-dlp, where it is installed: on the path, or where package managers
/// put it, which apps started from a desktop may not have on their path.
pub fn program() -> Option<PathBuf> {
    let name = if cfg!(windows) {
        "yt-dlp.exe"
    } else {
        "yt-dlp"
    };
    let mut folders: Vec<PathBuf> = std::env::var_os("PATH")
        .map(|path| std::env::split_paths(&path).collect())
        .unwrap_or_default();
    let home =
        std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).map(PathBuf::from);
    if cfg!(windows) {
        if let Some(local) = std::env::var_os("LOCALAPPDATA").map(PathBuf::from) {
            folders.push(local.join("Microsoft").join("WinGet").join("Links"));
        }
        if let Some(home) = &home {
            folders.push(home.join("scoop").join("shims"));
        }
    } else {
        folders.extend(["/opt/homebrew/bin", "/usr/local/bin", "/usr/bin"].map(PathBuf::from));
        if let Some(home) = &home {
            folders.push(home.join(".local").join("bin"));
        }
    }
    folders
        .into_iter()
        .map(|folder| folder.join(name))
        .find(|path| path.is_file())
}

/// Reads the page at `url` with yt-dlp, giving up once `stop` is set.
pub fn read(url: &str, stop: &AtomicBool) -> Result<Found, WebError> {
    let program = program().ok_or(WebError::Missing)?;
    let output = run(&program, url, stop)?;
    if !output.success {
        return Err(error(&output.errors));
    }
    let info: Info = serde_json::from_slice(&output.json)
        .map_err(|error| WebError::Failed(format!("yt-dlp: {error}")))?;
    Ok(found(info))
}

struct Output {
    success: bool,
    json: Vec<u8>,
    errors: String,
}

fn run(program: &Path, url: &str, stop: &AtomicBool) -> Result<Output, WebError> {
    let mut command = Command::new(program);
    command
        .args([
            "-J",
            "--flat-playlist",
            "--no-playlist",
            "--no-warnings",
            "--",
        ])
        .arg(url)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // Without a console window flashing up.
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    let mut child = command
        .spawn()
        .map_err(|error| WebError::Failed(format!("yt-dlp: {error}")))?;
    // Read as it writes, so a page's long answer does not fill the pipe.
    let drain = |stream: Option<Box<dyn Read + Send>>| {
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            if let Some(mut stream) = stream {
                let _ = stream.read_to_end(&mut bytes);
            }
            bytes
        })
    };
    let json = drain(child.stdout.take().map(|out| Box::new(out) as _));
    let errors = drain(child.stderr.take().map(|out| Box::new(out) as _));
    let started = Instant::now();
    let status = loop {
        if stop.load(Ordering::Relaxed) || started.elapsed() > PATIENCE {
            let _ = child.kill();
            let _ = child.wait();
            return Err(if stop.load(Ordering::Relaxed) {
                WebError::Stopped
            } else {
                WebError::Failed("yt-dlp took too long".to_owned())
            });
        }
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            Err(error) => return Err(WebError::Failed(format!("yt-dlp: {error}"))),
        }
    };
    Ok(Output {
        success: status.success(),
        json: json.join().unwrap_or_default(),
        errors: String::from_utf8_lossy(&errors.join().unwrap_or_default()).into_owned(),
    })
}

/// What yt-dlp's complaint means.
fn error(errors: &str) -> WebError {
    let line = errors
        .lines()
        .rev()
        .find_map(|line| line.trim().strip_prefix("ERROR:"))
        .map(str::trim);
    match line {
        Some(line) if line.starts_with("Unsupported URL") => WebError::Unsupported,
        Some(line) => WebError::Failed(line.to_owned()),
        None => WebError::Failed("yt-dlp could not read the page".to_owned()),
    }
}

impl Page {
    /// The picture heights there are to choose from, tallest first.
    pub fn heights(&self) -> Vec<u32> {
        let mut heights: Vec<u32> = self
            .formats
            .iter()
            .filter(|format| format.video)
            .filter_map(|format| format.height)
            .collect();
        heights.sort_unstable_by(|first, second| second.cmp(first));
        heights.dedup();
        heights
    }

    /// Whether there is sound to play alone.
    pub fn has_audio(&self) -> bool {
        self.formats.iter().any(|format| format.audio)
    }

    /// What to play for `quality`: a picture, with the sound apart when the
    /// picture has none, or sound alone. yt-dlp ranks the formats, so the
    /// last that fits is the best.
    pub fn choose(&self, quality: Quality) -> Option<(&Format, Option<&Format>)> {
        let sound_alone = self
            .formats
            .iter()
            .rev()
            .find(|format| format.audio && !format.video);
        if quality == Quality::AudioOnly {
            let sound =
                sound_alone.or_else(|| self.formats.iter().rev().find(|format| format.audio))?;
            return Some((sound, None));
        }
        let pictures = || self.formats.iter().rev().filter(|format| format.video);
        let fits = |format: &&Format| match quality {
            Quality::AtMost(lines) => format.height.is_none_or(|height| height <= lines),
            _ => true,
        };
        // Too small a limit: the smallest there is.
        let smallest = || {
            pictures()
                .filter(|format| format.height.is_some())
                .min_by_key(|format| format.height)
        };
        let picture = pictures().find(fits).or_else(smallest)?;
        if picture.audio {
            return Some((picture, None));
        }
        Some((picture, sound_alone))
    }
}

/// yt-dlp's answer, as far as plaa reads it.
#[derive(Deserialize, Default)]
#[serde(default)]
struct Info {
    #[serde(rename = "_type")]
    kind: Option<String>,
    title: Option<String>,
    track: Option<String>,
    artist: Option<String>,
    channel: Option<String>,
    uploader: Option<String>,
    album: Option<String>,
    duration: Option<f64>,
    is_live: Option<bool>,
    thumbnail: Option<String>,
    chapters: Option<Vec<RawChapter>>,
    formats: Option<Vec<RawFormat>>,
    entries: Option<Vec<RawEntry>>,
    /// A page with one format has it here rather than in `formats`.
    #[serde(flatten)]
    only: RawFormat,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawChapter {
    start_time: Option<f64>,
    end_time: Option<f64>,
    title: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawFormat {
    url: Option<String>,
    protocol: Option<String>,
    vcodec: Option<String>,
    acodec: Option<String>,
    width: Option<f64>,
    height: Option<f64>,
    fps: Option<f64>,
    http_headers: Option<std::collections::BTreeMap<String, String>>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawEntry {
    url: Option<String>,
    webpage_url: Option<String>,
    title: Option<String>,
}

fn found(info: Info) -> Found {
    if info.kind.as_deref() == Some("playlist") {
        let entries = info
            .entries
            .unwrap_or_default()
            .into_iter()
            .filter_map(|entry| {
                let url = [entry.url, entry.webpage_url]
                    .into_iter()
                    .flatten()
                    .find(|url| url.contains("://"))?;
                Some(Entry {
                    url,
                    title: entry.title,
                })
            })
            .collect();
        return Found::List(entries);
    }
    let mut raw = info.formats.unwrap_or_default();
    if raw.is_empty() {
        raw.push(info.only);
    }
    let formats = raw.into_iter().filter_map(format).collect();
    let seconds = |value: Option<f64>| {
        value
            .filter(|value| value.is_finite() && *value >= 0.0)
            .map(Duration::from_secs_f64)
    };
    let chapters = info
        .chapters
        .unwrap_or_default()
        .into_iter()
        .filter_map(|chapter| {
            Some(Chapter {
                title: chapter.title.filter(|title| !title.trim().is_empty()),
                start: seconds(chapter.start_time)?,
                end: seconds(chapter.end_time)?,
            })
        })
        .collect();
    Found::Page(Page {
        title: info.track.or(info.title),
        artist: info.artist.or(info.channel).or(info.uploader),
        album: info.album,
        duration: seconds(info.duration),
        chapters,
        thumbnail: info.thumbnail,
        live: info.is_live.unwrap_or(false),
        formats,
    })
}

/// A format FFmpeg can play straight from its address: not one in
/// fragments for yt-dlp to put together, nor a storyboard of stills.
fn format(raw: RawFormat) -> Option<Format> {
    let url = raw.url?;
    let protocol = raw.protocol.unwrap_or_else(|| "https".to_owned());
    let playable = ["http", "https", "m3u8", "m3u8_native", "rtmp", "rtsp"]
        .iter()
        .any(|known| protocol == *known);
    if !playable {
        return None;
    }
    // Unknown codecs may be there; "none" says they are not.
    let present = |codec: &Option<String>| codec.as_deref() != Some("none");
    let (video, audio) = (present(&raw.vcodec), present(&raw.acodec));
    if !video && !audio {
        return None;
    }
    let lines = |value: Option<f64>| {
        value
            .filter(|value| value.is_finite() && *value > 0.0)
            .map(|value| value as u32)
    };
    Some(Format {
        url,
        width: lines(raw.width),
        height: lines(raw.height),
        fps: raw.fps.filter(|fps| fps.is_finite() && *fps > 0.0),
        video,
        audio,
        headers: raw.http_headers.unwrap_or_default().into_iter().collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pages_are_told_from_streams() {
        assert!(is_page("https://www.youtube.com/watch?v=aqz-KE-bpKQ"));
        assert!(is_page("https://vimeo.com/76979871"));
        assert!(!is_page("https://example.com/live/stream.m3u8?token=1"));
        assert!(!is_page("http://example.com/movie.MP4"));
        assert!(!is_page("rtsp://camera.local/stream"));
        assert!(!is_page("/home/me/movie.mkv"));
    }

    #[test]
    fn errors_say_whether_the_page_is_unknown() {
        assert_eq!(
            error(
                "WARNING: [generic] Falling back\nERROR: Unsupported URL: https://example.com/\n"
            ),
            WebError::Unsupported
        );
        assert_eq!(
            error("ERROR: [youtube] abc: Video unavailable\n"),
            WebError::Failed("[youtube] abc: Video unavailable".to_owned())
        );
    }

    fn page() -> Page {
        let json = r#"{
            "_type": "video", "title": "A film", "channel": "Blender",
            "duration": 635, "is_live": false,
            "chapters": [{"start_time": 0, "end_time": 60, "title": "Opening"}],
            "formats": [
                {"url": "https://i/sb", "protocol": "mhtml", "vcodec": "none", "acodec": "none"},
                {"url": "https://a/139", "protocol": "https", "vcodec": "none", "acodec": "mp4a.40.5"},
                {"url": "https://a/251", "protocol": "https", "vcodec": "none", "acodec": "opus",
                 "http_headers": {"User-Agent": "Mozilla/5.0"}},
                {"url": "https://v/18", "protocol": "https", "vcodec": "avc1", "acodec": "mp4a", "height": 360},
                {"url": "https://v/136", "protocol": "https", "vcodec": "avc1", "acodec": "none", "height": 720},
                {"url": "https://v/dash", "protocol": "http_dash_segments", "vcodec": "vp9", "acodec": "none", "height": 1080},
                {"url": "https://v/401", "protocol": "https", "vcodec": "av01", "acodec": "none", "height": 2160, "fps": 60}
            ]
        }"#;
        match found(serde_json::from_str(json).expect("yt-dlp's answer")) {
            Found::Page(page) => page,
            Found::List(_) => panic!("a list"),
        }
    }

    #[test]
    fn a_page_reads_as_its_formats_and_details() {
        let page = page();
        assert_eq!(page.title.as_deref(), Some("A film"));
        assert_eq!(page.artist.as_deref(), Some("Blender"));
        assert_eq!(page.duration, Some(Duration::from_secs(635)));
        assert_eq!(page.chapters.len(), 1);
        // The storyboard and the fragments are left out.
        assert_eq!(page.formats.len(), 5);
        assert_eq!(page.heights(), vec![2160, 720, 360]);
        assert_eq!(page.formats[1].headers.len(), 1);
    }

    #[test]
    fn quality_picks_a_picture_and_its_sound() {
        let page = page();
        let urls = |choice: Option<(&Format, Option<&Format>)>| {
            choice
                .map(|(first, second)| (first.url.clone(), second.map(|second| second.url.clone())))
        };
        assert_eq!(
            urls(page.choose(Quality::Best)),
            Some(("https://v/401".into(), Some("https://a/251".into())))
        );
        assert_eq!(
            urls(page.choose(Quality::AtMost(1080))),
            Some(("https://v/136".into(), Some("https://a/251".into())))
        );
        // A picture with its own sound needs none apart.
        assert_eq!(
            urls(page.choose(Quality::AtMost(480))),
            Some(("https://v/18".into(), None))
        );
        assert_eq!(
            urls(page.choose(Quality::AtMost(144))),
            Some(("https://v/18".into(), None))
        );
        assert_eq!(
            urls(page.choose(Quality::AudioOnly)),
            Some(("https://a/251".into(), None))
        );
    }

    #[test]
    fn a_playlist_reads_as_its_pages() {
        let json = r#"{"_type": "playlist", "title": "Lectures", "entries": [
            {"_type": "url", "url": "https://www.youtube.com/watch?v=one", "title": "One"},
            {"_type": "url", "url": "two", "title": "No address"},
            {"_type": "url", "url": "https://www.youtube.com/watch?v=three"}
        ]}"#;
        let Found::List(entries) = found(serde_json::from_str(json).expect("a list")) else {
            panic!("not a list");
        };
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].title.as_deref(), Some("One"));
        assert_eq!(entries[1].title, None);
    }
}
