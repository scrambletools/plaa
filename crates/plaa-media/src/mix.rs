//! Exporting an edit that mixes files: clips from other files, and
//! pictures or sound replaced. The picture and the sound are read apart,
//! each part from the file it is in, made the output's size and format,
//! and encoded one after another in the edited time; parts with nothing
//! to read are black, or silent.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use ffmpeg::format::Pixel;
use ffmpeg_next as ffmpeg;

use crate::edit::{Edit, OWN, Span, Track, Transform};
use crate::export::{
    self, ExportError, ExportEvent, Format, Job, Output, Sound, failed, picture_filters,
};
use crate::ffi;
use crate::info::{self, MediaInfo};

/// How much of a black or silent part is made at a time.
const BLANK_STEP: f64 = 0.5;

/// Runs `job`, whose edit mixes files, writing to `temporary`.
pub(crate) fn run(
    job: &Job,
    edit: &Edit,
    temporary: &Path,
    stop: &AtomicBool,
    events: &dyn Fn(ExportEvent),
) -> Result<(), ExportError> {
    let main = ffmpeg::format::input(&job.source).map_err(failed)?;
    let info = MediaInfo::read(&main);
    let sound_alone = match job.format {
        Format::Sound(sound) => Some(sound),
        _ => None,
    };
    let picture_stream = job.video.filter(|_| sound_alone.is_none()).filter(|index| {
        info.video
            .iter()
            .any(|track| track.track.index == *index && !track.cover)
    });
    let own_sound = job
        .audio
        .filter(|index| info.audio.iter().any(|track| track.track.index == *index));
    let lanes = edit.lanes((picture_stream.is_some(), own_sound.is_some()));
    // Sound from another file needs a sound stream, even where the file
    // edited has none.
    let other_sound = lanes
        .sound
        .iter()
        .filter_map(|span| span.source)
        .find(|source| *source != OWN);
    let sound_from = match (own_sound, other_sound) {
        (Some(index), _) => Some(Opened::Own(index)),
        (None, Some(source)) => edit
            .other(source)
            .map(|other| Opened::Other(other.path.clone())),
        (None, None) => None,
    };
    if picture_stream.is_none() && sound_from.is_none() {
        return Err(failed("nothing to export"));
    }

    let muxer = sound_alone.map_or_else(|| export::muxer(&job.destination), Sound::muxer);
    let mut context = ffmpeg::format::output_as(&temporary, muxer).map_err(failed)?;
    let global_header = context
        .format()
        .flags()
        .contains(ffmpeg::format::Flags::GLOBAL_HEADER);
    context.set_metadata(main.metadata().to_owned());

    let mut picture = match picture_stream {
        Some(index) => Some(PictureSink::new(
            &main,
            &info,
            index,
            job,
            edit.transform(),
            &mut context,
            global_header,
        )?),
        None => None,
    };
    let sound = sound_alone.unwrap_or(Sound::Aac);
    let quality = job
        .quality
        .filter(|_| sound_alone.is_some())
        .unwrap_or(match sound {
            Sound::Aac => 192,
            other => other.usual_quality(),
        });
    let mut sound_sink = match sound_from {
        Some(from) => Some(SoundSink::new(
            &from,
            &main,
            &info,
            (sound, quality),
            &mut context,
            global_header,
        )?),
        None => None,
    };
    let mut header = ffmpeg::Dictionary::new();
    header.set("movflags", "+faststart");
    context.write_header_with(header).map_err(failed)?;
    events(ExportEvent::Started(
        picture.as_ref().map(|sink| sink.chosen),
    ));
    let mut output = Output {
        context,
        packet: ffmpeg::Packet::empty(),
        written: 0,
    };

    let total = edit.duration();
    let mut readers: Vec<Reader> = Vec::new();
    if picture.is_some() {
        readers.push(Reader::new(Track::Picture, lanes.picture, picture_stream));
    }
    if sound_sink.is_some() {
        readers.push(Reader::new(Track::Sound, lanes.sound, own_sound));
    }
    let mut reported = Instant::now();
    // The reader behind in the edited time goes next, so the output's
    // picture and sound come interleaved.
    while let Some(reader) = readers
        .iter_mut()
        .filter(|reader| !reader.done)
        .min_by(|first, second| first.reached.total_cmp(&second.reached))
    {
        if stop.load(Ordering::Relaxed) {
            return Err(ExportError::Cancelled);
        }
        let mut sinks = Sinks {
            picture: picture.as_mut(),
            sound: sound_sink.as_mut(),
            output: &mut output,
        };
        reader.step(job, edit, &mut sinks)?;
        if reported.elapsed() >= Duration::from_millis(250) && total > 0.0 {
            reported = Instant::now();
            let done = readers
                .iter()
                .map(|reader| if reader.done { total } else { reader.reached })
                .fold(total, f64::min);
            events(ExportEvent::Progress((done / total).clamp(0.0, 1.0)));
        }
    }
    if let Some(sink) = &mut picture {
        sink.finish(&mut output)?;
    }
    if let Some(sink) = &mut sound_sink {
        sink.finish(&mut output)?;
    }
    output.context.write_trailer().map_err(failed)?;
    events(ExportEvent::Progress(1.0));
    Ok(())
}

