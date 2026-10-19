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
use crate::ffi;
use crate::frame::{self, VideoFrame};
use crate::info::{self, MediaInfo};
use crate::player::Shared;
use crate::queue::{Closed, Item, PacketQueue};
use crate::subtitle::{self, Cue};

/// A still picture as RGBA, such as cover art.
#[derive(Clone, PartialEq, Eq)]
pub struct Picture {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
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
fn open_decoder(
    input: &ffmpeg::format::context::Input,
    index: usize,
) -> Result<(ffmpeg::codec::decoder::Decoder, ffmpeg::Rational), ffmpeg::Error> {
    let stream = input.stream(index).ok_or(ffmpeg::Error::StreamNotFound)?;
    let time_base = stream.time_base();
    let mut context = ffmpeg::codec::Context::from_parameters(stream.parameters())?;
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
    let (mut decoder, time_base) = open_decoder(input, index)?;
    let mut device = None;
    if hardware {
        device = ffi::attach_hardware(&mut decoder.0);
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
        needs_flush: false,
    };
    let thread = std::thread::Builder::new()
        .name("plaa-video".into())
        .spawn(move || worker.run())
        .map_err(|_| ffmpeg::Error::External)?;
    Ok(((queue, thread), device))
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
    /// The decoder was drained at the end of the file and must be reset
    /// before taking packets again.
    needs_flush: bool,
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
                Item::Packet(serial, packet) => {
                    self.renew(serial);
                    if serial != self.shared.serial.load(Ordering::Relaxed) {
                        continue;
                    }
                    if self.decoder.send_packet(&packet).is_err() {
                        continue;
                    }
                    self.drain(&mut scaler);
                }
                Item::End(serial) => {
                    self.renew(serial);
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

    /// Resets the decoder for packets of a new serial, after a seek.
    fn renew(&mut self, serial: u64) {
        if serial != self.serial || self.needs_flush {
            self.decoder.flush();
            self.serial = serial;
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
        let position = frame
            .timestamp()
            .or(frame.pts())
            .and_then(|timestamp| info::seconds(timestamp, self.time_base))
            .map(|seconds| seconds - self.start)
            .or_else(|| self.last_position.map(|last| last + self.frame_duration))
            .unwrap_or(0.0);
        self.last_position = Some(position);
        if let Some(target) = self.shared.target(serial)
            && position + self.frame_duration / 2.0 < target
        {
            return;
        }
        let frame = if ffi::is_hardware(&frame) {
            match ffi::download(&frame) {
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
    needs_flush: bool,
    graph: Option<AudioGraph>,
    /// The media time of the next sample out of the graph.
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
                Item::Packet(serial, packet) => {
                    self.renew(serial);
                    if serial != self.shared.serial.load(Ordering::Relaxed) {
                        continue;
                    }
                    if self.decoder.send_packet(&packet).is_err() {
                        continue;
                    }
                    self.drain();
                }
                Item::End(serial) => {
                    self.renew(serial);
                    let _ = self.decoder.send_eof();
                    self.drain();
                    if let Some(graph) = &mut self.graph
                        && let Some(mut source) = graph.graph.get("in")
                    {
                        let _ = source.source().flush();
                    }
                    self.pull();
                    self.needs_flush = true;
                    self.output.finish(serial);
                }
            }
        }
    }

    fn renew(&mut self, serial: u64) {
        if serial != self.serial || self.needs_flush {
            self.decoder.flush();
            self.serial = serial;
            self.needs_flush = false;
            self.graph = None;
            self.cursor = None;
        }
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
            // At normal speed the graph's timestamps are media time; with
            // tempo they count played time, so media time is counted here.
            let stamped = (tempo == 1.0)
                .then(|| out.pts())
                .flatten()
                .and_then(|pts| info::seconds(pts, time_base))
                .map(|seconds| seconds - self.start);
            let mut position = stamped.or(self.cursor).unwrap_or(0.0);
            self.cursor = Some(position + frames as f64 * step);
            let bytes = &out.data(0)[..(frames * channels * 4).min(out.data(0).len())];
            let mut samples: Vec<f32> = bytes
                .as_chunks::<4>()
                .0
                .iter()
                .map(|bytes| f32::from_ne_bytes(*bytes))
                .collect();
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
    let layout = frame.channel_layout();
    let layout = if layout.is_empty() {
        ffmpeg::ChannelLayout::default(i32::from(frame.channels()))
    } else {
        layout
    };
    let arguments = format!(
        "time_base={}/{}:sample_rate={}:sample_fmt={}:channel_layout={}",
        time_base.numerator(),
        time_base.denominator().max(1),
        frame.rate(),
        frame.format().name(),
        ffi::describe_layout(&layout),
    );
    let abuffer = ffmpeg::filter::find("abuffer").ok_or(ffmpeg::Error::FilterNotFound)?;
    let abuffersink = ffmpeg::filter::find("abuffersink").ok_or(ffmpeg::Error::FilterNotFound)?;
    graph.add(&abuffer, "in", &arguments)?;
    graph.add(&abuffersink, "out", "")?;
    let out_layout = ffi::describe_layout(&ffmpeg::ChannelLayout::default(i32::from(channels)));
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
pub(crate) struct SubtitleDecoder {
    decoder: ffmpeg::decoder::Subtitle,
    time_base: ffmpeg::Rational,
    start: f64,
}

impl SubtitleDecoder {
    pub fn new(
        input: &ffmpeg::format::context::Input,
        index: usize,
        start: f64,
    ) -> Result<Self, ffmpeg::Error> {
        let (decoder, time_base) = open_decoder(input, index)?;
        Ok(Self {
            decoder: decoder.subtitle()?,
            time_base,
            start,
        })
    }

    pub fn decode(&mut self, packet: &ffmpeg::Packet) -> Vec<Cue> {
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
        let end = match subtitle.end() {
            0 | u32::MAX => {
                let duration = packet.duration();
                if duration > 0 {
                    start + info::seconds(duration, self.time_base).unwrap_or(0.0)
                } else {
                    start
                }
            }
            end => base - self.start + f64::from(end) / 1000.0,
        };
        let text: Vec<String> = subtitle
            .rects()
            .filter_map(|rect| match rect {
                ffmpeg::codec::subtitle::Rect::Text(text) => Some(subtitle::plain_text(text.get())),
                ffmpeg::codec::subtitle::Rect::Ass(ass) => Some(subtitle::ass_text(ass.get())),
                _ => None,
            })
            .filter(|line| !line.is_empty())
            .collect();
        if text.is_empty() {
            return Vec::new();
        }
        vec![Cue {
            start,
            end,
            text: text.join("\n"),
        }]
    }

    pub fn flush(&mut self) {
        self.decoder.flush();
    }
}

/// The audio stream to play: the first in a preferred language, else the
/// one the file marks as default, else the first.
pub(crate) fn pick_audio(info: &MediaInfo, languages: &[String]) -> Option<usize> {
    let tracks: Vec<_> = info.audio.iter().map(|track| &track.track).collect();
    by_language(&tracks, languages)
        .or_else(|| tracks.iter().find(|track| track.default).copied())
        .or_else(|| tracks.first().copied())
        .map(|track| track.index)
}

/// The subtitles to show: text ones in a preferred language, else ones
/// the file marks forced or default. None when nothing asks for them.
pub(crate) fn pick_subtitles(info: &MediaInfo, languages: &[String]) -> Option<usize> {
    let tracks: Vec<_> = info
        .subtitles
        .iter()
        .filter(|track| track.text)
        .map(|track| &track.track)
        .collect();
    by_language(&tracks, languages)
        .or_else(|| tracks.iter().find(|track| track.forced).copied())
        .or_else(|| tracks.iter().find(|track| track.default).copied())
        .map(|track| track.index)
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
    let context = ffmpeg::codec::Context::from_parameters(stream.parameters()).ok()?;
    let mut decoder = context.decoder().video().ok()?;
    decoder.send_packet(&ffmpeg::Packet::copy(&bytes)).ok()?;
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
