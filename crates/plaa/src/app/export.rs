//! Exporting a window's file, as edited: the dialog choosing the format
//! and size, or the sound's format and quality, the save dialog, the
//! export's progress over the window, and the warning before closing a
//! window with changes not exported.

use std::collections::HashMap;
use std::path::PathBuf;

use iced::widget::{container, progress_bar, space, text};
use iced::{Center, Element, Fill, Task, Theme, window};
use plaa_media::export::{Encoder, Export, ExportError, ExportEvent, Format, Job, Sound};
use plaa_media::{Edit, Source};
use scramble_ui::button::Kind;
use scramble_ui::component;
use scramble_ui::{Icon, Scheme, Type, button, column, icon, row, style};

use super::{Content, Message, Plaa};
use crate::External;
use crate::window::PlayerWindow;

/// What the Export dialog has chosen.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Choice {
    pub format: Format,
    /// The picture's shorter side, when smaller than the file's.
    pub lines: Option<u32>,
    /// The format and quality for the sound alone, kept while a picture's
    /// format is chosen.
    pub sound: Sound,
    pub quality: u32,
}

/// An export under way from a window.
pub(crate) struct Exporting {
    export: Export,
    /// The file's name it is written to.
    name: String,
    progress: f64,
    /// The file and the edit exported, noted as kept once it is done.
    source: Source,
    edit: Option<Edit>,
}

/// An export's format, picture size and whether the picture is HDR,
/// which the encoder it uses depends on.
pub(crate) type Encoding = (Format, (u32, u32), bool);

/// What closing waits to be told: a window, or plaa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Closing {
    Window,
    Quit,
}

/// Picture heights the Size choice offers below the file's own.
const LINES: [u32; 4] = [1440, 1080, 720, 480];

impl Plaa {
    /// Opens the Export dialog in window `id`.
    pub(super) fn open_export(&mut self, id: window::Id) -> Task<Message> {
        let Some(window) = self.windows.get_mut(&id) else {
            return Task::none();
        };
        if window.exporting.is_some() {
            return Task::none();
        }
        let Content::Player(player) = &window.content else {
            return Task::none();
        };
        if !player.can_edit() {
            return Task::none();
        }
        // As chosen last time, at the file's own size.
        let mut choice = self.export_choice.unwrap_or(Choice {
            format: Format::H264,
            lines: None,
            sound: Sound::Aac,
            quality: Sound::Aac.usual_quality(),
        });
        choice.lines = None;
        if !player.is_video() {
            choice.format = Format::Sound(choice.sound);
        }
        if !choice.sound.available() {
            choice.sound = Sound::Aac;
            choice.quality = usual_quality(player, Sound::Aac);
        }
        window.export = Some(choice);
        self.find_encoder(id);
        Task::none()
    }

    /// Chooses `sound` in window `id`'s Export dialog, at the quality
    /// chosen when the format offers it, else its usual one.
    pub(super) fn choose_sound(&mut self, id: window::Id, sound: Sound) {
        let Some(window) = self.windows.get_mut(&id) else {
            return;
        };
        let Some(player) = window.player() else {
            return;
        };
        let usual = usual_quality(player, sound);
        if let Some(choice) = &mut window.export {
            if !sound.qualities().contains(&choice.quality)
                || choice.sound.lossless() != sound.lossless()
            {
                choice.quality = usual;
            }
            choice.sound = sound;
            choice.format = Format::Sound(sound);
        }
    }

    /// Looks for the encoder window `id`'s export dialog's choice would
    /// use, unless it is known, on a thread of its own, as a graphics
    /// card takes a moment to answer.
    pub(super) fn find_encoder(&mut self, id: window::Id) {
        let Some(window) = self.windows.get(&id) else {
            return;
        };
        let (Some(choice), Some(player)) = (window.export, window.player()) else {
            return;
        };
        let Some(encoding) = encoding(player, choice) else {
            return;
        };
        if self.encoders.contains_key(&encoding) || !self.probing.insert(encoding) {
            return;
        }
        let hardware = self.settings.hardware_decoding;
        std::thread::spawn(move || {
            let (format, size, hdr) = encoding;
            let found = plaa_media::export::encoder(format, size, hdr, hardware);
            crate::post(External::Encoder(encoding, found));
        });
    }