/// Where the sound's encoder takes its rate and channels from.
enum Opened {
    /// Stream `index` of the file edited.
    Own(usize),
    /// Another file's first sound.
    Other(PathBuf),
}

/// Where what is read goes.
struct Sinks<'a> {
    picture: Option<&'a mut PictureSink>,
    sound: Option<&'a mut SoundSink>,
    output: &'a mut Output,
}

/// A file open for one of the readers, at the stream it reads.
struct Source {
    number: usize,
    input: ffmpeg::format::context::Input,
    start: f64,
    stream: usize,
    decoder: Decoder,
    time_base: ffmpeg::Rational,
    /// The picture's own quarter turns.
    turns: u8,
    frame_duration: f64,
}

enum Decoder {
    Picture(ffmpeg::decoder::Video),
    Sound(ffmpeg::decoder::Audio),
}

impl Decoder {
    fn send_packet(&mut self, packet: &ffmpeg::Packet) -> bool {
        match self {
            Decoder::Picture(decoder) => decoder.send_packet(packet).is_ok(),
            Decoder::Sound(decoder) => decoder.send_packet(packet).is_ok(),
        }
    }

    fn send_eof(&mut self) {
        let _ = match self {
            Decoder::Picture(decoder) => decoder.send_eof(),
            Decoder::Sound(decoder) => decoder.send_eof(),
        };
    }

    fn flush(&mut self) {
        match self {
            Decoder::Picture(decoder) => decoder.flush(),
            Decoder::Sound(decoder) => decoder.flush(),
        }
    }
}

/// What a reader does now.
enum Doing {
    /// Reading part `span` of its file.
    Reading(Span),
    /// Making black or silence, from `at` to `end` of the edited time.
    Blank { at: f64, end: f64 },
}

/// Reads the picture or the sound of the edit, part by part.
struct Reader {
    track: Track,
    spans: Vec<Span>,
    next: usize,
    doing: Option<Doing>,
    /// The stream of the file edited that plays.
    own_stream: Option<usize>,
    source: Option<Source>,
    packet: ffmpeg::Packet,
    /// The file's time of the last picture or sound, for those without.
    last: Option<f64>,
    /// The edited time read up to.
    reached: f64,
    done: bool,
}

impl Reader {
    fn new(track: Track, spans: Vec<Span>, own_stream: Option<usize>) -> Self {
        Self {
            track,
            spans,
            next: 0,
            doing: None,
            own_stream,
            source: None,
            packet: ffmpeg::Packet::empty(),
            last: None,
            reached: 0.0,
            done: false,
        }
    }

