//! What plaa plays, on its way to the talker: two channels as the audio
//! output hands them to its device, at the device's rate, into a buffer
//! the talker drains on gPTP time, steering a resampler between the two
//! clocks by how full it keeps.

use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, PoisonError};

use avb_stream::audio::{FrameBuffer, SharedBuffer};

use crate::runtime::Input;

/// The channels plaa plays in, which the stream's first channels carry.
pub const CHANNELS: u16 = 2;

/// The rate assumed until the output says its own.
const FIRST_RATE: u32 = 48_000;

/// Where what plays goes, to be sent.
pub struct Sound {
    inner: Mutex<Inner>,
    /// Told when the rate changes, so the talker takes the new buffer.
    changed: Mutex<Option<Sender<Input>>>,
}

/// The buffer, and room to spread stereo over more channels in.
struct Inner {
    shared: SharedBuffer,
    spread: Vec<f32>,
}

/// A buffer for `channels` at `rate`, holding 10 ms beyond the device's
/// chunks, as for an audio device.
fn buffer(rate: u32, channels: u16) -> SharedBuffer {
    SharedBuffer {
        buffer: Arc::new(FrameBuffer::new(channels, rate as usize / 100)),
        channels,
        rate,
    }
}

impl Sound {
    pub(crate) fn new() -> Self {
        Self {
            inner: Mutex::new(Inner {
                shared: buffer(FIRST_RATE, CHANNELS),
                spread: Vec::new(),
            }),
            changed: Mutex::new(None),
        }
    }

    fn inner(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub(crate) fn tell(&self, sender: Sender<Input>) {
        *self.changed.lock().unwrap_or_else(PoisonError::into_inner) = Some(sender);
    }

    /// The buffer the talker takes the frames from.
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    pub(crate) fn buffer(&self) -> SharedBuffer {
        self.inner().shared.clone()
    }

    /// Sends in `channels` from now on, a new buffer taking the frames.
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    pub(crate) fn set_channels(&self, channels: u16) {
        let mut inner = self.inner();
        if inner.shared.channels != channels {
            inner.shared = buffer(inner.shared.rate, channels.max(CHANNELS));
        }
    }

    /// Takes interleaved stereo frames at `rate`, as the output hands them
    /// to its device, from the device's thread.
    pub fn take(&self, frames: &[f32], rate: u32) {
        let mut inner = self.inner();
        let inner = &mut *inner;
        if rate > 0 && inner.shared.rate != rate {
            inner.shared = buffer(rate, inner.shared.channels);
            if let Some(sender) = &*self.changed.lock().unwrap_or_else(PoisonError::into_inner) {
                let _ = sender.send(Input::SoundRate);
            }
        }
        let channels = usize::from(inner.shared.channels);
        if channels == usize::from(CHANNELS) {
            inner.shared.buffer.device_push(frames);
            return;
        }
        inner.spread.clear();
        for frame in frames.as_chunks::<2>().0 {
            inner.spread.extend_from_slice(frame);
            inner
                .spread
                .extend(std::iter::repeat_n(0.0, channels - usize::from(CHANNELS)));
        }
        inner.shared.buffer.device_push(&inner.spread);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_rate_makes_a_new_buffer_and_says_so() {
        let sound = Sound::new();
        let (sender, receiver) = std::sync::mpsc::channel();
        sound.tell(sender);
        sound.take(&[0.5; 20], 48_000);
        assert!(receiver.try_recv().is_err(), "the same rate");
        let first = sound.buffer();
        assert_eq!(first.buffer.pushed(), 10);
        sound.take(&[0.5; 20], 44_100);
        assert!(matches!(receiver.try_recv(), Ok(Input::SoundRate)));
        let second = sound.buffer();
        assert_eq!(second.rate, 44_100);
        assert_ne!(first, second);
        assert_eq!(second.buffer.pushed(), 10);
    }

    #[test]
    fn stereo_goes_out_in_the_first_of_eight() {
        let sound = Sound::new();
        sound.set_channels(8);
        sound.take(&[0.25, -0.25, 0.5, -0.5], 48_000);
        let shared = sound.buffer();
        assert_eq!(shared.channels, 8);
        let mut out = [1.0f32; 16];
        shared.buffer.pull(&mut out);
        assert_eq!(&out[..8], &[0.25, -0.25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(&out[8..10], &[0.5, -0.5]);
    }
}
