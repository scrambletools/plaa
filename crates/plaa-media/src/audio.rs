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

/// What the decoder and the device's callback share.
pub struct Shared {
    queue: Mutex<Queue>,
    room: Condvar,
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

    /// Fills `out` from the queue and sets the clock to the media time of
    /// its first frame, heard after `latency`.
    fn fill<T: cpal::SizedSample + cpal::FromSample<f32>>(&self, out: &mut [T], latency: Duration) {
        let heard_at = Instant::now() + latency;
        let channels = usize::from(self.channels.max(1));
        let gain = self.gain();
        let silence = T::from_sample(0.0f32);
        if self.clock.still() {
            out.fill(silence);
            return;
        }
        let mut queue = self.queue();
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
        self.room.notify_all();
        match first {
            Some(position) => self.clock.anchor(position, heard_at),
            None if finished => {}
            None => self.clock.stall(),
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
    _stream: cpal::Stream,
    pub shared: Arc<Shared>,
}

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
            _stream: stream,
            shared,
        })
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
    fn volume_is_cubed() {
        let shared = shared();
        shared.set_volume(0.5);
        assert!((shared.gain() - 0.125).abs() < 1e-6);
        shared.set_muted(true);
        assert_eq!(shared.gain(), 0.0);
    }
}
