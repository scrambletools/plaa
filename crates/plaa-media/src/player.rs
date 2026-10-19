//! Playing a file or stream: a reading thread that hands packets to a
//! thread per decoder, sound through the audio output, and pictures for
//! the interface to draw when the clock reaches them.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use ffmpeg_next as ffmpeg;

use crate::audio;
use crate::clock::Clock;
use crate::decode::{self, Picture};
use crate::frame::{self, VideoFrame};
use crate::info::MediaInfo;
use crate::queue::{Item, PacketQueue};
use crate::subtitle::{self, Cue, Cues, PictureCues, SubtitlePicture};

/// What to open: a file, or an address FFmpeg reads, such as an HTTP or
/// RTSP stream.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Source {
    File(PathBuf),
    Url(String),
}

impl Source {
    /// A command line argument or dropped text: an address when it has a
    /// scheme FFmpeg reads, else a file.
    pub fn parse(text: &str) -> Self {
        let looks_like_url = text.split_once("://").is_some_and(|(scheme, _)| {
            !scheme.is_empty()
                && scheme
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric() || "+-.".contains(character))
        });
        if looks_like_url && !text.starts_with("file://") {
            Source::Url(text.to_owned())
        } else {
            let path = text.strip_prefix("file://").unwrap_or(text);
            Source::File(PathBuf::from(path))
        }
    }

    /// The name to show: the file's name, or the address.
    pub fn name(&self) -> String {
        match self {
            Source::File(path) => path
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.display().to_string()),
            Source::Url(url) => url.clone(),
        }
    }

    fn path(&self) -> &Path {
        match self {
            Source::File(path) => path,
            Source::Url(url) => Path::new(url),
        }
    }

    pub fn is_url(&self) -> bool {
        matches!(self, Source::Url(_))
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    /// Decode video on the graphics card where it can.
    pub hardware: bool,
    pub volume: f32,
    pub muted: bool,
    /// Where to start.
    pub start: Option<Duration>,
    /// Start paused rather than playing.
    pub paused: bool,
    /// Preferred languages, most wanted first.
    pub audio_languages: Vec<String>,
    pub subtitle_languages: Vec<String>,
    /// The output device to play through, by id; the system's default
    /// when none.
    pub audio_device: Option<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            hardware: true,
            volume: 1.0,
            muted: false,
            start: None,
            paused: false,
            audio_languages: Vec::new(),
            subtitle_languages: Vec::new(),
            audio_device: None,
        }
    }
}

