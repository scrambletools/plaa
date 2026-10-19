//! Text subtitles: cues decoded ahead of the clock, and the one showing.

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
}

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
