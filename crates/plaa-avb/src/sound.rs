//! What plaa plays, on its way to the talker: two channels as the audio
//! output hands them to its device, at the device's rate, into a buffer
//! the talker drains on gPTP time, steering a resampler between the two
//! clocks by how full it keeps.

use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, PoisonError};

use avb_stream::audio::{FrameBuffer, SharedBuffer};

use crate::runtime::Input;

/// The channels the talker sends.
pub const CHANNELS: u16 = 2;

/// The rate assumed until the output says its own.
const FIRST_RATE: u32 = 48_000;

/// Where what plays goes, to be sent.
pub struct Sound {
    shared: Mutex<SharedBuffer>,
    /// Told when the rate changes, so the talker takes the new buffer.
    changed: Mutex<Option<Sender<Input>>>,
}

/// A buffer for frames at `rate`, holding 10 ms beyond the device's
/// chunks, as for an audio device.
fn buffer(rate: u32) -> SharedBuffer {
    SharedBuffer {
        buffer: Arc::new(FrameBuffer::new(CHANNELS, rate as usize / 100)),
        channels: CHANNELS,
        rate,
    }
}

impl Sound {
    pub(crate) fn new() -> Self {
        Self {
            shared: Mutex::new(buffer(FIRST_RATE)),
            changed: Mutex::new(None),
        }
    }

    pub(crate) fn tell(&self, sender: Sender<Input>) {
        *self.changed.lock().unwrap_or_else(PoisonError::into_inner) = Some(sender);
    }

    /// The buffer the talker takes the frames from.
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    pub(crate) fn buffer(&self) -> SharedBuffer {
        self.shared
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    /// Takes interleaved stereo frames at `rate`, as the output hands them
    /// to its device, from the device's thread.
    pub fn take(&self, frames: &[f32], rate: u32) {
        let target = {
            let mut shared = self.shared.lock().unwrap_or_else(PoisonError::into_inner);
            if rate > 0 && shared.rate != rate {
                *shared = buffer(rate);
                if let Some(sender) = &*self.changed.lock().unwrap_or_else(PoisonError::into_inner)
                {
                    let _ = sender.send(Input::SoundRate);
                }
            }
            shared.buffer.clone()
        };
        target.device_push(frames);
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
}
