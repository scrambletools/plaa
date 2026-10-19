//! The Settings dialog: a fixed header with the title, close button and
//! tabs, and below it the chosen tab's settings, which scroll.

use std::path::PathBuf;

use iced::widget::{container, mouse_area, opaque, space, toggler};
use iced::window;
use iced::{Center, Color, Element, Fill, Length, Task, Theme};
use plaa_store::settings::{Appearance, Settings};
use scramble_ui::button::Kind;
use scramble_ui::component::{self, Backdrop};
use scramble_ui::dropdown;
use scramble_ui::{Icon, Scheme, Type, button, column, row, style};

use crate::app::{Message, Plaa};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsTab {
    #[default]
    Appearance,
    Playback,
    Language,
}

/// The folders Settings chooses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Folder {
    Recordings,
    Screenshots,
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    Close(window::Id),
    Tab(SettingsTab),
    Appearance(Appearance),
    SystemAccent(bool),
    Accent(String),
    Overlay(bool),
    OverlayDelay(f32),
    OverlayTransparency(f32),
    CornerRadius(f32),
    Animations(bool),
    Hardware(bool),
    /// An output device by id, or the system's default.
    AudioDevice(Option<String>),
    Resume(bool),
    FolderPlaylists(bool),
    Language(String),
    /// The audio (0) or subtitle (1) languages, as typed.
    Languages(usize, String),
    ChooseFolder(Folder),
    FolderChosen(Folder, Result<Option<PathBuf>, String>),
    /// A slider was let go: save what it set.
    SliderReleased,
}

/// The colors offered when the system's accent is not used.
const ACCENT_SWATCHES: [Color; 11] = [
    scramble_ui::scheme::DEFAULT_SEED,
    Color::from_rgb8(0x00, 0x89, 0x7b),
    Color::from_rgb8(0x43, 0xa0, 0x47),
    Color::from_rgb8(0xf9, 0xa8, 0x25),
    Color::from_rgb8(0xf5, 0x7c, 0x00),
    Color::from_rgb8(0xe5, 0x39, 0x35),
    Color::from_rgb8(0xd8, 0x1b, 0x60),
    Color::from_rgb8(0x8e, 0x24, 0xaa),
    Color::from_rgb8(0x39, 0x49, 0xab),
    Color::from_rgb8(0x6d, 0x4c, 0x41),
    Color::from_rgb8(0x75, 0x75, 0x75),
];

/// plaa's own color, a violet like its icon.
pub const PLAA_SEED: Color = Color::from_rgb8(0x70, 0x48, 0xe8);

/// The color picked in Settings, else plaa's own.
pub fn chosen_accent(settings: &Settings) -> Color {
    settings
        .accent_color
        .as_deref()
        .and_then(scramble_ui::appearance::hex_to_color)
        .unwrap_or(PLAA_SEED)
}

/// How tall the dialog is at most, so it keeps one height across tabs.
const DIALOG_HEIGHT: f32 = 640.0;

fn note<'a>(text: String) -> Element<'a, SettingsMessage> {
    scramble_ui::aligned(
        scramble_ui::styled(text, Type::BodyMedium).style(style::on_surface_variant),
    )
}

/// A setting with a title, a note under it and a switch.
fn switch_row<'a>(
    title: String,
    explanation: String,
    on: bool,
    toggled: fn(bool) -> SettingsMessage,
) -> Element<'a, SettingsMessage> {
    row![
        column![
            scramble_ui::styled(title, Type::BodyLarge),
            note(explanation)
        ]
        .spacing(2)
        .width(Fill),
        toggler(on).on_toggle(toggled).size(28).style(style::switch),
    ]
    .spacing(16)
    .align_y(Center)
    .into()
}

