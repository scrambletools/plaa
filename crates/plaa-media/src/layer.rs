//! Sound mixed in over an edit: each layer's file decoded on its own,
//! made the rate and channels of the sound it joins, at the speed it
//! plays, and added to that sound sample by sample, in the edited time.

use std::collections::VecDeque;
use std::path::{Path, PathBuf};

use ffmpeg_next as ffmpeg;

use crate::decode;
use crate::edit::{Edit, Layer};
use crate::ffi;
use crate::info::{self, MediaInfo};

/// The layers of an edit, ready to mix into sound of one rate and
/// channels.
pub(crate) struct Layers {
    sounds: Vec<LayerSound>,
    rate: u32,
    channels: u16,
}

impl Layers {
    /// The layers of `edit`, whose file edited is at `main`, for sound of
    /// `rate` Hz in `channels` channels.
    pub fn new(edit: Option<&Edit>, main: Option<&Path>, rate: u32, channels: u16) -> Self {
        let sounds = edit
            .map(|edit| {
                edit.layers()
                    .iter()
                    .filter_map(|layer| {
                        let path = match edit.other(layer.source) {
                            Some(other) => other.path.clone(),
                            None => main?.to_path_buf(),
                        };
                        Some(LayerSound::new(*layer, path))
                    })
                    .collect()
            })
            .unwrap_or_default();
        Self {
            sounds,
            rate: rate.max(1),
            channels: channels.max(1),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.sounds.is_empty()
    }

    /// Adds the layers' sound to `samples`, interleaved frames from
    /// `position` seconds of the edited time on, `step` seconds apart, as
    /// it plays at `tempo`.
    pub fn add(&mut self, samples: &mut [f32], position: f64, step: f64, tempo: f64) {
        for sound in &mut self.sounds {
            sound.add(samples, (self.rate, self.channels), position, step, tempo);
        }
    }

    /// Adds the layers' sound to the planes of `frame`, planar floats
    /// starting at sample `first` of the edited time, at normal speed.
    pub fn add_planar(&mut self, frame: &mut ffmpeg::frame::Audio, first: i64) {
        if self.sounds.is_empty() {
            return;
        }
        let channels = usize::from(self.channels);
        let frames = frame.samples();
        let mut mixed = vec![0.0f32; frames * channels];
        let rate = f64::from(self.rate);
        self.add(&mut mixed, first as f64 / rate, 1.0 / rate, 1.0);
        for channel in 0..channels.min(frame.planes()) {
            let plane = frame.plane_mut::<f32>(channel);
            for (index, value) in plane.iter_mut().take(frames).enumerate() {
                *value += mixed[index * channels + channel];
            }
        }
    }
}

/// One layer's sound, decoded ahead of where it is mixed.
struct LayerSound {
    layer: Layer,
    path: PathBuf,
    open: Option<Open>,
    /// Samples decoded and made the sound's, interleaved, and the file's
    /// time of the first.
    buffer: VecDeque<f32>,
    buffered_at: f64,
    /// The file's time of the next sample out of the graph.
    cursor: Option<f64>,
    /// The file ran out, or would not open.
    ended: bool,
    failed: bool,
}

/// A layer's file, open at its sound, and the graph making it the
/// sound's.
struct Open {
    input: ffmpeg::format::context::Input,
    stream: usize,
    decoder: ffmpeg::decoder::Audio,
    time_base: ffmpeg::Rational,
    start: f64,
    graph: Option<(f64, ffmpeg::filter::Graph)>,
    packet: ffmpeg::Packet,
    tempo: f64,
}

impl LayerSound {
    fn new(layer: Layer, path: PathBuf) -> Self {
        Self {
            layer,
            path,
            open: None,
            buffer: VecDeque::new(),
            buffered_at: 0.0,
            cursor: None,
            ended: false,
            failed: false,
        }
    }

    /// Adds this layer to `samples`, as [`Layers::add`] says.
    fn add(
        &mut self,
        samples: &mut [f32],
        (rate, channels): (u32, u16),
        position: f64,
        step: f64,
        tempo: f64,
    ) {
        if self.failed || step <= 0.0 {
            return;
        }
        let channels = usize::from(channels);
        let frames = samples.len() / channels;
        let end = self.layer.end();
        // The frames this layer is heard in.
        let first = (((self.layer.at - position) / step).ceil().max(0.0)) as usize;
        let last = (((end - position) / step).ceil().max(0.0) as usize).min(frames);
        if first >= last {
            return;
        }
        let wanted = self.layer.from + (position + first as f64 * step - self.layer.at);
        // Within a frame of what is decoded goes on from it; otherwise the
        // file is read from there again, as after a seek.
        let tempo_changed = self
            .open
            .as_ref()
            .is_some_and(|open| (open.tempo - tempo).abs() > 1e-9);
        let buffered = self.buffer.len() / channels;
        let reach = self.buffered_at + buffered as f64 * step;
        let near = self.cursor.is_some()
            && wanted >= self.buffered_at - step
            && wanted <= reach.max(self.cursor.unwrap_or(reach)) + 0.5;
        if self.open.is_none() || tempo_changed || !near {
            self.restart(wanted, tempo);
        }
        // What comes before the frame wanted is left out.
        loop {
            if self.buffer.is_empty() && !self.decode_more(rate, channels, step) {
                return;
            }
            let skip = ((wanted - self.buffered_at) / step).round();
            if skip <= 0.0 {
                break;
            }
            let have = self.buffer.len() / channels;
            let drop = (skip as usize).min(have);
            self.buffer.drain(..drop * channels);
            self.buffered_at += drop as f64 * step;
            if drop < have {
                break;
            }
        }
        for frame in first..last {
            if self.buffer.len() < channels && !self.decode_more(rate, channels, step) {
                break;
            }
            let gain = self.layer.gain(position + frame as f64 * step);
            for channel in 0..channels {
                let value = self.buffer.pop_front().unwrap_or(0.0);
                samples[frame * channels + channel] += value * gain;
            }
            self.buffered_at += step;
        }
    }

