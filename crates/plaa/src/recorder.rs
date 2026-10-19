//! A window recording sound or the screen, as QuickTime Player's New
//! Audio Recording and New Screen Recording: a level meter that moves
//! before recording starts, a button to start and stop, how long and how
//! large the recording is, and what to record from and in.

use std::time::Duration;

use iced::widget::{container, progress_bar, space};
use iced::{Center, Element, Fill, Theme};
use plaa_media::export::{Format, Sound};
use plaa_media::record::{InputDevice, Microphone};
use plaa_media::screen::ScreenCapture;
use scramble_ui::button;
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
    /// A screen recording takes no sound.
    Silent,
    Format(Sound),
    /// The sound's quality: kb/s, or bits for lossless sound.
    Quality(u32),
    /// What a screen recording is recorded as.
    Video(Format),
    /// Asks the system again what to record.
    Reshare,
    Settings,
    Close,
}

/// How quiet the meter shows, in decibels below full scale.
const FLOOR: f32 = 60.0;
/// How far the meter falls back each tick, as a share of its length.
const FALL: f32 = 0.03;

pub struct Recorder {
    /// The input device, or why it would not open; none for a screen
    /// recording without sound.
    pub microphone: Option<Result<Microphone, String>>,
    /// The meter's levels, 0 to 1 along it, falling back gently.
    pub levels: Vec<f32>,
    /// Stopped, and the file is being finished.
    pub saving: bool,
    /// What a screen recording records.
    pub screen: Option<Screen>,
}

/// The screen or window a screen recording records.
pub struct Screen {
    /// Its pictures, or why they would not come.
    pub capture: Result<ScreenCapture, String>,
    /// A window is shared, not a whole screen.
    pub window: bool,
    /// Keeps it shared, until dropped.
    #[cfg(target_os = "linux")]
    pub _sharing: crate::screen_share::Sharing,
}

impl Recorder {
    /// Records sound from input device `device`, or the system's default.
    pub fn open(device: Option<&str>) -> Self {
        Self {
            microphone: Some(Microphone::open(device)),
            levels: Vec::new(),
            saving: false,
            screen: None,
        }
    }

    /// Records `screen`, with the sound of `sound` when given: an input
    /// device, or the system's default.
    #[cfg(target_os = "linux")]
    pub fn screen(screen: Screen, sound: Option<Option<&str>>) -> Self {
        Self {
            microphone: sound.map(Microphone::open),
            levels: Vec::new(),
            saving: false,
            screen: Some(screen),
        }
    }

    pub fn microphone(&self) -> Option<&Microphone> {
        self.microphone.as_ref()?.as_ref().ok()
    }

    pub fn capture_mut(&mut self) -> Option<&mut ScreenCapture> {
        self.screen.as_mut()?.capture.as_mut().ok()
    }

    pub fn is_recording(&self) -> bool {
        match &self.screen {
            Some(screen) => screen
                .capture
                .as_ref()
                .is_ok_and(ScreenCapture::is_recording),
            None => self.microphone().is_some_and(Microphone::is_recording),
        }
    }

