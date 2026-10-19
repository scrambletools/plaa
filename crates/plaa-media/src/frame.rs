//! Decoded pictures ready to draw, in the few layouts the interface's
//! shader reads, and the queue that hands them to it on time.

use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use ffmpeg::format::Pixel;
use ffmpeg::util::color;
use ffmpeg_next as ffmpeg;

/// How the planes of a picture are laid out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layout {
    /// Y, U and V in planes of their own, a byte per sample.
    Planar8,
    /// Y, U and V in planes of their own, two little endian bytes per
    /// sample.
    Planar16,
    /// Y in a plane, U and V interleaved in a second, a byte per sample.
    SemiPlanar8,
    /// As `SemiPlanar8`, with two little endian bytes per sample.
    SemiPlanar16,
    /// Red, green, blue and alpha, a byte each.
    Rgba,
}

/// The matrix that turns Y'CbCr into R'G'B'.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Matrix {
    Bt601,
    Bt709,
    Bt2020,
}

/// How the picture's values map to light.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transfer {
    /// Ordinary video, about sRGB.
    Standard,
    /// HDR10's perceptual quantizer (SMPTE ST 2084).
    Pq,
    /// Hybrid log gamma (ARIB STD-B67).
    Hlg,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Colors {
    pub matrix: Matrix,
    /// Values span 0 to 255 rather than 16 to 235.
    pub full_range: bool,
    pub transfer: Transfer,
    /// BT.2020 primaries, which a standard screen shows by mapping them
    /// into BT.709's.
    pub wide_gamut: bool,
    /// What a sample's stored value is multiplied by to give 0 to 1: one
    /// over the largest value for its bit depth.
    pub scale: f32,
    /// Bits per sample, which set where black, white and grey sit.
    pub bits: u32,
}

/// A picture to draw. The planes stay in FFmpeg's frame, shared rather
/// than copied.
pub struct VideoFrame {
    /// Unique among all pictures, so the interface uploads each once.
    pub id: u64,
    pub serial: u64,
    /// Media seconds it is shown at.
    pub position: f64,
    pub width: u32,
    pub height: u32,
    /// Width over height as shown.
    pub aspect: f64,
    pub quarter_turns: u8,
    pub layout: Layout,
    pub colors: Colors,
    frame: ffmpeg::frame::Video,
}

impl std::fmt::Debug for VideoFrame {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("VideoFrame")
            .field("id", &self.id)
            .field("position", &self.position)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("layout", &self.layout)
            .finish()
    }
}

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

impl VideoFrame {
    /// The bytes of plane `index`, with `stride` bytes per row.
    pub fn plane(&self, index: usize) -> (&[u8], usize) {
        (self.frame.data(index), self.frame.stride(index))
    }

    /// Plane `index`'s size in samples.
    pub fn plane_size(&self, index: usize) -> (u32, u32) {
        (
            self.frame.plane_width(index),
            self.frame.plane_height(index),
        )
    }

    /// How many planes the layout has.
    pub fn planes(&self) -> usize {
        match self.layout {
            Layout::Planar8 | Layout::Planar16 => 3,
            Layout::SemiPlanar8 | Layout::SemiPlanar16 => 2,
            Layout::Rgba => 1,
        }
    }

    /// The picture as tightly packed RGBA, for saving it or for the
    /// processor renderer.
    pub fn to_rgba(&self) -> Option<(u32, u32, Vec<u8>)> {
        let mut scaler = ffmpeg::software::scaling::Context::get(
            self.frame.format(),
            self.width,
            self.height,
            Pixel::RGBA,
            self.width,
            self.height,
            ffmpeg::software::scaling::Flags::BICUBIC,
        )
        .ok()?;
        let mut out = ffmpeg::frame::Video::empty();
        scaler.run(&self.frame, &mut out).ok()?;
        let stride = out.stride(0);
        let row = self.width as usize * 4;
        let mut pixels = Vec::with_capacity(row * self.height as usize);
        for line in out.data(0).chunks(stride).take(self.height as usize) {
            pixels.extend_from_slice(&line[..row.min(line.len())]);
        }
        Some((self.width, self.height, pixels))
    }

