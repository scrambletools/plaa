//! Recording sound or the screen in a window of its own: opening it, its
//! controls, and playing the recording once it is saved.

use std::time::Duration;

use iced::{Element, Size, Task, window};
use plaa_media::capture::{Capture, VideoEncoding};
use plaa_media::export::{Format, Sound};
use plaa_media::record::{Microphone, RecordEvent};
use plaa_media::{Quality, Source};

use super::{Content, Message, Opening, Plaa};
use crate::External;
use crate::playlist::Playlist;
use crate::recorder::{self, Recorder, RecorderControl};
use crate::shortcuts::Action;

/// The recording window's size.
const RECORDER_SIZE: Size = Size::new(440.0, 460.0);
/// The screen recording window's size, with room for what is shared.
const SCREEN_RECORDER_SIZE: Size = Size::new(440.0, 540.0);
/// The camera recording window's size: the picture, and the controls
/// below it.
const CAMERA_RECORDER_SIZE: Size = Size::new(720.0, 760.0);
/// How long a window takes to close out of the way of a screen recording
/// before it begins.
const HIDING: Duration = Duration::from_millis(800);

/// A screen recording whose window closed so as not to be in it.
pub(super) struct Hidden {
    /// The window it was in, which its events name.
    window: window::Id,
    recorder: Box<Recorder>,
    /// The dot in the tray that stops it.
    #[cfg(target_os = "linux")]
    _tray: Option<crate::tray::Tray>,
}

/// The screens and windows plaa offers to record, where it lists them
/// itself, or why it cannot, as when macOS does not let it record.
#[cfg(any(windows, target_os = "macos"))]
fn offered_list() -> Result<Vec<recorder::Offered>, String> {
    #[cfg(windows)]
    return Ok(crate::screens_windows::offered());
    #[cfg(target_os = "macos")]
    {
        use plaa_media::capture::Target;
        let shareable = plaa_media::capture::shareable()?;
        let screens = shareable
            .iter()
            .filter(|shareable| matches!(shareable.target, Target::Screen(_)))
            .count();
        Ok(shareable
            .into_iter()
            .map(|shareable| match shareable.target {
                Target::Screen(number) => {
                    let shown = number + 1;
                    recorder::Offered {
                        id: u64::from(number),
                        window: false,
                        name: match (screens > 1, shareable.main) {
                            (false, _) => crate::fl!("record-screen-whole"),
                            (true, true) => crate::fl!("record-screen-main", number = shown),
                            (true, false) => crate::fl!("record-screen-number", number = shown),
                        },
                    }
                }
                Target::Window(id) => recorder::Offered {
                    id,
                    window: true,
                    name: shareable.name,
                },
            })
            .collect())
    }
}

/// What recording `offered` takes: its capture, or why it would not open.
#[cfg(any(windows, target_os = "macos"))]
fn offered_screen(offered: recorder::Offered) -> recorder::Screen {
    use plaa_media::capture::Target;
    let target = if offered.window {
        Target::Window(offered.id)
    } else {
        Target::Screen(offered.id as u32)
    };
    recorder::Screen {
        capture: Capture::open(target),
        window: offered.window,
        offered: Some(offered),
        camera: None,
    }
}

/// What recording `camera` takes: its pictures, or why they would not
/// come.
fn camera_screen(camera: plaa_media::capture::Camera) -> recorder::Screen {
    recorder::Screen {
        capture: Capture::camera(&camera),
        window: false,
        offered: None,
        camera: Some(camera),
        #[cfg(target_os = "linux")]
        _sharing: None,
    }
}

/// Whether a dot in the tray can stop a hidden recording.
fn tray_shows() -> bool {
    #[cfg(target_os = "linux")]
    return crate::tray::available();
    #[cfg(not(target_os = "linux"))]
    false
}

