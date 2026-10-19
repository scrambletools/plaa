//! The few FFmpeg calls the safe wrapper lacks: hardware decoding, moving
//! a decoded picture out of graphics memory, and a stream's rotation.
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
    // opened; the codec pointer FFmpeg returns is static.
    unsafe {
        let context = context.as_mut_ptr();
        let codec = ffi::avcodec_find_decoder((*context).codec_id);
        if codec.is_null() {
            return None;
        }
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
            (*context).hw_device_ctx = reference;
            // The format to pick, read back by `choose_format`.
            (*context).opaque = pixel as i32 as isize as *mut std::ffi::c_void;
            (*context).get_format = Some(choose_format);
            let name = ffi::av_hwdevice_get_type_name(device);
            return Some(if name.is_null() {
                "hardware"
            } else {
                // FFmpeg's names are static strings.
                std::ffi::CStr::from_ptr(name)
                    .to_str()
                    .unwrap_or("hardware")
            });
        }
        None
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

/// Copies a frame from graphics memory into memory, with its timing and
/// colors.
pub fn download(frame: &ffmpeg::frame::Video) -> Result<ffmpeg::frame::Video, ffmpeg::Error> {
    let mut copy = ffmpeg::frame::Video::empty();
    // SAFETY: both frames are valid; FFmpeg allocates the copy's buffers
    // in the card's preferred memory format.
    unsafe {
        let result = ffi::av_hwframe_transfer_data(copy.as_mut_ptr(), frame.as_ptr(), 0);
        if result < 0 {
            return Err(ffmpeg::Error::from(result));
        }
        let result = ffi::av_frame_copy_props(copy.as_mut_ptr(), frame.as_ptr());
        if result < 0 {
            return Err(ffmpeg::Error::from(result));
        }
    }
    Ok(copy)
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

/// A channel layout's name, such as "stereo" or "5.1(side)".
pub fn describe_layout(layout: &ffmpeg::ChannelLayout) -> String {
    let mut buffer = [0u8; 64];
    // SAFETY: FFmpeg writes at most `buffer.len()` bytes, ending in a
    // zero, and returns how many it needed.
    let written = unsafe {
        ffi::av_channel_layout_describe(&layout.0, buffer.as_mut_ptr().cast(), buffer.len())
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
