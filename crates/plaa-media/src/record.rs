//! Recording sound from an input device, such as a microphone, to a
//! file. The device's callback notes how loud each channel is, for a
//! meter, and while recording hands its samples to a thread that encodes
//! them as exports encode sound alone.

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ffmpeg_next as ffmpeg;

use crate::export::{self, ExportError, Output, Sound};
use crate::ffi;

/// A sound input device: its id, which stays the same from one run to
/// the next, and its name for people.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDevice {
    pub id: String,
    pub name: String,
}

/// How the id of what an output plays ends, as PulseAudio names its
/// monitors; on Windows and macOS, the output's id with this added.
const PLAYS_BACK: &str = ".monitor";

/// The system's sound input devices, and what each output plays where
/// the system offers it.
pub fn input_devices() -> Vec<InputDevice> {
    crate::init();
    fn listed(devices: impl Iterator<Item = cpal::Device>, suffix: &str) -> Vec<InputDevice> {
        devices
            .filter_map(|device| {
                Some(InputDevice {
                    id: format!("{}{suffix}", device.id().ok()?),
                    name: device.description().ok()?.name().to_owned(),
                })
            })
            .collect()
    }
    let host = cpal::default_host();
    #[cfg_attr(not(any(windows, target_os = "macos")), expect(unused_mut))]
    let mut devices = listed(host.input_devices().into_iter().flatten(), "");
    // Windows and macOS record what an output plays from the output
    // itself, macOS through a tap of it.
    #[cfg(any(windows, target_os = "macos"))]
    devices.extend(listed(
        host.output_devices().into_iter().flatten(),
        PLAYS_BACK,
    ));
    devices
}

/// What a recording says once it ends: the file it made, or why it
/// could not.
#[derive(Debug, Clone, PartialEq)]
pub enum RecordEvent {
    Finished(Result<PathBuf, String>),
}

/// How long opening a device may take.
const OPENING: Duration = Duration::from_secs(5);
/// How long the encoding thread waits for sound before looking at its
/// flags.
const WAIT: Duration = Duration::from_millis(100);
/// How far behind the clock the sound may fall before it is timed by the
/// clock, in fractions of a second: further than an input's own latency.
const BEHIND: i64 = 5;

/// Where `frames` frames of sound taken now begin, in frames at `rate`
/// from `start`: right after those before, `counted`, unless the input
/// sent nothing for a while, as WASAPI sends nothing while the computer
/// plays nothing; then where the clock says, and the filter graph fills
/// the gap with silence.
pub(crate) fn sound_time(counted: i64, frames: usize, rate: u32, start: Instant) -> i64 {
    let by_clock = (start.elapsed().as_secs_f64() * f64::from(rate)) as i64 - frames as i64;
    if by_clock - counted > i64::from(rate) / BEHIND {
        by_clock
    } else {
        counted
    }
}

/// A frame of interleaved `samples` of `channels` channels at `rate`, at
/// `pts` in frames; none when there are none.
pub(crate) fn sound_frame(
    samples: &[f32],
    (rate, channels): (u32, u16),
    pts: i64,
) -> Option<ffmpeg::frame::Audio> {
    let frames = samples.len() / usize::from(channels.max(1));
    if frames == 0 {
        return None;
    }
    let mut frame = ffmpeg::frame::Audio::new(
        ffmpeg::format::Sample::F32(ffmpeg::format::sample::Type::Packed),
        frames,
        ffmpeg::ChannelLayout::default(i32::from(channels)),
    );
    frame.set_rate(rate);
    for (target, sample) in frame
        .data_mut(0)
        .as_chunks_mut::<4>()
        .0
        .iter_mut()
        .zip(samples)
    {
        *target = sample.to_ne_bytes();
    }
    frame.set_pts(Some(pts));
    Some(frame)
}

