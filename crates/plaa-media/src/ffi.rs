//! The few FFmpeg calls the safe wrapper lacks: hardware decoding, moving
//! a decoded picture out of graphics memory, a stream's rotation and its
//! index of keyframes.
#![allow(unsafe_code)]

use ffmpeg::ffi;
use ffmpeg_next as ffmpeg;

/// A graphics card decoder FFmpeg can open, in the order to try them on
/// this system.
const DEVICES: &[ffi::AVHWDeviceType] = {
    use ffi::AVHWDeviceType::*;
    if cfg!(target_os = "linux") {
        &[
            AV_HWDEVICE_TYPE_VAAPI,
            AV_HWDEVICE_TYPE_CUDA,
            AV_HWDEVICE_TYPE_VULKAN,
        ]
    } else if cfg!(windows) {
        &[
            AV_HWDEVICE_TYPE_D3D11VA,
            AV_HWDEVICE_TYPE_CUDA,
            AV_HWDEVICE_TYPE_DXVA2,
        ]
    } else if cfg!(target_os = "macos") {
        &[AV_HWDEVICE_TYPE_VIDEOTOOLBOX]
    } else {
        &[]
    }
};

/// Sets `context` up to decode on the graphics card, before it is opened.
/// Returns the device's name, or `None` when no device opens for this
/// codec, which leaves the context decoding on the processor.
pub fn attach_hardware(context: &mut ffmpeg::codec::Context) -> Option<&'static str> {
    // SAFETY: `context` owns a valid AVCodecContext that has not been
    // opened; the codec pointer FFmpeg returns is static, and each device
    // reference is either handed to the context or released.
    unsafe {
        let context = context.as_mut_ptr();
        let codec = ffi::avcodec_find_decoder((*context).codec_id);
        if codec.is_null() {
            return None;
        }
        // NVIDIA's VAAPI driver, kept for when nothing else opens.
        let mut deferred: Option<(*mut ffi::AVBufferRef, ffi::AVPixelFormat)> = None;
        for &device in DEVICES {
            let Some(pixel) = hardware_format(codec, device) else {
                continue;
            };
            let mut reference = std::ptr::null_mut();
            let opened = ffi::av_hwdevice_ctx_create(
                &mut reference,
                device,
                std::ptr::null(),
                std::ptr::null_mut(),
                0,
            );
            if opened < 0 || reference.is_null() {
                continue;
            }
            #[cfg(target_os = "linux")]
            if device == ffi::AVHWDeviceType::AV_HWDEVICE_TYPE_VAAPI
                && deferred.is_none()
                && wraps_nvidia(reference)
            {
                deferred = Some((reference, pixel));
                continue;
            }
            if let Some((mut unused, _)) = deferred.take() {
                ffi::av_buffer_unref(&mut unused);
            }
            return Some(use_device(context, reference, pixel, device));
        }
        deferred.map(|(reference, pixel)| {
            use_device(
                context,
                reference,
                pixel,
                ffi::AVHWDeviceType::AV_HWDEVICE_TYPE_VAAPI,
            )
        })
    }
}

/// Hands `reference`, a device of type `device`, to `context`, to decode
/// to `pixel`. Returns the device's name.
unsafe fn use_device(
    context: *mut ffi::AVCodecContext,
    reference: *mut ffi::AVBufferRef,
    pixel: ffi::AVPixelFormat,
    device: ffi::AVHWDeviceType,
) -> &'static str {
    // SAFETY: the caller passes a valid, unopened context and a device
    // reference it gives up; FFmpeg's device names are static strings.
    unsafe {
        (*context).hw_device_ctx = reference;
        // The format to pick, read back by `choose_format`.
        (*context).opaque = pixel as i32 as isize as *mut std::ffi::c_void;
        (*context).get_format = Some(choose_format);
        let name = ffi::av_hwdevice_get_type_name(device);
        if name.is_null() {
            "hardware"
        } else {
            std::ffi::CStr::from_ptr(name)
                .to_str()
                .unwrap_or("hardware")
        }
    }
}

