//! plaa's talker and listener on a Mac: the Mac's own AVB entity, which
//! macOS runs once AVB is on for the interface in Audio MIDI Setup's
//! Network Device Browser, as triib uses it. plaa's controller reaches the
//! entity through Apple's AudioVideoBridging framework
//! (`atdecc::blocking::Local`), and while its streams are connected, macOS
//! makes a Core Audio device of it: plaa plays what it plays into the
//! device's first two channels, and plays the stream its first input takes
//! from the device's first two input channels, the entity's clock following
//! that stream meanwhile.

#![allow(unsafe_code)]

use std::io;
use std::ptr::NonNull;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use atdecc::controller::Controller;
use atdecc::descriptor::{ClockSourceType, DescriptorType};
use atdecc::{EntityId, MacAddress};
use avb_stream::audio::{Input as AudioInput, Output, SharedBuffer, Sink, Source};
use objc2_core_audio::{
    AudioObjectGetPropertyData, AudioObjectGetPropertyDataSize, AudioObjectID,
    AudioObjectPropertyAddress, AudioObjectPropertySelector, kAudioDevicePropertyDeviceUID,
    kAudioHardwarePropertyDevices, kAudioObjectPropertyElementMain, kAudioObjectPropertyName,
    kAudioObjectPropertyScopeGlobal, kAudioObjectSystemObject,
};
use objc2_core_foundation::{CFRetained, CFString};

use crate::TalkerState;
use crate::sound::{CHANNELS, Sound};

/// How often it looks again whether AVB is on, for the entity, and for its
/// device.
const LOOK: Duration = Duration::from_secs(1);

/// The rate the device is fed at; it converts to its own.
const RATE: u32 = 48_000;

/// How often the feed hands the device what came in since.
const STEP: Duration = Duration::from_millis(5);

pub(crate) struct Host {
    interface: String,
    mac: MacAddress,
    sound: Arc<Sound>,
    /// The Mac's own entity, once the controller found it.
    entity: Option<EntityId>,
    /// Its Core Audio device, while macOS has one.
    device: Option<String>,
    feed: Option<Feed>,
    /// Plays the stream the entity's first input takes, while it takes
    /// one, into `into`.
    listen: Option<Feed>,
    /// Where that stream plays: a player's buffer, or the sound output
    /// where none.
    into: Option<SharedBuffer>,
    /// The device the listening feed would not open on, not tried again
    /// until it changes.
    listen_failed: Option<String>,
    /// The clock source the entity had before its clock followed the
    /// stream its input takes, to go back to after.
    clock_before: Option<u16>,
    looked: Option<Instant>,
    /// Why it cannot send: AVB is off, or the framework missing.
    blocked: Option<String>,
}

impl Host {
    pub(crate) fn new(interface: &str, mac: MacAddress, sound: Arc<Sound>) -> Self {
        Self {
            interface: interface.to_owned(),
            mac,
            sound,
            entity: None,
            device: None,
            feed: None,
            listen: None,
            into: None,
            listen_failed: None,
            clock_before: None,
            looked: None,
            blocked: None,
        }
    }

    pub(crate) fn entity_id(&self) -> Option<EntityId> {
        self.entity
    }

    /// Whether AVB is on for the interface, as last seen.
    pub(crate) fn avb_on(&self) -> bool {
        self.looked.is_some() && self.blocked.is_none()
    }

    pub(crate) fn state(&self) -> TalkerState {
        match (&self.blocked, &self.feed) {
            (Some(why), _) => TalkerState::Blocked(why.clone()),
            (None, Some(_)) => TalkerState::Streaming,
            (None, None) => TalkerState::Waiting,
        }
    }

    /// Looks again, each second, whether AVB is on, for the Mac's entity
    /// among those the controller found, and for its device, feeding it
    /// while it is there, and playing from it while the entity's first
    /// input takes a stream.
    pub(crate) fn turn(&mut self, controller: &mut Controller, now: atdecc::Instant) {
        if self.looked.is_some_and(|looked| looked.elapsed() < LOOK) {
            return;
        }
        self.looked = Some(Instant::now());
        self.blocked = match atdecc::blocking::avb_enabled(&self.interface) {
            Some(true) => None,
            Some(false) => Some(format!(
                "AVB is off on {}: turn it on in Audio MIDI Setup's Network Device Browser",
                self.interface
            )),
            None => Some("this Mac has no AVB framework".to_owned()),
        };
        if self.blocked.is_some() {
            self.entity = None;
            self.stop();
            return;
        }
        self.entity = controller
            .entities()
            .find(|entity| entity.mac == self.mac && entity.adp.talker_stream_sources > 0)
            .map(|entity| entity.entity_id());
        let device = self.entity.and_then(audio_device_of);
        if device != self.device {
            self.stop();
            self.device = device;
        }
        if self.feed.is_none()
            && let Some(device) = &self.device
        {
            self.sound.set_channels(CHANNELS);
            let source = Source::Shared(self.sound.buffer());
            let sink = Sink::Device {
                name: Some(device.clone()),
                first_channel: 0,
            };
            match Feed::start("plaa-host", source, sink) {
                Ok(feed) => self.feed = Some(feed),
                Err(error) => eprintln!("plaa: could not play into {device}: {error}"),
            }
        }
        let input = self
            .entity
            .and_then(|entity| bound_input(controller, entity));
        self.follow_clock(controller, now, input);
        match (&self.device, input.is_some()) {
            (Some(device), true) => {
                if self.listen.is_none() && self.listen_failed.as_ref() != Some(device) {
                    let source = Source::Device {
                        name: Some(device.clone()),
                        first_channel: 0,
                    };
                    let sink = match &self.into {
                        Some(buffer) => Sink::Shared(buffer.clone()),
                        None => Sink::Device {
                            name: None,
                            first_channel: 0,
                        },
                    };
                    match Feed::start("plaa-host-listen", source, sink) {
                        Ok(feed) => {
                            self.listen = Some(feed);
                            self.listen_failed = None;
                        }
                        Err(error) => {
                            eprintln!("plaa: could not listen to {device}: {error}");
                            self.listen_failed = Some(device.clone());
                        }
                    }
                }
            }
            _ => {
                self.listen = None;
                self.listen_failed = None;
            }
        }
    }