/// The silence of a hundredth of a second, at `pts`, which ends a
/// recording where the clock does when the input fell quiet before: the
/// filter graph fills the gap up to it.
pub(crate) fn closing_silence(
    counted: i64,
    (rate, channels): (u32, u16),
    start: Instant,
) -> Option<ffmpeg::frame::Audio> {
    let frames = (rate / 100).max(1) as usize;
    let at = sound_time(counted, frames, rate, start);
    (at > counted)
        .then(|| {
            sound_frame(
                &vec![0.0; frames * usize::from(channels.max(1))],
                (rate, channels),
                at,
            )
        })
        .flatten()
}

/// What the device's callback, the recording thread and the handle share.
struct Shared {
    rate: u32,
    channels: u16,
    /// The loudest sample of each channel since the levels were last
    /// read, as f32 bits; bits of positive floats order as the floats do.
    peaks: Vec<AtomicU32>,
    /// Samples waiting for the encoder, interleaved, while recording.
    waiting: Mutex<Option<VecDeque<f32>>>,
    arrived: Condvar,
    /// Frames recorded, and bytes written, so far.
    recorded: AtomicU64,
    written: AtomicU64,
    /// Ends the device's thread.
    stop: AtomicBool,
}

impl Shared {
    fn waiting(&self) -> MutexGuard<'_, Option<VecDeque<f32>>> {
        self.waiting.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Takes samples from the device: notes their peaks, and keeps them
    /// for the encoder while recording.
    fn take(&self, samples: &[f32]) {
        let channels = usize::from(self.channels.max(1));
        let mut peaks = vec![0.0f32; channels];
        for (index, sample) in samples.iter().enumerate() {
            let peak = &mut peaks[index % channels];
            *peak = peak.max(sample.abs());
        }
        for (shared, peak) in self.peaks.iter().zip(peaks) {
            shared.fetch_max(peak.to_bits(), Ordering::Relaxed);
        }
        if let Some(queue) = self.waiting().as_mut() {
            queue.extend(samples);
            self.arrived.notify_one();
        }
    }
}

/// An open input device, its levels read for a meter, that records to a
/// file when asked. Dropping it finishes any recording, then closes the
/// device.
pub struct Microphone {
    shared: Arc<Shared>,
    name: String,
    input: Option<JoinHandle<()>>,
    recording: Option<Recording>,
}