    /// Reads a packet, or makes a little black or silence, or starts the
    /// next part.
    fn step(&mut self, job: &Job, edit: &Edit, sinks: &mut Sinks) -> Result<(), ExportError> {
        match self.doing.take() {
            None => {
                let Some(span) = self.spans.get(self.next).copied() else {
                    self.done = true;
                    return Ok(());
                };
                self.next += 1;
                self.doing = Some(match span.source {
                    Some(number) if self.open(number, job, edit, span.from) => Doing::Reading(span),
                    // A file that will not open plays as nothing.
                    Some(_) => Doing::Blank {
                        at: span.from + span.shift,
                        end: span.to + span.shift,
                    },
                    None => Doing::Blank {
                        at: span.from,
                        end: span.to,
                    },
                });
            }
            Some(Doing::Blank { at, end }) => {
                let mut until = (at + BLANK_STEP).min(end);
                match self.track {
                    Track::Picture => {
                        if let Some(sink) = sinks.picture.as_deref_mut() {
                            // Whole frames at a time, so they keep one pace.
                            let frame = sink.frame_duration;
                            until = (at + (BLANK_STEP / frame).ceil() * frame).min(end);
                            sink.blank(at, until, sinks.output)?;
                        }
                    }
                    Track::Sound => {
                        if let Some(sink) = sinks.sound.as_deref_mut() {
                            sink.blank(at, until, sinks.output)?;
                        }
                    }
                }
                self.reached = self.reached.max(until);
                if until < end {
                    self.doing = Some(Doing::Blank { at: until, end });
                }
            }
            Some(Doing::Reading(span)) => {
                if !self.read(span, sinks)? {
                    self.doing = Some(Doing::Reading(span));
                } else {
                    // Sound its file lacked is silence, so what comes
                    // after keeps its place; a picture missing holds.
                    let read = self.last.unwrap_or(span.from).max(span.from);
                    if self.track == Track::Sound && read < span.to - 0.005 {
                        self.doing = Some(Doing::Blank {
                            at: read + span.shift,
                            end: span.to + span.shift,
                        });
                    } else {
                        self.reached = self.reached.max(span.to + span.shift);
                    }
                }
            }
        }
        Ok(())
    }

    /// Opens file `number` at its time `from`, or seeks the one open.
    /// Returns whether it can be read.
    fn open(&mut self, number: usize, job: &Job, edit: &Edit, from: f64) -> bool {
        if self
            .source
            .as_ref()
            .is_none_or(|source| source.number != number)
        {
            self.source = None;
            let path = match edit.other(number) {
                Some(other) => other.path.clone(),
                None => job.source.clone(),
            };
            match self.open_file(number, &path, job) {
                Ok(source) => self.source = Some(source),
                Err(error) => {
                    eprintln!("plaa: exporting from {}: {error}", path.display());
                    return false;
                }
            }
        }
        let Some(source) = &mut self.source else {
            return false;
        };
        source.decoder.flush();
        self.last = None;
        let timestamp = ((from + source.start) * f64::from(ffmpeg::ffi::AV_TIME_BASE)) as i64;
        source.input.seek(timestamp, ..timestamp).is_ok() || from <= 0.0
    }

    fn open_file(&self, number: usize, path: &Path, job: &Job) -> Result<Source, ExportError> {
        let mut input = ffmpeg::format::input(path).map_err(failed)?;
        let info = MediaInfo::read(&input);
        let own = (number == OWN).then_some(self.own_stream).flatten();
        let stream = match self.track {
            Track::Picture => own.or_else(|| info.picture(None).map(|track| track.track.index)),
            Track::Sound => own.or_else(|| info.audio.first().map(|track| track.track.index)),
        }
        .ok_or_else(|| failed("the file has nothing of this kind"))?;
        for index in 0..input.nb_streams() as usize {
            ffi::set_skipped(&mut input, index, index != stream);
        }
        let found = input
            .stream(stream)
            .ok_or_else(|| failed("the stream is gone"))?;
        let turns = ffi::quarter_turns(&found);
        let frame_duration = 1.0
            / Some(found.avg_frame_rate())
                .filter(|rate| rate.numerator() > 0 && rate.denominator() > 0)
                .map_or(25.0, f64::from);
        let (decoder, time_base) = match self.track {
            Track::Picture => {
                let (decoder, time_base, _) =
                    crate::decode::video_decoder(&input, stream, job.hardware).map_err(failed)?;
                (Decoder::Picture(decoder), time_base)
            }
            Track::Sound => {
                let (decoder, time_base) =
                    crate::decode::open_decoder(&input, stream).map_err(failed)?;
                (Decoder::Sound(decoder.audio().map_err(failed)?), time_base)
            }
        };
        Ok(Source {
            number,
            start: ffi::start_time(&input),
            input,
            stream,
            decoder,
            time_base,
            turns,
            frame_duration,
        })
    }

