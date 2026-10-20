//! ScreenCaptureKit's pictures, on macOS: a display, with plaa's own
//! windows left out, or a window; and what there is to record.

#![allow(unsafe_code)]

use std::sync::mpsc;
use std::sync::{Arc, PoisonError};
use std::time::{Duration, Instant};

use block2::RcBlock;
use dispatch2::{DispatchQueue, DispatchRetained};
use ffmpeg::format::Pixel;
use ffmpeg_next as ffmpeg;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObject, NSObjectProtocol, ProtocolObject};
use objc2::{AnyThread, DefinedClass, define_class, msg_send};
use objc2_core_foundation::CFArray;
use objc2_core_media::{CMSampleBuffer, CMTime};
use objc2_core_video::{
    CVPixelBufferGetBaseAddress, CVPixelBufferGetBytesPerRow, CVPixelBufferGetHeight,
    CVPixelBufferGetWidth, CVPixelBufferLockBaseAddress, CVPixelBufferLockFlags,
    CVPixelBufferUnlockBaseAddress, kCVPixelFormatType_32BGRA,
};
use objc2_foundation::{NSArray, NSDictionary, NSError, NSNumber, NSString};
use objc2_screen_capture_kit::{
    SCContentFilter, SCFrameStatus, SCShareableContent, SCStream, SCStreamConfiguration,
    SCStreamDelegate, SCStreamFrameInfoStatus, SCStreamOutput, SCStreamOutputType,
};

use super::{CaptureShared, Frames, Shareable, Target};

/// How long ScreenCaptureKit may take to answer.
const ANSWER: Duration = Duration::from_secs(5);

/// A ScreenCaptureKit object handed from the queue it was made on to the
/// thread that waits for it. They are immutable descriptions, or a stream
/// whose methods may be called from any thread.
struct Handed<T>(T);

// SAFETY: see `Handed`.
unsafe impl<T> Send for Handed<T> {}

/// What there is to record: the displays and windows, as the system
/// lists them, or why it would not say, as when not allowed to.
fn shareable_content() -> Result<Retained<SCShareableContent>, String> {
    let (sender, receiver) = mpsc::channel();
    let block = RcBlock::new(
        move |content: *mut SCShareableContent, error: *mut NSError| {
            // SAFETY: the pointers are the handler's arguments, valid for the
            // call, and one of them is set.
            let result = unsafe {
                match Retained::retain(content) {
                    Some(content) => Ok(Handed(content)),
                    None => Err(Retained::retain(error)
                        .map(|error| error.localizedDescription().to_string())
                        .unwrap_or_default()),
                }
            };
            let _ = sender.send(result);
        },
    );
    // SAFETY: the handler is a block of the type the method takes.
    unsafe { SCShareableContent::getShareableContentWithCompletionHandler(&block) };
    receiver
        .recv_timeout(ANSWER)
        .map_err(|_| "the system did not say what there is to record".to_owned())?
        .map(|Handed(content)| content)
}

/// The displays, the main one first, then the windows of other apps.
pub fn shareable() -> Result<Vec<Shareable>, String> {
    let content = shareable_content()?;
    let own = std::process::id() as i32;
    // SAFETY: plain queries of the objects the system returned.
    unsafe {
        let displays = content.displays();
        let main = objc2_core_graphics::CGMainDisplayID();
        let several = displays.count() > 1;
        let mut listed: Vec<Shareable> = displays
            .iter()
            .enumerate()
            .map(|(number, display)| Shareable {
                target: Target::Screen(number as u32),
                main: several && display.displayID() == main,
                name: String::new(),
            })
            .collect();
        listed.sort_by_key(|shareable| !shareable.main);
        for window in content.windows().iter() {
            let owner = window.owningApplication();
            let theirs = owner.as_ref().is_some_and(|owner| owner.processID() != own);
            let frame = window.frame();
            let title = window
                .title()
                .map(|title| title.to_string())
                .unwrap_or_default();
            // Windows people work in: on screen, in the normal layer, of
            // some size, and named.
            if !theirs
                || !window.isOnScreen()
                || window.windowLayer() != 0
                || frame.size.width < 40.0
                || frame.size.height < 40.0
                || title.trim().is_empty()
            {
                continue;
            }
            let app = owner
                .map(|owner| owner.applicationName().to_string())
                .unwrap_or_default();
            listed.push(Shareable {
                target: Target::Window(u64::from(window.windowID())),
                main: false,
                name: if app.is_empty() || app == title {
                    title
                } else {
                    format!("{app}: {title}")
                },
            });
        }
        Ok(listed)
    }
}

