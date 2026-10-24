//! Reading an edit that mixes files: the picture and the sound each read
//! their parts one after another, apart, each part from the file it is
//! in. A decoder switches to another file's packets as its lane reaches
//! them, and parts with nothing to read play black, or silent.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use ffmpeg_next as ffmpeg;

use crate::decode;
use crate::edit::{Edit, OWN, Span, Track};
use crate::info::MediaInfo;
use crate::player::{Decoders, Shared, feed_subtitles, packet_end, packet_order};
use crate::queue::{Item, PacketQueue};

/// The lanes of an edit that mixes files, and where each has read to.
pub(crate) struct Mixed {
    edit: Arc<Edit>,
    main: PathBuf,
    hardware: bool,
    lanes: Vec<Lane>,
    packet: ffmpeg::Packet,
}

/// The picture's or the sound's parts, and the file open for the one
/// being read.
struct Lane {
    track: Track,
    spans: Vec<Span>,
    current: usize,
    open: Option<Open>,
    /// The file the reader last told the decoder to switch to, since the
    /// last seek.
    sent: Option<usize>,
    done: bool,
}

/// A file a lane reads, and the stream it plays.
struct Open {
    source: usize,
    input: ffmpeg::format::context::Input,
    start: f64,
    stream: usize,
}

impl Mixed {
    /// Reads `edit` of the file at `main`, whose picture and sound play
    /// as `playing` says; its picture lane only when there is one.
    pub fn new(edit: Arc<Edit>, main: PathBuf, hardware: bool, playing: (bool, bool)) -> Self {
        let (picture, _) = playing;
        let lanes = edit.lanes(playing);
        let lane = |track, spans| Lane {
            track,
            spans,
            current: 0,
            open: None,
            sent: None,
            done: false,
        };
        let mut all = Vec::new();
        if picture {
            all.push(lane(Track::Picture, lanes.picture));
        }
        all.push(lane(Track::Sound, lanes.sound));
        Self {
            edit,
            main,
            hardware,
            lanes: all,
            packet: ffmpeg::Packet::empty(),
        }
    }

    /// The first file the sound reads, other than the one edited, and
    /// its time there: what a decoder starts from where the file edited
    /// has no sound.
    pub fn first_sound(&self) -> Option<(PathBuf, usize)> {
        let lane = self.lanes.iter().find(|lane| lane.track == Track::Sound)?;
        lane.spans.iter().find_map(|span| {
            let source = span.source.filter(|source| *source != OWN)?;
            Some((self.edit.other(source)?.path.clone(), source))
        })
    }

    /// Moves every lane to `position` of the edited time, for the seek
    /// of `serial`, once the queues are cleared.
    pub fn seek(&mut self, position: f64, serial: u64, decoders: &Decoders) {
        let context = Context {
            edit: &self.edit,
            main: &self.main,
            hardware: self.hardware,
        };
        for lane in &mut self.lanes {
            let Some((queue, stream)) = decoder_of(decoders, lane.track) else {
                lane.done = true;
                continue;
            };
            lane.done = false;
            lane.sent = None;
            let index = lane
                .spans
                .iter()
                .position(|span| position < span.to + span.shift)
                .unwrap_or(lane.spans.len().saturating_sub(1));
            let Some(span) = lane.spans.get(index) else {
                queue.put(Item::End(serial));
                lane.done = true;
                continue;
            };
            let at = (position - span.shift).clamp(span.from, span.to);
            lane.enter(index, at, serial, &queue, stream, decoders, &context);
        }
    }

    /// Whether every lane still reading has enough waiting for now.
    pub fn has_enough(&self, decoders: &Decoders) -> bool {
        self.lanes.iter().filter(|lane| !lane.done).all(|lane| {
            decoder_of(decoders, lane.track).is_none_or(|(queue, _)| queue.has_enough())
        })
    }

    /// Whether every lane has read to its end.
    pub fn read_all(&self) -> bool {
        self.lanes.iter().all(|lane| lane.done)
    }

    /// Reads a packet for the lane furthest behind that wants one, or
    /// moves it on to its next part.
    pub fn read(&mut self, serial: u64, decoders: &mut Decoders, shared: &Shared) {
        let context = Context {
            edit: &self.edit,
            main: &self.main,
            hardware: self.hardware,
        };
        let behind = self
            .lanes
            .iter_mut()
            .filter(|lane| !lane.done)
            .filter_map(|lane| {
                let (queue, _) = decoder_of(decoders, lane.track)?;
                (!queue.has_enough()).then(|| (queue.end().unwrap_or(f64::MIN), lane))
            })
            .min_by(|(first, _), (second, _)| first.total_cmp(second))
            .map(|(_, lane)| lane);
        let Some(lane) = behind else {
            return;
        };
        let Some((queue, stream)) = decoder_of(decoders, lane.track) else {
            lane.done = true;
            return;
        };
        let span = lane.spans[lane.current];
        let Some(open) = lane
            .open
            .as_mut()
            .filter(|open| span.source == Some(open.source))
        else {
            // A blank part, already handed to the decoder.
            lane.next(serial, &queue, stream, decoders, &context);
            return;
        };
        match self.packet.read(&mut open.input) {
            Ok(()) => {
                if self.packet.stream() != open.stream {
                    // The file edited's subtitles go with its picture.
                    if lane.track == Track::Picture && open.source == OWN {
                        feed_subtitles(decoders, shared, &self.packet, open.start);
                    }
                    return;
                }
                let past = packet_order(&open.input, &self.packet, open.start).is_some_and(
                    |(seconds, decoding)| {
                        seconds >= span.to + if decoding { 0.0 } else { REORDERED }
                    },
                );
                if past {
                    lane.next(serial, &queue, stream, decoders, &context);
                    return;
                }
                if let Some(end) = packet_end(&open.input, &self.packet, open.start) {
                    queue.reached(end.min(span.to) + span.shift);
                }
                let taken = std::mem::replace(&mut self.packet, ffmpeg::Packet::empty());
                queue.put(Item::Packet(serial, span, taken));
            }
            Err(ffmpeg::Error::InvalidData) => {}
            // The file's end, or a part of it that will not read.
            Err(_) => lane.next(serial, &queue, stream, decoders, &context),
        }
    }
}

