//! Recording the screen: pictures a capture hands over, as the ScreenCast
//! portal shares them through PipeWire on Linux, encoded as exports
//! encode pictures, with the sound of a recording input when one is
//! given, into an MP4.

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use ffmpeg::format::Pixel;
use ffmpeg_next as ffmpeg;

use crate::export::{self, Colors, ExportError, Format, Output, Picture, Sound};
use crate::ffi;
use crate::record::{RecordEvent, SoundTap};

/// Pictures waiting for the encoder; more than this, and the newest are
/// dropped until it catches up.
#[cfg(any(target_os = "linux", test))]
const MOST_WAITING: usize = 8;
/// How long the capture may take to say its size.
const FIRST_PICTURE: Duration = Duration::from_secs(5);
/// How often a picture is kept while not recording: a still screen sends
/// none, so the last one kept starts a recording.
#[cfg(any(target_os = "linux", test))]
const KEEP_EVERY: Duration = Duration::from_millis(250);
/// The frame rate encoders are told, for their rate control; pictures
/// come as the screen changes.
const RATE: i32 = 60;
/// Timestamps count microseconds.
const TIME_BASE: ffmpeg::Rational = ffmpeg::Rational(1, 1_000_000);

type Taken = (Instant, ffmpeg::frame::Video);

/// Pictures a capture hands over, with when each came: each while
/// recording, and now and then otherwise.
pub(crate) struct Frames {
    waiting: Mutex<Waiting>,
    arrived: Condvar,
}

#[derive(Default)]
struct Waiting {
    /// The pictures for the encoder, while recording.
    queue: Option<VecDeque<Taken>>,
    /// The last picture kept.
    latest: Option<Taken>,
}

impl Frames {
    #[cfg(any(target_os = "linux", test))]
    fn new() -> Self {
        Self {
            waiting: Mutex::new(Waiting::default()),
            arrived: Condvar::new(),
        }
    }

    fn waiting(&self) -> MutexGuard<'_, Waiting> {
        self.waiting.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Whether a picture that came at `at` is wanted.
    #[cfg(any(target_os = "linux", test))]
    pub fn wanted(&self, at: Instant) -> bool {
        let waiting = self.waiting();
        waiting.queue.is_some()
            || waiting
                .latest
                .as_ref()
                .is_none_or(|(kept, _)| at.saturating_duration_since(*kept) >= KEEP_EVERY)
    }

    /// Hands over a picture taken at `at`, unless too many wait.
    #[cfg(any(target_os = "linux", test))]
    pub fn push(&self, at: Instant, frame: ffmpeg::frame::Video) {
        let mut waiting = self.waiting();
        match waiting.queue.as_mut() {
            Some(queue) => {
                if queue.len() < MOST_WAITING {
                    queue.push_back((at, frame));
                    self.arrived.notify_one();
                }
            }
            None => waiting.latest = Some((at, frame)),
        }
    }

    /// Starts taking every picture, the last one kept first.
    fn start(&self) {
        let mut waiting = self.waiting();
        let latest = waiting.latest.take();
        waiting.queue = Some(latest.into_iter().collect());
    }

    fn end(&self) {
        self.waiting().queue = None;
        self.arrived.notify_all();
    }

    /// The pictures waiting, waiting up to `wait` for one.
    fn take(&self, wait: Duration) -> Vec<Taken> {
        let mut waiting = self.waiting();
        if waiting.queue.as_ref().is_some_and(VecDeque::is_empty) && !wait.is_zero() {
            waiting = self
                .arrived
                .wait_timeout(waiting, wait)
                .unwrap_or_else(PoisonError::into_inner)
                .0;
        }
        waiting
            .queue
            .as_mut()
            .map(|queue| queue.drain(..).collect())
            .unwrap_or_default()
    }
}

/// How far a recording has got: microseconds recorded, and bytes
/// written.
#[derive(Default)]
struct Progress {
    recorded: AtomicU64,
    written: AtomicU64,
}

