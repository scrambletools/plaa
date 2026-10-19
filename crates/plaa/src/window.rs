//! A window playing one file: the picture or the cover art, the toolbar
//! and the playback controls, docked or floating over the picture, and the
//! inspector.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use iced::widget::{container, image, mouse_area, slider, space, stack, text};
use iced::{Center, Element, Fill, Length, Theme};
use plaa_media::subtitle::Cue;
use plaa_media::{MediaInfo, Player, Source, Status};
use scramble_ui::component::{self, Backdrop};
use scramble_ui::dir::row;
use scramble_ui::dropdown::{self, Entry};
use scramble_ui::{Icon, Scheme, Type, button, column, icon, style};

use crate::format;
use crate::timeline::Timeline;

/// The speeds the speed menu offers.
pub const SPEEDS: [f64; 8] = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 2.0, 3.0];

/// How far the skip buttons and arrow keys move, in seconds.
pub const SKIP: f64 = 10.0;
pub const SKIP_SHORT: f64 = 5.0;

/// The menus a window can have open, which keep its overlay shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    Speed,
    Tracks,
}

/// When the pointer last moved over the picture, kept where the picture
/// widget sets it without rebuilding the window on every move.
#[derive(Debug)]
pub struct Activity {
    moved: Mutex<Instant>,
    /// Whether the overlay shows, as the window last drew it.
    shown: AtomicBool,
}

impl Activity {
    fn new() -> Self {
        Self {
            moved: Mutex::new(Instant::now()),
            shown: AtomicBool::new(true),
        }
    }

    /// Notes a move. Returns whether the overlay was hidden, so the window
    /// must show it.
    pub fn touch(&self) -> bool {
        *self.moved.lock().unwrap_or_else(PoisonError::into_inner) = Instant::now();
        !self.shown.load(Ordering::Relaxed)
    }

    fn idle(&self) -> Duration {
        self.moved
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .elapsed()
    }
}

pub struct PlayerWindow {
    pub player: Player,
    /// Tells this window's pictures from others' on the graphics card.
    pub surface: u64,
    pub info: Option<Arc<MediaInfo>>,
    pub cover: Option<image::Handle>,
    pub status: Status,
    /// The graphics card API pictures decode on, or none for the
    /// processor; unknown before the file opens.
    pub decoding: Option<Option<&'static str>>,
    pub error: Option<String>,
    /// Why the file plays without sound, if it does.
    pub no_sound: Option<String>,
    /// The timeline's value while it is dragged, in seconds.
    pub dragging: Option<f64>,
    pub activity: Arc<Activity>,
    /// Whether the pointer is inside the window.
    pub pointer_inside: bool,
    pub overlay_shown: bool,
    pub menu: Option<Menu>,
    pub inspector: bool,
    pub looping: bool,
    pub subtitle: Option<Cue>,
    pub volume: f32,
    pub muted: bool,
}

impl PlayerWindow {
    pub fn new(player: Player, surface: u64, volume: f32, muted: bool) -> Self {
        Self {
            status: player.status(),
            player,
            surface,
            info: None,
            cover: None,
            decoding: None,
            error: None,
            no_sound: None,
            dragging: None,
            activity: Arc::new(Activity::new()),
            pointer_inside: true,
            overlay_shown: true,
            menu: None,
            inspector: false,
            looping: false,
            subtitle: None,
            volume,
            muted,
        }
    }

    pub fn source(&self) -> &Source {
        self.player.source()
    }

    /// Whether the file shows a moving picture, rather than only sound.
    pub fn is_video(&self) -> bool {
        self.info.as_ref().is_some_and(|info| info.has_video())
    }

    /// The name to show for the file: its title tag for sound, else its
    /// file name.
    pub fn title(&self) -> String {
        let name = self.source().name();
        match &self.info {
            Some(info) if !info.has_video() => info.title.clone().unwrap_or(name),
            _ => name,
        }
    }

