//! Text subtitles: cues decoded ahead of the clock, and the one showing;
//! and the subtitle files that go with a video.

use std::path::{Path, PathBuf};

/// A line of subtitle text and when it shows, in media seconds.
#[derive(Debug, Clone, PartialEq)]
pub struct Cue {
    pub start: f64,
    pub end: f64,
    /// The text, with lines separated by `\n`.
    pub text: String,
}

/// Cues decoded but not yet over, in start order.
#[derive(Debug, Default)]
pub struct Cues {
    cues: Vec<Cue>,
}

/// How long a cue without an end shows, at most.
const OPEN_ENDED: f64 = 5.0;

impl Cues {
    pub fn add(&mut self, mut cue: Cue) {
        if cue.text.trim().is_empty() {
            return;
        }
        if cue.end.partial_cmp(&cue.start) != Some(std::cmp::Ordering::Greater) {
            cue.end = cue.start + OPEN_ENDED;
        }
        let at = self
            .cues
            .partition_point(|existing| existing.start <= cue.start);
        // A cue without an end gives way to the next one.
        if at > 0 {
            let before = &mut self.cues[at - 1];
            if before.end - before.start == OPEN_ENDED && before.end > cue.start {
                before.end = cue.start;
            }
        }
        self.cues.insert(at, cue);
    }

    /// The cue showing at `now`, the latest to start when several do;
    /// forgets the ones already over.
    pub fn at(&mut self, now: f64) -> Option<&Cue> {
        self.cues.retain(|cue| cue.end > now - 1.0);
        self.cues
            .iter()
            .rev()
            .find(|cue| cue.start <= now && now < cue.end)
    }

    pub fn clear(&mut self) {
        self.cues.clear();
    }

    /// Every cue, in start order.
    pub fn into_vec(self) -> Vec<Cue> {
        self.cues
    }
}

/// The cue showing at `now` among `cues`, in start order: the latest to
/// start when several do. Unlike `Cues::at`, it forgets nothing, for a
/// whole file's cues read at once.
pub fn showing(cues: &[Cue], now: f64) -> Option<&Cue> {
    let started = cues.partition_point(|cue| cue.start <= now);
    cues[..started].iter().rev().find(|cue| now < cue.end)
}

/// The extensions of subtitle files read beside a video.
const SUBTITLE_FILES: &[&str] = &["ass", "sami", "smi", "srt", "ssa", "sub", "vtt"];

/// Folders beside a video that releases keep subtitles in.
const SUBTITLE_FOLDERS: &[&str] = &["subs", "subtitles"];

/// The subtitle files that go with the video at `video`: named as it is,
/// with perhaps a language and other tags before the extension, beside it
/// or in a `Subs` or `Subtitles` folder there. In name order.
pub fn beside(video: &Path) -> Vec<PathBuf> {
    let (Some(folder), Some(stem)) = (video.parent(), video.file_stem()) else {
        return Vec::new();
    };
    let stem = stem.to_string_lossy();
    let entries = |folder: &Path| -> Vec<PathBuf> {
        std::fs::read_dir(folder)
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .map(|entry| entry.path())
                    .collect()
            })
            .unwrap_or_default()
    };
    let beside = entries(folder);
    let folders = beside.iter().filter(|path| {
        path.is_dir()
            && path.file_name().is_some_and(|name| {
                SUBTITLE_FOLDERS.contains(&name.to_string_lossy().to_lowercase().as_str())
            })
    });
    let mut files: Vec<PathBuf> = folders
        .flat_map(|folder| entries(folder))
        .chain(beside.iter().cloned())
        .filter(|path| {
            let subtitle = path.extension().is_some_and(|extension| {
                SUBTITLE_FILES.contains(&extension.to_string_lossy().to_lowercase().as_str())
            });
            subtitle && path.is_file() && name_tags(&stem, path).is_some()
        })
        .collect();
    files.sort();
    files
}

/// The tags in a subtitle file's name after the video's: in
/// `Film.en.forced.srt` for `Film`, `["en", "forced"]`. None when the file
/// is named for another video.
fn name_tags(stem: &str, file: &Path) -> Option<Vec<String>> {
    let name = file.file_stem()?.to_string_lossy().into_owned();
    if name == stem {
        return Some(Vec::new());
    }
    let rest = name.strip_prefix(stem)?.strip_prefix('.')?;
    Some(rest.split('.').map(str::to_owned).collect())
}

/// What a subtitle file's name says after the video's: its language, as
/// the code or English name that `is_language` knows, and whether it holds
/// only forced lines.
pub fn file_language(
    stem: &str,
    file: &Path,
    is_language: impl Fn(&str) -> bool,
) -> (Option<String>, bool) {
    let tags = name_tags(stem, file).unwrap_or_default();
    let forced = tags.iter().any(|tag| tag.eq_ignore_ascii_case("forced"));
    let language = tags.iter().find_map(|tag| {
        let lower = tag.to_lowercase();
        LANGUAGE_NAMES
            .iter()
            .find(|(name, _)| *name == lower)
            .map(|(_, code)| (*code).to_owned())
            .or_else(|| is_language(&lower).then_some(lower))
    });
    (language, forced)
}

