//! The decoder threads: one for the picture and one for the sound, each
//! taking packets from its queue, plus subtitles and cover art, which are
//! small enough to decode as they are read.

use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::thread::JoinHandle;
use std::time::Duration;

use ffmpeg::format::Pixel;
use ffmpeg::software::scaling;
use ffmpeg_next as ffmpeg;

use crate::audio;
use crate::edit::Span;
use crate::ffi;
use crate::frame::{self, VideoFrame};
use crate::info::{self, MediaInfo};
use crate::player::{Quality, Shared};
use crate::queue::{Closed, Item, PacketQueue};
use crate::subtitle::{self, Cue, PictureCue, SubtitlePicture};

/// A still picture as RGBA, such as cover art.
#[derive(Clone, PartialEq, Eq)]
pub struct Picture {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

impl Picture {
    /// The picture mirrored and turned as `transform` says.
    pub fn transformed(self, transform: crate::edit::Transform) -> Picture {
        let turns = transform.turns % 4;
        if turns == 0 && !transform.mirrored {
            return self;
        }
        let (width, height) = (self.width as usize, self.height as usize);
        let (out_width, out_height) = if turns % 2 == 1 {
            (height, width)
        } else {
            (width, height)
        };
        let mut rgba = vec![0; self.rgba.len()];
        for y in 0..height {
            for x in 0..width {
                let mirrored = if transform.mirrored { width - 1 - x } else { x };
                let (to_x, to_y) = match turns {
                    1 => (height - 1 - y, mirrored),
                    2 => (width - 1 - mirrored, height - 1 - y),
                    3 => (y, width - 1 - mirrored),
                    _ => (mirrored, y),
                };
                let from = (y * width + x) * 4;
                let to = (to_y * out_width + to_x) * 4;
                rgba[to..to + 4].copy_from_slice(&self.rgba[from..from + 4]);
            }
        }
        Picture {
            width: out_width as u32,
            height: out_height as u32,
            rgba,
        }
    }
}

impl std::fmt::Debug for Picture {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Picture({}x{})", self.width, self.height)
    }
}

/// How long a decoder waits for a packet before looking at the stop flag.
const WAIT: Duration = Duration::from_millis(50);

pub(crate) fn start_seconds(input: &ffmpeg::format::context::Input) -> f64 {
    ffi::start_time(input)
}

/// The decoder for stream `index`, its packets read in `time_base`.
pub(crate) fn open_decoder(
    input: &ffmpeg::format::context::Input,
    index: usize,
) -> Result<(ffmpeg::codec::decoder::Decoder, ffmpeg::Rational), ffmpeg::Error> {
    open_decoder_with(input, index, None)
}

/// The decoder for stream `index`, by the name of FFmpeg's decoder to use
/// rather than its first for the codec.
fn open_decoder_with(
    input: &ffmpeg::format::context::Input,
    index: usize,
    name: Option<&str>,
) -> Result<(ffmpeg::codec::decoder::Decoder, ffmpeg::Rational), ffmpeg::Error> {
    let stream = input.stream(index).ok_or(ffmpeg::Error::StreamNotFound)?;
    let time_base = stream.time_base();
    let mut context = match name.and_then(ffmpeg::codec::decoder::find_by_name) {
        Some(codec) => {
            let mut context = ffmpeg::codec::Context::new_with_codec(codec);
            context.set_parameters(stream.parameters())?;
            context
        }
        None => ffmpeg::codec::Context::from_parameters(stream.parameters())?,
    };
    // Every core the system has, which FFmpeg picks with a count of 0.
    context.set_threading(ffmpeg::codec::threading::Config::kind(
        ffmpeg::codec::threading::Type::Frame,
    ));
    let mut decoder = context.decoder();
    decoder.set_packet_time_base(time_base);
    Ok((decoder, time_base))
}

/// Starts decoding the picture of stream `index`. Returns its packet
/// queue, its thread, and the graphics card API it decodes on, if any.
/// A started decoder: its packet queue and its thread.
pub(crate) type Started = (Arc<PacketQueue>, JoinHandle<()>);