    /// Whether the floating overlay should show now: always for sound and
    /// while paused, else while the pointer moved lately inside the window,
    /// a menu is open or the timeline is dragged.
    pub fn wants_overlay(&self, delay: Duration) -> bool {
        if !self.is_video() || self.status.paused || self.error.is_some() {
            return true;
        }
        if self.menu.is_some() || self.dragging.is_some() || self.inspector {
            return true;
        }
        self.pointer_inside && self.activity.idle() < delay
    }

    /// Shows or hides the overlay as `wants_overlay` says, returning
    /// whether that changed.
    pub fn refresh_overlay(&mut self, delay: Duration) -> bool {
        let shown = self.wants_overlay(delay);
        self.activity.shown.store(shown, Ordering::Relaxed);
        let changed = shown != self.overlay_shown;
        self.overlay_shown = shown;
        changed
    }

    /// Takes the player's latest status and subtitle.
    pub fn refresh(&mut self) {
        self.status = self.player.status();
        self.subtitle = self.player.subtitle();
    }

    /// The position the timeline shows: the dragged one while dragging.
    pub fn shown_position(&self) -> f64 {
        self.dragging.unwrap_or(self.status.position.as_secs_f64())
    }
}

/// What the window's controls send, carried in the app's own message with
/// the window's id.
#[derive(Debug, Clone)]
pub enum Control {
    TogglePlay,
    /// The timeline moved to this many seconds.
    Scrub(f64),
    ScrubEnded,
    SkipBy(f64),
    Volume(f32),
    VolumeReleased,
    ToggleMute,
    Speed(f64),
    Audio(Option<usize>),
    Subtitles(Option<usize>),
    Menu(Menu, bool),
    ToggleFullScreen,
    ToggleInspector,
    ToggleLoop,
    /// Floats the bars over pictures or docks them, for every window.
    ToggleOverlay,
    Screenshot,
    Open,
    Settings,
    Close,
    /// The pointer moved over the picture while the overlay was hidden.
    Activity,
    /// The wheel turned over the picture, by lines.
    Wheel(f32),
}

/// Everything the window's view needs besides its own state.
pub struct Context {
    pub fullscreen: bool,
    pub width: f32,
    /// The window's scale factor.
    pub scale: f32,
    /// The window manager draws no title bar, so the toolbar ends with a
    /// close button.
    pub close_button: bool,
}

/// The window: the picture with bars over it or around it, or for sound,
/// the cover with bars docked above and below.
pub fn view<'a>(window: &'a PlayerWindow, context: &Context) -> Element<'a, Control> {
    if let Some(error) = &window.error {
        let body = container(component::empty_state(
            Icon::Error,
            window.source().name(),
            error.as_str(),
        ))
        .style(style::surface);
        return column![docked(toolbar(window, context)), body].into();
    }
    if window.info.is_some() && !window.is_video() {
        return audio_view(window, context);
    }
    let picture = mouse_area(crate::video::view(
        &window.player,
        window.surface,
        window.activity.clone(),
        !window.overlay_shown,
        Control::Activity,
    ))
    .on_double_click(Control::ToggleFullScreen)
    .on_scroll(|delta| {
        Control::Wheel(match delta {
            iced::mouse::ScrollDelta::Lines { y, .. } => y,
            iced::mouse::ScrollDelta::Pixels { y, .. } => y / 40.0,
        })
    });
    let picture = container(picture)
        .width(Fill)
        .height(Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Color::BLACK.into()),
            ..container::Style::default()
        });
    let with_subtitles: Element<'a, Control> = match &window.subtitle {
        Some(cue) => stack![picture, subtitle_view(cue, window, context)].into(),
        None => picture.into(),
    };
    let body: Element<'a, Control> = if window.inspector {
        let sheet = component::between_bars(
            inspector(window),
            component::floating_room(true),
            component::floating_room(true),
        );
        iced::widget::row![with_subtitles, sheet].into()
    } else {
        with_subtitles
    };
    // In full screen the toolbar stays away; the controls are enough.
    let top: Element<'a, Control> = if context.fullscreen && component::floating_bars_enabled() {
        space().into()
    } else {
        component::toolbar(toolbar(window, context))
    };
    component::window_bars(
        top,
        Some(component::toolbar(transport(window, context))),
        body,
        window.overlay_shown,
    )
}

