//! Small pictures of a file at the times the timeline is pointed at, and
//! along it for a filmstrip: read on a thread of their own from the
//! file's keyframes, the latest time pointed at first, and kept for when
//! they are asked for again.

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex, PoisonError};
use std::thread::JoinHandle;
use std::time::Duration;

use ffmpeg::format::Pixel;
use ffmpeg::software::scaling;
use ffmpeg_next as ffmpeg;

use crate::decode::{self, Picture};
use crate::ffi;
use crate::info::MediaInfo;

/// How many pictures are kept, the oldest going first.
const KEPT: usize = 300;

/// A file's picture near a time.
#[derive(Debug, Clone)]
pub struct Thumbnail {
    /// The media time of the picture: the keyframe at or before the time
    /// asked for.
    pub seconds: f64,
    /// The time asked for.
    pub asked: f64,
    /// Asked for a filmstrip, rather than where the timeline is pointed.
    pub strip: bool,
    pub picture: Arc<Picture>,
}

/// The thread making a file's thumbnails. Dropping it stops the thread.
pub struct Thumbnails {
    shared: Arc<Shared>,
    thread: Option<JoinHandle<()>>,
}

struct Shared {
    wanted: Mutex<Wanted>,
    wake: Condvar,
    stop: AtomicBool,
}

/// The times asked for that the thread has yet to take.
#[derive(Default)]
struct Wanted {
    /// The latest time pointed at, made first.
    pointed: Option<f64>,
    /// A filmstrip's times, in order.
    strip: VecDeque<f64>,
}

impl Thumbnails {
    /// Starts making thumbnails of the file at `path`, `width` pixels wide;
    /// each is handed to `ready` from the thread.
    pub fn open(path: PathBuf, width: u32, ready: impl Fn(Thumbnail) + Send + 'static) -> Self {
        crate::init();
        let shared = Arc::new(Shared {
            wanted: Mutex::new(Wanted::default()),
            wake: Condvar::new(),
            stop: AtomicBool::new(false),
        });
        let thread = {
            let shared = shared.clone();
            std::thread::Builder::new()
                .name("plaa-thumbnails".into())
                .spawn(move || run(&path, width.max(16), &shared, &ready))
                .ok()
        };
        Self { shared, thread }
    }

    /// Asks for the picture at `seconds`, in place of any pointed at not
    /// yet made.
    pub fn request(&self, seconds: f64) {
        self.shared
            .wanted
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .pointed = Some(seconds.max(0.0));
        self.shared.wake.notify_one();
    }

    /// Asks for pictures at each of `times`, for a filmstrip, in place of
    /// one asked for before.
    pub fn request_strip(&self, times: impl IntoIterator<Item = f64>) {
        self.shared
            .wanted
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .strip = times.into_iter().map(|seconds| seconds.max(0.0)).collect();
        self.shared.wake.notify_one();
    }
}