    /// Reads a packet of part `span`. Returns whether the part is done.
    fn read(&mut self, span: Span, sinks: &mut Sinks) -> Result<bool, ExportError> {
        let Some(source) = &mut self.source else {
            return Ok(true);
        };
        let ended = match self.packet.read(&mut source.input) {
            Ok(()) => {
                if self.packet.stream() != source.stream {
                    return Ok(false);
                }
                // A packet decoded at or past the part's end holds none of
                // it; by presentation time alone, a second past.
                let past = order(&source.input, &self.packet, source.start).is_some_and(
                    |(seconds, decoding)| seconds >= span.to + if decoding { 0.0 } else { 1.0 },
                );
                if !past {
                    if source.decoder.send_packet(&self.packet) {
                        self.receive(span, sinks)?;
                    }
                    return Ok(false);
                }
                true
            }
            Err(ffmpeg::Error::Eof) => true,
            Err(ffmpeg::Error::InvalidData) => return Ok(false),
            Err(error) => return Err(failed(error)),
        };
        if ended && let Some(source) = &mut self.source {
            source.decoder.send_eof();
            self.receive(span, sinks)?;
            if let Some(source) = &mut self.source {
                source.decoder.flush();
            }
        }
        Ok(ended)
    }

    /// Takes what the decoder has of part `span` to the sinks.
    fn receive(&mut self, span: Span, sinks: &mut Sinks) -> Result<(), ExportError> {
        let Some(source) = &mut self.source else {
            return Ok(());
        };
        match &mut source.decoder {
            Decoder::Picture(decoder) => {
                let mut decoded = ffmpeg::frame::Video::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let at = decoded
                        .timestamp()
                        .or(decoded.pts())
                        .and_then(|timestamp| info::seconds(timestamp, source.time_base))
                        .map(|seconds| seconds - source.start)
                        .or_else(|| self.last.map(|last| last + source.frame_duration))
                        .unwrap_or(0.0);
                    self.last = Some(at);
                    let middle = at + source.frame_duration / 2.0;
                    if middle < span.from || middle >= span.to {
                        continue;
                    }
                    let edited = at + span.shift;
                    let frame = std::mem::replace(&mut decoded, ffmpeg::frame::Video::empty());
                    if let Some(sink) = sinks.picture.as_deref_mut() {
                        let kind = if source.number == OWN {
                            Kind::Own
                        } else {
                            Kind::Other(source.turns)
                        };
                        sink.push(frame, edited, kind, sinks.output)?;
                    }
                    self.reached = self.reached.max(edited);
                }
            }
            Decoder::Sound(decoder) => {
                let mut decoded = ffmpeg::frame::Audio::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let rate = f64::from(decoded.rate().max(1));
                    let samples = decoded.samples();
                    let at = decoded
                        .timestamp()
                        .or(decoded.pts())
                        .and_then(|timestamp| info::seconds(timestamp, source.time_base))
                        .map(|seconds| seconds - source.start)
                        .or(self.last)
                        .unwrap_or(0.0);
                    let end = at + samples as f64 / rate;
                    self.last = Some(end);
                    if end <= span.from || at >= span.to {
                        continue;
                    }
                    let skip = if at < span.from {
                        (((span.from - at) * rate).round() as usize).min(samples)
                    } else {
                        0
                    };
                    let until = if end > span.to {
                        (((span.to - at) * rate).round() as usize).min(samples)
                    } else {
                        samples
                    };
                    if until <= skip {
                        continue;
                    }
                    let mut frame = if skip > 0 || until < samples {
                        ffi::audio_slice(&decoded, skip, until - skip)
                            .ok_or_else(|| failed("cutting sound"))?
                    } else {
                        decoded.clone()
                    };
                    let edited = at + skip as f64 / rate + span.shift;
                    frame.set_pts(Some((edited * rate).round() as i64));
                    if let Some(sink) = sinks.sound.as_deref_mut() {
                        sink.push(&frame, sinks.output)?;
                    }
                    self.reached = self.reached.max(edited + (until - skip) as f64 / rate);
                }
            }
        }
        Ok(())
    }
}

/// Where `packet` comes in its file's time, by its decoding time where it
/// has one, else its presentation time; and which.
fn order(
    input: &ffmpeg::format::context::Input,
    packet: &ffmpeg::Packet,
    start: f64,
) -> Option<(f64, bool)> {
    let time_base = input.stream(packet.stream())?.time_base();
    let (time, decoding) = match packet.dts() {
        Some(dts) => (dts, true),
        None => (packet.pts()?, false),
    };
    Some((info::seconds(time, time_base)? - start, decoding))
}

/// Whose picture a frame is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// The file edited's, sized as the output is.
    Own,
    /// Another file's, with its own quarter turns: fitted in the output.
    Other(u8),
    /// Black, the output's size.
    Black,
}

/// What a picture graph was built for: the pictures' size, format and
/// pixel shape, and whose.
type GraphKey = (u32, u32, Pixel, (i32, i32), Kind);

