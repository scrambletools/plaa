//! A window recording sound, the screen or a camera, as QuickTime
//! Player's New Audio, Screen and Movie Recording: a level meter that
//! moves before recording starts, the camera's picture, a button to start
//! and stop, how long and how large the recording is, and what to record
//! from and in.

use std::path::PathBuf;
use std::time::Duration;

use iced::widget::{container, image, opaque, progress_bar, slider, space, stack, toggler};
use iced::{Center, Element, Fill, Theme};
use plaa_media::capture::{Camera, Capture, Preview};
use plaa_media::export::{Format, Sound};
use plaa_media::record::{InputDevice, Microphone};
use scramble_ui::button;
use scramble_ui::component::Backdrop;
use scramble_ui::dropdown::{self, Entry};
use scramble_ui::{Icon, Scheme, Type, column, component, icon, style};

use crate::format;
use crate::window::{Control, Menu};

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
    /// The input taken this many decibels louder, as the slider moves.
    Gain(f32),
    /// The slider was let go: the input keeps that level.
    GainSet,
    /// Recordings even out the sound's level, or not.
    Leveled(bool),
    /// What a screen recording is recorded as.
    Video(Format),
    /// The most pictures a second a screen recording keeps; 0 for every
    /// picture the screen sends.
    Rate(u32),
    /// Asks the system again what to record.
    Reshare,
    /// Records this screen or window, where plaa offers them.
    Choose(Offered),
    /// The menu of what to record opened, or closed.
    Choosing(bool),
    /// Records from this camera.
    Camera(Camera),
    /// Records the screen with plaa hidden, as the warning said.
    Hide,
    /// Closes the warning, recording nothing.
    Cancel,
    Settings,
    Close,
    /// What the toolbar's Open, Connect, Network Audio and assistant
    /// tools send, as in a player window.
    Window(Control),
    /// The finished recording's name, as typed.
    Name(String),
    /// The finished recording takes the name typed.
    Rename,
    /// The finished recording keeps the name it was saved with.
    KeepName,
}

/// The text field a finished recording's name is typed in.
pub const NAME_FIELD: &str = "recording-name";

/// How much louder an input can be taken, in decibels.
const MOST_GAIN: f32 = 30.0;

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
    /// The warning before recording a screen, which hides plaa.
    pub warning: Option<Warning>,
    /// The window closes while it records the screen, so as not to be in
    /// the recording.
    pub hiding: bool,
    /// The menu of what to record is open.
    pub choosing: bool,
    /// The Open or Connect menu is open, or a listener's formats in the
    /// Network Audio sheet.
    pub menu: Option<Menu>,
    /// The Network Audio sheet is open beside the controls.
    pub network: bool,
    /// The listener chosen in the Network Audio sheet.
    pub listener: Option<(plaa_avb::EntityId, u16)>,
    /// The recording just saved, asking for its name.
    pub naming: Option<Naming>,
    /// A camera's latest picture as the window shows it, mirrored, and
    /// its count.
    pub preview: Option<(u64, image::Handle)>,
}

/// A screen or window plaa offers to record, where it lists them itself,
/// as on Windows.
#[derive(Debug, Clone, PartialEq)]
pub struct Offered {
    /// The screen's number, or the window's handle.
    pub id: u64,
    pub window: bool,
    pub name: String,
}

/// A finished recording, waiting for the name the user gives it.
pub struct Naming {
    /// Where it was saved.
    pub path: PathBuf,
    /// The name as typed, without the file's extension.
    pub name: String,
    /// Why the name typed could not be given.
    pub error: Option<String>,
}

/// How a recording that hides plaa is stopped, as its warning says.
pub struct Warning {
    /// A dot in the tray stops it.
    pub tray: bool,
}