    /// Wraps a decoded frame the shader can read as it is, or returns it
    /// to be converted first.
    pub(crate) fn wrap(
        frame: ffmpeg::frame::Video,
        serial: u64,
        position: f64,
        aspect: f64,
        quarter_turns: u8,
    ) -> Result<Self, ffmpeg::frame::Video> {
        let Some((layout, bits, shift)) = layout_of(frame.format()) else {
            return Err(frame);
        };
        let full_range = match frame.color_range() {
            color::Range::JPEG => true,
            color::Range::MPEG => false,
            _ => matches!(
                frame.format(),
                Pixel::YUVJ420P | Pixel::YUVJ422P | Pixel::YUVJ444P
            ),
        };
        let matrix = match frame.color_space() {
            color::Space::BT709 => Matrix::Bt709,
            color::Space::BT2020NCL | color::Space::BT2020CL => Matrix::Bt2020,
            color::Space::BT470BG | color::Space::SMPTE170M | color::Space::SMPTE240M => {
                Matrix::Bt601
            }
            // Unmarked: high definition is BT.709, smaller BT.601.
            _ if frame.width() >= 1280 || frame.height() > 576 => Matrix::Bt709,
            _ => Matrix::Bt601,
        };
        let transfer = match frame.color_transfer_characteristic() {
            color::TransferCharacteristic::SMPTE2084 => Transfer::Pq,
            color::TransferCharacteristic::ARIB_STD_B67 => Transfer::Hlg,
            _ => Transfer::Standard,
        };
        let wide_gamut = frame.color_primaries() == color::Primaries::BT2020;
        let largest = ((1u32 << bits) - 1) << shift;
        let colors = Colors {
            matrix,
            full_range: full_range || layout == Layout::Rgba,
            transfer,
            wide_gamut,
            scale: 1.0 / largest as f32,
            bits,
        };
        Ok(Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            serial,
            position,
            width: frame.width(),
            height: frame.height(),
            aspect,
            quarter_turns,
            layout,
            colors,
            frame,
        })
    }
}

/// The layout of a pixel format the shader reads directly, its bit depth,
/// and how far its values are shifted up within two bytes.
fn layout_of(format: Pixel) -> Option<(Layout, u32, u32)> {
    Some(match format {
        Pixel::YUV420P
        | Pixel::YUVJ420P
        | Pixel::YUV422P
        | Pixel::YUVJ422P
        | Pixel::YUV444P
        | Pixel::YUVJ444P => (Layout::Planar8, 8, 0),
        Pixel::YUV420P10LE | Pixel::YUV422P10LE | Pixel::YUV444P10LE => (Layout::Planar16, 10, 0),
        Pixel::YUV420P12LE | Pixel::YUV422P12LE | Pixel::YUV444P12LE => (Layout::Planar16, 12, 0),
        Pixel::YUV420P16LE => (Layout::Planar16, 16, 0),
        Pixel::NV12 => (Layout::SemiPlanar8, 8, 0),
        Pixel::P010LE => (Layout::SemiPlanar16, 10, 6),
        Pixel::P016LE => (Layout::SemiPlanar16, 16, 0),
        Pixel::RGBA => (Layout::Rgba, 8, 0),
        _ => return None,
    })
}

/// The format a picture the shader cannot read is converted to: 4:2:0 at
/// its own depth for YUV, else RGBA.
pub(crate) fn conversion_target(format: Pixel) -> Pixel {
    let Some(descriptor) = format.descriptor() else {
        return Pixel::RGBA;
    };
    let is_rgb_like = descriptor.nb_components() < 3
        || descriptor.name().starts_with("rgb")
        || descriptor.name().starts_with("bgr")
        || descriptor.name().starts_with("gbr")
        || descriptor.name().starts_with("argb")
        || descriptor.name().starts_with("abgr")
        || descriptor.name().starts_with("pal")
        || descriptor.name().starts_with("gray")
        || descriptor.name().starts_with("x");
    if is_rgb_like {
        return Pixel::RGBA;
    }
    if descriptor.name().contains("p10")
        || descriptor.name().contains("p12")
        || descriptor.name().contains("p14")
        || descriptor.name().contains("p16")
        || descriptor.name().contains("p9")
    {
        Pixel::YUV420P10LE
    } else {
        Pixel::YUV420P
    }
}

