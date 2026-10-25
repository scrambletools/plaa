//! An edit of a file: the parts of it kept, in the order they play, and
//! how its picture is turned and flipped. Clips are in seconds of the
//! file they are from, as plaa counts them from its start; the edited
//! movie's own time runs through its clips one after another. A clip may
//! be from another file, and its picture or sound may be another file's,
//! or none.

use std::path::{Path, PathBuf};

/// The shortest clip trimming or splitting leaves.
pub const SHORTEST: f64 = 0.1;

/// Parts left shorter than this, about a frame, are dropped rather than
/// kept as a clip.
const SLIVER: f64 = 0.02;

/// The file edited, as clips number the files they are from.
pub const OWN: usize = 0;

/// Where a clip's picture or sound comes from in place of its own.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Feed {
    /// From file `source`, numbered as [`Clip::source`] is, from `from`
    /// seconds of it on.
    File { source: usize, from: f64 },
    /// A black picture, or silence.
    Blank,
}

impl Feed {
    /// The same feed `offset` seconds on.
    fn later(self, offset: f64) -> Feed {
        match self {
            Feed::File { source, from } => Feed::File {
                source,
                from: from + offset,
            },
            Feed::Blank => Feed::Blank,
        }
    }
}

/// A part of a file, from and to seconds of it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Clip {
    /// The file it is from: [`OWN`], or one of the edit's others,
    /// numbered from 1.
    pub source: usize,
    pub from: f64,
    pub to: f64,
    /// The picture shown in place of the clip's own, when replaced.
    pub picture: Option<Feed>,
    /// The sound heard in place of the clip's own, when replaced.
    pub sound: Option<Feed>,
    /// The filters its picture and sound go through, by their number in
    /// the edit's.
    pub filter: Option<usize>,
}

impl Clip {
    /// Part `from` to `to` of the file edited, as it is.
    pub fn new(from: f64, to: f64) -> Self {
        Self {
            source: OWN,
            from,
            to,
            picture: None,
            sound: None,
            filter: None,
        }
    }

    pub fn length(&self) -> f64 {
        (self.to - self.from).max(0.0)
    }

    /// Where its picture comes from.
    pub fn picture_feed(&self) -> Feed {
        self.picture.unwrap_or(Feed::File {
            source: self.source,
            from: self.from,
        })
    }

    /// Where its sound comes from.
    pub fn sound_feed(&self) -> Feed {
        self.sound.unwrap_or(Feed::File {
            source: self.source,
            from: self.from,
        })
    }

    /// Whether it plays the file edited, its own picture and sound, as
    /// they are.
    pub fn is_plain(&self) -> bool {
        self.is_own() && self.filter.is_none()
    }

    /// Whether it plays the file edited, its own picture and sound, read
    /// as one, filtered or not.
    fn is_own(&self) -> bool {
        self.source == OWN && self.picture.is_none() && self.sound.is_none()
    }

    /// Its part from `low` to `high` seconds into it.
    fn part(&self, low: f64, high: f64) -> Clip {
        Clip {
            source: self.source,
            from: self.from + low,
            to: self.from + high,
            picture: self.picture.map(|feed| feed.later(low)),
            sound: self.sound.map(|feed| feed.later(low)),
            filter: self.filter,
        }
    }
}

/// FFmpeg filter graphs a clip's picture and sound go through, as `-vf`
/// and `-af` take them.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Filters {
    pub picture: Option<String>,
    pub sound: Option<String>,
}

impl Filters {
    /// These filters, then `more` after them.
    pub fn then(&self, more: &Filters) -> Filters {
        let join = |first: &Option<String>, second: &Option<String>| match (first, second) {
            (Some(first), Some(second)) => Some(format!("{first},{second}")),
            (first, second) => first.clone().or_else(|| second.clone()),
        };
        Filters {
            picture: join(&self.picture, &more.picture),
            sound: join(&self.sound, &more.sound),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.picture.is_none() && self.sound.is_none()
    }
}

/// Another file an edit takes clips, pictures or sound from.
#[derive(Debug, Clone, PartialEq)]
pub struct Other {
    pub path: PathBuf,
    /// How long it lasts, in seconds.
    pub duration: f64,
    /// Whether it has a picture, other than a cover.
    pub picture: bool,
    pub sound: bool,
}

impl Other {
    /// The file at `path`, as `info` describes it.
    pub fn new(path: PathBuf, info: &crate::info::MediaInfo) -> Self {
        Self {
            path,
            duration: info.duration.map_or(0.0, |duration| duration.as_secs_f64()),
            picture: info.has_video(),
            sound: !info.audio.is_empty(),
        }
    }
}

/// A clip's picture or its sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Track {
    Picture,
    Sound,
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
    others: Vec<Other>,
    filters: Vec<Filters>,
    layers: Vec<Layer>,
}