/// The screen or window a screen recording records.
pub struct Screen {
    /// Its pictures, or why they would not come.
    pub capture: Result<Capture, String>,
    /// A window is shared, not a whole screen.
    pub window: bool,
    /// What plaa offered and this records, where it offers them.
    pub offered: Option<Offered>,
    /// The camera this records, rather than a screen.
    pub camera: Option<Camera>,
    /// Keeps a screen shared, until dropped.
    #[cfg(target_os = "linux")]
    pub _sharing: Option<crate::screen_share::Sharing>,
}

impl Recorder {
    /// Records sound from input device `device`, or the system's default.
    pub fn open(device: Option<&str>) -> Self {
        Self {
            microphone: Some(Microphone::open(device)),
            levels: Vec::new(),
            saving: false,
            screen: None,
            warning: None,
            hiding: false,
            choosing: false,
            menu: None,
            network: false,
            listener: None,
            naming: None,
            preview: None,
        }
    }

    /// Records `screen`, with the sound of `sound` when given: an input
    /// device, or the system's default.
    pub fn screen(screen: Screen, sound: Option<Option<&str>>) -> Self {
        Self {
            microphone: sound.map(Microphone::open),
            levels: Vec::new(),
            saving: false,
            screen: Some(screen),
            warning: None,
            hiding: false,
            choosing: false,
            menu: None,
            network: false,
            listener: None,
            naming: None,
            preview: None,
        }
    }

    pub fn microphone(&self) -> Option<&Microphone> {
        self.microphone.as_ref()?.as_ref().ok()
    }

    pub fn capture_mut(&mut self) -> Option<&mut Capture> {
        self.screen.as_mut()?.capture.as_mut().ok()
    }

