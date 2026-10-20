//! Recording the screen or a camera: pictures a capture hands over, as
//! the ScreenCast portal shares a screen's through PipeWire on Linux,
//! Windows.Graphics.Capture takes them on Windows and ScreenCaptureKit on
//! macOS, and FFmpeg's device inputs read a camera's, encoded as exports
//! encode pictures, with the sound of a recording input when one is given,
//! into an MP4.

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
use crate::record::{self, RecordEvent, SoundTap};

/// Pictures waiting for the encoder; more than this, and the newest are
/// dropped until it catches up.
#[cfg(any(target_os = "linux", windows, target_os = "macos", test))]
const MOST_WAITING: usize = 8;
/// How long the capture may take to say its size.
const FIRST_PICTURE: Duration = Duration::from_secs(5);
/// How often a picture is kept while not recording: a still screen sends
/// none, so the last one kept starts a recording.
#[cfg(any(target_os = "linux", windows, target_os = "macos", test))]
const KEEP_EVERY: Duration = Duration::from_millis(250);
/// The frame rate encoders are told, for their rate control, when every
/// picture the screen sends is kept; pictures come as the screen changes.
const USUAL_RATE: u32 = 60;
/// Timestamps count microseconds.
const TIME_BASE: ffmpeg::Rational = ffmpeg::Rational(1, 1_000_000);

type Taken = (Instant, ffmpeg::frame::Video);

/// Pictures a capture hands over, with when each came: each while
/// recording, and now and then otherwise.
pub(crate) struct Frames {
    waiting: Mutex<Waiting>,
    arrived: Condvar,
    /// Pictures dropped while recording, as the encoder fell behind.
    dropped: AtomicU64,
}

#[derive(Default)]
struct Waiting {
    /// The pictures for the encoder, while recording.
    queue: Option<VecDeque<Taken>>,
    /// The last picture kept.
    latest: Option<Taken>,
    /// The rate pictures are kept to while recording, if any.
    pace: Option<Pace>,
}

/// Pictures kept to a rate, the rest skipped before they are copied:
/// credit for one picture builds up each `every`, and a picture is kept
/// once there is most of one. A screen as fast as the rate or slower
/// keeps every picture, though they come a little early or late, and a
/// faster one keeps them as evenly as its own come. Unused where no
/// capture hands pictures over yet.
#[cfg_attr(
    not(any(target_os = "linux", windows, target_os = "macos")),
    allow(dead_code)
)]
struct Pace {
    every: Duration,
    /// Credit left when the last picture was kept, in pictures.
    credit: f64,
    last: Option<Instant>,
}

/// The credit a picture takes to be kept: most of one picture, so one
/// that comes a little early still is.
#[cfg_attr(
    not(any(target_os = "linux", windows, target_os = "macos")),
    allow(dead_code)
)]
const CREDIT_TO_KEEP: f64 = 0.75;
/// The most credit there is: a little more than one picture's, so one
/// that came late leaves the next its due, but a still screen saves next
/// to nothing up for later.
#[cfg_attr(
    not(any(target_os = "linux", windows, target_os = "macos")),
    allow(dead_code)
)]
const MOST_CREDIT: f64 = 1.25;

#[cfg_attr(
    not(any(target_os = "linux", windows, target_os = "macos")),
    allow(dead_code)
)]
impl Pace {
    /// The credit there is at `at`.
    fn credit(&self, at: Instant) -> f64 {
        match self.last {
            None => 1.0,
            Some(last) => {
                let built = at.saturating_duration_since(last).as_secs_f64()
                    / self.every.as_secs_f64().max(f64::EPSILON);
                (self.credit + built).min(MOST_CREDIT)
            }
        }
    }

    fn takes(&self, at: Instant) -> bool {
        self.credit(at) >= CREDIT_TO_KEEP
    }

    fn keep(&mut self, at: Instant) {
        self.credit = self.credit(at) - 1.0;
        self.last = Some(at);
    }
}

/// How a screen is recorded: as `format`, on a graphics card when
/// `hardware` allows, at most `rate` pictures a second, or as many as the
/// screen sends when none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoEncoding {
    pub format: Format,
    pub hardware: bool,
    pub rate: Option<u32>,
}

impl Frames {
    #[cfg(any(target_os = "linux", windows, target_os = "macos", test))]
    fn new() -> Self {
        Self {
            waiting: Mutex::new(Waiting::default()),
            arrived: Condvar::new(),
            dropped: AtomicU64::new(0),
        }
    }

    fn waiting(&self) -> MutexGuard<'_, Waiting> {
        self.waiting.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Whether a picture that came at `at` is wanted.
    #[cfg(any(target_os = "linux", windows, target_os = "macos", test))]
    pub fn wanted(&self, at: Instant) -> bool {
        let waiting = self.waiting();
        if waiting.queue.is_some() {
            return waiting.pace.as_ref().is_none_or(|pace| pace.takes(at));
        }
        waiting
            .latest
            .as_ref()
            .is_none_or(|(kept, _)| at.saturating_duration_since(*kept) >= KEEP_EVERY)
    }