    /// Forgets which encoders exports use, as when graphics cards are
    /// allowed or not.
    pub(crate) fn forget_encoders(&mut self) {
        self.encoders.clear();
    }

    /// Asks where to export what window `id`'s dialog chose.
    pub(super) fn choose_export_file(&mut self, id: window::Id) -> Task<Message> {
        let Some(window) = self.windows.get(&id) else {
            return Task::none();
        };
        let (Some(choice), Some(player)) = (window.export, window.player()) else {
            return Task::none();
        };
        let Source::File(path) = player.source() else {
            return Task::none();
        };
        let stem = path
            .file_stem()
            .map(|stem| stem.to_string_lossy().into_owned())
            .unwrap_or_default();
        let name = if player.editor.is_edited() {
            crate::fl!("export-name-edited", name = stem)
        } else {
            crate::fl!("export-name", name = stem)
        };
        let name = format!("{name}.{}", choice.format.extension());
        let folder = path.parent().map(PathBuf::from);
        Task::perform(
            crate::dialog::save_file(crate::fl!("export-title"), name, folder),
            move |result| Message::ExportTo(id, result),
        )
    }

    /// Starts exporting window `id`'s file to `destination` as its dialog
    /// chose.
    pub(super) fn start_export(&mut self, id: window::Id, destination: PathBuf) {
        let hardware = self.settings.hardware_decoding;
        let Some(window) = self.windows.get_mut(&id) else {
            return;
        };
        let Some(choice) = window.export.take() else {
            return;
        };
        self.export_choice = Some(choice);
        let Content::Player(player) = &window.content else {
            return;
        };
        let Source::File(path) = player.source() else {
            return;
        };
        let edit = player.editor.edit().cloned();
        let job = Job {
            source: path.clone(),
            edit: edit.clone().map(std::sync::Arc::new),
            video: player.status.video,
            audio: player.status.audio,
            format: choice.format,
            lines: choice.lines,
            quality: Some(choice.quality),
            hardware,
            destination: destination.clone(),
        };
        let export = Export::start(job, move |event| {
            crate::post(External::Export(id, event));
        });
        window.exporting = Some(Exporting {
            export,
            name: destination
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default(),
            progress: 0.0,
            source: player.source().clone(),
            edit,
        });
    }

    /// What window `id`'s export says.
    pub(super) fn export_event(&mut self, id: window::Id, event: ExportEvent) {
        let Some(window) = self.windows.get_mut(&id) else {
            return;
        };
        match event {
            ExportEvent::Started(_) => {}
            ExportEvent::Progress(share) => {
                if let Some(exporting) = &mut window.exporting {
                    exporting.progress = share;
                }
            }
            ExportEvent::Finished(result) => {
                let Some(exporting) = window.exporting.take() else {
                    return;
                };
                let notice = match result {
                    Ok(_) => {
                        // Its changes are kept, unless it plays another
                        // file now.
                        if let Content::Player(player) = &mut window.content
                            && player.source() == &exporting.source
                        {
                            player.editor.exported(exporting.edit);
                        }
                        crate::fl!("notice-exported", name = exporting.name)
                    }
                    Err(ExportError::Cancelled) => crate::fl!("notice-export-stopped"),
                    Err(ExportError::Failed(error)) => {
                        crate::fl!("notice-export-failed", error = error)
                    }
                };
                self.notify(id, notice);
            }
        }
    }

    /// Stops window `id`'s export.
    pub(super) fn stop_export(&mut self, id: window::Id) {
        if let Some(exporting) = self
            .windows
            .get(&id)
            .and_then(|window| window.exporting.as_ref())
        {
            exporting.export.cancel();
        }
    }

