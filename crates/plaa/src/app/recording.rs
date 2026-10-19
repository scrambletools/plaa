//! Recording sound in a window of its own: opening it, its controls, and
//! playing the recording once it is saved.

use iced::{Element, Size, Task, window};
use plaa_media::export::Sound;
use plaa_media::record::RecordEvent;
use plaa_media::{Quality, Source};

use super::{Content, Message, Opening, Plaa};
use crate::External;
use crate::playlist::Playlist;
use crate::recorder::{self, Recorder, RecorderControl};
use crate::shortcuts::Action;

/// The recording window's size.
const RECORDER_SIZE: Size = Size::new(440.0, 380.0);

impl Plaa {
    /// Opens a recording window: in place of window `id`'s start view, else
    /// a new one.
    pub(super) fn open_recorder(&mut self, id: window::Id) -> Task<Message> {
        let recorder = Box::new(Recorder::open(self.settings.microphone.as_deref()));
        self.input_devices = plaa_media::record::input_devices();
        if let Some(window) = self.windows.get_mut(&id)
            && matches!(window.content, Content::Start)
        {
            window.content = Content::Recorder(recorder);
            return window::resize(id, RECORDER_SIZE);
        }
        let (opened, task) = self.new_window(RECORDER_SIZE);
        if let Some(window) = self.windows.get_mut(&opened) {
            window.content = Content::Recorder(recorder);
        }
        task
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
            _ => {}
        }
        let sound = self.recording_sound();
        let folder = self.settings.recordings.clone();
        let Some(window) = self.windows.get_mut(&id) else {
            return Task::none();
        };
        let Content::Recorder(recorder) = &mut window.content else {
            return Task::none();
        };
        match control {
            RecorderControl::Toggle => {
                let Ok(microphone) = &mut recorder.microphone else {
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
            RecorderControl::Microphone(device) => {
                if recorder.is_recording() {
                    return Task::none();
                }
                // The old device closes before the new one opens.
                recorder.microphone = Err(String::new());
                recorder.microphone = Recorder::open(device.as_deref()).microphone;
                recorder.levels.clear();
                self.settings.microphone = device;
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
            RecorderControl::Settings | RecorderControl::Close => {}
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
                    if let Ok(microphone) = &mut recorder.microphone {
                        microphone.ended();
                    }
                }
                self.notify(id, crate::fl!("notice-record-failed", error = error));
            }
        }
        Task::none()
    }
}