/// Sound mixed in over the edit: part of a file, heard with what plays
/// from `at` seconds of the edited time, where it stays as clips change.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Layer {
    /// The file, numbered as [`Clip::source`] is.
    pub source: usize,
    pub at: f64,
    /// The part of the file, from and to seconds of it.
    pub from: f64,
    pub to: f64,
    /// How much louder, in decibels: negative is quieter.
    pub volume: f64,
    /// Seconds it fades in over at its start, and out over at its end.
    pub fade_in: f64,
    pub fade_out: f64,
}

impl Layer {
    /// Where it ends in the edited time.
    pub fn end(&self) -> f64 {
        self.at + (self.to - self.from).max(0.0)
    }

    /// How loud it is at `seconds` of the edited time, as a factor: its
    /// volume, faded in and out; nothing outside it.
    pub fn gain(&self, seconds: f64) -> f32 {
        let end = self.end();
        if seconds < self.at || seconds >= end {
            return 0.0;
        }
        let mut gain = 10f64.powf(self.volume / 20.0);
        if self.fade_in > 0.0 {
            gain *= ((seconds - self.at) / self.fade_in).min(1.0);
        }
        if self.fade_out > 0.0 {
            gain *= ((end - seconds) / self.fade_out).min(1.0);
        }
        gain as f32
    }
}

/// Part of a file read in one go: clips one after another in a file play
/// as one. Its packets carry it, so decoders know which part they are in
/// and where its pictures and sound go in the edited time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Span {
    pub index: usize,
    /// The file read, numbered as [`Clip::source`] is; none for a black
    /// picture or silence, from `from` to `to` of the edited time.
    pub source: Option<usize>,
    /// The filters it goes through, by their number in the edit's.
    pub filter: Option<usize>,
    pub from: f64,
    pub to: f64,
    /// Added to the file's time to give the edited time.
    pub shift: f64,
}

impl Span {
    /// The whole file, played as it is.
    pub const WHOLE: Span = Span {
        index: 0,
        source: Some(OWN),
        filter: None,
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

/// What the picture and the sound of an edit read: the parts of each
/// file in order, apart when a clip is from another file or has its
/// picture or sound replaced.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Lanes {
    pub picture: Vec<Span>,
    pub sound: Vec<Span>,
}

impl Edit {
    /// The whole file, `duration` seconds long, as it is.
    pub fn whole(duration: f64) -> Self {
        Self::new(vec![Clip::new(0.0, duration.max(0.0))])
    }

    /// `clips` of a file, played in that order, as they are.
    pub fn new(clips: Vec<Clip>) -> Self {
        Self {
            clips,
            transform: Transform::default(),
            others: Vec::new(),
            filters: Vec::new(),
            layers: Vec::new(),
        }
    }

    pub fn clips(&self) -> &[Clip] {
        &self.clips
    }

    /// The other files the edit takes from, numbered from 1 in clips.
    pub fn others(&self) -> &[Other] {
        &self.others
    }

    /// File `source` of the clips, unless it is the file edited.
    pub fn other(&self, source: usize) -> Option<&Other> {
        source
            .checked_sub(1)
            .and_then(|index| self.others.get(index))
    }

    /// Adds `other` to the files the edit takes from, unless it is there
    /// already. Returns its number in clips.
    pub fn add_other(&mut self, other: Other) -> usize {
        match self
            .others
            .iter()
            .position(|known| known.path == other.path)
        {
            Some(index) => {
                self.others[index] = other;
                index + 1
            }
            None => {
                self.others.push(other);
                self.others.len()
            }
        }
    }

