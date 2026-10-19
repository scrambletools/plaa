//! Recording sound or the screen in a window of its own: opening it, its
//! controls, and playing the recording once it is saved.

use iced::{Element, Size, Task, window};
use plaa_media::export::{Format, Sound};
use plaa_media::record::{Microphone, RecordEvent};
use plaa_media::{Quality, Source};

use super::{Content, Message, Opening, Plaa};
use crate::External;
use crate::playlist::Playlist;
use crate::recorder::{self, Recorder, RecorderControl};
use crate::shortcuts::Action;

/// The recording window's size.
const RECORDER_SIZE: Size = Size::new(440.0, 380.0);
/// The screen recording window's size, with room for what is shared.
#[cfg(target_os = "linux")]
const SCREEN_RECORDER_SIZE: Size = Size::new(440.0, 440.0);

impl Plaa {
    /// Opens a recording window: in place of window `id`'s start view, else
    /// a new one.
    pub(super) fn open_recorder(&mut self, id: window::Id) -> Task<Message> {
        let recorder = Recorder::open(self.settings.microphone.as_deref());
        self.show_recorder(id, recorder, RECORDER_SIZE)
    }

    /// Shows `recorder` in window `id` if it shows the start view or a
    /// recorder that is not recording, else in a new window.
    fn show_recorder(&mut self, id: window::Id, recorder: Recorder, size: Size) -> Task<Message> {
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
            return window::resize(id, size);
        }
        let (opened, task) = self.new_window(size);
        if let Some(window) = self.windows.get_mut(&opened) {
            window.content = Content::Recorder(recorder);
        }
        task
    }

    /// Asks the system what screen or window to record, for window `id`.
    pub(super) fn open_screen_recorder(&mut self, id: window::Id) -> Task<Message> {
        #[cfg(target_os = "linux")]
        return Task::perform(crate::screen_share::share(), move |result| {
            Message::ScreenShared(id, result)
        });
        #[cfg(not(target_os = "linux"))]
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
            Some((fd, node)) => plaa_media::screen::ScreenCapture::open(fd, node),
            None => Err(String::new()),
        };
        let screen = recorder::Screen {
            capture,
            window: sharing.window,
            _sharing: sharing,
        };
        let sound = self
            .settings
            .screen_sound
            .then_some(self.settings.microphone.as_deref());
        let recorder = Recorder::screen(screen, sound);
        self.show_recorder(id, recorder, SCREEN_RECORDER_SIZE)
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
            _ => {}
        }
        let sound = self.recording_sound();
        let video = self.recording_video();
        let hardware = self.settings.hardware_decoding;
        let folder = self.settings.recordings.clone();
        let Some(window) = self.windows.get_mut(&id) else {
            return Task::none();
        };
        let Content::Recorder(recorder) = &mut window.content else {
            return Task::none();
        };
        match control {
            RecorderControl::Toggle => {
                let started = if recorder.screen.is_some() {
                    let name = format!("{}.mp4", crate::fl!("record-screen-file-name"));
                    let microphone = recorder
                        .microphone
                        .as_ref()
                        .and_then(|microphone| microphone.as_ref().ok());
                    let Some(capture) = recorder
                        .screen
                        .as_mut()
                        .and_then(|screen| screen.capture.as_mut().ok())
                    else {
                        return Task::none();
                    };
                    if capture.is_recording() {
                        capture.stop();
                        recorder.saving = true;
                        return Task::none();
                    }
                    std::fs::create_dir_all(&folder)
                        .map_err(|error| error.to_string())
                        .and_then(|()| {
                            let destination = crate::screenshot::unused(&folder.join(name));
                            capture.record(
                                microphone,
                                (video, hardware),
                                destination,
                                move |event| crate::post(External::Recorded(id, event)),
                            )
                        })
                } else {
                    let Some(Ok(microphone)) = &mut recorder.microphone else {
                        return Task::none();
                    };
                    if microphone.is_recording() {
                        microphone.stop();
                        recorder.saving = true;
                        return Task::none();
                    }
                    let name =
                        format!("{}.{}", crate::fl!("record-file-name"), sound.0.extension());
                    std::fs::create_dir_all(&folder)
                        .map_err(|error| error.to_string())
                        .and_then(|()| {
                            let destination = crate::screenshot::unused(&folder.join(name));
                            microphone.record(destination, sound, move |event| {
                                crate::post(External::Recorded(id, event));
                            })
                        })
                };
                if let Err(error) = started {
                    self.notify(id, crate::fl!("notice-record-failed", error = error));
                }
            }
            RecorderControl::Microphone(device) => {
                if recorder.is_recording() || recorder.saving {
                    return Task::none();
                }
                // The old device closes before the new one opens.
                recorder.microphone = None;
                recorder.microphone = Some(Microphone::open(device.as_deref()));
                recorder.levels.clear();
                if recorder.screen.is_some() {
                    self.settings.screen_sound = true;
                }
                self.settings.microphone = device;
                self.save_settings();
            }
            RecorderControl::Silent => {
                if recorder.is_recording() || recorder.saving {
                    return Task::none();
                }
                recorder.microphone = None;
                recorder.levels.clear();
                self.settings.screen_sound = false;
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
            RecorderControl::Video(format) => {
                self.settings.screen_format = format.key().to_owned();
                self.save_settings();
            }
            RecorderControl::Settings | RecorderControl::Close | RecorderControl::Reshare => {}
        }
        Task::none()
    }

    /// What window `id`'s recording says: once saved, the window plays it.
    pub(super) fn recorded(&mut self, id: window::Id, event: RecordEvent) -> Task<Message> {
        let RecordEvent::Finished(result) = event;
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
        Task::none()
    }
}