    /// Hands over a picture taken at `at`, unless too many wait.
    #[cfg(any(target_os = "linux", windows, target_os = "macos", test))]
    pub fn push(&self, at: Instant, frame: ffmpeg::frame::Video) {
        let mut waiting = self.waiting();
        if waiting.queue.is_some()
            && let Some(pace) = waiting.pace.as_mut()
        {
            if !pace.takes(at) {
                return;
            }
            pace.keep(at);
        }
        match waiting.queue.as_mut() {
            Some(queue) => {
                if queue.len() < MOST_WAITING {
                    queue.push_back((at, frame));
                    self.arrived.notify_one();
                } else {
                    self.dropped.fetch_add(1, Ordering::Relaxed);
                }
            }
            None => waiting.latest = Some((at, frame)),
        }
    }

    /// Starts taking every picture, the last one kept first.
    /// Keeping them to at most one each `every`, when given.
    fn start(&self, every: Option<Duration>) {
        self.dropped.store(0, Ordering::Relaxed);
        let mut waiting = self.waiting();
        waiting.pace = every.map(|every| Pace {
            every,
            credit: 0.0,
            last: None,
        });
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
pub struct Capture {
    frames: Arc<Frames>,
    shared: Arc<CaptureShared>,
    progress: Arc<Progress>,
    recording: Option<Recording>,
    #[cfg(target_os = "linux")]
    capture: Option<pipewire_capture::Capture>,
    #[cfg(windows)]
    capture: Option<graphics_capture::Capture>,
    #[cfg(target_os = "macos")]
    capture: Option<screen_capture_kit::Capture>,
    camera: Option<camera_capture::Capture>,
}

/// A camera there is to record: the FFmpeg device input that reads it
/// (`v4l2`, `dshow` or `avfoundation`, or `lavfi` for a test picture),
/// the name that input opens it by, and the one people know it by.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Camera {
    pub input: String,
    pub id: String,
    pub name: String,
}

/// The cameras the system lists, and with `PLAA_TEST_CAMERA` set, FFmpeg's
/// test picture as one, first, to try recording a camera without one.
pub fn cameras() -> Vec<Camera> {
    crate::init();
    let mut cameras = listed_cameras();
    if std::env::var_os("PLAA_TEST_CAMERA").is_some() {
        cameras.insert(
            0,
            Camera {
                input: "lavfi".to_owned(),
                id: "testsrc2=s=1280x720:r=30,realtime".to_owned(),
                name: "Test picture".to_owned(),
            },
        );
    }
    cameras
}

fn listed_cameras() -> Vec<Camera> {
    #[cfg(target_os = "macos")]
    return camera_macos::cameras();
    #[cfg(not(target_os = "macos"))]
    {
        let input = if cfg!(windows) { "dshow" } else { "v4l2" };
        let Some(format) = camera_capture::device_input(input) else {
            return Vec::new();
        };
        let mut sources = crate::ffi::video_sources(&format);
        // A camera may have several nodes, as one for its pictures and one
        // for what it says of them: the first is the one to open.
        sources.sort();
        let mut cameras: Vec<Camera> = Vec::new();
        for (id, description) in sources {
            let name = camera_name(&description).unwrap_or_else(|| id.clone());
            if cameras.iter().any(|camera| camera.name == name) {
                continue;
            }
            cameras.push(Camera {
                input: input.to_owned(),
                // DirectShow opens a camera as `video=` its name.
                id: if cfg!(windows) {
                    format!("video={id}")
                } else {
                    id
                },
                name,
            });
        }
        cameras
    }
}

/// The name of a camera as its driver describes it, without the second
/// copy cut short that Linux's adds, as in `Anker PowerConf C200: Anker
/// Pow`.
#[cfg_attr(target_os = "macos", allow(dead_code))]
fn camera_name(description: &str) -> Option<String> {
    let description = description.trim();
    if description.is_empty() {
        return None;
    }
    Some(match description.split_once(": ") {
        Some((name, again)) if name.starts_with(again.trim()) => name.to_owned(),
        _ => description.to_owned(),
    })
}

/// A camera's latest picture, made small for a window to show, as RGBA:
/// `count` tells one from the one before.
#[derive(Debug)]
pub struct Preview {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
    pub count: u64,
}

#[cfg(target_os = "macos")]
#[path = "camera_macos.rs"]
mod camera_macos;

/// A screen to record on Windows or macOS, by its number in the order the
/// system lists screens in from 0, or a window, by the system's id of it.
#[cfg(any(windows, target_os = "macos"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Screen(u32),
    Window(u64),
}