/// Whether a VAAPI device is NVIDIA's VAAPI driver. It decodes on the same
/// engine CUDA does but copies pictures out of graphics memory several
/// times slower, so CUDA goes first.
#[cfg(target_os = "linux")]
unsafe fn wraps_nvidia(device: *mut ffi::AVBufferRef) -> bool {
    type QueryVendor = unsafe extern "C" fn(*mut std::ffi::c_void) -> *const std::ffi::c_char;
    // SAFETY: libva, which FFmpeg loaded to open the device, has this
    // function with this signature; the device is a VAAPI one, whose
    // context holds its display; the vendor string is libva's own.
    unsafe {
        let symbol = libc::dlsym(libc::RTLD_DEFAULT, c"vaQueryVendorString".as_ptr());
        if symbol.is_null() {
            return false;
        }
        let query = std::mem::transmute::<*mut std::ffi::c_void, QueryVendor>(symbol);
        let context = (*device).data.cast::<ffi::AVHWDeviceContext>();
        let vaapi = (*context).hwctx.cast::<ffi::AVVAAPIDeviceContext>();
        let vendor = query((*vaapi).display);
        !vendor.is_null()
            && std::ffi::CStr::from_ptr(vendor)
                .to_string_lossy()
                .contains("NVDEC")
    }
}

/// The pixel format `codec` decodes to on `device`, if it can.
unsafe fn hardware_format(
    codec: *const ffi::AVCodec,
    device: ffi::AVHWDeviceType,
) -> Option<ffi::AVPixelFormat> {
    let mut index = 0;
    loop {
        // SAFETY: FFmpeg returns null past the last configuration.
        let config = unsafe { ffi::avcodec_get_hw_config(codec, index) };
        if config.is_null() {
            return None;
        }
        // SAFETY: not null, and static.
        let config = unsafe { &*config };
        let by_device = config.methods & ffi::AV_CODEC_HW_CONFIG_METHOD_HW_DEVICE_CTX as i32 != 0;
        if by_device && config.device_type == device {
            return Some(config.pix_fmt);
        }
        index += 1;
    }
}

/// Picks the hardware format `attach_hardware` chose when the decoder
/// offers it, else the first format it can decode to on the processor.
unsafe extern "C" fn choose_format(
    context: *mut ffi::AVCodecContext,
    formats: *const ffi::AVPixelFormat,
) -> ffi::AVPixelFormat {
    // SAFETY: FFmpeg passes its context and a list ending in
    // AV_PIX_FMT_NONE.
    unsafe {
        let wanted = (*context).opaque as isize as i32;
        let mut software = ffi::AVPixelFormat::AV_PIX_FMT_NONE;
        let mut cursor = formats;
        while *cursor != ffi::AVPixelFormat::AV_PIX_FMT_NONE {
            if *cursor as i32 == wanted {
                return *cursor;
            }
            let descriptor = ffi::av_pix_fmt_desc_get(*cursor);
            let hardware = !descriptor.is_null()
                && (*descriptor).flags & ffi::AV_PIX_FMT_FLAG_HWACCEL as u64 != 0;
            if software == ffi::AVPixelFormat::AV_PIX_FMT_NONE && !hardware {
                software = *cursor;
            }
            cursor = cursor.add(1);
        }
        software
    }
}

/// Whether `frame` is still in graphics memory.
pub fn is_hardware(frame: &ffmpeg::frame::Video) -> bool {
    // SAFETY: a valid frame; `hw_frames_ctx` is null for frames in memory.
    unsafe { !(*frame.as_ptr()).hw_frames_ctx.is_null() }
}

/// Rows and planes start on this many bytes, as in FFmpeg's own buffers.
const ALIGN: usize = 64;

/// ENOMEM, the same on every system plaa runs on.
const NO_MEMORY: ffmpeg::Error = ffmpeg::Error::Other { errno: 12 };

/// Memory that pictures are copied into from graphics memory, reused
/// rather than fresh for each one: at 8K, the system handing out fresh
/// memory a page at a time costs several times what the copy does.
pub struct Downloads {
    pool: *mut ffi::AVBufferPool,
    /// The format and size the pool's buffers fit.
    shape: (ffi::AVPixelFormat, i32, i32),
    linesizes: [i32; 4],
    offsets: [usize; 4],
}

// SAFETY: FFmpeg's buffer pools take a lock of their own, and buffers go
// back to them from whichever thread lets go of a picture.
unsafe impl Send for Downloads {}

impl Default for Downloads {
    fn default() -> Self {
        Self {
            pool: std::ptr::null_mut(),
            shape: (ffi::AVPixelFormat::AV_PIX_FMT_NONE, 0, 0),
            linesizes: [0; 4],
            offsets: [0; 4],
        }
    }
}