    /// The number of the other file at `path`, if the edit takes from it.
    pub fn source_named(&self, path: &Path) -> Option<usize> {
        self.others
            .iter()
            .position(|other| other.path == path)
            .map(|index| index + 1)
    }

    /// Whether a clip is from another file, or has its picture or sound
    /// replaced, so the picture and sound may be read apart.
    pub fn is_mixed(&self) -> bool {
        self.clips.iter().any(|clip| !clip.is_own()) || !self.layers.is_empty()
    }

    /// Whether `other` plays as this does: the same clips, through the
    /// same filters, from the same files, with the same sound mixed in.
    /// Turning and flipping are the picture's alone, so they may differ.
    pub fn plays_like(&self, other: &Edit) -> bool {
        self.clips == other.clips
            && self.filters == other.filters
            && self.others == other.others
            && self.layers == other.layers
    }

    /// The sound mixed in over the edit, in the order it was added.
    pub fn layers(&self) -> &[Layer] {
        &self.layers
    }

    /// Mixes `layer` in. Returns whether it did: it must last at least
    /// [`SHORTEST`] and start within the edit.
    pub fn add_layer(&mut self, layer: Layer) -> bool {
        if layer.to - layer.from < SHORTEST - 1e-9 || layer.at < 0.0 || layer.at >= self.duration()
        {
            return false;
        }
        self.layers.push(layer);
        true
    }

    /// Takes mixed-in sound number `index` out.
    pub fn remove_layer(&mut self, index: usize) -> bool {
        if index >= self.layers.len() {
            return false;
        }
        self.layers.remove(index);
        true
    }

    /// A file the edit's sound may come from other than the file edited:
    /// one its sound reads, else one mixed in.
    pub fn first_other_sound(&self, sound_lane: &[usize]) -> Option<usize> {
        sound_lane
            .iter()
            .copied()
            .find(|source| *source != OWN)
            .or_else(|| self.layers.first().map(|layer| layer.source))
            .filter(|source| *source != OWN)
    }

    /// Whether a clip's picture or sound goes through filters.
    pub fn is_filtered(&self) -> bool {
        self.clips.iter().any(|clip| clip.filter.is_some())
    }

    /// Filters number `index` of the edit's, as clips and spans name them.
    pub fn filter(&self, index: usize) -> Option<&Filters> {
        self.filters.get(index)
    }

    /// The picture filters every clip goes through, when they all go
    /// through the same: those of the whole movie.
    pub fn whole_picture_filters(&self) -> Option<&str> {
        let first = self.clips.first()?.filter?;
        self.clips
            .iter()
            .all(|clip| clip.filter == Some(first))
            .then(|| self.filters.get(first)?.picture.as_deref())
            .flatten()
    }

    /// The filters clip `clip` goes through, if any.
    pub fn filters_of(&self, clip: &Clip) -> Option<&Filters> {
        clip.filter.and_then(|index| self.filters.get(index))
    }

    /// The filters the clips from `from` to `to` seconds of the edited
    /// time would go through with `added` after their own: each set
    /// once, to check them before adding them.
    pub fn filters_with(&self, from: f64, to: f64, added: &Filters) -> Vec<Filters> {
        let mut all: Vec<Filters> = Vec::new();
        for (clip, start) in self.clips.iter().zip(self.starts()) {
            if start + clip.length() <= from + SLIVER || start >= to - SLIVER {
                continue;
            }
            let combined = match self.filters_of(clip) {
                Some(own) => own.then(added),
                None => added.clone(),
            };
            if !all.contains(&combined) {
                all.push(combined);
            }
        }
        all
    }

    /// Puts the picture and sound from `from` to `to` seconds of the
    /// edited time through `added`, after the filters they have. Returns
    /// whether that changed the edit.
    pub fn add_filters(&mut self, from: f64, to: f64, added: &Filters) -> bool {
        if added.is_empty() {
            return false;
        }
        self.set_filters(from, to, |own| {
            Some(match own {
                Some(own) => own.then(added),
                None => added.clone(),
            })
        })
    }

    /// Takes every filter off the picture and sound from `from` to `to`
    /// seconds of the edited time. Returns whether that changed the edit.
    pub fn remove_filters(&mut self, from: f64, to: f64) -> bool {
        self.set_filters(from, to, |_| None)
    }