/// Packets read by their presentation time alone count as past a part's
/// end only this far past it, as they may come before their pictures.
const REORDERED: f64 = 1.0;

/// What lanes need to open files.
struct Context<'a> {
    edit: &'a Edit,
    main: &'a Path,
    hardware: bool,
}

/// The decoder for `track`: its queue, and the stream of the file edited
/// it plays.
fn decoder_of(decoders: &Decoders, track: Track) -> Option<(Arc<PacketQueue>, usize)> {
    let decoder = match track {
        Track::Picture => decoders.video.as_ref(),
        Track::Sound => decoders.audio.as_ref(),
    };
    decoder.map(|(stream, queue, _)| (queue.clone(), *stream))
}

impl Lane {
    /// Moves on to the next part, or ends.
    fn next(
        &mut self,
        serial: u64,
        queue: &PacketQueue,
        stream: usize,
        decoders: &Decoders,
        context: &Context,
    ) {
        if self.current + 1 < self.spans.len() {
            let next = self.current + 1;
            let from = self.spans[next].from;
            self.enter(next, from, serial, queue, stream, decoders, context);
        } else {
            queue.put(Item::End(serial));
            self.done = true;
        }
    }

    /// Starts reading part `index` from `at`, its file's time: opens its
    /// file and has the decoder switch to it, or hands the decoder a
    /// blank part.
    #[expect(clippy::too_many_arguments)]
    fn enter(
        &mut self,
        index: usize,
        at: f64,
        serial: u64,
        queue: &PacketQueue,
        own_stream: usize,
        decoders: &Decoders,
        context: &Context,
    ) {
        self.current = index;
        let span = self.spans[index];
        let blank = |span: Span| {
            // In the edited time, as blank parts count.
            let blank = Span {
                source: None,
                from: span.from + span.shift,
                to: span.to + span.shift,
                shift: 0.0,
                ..span
            };
            queue.put(Item::Blank(serial, blank));
            queue.reached(blank.to);
        };
        let Some(source) = span.source else {
            blank(span);
            return;
        };
        // The file edited opens again for another of its streams.
        if self.open.as_ref().is_none_or(|open| {
            open.source != source || (source == OWN && open.stream != own_stream)
        }) {
            self.open = self.open_file(source, own_stream, context);
        }
        let subtitles = decoders.subtitles.as_ref().map(|(index, _)| *index);
        let Some(open) = &mut self.open else {
            blank(span);
            return;
        };
        // Only the stream played is read, and the file edited's subtitles
        // with its picture.
        for index in 0..open.input.nb_streams() as usize {
            let wanted = index == open.stream
                || (self.track == Track::Picture && source == OWN && Some(index) == subtitles);
            crate::ffi::set_skipped(&mut open.input, index, !wanted);
        }
        let timestamp = ((at + open.start) * f64::from(ffmpeg::ffi::AV_TIME_BASE)) as i64;
        if let Err(error) = open.input.seek(timestamp, ..timestamp) {
            eprintln!("plaa: seeking: {error}");
        }
        // The decoder switches files where it has to: what it was last
        // told, else what it decodes.
        let decoding = self.sent.unwrap_or_else(|| *queue.decoding());
        if decoding != source {
            let setup = match self.track {
                Track::Picture => {
                    decode::picture_setup(&open.input, open.stream, source, context.hardware)
                }
                Track::Sound => decode::sound_setup(&open.input, open.stream, source),
            };
            match setup {
                Ok(setup) => {
                    queue.put(Item::Switch(serial, Box::new(setup)));
                    self.sent = Some(source);
                }
                Err(error) => {
                    eprintln!("plaa: decoding part of an edit: {error}");
                    self.open = None;
                    blank(span);
                }
            }
        }
    }

    /// Opens file `source` of the edit, at the stream this lane plays:
    /// `own_stream` of the file edited, else its first.
    fn open_file(&self, source: usize, own_stream: usize, context: &Context) -> Option<Open> {
        let path = match context.edit.other(source) {
            Some(other) => other.path.clone(),
            None => context.main.to_path_buf(),
        };
        let input = match ffmpeg::format::input(&path) {
            Ok(input) => input,
            Err(error) => {
                eprintln!("plaa: opening {}: {error}", path.display());
                return None;
            }
        };
        let stream = if source == OWN {
            own_stream
        } else {
            let info = MediaInfo::read(&input);
            match self.track {
                Track::Picture => info.picture(None).map(|track| track.track.index),
                Track::Sound => info.audio.first().map(|track| track.track.index),
            }?
        };
        Some(Open {
            source,
            start: decode::start_seconds(&input),
            input,
            stream,
        })
    }
}