struct Ivars {
    frames: Arc<Frames>,
    shared: Arc<CaptureShared>,
}

define_class!(
    /// Takes the stream's pictures, and hears when it stops.
    #[unsafe(super(NSObject))]
    #[name = "PlaaScreenCapture"]
    #[ivars = Ivars]
    struct Taker;

    unsafe impl NSObjectProtocol for Taker {}

    unsafe impl SCStreamOutput for Taker {
        #[unsafe(method(stream:didOutputSampleBuffer:ofType:))]
        fn stream_did_output(
            &self,
            _stream: &SCStream,
            sample: &CMSampleBuffer,
            kind: SCStreamOutputType,
        ) {
            if kind == SCStreamOutputType::Screen {
                self.take(sample);
            }
        }
    }

    unsafe impl SCStreamDelegate for Taker {
        #[unsafe(method(stream:didStopWithError:))]
        fn stream_did_stop(&self, _stream: &SCStream, error: &NSError) {
            *self
                .ivars()
                .shared
                .ended
                .lock()
                .unwrap_or_else(PoisonError::into_inner) =
                Some(error.localizedDescription().to_string());
        }
    }
);

impl Taker {
    fn new(frames: Arc<Frames>, shared: Arc<CaptureShared>) -> Retained<Self> {
        let this = Self::alloc().set_ivars(Ivars { frames, shared });
        // SAFETY: NSObject's init, on an object just allocated.
        unsafe { msg_send![super(this), init] }
    }

    /// Copies a new picture out of `sample`, if it holds one and one is
    /// wanted.
    fn take(&self, sample: &CMSampleBuffer) {
        let at = Instant::now();
        let ivars = self.ivars();
        if !complete(sample) || !ivars.frames.wanted(at) {
            return;
        }
        // SAFETY: the sample buffer is the callback's, valid for it.
        let Some(pixels) = (unsafe { sample.image_buffer() }) else {
            return;
        };
        let (width, height) = (
            CVPixelBufferGetWidth(&pixels),
            CVPixelBufferGetHeight(&pixels),
        );
        if width == 0 || height == 0 {
            return;
        }
        *ivars
            .shared
            .size
            .lock()
            .unwrap_or_else(PoisonError::into_inner) = Some((width as u32, height as u32));
        let mut frame = ffmpeg::frame::Video::new(Pixel::BGRA, width as u32, height as u32);
        let row = width * 4;
        let target_stride = frame.stride(0);
        // SAFETY: the buffer is locked for reading while its rows are
        // copied, each `row` bytes long and `stride` apart, as it says.
        unsafe {
            if CVPixelBufferLockBaseAddress(&pixels, CVPixelBufferLockFlags::ReadOnly) != 0 {
                return;
            }
            let base = CVPixelBufferGetBaseAddress(&pixels).cast::<u8>();
            let stride = CVPixelBufferGetBytesPerRow(&pixels);
            if !base.is_null() && stride >= row {
                let target = frame.data_mut(0);
                for line in 0..height {
                    let from = std::slice::from_raw_parts(base.add(line * stride), row);
                    target[line * target_stride..line * target_stride + row].copy_from_slice(from);
                }
            }
            CVPixelBufferUnlockBaseAddress(&pixels, CVPixelBufferLockFlags::ReadOnly);
        }
        ivars.frames.push(at, frame);
    }
}

/// Whether `sample` holds a new picture: ScreenCaptureKit also sends ones
/// saying nothing changed.
fn complete(sample: &CMSampleBuffer) -> bool {
    // SAFETY: the sample buffer is the callback's; its attachments are a
    // CFArray of CFDictionary, toll-free bridged to NSArray of
    // NSDictionary.
    unsafe {
        let Some(attachments) = sample.sample_attachments_array(false) else {
            return false;
        };
        let attachments: &NSArray<NSDictionary<NSString, AnyObject>> =
            &*(&*attachments as *const CFArray).cast();
        let Some(first) = attachments.firstObject() else {
            return false;
        };
        let Some(status) = first.objectForKey(SCStreamFrameInfoStatus) else {
            return false;
        };
        status
            .downcast::<NSNumber>()
            .is_ok_and(|status| status.integerValue() == SCFrameStatus::Complete.0)
    }
}