    pub fn is_recording(&self) -> bool {
        match &self.screen {
            Some(screen) => screen.capture.as_ref().is_ok_and(Capture::is_recording),
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
        // A camera's newest picture, mirrored, as people see themselves.
        if let Some(screen) = &self.screen
            && screen.camera.is_some()
            && let Ok(capture) = &screen.capture
            && let Some(preview) = capture.preview()
            && self
                .preview
                .as_ref()
                .is_none_or(|(count, _)| *count != preview.count)
        {
            self.preview = Some((preview.count, mirrored(&preview)));
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

/// `preview` flipped left to right, as a mirror shows.
fn mirrored(preview: &Preview) -> image::Handle {
    let row = preview.width as usize * 4;
    let mut pixels = Vec::with_capacity(preview.rgba.len());
    for line in preview.rgba.chunks_exact(row) {
        for pixel in line.as_chunks::<4>().0.iter().rev() {
            pixels.extend_from_slice(pixel);
        }
    }
    image::Handle::from_rgba(preview.width, preview.height, pixels)
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
    /// The most pictures a second it keeps; 0 for every picture.
    pub rate: u32,
    /// How much louder the input is taken, in decibels.
    pub gain: f32,
    /// Recordings even out the sound's level.
    pub leveled: bool,
    /// The screens and windows plaa offers to record, where it lists them.
    pub offers: &'a [Offered],
    /// The cameras there are.
    pub cameras: &'a [Camera],
    pub close_button: bool,
    /// The AVB network, while plaa is on one.
    pub avb: Option<std::sync::Arc<plaa_avb::Snapshot>>,
    /// The assistant's panel is open beside the window.
    pub assistant_open: bool,
}

pub fn view<'a>(recorder: &'a Recorder, context: &Context<'_>) -> Element<'a, RecorderControl> {
    let (glyph, title) = match &recorder.screen {
        Some(screen) if screen.camera.is_some() => {
            (Icon::Videocam, crate::fl!("record-camera-title"))
        }
        Some(_) => (Icon::ScreenRecord, crate::fl!("record-screen-title")),
        None => (Icon::Mic, crate::fl!("record-title")),
    };
    // Opening and connecting first, as in a player window, so more can
    // be opened from here.
    let mut tools: Vec<Element<'a, RecorderControl>> = vec![
        crate::window::open_menu(recorder.menu == Some(Menu::Open)).map(RecorderControl::Window),
    ];
    if let Some(snapshot) = &context.avb {
        tools.push(
            crate::window::connect_menu(snapshot, recorder.menu == Some(Menu::Connect))
                .map(RecorderControl::Window),
        );
    }
    tools.extend([
        icon::icon(glyph, 20)
            .style(style::on_surface_variant)
            .into(),
        scramble_ui::styled(title, Type::TitleMedium).into(),
        space().width(Fill).into(),
    ]);
    // The player's tools that need no file, so they stay while recording.
    if context.avb.is_some() {
        tools.push(component::toggle_tool(
            Icon::Lan,
            crate::fl!("action-network"),
            recorder.network,
            RecorderControl::Window(Control::ToggleNetwork),
        ));
    }
    tools.push(component::toggle_tool(
        Icon::SmartToy,
        crate::fl!("action-assistant"),
        context.assistant_open,
        RecorderControl::Window(Control::ToggleAssistant),
    ));
    tools.push(component::tool(
        Icon::Settings,
        crate::fl!("action-settings"),
        Some(RecorderControl::Settings),
    ));
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
    let body = match (&recorder.screen, &recorder.warning) {
        (Some(_), Some(warning)) => warning_body(warning),
        (Some(screen), None) if screen.camera.is_some() => camera_body(recorder, screen, context),
        (Some(screen), None) => screen_body(recorder, screen, context),
        (None, _) => sound_body(recorder, context),
    };
    let body = container(body)
        .width(Fill)
        .height(Fill)
        .style(style::surface);
    let window = column![toolbar, body];
    match &recorder.naming {
        Some(naming) => stack![window, naming_dialog(naming)].into(),
        None => window.into(),
    }
}

/// The Network Audio sheet beside the window, the whole of its height,
/// when open on an AVB network.
pub fn side_sheet<'a>(
    recorder: &Recorder,
    avb: Option<std::sync::Arc<plaa_avb::Snapshot>>,
) -> Option<Element<'a, RecorderControl>> {
    let snapshot = avb.filter(|_| recorder.network)?;
    Some(
        crate::network::sheet(snapshot, recorder.menu, recorder.listener)
            .map(RecorderControl::Window),
    )
}

/// Asks the name of the recording just saved: kept as it is, or the one
/// typed, why it could not be given below it.
fn naming_dialog(naming: &Naming) -> Element<'_, RecorderControl> {
    let shown = |path: &std::path::Path| {
        plaa_store::paths::abbreviate_home(path)
            .display()
            .to_string()
    };
    let note = crate::fl!(
        "record-name-note",
        name = naming
            .path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default(),
        folder = naming.path.parent().map(shown).unwrap_or_default()
    );
    let field = component::text_field(
        crate::fl!("record-name-label"),
        &naming.name,
        Backdrop::ContainerHigh,
        |input| {
            input
                .id(NAME_FIELD)
                .on_input(RecorderControl::Name)
                .on_submit(RecorderControl::Rename)
        },
    );
    let mut body = column![
        scramble_ui::styled(crate::fl!("record-name-title"), Type::HeadlineSmall),
        scramble_ui::aligned(
            scramble_ui::styled(note, Type::BodyMedium).style(style::on_surface_variant)
        ),
        field,
    ]
    .spacing(16);
    if let Some(error) = &naming.error {
        body = body.push(scramble_ui::aligned(
            scramble_ui::styled(error, Type::BodyMedium).style(style::error_text),
        ));
    }
    let actions = vec![
        scramble_ui::button(button::Kind::Text, crate::fl!("record-name-keep"))
            .on_press(RecorderControl::KeepName)
            .into(),
        scramble_ui::button(button::Kind::Filled, crate::fl!("record-name-rename"))
            .on_press_maybe((!naming.name.trim().is_empty()).then_some(RecorderControl::Rename))
            .into(),
    ];
    let body = body.push(container(scramble_ui::dir::row(actions).spacing(8)).align_right(Fill));
    let card = container(body)
        .padding(24)
        .width(Fill)
        .max_width(420)
        .style(style::dialog);
    // Nothing under it takes the pointer while it asks.
    opaque(container(card).center(Fill).padding(16).style(style::scrim))
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
    let (time, size) = progress(microphone.recorded(), 0, busy);
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
            level_controls(context, busy),
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

/// A camera recording: the camera's picture, and below it, how long and
/// how large the recording is, the button, the camera, the sound it takes
/// and the format.
fn camera_body<'a>(
    recorder: &'a Recorder,
    screen: &'a Screen,
    context: &Context<'_>,
) -> Element<'a, RecorderControl> {
    let recording = recorder.is_recording();
    let busy = recording || recorder.saving;
    let (picture, progress_line): (Element<'a, RecorderControl>, _) = match &screen.capture {
        Ok(capture) => {
            let picture: Element<'a, RecorderControl> = match &recorder.preview {
                Some((_, handle)) => image(handle.clone())
                    .content_fit(iced::ContentFit::Contain)
                    .width(Fill)
                    .height(Fill)
                    .into(),
                None => space().width(Fill).height(Fill).into(),
            };
            (
                picture,
                Some(progress(capture.recorded(), capture.dropped(), busy)),
            )
        }
        Err(error) => (
            component::empty_state(Icon::VideocamOff, crate::fl!("record-camera-failed"), error),
            None,
        ),
    };
    let picture = container(picture)
        .width(Fill)
        .height(Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Color::BLACK.into()),
            ..container::Style::default()
        });
    let camera_name = screen
        .camera
        .as_ref()
        .map(|camera| camera.name.clone())
        .unwrap_or_default();
    let cameras = context
        .cameras
        .iter()
        .map(|camera| {
            Entry::item(camera.name.clone(), RecorderControl::Camera(camera.clone()))
                .icon(Icon::Videocam)
                .checked(screen.camera.as_ref() == Some(camera))
        })
        .collect();
    let camera_menu = menu_or_chosen(
        dropdown::menu(camera_name.clone(), cameras)
            .tip(crate::fl!("record-camera"))
            .open(recorder.choosing, RecorderControl::Choosing),
        busy,
        &camera_name,
    );
    let sound_name = match &recorder.microphone {
        None => crate::fl!("record-no-sound"),
        Some(Ok(microphone)) => microphone.name().to_owned(),
        Some(Err(_)) => crate::fl!("record-no-input"),
    };
    let mut sounds = vec![
        Entry::item(crate::fl!("record-no-sound"), RecorderControl::Silent)
            .icon(Icon::MicOff)
            .checked(recorder.microphone.is_none()),
        Entry::divider(),
    ];
    sounds.extend(input_entries(context, recorder.microphone.is_some()));
    let sound_menu = menu_or_chosen(
        dropdown::menu(sound_name.clone(), sounds).tip(crate::fl!("record-sound")),
        busy,
        &sound_name,
    );
    let (time, size) = match progress_line {
        Some(line) => line,
        None => (space().into(), space().into()),
    };
    let ready = screen.capture.is_ok() && !recorder.saving;
    let mut controls = column![
        scramble_ui::dir::row(vec![time, size])
            .spacing(16)
            .align_y(Center),
        record_button(recording, ready),
        scramble_ui::dir::row(vec![camera_menu, sound_menu])
            .spacing(8)
            .align_y(Center),
    ]
    .spacing(10)
    .align_x(Center);
    if recorder.microphone().is_some() {
        controls = controls
            .push(meters(&recorder.levels))
            .push(level_controls(context, busy));
    }
    controls = controls.push(video_menus(context.video, context.rate, busy));
    column![
        picture,
        container(controls)
            .width(Fill)
            .padding([12, 16])
            .center_x(Fill)
    ]
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
    let (time, size) = progress(capture.recorded(), capture.dropped(), busy);
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
    ];
    match &screen.offered {
        // Offered by plaa: a menu of the others.
        Some(offered) if !busy => {
            let label = match capture.size() {
                Some((width, height)) if !ended => crate::fl!(
                    "record-screen-size",
                    what = shortened(&offered.name),
                    width = width,
                    height = height
                ),
                _ => shared,
            };
            source.push(
                dropdown::menu(label, offer_entries(context.offers, offered))
                    .tip(crate::fl!("record-screen-what"))
                    .open(recorder.choosing, RecorderControl::Choosing)
                    .into(),
            );
        }
        Some(offered) => source.push(
            scramble_ui::styled(shortened(&offered.name), Type::LabelLarge)
                .style(style::on_surface_variant)
                .into(),
        ),
        // Picked through the system: it is asked again.
        None => {
            source.push(
                scramble_ui::styled(shared, Type::LabelLarge)
                    .style(style::on_surface_variant)
                    .into(),
            );
            if !busy {
                source.push(
                    scramble_ui::button(button::Kind::Text, crate::fl!("record-screen-again"))
                        .size(button::Size::Small)
                        .on_press(RecorderControl::Reshare)
                        .into(),
                );
            }
        }
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
        controls = controls
            .push(meters(&recorder.levels))
            .push(level_controls(context, busy));
    }
    controls = controls
        .push(video_menus(context.video, context.rate, busy))
        .push(record_button(recording, !recorder.saving && !ended));
    container(controls).center(Fill).into()
}