/// Turns pictures from every file into the encoder's, and encodes them.
struct PictureSink {
    chosen: export::Encoder,
    encoder: export::VideoEncoder,
    output: usize,
    size: (u32, u32),
    transform: Transform,
    /// How the file edited's pictures are turned.
    shown: Transform,
    frame_duration: f64,
    graph: Option<(GraphKey, ffmpeg::filter::Graph)>,
    downloads: ffi::Downloads,
    /// The last picture's time, in the encoder's time base.
    last_pts: Option<i64>,
}

impl PictureSink {
    fn new(
        input: &ffmpeg::format::context::Input,
        info: &MediaInfo,
        index: usize,
        job: &Job,
        transform: Transform,
        context: &mut ffmpeg::format::context::Output,
        global_header: bool,
    ) -> Result<Self, ExportError> {
        let out = export::picture_out(input, info, index, job, transform, context, global_header)?;
        Ok(Self {
            chosen: out.chosen,
            encoder: out.encoder,
            output: out.output,
            size: out.size,
            transform,
            shown: out.shown,
            frame_duration: out.frame_duration,
            graph: None,
            downloads: ffi::Downloads::default(),
            last_pts: None,
        })
    }

    /// Encodes `frame` at `edited` seconds of the edited time.
    fn push(
        &mut self,
        frame: ffmpeg::frame::Video,
        edited: f64,
        kind: Kind,
        output: &mut Output,
    ) -> Result<(), ExportError> {
        let mut frame = if ffi::is_hardware(&frame) {
            self.downloads.download(&frame).map_err(failed)?
        } else {
            frame
        };
        let base = self.encoder.encoder.time_base();
        let pts = (edited / f64::from(base)).round() as i64;
        // Where parts meet, a picture at or before the last one is left
        // out.
        if self.last_pts.is_some_and(|last| pts <= last) {
            return Ok(());
        }
        self.last_pts = Some(pts);
        frame.set_pts(Some(pts));
        let sample = frame.aspect_ratio();
        let sample = if sample.numerator() > 0 && sample.denominator() > 0 {
            (sample.numerator(), sample.denominator())
        } else {
            (1, 1)
        };
        let key = (frame.width(), frame.height(), frame.format(), sample, kind);
        if self.graph.as_ref().is_none_or(|(built, _)| *built != key) {
            // What the last graph holds goes first.
            self.flush(output)?;
            self.graph = Some((key, self.build(key)?));
        }
        if let Some((_, graph)) = &mut self.graph
            && let Some(mut source) = graph.get("in")
        {
            source.source().add(&frame).map_err(failed)?;
        }
        self.pull(output)
    }

    /// Encodes black from `from` to `to` seconds of the edited time.
    fn blank(&mut self, from: f64, to: f64, output: &mut Output) -> Result<(), ExportError> {
        let (width, height) = self.size;
        let count = ((to - from) / self.frame_duration - 1e-6).ceil().max(0.0) as usize;
        for index in 0..count {
            let mut frame = ffmpeg::frame::Video::new(Pixel::YUV420P, width, height);
            for (plane, value) in [(0, 16), (1, 128), (2, 128)] {
                frame.data_mut(plane).fill(value);
            }
            self.push(
                frame,
                from + index as f64 * self.frame_duration,
                Kind::Black,
                output,
            )?;
        }
        Ok(())
    }

    /// The graph for pictures as `key` says.
    fn build(&self, key: GraphKey) -> Result<ffmpeg::filter::Graph, ExportError> {
        let (width, height, format, (sample_across, sample_down), kind) = key;
        let base = self.encoder.encoder.time_base();
        let arguments = format!(
            "video_size={width}x{height}:pix_fmt={}:time_base={}/{}:pixel_aspect={sample_across}/{sample_down}",
            format
                .descriptor()
                .map_or("yuv420p", |descriptor| descriptor.name()),
            base.numerator(),
            base.denominator().max(1),
        );
        let filters = match kind {
            Kind::Own => picture_filters(self.shown, self.size, self.encoder.format),
            Kind::Black => picture_filters(Transform::default(), self.size, self.encoder.format),
            Kind::Other(turns) => fit_filters(
                self.transform.after_turns(turns),
                (sample_across, sample_down),
                self.size,
                self.encoder.format,
            ),
        };
        let mut graph = ffmpeg::filter::Graph::new();
        let buffer = ffmpeg::filter::find("buffer").ok_or_else(|| failed("no buffer filter"))?;
        let sink =
            ffmpeg::filter::find("buffersink").ok_or_else(|| failed("no buffersink filter"))?;
        graph.add(&buffer, "in", &arguments).map_err(failed)?;
        graph.add(&sink, "out", "").map_err(failed)?;
        graph
            .output("in", 0)
            .and_then(|parser| parser.input("out", 0))
            .and_then(|parser| parser.parse(&filters))
            .map_err(failed)?;
        graph.validate().map_err(failed)?;
        Ok(graph)
    }

