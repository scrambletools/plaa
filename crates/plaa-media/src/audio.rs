//! Sound out through the system's default output device, from a queue the
//! audio decoder fills. The device's callback sets the playback clock to
//! the media time of the sound it is about to play.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::clock::Clock;

/// How much decoded sound is kept ahead of the device.
pub const AHEAD: Duration = Duration::from_millis(400);

/// How many frames of the sound played are kept for a visualizer to look
/// at, a third of a second at 48 kHz: enough to reach past the device's
/// latency to the sound heard.
const RECENT: usize = 16_384;

/// Sound played longer ago than this is not heard any more, so a paused
/// visualizer comes to rest.
const STALE: Duration = Duration::from_millis(150);

#[derive(Debug)]
pub enum OutputError {
    NoDevice,
    Device(String),
}

impl std::fmt::Display for OutputError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoDevice => formatter.write_str("no audio output device"),
            Self::Device(error) => write!(formatter, "audio output: {error}"),
        }
    }
}

impl std::error::Error for OutputError {}

/// Decoded sound waiting for the device, with the media time it starts at.
struct Chunk {
    serial: u64,
    /// Media seconds of the chunk's first frame.
    position: f64,
    /// Media seconds each frame advances, the speed over the rate.
    step: f64,
    /// Interleaved samples at the device's rate and channels.
    samples: Vec<f32>,
    /// Samples already played.
    played: usize,
}

#[derive(Default)]
struct Queue {
    chunks: VecDeque<Chunk>,
    /// Frames waiting, over all chunks.
    frames: usize,
    /// Only chunks of this serial play; older ones are dropped.
    serial: u64,
    /// Set when the decoder has nothing more until the next seek.
    finished: bool,
}

/// The latest sound handed to the device, mixed to one channel before
/// the volume, and when the newest of it is heard.
#[derive(Default)]
struct Recent {
    frames: VecDeque<f32>,
    heard_by: Option<Instant>,
}

impl Recent {
    fn push(&mut self, frame: f32) {
        if self.frames.len() >= RECENT {
            self.frames.pop_front();
        }
        self.frames.push_back(frame);
    }
}

/// Takes a copy of what an output plays, from its device's thread: two
/// channels interleaved, after the volume, and their rate.
pub type SoundCopy = Arc<dyn Fn(&[f32], u32) + Send + Sync>;

/// Where the copy goes, and the room it is made in, kept from one
/// callback to the next.
#[derive(Default)]
struct Copying {
    to: Option<SoundCopy>,
    frames: Vec<f32>,
}

/// What the decoder and the device's callback share.
pub struct Shared {
    queue: Mutex<Queue>,
    room: Condvar,
    recent: Mutex<Recent>,
    copying: Mutex<Copying>,
    clock: Arc<Clock>,
    /// The volume as the slider has it, 0 to 1, as f32 bits.
    volume: AtomicU32,
    muted: AtomicBool,
    pub rate: u32,
    pub channels: u16,
}

impl Shared {
    fn queue(&self) -> MutexGuard<'_, Queue> {
        self.queue.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Adds sound starting at media time `position` to play once the
    /// queue has room, waiting while it is full. Returns `false` when the
    /// sound was dropped because a seek made it stale, or `stop` was set.
    pub fn push(
        &self,
        serial: u64,
        position: f64,
        step: f64,
        samples: Vec<f32>,
        stop: &AtomicBool,
    ) -> bool {
        let frames = samples.len() / usize::from(self.channels.max(1));
        let limit = (AHEAD.as_secs_f64() * f64::from(self.rate)) as usize;
        let mut queue = self.queue();
        loop {
            if queue.serial != serial || stop.load(Ordering::Relaxed) {
                return false;
            }
            if queue.frames < limit {
                break;
            }
            queue = self
                .room
                .wait_timeout(queue, Duration::from_millis(20))
                .unwrap_or_else(PoisonError::into_inner)
                .0;
        }
        queue.frames += frames;
        queue.finished = false;
        queue.chunks.push_back(Chunk {
            serial,
            position,
            step,
            samples,
            played: 0,
        });
        true
    }

