//! Exporting a file, as edited, to a new one: its picture to H.264, HEVC
//! or AV1, on a graphics card's encoder where one opens, else on the
//! processor, with its sound as AAC; or the sound alone, compressed or
//! lossless. Pictures are turned and flipped as the edit says, and made
//! smaller when asked.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use ffmpeg::format::Pixel;
use ffmpeg_next as ffmpeg;

use crate::edit::{Edit, Span, Transform};
use crate::ffi;
use crate::info::{self, MediaInfo};

/// What an export makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    H264,
    Hevc,
    Av1,
    /// The sound alone.
    Sound(Sound),
}

impl Format {
    /// The file name ending its files usually have.
    pub fn extension(self) -> &'static str {
        match self {
            Format::Sound(sound) => sound.extension(),
            _ => "mp4",
        }
    }
}

/// What sound alone is exported as.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sound {
    Aac,
    Mp3,
    Opus,
    Flac,
    /// Apple Lossless.
    Alac,
    /// Uncompressed, in a WAV file.
    Wav,
}

impl Sound {
    pub const ALL: [Sound; 6] = [
        Sound::Aac,
        Sound::Mp3,
        Sound::Opus,
        Sound::Flac,
        Sound::Alac,
        Sound::Wav,
    ];

    /// Whether it keeps the sound exactly.
    pub fn lossless(self) -> bool {
        matches!(self, Sound::Flac | Sound::Alac | Sound::Wav)
    }

    /// The qualities offered: bit rates in kb/s for two channels, or for
    /// lossless sound, bits per sample.
    pub fn qualities(self) -> &'static [u32] {
        match self {
            Sound::Aac | Sound::Mp3 => &[128, 192, 256, 320],
            Sound::Opus => &[64, 96, 128, 192],
            Sound::Flac | Sound::Alac | Sound::Wav => &[16, 24],
        }
    }

    /// The quality offered first: plenty for listening, or CD quality.
    pub fn usual_quality(self) -> u32 {
        match self {
            Sound::Aac | Sound::Mp3 => 256,
            Sound::Opus => 128,
            Sound::Flac | Sound::Alac | Sound::Wav => 16,
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Sound::Aac | Sound::Alac => "m4a",
            Sound::Mp3 => "mp3",
            Sound::Opus => "opus",
            Sound::Flac => "flac",
            Sound::Wav => "wav",
        }
    }

    /// FFmpeg's muxer for its files.
    fn muxer(self) -> &'static str {
        match self {
            Sound::Aac | Sound::Alac => "ipod",
            Sound::Mp3 => "mp3",
            Sound::Opus => "opus",
            Sound::Flac => "flac",
            Sound::Wav => "wav",
        }
    }

    /// FFmpeg's encoders for it at `quality`, best first: Apple's AAC
    /// encoder where there is one, as it sounds better.
    fn encoders(self, quality: u32) -> &'static [&'static str] {
        match self {
            Sound::Aac if cfg!(target_os = "macos") => &["aac_at", "aac"],
            Sound::Aac => &["aac"],
            Sound::Mp3 => &["libmp3lame"],
            Sound::Opus => &["libopus"],
            Sound::Flac => &["flac"],
            Sound::Alac => &["alac"],
            Sound::Wav if quality > 16 => &["pcm_s24le"],
            Sound::Wav => &["pcm_s16le"],
        }
    }

    /// Whether this system's FFmpeg can encode it.
    pub fn available(self) -> bool {
        self.encoders(self.usual_quality())
            .iter()
            .any(|name| ffmpeg::encoder::find_by_name(name).is_some())
    }
}

/// An encoder an export can use: FFmpeg's name for it, and whether it
/// runs on a graphics card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Encoder {
    pub name: &'static str,
    pub hardware: bool,
}

