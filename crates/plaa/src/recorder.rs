//! A window recording sound, as QuickTime Player's New Audio Recording:
//! a level meter that moves before recording starts, a button to start
//! and stop, how long and how large the recording is, and the input,
//! format and quality to record with.

use std::time::Duration;

use iced::widget::{container, progress_bar, space};
use iced::{Center, Element, Fill, Theme};
use plaa_media::export::Sound;
use plaa_media::record::{InputDevice, Microphone};
use scramble_ui::dropdown::{self, Entry};
use scramble_ui::{Icon, Scheme, Type, column, component, icon, style};

use crate::format;

/// What the recording window's controls send.
#[derive(Debug, Clone)]
pub enum RecorderControl {
    /// Starts recording, or stops it.
    Toggle,
    /// Records from this input device, or the system's default.
    Microphone(Option<String>),
    Format(Sound),
    /// The sound's quality: kb/s, or bits for lossless sound.
    Quality(u32),
    Settings,
    Close,
}

/// How quiet the meter shows, in decibels below full scale.
const FLOOR: f32 = 60.0;
/// How far the meter falls back each tick, as a share of its length.
const FALL: f32 = 0.03;

pub struct Recorder {
    /// The input device, or why it would not open.
    pub microphone: Result<Microphone, String>,
    /// The meter's levels, 0 to 1 along it, falling back gently.
    pub levels: Vec<f32>,
    /// Stopped, and the file is being finished.
    pub saving: bool,
}

impl Recorder {
    /// Opens input device `device`, or the system's default.
    pub fn open(device: Option<&str>) -> Self {
        Self {
            microphone: Microphone::open(device),
            levels: Vec::new(),
            saving: false,
        }
    }

    pub fn is_recording(&self) -> bool {
        self.microphone.as_ref().is_ok_and(Microphone::is_recording)
    }

    /// Takes the device's levels since the last tick into the meter.
    pub fn tick(&mut self) {
        let Ok(microphone) = &self.microphone else {
            return;
        };
        let peaks = microphone.levels();
        self.levels.resize(peaks.len(), 0.0);
        for (shown, peak) in self.levels.iter_mut().zip(peaks) {
            *shown = meter_position(peak).max(*shown - FALL);
        }
    }
}

/// Where a peak of `level`, 0 to 1, falls along the meter: by decibels,
/// from `FLOOR` below full scale.
pub fn meter_position(level: f32) -> f32 {
    if level <= 0.0 {
        return 0.0;
    }
    let decibels = 20.0 * level.log10();
    ((decibels + FLOOR) / FLOOR).clamp(0.0, 1.0)
}

/// What the window's view needs besides the recorder.
pub struct Context<'a> {
    pub devices: &'a [InputDevice],
    pub chosen: Option<&'a str>,
    pub sound: Sound,
    pub quality: u32,
    pub close_button: bool,
}

pub fn view<'a>(recorder: &'a Recorder, context: &Context<'_>) -> Element<'a, RecorderControl> {
    let recording = recorder.is_recording();
    let mut tools: Vec<Element<'a, RecorderControl>> = vec![
        icon::icon(Icon::Mic, 20)
            .style(style::on_surface_variant)
            .into(),
        scramble_ui::styled(crate::fl!("record-title"), Type::TitleMedium).into(),
        space().width(Fill).into(),
        component::tool(
            Icon::Settings,
            crate::fl!("action-settings"),
            Some(RecorderControl::Settings),
        ),
    ];
    if context.close_button {
        tools.push(component::tool(
            Icon::Close,
            crate::fl!("common-close"),
            Some(RecorderControl::Close),
        ));
    }
    let toolbar = container(scramble_ui::dir::row(tools).spacing(8).align_y(Center))
        .padding([0, 8])
        .height(component::TOOLBAR_HEIGHT)
        .width(Fill)
        .align_y(Center)
        .style(style::chrome);
    let body: Element<'a, RecorderControl> = match &recorder.microphone {
        Err(error) => component::empty_state(Icon::MicOff, crate::fl!("record-no-input"), error),
        Ok(microphone) => {
            let (recorded, written) = microphone.recorded();
            let time = format::precise_time(
                if recording || recorder.saving {
                    recorded.as_secs_f64()
                } else {
                    0.0
                },
                recorded >= Duration::from_secs(3600),
            );
            let busy = recording || recorder.saving;
            let size = scramble_ui::styled(
                if busy {
                    format::size(written)
                } else {
                    String::new()
                },
                Type::BodyMedium,
            )
            .style(style::on_surface_variant);
            let meters = iced::widget::Column::with_children(recorder.levels.iter().map(|level| {
                progress_bar(0.0..=1.0, *level)
                    .girth(8)
                    .style(|theme: &Theme| {
                        let scheme = Scheme::of(theme);
                        progress_bar::Style {
                            background: scheme.surface_container_highest.into(),
                            bar: scheme.primary.into(),
                            border: iced::border::rounded(4),
                        }
                    })
                    .into()
            }))
            .spacing(6)
            .width(280);
            container(
                column![
                    scramble_ui::styled(time, Type::DisplaySmall),
                    size,
                    input_menu(microphone.name(), context, busy),
                    meters,
                    sound_menus(context, busy),
                    record_button(recording, recorder.saving),
                ]
                .spacing(12)
                .align_x(Center),
            )
            .center(Fill)
            .into()
        }
    };
    column![
        toolbar,
        container(body)
            .width(Fill)
            .height(Fill)
            .style(style::surface)
    ]
    .into()
}