/// A recording's writing thread, and the flag that ends it.
struct Recording {
    finish: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

/// The screen, or part of it, as a capture shares it, ready to record.
/// Dropping it finishes any recording, then ends the capture.
pub struct ScreenCapture {
    frames: Arc<Frames>,
    shared: Arc<CaptureShared>,
    progress: Arc<Progress>,
    recording: Option<Recording>,
    #[cfg(target_os = "linux")]
    capture: Option<pipewire_capture::Capture>,
}

/// What a capture says of itself.
#[derive(Default)]
struct CaptureShared {
    /// The size of its pictures, once known.
    size: Mutex<Option<(u32, u32)>>,
    /// Why it ended, if it did: the person stopped sharing, or it failed.
    ended: Mutex<Option<String>>,
}

impl ScreenCapture {
    /// Takes the pictures of stream `node` of the PipeWire remote `fd`,
    /// as the ScreenCast portal gives them.
    #[cfg(target_os = "linux")]
    pub fn open(fd: std::os::fd::OwnedFd, node: u32) -> Result<Self, String> {
        crate::init();
        let frames = Arc::new(Frames::new());
        let shared = Arc::new(CaptureShared::default());
        let capture = pipewire_capture::Capture::open(fd, node, frames.clone(), shared.clone())?;
        Ok(Self {
            frames,
            shared,
            progress: Arc::new(Progress::default()),
            recording: None,
            capture: Some(capture),
        })
    }