    /// The frames waiting for the device.
    pub fn queued(&self) -> usize {
        self.queue().frames
    }

    /// Drops all sound and plays only `serial`'s from now on.
    pub fn flush(&self, serial: u64) {
        let mut queue = self.queue();
        queue.chunks.clear();
        queue.frames = 0;
        queue.serial = serial;
        queue.finished = false;
        self.room.notify_all();
    }

    /// Says no more sound comes until the next seek.
    pub fn finish(&self, serial: u64) {
        let mut queue = self.queue();
        if queue.serial == serial {
            queue.finished = true;
        }
    }

    /// Drops `serial`'s sound before media time `position`, as the
    /// timeline is dragged on from where it was.
    pub fn discard_before(&self, serial: u64, position: f64) {
        let channels = usize::from(self.channels.max(1));
        let mut queue = self.queue();
        while let Some(chunk) = queue.chunks.front_mut() {
            let left = (chunk.samples.len() - chunk.played) / channels;
            let at = chunk.position + (chunk.played / channels) as f64 * chunk.step;
            let skip = if chunk.serial != serial {
                left
            } else {
                // Less a hair, so a frame right at `position` stays.
                (((position - at) / chunk.step - 1e-6).ceil().max(0.0) as usize).min(left)
            };
            chunk.played += skip * channels;
            let done = skip == left;
            queue.frames = queue.frames.saturating_sub(skip);
            if !done {
                break;
            }
            queue.chunks.pop_front();
        }
        self.room.notify_all();
    }

    /// Whether sound of `serial` has come, or none will.
    pub fn reached(&self, serial: u64) -> bool {
        let queue = self.queue();
        queue.serial == serial && (queue.frames > 0 || queue.finished)
    }

    /// Whether everything decoded has been played and nothing more comes.
    pub fn drained(&self) -> bool {
        let queue = self.queue();
        queue.finished && queue.frames == 0
    }

    /// Wakes a decoder waiting for room, such as on close.
    pub fn wake(&self) {
        self.room.notify_all();
    }

    /// The volume as the slider has it, and whether it is muted.
    pub fn volume(&self) -> (f32, bool) {
        (
            f32::from_bits(self.volume.load(Ordering::Relaxed)),
            self.muted.load(Ordering::Relaxed),
        )
    }

    pub fn set_volume(&self, volume: f32) {
        self.volume
            .store(volume.clamp(0.0, 1.0).to_bits(), Ordering::Relaxed);
    }

    pub fn set_muted(&self, muted: bool) {
        self.muted.store(muted, Ordering::Relaxed);
    }

    /// The gain for the slider's volume: its cube, which sounds even
    /// across the slider.
    fn gain(&self) -> f32 {
        if self.muted.load(Ordering::Relaxed) {
            return 0.0;
        }
        let volume = f32::from_bits(self.volume.load(Ordering::Relaxed));
        volume * volume * volume
    }

    /// Hands a copy of what plays to `to`, or to nothing.
    pub fn set_copy(&self, to: Option<SoundCopy>) {
        self.copying
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .to = to;
    }

    /// The `count` frames heard last, mixed to one channel before the
    /// volume, the newest last and silence before the first; None once
    /// nothing has been heard for a while, as when paused.
    pub fn heard(&self, count: usize) -> Option<Vec<f32>> {
        self.heard_at(count, Instant::now())
    }

    /// The frames heard last as at `now`, as [`Self::heard`] has them.
    fn heard_at(&self, count: usize, now: Instant) -> Option<Vec<f32>> {
        let recent = self.recent.lock().unwrap_or_else(PoisonError::into_inner);
        let heard_by = recent.heard_by?;
        if now > heard_by + STALE {
            return None;
        }
        // The device plays what it was handed after its latency: the
        // frames still to be heard are left out.
        let coming = heard_by.saturating_duration_since(now).as_secs_f64() * f64::from(self.rate);
        let end = recent.frames.len().saturating_sub(coming.ceil() as usize);
        let start = end.saturating_sub(count);
        let mut frames = vec![0.0; count - (end - start)];
        frames.extend(recent.frames.range(start..end));
        Some(frames)
    }

