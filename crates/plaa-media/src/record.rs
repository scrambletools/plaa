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
use std::time::Duration;

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

/// The system's sound input devices.
pub fn input_devices() -> Vec<InputDevice> {
    crate::init();
    let host = cpal::default_host();
    let Ok(devices) = host.input_devices() else {
        return Vec::new();
    };
    devices
        .filter_map(|device| {
            Some(InputDevice {
                id: device.id().ok()?.to_string(),
                name: device.description().ok()?.name().to_owned(),
            })
        })
        .collect()
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
    let chosen = device.and_then(|wanted| {
        host.input_devices()
            .ok()?
            .find(|candidate| candidate.id().is_ok_and(|id| id.to_string() == wanted))
    });
    let device = chosen
        .or_else(|| host.default_input_device())
        .ok_or_else(|| "no sound input device".to_owned())?;
    let name = device
        .description()
        .map(|description| description.name().to_owned())
        .unwrap_or_default();
    let supported = device
        .default_input_config()
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
        if frames > 0 {
            let mut frame = ffmpeg::frame::Audio::new(
                samples_format,
                frames,
                ffmpeg::ChannelLayout::default(i32::from(shared.channels)),
            );
            frame.set_rate(shared.rate);
            let bytes = frame.data_mut(0);
            for (target, sample) in bytes
                .as_chunks_mut::<4>()
                .0
                .iter_mut()
                .zip(&samples[..frames * channels])
            {
                *target = sample.to_ne_bytes();
            }
            frame.set_pts(Some(recorded));
            recorded += frames as i64;
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
    if let Some(mut source) = graph.get("in") {
        let _ = source.source().flush();
    }
    pull(&mut graph, &mut encoder, &mut output)?;
    let _ = encoder.send_eof();
    output.drain(&mut encoder, 0, time_base, 0)?;
    output.context.write_trailer().map_err(export::failed)?;
    Ok(())
}