pub(crate) fn spawn_video(
    input: &ffmpeg::format::context::Input,
    index: usize,
    start: f64,
    hardware: bool,
    shared: Arc<Shared>,
) -> Result<(Started, Option<&'static str>), ffmpeg::Error> {
    let stream = input.stream(index).ok_or(ffmpeg::Error::StreamNotFound)?;
    let quarter_turns = ffi::quarter_turns(&stream);
    let frame_rate = Some(stream.avg_frame_rate())
        .filter(|rate| rate.numerator() > 0 && rate.denominator() > 0)
        .map(f64::from)
        .unwrap_or(25.0);
    let (decoder, time_base, device) = video_decoder(input, index, hardware)?;
    let queue = Arc::new(PacketQueue::default());
    let mut worker = VideoDecoder {
        decoder,
        time_base,
        start,
        frame_duration: 1.0 / frame_rate,
        quarter_turns,
        last_position: None,
        shared,
        queue: queue.clone(),
        serial: 0,
        span: Span::WHOLE,
        needs_flush: false,
        downloads: ffi::Downloads::default(),
    };
    let thread = std::thread::Builder::new()
        .name("plaa-video".into())
        .spawn(move || worker.run())
        .map_err(|_| ffmpeg::Error::External)?;
    Ok(((queue, thread), device))
}

/// The picture decoder for stream `index`, on a graphics card when
/// `hardware` and one opens: the decoder, the time base of its packets,
/// and the card's API.
pub(crate) fn video_decoder(
    input: &ffmpeg::format::context::Input,
    index: usize,
    hardware: bool,
) -> Result<
    (
        ffmpeg::decoder::Video,
        ffmpeg::Rational,
        Option<&'static str>,
    ),
    ffmpeg::Error,
> {
    let stream = input.stream(index).ok_or(ffmpeg::Error::StreamNotFound)?;
    let (mut decoder, time_base) = open_decoder(input, index)?;
    let mut device = None;
    if hardware {
        device = ffi::attach_hardware(&mut decoder.0);
        // FFmpeg's first AV1 decoder, dav1d, decodes on the processor
        // alone; its own decodes on graphics cards, and nothing else.
        if device.is_none()
            && stream.parameters().id() == ffmpeg::codec::Id::AV1
            && let Ok((mut native, _)) = open_decoder_with(input, index, Some("av1"))
            && let Some(found) = ffi::attach_hardware(&mut native.0)
        {
            decoder = native;
            device = Some(found);
        }
    }
    let decoder = match decoder.video() {
        Ok(decoder) => decoder,
        // A card that will not open this stream: decode on the processor.
        Err(_) if device.is_some() => {
            device = None;
            open_decoder(input, index)?.0.video()?
        }
        Err(error) => return Err(error),
    };
    Ok((decoder, time_base, device))
}

struct VideoDecoder {
    decoder: ffmpeg::decoder::Video,
    time_base: ffmpeg::Rational,
    start: f64,
    frame_duration: f64,
    quarter_turns: u8,
    last_position: Option<f64>,
    shared: Arc<Shared>,
    queue: Arc<PacketQueue>,
    serial: u64,
    /// The part of the file the packets decoding are from.
    span: Span,
    /// The decoder was drained at the end of the file and must be reset
    /// before taking packets again.
    needs_flush: bool,
    /// Memory for pictures copied out of graphics memory.
    downloads: ffi::Downloads,
}

/// Converts pictures the shader cannot read: their format and size, and
/// the context for them. FFmpeg's scaler stays on the thread that made it.
type Scaler = Option<(Pixel, u32, u32, scaling::Context)>;

impl VideoDecoder {
    fn run(&mut self) {
        let mut scaler: Scaler = None;
        loop {
            let item = match self.queue.get(WAIT) {
                Err(Closed) => return,
                Ok(None) if self.shared.stopping() => return,
                Ok(None) => continue,
                Ok(Some(item)) => item,
            };
            match item {
                Item::Packet(serial, span, packet) => {
                    self.renew(serial, span);
                    if serial != self.shared.serial.load(Ordering::Relaxed) {
                        continue;
                    }
                    if span.index != self.span.index {
                        // The last part's pictures still in the decoder
                        // come out before the next part's go in.
                        let _ = self.decoder.send_eof();
                        self.drain(&mut scaler);
                        self.decoder.flush();
                        self.last_position = None;
                        self.span = span;
                    }
                    if self.decoder.send_packet(&packet).is_err() {
                        continue;
                    }
                    self.drain(&mut scaler);
                }
                Item::End(serial) => {
                    self.renew(serial, self.span);
                    let _ = self.decoder.send_eof();
                    self.drain(&mut scaler);
                    self.needs_flush = true;
                    let mut video = self.shared.video();
                    if video.serial() == serial {
                        video.finished = true;
                    }
                }
            }
        }
    }

    /// Resets the decoder for packets of a new serial, after a seek, from
    /// `span`.
    fn renew(&mut self, serial: u64, span: Span) {
        if serial != self.serial || self.needs_flush {
            self.decoder.flush();
            self.serial = serial;
            self.span = span;
            self.needs_flush = false;
            self.last_position = None;
        }
    }