/// What happens once a screen's recording starts: plaa hides, and how
/// it is stopped then.
fn warning_body<'a>(warning: &Warning) -> Element<'a, RecorderControl> {
    let stopping = if warning.tray {
        crate::fl!("record-hide-tray")
    } else {
        crate::fl!("record-hide-reopen")
    };
    container(
        column![
            icon::icon(Icon::Info, 40).style(style::primary_text),
            scramble_ui::styled(crate::fl!("record-hide-title"), Type::TitleLarge),
            scramble_ui::styled(stopping, Type::BodyMedium)
                .style(style::on_surface_variant)
                .align_x(Center)
                .width(320),
            scramble_ui::dir::row(vec![
                scramble_ui::button(button::Kind::Text, crate::fl!("common-cancel"))
                    .on_press(RecorderControl::Cancel)
                    .into(),
                scramble_ui::button(button::Kind::Filled, crate::fl!("record-hide-start"))
                    .on_press(RecorderControl::Hide)
                    .into(),
            ])
            .spacing(8),
        ]
        .spacing(16)
        .align_x(Center),
    )
    .center(Fill)
    .into()
}

/// The screens and windows to record, `chosen` checked.
fn offer_entries(offers: &[Offered], chosen: &Offered) -> Vec<Entry<RecorderControl>> {
    let item = |offered: &Offered| {
        let glyph = if offered.window {
            Icon::DesktopWindows
        } else {
            Icon::ScreenshotMonitor
        };
        Entry::item(
            shortened(&offered.name),
            RecorderControl::Choose(offered.clone()),
        )
        .icon(glyph)
        .checked(offered.id == chosen.id && offered.window == chosen.window)
    };
    let (windows, screens): (Vec<&Offered>, Vec<&Offered>) =
        offers.iter().partition(|offered| offered.window);
    let mut entries = vec![Entry::heading(crate::fl!("record-screens"))];
    entries.extend(screens.into_iter().map(item));
    if !windows.is_empty() {
        entries.push(Entry::divider());
        entries.push(Entry::heading(crate::fl!("record-windows")));
        entries.extend(windows.into_iter().map(item));
    }
    entries
}