    /// Takes the device's levels since the last tick into the meter. A
    /// screen no longer shared ends its recording.
    pub fn tick(&mut self) {
        if let Some(capture) = self.capture_mut()
            && capture.is_recording()
            && capture.ended_because().is_some()
        {
            capture.stop();
            self.saving = true;
        }
        let Some(microphone) = self.microphone() else {
            self.levels.clear();
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
    /// What a screen recording is recorded as.
    pub video: Format,
    pub close_button: bool,
}

pub fn view<'a>(recorder: &'a Recorder, context: &Context<'_>) -> Element<'a, RecorderControl> {
    let (glyph, title) = if recorder.screen.is_some() {
        (Icon::ScreenRecord, crate::fl!("record-screen-title"))
    } else {
        (Icon::Mic, crate::fl!("record-title"))
    };
    let mut tools: Vec<Element<'a, RecorderControl>> = vec![
        icon::icon(glyph, 20)
            .style(style::on_surface_variant)
            .into(),
        scramble_ui::styled(title, Type::TitleMedium).into(),
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
    let body = match &recorder.screen {
        Some(screen) => screen_body(recorder, screen, context),
        None => sound_body(recorder, context),
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

/// A sound recording's controls.
fn sound_body<'a>(recorder: &'a Recorder, context: &Context<'_>) -> Element<'a, RecorderControl> {
    let microphone = match &recorder.microphone {
        Some(Ok(microphone)) => microphone,
        Some(Err(error)) => {
            return component::empty_state(Icon::MicOff, crate::fl!("record-no-input"), error);
        }
        None => {
            return component::empty_state(Icon::MicOff, crate::fl!("record-no-input"), "");
        }
    };
    let recording = recorder.is_recording();
    let busy = recording || recorder.saving;
    let (time, size) = progress(microphone.recorded(), busy);
    container(
        column![
            time,
            size,
            menu_or_chosen(
                dropdown::menu(microphone.name().to_owned(), input_entries(context, true))
                    .tip(crate::fl!("record-microphone")),
                busy,
                microphone.name()
            ),
            meters(&recorder.levels),
            sound_menus(context, busy),
            scramble_ui::styled(
                recorded_format(microphone.format(), context),
                Type::BodySmall
            )
            .style(style::on_surface_variant),
            record_button(recording, !recorder.saving),
        ]
        .spacing(12)
        .align_x(Center),
    )
    .center(Fill)
    .into()
}

/// A screen recording's controls: what is shared, the sound it takes, and
/// the format it is recorded in.
fn screen_body<'a>(
    recorder: &'a Recorder,
    screen: &'a Screen,
    context: &Context<'_>,
) -> Element<'a, RecorderControl> {
    let capture = match &screen.capture {
        Ok(capture) => capture,
        Err(error) => {
            return column![
                component::empty_state(
                    Icon::ScreenRecord,
                    crate::fl!("record-screen-failed"),
                    error
                ),
                scramble_ui::button(button::Kind::Tonal, crate::fl!("record-screen-again"))
                    .on_press(RecorderControl::Reshare),
            ]
            .spacing(12)
            .align_x(Center)
            .into();
        }
    };
    let recording = recorder.is_recording();
    let busy = recording || recorder.saving;
    let (time, size) = progress(capture.recorded(), busy);
    let ended = capture.ended_because().is_some();
    let shared = if ended {
        crate::fl!("record-screen-ended")
    } else {
        let what = if screen.window {
            crate::fl!("record-screen-window")
        } else {
            crate::fl!("record-screen-whole")
        };
        match capture.size() {
            Some((width, height)) => crate::fl!(
                "record-screen-size",
                what = what,
                width = width,
                height = height
            ),
            None => what,
        }
    };
    let glyph = if screen.window {
        Icon::DesktopWindows
    } else {
        Icon::ScreenshotMonitor
    };
    let mut source: Vec<Element<'a, RecorderControl>> = vec![
        icon::icon(glyph, 20)
            .style(style::on_surface_variant)
            .into(),
        scramble_ui::styled(shared, Type::LabelLarge)
            .style(style::on_surface_variant)
            .into(),
    ];
    if !busy {
        source.push(
            scramble_ui::button(button::Kind::Text, crate::fl!("record-screen-again"))
                .size(button::Size::Small)
                .on_press(RecorderControl::Reshare)
                .into(),
        );
    }
    let sound_name = match &recorder.microphone {
        None => crate::fl!("record-no-sound"),
        Some(Ok(microphone)) => microphone.name().to_owned(),
        Some(Err(_)) => crate::fl!("record-no-input"),
    };
    let mut entries = vec![
        Entry::item(crate::fl!("record-no-sound"), RecorderControl::Silent)
            .icon(Icon::MicOff)
            .checked(recorder.microphone.is_none()),
        Entry::divider(),
    ];
    entries.extend(input_entries(context, recorder.microphone.is_some()));
    let mut controls = column![
        time,
        size,
        scramble_ui::dir::row(source).spacing(8).align_y(Center),
        menu_or_chosen(
            dropdown::menu(sound_name.clone(), entries).tip(crate::fl!("record-sound")),
            busy,
            &sound_name
        ),
    ]
    .spacing(12)
    .align_x(Center);
    if let Some(Err(error)) = &recorder.microphone {
        controls = controls.push(
            scramble_ui::styled(error.clone(), Type::BodySmall).style(style::on_surface_variant),
        );
    }
    if recorder.microphone().is_some() {
        controls = controls.push(meters(&recorder.levels));
    }
    controls = controls
        .push(video_menu(context.video, busy))
        .push(record_button(recording, !recorder.saving && !ended));
    container(controls).center(Fill).into()
}

/// How long the recording lasts and how large it is, shown while it goes
/// and as it is saved.
fn progress<'a>(
    (recorded, written): (Duration, u64),
    busy: bool,
) -> (Element<'a, RecorderControl>, Element<'a, RecorderControl>) {
    let time = format::precise_time(
        if busy { recorded.as_secs_f64() } else { 0.0 },
        recorded >= Duration::from_secs(3600),
    );
    let size = scramble_ui::styled(
        if busy {
            format::size(written)
        } else {
            String::new()
        },
        Type::BodyMedium,
    )
    .style(style::on_surface_variant);
    (
        scramble_ui::styled(time, Type::DisplaySmall).into(),
        size.into(),
    )
}