/// A stream of a display's or window's pictures. Dropping it stops it.
pub struct Capture {
    stream: Retained<SCStream>,
    _taker: Retained<Taker>,
    _queue: DispatchRetained<DispatchQueue>,
}

// SAFETY: a stream's methods may be called from any thread; the taker
// and queue are only kept alive here.
unsafe impl Send for Capture {}

impl Capture {
    pub fn open(
        target: Target,
        frames: Arc<Frames>,
        shared: Arc<CaptureShared>,
    ) -> Result<Self, String> {
        let content = shareable_content()?;
        let own = std::process::id() as i32;
        // SAFETY: the objects are the system's, and the calls are as
        // ScreenCaptureKit documents them.
        unsafe {
            let filter = match target {
                // A display, but plaa's own windows, as QuickTime Player
                // leaves its own out.
                Target::Screen(number) => {
                    let displays = content.displays();
                    let display = displays
                        .iter()
                        .nth(number as usize)
                        .ok_or_else(|| "that screen is gone".to_owned())?;
                    let plaa: Vec<_> = content
                        .applications()
                        .iter()
                        .filter(|application| application.processID() == own)
                        .collect();
                    SCContentFilter::initWithDisplay_excludingApplications_exceptingWindows(
                        SCContentFilter::alloc(),
                        &display,
                        &NSArray::from_retained_slice(&plaa),
                        &NSArray::new(),
                    )
                }
                Target::Window(id) => {
                    let window = content
                        .windows()
                        .iter()
                        .find(|window| u64::from(window.windowID()) == id)
                        .ok_or_else(|| "that window is gone".to_owned())?;
                    SCContentFilter::initWithDesktopIndependentWindow(
                        SCContentFilter::alloc(),
                        &window,
                    )
                }
            };
            let info = SCShareableContent::infoForFilter(&filter);
            let rect = info.contentRect();
            let scale = f64::from(info.pointPixelScale());
            let config = SCStreamConfiguration::new();
            config.setWidth((rect.size.width * scale).round().max(2.0) as usize);
            config.setHeight((rect.size.height * scale).round().max(2.0) as usize);
            config.setPixelFormat(kCVPixelFormatType_32BGRA);
            config.setShowsCursor(true);
            // As many pictures as the screen sends; a recording keeps them
            // to its rate.
            config.setMinimumFrameInterval(CMTime::new(1, 240));
            config.setQueueDepth(6);
            // The display's own colors may be P3: sRGB, as recordings say.
            config.setColorSpaceName(objc2_core_graphics::kCGColorSpaceSRGB);
            let taker = Taker::new(frames, shared);
            let stream = SCStream::initWithFilter_configuration_delegate(
                SCStream::alloc(),
                &filter,
                &config,
                Some(ProtocolObject::from_ref(&*taker)),
            );
            let queue = DispatchQueue::new("io.github.scrambletools.plaa.screen", None);
            stream
                .addStreamOutput_type_sampleHandlerQueue_error(
                    ProtocolObject::from_ref(&*taker),
                    SCStreamOutputType::Screen,
                    Some(&queue),
                )
                .map_err(|error| error.localizedDescription().to_string())?;
            let (sender, receiver) = mpsc::channel();
            let started = RcBlock::new(move |error: *mut NSError| {
                // SAFETY: the handler's argument, valid for the call.
                let error = Retained::retain(error);
                let _ = sender.send(error.map(|error| error.localizedDescription().to_string()));
            });
            stream.startCaptureWithCompletionHandler(Some(&started));
            if let Some(error) = receiver
                .recv_timeout(ANSWER)
                .map_err(|_| "the screen did not open".to_owned())?
            {
                return Err(error);
            }
            Ok(Self {
                stream,
                _taker: taker,
                _queue: queue,
            })
        }
    }
}

impl Drop for Capture {
    fn drop(&mut self) {
        let (sender, receiver) = mpsc::channel();
        let stopped = RcBlock::new(move |_: *mut NSError| {
            let _ = sender.send(());
        });
        // SAFETY: stopping a stream that started, with a handler of the
        // type the method takes.
        unsafe { self.stream.stopCaptureWithCompletionHandler(Some(&stopped)) };
        let _ = receiver.recv_timeout(Duration::from_secs(1));
    }
}