    fn drain(&mut self, scaler: &mut Scaler) {
        let mut decoded = ffmpeg::frame::Video::empty();
        while self.decoder.receive_frame(&mut decoded).is_ok() {
            let frame = std::mem::replace(&mut decoded, ffmpeg::frame::Video::empty());
            self.deliver(frame, scaler);
            if self.shared.stopping() {
                return;
            }
        }
    }

    fn deliver(&mut self, frame: ffmpeg::frame::Video, scaler: &mut Scaler) {
        let serial = self.serial;
        if serial != self.shared.serial.load(Ordering::Relaxed) {
            return;
        }
        let source = frame
            .timestamp()
            .or(frame.pts())
            .and_then(|timestamp| info::seconds(timestamp, self.time_base))
            .map(|seconds| seconds - self.start)
            .or_else(|| self.last_position.map(|last| last + self.frame_duration))
            .unwrap_or(0.0);
        self.last_position = Some(source);
        // A picture belongs to the part its middle shows in.
        let middle = source + self.frame_duration / 2.0;
        if middle < self.span.from || middle >= self.span.to {
            return;
        }
        let position = source + self.span.shift;
        *self
            .shared
            .decoded
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = (serial, position);
        if let Some(target) = self.shared.target(serial)
            && position + self.frame_duration / 2.0 < target
        {
            return;
        }
        let frame = if ffi::is_hardware(&frame) {
            match self.downloads.download(&frame) {
                Ok(frame) => frame,
                Err(error) => {
                    eprintln!("plaa: copying a picture from the graphics card: {error}");
                    return;
                }
            }
        } else {
            frame
        };
        let aspect = display_aspect(&frame);
        let picture = match VideoFrame::wrap(frame, serial, position, aspect, self.quarter_turns) {
            Ok(picture) => picture,
            Err(frame) => match convert(scaler, &frame) {
                Some(converted) => {
                    match VideoFrame::wrap(converted, serial, position, aspect, self.quarter_turns)
                    {
                        Ok(picture) => picture,
                        Err(_) => return,
                    }
                }
                None => return,
            },
        };
        let mut video = self.shared.video();
        while video.is_full() && video.serial() == serial && !self.shared.stopping() {
            video = self
                .shared
                .video_room
                .wait_timeout(video, Duration::from_millis(20))
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .0;
        }
        video.push(picture);
    }
}

/// Converts a picture in a format the shader does not read.
fn convert(scaler: &mut Scaler, frame: &ffmpeg::frame::Video) -> Option<ffmpeg::frame::Video> {
    let (format, width, height) = (frame.format(), frame.width(), frame.height());
    let reusable = matches!(
        &*scaler,
        Some((cached, cached_width, cached_height, _))
            if *cached == format && *cached_width == width && *cached_height == height
    );
    if !reusable {
        let context = scaling::Context::get(
            format,
            width,
            height,
            frame::conversion_target(format),
            width,
            height,
            scaling::Flags::BILINEAR,
        )
        .ok()?;
        *scaler = Some((format, width, height, context));
    }
    let (_, _, _, context) = scaler.as_mut()?;
    let mut out = ffmpeg::frame::Video::empty();
    context.run(frame, &mut out).ok()?;
    out.set_pts(frame.pts());
    out.set_color_range(frame.color_range());
    out.set_color_space(frame.color_space());
    out.set_color_primaries(frame.color_primaries());
    out.set_color_transfer_characteristic(frame.color_transfer_characteristic());
    Some(out)
}

/// Width over height as shown, with the pixels' shape.
fn display_aspect(frame: &ffmpeg::frame::Video) -> f64 {
    let sample = frame.aspect_ratio();
    let sample = if sample.numerator() > 0 && sample.denominator() > 0 {
        f64::from(sample)
    } else {
        1.0
    };
    if frame.height() == 0 {
        return 1.0;
    }
    f64::from(frame.width()) * sample / f64::from(frame.height())
}

/// Starts decoding the sound of stream `index` into `output`.
pub(crate) fn spawn_audio(
    input: &ffmpeg::format::context::Input,
    index: usize,
    start: f64,
    output: Arc<audio::Shared>,
    shared: Arc<Shared>,
) -> Result<Started, ffmpeg::Error> {
    let (decoder, time_base) = open_decoder(input, index)?;
    let decoder = decoder.audio()?;
    let queue = Arc::new(PacketQueue::default());
    let mut worker = AudioDecoder {
        decoder,
        time_base,
        start,
        output,
        shared,
        queue: queue.clone(),
        serial: 0,
        span: Span::WHOLE,
        needs_flush: false,
        graph: None,
        cursor: None,
    };
    let thread = std::thread::Builder::new()
        .name("plaa-audio".into())
        .spawn(move || worker.run())
        .map_err(|_| ffmpeg::Error::External)?;
    Ok((queue, thread))
}