/// The toolbar: open, the file's name, and the window's tools.
fn toolbar<'a>(window: &'a PlayerWindow, context: &Context) -> Element<'a, Control> {
    let name = text(window.title())
        .size(Type::TitleSmall.size())
        .font(Type::TitleSmall.font(false))
        .wrapping(text::Wrapping::None)
        .width(Fill);
    let mut tools = vec![
        component::tool(
            Icon::FolderOpen,
            crate::fl!("action-open"),
            Some(Control::Open),
        ),
        container(name).padding([0, 8]).width(Fill).into(),
    ];
    if window.is_video() {
        tools.push(component::tool(
            Icon::Photo,
            crate::fl!("action-screenshot"),
            Some(Control::Screenshot),
        ));
    }
    tools.push(component::toggle_tool(
        Icon::Repeat,
        crate::fl!("action-loop"),
        window.looping,
        Control::ToggleLoop,
    ));
    tools.push(component::toggle_tool(
        Icon::Info,
        crate::fl!("action-info"),
        window.inspector,
        Control::ToggleInspector,
    ));
    if window.is_video() {
        tools.push(component::floating_bars_toggle(Control::ToggleOverlay));
    }
    tools.push(component::tool(
        Icon::Settings,
        crate::fl!("action-settings"),
        Some(Control::Settings),
    ));
    if context.close_button {
        tools.push(component::tool(
            Icon::Close,
            crate::fl!("common-close"),
            Some(Control::Close),
        ));
    }
    row(tools).spacing(4).align_y(Center).width(Fill).into()
}

/// The playback controls: play, skip, the timeline, volume, speed, tracks
/// and full screen.
fn transport<'a>(window: &'a PlayerWindow, context: &Context) -> Element<'a, Control> {
    let status = &window.status;
    let playing = !status.paused;
    let play_glyph = if status.ended {
        Icon::Replay
    } else if playing {
        Icon::Pause
    } else {
        Icon::PlayArrow
    };
    let play_label = if playing {
        crate::fl!("action-pause")
    } else {
        crate::fl!("action-play")
    };
    let play = component::tip(
        button::icon_button(play_glyph)
            .kind(button::Kind::Filled)
            .always_filled()
            .on_press(Control::TogglePlay),
        play_label,
    );
    let seekable = window.info.as_ref().is_some_and(|info| info.seekable);
    let back = component::tool(
        Icon::Replay10,
        crate::fl!("action-back", seconds = (SKIP as i64)),
        seekable.then_some(Control::SkipBy(-SKIP)),
    );
    let forward = component::tool(
        Icon::Forward10,
        crate::fl!("action-forward", seconds = (SKIP as i64)),
        seekable.then_some(Control::SkipBy(SKIP)),
    );
    let duration = status.duration.map(|duration| duration.as_secs_f64());
    let position = window.shown_position();
    let long = duration.unwrap_or(position) >= 3600.0;
    let elapsed = format::time(position, long);
    let timeline: Element<'a, Control> = match duration.filter(|_| seekable) {
        Some(duration) if duration > 0.0 => Timeline::new(
            &window.player,
            duration,
            context.scale,
            Control::Scrub,
            Control::ScrubEnded,
        )
        .height(style::SLIDER_HEIGHT)
        .style(|theme: &Theme, status| style::slider(timeline_backdrop(theme))(theme, status))
        .into(),
        // Live streams: no timeline, only the time played.
        _ => space().width(Fill).into(),
    };
    let remaining = duration.map_or_else(String::new, |duration| {
        format!("−{}", format::time((duration - position).max(0.0), long))
    });
    let time = |label: String| {
        text(label)
            .size(Type::LabelLarge.size())
            .font(Type::LabelLarge.font(false))
            .wrapping(text::Wrapping::None)
    };
    let volume_glyph = if window.muted || window.volume == 0.0 {
        Icon::VolumeOff
    } else if window.volume < 0.5 {
        Icon::VolumeDown
    } else {
        Icon::VolumeUp
    };
    let mute = component::toggle_tool(
        volume_glyph,
        if window.muted {
            crate::fl!("action-unmute")
        } else {
            crate::fl!("action-mute")
        },
        window.muted,
        Control::ToggleMute,
    );
    let volume = slider(0.0..=1.0, window.volume, Control::Volume)
        .step(0.01_f32)
        .on_release(Control::VolumeReleased)
        .width(96)
        .height(style::SLIDER_HEIGHT)
        .style(|theme: &Theme, status| style::slider(timeline_backdrop(theme))(theme, status));
    let mut controls: Vec<Element<'a, Control>> = vec![
        play,
        back,
        forward,
        container(time(elapsed)).padding([0, 4]).into(),
        timeline,
        container(time(remaining)).padding([0, 4]).into(),
        mute,
    ];
    // Narrow windows keep the mute button and lose the slider.
    if context.width >= 640.0 {
        controls.push(volume.into());
    }
    controls.push(speed_menu(window));
    if let Some(menu) = tracks_menu(window) {
        controls.push(menu);
    }
    if window.is_video() {
        controls.push(component::tool(
            if context.fullscreen {
                Icon::FullscreenExit
            } else {
                Icon::Fullscreen
            },
            if context.fullscreen {
                crate::fl!("action-exit-full-screen")
            } else {
                crate::fl!("action-full-screen")
            },
            Some(Control::ToggleFullScreen),
        ));
    }
    row(controls).spacing(4).align_y(Center).width(Fill).into()
}