    /// Whether closing window `id` would lose something: changes not
    /// exported, or an export under way.
    pub(super) fn would_lose(&self, id: window::Id) -> bool {
        self.windows.get(&id).is_some_and(|window| {
            window.exporting.is_some()
                || window
                    .player()
                    .is_some_and(|player| player.editor.unexported())
        })
    }

    /// Asks before closing window `id`, or plaa, when that would lose
    /// something. Returns whether it asked.
    pub(super) fn ask_before_closing(&mut self, id: window::Id, closing: Closing) -> bool {
        if !self.would_lose(id) {
            return false;
        }
        if let Some(window) = self.windows.get_mut(&id) {
            window.closing = Some(closing);
        }
        true
    }

    /// Closes window `id`, or quits, as was asked and confirmed.
    pub(super) fn confirm_close(&mut self, id: window::Id) -> Task<Message> {
        let Some(closing) = self
            .windows
            .get_mut(&id)
            .and_then(|window| window.closing.take())
        else {
            return Task::none();
        };
        match closing {
            Closing::Window => window::close(id),
            Closing::Quit => self.quit(),
        }
    }
}

/// The quality `sound` is first offered at: for lossless sound, as many
/// bits as the file has, at least CD quality's.
fn usual_quality(player: &PlayerWindow, sound: Sound) -> u32 {
    if !sound.lossless() {
        return sound.usual_quality();
    }
    let bits = sound_track(player)
        .and_then(|track| track.bits)
        .unwrap_or(16);
    if bits > 16 { 24 } else { 16 }
}

/// The sound track playing, which is the one exported.
fn sound_track(player: &PlayerWindow) -> Option<&plaa_media::info::AudioTrack> {
    player
        .info
        .as_ref()?
        .audio
        .iter()
        .find(|track| Some(track.track.index) == player.status.audio)
}

/// The format and picture size `choice` exports `player`'s file in; none
/// for sound alone.
fn encoding(player: &PlayerWindow, choice: Choice) -> Option<Encoding> {
    if matches!(choice.format, Format::Sound(_)) {
        return None;
    }
    let track = player.picture()?;
    let shown = player.transform().after_turns(track.quarter_turns);
    let size = plaa_media::export::output_size(
        (track.width, track.height),
        track.aspect,
        shown,
        choice.lines,
    );
    Some((choice.format, size, track.hdr))
}

/// What an encoder runs on, as the dialog says: the graphics card's API,
/// or the processor.
fn runs_on(encoder: Encoder) -> String {
    if !encoder.hardware {
        return crate::fl!("export-on-processor");
    }
    let api = [
        ("_nvenc", "NVENC"),
        ("_amf", "AMF"),
        ("_qsv", "Quick Sync"),
        ("_vaapi", "VAAPI"),
        ("_videotoolbox", "VideoToolbox"),
    ]
    .into_iter()
    .find(|(ending, _)| encoder.name.ends_with(ending))
    .map_or("", |(_, api)| api);
    crate::fl!("export-on-graphics-card", api = api)
}