impl Downloads {
    /// Copies `frame` from graphics memory into memory, with its timing and
    /// colors.
    pub fn download(
        &mut self,
        frame: &ffmpeg::frame::Video,
    ) -> Result<ffmpeg::frame::Video, ffmpeg::Error> {
        let mut copy = ffmpeg::frame::Video::empty();
        // SAFETY: `frame` is a valid frame in graphics memory; the copy is
        // empty until it takes one of the pool's buffers, laid out for its
        // format and size by `reshape`, and FFmpeg only writes inside it.
        unsafe {
            let source = frame.as_ptr();
            let format = transfer_format(source)?;
            let shape = (format, (*source).width, (*source).height);
            if self.pool.is_null() || self.shape != shape {
                self.reshape(shape)?;
            }
            let buffer = ffi::av_buffer_pool_get(self.pool);
            if buffer.is_null() {
                return Err(NO_MEMORY);
            }
            let target = copy.as_mut_ptr();
            (*target).buf[0] = buffer;
            for plane in 0..4 {
                if self.linesizes[plane] > 0 {
                    (*target).data[plane] = (*buffer).data.add(self.offsets[plane]);
                    (*target).linesize[plane] = self.linesizes[plane];
                }
            }
            (*target).format = format as i32;
            (*target).width = shape.1;
            (*target).height = shape.2;
            let result = ffi::av_hwframe_transfer_data(target, source, 0);
            if result < 0 {
                return Err(ffmpeg::Error::from(result));
            }
            let result = ffi::av_frame_copy_props(target, source);
            if result < 0 {
                return Err(ffmpeg::Error::from(result));
            }
        }
        Ok(copy)
    }

    /// Starts a pool of buffers for pictures of `shape`; the last one goes
    /// once its buffers come back.
    unsafe fn reshape(
        &mut self,
        shape: (ffi::AVPixelFormat, i32, i32),
    ) -> Result<(), ffmpeg::Error> {
        let (format, width, height) = shape;
        let mut linesizes = [0i32; 4];
        // SAFETY: FFmpeg fills four line sizes and four plane sizes; the
        // old pool is released, not freed, while its buffers are out.
        let offsets = unsafe {
            let result = ffi::av_image_fill_linesizes(linesizes.as_mut_ptr(), format, width);
            if result < 0 {
                return Err(ffmpeg::Error::from(result));
            }
            let mut strides = [0isize; 4];
            for (linesize, stride) in linesizes.iter_mut().zip(&mut strides) {
                *linesize = (*linesize as usize).next_multiple_of(ALIGN) as i32;
                *stride = *linesize as isize;
            }
            let mut sizes = [0usize; 4];
            let result = ffi::av_image_fill_plane_sizes(
                sizes.as_mut_ptr(),
                format,
                height,
                strides.as_ptr(),
            );
            if result < 0 {
                return Err(ffmpeg::Error::from(result));
            }
            let mut offsets = [0usize; 4];
            let mut total = 0;
            for (offset, size) in offsets.iter_mut().zip(sizes) {
                *offset = total;
                total += size.next_multiple_of(ALIGN);
            }
            if !self.pool.is_null() {
                ffi::av_buffer_pool_uninit(&mut self.pool);
            }
            // Room past the end, as FFmpeg's readers may read a little over.
            self.pool = ffi::av_buffer_pool_init(total + ALIGN, None);
            if self.pool.is_null() {
                return Err(NO_MEMORY);
            }
            offsets
        };
        self.shape = shape;
        self.linesizes = linesizes;
        self.offsets = offsets;
        Ok(())
    }
}

impl Drop for Downloads {
    fn drop(&mut self) {
        if !self.pool.is_null() {
            // SAFETY: the pool is freed once the last buffer out comes back.
            unsafe { ffi::av_buffer_pool_uninit(&mut self.pool) };
        }
    }
}

/// The format a picture comes out of graphics memory in: the first the
/// card offers.
unsafe fn transfer_format(frame: *const ffi::AVFrame) -> Result<ffi::AVPixelFormat, ffmpeg::Error> {
    let mut formats = std::ptr::null_mut();
    // SAFETY: a frame in graphics memory has a frames context; FFmpeg
    // allocates the list, ending in AV_PIX_FMT_NONE, and it is freed here.
    unsafe {
        let result = ffi::av_hwframe_transfer_get_formats(
            (*frame).hw_frames_ctx,
            ffi::AVHWFrameTransferDirection::AV_HWFRAME_TRANSFER_DIRECTION_FROM,
            &mut formats,
            0,
        );
        if result < 0 {
            return Err(ffmpeg::Error::from(result));
        }
        if formats.is_null() {
            return Err(NO_MEMORY);
        }
        let format = *formats;
        ffi::av_free(formats.cast());
        if format == ffi::AVPixelFormat::AV_PIX_FMT_NONE {
            return Err(ffmpeg::Error::InvalidData);
        }
        Ok(format)
    }
}