/// The color behind the sliders' handles: the bars' own.
fn timeline_backdrop(theme: &Theme) -> iced::Color {
    Backdrop::ContainerLow.color(&Scheme::of(theme))
}

fn speed_menu(window: &PlayerWindow) -> Element<'_, Control> {
    let current = window.status.speed;
    let entries = SPEEDS
        .iter()
        .map(|&speed| {
            Entry::item(format::speed(speed), Control::Speed(speed))
                .checked((speed - current).abs() < 1e-6)
        })
        .collect();
    dropdown::icon_menu(Icon::Speed, entries)
        .tip(crate::fl!("action-speed"))
        .selected((current - 1.0).abs() > 1e-6)
        .open(window.menu == Some(Menu::Speed), |open| {
            Control::Menu(Menu::Speed, open)
        })
        .into()
}

/// The audio and subtitle tracks to choose from, when there is a choice.
fn tracks_menu(window: &PlayerWindow) -> Option<Element<'_, Control>> {
    let info = window.info.as_ref()?;
    let subtitles: Vec<_> = info.subtitles.iter().filter(|track| track.text).collect();
    if info.audio.len() < 2 && subtitles.is_empty() {
        return None;
    }
    let mut entries = Vec::new();
    if info.audio.len() > 1 {
        entries.push(Entry::heading(crate::fl!("tracks-audio")));
        for (number, track) in info.audio.iter().enumerate() {
            entries.push(
                Entry::item(
                    format::track_name(&track.track, number + 1, Some(&track.layout)),
                    Control::Audio(Some(track.track.index)),
                )
                .checked(window.status.audio == Some(track.track.index)),
            );
        }
    }
    if !subtitles.is_empty() {
        if !entries.is_empty() {
            entries.push(Entry::divider());
        }
        entries.push(Entry::heading(crate::fl!("tracks-subtitles")));
        entries.push(
            Entry::item(crate::fl!("tracks-off"), Control::Subtitles(None))
                .checked(window.status.subtitles.is_none()),
        );
        for (number, track) in subtitles.iter().enumerate() {
            entries.push(
                Entry::item(
                    format::track_name(&track.track, number + 1, None),
                    Control::Subtitles(Some(track.track.index)),
                )
                .checked(window.status.subtitles == Some(track.track.index)),
            );
        }
    }
    Some(
        dropdown::icon_menu(Icon::Subtitles, entries)
            .tip(crate::fl!("action-tracks"))
            .selected(window.status.subtitles.is_some())
            .open(window.menu == Some(Menu::Tracks), |open| {
                Control::Menu(Menu::Tracks, open)
            })
            .into(),
    )
}