/// What to export and how.
#[derive(Debug, Clone)]
pub struct Job {
    pub source: PathBuf,
    /// The edit to export, or the whole file as it is.
    pub edit: Option<Arc<Edit>>,
    /// The picture and sound streams to export, as they play.
    pub video: Option<usize>,
    pub audio: Option<usize>,
    pub format: Format,
    /// The picture's shorter side, in pixels, when smaller than the
    /// file's.
    pub lines: Option<u32>,
    /// The sound's quality when it is exported alone, as its format
    /// counts it.
    pub quality: Option<u32>,
    /// Decode and encode on a graphics card where one does it.
    pub hardware: bool,
    pub destination: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExportEvent {
    /// The export started, its picture encoded by this encoder.
    Started(Option<Encoder>),
    /// This share of it is done, 0 to 1.
    Progress(f64),
    Finished(Result<PathBuf, ExportError>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExportError {
    Cancelled,
    Failed(String),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportError::Cancelled => formatter.write_str("cancelled"),
            ExportError::Failed(reason) => formatter.write_str(reason),
        }
    }
}

/// An export under way on a thread of its own. Dropping it cancels it.
pub struct Export {
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Export {
    /// Starts `job`; `events` hears how it goes, from its thread.
    pub fn start(job: Job, events: impl Fn(ExportEvent) + Send + 'static) -> Self {
        crate::init();
        let stop = Arc::new(AtomicBool::new(false));
        let thread = {
            let stop = stop.clone();
            std::thread::Builder::new()
                .name("plaa-export".into())
                .spawn(move || {
                    let temporary = temporary_path(&job.destination);
                    let result = run(&job, &temporary, &stop, &events);
                    if result.is_err() {
                        let _ = std::fs::remove_file(&temporary);
                    }
                    let result = result.and_then(|()| {
                        std::fs::rename(&temporary, &job.destination)
                            .map(|()| job.destination.clone())
                            .map_err(|error| {
                                let _ = std::fs::remove_file(&temporary);
                                ExportError::Failed(error.to_string())
                            })
                    });
                    events(ExportEvent::Finished(result));
                })
                .ok()
        };
        Self { stop, thread }
    }

    pub fn cancel(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

impl Drop for Export {
    fn drop(&mut self) {
        self.cancel();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// Where an export is written until it is done: a hidden file beside the
/// one asked for, so a file of that name stays until it is replaced
/// whole.
fn temporary_path(destination: &Path) -> PathBuf {
    let name = destination
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "export".to_owned());
    destination.with_file_name(format!(".{name}.part"))
}

/// The encoders to try for `format` on this system, best first: the
/// graphics cards' APIs, then the processor's.
fn candidates(format: Format) -> Vec<Encoder> {
    let hardware: &[&str] = match format {
        Format::H264 if cfg!(target_os = "macos") => &["h264_videotoolbox"],
        Format::H264 if cfg!(windows) => &["h264_nvenc", "h264_amf", "h264_qsv"],
        Format::H264 => &["h264_nvenc", "h264_vaapi", "h264_qsv"],
        Format::Hevc if cfg!(target_os = "macos") => &["hevc_videotoolbox"],
        Format::Hevc if cfg!(windows) => &["hevc_nvenc", "hevc_amf", "hevc_qsv"],
        Format::Hevc => &["hevc_nvenc", "hevc_vaapi", "hevc_qsv"],
        Format::Av1 if cfg!(target_os = "macos") => &[],
        Format::Av1 if cfg!(windows) => &["av1_nvenc", "av1_amf", "av1_qsv"],
        Format::Av1 => &["av1_nvenc", "av1_vaapi", "av1_qsv"],
        Format::Sound(_) => &[],
    };
    let software: &[&str] = match format {
        Format::H264 => &["libx264"],
        Format::Hevc => &["libx265"],
        Format::Av1 => &["libsvtav1", "libaom-av1"],
        Format::Sound(_) => &[],
    };
    hardware
        .iter()
        .map(|name| Encoder {
            name,
            hardware: true,
        })
        .chain(software.iter().map(|name| Encoder {
            name,
            hardware: false,
        }))
        .collect()
}

/// The encoder an export to `format` of `size` pictures would use, on a
/// graphics card only when `hardware` allows: the first that opens, as
/// cards have sizes they cannot encode, in ten bits for `hdr` pictures.
/// Asking again gives the answer found the first time.
pub fn encoder(format: Format, size: (u32, u32), hdr: bool, hardware: bool) -> Option<Encoder> {
    /// The encoder found for each format, size and depth, with or
    /// without graphics cards.
    type Found = HashMap<(Format, (u32, u32), bool, bool), Option<Encoder>>;
    static FOUND: Mutex<Option<Found>> = Mutex::new(None);
    let key = (format, size, hdr, hardware);
    if let Some(found) = FOUND
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .as_ref()
        .and_then(|found| found.get(&key))
    {
        return *found;
    }
    crate::init();
    let picture = Picture {
        width: size.0.max(2),
        height: size.1.max(2),
        rate: ffmpeg::Rational::new(30, 1),
        time_base: ffmpeg::Rational::new(1, 30),
        ten_bit: hdr,
    };
    let chosen = candidates(format)
        .into_iter()
        .filter(|candidate| hardware || !candidate.hardware)
        .find(|candidate| open_video(*candidate, format, &picture, None, false).is_some());
    FOUND
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .get_or_insert_with(HashMap::new)
        .insert(key, chosen);
    chosen
}

/// What a picture encoder is opened for.
#[derive(Clone, Copy)]
struct Picture {
    width: u32,
    height: u32,
    rate: ffmpeg::Rational,
    time_base: ffmpeg::Rational,
    ten_bit: bool,
}

/// An opened picture encoder: the pixel format it takes from memory, and
/// the graphics memory pictures go through first when it needs that.
struct VideoEncoder {
    encoder: ffmpeg::encoder::video::Encoder,
    format: Pixel,
    frames: Option<ffi::HardwareFrames>,
}

/// Opens `candidate` for `picture`, as an encoder of `format`, with the
/// colors of `stream`. None when it is not there or will not open.
fn open_video(
    candidate: Encoder,
    format: Format,
    picture: &Picture,
    stream: Option<&ffmpeg::format::stream::Stream>,
    global_header: bool,
) -> Option<VideoEncoder> {
    let codec = ffmpeg::encoder::find_by_name(candidate.name)?;
    let formats: Vec<Pixel> = codec
        .video()
        .ok()?
        .formats()
        .map(Iterator::collect)
        .unwrap_or_default();
    let through_memory = formats.contains(&Pixel::VAAPI);
    // Ten bits where the picture has them and the encoder takes them.
    let wanted: &[Pixel] = if picture.ten_bit && format != Format::H264 {
        &[
            Pixel::P010LE,
            Pixel::YUV420P10LE,
            Pixel::NV12,
            Pixel::YUV420P,
        ]
    } else {
        &[Pixel::NV12, Pixel::YUV420P]
    };
    let pixel = if through_memory {
        wanted[0]
    } else {
        *wanted.iter().find(|pixel| formats.contains(pixel))?
    };
    let mut context = ffmpeg::codec::Context::new_with_codec(codec);
    if let Some(stream) = stream {
        ffi::copy_colors(stream, &mut context);
    }
    let mut encoder = context.encoder().video().ok()?;
    encoder.set_width(picture.width);
    encoder.set_height(picture.height);
    encoder.set_format(pixel);
    encoder.set_time_base(picture.time_base);
    encoder.set_frame_rate(Some(picture.rate));
    encoder.set_aspect_ratio(ffmpeg::Rational::new(1, 1));
    if global_header {
        encoder.set_flags(ffmpeg::codec::Flags::GLOBAL_HEADER);
    }
    let frames = if through_memory {
        Some(ffi::HardwareFrames::vaapi(
            &mut encoder,
            picture.width,
            picture.height,
            pixel,
        )?)
    } else {
        None
    };
    let (options, bit_rate) = quality(candidate.name, format, picture);
    if let Some(bit_rate) = bit_rate {
        encoder.set_bit_rate(bit_rate);
        encoder.set_max_bit_rate(bit_rate * 2);
    }
    let encoder = encoder.open_as_with(codec, options).ok()?;
    Some(VideoEncoder {
        encoder,
        format: pixel,
        frames,
    })
}

/// How each encoder is asked for good quality at a reasonable size: its
/// options, and the bit rate for those that need one.
fn quality(
    name: &str,
    format: Format,
    picture: &Picture,
) -> (ffmpeg::Dictionary<'static>, Option<usize>) {
    let mut options = ffmpeg::Dictionary::new();
    // Bits a pixel takes for good quality, about.
    let per_pixel = match format {
        Format::H264 => 0.1,
        Format::Hevc => 0.065,
        _ => 0.05,
    };
    let pixels = f64::from(picture.width) * f64::from(picture.height);
    let estimate = ((pixels * f64::from(picture.rate) * per_pixel) as usize).max(1_000_000);
    // A level of quality where each encoder counts it, lower better.
    let level = match format {
        Format::H264 => 21,
        Format::Hevc => 24,
        _ => 30,
    };
    let mut bit_rate = None;
    if name.ends_with("_nvenc") {
        options.set("preset", "p5");
        options.set("tune", "hq");
        options.set("rc", "vbr");
        options.set("cq", &level.to_string());
        options.set("b", "0");
    } else if name.ends_with("_amf") {
        options.set("quality", "quality");
        options.set("rc", "qvbr");
        options.set("qvbr_quality_level", &level.to_string());
        bit_rate = Some(estimate);
    } else if name.ends_with("_qsv") {
        options.set("preset", "medium");
        options.set("global_quality", &level.to_string());
    } else if name.ends_with("_vaapi") {
        options.set("rc_mode", "QVBR");
        options.set("global_quality", &level.to_string());
        bit_rate = Some(estimate);
    } else if name.ends_with("_videotoolbox") {
        bit_rate = Some(estimate);
    } else {
        match name {
            "libx264" => {
                options.set("preset", "medium");
                options.set("crf", "20");
            }
            "libx265" => {
                options.set("preset", "fast");
                options.set("crf", "23");
                options.set("x265-params", "log-level=error");
            }
            "libsvtav1" => {
                options.set("preset", "8");
                options.set("crf", "32");
            }
            "libaom-av1" => {
                options.set("cpu-used", "6");
                options.set("crf", "32");
                options.set("row-mt", "1");
                options.set("b", "0");
            }
            _ => {}
        }
    }
    (options, bit_rate)
}

/// The size pictures come out at: the file's, `aspect` wide for each
/// unit tall, turned as `transform` says, its shorter side made `lines`
/// long when that is shorter, in even numbers of pixels.
pub fn output_size(
    (width, height): (u32, u32),
    aspect: f64,
    transform: Transform,
    lines: Option<u32>,
) -> (u32, u32) {
    let shown_width = if aspect.is_finite() && aspect > 0.0 {
        f64::from(height) * aspect
    } else {
        f64::from(width)
    };
    let (mut across, mut down) = (shown_width, f64::from(height));
    if transform.turns % 2 == 1 {
        std::mem::swap(&mut across, &mut down);
    }
    let shorter = across.min(down);
    if let Some(lines) = lines
        && f64::from(lines) < shorter
        && shorter > 0.0
    {
        let scale = f64::from(lines) / shorter;
        across *= scale;
        down *= scale;
    }
    let even = |size: f64| ((size / 2.0).round() as u32 * 2).max(2);
    (even(across), even(down))
}

/// The filters that turn, flip and size pictures for the encoder.
fn picture_filters(transform: Transform, size: (u32, u32), format: Pixel) -> String {
    let mut filters = Vec::new();
    if transform.mirrored {
        filters.push("hflip".to_owned());
    }
    match transform.turns % 4 {
        1 => filters.push("transpose=clock".to_owned()),
        2 => filters.push("hflip,vflip".to_owned()),
        3 => filters.push("transpose=cclock".to_owned()),
        _ => {}
    }
    filters.push(format!("scale={}:{}:flags=bicubic", size.0, size.1));
    filters.push(format!(
        "format={}",
        format
            .descriptor()
            .map_or("yuv420p", |descriptor| descriptor.name())
    ));
    filters.push("setsar=1".to_owned());
    filters.join(",")
}

/// The output's streams, as each is fed.
struct Output {
    context: ffmpeg::format::context::Output,
    /// Packets to write, through `interleaved` order.
    packet: ffmpeg::Packet,
}

impl Output {
    /// Takes every packet `encoder` has ready and writes it as stream
    /// `index`, from `time_base` to the stream's. Packets that do not
    /// say how long they last get `duration`, as an encoder's last ones
    /// may not, which would leave out the last picture.
    fn drain(
        &mut self,
        encoder: &mut ffmpeg::encoder::Encoder,
        index: usize,
        time_base: ffmpeg::Rational,
        duration: i64,
    ) -> Result<(), ExportError> {
        let stream_base = self
            .context
            .stream(index)
            .map_or(time_base, |stream| stream.time_base());
        while encoder.receive_packet(&mut self.packet).is_ok() {
            if self.packet.duration() <= 0 {
                self.packet.set_duration(duration);
            }
            self.packet.set_stream(index);
            self.packet.rescale_ts(time_base, stream_base);
            self.packet
                .write_interleaved(&mut self.context)
                .map_err(failed)?;
        }
        Ok(())
    }
}

fn failed(error: impl std::fmt::Display) -> ExportError {
    ExportError::Failed(error.to_string())
}

/// The picture's side of an export.
struct VideoSide {
    stream: usize,
    chosen: Encoder,
    decoder: ffmpeg::decoder::Video,
    time_base: ffmpeg::Rational,
    downloads: ffi::Downloads,
    encoder: VideoEncoder,
    output: usize,
    graph: Option<ffmpeg::filter::Graph>,
    filters: String,
    frame_duration: f64,
    last: Option<f64>,
}

/// The sound's side of an export.
struct AudioSide {
    stream: usize,
    decoder: ffmpeg::decoder::Audio,
    time_base: ffmpeg::Rational,
    encoder: ffmpeg::encoder::audio::Encoder,
    output: usize,
    graph: Option<ffmpeg::filter::Graph>,
    /// The file's time the next sound starts at, for frames without one.
    cursor: Option<f64>,
}

/// Runs `job`, writing to `temporary`.
fn run(
    job: &Job,
    temporary: &Path,
    stop: &AtomicBool,
    events: &dyn Fn(ExportEvent),
) -> Result<(), ExportError> {
    if same_file(&job.source, &job.destination) {
        return Err(ExportError::Failed(
            "a file cannot be exported over itself".to_owned(),
        ));
    }
    let mut input = ffmpeg::format::input(&job.source).map_err(failed)?;
    let start = ffi::start_time(&input);
    let info = MediaInfo::read(&input);
    let duration = info.duration.map_or(0.0, |duration| duration.as_secs_f64());
    let (spans, total, transform) = match &job.edit {
        Some(edit) => (edit.spans(), edit.duration(), edit.transform()),
        None => (vec![Span::WHOLE], duration, Transform::default()),
    };
    let sound_alone = match job.format {
        Format::Sound(sound) => Some(sound),
        _ => None,
    };
    let video_stream = job.video.filter(|_| sound_alone.is_none()).filter(|index| {
        info.video
            .iter()
            .any(|track| track.track.index == *index && !track.cover)
    });
    let audio_stream = job
        .audio
        .filter(|index| info.audio.iter().any(|track| track.track.index == *index));
    if video_stream.is_none() && audio_stream.is_none() {
        return Err(ExportError::Failed("nothing to export".to_owned()));
    }

    let muxer = sound_alone.map_or_else(|| muxer(&job.destination), Sound::muxer);
    let mut context = ffmpeg::format::output_as(&temporary, muxer).map_err(failed)?;
    let global_header = context
        .format()
        .flags()
        .contains(ffmpeg::format::Flags::GLOBAL_HEADER);
    context.set_metadata(input.metadata().to_owned());

    let mut video = match video_stream {
        Some(index) => Some(video_side(
            &input,
            &info,
            index,
            job,
            transform,
            &mut context,
            global_header,
        )?),
        None => None,
    };
    // Sound with a picture is AAC.
    let sound = sound_alone.unwrap_or(Sound::Aac);
    let quality = job
        .quality
        .filter(|_| sound_alone.is_some())
        .unwrap_or(match sound {
            Sound::Aac => 192,
            other => other.usual_quality(),
        });
    let mut audio = match audio_stream {
        Some(index) => Some(audio_side(
            &input,
            &info,
            index,
            (sound, quality),
            &mut context,
            global_header,
        )?),
        None => None,
    };
    let mut header = ffmpeg::Dictionary::new();
    header.set("movflags", "+faststart");
    context.write_header_with(header).map_err(failed)?;
    events(ExportEvent::Started(video.as_ref().map(|side| side.chosen)));
    let mut output = Output {
        context,
        packet: ffmpeg::Packet::empty(),
    };

    let mut packet = ffmpeg::Packet::empty();
    let mut reported = Instant::now();
    let mut done = 0.0;
    for span in &spans {
        if span.from.is_finite() {
            let timestamp = ((span.from + start) * f64::from(ffmpeg::ffi::AV_TIME_BASE)) as i64;
            input.seek(timestamp, ..timestamp).map_err(failed)?;
        }
        let playing: Vec<usize> = video
            .iter()
            .map(|side| side.stream)
            .chain(audio.iter().map(|side| side.stream))
            .collect();
        let mut passed: Vec<usize> = Vec::new();
        loop {
            if stop.load(Ordering::Relaxed) {
                return Err(ExportError::Cancelled);
            }
            match packet.read(&mut input) {
                Ok(()) => {}
                Err(ffmpeg::Error::Eof) => break,
                Err(ffmpeg::Error::InvalidData) => continue,
                Err(error) => return Err(failed(error)),
            }
            let index = packet.stream();
            if !playing.contains(&index) {
                continue;
            }
            // Past the part's end, as the player reads it.
            if let Some((seconds, decoding)) = order(&input, &packet, start) {
                let margin = if decoding { 0.0 } else { 1.0 };
                if seconds >= span.to + margin {
                    if seconds >= span.to + 3.0 {
                        passed = playing.clone();
                    } else if !passed.contains(&index) {
                        passed.push(index);
                    }
                    if playing.iter().all(|stream| passed.contains(stream)) {
                        break;
                    }
                    continue;
                }
            }
            if let Some(side) = video.as_mut().filter(|side| side.stream == index) {
                if side.decoder.send_packet(&packet).is_ok() {
                    side.receive(span, start, &mut output, &mut done)?;
                }
            } else if let Some(side) = audio.as_mut().filter(|side| side.stream == index)
                && side.decoder.send_packet(&packet).is_ok()
            {
                side.receive(span, start, &mut output, &mut done)?;
            }
            if reported.elapsed() >= Duration::from_millis(250) && total > 0.0 {
                reported = Instant::now();
                events(ExportEvent::Progress((done / total).clamp(0.0, 1.0)));
            }
        }
        // What the decoders still hold of this part.
        if let Some(side) = &mut video {
            let _ = side.decoder.send_eof();
            side.receive(span, start, &mut output, &mut done)?;
            side.decoder.flush();
            side.last = None;
        }
        if let Some(side) = &mut audio {
            let _ = side.decoder.send_eof();
            side.receive(span, start, &mut output, &mut done)?;
            side.decoder.flush();
            side.cursor = None;
        }
    }
    if let Some(side) = &mut video {
        side.finish(&mut output)?;
    }
    if let Some(side) = &mut audio {
        side.finish(&mut output)?;
    }
    output.context.write_trailer().map_err(failed)?;
    events(ExportEvent::Progress(1.0));
    Ok(())
}

/// Whether two paths name the same file.
fn same_file(first: &Path, second: &Path) -> bool {
    match (first.canonicalize(), second.canonicalize()) {
        (Ok(first), Ok(second)) => first == second,
        _ => first == second,
    }
}

/// FFmpeg's muxer for a file named `path`, by its ending: MP4 when it has
/// none it knows.
fn muxer(path: &Path) -> &'static str {
    let extension = path
        .extension()
        .map(|extension| extension.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    match extension.as_str() {
        "mov" => "mov",
        "m4a" => "ipod",
        "mkv" | "mka" => "matroska",
        "webm" => "webm",
        _ => "mp4",
    }
}

/// Where `packet` comes in the file's time, by its decoding time where
/// it has one, which is in order, else its presentation time; and which.
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

/// Sets up the picture's side: its decoder, an encoder that opens, and
/// its stream in the output.
fn video_side(
    input: &ffmpeg::format::context::Input,
    info: &MediaInfo,
    index: usize,
    job: &Job,
    transform: Transform,
    context: &mut ffmpeg::format::context::Output,
    global_header: bool,
) -> Result<VideoSide, ExportError> {
    let stream = input
        .stream(index)
        .ok_or_else(|| failed("the picture's stream is gone"))?;
    let track = info
        .video
        .iter()
        .find(|track| track.track.index == index)
        .ok_or_else(|| failed("the picture's stream is gone"))?;
    let (width, height, aspect) = (track.width.max(1), track.height.max(1), track.aspect);
    let hdr = track.hdr;
    let own = ffi::quarter_turns(&stream);
    let shown = transform.after_turns(own);
    let size = output_size((width, height), aspect, shown, job.lines);
    let rate = Some(stream.avg_frame_rate())
        .filter(|rate| rate.numerator() > 0 && rate.denominator() > 0)
        .unwrap_or_else(|| ffmpeg::Rational::new(30, 1));
    let picture = Picture {
        width: size.0,
        height: size.1,
        rate,
        time_base: stream.time_base(),
        ten_bit: ffi::bit_depth(&stream) > 8,
    };
    // A card that will not take ten bits encodes eight, unless the
    // picture is HDR, which needs them.
    let eight_bit = Picture {
        ten_bit: false,
        ..picture
    };
    let encoder = candidates(job.format)
        .into_iter()
        .filter(|candidate| job.hardware || !candidate.hardware)
        .find_map(|candidate| {
            let open = |picture: &Picture| {
                open_video(candidate, job.format, picture, Some(&stream), global_header)
            };
            open(&picture)
                .or_else(|| {
                    (candidate.hardware && picture.ten_bit && !hdr)
                        .then(|| open(&eight_bit))
                        .flatten()
                })
                .map(|opened| (candidate, opened))
        });
    let Some((candidate, encoder)) = encoder else {
        return Err(failed("no encoder for this format opens"));
    };
    let codec = ffmpeg::encoder::find_by_name(candidate.name)
        .ok_or_else(|| failed("the encoder is gone"))?;
    let mut out = context.add_stream(codec).map_err(failed)?;
    out.set_parameters(&encoder.encoder);
    out.set_time_base(picture.time_base);
    if job.format == Format::Hevc {
        ffi::set_codec_tag(&mut out, b"hvc1");
    }
    let output = out.index();
    let (decoder, time_base, _) =
        crate::decode::video_decoder(input, index, job.hardware).map_err(failed)?;
    let frame_duration = 1.0 / f64::from(rate).max(1.0);
    let filters = picture_filters(shown, size, encoder.format);
    Ok(VideoSide {
        stream: index,
        chosen: candidate,
        decoder,
        time_base,
        downloads: ffi::Downloads::default(),
        encoder,
        output,
        graph: None,
        filters,
        frame_duration,
        last: None,
    })
}

impl VideoSide {
    /// Takes every picture the decoder has, keeps those within `span`, and
    /// encodes them at their place in the edited time.
    fn receive(
        &mut self,
        span: &Span,
        start: f64,
        output: &mut Output,
        done: &mut f64,
    ) -> Result<(), ExportError> {
        let mut decoded = ffmpeg::frame::Video::empty();
        while self.decoder.receive_frame(&mut decoded).is_ok() {
            let source = decoded
                .timestamp()
                .or(decoded.pts())
                .and_then(|timestamp| info::seconds(timestamp, self.time_base))
                .map(|seconds| seconds - start)
                .or_else(|| self.last.map(|last| last + self.frame_duration))
                .unwrap_or(0.0);
            self.last = Some(source);
            let middle = source + self.frame_duration / 2.0;
            if middle < span.from || middle >= span.to {
                continue;
            }
            let edited = source + span.shift;
            let mut frame = if ffi::is_hardware(&decoded) {
                self.downloads.download(&decoded).map_err(failed)?
            } else {
                std::mem::replace(&mut decoded, ffmpeg::frame::Video::empty())
            };
            let base = self.encoder.encoder.time_base();
            frame.set_pts(Some((edited / f64::from(base)).round() as i64));
            if self.graph.is_none() {
                self.graph = Some(self.build(&frame)?);
            }
            if let Some(graph) = &mut self.graph
                && let Some(mut source) = graph.get("in")
            {
                source.source().add(&frame).map_err(failed)?;
            }
            self.pull(output)?;
            *done = done.max(edited);
        }
        Ok(())
    }

    /// The filter graph for pictures like `frame`.
    fn build(&self, frame: &ffmpeg::frame::Video) -> Result<ffmpeg::filter::Graph, ExportError> {
        let mut graph = ffmpeg::filter::Graph::new();
        let base = self.encoder.encoder.time_base();
        let sample = frame.aspect_ratio();
        let sample = if sample.numerator() > 0 && sample.denominator() > 0 {
            sample
        } else {
            ffmpeg::Rational::new(1, 1)
        };
        let arguments = format!(
            "video_size={}x{}:pix_fmt={}:time_base={}/{}:pixel_aspect={}/{}",
            frame.width(),
            frame.height(),
            frame
                .format()
                .descriptor()
                .map_or("yuv420p", |descriptor| descriptor.name()),
            base.numerator(),
            base.denominator().max(1),
            sample.numerator(),
            sample.denominator(),
        );
        let buffer = ffmpeg::filter::find("buffer").ok_or_else(|| failed("no buffer filter"))?;
        let sink =
            ffmpeg::filter::find("buffersink").ok_or_else(|| failed("no buffersink filter"))?;
        graph.add(&buffer, "in", &arguments).map_err(failed)?;
        graph.add(&sink, "out", "").map_err(failed)?;
        graph
            .output("in", 0)
            .and_then(|parser| parser.input("out", 0))
            .and_then(|parser| parser.parse(&self.filters))
            .map_err(failed)?;
        graph.validate().map_err(failed)?;
        Ok(graph)
    }

    /// Encodes what the graph has ready.
    fn pull(&mut self, output: &mut Output) -> Result<(), ExportError> {
        let Some(graph) = &mut self.graph else {
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

    /// Lets the last pictures out of the graph and the encoder.
    fn finish(&mut self, output: &mut Output) -> Result<(), ExportError> {
        if let Some(graph) = &mut self.graph
            && let Some(mut source) = graph.get("in")
        {
            let _ = source.source().flush();
        }
        self.pull(output)?;
        let _ = self.encoder.encoder.send_eof();
        let base = self.encoder.encoder.time_base();
        let duration = (self.frame_duration / f64::from(base)).round().max(1.0) as i64;
        output.drain(&mut self.encoder.encoder, self.output, base, duration)
    }
}

/// The sample format to encode `sound` at `quality` in, of those the
/// encoder takes, if it says.
fn sample_format(
    sound: Sound,
    quality: u32,
    takes: &[ffmpeg::format::Sample],
) -> Option<ffmpeg::format::Sample> {
    use ffmpeg::format::Sample::{F32, I16, I32};
    use ffmpeg::format::sample::Type::{Packed, Planar};
    let wanted: &[ffmpeg::format::Sample] = if !sound.lossless() {
        &[
            F32(Planar),
            F32(Packed),
            I16(Packed),
            I16(Planar),
            I32(Planar),
        ]
    } else if quality > 16 {
        &[I32(Packed), I32(Planar)]
    } else {
        &[I16(Packed), I16(Planar)]
    };
    wanted
        .iter()
        .copied()
        .find(|sample| takes.is_empty() || takes.contains(sample))
}

/// The sample rate to encode at: the file's, where the encoder takes it,
/// else 48 kHz, else the highest it takes.
fn sample_rate(file: u32, takes: &[i32]) -> u32 {
    let takes: Vec<u32> = takes
        .iter()
        .filter_map(|rate| u32::try_from(*rate).ok())
        .collect();
    if takes.is_empty() || takes.contains(&file) {
        file
    } else if takes.contains(&48000) {
        48000
    } else {
        takes.iter().copied().max().unwrap_or(48000)
    }
}

/// Sets up the sound's side: its decoder, an encoder for `sound` at
/// `quality`, and its stream in the output.
fn audio_side(
    input: &ffmpeg::format::context::Input,
    info: &MediaInfo,
    index: usize,
    (sound, quality): (Sound, u32),
    context: &mut ffmpeg::format::context::Output,
    global_header: bool,
) -> Result<AudioSide, ExportError> {
    let stream = input
        .stream(index)
        .ok_or_else(|| failed("the sound's stream is gone"))?;
    let (decoder, time_base) = crate::decode::open_decoder(input, index).map_err(failed)?;
    let decoder = decoder.audio().map_err(failed)?;
    let codec = sound
        .encoders(quality)
        .iter()
        .find_map(|name| ffmpeg::encoder::find_by_name(name))
        .ok_or_else(|| failed("no encoder for this sound format"))?;
    let audio = codec.audio().map_err(failed)?;
    let rates: Vec<i32> = audio.rates().map(Iterator::collect).unwrap_or_default();
    let file_rate = info
        .audio
        .iter()
        .find(|track| track.track.index == index)
        .map_or(decoder.rate(), |track| track.sample_rate);
    let rate = sample_rate(file_rate.max(1), &rates);
    // MP3 holds two channels at most.
    let most = if sound == Sound::Mp3 { 2 } else { 8 };
    let channels = decoder.channels().clamp(1, most);
    let formats: Vec<ffmpeg::format::Sample> =
        audio.formats().map(Iterator::collect).unwrap_or_default();
    let sample = sample_format(sound, quality, &formats)
        .ok_or_else(|| failed("no sample format the sound encoder takes"))?;
    let mut encoder = ffmpeg::codec::Context::new_with_codec(codec);
    if sound.lossless() {
        ffi::set_sample_bits(&mut encoder, quality);
    }
    let mut encoder = encoder.encoder().audio().map_err(failed)?;
    encoder.set_rate(rate as i32);
    encoder.set_channel_layout(ffmpeg::ChannelLayout::default(i32::from(channels)));
    encoder.set_format(sample);
    encoder.set_time_base(ffmpeg::Rational::new(1, rate as i32));
    if !sound.lossless() {
        // The quality counts for two channels.
        let per_channel = quality as usize * 1000 / 2;
        encoder.set_bit_rate(per_channel * usize::from(channels.max(2)));
    }
    if global_header {
        encoder.set_flags(ffmpeg::codec::Flags::GLOBAL_HEADER);
    }
    let encoder = encoder
        .open_as_with(codec, ffmpeg::Dictionary::new())
        .map_err(failed)?;
    let mut out = context.add_stream(codec).map_err(failed)?;
    out.set_parameters(&encoder);
    out.set_time_base(ffmpeg::Rational::new(1, rate as i32));
    out.set_metadata(stream.metadata().to_owned());
    let output = out.index();
    Ok(AudioSide {
        stream: index,
        decoder,
        time_base,
        encoder,
        output,
        graph: None,
        cursor: None,
    })
}

impl AudioSide {
    /// Takes the sound the decoder has, keeps what is within `span`, and
    /// encodes it at its place in the edited time.
    fn receive(
        &mut self,
        span: &Span,
        start: f64,
        output: &mut Output,
        done: &mut f64,
    ) -> Result<(), ExportError> {
        let mut decoded = ffmpeg::frame::Audio::empty();
        while self.decoder.receive_frame(&mut decoded).is_ok() {
            let rate = f64::from(decoded.rate().max(1));
            let samples = decoded.samples();
            let source = decoded
                .timestamp()
                .or(decoded.pts())
                .and_then(|timestamp| info::seconds(timestamp, self.time_base))
                .map(|seconds| seconds - start)
                .or(self.cursor)
                .unwrap_or(0.0);
            let end = source + samples as f64 / rate;
            self.cursor = Some(end);
            if end <= span.from || source >= span.to {
                continue;
            }
            let skip = if source < span.from {
                (((span.from - source) * rate).round() as usize).min(samples)
            } else {
                0
            };
            let until = if end > span.to {
                (((span.to - source) * rate).round() as usize).min(samples)
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
            let edited = source + skip as f64 / rate + span.shift;
            frame.set_pts(Some((edited * rate).round() as i64));
            if self.graph.is_none() {
                self.graph = Some(self.build(&frame)?);
            }
            if let Some(graph) = &mut self.graph
                && let Some(mut source) = graph.get("in")
            {
                source.source().add(&frame).map_err(failed)?;
            }
            self.pull(output)?;
            *done = done.max(edited);
        }
        Ok(())
    }

    /// The graph that turns sound like `frame` into the encoder's format,
    /// in frames of the size it takes, filling any gap between parts.
    fn build(&self, frame: &ffmpeg::frame::Audio) -> Result<ffmpeg::filter::Graph, ExportError> {
        let mut graph = ffmpeg::filter::Graph::new();
        let arguments = format!(
            "time_base=1/{}:sample_rate={}:sample_fmt={}:channel_layout={}",
            frame.rate(),
            frame.rate(),
            frame.format().name(),
            ffi::frame_layout(frame),
        );
        let abuffer = ffmpeg::filter::find("abuffer").ok_or_else(|| failed("no abuffer filter"))?;
        let sink =
            ffmpeg::filter::find("abuffersink").ok_or_else(|| failed("no abuffersink filter"))?;
        graph.add(&abuffer, "in", &arguments).map_err(failed)?;
        graph.add(&sink, "out", "").map_err(failed)?;
        let mut filters = format!(
            "aresample={rate}:async=1:first_pts=0,\
             aformat=sample_fmts={format}:channel_layouts={layout}",
            rate = self.encoder.rate(),
            format = self.encoder.format().name(),
            layout = ffi::default_layout(i32::from(self.encoder.channels())),
        );
        // In the frames the encoder takes, unless it takes any; the last
        // one short, which FFmpeg pads for encoders that need it, so
        // lossless sound ends where it does.
        let size = self.encoder.frame_size();
        if size > 0 {
            filters.push_str(&format!(",asetnsamples=n={size}:p=0"));
        }
        graph
            .output("in", 0)
            .and_then(|parser| parser.input("out", 0))
            .and_then(|parser| parser.parse(&filters))
            .map_err(failed)?;
        graph.validate().map_err(failed)?;
        Ok(graph)
    }

    fn pull(&mut self, output: &mut Output) -> Result<(), ExportError> {
        let Some(graph) = &mut self.graph else {
            return Ok(());
        };
        let mut filtered = ffmpeg::frame::Audio::empty();
        while let Some(mut sink) = graph.get("out")
            && sink.sink().frame(&mut filtered).is_ok()
        {
            self.encoder.send_frame(&filtered).map_err(failed)?;
            let base = self.encoder.time_base();
            output.drain(&mut self.encoder, self.output, base, 0)?;
        }
        Ok(())
    }

    fn finish(&mut self, output: &mut Output) -> Result<(), ExportError> {
        if let Some(graph) = &mut self.graph
            && let Some(mut source) = graph.get("in")
        {
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
    fn pictures_keep_their_shape_turned_and_made_smaller() {
        let plain = Transform::default();
        let turned = Transform {
            turns: 1,
            mirrored: false,
        };
        assert_eq!(
            output_size((1920, 1080), 16.0 / 9.0, plain, None),
            (1920, 1080)
        );
        assert_eq!(
            output_size((1920, 1080), 16.0 / 9.0, turned, None),
            (1080, 1920)
        );
        // The shorter side is the one asked for, turned or not.
        assert_eq!(
            output_size((3840, 2160), 16.0 / 9.0, plain, Some(720)),
            (1280, 720)
        );
        assert_eq!(
            output_size((3840, 2160), 16.0 / 9.0, turned, Some(720)),
            (720, 1280)
        );
        // Never larger, and anamorphic pictures come out square.
        assert_eq!(
            output_size((640, 360), 16.0 / 9.0, plain, Some(1080)),
            (640, 360)
        );
        assert_eq!(
            output_size((720, 576), 16.0 / 9.0, plain, None),
            (1024, 576)
        );
    }

    #[test]
    fn filters_turn_then_size() {
        let transform = Transform {
            turns: 1,
            mirrored: true,
        };
        assert_eq!(
            picture_filters(transform, (720, 1280), Pixel::NV12),
            "hflip,transpose=clock,scale=720:1280:flags=bicubic,format=nv12,setsar=1"
        );
    }

    #[test]
    fn sound_is_encoded_as_its_encoder_takes_it() {
        use ffmpeg::format::Sample::{F32, I16, I32};
        use ffmpeg::format::sample::Type::{Packed, Planar};
        assert_eq!(
            sample_format(Sound::Flac, 24, &[I16(Packed), I32(Packed)]),
            Some(I32(Packed))
        );
        assert_eq!(
            sample_format(Sound::Alac, 16, &[I16(Planar), I32(Planar)]),
            Some(I16(Planar))
        );
        assert_eq!(
            sample_format(Sound::Opus, 128, &[I16(Packed), F32(Packed)]),
            Some(F32(Packed))
        );
        // Opus takes 48 kHz, not 44.1.
        assert_eq!(sample_rate(44100, &[48000, 24000, 16000]), 48000);
        assert_eq!(sample_rate(44100, &[]), 44100);
        assert_eq!(sample_rate(96000, &[44100, 32000]), 44100);
        for sound in Sound::ALL {
            assert!(sound.qualities().contains(&sound.usual_quality()));
        }
    }

    #[test]
    fn files_are_written_by_their_ending() {
        assert_eq!(muxer(Path::new("a.MOV")), "mov");
        assert_eq!(muxer(Path::new("a.m4a")), "ipod");
        assert_eq!(muxer(Path::new("a")), "mp4");
        assert_eq!(
            temporary_path(Path::new("/tmp/clip.mp4")),
            Path::new("/tmp/.clip.mp4.part")
        );
    }
}
