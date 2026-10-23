//! The Settings dialog: a fixed header with the title, close button and
//! tabs, and below it the chosen tab's settings, which scroll.

use std::path::PathBuf;

use iced::widget::{container, mouse_area, opaque, space, toggler};
use iced::window;
use iced::{Center, Color, Element, Fill, Length, Task, Theme};
use plaa_store::settings::{Appearance, Settings, StreamQuality};
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
    Storage,
    Network,
    Language,
    Agents,
    Assistant,
}

/// The folders Settings chooses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Folder {
    SoundRecordings,
    VideoRecordings,
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
    StreamQuality(StreamQuality),
    WebPages(bool),
    Language(String),
    /// The audio (0) or subtitle (1) languages, as typed.
    Languages(usize, String),
    ChooseFolder(Folder),
    FolderChosen(Folder, Result<Option<PathBuf>, String>),
    /// A slider was let go: save what it set.
    SliderReleased,
    /// The interface plaa goes on the AVB network on, or none.
    AvbInterface(Option<String>),
    AvbTalker(bool),
    /// The entity's name as typed, and Enter.
    AvbName(String),
    AvbNameDone,
    OutsideControl(bool),
    /// The allowed agents are asked about again.
    ForgetAgents,
    AskBefore(crate::app::tools::Kind, bool),
    /// What the Assistant tab asks of the models.
    Model(crate::app::assistant::ModelMessage),
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
/// How wide it is at most, room for its seven tabs.
const DIALOG_WIDTH: f32 = 720.0;

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
            tab(crate::fl!("settings-tab-storage"), SettingsTab::Storage),
            tab(crate::fl!("settings-tab-network"), SettingsTab::Network),
            tab(crate::fl!("settings-tab-language"), SettingsTab::Language),
            tab(crate::fl!("settings-tab-agents"), SettingsTab::Agents),
            tab(crate::fl!("settings-tab-assistant"), SettingsTab::Assistant),
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
            SettingsTab::Storage => self.storage_settings(),
            SettingsTab::Network => self.network_settings(),
            SettingsTab::Language => self.language_settings(),
            SettingsTab::Agents => self.agent_settings(),
            SettingsTab::Assistant => self.assistant_settings(),
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
        let window_size = self
            .windows
            .get(&id)
            .map_or(iced::Size::new(DIALOG_WIDTH, DIALOG_HEIGHT), |window| {
                window.size
            });
        let height = (window_size.height - 48.0).clamp(240.0, DIALOG_HEIGHT);
        let width = (window_size.width - 48.0).clamp(320.0, DIALOG_WIDTH);
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
        .width(Length::Fixed(width))
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

    /// Decoding, resuming, folders played as lists, and streams.
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
            component::section(crate::fl!("settings-streams")),
            self.stream_quality_row(),
            switch_row(
                crate::fl!("settings-web-pages"),
                if self.yt_dlp_found {
                    crate::fl!("settings-web-pages-found")
                } else {
                    crate::fl!("settings-web-pages-missing")
                },
                self.settings.web_pages,
                SettingsMessage::WebPages,
            ),
        ]
        .spacing(12)
    }

    /// Where new recordings and screenshots go.
    fn storage_settings(&self) -> iced::widget::Column<'_, SettingsMessage> {
        column![
            component::section(crate::fl!("settings-folders")),
            folder_row(
                crate::fl!("settings-sound-recordings"),
                &self.settings.sound_recordings,
                Folder::SoundRecordings
            ),
            folder_row(
                crate::fl!("settings-video-recordings"),
                &self.settings.video_recordings,
                Folder::VideoRecordings
            ),
            folder_row(
                crate::fl!("settings-screenshots"),
                &self.settings.screenshots,
                Folder::Screenshots
            ),
        ]
        .spacing(12)
    }

    /// The AVB network: the interface plaa goes on, and plaa's own talker.
    fn network_settings(&self) -> iced::widget::Column<'_, SettingsMessage> {
        let off = crate::fl!("settings-avb-off");
        let label = |interface: &plaa_avb::Interface| {
            if interface.can_send {
                interface.name.clone()
            } else if cfg!(target_os = "macos") {
                crate::fl!("settings-avb-off-mac", interface = interface.name.clone())
            } else {
                crate::fl!("settings-avb-no-clock", interface = interface.name.clone())
            }
        };
        let mut options = vec![off.clone()];
        options.extend(self.avb_interfaces.iter().map(label));
        let chosen = self.settings.avb_interface.as_ref();
        let selected = chosen.map_or(off, |name| {
            self.avb_interfaces
                .iter()
                .find(|interface| &interface.name == name)
                .map_or_else(|| name.clone(), label)
        });
        let interfaces = self.avb_interfaces.clone();
        let pick = dropdown::pick(options, Some(selected), move |picked| {
            SettingsMessage::AvbInterface(
                interfaces
                    .iter()
                    .find(|interface| label(interface) == picked)
                    .map(|interface| interface.name.clone()),
            )
        })
        .width(Fill);
        let idle_note = if cfg!(target_os = "macos") {
            crate::fl!("settings-avb-talker-note-mac")
        } else {
            crate::fl!("settings-avb-talker-note")
        };
        let talker_note = match &self.avb_snapshot.talker {
            _ if self.settings.avb_interface.is_none() => idle_note,
            plaa_avb::TalkerState::Blocked(why) => {
                crate::fl!("network-blocked", reason = why.clone())
            }
            plaa_avb::TalkerState::Waiting => crate::fl!("network-waiting"),
            plaa_avb::TalkerState::Streaming => crate::fl!("network-sending"),
            plaa_avb::TalkerState::Off => idle_note,
        };
        let mut content = column![
            column![
                scramble_ui::styled(crate::fl!("settings-avb-interface"), Type::BodyLarge),
                note(if cfg!(target_os = "macos") {
                    crate::fl!("settings-avb-interface-note-mac")
                } else {
                    crate::fl!("settings-avb-interface-note")
                }),
                pick,
            ]
            .spacing(8),
        ]
        .spacing(12);
        if let Some(error) = &self.avb_error {
            content = content.push(scramble_ui::aligned(
                scramble_ui::styled(error, Type::BodyMedium).style(style::error_text),
            ));
        }
        content = content.push(switch_row(
            crate::fl!("settings-avb-talker"),
            talker_note,
            self.settings.avb_talker,
            SettingsMessage::AvbTalker,
        ));
        // macOS names the Mac's own entity, in Audio MIDI Setup.
        if cfg!(target_os = "macos") {
            return content;
        }
        content = content.push(component::text_field(
            crate::fl!("settings-avb-name"),
            &self.settings.avb_name,
            Backdrop::ContainerHigh,
            |input| {
                input
                    .on_input(SettingsMessage::AvbName)
                    .on_submit(SettingsMessage::AvbNameDone)
            },
        ));
        content.push(note(crate::fl!(
            "settings-avb-name-note",
            name = self.avb_name()
        )))
    }

    /// The quality streams play in: as tall as the screen, the best, or
    /// at most a height.
    fn stream_quality_row(&self) -> Element<'_, SettingsMessage> {
        let label = |quality: StreamQuality| match quality {
            StreamQuality::Auto => crate::fl!("settings-quality-auto"),
            StreamQuality::Best => crate::fl!("settings-quality-best"),
            StreamQuality::AtMost(lines) => crate::fl!("quality-lines", lines = lines.to_string()),
        };
        let choices: Vec<StreamQuality> = [StreamQuality::Auto, StreamQuality::Best]
            .into_iter()
            .chain(
                [2160, 1440, 1080, 720, 480, 360]
                    .into_iter()
                    .map(StreamQuality::AtMost),
            )
            .collect();
        let options: Vec<String> = choices.iter().copied().map(label).collect();
        let selected = label(self.settings.stream_quality);
        let pick = dropdown::pick(options.clone(), Some(selected), move |name| {
            let at = options.iter().position(|option| *option == name);
            SettingsMessage::StreamQuality(at.map_or(StreamQuality::Auto, |at| choices[at]))
        })
        .width(Fill);
        column![
            scramble_ui::styled(crate::fl!("settings-stream-quality"), Type::BodyLarge),
            note(crate::fl!("settings-stream-quality-note")),
            pick,
        ]
        .spacing(8)
        .into()
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

    /// Outside control: whether agents may control plaa, those allowed,
    /// and what plaa asks about first.
    fn agent_settings(&self) -> iced::widget::Column<'_, SettingsMessage> {
        use crate::app::tools::Kind as ToolKind;
        let mut content = column![switch_row(
            crate::fl!("settings-allow-outside-control"),
            crate::fl!("settings-allow-outside-control-note"),
            self.settings.outside_control,
            SettingsMessage::OutsideControl,
        )]
        .spacing(12);
        if !self.settings.allowed_agents.is_empty() {
            content = content.push(
                row![
                    container(note(crate::fl!(
                        "settings-allowed-agents",
                        agents = self.settings.allowed_agents.join(", ")
                    )))
                    .width(Fill),
                    scramble_ui::button(Kind::Text, crate::fl!("settings-forget-agents"))
                        .on_press(SettingsMessage::ForgetAgents),
                ]
                .spacing(16)
                .align_y(Center),
            );
        }
        let mut asks = column![note(crate::fl!("settings-ask-before-note"))].spacing(4);
        for kind in ToolKind::ALL {
            let label = match kind {
                ToolKind::Read => crate::fl!("settings-ask-reading"),
                ToolKind::Play => crate::fl!("settings-ask-playing"),
                ToolKind::Open => crate::fl!("settings-ask-opening"),
            };
            asks = asks.push(
                row![
                    container(scramble_ui::styled(label, Type::BodyLarge)).width(Fill),
                    toggler(kind.asks(&self.settings.ask_before))
                        .on_toggle(move |on| SettingsMessage::AskBefore(kind, on))
                        .size(28)
                        .style(style::switch),
                ]
                .spacing(16)
                .align_y(Center),
            );
        }
        content.push(asks)
    }

    /// The models the assistant panel can use, and adding one.
    fn assistant_settings(&self) -> iced::widget::Column<'_, SettingsMessage> {
        use crate::app::assistant::{
            ContextSize, Found, ModelMessage, ProviderChoice, choice_of, context_sizes,
            provider_label, shown_name,
        };
        let send = |message: ModelMessage| SettingsMessage::Model(message);
        let note = |text: String| {
            scramble_ui::aligned(
                scramble_ui::styled(text, Type::BodyMedium).style(style::on_surface_variant),
            )
        };
        let small = |text: String| {
            scramble_ui::aligned(
                scramble_ui::styled(text, Type::BodySmall).style(style::on_surface_variant),
            )
        };
        let context_menu = |size: u32, on_pick: Box<dyn Fn(ContextSize) -> SettingsMessage>| {
            scramble_ui::dropdown::pick(context_sizes(), Some(ContextSize(size)), on_pick)
                .size(button::Size::ExtraSmall)
        };
        let mut models = column![note(crate::fl!("settings-assistant-note"))].spacing(8);
        if self.settings.assistant_models.is_empty() {
            models = models.push(note(crate::fl!("settings-assistant-none")));
        }
        for model in &self.settings.assistant_models {
            let used = self.settings.assistant_model.as_deref() == Some(model.id.as_str());
            let choice = choice_of(model);
            let provider = choice.as_ref().map_or_else(
                || model.provider.clone(),
                |choice| provider_label(choice.provider),
            );
            let detail = match &model.address {
                Some(address) => format!("{provider} · {address}"),
                None => provider,
            };
            let context: Element<'_, SettingsMessage> = match &choice {
                Some(choice) if choice.provider.sets_context() => {
                    let id = model.id.clone();
                    component::tip(
                        context_menu(
                            model.context.unwrap_or(plaa_assist::DEFAULT_CONTEXT),
                            Box::new(move |size| send(ModelMessage::Context(id.clone(), size))),
                        ),
                        crate::fl!("settings-assistant-context"),
                    )
                }
                _ => space().into(),
            };
            let action: Element<'_, SettingsMessage> = if used {
                scramble_ui::styled(crate::fl!("settings-assistant-in-use"), Type::LabelLarge)
                    .style(style::on_surface_variant)
                    .into()
            } else {
                scramble_ui::button(Kind::Text, crate::fl!("settings-assistant-use"))
                    .on_press(send(ModelMessage::Use(model.id.clone())))
                    .into()
            };
            // One width, so the context menus line up.
            let action = container(action)
                .width(72)
                .align_x(iced::alignment::Horizontal::Center);
            models = models.push(
                row![
                    column![
                        scramble_ui::styled(shown_name(model), Type::BodyLarge),
                        scramble_ui::styled(detail, Type::BodyMedium)
                            .style(style::on_surface_variant),
                    ]
                    .spacing(2)
                    .width(Fill),
                    context,
                    action,
                    component::tip(
                        scramble_ui::icon_button(Icon::Delete)
                            .on_press(send(ModelMessage::Remove(model.id.clone()))),
                        crate::fl!("settings-assistant-remove")
                    ),
                ]
                .spacing(8)
                .align_y(Center),
            );
        }

        let form = &self.model_form;
        let provider = form.provider;
        let label = provider_label(provider);
        let provider_menu = scramble_ui::dropdown::pick(
            plaa_assist::Provider::ALL.map(ProviderChoice),
            Some(ProviderChoice(provider)),
            |choice| SettingsMessage::Model(ModelMessage::Provider(choice)),
        )
        .width(Fill);
        let mut add = column![
            component::section(crate::fl!("settings-assistant-add")),
            provider_menu,
        ]
        .spacing(12);

        // How plaa reaches the provider: its key, or its server.
        if provider.needs_key() {
            if form.asks_for_key() {
                add = add.push(
                    row![
                        container(component::text_field(
                            crate::fl!("settings-assistant-key"),
                            &form.key,
                            Backdrop::ContainerHigh,
                            |input| {
                                input
                                    .secure(true)
                                    .on_input(move |key| send(ModelMessage::Key(key)))
                                    .on_submit(send(ModelMessage::UseKey))
                            },
                        ))
                        .width(Fill),
                        component::beside_field(
                            scramble_ui::button(
                                Kind::Filled,
                                crate::fl!("settings-assistant-use-key")
                            )
                            .on_press(send(ModelMessage::UseKey))
                        ),
                    ]
                    .spacing(8)
                    .align_y(Center),
                );
                if let Some(page) = plaa_assist::key_page(provider) {
                    add = add.push(
                        row![
                            container(small(crate::fl!(
                                "settings-assistant-key-where",
                                provider = label.clone()
                            )))
                            .width(Fill),
                            scramble_ui::button(
                                Kind::Text,
                                crate::fl!("settings-assistant-get-key")
                            )
                            .on_press(send(ModelMessage::OpenLink(page))),
                        ]
                        .spacing(8)
                        .align_y(Center),
                    );
                }
            } else if form.key_kept == Some(true) {
                add = add.push(
                    row![
                        container(note(crate::fl!(
                            "settings-assistant-key-kept",
                            provider = label.clone()
                        )))
                        .width(Fill),
                        scramble_ui::button(
                            Kind::Text,
                            crate::fl!("settings-assistant-change-key")
                        )
                        .on_press(send(ModelMessage::ChangeKey)),
                    ]
                    .spacing(8)
                    .align_y(Center),
                );
            }
        } else {
            // The address field shows when no server answered, or one was
            // found at the address typed; it has its own Look again.
            let found = matches!(form.found, Found::Models(..) | Found::Looking);
            let field_shown = !(found && form.address.is_empty()) || form.other_address;
            let server: Element<'_, SettingsMessage> = match &form.found {
                Found::Models(Some(address), _) => {
                    let mut line = row![
                        container(small(crate::fl!(
                            "settings-assistant-found-at",
                            provider = label.clone(),
                            // Without the scheme, which would wrap alone.
                            address = address
                                .split_once("://")
                                .map_or(address.as_str(), |(_, rest)| rest)
                                .to_owned()
                        )))
                        .width(Fill),
                    ]
                    .spacing(8)
                    .align_y(Center);
                    if !field_shown {
                        line = line
                            .push(
                                scramble_ui::button(
                                    Kind::Text,
                                    crate::fl!("settings-assistant-other-address"),
                                )
                                .on_press(send(ModelMessage::OtherAddress)),
                            )
                            .push(
                                scramble_ui::button(
                                    Kind::Text,
                                    crate::fl!("settings-assistant-look-again"),
                                )
                                .on_press(send(ModelMessage::Search)),
                            );
                    }
                    line.into()
                }
                Found::NoServer => {
                    let mut lines = column![note(crate::fl!(
                        "settings-assistant-no-server",
                        provider = label.clone()
                    ))]
                    .spacing(8);
                    if let Some(page) = plaa_assist::server_page(provider) {
                        lines = lines.push(
                            scramble_ui::button(
                                Kind::Tonal,
                                crate::fl!(
                                    "settings-assistant-get-server",
                                    provider = label.clone()
                                ),
                            )
                            .on_press(send(ModelMessage::OpenLink(page))),
                        );
                    }
                    lines.into()
                }
                _ => space().into(),
            };
            add = add.push(server);
            if field_shown {
                add = add.push(
                    row![
                        container(component::text_field(
                            crate::fl!(
                                "settings-assistant-address",
                                example = provider.default_address().unwrap_or_default()
                            ),
                            &form.address,
                            Backdrop::ContainerHigh,
                            |input| {
                                input
                                    .on_input(move |address| send(ModelMessage::Address(address)))
                                    .on_submit(send(ModelMessage::Search))
                            },
                        ))
                        .width(Fill),
                        component::beside_field(
                            scramble_ui::button(
                                Kind::Text,
                                crate::fl!("settings-assistant-look-again")
                            )
                            .on_press_maybe(
                                (!matches!(form.found, Found::Looking))
                                    .then_some(send(ModelMessage::Search)),
                            )
                        ),
                    ]
                    .spacing(8)
                    .align_y(Center),
                );
            }
        }
        if provider.sets_context() && matches!(form.found, Found::Models(..)) {
            add = add.push(
                column![
                    row![
                        container(scramble_ui::aligned(scramble_ui::styled(
                            crate::fl!("settings-assistant-context"),
                            Type::BodyLarge
                        )))
                        .width(Fill),
                        context_menu(
                            form.context,
                            Box::new(move |size| send(ModelMessage::FormContext(size))),
                        ),
                    ]
                    .spacing(8)
                    .align_y(Center),
                    small(crate::fl!("settings-assistant-context-note")),
                ]
                .spacing(4),
            );
        }

        // The models found, to add with one click.
        match &form.found {
            Found::Looking => add = add.push(note(crate::fl!("settings-assistant-looking"))),
            Found::Failed(problem) => {
                add = add.push(scramble_ui::aligned(
                    scramble_ui::styled(problem.clone(), Type::BodyMedium).style(style::error_text),
                ));
            }
            Found::Models(address, found) => {
                if found.is_empty() {
                    add = add.push(note(crate::fl!(
                        "settings-assistant-found-none",
                        provider = label.clone()
                    )));
                }
                for info in found {
                    // Ollama answers to a name with or without `:latest`.
                    let bare = info.id.strip_suffix(":latest").unwrap_or(&info.id);
                    let added = self.settings.assistant_models.iter().any(|model| {
                        let saved = model.model.strip_suffix(":latest").unwrap_or(&model.model);
                        saved == bare
                            && choice_of(model).is_some_and(|choice| choice.provider == provider)
                            && (model.address == *address
                                || model.address.is_none()
                                    && address.as_deref() == provider.default_address())
                    });
                    let mut tags = Vec::new();
                    if info.recommended {
                        tags.push(crate::fl!("settings-assistant-recommended"));
                    }
                    match info.tools {
                        Some(false) => tags.push(crate::fl!("settings-assistant-no-tools")),
                        Some(true) if !provider.needs_key() => {
                            tags.push(crate::fl!("settings-assistant-uses-tools"));
                        }
                        _ => {}
                    }
                    if info.vision == Some(true) && !provider.needs_key() {
                        tags.push(crate::fl!("settings-assistant-sees"));
                    }
                    let mut detail = info.id.clone();
                    if !tags.is_empty() {
                        detail = format!("{detail} · {}", tags.join(" · "));
                    }
                    let usable = info.tools != Some(false);
                    let button: Element<'_, SettingsMessage> = if added {
                        scramble_ui::styled(
                            crate::fl!("settings-assistant-added-tag"),
                            Type::LabelLarge,
                        )
                        .style(style::on_surface_variant)
                        .into()
                    } else if form.adding.as_deref() == Some(info.id.as_str()) {
                        scramble_ui::styled(
                            crate::fl!("settings-assistant-trying"),
                            Type::LabelLarge,
                        )
                        .style(style::on_surface_variant)
                        .into()
                    } else {
                        scramble_ui::button(
                            Kind::Tonal,
                            crate::fl!("settings-assistant-add-button"),
                        )
                        .on_press_maybe(
                            (usable && form.adding.is_none())
                                .then(|| send(ModelMessage::Add(info.id.clone()))),
                        )
                        .into()
                    };
                    let name = scramble_ui::styled(info.name.clone(), Type::BodyLarge);
                    let name = if usable {
                        name
                    } else {
                        name.style(style::on_surface_variant)
                    };
                    add = add.push(
                        row![column![name, small(detail)].spacing(2).width(Fill), button,]
                            .spacing(8)
                            .align_y(Center),
                    );
                    // Why its trial failed, on the row tried.
                    if let Some((_, why, help)) =
                        form.failed.as_ref().filter(|(id, ..)| *id == info.id)
                    {
                        let mut line = row![
                            container(scramble_ui::aligned(
                                scramble_ui::styled(why.clone(), Type::BodySmall)
                                    .style(style::error_text)
                                    .wrapping(iced::widget::text::Wrapping::WordOrGlyph),
                            ))
                            .width(Fill)
                        ]
                        .spacing(8)
                        .align_y(Center);
                        if let Some(page) = help {
                            line = line.push(
                                scramble_ui::button(
                                    Kind::Text,
                                    crate::fl!("settings-assistant-fix-it"),
                                )
                                .on_press(send(ModelMessage::OpenLink(page))),
                            );
                        }
                        add = add.push(line);
                    }
                }
            }
            Found::Nothing | Found::NoServer => {}
        }

        // A model the provider does not list, by its name.
        let reachable = match &form.found {
            Found::Models(..) => true,
            Found::NoServer | Found::Looking => false,
            Found::Nothing | Found::Failed(_) => {
                !provider.needs_key() || form.key_kept == Some(true)
            }
        };
        if reachable {
            add = add.push(
                row![
                    container(component::text_field(
                        crate::fl!(
                            "settings-assistant-model",
                            example = plaa_assist::example_model(provider)
                        ),
                        &form.typed,
                        Backdrop::ContainerHigh,
                        |input| {
                            input
                                .on_input(move |name| send(ModelMessage::Typed(name)))
                                .on_submit(send(ModelMessage::Add(form.typed.clone())))
                        },
                    ))
                    .width(Fill),
                    component::beside_field(
                        scramble_ui::button(
                            Kind::Text,
                            crate::fl!("settings-assistant-add-button")
                        )
                        .on_press_maybe(
                            (!form.typed.trim().is_empty() && form.adding.is_none())
                                .then(|| send(ModelMessage::Add(form.typed.clone()))),
                        )
                    ),
                ]
                .spacing(8)
                .align_y(Center),
            );
        }
        match &form.status {
            Some(Ok(done)) => add = add.push(note(done.clone())),
            Some(Err(problem)) => {
                add = add.push(scramble_ui::aligned(
                    scramble_ui::styled(problem.clone(), Type::BodyMedium).style(style::error_text),
                ));
            }
            None => {}
        }
        column![models, add].spacing(20)
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
                return self.assistant_tab_shown();
            }
            SettingsMessage::Model(message) => return self.model_settings(message),
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
            SettingsMessage::Hardware(on) => {
                self.settings.hardware_decoding = on;
                // Exports choose their encoders again.
                self.forget_encoders();
            }
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
            SettingsMessage::StreamQuality(quality) => self.settings.stream_quality = quality,
            SettingsMessage::WebPages(on) => self.settings.web_pages = on,
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
                    Folder::SoundRecordings => (
                        crate::fl!("settings-sound-recordings"),
                        self.settings.sound_recordings.clone(),
                    ),
                    Folder::VideoRecordings => (
                        crate::fl!("settings-video-recordings"),
                        self.settings.video_recordings.clone(),
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
                    Folder::SoundRecordings => self.settings.sound_recordings = path,
                    Folder::VideoRecordings => self.settings.video_recordings = path,
                    Folder::Screenshots => self.settings.screenshots = path,
                },
                Ok(None) => return Task::none(),
                Err(error) => {
                    self.settings_error = Some(error);
                    return Task::none();
                }
            },
            SettingsMessage::SliderReleased => {}
            SettingsMessage::AvbInterface(interface) => {
                self.settings.avb_interface = interface;
                self.apply_avb();
            }
            SettingsMessage::AvbTalker(on) => {
                self.settings.avb_talker = on;
                self.apply_avb();
            }
            SettingsMessage::AvbName(name) => self.settings.avb_name = name,
            // The entity starts again under its new name.
            SettingsMessage::AvbNameDone => {
                if self.settings.avb_talker {
                    self.apply_avb();
                }
            }
            SettingsMessage::OutsideControl(on) => {
                self.settings.outside_control = on;
                if !on {
                    self.outside_control_off();
                }
            }
            SettingsMessage::ForgetAgents => self.settings.allowed_agents.clear(),
            SettingsMessage::AskBefore(kind, on) => {
                kind.set_asks(&mut self.settings.ask_before, on)
            }
        }
        self.save_settings();
        Task::none()
    }
}