    /// Fills `out` from the queue and sets the clock to the media time of
    /// its first frame, heard after `latency`.
    fn fill<T: cpal::SizedSample + cpal::FromSample<f32>>(&self, out: &mut [T], latency: Duration) {
        let heard_at = Instant::now() + latency;
        let channels = usize::from(self.channels.max(1));
        let gain = self.gain();
        let silence = T::from_sample(0.0f32);
        let mut copying = self.copying.lock().unwrap_or_else(PoisonError::into_inner);
        let copying = &mut *copying;
        copying.frames.clear();
        let copy = copying.to.clone();
        if self.clock.still() {
            out.fill(silence);
            if let Some(copy) = copy {
                copying.frames.resize(out.len() / channels * 2, 0.0);
                copy(&copying.frames, self.rate);
            }
            return;
        }
        let mut queue = self.queue();
        let mut recent = self.recent.lock().unwrap_or_else(PoisonError::into_inner);
        let serial = queue.serial;
        let mut first = None;
        let mut written = 0;
        while written < out.len() {
            let Some(chunk) = queue.chunks.front_mut() else {
                break;
            };
            if chunk.serial != serial {
                let left = (chunk.samples.len() - chunk.played) / channels;
                queue.frames = queue.frames.saturating_sub(left);
                queue.chunks.pop_front();
                continue;
            }
            if first.is_none() {
                first = Some(chunk.position + (chunk.played / channels) as f64 * chunk.step);
            }
            let take = (chunk.samples.len() - chunk.played).min(out.len() - written);
            for (target, sample) in out[written..written + take]
                .iter_mut()
                .zip(&chunk.samples[chunk.played..chunk.played + take])
            {
                *target = T::from_sample(sample * gain);
            }
            for frame in chunk.samples[chunk.played..chunk.played + take].chunks_exact(channels) {
                recent.push(frame.iter().sum::<f32>() / channels as f32);
                if copy.is_some() {
                    let left = frame[0] * gain;
                    copying.frames.push(left);
                    copying
                        .frames
                        .push(frame.get(1).map_or(left, |right| right * gain));
                }
            }
            chunk.played += take;
            written += take;
            let done = chunk.played >= chunk.samples.len();
            queue.frames = queue.frames.saturating_sub(take / channels);
            if done {
                queue.chunks.pop_front();
            }
        }
        // Once the sound has ended for good, pictures go on by the
        // system's clock; before that, they wait for it.
        let finished = queue.finished && queue.chunks.is_empty();
        drop(queue);
        out[written..].fill(silence);
        for _ in (written..out.len()).step_by(channels) {
            recent.push(0.0);
        }
        if let Some(copy) = copy {
            copying.frames.resize(out.len() / channels * 2, 0.0);
            copy(&copying.frames, self.rate);
        }
        recent.heard_by = Some(
            heard_at
                + Duration::from_secs_f64(
                    (out.len() / channels) as f64 / f64::from(self.rate.max(1)),
                ),
        );
        drop(recent);
        self.room.notify_all();
        match first {
            Some(position) => self.clock.anchor(position, heard_at),
            None if finished => {}
            None => self.clock.ran_out(),
        }
    }
}

/// A sound output device: its id, which stays the same from one run to
/// the next, and its name for people.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputDevice {
    pub id: String,
    pub name: String,
}

/// The system's sound output devices.
pub fn output_devices() -> Vec<OutputDevice> {
    let host = cpal::default_host();
    let Ok(devices) = host.output_devices() else {
        return Vec::new();
    };
    devices
        .filter_map(|device| {
            Some(OutputDevice {
                id: device.id().ok()?.to_string(),
                name: device.description().ok()?.name().to_owned(),
            })
        })
        .collect()
}

