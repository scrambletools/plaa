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
use crate::edit::{Edit, OWN, Span};
use crate::frame::{self, VideoFrame};
use crate::info::MediaInfo;
use crate::inputs::{self, Inputs, SOUND_TRACKS};
use crate::lanes::Mixed;
use crate::queue::{Item, PacketQueue};
use crate::subtitle::{self, Cue, Cues, PictureCues, SubtitlePicture};
use crate::web::{Entry, Page};

/// What to open: a file, an address FFmpeg reads, such as an HTTP or
/// RTSP stream, or sound arriving live from elsewhere in the program, as
/// a network stream plaa listens to, by its own id and name, its sound
/// from [`Options::live`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Source {
    File(PathBuf),
    Url(String),
    Live { id: String, name: String },
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
            Source::Live { name, .. } => name.clone(),
        }
    }

    pub(crate) fn path(&self) -> &Path {
        match self {
            Source::File(path) => path,
            Source::Url(url) => Path::new(url),
            Source::Live { id, .. } => Path::new(id),
        }
    }

    pub fn is_url(&self) -> bool {
        matches!(self, Source::Url(_))
    }

    pub fn is_live(&self) -> bool {
        matches!(self, Source::Live { .. })
    }
}

/// Where a live source's sound comes from: given the output's rate, a
/// pull that fills two interleaved channels at that rate with what came,
/// taken a chunk at a time as a sound device takes it.
#[derive(Clone)]
pub struct LiveSound(pub Arc<dyn Fn(u32) -> LivePull + Send + Sync>);

/// Fills its slice with the next frames, silence where none came.
pub type LivePull = Box<dyn FnMut(&mut [f32]) + Send>;

impl std::fmt::Debug for LiveSound {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("LiveSound")
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
    /// Which version to play, where the source offers several.
    pub quality: Quality,
    /// Reads web pages with yt-dlp, when it is installed.
    pub web: bool,
    /// The web page yt-dlp read for the source already, so opening it
    /// again, as in another quality, does not ask it again.
    pub page: Option<Arc<Page>>,
    /// A live source's sound.
    pub live: Option<LiveSound>,
}

/// Which of a source's versions to play, as an HLS stream's qualities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Quality {
    /// The best there is.
    #[default]
    Best,
    /// The best whose picture is at most this many lines tall, else the
    /// smallest.
    AtMost(u32),
    /// Sound alone.
    AudioOnly,
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
            quality: Quality::Best,
            web: true,
            page: None,
            live: None,
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
    /// The source is a web page yt-dlp read: what it says of itself, and
    /// the qualities it comes in.
    Page(Arc<Page>),
    /// The source is a list of web pages, such as a playlist, for the
    /// interface to play one at a time, and what the list is called.
    List(Option<String>, Vec<Entry>),
    /// The source looks like a web page, and plaa reads those with
    /// yt-dlp, which is not installed.
    NeedsYtDlp,
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
    /// How far ahead the source has been read.
    pub buffered: Option<Duration>,
    pub speed: f64,
    /// The streams playing: picture, sound and subtitles.
    pub video: Option<usize>,
    pub audio: Option<usize>,
    pub subtitles: Option<usize>,
    /// A seek was asked for and has yet to be made, so the position is
    /// about to jump.
    pub seeking: bool,
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
    SelectVideo(usize),
    SelectAudio(Option<usize>),
    SelectSubtitles(Option<usize>),
    /// Plays this edit of the file, or all of it when none, from this
    /// many seconds of the edited time.
    SetEdit(Option<Arc<Edit>>, f64),
}

/// What the player's threads and its handle share.
pub(crate) struct Shared {
    pub clock: Arc<Clock>,
    pub video: Mutex<frame::Queue>,
    pub video_room: Condvar,
    pub audio: Mutex<Option<Arc<audio::Shared>>>,
    /// Where a copy of what plays goes, for every output the player opens.
    pub copy: Mutex<Option<audio::SoundCopy>>,
    pub cues: Mutex<Cues>,
    /// Picture subtitles decoded ahead of the clock.
    pub picture_cues: Mutex<PictureCues>,
    /// The ASS track libass draws, when it shows.
    pub ass: Mutex<Option<crate::ass::AssTrack>>,
    /// The latest seek's serial: older packets, pictures and sound are
    /// dropped.
    pub serial: AtomicU64,
    /// Seeks asked for that the reader has yet to make.
    pub seeks: AtomicU64,
    /// The media time read up to, as f64 bits: NaN when unknown.
    pub buffered: AtomicU64,
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
    /// The quality to choose, which may change until the source is open.
    pub quality: Mutex<Quality>,
    /// The edit playing, if the file is edited: its time is what the
    /// clock, pictures and sound count in.
    pub edit: Mutex<Option<Arc<Edit>>>,
    /// The edit playing mixes files, so its picture and sound are read
    /// apart, and sound is placed exactly.
    pub mixed: AtomicBool,
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
    pub video: Option<usize>,
    pub audio: Option<usize>,
    pub subtitles: Option<usize>,
    /// How long the edit playing lasts, if the file is edited.
    pub edited: Option<Duration>,
}