/// The Export dialog: the format, and the size and what encodes it, or
/// for the sound alone, its format and quality.
pub(super) fn dialog<'a>(
    id: window::Id,
    choice: Choice,
    player: &'a PlayerWindow,
    encoders: &HashMap<Encoding, Option<Encoder>>,
) -> Element<'a, Message> {
    let has_audio = player.status.audio.is_some();
    let note = |text: String| {
        scramble_ui::aligned(
            scramble_ui::styled(text, Type::BodyMedium).style(style::on_surface_variant),
        )
    };
    let mut body = column![scramble_ui::styled(
        crate::fl!("export-title"),
        Type::HeadlineSmall
    )]
    .spacing(8);
    let mut ready = true;
    // A file with a picture chooses between pictures' formats and the
    // sound alone; one without has only sound.
    if player.is_video() {
        let mut formats = vec![
            (Format::H264, crate::fl!("export-h264")),
            (Format::Hevc, crate::fl!("export-hevc")),
            (Format::Av1, crate::fl!("export-av1")),
        ];
        if has_audio {
            formats.push((Format::Sound(choice.sound), crate::fl!("export-audio-only")));
        }
        let format_buttons = formats
            .into_iter()
            .map(|(format, label)| {
                scramble_ui::button(Kind::Tonal, label)
                    .selected(choice.format == format)
                    .on_press(Message::ExportFormat(id, format))
            })
            .collect();
        body = body
            .push(component::section(crate::fl!("export-format")))
            .push(component::connected(format_buttons));
    }
    if let Some(encoding) = encoding(player, choice) {
        let (text, found) = match encoders.get(&encoding) {
            Some(Some(encoder)) => (runs_on(*encoder), true),
            Some(None) => (crate::fl!("export-no-encoder"), false),
            None => (crate::fl!("export-checking"), false),
        };
        ready = found;
        body = body.push(note(text));
        if let Some(sizes) = size_buttons(id, choice, player) {
            body = body
                .push(component::section(crate::fl!("export-size")))
                .push(sizes);
        }
    } else {
        let sound_buttons = Sound::ALL
            .into_iter()
            .filter(|sound| sound.available())
            .map(|sound| {
                scramble_ui::button(Kind::Tonal, crate::format::sound_name(sound))
                    .selected(choice.sound == sound)
                    .on_press(Message::ExportSound(id, sound))
            })
            .collect();
        let quality_buttons = choice
            .sound
            .qualities()
            .iter()
            .map(|&quality| {
                let label = if choice.sound.lossless() {
                    crate::fl!("export-bits", bits = quality)
                } else {
                    crate::fl!("export-kilobits", rate = quality)
                };
                scramble_ui::button(Kind::Tonal, label)
                    .selected(choice.quality == quality)
                    .on_press(Message::ExportQuality(id, quality))
            })
            .collect();
        body = body
            .push(component::section(crate::fl!("export-sound-format")))
            .push(component::connected(sound_buttons))
            .push(note(sound_note(choice.sound)))
            .push(component::section(crate::fl!("export-quality")))
            .push(component::connected(quality_buttons));
        // Formats that cannot keep the file's rate say so.
        if let Some(file) = sound_track(player).map(|track| track.sample_rate) {
            let rate = choice.sound.rate_for(choice.quality, file);
            if rate < file {
                body = body.push(note(crate::fl!(
                    "export-rate-lower",
                    rate = crate::format::kilohertz(rate),
                    file = crate::format::kilohertz(file)
                )));
            }
        }
    }
    let actions = vec![
        scramble_ui::button(Kind::Text, crate::fl!("common-cancel"))
            .on_press(Message::ExportDismissed(id))
            .into(),
        scramble_ui::button(Kind::Filled, crate::fl!("export-choose"))
            .on_press_maybe(ready.then_some(Message::ExportChosen(id)))
            .into(),
    ];
    let body = body.push(
        container(scramble_ui::dir::row(actions).spacing(8))
            .align_right(Fill)
            .padding(iced::Padding {
                top: 16.0,
                ..iced::Padding::ZERO
            }),
    );
    let card = container(body).padding(24).width(540).style(style::dialog);
    iced::widget::mouse_area(
        container(iced::widget::opaque(card))
            .center(Fill)
            .padding(24)
            .style(style::scrim),
    )
    .on_press(Message::ExportDismissed(id))
    .into()
}