    /// Encodes what the graph has ready.
    fn pull(&mut self, output: &mut Output) -> Result<(), ExportError> {
        let Some((_, graph)) = &mut self.graph else {
            return Ok(());
        };
        let base = self.encoder.encoder.time_base();
        let duration = (self.frame_duration / f64::from(base)).round().max(1.0) as i64;
        let mut filtered = ffmpeg::frame::Video::empty();
        while let Some(mut sink) = graph.get("out")
            && sink.sink().frame(&mut filtered).is_ok()
        {
            let frame = match &self.encoder.frames {
                Some(frames) => frames
                    .upload(&filtered)
                    .ok_or_else(|| failed("copying a picture to the graphics card"))?,
                None => std::mem::replace(&mut filtered, ffmpeg::frame::Video::empty()),
            };
            self.encoder.encoder.send_frame(&frame).map_err(failed)?;
            output.drain(&mut self.encoder.encoder, self.output, base, duration)?;
        }
        Ok(())
    }

    /// Lets the pictures still in the graph out.
    fn flush(&mut self, output: &mut Output) -> Result<(), ExportError> {
        if let Some((_, graph)) = &mut self.graph
            && let Some(mut source) = graph.get("in")
        {
            let _ = source.source().flush();
        }
        self.pull(output)
    }

    fn finish(&mut self, output: &mut Output) -> Result<(), ExportError> {
        self.flush(output)?;
        let _ = self.encoder.encoder.send_eof();
        let base = self.encoder.encoder.time_base();
        let duration = (self.frame_duration / f64::from(base)).round().max(1.0) as i64;
        output.drain(&mut self.encoder.encoder, self.output, base, duration)
    }
}

/// The filters that turn and flip another file's pictures, then fit them
/// in the output's `size`, black around them, for an encoder taking
/// `format`.
fn fit_filters(
    transform: Transform,
    (sample_across, sample_down): (i32, i32),
    (width, height): (u32, u32),
    format: Pixel,
) -> String {
    let mut filters = Vec::new();
    // Square pixels first, so the fit keeps the picture's shape.
    if sample_across != sample_down {
        filters.push(format!(
            "scale=trunc(iw*{sample_across}/{sample_down}/2)*2:ih,setsar=1"
        ));
    }
    if transform.mirrored {
        filters.push("hflip".to_owned());
    }
    match transform.turns % 4 {
        1 => filters.push("transpose=clock".to_owned()),
        2 => filters.push("hflip,vflip".to_owned()),
        3 => filters.push("transpose=cclock".to_owned()),
        _ => {}
    }
    filters.push(format!(
        "scale={width}:{height}:force_original_aspect_ratio=decrease:force_divisible_by=2:flags=bicubic"
    ));
    filters.push(format!("pad={width}:{height}:(ow-iw)/2:(oh-ih)/2"));
    filters.push(format!(
        "format={}",
        format
            .descriptor()
            .map_or("yuv420p", |descriptor| descriptor.name())
    ));
    filters.push("setsar=1".to_owned());
    filters.join(",")
}

/// What a sound converter was built for: the sound's rate, format and
/// channels.
type SoundKey = (u32, ffmpeg::format::Sample, String);

/// Turns sound from every file into the encoder's, fills what is missing
/// with silence, and encodes it.
struct SoundSink {
    encoder: ffmpeg::encoder::audio::Encoder,
    output: usize,
    rate: u32,
    layout: String,
    channels: u16,
    /// Makes one file's sound the mix's rate, planar floats, and channels.
    converter: Option<(SoundKey, ffmpeg::filter::Graph)>,
    /// Cuts the sound in the encoder's frames.
    graph: ffmpeg::filter::Graph,
    /// The edited time sound has been placed up to, in samples.
    next: i64,
}