/// What the player tells the interface, from its own threads.
#[derive(Debug, Clone)]
pub enum Event {
    Opened {
        info: Arc<MediaInfo>,
        /// The file's cover art, decoded.
        cover: Option<Arc<Picture>>,
    },
    Failed(String),
    /// Video decodes on this graphics card API, such as "vaapi", or on
    /// the processor.
    Decoding(Option<&'static str>),
    /// The audio output could not open, so the file plays silently.
    NoSound(String),
    /// Playback reached the end.
    Ended,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Status {
    pub position: Duration,
    pub duration: Option<Duration>,
    pub paused: bool,
    pub ended: bool,
    /// Waiting for data, as a stream does over a slow network.
    pub buffering: bool,
    pub speed: f64,
    pub audio: Option<usize>,
    pub subtitles: Option<usize>,
}

pub(crate) enum Command {
    Seek {
        position: f64,
        exact: bool,
    },
    /// The timeline's handle moved to `Shared::scrub_to`.
    Scrub,
    /// The drag ended.
    EndScrub,
    /// Plays through this output device, by id, or the system's default.
    SetAudioDevice(Option<String>),
    SelectAudio(Option<usize>),
    SelectSubtitles(Option<usize>),
}

/// What the player's threads and its handle share.
pub(crate) struct Shared {
    pub clock: Arc<Clock>,
    pub video: Mutex<frame::Queue>,
    pub video_room: Condvar,
    pub audio: Mutex<Option<Arc<audio::Shared>>>,
    pub cues: Mutex<Cues>,
    /// Picture subtitles decoded ahead of the clock.
    pub picture_cues: Mutex<PictureCues>,
    /// The ASS track libass draws, when it shows.
    pub ass: Mutex<Option<crate::ass::AssTrack>>,
    /// The latest seek's serial: older packets, pictures and sound are
    /// dropped.
    pub serial: AtomicU64,
    /// Where an exact seek lands, for its serial: decoders drop what comes
    /// before it.
    pub target: Mutex<Option<(u64, f64)>>,
    /// The latest picture decoded, shown or not: its serial and media time.
    pub decoded: Mutex<(u64, f64)>,
    /// Where the timeline's handle is while it is dragged.
    pub scrub_to: Mutex<Option<f64>>,
    /// Plays from the first time again on reaching the second.
    pub ab_loop: Mutex<Option<(f64, f64)>>,
    /// The cues of the subtitle file beside the video that shows, all read
    /// at once, in place of the video's own.
    pub file_cues: Mutex<Option<Arc<Vec<Cue>>>>,
    /// Set to stop every thread, and to cut FFmpeg's reads short.
    pub stop: Arc<AtomicBool>,
    /// The playback speed, as f64 bits.
    pub speed: AtomicU64,
    pub state: Mutex<State>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct State {
    pub info: Option<Arc<MediaInfo>>,
    pub has_video: bool,
    pub has_audio: bool,
    /// The reader reached the end of the file.
    pub read_all: bool,
    pub ended: bool,
    pub audio: Option<usize>,
    pub subtitles: Option<usize>,
    pub buffering: bool,
}

impl Shared {
    pub fn state(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn video(&self) -> MutexGuard<'_, frame::Queue> {
        self.video.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn audio_output(&self) -> Option<Arc<audio::Shared>> {
        self.audio
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    pub fn cues(&self) -> MutexGuard<'_, Cues> {
        self.cues.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn ass(&self) -> MutexGuard<'_, Option<crate::ass::AssTrack>> {
        self.ass.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn picture_cues(&self) -> MutexGuard<'_, PictureCues> {
        self.picture_cues
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    pub fn speed(&self) -> f64 {
        f64::from_bits(self.speed.load(Ordering::Relaxed))
    }

    pub fn stopping(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }

    /// Where exact seeking of `serial` lands, if it does.
    pub fn target(&self, serial: u64) -> Option<f64> {
        let target = *self.target.lock().unwrap_or_else(PoisonError::into_inner);
        target.and_then(|(of, position)| (of == serial).then_some(position))
    }
}

/// A playing file. Dropping it stops playback and its threads.
pub struct Player {
    shared: Arc<Shared>,
    commands: Sender<Command>,
    reader: Option<JoinHandle<()>>,
    source: Source,
}

impl Player {
    /// Starts opening `source`; `events` hears how it goes, from another
    /// thread.
    pub fn open(source: Source, options: Options, events: impl Fn(Event) + Send + 'static) -> Self {
        crate::init();
        let shared = Arc::new(Shared {
            clock: Arc::new(Clock::new()),
            video: Mutex::new(frame::Queue::default()),
            video_room: Condvar::new(),
            audio: Mutex::new(None),
            cues: Mutex::new(Cues::default()),
            picture_cues: Mutex::new(PictureCues::default()),
            ass: Mutex::new(None),
            serial: AtomicU64::new(0),
            target: Mutex::new(None),
            decoded: Mutex::new((0, 0.0)),
            scrub_to: Mutex::new(None),
            ab_loop: Mutex::new(None),
            file_cues: Mutex::new(None),
            stop: Arc::new(AtomicBool::new(false)),
            speed: AtomicU64::new(1.0f64.to_bits()),
            state: Mutex::new(State::default()),
        });
        let (commands, receiver) = mpsc::channel();
        let reader = {
            let shared = shared.clone();
            let source = source.clone();
            std::thread::Builder::new()
                .name("plaa-read".into())
                .spawn(move || read(source, options, shared, receiver, Box::new(events)))
                .ok()
        };
        Self {
            shared,
            commands,
            reader,
            source,
        }
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn info(&self) -> Option<Arc<MediaInfo>> {
        self.shared.state().info.clone()
    }

    pub fn play(&self) {
        let ended = self.shared.state().ended;
        if ended {
            self.seek(Duration::ZERO, false);
        }
        self.shared.clock.set_paused(false);
    }

    pub fn pause(&self) {
        self.shared.clock.set_paused(true);
    }

    pub fn toggle(&self) {
        if self.shared.clock.paused() {
            self.play();
        } else {
            self.pause();
        }
    }

    pub fn paused(&self) -> bool {
        self.shared.clock.paused()
    }

    /// Moves to `position`: to the keyframe before it when not `exact`,
    /// quicker while dragging along the timeline.
    pub fn seek(&self, position: Duration, exact: bool) {
        let duration = self
            .shared
            .state()
            .info
            .as_ref()
            .and_then(|info| info.duration);
        let position = match duration {
            Some(duration) => position.min(duration),
            None => position,
        };
        {
            let mut state = self.shared.state();
            state.ended = false;
        }
        let _ = self.commands.send(Command::Seek {
            position: position.as_secs_f64(),
            exact,
        });
    }

    /// Moves by `seconds`, back when negative.
    pub fn seek_by(&self, seconds: f64) {
        let now = self.shared.clock.now();
        self.seek(Duration::from_secs_f64((now + seconds).max(0.0)), true);
    }

    /// Follows the timeline's handle while it is dragged to `position`:
    /// pictures land exactly there and only move the way it goes, the
    /// clock held meanwhile. `end_scrub` lets go.
    pub fn scrub(&self, position: Duration) {
        let duration = self
            .shared
            .state()
            .info
            .as_ref()
            .and_then(|info| info.duration);
        let position = match duration {
            Some(duration) => position.min(duration),
            None => position,
        };
        self.shared.state().ended = false;
        self.shared.clock.hold(true);
        *self
            .shared
            .scrub_to
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = Some(position.as_secs_f64());
        let _ = self.commands.send(Command::Scrub);
    }

    /// Plays from `from` again each time playback reaches `to`, or stops
    /// looping.
    pub fn set_ab_loop(&self, between: Option<(Duration, Duration)>) {
        *self
            .shared
            .ab_loop
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = between
            .filter(|(from, to)| from < to)
            .map(|(from, to)| (from.as_secs_f64(), to.as_secs_f64()));
    }

    /// Ends a drag: lands where the handle was let go, then plays on if
    /// playing.
    pub fn end_scrub(&self) {
        let _ = self.commands.send(Command::EndScrub);
    }

    /// Plays through output device `device`, by its id, or the system's
    /// default when none.
    pub fn set_audio_device(&self, device: Option<String>) {
        let _ = self.commands.send(Command::SetAudioDevice(device));
    }

    pub fn set_volume(&self, volume: f32) {
        if let Some(output) = self.shared.audio_output() {
            output.set_volume(volume);
        }
    }

    pub fn set_muted(&self, muted: bool) {
        if let Some(output) = self.shared.audio_output() {
            output.set_muted(muted);
        }
    }

    /// Plays at `speed` times, keeping the sound's pitch.
    pub fn set_speed(&self, speed: f64) {
        let speed = speed.clamp(0.1, 16.0);
        self.shared.speed.store(speed.to_bits(), Ordering::Relaxed);
        self.shared.clock.set_rate(speed);
    }

    pub fn select_audio(&self, stream: Option<usize>) {
        let _ = self.commands.send(Command::SelectAudio(stream));
    }

    pub fn select_subtitles(&self, stream: Option<usize>) {
        let _ = self.commands.send(Command::SelectSubtitles(stream));
    }

    /// The media time now.
    pub fn position(&self) -> Duration {
        let seconds = self.shared.clock.now().max(0.0);
        let duration = self
            .shared
            .state()
            .info
            .as_ref()
            .and_then(|info| info.duration);
        let position = Duration::from_secs_f64(seconds);
        match duration {
            Some(duration) => position.min(duration),
            None => position,
        }
    }

    /// Media seconds a second the position moves at now: the speed while
    /// playing, none while paused or waiting for sound.
    pub fn pace(&self) -> f64 {
        self.shared.clock.pace()
    }

    pub fn status(&self) -> Status {
        check_ended(&self.shared);
        let state = self.shared.state().clone();
        Status {
            position: self.position(),
            duration: state.info.as_ref().and_then(|info| info.duration),
            paused: self.shared.clock.paused(),
            ended: state.ended,
            buffering: state.buffering,
            speed: self.shared.speed(),
            audio: state.audio,
            subtitles: state.subtitles,
        }
    }

    /// The picture to draw now. Called on every frame the interface draws.
    pub fn picture(&self) -> Option<Arc<VideoFrame>> {
        let now = self.shared.clock.now();
        let (shown, first_of_seek) = {
            let mut video = self.shared.video();
            let first = video.has_new();
            (video.take_due(now), first)
        };
        self.shared.video_room.notify_all();
        // Without sound, the clock starts from the first picture after a
        // seek, so it does not run ahead while that is decoded.
        if first_of_seek
            && !self.shared.state().has_audio
            && let Some(shown) = &shown
        {
            self.shared.clock.anchor(shown.position, Instant::now());
        }
        shown
    }

    /// When the interface should draw again: the next picture's time while
    /// playing, at once when a picture after a seek waits, or `None` when
    /// nothing will change by itself.
    pub fn next_redraw(&self) -> Option<Instant> {
        let soon = || Some(Instant::now() + Duration::from_millis(10));
        let now = self.shared.clock.now();
        let video = self.shared.video();
        if video.has_new() || video.has_due(now) {
            return Some(Instant::now());
        }
        let due = video.next_due();
        let waiting = video.waiting();
        drop(video);
        if self.shared.clock.still() {
            // Pictures on their way after a seek, or while the timeline is
            // dragged, show as they come.
            let seeking = waiting && self.shared.state().has_video;
            return if seeking || self.shared.clock.held() {
                soon()
            } else {
                None
            };
        }
        match due.and_then(|due| self.shared.clock.instant_of(due)) {
            Some(at) => Some(at),
            // Waiting for pictures: look again soon.
            None => soon(),
        }
    }

    /// The picture subtitle showing now.
    pub fn subtitle_picture(&self) -> Option<Arc<SubtitlePicture>> {
        let now = self.shared.clock.now();
        if let Some(track) = self.shared.ass().as_mut() {
            return track.render(now);
        }
        self.shared.picture_cues().at(now)
    }

    /// The subtitle text showing now.
    pub fn subtitle(&self) -> Option<Cue> {
        let now = self.shared.clock.now();
        let file = self
            .shared
            .file_cues
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone();
        if let Some(cues) = file {
            return subtitle::showing(&cues, now).cloned();
        }
        self.shared.cues().at(now).cloned()
    }

    /// The picture shown, as RGBA, for saving a frame.
    pub fn snapshot(&self) -> Option<(u32, u32, Vec<u8>)> {
        self.shared.video().shown()?.to_rgba()
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        self.shared.stop.store(true, Ordering::Relaxed);
        self.shared.video_room.notify_all();
        if let Some(output) = self.shared.audio_output() {
            output.wake();
        }
        // The reader joins the decoders; a stuck network read is cut short
        // by the stop flag through FFmpeg's interrupt callback.
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
    }
}

/// The streams being decoded, each with its packet queue and thread.
struct Decoders {
    video: Option<(usize, Arc<PacketQueue>, JoinHandle<()>)>,
    audio: Option<(usize, Arc<PacketQueue>, JoinHandle<()>)>,
    subtitles: Option<(usize, SubtitleSink)>,
}

/// Where a subtitle track's packets go.
enum SubtitleSink {
    Decoder(decode::SubtitleDecoder),
    /// To libass, through `Shared::ass`, by the stream's time base.
    Ass(ffmpeg::Rational),
}

impl Decoders {
    fn queues(&self) -> impl Iterator<Item = &Arc<PacketQueue>> {
        self.video
            .iter()
            .map(|(_, queue, _)| queue)
            .chain(self.audio.iter().map(|(_, queue, _)| queue))
    }

    fn has_enough(&self) -> bool {
        let total: usize = self.queues().map(|queue| queue.bytes()).sum();
        total > 24 * 1024 * 1024 || self.queues().all(|queue| queue.has_enough())
    }

    fn stop_audio(&mut self) {
        if let Some((_, queue, thread)) = self.audio.take() {
            queue.close();
            let _ = thread.join();
        }
    }

    fn stop(&mut self) {
        self.stop_audio();
        if let Some((_, queue, thread)) = self.video.take() {
            queue.close();
            let _ = thread.join();
        }
    }
}

/// The reading thread: opens the source, starts the decoders and feeds
/// them until stopped.
fn read(
    source: Source,
    options: Options,
    shared: Arc<Shared>,
    commands: Receiver<Command>,
    events: Box<dyn Fn(Event) + Send>,
) {
    let stop = shared.stop.clone();
    let mut dictionary = ffmpeg::Dictionary::new();
    if source.is_url() {
        dictionary.set("reconnect", "1");
        dictionary.set("reconnect_streamed", "1");
        dictionary.set("rw_timeout", "15000000");
        dictionary.set("user_agent", concat!("plaa/", env!("CARGO_PKG_VERSION")));
    }
    let mut input = match ffmpeg::format::input_with_interrupt_and_dictionary(
        source.path(),
        move || stop.load(Ordering::Relaxed),
        dictionary,
    ) {
        Ok(input) => input,
        Err(error) => {
            if !shared.stopping() {
                events(Event::Failed(describe(&error)));
            }
            return;
        }
    };
    let mut info = MediaInfo::read(&input);
    let subtitle_files = match &source {
        Source::File(path) => read_subtitle_files(path, &mut info),
        Source::Url(_) => Vec::new(),
    };
    let info = Arc::new(info);
    let start = decode::start_seconds(&input);

    // The output opens before the decoders, which resample to its format.
    let audio_stream = decode::pick_audio(&info, &options.audio_languages);
    let mut output = None;
    if audio_stream.is_some() {
        match audio::Output::open(
            shared.clock.clone(),
            options.volume,
            options.muted,
            options.audio_device.as_deref(),
        ) {
            Ok(opened) => output = Some(opened),
            Err(error) => events(Event::NoSound(error.to_string())),
        }
    }
    *shared.audio.lock().unwrap_or_else(PoisonError::into_inner) =
        output.as_ref().map(|output| output.shared.clone());

    let video_stream = info
        .video
        .iter()
        .find(|track| !track.cover)
        .map(|track| track.track.index);
    let mut decoders = Decoders {
        video: None,
        audio: None,
        subtitles: None,
    };
    if let Some(index) = video_stream {
        match decode::spawn_video(&input, index, start, options.hardware, shared.clone()) {
            Ok(((queue, thread), hardware)) => {
                decoders.video = Some((index, queue, thread));
                events(Event::Decoding(hardware));
            }
            Err(error) => eprintln!("plaa: video stream {index}: {error}"),
        }
    }
    let mut audio_index = None;
    if let (Some(index), Some(output)) = (audio_stream, &output) {
        match decode::spawn_audio(&input, index, start, output.shared.clone(), shared.clone()) {
            Ok((queue, thread)) => {
                decoders.audio = Some((index, queue, thread));
                audio_index = Some(index);
            }
            Err(error) => eprintln!("plaa: audio stream {index}: {error}"),
        }
    }
    let subtitle_stream = decode::pick_subtitles(&info, &options.subtitle_languages);
    // The frame picture subtitles are placed in, unless they say: the
    // video's.
    let canvas = info
        .video
        .iter()
        .find(|track| !track.cover)
        .map_or((1920, 1080), |track| (track.width, track.height));
    let showing = Subtitles {
        input: &input,
        info: &info,
        start,
        canvas,
        files: &subtitle_files,
    }
    .choose(subtitle_stream, &shared, &mut decoders);
    let cover = info
        .cover()
        .and_then(|track| decode::cover(&input, track.track.index))
        .map(Arc::new);
    {
        let mut state = shared.state();
        state.info = Some(info.clone());
        state.has_video = decoders.video.is_some();
        state.has_audio = decoders.audio.is_some();
        state.audio = audio_index;
        state.subtitles = showing;
    }
    if decoders.video.is_none() && decoders.audio.is_none() {
        events(Event::Failed(
            "nothing in this file can be played".to_owned(),
        ));
        return;
    }
    events(Event::Opened {
        info: info.clone(),
        cover,
    });

    // Starts like a seek, so the clock waits for the first sound or
    // picture.
    shared.clock.stall();
    shared.clock.set_paused(options.paused);
    if let Some(at) = options.start.filter(|_| info.seekable) {
        seek(
            &mut input,
            &shared,
            &mut decoders,
            at.as_secs_f64(),
            true,
            start,
        );
    }

    let mut read_all = false;
    let mut announced_end = false;
    let mut scrub: Option<Scrub> = None;
    // Where playback was last time round, to see it reach a loop's end.
    let mut played = shared.clock.now();
    let mut packet = ffmpeg::Packet::empty();
    while !shared.stopping() {
        let now = shared.clock.now();
        let between = *shared
            .ab_loop
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        // Playing on past the end, not a seek past it.
        if let Some((from, to)) = between
            && info.seekable
            && played < to
            && now >= to
            && now - played < 0.5
        {
            seek(&mut input, &shared, &mut decoders, from, true, start);
            read_all = false;
            announced_end = false;
            shared.state().read_all = false;
        }
        played = shared.clock.now();
        if info.seekable
            && scrub.is_some()
            && steer(
                &mut scrub,
                &mut input,
                &shared,
                &mut decoders,
                video_stream,
                start,
            )
        {
            read_all = false;
            announced_end = false;
            shared.state().read_all = false;
        }
        // Commands first, waiting for one only while there is nothing to
        // read for now, and not long while the timeline is dragged.
        let idle = read_all || decoders.has_enough();
        let wait = if scrub.is_some() { 4 } else { 20 };
        let command = if idle {
            match commands.recv_timeout(Duration::from_millis(wait)) {
                Ok(command) => Some(command),
                Err(RecvTimeoutError::Timeout) => None,
                Err(RecvTimeoutError::Disconnected) => break,
            }
        } else {
            commands.try_recv().ok()
        };
        if let Some(command) = command {
            match command {
                Command::Seek { position, exact } => {
                    if info.seekable {
                        seek(&mut input, &shared, &mut decoders, position, exact, start);
                        read_all = false;
                        announced_end = false;
                        shared.state().read_all = false;
                    }
                }
                Command::Scrub => {
                    if !info.seekable {
                        *shared
                            .scrub_to
                            .lock()
                            .unwrap_or_else(PoisonError::into_inner) = None;
                        shared.clock.hold(false);
                    } else if steer(
                        &mut scrub,
                        &mut input,
                        &shared,
                        &mut decoders,
                        video_stream,
                        start,
                    ) {
                        read_all = false;
                        announced_end = false;
                        shared.state().read_all = false;
                    }
                }
                Command::EndScrub => match &mut scrub {
                    Some(scrub) => scrub.ending = true,
                    None => shared.clock.hold(false),
                },
                Command::SelectAudio(index) => {
                    decoders.stop_audio();
                    start_audio(
                        index,
                        &input,
                        start,
                        output.as_ref(),
                        &shared,
                        &mut decoders,
                    );
                    // Back to where it was, so the new track starts in step.
                    let now = shared.clock.now().max(0.0);
                    if info.seekable {
                        seek(&mut input, &shared, &mut decoders, now, true, start);
                        read_all = false;
                        shared.state().read_all = false;
                    }
                }
                Command::SetAudioDevice(device) => {
                    decoders.stop_audio();
                    let (volume, muted) = output
                        .as_ref()
                        .map_or((options.volume, options.muted), |output| {
                            output.shared.volume()
                        });
                    let playing = shared.state().audio.or(audio_stream);
                    // The old stream closes before the new one opens.
                    output = None;
                    match audio::Output::open(
                        shared.clock.clone(),
                        volume,
                        muted,
                        device.as_deref(),
                    ) {
                        Ok(opened) => output = Some(opened),
                        Err(error) => events(Event::NoSound(error.to_string())),
                    }
                    *shared.audio.lock().unwrap_or_else(PoisonError::into_inner) =
                        output.as_ref().map(|output| output.shared.clone());
                    start_audio(
                        playing,
                        &input,
                        start,
                        output.as_ref(),
                        &shared,
                        &mut decoders,
                    );
                    let now = shared.clock.now().max(0.0);
                    if info.seekable {
                        seek(&mut input, &shared, &mut decoders, now, true, start);
                        read_all = false;
                        shared.state().read_all = false;
                    }
                }
                Command::SelectSubtitles(index) => {
                    let showing = Subtitles {
                        input: &input,
                        info: &info,
                        start,
                        canvas,
                        files: &subtitle_files,
                    }
                    .choose(index, &shared, &mut decoders);
                    shared.state().subtitles = showing;
                    // A track's lines already passed show again from where
                    // it is; a file's are all there already.
                    let now = shared.clock.now().max(0.0);
                    if info.seekable && decoders.subtitles.is_some() {
                        seek(&mut input, &shared, &mut decoders, now, true, start);
                        read_all = false;
                        shared.state().read_all = false;
                    }
                }
            }
            continue;
        }
        if idle {
            if read_all && !announced_end && check_ended(&shared) {
                announced_end = true;
                events(Event::Ended);
            }
            continue;
        }
        match packet.read(&mut input) {
            Ok(()) => {
                let serial = shared.serial.load(Ordering::Relaxed);
                let index = packet.stream();
                let queue = decoders
                    .video
                    .iter()
                    .chain(&decoders.audio)
                    .find(|(stream, _, _)| *stream == index)
                    .map(|(_, queue, _)| queue);
                if let Some(queue) = queue {
                    let taken = std::mem::replace(&mut packet, ffmpeg::Packet::empty());
                    queue.put(Item::Packet(serial, taken));
                } else if let Some((stream, sink)) = &mut decoders.subtitles
                    && *stream == index
                {
                    match sink {
                        SubtitleSink::Decoder(decoder) => {
                            for decoded in decoder.decode(&packet) {
                                match decoded {
                                    decode::Decoded::Text(cue) => shared.cues().add(cue),
                                    decode::Decoded::Picture(cue) => {
                                        shared.picture_cues().add(cue);
                                    }
                                    decode::Decoded::Clear(at) => {
                                        shared.picture_cues().clear_from(at);
                                    }
                                }
                            }
                        }
                        SubtitleSink::Ass(time_base) => {
                            let milliseconds = |value: i64| {
                                crate::info::seconds(value, *time_base)
                                    .map(|seconds| (seconds * 1000.0) as i64)
                            };
                            let begins = packet
                                .pts()
                                .and_then(milliseconds)
                                .map(|begins| begins - (start * 1000.0) as i64);
                            if let (Some(begins), Some(track), Some(data)) =
                                (begins, shared.ass().as_mut(), packet.data())
                            {
                                let lasts = milliseconds(packet.duration()).unwrap_or(0);
                                track.add(data, begins, lasts);
                            }
                        }
                    }
                }
            }
            Err(ffmpeg::Error::Eof) => {
                let serial = shared.serial.load(Ordering::Relaxed);
                for queue in decoders.queues() {
                    queue.put(Item::End(serial));
                }
                read_all = true;
                shared.state().read_all = true;
            }
            Err(ffmpeg::Error::Exit) => break,
            Err(ffmpeg::Error::InvalidData) => {}
            Err(error) => {
                if shared.stopping() {
                    break;
                }
                eprintln!("plaa: reading {}: {error}", source.name());
                let serial = shared.serial.load(Ordering::Relaxed);
                for queue in decoders.queues() {
                    queue.put(Item::End(serial));
                }
                read_all = true;
                shared.state().read_all = true;
            }
        }
    }
    decoders.stop();
    drop(output);
}

/// Decodes audio track `index` into `output`, and notes the track that
/// plays: none when there is no output or the track will not decode.
fn start_audio(
    index: Option<usize>,
    input: &ffmpeg::format::context::Input,
    start: f64,
    output: Option<&audio::Output>,
    shared: &Arc<Shared>,
    decoders: &mut Decoders,
) {
    let mut chosen = None;
    if let (Some(index), Some(output)) = (index, output)
        && let Ok((queue, thread)) =
            decode::spawn_audio(input, index, start, output.shared.clone(), shared.clone())
    {
        decoders.audio = Some((index, queue, thread));
        chosen = Some(index);
    }
    let mut state = shared.state();
    state.audio = chosen;
    state.has_audio = chosen.is_some();
}

/// A drag along the timeline: where its pictures are headed.
struct Scrub {
    /// The exact target pictures and sound go to, for `serial`.
    target: f64,
    serial: u64,
    /// When the seek for `serial` began.
    since: Instant,
    /// The drag ended: land on the last position, then let the clock go.
    ending: bool,
}

/// How long a drag waits for a seek's picture before seeking again.
const PATIENCE: Duration = Duration::from_secs(1);

/// How far ahead decoding goes on rather than seeking, in a file without
/// an index of its keyframes.
const DECODE_ON: f64 = 1.0;

/// Moves a drag's pictures toward the handle. Ahead of them, with no
/// keyframe between, decoding goes on to it; otherwise a seek lands
/// exactly on it, once the last seek's picture has come, so pictures
/// follow the handle instead of starting over from a keyframe at every
/// move. Returns whether it sought.
fn steer(
    scrub: &mut Option<Scrub>,
    input: &mut ffmpeg::format::context::Input,
    shared: &Shared,
    decoders: &mut Decoders,
    video_stream: Option<usize>,
    start: f64,
) -> bool {
    let wanted = *shared
        .scrub_to
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    let Some(wanted) = wanted else {
        return false;
    };
    let serial = shared.serial.load(Ordering::Relaxed);
    let Some(current) = scrub else {
        seek(input, shared, decoders, wanted, true, start);
        *scrub = Some(Scrub {
            target: wanted,
            serial: shared.serial.load(Ordering::Relaxed),
            since: Instant::now(),
            ending: false,
        });
        return true;
    };
    let same = current.serial == serial;
    if current.target == wanted {
        if current.ending
            && (!same || arrived(shared, serial, wanted) || current.since.elapsed() > PATIENCE)
        {
            *shared
                .scrub_to
                .lock()
                .unwrap_or_else(PoisonError::into_inner) = None;
            *scrub = None;
            shared.clock.hold(false);
        }
        return false;
    }
    if same
        && wanted > current.target
        && decodes_on(input, shared, video_stream, start, current, wanted)
    {
        *shared.target.lock().unwrap_or_else(PoisonError::into_inner) = Some((serial, wanted));
        shared.clock.jump(wanted);
        if let Some(output) = shared.audio_output() {
            output.discard_before(serial, wanted);
        }
        current.target = wanted;
        return false;
    }
    if same && !arrived(shared, serial, current.target) && current.since.elapsed() < PATIENCE {
        return false;
    }
    seek(input, shared, decoders, wanted, true, start);
    current.target = wanted;
    current.serial = shared.serial.load(Ordering::Relaxed);
    current.since = Instant::now();
    true
}

/// Whether decoding on reaches `wanted` sooner than a seek would: no
/// keyframe lies between it and where the decoder is.
fn decodes_on(
    input: &ffmpeg::format::context::Input,
    shared: &Shared,
    video_stream: Option<usize>,
    start: f64,
    scrub: &Scrub,
    wanted: f64,
) -> bool {
    // Sound alone seeks quickly.
    let Some(stream) = video_stream else {
        return false;
    };
    let keyframe = |seconds| crate::ffi::keyframe_before(input, stream, seconds, start);
    let (of, decoded) = *shared
        .decoded
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    // Before its first picture, the decoder starts from the target's
    // keyframe.
    let reached = if of == scrub.serial {
        decoded
    } else {
        keyframe(scrub.target).unwrap_or(scrub.target)
    };
    match keyframe(wanted) {
        Some(keyframe) => keyframe <= reached + 1e-3,
        None => wanted - reached <= DECODE_ON,
    }
}

/// Whether a seek of `serial` has reached `target`: its picture shown, so
/// the next seek does not throw it away unseen, or for sound alone, its
/// sound come.
fn arrived(shared: &Shared, serial: u64, target: f64) -> bool {
    if shared.state().has_video {
        let (of, decoded) = *shared
            .decoded
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let video = shared.video();
        // Pictures kept start at most half a frame before the target.
        let decoded = of == serial && decoded >= target - 0.05;
        (decoded && video.settled(serial, target)) || (video.serial() == serial && video.finished)
    } else {
        shared
            .audio_output()
            .is_none_or(|output| output.reached(serial))
    }
}

/// Where subtitle files beside a video are numbered from among its tracks,
/// past any stream a file has.
pub const SUBTITLE_FILE_TRACKS: usize = 1 << 20;

/// Reads the subtitle files beside the video at `path` and lists them
/// among `info`'s subtitle tracks. Returns their cues, by their track
/// number less `SUBTITLE_FILE_TRACKS`.
fn read_subtitle_files(path: &Path, info: &mut MediaInfo) -> Vec<SubtitleFile> {
    let stem = path
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_default();
    let mut all = Vec::new();
    for file in subtitle::beside(path) {
        let Some((mut track, cues)) = decode::read_subtitle_file(&file) else {
            continue;
        };
        let (language, forced) = subtitle::file_language(&stem, &file, decode::is_language_code);
        track.index = SUBTITLE_FILE_TRACKS + all.len();
        track.language = language;
        track.forced = forced;
        info.subtitles.push(crate::info::SubtitleTrack {
            track,
            text: true,
            file: Some(file.clone()),
        });
        all.push(SubtitleFile {
            path: file,
            cues: Arc::new(cues),
        });
    }
    all
}

/// A subtitle file beside the video: where it is, and its lines as text.
struct SubtitleFile {
    path: PathBuf,
    cues: Arc<Vec<Cue>>,
}

/// Subtitle track `index`, when it is a file's.
fn subtitle_file(files: &[SubtitleFile], index: usize) -> Option<&SubtitleFile> {
    index
        .checked_sub(SUBTITLE_FILE_TRACKS)
        .and_then(|number| files.get(number))
}

/// What choosing a subtitle track needs to know of the file.
struct Subtitles<'a> {
    input: &'a ffmpeg::format::context::Input,
    info: &'a MediaInfo,
    start: f64,
    /// The video's size, which picture subtitles are placed in.
    canvas: (u32, u32),
    files: &'a [SubtitleFile],
}

impl Subtitles<'_> {
    /// Shows subtitle track `index`, or none: a file beside the video, as
    /// text or drawn by libass, the video's own ASS drawn by libass, or
    /// FFmpeg's decoder's text and pictures. Returns the track showing.
    fn choose(
        &self,
        index: Option<usize>,
        shared: &Shared,
        decoders: &mut Decoders,
    ) -> Option<usize> {
        shared.cues().clear();
        shared.picture_cues().clear();
        *shared.ass() = None;
        *shared
            .file_cues
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = None;
        decoders.subtitles = None;
        let index = index?;
        // libass draws over a picture; sound alone has none to draw on.
        let drawn = self.info.has_video() && crate::ass::available();
        let (canvas, video) = (ass_canvas(self.canvas), self.canvas);
        if let Some(file) = subtitle_file(self.files, index) {
            let styled = file.path.extension().is_some_and(|extension| {
                matches!(
                    extension.to_string_lossy().to_lowercase().as_str(),
                    "ass" | "ssa"
                )
            });
            let track = (drawn && styled)
                .then(|| std::fs::read(&file.path).ok())
                .flatten()
                .and_then(|contents| crate::ass::AssTrack::read(&contents, canvas, video));
            match track {
                Some(track) => *shared.ass() = Some(track),
                None => {
                    *shared
                        .file_cues
                        .lock()
                        .unwrap_or_else(PoisonError::into_inner) = Some(file.cues.clone());
                }
            }
            return Some(index);
        }
        let stream = self.input.stream(index)?;
        let styled = self.info.subtitles.iter().any(|track| {
            track.track.index == index && matches!(track.track.codec.as_str(), "ass" | "ssa")
        });
        if drawn && styled {
            let header = crate::ffi::stream_extradata(&stream).unwrap_or_default();
            if let Some(track) =
                crate::ass::AssTrack::new(&header, &fonts(self.input), canvas, video)
            {
                *shared.ass() = Some(track);
                decoders.subtitles = Some((index, SubtitleSink::Ass(stream.time_base())));
                return Some(index);
            }
        }
        let decoder =
            decode::SubtitleDecoder::new(self.input, index, self.start, self.canvas).ok()?;
        decoders.subtitles = Some((index, SubtitleSink::Decoder(decoder)));
        Some(index)
    }
}

/// The frame libass draws in: the video's, at least 1080 lines tall, so
/// lines over a small video are drawn sharp rather than enlarged.
fn ass_canvas((width, height): (u32, u32)) -> (u32, u32) {
    let scale = (1080.0 / f64::from(height.max(1))).max(1.0);
    (
        (f64::from(width) * scale).round() as u32,
        (f64::from(height) * scale).round() as u32,
    )
}

/// The fonts a file carries for its ASS subtitles: names and data.
fn fonts(input: &ffmpeg::format::context::Input) -> Vec<(String, Vec<u8>)> {
    input
        .streams()
        .filter(|stream| stream.parameters().medium() == ffmpeg::media::Type::Attachment)
        .filter_map(|stream| {
            let metadata = stream.metadata();
            let name = metadata.get("filename").unwrap_or_default().to_owned();
            let mime = metadata.get("mimetype").unwrap_or_default().to_lowercase();
            let lower = name.to_lowercase();
            let font = mime.contains("font")
                || mime.contains("opentype")
                || mime.contains("truetype")
                || [".ttf", ".otf", ".ttc"]
                    .iter()
                    .any(|ending| lower.ends_with(ending));
            font.then(|| Some((name, crate::ffi::stream_extradata(&stream)?)))
                .flatten()
        })
        .collect()
}

/// Notices the end of playback: everything read, decoded and played,
/// and holds the clock there.
fn check_ended(shared: &Shared) -> bool {
    let mut state = shared.state();
    if state.ended {
        return true;
    }
    if !state.read_all || state.info.is_none() {
        return false;
    }
    let video_done = !state.has_video || {
        let video = shared.video();
        video.finished && video.is_empty()
    };
    let audio_done =
        !state.has_audio || shared.audio_output().is_none_or(|output| output.drained());
    if video_done && audio_done {
        state.ended = true;
        drop(state);
        shared.clock.set_paused(true);
        return true;
    }
    false
}

/// Moves the reader and every decoder to `position`, seconds from the
/// start of the file.
fn seek(
    input: &mut ffmpeg::format::context::Input,
    shared: &Shared,
    decoders: &mut Decoders,
    position: f64,
    exact: bool,
    start: f64,
) {
    let serial = shared.serial.fetch_add(1, Ordering::Relaxed) + 1;
    *shared.target.lock().unwrap_or_else(PoisonError::into_inner) =
        exact.then_some((serial, position));
    let timestamp = ((position + start) * f64::from(ffmpeg::ffi::AV_TIME_BASE)) as i64;
    if let Err(error) = input.seek(timestamp, ..timestamp) {
        eprintln!("plaa: seeking: {error}");
    }
    for queue in decoders.queues() {
        queue.clear();
    }
    shared.video().flush(serial);
    shared.video_room.notify_all();
    if let Some(output) = shared.audio_output() {
        output.flush(serial);
    }
    shared.cues().clear();
    shared.picture_cues().clear();
    if let Some((_, SubtitleSink::Decoder(decoder))) = &mut decoders.subtitles {
        decoder.flush();
    }
    shared.clock.jump(position);
    shared.clock.stall();
    shared.state().ended = false;
}

fn describe(error: &ffmpeg::Error) -> String {
    match error {
        ffmpeg::Error::InvalidData => "this is not a file plaa can read".to_owned(),
        ffmpeg::Error::Other { errno } if *errno == libc_enoent() => {
            "the file does not exist".to_owned()
        }
        error => error.to_string(),
    }
}

/// ENOENT, the same on every system plaa runs on.
const fn libc_enoent() -> i32 {
    2
}