/// An open output stream. Dropping it stops the sound.
pub struct Output {
    _sink: Sink,
    pub shared: Arc<Shared>,
}

/// Where the sound goes: a device, or nowhere.
enum Sink {
    Device {
        _stream: cpal::Stream,
    },
    /// A thread that takes the sound as a device would, in time, without
    /// playing it: for when no device opens, so playback goes on silently.
    Silent {
        stop: Arc<AtomicBool>,
        thread: Option<std::thread::JoinHandle<()>>,
    },
}

impl Drop for Sink {
    fn drop(&mut self) {
        if let Sink::Silent { stop, thread } = self {
            stop.store(true, Ordering::Relaxed);
            if let Some(thread) = thread.take() {
                let _ = thread.join();
            }
        }
    }
}

/// How much sound the silent output takes at a time.
const SILENT_PERIOD: Duration = Duration::from_millis(10);

impl Output {
    /// Opens the default output device at its own rate and channels.
    /// Opens output device `device`, by its id, or the system's default
    /// when it is none or gone.
    pub fn open(
        clock: Arc<Clock>,
        volume: f32,
        muted: bool,
        device: Option<&str>,
    ) -> Result<Self, OutputError> {
        let host = cpal::default_host();
        let chosen = device.and_then(|wanted| {
            host.output_devices()
                .ok()?
                .find(|candidate| candidate.id().is_ok_and(|id| id.to_string() == wanted))
        });
        let device = chosen
            .or_else(|| host.default_output_device())
            .ok_or(OutputError::NoDevice)?;
        let supported = device
            .default_output_config()
            .map_err(|error| OutputError::Device(error.to_string()))?;
        #[cfg_attr(not(target_os = "linux"), expect(unused_mut))]
        let mut config = supported.config();
        // The sound server holds two seconds of sound unless asked for
        // less, so each start and seek would be heard that late: periods
        // of 25 ms, two of them buffered.
        #[cfg(target_os = "linux")]
        if host.id() == cpal::HostId::PulseAudio {
            config.buffer_size = cpal::BufferSize::Fixed((config.sample_rate / 40).max(64));
        }
        let shared = Arc::new(Shared {
            queue: Mutex::new(Queue::default()),
            room: Condvar::new(),
            recent: Mutex::new(Recent::default()),
            copying: Mutex::new(Copying::default()),
            clock,
            volume: AtomicU32::new(volume.clamp(0.0, 1.0).to_bits()),
            muted: AtomicBool::new(muted),
            rate: config.sample_rate,
            channels: config.channels,
        });
        let stream = match supported.sample_format() {
            cpal::SampleFormat::F32 => build::<f32>(&device, config, &shared),
            cpal::SampleFormat::I16 => build::<i16>(&device, config, &shared),
            cpal::SampleFormat::U16 => build::<u16>(&device, config, &shared),
            cpal::SampleFormat::I32 => build::<i32>(&device, config, &shared),
            cpal::SampleFormat::F64 => build::<f64>(&device, config, &shared),
            other => {
                return Err(OutputError::Device(format!(
                    "unsupported sample format {other}"
                )));
            }
        }?;
        stream
            .play()
            .map_err(|error| OutputError::Device(error.to_string()))?;
        Ok(Self {
            _sink: Sink::Device { _stream: stream },
            shared,
        })
    }