/// Pictures decoded ahead of the clock, the one shown, and when the next
/// is due.
#[derive(Default)]
pub struct Queue {
    frames: VecDeque<Arc<VideoFrame>>,
    /// The picture last chosen to show.
    shown: Option<Arc<VideoFrame>>,
    /// Only pictures of this serial are shown.
    serial: u64,
    /// The decoder has nothing more until the next seek.
    pub finished: bool,
}

/// How many pictures are decoded ahead.
pub const AHEAD: usize = 6;

impl Queue {
    pub fn is_full(&self) -> bool {
        self.frames.len() >= AHEAD
    }

    pub fn push(&mut self, frame: VideoFrame) {
        if frame.serial == self.serial {
            self.finished = false;
            self.frames.push_back(Arc::new(frame));
        }
    }

    /// Drops every picture waiting and takes only `serial`'s from now on,
    /// keeping the one shown until a new one comes.
    pub fn flush(&mut self, serial: u64) {
        self.frames.clear();
        self.serial = serial;
        self.finished = false;
    }

    pub fn serial(&self) -> u64 {
        self.serial
    }

    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// The picture to show at media time `now`: the last one due, passing
    /// over any that came too late. The first picture after a seek shows
    /// at once, so the new position appears while paused.
    pub fn take_due(&mut self, now: f64) -> Option<Arc<VideoFrame>> {
        let shown_is_current = self
            .shown
            .as_ref()
            .is_some_and(|shown| shown.serial == self.serial);
        if !shown_is_current && let Some(first) = self.frames.pop_front() {
            self.shown = Some(first);
        }
        while let Some(next) = self.frames.front() {
            if next.position > now {
                break;
            }
            self.shown = self.frames.pop_front();
        }
        self.shown.clone()
    }

    /// The picture shown, without moving on.
    pub fn shown(&self) -> Option<Arc<VideoFrame>> {
        self.shown.clone()
    }

    /// The media time the next picture is due.
    pub fn next_due(&self) -> Option<f64> {
        self.frames.front().map(|frame| frame.position)
    }

    /// Whether a picture waits that should show at once: the first after a
    /// seek.
    pub fn has_new(&self) -> bool {
        let shown_is_current = self
            .shown
            .as_ref()
            .is_some_and(|shown| shown.serial == self.serial);
        !shown_is_current && !self.frames.is_empty()
    }

    pub fn clear_shown(&mut self) {
        self.shown = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(serial: u64, position: f64) -> VideoFrame {
        let mut picture = ffmpeg::frame::Video::new(Pixel::YUV420P, 4, 4);
        picture.set_color_range(color::Range::MPEG);
        match VideoFrame::wrap(picture, serial, position, 1.0, 0) {
            Ok(frame) => frame,
            Err(_) => panic!("a readable layout"),
        }
    }

    #[test]
    fn shows_the_last_picture_due() {
        let mut queue = Queue::default();
        for position in [0.0, 0.04, 0.08, 0.12] {
            queue.push(frame(0, position));
        }
        assert_eq!(queue.take_due(0.0).unwrap().position, 0.0);
        assert_eq!(queue.take_due(0.09).unwrap().position, 0.08);
        assert_eq!(queue.next_due(), Some(0.12));
    }

    #[test]
    fn first_picture_after_a_seek_shows_at_once() {
        let mut queue = Queue::default();
        queue.push(frame(0, 0.0));
        queue.take_due(0.0);
        queue.flush(1);
        queue.push(frame(0, 0.5));
        assert!(!queue.has_new(), "stale pictures are refused");
        queue.push(frame(1, 30.0));
        assert!(queue.has_new());
        assert_eq!(queue.take_due(0.0).unwrap().position, 30.0);
    }

    #[test]
    fn unusual_formats_are_converted() {
        assert_eq!(conversion_target(Pixel::BGR24), Pixel::RGBA);
        assert_eq!(conversion_target(Pixel::GRAY8), Pixel::RGBA);
        assert_eq!(conversion_target(Pixel::YUV410P), Pixel::YUV420P);
        assert_eq!(conversion_target(Pixel::YUV440P10LE), Pixel::YUV420P10LE);
    }
}