/// A meter for each of the input's channels.
fn meters<'a>(levels: &[f32]) -> Element<'a, RecorderControl> {
    iced::widget::Column::with_children(levels.iter().map(|level| {
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
    .width(280)
    .into()
}

/// The input's rate and channels, which it records at, and the rate the
/// file is saved at when its format will not take the input's.
fn recorded_format((rate, channels): (u32, u16), context: &Context<'_>) -> String {
    let channels = match channels {
        1 => crate::fl!("record-mono"),
        2 => crate::fl!("record-stereo"),
        count => crate::fl!("record-channels", count = count),
    };
    let saved = context.sound.rate_for(context.quality, rate);
    if saved < rate {
        crate::fl!(
            "record-input-format-lowered",
            rate = format::kilohertz(rate),
            channels = channels,
            saved = format::kilohertz(saved)
        )
    } else {
        crate::fl!(
            "record-input-format",
            rate = format::kilohertz(rate),
            channels = channels
        )
    }
}

/// The round button that starts recording, red as a recording light, and
/// stops it.
fn record_button<'a>(recording: bool, enabled: bool) -> Element<'a, RecorderControl> {
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
    .on_press_maybe(enabled.then_some(RecorderControl::Toggle))
    .style(|theme: &Theme, status| {
        let scheme = Scheme::of(theme);
        let layer = match status {
            iced::widget::button::Status::Hovered => scramble_ui::state_layer::HOVERED,
            iced::widget::button::Status::Pressed => scramble_ui::state_layer::PRESSED,
            _ => 0.0,
        };
        let mix = |base: f32, top: f32| base * (1.0 - layer) + top * layer;
        let (background, over) = if matches!(status, iced::widget::button::Status::Disabled) {
            (scheme.surface_container_highest, scheme.on_surface_variant)
        } else {
            (scheme.error, scheme.on_error)
        };
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

/// The inputs to record from: microphones, and what the computer plays,
/// which the sound server lists too. One is checked when `choosing`.
fn input_entries(context: &Context<'_>, choosing: bool) -> Vec<Entry<RecorderControl>> {
    let item = |label: String, device: Option<&InputDevice>| {
        let glyph = if device.is_some_and(plays_back) {
            Icon::Speaker
        } else {
            Icon::Mic
        };
        let id = device.map(|device| device.id.clone());
        let checked = choosing && context.chosen == id.as_deref();
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
    entries
}

/// A menu, fixed while recording: then only what it chose shows.
fn menu_or_chosen<'a>(
    menu: dropdown::Dropdown<RecorderControl>,
    busy: bool,
    chosen: &str,
) -> Element<'a, RecorderControl> {
    if busy {
        scramble_ui::styled(chosen.to_owned(), Type::LabelLarge)
            .style(style::on_surface_variant)
            .into()
    } else {
        menu.into()
    }
}

/// Whether an input is what an output plays, as the sound server offers
/// to record.
fn plays_back(device: &InputDevice) -> bool {
    device.id.ends_with(".monitor")
}

/// A sound format, as the format menu lists it.
#[derive(Debug, Clone, Copy, PartialEq)]
struct SoundFormat(Sound);

impl std::fmt::Display for SoundFormat {
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
            format!("{} · {}", SoundFormat(sound), Level(sound, quality)),
            Type::LabelLarge,
        )
        .style(style::on_surface_variant)
        .into();
    }
    let formats = dropdown::pick(
        Sound::ALL
            .into_iter()
            .filter(|sound| sound.available())
            .map(SoundFormat),
        Some(SoundFormat(sound)),
        |SoundFormat(sound)| RecorderControl::Format(sound),
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

/// A picture format, as the screen recording's format menu lists it.
#[derive(Debug, Clone, Copy, PartialEq)]
struct VideoFormat(Format);

impl std::fmt::Display for VideoFormat {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&match self.0 {
            Format::Hevc => crate::fl!("export-hevc"),
            Format::Av1 => crate::fl!("export-av1"),
            _ => crate::fl!("export-h264"),
        })
    }
}

/// The format a screen recording is recorded in, fixed while recording.
fn video_menu<'a>(video: Format, busy: bool) -> Element<'a, RecorderControl> {
    if busy {
        return scramble_ui::styled(VideoFormat(video).to_string(), Type::LabelLarge)
            .style(style::on_surface_variant)
            .into();
    }
    dropdown::pick(
        Format::VIDEO.into_iter().map(VideoFormat),
        Some(VideoFormat(video)),
        |VideoFormat(format)| RecorderControl::Video(format),
    )
    .tip(crate::fl!("export-format"))
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