/// Starts recording `recorder`'s screen as `encoding` says, `after` from
/// now, saving it in `folder`; window `id` hears when it is saved.
fn record_screen(
    recorder: &mut Recorder,
    id: window::Id,
    encoding: VideoEncoding,
    folder: &std::path::Path,
    after: Duration,
) -> Result<(), String> {
    let camera = recorder
        .screen
        .as_ref()
        .is_some_and(|screen| screen.camera.is_some());
    let name = if camera {
        crate::fl!("record-camera-file-name")
    } else {
        crate::fl!("record-screen-file-name")
    };
    let name = format!("{name}.mp4");
    let microphone = recorder
        .microphone
        .as_ref()
        .and_then(|microphone| microphone.as_ref().ok());
    let Some(capture) = recorder
        .screen
        .as_mut()
        .and_then(|screen| screen.capture.as_mut().ok())
    else {
        return Ok(());
    };
    std::fs::create_dir_all(folder).map_err(|error| error.to_string())?;
    let destination = crate::screenshot::unused(&folder.join(name));
    capture.record(microphone, encoding, destination, after, move |event| {
        crate::post(External::Recorded(id, event))
    })
}

impl Plaa {
    /// Opens a recording window: in place of window `id`'s start view, else
    /// a new one.
    pub(super) fn open_recorder(&mut self, id: window::Id) -> Task<Message> {
        let recorder = Recorder::open(self.settings.microphone.as_deref());
        self.show_recorder(id, recorder, RECORDER_SIZE).1
    }

    /// How much louder the recording input is taken, in decibels, as
    /// Settings keep it for that input.
    fn gain(&self) -> f32 {
        let input = self.settings.microphone.as_deref().unwrap_or_default();
        self.settings.input_gains.get(input).copied().unwrap_or(0.0)
    }

    /// Shows `recorder` in window `id` if it shows the start view or a
    /// recorder that is not recording, else in a new window: the window,
    /// and the task opening or resizing it.
    fn show_recorder(
        &mut self,
        id: window::Id,
        recorder: Recorder,
        size: Size,
    ) -> (window::Id, Task<Message>) {
        if let Some(microphone) = recorder.microphone() {
            microphone.set_gain(self.gain());
            microphone.set_leveled(self.settings.level_automatically);
        }
        let recorder = Box::new(recorder);
        self.input_devices = plaa_media::record::input_devices();
        if let Some(window) = self.windows.get_mut(&id)
            && match &window.content {
                Content::Start => true,
                Content::Recorder(shown) => !shown.is_recording() && !shown.saving,
                Content::Player(_) => false,
            }
        {
            window.content = Content::Recorder(recorder);
            return (id, window::resize(id, size));
        }
        let (opened, task) = self.new_window(size);
        if let Some(window) = self.windows.get_mut(&opened) {
            window.content = Content::Recorder(recorder);
        }
        (opened, task)
    }

    /// Asks the system what screen or window to record, for window `id`;
    /// on Windows and macOS, a window opens to record the main screen,
    /// with a menu of the others and of windows.
    pub(super) fn open_screen_recorder(&mut self, id: window::Id) -> Task<Message> {
        #[cfg(target_os = "linux")]
        return Task::perform(crate::screen_share::share(), move |result| {
            Message::ScreenShared(id, result)
        });
        #[cfg(any(windows, target_os = "macos"))]
        {
            self.offers = match offered_list() {
                Ok(offers) => offers,
                Err(error) => {
                    self.notify(id, crate::fl!("notice-record-failed", error = error));
                    return Task::none();
                }
            };
            let Some(first) = self.offers.first().cloned() else {
                return Task::none();
            };
            let sound = self
                .settings
                .screen_sound
                .then_some(self.settings.microphone.as_deref());
            let recorder = Recorder::screen(offered_screen(first), sound);
            self.show_recorder(id, recorder, SCREEN_RECORDER_SIZE).1
        }
        #[cfg(not(any(target_os = "linux", windows, target_os = "macos")))]
        {
            let _ = id;
            Task::none()
        }
    }

    /// What the system shared for window `id` to record: a recording
    /// window opens for it.
    #[cfg(target_os = "linux")]
    pub(super) fn screen_shared(
        &mut self,
        id: window::Id,
        result: Result<Option<crate::screen_share::Shared>, String>,
    ) -> Task<Message> {
        let sharing = match result {
            Ok(Some(shared)) => shared.take(),
            Ok(None) => None,
            Err(error) => {
                self.notify(id, crate::fl!("notice-record-failed", error = error));
                None
            }
        };
        let Some(mut sharing) = sharing else {
            return Task::none();
        };
        let capture = match sharing.stream() {
            Some((fd, node)) => plaa_media::capture::Capture::open(fd, node),
            None => Err(String::new()),
        };
        let screen = recorder::Screen {
            capture,
            window: sharing.window,
            offered: None,
            camera: None,
            _sharing: Some(sharing),
        };
        let sound = self
            .settings
            .screen_sound
            .then_some(self.settings.microphone.as_deref());
        let recorder = Recorder::screen(screen, sound);
        self.show_recorder(id, recorder, SCREEN_RECORDER_SIZE).1
    }