    /// Gives the clips from `from` to `to` the filters `filters` makes of
    /// theirs, splitting clips at both ends.
    fn set_filters(
        &mut self,
        from: f64,
        to: f64,
        filters: impl Fn(Option<&Filters>) -> Option<Filters>,
    ) -> bool {
        let (from, to) = (from.max(0.0), to.min(self.duration()));
        if to - from < SLIVER {
            return false;
        }
        let before = (self.clips.clone(), self.filters.clone());
        let first = self.cut_at(from);
        let last = self.cut_at(to);
        for index in first..last {
            let made = filters(self.filters_of(&self.clips[index])).filter(|made| !made.is_empty());
            let number =
                made.map(
                    |made| match self.filters.iter().position(|known| *known == made) {
                        Some(known) => known,
                        None => {
                            self.filters.push(made);
                            self.filters.len() - 1
                        }
                    },
                );
            self.clips[index].filter = number;
        }
        (self.clips.clone(), self.filters.clone()) != before
    }

    /// How the picture is turned and flipped.
    pub fn transform(&self) -> Transform {
        self.transform
    }

    /// Whether this is the whole of a file `duration` seconds long, as it
    /// is.
    pub fn is_whole(&self, duration: f64) -> bool {
        self.transform.is_identity()
            && self.layers.is_empty()
            && matches!(self.clips.as_slice(), [only]
                if only.is_plain()
                    && only.from <= 1e-6
                    && (only.to - duration).abs() <= 1e-6)
    }

    /// Whether the clips, one after another, play the whole file `duration`
    /// seconds long through, as after splitting alone.
    pub fn plays_whole(&self, duration: f64) -> bool {
        let mut at = 0.0;
        for clip in &self.clips {
            if !clip.is_plain() || (clip.from - at).abs() > 1e-6 {
                return false;
            }
            at = clip.to;
        }
        (at - duration).abs() <= 1e-6 && self.layers.is_empty()
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

    /// The file the clip at edited time `seconds` is from, and its time
    /// there.
    pub fn locate(&self, seconds: f64) -> Option<(usize, f64)> {
        let (index, start) = self.clip_at(seconds)?;
        let clip = self.clips[index];
        Some((
            clip.source,
            (clip.from + (seconds - start).max(0.0)).min(clip.to),
        ))
    }

    /// Where `track` plays from at `seconds` of the edited time: a file
    /// and its time there; none where it is black or silent, or past the
    /// end of the file it plays.
    pub fn fed_at(&self, track: Track, seconds: f64) -> Option<(usize, f64)> {
        let (index, start) = self.clip_at(seconds)?;
        let clip = self.clips[index];
        let feed = match track {
            Track::Picture => clip.picture_feed(),
            Track::Sound => clip.sound_feed(),
        };
        match feed {
            Feed::File { source, from } => {
                let at = from + (seconds - start).clamp(0.0, clip.length());
                let past = self.other(source).is_some_and(|other| at >= other.duration);
                (!past).then_some((source, at))
            }
            Feed::Blank => None,
        }
    }

    /// The time at edited time `seconds` in the file its clip is from.
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
            if clip.source == OWN && seconds >= clip.from - 1e-9 && seconds < clip.to {
                return Some(start + (seconds - clip.from).max(0.0));
            }
            start += clip.length();
        }
        None
    }

    /// The time in the file edited at edited time `seconds`: in its own
    /// clip there, or where the next of its own clips starts where
    /// another file's part plays; none past its last clip.
    pub fn own_time_at(&self, seconds: f64) -> Option<f64> {
        let mut start = 0.0;
        for clip in &self.clips {
            let end = start + clip.length();
            if clip.source == OWN && seconds < end {
                return Some((clip.from + (seconds - start).max(0.0)).min(clip.to));
            }
            start = end;
        }
        None
    }