/// The clockwise turn, in quarter turns, a stream's display matrix asks
/// for, as phones record portrait video.
pub fn quarter_turns(stream: &ffmpeg::format::stream::Stream) -> u8 {
    // SAFETY: the stream and its parameters are valid while `stream` is
    // borrowed; the side data FFmpeg finds is nine 32 bit integers.
    let degrees = unsafe {
        let parameters = (*stream.as_ptr()).codecpar;
        if parameters.is_null() {
            return 0;
        }
        let side = ffi::av_packet_side_data_get(
            (*parameters).coded_side_data,
            (*parameters).nb_coded_side_data,
            ffi::AVPacketSideDataType::AV_PKT_DATA_DISPLAYMATRIX,
        );
        if side.is_null() || (*side).size < 9 * 4 {
            return 0;
        }
        ffi::av_display_rotation_get((*side).data as *const i32)
    };
    if !degrees.is_finite() {
        return 0;
    }
    // FFmpeg gives the counterclockwise angle.
    let clockwise = (-degrees).rem_euclid(360.0);
    ((clockwise / 90.0).round() as u8) % 4
}

/// The picture a stream carries as cover art, such as an album's, still
/// encoded.
pub fn attached_picture(stream: &ffmpeg::format::stream::Stream) -> Option<Vec<u8>> {
    // SAFETY: `attached_pic` is a packet the demuxer filled for streams
    // with the attached picture disposition, valid while the stream is.
    unsafe {
        let packet = &(*stream.as_ptr()).attached_pic;
        if packet.data.is_null() || packet.size <= 0 {
            return None;
        }
        Some(std::slice::from_raw_parts(packet.data, packet.size as usize).to_vec())
    }
}

// Channel layouts are read from FFmpeg's own `ch_layout` fields, which
// every FFmpeg since 5.1 has, as the wrapper's layout type differs between
// FFmpeg versions.

/// The name of a decoder's channel layout, such as "stereo" or
/// "5.1(side)", or "" when it has none.
pub fn decoder_layout(decoder: &ffmpeg::decoder::Audio) -> String {
    // SAFETY: the decoder's context is valid while it is borrowed.
    unsafe { describe(&(*decoder.as_ptr()).ch_layout) }
}

/// The name of a frame's channel layout, as FFmpeg's filters take it: the
/// usual layout for its number of channels when the file gives none.
pub fn frame_layout(frame: &ffmpeg::frame::Audio) -> String {
    // SAFETY: the frame is valid while it is borrowed.
    let layout = unsafe { &(*frame.as_ptr()).ch_layout };
    if layout.order == ffi::AVChannelOrder::AV_CHANNEL_ORDER_UNSPEC {
        default_layout(layout.nb_channels)
    } else {
        // SAFETY: as above.
        unsafe { describe(layout) }
    }
}

/// The name of the usual layout for `channels` channels.
pub fn default_layout(channels: i32) -> String {
    // SAFETY: FFmpeg fills the zeroed layout, which is released after.
    unsafe {
        let mut layout: ffi::AVChannelLayout = std::mem::zeroed();
        ffi::av_channel_layout_default(&mut layout, channels);
        let name = describe(&layout);
        ffi::av_channel_layout_uninit(&mut layout);
        name
    }
}