/// The filter graph sound goes through: tempo, then the output's format.
struct AudioGraph {
    graph: ffmpeg::filter::Graph,
    /// What it was built for: the input's format, rate and channels, and
    /// the tempo.
    key: (ffmpeg::format::Sample, u32, u16, u64),
    tempo: f64,
}

struct AudioDecoder {
    decoder: ffmpeg::decoder::Audio,
    time_base: ffmpeg::Rational,
    start: f64,
    output: Arc<audio::Shared>,
    shared: Arc<Shared>,
    queue: Arc<PacketQueue>,
    serial: u64,
    /// The part of the file the sound in the decoder and graph is from.
    span: Span,
    needs_flush: bool,
    graph: Option<AudioGraph>,
    /// The file's time of the next sample out of the graph.
    cursor: Option<f64>,
}

impl AudioDecoder {
    fn run(&mut self) {
        loop {
            let item = match self.queue.get(WAIT) {
                Err(Closed) => return,
                Ok(None) if self.shared.stopping() => return,
                Ok(None) => continue,
                Ok(Some(item)) => item,
            };
            match item {
                Item::Packet(serial, span, packet) => {
                    self.renew(serial, span);
                    if serial != self.shared.serial.load(Ordering::Relaxed) {
                        continue;
                    }
                    if span.index != self.span.index {
                        // The last part's sound plays out to its end
                        // before the next part's starts.
                        self.finish_part();
                        self.decoder.flush();
                        self.graph = None;
                        self.cursor = None;
                        self.span = span;
                    }
                    if self.decoder.send_packet(&packet).is_err() {
                        continue;
                    }
                    self.drain();
                }
                Item::End(serial) => {
                    self.renew(serial, self.span);
                    self.finish_part();
                    self.needs_flush = true;
                    self.output.finish(serial);
                }
            }
        }
    }

    fn renew(&mut self, serial: u64, span: Span) {
        if serial != self.serial || self.needs_flush {
            self.decoder.flush();
            self.serial = serial;
            self.span = span;
            self.needs_flush = false;
            self.graph = None;
            self.cursor = None;
        }
    }

    /// Takes everything left in the decoder and the graph.
    fn finish_part(&mut self) {
        let _ = self.decoder.send_eof();
        self.drain();
        if let Some(graph) = &mut self.graph
            && let Some(mut source) = graph.graph.get("in")
        {
            let _ = source.source().flush();
        }
        self.pull();
    }

    fn drain(&mut self) {
        let mut decoded = ffmpeg::frame::Audio::empty();
        while self.decoder.receive_frame(&mut decoded).is_ok() {
            self.filter(&decoded);
            if self.shared.stopping() {
                return;
            }
        }
    }

    fn filter(&mut self, frame: &ffmpeg::frame::Audio) {
        if self.serial != self.shared.serial.load(Ordering::Relaxed) {
            return;
        }
        let tempo = self.shared.speed();
        let key = (
            frame.format(),
            frame.rate(),
            frame.channels(),
            tempo.to_bits(),
        );
        if self.graph.as_ref().is_none_or(|graph| graph.key != key) {
            // Sound still inside the old graph is let go: a speed change
            // loses a fraction of a second.
            match build_graph(
                frame,
                self.time_base,
                tempo,
                self.output.rate,
                self.output.channels,
            ) {
                Ok(graph) => {
                    self.graph = Some(AudioGraph { graph, key, tempo });
                    self.cursor = None;
                }
                Err(error) => {
                    eprintln!("plaa: sound filter: {error}");
                    return;
                }
            }
        }
        if self.cursor.is_none() {
            self.cursor = frame
                .timestamp()
                .or(frame.pts())
                .and_then(|timestamp| info::seconds(timestamp, self.time_base))
                .map(|seconds| seconds - self.start);
        }
        let Some(graph) = &mut self.graph else {
            return;
        };
        let Some(mut source) = graph.graph.get("in") else {
            return;
        };
        if source.source().add(frame).is_err() {
            return;
        }
        self.pull();
    }