    /// The edited time the file's time `seconds` plays at, or, where that
    /// part was taken out, where the first part of the file after it
    /// starts; none past the last part kept.
    pub fn edited_from(&self, seconds: f64) -> Option<f64> {
        if let Some(edited) = self.edited_of(seconds) {
            return Some(edited);
        }
        let mut start = 0.0;
        let mut next: Option<(f64, f64)> = None;
        for clip in &self.clips {
            if clip.source == OWN
                && clip.from > seconds
                && next.is_none_or(|(from, _)| clip.from < from)
            {
                next = Some((clip.from, start));
            }
            start += clip.length();
        }
        next.map(|(_, edited)| edited)
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
                kept.push(clip.part(low - start, high - start));
            }
            start = end;
        }
        self.replace(kept)
    }

    /// Takes the edited movie from `from` to `to` seconds out. Returns
    /// whether that changed it; it does not when too little would be
    /// left.
    pub fn cut(&mut self, from: f64, to: f64) -> bool {
        let (from, to) = (from.max(0.0), to.min(self.duration()));
        if to - from < SLIVER || self.duration() - (to - from) < SHORTEST - 1e-9 {
            return false;
        }
        let mut kept = Vec::new();
        let mut start = 0.0;
        for clip in &self.clips {
            let end = start + clip.length();
            for (low, high) in [(start, end.min(from)), (start.max(to), end)] {
                if high - low >= SLIVER {
                    kept.push(clip.part(low - start, high - start));
                }
            }
            start = end;
        }
        self.replace(kept)
    }

    /// Puts `clip` in at edited time `at`, splitting the clip there.
    /// Returns whether it did: a clip shorter than [`SHORTEST`] is not
    /// put in.
    pub fn insert(&mut self, at: f64, clip: Clip) -> bool {
        if clip.length() < SHORTEST - 1e-9 {
            return false;
        }
        let index = self.cut_at(at);
        self.clips.insert(index, clip);
        true
    }

    /// Plays `feed` for `track` from `from` to `to` seconds of the edited
    /// time, from its start there on, in place of what the clips there
    /// have; none puts the clips' own back. Returns whether that changed
    /// the edit.
    pub fn replace_track(&mut self, track: Track, from: f64, to: f64, feed: Option<Feed>) -> bool {
        let (from, to) = (from.max(0.0), to.min(self.duration()));
        if to - from < SLIVER {
            return false;
        }
        let before = self.clips.clone();
        let first = self.cut_at(from);
        let last = self.cut_at(to);
        let starts = self.starts();
        for (clip, start) in self.clips[first..last].iter_mut().zip(&starts[first..last]) {
            let feed = feed.map(|feed| feed.later(start - from));
            // A clip's own picture or sound is no replacement.
            let own = matches!(feed, Some(Feed::File { source, from })
                if source == clip.source && (from - clip.from).abs() < 1e-6);
            let feed = if own { None } else { feed };
            match track {
                Track::Picture => clip.picture = feed,
                Track::Sound => clip.sound = feed,
            }
        }
        self.clips != before
    }

    /// Splits the clip playing at edited time `seconds` there, unless
    /// that is within a sliver of its ends. Returns the number of the
    /// clip that starts there, or would.
    fn cut_at(&mut self, seconds: f64) -> usize {
        let mut start = 0.0;
        for (index, clip) in self.clips.iter().enumerate() {
            let into = seconds - start;
            if into < SLIVER {
                return index;
            }
            if into <= clip.length() - SLIVER {
                let clip = *clip;
                self.clips[index] = clip.part(0.0, into);
                self.clips.insert(index + 1, clip.part(into, clip.length()));
                return index + 1;
            }
            start += clip.length();
        }
        self.clips.len()
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
        clips[index] = clip.part(low, high);
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
        self.clips[index] = clip.part(0.0, into);
        self.clips.insert(index + 1, clip.part(into, clip.length()));
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
    /// another read as one. Only for an edit that is not mixed.
    pub(crate) fn spans(&self) -> Vec<Span> {
        self.lane(|clip| Feed::File {
            source: OWN,
            from: clip.from,
        })
    }

    /// The parts of each file the picture and the sound read, in order:
    /// none past another file's end, or of what it lacks. The file edited
    /// has a picture and sound as `own` says.
    pub(crate) fn lanes(&self, own: (bool, bool)) -> Lanes {
        let has = |source: usize, track: Track| match self.other(source) {
            Some(other) => match track {
                Track::Picture => other.picture,
                Track::Sound => other.sound,
            },
            None => match track {
                Track::Picture => own.0,
                Track::Sound => own.1,
            },
        };
        let length = |source: usize| self.other(source).map(|other| other.duration);
        let fed = |feed: Feed, track: Track| match feed {
            Feed::File { source, .. } if !has(source, track) => Feed::Blank,
            feed => feed,
        };
        Lanes {
            picture: self.lane_within(|clip| fed(clip.picture_feed(), Track::Picture), length),
            sound: self.lane_within(|clip| fed(clip.sound_feed(), Track::Sound), length),
        }
    }

    /// The parts read for the feed `feed` gives each clip.
    fn lane(&self, feed: impl Fn(&Clip) -> Feed) -> Vec<Span> {
        self.lane_within(feed, |_| None)
    }

    /// The parts read for the feed `feed` gives each clip, blank past the
    /// end of a file `length` gives.
    fn lane_within(
        &self,
        feed: impl Fn(&Clip) -> Feed,
        length: impl Fn(usize) -> Option<f64>,
    ) -> Vec<Span> {
        let mut spans: Vec<Span> = Vec::new();
        let mut push =
            |source: Option<usize>, from: f64, to: f64, at: f64, filter: Option<usize>| {
                if to - from <= 1e-9 {
                    return;
                }
                // Blank parts count in the edited time.
                let (from, to, shift) = match source {
                    Some(_) => (from, to, at - from),
                    None => (at, at + (to - from), 0.0),
                };
                match spans.last_mut() {
                    Some(last)
                        if last.source == source
                            && last.filter == filter
                            && (from - last.to).abs() < 1e-9
                            && (last.shift - shift).abs() < 1e-9 =>
                    {
                        last.to = to;
                    }
                    _ => spans.push(Span {
                        index: spans.len(),
                        source,
                        filter,
                        from,
                        to,
                        shift,
                    }),
                }
            };
        let mut at = 0.0;
        for clip in &self.clips {
            let span = clip.length();
            match feed(clip) {
                Feed::File { source, from } => {
                    let end =
                        length(source).map_or(from + span, |length| (from + span).min(length));
                    push(Some(source), from, end.max(from), at, clip.filter);
                    let read = (end - from).clamp(0.0, span);
                    push(None, 0.0, span - read, at + read, None);
                }
                Feed::Blank => push(None, 0.0, span, at, None),
            }
            at += span;
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

    #[test]
    fn cutting_takes_a_part_out() {
        let mut edit = Edit::whole(30.0);
        assert!(edit.cut(10.0, 20.0));
        assert_eq!(froms(&edit), [(0.0, 10.0), (20.0, 30.0)]);
        assert!(close(edit.duration(), 20.0));
        // Across the cut just made.
        assert!(edit.cut(5.0, 15.0));
        assert_eq!(froms(&edit), [(0.0, 5.0), (25.0, 30.0)]);
        // Not all of it.
        assert!(!edit.cut(0.0, 10.0));
        assert!(edit.cut(0.0, 5.0));
        assert_eq!(froms(&edit), [(25.0, 30.0)]);
    }

    fn other(path: &str, duration: f64, picture: bool) -> Other {
        Other {
            path: PathBuf::from(path),
            duration,
            picture,
            sound: true,
        }
    }

    #[test]
    fn casting_maps_the_edit_to_the_file_and_back() {
        let mut edit = Edit::whole(60.0);
        assert!(edit.cut(10.0, 20.0));
        let intro = edit.add_other(other("/intro.mp4", 5.0, true));
        let clip = Clip {
            source: intro,
            ..Clip::new(0.0, 5.0)
        };
        assert!(edit.insert(0.0, clip));
        // Edited: 0 to 5 the intro, 5 to 15 the file's 0 to 10, then 15
        // on its 20 to 60.
        assert_eq!(edit.own_time_at(2.0), Some(0.0), "the intro plays");
        assert_eq!(edit.own_time_at(7.0), Some(2.0));
        assert_eq!(edit.own_time_at(16.0), Some(21.0));
        assert_eq!(edit.own_time_at(edit.duration()), None);
        assert_eq!(edit.edited_from(2.0), Some(7.0));
        assert_eq!(edit.edited_from(12.0), Some(15.0), "cut: the part after");
        assert_eq!(edit.edited_from(21.0), Some(16.0));
        assert_eq!(edit.edited_from(60.0), None);
        let whole = Edit::whole(30.0);
        assert_eq!(whole.own_time_at(12.0), Some(12.0));
        assert_eq!(whole.edited_from(12.0), Some(12.0));
    }

    #[test]
    fn inserting_puts_another_file_in() {
        let mut edit = Edit::whole(30.0);
        let intro = edit.add_other(other("/intro.mp4", 5.0, true));
        assert_eq!(intro, 1);
        assert_eq!(
            edit.add_other(other("/intro.mp4", 5.0, true)),
            1,
            "known already"
        );
        let clip = Clip {
            source: intro,
            ..Clip::new(0.0, 5.0)
        };
        assert!(edit.insert(10.0, clip));
        assert_eq!(froms(&edit), [(0.0, 10.0), (0.0, 5.0), (10.0, 30.0)]);
        assert_eq!(edit.clips()[1].source, intro);
        assert!(edit.is_mixed());
        assert!(!edit.plays_whole(30.0));
        assert!(close(edit.duration(), 35.0));
        assert_eq!(edit.locate(12.0), Some((intro, 2.0)));
        assert_eq!(edit.locate(16.0), Some((OWN, 11.0)));
        // The file edited's time is found only in its own clips.
        assert_eq!(edit.edited_of(2.0), Some(2.0));
        assert_eq!(edit.edited_of(11.0), Some(16.0));
        // At the start and at the end, nothing is split.
        assert!(edit.insert(0.0, clip));
        assert!(edit.insert(edit.duration(), clip));
        assert_eq!(edit.clips().len(), 5);
        assert!(!edit.insert(1.0, Clip::new(0.0, 0.05)), "too short");
        let lanes = edit.lanes((true, true));
        assert_eq!(lanes.picture, lanes.sound);
        let sources: Vec<_> = lanes.picture.iter().map(|span| span.source).collect();
        assert_eq!(
            sources,
            [Some(intro), Some(OWN), Some(intro), Some(OWN), Some(intro)]
        );
    }

    #[test]
    fn replacing_the_sound_leaves_the_picture() {
        let mut edit = Edit::whole(60.0);
        let music = edit.add_other(other("/music.flac", 25.0, false));
        assert!(edit.replace_track(
            Track::Sound,
            10.0,
            40.0,
            Some(Feed::File {
                source: music,
                from: 0.0,
            }),
        ));
        assert_eq!(froms(&edit), [(0.0, 10.0), (10.0, 40.0), (40.0, 60.0)]);
        assert_eq!(edit.clips()[1].picture, None);
        assert_eq!(
            edit.clips()[1].sound,
            Some(Feed::File {
                source: music,
                from: 0.0,
            })
        );
        let lanes = edit.lanes((true, true));
        assert_eq!(lanes.picture.len(), 1, "the picture reads on through");
        let sound: Vec<_> = lanes
            .sound
            .iter()
            .map(|span| (span.source, span.from, span.to, span.shift))
            .collect();
        // The music lasts 25 seconds of the 30: then silence.
        assert_eq!(
            sound,
            [
                (Some(OWN), 0.0, 10.0, 0.0),
                (Some(music), 0.0, 25.0, 10.0),
                (None, 35.0, 40.0, 0.0),
                (Some(OWN), 40.0, 60.0, 0.0),
            ]
        );
        // Splitting a replaced clip keeps each half's part of the music.
        assert!(edit.split(20.0));
        assert_eq!(
            edit.clips()[2].sound,
            Some(Feed::File {
                source: music,
                from: 10.0,
            })
        );
        // Its own sound back.
        assert!(edit.replace_track(Track::Sound, 0.0, 60.0, None));
        assert!(edit.clips().iter().all(|clip| clip.sound.is_none()));
        assert!(edit.plays_whole(60.0));
        // A picture with no file is black, and not the clip's own.
        assert!(edit.replace_track(Track::Picture, 50.0, 60.0, Some(Feed::Blank)));
        let picture = edit.lanes((true, true)).picture;
        assert_eq!(picture.last().unwrap().source, None);
        assert!(close(picture.last().unwrap().from, 50.0));
        // Its own sound as a replacement is none.
        assert!(!edit.replace_track(
            Track::Sound,
            0.0,
            10.0,
            Some(Feed::File {
                source: OWN,
                from: 0.0,
            }),
        ));
    }

    #[test]
    fn a_file_without_a_picture_shows_black() {
        let mut edit = Edit::whole(10.0);
        let song = edit.add_other(other("/song.mp3", 4.0, false));
        edit.insert(
            10.0,
            Clip {
                source: song,
                ..Clip::new(0.0, 4.0)
            },
        );
        let lanes = edit.lanes((true, true));
        assert_eq!(lanes.picture.last().unwrap().source, None);
        assert_eq!(lanes.sound.last().unwrap().source, Some(song));
        // Plain clips read as before.
        assert_eq!(
            Edit::whole(10.0).spans(),
            Edit::whole(10.0).lanes((true, true)).picture
        );
    }

    #[test]
    fn filters_stack_on_a_part_and_come_off() {
        let gray = Filters {
            picture: Some("hue=s=0".to_owned()),
            sound: None,
        };
        let louder = Filters {
            picture: None,
            sound: Some("volume=2".to_owned()),
        };
        let mut edit = Edit::whole(30.0);
        assert!(edit.add_filters(0.0, 30.0, &gray));
        assert!(edit.is_filtered() && !edit.is_mixed());
        assert!(!edit.plays_whole(30.0), "the player plays the filters");
        // A part gets more, after what it has.
        assert_eq!(edit.filters_with(10.0, 20.0, &louder).len(), 1);
        assert!(edit.add_filters(10.0, 20.0, &louder));
        assert_eq!(froms(&edit), [(0.0, 10.0), (10.0, 20.0), (20.0, 30.0)]);
        let middle = edit.filters_of(&edit.clips()[1]).unwrap();
        assert_eq!(middle.picture.as_deref(), Some("hue=s=0"));
        assert_eq!(middle.sound.as_deref(), Some("volume=2"));
        // Spans part where filters change, though the file carries on.
        let spans = edit.spans();
        assert_eq!(spans.len(), 3);
        assert_ne!(spans[0].filter, spans[1].filter);
        assert_eq!(spans[0].filter, spans[2].filter);
        // Splitting keeps each half's filters.
        assert!(edit.split(15.0));
        assert_eq!(edit.clips()[1].filter, edit.clips()[2].filter);
        assert!(edit.remove_filters(0.0, 30.0));
        assert!(!edit.is_filtered());
        assert!(edit.plays_whole(30.0));
        assert!(
            !edit.add_filters(0.0, 5.0, &Filters::default()),
            "nothing to add"
        );
    }

    #[test]
    fn sound_mixed_in_keeps_its_place_and_fades() {
        let mut edit = Edit::whole(30.0);
        let music = edit.add_other(other("/music.flac", 60.0, false));
        let layer = Layer {
            source: music,
            at: 5.0,
            from: 10.0,
            to: 20.0,
            volume: -6.0,
            fade_in: 2.0,
            fade_out: 4.0,
        };
        assert!(edit.add_layer(layer));
        assert!(edit.is_mixed() && !edit.plays_whole(30.0) && !edit.is_whole(30.0));
        assert_eq!(edit.layers()[0].end(), 15.0);
        // Silent outside it, half way in half way through its fade in.
        assert_eq!(layer.gain(4.9), 0.0);
        assert!((layer.gain(6.0) - 0.5 * 0.501).abs() < 0.01);
        assert!((layer.gain(10.0) - 0.501).abs() < 0.01);
        assert!((layer.gain(14.0) - 0.501 / 4.0).abs() < 0.01);
        assert_eq!(layer.gain(15.0), 0.0);
        // Cutting clips leaves it where it was.
        edit.cut(0.0, 3.0);
        assert_eq!(edit.layers()[0].at, 5.0);
        assert!(!edit.add_layer(Layer { at: 40.0, ..layer }), "past the end");
        // Playing differently once mixed in, though the clips are the same.
        let mut quieter = edit.clone();
        quieter.layers[0].volume = -12.0;
        assert_eq!(quieter.clips(), edit.clips());
        assert!(!quieter.plays_like(&edit));
        let mut turned = edit.clone();
        turned.rotate(true);
        assert!(turned.plays_like(&edit), "turning is the picture's alone");
        assert!(edit.remove_layer(0) && !edit.remove_layer(0));
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