    /// Opens the file, or seeks it, to read from its time `from`.
    fn restart(&mut self, from: f64, tempo: f64) {
        self.buffer.clear();
        self.cursor = None;
        self.ended = false;
        if self.open.is_none() {
            match open(&self.path, tempo) {
                Ok(open) => self.open = Some(open),
                Err(error) => {
                    eprintln!("plaa: mixing in {}: {error}", self.path.display());
                    self.failed = true;
                    return;
                }
            }
        }
        if let Some(open) = &mut self.open {
            let timestamp = ((from + open.start) * f64::from(ffmpeg::ffi::AV_TIME_BASE)) as i64;
            let _ = open.input.seek(timestamp, ..timestamp);
            open.decoder.flush();
            open.graph = None;
            open.tempo = tempo;
        }
        self.buffered_at = from;
    }

    /// Decodes more of the file into the buffer. Returns whether any came.
    fn decode_more(&mut self, rate: u32, channels: usize, step: f64) -> bool {
        let Some(open) = &mut self.open else {
            return false;
        };
        loop {
            // What the graph has ready first.
            if let Some((_, graph)) = &mut open.graph {
                let mut out = ffmpeg::frame::Audio::empty();
                if let Some(mut sink) = graph.get("out")
                    && sink.sink().frame(&mut out).is_ok()
                {
                    let count = out.samples();
                    if count == 0 {
                        continue;
                    }
                    let data = &out.data(0)[..(count * channels * 4).min(out.data(0).len())];
                    if self.buffer.is_empty() {
                        self.buffered_at = self.cursor.unwrap_or(self.buffered_at);
                    }
                    self.buffer.extend(
                        data.as_chunks::<4>()
                            .0
                            .iter()
                            .map(|bytes| f32::from_ne_bytes(*bytes)),
                    );
                    self.cursor =
                        Some(self.cursor.unwrap_or(self.buffered_at) + count as f64 * step);
                    return true;
                }
            }
            if self.ended {
                return false;
            }
            let mut decoded = ffmpeg::frame::Audio::empty();
            if open.decoder.receive_frame(&mut decoded).is_ok() {
                if self.cursor.is_none() {
                    self.cursor = decoded
                        .timestamp()
                        .or(decoded.pts())
                        .and_then(|timestamp| info::seconds(timestamp, open.time_base))
                        .map(|seconds| seconds - open.start);
                }
                if open.graph.is_none() {
                    match decode::build_graph(
                        &decoded,
                        open.time_base,
                        open.tempo,
                        rate,
                        channels as u16,
                    ) {
                        Ok(graph) => open.graph = Some((open.tempo, graph)),
                        Err(error) => {
                            eprintln!("plaa: mixing in {}: {error}", self.path.display());
                            self.failed = true;
                            return false;
                        }
                    }
                }
                if let Some((_, graph)) = &mut open.graph
                    && let Some(mut source) = graph.get("in")
                {
                    let _ = source.source().add(&decoded);
                }
                continue;
            }
            match open.packet.read(&mut open.input) {
                Ok(()) => {
                    if open.packet.stream() == open.stream {
                        let _ = open.decoder.send_packet(&open.packet);
                    }
                }
                Err(ffmpeg::Error::InvalidData) => {}
                Err(_) => {
                    // The file's end: what the decoder and graph hold, then
                    // nothing more.
                    let _ = open.decoder.send_eof();
                    let mut decoded = ffmpeg::frame::Audio::empty();
                    while open.decoder.receive_frame(&mut decoded).is_ok() {
                        if let Some((_, graph)) = &mut open.graph
                            && let Some(mut source) = graph.get("in")
                        {
                            let _ = source.source().add(&decoded);
                        }
                    }
                    if let Some((_, graph)) = &mut open.graph
                        && let Some(mut source) = graph.get("in")
                    {
                        let _ = source.source().flush();
                    }
                    self.ended = true;
                }
            }
        }
    }
}

/// Opens the sound of the file at `path`, to play at `tempo`.
fn open(path: &Path, tempo: f64) -> Result<Open, String> {
    let failed = |error: ffmpeg::Error| error.to_string();
    let mut input = ffmpeg::format::input(path).map_err(failed)?;
    let info = MediaInfo::read(&input);
    let stream = info
        .audio
        .first()
        .map(|track| track.track.index)
        .ok_or("it has no sound")?;
    for other in 0..input.nb_streams() as usize {
        ffi::set_skipped(&mut input, other, other != stream);
    }
    let (decoder, time_base) = decode::open_decoder(&input, stream).map_err(failed)?;
    Ok(Open {
        start: decode::start_seconds(&input),
        decoder: decoder.audio().map_err(failed)?,
        input,
        stream,
        time_base,
        graph: None,
        packet: ffmpeg::Packet::empty(),
        tempo,
    })
}