/// A recording's encoding thread, and the flag that ends it.
struct Recording {
    finish: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Microphone {
    /// Opens input device `device`, by its id, or the system's default
    /// when it is none or gone.
    pub fn open(device: Option<&str>) -> Result<Self, String> {
        crate::init();
        let device = device.map(str::to_owned);
        let (sender, receiver) = mpsc::channel();
        // The stream stays on the thread that made it, which keeps it
        // until told to stop.
        let input = std::thread::Builder::new()
            .name("plaa-microphone".into())
            .spawn(move || match open_stream(device.as_deref()) {
                Ok((stream, shared, name)) => {
                    let _ = sender.send(Ok((shared.clone(), name)));
                    while !shared.stop.load(Ordering::Relaxed) {
                        std::thread::sleep(Duration::from_millis(50));
                    }
                    drop(stream);
                }
                Err(error) => {
                    let _ = sender.send(Err(error));
                }
            })
            .map_err(|error| error.to_string())?;
        let (shared, name) = receiver
            .recv_timeout(OPENING)
            .map_err(|_| "the input device did not open".to_owned())??;
        Ok(Self {
            shared,
            name,
            input: Some(input),
            recording: None,
        })
    }

    /// The device's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The device's sample rate and channels, which it records at.
    pub fn format(&self) -> (u32, u16) {
        (self.shared.rate, self.shared.channels)
    }

    /// The device's sound for something else to record, as a screen
    /// recording takes it.
    pub(crate) fn tap(&self) -> SoundTap {
        SoundTap {
            shared: self.shared.clone(),
        }
    }

    /// The loudest sample of each channel since last asked, 0 to 1.
    pub fn levels(&self) -> Vec<f32> {
        self.shared
            .peaks
            .iter()
            .map(|peak| f32::from_bits(peak.swap(0, Ordering::Relaxed)).min(1.0))
            .collect()
    }

    /// Starts recording to `destination`, as `sound` at `quality`;
    /// `events` hears when it ends, from its thread.
    pub fn record(
        &mut self,
        destination: PathBuf,
        (sound, quality): (Sound, u32),
        events: impl Fn(RecordEvent) + Send + 'static,
    ) -> Result<(), String> {
        if self.recording.is_some() {
            return Err("already recording".to_owned());
        }
        self.shared.recorded.store(0, Ordering::Relaxed);
        self.shared.written.store(0, Ordering::Relaxed);
        *self.shared.waiting() = Some(VecDeque::new());
        let finish = Arc::new(AtomicBool::new(false));
        let thread = {
            let (shared, finish) = (self.shared.clone(), finish.clone());
            std::thread::Builder::new()
                .name("plaa-record".into())
                .spawn(move || {
                    let temporary = export::temporary_path(&destination);
                    let written = write(&shared, &temporary, (sound, quality), &finish);
                    *shared.waiting() = None;
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

    /// Ends the recording: what was recorded is written out, and the
    /// event says so.
    pub fn stop(&mut self) {
        if let Some(recording) = &self.recording {
            recording.finish.store(true, Ordering::Relaxed);
            self.shared.arrived.notify_all();
        }
    }

    pub fn is_recording(&self) -> bool {
        self.recording
            .as_ref()
            .is_some_and(|recording| !recording.finish.load(Ordering::Relaxed))
    }

    /// How long the recording lasts so far, and how many bytes it takes.
    pub fn recorded(&self) -> (Duration, u64) {
        let frames = self.shared.recorded.load(Ordering::Relaxed);
        (
            Duration::from_secs_f64(frames as f64 / f64::from(self.shared.rate.max(1))),
            self.shared.written.load(Ordering::Relaxed),
        )
    }

    /// Forgets a recording that has ended, so another can start.
    pub fn ended(&mut self) {
        if let Some(mut recording) = self.recording.take()
            && let Some(thread) = recording.thread.take()
        {
            let _ = thread.join();
        }
    }
}

impl Drop for Microphone {
    fn drop(&mut self) {
        self.stop();
        self.ended();
        self.shared.stop.store(true, Ordering::Relaxed);
        if let Some(input) = self.input.take() {
            let _ = input.join();
        }
    }
}

/// An input device's sound, taken by something else that records.
pub(crate) struct SoundTap {
    shared: Arc<Shared>,
}

impl SoundTap {
    /// The sample rate and channels.
    pub fn format(&self) -> (u32, u16) {
        (self.shared.rate, self.shared.channels)
    }

    /// Starts keeping the sound, from now on.
    pub fn start(&self) {
        *self.shared.waiting() = Some(VecDeque::new());
    }

    /// The sound kept since last asked, interleaved, waiting up to `wait`
    /// for some.
    pub fn take(&self, wait: Duration) -> Vec<f32> {
        let mut waiting = self.shared.waiting();
        if waiting.as_ref().is_some_and(VecDeque::is_empty) && !wait.is_zero() {
            waiting = self
                .shared
                .arrived
                .wait_timeout(waiting, wait)
                .unwrap_or_else(PoisonError::into_inner)
                .0;
        }
        waiting
            .as_mut()
            .map(|queue| queue.drain(..).collect())
            .unwrap_or_default()
    }

    /// Stops keeping the sound.
    pub fn end(&self) {
        *self.shared.waiting() = None;
    }
}

/// Opens an input stream from `device`, or the default one: the stream,
/// what it shares, and the device's name.
fn open_stream(device: Option<&str>) -> Result<(cpal::Stream, Arc<Shared>, String), String> {
    let host = cpal::default_host();
    fn find(mut devices: impl Iterator<Item = cpal::Device>, wanted: &str) -> Option<cpal::Device> {
        devices.find(|candidate| candidate.id().is_ok_and(|id| id.to_string() == wanted))
    }
    let chosen = device.and_then(|wanted| find(host.input_devices().into_iter().flatten(), wanted));
    // What an output plays, on Windows and macOS: an input stream on the
    // output.
    let playing = match (
        &chosen,
        device.and_then(|wanted| wanted.strip_suffix(PLAYS_BACK)),
    ) {
        (None, Some(output)) if cfg!(any(windows, target_os = "macos")) => {
            find(host.output_devices().into_iter().flatten(), output)
        }
        _ => None,
    };
    let plays_back = playing.is_some();
    let device = chosen
        .or(playing)
        .or_else(|| host.default_input_device())
        .ok_or_else(|| "no sound input device".to_owned())?;
    let name = device
        .description()
        .map(|description| description.name().to_owned())
        .unwrap_or_default();
    let supported = if plays_back {
        device.default_output_config()
    } else {
        device.default_input_config()
    }
    .map_err(|error| error.to_string())?;
    #[cfg_attr(not(target_os = "linux"), expect(unused_mut))]
    let mut config = supported.config();
    // The sound server hands sound over in large pieces unless asked for
    // small ones, so the meter would lag: periods of 25 ms.
    #[cfg(target_os = "linux")]
    if host.id() == cpal::HostId::PulseAudio {
        config.buffer_size = cpal::BufferSize::Fixed((config.sample_rate / 40).max(64));
    }
    let shared = Arc::new(Shared {
        rate: config.sample_rate,
        channels: config.channels,
        peaks: (0..config.channels).map(|_| AtomicU32::new(0)).collect(),
        waiting: Mutex::new(None),
        arrived: Condvar::new(),
        recorded: AtomicU64::new(0),
        written: AtomicU64::new(0),
        stop: AtomicBool::new(false),
    });
    let stream = match supported.sample_format() {
        cpal::SampleFormat::F32 => build::<f32>(&device, config, &shared),
        cpal::SampleFormat::I16 => build::<i16>(&device, config, &shared),
        cpal::SampleFormat::U16 => build::<u16>(&device, config, &shared),
        cpal::SampleFormat::I32 => build::<i32>(&device, config, &shared),
        cpal::SampleFormat::F64 => build::<f64>(&device, config, &shared),
        other => return Err(format!("unsupported sample format {other}")),
    }?;
    stream.play().map_err(|error| error.to_string())?;
    Ok((stream, shared, name))
}

fn build<T: cpal::SizedSample>(
    device: &cpal::Device,
    config: cpal::StreamConfig,
    shared: &Arc<Shared>,
) -> Result<cpal::Stream, String>
where
    f32: cpal::FromSample<T>,
{
    let callback = shared.clone();
    device
        .build_input_stream::<T, _, _>(
            config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                let samples: Vec<f32> = data
                    .iter()
                    .map(|sample| cpal::Sample::to_sample::<f32>(*sample))
                    .collect();
                callback.take(&samples);
            },
            |error| eprintln!("plaa: sound input: {error}"),
            None,
        )
        .map_err(|error| error.to_string())
}

/// Encodes what the device records into `temporary` until `finish` is
/// set, then what is left.
fn write(
    shared: &Shared,
    temporary: &std::path::Path,
    sound: (Sound, u32),
    finish: &AtomicBool,
) -> Result<(), ExportError> {
    let mut context =
        ffmpeg::format::output_as(&temporary, sound.0.muxer()).map_err(export::failed)?;
    let global_header = context
        .format()
        .flags()
        .contains(ffmpeg::format::Flags::GLOBAL_HEADER);
    let (codec, mut encoder) =
        export::sound_encoder(sound, shared.rate, shared.channels, global_header)?;
    let time_base = encoder.time_base();
    {
        let mut stream = context.add_stream(codec).map_err(export::failed)?;
        stream.set_parameters(&encoder);
        stream.set_time_base(time_base);
    }
    let mut header = ffmpeg::Dictionary::new();
    header.set("movflags", "+faststart");
    context.write_header_with(header).map_err(export::failed)?;
    let mut output = Output {
        context,
        packet: ffmpeg::Packet::empty(),
        written: 0,
    };
    let channels = usize::from(shared.channels.max(1));
    let layout = ffi::default_layout(i32::from(shared.channels));
    let samples_format = ffmpeg::format::Sample::F32(ffmpeg::format::sample::Type::Packed);
    let mut graph = export::sound_graph((shared.rate, samples_format, &layout), &encoder)?;
    let pull = |graph: &mut ffmpeg::filter::Graph,
                encoder: &mut ffmpeg::encoder::audio::Encoder,
                output: &mut Output|
     -> Result<(), ExportError> {
        let mut filtered = ffmpeg::frame::Audio::empty();
        while let Some(mut sink) = graph.get("out")
            && sink.sink().frame(&mut filtered).is_ok()
        {
            encoder.send_frame(&filtered).map_err(export::failed)?;
            output.drain(encoder, 0, time_base, 0)?;
        }
        Ok(())
    };
    let start = Instant::now();
    let mut recorded: i64 = 0;
    loop {
        let finishing = finish.load(Ordering::Relaxed);
        let samples: Vec<f32> = {
            let mut waiting = shared.waiting();
            if waiting.as_ref().is_some_and(VecDeque::is_empty) && !finishing {
                waiting = shared
                    .arrived
                    .wait_timeout(waiting, WAIT)
                    .unwrap_or_else(PoisonError::into_inner)
                    .0;
            }
            waiting
                .as_mut()
                .map(|queue| queue.drain(..).collect())
                .unwrap_or_default()
        };
        let frames = samples.len() / channels;
        let at = sound_time(recorded, frames, shared.rate, start);
        if let Some(frame) = sound_frame(&samples, (shared.rate, shared.channels), at) {
            recorded = at + frames as i64;
            if let Some(mut source) = graph.get("in") {
                source.source().add(&frame).map_err(export::failed)?;
            }
            pull(&mut graph, &mut encoder, &mut output)?;
            shared.recorded.store(recorded as u64, Ordering::Relaxed);
            shared.written.store(output.written, Ordering::Relaxed);
        }
        if finishing && frames == 0 {
            break;
        }
    }
    if let Some(silence) = closing_silence(recorded, (shared.rate, shared.channels), start)
        && let Some(mut source) = graph.get("in")
    {
        source.source().add(&silence).map_err(export::failed)?;
    }
    if let Some(mut source) = graph.get("in") {
        let _ = source.source().flush();
    }
    pull(&mut graph, &mut encoder, &mut output)?;
    let _ = encoder.send_eof();
    output.drain(&mut encoder, 0, time_base, 0)?;
    output.context.write_trailer().map_err(export::failed)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use super::{closing_silence, sound_time};

    #[test]
    fn sound_follows_the_clock_after_a_silence() {
        let rate = 48_000;
        let start = Instant::now() - Duration::from_secs(2);
        // Sound coming steadily is timed by what came before.
        assert_eq!(sound_time(95_000, 480, rate, start), 95_000);
        // A little late, as an input's latency makes it: still so.
        assert_eq!(sound_time(90_000, 480, rate, start), 90_000);
        // Nothing for a second and more: where the clock says.
        let at = sound_time(40_000, 480, rate, start);
        assert!((95_000..96_100).contains(&at), "{at}");
        // A recording whose input fell quiet ends where the clock does.
        let silence = closing_silence(40_000, (rate, 2), start).unwrap();
        let end = silence.pts().unwrap() + silence.samples() as i64;
        assert!((95_500..96_100).contains(&end), "{end}");
        assert!(closing_silence(95_900, (rate, 2), start).is_none());
    }
}