    /// Takes what the graph has ready and queues it for the device.
    fn pull(&mut self) {
        let channels = usize::from(self.output.channels.max(1));
        let rate = f64::from(self.output.rate);
        loop {
            let Some(graph) = &mut self.graph else {
                return;
            };
            let tempo = graph.tempo;
            let Some(mut sink) = graph.graph.get("out") else {
                return;
            };
            let mut out = ffmpeg::frame::Audio::empty();
            if sink.sink().frame(&mut out).is_err() {
                return;
            }
            let time_base = sink.sink().time_base();
            let frames = out.samples();
            if frames == 0 {
                continue;
            }
            let step = tempo / rate;
            // At normal speed the graph's timestamps are the file's time;
            // with tempo they count played time, so its time is counted
            // here.
            let stamped = (tempo == 1.0)
                .then(|| out.pts())
                .flatten()
                .and_then(|pts| info::seconds(pts, time_base))
                .map(|seconds| seconds - self.start);
            let mut source = stamped.or(self.cursor).unwrap_or(0.0);
            let end = source + frames as f64 * step;
            self.cursor = Some(end);
            let span = self.span;
            if end <= span.from || source >= span.to {
                continue;
            }
            let bytes = &out.data(0)[..(frames * channels * 4).min(out.data(0).len())];
            let mut samples: Vec<f32> = bytes
                .as_chunks::<4>()
                .0
                .iter()
                .map(|bytes| f32::from_ne_bytes(*bytes))
                .collect();
            // Only the sound within the part plays.
            if source < span.from {
                let skip = (((span.from - source) / step) as usize).min(frames);
                samples.drain(..skip * channels);
                source += skip as f64 * step;
            }
            if end > span.to {
                let keep = (((span.to - source) / step).ceil().max(0.0) as usize) * channels;
                samples.truncate(keep);
            }
            if samples.is_empty() {
                continue;
            }
            let frames = samples.len() / channels;
            let mut position = source + span.shift;
            if let Some(target) = self.shared.target(self.serial) {
                let end = position + frames as f64 * step;
                if end <= target {
                    continue;
                }
                if position < target {
                    let skip = (((target - position) / step) as usize).min(frames);
                    samples.drain(..skip * channels);
                    position += skip as f64 * step;
                }
            }
            if !self
                .output
                .push(self.serial, position, step, samples, &self.shared.stop)
            {
                return;
            }
        }
    }
}

/// abuffer, then `atempo` for any speed but normal, then the output's
/// sample format, rate and channels.
fn build_graph(
    frame: &ffmpeg::frame::Audio,
    time_base: ffmpeg::Rational,
    tempo: f64,
    rate: u32,
    channels: u16,
) -> Result<ffmpeg::filter::Graph, ffmpeg::Error> {
    let mut graph = ffmpeg::filter::Graph::new();
    let arguments = format!(
        "time_base={}/{}:sample_rate={}:sample_fmt={}:channel_layout={}",
        time_base.numerator(),
        time_base.denominator().max(1),
        frame.rate(),
        frame.format().name(),
        ffi::frame_layout(frame),
    );
    let abuffer = ffmpeg::filter::find("abuffer").ok_or(ffmpeg::Error::FilterNotFound)?;
    let abuffersink = ffmpeg::filter::find("abuffersink").ok_or(ffmpeg::Error::FilterNotFound)?;
    graph.add(&abuffer, "in", &arguments)?;
    graph.add(&abuffersink, "out", "")?;
    let out_layout = ffi::default_layout(i32::from(channels));
    let specification = format!(
        "{}aformat=sample_fmts=flt:sample_rates={rate}:channel_layouts={out_layout}",
        tempo_filters(tempo)
    );
    graph
        .output("in", 0)?
        .input("out", 0)?
        .parse(&specification)?;
    graph.validate()?;
    Ok(graph)
}

/// `atempo` filters for `tempo`, chained below its one filter range of
/// 0.5 to 100; nothing at normal speed.
fn tempo_filters(tempo: f64) -> String {
    if (tempo - 1.0).abs() < 1e-6 {
        return String::new();
    }
    let mut filters = String::new();
    let mut left = tempo;
    while left < 0.5 {
        filters.push_str("atempo=0.5,");
        left /= 0.5;
    }
    filters.push_str(&format!("atempo={left:.6},"));
    filters
}

/// Decodes text subtitles as the reader comes across their packets.
/// What a subtitle packet decodes to.
pub(crate) enum Decoded {
    Text(Cue),
    Picture(PictureCue),
    /// Whatever picture shows ends at this many seconds, as an empty
    /// Blu-ray display set says.
    Clear(f64),
}

pub(crate) struct SubtitleDecoder {
    decoder: ffmpeg::decoder::Subtitle,
    time_base: ffmpeg::Rational,
    start: f64,
    /// The frame pictures are placed in when the decoder does not say:
    /// the video's.
    canvas: (u32, u32),
}