/// Languages that subtitle files are named for in English.
const LANGUAGE_NAMES: &[(&str, &str)] = &[
    ("arabic", "ar"),
    ("chinese", "zh"),
    ("czech", "cs"),
    ("danish", "da"),
    ("dutch", "nl"),
    ("english", "en"),
    ("finnish", "fi"),
    ("french", "fr"),
    ("german", "de"),
    ("greek", "el"),
    ("hebrew", "he"),
    ("hindi", "hi"),
    ("hungarian", "hu"),
    ("indonesian", "id"),
    ("italian", "it"),
    ("japanese", "ja"),
    ("korean", "ko"),
    ("norwegian", "no"),
    ("polish", "pl"),
    ("portuguese", "pt"),
    ("romanian", "ro"),
    ("russian", "ru"),
    ("spanish", "es"),
    ("swedish", "sv"),
    ("thai", "th"),
    ("turkish", "tr"),
    ("ukrainian", "uk"),
    ("vietnamese", "vi"),
];

/// The text of an ASS dialogue line as FFmpeg gives it: the fields up to
/// the text dropped, override blocks such as `{\i1}` removed, and ASS's
/// line breaks made real ones.
pub fn ass_text(line: &str) -> String {
    // ReadOrder, Layer, Style, Name, MarginL, MarginR, MarginV, Effect,
    // then the text, which may itself hold commas.
    let text = line.splitn(9, ',').nth(8).unwrap_or(line);
    let mut out = String::with_capacity(text.len());
    let mut in_override = false;
    let mut characters = text.chars().peekable();
    while let Some(character) = characters.next() {
        match character {
            '{' => in_override = true,
            '}' if in_override => in_override = false,
            _ if in_override => {}
            '\\' => match characters.peek() {
                Some('N') | Some('n') => {
                    characters.next();
                    out.push('\n');
                }
                Some('h') => {
                    characters.next();
                    out.push('\u{a0}');
                }
                _ => out.push('\\'),
            },
            character => out.push(character),
        }
    }
    out.trim().to_owned()
}

/// Plain text from a subtitle format with simple markup, such as SubRip's
/// `<i>` tags.
pub fn plain_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_tag = false;
    for character in text.chars() {
        match character {
            '<' => in_tag = true,
            '>' if in_tag => in_tag = false,
            _ if in_tag => {}
            character => out.push(character),
        }
    }
    out.replace("\r\n", "\n").trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ass_lines_lose_their_fields_and_overrides() {
        assert_eq!(
            ass_text(r"0,0,Default,,0,0,0,,{\i1}Hello,{\i0} there\Nfriend"),
            "Hello, there\nfriend"
        );
        assert_eq!(ass_text(r"3,0,Sign,,0,0,0,,A\hB"), "A\u{a0}B");
    }

    #[test]
    fn markup_is_dropped_from_plain_text() {
        assert_eq!(plain_text("<i>Hi</i> <b>you</b>\r\nthere"), "Hi you\nthere");
    }

    #[test]
    fn files_named_for_the_video_are_its_subtitles() {
        let folder = tempfile::tempdir().expect("a folder");
        let subs = folder.path().join("Subs");
        std::fs::create_dir(&subs).expect("a folder");
        for name in [
            "Film.mkv",
            "Film.srt",
            "Film.en.forced.srt",
            "Film.English.ass",
            "Filmography.srt",
            "Other.en.srt",
            "Film.nfo",
        ] {
            std::fs::write(folder.path().join(name), b"").expect("a file");
        }
        std::fs::write(subs.join("Film.pt-BR.srt"), b"").expect("a file");
        let video = folder.path().join("Film.mkv");
        let found: Vec<String> = beside(&video)
            .iter()
            .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            found,
            [
                "Film.English.ass",
                "Film.en.forced.srt",
                "Film.srt",
                "Film.pt-BR.srt"
            ]
        );
        let known = |code: &str| ["en", "pt-br"].contains(&code);
        let language = |name: &str| file_language("Film", &folder.path().join(name), known);
        assert_eq!(
            language("Film.en.forced.srt"),
            (Some("en".to_owned()), true)
        );
        assert_eq!(language("Film.English.ass"), (Some("en".to_owned()), false));
        assert_eq!(language("Film.srt"), (None, false));
        assert_eq!(
            language("Film.pt-BR.srt"),
            (Some("pt-br".to_owned()), false)
        );
    }

    #[test]
    fn a_whole_files_cues_are_looked_up_without_forgetting() {
        let cue = |start: f64, end: f64, text: &str| Cue {
            start,
            end,
            text: text.to_owned(),
        };
        let cues = [
            cue(1.0, 3.0, "one"),
            cue(2.0, 4.0, "two"),
            cue(5.0, 6.0, "three"),
        ];
        assert_eq!(
            showing(&cues, 2.5).map(|cue| cue.text.as_str()),
            Some("two")
        );
        assert_eq!(
            showing(&cues, 3.5).map(|cue| cue.text.as_str()),
            Some("two")
        );
        assert_eq!(showing(&cues, 4.5), None);
        assert_eq!(
            showing(&cues, 1.5).map(|cue| cue.text.as_str()),
            Some("one")
        );
    }

    #[test]
    fn the_latest_cue_showing_wins_and_open_ends_give_way() {
        let mut cues = Cues::default();
        cues.add(Cue {
            start: 1.0,
            end: 0.0,
            text: "first".into(),
        });
        cues.add(Cue {
            start: 2.0,
            end: 4.0,
            text: "second".into(),
        });
        assert_eq!(cues.at(1.5).map(|cue| cue.text.as_str()), Some("first"));
        assert_eq!(cues.at(2.5).map(|cue| cue.text.as_str()), Some("second"));
        assert_eq!(cues.at(4.5), None);
    }
}
