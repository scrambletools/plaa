//! Sound carried between plaa and the audio device of the entity that
//! sends and takes its streams: the Mac's own entity's Core Audio device,
//! or avb-endpointd's devices for plaa's entity on Linux.

use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use atdecc::EntityId;
use atdecc::controller::Controller;
use avb_stream::audio::{Input as AudioInput, Output, Sink, Source};

use crate::sound::CHANNELS;

/// The rate the device is fed at; it converts to its own.
const RATE: u32 = 48_000;

/// How often the feed hands the device what came in since.
const STEP: Duration = Duration::from_millis(5);

/// The entity's first audio stream input, while it takes a talker's
/// stream, as the controller last heard.
pub(crate) fn bound_input(controller: &Controller, entity: EntityId) -> Option<u16> {
    let model = controller.model(entity)?;
    let input = model
        .streams(true)
        .find(|stream| !stream.is_media_clock())?;
    model.binding(input.index)?.talker_stream()?;
    Some(input.index)
}

/// Plays `source` into `sink` on a thread of its own, two channels at a
/// time, each side steered to its own clock: what plaa plays into the
/// device, or the device's stream out of it.
pub(crate) struct Feed {
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Feed {
    pub(crate) fn start(name: &str, source: Source, sink: Sink) -> io::Result<Self> {
        let stop = Arc::new(AtomicBool::new(false));
        let (opened, result) = std::sync::mpsc::sync_channel(1);
        let thread = {
            let stop = stop.clone();
            std::thread::Builder::new()
                .name(name.into())
                .spawn(move || {
                    let streams = AudioInput::open(&source, CHANNELS, RATE)
                        .and_then(|input| Ok((input, Output::open(&sink, CHANNELS, RATE)?)));
                    let (mut input, output) = match streams {
                        Ok(streams) => {
                            let _ = opened.send(Ok(()));
                            streams
                        }
                        Err(error) => {
                            let _ = opened.send(Err(error));
                            return;
                        }
                    };
                    let started = Instant::now();
                    let mut sent = 0u64;
                    let mut frames = Vec::new();
                    while !stop.load(Ordering::Relaxed) {
                        std::thread::sleep(STEP);
                        let due = (started.elapsed().as_secs_f64() * f64::from(RATE)) as u64;
                        // After a stall, only the last tenth of a second.
                        let count = due.saturating_sub(sent).min(u64::from(RATE / 10));
                        sent = due;
                        frames.resize(count as usize * usize::from(CHANNELS), 0.0);
                        input.read(&mut frames);
                        output.write(&frames, None);
                    }
                })?
        };
        let feed = Self {
            stop,
            thread: Some(thread),
        };
        result
            .recv()
            .map_err(|_| io::Error::other("the feed ended"))??;
        Ok(feed)
    }
}

impl Drop for Feed {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}
