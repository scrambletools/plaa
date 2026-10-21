//! ASS and SSA subtitles drawn as their authors styled them, by libass:
//! loaded as plaa runs rather than linked, so plaa runs without it, and
//! shows those subtitles as plain text then.
#![allow(unsafe_code)]

use std::ffi::{CString, c_char, c_int, c_longlong, c_void};
use std::sync::{Arc, OnceLock};

use crate::subtitle::SubtitlePicture;

/// The names libass goes by on each system, most likely first: on macOS
/// the copy plaa.app carries first.
const NAMES: &[&str] = if cfg!(target_os = "macos") {
    &[
        "@executable_path/../Frameworks/libass.9.dylib",
        "libass.9.dylib",
        "/opt/homebrew/lib/libass.9.dylib",
        "/usr/local/lib/libass.9.dylib",
        "libass.dylib",
    ]
} else if cfg!(windows) {
    &["libass-9.dll", "ass-9.dll", "ass.dll", "libass.dll"]
} else {
    &["libass.so.9", "libass.so"]
};

/// One picture libass draws: an alpha bitmap of one color.
#[repr(C)]
struct AssImage {
    w: c_int,
    h: c_int,
    stride: c_int,
    bitmap: *const u8,
    /// RGBA, the A being transparency: 0 is opaque.
    color: u32,
    dst_x: c_int,
    dst_y: c_int,
    next: *const AssImage,
    kind: c_int,
}

/// libass's message callback; the va_list is passed on as a pointer on
/// the systems plaa runs on, and this one ignores everything.
type Message = unsafe extern "C" fn(c_int, *const c_char, *mut c_void, *mut c_void);

/// libass's functions, as loaded.
struct Functions {
    _library: libloading::Library,
    library_version: unsafe extern "C" fn() -> c_int,
    library_init: unsafe extern "C" fn() -> *mut c_void,
    library_done: unsafe extern "C" fn(*mut c_void),
    set_message_cb: unsafe extern "C" fn(*mut c_void, Option<Message>, *mut c_void),
    add_font: unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char, c_int),
    renderer_init: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    renderer_done: unsafe extern "C" fn(*mut c_void),
    set_frame_size: unsafe extern "C" fn(*mut c_void, c_int, c_int),
    set_storage_size: unsafe extern "C" fn(*mut c_void, c_int, c_int),
    set_fonts: unsafe extern "C" fn(
        *mut c_void,
        *const c_char,
        *const c_char,
        c_int,
        *const c_char,
        c_int,
    ),
    new_track: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
    read_memory:
        unsafe extern "C" fn(*mut c_void, *mut c_char, usize, *const c_char) -> *mut c_void,
    free_track: unsafe extern "C" fn(*mut c_void),
    process_codec_private: unsafe extern "C" fn(*mut c_void, *const c_char, c_int),
    process_chunk: unsafe extern "C" fn(*mut c_void, *const c_char, c_int, c_longlong, c_longlong),
    render_frame:
        unsafe extern "C" fn(*mut c_void, *mut c_void, c_longlong, *mut c_int) -> *const AssImage,
}

/// libass, when the system has it.
fn functions() -> Option<&'static Functions> {
    static LOADED: OnceLock<Option<Functions>> = OnceLock::new();
    LOADED.get_or_init(load).as_ref()
}

/// Whether libass is there to draw ASS subtitles.
pub fn available() -> bool {
    functions().is_some()
}

/// libass's version, such as "0.17.3", where it is there.
pub fn version() -> Option<String> {
    let functions = functions()?;
    // SAFETY: ass_library_version takes nothing and returns a number.
    let number = unsafe { (functions.library_version)() } as u32;
    Some(version_name(number))
}

/// A version as libass numbers it, 0x01703000 for 0.17.3: the major in
/// the top four bits, then the minor and the patch in two hexadecimal
/// digits each that read as decimal ones.
fn version_name(number: u32) -> String {
    format!(
        "{}.{:x}.{:x}",
        number >> 28,
        (number >> 20) & 0xff,
        (number >> 12) & 0xff
    )
}

fn load() -> Option<Functions> {
    NAMES.iter().find_map(|name| {
        // SAFETY: loading libass runs its initializers, which do nothing
        // harmful; each symbol is looked up with the signature its header
        // gives it, and the library is kept loaded beside them.
        unsafe {
            let library = libloading::Library::new(*name).ok()?;
            macro_rules! symbol {
                ($name:literal) => {
                    *library.get(concat!($name, "\0").as_bytes()).ok()?
                };
            }
            Some(Functions {
                library_version: symbol!("ass_library_version"),
                library_init: symbol!("ass_library_init"),
                library_done: symbol!("ass_library_done"),
                set_message_cb: symbol!("ass_set_message_cb"),
                add_font: symbol!("ass_add_font"),
                renderer_init: symbol!("ass_renderer_init"),
                renderer_done: symbol!("ass_renderer_done"),
                set_frame_size: symbol!("ass_set_frame_size"),
                set_storage_size: symbol!("ass_set_storage_size"),
                set_fonts: symbol!("ass_set_fonts"),
                new_track: symbol!("ass_new_track"),
                read_memory: symbol!("ass_read_memory"),
                free_track: symbol!("ass_free_track"),
                process_codec_private: symbol!("ass_process_codec_private"),
                process_chunk: symbol!("ass_process_chunk"),
                render_frame: symbol!("ass_render_frame"),
                _library: library,
            })
        }
    })
}