/// A screen or window there is to record, as macOS lists them.
#[cfg(target_os = "macos")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shareable {
    pub target: Target,
    /// The main one of several screens.
    pub main: bool,
    /// A window's name: its app's, and its title.
    pub name: String,
}

/// The screens, the main one first, then the windows of other apps, or
/// why the system would not say, as when plaa may not record the screen.
#[cfg(target_os = "macos")]
pub fn shareable() -> Result<Vec<Shareable>, String> {
    screen_capture_kit::shareable()
}

#[cfg(target_os = "macos")]
#[path = "screen_capture_kit.rs"]
mod screen_capture_kit;

/// What a capture says of itself.
#[derive(Default)]
struct CaptureShared {
    /// The size of its pictures, once known.
    size: Mutex<Option<(u32, u32)>>,
    /// Why it ended, if it did: the person stopped sharing, or it failed.
    ended: Mutex<Option<String>>,
    /// The pictures are a camera's, whose colors are video's.
    camera: bool,
    /// A camera's latest picture, made small.
    preview: Mutex<Option<Arc<Preview>>>,
}

impl Capture {
    /// Takes the pictures of `camera`.
    pub fn camera(camera: &Camera) -> Result<Self, String> {
        crate::init();
        let frames = Arc::new(Frames::new());
        let shared = Arc::new(CaptureShared {
            camera: true,
            ..CaptureShared::default()
        });
        let capture = camera_capture::Capture::open(camera, frames.clone(), shared.clone())?;
        let mut taking = Self::taking(frames, shared);
        taking.camera = Some(capture);
        Ok(taking)
    }

    /// A capture taking pictures into `frames`, of no source yet.
    fn taking(frames: Arc<Frames>, shared: Arc<CaptureShared>) -> Self {
        Self {
            frames,
            shared,
            progress: Arc::new(Progress::default()),
            recording: None,
            #[cfg(any(target_os = "linux", windows, target_os = "macos"))]
            capture: None,
            camera: None,
        }
    }

