//! A window's list of files and streams: those opened or dropped together,
//! the entries of a list file, or the other files in the opened file's
//! folder.

use std::cmp::Ordering;
use std::path::{Path, PathBuf};

use plaa_media::Source;

/// What a folder's files are listed by: the extensions of what FFmpeg
/// plays that people keep in folders.
const PLAYABLE: &[&str] = &[
    // Video.
    "3g2", "3gp", "asf", "avi", "divx", "f4v", "flv", "h264", "h265", "hevc", "m2t", "m2ts", "m2v",
    "m4v", "mkv", "mov", "mp4", "mpeg", "mpg", "mts", "mxf", "nut", "ogm", "ogv", "qt", "rm",
    "rmvb", "ts", "vob", "webm", "wmv", "y4m", // Sound.
    "aac", "ac3", "aif", "aifc", "aiff", "alac", "amr", "ape", "au", "caf", "dff", "dsf", "dts",
    "eac3", "flac", "m4a", "m4b", "mka", "mp2", "mp3", "mpc", "oga", "ogg", "opus", "spx", "tta",
    "w64", "wav", "weba", "wma", "wv",
];

/// List files, whose entries play in turn.
const LISTS: &[&str] = &["m3u", "m3u8", "pls"];

#[derive(Debug, Clone, PartialEq)]
pub struct Playlist {
    items: Vec<Source>,
    current: usize,
}

impl Playlist {
    /// A list of `items` at `current`, or none when it is empty.
    pub fn new(items: Vec<Source>, current: usize) -> Option<Self> {
        (!items.is_empty()).then(|| Self {
            current: current.min(items.len() - 1),
            items,
        })
    }