/// Keeps libass's notes about fonts and the like off the terminal.
unsafe extern "C" fn quiet(_: c_int, _: *const c_char, _: *mut c_void, _: *mut c_void) {}

/// libass's autodetected font provider: fontconfig, Core Text or
/// DirectWrite.
const FONT_PROVIDER_AUTODETECT: c_int = 1;

/// A track of ASS subtitles and what draws it.
pub struct AssTrack {
    functions: &'static Functions,
    library: *mut c_void,
    renderer: *mut c_void,
    track: *mut c_void,
    canvas: (u32, u32),
    /// The last picture drawn, given again while nothing changes.
    last: Option<Arc<SubtitlePicture>>,
}

// SAFETY: libass's objects belong to this track alone, which is used
// behind a lock, by one thread at a time.
unsafe impl Send for AssTrack {}

impl AssTrack {
    /// A track from a stream's header, with the file's fonts: names and
    /// data. It is drawn in a `canvas` sized frame over a `video` sized
    /// picture.
    pub fn new(
        header: &[u8],
        fonts: &[(String, Vec<u8>)],
        canvas: (u32, u32),
        video: (u32, u32),
    ) -> Option<Self> {
        let mut track = Self::start(fonts, canvas, video)?;
        // SAFETY: a new track takes its header as bytes of the length
        // given.
        unsafe {
            track.track = (track.functions.new_track)(track.library);
            if track.track.is_null() {
                return None;
            }
            (track.functions.process_codec_private)(
                track.track,
                header.as_ptr().cast(),
                c_int::try_from(header.len()).ok()?,
            );
        }
        Some(track)
    }

    /// A whole `.ass` or `.ssa` file's track.
    pub fn read(contents: &[u8], canvas: (u32, u32), video: (u32, u32)) -> Option<Self> {
        let mut track = Self::start(&[], canvas, video)?;
        let mut buffer = contents.to_vec();
        // SAFETY: libass reads the buffer, of the length given, and may
        // write to it while it parses; it keeps no pointer into it.
        unsafe {
            track.track = (track.functions.read_memory)(
                track.library,
                buffer.as_mut_ptr().cast(),
                buffer.len(),
                std::ptr::null(),
            );
        }
        (!track.track.is_null()).then_some(track)
    }

    fn start(fonts: &[(String, Vec<u8>)], canvas: (u32, u32), video: (u32, u32)) -> Option<Self> {
        let functions = functions()?;
        let family = CString::new("sans-serif").ok()?;
        // SAFETY: each call gets the objects libass just made; fonts are
        // copied in, by name and data of the length given.
        unsafe {
            let library = (functions.library_init)();
            if library.is_null() {
                return None;
            }
            (functions.set_message_cb)(library, Some(quiet), std::ptr::null_mut());
            for (name, data) in fonts {
                let (Ok(name), Ok(size)) =
                    (CString::new(name.as_str()), c_int::try_from(data.len()))
                else {
                    continue;
                };
                (functions.add_font)(library, name.as_ptr(), data.as_ptr().cast(), size);
            }
            let renderer = (functions.renderer_init)(library);
            if renderer.is_null() {
                (functions.library_done)(library);
                return None;
            }
            let size = |(width, height): (u32, u32)| {
                (
                    c_int::try_from(width).unwrap_or(c_int::MAX),
                    c_int::try_from(height).unwrap_or(c_int::MAX),
                )
            };
            let (width, height) = size(canvas);
            (functions.set_frame_size)(renderer, width, height);
            let (width, height) = size(video);
            (functions.set_storage_size)(renderer, width, height);
            (functions.set_fonts)(
                renderer,
                std::ptr::null(),
                family.as_ptr(),
                FONT_PROVIDER_AUTODETECT,
                std::ptr::null(),
                1,
            );
            Some(Self {
                functions,
                library,
                renderer,
                track: std::ptr::null_mut(),
                canvas,
                last: None,
            })
        }
    }

    /// Adds an event as a stream's packet holds it, starting and lasting
    /// as many milliseconds.
    pub fn add(&mut self, event: &[u8], start: i64, duration: i64) {
        let Ok(size) = c_int::try_from(event.len()) else {
            return;
        };
        // SAFETY: the track is libass's own; the event is bytes of the
        // length given, copied by libass.
        unsafe {
            (self.functions.process_chunk)(
                self.track,
                event.as_ptr().cast(),
                size,
                start,
                duration,
            );
        }
    }