impl SubtitleDecoder {
    pub fn new(
        input: &ffmpeg::format::context::Input,
        index: usize,
        start: f64,
        canvas: (u32, u32),
    ) -> Result<Self, ffmpeg::Error> {
        let (decoder, time_base) = open_decoder(input, index)?;
        Ok(Self {
            decoder: decoder.subtitle()?,
            time_base,
            start,
            canvas,
        })
    }

    pub fn decode(&mut self, packet: &ffmpeg::Packet) -> Vec<Decoded> {
        let mut subtitle = ffmpeg::Subtitle::new();
        if !matches!(self.decoder.decode(packet, &mut subtitle), Ok(true)) {
            return Vec::new();
        }
        let base = subtitle
            .pts()
            .map(|pts| pts as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE))
            .or_else(|| {
                packet
                    .pts()
                    .and_then(|pts| info::seconds(pts, self.time_base))
            });
        let Some(base) = base else {
            return Vec::new();
        };
        let start = base - self.start + f64::from(subtitle.start()) / 1000.0;
        let known_end = match subtitle.end() {
            0 | u32::MAX => {
                let duration = packet.duration();
                (duration > 0)
                    .then(|| start + info::seconds(duration, self.time_base).unwrap_or(0.0))
            }
            end => Some(base - self.start + f64::from(end) / 1000.0),
        };
        let mut decoded = Vec::new();
        let text: Vec<String> = subtitle
            .rects()
            .filter_map(|rect| match rect {
                ffmpeg::codec::subtitle::Rect::Text(text) => Some(subtitle::plain_text(text.get())),
                ffmpeg::codec::subtitle::Rect::Ass(ass) => Some(subtitle::ass_text(ass.get())),
                _ => None,
            })
            .filter(|line| !line.is_empty())
            .collect();
        if !text.is_empty() {
            decoded.push(Decoded::Text(Cue {
                start,
                end: known_end.unwrap_or(start),
                text: text.join("\n"),
            }));
        }
        let bitmaps: Vec<_> = subtitle
            .rects()
            .filter_map(|rect| match rect {
                ffmpeg::codec::subtitle::Rect::Bitmap(bitmap) => Some((
                    bitmap.x() as u32,
                    bitmap.y() as u32,
                    bitmap.width(),
                    bitmap.height(),
                    ffi::bitmap_rgba(&bitmap)?,
                )),
                _ => None,
            })
            .collect();
        if subtitle.rects().next().is_none() {
            decoded.push(Decoded::Clear(start));
        } else if let Some(picture) = composite(&bitmaps) {
            let canvas = ffi::subtitle_canvas(&self.decoder).unwrap_or(self.canvas);
            let (x, y, width, height, rgba) = picture;
            decoded.push(Decoded::Picture(PictureCue {
                start,
                // Shown until the next, or until a clear, when no end is
                // given.
                end: known_end
                    .filter(|end| *end > start)
                    .unwrap_or(f64::INFINITY),
                picture: Arc::new(SubtitlePicture::new(canvas, x, y, width, height, rgba)),
            }));
        }
        decoded
    }

    pub fn flush(&mut self) {
        self.decoder.flush();
    }
}

/// The picture stream to show: of the variants, the best `quality`
/// allows, else the first picture that is not a cover.
pub(crate) fn pick_video(info: &MediaInfo, quality: Quality) -> Option<usize> {
    if quality == Quality::AudioOnly {
        return None;
    }
    let variant = match quality {
        Quality::AtMost(lines) => info
            .variants
            .iter()
            .find(|variant| variant.height <= lines)
            .or(info.variants.last()),
        _ => info.variants.first(),
    };
    variant
        .map(|variant| variant.video)
        .or_else(|| info.picture(None).map(|track| track.track.index))
}

/// The audio stream to play of `tracks`: the first in a preferred
/// language, else the one the file marks as default, else the first.
pub(crate) fn pick_audio(tracks: &[&info::AudioTrack], languages: &[String]) -> Option<usize> {
    let tracks: Vec<_> = tracks.iter().map(|track| &track.track).collect();
    by_language(&tracks, languages)
        .or_else(|| tracks.iter().find(|track| track.default).copied())
        .or_else(|| tracks.first().copied())
        .map(|track| track.index)
}

/// The subtitles to show: text ones in a preferred language, else ones
/// the file marks forced or default. None when nothing asks for them.
/// A subtitle's bitmaps, each at its place, as one picture over the box
/// they share: its place, size and RGBA pixels.
type Placed = (u32, u32, u32, u32, Vec<u8>);