impl SoundSink {
    fn new(
        from: &Opened,
        main: &ffmpeg::format::context::Input,
        info: &MediaInfo,
        (sound, quality): (Sound, u32),
        context: &mut ffmpeg::format::context::Output,
        global_header: bool,
    ) -> Result<Self, ExportError> {
        let (rate, channels, metadata) = match from {
            Opened::Own(index) => {
                let track = info
                    .audio
                    .iter()
                    .find(|track| track.track.index == *index)
                    .ok_or_else(|| failed("the sound's stream is gone"))?;
                let metadata = main
                    .stream(*index)
                    .map(|stream| stream.metadata().to_owned());
                (track.sample_rate, track.channels, metadata)
            }
            Opened::Other(path) => {
                let input = ffmpeg::format::input(path).map_err(failed)?;
                let other = MediaInfo::read(&input);
                let track = other
                    .audio
                    .first()
                    .ok_or_else(|| failed("the file has no sound"))?;
                (track.sample_rate, track.channels, None)
            }
        };
        let (codec, encoder) = export::sound_encoder(
            (sound, quality),
            rate.max(8000),
            channels.max(1),
            global_header,
        )?;
        let mut out = context.add_stream(codec).map_err(failed)?;
        out.set_parameters(&encoder);
        out.set_time_base(ffmpeg::Rational::new(1, encoder.rate() as i32));
        if let Some(metadata) = metadata {
            out.set_metadata(metadata);
        }
        let output = out.index();
        let channels = encoder.channels();
        let layout = ffi::default_layout(i32::from(channels));
        let rate = encoder.rate();
        let planar = ffmpeg::format::Sample::F32(ffmpeg::format::sample::Type::Planar);
        let graph = export::sound_graph((rate, planar, &layout), &encoder, false)?;
        Ok(Self {
            encoder,
            output,
            rate,
            layout,
            channels,
            converter: None,
            graph,
            next: 0,
        })
    }

    /// Encodes `frame`, stamped in its rate's samples of the edited time.
    fn push(
        &mut self,
        frame: &ffmpeg::frame::Audio,
        output: &mut Output,
    ) -> Result<(), ExportError> {
        let key = (frame.rate(), frame.format(), ffi::frame_layout(frame));
        if self
            .converter
            .as_ref()
            .is_none_or(|(built, _)| *built != key)
        {
            self.flush_converter(output)?;
            self.converter = Some((key.clone(), self.converter_for(&key)?));
        }
        if let Some((_, converter)) = &mut self.converter
            && let Some(mut source) = converter.get("in")
        {
            source.source().add(frame).map_err(failed)?;
        }
        self.pull_converter(output)
    }

    /// Encodes silence from `from` to `to` seconds of the edited time.
    fn blank(&mut self, from: f64, to: f64, output: &mut Output) -> Result<(), ExportError> {
        let rate = f64::from(self.rate);
        let first = ((from * rate).round() as i64).max(self.next);
        self.silence(first, (to * rate).round() as i64, output)
    }

    /// Encodes silence up to sample `to` of the edited time, from where
    /// the sound is.
    fn silence(&mut self, from: i64, to: i64, output: &mut Output) -> Result<(), ExportError> {
        let mut at = from.max(self.next);
        while at < to {
            let samples = (to - at).min(1024) as usize;
            let mut frame = ffmpeg::frame::Audio::new(
                ffmpeg::format::Sample::F32(ffmpeg::format::sample::Type::Planar),
                samples,
                ffmpeg::ChannelLayout::default(i32::from(self.channels)),
            );
            frame.set_rate(self.rate);
            for plane in 0..frame.planes() {
                frame.data_mut(plane).fill(0);
            }
            frame.set_pts(Some(at));
            self.encode(&frame, output)?;
            at += samples as i64;
        }
        Ok(())
    }

    /// Places `frame`, of the mix's format, at its time: silence before
    /// it where sound is missing, and what overlaps the sound before it
    /// left out, so sound never drifts from the picture.
    fn place(
        &mut self,
        frame: &ffmpeg::frame::Audio,
        output: &mut Output,
    ) -> Result<(), ExportError> {
        let samples = frame.samples() as i64;
        let at = frame.pts().unwrap_or(self.next);
        if at > self.next {
            self.silence(self.next, at, output)?;
        }
        let skip = self.next - at;
        if skip >= samples {
            return Ok(());
        }
        let mut frame = if skip > 0 {
            ffi::audio_slice(frame, skip as usize, (samples - skip) as usize)
                .ok_or_else(|| failed("cutting sound"))?
        } else {
            frame.clone()
        };
        frame.set_pts(Some(self.next));
        self.encode(&frame, output)
    }