/// The round button that starts recording, red as a recording light, and
/// stops it.
fn record_button<'a>(recording: bool, saving: bool) -> Element<'a, RecorderControl> {
    let (glyph, label) = if recording {
        (Icon::Stop, crate::fl!("record-stop"))
    } else {
        (Icon::Record, crate::fl!("record-start"))
    };
    let button = iced::widget::button(
        container(icon::filled(glyph, 36))
            .center(Fill)
            .width(72)
            .height(72),
    )
    .padding(0)
    .width(72)
    .height(72)
    .on_press_maybe((!saving).then_some(RecorderControl::Toggle))
    .style(|theme: &Theme, status| {
        let scheme = Scheme::of(theme);
        let layer = match status {
            iced::widget::button::Status::Hovered => scramble_ui::state_layer::HOVERED,
            iced::widget::button::Status::Pressed => scramble_ui::state_layer::PRESSED,
            _ => 0.0,
        };
        let mix = |base: f32, top: f32| base * (1.0 - layer) + top * layer;
        let background = scheme.error;
        let over = scheme.on_error;
        iced::widget::button::Style {
            background: Some(
                iced::Color::from_rgb(
                    mix(background.r, over.r),
                    mix(background.g, over.g),
                    mix(background.b, over.b),
                )
                .into(),
            ),
            text_color: over,
            border: iced::border::rounded(36),
            ..iced::widget::button::Style::default()
        }
    });
    component::tip(button, label)
}

/// The input recorded from, and a menu of the others: microphones, and
/// what the computer plays, which the sound server lists too.
fn input_menu<'a>(name: &str, context: &Context<'_>, busy: bool) -> Element<'a, RecorderControl> {
    if busy {
        return scramble_ui::styled(name.to_owned(), Type::LabelLarge)
            .style(style::on_surface_variant)
            .into();
    }
    let item = |label: String, device: Option<&InputDevice>| {
        let glyph = if device.is_some_and(plays_back) {
            Icon::Speaker
        } else {
            Icon::Mic
        };
        let id = device.map(|device| device.id.clone());
        let checked = context.chosen == id.as_deref();
        Entry::item(label, RecorderControl::Microphone(id))
            .icon(glyph)
            .checked(checked)
    };
    let (monitors, inputs): (Vec<&InputDevice>, Vec<&InputDevice>) = context
        .devices
        .iter()
        .partition(|device| plays_back(device));
    let mut entries = vec![
        Entry::heading(crate::fl!("record-microphone")),
        item(crate::fl!("record-default-microphone"), None),
    ];
    entries.extend(
        inputs
            .into_iter()
            .map(|device| item(device.name.clone(), Some(device))),
    );
    if !monitors.is_empty() {
        entries.push(Entry::divider());
        entries.push(Entry::heading(crate::fl!("record-playing")));
        entries.extend(
            monitors
                .into_iter()
                .map(|device| item(device.name.clone(), Some(device))),
        );
    }
    dropdown::menu(name.to_owned(), entries)
        .tip(crate::fl!("record-microphone"))
        .into()
}

/// Whether an input is what an output plays, as the sound server offers
/// to record.
fn plays_back(device: &InputDevice) -> bool {
    device.id.ends_with(".monitor")
}

/// A sound format, as the format menu lists it.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Format(Sound);

impl std::fmt::Display for Format {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&format::sound_name(self.0))
    }
}

/// A quality of a sound format, as the quality menu lists it.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Level(Sound, u32);

impl std::fmt::Display for Level {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Level(sound, quality) = *self;
        formatter.write_str(&if sound.lossless() {
            crate::fl!("export-bits", bits = quality)
        } else {
            crate::fl!("export-kilobits", rate = quality)
        })
    }
}

/// The format and quality to record in, fixed while recording.
fn sound_menus<'a>(context: &Context<'_>, busy: bool) -> Element<'a, RecorderControl> {
    let (sound, quality) = (context.sound, context.quality);
    if busy {
        return scramble_ui::styled(
            format!("{} · {}", Format(sound), Level(sound, quality)),
            Type::LabelLarge,
        )
        .style(style::on_surface_variant)
        .into();
    }
    let formats = dropdown::pick(
        Sound::ALL
            .into_iter()
            .filter(|sound| sound.available())
            .map(Format),
        Some(Format(sound)),
        |Format(sound)| RecorderControl::Format(sound),
    )
    .tip(crate::fl!("export-sound-format"));
    let qualities = dropdown::pick(
        sound
            .qualities()
            .iter()
            .map(|&quality| Level(sound, quality)),
        Some(Level(sound, quality)),
        |Level(_, quality)| RecorderControl::Quality(quality),
    )
    .tip(crate::fl!("export-quality"));
    scramble_ui::dir::row(vec![formats.into(), qualities.into()])
        .spacing(8)
        .align_y(Center)
        .into()
}

#[cfg(test)]
mod tests {
    use super::meter_position;

    #[test]
    fn the_meter_reads_in_decibels() {
        assert_eq!(meter_position(0.0), 0.0);
        assert_eq!(meter_position(1.0), 1.0);
        // Half the level is 6 dB down, a tenth of the way along 60.
        assert!((meter_position(0.5) - 0.9).abs() < 0.01);
        assert_eq!(meter_position(0.0001), 0.0);
    }
}