fn composite(bitmaps: &[Placed]) -> Option<Placed> {
    let left = bitmaps.iter().map(|(x, ..)| *x).min()?;
    let top = bitmaps.iter().map(|(_, y, ..)| *y).min()?;
    let right = bitmaps.iter().map(|(x, _, width, ..)| x + width).max()?;
    let bottom = bitmaps
        .iter()
        .map(|(_, y, _, height, _)| y + height)
        .max()?;
    let (width, height) = (right - left, bottom - top);
    if let [only] = bitmaps {
        return Some(only.clone());
    }
    let mut rgba = vec![0; width as usize * height as usize * 4];
    for (x, y, bitmap_width, bitmap_height, pixels) in bitmaps {
        let row = *bitmap_width as usize * 4;
        for line in 0..*bitmap_height as usize {
            let to = ((*y - top) as usize + line) * width as usize * 4 + (*x - left) as usize * 4;
            rgba[to..to + row].copy_from_slice(&pixels[line * row..(line + 1) * row]);
        }
    }
    Some((left, top, width, height, rgba))
}

/// The subtitles to show at first: in a preferred language, a file beside
/// the video before the video's own; else a file beside the video, as it
/// was put there to be shown; else the video's forced or default track.
pub(crate) fn pick_subtitles(info: &MediaInfo, languages: &[String]) -> Option<usize> {
    let text = || info.subtitles.iter();
    let files: Vec<_> = text()
        .filter(|track| track.file.is_some())
        .map(|track| &track.track)
        .collect();
    let own: Vec<_> = text()
        .filter(|track| track.file.is_none())
        .map(|track| &track.track)
        .collect();
    let tracks: Vec<_> = files.iter().chain(&own).copied().collect();
    by_language(&tracks, languages)
        .or_else(|| files.first().copied())
        .or_else(|| own.iter().find(|track| track.forced).copied())
        .or_else(|| own.iter().find(|track| track.default).copied())
        .map(|track| track.index)
}

/// Whether `code` is an ISO 639 code plaa knows, two or three letters,
/// perhaps with a region as in `pt-BR`.
pub fn is_language_code(code: &str) -> bool {
    let code = code.to_ascii_lowercase();
    let base = code.split(['-', '_']).next().unwrap_or("");
    LANGUAGE_CODES
        .iter()
        .any(|(two, three)| *two == base || three.split('/').any(|three| three == base))
}

/// A subtitle file's codec and all its cues, read at once: such files are
/// small. None when it holds no text subtitles.
pub(crate) fn read_subtitle_file(path: &std::path::Path) -> Option<(info::Track, Vec<Cue>)> {
    let mut input = ffmpeg::format::input(path).ok()?;
    let stream = input.streams().best(ffmpeg::media::Type::Subtitle)?;
    let index = stream.index();
    let codec = stream.parameters().id();
    let track = info::Track {
        codec: codec.name().to_owned(),
        codec_name: ffmpeg::codec::decoder::find(codec)
            .map(|decoder| decoder.description().to_owned())
            .unwrap_or_default(),
        ..info::Track::default()
    };
    // Its times count from the video's start, whatever the video's own
    // timestamps start at.
    let mut decoder = SubtitleDecoder::new(&input, index, 0.0, (0, 0)).ok()?;
    let mut cues = crate::subtitle::Cues::default();
    for (stream, packet) in input.packets() {
        if stream.index() == index {
            for decoded in decoder.decode(&packet) {
                if let Decoded::Text(cue) = decoded {
                    cues.add(cue);
                }
            }
        }
    }
    let cues = cues.into_vec();
    (!cues.is_empty()).then_some((track, cues))
}

fn by_language<'a>(tracks: &[&'a info::Track], languages: &[String]) -> Option<&'a info::Track> {
    languages.iter().find_map(|wanted| {
        tracks
            .iter()
            .find(|track| {
                track
                    .language
                    .as_deref()
                    .is_some_and(|language| same_language(language, wanted))
            })
            .copied()
    })
}

/// Whether two ISO 639 codes name the same language, as files use the
/// three letter codes and people often the two letter ones.
pub fn same_language(first: &str, second: &str) -> bool {
    let normal = |code: &str| {
        let code = code.trim().to_ascii_lowercase();
        let code = code.split(['-', '_']).next().unwrap_or("").to_owned();
        LANGUAGE_CODES
            .iter()
            .find(|(_, three)| three.split('/').any(|three| three == code))
            .map(|(two, _)| (*two).to_owned())
            .unwrap_or(code)
    };
    let (first, second) = (normal(first), normal(second));
    !first.is_empty() && first == second
}