impl Shared {
    pub fn state(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn video(&self) -> MutexGuard<'_, frame::Queue> {
        self.video.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Plays through `output` from now on, handing it where the copy of
    /// what plays goes.
    pub fn use_output(&self, output: Option<Arc<audio::Shared>>) {
        if let Some(output) = &output {
            output.set_copy(
                self.copy
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .clone(),
            );
        }
        *self.audio.lock().unwrap_or_else(PoisonError::into_inner) = output;
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

    pub fn quality(&self) -> Quality {
        *self.quality.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// How long playback lasts: the edit's length, else the file's.
    pub fn duration(&self) -> Option<Duration> {
        let state = self.state();
        state
            .edited
            .or_else(|| state.info.as_ref().and_then(|info| info.duration))
    }

    /// The file's time at `seconds` of what plays: none while a clip from
    /// another file plays.
    pub fn source_time(&self, seconds: f64) -> Option<f64> {
        match &*self.edit.lock().unwrap_or_else(PoisonError::into_inner) {
            Some(edit) => match edit.locate(seconds) {
                Some((OWN, time)) => Some(time),
                Some(_) => None,
                None => Some(seconds),
            },
            None => Some(seconds),
        }
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
            copy: Mutex::new(None),
            cues: Mutex::new(Cues::default()),
            picture_cues: Mutex::new(PictureCues::default()),
            ass: Mutex::new(None),
            serial: AtomicU64::new(0),
            seeks: AtomicU64::new(0),
            buffered: AtomicU64::new(f64::NAN.to_bits()),
            target: Mutex::new(None),
            decoded: Mutex::new((0, 0.0)),
            scrub_to: Mutex::new(None),
            ab_loop: Mutex::new(None),
            file_cues: Mutex::new(None),
            stop: Arc::new(AtomicBool::new(false)),
            speed: AtomicU64::new(1.0f64.to_bits()),
            quality: Mutex::new(options.quality),
            edit: Mutex::new(None),
            mixed: AtomicBool::new(false),
            state: Mutex::new(State::default()),
        });
        let (commands, receiver) = mpsc::channel();
        let reader = {
            let shared = shared.clone();
            let source = source.clone();
            std::thread::Builder::new()
                .name("plaa-read".into())
                .spawn(move || {
                    read(source, options, shared.clone(), receiver, Box::new(events));
                    // Seeks still asked for will never be made.
                    shared.seeks.store(0, Ordering::Relaxed);
                })
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
        let duration = self.shared.duration();
        let position = match duration {
            Some(duration) => position.min(duration),
            None => position,
        };
        {
            let mut state = self.shared.state();
            state.ended = false;
        }
        self.shared.seeks.fetch_add(1, Ordering::Relaxed);
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
        let duration = self.shared.duration();
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

    /// Hands a copy of what plays to `to`, from the output device's
    /// thread, or to nothing.
    pub fn set_copy(&self, to: Option<audio::SoundCopy>) {
        *self
            .shared
            .copy
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = to.clone();
        if let Some(output) = self.shared.audio_output() {
            output.set_copy(to);
        }
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

    /// The quality to choose among the source's, if it has yet to choose:
    /// for when the screen it shows on is known only after it starts
    /// opening.
    pub fn prefer(&self, quality: Quality) {
        *self
            .shared
            .quality
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = quality;
    }

    /// Shows picture stream `stream`, as another of the source's variants,
    /// with its own sound if it has some.
    pub fn select_video(&self, stream: usize) {
        let _ = self.commands.send(Command::SelectVideo(stream));
    }

    pub fn select_audio(&self, stream: Option<usize>) {
        let _ = self.commands.send(Command::SelectAudio(stream));
    }

    pub fn select_subtitles(&self, stream: Option<usize>) {
        let _ = self.commands.send(Command::SelectSubtitles(stream));
    }

    /// Plays `edit` of the file, or all of it when none, from `position`
    /// of the edited time. Positions and the duration count in the
    /// edit's time from then on.
    pub fn set_edit(&self, edit: Option<Arc<Edit>>, position: Duration) {
        {
            let mut state = self.shared.state();
            state.edited = edit
                .as_ref()
                .map(|edit| Duration::from_secs_f64(edit.duration()));
            state.ended = false;
        }
        *self
            .shared
            .edit
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = edit.clone();
        self.shared.seeks.fetch_add(1, Ordering::Relaxed);
        let _ = self
            .commands
            .send(Command::SetEdit(edit, position.as_secs_f64()));
    }

    /// The media time now: of the edit, if the file is edited.
    pub fn position(&self) -> Duration {
        let seconds = self.shared.clock.now().max(0.0);
        let duration = self.shared.duration();
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

    /// The `count` frames heard last, mixed to one channel, and their
    /// rate, for a visualizer; None while nothing is heard.
    pub fn heard(&self, count: usize) -> Option<(Vec<f32>, u32)> {
        let output = self.shared.audio_output()?;
        Some((output.heard(count)?, output.rate))
    }

    /// How far ahead the source has been read, if known.
    pub fn buffered(&self) -> Option<Duration> {
        let seconds = f64::from_bits(self.shared.buffered.load(Ordering::Relaxed));
        (seconds.is_finite() && seconds >= 0.0).then(|| Duration::from_secs_f64(seconds))
    }

    pub fn status(&self) -> Status {
        check_ended(&self.shared);
        let state = self.shared.state().clone();
        let position = self.position();
        let buffered = self.buffered();
        let paused = self.shared.clock.paused();
        let buffering = !paused
            && self
                .shared
                .clock
                .waited()
                .is_some_and(|waited| waited >= BUFFERING_SHOWN);
        Status {
            position,
            duration: self.shared.duration(),
            paused,
            ended: state.ended,
            buffering,
            buffered,
            speed: self.shared.speed(),
            video: state.video,
            audio: state.audio,
            subtitles: state.subtitles,
            seeking: self.shared.seeks.load(Ordering::Relaxed) > 0,
        }
    }

    /// The picture to draw now. Called on every frame the interface draws.
    pub fn picture(&self) -> Option<Arc<VideoFrame>> {
        let now = self.shared.clock.now();
        let (shown, first_of_seek, ran_dry) = {
            let mut video = self.shared.video();
            let first = video.has_new();
            let shown = video.take_due(now);
            (shown, first, video.is_empty() && !video.finished)
        };
        self.shared.video_room.notify_all();
        let (has_audio, read_all) = {
            let state = self.shared.state();
            (state.has_audio, state.read_all)
        };
        // Without sound, the clock starts from the first picture after a
        // seek, so it does not run ahead while that is decoded.
        if first_of_seek
            && !has_audio
            && let Some(shown) = &shown
        {
            self.shared.clock.anchor(shown.position, Instant::now());
        }
        // Nor does it run on once the source has run out of pictures, as a
        // stream does: it waits for the source to read ahead, as for sound.
        // Pictures decoding slower than they play still have packets read.
        let starved = self
            .buffered()
            .is_none_or(|read| read.as_secs_f64() < now + STARVED);
        if !has_audio && ran_dry && !read_all && starved && self.shared.clock.running() {
            self.shared.clock.wait();
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
        let now = self.shared.source_time(self.shared.clock.now())?;
        if let Some(track) = self.shared.ass().as_mut() {
            return track.render(now);
        }
        self.shared.picture_cues().at(now)
    }

    /// The subtitle text showing now.
    pub fn subtitle(&self) -> Option<Cue> {
        let now = self.shared.source_time(self.shared.clock.now())?;
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

/// How far ahead a source reads before playing on after a wait, in
/// seconds of media time.
const READY: f64 = 2.0;

/// How far ahead a stream over a network reads at most: in media time,
/// and in memory.
const STREAM_AHEAD: f64 = 30.0;
const STREAM_BYTES: usize = 128 * 1024 * 1024;

/// A source read less than this far past the clock has run out.
const STARVED: f64 = 0.1;

/// Waits shorter than this, as after most seeks, are not worth showing.
const BUFFERING_SHOWN: Duration = Duration::from_millis(300);

/// The streams being decoded, each with its packet queue and thread.
pub(crate) struct Decoders {
    pub video: Option<(usize, Arc<PacketQueue>, JoinHandle<()>)>,
    pub audio: Option<(usize, Arc<PacketQueue>, JoinHandle<()>)>,
    pub subtitles: Option<(usize, SubtitleSink)>,
    /// The picture and the sound are read apart, as for an edit that
    /// mixes files.
    pub apart: bool,
}

/// Where a subtitle track's packets go.
pub(crate) enum SubtitleSink {
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

    /// Whether enough is read ahead of `now` for now. Streams over a
    /// network read much further ahead than files, to ride out the
    /// network slowing down.
    fn has_enough(&self, network: bool, now: f64) -> bool {
        let total: usize = self.queues().map(|queue| queue.bytes()).sum();
        if network {
            total > STREAM_BYTES || self.read_to().is_some_and(|end| end - now >= STREAM_AHEAD)
        } else {
            total > 24 * 1024 * 1024 || self.queues().all(|queue| queue.has_enough())
        }
    }

    /// The media time the source has been read up to. An input's streams
    /// are read in step, so for one input, the latest packet's, as a stream
    /// with nothing for a while would hold back the earliest. Inputs read
    /// apart get as far as the one behind.
    fn read_to(&self) -> Option<f64> {
        if self.sound_apart() || self.apart {
            self.queues().try_fold(f64::INFINITY, |earliest, queue| {
                Some(earliest.min(queue.end()?))
            })
        } else {
            self.queues()
                .filter_map(|queue| queue.end())
                .reduce(f64::max)
        }
    }

    /// The streams decoding, which a part's end waits for.
    fn playing(&self) -> Vec<usize> {
        self.video
            .iter()
            .chain(&self.audio)
            .map(|(index, ..)| *index)
            .collect()
    }

    /// Whether the sound plays from an input of its own.
    fn sound_apart(&self) -> bool {
        self.audio
            .as_ref()
            .is_some_and(|(index, ..)| *index >= SOUND_TRACKS)
    }

    fn stop_audio(&mut self) {
        if let Some((_, queue, thread)) = self.audio.take() {
            queue.close();
            let _ = thread.join();
        }
    }

    fn stop_video(&mut self) {
        if let Some((_, queue, thread)) = self.video.take() {
            queue.close();
            let _ = thread.join();
        }
    }

    fn stop(&mut self) {
        self.stop_audio();
        self.stop_video();
    }

    /// Lets the reader skip every stream nothing plays, which over a
    /// network is not fetched at all: an HLS stream's other qualities.
    fn skip_unused(&self, inputs: &mut Inputs) {
        let playing = [
            self.video.as_ref().map(|(index, ..)| *index),
            self.audio.as_ref().map(|(index, ..)| *index),
            self.subtitles.as_ref().map(|(index, _)| *index),
        ];
        for (input, first, _) in inputs.each_mut() {
            for index in 0..input.nb_streams() as usize {
                let skipped = !playing.contains(&Some(first + index));
                crate::ffi::set_skipped(input, index, skipped);
            }
        }
    }
}

/// Packets read without a decoding timestamp, in an order that may run
/// ahead of their pictures', count as past a part's end only this far
/// past it.
const REORDERED: f64 = 1.0;

/// A stream this far past a part's end while another has yet to reach it
/// takes the other as done too, as when the other's sound ends early.
const LAGGING: f64 = 3.0;

/// Where the reader is among the parts of the file it plays: the part its
/// packets are for, and the streams playing that have read past its end.
struct Reading {
    spans: Vec<Span>,
    current: usize,
    passed: Vec<usize>,
    /// For an edit that mixes files, its lanes, read in place of these
    /// parts.
    mixed: Option<Mixed>,
}

impl Reading {
    /// Reads `spans`, or all of the file when none.
    fn new(spans: Vec<Span>) -> Self {
        Self {
            spans: if spans.is_empty() {
                vec![Span::WHOLE]
            } else {
                spans
            },
            current: 0,
            passed: Vec::new(),
            mixed: None,
        }
    }

    fn span(&self) -> Span {
        self.spans[self.current]
    }

    fn has_next(&self) -> bool {
        self.current + 1 < self.spans.len()
    }

    /// The part `seconds` of the edited time is in, and the file's time
    /// there.
    fn locate(&self, seconds: f64) -> (usize, f64) {
        let index = self
            .spans
            .iter()
            .position(|span| seconds < span.to + span.shift)
            .unwrap_or(self.spans.len() - 1);
        let span = self.spans[index];
        (index, (seconds - span.shift).clamp(span.from, span.to))
    }

    /// Starts reading part `index` from the file's time `seconds`.
    fn go_to(&mut self, index: usize, inputs: &mut Inputs, seconds: f64) {
        self.current = index;
        self.passed.clear();
        inputs.seek(seconds);
    }

    /// Notes that `stream` read a packet at `seconds` of the file, by its
    /// decoding time when `decoding`. Returns whether that is past the
    /// part's end, so the packet belongs to no clip here.
    fn passes(&mut self, stream: usize, seconds: f64, decoding: bool, playing: &[usize]) -> bool {
        let end = self.span().to;
        let margin = if decoding { 0.0 } else { REORDERED };
        if seconds < end + margin {
            return false;
        }
        if seconds >= end + LAGGING {
            self.passed = playing.to_vec();
        } else if !self.passed.contains(&stream) {
            self.passed.push(stream);
        }
        true
    }

    /// Whether every stream playing has read past the part's end.
    fn finished(&self, playing: &[usize]) -> bool {
        playing.iter().all(|stream| self.passed.contains(stream))
    }
}

/// The reading thread: opens the source, starts the decoders and feeds
/// them until stopped.
/// How much live sound waits for the output, little, so it is heard soon
/// after it arrives.
const LIVE_AHEAD: Duration = Duration::from_millis(40);

/// How much live sound is taken at a time.
const LIVE_CHUNK: Duration = Duration::from_millis(10);

/// Plays a live source's sound as it comes, at the output's rate: it has
/// no duration and cannot seek, and it plays from where it is on a new
/// output, as the sound arrives at that one's rate.
fn read_live(
    source: &Source,
    options: Options,
    shared: &Shared,
    commands: &Receiver<Command>,
    events: &dyn Fn(Event),
) {
    let Some(live) = options.live.clone() else {
        events(Event::Failed("nothing sends this sound".to_owned()));
        return;
    };
    let open = |device: Option<&str>| {
        let output =
            audio::Output::open(shared.clock.clone(), options.volume, options.muted, device)
                .unwrap_or_else(|error| {
                    events(Event::NoSound(error.to_string()));
                    audio::Output::silent(shared.clock.clone(), options.volume, options.muted)
                });
        let pull = (live.0)(output.shared.rate);
        shared.use_output(Some(output.shared.clone()));
        (output, pull)
    };
    let (mut output, mut pull) = open(options.audio_device.as_deref());
    let info = Arc::new(MediaInfo {
        format: source.name(),
        audio: vec![crate::info::AudioTrack {
            sample_rate: output.shared.rate,
            channels: 2,
            layout: "stereo".to_owned(),
            ..crate::info::AudioTrack::default()
        }],
        ..MediaInfo::default()
    });
    shared.state().info = Some(info.clone());
    events(Event::Opened { info, cover: None });
    shared.clock.set_paused(options.paused);
    let mut position = 0.0;
    while !shared.stopping() {
        for command in commands.try_iter() {
            if let Command::SetAudioDevice(device) = command {
                // Volume and mute as they are now.
                let (volume, muted) = output.shared.volume();
                drop(output);
                (output, pull) = open(device.as_deref());
                output.shared.set_volume(volume);
                output.shared.set_muted(muted);
            }
        }
        let sound = &output.shared;
        let rate = sound.rate.max(1);
        let ahead = (LIVE_AHEAD.as_secs_f64() * f64::from(rate)) as usize;
        if shared.clock.paused() || sound.queued() >= ahead {
            if sound.queued() > 0 {
                shared.clock.end_wait();
            }
            std::thread::sleep(LIVE_CHUNK / 2);
            continue;
        }
        let frames = (LIVE_CHUNK.as_secs_f64() * f64::from(rate)) as usize;
        let mut two = vec![0.0f32; frames * 2];
        pull(&mut two);
        // Its two channels on the output's first two.
        let channels = usize::from(sound.channels.max(1));
        let samples: Vec<f32> = two
            .as_chunks::<2>()
            .0
            .iter()
            .flat_map(|frame| {
                (0..channels).map(move |channel| frame.get(channel).copied().unwrap_or(0.0))
            })
            .collect();
        let serial = shared.serial.load(Ordering::Relaxed);
        let step = 1.0 / f64::from(rate);
        if sound.push(serial, position, step, samples, &shared.stop) {
            position += frames as f64 * step;
        }
        shared.clock.end_wait();
    }
}

fn read(
    source: Source,
    options: Options,
    shared: Arc<Shared>,
    commands: Receiver<Command>,
    events: Box<dyn Fn(Event) + Send>,
) {
    if source.is_live() {
        read_live(&source, options, &shared, &commands, &*events);
        return;
    }
    let Some(opened) = inputs::open(&source, &options, &shared, &*events) else {
        return;
    };
    let mut inputs = opened.inputs;
    let mut info = MediaInfo::read(&inputs.main);
    let sound = inputs
        .sound
        .as_ref()
        .map(|(sound, _)| MediaInfo::read(sound));
    inputs::merge(&mut info, sound, opened.page.as_deref());
    let subtitle_files = match &source {
        Source::File(path) => read_subtitle_files(path, &mut info),
        Source::Url(_) | Source::Live { .. } => Vec::new(),
    };
    let info = Arc::new(info);
    let start = inputs.start;

    let video_stream = decode::pick_video(&info, shared.quality());
    // The output opens before the decoders, which resample to its format.
    let audio_stream = decode::pick_audio(&info.audio_with(video_stream), &options.audio_languages);
    let mut output = None;
    if audio_stream.is_some() {
        match audio::Output::open(
            shared.clock.clone(),
            options.volume,
            options.muted,
            options.audio_device.as_deref(),
        ) {
            Ok(opened) => output = Some(opened),
            Err(error) => {
                events(Event::NoSound(error.to_string()));
                output = Some(audio::Output::silent(
                    shared.clock.clone(),
                    options.volume,
                    options.muted,
                ));
            }
        }
    }
    shared.use_output(output.as_ref().map(|output| output.shared.clone()));

    let mut decoders = Decoders {
        video: None,
        audio: None,
        subtitles: None,
        apart: false,
    };
    start_video(
        video_stream,
        &inputs.main,
        start,
        options.hardware,
        &shared,
        &mut decoders,
        &events,
    );
    start_audio(
        audio_stream,
        &inputs,
        output.as_ref(),
        &shared,
        &mut decoders,
    );
    let subtitle_stream = decode::pick_subtitles(&info, &options.subtitle_languages);
    // The frame picture subtitles are placed in, unless they say: the
    // video's.
    let canvas = info
        .video
        .iter()
        .find(|track| !track.cover)
        .map_or((1920, 1080), |track| (track.width, track.height));
    let showing = Subtitles {
        input: &inputs.main,
        info: &info,
        start,
        canvas,
        files: &subtitle_files,
    }
    .choose(subtitle_stream, &shared, &mut decoders);
    let cover = info
        .cover()
        .and_then(|track| decode::cover(&inputs.main, track.track.index))
        .or(opened.thumbnail)
        .map(Arc::new);
    decoders.skip_unused(&mut inputs);
    {
        let mut state = shared.state();
        state.info = Some(info.clone());
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
    // picture, and for the source to read ahead.
    shared.clock.stall();
    shared.clock.wait();
    shared.clock.set_paused(options.paused);
    let mut reading = Reading::new(Vec::new());
    if let Some(at) = options.start.filter(|_| info.seekable) {
        seek(
            &mut inputs,
            &mut reading,
            &shared,
            &mut decoders,
            at.as_secs_f64(),
            true,
        );
    }

    let mut read_all = false;
    let mut announced_end = false;
    let mut scrub: Option<Scrub> = None;
    // Where playback was last time round, to see it reach a loop's end.
    let mut played = shared.clock.now();
    let mut packet = ffmpeg::Packet::empty();
    let network = source.is_url();
    while !shared.stopping() {
        let now = shared.clock.now();
        let read_to = decoders.read_to();
        shared
            .buffered
            .store(read_to.unwrap_or(f64::NAN).to_bits(), Ordering::Relaxed);
        // Waiting for the source: on once it has read far enough ahead,
        // or as far as it will.
        if shared.clock.waited().is_some()
            && (read_all
                || decoders.has_enough(network, now)
                || read_to.is_some_and(|end| end - now >= READY))
        {
            shared.clock.end_wait();
        }
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
            seek(
                &mut inputs,
                &mut reading,
                &shared,
                &mut decoders,
                from,
                true,
            );
            read_all = false;
            announced_end = false;
            shared.state().read_all = false;
        }
        played = shared.clock.now();
        if info.seekable
            && scrub.is_some()
            && steer(
                &mut scrub,
                &mut inputs,
                &mut reading,
                &shared,
                &mut decoders,
            )
        {
            read_all = false;
            announced_end = false;
            shared.state().read_all = false;
        }
        // Commands first, waiting for one only while there is nothing to
        // read for now, and not long while the timeline is dragged.
        let idle = read_all
            || match &reading.mixed {
                Some(mixed) => mixed.has_enough(&decoders),
                None => decoders.has_enough(network, now),
            };
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
                        seek(
                            &mut inputs,
                            &mut reading,
                            &shared,
                            &mut decoders,
                            position,
                            exact,
                        );
                        read_all = false;
                        announced_end = false;
                        shared.state().read_all = false;
                    }
                    shared.seeks.fetch_sub(1, Ordering::Relaxed);
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
                        &mut inputs,
                        &mut reading,
                        &shared,
                        &mut decoders,
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
                Command::SelectVideo(index) => {
                    decoders.stop_video();
                    start_video(
                        Some(index),
                        &inputs.main,
                        start,
                        options.hardware,
                        &shared,
                        &mut decoders,
                        &events,
                    );
                    // A variant with sound of its own plays it.
                    let tracks = info.audio_with(Some(index));
                    let playing = decoders.audio.as_ref().map(|(index, ..)| *index);
                    if !tracks
                        .iter()
                        .any(|track| Some(track.track.index) == playing)
                    {
                        decoders.stop_audio();
                        start_audio(
                            decode::pick_audio(&tracks, &options.audio_languages),
                            &inputs,
                            output.as_ref(),
                            &shared,
                            &mut decoders,
                        );
                    }
                    decoders.skip_unused(&mut inputs);
                    keep_other_sound(&reading, output.as_ref(), &shared, &mut decoders);
                    let now = shared.clock.now().max(0.0);
                    if info.seekable {
                        seek(&mut inputs, &mut reading, &shared, &mut decoders, now, true);
                        read_all = false;
                        shared.state().read_all = false;
                    }
                }
                Command::SelectAudio(index) => {
                    decoders.stop_audio();
                    start_audio(index, &inputs, output.as_ref(), &shared, &mut decoders);
                    decoders.skip_unused(&mut inputs);
                    // Back to where it was, so the new track starts in step.
                    let now = shared.clock.now().max(0.0);
                    if info.seekable {
                        seek(&mut inputs, &mut reading, &shared, &mut decoders, now, true);
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
                    drop(output.take());
                    match audio::Output::open(
                        shared.clock.clone(),
                        volume,
                        muted,
                        device.as_deref(),
                    ) {
                        Ok(opened) => output = Some(opened),
                        Err(error) => {
                            events(Event::NoSound(error.to_string()));
                            output =
                                Some(audio::Output::silent(shared.clock.clone(), volume, muted));
                        }
                    }
                    shared.use_output(output.as_ref().map(|output| output.shared.clone()));
                    start_audio(playing, &inputs, output.as_ref(), &shared, &mut decoders);
                    decoders.skip_unused(&mut inputs);
                    keep_other_sound(&reading, output.as_ref(), &shared, &mut decoders);
                    let now = shared.clock.now().max(0.0);
                    if info.seekable {
                        seek(&mut inputs, &mut reading, &shared, &mut decoders, now, true);
                        read_all = false;
                        shared.state().read_all = false;
                    }
                }
                Command::SetEdit(edit, position) => {
                    // A web page's picture and sound read apart are not
                    // edited.
                    if inputs.sound.is_none() {
                        let mixed = edit
                            .clone()
                            .filter(|edit| edit.is_mixed())
                            .and_then(|edit| {
                                let Source::File(path) = &source else {
                                    return None;
                                };
                                Some(Mixed::new(
                                    edit,
                                    path.clone(),
                                    options.hardware,
                                    (decoders.video.is_some(), decoders.audio.is_some()),
                                ))
                            });
                        let was_mixed = reading.mixed.is_some();
                        let spans = edit
                            .as_deref()
                            .filter(|_| mixed.is_none())
                            .map(Edit::spans)
                            .unwrap_or_default();
                        reading = Reading::new(spans);
                        if let Some(mixed) = mixed {
                            // The sound of other files plays even where the
                            // file edited has none.
                            if decoders.audio.is_none()
                                && let Some((path, number)) = mixed.first_sound()
                            {
                                if output.is_none() {
                                    output = Some(open_output(&shared, &options, &*events));
                                    shared.use_output(
                                        output.as_ref().map(|output| output.shared.clone()),
                                    );
                                }
                                start_other_audio(
                                    &path,
                                    number,
                                    output.as_ref(),
                                    &shared,
                                    &mut decoders,
                                );
                            }
                            shared.mixed.store(true, Ordering::Relaxed);
                            decoders.apart = true;
                            reading.mixed = Some(mixed);
                        } else if was_mixed {
                            // Back to the file's own streams.
                            shared.mixed.store(false, Ordering::Relaxed);
                            decoders.apart = false;
                            let video = decoders.video.as_ref().map(|(index, ..)| *index);
                            let audio = decoders.audio.as_ref().map(|(index, ..)| *index);
                            decoders.stop();
                            start_video(
                                video,
                                &inputs.main,
                                start,
                                options.hardware,
                                &shared,
                                &mut decoders,
                                &events,
                            );
                            start_audio(
                                audio.filter(|index| *index != OTHER_SOUND),
                                &inputs,
                                output.as_ref(),
                                &shared,
                                &mut decoders,
                            );
                            decoders.skip_unused(&mut inputs);
                        }
                    }
                    if info.seekable {
                        seek(
                            &mut inputs,
                            &mut reading,
                            &shared,
                            &mut decoders,
                            position,
                            true,
                        );
                        read_all = false;
                        announced_end = false;
                        shared.state().read_all = false;
                    }
                    shared.seeks.fetch_sub(1, Ordering::Relaxed);
                }
                Command::SelectSubtitles(index) => {
                    let showing = Subtitles {
                        input: &inputs.main,
                        info: &info,
                        start,
                        canvas,
                        files: &subtitle_files,
                    }
                    .choose(index, &shared, &mut decoders);
                    shared.state().subtitles = showing;
                    decoders.skip_unused(&mut inputs);
                    // A track's lines already passed show again from where
                    // it is; a file's are all there already.
                    let now = shared.clock.now().max(0.0);
                    if info.seekable && decoders.subtitles.is_some() {
                        seek(&mut inputs, &mut reading, &shared, &mut decoders, now, true);
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
        if let Some(mixed) = &mut reading.mixed {
            let serial = shared.serial.load(Ordering::Relaxed);
            mixed.read(serial, &mut decoders, &shared);
            if mixed.read_all() {
                read_all = true;
                shared.state().read_all = true;
            }
            continue;
        }
        // Sound read apart from the picture keeps up with it.
        let sound_apart = decoders.sound_apart();
        let from_sound = sound_apart
            && !inputs.sound_read
            && (inputs.main_read || {
                let end = |queue: Option<&(usize, Arc<PacketQueue>, JoinHandle<()>)>| {
                    queue
                        .and_then(|(_, queue, _)| queue.end())
                        .unwrap_or(f64::MIN)
                };
                end(decoders.audio.as_ref()) <= end(decoders.video.as_ref())
            });
        let (input, first, input_start) = match (&mut inputs.sound, from_sound) {
            (Some((sound, sound_start)), true) => (sound, SOUND_TRACKS, *sound_start),
            _ => (&mut inputs.main, 0, start),
        };
        match packet.read(input) {
            Ok(()) => {
                let serial = shared.serial.load(Ordering::Relaxed);
                let index = first + packet.stream();
                let queue = decoders
                    .video
                    .iter()
                    .chain(&decoders.audio)
                    .find(|(stream, _, _)| *stream == index)
                    .map(|(_, queue, _)| queue);
                if let Some(queue) = queue {
                    let span = reading.span();
                    let playing = decoders.playing();
                    let past = span.to.is_finite()
                        && packet_order(input, &packet, input_start).is_some_and(
                            |(seconds, decoding)| {
                                reading.passes(index, seconds, decoding, &playing)
                            },
                        );
                    if !past {
                        if let Some(end) = packet_end(input, &packet, input_start) {
                            queue.reached(end.min(span.to) + span.shift);
                        }
                        let taken = std::mem::replace(&mut packet, ffmpeg::Packet::empty());
                        queue.put(Item::Packet(serial, span, taken));
                    } else if reading.finished(&playing) {
                        // On to the next part, or that was the last.
                        if reading.has_next() {
                            let next = reading.current + 1;
                            let from = reading.spans[next].from;
                            reading.go_to(next, &mut inputs, from);
                        } else {
                            for queue in decoders.queues() {
                                queue.put(Item::End(serial));
                            }
                            read_all = true;
                            shared.state().read_all = true;
                        }
                    }
                } else if first == 0 {
                    feed_subtitles(&mut decoders, &shared, &packet, start);
                }
            }
            Err(ffmpeg::Error::Exit) => break,
            Err(ffmpeg::Error::InvalidData) => {}
            Err(error) => {
                if shared.stopping() {
                    break;
                }
                if error != ffmpeg::Error::Eof {
                    eprintln!("plaa: reading {}: {error}", source.name());
                }
                // The file ended before a part did: on to the next.
                if !from_sound && reading.has_next() {
                    let next = reading.current + 1;
                    let from = reading.spans[next].from;
                    reading.go_to(next, &mut inputs, from);
                    continue;
                }
                // That input is done: its streams' decoders hear so.
                let serial = shared.serial.load(Ordering::Relaxed);
                let (from, to) = if from_sound {
                    inputs.sound_read = true;
                    (SOUND_TRACKS, usize::MAX)
                } else {
                    inputs.main_read = true;
                    (0, SOUND_TRACKS)
                };
                for (index, queue, _) in decoders.video.iter().chain(&decoders.audio) {
                    if (from..to).contains(index) {
                        queue.put(Item::End(serial));
                    }
                }
                read_all = inputs.main_read && (inputs.sound_read || !sound_apart);
                shared.state().read_all = read_all;
            }
        }
    }
    decoders.stop();
    drop(output);
}

/// Decodes the picture of stream `index`, and notes the stream that
/// shows: none when it will not decode.
fn start_video(
    index: Option<usize>,
    input: &ffmpeg::format::context::Input,
    start: f64,
    hardware: bool,
    shared: &Arc<Shared>,
    decoders: &mut Decoders,
    events: &dyn Fn(Event),
) {
    let mut chosen = None;
    if let Some(index) = index {
        match decode::spawn_video(input, index, start, hardware, shared.clone()) {
            Ok(((queue, thread), device)) => {
                decoders.video = Some((index, queue, thread));
                chosen = Some(index);
                events(Event::Decoding(device));
            }
            Err(error) => eprintln!("plaa: video stream {index}: {error}"),
        }
    }
    let mut state = shared.state();
    state.video = chosen;
    state.has_video = chosen.is_some();
}

/// Decodes audio track `index` into `output`, and notes the track that
/// plays: none when there is no output or the track will not decode.
fn start_audio(
    index: Option<usize>,
    inputs: &Inputs,
    output: Option<&audio::Output>,
    shared: &Arc<Shared>,
    decoders: &mut Decoders,
) {
    let mut chosen = None;
    if let (Some(index), Some(output)) = (index, output) {
        let (input, local, start) = inputs.locate(index);
        match decode::spawn_audio(input, local, start, output.shared.clone(), shared.clone()) {
            Ok((queue, thread)) => {
                decoders.audio = Some((index, queue, thread));
                chosen = Some(index);
            }
            Err(error) => eprintln!("plaa: audio stream {index}: {error}"),
        }
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
    inputs: &mut Inputs,
    reading: &mut Reading,
    shared: &Shared,
    decoders: &mut Decoders,
) -> bool {
    let video_stream = decoders.video.as_ref().map(|(index, ..)| *index);
    let wanted = *shared
        .scrub_to
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    let Some(wanted) = wanted else {
        return false;
    };
    let serial = shared.serial.load(Ordering::Relaxed);
    let Some(current) = scrub else {
        seek(inputs, reading, shared, decoders, wanted, true);
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
        && decodes_on(
            &inputs.main,
            reading,
            shared,
            video_stream,
            inputs.start,
            current,
            wanted,
        )
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
    seek(inputs, reading, shared, decoders, wanted, true);
    current.target = wanted;
    current.serial = shared.serial.load(Ordering::Relaxed);
    current.since = Instant::now();
    true
}

/// Whether decoding on reaches `wanted` sooner than a seek would: no
/// keyframe lies between it and where the decoder is, within the part of
/// the file being read.
fn decodes_on(
    input: &ffmpeg::format::context::Input,
    reading: &Reading,
    shared: &Shared,
    video_stream: Option<usize>,
    start: f64,
    scrub: &Scrub,
    wanted: f64,
) -> bool {
    // Sound alone seeks quickly, and an edit mixing files seeks each lane.
    let Some(stream) = video_stream.filter(|_| reading.mixed.is_none()) else {
        return false;
    };
    let (part, wanted) = reading.locate(wanted);
    let (target_part, target) = reading.locate(scrub.target);
    if part != target_part || part != reading.current {
        return false;
    }
    let span = reading.spans[part];
    let keyframe = |seconds| crate::ffi::keyframe_before(input, stream, seconds, start);
    let (of, decoded) = *shared
        .decoded
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    // Before its first picture, the decoder starts from the target's
    // keyframe.
    let reached = if of == scrub.serial {
        if !span.holds(decoded) {
            return false;
        }
        decoded - span.shift
    } else {
        keyframe(target).unwrap_or(target)
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
    // A seek asked for and not yet made, as playing again from the end
    // makes, leaves what was read there until it is: not the end.
    if !state.read_all || state.info.is_none() || shared.seeks.load(Ordering::Relaxed) > 0 {
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
/// start of the file, or of the edit playing.
fn seek(
    inputs: &mut Inputs,
    reading: &mut Reading,
    shared: &Shared,
    decoders: &mut Decoders,
    position: f64,
    exact: bool,
) {
    let serial = shared.serial.fetch_add(1, Ordering::Relaxed) + 1;
    *shared.target.lock().unwrap_or_else(PoisonError::into_inner) =
        exact.then_some((serial, position));
    if reading.mixed.is_none() {
        let (part, source) = reading.locate(position);
        reading.go_to(part, inputs, source);
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
    shared.clock.wait();
    shared.state().ended = false;
    // Each lane moves to its own part, and its decoder to that part's
    // file.
    if let Some(mixed) = &mut reading.mixed {
        mixed.seek(position, serial, decoders);
    }
}

/// The audio decoder's stream number when it decodes another file's sound,
/// as the file edited has none.
const OTHER_SOUND: usize = usize::MAX;

/// Opens the sound output, or a silent one when none opens.
fn open_output(shared: &Shared, options: &Options, events: &dyn Fn(Event)) -> audio::Output {
    audio::Output::open(
        shared.clock.clone(),
        options.volume,
        options.muted,
        options.audio_device.as_deref(),
    )
    .unwrap_or_else(|error| {
        events(Event::NoSound(error.to_string()));
        audio::Output::silent(shared.clock.clone(), options.volume, options.muted)
    })
}

/// Decodes the sound of file `number` of an edit, at `path`, where the
/// file edited has none of its own.
fn start_other_audio(
    path: &Path,
    number: usize,
    output: Option<&audio::Output>,
    shared: &Arc<Shared>,
    decoders: &mut Decoders,
) {
    let Some(output) = output else {
        return;
    };
    let input = match ffmpeg::format::input(path) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("plaa: opening {}: {error}", path.display());
            return;
        }
    };
    let info = MediaInfo::read(&input);
    let Some(index) = info.audio.first().map(|track| track.track.index) else {
        return;
    };
    let start = decode::start_seconds(&input);
    match decode::spawn_audio(&input, index, start, output.shared.clone(), shared.clone()) {
        Ok((queue, thread)) => {
            *queue.decoding() = number;
            decoders.audio = Some((OTHER_SOUND, queue, thread));
            shared.state().has_audio = true;
        }
        Err(error) => eprintln!("plaa: sound of {}: {error}", path.display()),
    }
}

/// In an edit that mixes files, decodes another file's sound again where
/// the file edited has none, once the decoders started over.
fn keep_other_sound(
    reading: &Reading,
    output: Option<&audio::Output>,
    shared: &Arc<Shared>,
    decoders: &mut Decoders,
) {
    if decoders.audio.is_none()
        && let Some((path, number)) = reading.mixed.as_ref().and_then(Mixed::first_sound)
    {
        start_other_audio(&path, number, output, shared, decoders);
    }
}

/// Hands `packet` to the subtitles showing, when it is theirs: the file
/// edited's, whose timestamps start at `start`.
pub(crate) fn feed_subtitles(
    decoders: &mut Decoders,
    shared: &Shared,
    packet: &ffmpeg::Packet,
    start: f64,
) {
    let Some((stream, sink)) = &mut decoders.subtitles else {
        return;
    };
    if *stream != packet.stream() {
        return;
    }
    match sink {
        SubtitleSink::Decoder(decoder) => {
            for decoded in decoder.decode(packet) {
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
                crate::info::seconds(value, *time_base).map(|seconds| (seconds * 1000.0) as i64)
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

/// Where `packet` comes in the file's time, in seconds from the start:
/// by its decoding time where it has one, which is in order, else by its
/// presentation time. Says which.
pub(crate) fn packet_order(
    input: &ffmpeg::format::context::Input,
    packet: &ffmpeg::Packet,
    start: f64,
) -> Option<(f64, bool)> {
    let time_base = input.stream(packet.stream())?.time_base();
    let (time, decoding) = match packet.dts() {
        Some(dts) => (dts, true),
        None => (packet.pts()?, false),
    };
    Some((crate::info::seconds(time, time_base)? - start, decoding))
}

/// The media time `packet` lasts until, in seconds from the start.
pub(crate) fn packet_end(
    input: &ffmpeg::format::context::Input,
    packet: &ffmpeg::Packet,
    start: f64,
) -> Option<f64> {
    let time_base = input.stream(packet.stream())?.time_base();
    let begins = packet.pts().or(packet.dts())?;
    let seconds = crate::info::seconds(begins + packet.duration().max(0), time_base)?;
    Some(seconds - start)
}
