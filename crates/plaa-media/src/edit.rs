//! An edit of a file: the parts of it kept, in the order they play, and
//! how its picture is turned and flipped. Clips are in seconds of the
//! file, as plaa counts them from its start; the edited movie's own time
//! runs through its clips one after another.

/// The shortest clip trimming or splitting leaves.
pub const SHORTEST: f64 = 0.1;

/// Parts left shorter than this, about a frame, are dropped rather than
/// kept as a clip.
const SLIVER: f64 = 0.02;

/// A part of the file, from and to seconds of it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Clip {
    pub from: f64,
    pub to: f64,
}

impl Clip {
    pub fn length(&self) -> f64 {
        (self.to - self.from).max(0.0)
    }
}

/// How a picture is turned and flipped as it shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Transform {
    /// Clockwise quarter turns, after mirroring.
    pub turns: u8,
    /// Mirrored left to right, before turning.
    pub mirrored: bool,
}

impl Transform {
    pub fn is_identity(self) -> bool {
        self.turns.is_multiple_of(4) && !self.mirrored
    }

    /// Turns the picture a quarter turn as it shows.
    pub fn rotate(&mut self, clockwise: bool) {
        self.turns = (self.turns + if clockwise { 1 } else { 3 }) % 4;
    }

    /// Flips the picture as it shows: left to right when `horizontal`,
    /// else top to bottom.
    pub fn flip(&mut self, horizontal: bool) {
        // Mirroring after a turn is the opposite turn after mirroring; a
        // vertical flip is a horizontal one turned upside down.
        let base = if horizontal { 4 } else { 6 };
        self.turns = (base - self.turns % 4) % 4;
        self.mirrored = !self.mirrored;
    }