/// The cue showing, along the bottom of the picture, above the controls
/// while they show.
fn subtitle_view<'a>(
    cue: &'a Cue,
    window: &PlayerWindow,
    context: &Context,
) -> Element<'a, Control> {
    let size = (context.width * 0.028).clamp(16.0, 44.0);
    let lifted = if window.overlay_shown {
        component::floating_room(true) + 8.0
    } else {
        32.0
    };
    container(
        container(
            text(cue.text.as_str())
                .size(size)
                .line_height(text::LineHeight::Relative(1.25))
                .align_x(Center)
                .color(iced::Color::WHITE),
        )
        .padding([4, 12])
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.6).into()),
            border: iced::border::rounded(6),
            ..container::Style::default()
        }),
    )
    .align_bottom(Fill)
    .center_x(Fill)
    .padding(iced::Padding {
        bottom: lifted,
        left: 24.0,
        right: 24.0,
        top: 24.0,
    })
    .into()
}

/// A sound file: its cover art, or a note, with its title, artist and
/// album, and the bars docked above and below.
fn audio_view<'a>(window: &'a PlayerWindow, context: &Context) -> Element<'a, Control> {
    let info = window.info.as_deref();
    let art: Element<'a, Control> = match &window.cover {
        Some(cover) => container(image(cover.clone()).content_fit(iced::ContentFit::Contain))
            .max_width(480)
            .max_height(480)
            .into(),
        None => container(icon::filled(Icon::MusicNote, 96).style(style::primary_text))
            .width(192)
            .height(192)
            .center(192)
            .style(|theme: &Theme| {
                let scheme = Scheme::of(theme);
                container::Style {
                    background: Some(scheme.secondary_container.into()),
                    border: iced::border::rounded(scramble_ui::shape::EXTRA_LARGE),
                    ..container::Style::default()
                }
            })
            .into(),
    };
    let mut lines = column![scramble_ui::styled(window.title(), Type::HeadlineSmall).center()]
        .spacing(4)
        .align_x(Center);
    if let Some(artist) = info.and_then(|info| info.artist.as_deref()) {
        lines = lines
            .push(scramble_ui::styled(artist, Type::TitleMedium).style(style::on_surface_variant));
    }
    if let Some(album) = info.and_then(|info| info.album.as_deref()) {
        lines = lines
            .push(scramble_ui::styled(album, Type::BodyMedium).style(style::on_surface_variant));
    }
    let body = container(column![art, lines].spacing(24).align_x(Center).padding(24))
        .center(Fill)
        .style(style::surface);
    let body: Element<'a, Control> = if window.inspector {
        iced::widget::row![body, inspector(window)].into()
    } else {
        body.into()
    };
    column![
        docked(toolbar(window, context)),
        body,
        docked(transport(window, context))
    ]
    .into()
}

/// A bar docked along an edge, whether or not bars float over pictures.
fn docked<'a>(content: Element<'a, Control>) -> Element<'a, Control> {
    container(content)
        .padding([0, 8])
        .height(component::TOOLBAR_HEIGHT)
        .width(Fill)
        .align_y(Center)
        .style(style::chrome)
        .into()
}