/// A window's title, cut short where it is long.
fn shortened(name: &str) -> String {
    const MOST: usize = 48;
    if name.chars().count() <= MOST {
        return name.to_owned();
    }
    let kept: String = name.chars().take(MOST - 1).collect();
    format!("{}…", kept.trim_end())
}

/// How long the recording lasts and how large it is, shown while it goes
/// and as it is saved.
fn progress<'a>(
    (recorded, written): (Duration, u64),
    dropped: u64,
    busy: bool,
) -> (Element<'a, RecorderControl>, Element<'a, RecorderControl>) {
    let time = format::precise_time(
        if busy { recorded.as_secs_f64() } else { 0.0 },
        recorded >= Duration::from_secs(3600),
    );
    // Pictures dropped as the encoder fell behind, where there were any.
    let size = match (busy, dropped) {
        (false, _) => String::new(),
        (true, 0) => format::size(written),
        (true, count) => crate::fl!(
            "record-dropped",
            size = format::size(written),
            count = count
        ),
    };
    let size = scramble_ui::styled(size, Type::BodyMedium).style(style::on_surface_variant);
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

/// How much louder the input is taken, which the meter shows and may
/// change while recording, and whether recordings even out its level,
/// fixed while recording.
fn level_controls<'a>(context: &Context<'_>, busy: bool) -> Element<'a, RecorderControl> {
    let gain = slider(0.0..=MOST_GAIN, context.gain, RecorderControl::Gain)
        .step(1.0_f32)
        .on_release(RecorderControl::GainSet)
        .width(150)
        .height(style::SLIDER_HEIGHT)
        .style(|theme: &Theme, status| style::slider(Scheme::of(theme).surface)(theme, status));
    let louder = scramble_ui::dir::row(vec![
        scramble_ui::styled(crate::fl!("record-gain"), Type::BodyMedium)
            .style(style::on_surface_variant)
            .into(),
        gain.into(),
        scramble_ui::styled(
            {
                let decibels = context.gain.round() as i32;
                crate::fl!("record-gain-value", decibels = decibels)
            },
            Type::BodyMedium,
        )
        .style(style::on_surface_variant)
        .width(52)
        .into(),
    ])
    .spacing(10)
    .align_y(Center);
    let leveling = toggler(context.leveled)
        .label(crate::fl!("record-level-automatically"))
        .size(20)
        .style(style::switch);
    let leveling = if busy {
        leveling
    } else {
        leveling.on_toggle(RecorderControl::Leveled)
    };
    column![louder, leveling].spacing(8).align_x(Center).into()
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