/// `layout`'s name.
///
/// # Safety
/// `layout` must be a valid channel layout.
unsafe fn describe(layout: *const ffi::AVChannelLayout) -> String {
    let mut buffer = [0u8; 64];
    // SAFETY: FFmpeg writes at most `buffer.len()` bytes, ending in a
    // zero, and returns how many it needed.
    let written = unsafe {
        ffi::av_channel_layout_describe(layout, buffer.as_mut_ptr().cast(), buffer.len())
    };
    if written <= 0 {
        return String::new();
    }
    std::ffi::CStr::from_bytes_until_nul(&buffer)
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Seconds the file's timestamps start at, which plaa counts from.
pub fn start_time(input: &ffmpeg::format::context::Input) -> f64 {
    // SAFETY: a valid format context.
    let start = unsafe { (*input.as_ptr()).start_time };
    if start == ffi::AV_NOPTS_VALUE {
        0.0
    } else {
        start as f64 / f64::from(ffi::AV_TIME_BASE)
    }
}

/// The media time of the last keyframe at or before `seconds` in
/// `stream`, from the file's index, if it has one there. `start` is where
/// the file's timestamps start.
pub fn keyframe_before(
    input: &ffmpeg::format::context::Input,
    stream: usize,
    seconds: f64,
    start: f64,
) -> Option<f64> {
    let stream = input.stream(stream)?;
    let base = f64::from(stream.time_base());
    if base <= 0.0 {
        return None;
    }
    let timestamp = ((seconds + start) / base).floor() as i64;
    // SAFETY: the stream is valid while `input` is borrowed, and FFmpeg
    // only reads its index; the entry it returns lives in that index.
    let entry = unsafe {
        ffi::avformat_index_get_entry_from_timestamp(
            stream.as_ptr().cast_mut(),
            timestamp,
            ffi::AVSEEK_FLAG_BACKWARD,
        )
        .as_ref()
    }?;
    Some(entry.timestamp as f64 * base - start)
}

/// A bitmap subtitle rectangle's pixels as RGBA with straight alpha, from
/// its palette image.
pub fn bitmap_rgba(rect: &ffmpeg::codec::subtitle::Bitmap) -> Option<Vec<u8>> {
    let (width, height) = (rect.width() as usize, rect.height() as usize);
    if width == 0 || height == 0 || rect.colors() == 0 {
        return None;
    }
    // SAFETY: a bitmap rectangle's first plane holds `height` rows of
    // `width` palette indices, `linesize[0]` apart, and its second the
    // palette, `nb_colors` 32 bit ARGB entries; both live while `rect`
    // does.
    unsafe {
        let raw = rect.as_ptr();
        let indices = (*raw).data[0];
        let palette = (*raw).data[1].cast::<u32>();
        let stride = usize::try_from((*raw).linesize[0]).ok()?;
        if indices.is_null() || palette.is_null() || stride < width {
            return None;
        }
        let palette = std::slice::from_raw_parts(palette, rect.colors().min(256));
        let mut rgba = Vec::with_capacity(width * height * 4);
        for row in 0..height {
            let line = std::slice::from_raw_parts(indices.add(row * stride), width);
            for &index in line {
                let color = palette.get(usize::from(index)).copied().unwrap_or(0);
                rgba.extend_from_slice(&[
                    (color >> 16) as u8,
                    (color >> 8) as u8,
                    color as u8,
                    (color >> 24) as u8,
                ]);
            }
        }
        Some(rgba)
    }
}

/// The size a subtitle decoder says its pictures are placed in, when it
/// knows.
pub fn subtitle_canvas(decoder: &ffmpeg::decoder::Subtitle) -> Option<(u32, u32)> {
    // SAFETY: the decoder's context is valid while it is borrowed.
    let (width, height) = unsafe {
        let context = decoder.as_ptr();
        ((*context).width, (*context).height)
    };
    (width > 0 && height > 0).then_some((width as u32, height as u32))
}

/// A stream's extra data: an ASS track's header, an attached font's
/// bytes.
pub fn stream_extradata(stream: &ffmpeg::format::stream::Stream) -> Option<Vec<u8>> {
    // SAFETY: the stream's parameters live while it is borrowed, and their
    // extra data is `extradata_size` bytes when not null.
    unsafe {
        let parameters = (*stream.as_ptr()).codecpar;
        if parameters.is_null() || (*parameters).extradata.is_null() {
            return None;
        }
        let size = usize::try_from((*parameters).extradata_size).ok()?;
        Some(std::slice::from_raw_parts((*parameters).extradata, size).to_vec())
    }
}

/// FFmpeg's release, such as "n9.0.1", as it was built.
pub fn version() -> String {
    // SAFETY: FFmpeg returns a static string.
    unsafe {
        let info = ffi::av_version_info();
        if info.is_null() {
            return String::new();
        }
        std::ffi::CStr::from_ptr(info)
            .to_string_lossy()
            .into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copies_are_laid_out_as_ffmpeg_lays_out_pictures() {
        use ffi::AVPixelFormat::{AV_PIX_FMT_NV12, AV_PIX_FMT_P010LE};
        let mut downloads = Downloads::default();
        // SAFETY: only fills in the layout and starts a pool.
        unsafe { downloads.reshape((AV_PIX_FMT_P010LE, 7680, 4320)) }.expect("a pool");
        assert_eq!(downloads.linesizes, [15360, 15360, 0, 0]);
        assert_eq!(downloads.offsets[1], 15360 * 4320);
        // Rows round up to 64 bytes, and chroma takes the odd row.
        // SAFETY: as above; the first pool goes.
        unsafe { downloads.reshape((AV_PIX_FMT_NV12, 1919, 1081)) }.expect("a pool");
        assert_eq!(downloads.linesizes, [1920, 1920, 0, 0]);
        assert_eq!(downloads.offsets[1], 1920 * 1081);
    }
}