    /// The size of the pictures, once the stream says.
    pub fn size(&self) -> Option<(u32, u32)> {
        *self
            .shared
            .size
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    /// Why the capture ended, if it did.
    pub fn ended_because(&self) -> Option<String> {
        self.shared
            .ended
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    /// Starts recording to `destination` in `format`, on a graphics card
    /// when `hardware` allows, with the sound of `microphone` when given;
    /// `events` hears when it ends, from its thread.
    pub fn record(
        &mut self,
        microphone: Option<&crate::record::Microphone>,
        (format, hardware): (Format, bool),
        destination: PathBuf,
        events: impl Fn(RecordEvent) + Send + 'static,
    ) -> Result<(), String> {
        if self.recording.is_some() {
            return Err("already recording".to_owned());
        }
        if matches!(format, Format::Sound(_)) {
            return Err("a screen is recorded as video".to_owned());
        }
        self.progress.recorded.store(0, Ordering::Relaxed);
        self.progress.written.store(0, Ordering::Relaxed);
        let sound = microphone.map(crate::record::Microphone::tap);
        let finish = Arc::new(AtomicBool::new(false));
        let thread = {
            let (frames, shared, progress, finish) = (
                self.frames.clone(),
                self.shared.clone(),
                self.progress.clone(),
                finish.clone(),
            );
            std::thread::Builder::new()
                .name("plaa-screen-record".into())
                .spawn(move || {
                    let temporary = export::temporary_path(&destination);
                    let written = write(
                        (&frames, &shared, sound.as_ref()),
                        (format, hardware),
                        &temporary,
                        (&finish, &progress),
                    );
                    frames.end();
                    if let Some(sound) = &sound {
                        sound.end();
                    }
                    let result = written.map_err(|error| error.to_string()).and_then(|()| {
                        std::fs::rename(&temporary, &destination).map_err(|error| error.to_string())
                    });
                    if result.is_err() {
                        let _ = std::fs::remove_file(&temporary);
                    }
                    events(RecordEvent::Finished(result.map(|()| destination)));
                })
                .map_err(|error| error.to_string())?
        };
        self.recording = Some(Recording {
            finish,
            thread: Some(thread),
        });
        Ok(())
    }

    /// Ends the recording: what was recorded is written out, and the event
    /// says so.
    pub fn stop(&mut self) {
        if let Some(recording) = &self.recording {
            recording.finish.store(true, Ordering::Relaxed);
            self.frames.arrived.notify_all();
        }
    }

    pub fn is_recording(&self) -> bool {
        self.recording
            .as_ref()
            .is_some_and(|recording| !recording.finish.load(Ordering::Relaxed))
    }

    /// How long the recording lasts so far, and how many bytes it takes.
    pub fn recorded(&self) -> (Duration, u64) {
        (
            Duration::from_micros(self.progress.recorded.load(Ordering::Relaxed)),
            self.progress.written.load(Ordering::Relaxed),
        )
    }

    /// Forgets a recording that has ended.
    pub fn ended(&mut self) {
        if let Some(mut recording) = self.recording.take()
            && let Some(thread) = recording.thread.take()
        {
            let _ = thread.join();
        }
    }
}

impl Drop for ScreenCapture {
    fn drop(&mut self) {
        self.stop();
        self.ended();
        #[cfg(target_os = "linux")]
        drop(self.capture.take());
    }
}

/// The picture side of a screen recording: its encoder and the graph
/// that makes pictures as it takes them.
struct VideoSide {
    encoder: export::VideoEncoder,
    size: (u32, u32),
    graph: Option<(ffmpeg::filter::Graph, (u32, u32, Pixel))>,
}

impl VideoSide {
    /// Encodes `frame`, taken `micros` into the recording.
    fn encode(
        &mut self,
        mut frame: ffmpeg::frame::Video,
        micros: i64,
        output: &mut Output,
    ) -> Result<(), ExportError> {
        let shape = (frame.width(), frame.height(), frame.format());
        if self.graph.as_ref().is_none_or(|(_, built)| *built != shape) {
            // A window captured may change size: its pictures are fitted
            // to the recording's.
            self.graph = Some((self.build(shape)?, shape));
        }
        frame.set_pts(Some(micros));
        if let Some((graph, _)) = &mut self.graph
            && let Some(mut source) = graph.get("in")
        {
            source.source().add(&frame).map_err(export::failed)?;
        }
        self.pull(output)
    }

    fn build(
        &self,
        (width, height, format): (u32, u32, Pixel),
    ) -> Result<ffmpeg::filter::Graph, ExportError> {
        let mut graph = ffmpeg::filter::Graph::new();
        let arguments = format!(
            "video_size={width}x{height}:pix_fmt={}:time_base=1/1000000:pixel_aspect=1/1",
            format
                .descriptor()
                .map_or("bgr0", |descriptor| descriptor.name()),
        );
        let buffer =
            ffmpeg::filter::find("buffer").ok_or_else(|| export::failed("no buffer filter"))?;
        let sink = ffmpeg::filter::find("buffersink")
            .ok_or_else(|| export::failed("no buffersink filter"))?;
        graph
            .add(&buffer, "in", &arguments)
            .map_err(export::failed)?;
        graph.add(&sink, "out", "").map_err(export::failed)?;
        // Encoders that take RGB make it YUV as they please, NVENC by
        // BT.601, so it is done here, by the BT.709 the file says.
        let filters = format!(
            "scale={}:{}:flags=bicubic:out_color_matrix=bt709:out_range=tv:threads={},format={}",
            self.size.0,
            self.size.1,
            std::thread::available_parallelism().map_or(1, |count| count.get().min(4)),
            self.encoder
                .format
                .descriptor()
                .map_or("nv12", |descriptor| descriptor.name()),
        );
        graph
            .output("in", 0)
            .and_then(|parser| parser.input("out", 0))
            .and_then(|parser| parser.parse(&filters))
            .map_err(export::failed)?;
        graph.validate().map_err(export::failed)?;
        Ok(graph)
    }

    fn pull(&mut self, output: &mut Output) -> Result<(), ExportError> {
        let Some((graph, _)) = &mut self.graph else {
            return Ok(());
        };
        let duration = i64::from(TIME_BASE.denominator() / RATE);
        let mut filtered = ffmpeg::frame::Video::empty();
        while let Some(mut sink) = graph.get("out")
            && sink.sink().frame(&mut filtered).is_ok()
        {
            let frame = match &self.encoder.frames {
                Some(frames) => frames
                    .upload(&filtered)
                    .ok_or_else(|| export::failed("copying a picture to the graphics card"))?,
                None => std::mem::replace(&mut filtered, ffmpeg::frame::Video::empty()),
            };
            self.encoder
                .encoder
                .send_frame(&frame)
                .map_err(export::failed)?;
            output.drain(&mut self.encoder.encoder, 0, TIME_BASE, duration)?;
        }
        Ok(())
    }

    fn finish(&mut self, output: &mut Output) -> Result<(), ExportError> {
        if let Some((graph, _)) = &mut self.graph
            && let Some(mut source) = graph.get("in")
        {
            let _ = source.source().flush();
        }
        self.pull(output)?;
        let _ = self.encoder.encoder.send_eof();
        let duration = i64::from(TIME_BASE.denominator() / RATE);
        output.drain(&mut self.encoder.encoder, 0, TIME_BASE, duration)
    }
}

/// The sound side: its encoder, and the graph that makes the input's
/// sound what it takes.
struct SoundSide {
    encoder: ffmpeg::encoder::audio::Encoder,
    graph: ffmpeg::filter::Graph,
    format: (u32, u16),
    /// Frames of sound taken so far, which time it.
    taken: i64,
}

impl SoundSide {
    fn encode(&mut self, samples: &[f32], output: &mut Output) -> Result<(), ExportError> {
        let channels = usize::from(self.format.1.max(1));
        let frames = samples.len() / channels;
        if frames == 0 {
            return Ok(());
        }
        let mut frame = ffmpeg::frame::Audio::new(
            ffmpeg::format::Sample::F32(ffmpeg::format::sample::Type::Packed),
            frames,
            ffmpeg::ChannelLayout::default(i32::from(self.format.1)),
        );
        frame.set_rate(self.format.0);
        for (target, sample) in frame
            .data_mut(0)
            .as_chunks_mut::<4>()
            .0
            .iter_mut()
            .zip(&samples[..frames * channels])
        {
            *target = sample.to_ne_bytes();
        }
        frame.set_pts(Some(self.taken));
        self.taken += frames as i64;
        if let Some(mut source) = self.graph.get("in") {
            source.source().add(&frame).map_err(export::failed)?;
        }
        self.pull(output)
    }

    fn pull(&mut self, output: &mut Output) -> Result<(), ExportError> {
        let base = self.encoder.time_base();
        let mut filtered = ffmpeg::frame::Audio::empty();
        while let Some(mut sink) = self.graph.get("out")
            && sink.sink().frame(&mut filtered).is_ok()
        {
            self.encoder.send_frame(&filtered).map_err(export::failed)?;
            output.drain(&mut self.encoder, 1, base, 0)?;
        }
        Ok(())
    }

    fn finish(&mut self, output: &mut Output) -> Result<(), ExportError> {
        if let Some(mut source) = self.graph.get("in") {
            let _ = source.source().flush();
        }
        self.pull(output)?;
        let _ = self.encoder.send_eof();
        let base = self.encoder.time_base();
        output.drain(&mut self.encoder, 1, base, 0)
    }
}

/// Writes the recording into `temporary` until `finish` is set. Pictures
/// and sound are taken once the encoders are open, which takes a moment
/// on a graphics card, from when the recording starts.
fn write(
    (frames, shared, sound): (&Frames, &CaptureShared, Option<&SoundTap>),
    (format, hardware): (Format, bool),
    temporary: &std::path::Path,
    (finish, progress): (&AtomicBool, &Progress),
) -> Result<(), ExportError> {
    let waited = Instant::now();
    let (width, height) = loop {
        let size = *shared.size.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(size) = size {
            break size;
        }
        if finish.load(Ordering::Relaxed) || waited.elapsed() > FIRST_PICTURE {
            return Err(export::failed("the screen sent no picture"));
        }
        std::thread::sleep(Duration::from_millis(20));
    };
    let even = |size: u32| (size / 2 * 2).max(2);
    let size = (even(width), even(height));
    let mut context = ffmpeg::format::output_as(&temporary, "mp4").map_err(export::failed)?;
    let global_header = context
        .format()
        .flags()
        .contains(ffmpeg::format::Flags::GLOBAL_HEADER);
    let picture = Picture {
        width: size.0,
        height: size.1,
        rate: ffmpeg::Rational::new(RATE, 1),
        time_base: TIME_BASE,
        ten_bit: false,
        live: true,
    };
    let (candidate, encoder) = export::candidates(format)
        .into_iter()
        .filter(|candidate| hardware || !candidate.hardware)
        .find_map(|candidate| {
            export::open_video(candidate, format, &picture, Colors::Screen, global_header)
                .map(|opened| (candidate, opened))
        })
        .ok_or_else(|| export::failed("no encoder for this format opens"))?;
    let codec = ffmpeg::encoder::find_by_name(candidate.name)
        .ok_or_else(|| export::failed("the encoder is gone"))?;
    {
        let mut stream = context.add_stream(codec).map_err(export::failed)?;
        stream.set_parameters(&encoder.encoder);
        stream.set_time_base(TIME_BASE);
        if format == Format::Hevc {
            ffi::set_codec_tag(&mut stream, b"hvc1");
        }
    }
    let mut sound_side = match sound {
        Some(sound) => {
            let (rate, channels) = sound.format();
            let (codec, encoder) =
                export::sound_encoder((Sound::Aac, 192), rate, channels, global_header)?;
            let mut stream = context.add_stream(codec).map_err(export::failed)?;
            stream.set_parameters(&encoder);
            stream.set_time_base(ffmpeg::Rational::new(1, encoder.rate() as i32));
            let layout = ffi::default_layout(i32::from(channels));
            let graph = export::sound_graph(
                (
                    rate,
                    ffmpeg::format::Sample::F32(ffmpeg::format::sample::Type::Packed),
                    &layout,
                ),
                &encoder,
            )?;
            Some(SoundSide {
                encoder,
                graph,
                format: (rate, channels),
                taken: 0,
            })
        }
        None => None,
    };
    let mut header = ffmpeg::Dictionary::new();
    header.set("movflags", "+faststart");
    context.write_header_with(header).map_err(export::failed)?;
    let mut output = Output {
        context,
        packet: ffmpeg::Packet::empty(),
        written: 0,
    };
    let mut video = VideoSide {
        encoder,
        size,
        graph: None,
    };
    let start = Instant::now();
    frames.start();
    if let Some(sound) = sound {
        sound.start();
    }
    let micros = |at: Instant| at.saturating_duration_since(start).as_micros() as i64;
    let mut last: Option<(i64, ffmpeg::frame::Video)> = None;
    let mut pending: Vec<Taken> = Vec::new();
    loop {
        let finishing = finish.load(Ordering::Relaxed);
        for (at, frame) in pending.drain(..) {
            let time = micros(at);
            // Each later than the last, as encoders need.
            if last.as_ref().is_some_and(|(previous, _)| time <= *previous) {
                continue;
            }
            last = Some((time, frame.clone()));
            video.encode(frame, time, &mut output)?;
            progress
                .recorded
                .store(time.max(0) as u64, Ordering::Relaxed);
        }
        if let (Some(sound), Some(side)) = (sound, &mut sound_side) {
            side.encode(&sound.take(Duration::ZERO), &mut output)?;
        }
        progress.written.store(output.written, Ordering::Relaxed);
        if finishing {
            break;
        }
        pending = frames.take(Duration::from_millis(20));
        if pending.is_empty() {
            // Nothing changed on screen: time still passes.
            progress
                .recorded
                .store(micros(Instant::now()) as u64, Ordering::Relaxed);
        }
    }
    // The last picture shows until the recording ends.
    let end = micros(Instant::now());
    let Some((time, frame)) = last else {
        return Err(export::failed("the screen sent no picture"));
    };
    if end > time {
        video.encode(frame, end, &mut output)?;
    }
    video.finish(&mut output)?;
    if let (Some(sound), Some(side)) = (sound, &mut sound_side) {
        side.encode(&sound.take(Duration::ZERO), &mut output)?;
        side.finish(&mut output)?;
    }
    output.context.write_trailer().map_err(export::failed)?;
    progress.written.store(output.written, Ordering::Relaxed);
    Ok(())
}

/// The ScreenCast portal's pictures, through PipeWire.
#[cfg(target_os = "linux")]
mod pipewire_capture {
    use std::os::fd::OwnedFd;
    use std::sync::{Arc, PoisonError, mpsc};
    use std::thread::JoinHandle;
    use std::time::{Duration, Instant};

    use ffmpeg::format::Pixel;
    use ffmpeg_next as ffmpeg;
    use pipewire as pw;
    use pw::spa;
    use pw::spa::param::video::VideoFormat;

    use super::{CaptureShared, Frames};

    /// The capture's thread, and the channel that ends it.
    pub struct Capture {
        quit: pw::channel::Sender<()>,
        thread: Option<JoinHandle<()>>,
    }

    impl Capture {
        pub fn open(
            fd: OwnedFd,
            node: u32,
            frames: Arc<Frames>,
            shared: Arc<CaptureShared>,
        ) -> Result<Self, String> {
            let (quit, quitting) = pw::channel::channel::<()>();
            let (ready, readied) = mpsc::channel();
            let thread = std::thread::Builder::new()
                .name("plaa-screen".into())
                .spawn(move || {
                    if let Err(error) = run(fd, node, &frames, &shared, quitting, &ready) {
                        let _ = ready.send(Err(error));
                    }
                })
                .map_err(|error| error.to_string())?;
            readied
                .recv_timeout(Duration::from_secs(5))
                .map_err(|_| "the screen's stream did not open".to_owned())??;
            Ok(Self {
                quit,
                thread: Some(thread),
            })
        }
    }

    impl Drop for Capture {
        fn drop(&mut self) {
            let _ = self.quit.send(());
            if let Some(thread) = self.thread.take() {
                let _ = thread.join();
            }
        }
    }

    /// The pixel format FFmpeg names a PipeWire video format by, for the
    /// four byte RGB ones screens come in.
    fn pixel(format: VideoFormat) -> Option<Pixel> {
        Some(match format {
            VideoFormat::BGRx => Pixel::BGRZ,
            VideoFormat::RGBx => Pixel::RGBZ,
            VideoFormat::xBGR => Pixel::ZBGR,
            VideoFormat::xRGB => Pixel::ZRGB,
            VideoFormat::BGRA => Pixel::BGRA,
            VideoFormat::RGBA => Pixel::RGBA,
            _ => return None,
        })
    }

    /// What the stream's callbacks keep: the format it settled on.
    struct State {
        format: Option<(Pixel, u32, u32)>,
    }

    fn run(
        fd: OwnedFd,
        node: u32,
        frames: &Arc<Frames>,
        shared: &Arc<CaptureShared>,
        quitting: pw::channel::Receiver<()>,
        ready: &mpsc::Sender<Result<(), String>>,
    ) -> Result<(), String> {
        pw::init();
        let failed = |error: pw::Error| error.to_string();
        let mainloop = pw::main_loop::MainLoopRc::new(None).map_err(failed)?;
        let context = pw::context::ContextRc::new(&mainloop, None).map_err(failed)?;
        let core = context.connect_fd_rc(fd, None).map_err(failed)?;
        let stream = pw::stream::StreamBox::new(
            &core,
            "plaa-screen",
            pw::properties::properties! {
                *pw::keys::MEDIA_TYPE => "Video",
                *pw::keys::MEDIA_CATEGORY => "Capture",
                *pw::keys::MEDIA_ROLE => "Screen",
            },
        )
        .map_err(failed)?;
        let (ended, sized, taken) = (shared.clone(), shared.clone(), frames.clone());
        let _listener = stream
            .add_local_listener_with_user_data(State { format: None })
            .state_changed(move |_, _, _, now| {
                let reason = match now {
                    pw::stream::StreamState::Error(error) => Some(error),
                    pw::stream::StreamState::Unconnected => Some(String::new()),
                    _ => None,
                };
                if let Some(reason) = reason {
                    *ended.ended.lock().unwrap_or_else(PoisonError::into_inner) = Some(reason);
                }
            })
            .param_changed(move |_, state, id, param| {
                let Some(param) = param else {
                    return;
                };
                if id != spa::param::ParamType::Format.as_raw() {
                    return;
                }
                let mut info = spa::param::video::VideoInfoRaw::default();
                if info.parse(param).is_err() {
                    return;
                }
                let size = info.size();
                state.format = pixel(info.format()).map(|pixel| (pixel, size.width, size.height));
                *sized.size.lock().unwrap_or_else(PoisonError::into_inner) =
                    Some((size.width, size.height));
            })
            .process(move |stream, state| {
                let Some(mut buffer) = stream.dequeue_buffer() else {
                    return;
                };
                let at = Instant::now();
                if !taken.wanted(at) {
                    return;
                }
                let Some((pixel, width, height)) = state.format else {
                    return;
                };
                let datas = buffer.datas_mut();
                let Some(data) = datas.first_mut() else {
                    return;
                };
                let (offset, stride) = (data.chunk().offset() as usize, data.chunk().stride());
                let row = width as usize * 4;
                let stride = if stride > 0 { stride as usize } else { row };
                let Some(bytes) = data.data() else {
                    return;
                };
                if bytes.len() < offset + stride * (height as usize - 1) + row {
                    return;
                }
                let mut frame = ffmpeg::frame::Video::new(pixel, width, height);
                let target_stride = frame.stride(0);
                let target = frame.data_mut(0);
                for line in 0..height as usize {
                    let from = offset + line * stride;
                    target[line * target_stride..line * target_stride + row]
                        .copy_from_slice(&bytes[from..from + row]);
                }
                taken.push(at, frame);
            })
            .register()
            .map_err(failed)?;
        let formats = spa::pod::object!(
            spa::utils::SpaTypes::ObjectParamFormat,
            spa::param::ParamType::EnumFormat,
            spa::pod::property!(
                spa::param::format::FormatProperties::MediaType,
                Id,
                spa::param::format::MediaType::Video
            ),
            spa::pod::property!(
                spa::param::format::FormatProperties::MediaSubtype,
                Id,
                spa::param::format::MediaSubtype::Raw
            ),
            spa::pod::property!(
                spa::param::format::FormatProperties::VideoFormat,
                Choice,
                Enum,
                Id,
                VideoFormat::BGRx,
                VideoFormat::BGRx,
                VideoFormat::RGBx,
                VideoFormat::BGRA,
                VideoFormat::RGBA,
                VideoFormat::xRGB,
                VideoFormat::xBGR,
            ),
            spa::pod::property!(
                spa::param::format::FormatProperties::VideoSize,
                Choice,
                Range,
                Rectangle,
                spa::utils::Rectangle {
                    width: 1920,
                    height: 1080
                },
                spa::utils::Rectangle {
                    width: 1,
                    height: 1
                },
                spa::utils::Rectangle {
                    width: 16384,
                    height: 16384
                }
            ),
            spa::pod::property!(
                spa::param::format::FormatProperties::VideoFramerate,
                Choice,
                Range,
                Fraction,
                spa::utils::Fraction { num: 60, denom: 1 },
                spa::utils::Fraction { num: 0, denom: 1 },
                spa::utils::Fraction { num: 240, denom: 1 }
            ),
        );
        let values: Vec<u8> = spa::pod::serialize::PodSerializer::serialize(
            std::io::Cursor::new(Vec::new()),
            &spa::pod::Value::Object(formats),
        )
        .map_err(|error| format!("{error:?}"))?
        .0
        .into_inner();
        let mut params =
            [spa::pod::Pod::from_bytes(&values)
                .ok_or_else(|| "the screen's formats".to_owned())?];
        stream
            .connect(
                spa::utils::Direction::Input,
                Some(node),
                pw::stream::StreamFlags::AUTOCONNECT | pw::stream::StreamFlags::MAP_BUFFERS,
                &mut params,
            )
            .map_err(failed)?;
        let looping = mainloop.clone();
        let _quit = quitting.attach(mainloop.loop_(), move |()| looping.quit());
        let _ = ready.send(Ok(()));
        mainloop.run();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A picture of one gray `level`, as a screen gives it.
    fn picture(width: u32, height: u32, level: u8) -> ffmpeg::frame::Video {
        let mut frame = ffmpeg::frame::Video::new(Pixel::BGRZ, width, height);
        frame.data_mut(0).fill(level);
        frame
    }

    #[test]
    fn pictures_are_kept_now_and_then_until_recording() {
        crate::init();
        let frames = Frames::new();
        let start = Instant::now();
        assert!(frames.wanted(start));
        frames.push(start, picture(4, 4, 1));
        assert!(!frames.wanted(start + Duration::from_millis(100)));
        assert!(frames.wanted(start + KEEP_EVERY));
        // Nothing is queued before recording; the last picture kept
        // comes first once it starts.
        assert!(frames.take(Duration::ZERO).is_empty());
        frames.start();
        assert!(frames.wanted(start + Duration::from_millis(1)));
        let taken = frames.take(Duration::ZERO);
        assert_eq!(taken.len(), 1);
        assert_eq!(taken[0].0, start);
        // Too many waiting: the newest are dropped.
        for _ in 0..MOST_WAITING + 3 {
            frames.push(Instant::now(), picture(4, 4, 2));
        }
        assert_eq!(frames.take(Duration::ZERO).len(), MOST_WAITING);
        frames.end();
        assert!(frames.take(Duration::from_millis(50)).is_empty());
    }

    #[test]
    fn pictures_are_recorded_as_they_came() {
        crate::init();
        // Odd sizes are recorded a pixel smaller, as encoders need.
        let size = (321, 241);
        if export::encoder(Format::H264, (320, 240), false, true).is_none() {
            eprintln!("no H.264 encoder here");
            return;
        }
        let frames = Frames::new();
        let shared = CaptureShared::default();
        *shared.size.lock().unwrap() = Some(size);
        let progress = Progress::default();
        let finish = AtomicBool::new(false);
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("screen.mp4");
        // A still screen sends nothing: the picture kept starts it.
        frames.push(Instant::now(), picture(size.0, size.1, 40));
        std::thread::scope(|scope| {
            let writer = scope.spawn(|| {
                write(
                    (&frames, &shared, None),
                    (Format::H264, true),
                    &path,
                    (&finish, &progress),
                )
            });
            while frames.waiting().queue.is_none() {
                std::thread::sleep(Duration::from_millis(5));
            }
            for level in 0..10u8 {
                std::thread::sleep(Duration::from_millis(50));
                // A window shared may change size on the way.
                let shape = if level == 5 { (200, 100) } else { size };
                frames.push(Instant::now(), picture(shape.0, shape.1, level * 20));
            }
            std::thread::sleep(Duration::from_millis(200));
            finish.store(true, Ordering::Relaxed);
            writer.join().unwrap().unwrap();
        });
        let input = ffmpeg::format::input(&path).unwrap();
        let stream = input.streams().best(ffmpeg::media::Type::Video).unwrap();
        let parameters = stream.parameters();
        let decoder = ffmpeg::codec::Context::from_parameters(parameters)
            .unwrap()
            .decoder()
            .video()
            .unwrap();
        assert_eq!((decoder.width(), decoder.height()), (320, 240));
        let mut input = input;
        let mut times: Vec<i64> = input
            .packets()
            .filter(|(stream, _)| stream.index() == 0)
            .filter_map(|(_, packet)| packet.pts())
            .collect();
        times.sort_unstable();
        // The one kept, ten more, and the last again where it ends.
        assert_eq!(times.len(), 12, "{times:?}");
        let base = f64::from(stream_base(&path));
        let seconds: Vec<f64> = times.iter().map(|&time| time as f64 / base).collect();
        assert!(seconds[0].abs() < 0.001, "{seconds:?}");
        // About 50 ms apart, then the 200 ms before it ended.
        assert!((0.6..1.2).contains(&seconds[11]), "{seconds:?}");
        assert!(progress.written.load(Ordering::Relaxed) > 0);
    }

    /// The video stream's time base's units a second.
    fn stream_base(path: &std::path::Path) -> i32 {
        let input = ffmpeg::format::input(&path).unwrap();
        let stream = input.streams().best(ffmpeg::media::Type::Video).unwrap();
        stream.time_base().denominator() / stream.time_base().numerator()
    }
}