/// The sizes the picture can be exported at, its own first: none for a
/// file without a picture.
fn size_buttons<'a>(
    id: window::Id,
    choice: Choice,
    player: &PlayerWindow,
) -> Option<Element<'a, Message>> {
    let track = player.picture()?;
    let turned = player.transform().after_turns(track.quarter_turns).turns % 2 == 1;
    let shown_width = (f64::from(track.height) * track.aspect).round() as u32;
    let shorter = if turned {
        shown_width.min(track.height)
    } else {
        track.height.min(shown_width)
    };
    let mut sizes = vec![(None, crate::fl!("export-size-original", lines = shorter))];
    sizes.extend(
        LINES
            .into_iter()
            .filter(|lines| *lines < shorter)
            .map(|lines| {
                (
                    Some(lines),
                    crate::fl!("quality-lines", lines = lines.to_string()),
                )
            }),
    );
    let buttons = sizes
        .into_iter()
        .map(|(lines, label)| {
            scramble_ui::button(Kind::Tonal, label)
                .selected(choice.lines == lines)
                .on_press(Message::ExportSize(id, lines))
        })
        .collect();
    Some(component::connected(buttons))
}

/// What a sound format is good for, as the dialog says.
fn sound_note(sound: Sound) -> String {
    match sound {
        Sound::Aac => crate::fl!("export-aac-note"),
        Sound::Mp3 => crate::fl!("export-mp3-note"),
        Sound::Opus => crate::fl!("export-opus-note"),
        Sound::Flac => crate::fl!("export-flac-note"),
        Sound::Alac => crate::fl!("export-alac-note"),
        Sound::Wav => crate::fl!("export-wav-note"),
    }
}

/// The export under way, over the top of the window, below the toolbar:
/// what it writes, how far it is, and a button to stop it.
pub(super) fn progress<'a>(id: window::Id, exporting: &'a Exporting) -> Element<'a, Message> {
    let percent = (exporting.progress * 100.0).floor() as u32;
    let card = container(
        row![
            icon::icon(Icon::FileExport, 20).style(style::on_surface_variant),
            column![
                scramble_ui::styled(
                    crate::fl!("export-progress", name = exporting.name.as_str()),
                    Type::LabelLarge
                )
                .wrapping(text::Wrapping::None),
                progress_bar(0.0..=1.0, exporting.progress as f32)
                    .girth(4)
                    .style(|theme: &Theme| {
                        let scheme = Scheme::of(theme);
                        progress_bar::Style {
                            background: scheme.secondary_container.into(),
                            bar: scheme.primary.into(),
                            border: iced::border::rounded(2),
                        }
                    }),
            ]
            .spacing(6)
            .width(240),
            scramble_ui::styled(format!("{percent}%"), Type::LabelLarge)
                .style(style::on_surface_variant),
            component::tip(
                button::icon_button(Icon::Close)
                    .size(button::Size::ExtraSmall)
                    .on_press(Message::ExportStop(id)),
                crate::fl!("export-stop"),
            ),
        ]
        .spacing(12)
        .align_y(Center),
    )
    .padding([8, 16])
    .style(style::snackbar);
    container(column![
        space().height(component::floating_room(true)),
        card
    ])
    .center_x(Fill)
    .into()
}

/// The question before closing a window, or plaa, that would lose
/// changes or stop an export.
pub(super) fn close_dialog<'a>(
    id: window::Id,
    closing: Closing,
    exporting: bool,
    title: String,
) -> Element<'a, Message> {
    let supporting = if exporting {
        crate::fl!("close-exporting", name = title)
    } else {
        crate::fl!("close-unexported", name = title)
    };
    let confirm = match closing {
        Closing::Window => crate::fl!("close-anyway"),
        Closing::Quit => crate::fl!("close-quit-anyway"),
    };
    component::dialog(
        space().width(Fill).height(Fill),
        Some(Icon::Warning),
        crate::fl!("close-title"),
        supporting,
        vec![
            scramble_ui::button(Kind::Text, crate::fl!("common-cancel"))
                .on_press(Message::CloseDismissed(id))
                .into(),
            scramble_ui::button(Kind::Filled, confirm)
                .on_press(Message::CloseConfirmed(id))
                .into(),
        ],
    )
}