    /// Opens a camera recording window: the camera Settings chose, or the
    /// first there is, showing its picture.
    pub(super) fn open_camera_recorder(&mut self, id: window::Id) -> Task<Message> {
        self.cameras = plaa_media::capture::cameras();
        let chosen = self
            .settings
            .camera
            .as_ref()
            .and_then(|wanted| self.cameras.iter().find(|camera| camera.id == *wanted))
            .or(self.cameras.first())
            .cloned();
        let Some(camera) = chosen else {
            self.notify(id, crate::fl!("notice-no-camera"));
            return Task::none();
        };
        let sound = self
            .settings
            .camera_sound
            .then_some(self.settings.microphone.as_deref());
        let recorder = Recorder::screen(camera_screen(camera), sound);
        self.show_recorder(id, recorder, CAMERA_RECORDER_SIZE).1
    }

    /// The format and quality recordings are made in, as Settings say,
    /// where this system can encode it.
    fn recording_sound(&self) -> (Sound, u32) {
        let sound = Sound::from_key(&self.settings.recording_format)
            .filter(|sound| sound.available())
            .unwrap_or(Sound::Aac);
        let quality = Some(self.settings.recording_quality)
            .filter(|quality| sound.qualities().contains(quality))
            .unwrap_or(sound.usual_quality());
        (sound, quality)
    }

    /// The format screen recordings are made in, as Settings say.
    fn recording_video(&self) -> Format {
        Format::from_key(&self.settings.screen_format)
            .filter(|format| Format::VIDEO.contains(format))
            .unwrap_or(Format::H264)
    }