/// The inspector: what the file holds and how it plays.
fn inspector(window: &PlayerWindow) -> Element<'_, Control> {
    let mut content = column![].spacing(6).width(Fill);
    let field = |label: String, value: String| -> Element<'_, Control> {
        column![
            scramble_ui::styled(label, Type::LabelMedium).style(style::on_surface_variant),
            scramble_ui::aligned(scramble_ui::styled(value, Type::BodyMedium)),
        ]
        .spacing(2)
        .into()
    };
    content = content.push(component::section(crate::fl!("info-file")));
    content = content.push(field(crate::fl!("info-name"), window.source().name()));
    if let Source::File(path) = window.source() {
        if let Some(folder) = path.parent() {
            content = content.push(field(
                crate::fl!("info-folder"),
                plaa_store::paths::abbreviate_home(folder)
                    .display()
                    .to_string(),
            ));
        }
        if let Ok(metadata) = std::fs::metadata(path) {
            content = content.push(field(crate::fl!("info-size"), format::size(metadata.len())));
        }
    }
    if let Some(info) = &window.info {
        content = content.push(field(crate::fl!("info-format"), info.format.clone()));
        if let Some(duration) = info.duration {
            content = content.push(field(
                crate::fl!("info-duration"),
                format::time(duration.as_secs_f64(), true),
            ));
        }
        if let Some(rate) = info.bit_rate {
            content = content.push(field(crate::fl!("info-bit-rate"), format::bit_rate(rate)));
        }
        for track in info.video.iter().filter(|track| !track.cover) {
            content = content.push(component::section(crate::fl!("info-video")));
            content = content.push(field(crate::fl!("info-codec"), codec_label(&track.track)));
            content = content.push(field(
                crate::fl!("info-dimensions"),
                format!("{} × {}", track.width, track.height),
            ));
            if let Some(rate) = track.frame_rate {
                content = content.push(field(
                    crate::fl!("info-frame-rate"),
                    crate::fl!(
                        "info-frames-per-second",
                        rate = format!("{rate:.3}")
                            .trim_end_matches('0')
                            .trim_end_matches('.')
                            .to_owned()
                    ),
                ));
            }
            content = content.push(field(
                crate::fl!("info-pixel-format"),
                track.pixel_format.clone(),
            ));
            let decoding = match window.decoding {
                Some(Some(api)) => crate::fl!("info-decoding-hardware", api = api),
                Some(None) => crate::fl!("info-decoding-software"),
                None => String::new(),
            };
            if !decoding.is_empty() {
                content = content.push(field(crate::fl!("info-decoding"), decoding));
            }
        }
        for track in &info.audio {
            content = content.push(component::section(crate::fl!("info-audio")));
            content = content.push(field(crate::fl!("info-codec"), codec_label(&track.track)));
            content = content.push(field(
                crate::fl!("info-sample-rate"),
                crate::fl!("info-hertz", rate = track.sample_rate),
            ));
            content = content.push(field(crate::fl!("info-channels"), track.layout.clone()));
            if let Some(language) = &track.track.language {
                content = content.push(field(crate::fl!("info-language"), language.clone()));
            }
        }
        if let Some(reason) = &window.no_sound {
            content = content.push(field(crate::fl!("info-no-sound"), reason.clone()));
        }
        let shown_tags: Vec<_> = info
            .tags
            .iter()
            .filter(|(key, _)| {
                !matches!(
                    key.to_ascii_lowercase().as_str(),
                    "major_brand" | "minor_version" | "compatible_brands" | "encoder"
                )
            })
            .collect();
        if !shown_tags.is_empty() {
            content = content.push(component::section(crate::fl!("info-tags")));
            for (key, value) in shown_tags {
                content = content.push(field(key.clone(), value.clone()));
            }
        }
    }
    component::side_sheet_sized(
        crate::fl!("info-title"),
        Control::ToggleInspector,
        content,
        Length::Fixed(component::SIDE_SHEET_WIDTH),
    )
}

fn codec_label(track: &plaa_media::info::Track) -> String {
    if track.codec_name.is_empty() {
        track.codec.clone()
    } else {
        track.codec_name.clone()
    }
}