    /// An output that plays nothing but takes the sound in time, so the
    /// clock and the end of playback go as with a device.
    pub fn silent(clock: Arc<Clock>, volume: f32, muted: bool) -> Self {
        let shared = Arc::new(Shared {
            queue: Mutex::new(Queue::default()),
            room: Condvar::new(),
            recent: Mutex::new(Recent::default()),
            copying: Mutex::new(Copying::default()),
            clock,
            volume: AtomicU32::new(volume.clamp(0.0, 1.0).to_bits()),
            muted: AtomicBool::new(muted),
            rate: 48_000,
            channels: 2,
        });
        let stop = Arc::new(AtomicBool::new(false));
        let thread = {
            let (shared, stop) = (shared.clone(), stop.clone());
            std::thread::Builder::new()
                .name("plaa-silent".into())
                .spawn(move || {
                    let frames = (shared.rate as f64 * SILENT_PERIOD.as_secs_f64()) as usize;
                    let mut out = vec![0.0f32; frames * usize::from(shared.channels)];
                    let mut next = Instant::now();
                    while !stop.load(Ordering::Relaxed) {
                        shared.fill(&mut out, Duration::ZERO);
                        next += SILENT_PERIOD;
                        match next.checked_duration_since(Instant::now()) {
                            Some(wait) => std::thread::sleep(wait),
                            // Fallen behind: from now on.
                            None => next = Instant::now(),
                        }
                    }
                })
                .ok()
        };
        Self {
            _sink: Sink::Silent { stop, thread },
            shared,
        }
    }
}