/// A setting with a title, a note, its value and a slider.
#[allow(clippy::too_many_arguments)]
fn slider_row<'a>(
    title: String,
    explanation: String,
    value_label: String,
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    step: f32,
    changed: fn(f32) -> SettingsMessage,
) -> Element<'a, SettingsMessage> {
    row![
        column![
            scramble_ui::styled(title, Type::BodyLarge),
            note(explanation)
        ]
        .spacing(2)
        .width(Fill),
        scramble_ui::styled(value_label, Type::LabelLarge).style(style::on_surface_variant),
        iced::widget::slider(range, value, changed)
            .step(step)
            .on_release(SettingsMessage::SliderReleased)
            .width(160)
            .height(style::SLIDER_HEIGHT)
            .style(|theme: &Theme, status| {
                style::slider(Backdrop::ContainerHigh.color(&Scheme::of(theme)))(theme, status)
            }),
    ]
    .spacing(16)
    .align_y(Center)
    .into()
}

/// A round button that picks `color` for the scheme, ringed when chosen.
fn accent_swatch<'a>(color: Color, selected: bool) -> Element<'a, SettingsMessage> {
    let dot = container(space().width(24).height(24)).style(move |theme: &Theme| {
        let scheme = Scheme::of(theme);
        container::Style {
            background: Some(color.into()),
            border: iced::Border {
                color: if selected {
                    scheme.on_surface
                } else {
                    scheme.outline_variant
                },
                width: if selected { 3.0 } else { 1.0 },
                radius: scramble_ui::shape::FULL.into(),
            },
            ..container::Style::default()
        }
    });
    let hex = scramble_ui::appearance::color_to_hex(color);
    component::tip(
        button::custom(Kind::Standard, dot)
            .size(button::Size::ExtraSmall)
            .width(36)
            .on_press(SettingsMessage::Accent(hex.clone())),
        hex,
    )
}

/// A folder setting: where it is, and a button to choose another.
fn folder_row<'a>(
    title: String,
    path: &std::path::Path,
    folder: Folder,
) -> Element<'a, SettingsMessage> {
    row![
        column![
            scramble_ui::styled(title, Type::BodyLarge),
            note(
                plaa_store::paths::abbreviate_home(path)
                    .display()
                    .to_string()
            ),
        ]
        .spacing(2)
        .width(Fill),
        scramble_ui::button(Kind::Outlined, crate::fl!("settings-choose"))
            .on_press(SettingsMessage::ChooseFolder(folder)),
    ]
    .spacing(16)
    .align_y(Center)
    .into()
}