impl Drop for Thumbnails {
    fn drop(&mut self) {
        self.shared.stop.store(true, Ordering::Relaxed);
        self.shared.wake.notify_one();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// The file's video stream, how it is turned and its shape on screen.
struct Video {
    index: usize,
    quarter_turns: u8,
    aspect: f64,
}

fn run(path: &std::path::Path, width: u32, shared: &Shared, ready: &dyn Fn(Thumbnail)) {
    let stop = || shared.stop.load(Ordering::Relaxed);
    let Ok(mut input) = ffmpeg::format::input(path) else {
        return;
    };
    let info = MediaInfo::read(&input);
    let Some(video) = info
        .video
        .iter()
        .find(|track| !track.cover)
        .map(|track| Video {
            index: track.track.index,
            quarter_turns: track.quarter_turns,
            aspect: track.aspect,
        })
    else {
        return;
    };
    let start = decode::start_seconds(&input);
    let Some(mut decoder) = open(&input, video.index) else {
        return;
    };
    let mut kept: HashMap<u64, Thumbnail> = HashMap::new();
    let mut order: VecDeque<u64> = VecDeque::new();
    while !stop() {
        let wanted = {
            let mut wanted = shared.wanted.lock().unwrap_or_else(PoisonError::into_inner);
            while wanted.pointed.is_none() && wanted.strip.is_empty() && !stop() {
                wanted = shared
                    .wake
                    .wait_timeout(wanted, Duration::from_millis(200))
                    .unwrap_or_else(PoisonError::into_inner)
                    .0;
            }
            match wanted.pointed.take() {
                Some(seconds) => Some((seconds, false)),
                None => wanted.strip.pop_front().map(|seconds| (seconds, true)),
            }
        };
        let Some((seconds, strip)) = wanted else {
            continue;
        };
        // By the keyframe a seek lands on, from the index when the file
        // has one; else by the second.
        let keyframe = ffi::keyframe_before(&input, video.index, seconds, start);
        let key = (keyframe.unwrap_or(seconds.floor()) * 1000.0) as u64;
        if let Some(thumbnail) = kept.get(&key) {
            ready(Thumbnail {
                asked: seconds,
                strip,
                ..thumbnail.clone()
            });
            continue;
        }
        let Some(mut thumbnail) = make(&mut input, &mut decoder, &video, seconds, start, width)
        else {
            continue;
        };
        thumbnail.strip = strip;
        if kept.len() >= KEPT
            && let Some(oldest) = order.pop_front()
        {
            kept.remove(&oldest);
        }
        order.push_back(key);
        kept.insert(key, thumbnail.clone());
        ready(thumbnail);
    }
}

/// A decoder for stream `index` that hands out each picture as soon as it
/// is decoded: slices on threads, not whole frames, which would hold
/// pictures back for more packets.
fn open(input: &ffmpeg::format::context::Input, index: usize) -> Option<ffmpeg::decoder::Video> {
    let stream = input.stream(index)?;
    let mut context = ffmpeg::codec::Context::from_parameters(stream.parameters()).ok()?;
    context.set_threading(ffmpeg::codec::threading::Config::kind(
        ffmpeg::codec::threading::Type::Slice,
    ));
    let mut decoder = context.decoder();
    decoder.set_packet_time_base(stream.time_base());
    decoder.video().ok()
}

/// The first picture from the keyframe at or before `seconds`, scaled to
/// `width` and turned upright.
fn make(
    input: &mut ffmpeg::format::context::Input,
    decoder: &mut ffmpeg::decoder::Video,
    video: &Video,
    seconds: f64,
    start: f64,
    width: u32,
) -> Option<Thumbnail> {
    let timestamp = ((seconds + start) * f64::from(ffmpeg::ffi::AV_TIME_BASE)) as i64;
    input.seek(timestamp, ..timestamp).ok()?;
    decoder.flush();
    let time_base = input.stream(video.index)?.time_base();
    let mut frame = ffmpeg::frame::Video::empty();
    // Enough packets for a keyframe's slices, without reading on for ever
    // in a file whose pictures do not decode.
    for (stream, packet) in input.packets().take(2000) {
        if stream.index() != video.index {
            continue;
        }
        if decoder.send_packet(&packet).is_err() {
            continue;
        }
        if decoder.receive_frame(&mut frame).is_ok() {
            let at = frame
                .timestamp()
                .and_then(|timestamp| crate::info::seconds(timestamp, time_base))
                .map_or(seconds, |time| time - start);
            let picture = scale(&frame, video, width)?;
            return Some(Thumbnail {
                seconds: at,
                asked: seconds,
                strip: false,
                picture: Arc::new(picture),
            });
        }
    }
    None
}

/// `frame` as RGBA `width` pixels wide once turned upright, in its shape
/// on screen.
fn scale(frame: &ffmpeg::frame::Video, video: &Video, width: u32) -> Option<Picture> {
    let (frame_width, frame_height) = (frame.width(), frame.height());
    if frame_width == 0 || frame_height == 0 {
        return None;
    }
    let aspect = if video.aspect.is_finite() && video.aspect > 0.0 {
        video.aspect
    } else {
        f64::from(frame_width) / f64::from(frame_height)
    };
    let turned = video.quarter_turns % 2 == 1;
    // The size before turning, so that turned it is `width` wide.
    let (out_width, out_height) = if turned {
        let height = width;
        (((f64::from(height) * aspect).round() as u32).max(1), height)
    } else {
        (width, ((f64::from(width) / aspect).round() as u32).max(1))
    };
    let mut scaler = scaling::Context::get(
        frame.format(),
        frame_width,
        frame_height,
        Pixel::RGBA,
        out_width,
        out_height,
        scaling::Flags::AREA,
    )
    .ok()?;
    let mut out = ffmpeg::frame::Video::empty();
    scaler.run(frame, &mut out).ok()?;
    let stride = out.stride(0);
    let row = out_width as usize * 4;
    let mut rgba = Vec::with_capacity(row * out_height as usize);
    for line in out.data(0).chunks(stride).take(out_height as usize) {
        rgba.extend_from_slice(&line[..row.min(line.len())]);
    }
    Some(
        Picture {
            width: out_width,
            height: out_height,
            rgba,
        }
        .transformed(crate::edit::Transform {
            turns: video.quarter_turns,
            mirrored: false,
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::edit::Transform;

    #[test]
    fn pictures_turn_clockwise_and_mirror() {
        // Two pixels side by side, red then blue.
        let picture = || Picture {
            width: 2,
            height: 1,
            rgba: vec![255, 0, 0, 255, 0, 0, 255, 255],
        };
        let turned = picture().transformed(Transform {
            turns: 1,
            mirrored: false,
        });
        assert_eq!((turned.width, turned.height), (1, 2));
        // A clockwise turn puts the left pixel on top.
        assert_eq!(&turned.rgba[..4], &[255, 0, 0, 255]);
        assert_eq!(&turned.rgba[4..], &[0, 0, 255, 255]);
        let mirrored = picture().transformed(Transform {
            turns: 0,
            mirrored: true,
        });
        assert_eq!(&mirrored.rgba[..4], &[0, 0, 255, 255]);
        // Mirrored, then turned: the right pixel ends on top.
        let both = picture().transformed(Transform {
            turns: 1,
            mirrored: true,
        });
        assert_eq!(&both.rgba[..4], &[0, 0, 255, 255]);
    }
}
