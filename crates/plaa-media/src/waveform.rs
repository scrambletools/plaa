//! The shape of a sound file's sound from start to end, for the visualizer
//! to draw: read on a thread of its own, the loudest sample and the
//! average loudness of each short stretch, handed over as it goes.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use ffmpeg_next as ffmpeg;

use crate::{decode, ffi};

/// How many stretches a file's sound is cut into: enough for a wide
/// window to draw one a line.
pub const STRETCHES: usize = 4096;

/// How often the shape read so far is handed over.
const HAND_OVER: Duration = Duration::from_millis(250);

/// How loud a stretch of sound is, from 0 to 1.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Loudness {
    /// The loudest sample.
    pub peak: f32,
    /// The root mean square of the samples.
    pub average: f32,
}

/// How loud each stretch of a file's sound is, from its start.
#[derive(Debug, Clone)]
pub struct Shape {
    /// The file's length in seconds, which the stretches share evenly.
    pub duration: f64,
    pub stretches: Vec<Loudness>,
    /// How many stretches have been read, from the first.
    pub read: usize,
}

impl Shape {
    /// The loudness between `from` and `to` seconds, of all the stretches
    /// there; None where none has been read yet.
    pub fn between(&self, from: f64, to: f64) -> Option<Loudness> {
        if self.duration <= 0.0 {
            return None;
        }
        let index = |seconds: f64| {
            ((seconds / self.duration * STRETCHES as f64).max(0.0) as usize).min(STRETCHES - 1)
        };
        let first = index(from);
        if first >= self.read {
            return None;
        }
        let last = index(to).max(first).min(self.read - 1);
        let stretches = self.stretches.get(first..=last)?;
        let peak = stretches
            .iter()
            .map(|stretch| stretch.peak)
            .fold(0.0, f32::max);
        let squares: f32 = stretches
            .iter()
            .map(|stretch| stretch.average * stretch.average)
            .sum();
        Some(Loudness {
            peak,
            average: (squares / stretches.len() as f32).sqrt(),
        })
    }
}

/// The thread reading a file's shape. Dropping it stops the thread.
pub struct Waveform {
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Waveform {
    /// Starts reading the shape of the sound of the file at `path`, handed
    /// to `ready` from the thread now and then as it is read, and whole at
    /// the end. Files of unknown length have none.
    pub fn open(path: PathBuf, ready: impl Fn(Arc<Shape>) + Send + 'static) -> Self {
        crate::init();
        let stop = Arc::new(AtomicBool::new(false));
        let thread = {
            let stop = stop.clone();
            std::thread::Builder::new()
                .name("plaa-waveform".into())
                .spawn(move || read(&path, &stop, &ready))
                .ok()
        };
        Self { stop, thread }
    }
}

impl Drop for Waveform {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// Each stretch's loudest sample and sum of squares as it is read.
struct Reading {
    shape: Shape,
    squares: Vec<(f64, u64)>,
    /// Media seconds of the next frame, from the first frame's timestamp.
    at: Option<f64>,
    handed: Instant,
}

impl Reading {
    /// Takes in `samples` of `channels` interleaved at `rate`, from `at`
    /// seconds.
    fn take(&mut self, samples: &[f32], channels: usize, rate: f64) {
        let at = self.at.unwrap_or(0.0);
        let per_stretch = self.shape.duration / STRETCHES as f64;
        for (index, frame) in samples.chunks_exact(channels).enumerate() {
            let seconds = at + index as f64 / rate;
            let stretch = ((seconds / per_stretch).max(0.0) as usize).min(STRETCHES - 1);
            let loudest = frame
                .iter()
                .map(|sample| sample.abs())
                .fold(0.0, f32::max)
                .min(1.0);
            let peak = &mut self.shape.stretches[stretch].peak;
            *peak = peak.max(loudest);
            let (sum, count) = &mut self.squares[stretch];
            *sum += frame
                .iter()
                .map(|sample| f64::from(*sample) * f64::from(*sample))
                .sum::<f64>();
            *count += channels as u64;
        }
        let frames = samples.len() / channels;
        let next = at + frames as f64 / rate;
        self.at = Some(next);
        // The stretch now being read is not done yet.
        let read = ((next / per_stretch).max(0.0) as usize).min(STRETCHES);
        self.settle(read);
    }