    /// The subtitles at `seconds` as one picture, none when nothing
    /// shows.
    pub fn render(&mut self, seconds: f64) -> Option<Arc<SubtitlePicture>> {
        let mut changed: c_int = 0;
        // SAFETY: renderer and track are libass's own. The images it
        // returns live until the next render, and each bitmap holds `h`
        // rows of `w` bytes, `stride` apart.
        let images = unsafe {
            let mut image = (self.functions.render_frame)(
                self.renderer,
                self.track,
                (seconds * 1000.0) as c_longlong,
                &mut changed,
            );
            if changed == 0 {
                return self.last.clone();
            }
            let mut images = Vec::new();
            while let Some(current) = image.as_ref() {
                if current.w > 0 && current.h > 0 && !current.bitmap.is_null() {
                    let (width, height) = (current.w as usize, current.h as usize);
                    let stride = current.stride.max(current.w) as usize;
                    let bitmap =
                        std::slice::from_raw_parts(current.bitmap, stride * (height - 1) + width);
                    images.push(Layer {
                        x: current.dst_x,
                        y: current.dst_y,
                        width,
                        height,
                        stride,
                        bitmap,
                        color: current.color,
                    });
                }
                image = current.next;
            }
            composite(&images, self.canvas)
        };
        self.last = images.map(Arc::new);
        self.last.clone()
    }
}

impl Drop for AssTrack {
    fn drop(&mut self) {
        // SAFETY: each object is libass's own and freed once, the track
        // and renderer before the library they came from.
        unsafe {
            if !self.track.is_null() {
                (self.functions.free_track)(self.track);
            }
            (self.functions.renderer_done)(self.renderer);
            (self.functions.library_done)(self.library);
        }
    }
}

/// One of libass's bitmaps, where it goes and its color.
struct Layer<'a> {
    x: c_int,
    y: c_int,
    width: usize,
    height: usize,
    stride: usize,
    bitmap: &'a [u8],
    color: u32,
}

/// libass's bitmaps, back to front, laid over one another as one picture
/// over the box they share, within `canvas`.
fn composite(layers: &[Layer], canvas: (u32, u32)) -> Option<SubtitlePicture> {
    let (canvas_width, canvas_height) = (canvas.0 as i64, canvas.1 as i64);
    let left = layers.iter().map(|layer| i64::from(layer.x)).min()?.max(0);
    let top = layers.iter().map(|layer| i64::from(layer.y)).min()?.max(0);
    let right = layers
        .iter()
        .map(|layer| i64::from(layer.x) + layer.width as i64)
        .max()?
        .min(canvas_width);
    let bottom = layers
        .iter()
        .map(|layer| i64::from(layer.y) + layer.height as i64)
        .max()?
        .min(canvas_height);
    if right <= left || bottom <= top {
        return None;
    }
    let (width, height) = ((right - left) as usize, (bottom - top) as usize);
    let mut rgba = vec![0u8; width * height * 4];
    for layer in layers {
        let [red, green, blue, transparency] = layer.color.to_be_bytes();
        let opacity = 255 - u32::from(transparency);
        for row in 0..layer.height {
            let y = i64::from(layer.y) + row as i64 - top;
            if !(0..height as i64).contains(&y) {
                continue;
            }
            for column in 0..layer.width {
                let x = i64::from(layer.x) + column as i64 - left;
                if !(0..width as i64).contains(&x) {
                    continue;
                }
                let coverage = u32::from(layer.bitmap[row * layer.stride + column]);
                let alpha = coverage * opacity / 255;
                if alpha == 0 {
                    continue;
                }
                let at = (y as usize * width + x as usize) * 4;
                let pixel = &mut rgba[at..at + 4];
                // Over what is there, in straight alpha.
                let below = u32::from(pixel[3]) * (255 - alpha) / 255;
                let out = alpha + below;
                for (channel, source) in pixel[..3].iter_mut().zip([red, green, blue]) {
                    *channel = ((u32::from(source) * alpha + u32::from(*channel) * below)
                        / out.max(1)) as u8;
                }
                pixel[3] = out as u8;
            }
        }
    }
    Some(SubtitlePicture::new(
        canvas,
        left as u32,
        top as u32,
        width as u32,
        height as u32,
        rgba,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_read_as_libass_writes_them() {
        assert_eq!(version_name(0x0170_3000), "0.17.3");
        assert_eq!(version_name(0x0160_0000), "0.16.0");
        assert_eq!(version_name(0x0140_1000), "0.14.1");
    }

    #[test]
    fn bitmaps_lay_over_one_another_in_their_colors() {
        // A half covered red pixel under a fully covered, half see-through
        // blue one, a pixel to the right of it.
        let red = [128u8];
        let blue = [255u8];
        let layers = [
            Layer {
                x: 10,
                y: 20,
                width: 1,
                height: 1,
                stride: 1,
                bitmap: &red,
                color: 0xff00_0000,
            },
            Layer {
                x: 11,
                y: 20,
                width: 1,
                height: 1,
                stride: 1,
                bitmap: &blue,
                color: 0x0000_ff80,
            },
        ];
        let picture = composite(&layers, (100, 100)).expect("a picture");
        assert_eq!(
            (picture.x, picture.y, picture.width, picture.height),
            (10, 20, 2, 1)
        );
        assert_eq!(&picture.rgba[..4], &[255, 0, 0, 128]);
        assert_eq!(&picture.rgba[4..], &[0, 0, 255, 127]);
    }
}