/// ISO 639-1 codes and the ISO 639-2 codes for them, bibliographic and
/// terminology forms where they differ.
const LANGUAGE_CODES: &[(&str, &str)] = &[
    ("ar", "ara"),
    ("bn", "ben"),
    ("ca", "cat"),
    ("cs", "cze/ces"),
    ("da", "dan"),
    ("de", "ger/deu"),
    ("el", "gre/ell"),
    ("en", "eng"),
    ("es", "spa"),
    ("fa", "per/fas"),
    ("fi", "fin"),
    ("fr", "fre/fra"),
    ("he", "heb"),
    ("hi", "hin"),
    ("hu", "hun"),
    ("id", "ind"),
    ("it", "ita"),
    ("ja", "jpn"),
    ("ko", "kor"),
    ("ms", "may/msa"),
    ("nb", "nob/nor"),
    ("nl", "dut/nld"),
    ("pl", "pol"),
    ("pt", "por"),
    ("ro", "rum/ron"),
    ("ru", "rus"),
    ("sv", "swe"),
    ("sw", "swa"),
    ("ta", "tam"),
    ("th", "tha"),
    ("tr", "tur"),
    ("uk", "ukr"),
    ("ur", "urd"),
    ("vi", "vie"),
    ("zh", "chi/zho"),
];

/// The cover art in stream `index`, decoded and made at most 1024 pixels
/// on its long side.
pub(crate) fn cover(input: &ffmpeg::format::context::Input, index: usize) -> Option<Picture> {
    let stream = input.stream(index)?;
    let bytes = ffi::attached_picture(&stream)?;
    still(stream.parameters(), &ffmpeg::Packet::copy(&bytes))
}

/// The picture at `address`, such as a web video's thumbnail, decoded as
/// a cover is. Gives up once `stop` says so.
pub(crate) fn picture_at(
    address: &str,
    stop: impl FnMut() -> bool + Send + 'static,
) -> Option<Picture> {
    let mut input = ffmpeg::format::input_with_interrupt_and_dictionary(
        address,
        stop,
        ffmpeg::Dictionary::new(),
    )
    .ok()?;
    let index = input.streams().best(ffmpeg::media::Type::Video)?.index();
    let mut packet = ffmpeg::Packet::empty();
    while packet.read(&mut input).is_ok() {
        if packet.stream() == index {
            return still(input.stream(index)?.parameters(), &packet);
        }
    }
    None
}

/// The still picture in `packet`, decoded and made at most 1024 pixels on
/// its long side.
fn still(parameters: ffmpeg::codec::Parameters, packet: &ffmpeg::Packet) -> Option<Picture> {
    let context = ffmpeg::codec::Context::from_parameters(parameters).ok()?;
    let mut decoder = context.decoder().video().ok()?;
    decoder.send_packet(packet).ok()?;
    let _ = decoder.send_eof();
    let mut frame = ffmpeg::frame::Video::empty();
    decoder.receive_frame(&mut frame).ok()?;
    let (width, height) = (frame.width(), frame.height());
    if width == 0 || height == 0 {
        return None;
    }
    let scale = (1024.0 / f64::from(width.max(height))).min(1.0);
    let (out_width, out_height) = (
        ((f64::from(width) * scale).round() as u32).max(1),
        ((f64::from(height) * scale).round() as u32).max(1),
    );
    let mut scaler = scaling::Context::get(
        frame.format(),
        width,
        height,
        Pixel::RGBA,
        out_width,
        out_height,
        scaling::Flags::BICUBIC,
    )
    .ok()?;
    let mut out = ffmpeg::frame::Video::empty();
    scaler.run(&frame, &mut out).ok()?;
    let stride = out.stride(0);
    let row = out_width as usize * 4;
    let mut rgba = Vec::with_capacity(row * out_height as usize);
    for line in out.data(0).chunks(stride).take(out_height as usize) {
        rgba.extend_from_slice(&line[..row.min(line.len())]);
    }
    Some(Picture {
        width: out_width,
        height: out_height,
        rgba,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tempo_is_chained_below_half_speed() {
        assert_eq!(tempo_filters(1.0), "");
        assert_eq!(tempo_filters(1.5), "atempo=1.500000,");
        assert_eq!(tempo_filters(0.25), "atempo=0.5,atempo=0.500000,");
    }

    #[test]
    fn two_and_three_letter_codes_match() {
        assert!(same_language("en", "eng"));
        assert!(same_language("ger", "de"));
        assert!(same_language("deu", "ger"));
        assert!(same_language("pt-BR", "por"));
        assert!(!same_language("en", "fr"));
        assert!(!same_language("", ""));
    }
}