    /// Works out the averages of the stretches before `read`.
    fn settle(&mut self, read: usize) {
        for stretch in self.shape.read..read.max(self.shape.read) {
            let (sum, count) = self.squares[stretch];
            if count > 0 {
                self.shape.stretches[stretch].average =
                    ((sum / count as f64).sqrt() as f32).min(1.0);
            }
        }
        self.shape.read = read.max(self.shape.read);
    }

    /// Hands over the shape read so far, when it is time to.
    fn hand_over(&mut self, ready: &dyn Fn(Arc<Shape>)) {
        if self.handed.elapsed() >= HAND_OVER {
            self.handed = Instant::now();
            ready(Arc::new(self.shape.clone()));
        }
    }
}

fn read(path: &Path, stop: &AtomicBool, ready: &dyn Fn(Arc<Shape>)) {
    let Ok(mut input) = ffmpeg::format::input(path) else {
        return;
    };
    let Some(index) = input
        .streams()
        .best(ffmpeg::media::Type::Audio)
        .map(|stream| stream.index())
    else {
        return;
    };
    if input.duration() <= 0 {
        return;
    }
    let duration = input.duration() as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE);
    let start = decode::start_seconds(&input);
    let Ok((decoder, time_base)) = decode::open_decoder(&input, index) else {
        return;
    };
    let Ok(mut decoder) = decoder.audio() else {
        return;
    };
    let mut reading = Reading {
        shape: Shape {
            duration,
            stretches: vec![Loudness::default(); STRETCHES],
            read: 0,
        },
        squares: vec![(0.0, 0); STRETCHES],
        at: None,
        handed: Instant::now(),
    };
    let mut graph = None;
    let mut decoded = ffmpeg::frame::Audio::empty();
    let mut converted = ffmpeg::frame::Audio::empty();
    let mut pull = |decoder: &mut ffmpeg::decoder::Audio, reading: &mut Reading| {
        while decoder.receive_frame(&mut decoded).is_ok() {
            if reading.at.is_none() {
                reading.at = decoded
                    .timestamp()
                    .and_then(|timestamp| crate::info::seconds(timestamp, time_base))
                    .map(|seconds| seconds - start);
            }
            if graph.is_none() {
                graph = float_graph(&decoded, time_base).ok();
            }
            let Some(graph) = &mut graph else {
                return;
            };
            if graph
                .get("in")
                .is_none_or(|mut source| source.source().add(&decoded).is_err())
            {
                continue;
            }
            while let Some(mut sink) = graph.get("out")
                && sink.sink().frame(&mut converted).is_ok()
            {
                let channels = usize::from(converted.channels().max(1));
                let length = (converted.samples() * channels * 4).min(converted.data(0).len());
                let samples: Vec<f32> = converted.data(0)[..length]
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .map(|bytes| f32::from_ne_bytes(*bytes))
                    .collect();
                reading.take(&samples, channels, f64::from(converted.rate().max(1)));
            }
        }
    };
    for (stream, packet) in input.packets() {
        if stop.load(Ordering::Relaxed) {
            return;
        }
        if stream.index() != index || decoder.send_packet(&packet).is_err() {
            continue;
        }
        pull(&mut decoder, &mut reading);
        reading.hand_over(ready);
    }
    let _ = decoder.send_eof();
    pull(&mut decoder, &mut reading);
    // What the length promised but the file did not hold stays silent.
    reading.settle(STRETCHES);
    ready(Arc::new(reading.shape));
}

/// abuffer, then the decoded sound as packed floats, its rate and channels
/// kept.
fn float_graph(
    frame: &ffmpeg::frame::Audio,
    time_base: ffmpeg::Rational,
) -> Result<ffmpeg::filter::Graph, ffmpeg::Error> {
    let mut graph = ffmpeg::filter::Graph::new();
    let arguments = format!(
        "time_base={}/{}:sample_rate={}:sample_fmt={}:channel_layout={}",
        time_base.numerator(),
        time_base.denominator().max(1),
        frame.rate(),
        frame.format().name(),
        ffi::frame_layout(frame),
    );
    let abuffer = ffmpeg::filter::find("abuffer").ok_or(ffmpeg::Error::FilterNotFound)?;
    let abuffersink = ffmpeg::filter::find("abuffersink").ok_or(ffmpeg::Error::FilterNotFound)?;
    graph.add(&abuffer, "in", &arguments)?;
    graph.add(&abuffersink, "out", "")?;
    graph
        .output("in", 0)?
        .input("out", 0)?
        .parse("aformat=sample_fmts=flt")?;
    graph.validate()?;
    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::TAU;
    use std::io::Write;

    /// A WAV file of 16-bit stereo, a second of a quiet tone and then a
    /// second of a loud one.
    fn write_tones(path: &Path) {
        let rate: u32 = 8000;
        let mut samples = Vec::new();
        for (amplitude, seconds) in [(0.1, 1), (0.8, 1)] {
            for index in 0..rate * seconds {
                let value = amplitude * (TAU * 200.0 * f64::from(index) / f64::from(rate)).sin();
                let sample = (value * f64::from(i16::MAX)) as i16;
                samples.extend([sample, sample]);
            }
        }
        let bytes = (samples.len() * 2) as u32;
        let mut file = std::fs::File::create(path).expect("a file");
        file.write_all(b"RIFF").unwrap();
        file.write_all(&(36 + bytes).to_le_bytes()).unwrap();
        file.write_all(b"WAVEfmt ").unwrap();
        file.write_all(&16u32.to_le_bytes()).unwrap();
        file.write_all(&1u16.to_le_bytes()).unwrap();
        file.write_all(&2u16.to_le_bytes()).unwrap();
        file.write_all(&rate.to_le_bytes()).unwrap();
        file.write_all(&(rate * 4).to_le_bytes()).unwrap();
        file.write_all(&4u16.to_le_bytes()).unwrap();
        file.write_all(&16u16.to_le_bytes()).unwrap();
        file.write_all(b"data").unwrap();
        file.write_all(&bytes.to_le_bytes()).unwrap();
        for sample in samples {
            file.write_all(&sample.to_le_bytes()).unwrap();
        }
    }

    #[test]
    fn a_files_shape_is_read_to_the_end() {
        let folder = tempfile::tempdir().expect("a folder");
        let path = folder.path().join("tones.wav");
        write_tones(&path);
        let (sender, receiver) = std::sync::mpsc::channel();
        let _waveform = Waveform::open(path, move |shape| {
            let _ = sender.send(shape);
        });
        let shape = loop {
            let shape = receiver
                .recv_timeout(Duration::from_secs(10))
                .expect("a shape");
            if shape.read == STRETCHES {
                break shape;
            }
        };
        assert!((shape.duration - 2.0).abs() < 0.01);
        let quiet = shape.between(0.1, 0.9).expect("read");
        let loud = shape.between(1.1, 1.9).expect("read");
        assert!((quiet.peak - 0.1).abs() < 0.01, "{quiet:?}");
        assert!((loud.peak - 0.8).abs() < 0.01, "{loud:?}");
        // A sine's average is its peak over the root of two.
        assert!(
            (loud.average - 0.8 / 2.0f32.sqrt()).abs() < 0.02,
            "{loud:?}"
        );
    }

    #[test]
    fn nothing_read_is_none() {
        let shape = Shape {
            duration: 10.0,
            stretches: vec![Loudness::default(); STRETCHES],
            read: 0,
        };
        assert!(shape.between(0.0, 1.0).is_none());
        let shape = Shape { read: 100, ..shape };
        assert!(shape.between(0.0, 0.1).is_some());
        assert!(shape.between(9.0, 10.0).is_none(), "not read that far");
    }
}