impl Plaa {
    pub(crate) fn settings_dialog(&self, id: window::Id) -> Element<'_, SettingsMessage> {
        let tab = |label: String, which: SettingsTab| component::Tab {
            label: crate::i18n::lasting(label),
            icon: None,
            selected: self.settings_tab == which,
            on_press: SettingsMessage::Tab(which),
        };
        let tabs = component::tabs(vec![
            tab(
                crate::fl!("settings-tab-appearance"),
                SettingsTab::Appearance,
            ),
            tab(crate::fl!("settings-tab-playback"), SettingsTab::Playback),
            tab(crate::fl!("settings-tab-language"), SettingsTab::Language),
        ]);
        let header = column![
            row![
                scramble_ui::aligned(scramble_ui::styled(
                    crate::fl!("settings-title"),
                    Type::HeadlineSmall
                )),
                component::tip(
                    scramble_ui::icon_button(Icon::Close).on_press(SettingsMessage::Close(id)),
                    crate::fl!("common-close")
                ),
            ]
            .align_y(Center),
            tabs,
        ]
        .spacing(12);
        let mut content = match self.settings_tab {
            SettingsTab::Appearance => self.appearance_settings(),
            SettingsTab::Playback => self.playback_settings(),
            SettingsTab::Language => self.language_settings(),
        };
        if let Some(error) = &self.settings_error {
            content = content.push(scramble_ui::aligned(
                scramble_ui::styled(error, Type::BodyMedium).style(style::error_text),
            ));
        }
        let version = env!("CARGO_PKG_VERSION");
        let build = if plaa_store::paths::PRODUCTION {
            crate::fl!(
                "settings-version",
                version = version,
                build = env!("PLAA_COMMIT")
            )
        } else {
            let build = concat!(env!("PLAA_COMMIT"), ", ", env!("PLAA_BUILT"), " UTC");
            crate::fl!(
                "settings-version-development",
                version = version,
                build = build
            )
        };
        let footer = container(
            scramble_ui::styled(
                format!("{build} · FFmpeg {}", plaa_media::ffmpeg_version()),
                Type::BodySmall,
            )
            .style(style::on_surface_variant),
        )
        .padding(iced::Padding {
            top: 12.0,
            right: 24.0,
            bottom: 16.0,
            left: 24.0,
        })
        .center_x(Fill);
        let window_height = self
            .windows
            .get(&id)
            .map_or(DIALOG_HEIGHT, |window| window.size.height);
        let height = (window_height - 48.0).clamp(240.0, DIALOG_HEIGHT);
        let card = container(column![
            container(header).padding(iced::Padding {
                top: 24.0,
                right: 24.0,
                bottom: 0.0,
                left: 24.0,
            }),
            container(component::scroll(container(content).padding(
                iced::Padding {
                    top: 16.0,
                    right: 24.0,
                    bottom: 8.0,
                    left: 24.0,
                }
            )))
            .height(Fill),
            footer,
        ])
        .width(Length::Fixed(520.0))
        .height(Length::Fixed(height))
        .style(style::dialog);
        // A click on the dimmed window around the dialog closes it.
        mouse_area(
            container(opaque(card))
                .padding(24)
                .center(Fill)
                .style(style::scrim),
        )
        .on_press(SettingsMessage::Close(id))
        .into()
    }

    /// Light or dark, colors, and how the controls float and move.
    fn appearance_settings(&self) -> iced::widget::Column<'_, SettingsMessage> {
        let appearance = self.settings.appearance;
        let choice = |glyph: Icon, label: String, value: Appearance| {
            scramble_ui::with_icon(Kind::Tonal, glyph, label)
                .selected(appearance == value)
                .on_press(SettingsMessage::Appearance(value))
        };
        let accent_note = match (
            self.settings.system_accent,
            &self.omarchy,
            self.system_accent,
        ) {
            (false, _, _) => crate::fl!("settings-accent-chosen-note"),
            (true, Some(palette), _) => {
                crate::fl!("settings-omarchy-note", theme = palette.name.as_str())
            }
            (true, None, Some(_)) => crate::fl!("settings-system-accent-note"),
            (true, None, None) => crate::fl!("settings-system-accent-none"),
        };
        let picking = !self.settings.system_accent
            || (self.omarchy.is_none() && self.system_accent.is_none());
        let chosen = scramble_ui::appearance::color_to_hex(chosen_accent(&self.settings));
        let swatches = picking.then(|| {
            scramble_ui::dir::row(ACCENT_SWATCHES.iter().map(|&swatch| {
                accent_swatch(
                    swatch,
                    scramble_ui::appearance::color_to_hex(swatch) == chosen,
                )
            }))
            .spacing(4)
            .wrap()
        });
        let animations_note = if self.system_animations {
            crate::fl!("settings-animations-note")
        } else {
            crate::fl!("settings-animations-reduced")
        };
        column![
            component::connected(vec![
                choice(
                    Icon::Settings,
                    crate::fl!("settings-appearance-system"),
                    Appearance::System
                ),
                choice(
                    Icon::LightMode,
                    crate::fl!("settings-appearance-light"),
                    Appearance::Light
                ),
                choice(
                    Icon::DarkMode,
                    crate::fl!("settings-appearance-dark"),
                    Appearance::Dark
                ),
            ]),
            component::section(crate::fl!("settings-colors")),
            switch_row(
                crate::fl!("settings-system-accent"),
                accent_note,
                self.settings.system_accent,
                SettingsMessage::SystemAccent,
            ),
            swatches.map_or_else(|| Element::from(space()), Element::from),
            component::section(crate::fl!("settings-controls")),
            switch_row(
                crate::fl!("settings-overlay"),
                crate::fl!("settings-overlay-note"),
                self.settings.overlay,
                SettingsMessage::Overlay,
            ),
            slider_row(
                crate::fl!("settings-overlay-delay"),
                crate::fl!("settings-overlay-delay-note"),
                crate::fl!(
                    "settings-seconds",
                    seconds = format!("{:.1}", self.settings.overlay_delay().as_secs_f32())
                ),
                1.0..=10.0,
                self.settings.overlay_delay().as_secs_f32(),
                0.5,
                SettingsMessage::OverlayDelay,
            ),
            slider_row(
                crate::fl!("settings-transparency"),
                crate::fl!("settings-transparency-note"),
                crate::fl!(
                    "settings-percent",
                    percent = format!("{:.0}", self.settings.overlay_transparency)
                ),
                0.0..=90.0,
                self.settings.overlay_transparency,
                5.0,
                SettingsMessage::OverlayTransparency,
            ),
            slider_row(
                crate::fl!("settings-corner-radius"),
                crate::fl!("settings-corner-radius-note"),
                crate::fl!(
                    "settings-pixels",
                    pixels = format!("{:.0}", self.settings.corner_radius)
                ),
                0.0..=32.0,
                self.settings.corner_radius,
                1.0,
                SettingsMessage::CornerRadius,
            ),
            switch_row(
                crate::fl!("settings-animations"),
                animations_note,
                self.settings.animations,
                SettingsMessage::Animations,
            ),
        ]
        .spacing(12)
    }

    /// Decoding, resuming, and where recordings and pictures go.
    fn playback_settings(&self) -> iced::widget::Column<'_, SettingsMessage> {
        column![
            switch_row(
                crate::fl!("settings-hardware"),
                crate::fl!("settings-hardware-note"),
                self.settings.hardware_decoding,
                SettingsMessage::Hardware,
            ),
            self.audio_device_row(),
            switch_row(
                crate::fl!("settings-resume"),
                crate::fl!("settings-resume-note"),
                self.settings.resume,
                SettingsMessage::Resume,
            ),
            switch_row(
                crate::fl!("settings-folder-playlists"),
                crate::fl!("settings-folder-playlists-note"),
                self.settings.folder_playlists,
                SettingsMessage::FolderPlaylists,
            ),
            component::section(crate::fl!("settings-folders")),
            folder_row(
                crate::fl!("settings-recordings"),
                &self.settings.recordings,
                Folder::Recordings
            ),
            folder_row(
                crate::fl!("settings-screenshots"),
                &self.settings.screenshots,
                Folder::Screenshots
            ),
        ]
        .spacing(12)
    }

    /// The sound output device: the system's default, or one of those it
    /// had when Settings opened.
    fn audio_device_row(&self) -> Element<'_, SettingsMessage> {
        let system = crate::fl!("settings-audio-device-system");
        let mut options = vec![system.clone()];
        options.extend(self.audio_devices.iter().map(|device| device.name.clone()));
        let selected = self
            .settings
            .audio_device
            .as_ref()
            .and_then(|id| self.audio_devices.iter().find(|device| &device.id == id))
            .map_or(system, |device| device.name.clone());
        let devices = self.audio_devices.clone();
        let pick = dropdown::pick(options, Some(selected), move |name| {
            SettingsMessage::AudioDevice(
                devices
                    .iter()
                    .find(|device| device.name == name)
                    .map(|device| device.id.clone()),
            )
        })
        .width(Fill);
        column![
            scramble_ui::styled(crate::fl!("settings-audio-device"), Type::BodyLarge),
            pick,
        ]
        .spacing(8)
        .into()
    }

    /// The interface's language, and the languages of tracks to pick.
    fn language_settings(&self) -> iced::widget::Column<'_, SettingsMessage> {
        let system = crate::fl!("settings-language-system");
        let mut options = vec![system.clone()];
        options.extend(
            crate::i18n::languages()
                .iter()
                .map(|(_, name)| name.clone()),
        );
        let selected = match self.settings.chosen_language() {
            None => system.clone(),
            Some(tag) => crate::i18n::languages()
                .iter()
                .find(|(candidate, _)| candidate == tag)
                .map_or(system.clone(), |(_, name)| name.clone()),
        };
        let language = dropdown::pick(options, Some(selected), move |name| {
            let tag = crate::i18n::languages()
                .iter()
                .find(|(_, candidate)| *candidate == name)
                .map_or_else(
                    || plaa_store::settings::SYSTEM_LANGUAGE.to_owned(),
                    |(tag, _)| tag.clone(),
                );
            SettingsMessage::Language(tag)
        })
        .width(Fill);
        let field = |index: usize, label: String| {
            component::text_field(
                label,
                &self.language_drafts[index],
                Backdrop::ContainerHigh,
                move |input| input.on_input(move |text| SettingsMessage::Languages(index, text)),
            )
        };
        column![
            component::section(crate::fl!("settings-interface-language")),
            language,
            component::section(crate::fl!("settings-track-languages")),
            note(crate::fl!("settings-track-languages-note")),
            field(0, crate::fl!("settings-audio-languages")),
            field(1, crate::fl!("settings-subtitle-languages")),
        ]
        .spacing(12)
    }

    pub(crate) fn settings_update(&mut self, message: SettingsMessage) -> Task<Message> {
        match message {
            SettingsMessage::Close(id) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.settings_open = false;
                }
                return Task::none();
            }
            SettingsMessage::Tab(tab) => {
                self.settings_tab = tab;
                return Task::none();
            }
            SettingsMessage::Appearance(appearance) => {
                self.settings.appearance = appearance;
                self.refresh_theme();
            }
            SettingsMessage::SystemAccent(on) => {
                self.settings.system_accent = on;
                self.refresh_theme();
            }
            SettingsMessage::Accent(hex) => {
                self.settings.accent_color = Some(hex);
                self.refresh_theme();
            }
            SettingsMessage::Overlay(on) => {
                self.settings.overlay = on;
                self.apply_looks();
            }
            SettingsMessage::OverlayDelay(seconds) => {
                self.settings.overlay_delay = seconds;
                return Task::none();
            }
            SettingsMessage::OverlayTransparency(percent) => {
                self.settings.overlay_transparency = percent;
                self.apply_looks();
                return Task::none();
            }
            SettingsMessage::CornerRadius(radius) => {
                self.settings.corner_radius = radius;
                self.apply_looks();
                return Task::none();
            }
            SettingsMessage::Animations(on) => {
                self.settings.animations = on;
                self.apply_looks();
            }
            SettingsMessage::Hardware(on) => self.settings.hardware_decoding = on,
            SettingsMessage::AudioDevice(device) => {
                for window in self.windows.values() {
                    if let Some(player) = window.player() {
                        player.player.set_audio_device(device.clone());
                    }
                }
                self.settings.audio_device = device;
            }
            SettingsMessage::Resume(on) => self.settings.resume = on,
            SettingsMessage::FolderPlaylists(on) => self.settings.folder_playlists = on,
            SettingsMessage::Language(tag) => {
                self.settings.language = tag;
                crate::i18n::set_language(self.settings.chosen_language());
                #[cfg(target_os = "macos")]
                crate::menu_macos::install();
            }
            SettingsMessage::Languages(index, text) => {
                let codes: Vec<String> = text
                    .split([',', ' '])
                    .map(str::trim)
                    .filter(|code| !code.is_empty())
                    .map(str::to_owned)
                    .collect();
                if index == 0 {
                    self.settings.audio_languages = codes;
                } else {
                    self.settings.subtitle_languages = codes;
                }
                self.language_drafts[index] = text;
            }
            SettingsMessage::ChooseFolder(folder) => {
                let (title, current) = match folder {
                    Folder::Recordings => (
                        crate::fl!("settings-recordings"),
                        self.settings.recordings.clone(),
                    ),
                    Folder::Screenshots => (
                        crate::fl!("settings-screenshots"),
                        self.settings.screenshots.clone(),
                    ),
                };
                return Task::perform(
                    crate::dialog::choose_folder(title, Some(current)),
                    move |result| Message::Settings(SettingsMessage::FolderChosen(folder, result)),
                );
            }
            SettingsMessage::FolderChosen(folder, result) => match result {
                Ok(Some(path)) => match folder {
                    Folder::Recordings => self.settings.recordings = path,
                    Folder::Screenshots => self.settings.screenshots = path,
                },
                Ok(None) => return Task::none(),
                Err(error) => {
                    self.settings_error = Some(error);
                    return Task::none();
                }
            },
            SettingsMessage::SliderReleased => {}
        }
        self.save_settings();
        Task::none()
    }
}