/// The most pictures a second, as the rate menu lists it: 0 for every
/// picture the screen sends.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Rate(u32);

impl Rate {
    const OFFERED: [u32; 5] = [24, 30, 60, 120, 0];
}

impl std::fmt::Display for Rate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&match self.0 {
            0 => crate::fl!("record-rate-every"),
            rate => crate::fl!("record-rate", rate = rate),
        })
    }
}

/// The format a screen recording is recorded in, and its pictures a
/// second, fixed while recording.
fn video_menus<'a>(video: Format, rate: u32, busy: bool) -> Element<'a, RecorderControl> {
    if busy {
        return scramble_ui::styled(
            format!("{} · {}", VideoFormat(video), Rate(rate)),
            Type::LabelLarge,
        )
        .style(style::on_surface_variant)
        .into();
    }
    let formats = dropdown::pick(
        Format::VIDEO.into_iter().map(VideoFormat),
        Some(VideoFormat(video)),
        |VideoFormat(format)| RecorderControl::Video(format),
    )
    .tip(crate::fl!("export-format"));
    let rates = dropdown::pick(
        Rate::OFFERED.into_iter().map(Rate),
        Some(Rate(rate)),
        |Rate(rate)| RecorderControl::Rate(rate),
    )
    .tip(crate::fl!("record-rate-tip"));
    scramble_ui::dir::row(vec![formats.into(), rates.into()])
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