    pub(super) fn recorder_view<'a>(
        &'a self,
        id: window::Id,
        recorder: &'a Recorder,
        close_button: bool,
    ) -> Element<'a, Message> {
        let (sound, quality) = self.recording_sound();
        recorder::view(
            recorder,
            &recorder::Context {
                devices: &self.input_devices,
                chosen: self.settings.microphone.as_deref(),
                sound,
                quality,
                video: self.recording_video(),
                rate: self.settings.screen_rate,
                gain: self.gain(),
                leveled: self.settings.level_automatically,
                offers: &self.offers,
                cameras: &self.cameras,
                close_button,
            },
        )
        .map(move |control| Message::Recorder(id, control))
    }

    pub(super) fn recorder_control(
        &mut self,
        id: window::Id,
        control: RecorderControl,
    ) -> Task<Message> {
        match control {
            RecorderControl::Settings => return self.perform(id, Action::Settings),
            RecorderControl::Close => return window::close(id),
            RecorderControl::Reshare => return self.open_screen_recorder(id),
            // Windows and cameras come and go: the menu lists them as
            // they are.
            RecorderControl::Choosing(true) => {
                let camera = self.windows.get(&id).is_some_and(|window| {
                    matches!(&window.content, Content::Recorder(recorder)
                        if recorder.screen.as_ref().is_some_and(|screen| screen.camera.is_some()))
                });
                if camera {
                    self.cameras = plaa_media::capture::cameras();
                }
                #[cfg(any(windows, target_os = "macos"))]
                if !camera && let Ok(offers) = offered_list() {
                    self.offers = offers;
                }
            }
            _ => {}
        }
        let sound = self.recording_sound();
        let video = self.recording_video();
        let (gains, leveled) = (
            self.settings.input_gains.clone(),
            self.settings.level_automatically,
        );
        let encoding = VideoEncoding {
            format: video,
            hardware: self.settings.hardware_decoding,
            rate: Some(self.settings.screen_rate).filter(|rate| *rate > 0),
        };
        let folder = self.settings.recordings.clone();
        // One recording hides at a time.
        let can_hide = self.hidden.is_none();
        let Some(window) = self.windows.get_mut(&id) else {
            return Task::none();
        };
        let Content::Recorder(recorder) = &mut window.content else {
            return Task::none();
        };
        match control {
            RecorderControl::Toggle | RecorderControl::Hide if recorder.screen.is_some() => {
                if recorder.saving {
                    return Task::none();
                }
                if let Some(capture) = recorder.capture_mut()
                    && capture.is_recording()
                {
                    capture.stop();
                    recorder.saving = true;
                    return Task::none();
                }
                // A whole screen would show this window: it closes while
                // recording, once a warning has said how to stop then.
                // Space agrees, as the button does.
                let whole_screen = recorder
                    .screen
                    .as_ref()
                    .is_some_and(|screen| !screen.window && screen.camera.is_none());
                // macOS leaves plaa's own windows out of a screen's
                // recording, so nothing needs to hide there.
                let hides = whole_screen && can_hide && cfg!(not(target_os = "macos"));
                let agreed =
                    recorder.warning.take().is_some() || matches!(control, RecorderControl::Hide);
                if hides && !agreed {
                    recorder.warning = Some(recorder::Warning { tray: tray_shows() });
                    return Task::none();
                }
                let after = if hides { HIDING } else { Duration::ZERO };
                match record_screen(recorder, id, encoding, &folder, after) {
                    Ok(()) if hides => {
                        recorder.hiding = true;
                        return window::close(id);
                    }
                    Ok(()) => {}
                    Err(error) => {
                        self.notify(id, crate::fl!("notice-record-failed", error = error));
                    }
                }
            }
            RecorderControl::Toggle => {
                let Some(Ok(microphone)) = &mut recorder.microphone else {
                    return Task::none();
                };
                if microphone.is_recording() {
                    microphone.stop();
                    recorder.saving = true;
                    return Task::none();
                }
                let name = format!("{}.{}", crate::fl!("record-file-name"), sound.0.extension());
                let started = std::fs::create_dir_all(&folder)
                    .map_err(|error| error.to_string())
                    .and_then(|()| {
                        let destination = crate::screenshot::unused(&folder.join(name));
                        microphone.record(destination, sound, move |event| {
                            crate::post(External::Recorded(id, event));
                        })
                    });
                if let Err(error) = started {
                    self.notify(id, crate::fl!("notice-record-failed", error = error));
                }
            }
            RecorderControl::Cancel => recorder.warning = None,
            RecorderControl::Choosing(open) => recorder.choosing = open,
            RecorderControl::Camera(camera) => {
                recorder.choosing = false;
                if recorder.is_recording() || recorder.saving {
                    return Task::none();
                }
                // The old camera closes before the new one opens.
                recorder.screen = None;
                recorder.preview = None;
                self.settings.camera = Some(camera.id.clone());
                recorder.screen = Some(camera_screen(camera));
                self.save_settings();
            }
            RecorderControl::Choose(offered) => {
                recorder.choosing = false;
                if recorder.is_recording() || recorder.saving {
                    return Task::none();
                }
                #[cfg(any(windows, target_os = "macos"))]
                {
                    recorder.screen = Some(offered_screen(offered));
                }
                #[cfg(not(any(windows, target_os = "macos")))]
                let _ = offered;
            }
            RecorderControl::Microphone(device) => {
                if recorder.is_recording() || recorder.saving {
                    return Task::none();
                }
                // The old device closes before the new one opens.
                recorder.microphone = None;
                recorder.microphone = Some(Microphone::open(device.as_deref()));
                recorder.levels.clear();
                if let Some(microphone) = recorder.microphone() {
                    let input = device.as_deref().unwrap_or_default();
                    microphone.set_gain(gains.get(input).copied().unwrap_or(0.0));
                    microphone.set_leveled(leveled);
                }
                match &recorder.screen {
                    Some(screen) if screen.camera.is_some() => self.settings.camera_sound = true,
                    Some(_) => self.settings.screen_sound = true,
                    None => {}
                }
                self.settings.microphone = device;
                self.save_settings();
            }
            RecorderControl::Silent => {
                if recorder.is_recording() || recorder.saving {
                    return Task::none();
                }
                let camera = recorder
                    .screen
                    .as_ref()
                    .is_some_and(|screen| screen.camera.is_some());
                recorder.microphone = None;
                recorder.levels.clear();
                if camera {
                    self.settings.camera_sound = false;
                } else {
                    self.settings.screen_sound = false;
                }
                self.save_settings();
            }
            RecorderControl::Format(chosen) => {
                self.settings.recording_format = chosen.key().to_owned();
                if !chosen
                    .qualities()
                    .contains(&self.settings.recording_quality)
                {
                    self.settings.recording_quality = chosen.usual_quality();
                }
                self.save_settings();
            }
            RecorderControl::Quality(quality) => {
                self.settings.recording_quality = quality;
                self.save_settings();
            }
            RecorderControl::Gain(decibels) => {
                if let Some(microphone) = recorder.microphone() {
                    microphone.set_gain(decibels);
                }
                let input = self.settings.microphone.clone().unwrap_or_default();
                self.settings.input_gains.insert(input, decibels);
            }
            RecorderControl::GainSet => self.save_settings(),
            RecorderControl::Leveled(on) => {
                if let Some(microphone) = recorder.microphone() {
                    microphone.set_leveled(on);
                }
                self.settings.level_automatically = on;
                self.save_settings();
            }
            RecorderControl::Video(format) => {
                self.settings.screen_format = format.key().to_owned();
                self.save_settings();
            }
            RecorderControl::Rate(rate) => {
                self.settings.screen_rate = rate;
                self.save_settings();
            }
            RecorderControl::Settings
            | RecorderControl::Close
            | RecorderControl::Reshare
            | RecorderControl::Hide => {}
        }
        Task::none()
    }

    /// Keeps recording window `id`'s recording once the window has
    /// closed, with a dot in the tray to stop it.
    pub(super) fn hide_recording(&mut self, id: window::Id, recorder: Box<Recorder>) {
        self.hidden = Some(Hidden {
            window: id,
            recorder,
            #[cfg(target_os = "linux")]
            _tray: crate::tray::Tray::show(
                crate::fl!("record-tray-title"),
                crate::fl!("record-tray-tip"),
                || crate::post(External::StopHidden),
            ),
        });
    }

    /// Stops the recording that hides, if there is one: its window shows
    /// again once it is saved.
    pub(super) fn stop_hidden(&mut self) -> bool {
        let Some(hidden) = &mut self.hidden else {
            return false;
        };
        if let Some(capture) = hidden.recorder.capture_mut()
            && capture.is_recording()
        {
            capture.stop();
            hidden.recorder.saving = true;
        }
        true
    }

    /// The recording that hides, if there is one.
    pub(super) fn hidden_recorder(&mut self) -> Option<&mut Recorder> {
        self.hidden.as_mut().map(|hidden| &mut *hidden.recorder)
    }

    /// What window `id`'s recording says: once saved, the window plays it.
    pub(super) fn recorded(&mut self, id: window::Id, event: RecordEvent) -> Task<Message> {
        let RecordEvent::Finished(result) = event;
        // A recording that hid comes back in a window of its own, which
        // then plays it.
        let mut id = id;
        let mut task = Task::none();
        if self
            .hidden
            .as_ref()
            .is_some_and(|hidden| hidden.window == id)
            && let Some(hidden) = self.hidden.take()
        {
            let mut recorder = hidden.recorder;
            recorder.hiding = false;
            (id, task) = self.show_recorder(id, *recorder, SCREEN_RECORDER_SIZE);
        }
        match result {
            Ok(path) => {
                let name = path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let folder = path
                    .parent()
                    .map(|folder| {
                        plaa_store::paths::abbreviate_home(folder)
                            .display()
                            .to_string()
                    })
                    .unwrap_or_default();
                // Only a window still recording turns into a player.
                let recording = self
                    .windows
                    .get(&id)
                    .is_some_and(|window| matches!(window.content, Content::Recorder(_)));
                if recording && let Some(list) = Playlist::new(vec![Source::File(path)], 0) {
                    self.open_player(
                        id,
                        list,
                        Opening {
                            quality: Quality::Best,
                            page: None,
                            start: None,
                            paused: true,
                            moving_on: false,
                        },
                    );
                }
                self.notify(
                    id,
                    crate::fl!("notice-recorded", name = name, folder = folder),
                );
            }
            Err(error) => {
                if let Some(window) = self.windows.get_mut(&id)
                    && let Content::Recorder(recorder) = &mut window.content
                {
                    recorder.saving = false;
                    if let Some(capture) = recorder.capture_mut() {
                        capture.ended();
                    } else if let Some(Ok(microphone)) = &mut recorder.microphone {
                        microphone.ended();
                    }
                }
                self.notify(id, crate::fl!("notice-record-failed", error = error));
            }
        }
        task
    }
}