    /// This after the `own` quarter turns a file asks for itself, as
    /// phones record portrait video.
    pub fn after_turns(self, own: u8) -> Transform {
        let own = if self.mirrored {
            (4 - own % 4) % 4
        } else {
            own % 4
        };
        Transform {
            turns: (self.turns + own) % 4,
            mirrored: self.mirrored,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edit {
    clips: Vec<Clip>,
    transform: Transform,
}

/// Part of the file read in one go: clips one after another in the file
/// play as one. Its packets carry it, so decoders know which part they
/// are in and where its pictures and sound go in the edited time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Span {
    pub index: usize,
    pub from: f64,
    pub to: f64,
    /// Added to the file's time to give the edited time.
    pub shift: f64,
}

impl Span {
    /// The whole file, played as it is.
    pub const WHOLE: Span = Span {
        index: 0,
        from: f64::NEG_INFINITY,
        to: f64::INFINITY,
        shift: 0.0,
    };

    /// Whether the edited time `seconds` is in the span, or right at its
    /// end.
    pub fn holds(&self, seconds: f64) -> bool {
        let source = seconds - self.shift;
        source >= self.from - 1e-9 && source <= self.to + 1e-9
    }
}

impl Edit {
    /// The whole file, `duration` seconds long, as it is.
    pub fn whole(duration: f64) -> Self {
        Self {
            clips: vec![Clip {
                from: 0.0,
                to: duration.max(0.0),
            }],
            transform: Transform::default(),
        }
    }

    /// `clips` of a file, played in that order, as they are.
    pub fn new(clips: Vec<Clip>) -> Self {
        Self {
            clips,
            transform: Transform::default(),
        }
    }

    pub fn clips(&self) -> &[Clip] {
        &self.clips
    }

    /// How the picture is turned and flipped.
    pub fn transform(&self) -> Transform {
        self.transform
    }

    /// Whether this is the whole of a file `duration` seconds long, as it
    /// is.
    pub fn is_whole(&self, duration: f64) -> bool {
        self.transform.is_identity()
            && matches!(self.clips.as_slice(), [only]
                if only.from <= 1e-6 && (only.to - duration).abs() <= 1e-6)
    }

    /// Whether the clips, one after another, play the whole file `duration`
    /// seconds long through, as after splitting alone.
    pub fn plays_whole(&self, duration: f64) -> bool {
        let mut at = 0.0;
        for clip in &self.clips {
            if (clip.from - at).abs() > 1e-6 {
                return false;
            }
            at = clip.to;
        }
        (at - duration).abs() <= 1e-6
    }

    /// How long the edited movie lasts.
    pub fn duration(&self) -> f64 {
        self.clips.iter().map(Clip::length).sum()
    }

    /// Where each clip starts in the edited time.
    pub fn starts(&self) -> Vec<f64> {
        let mut at = 0.0;
        self.clips
            .iter()
            .map(|clip| {
                let start = at;
                at += clip.length();
                start
            })
            .collect()
    }

    /// The clip playing at edited time `seconds`, and where it starts:
    /// the last one at the very end.
    pub fn clip_at(&self, seconds: f64) -> Option<(usize, f64)> {
        let mut start = 0.0;
        for (index, clip) in self.clips.iter().enumerate() {
            let end = start + clip.length();
            if seconds < end || index + 1 == self.clips.len() {
                return Some((index, start));
            }
            start = end;
        }
        None
    }

    /// The file's time at edited time `seconds`.
    pub fn source_of(&self, seconds: f64) -> f64 {
        match self.clip_at(seconds) {
            Some((index, start)) => {
                let clip = self.clips[index];
                (clip.from + (seconds - start).max(0.0)).min(clip.to)
            }
            None => seconds,
        }
    }

    /// The edited time the file's time `seconds` plays at, if it is kept.
    pub fn edited_of(&self, seconds: f64) -> Option<f64> {
        let mut start = 0.0;
        for clip in &self.clips {
            if seconds >= clip.from - 1e-9 && seconds < clip.to {
                return Some(start + (seconds - clip.from).max(0.0));
            }
            start += clip.length();
        }
        None
    }

    /// Keeps the edited movie from `from` to `to` seconds. Returns whether
    /// that changed it.
    pub fn trim(&mut self, from: f64, to: f64) -> bool {
        let (from, to) = (from.max(0.0), to.min(self.duration()));
        if to - from < SHORTEST - 1e-9 {
            return false;
        }
        let mut kept = Vec::new();
        let mut start = 0.0;
        for clip in &self.clips {
            let end = start + clip.length();
            let (low, high) = (from.max(start), to.min(end));
            if high - low >= SLIVER {
                kept.push(Clip {
                    from: clip.from + (low - start),
                    to: clip.from + (high - start),
                });
            }
            start = end;
        }
        self.replace(kept)
    }

    /// Keeps clip `index` from `from` to `to` seconds of the edited time,
    /// within it. Returns whether that changed it.
    pub fn trim_clip(&mut self, index: usize, from: f64, to: f64) -> bool {
        let Some(start) = self.starts().get(index).copied() else {
            return false;
        };
        let clip = self.clips[index];
        let low = (from - start).clamp(0.0, clip.length());
        let high = (to - start).clamp(0.0, clip.length());
        if high - low < SHORTEST - 1e-9 {
            return false;
        }
        let mut clips = self.clips.clone();
        clips[index] = Clip {
            from: clip.from + low,
            to: clip.from + high,
        };
        self.replace(clips)
    }

    /// Splits the clip playing at edited time `seconds` in two there,
    /// unless that is too close to its ends. Returns whether it did.
    pub fn split(&mut self, seconds: f64) -> bool {
        let Some((index, start)) = self.clip_at(seconds) else {
            return false;
        };
        let clip = self.clips[index];
        let into = seconds - start;
        if into < SHORTEST || clip.length() - into < SHORTEST {
            return false;
        }
        let middle = clip.from + into;
        self.clips[index].to = middle;
        self.clips.insert(
            index + 1,
            Clip {
                from: middle,
                to: clip.to,
            },
        );
        true
    }

    /// Takes clip `index` out, unless it is the only one.
    pub fn remove(&mut self, index: usize) -> bool {
        if self.clips.len() < 2 || index >= self.clips.len() {
            return false;
        }
        self.clips.remove(index);
        true
    }

    /// Moves clip `from` to place `to` among the others.
    pub fn move_clip(&mut self, from: usize, to: usize) -> bool {
        if from >= self.clips.len() || to >= self.clips.len() || from == to {
            return false;
        }
        let clip = self.clips.remove(from);
        self.clips.insert(to, clip);
        true
    }

    /// Turns the picture a quarter turn as it shows.
    pub fn rotate(&mut self, clockwise: bool) {
        self.transform.rotate(clockwise);
    }

    /// Flips the picture as it shows: left to right when `horizontal`,
    /// else top to bottom.
    pub fn flip(&mut self, horizontal: bool) {
        self.transform.flip(horizontal);
    }

    /// The parts of the file to read, in order: clips that carry on one
    /// another read as one.
    pub(crate) fn spans(&self) -> Vec<Span> {
        let mut spans: Vec<Span> = Vec::new();
        let mut at = 0.0;
        for clip in &self.clips {
            match spans.last_mut() {
                Some(last) if (clip.from - last.to).abs() < 1e-9 => last.to = clip.to,
                _ => spans.push(Span {
                    index: spans.len(),
                    from: clip.from,
                    to: clip.to,
                    shift: at - clip.from,
                }),
            }
            at += clip.length();
        }
        spans
    }

    fn replace(&mut self, clips: Vec<Clip>) -> bool {
        if clips.is_empty() || clips == self.clips {
            return false;
        }
        self.clips = clips;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(first: f64, second: f64) -> bool {
        (first - second).abs() < 1e-9
    }

    fn froms(edit: &Edit) -> Vec<(f64, f64)> {
        edit.clips()
            .iter()
            .map(|clip| (clip.from, clip.to))
            .collect()
    }

    #[test]
    fn trimming_keeps_the_part_chosen() {
        let mut edit = Edit::whole(60.0);
        assert!(edit.trim(10.0, 40.0));
        assert_eq!(froms(&edit), [(10.0, 40.0)]);
        assert!(close(edit.duration(), 30.0));
        assert!(close(edit.source_of(5.0), 15.0));
        assert_eq!(edit.edited_of(12.0), Some(2.0));
        assert_eq!(edit.edited_of(5.0), None);
        // Too short to keep.
        assert!(!edit.trim(1.0, 1.05));
        // Trimming again works on the edited time.
        assert!(edit.trim(0.0, 10.0));
        assert_eq!(froms(&edit), [(10.0, 20.0)]);
    }

    #[test]
    fn splitting_and_moving_reorders_the_movie() {
        let mut edit = Edit::whole(30.0);
        assert!(edit.split(10.0));
        assert!(edit.split(20.0));
        assert!(!edit.split(20.05), "too close to a clip's start");
        assert!(edit.plays_whole(30.0));
        assert_eq!(froms(&edit), [(0.0, 10.0), (10.0, 20.0), (20.0, 30.0)]);
        // Clips that carry on one another are read as one.
        assert_eq!(edit.spans().len(), 1);
        assert!(edit.move_clip(2, 0));
        assert_eq!(froms(&edit), [(20.0, 30.0), (0.0, 10.0), (10.0, 20.0)]);
        assert!(!edit.plays_whole(30.0));
        let spans = edit.spans();
        assert_eq!(spans.len(), 2);
        assert!(close(spans[0].shift, -20.0));
        assert!(close(spans[1].from, 0.0) && close(spans[1].to, 20.0));
        assert!(close(spans[1].shift, 10.0));
        assert!(close(edit.source_of(12.0), 2.0));
        assert_eq!(edit.edited_of(25.0), Some(5.0));
        assert!(edit.remove(0));
        assert_eq!(froms(&edit), [(0.0, 10.0), (10.0, 20.0)]);
        assert!(edit.remove(0));
        assert!(!edit.remove(0), "the last clip stays");
    }

    #[test]
    fn trimming_across_clips_keeps_their_parts() {
        let mut edit = Edit::whole(30.0);
        edit.split(10.0);
        edit.move_clip(1, 0);
        // 10 to 30, then 0 to 10.
        assert!(edit.trim(15.0, 25.0));
        assert_eq!(froms(&edit), [(25.0, 30.0), (0.0, 5.0)]);
        // A sliver of a clip at an edge is dropped.
        let mut edit = Edit::whole(30.0);
        edit.split(10.0);
        assert!(edit.trim(9.99, 20.0));
        assert_eq!(froms(&edit), [(10.0, 20.0)]);
        // A clip trimmed by itself.
        let mut edit = Edit::whole(30.0);
        edit.split(10.0);
        assert!(edit.trim_clip(1, 12.0, 18.0));
        assert_eq!(froms(&edit), [(0.0, 10.0), (12.0, 18.0)]);
        assert!(!edit.is_whole(30.0));
        assert!(Edit::whole(30.0).is_whole(30.0));
    }

    /// Where a point of a square picture shows: mirrored, then turned
    /// clockwise, x right and y down.
    fn shown(turns: u8, mirrored: bool, (x, y): (i32, i32)) -> (i32, i32) {
        let mut point = if mirrored { (1 - x, y) } else { (x, y) };
        for _ in 0..turns {
            point = (1 - point.1, point.0);
        }
        point
    }

    #[test]
    fn turns_and_flips_act_on_the_picture_as_it_shows() {
        let corners = [(0, 0), (1, 0), (0, 1), (1, 1)];
        let horizontal = |(x, y): (i32, i32)| (1 - x, y);
        let vertical = |(x, y): (i32, i32)| (x, 1 - y);
        let clockwise = |(x, y): (i32, i32)| (1 - y, x);
        for turns in 0..4 {
            for mirrored in [false, true] {
                let before = Transform { turns, mirrored };
                for (change, expected) in [
                    (0, horizontal as fn((i32, i32)) -> (i32, i32)),
                    (1, vertical),
                    (2, clockwise),
                ] {
                    let mut after = before;
                    match change {
                        0 => after.flip(true),
                        1 => after.flip(false),
                        _ => after.rotate(true),
                    }
                    for corner in corners {
                        assert_eq!(
                            shown(after.turns, after.mirrored, corner),
                            expected(shown(turns, mirrored, corner)),
                        );
                    }
                }
            }
        }
        // A file's own turn comes before the edit's.
        for own in 0..4 {
            let mut flipped = Transform::default();
            flipped.flip(true);
            flipped.rotate(true);
            let combined = flipped.after_turns(own);
            for corner in corners {
                assert_eq!(
                    shown(combined.turns, combined.mirrored, corner),
                    clockwise(horizontal(shown(own, false, corner)))
                );
            }
        }
    }
}