    /// Hands `frame`, placed, to the graph and the encoder.
    fn encode(
        &mut self,
        frame: &ffmpeg::frame::Audio,
        output: &mut Output,
    ) -> Result<(), ExportError> {
        self.next = frame.pts().unwrap_or(self.next) + frame.samples() as i64;
        if let Some(mut source) = self.graph.get("in") {
            source.source().add(frame).map_err(failed)?;
        }
        self.pull(output)
    }

    /// A graph that makes sound as `key` says the mix's.
    fn converter_for(&self, key: &SoundKey) -> Result<ffmpeg::filter::Graph, ExportError> {
        let (rate, format, layout) = key;
        let mut graph = ffmpeg::filter::Graph::new();
        let arguments = format!(
            "time_base=1/{rate}:sample_rate={rate}:sample_fmt={}:channel_layout={layout}",
            format.name(),
        );
        let abuffer = ffmpeg::filter::find("abuffer").ok_or_else(|| failed("no abuffer filter"))?;
        let sink =
            ffmpeg::filter::find("abuffersink").ok_or_else(|| failed("no abuffersink filter"))?;
        graph.add(&abuffer, "in", &arguments).map_err(failed)?;
        graph.add(&sink, "out", "").map_err(failed)?;
        let filters = format!(
            "aresample={},aformat=sample_fmts=fltp:channel_layouts={}",
            self.rate, self.layout
        );
        graph
            .output("in", 0)
            .and_then(|parser| parser.input("out", 0))
            .and_then(|parser| parser.parse(&filters))
            .map_err(failed)?;
        graph.validate().map_err(failed)?;
        Ok(graph)
    }

    /// Hands what the converter has ready on.
    fn pull_converter(&mut self, output: &mut Output) -> Result<(), ExportError> {
        let Some((_, converter)) = &mut self.converter else {
            return Ok(());
        };
        let mut ready = Vec::new();
        let mut converted = ffmpeg::frame::Audio::empty();
        while let Some(mut sink) = converter.get("out")
            && sink.sink().frame(&mut converted).is_ok()
        {
            ready.push(std::mem::replace(
                &mut converted,
                ffmpeg::frame::Audio::empty(),
            ));
        }
        for frame in ready {
            self.place(&frame, output)?;
        }
        Ok(())
    }

    fn flush_converter(&mut self, output: &mut Output) -> Result<(), ExportError> {
        if let Some((_, converter)) = &mut self.converter
            && let Some(mut source) = converter.get("in")
        {
            let _ = source.source().flush();
        }
        self.pull_converter(output)?;
        self.converter = None;
        Ok(())
    }

    /// Encodes what the main graph has ready.
    fn pull(&mut self, output: &mut Output) -> Result<(), ExportError> {
        let mut filtered = ffmpeg::frame::Audio::empty();
        while let Some(mut sink) = self.graph.get("out")
            && sink.sink().frame(&mut filtered).is_ok()
        {
            self.encoder.send_frame(&filtered).map_err(failed)?;
            let base = self.encoder.time_base();
            output.drain(&mut self.encoder, self.output, base, 0)?;
        }
        Ok(())
    }

    fn finish(&mut self, output: &mut Output) -> Result<(), ExportError> {
        self.flush_converter(output)?;
        if let Some(mut source) = self.graph.get("in") {
            let _ = source.source().flush();
        }
        self.pull(output)?;
        let _ = self.encoder.send_eof();
        let base = self.encoder.time_base();
        output.drain(&mut self.encoder, self.output, base, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn other_pictures_fit_in_the_output() {
        assert_eq!(
            fit_filters(Transform::default(), (1, 1), (1280, 720), Pixel::YUV420P),
            "scale=1280:720:force_original_aspect_ratio=decrease:force_divisible_by=2:\
             flags=bicubic,pad=1280:720:(ow-iw)/2:(oh-ih)/2,format=yuv420p,setsar=1"
        );
        let turned = Transform {
            turns: 1,
            mirrored: false,
        };
        assert!(
            fit_filters(turned, (4, 3), (720, 1280), Pixel::NV12)
                .starts_with("scale=trunc(iw*4/3/2)*2:ih,setsar=1,transpose=clock,")
        );
    }
}