    /// A camera's latest picture, made small, once one has come.
    pub fn preview(&self) -> Option<Arc<Preview>> {
        self.shared
            .preview
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    /// Takes the pictures of stream `node` of the PipeWire remote `fd`,
    /// as the ScreenCast portal gives them.
    #[cfg(target_os = "linux")]
    pub fn open(fd: std::os::fd::OwnedFd, node: u32) -> Result<Self, String> {
        crate::init();
        let frames = Arc::new(Frames::new());
        let shared = Arc::new(CaptureShared::default());
        let capture = pipewire_capture::Capture::open(fd, node, frames.clone(), shared.clone())?;
        let mut taking = Self::taking(frames, shared);
        taking.capture = Some(capture);
        Ok(taking)
    }

    /// Takes the pictures of `target` through Windows.Graphics.Capture,
    /// or ScreenCaptureKit on macOS, which leaves plaa's own windows out
    /// of a screen.
    #[cfg(any(windows, target_os = "macos"))]
    pub fn open(target: Target) -> Result<Self, String> {
        crate::init();
        let frames = Arc::new(Frames::new());
        let shared = Arc::new(CaptureShared::default());
        #[cfg(windows)]
        let capture = graphics_capture::Capture::open(target, frames.clone(), shared.clone())?;
        #[cfg(target_os = "macos")]
        let capture = screen_capture_kit::Capture::open(target, frames.clone(), shared.clone())?;
        let mut taking = Self::taking(frames, shared);
        taking.capture = Some(capture);
        Ok(taking)
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

    /// Starts recording to `destination` as `encoding` says, with the
    /// sound of `microphone` when given. It begins `after` from now, as a
    /// window gets out of the way first: of what shows until then, only
    /// the last picture is kept, which begins it. `events` hears when it
    /// ends, from its thread.
    pub fn record(
        &mut self,
        microphone: Option<&crate::record::Microphone>,
        encoding: VideoEncoding,
        destination: PathBuf,
        after: Duration,
        events: impl Fn(RecordEvent) + Send + 'static,
    ) -> Result<(), String> {
        if self.recording.is_some() {
            return Err("already recording".to_owned());
        }
        if matches!(encoding.format, Format::Sound(_)) {
            return Err("a screen is recorded as video".to_owned());
        }
        self.progress.recorded.store(0, Ordering::Relaxed);
        self.progress.written.store(0, Ordering::Relaxed);
        let sound = microphone.map(crate::record::Microphone::tap);
        let begin = Instant::now() + after;
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
                        encoding,
                        &temporary,
                        (begin, &finish, &progress),
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

    /// How many pictures the recording has dropped so far, as the encoder
    /// fell behind the screen.
    pub fn dropped(&self) -> u64 {
        self.frames.dropped.load(Ordering::Relaxed)
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

impl Drop for Capture {
    fn drop(&mut self) {
        self.stop();
        self.ended();
        #[cfg(any(target_os = "linux", windows, target_os = "macos"))]
        drop(self.capture.take());
        drop(self.camera.take());
    }
}

/// The picture side of a screen recording: its encoder and the graph
/// that makes pictures as it takes them.
struct VideoSide {
    encoder: export::VideoEncoder,
    size: (u32, u32),
    graph: Option<(ffmpeg::filter::Graph, (u32, u32, Pixel))>,
    /// How long a picture lasts at the rate the encoder is told, in
    /// microseconds, for the packets that do not say.
    duration: i64,
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
        let duration = self.duration;
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
        output.drain(&mut self.encoder.encoder, 0, TIME_BASE, self.duration)
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
    /// When the sound began to be taken.
    start: Instant,
}

impl SoundSide {
    fn encode(&mut self, samples: &[f32], output: &mut Output) -> Result<(), ExportError> {
        let frames = samples.len() / usize::from(self.format.1.max(1));
        let at = record::sound_time(self.taken, frames, self.format.0, self.start);
        let Some(frame) = record::sound_frame(samples, self.format, at) else {
            return Ok(());
        };
        self.taken = at + frames as i64;
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
        if let Some(silence) = record::closing_silence(self.taken, self.format, self.start)
            && let Some(mut source) = self.graph.get("in")
        {
            source.source().add(&silence).map_err(export::failed)?;
        }
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
/// on a graphics card, and from `begin` if that is later.
fn write(
    (frames, shared, sound): (&Frames, &CaptureShared, Option<&SoundTap>),
    encoding: VideoEncoding,
    temporary: &std::path::Path,
    (begin, finish, progress): (Instant, &AtomicBool, &Progress),
) -> Result<(), ExportError> {
    let VideoEncoding {
        format,
        hardware,
        rate,
    } = encoding;
    let told = rate.unwrap_or(USUAL_RATE).max(1);
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
        rate: ffmpeg::Rational::new(told as i32, 1),
        time_base: TIME_BASE,
        ten_bit: false,
        live: true,
    };
    let (candidate, encoder) = export::candidates(format)
        .into_iter()
        .filter(|candidate| hardware || !candidate.hardware)
        .find_map(|candidate| {
            let colors = if shared.camera {
                Colors::Camera
            } else {
                Colors::Screen
            };
            export::open_video(candidate, format, &picture, colors, global_header)
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
                sound.leveled(),
            )?;
            Some(SoundSide {
                encoder,
                graph,
                format: (rate, channels),
                taken: 0,
                start: Instant::now(),
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
        duration: i64::from(TIME_BASE.denominator()) / i64::from(told),
    };
    frames.start(rate.map(|rate| Duration::from_secs(1) / rate.max(1)));
    // Until it begins, each picture replaces the one before, the last
    // beginning the recording.
    let mut pending: Vec<Taken> = Vec::new();
    loop {
        let now = Instant::now();
        if now >= begin || finish.load(Ordering::Relaxed) {
            break;
        }
        let wait = (begin - now).min(Duration::from_millis(20));
        if let Some(newest) = frames.take(wait).pop() {
            pending = vec![newest];
        }
    }
    if let Some(newest) = frames.take(Duration::ZERO).pop() {
        pending = vec![newest];
    }
    let start = Instant::now();
    if let Some(sound) = sound {
        sound.start();
    }
    if let Some(side) = &mut sound_side {
        side.start = start;
    }
    let micros = |at: Instant| at.saturating_duration_since(start).as_micros() as i64;
    let mut last: Option<(i64, ffmpeg::frame::Video)> = None;
    loop {
        let finishing = finish.load(Ordering::Relaxed);
        for (at, frame) in pending.drain(..) {
            let time = micros(at);
            // Each later than the last, as encoders need; those from
            // before it began are left out, but the last of them.
            if last.as_ref().is_some_and(|(previous, _)| time <= *previous) {
                if time > 0 {
                    frames.dropped.fetch_add(1, Ordering::Relaxed);
                }
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

/// A camera's pictures, as FFmpeg's device input reads and decodes them.
mod camera_capture {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, PoisonError, mpsc};
    use std::thread::JoinHandle;
    use std::time::{Duration, Instant};

    use ffmpeg::format::Pixel;
    use ffmpeg::software::scaling;
    use ffmpeg_next as ffmpeg;

    use super::{Camera, CaptureShared, Frames, Preview};

    /// How wide and tall a preview is at most.
    const PREVIEW: (u32, u32) = (960, 540);

    /// The device input format named `name`, as `v4l2`, if FFmpeg has it:
    /// some have several names, as `video4linux2,v4l2`.
    pub fn device_input(name: &str) -> Option<ffmpeg::format::Input> {
        ffmpeg::device::input::video().find_map(|format| match format {
            ffmpeg::Format::Input(input) if input.name().split(',').any(|named| named == name) => {
                Some(input)
            }
            _ => None,
        })
    }

    /// The options to open a camera with, the most wanted first: a
    /// camera's best is often MJPEG, as raw pictures of a large size come
    /// slowly over USB, and macOS's needs a rate it offers.
    fn attempts(input: &str) -> Vec<Vec<(&'static str, &'static str)>> {
        match input {
            "v4l2" => vec![
                vec![
                    ("input_format", "mjpeg"),
                    ("video_size", "1920x1080"),
                    ("framerate", "30"),
                ],
                vec![("video_size", "1280x720"), ("framerate", "30")],
                vec![],
            ],
            "avfoundation" => vec![
                vec![("video_size", "1920x1080"), ("framerate", "30")],
                vec![("video_size", "1280x720"), ("framerate", "30")],
                vec![("framerate", "30")],
            ],
            _ => vec![vec![]],
        }
    }

    /// The camera's thread, and the flag that ends it.
    pub struct Capture {
        quit: Arc<AtomicBool>,
        thread: Option<JoinHandle<()>>,
    }

    impl Capture {
        pub fn open(
            camera: &Camera,
            frames: Arc<Frames>,
            shared: Arc<CaptureShared>,
        ) -> Result<Self, String> {
            let quit = Arc::new(AtomicBool::new(false));
            let (ready, readied) = mpsc::channel();
            let thread = {
                let (camera, quit) = (camera.clone(), quit.clone());
                std::thread::Builder::new()
                    .name("plaa-camera".into())
                    .spawn(move || match open(&camera) {
                        Ok(opened) => {
                            let _ = ready.send(Ok(()));
                            run(opened, &frames, &shared, &quit);
                        }
                        Err(error) => {
                            let _ = ready.send(Err(error));
                        }
                    })
                    .map_err(|error| error.to_string())?
            };
            readied
                .recv_timeout(Duration::from_secs(10))
                .map_err(|_| "the camera did not open".to_owned())??;
            Ok(Self {
                quit,
                thread: Some(thread),
            })
        }
    }

    impl Drop for Capture {
        fn drop(&mut self) {
            self.quit.store(true, Ordering::Relaxed);
            // A camera that stopped sending would keep its thread
            // waiting: it is left to end on its own after a second.
            let waited = Instant::now();
            while self
                .thread
                .as_ref()
                .is_some_and(|thread| !thread.is_finished())
                && waited.elapsed() < Duration::from_secs(1)
            {
                std::thread::sleep(Duration::from_millis(10));
            }
            if let Some(thread) = self.thread.take()
                && thread.is_finished()
            {
                let _ = thread.join();
            }
        }
    }

    /// A camera opened: its input, the stream of pictures, and their
    /// decoder.
    type Opened = (
        ffmpeg::format::context::Input,
        usize,
        ffmpeg::decoder::Video,
    );

    fn open(camera: &Camera) -> Result<Opened, String> {
        let format = device_input(&camera.input)
            .map(ffmpeg::Format::Input)
            .ok_or_else(|| format!("this FFmpeg has no {} input", camera.input))?;
        let mut last = String::new();
        for options in attempts(&camera.input) {
            let mut dictionary = ffmpeg::Dictionary::new();
            for (key, value) in options {
                dictionary.set(key, value);
            }
            let input = match ffmpeg::format::open_with(&camera.id, &format, dictionary) {
                Ok(ffmpeg::format::context::Context::Input(input)) => input,
                Ok(_) => continue,
                Err(error) => {
                    last = error.to_string();
                    continue;
                }
            };
            let Some(stream) = input.streams().best(ffmpeg::media::Type::Video) else {
                last = "the camera sends no pictures".to_owned();
                continue;
            };
            let index = stream.index();
            let decoder = ffmpeg::codec::Context::from_parameters(stream.parameters())
                .and_then(|context| context.decoder().video())
                .map_err(|error| error.to_string())?;
            return Ok((input, index, decoder));
        }
        Err(last)
    }

    fn run(
        (mut input, index, mut decoder): Opened,
        frames: &Frames,
        shared: &CaptureShared,
        quit: &AtomicBool,
    ) {
        let mut decoded = ffmpeg::frame::Video::empty();
        let mut previewer: Option<((u32, u32, Pixel), scaling::Context)> = None;
        let mut count = 0;
        for (stream, packet) in input.packets() {
            if quit.load(Ordering::Relaxed) {
                return;
            }
            if stream.index() != index || decoder.send_packet(&packet).is_err() {
                continue;
            }
            while decoder.receive_frame(&mut decoded).is_ok() {
                let at = Instant::now();
                let shape = (decoded.width(), decoded.height(), decoded.format());
                *shared.size.lock().unwrap_or_else(PoisonError::into_inner) =
                    Some((shape.0, shape.1));
                count += 1;
                if let Some(preview) = preview(&decoded, shape, &mut previewer, count) {
                    *shared
                        .preview
                        .lock()
                        .unwrap_or_else(PoisonError::into_inner) = Some(Arc::new(preview));
                }
                if frames.wanted(at) {
                    frames.push(at, decoded.clone());
                }
            }
        }
        if !quit.load(Ordering::Relaxed) {
            *shared.ended.lock().unwrap_or_else(PoisonError::into_inner) =
                Some("the camera stopped".to_owned());
        }
    }

    /// `frame` made small, as RGBA, through a scaler kept for its shape.
    fn preview(
        frame: &ffmpeg::frame::Video,
        (width, height, format): (u32, u32, Pixel),
        previewer: &mut Option<((u32, u32, Pixel), scaling::Context)>,
        count: u64,
    ) -> Option<Preview> {
        if width == 0 || height == 0 {
            return None;
        }
        let fit = f64::min(
            f64::from(PREVIEW.0) / f64::from(width),
            f64::from(PREVIEW.1) / f64::from(height),
        )
        .min(1.0);
        let shown = (
            ((f64::from(width) * fit) as u32).max(2) / 2 * 2,
            ((f64::from(height) * fit) as u32).max(2) / 2 * 2,
        );
        if previewer
            .as_ref()
            .is_none_or(|(built, _)| *built != (width, height, format))
        {
            let scaler = scaling::Context::get(
                format,
                width,
                height,
                Pixel::RGBA,
                shown.0,
                shown.1,
                scaling::Flags::BILINEAR,
            )
            .ok()?;
            *previewer = Some(((width, height, format), scaler));
        }
        let (_, scaler) = previewer.as_mut()?;
        let mut rgba = ffmpeg::frame::Video::empty();
        scaler.run(frame, &mut rgba).ok()?;
        let row = shown.0 as usize * 4;
        let stride = rgba.stride(0);
        let data = rgba.data(0);
        let mut packed = Vec::with_capacity(row * shown.1 as usize);
        for line in 0..shown.1 as usize {
            packed.extend_from_slice(&data[line * stride..line * stride + row]);
        }
        Some(Preview {
            width: shown.0,
            height: shown.1,
            rgba: packed,
            count,
        })
    }
}

/// Windows.Graphics.Capture's pictures, through FFmpeg's gfxcapture.
#[cfg(windows)]
mod graphics_capture {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, PoisonError, mpsc};
    use std::thread::JoinHandle;
    use std::time::{Duration, Instant};

    use ffmpeg_next as ffmpeg;

    use super::{CaptureShared, Frames, Target};

    /// The capture's thread, and the flag that ends it.
    pub struct Capture {
        quit: Arc<AtomicBool>,
        thread: Option<JoinHandle<()>>,
    }

    impl Capture {
        pub fn open(
            target: Target,
            frames: Arc<Frames>,
            shared: Arc<CaptureShared>,
        ) -> Result<Self, String> {
            let quit = Arc::new(AtomicBool::new(false));
            let (ready, readied) = mpsc::channel();
            let thread = {
                let quit = quit.clone();
                std::thread::Builder::new()
                    .name("plaa-screen".into())
                    .spawn(move || match graph(target) {
                        Ok(graph) => {
                            let _ = ready.send(Ok(()));
                            run(graph, &frames, &shared, &quit);
                        }
                        Err(error) => {
                            let _ = ready.send(Err(error));
                        }
                    })
                    .map_err(|error| error.to_string())?
            };
            readied
                .recv_timeout(Duration::from_secs(5))
                .map_err(|_| "the screen did not open".to_owned())??;
            Ok(Self {
                quit,
                thread: Some(thread),
            })
        }
    }

    impl Drop for Capture {
        fn drop(&mut self) {
            // The filter looks for a picture for a second at most.
            self.quit.store(true, Ordering::Relaxed);
            if let Some(thread) = self.thread.take() {
                let _ = thread.join();
            }
        }
    }

    /// The graph that brings `target`'s pictures from the graphics card
    /// into memory, as BGRA.
    fn graph(target: Target) -> Result<ffmpeg::filter::Graph, String> {
        let failed = |error: ffmpeg::Error| error.to_string();
        let capture = ffmpeg::filter::find("gfxcapture")
            .ok_or_else(|| "this FFmpeg cannot capture the screen".to_owned())?;
        let sink = ffmpeg::filter::find("buffersink").ok_or_else(|| "no buffersink".to_owned())?;
        // A window may change size: it is fitted into the size it began
        // with, as it is, rather than cut.
        let arguments = match target {
            Target::Screen(number) => {
                format!("monitor_idx={number}:capture_cursor=1:max_framerate=240")
            }
            Target::Window(window) => {
                format!("hwnd={window}:capture_cursor=1:max_framerate=240:resize_mode=scale_aspect")
            }
        };
        let mut graph = ffmpeg::filter::Graph::new();
        graph.add(&capture, "in", &arguments).map_err(failed)?;
        graph.add(&sink, "out", "").map_err(failed)?;
        graph
            .output("in", 0)
            .and_then(|parser| parser.input("out", 0))
            .and_then(|parser| parser.parse("hwdownload,format=bgra"))
            .map_err(failed)?;
        graph.validate().map_err(failed)?;
        Ok(graph)
    }

    fn run(
        mut graph: ffmpeg::filter::Graph,
        frames: &Frames,
        shared: &CaptureShared,
        quit: &AtomicBool,
    ) {
        let end = |reason: String| {
            *shared.ended.lock().unwrap_or_else(PoisonError::into_inner) = Some(reason);
        };
        while !quit.load(Ordering::Relaxed) {
            let Some(mut sink) = graph.get("out") else {
                return end("the screen's graph broke".to_owned());
            };
            let mut frame = ffmpeg::frame::Video::empty();
            match sink.sink().frame(&mut frame) {
                Ok(()) => {
                    let at = Instant::now();
                    *shared.size.lock().unwrap_or_else(PoisonError::into_inner) =
                        Some((frame.width(), frame.height()));
                    if frames.wanted(at) {
                        frames.push(at, frame);
                    }
                }
                // Nothing changed on screen within a second.
                Err(ffmpeg::Error::Other {
                    errno: ffmpeg::error::EAGAIN,
                }) => {}
                // A window recorded closed.
                Err(ffmpeg::Error::Eof) => return end(String::new()),
                Err(error) => return end(error.to_string()),
            }
        }
    }
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
        frames.start(None);
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
    fn pictures_are_kept_to_the_rate_chosen() {
        crate::init();
        // The pictures kept of `count` coming each `every`, give or take
        // `jitter` by turns, and the shortest and longest gap between them.
        let kept = |rate: Option<u32>, every: Duration, count: u32, jitter: Duration| {
            let frames = Frames::new();
            frames.start(rate.map(|rate| Duration::from_secs(1) / rate));
            let origin = Instant::now() + Duration::from_millis(10);
            let mut times = Vec::new();
            for index in 0..count {
                let ideal = origin + every * index;
                let at = if index % 2 == 0 {
                    ideal + jitter
                } else {
                    ideal - jitter
                };
                if frames.wanted(at) {
                    frames.push(at, picture(4, 4, 1));
                }
                times.extend(frames.take(Duration::ZERO).into_iter().map(|(at, _)| at));
            }
            let gaps: Vec<Duration> = times.windows(2).map(|pair| pair[1] - pair[0]).collect();
            (
                times.len(),
                gaps.iter().min().copied().unwrap_or_default(),
                gaps.iter().max().copied().unwrap_or_default(),
            )
        };
        let still = Duration::ZERO;
        let ms = Duration::from_millis;
        let hz = |rate: u32| Duration::from_secs(1) / rate;
        // A second of a 120 Hz screen kept to 60: every other picture,
        // evenly.
        let (count, shortest, longest) = kept(Some(60), hz(120), 120, still);
        assert!((59..=61).contains(&count), "{count}");
        assert!(
            shortest >= ms(16) && longest <= ms(17),
            "{shortest:?} {longest:?}"
        );
        let (count, ..) = kept(Some(30), hz(120), 120, still);
        assert!((29..=31).contains(&count), "{count}");
        // Every picture, or a rate above the screen's: all of them.
        assert_eq!(kept(None, hz(120), 120, still).0, 120);
        assert_eq!(kept(Some(240), hz(120), 120, still).0, 120);
        // A 60 Hz screen kept to 60 loses none, though its pictures come
        // 2 ms early or late by turns, and at most one coming 3 ms so.
        assert_eq!(kept(Some(60), hz(60), 60, ms(2)).0, 60);
        assert!(kept(Some(60), hz(60), 60, ms(3)).0 >= 59);
        // A 144 Hz screen kept to 60: no more than 60.
        let (count, ..) = kept(Some(60), hz(144), 144, still);
        assert!((58..=61).contains(&count), "{count}");
        // A screen that sends less often than the rate loses nothing.
        assert_eq!(kept(Some(60), ms(100), 10, still).0, 10);
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
        // A still screen sends nothing: the picture kept would start it.
        frames.push(Instant::now(), picture(size.0, size.1, 40));
        // It begins a second from now, as a window gets out of the way.
        let begin = Instant::now() + Duration::from_secs(1);
        std::thread::scope(|scope| {
            let writer = scope.spawn(|| {
                write(
                    (&frames, &shared, None),
                    VideoEncoding {
                        format: Format::H264,
                        hardware: true,
                        rate: None,
                    },
                    &path,
                    (begin, &finish, &progress),
                )
            });
            while frames.waiting().queue.is_none() {
                std::thread::sleep(Duration::from_millis(5));
            }
            // Of what shows before it begins, the last picture begins it.
            for level in 1..=3u8 {
                let before = Duration::from_millis(100 * u64::from(4 - level));
                frames.push(begin - before, picture(size.0, size.1, level * 10));
            }
            while Instant::now() < begin + Duration::from_millis(50) {
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
        // Gray 30 is 42 in limited range.
        let first = first_luma(&path);
        assert!((40..=44).contains(&first), "{first}");
        let mut input = input;
        let mut times: Vec<i64> = input
            .packets()
            .filter(|(stream, _)| stream.index() == 0)
            .filter_map(|(_, packet)| packet.pts())
            .collect();
        times.sort_unstable();
        // The last before it began, ten more, and the last again where
        // it ends.
        assert_eq!(times.len(), 12, "{times:?}");
        let base = f64::from(stream_base(&path));
        let seconds: Vec<f64> = times.iter().map(|&time| time as f64 / base).collect();
        assert!(seconds[0].abs() < 0.001, "{seconds:?}");
        // About 50 ms apart, then the 200 ms before it ended, with room for
        // a slow machine's sleeps.
        assert!((0.6..2.0).contains(&seconds[11]), "{seconds:?}");
        assert!(progress.written.load(Ordering::Relaxed) > 0);
    }

    /// The middle of the first picture's luma.
    fn first_luma(path: &std::path::Path) -> u8 {
        let mut input = ffmpeg::format::input(&path).unwrap();
        let stream = input.streams().best(ffmpeg::media::Type::Video).unwrap();
        let index = stream.index();
        let mut decoder = ffmpeg::codec::Context::from_parameters(stream.parameters())
            .unwrap()
            .decoder()
            .video()
            .unwrap();
        let mut frame = ffmpeg::frame::Video::empty();
        for (stream, packet) in input.packets() {
            if stream.index() != index {
                continue;
            }
            decoder.send_packet(&packet).unwrap();
            if decoder.receive_frame(&mut frame).is_ok() {
                break;
            }
        }
        if frame.width() == 0 {
            decoder.send_eof().unwrap();
            decoder.receive_frame(&mut frame).unwrap();
        }
        let (width, height) = (frame.width() as usize, frame.height() as usize);
        frame.data(0)[height / 2 * frame.stride(0) + width / 2]
    }

    #[test]
    fn cameras_are_named_once() {
        assert_eq!(
            camera_name("Anker PowerConf C200: Anker Pow").as_deref(),
            Some("Anker PowerConf C200")
        );
        assert_eq!(
            camera_name("Integrated Camera: Integrated C").as_deref(),
            Some("Integrated Camera")
        );
        assert_eq!(
            camera_name("Logitech: C920").as_deref(),
            Some("Logitech: C920")
        );
        assert_eq!(camera_name(" "), None);
    }

    #[test]
    fn a_camera_is_shown_and_recorded() {
        crate::init();
        if export::encoder(Format::H264, (320, 240), false, true).is_none() {
            eprintln!("no H.264 encoder here");
            return;
        }
        // FFmpeg's test picture, read in real time as a camera's come.
        let camera = Camera {
            input: "lavfi".to_owned(),
            id: "testsrc2=s=320x240:r=30,realtime".to_owned(),
            name: "test".to_owned(),
        };
        let mut capture = Capture::camera(&camera).unwrap();
        let waited = Instant::now();
        while capture.preview().is_none() && waited.elapsed() < Duration::from_secs(5) {
            std::thread::sleep(Duration::from_millis(20));
        }
        let preview = capture.preview().expect("a preview");
        assert_eq!((preview.width, preview.height), (320, 240));
        assert_eq!(preview.rgba.len(), 320 * 240 * 4);
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("camera.mp4");
        let (sender, receiver) = std::sync::mpsc::channel();
        capture
            .record(
                None,
                VideoEncoding {
                    format: Format::H264,
                    hardware: true,
                    rate: None,
                },
                path.clone(),
                Duration::ZERO,
                move |event| {
                    let _ = sender.send(event);
                },
            )
            .unwrap();
        std::thread::sleep(Duration::from_secs(1));
        capture.stop();
        let RecordEvent::Finished(saved) = receiver.recv_timeout(Duration::from_secs(10)).unwrap();
        assert_eq!(saved.unwrap(), path);
        let input = ffmpeg::format::input(&path).unwrap();
        let stream = input.streams().best(ffmpeg::media::Type::Video).unwrap();
        let frames = stream.frames();
        // About a second of 30 a second, and video's colors.
        assert!((24..=36).contains(&frames), "{frames}");
        let decoder = ffmpeg::codec::Context::from_parameters(stream.parameters())
            .unwrap()
            .decoder()
            .video()
            .unwrap();
        assert_eq!(
            decoder.color_transfer_characteristic(),
            ffmpeg::util::color::TransferCharacteristic::BT709
        );
    }

    /// The video stream's time base's units a second.
    fn stream_base(path: &std::path::Path) -> i32 {
        let input = ffmpeg::format::input(&path).unwrap();
        let stream = input.streams().best(ffmpeg::media::Type::Video).unwrap();
        stream.time_base().denominator() / stream.time_base().numerator()
    }
}