    /// Has the entity's clock follow the stream its `input` takes, as on
    /// its own clock the Mac's device leaves gaps in what it takes, and,
    /// once the input takes none, go back to the source it had.
    fn follow_clock(
        &mut self,
        controller: &mut Controller,
        now: atdecc::Instant,
        input: Option<u16>,
    ) {
        let Some(entity) = self.entity else {
            return;
        };
        let Some(model) = controller.model(entity) else {
            return;
        };
        let Some(domain) = model.clock_domains().next() else {
            return;
        };
        let (index, current) = (domain.index, domain.clock_source_index);
        let wanted = match input {
            Some(input) => model
                .clock_sources()
                .find(|source| {
                    source.clock_source_type == ClockSourceType::INPUT_STREAM
                        && source.location_type == DescriptorType::STREAM_INPUT
                        && source.location_index == input
                })
                .map(|source| source.index),
            None => self.clock_before,
        };
        let Some(wanted) = wanted else {
            return;
        };
        if input.is_some() {
            self.clock_before.get_or_insert(current);
        } else {
            self.clock_before = None;
        }
        if wanted != current {
            controller.set_clock_source(now, entity, index, wanted);
        }
    }

    /// Plays the stream the entity's input takes into `buffer` from now
    /// on, or on the sound output where none, starting again where it
    /// plays already.
    pub(crate) fn listen_into(&mut self, buffer: Option<SharedBuffer>) {
        if self.into == buffer {
            return;
        }
        self.into = buffer;
        self.listen = None;
        self.listen_failed = None;
        self.looked = None;
    }

    /// What plays arrives at another rate: the feed takes the new buffer.
    pub(crate) fn sound_changed(&mut self) {
        if self.feed.is_some() {
            self.feed = None;
            self.looked = None;
        }
    }

    fn stop(&mut self) {
        self.feed = None;
        self.listen = None;
        self.listen_failed = None;
    }
}

/// The entity's first audio stream input, while it takes a talker's
/// stream, as the controller last heard.
fn bound_input(controller: &Controller, entity: EntityId) -> Option<u16> {
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
struct Feed {
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Feed {
    fn start(name: &str, source: Source, sink: Sink) -> io::Result<Self> {
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

fn address(selector: AudioObjectPropertySelector) -> AudioObjectPropertyAddress {
    AudioObjectPropertyAddress {
        mSelector: selector,
        mScope: kAudioObjectPropertyScopeGlobal,
        mElement: kAudioObjectPropertyElementMain,
    }
}

/// Every Core Audio device.
fn devices() -> Vec<AudioObjectID> {
    let system = kAudioObjectSystemObject as AudioObjectID;
    let mut address = address(kAudioHardwarePropertyDevices);
    let mut size = 0u32;
    // SAFETY: an address and a size to write to.
    let status = unsafe {
        AudioObjectGetPropertyDataSize(
            system,
            NonNull::from(&mut address),
            0,
            std::ptr::null(),
            NonNull::from(&mut size),
        )
    };
    if status != 0 {
        return Vec::new();
    }
    let mut devices: Vec<AudioObjectID> =
        vec![0; size as usize / std::mem::size_of::<AudioObjectID>()];
    // SAFETY: room for `size` octets of device IDs.
    let status = unsafe {
        AudioObjectGetPropertyData(
            system,
            NonNull::from(&mut address),
            0,
            std::ptr::null(),
            NonNull::from(&mut size),
            NonNull::new_unchecked(devices.as_mut_ptr().cast()),
        )
    };
    if status != 0 {
        return Vec::new();
    }
    devices.truncate(size as usize / std::mem::size_of::<AudioObjectID>());
    devices
}

/// A device's text property, such as its name or UID.
fn text(device: AudioObjectID, selector: AudioObjectPropertySelector) -> Option<String> {
    let mut address = address(selector);
    let mut value: *const CFString = std::ptr::null();
    let mut size = std::mem::size_of::<*const CFString>() as u32;
    // SAFETY: room for the one CFStringRef the property gives, retained.
    let status = unsafe {
        AudioObjectGetPropertyData(
            device,
            NonNull::from(&mut address),
            0,
            std::ptr::null(),
            NonNull::from(&mut size),
            NonNull::from(&mut value).cast(),
        )
    };
    if status != 0 {
        return None;
    }
    // SAFETY: the property hands over a retained string, or none.
    let value = unsafe { CFRetained::from_raw(NonNull::new(value.cast_mut())?) };
    Some(value.to_string())
}

/// The name of the Core Audio device macOS made of AVB entity
/// `entity_id`, whose UID carries the entity's ID, as
/// "AAA:0xd111e597f5448000:0x0000:0x0000" does.
fn audio_device_of(entity_id: EntityId) -> Option<String> {
    let id = format!(":{entity_id}:").to_lowercase();
    devices()
        .into_iter()
        .find(|&device| {
            text(device, kAudioDevicePropertyDeviceUID)
                .is_some_and(|uid| uid.to_lowercase().contains(&id))
        })
        .and_then(|device| text(device, kAudioObjectPropertyName))
}