fn build<T: cpal::SizedSample + cpal::FromSample<f32>>(
    device: &cpal::Device,
    config: cpal::StreamConfig,
    shared: &Arc<Shared>,
) -> Result<cpal::Stream, OutputError> {
    let callback = shared.clone();
    device
        .build_output_stream::<T, _, _>(
            config,
            move |out: &mut [T], info: &cpal::OutputCallbackInfo| {
                let timestamp = info.timestamp();
                let latency = timestamp
                    .playback
                    .checked_duration_since(timestamp.callback)
                    .unwrap_or_default();
                callback.fill(out, latency);
            },
            |error| eprintln!("plaa: audio output: {error}"),
            None,
        )
        .map_err(|error| OutputError::Device(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shared() -> Shared {
        let clock = Arc::new(Clock::new());
        clock.set_paused(false);
        Shared {
            queue: Mutex::new(Queue::default()),
            room: Condvar::new(),
            recent: Mutex::new(Recent::default()),
            copying: Mutex::new(Copying::default()),
            clock,
            volume: AtomicU32::new(1.0f32.to_bits()),
            muted: AtomicBool::new(false),
            rate: 1000,
            channels: 2,
        }
    }

    #[test]
    fn fills_from_the_queue_and_sets_the_clock() {
        let shared = shared();
        let stop = AtomicBool::new(false);
        assert!(shared.push(0, 2.0, 0.001, vec![0.5; 20], &stop));
        let mut out = [0.0f32; 8];
        shared.fill(&mut out, Duration::ZERO);
        assert_eq!(out, [0.5; 8]);
        let mut out = [0.0f32; 16];
        shared.fill(&mut out, Duration::ZERO);
        assert_eq!(&out[..12], &[0.5; 12]);
        assert_eq!(&out[12..], &[0.0; 4]);
        // Four frames were played before the second fill.
        assert!((shared.clock.now() - 2.004).abs() < 0.01);
    }

    #[test]
    fn dragging_on_drops_the_sound_passed() {
        let shared = shared();
        let stop = AtomicBool::new(false);
        // Two chunks of ten frames from 1.0 s, a frame a millisecond.
        let first: Vec<f32> = (0..20).map(|sample| sample as f32).collect();
        assert!(shared.push(0, 1.0, 0.001, first, &stop));
        assert!(shared.push(0, 1.01, 0.001, vec![0.5; 20], &stop));
        shared.discard_before(0, 1.004);
        assert_eq!(shared.queue().frames, 16);
        let mut out = [0.0f32; 2];
        shared.fill(&mut out, Duration::ZERO);
        // The fifth frame plays first.
        assert_eq!(out, [8.0, 9.0]);
        assert!((shared.clock.now() - 1.004).abs() < 0.01);
        shared.discard_before(0, 2.0);
        assert_eq!(shared.queue().frames, 0);
        assert!(!shared.reached(1));
    }

    #[test]
    fn held_clock_keeps_the_sound() {
        let shared = shared();
        let stop = AtomicBool::new(false);
        assert!(shared.push(0, 0.0, 0.001, vec![0.5; 20], &stop));
        shared.clock.hold(true);
        let mut out = [1.0f32; 4];
        shared.fill(&mut out, Duration::ZERO);
        assert_eq!(out, [0.0; 4]);
        assert_eq!(shared.queue().frames, 10);
        assert!(shared.reached(0));
    }

    #[test]
    fn stale_sound_is_dropped() {
        let shared = shared();
        let stop = AtomicBool::new(false);
        assert!(shared.push(0, 0.0, 0.001, vec![0.5; 20], &stop));
        shared.flush(1);
        assert!(!shared.push(0, 0.0, 0.001, vec![0.5; 20], &stop));
        let mut out = [1.0f32; 4];
        shared.fill(&mut out, Duration::ZERO);
        assert_eq!(out, [0.0; 4]);
        assert!(!shared.clock.running(), "no sound stalls the clock");
    }

    #[test]
    fn the_sound_heard_is_kept_for_a_visualizer() {
        let shared = shared();
        let stop = AtomicBool::new(false);
        assert!(shared.heard(4).is_none(), "nothing heard yet");
        shared.set_volume(0.5);
        // Left and right differ; the mix is their middle, at full volume.
        let samples: Vec<f32> = (0..10)
            .flat_map(|frame| [frame as f32, frame as f32 + 1.0])
            .collect();
        assert!(shared.push(0, 0.0, 0.001, samples, &stop));
        // Ten frames of sound and two of silence, heard after the device's
        // 200 ms: the last of them 212 ms after the fill. Looked at as of
        // given moments, so a slow machine's sleeps do not matter.
        let latency = Duration::from_millis(200);
        let before = Instant::now();
        let mut out = [0.0f32; 24];
        shared.fill(&mut out, latency);
        let after = Instant::now();
        let all_heard = after + latency + Duration::from_millis(13);
        let none_heard = shared.heard_at(4, before).unwrap();
        assert!(none_heard.iter().all(|&frame| frame == 0.0));
        let mut expected: Vec<f32> = (0..10).map(|frame| frame as f32 + 0.5).collect();
        expected.extend([0.0, 0.0]);
        assert_eq!(shared.heard_at(12, all_heard).unwrap(), expected);
        let long_after = all_heard + STALE + Duration::from_secs(1);
        assert!(shared.heard_at(4, long_after).is_none(), "long since heard");
    }

    #[test]
    fn a_copy_of_what_plays_goes_out_in_stereo() {
        let shared = shared();
        let stop = AtomicBool::new(false);
        let copied = Arc::new(Mutex::new(Vec::new()));
        let into = copied.clone();
        shared.set_copy(Some(Arc::new(move |frames: &[f32], rate| {
            assert_eq!(rate, 1000);
            into.lock().unwrap().extend_from_slice(frames);
        })));
        shared.set_volume(0.5);
        assert!(shared.push(0, 0.0, 0.001, vec![0.8, -0.8, 0.4, -0.4], &stop));
        let mut out = [0.0f32; 8];
        shared.fill(&mut out, Duration::ZERO);
        // After the volume, its cube, and silence where the sound ran out.
        let expected = [0.1, -0.1, 0.05, -0.05, 0.0, 0.0, 0.0, 0.0];
        let got = copied.lock().unwrap().clone();
        assert_eq!(got.len(), expected.len());
        for (got, expected) in got.iter().zip(expected) {
            assert!((got - expected).abs() < 1e-6, "{got} for {expected}");
        }
        // Held still, as paused: silence still goes out, to keep it fed.
        shared.clock.hold(true);
        shared.fill(&mut out, Duration::ZERO);
        assert_eq!(copied.lock().unwrap().len(), 16);
        shared.set_copy(None);
        shared.fill(&mut out, Duration::ZERO);
        assert_eq!(copied.lock().unwrap().len(), 16);
    }

    #[test]
    fn volume_is_cubed() {
        let shared = shared();
        shared.set_volume(0.5);
        assert!((shared.gain() - 0.125).abs() < 1e-6);
        shared.set_muted(true);
        assert_eq!(shared.gain(), 0.0);
    }
}
