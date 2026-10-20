//! The cameras macOS lists, through AVFoundation: built in, plugged in,
//! and iPhones as Continuity Camera. FFmpeg's avfoundation input opens
//! them by name.

#![allow(unsafe_code)]

use objc2_av_foundation::{
    AVCaptureDeviceDiscoverySession, AVCaptureDevicePosition,
    AVCaptureDeviceTypeBuiltInWideAngleCamera, AVCaptureDeviceTypeContinuityCamera,
    AVCaptureDeviceTypeExternal, AVMediaTypeVideo,
};
use objc2_foundation::NSArray;

use super::Camera;

pub fn cameras() -> Vec<Camera> {
    // SAFETY: AVFoundation's discovery of video devices, with its own
    // constants, and plain queries of what it found.
    unsafe {
        let Some(video) = AVMediaTypeVideo else {
            return Vec::new();
        };
        let types = NSArray::from_slice(&[
            AVCaptureDeviceTypeBuiltInWideAngleCamera,
            AVCaptureDeviceTypeExternal,
            AVCaptureDeviceTypeContinuityCamera,
        ]);
        let session =
            AVCaptureDeviceDiscoverySession::discoverySessionWithDeviceTypes_mediaType_position(
                &types,
                Some(video),
                AVCaptureDevicePosition::Unspecified,
            );
        session
            .devices()
            .iter()
            .map(|device| {
                let name = device.localizedName().to_string();
                Camera {
                    input: "avfoundation".to_owned(),
                    id: name.clone(),
                    name,
                }
            })
            .collect()
    }
}