    pub fn items(&self) -> &[Source] {
        &self.items
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn source(&self) -> &Source {
        &self.items[self.current]
    }

    /// Whether there is more than one item to move between.
    pub fn is_list(&self) -> bool {
        self.items.len() > 1
    }

    pub fn next(&self) -> Option<usize> {
        (self.current + 1 < self.items.len()).then_some(self.current + 1)
    }

    pub fn previous(&self) -> Option<usize> {
        self.current.checked_sub(1)
    }

    /// Moves to item `index`, if there is one.
    pub fn select(&mut self, index: usize) -> bool {
        let exists = index < self.items.len();
        if exists {
            self.current = index;
        }
        exists
    }
}

/// `sources` as a list: each list file's entries in its place.
pub fn expand(sources: Vec<Source>) -> Vec<Source> {
    sources
        .into_iter()
        .flat_map(|source| match &source {
            Source::File(path) if is_list_file(path) => match read_list(path) {
                Some(entries) => entries,
                None => vec![source],
            },
            _ => vec![source],
        })
        .collect()
}

fn extension(path: &Path) -> Option<String> {
    path.extension()
        .map(|extension| extension.to_string_lossy().to_lowercase())
}

fn is_list_file(path: &Path) -> bool {
    extension(path).is_some_and(|extension| LISTS.contains(&extension.as_str()))
}

/// A list file's entries: files, relative to it or not, and addresses.
/// None for an HLS stream's `.m3u8`, which plays as a stream, or a file
/// that cannot be read.
fn read_list(path: &Path) -> Option<Vec<Source>> {
    let bytes = std::fs::read(path).ok()?;
    let text = String::from_utf8_lossy(&bytes);
    let base = path.parent().unwrap_or(Path::new(""));
    parse_list(&text, extension(path).as_deref() == Some("pls"), base)
}

fn parse_list(text: &str, pls: bool, base: &Path) -> Option<Vec<Source>> {
    if text.contains("#EXT-X-") {
        return None;
    }
    let entries = text.lines().filter_map(|line| {
        let line = line.trim().trim_start_matches('\u{feff}');
        if pls {
            let (key, value) = line.split_once('=')?;
            let number = key.trim().strip_prefix("File")?;
            number.parse::<u32>().ok()?;
            Some(value.trim())
        } else {
            (!line.is_empty() && !line.starts_with('#')).then_some(line)
        }
    });
    Some(entries.map(|entry| entry_source(entry, base)).collect())
}

fn entry_source(entry: &str, base: &Path) -> Source {
    if let Some(uri) = entry.strip_prefix("file://") {
        let path = uri.strip_prefix("localhost").unwrap_or(uri);
        return Source::File(PathBuf::from(percent_decoded(path)));
    }
    match Source::parse(entry) {
        Source::File(path) if path.is_relative() => Source::File(base.join(path)),
        source => source,
    }
}

/// `%20` and the like back into their characters.
fn percent_decoded(text: &str) -> String {
    let mut bytes = Vec::with_capacity(text.len());
    let mut input = text.bytes();
    while let Some(byte) = input.next() {
        if byte == b'%' {
            let digits = [input.next(), input.next()];
            let value = digits
                .iter()
                .map(|digit| digit.and_then(|digit| (digit as char).to_digit(16)))
                .try_fold(0, |value, digit| Some(value * 16 + digit?));
            match value {
                Some(value) => bytes.push(value as u8),
                None => bytes.push(byte),
            }
        } else {
            bytes.push(byte);
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

/// The files plaa plays in `path`'s folder, in the order file managers
/// show them, and where `path` is among them.
pub fn folder(path: &Path) -> Option<(Vec<PathBuf>, usize)> {
    let directory = path.parent()?;
    let mut files: Vec<PathBuf> = std::fs::read_dir(directory)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|file| file == path || is_playable(file))
        .filter(|file| file.is_file())
        .collect();
    files.sort_by(|left, right| natural(&file_name(left), &file_name(right)));
    let at = files.iter().position(|file| file == path)?;
    Some((files, at))
}

fn is_playable(path: &Path) -> bool {
    let hidden = path
        .file_name()
        .is_some_and(|name| name.to_string_lossy().starts_with('.'));
    !hidden && extension(path).is_some_and(|extension| PLAYABLE.contains(&extension.as_str()))
}

fn file_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Compares names as people read them: runs of digits as numbers, so
/// "2" comes before "10", and letters regardless of case.
pub fn natural(left: &str, right: &str) -> Ordering {
    let mut left = left.chars().peekable();
    let mut right = right.chars().peekable();
    loop {
        match (left.peek().copied(), right.peek().copied()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(a), Some(b)) if a.is_ascii_digit() && b.is_ascii_digit() => {
                let take = |chars: &mut std::iter::Peekable<std::str::Chars>| {
                    let mut digits = String::new();
                    while let Some(digit) = chars.next_if(char::is_ascii_digit) {
                        digits.push(digit);
                    }
                    digits
                };
                let (a, b) = (take(&mut left), take(&mut right));
                let (a_trimmed, b_trimmed) = (a.trim_start_matches('0'), b.trim_start_matches('0'));
                let order = a_trimmed
                    .len()
                    .cmp(&b_trimmed.len())
                    .then_with(|| a_trimmed.cmp(b_trimmed))
                    .then_with(|| a.len().cmp(&b.len()));
                if order != Ordering::Equal {
                    return order;
                }
            }
            (Some(a), Some(b)) => {
                let order = a.to_lowercase().cmp(b.to_lowercase());
                if order != Ordering::Equal {
                    return order;
                }
                left.next();
                right.next();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_sort_as_people_read_them() {
        let mut names = vec![
            "Episode 10.mkv",
            "episode 2.mkv",
            "Episode 1.mkv",
            "Extra.mkv",
        ];
        names.sort_by(|left, right| natural(left, right));
        assert_eq!(
            names,
            [
                "Episode 1.mkv",
                "episode 2.mkv",
                "Episode 10.mkv",
                "Extra.mkv"
            ]
        );
        assert_eq!(natural("a02", "a2"), Ordering::Greater);
        assert_eq!(natural("a9", "a10"), Ordering::Less);
    }

    #[test]
    fn m3u_entries_are_files_beside_it_or_addresses() {
        let base = Path::new("/music");
        let text = "\u{feff}#EXTM3U\n#EXTINF:123,Song\nalbum/one.mp3\n\n/abs/two.flac\n\
                    https://example.com/live.m3u8\nfile:///my%20music/three.ogg\n";
        assert_eq!(
            parse_list(text, false, base),
            Some(vec![
                Source::File(PathBuf::from("/music/album/one.mp3")),
                Source::File(PathBuf::from("/abs/two.flac")),
                Source::Url("https://example.com/live.m3u8".to_owned()),
                Source::File(PathBuf::from("/my music/three.ogg")),
            ])
        );
    }

    #[test]
    fn a_streams_m3u8_plays_as_a_stream() {
        let text = "#EXTM3U\n#EXT-X-VERSION:3\n#EXT-X-TARGETDURATION:6\nsegment0.ts\n";
        assert_eq!(parse_list(text, false, Path::new("/")), None);
    }

    #[test]
    fn pls_entries_are_read_by_number() {
        let text = "[playlist]\nFile1=one.mp3\nTitle1=One\nFile2=http://radio.example/stream\n\
                    NumberOfEntries=2\nVersion=2\n";
        assert_eq!(
            parse_list(text, true, Path::new("/lists")),
            Some(vec![
                Source::File(PathBuf::from("/lists/one.mp3")),
                Source::Url("http://radio.example/stream".to_owned()),
            ])
        );
    }

    #[test]
    fn a_folder_lists_what_plays_in_order() {
        let directory = tempfile::tempdir().expect("a folder");
        for name in [
            "b 10.mp4",
            "b 2.mkv",
            "notes.txt",
            ".hidden.mp4",
            "a.MP3",
            "cover.jpg",
        ] {
            std::fs::write(directory.path().join(name), b"").expect("a file");
        }
        let opened = directory.path().join("b 2.mkv");
        let (files, at) = folder(&opened).expect("a list");
        let names: Vec<String> = files.iter().map(|file| file_name(file)).collect();
        assert_eq!(names, ["a.MP3", "b 2.mkv", "b 10.mp4"]);
        assert_eq!(at, 1);
    }

    #[test]
    fn moves_through_the_list() {
        let mut list = Playlist::new(
            vec![
                Source::parse("/a.mp4"),
                Source::parse("/b.mp4"),
                Source::parse("/c.mp4"),
            ],
            9,
        )
        .expect("a list");
        assert_eq!(list.current(), 2);
        assert_eq!(list.next(), None);
        assert_eq!(list.previous(), Some(1));
        assert!(list.select(0));
        assert_eq!(list.source(), &Source::parse("/a.mp4"));
        assert!(!list.select(3));
        assert_eq!(Playlist::new(Vec::new(), 0), None);
    }
}
