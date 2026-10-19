//! The iced application: a window per file, a start window when there is
//! none, and Settings over whichever window asked.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use iced::keyboard::{self, Key, Modifiers};
use iced::widget::{container, image, space, stack};
use iced::window::{self, settings::PlatformSpecific};
use iced::{Center, Color, Element, Event, Fill, Size, Subscription, Task, Theme, event};
use plaa_media::{Event as PlayerEvent, Options, Player, Source};
use plaa_store::positions::{self, Positions};
use plaa_store::settings::{self, Appearance, Settings};
use scramble_ui::component::{self, Backdrop};
use scramble_ui::{Icon, Type, button, column, icon, omarchy, row, style};

use crate::External;
use crate::settings_view::{self, SettingsMessage, SettingsTab};
use crate::shortcuts::{self, Action};
use crate::window::{self as player_window, Control, Menu, PlayerWindow, SPEEDS};

#[cfg(target_os = "linux")]
pub const APP_ID: &str = if plaa_store::paths::PRODUCTION {
    "io.github.scrambletools.plaa"
} else {
    "io.github.scrambletools.plaa.Devel"
};

/// How long a notice shows before it goes away by itself.
const NOTICE_TIME: Duration = Duration::from_secs(6);
/// How often windows take the players' status while something plays.
const TICK: Duration = Duration::from_millis(100);
/// How often positions are saved while playing, in case plaa stops
/// without closing its windows.
const POSITIONS_SAVE: Duration = Duration::from_secs(60);

pub struct Plaa {
    pub(crate) windows: BTreeMap<window::Id, Window>,
    pub(crate) settings: Settings,
    settings_path: Option<PathBuf>,
    pub(crate) settings_error: Option<String>,
    positions: Positions,
    positions_path: Option<PathBuf>,
    positions_saved: Instant,
    omarchy_dir: Option<PathBuf>,
    pub(crate) omarchy: Option<omarchy::Palette>,
    /// The desktop's, Windows' or macOS's accent color, when it has one.
    pub(crate) system_accent: Option<Color>,
    system_mode: iced::theme::Mode,
    theme: Theme,
    /// Whether the system allows animations (its reduced motion setting).
    pub(crate) system_animations: bool,
    pub(crate) settings_tab: SettingsTab,
    /// Language lists as typed in Settings, applied as they change.
    pub(crate) language_drafts: [String; 2],
    /// The sound output devices, as they were when Settings last opened.
    pub(crate) audio_devices: Vec<plaa_media::OutputDevice>,
    next_surface: u64,
}

pub(crate) struct Window {
    content: Content,
    pub(crate) size: Size,
    scale: f32,
    fullscreen: bool,
    pub(crate) settings_open: bool,
    notice: Option<(String, Instant)>,
    /// The address being typed in the Open Address dialog, while it shows.
    address: Option<String>,
}

enum Content {
    Start,
    Player(Box<PlayerWindow>),
}

#[derive(Debug, Clone)]
pub enum Message {
    External(External),
    WindowOpened,
    WindowClosed(window::Id),
    CloseRequested(window::Id),
    Resized(window::Id, Size),
    Rescaled(window::Id, f32),
    Key(window::Id, Key, Modifiers),
    Pointer(window::Id, bool),
    FileDropped(window::Id, PathBuf),
    Tick,
    Control(window::Id, Control),
    Perform(window::Id, Action),
    /// The open dialog closed, with the files chosen in it.
    Chosen(window::Id, Result<Vec<PathBuf>, String>),
    AddressChanged(window::Id, String),
    AddressSubmitted(window::Id),
    AddressCancelled(window::Id),
    DismissNotice(window::Id),
    ScreenshotSaved(window::Id, Result<PathBuf, String>),
    SystemTheme(iced::theme::Mode),
    SystemAccent(Option<(u8, u8, u8)>),
    AnimationsEnabled(Option<bool>),
    Settings(SettingsMessage),
    /// The portal had no answer.
    #[cfg(target_os = "linux")]
    Nothing,
}

fn window_icon() -> Option<window::Icon> {
    let png =
        include_bytes!("../../../data/icons/hicolor/64x64/apps/io.github.scrambletools.plaa.png");
    let decoder = png::Decoder::new(std::io::Cursor::new(&png[..]));
    let mut reader = decoder.read_info().ok()?;
    let mut pixels = vec![0; reader.output_buffer_size()?];
    let frame = reader.next_frame(&mut pixels).ok()?;
    if frame.color_type != png::ColorType::Rgba || frame.bit_depth != png::BitDepth::Eight {
        return None;
    }
    pixels.truncate(frame.buffer_size());
    window::icon::from_rgba(pixels, frame.width, frame.height).ok()
}

impl Plaa {
    pub fn boot(sources: Vec<Source>, omarchy_dir: Option<PathBuf>) -> (Self, Task<Message>) {
        let settings_path = settings::default_path();
        let (settings, settings_error) =
            match settings_path.as_deref().map(Settings::load_or_create) {
                Some(Ok(settings)) => (settings, None),
                Some(Err(error)) => (Settings::default(), Some(error.to_string())),
                None => (Settings::default(), None),
            };
        let key_problems = shortcuts::configure(&settings.keys);
        for problem in &key_problems {
            eprintln!("plaa: {problem}");
        }
        let settings_error =
            settings_error.or_else(|| (!key_problems.is_empty()).then(|| key_problems.join("\n")));
        crate::i18n::set_language(settings.chosen_language());
        let positions_path = positions::default_path();
        let positions = positions_path
            .as_deref()
            .map(Positions::load)
            .unwrap_or_default();
        let language_drafts = [
            settings.audio_languages.join(", "),
            settings.subtitle_languages.join(", "),
        ];
        let mut plaa = Self {
            windows: BTreeMap::new(),
            settings,
            settings_path,
            settings_error,
            positions,
            positions_path,
            positions_saved: Instant::now(),
            omarchy_dir,
            omarchy: None,
            system_accent: None,
            system_mode: iced::theme::Mode::None,
            theme: Theme::Dark,
            system_animations: true,
            settings_tab: SettingsTab::default(),
            language_drafts,
            audio_devices: Vec::new(),
            next_surface: 1,
        };
        plaa.apply_looks();
        plaa.reload_omarchy();
        let task = plaa.open_sources(sources, None);
        let system = iced::system::theme().map(Message::SystemTheme);
        #[cfg(target_os = "linux")]
        let system = Task::batch([
            system,
            Task::perform(scramble_ui::desktop::color_scheme(), |mode| {
                mode.map_or(Message::Nothing, Message::SystemTheme)
            }),
        ]);
        let motion = Task::perform(
            scramble_ui::desktop::animations_enabled(),
            Message::AnimationsEnabled,
        );
        let accent = Task::perform(scramble_ui::desktop::accent_color(), Message::SystemAccent);
        (plaa, Task::batch([task, system, motion, accent]))
    }

    /// Hands the settings that change how components look to scramble-ui.
    pub(crate) fn apply_looks(&self) {
        component::set_floating_bars(self.settings.overlay);
        component::set_floating_transparency(self.settings.overlay_transparency);
        scramble_ui::shape::set_surface(self.settings.corner_radius);
        scramble_ui::motion::set_reduced(!self.settings.animations || !self.system_animations);
    }

    fn reload_omarchy(&mut self) {
        self.omarchy = self.omarchy_dir.as_deref().and_then(omarchy::load);
        self.refresh_theme();
    }

    /// Rebuilds the M3 scheme after a setting, the system mode or the
    /// Omarchy theme changed.
    pub(crate) fn refresh_theme(&mut self) {
        let (seed, dark) = scramble_ui::appearance::choose(scramble_ui::appearance::Inputs {
            appearance: match self.settings.appearance {
                Appearance::System => scramble_ui::appearance::Appearance::System,
                Appearance::Light => scramble_ui::appearance::Appearance::Light,
                Appearance::Dark => scramble_ui::appearance::Appearance::Dark,
            },
            system_accent: self.settings.system_accent,
            omarchy: self.omarchy.as_ref(),
            system_accent_color: self.system_accent,
            system_dark: self.system_mode == iced::theme::Mode::Dark,
            chosen_accent: settings_view::chosen_accent(&self.settings),
        });
        let name = match (&self.omarchy, self.settings.system_accent) {
            (Some(palette), true) => format!("plaa ({})", palette.name),
            _ => "plaa".to_owned(),
        };
        self.theme = scramble_ui::scheme::theme(name, seed, dark);
    }

    pub(crate) fn save_settings(&mut self) {
        if let Some(path) = &self.settings_path
            && let Err(error) = self.settings.save_to(path)
        {
            self.settings_error = Some(error.to_string());
        }
    }

    fn save_positions(&mut self) {
        for window in self.windows.values() {
            if let Content::Player(player) = &window.content {
                remember_position(&mut self.positions, player);
            }
        }
        if let Some(path) = &self.positions_path {
            let _ = self.positions.save(path);
        }
        self.positions_saved = Instant::now();
    }

    /// Opens a window per source, going to the window already playing one
    /// where there is one; `into` is a start window the first one may use.
    fn open_sources(&mut self, sources: Vec<Source>, into: Option<window::Id>) -> Task<Message> {
        if sources.is_empty() {
            if into.is_some() {
                return Task::none();
            }
            return self.open_window(None);
        }
        let mut tasks = Vec::new();
        let mut into = into.filter(|id| {
            self.windows
                .get(id)
                .is_some_and(|window| matches!(window.content, Content::Start))
        });
        for source in sources {
            let source = match source {
                Source::File(path) => Source::File(plaa_store::paths::canonical(&path)),
                other => other,
            };
            if let Some(id) = self.window_playing(&source) {
                tasks.push(window::gain_focus(id));
                continue;
            }
            match into.take() {
                Some(id) => self.play_in(id, source),
                None => tasks.push(self.open_window(Some(source))),
            }
        }
        Task::batch(tasks)
    }

    fn window_playing(&self, source: &Source) -> Option<window::Id> {
        self.windows
            .iter()
            .find_map(|(id, window)| match &window.content {
                Content::Player(player) if player.source() == source => Some(*id),
                _ => None,
            })
    }

    fn open_window(&mut self, source: Option<Source>) -> Task<Message> {
        let size = match source {
            None => Size::new(640.0, 440.0),
            Some(_) => Size::new(960.0, 600.0),
        };
        let (id, opened) = window::open(window::Settings {
            size,
            min_size: Some(Size::new(360.0, 240.0)),
            platform_specific: platform_settings(),
            exit_on_close_request: false,
            icon: window_icon(),
            ..window::Settings::default()
        });
        self.windows.insert(
            id,
            Window {
                content: Content::Start,
                size,
                scale: 1.0,
                fullscreen: false,
                settings_open: false,
                notice: None,
                address: None,
            },
        );
        if let Some(source) = source {
            self.play_in(id, source);
        }
        opened.then(move |_| {
            Task::batch([
                Task::done(Message::WindowOpened),
                window::scale_factor(id).map(move |scale| Message::Rescaled(id, scale)),
            ])
        })
    }

    /// Plays `source` in window `id`, in place of what it showed.
    fn play_in(&mut self, id: window::Id, source: Source) {
        let start = match &source {
            Source::File(path) if self.settings.resume => {
                self.positions.get(&path.to_string_lossy())
            }
            _ => None,
        };
        let options = Options {
            hardware: self.settings.hardware_decoding,
            volume: self.settings.volume,
            muted: self.settings.muted,
            start,
            paused: false,
            audio_languages: self.settings.audio_languages.clone(),
            subtitle_languages: self.settings.subtitle_languages.clone(),
            audio_device: self.settings.audio_device.clone(),
        };
        let player = Player::open(source, options, move |event| {
            crate::post(External::Player(id, event));
        });
        let surface = self.next_surface;
        self.next_surface += 1;
        if let Some(window) = self.windows.get_mut(&id) {
            if let Content::Player(previous) = &window.content {
                remember_position(&mut self.positions, previous);
            }
            window.content = Content::Player(Box::new(PlayerWindow::new(
                player,
                surface,
                self.settings.volume,
                self.settings.muted,
            )));
        }
    }

    fn player_mut(&mut self, id: window::Id) -> Option<&mut PlayerWindow> {
        match &mut self.windows.get_mut(&id)?.content {
            Content::Player(player) => Some(player),
            Content::Start => None,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::External(External::OpenSources(sources)) => self.open_sources(sources, None),
            Message::External(External::OmarchyThemeChanged) => {
                self.reload_omarchy();
                Task::none()
            }
            Message::External(External::Player(id, event)) => self.player_event(id, event),
            Message::WindowOpened => Task::none(),
            Message::WindowClosed(id) => {
                if let Some(Window {
                    content: Content::Player(player),
                    ..
                }) = self.windows.remove(&id)
                {
                    remember_position(&mut self.positions, &player);
                    if let Some(path) = &self.positions_path {
                        let _ = self.positions.save(path);
                    }
                }
                if self.windows.is_empty() {
                    return iced::exit();
                }
                Task::none()
            }
            Message::CloseRequested(id) => window::close(id),
            Message::Resized(id, size) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.size = size;
                }
                Task::none()
            }
            Message::Rescaled(id, scale) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.scale = scale;
                }
                Task::none()
            }
            Message::Key(id, key, modifiers) => self.key(id, key, modifiers),
            Message::Pointer(id, inside) => {
                let delay = self.settings.overlay_delay();
                if let Some(player) = self.player_mut(id) {
                    player.pointer_inside = inside;
                    if inside {
                        player.activity.touch();
                    }
                    player.refresh_overlay(delay);
                }
                Task::none()
            }
            Message::FileDropped(id, path) => {
                let source = Source::File(plaa_store::paths::canonical(&path));
                if self.windows.contains_key(&id) {
                    self.play_in(id, source);
                }
                Task::none()
            }
            Message::Tick => {
                self.tick();
                Task::none()
            }
            Message::Control(id, control) => self.control(id, control),
            Message::Perform(id, action) => self.perform(id, action),
            Message::Chosen(id, result) => match result {
                Ok(paths) => {
                    let sources = paths.into_iter().map(Source::File).collect();
                    self.open_sources(sources, Some(id))
                }
                Err(error) => {
                    self.notify(id, crate::fl!("notice-dialog-failed", error = error));
                    Task::none()
                }
            },
            Message::AddressChanged(id, text) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.address = Some(text);
                }
                Task::none()
            }
            Message::AddressSubmitted(id) => {
                let Some(text) = self
                    .windows
                    .get_mut(&id)
                    .and_then(|window| window.address.take())
                else {
                    return Task::none();
                };
                let text = text.trim();
                if text.is_empty() {
                    return Task::none();
                }
                let source = Source::parse(text);
                let start = self
                    .windows
                    .get(&id)
                    .is_some_and(|window| matches!(window.content, Content::Start));
                if start {
                    self.play_in(id, source);
                    Task::none()
                } else {
                    self.open_sources(vec![source], None)
                }
            }
            Message::AddressCancelled(id) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.address = None;
                }
                Task::none()
            }
            Message::DismissNotice(id) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.notice = None;
                }
                Task::none()
            }
            Message::ScreenshotSaved(id, result) => {
                let notice = match result {
                    Ok(path) => crate::fl!(
                        "notice-screenshot-saved",
                        name = path
                            .file_name()
                            .map(|name| name.to_string_lossy().into_owned())
                            .unwrap_or_default()
                    ),
                    Err(error) => crate::fl!("notice-screenshot-failed", error = error),
                };
                self.notify(id, notice);
                Task::none()
            }
            Message::SystemTheme(mode) => {
                self.system_mode = mode;
                self.refresh_theme();
                Task::none()
            }
            Message::SystemAccent(accent) => {
                self.system_accent =
                    accent.map(|(red, green, blue)| Color::from_rgb8(red, green, blue));
                self.refresh_theme();
                Task::none()
            }
            Message::AnimationsEnabled(enabled) => {
                self.system_animations = enabled.unwrap_or(true);
                self.apply_looks();
                Task::none()
            }
            Message::Settings(message) => self.settings_update(message),
            #[cfg(target_os = "linux")]
            Message::Nothing => Task::none(),
        }
    }

    fn notify(&mut self, id: window::Id, notice: String) {
        if let Some(window) = self.windows.get_mut(&id) {
            window.notice = Some((notice, Instant::now()));
        }
    }

    fn player_event(&mut self, id: window::Id, event: PlayerEvent) -> Task<Message> {
        let fullscreen = self
            .windows
            .get(&id)
            .is_some_and(|window| window.fullscreen);
        let Some(player) = self.player_mut(id) else {
            return Task::none();
        };
        match event {
            PlayerEvent::Opened { info, cover } => {
                player.cover = cover.map(|cover| {
                    image::Handle::from_rgba(cover.width, cover.height, cover.rgba.clone())
                });
                let video =
                    info.video.iter().find(|track| !track.cover).map(|track| {
                        (track.width, track.height, track.aspect, track.quarter_turns)
                    });
                player.info = Some(info);
                player.refresh();
                let size = match video {
                    Some((width, height, aspect, turns)) => {
                        fitted_window(width, height, aspect, turns, self.settings.overlay)
                    }
                    None => Size::new(440.0, 600.0),
                };
                if fullscreen {
                    Task::none()
                } else {
                    window::resize(id, size)
                }
            }
            PlayerEvent::Failed(error) => {
                player.error = Some(error);
                Task::none()
            }
            PlayerEvent::Decoding(api) => {
                player.decoding = Some(api);
                Task::none()
            }
            PlayerEvent::NoSound(reason) => {
                player.no_sound = Some(reason.clone());
                self.notify(id, crate::fl!("notice-no-sound", reason = reason));
                Task::none()
            }
            PlayerEvent::Ended => {
                if player.looping {
                    player.player.play();
                }
                player.refresh();
                Task::none()
            }
        }
    }

    /// Takes each player's status, hides overlays that rested long enough
    /// and lets old notices go.
    fn tick(&mut self) {
        let delay = self.settings.overlay_delay();
        for window in self.windows.values_mut() {
            if let Content::Player(player) = &mut window.content {
                player.refresh();
                if player.status.ended && player.looping {
                    player.player.play();
                }
                player.refresh_overlay(delay);
            }
            if window
                .notice
                .as_ref()
                .is_some_and(|(_, shown)| shown.elapsed() > NOTICE_TIME)
            {
                window.notice = None;
            }
        }
        if self.positions_saved.elapsed() > POSITIONS_SAVE {
            self.save_positions();
        }
    }

    fn key(&mut self, id: window::Id, key: Key, modifiers: Modifiers) -> Task<Message> {
        let Some(window) = self.windows.get_mut(&id) else {
            return Task::none();
        };
        if shortcuts::is_escape(&key) {
            if window.settings_open {
                window.settings_open = false;
                return Task::none();
            }
            if window.address.is_some() {
                window.address = None;
                return Task::none();
            }
            if let Content::Player(player) = &mut window.content
                && (player.menu.is_some() || player.inspector)
            {
                player.menu = None;
                player.inspector = false;
                return Task::none();
            }
            if window.fullscreen {
                return self.set_fullscreen(id, false);
            }
            return Task::none();
        }
        if window.settings_open || window.address.is_some() {
            return Task::none();
        }
        match shortcuts::action_for(&key, modifiers) {
            Some(action) => self.perform(id, action),
            None => Task::none(),
        }
    }

    fn perform(&mut self, id: window::Id, action: Action) -> Task<Message> {
        match action {
            Action::Open => {
                return Task::perform(crate::dialog::open_files(), move |result| {
                    Message::Chosen(id, result)
                });
            }
            Action::OpenAddress => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.address = Some(String::new());
                }
                return iced::widget::operation::focus(ADDRESS_FIELD);
            }
            Action::CloseWindow => return window::close(id),
            Action::Quit => {
                self.save_positions();
                return iced::exit();
            }
            Action::Settings => {
                self.audio_devices = plaa_media::output_devices();
                if let Some(window) = self.windows.get_mut(&id) {
                    window.settings_open = true;
                }
                return Task::none();
            }
            Action::FullScreen => {
                let fullscreen = self
                    .windows
                    .get(&id)
                    .is_some_and(|window| window.fullscreen);
                return self.set_fullscreen(id, !fullscreen);
            }
            _ => {}
        }
        let delay = self.settings.overlay_delay();
        let volume = self.settings.volume;
        let Some(player) = self.player_mut(id) else {
            return Task::none();
        };
        let frame = player
            .info
            .as_ref()
            .and_then(|info| info.video.iter().find(|track| !track.cover))
            .and_then(|track| track.frame_rate)
            .map_or(1.0 / 25.0, |rate| 1.0 / rate);
        match action {
            Action::Info => player.inspector = !player.inspector,
            Action::PlayPause => player.player.toggle(),
            Action::Back => player.player.seek_by(-player_window::SKIP_SHORT),
            Action::Forward => player.player.seek_by(player_window::SKIP_SHORT),
            Action::BackMore => player.player.seek_by(-player_window::SKIP),
            Action::ForwardMore => player.player.seek_by(player_window::SKIP),
            Action::PreviousFrame => {
                player.player.pause();
                player.player.seek_by(-frame);
            }
            Action::NextFrame => {
                player.player.pause();
                player.player.seek_by(frame);
            }
            Action::VolumeUp | Action::VolumeDown => {
                let change = if action == Action::VolumeUp {
                    0.05
                } else {
                    -0.05
                };
                let volume = (volume + change).clamp(0.0, 1.0);
                return self
                    .control(id, Control::Volume(volume))
                    .chain(Task::done(Message::Control(id, Control::VolumeReleased)));
            }
            Action::Mute => return self.control(id, Control::ToggleMute),
            Action::Faster | Action::Slower => {
                let current = player.status.speed;
                let next = if action == Action::Faster {
                    SPEEDS.iter().copied().find(|speed| *speed > current + 1e-6)
                } else {
                    SPEEDS
                        .iter()
                        .rev()
                        .copied()
                        .find(|speed| *speed < current - 1e-6)
                };
                if let Some(speed) = next {
                    player.player.set_speed(speed);
                }
            }
            Action::NormalSpeed => player.player.set_speed(1.0),
            Action::Loop => player.looping = !player.looping,
            Action::Screenshot => return self.control(id, Control::Screenshot),
            Action::Subtitles => {
                let Some(info) = &player.info else {
                    return Task::none();
                };
                let tracks: Vec<usize> = info
                    .subtitles
                    .iter()
                    .filter(|track| track.text)
                    .map(|track| track.track.index)
                    .collect();
                let next = match player.status.subtitles {
                    None => tracks.first().copied(),
                    Some(current) => tracks
                        .iter()
                        .position(|index| *index == current)
                        .and_then(|at| tracks.get(at + 1).copied()),
                };
                player.player.select_subtitles(next);
            }
            Action::AudioTrack => {
                let Some(info) = &player.info else {
                    return Task::none();
                };
                let tracks: Vec<usize> = info.audio.iter().map(|track| track.track.index).collect();
                if tracks.len() > 1 {
                    let at = player
                        .status
                        .audio
                        .and_then(|current| tracks.iter().position(|index| *index == current))
                        .map_or(0, |at| (at + 1) % tracks.len());
                    player.player.select_audio(Some(tracks[at]));
                }
            }
            Action::ToStart => player.player.seek(Duration::ZERO, true),
            _ => {}
        }
        player.refresh();
        player.activity.touch();
        player.refresh_overlay(delay);
        Task::none()
    }

    fn set_fullscreen(&mut self, id: window::Id, fullscreen: bool) -> Task<Message> {
        if let Some(window) = self.windows.get_mut(&id) {
            window.fullscreen = fullscreen;
        }
        window::set_mode(
            id,
            if fullscreen {
                window::Mode::Fullscreen
            } else {
                window::Mode::Windowed
            },
        )
    }

    fn control(&mut self, id: window::Id, control: Control) -> Task<Message> {
        let delay = self.settings.overlay_delay();
        match control {
            Control::Open => return self.perform(id, Action::Open),
            Control::Settings => return self.perform(id, Action::Settings),
            Control::Close => return window::close(id),
            Control::ToggleFullScreen => return self.perform(id, Action::FullScreen),
            Control::ToggleOverlay => {
                self.settings.overlay = !self.settings.overlay;
                self.apply_looks();
                self.save_settings();
                return Task::none();
            }
            Control::Volume(volume) => {
                self.settings.volume = volume;
                if volume > 0.0 {
                    self.settings.muted = false;
                }
                let muted = self.settings.muted;
                for window in self.windows.values_mut() {
                    if let Content::Player(player) = &mut window.content {
                        player.volume = volume;
                        player.muted = muted;
                        player.player.set_volume(volume);
                        player.player.set_muted(muted);
                    }
                }
                return Task::none();
            }
            Control::VolumeReleased => {
                self.save_settings();
                return Task::none();
            }
            Control::ToggleMute => {
                self.settings.muted = !self.settings.muted;
                let muted = self.settings.muted;
                for window in self.windows.values_mut() {
                    if let Content::Player(player) = &mut window.content {
                        player.muted = muted;
                        player.player.set_muted(muted);
                    }
                }
                self.save_settings();
                return Task::none();
            }
            Control::Wheel(lines) => {
                let volume = (self.settings.volume + lines * 0.05).clamp(0.0, 1.0);
                return self
                    .control(id, Control::Volume(volume))
                    .chain(Task::done(Message::Control(id, Control::VolumeReleased)));
            }
            Control::Screenshot => {
                let folder = self.settings.screenshots.clone();
                let Some(player) = self.player_mut(id) else {
                    return Task::none();
                };
                let Some(picture) = player.player.snapshot() else {
                    return Task::none();
                };
                let name = screenshot_name(&player.source().name(), player.status.position);
                return Task::perform(
                    crate::screenshot::save(picture, folder.join(name)),
                    move |result| Message::ScreenshotSaved(id, result),
                );
            }
            _ => {}
        }
        let Some(player) = self.player_mut(id) else {
            return Task::none();
        };
        match control {
            Control::TogglePlay => player.player.toggle(),
            Control::Scrub(position) => {
                player.dragging = Some(position);
                player
                    .player
                    .scrub(Duration::from_secs_f64(position.max(0.0)));
            }
            Control::ScrubEnded => {
                if player.dragging.take().is_some() {
                    player.player.end_scrub();
                }
            }
            Control::SkipBy(seconds) => player.player.seek_by(seconds),
            Control::Speed(speed) => {
                player.player.set_speed(speed);
                player.menu = None;
            }
            Control::Audio(index) => {
                player.player.select_audio(index);
                player.menu = None;
            }
            Control::Subtitles(index) => {
                player.player.select_subtitles(index);
                player.menu = None;
            }
            Control::Menu(menu, open) => {
                player.menu = if open { Some(menu) } else { None::<Menu> };
            }
            Control::ToggleInspector => player.inspector = !player.inspector,
            Control::ToggleLoop => player.looping = !player.looping,
            Control::Activity => {}
            _ => {}
        }
        player.refresh();
        player.activity.touch();
        player.refresh_overlay(delay);
        Task::none()
    }

    pub fn title(&self, id: window::Id) -> String {
        let base = match self.windows.get(&id).map(|window| &window.content) {
            Some(Content::Player(player)) => player.title(),
            _ => crate::fl!("app-name"),
        };
        if plaa_store::paths::PRODUCTION {
            base
        } else {
            format!("{base} (dev)")
        }
    }

    pub fn theme(&self, _id: window::Id) -> Option<Theme> {
        Some(self.theme.clone())
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // Status and overlays need looking at while something plays or an
        // overlay or notice waits to go.
        let ticking = self.windows.values().any(|window| {
            window.notice.is_some()
                || matches!(&window.content, Content::Player(player)
                    if !player.status.paused || player.overlay_shown != !player.is_video() || player.info.is_none())
        });
        let ticks = if ticking {
            Subscription::run(ticks).map(|_| Message::Tick)
        } else {
            Subscription::none()
        };
        Subscription::batch([
            ticks,
            iced::system::theme_changes().map(Message::SystemTheme),
            Subscription::run(scramble_ui::desktop::accent_changes).map(Message::SystemAccent),
            Subscription::run(crate::external_events).map(Message::External),
            window::close_events().map(Message::WindowClosed),
            window::close_requests().map(Message::CloseRequested),
            event::listen_with(|event, status, id| match (event, status) {
                (
                    Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }),
                    event::Status::Ignored,
                ) => Some(Message::Key(id, key, modifiers)),
                (Event::Mouse(iced::mouse::Event::CursorEntered), _) => {
                    Some(Message::Pointer(id, true))
                }
                (Event::Mouse(iced::mouse::Event::CursorLeft), _) => {
                    Some(Message::Pointer(id, false))
                }
                (Event::Window(window::Event::Resized(size)), _) => {
                    Some(Message::Resized(id, size))
                }
                (Event::Window(window::Event::Rescaled(scale)), _) => {
                    Some(Message::Rescaled(id, scale))
                }
                (Event::Window(window::Event::FileDropped(path)), _) => {
                    Some(Message::FileDropped(id, path))
                }
                _ => None,
            }),
        ])
    }

    pub fn view(&self, id: window::Id) -> Element<'_, Message> {
        let Some(window) = self.windows.get(&id) else {
            return space().into();
        };
        let close_button = !scramble_ui::desktop::draws_title_bars();
        let body: Element<'_, Message> = match &window.content {
            Content::Start => start_view(id, close_button),
            Content::Player(player) => player_window::view(
                player,
                &player_window::Context {
                    fullscreen: window.fullscreen,
                    width: window.size.width,
                    scale: window.scale,
                    close_button,
                },
            )
            .map(move |control| Message::Control(id, control)),
        };
        // Layers over the window are always in the tree, empty when not
        // shown, so opening one keeps the state underneath.
        let full = || Element::from(space().width(Fill).height(Fill));
        let notice = match &window.notice {
            Some((notice, _)) => component::snackbar(full(), notice, Message::DismissNotice(id)),
            None => full(),
        };
        let address = match &window.address {
            Some(text) => address_dialog(id, text),
            None => full(),
        };
        let settings = if window.settings_open {
            self.settings_dialog(id).map(Message::Settings)
        } else {
            full()
        };
        scramble_ui::smooth::smooth(stack![body, notice, address, settings])
    }
}

/// Remembers where `player` is, for its file to start there next time.
impl Window {
    /// What the window plays, if anything.
    pub(crate) fn player(&self) -> Option<&PlayerWindow> {
        match &self.content {
            Content::Player(player) => Some(player),
            Content::Start => None,
        }
    }
}

fn remember_position(positions: &mut Positions, player: &PlayerWindow) {
    if let Source::File(path) = player.source()
        && player.info.as_ref().is_some_and(|info| info.seekable)
    {
        positions.set(
            &path.to_string_lossy(),
            player.player.position(),
            player.status.duration,
        );
    }
}

/// A window the video's size, made smaller to fit a typical screen, with
/// room for docked bars.
fn fitted_window(width: u32, height: u32, aspect: f64, turns: u8, overlay: bool) -> Size {
    let shown_width = if aspect.is_finite() && aspect > 0.0 {
        height as f32 * aspect as f32
    } else {
        width as f32
    };
    let (mut width, mut height) = (shown_width, height as f32);
    if width <= 0.0 || height <= 0.0 {
        return Size::new(960.0, 600.0);
    }
    if turns % 2 == 1 {
        std::mem::swap(&mut width, &mut height);
    }
    let scale = (1280.0 / width).min(800.0 / height).min(1.0);
    let bars = if overlay {
        0.0
    } else {
        2.0 * component::TOOLBAR_HEIGHT
    };
    Size::new(
        (width * scale).round().max(360.0),
        (height * scale).round().max(240.0) + bars,
    )
}

/// "plaa <file> 0.01.02.png", without characters file systems refuse.
fn screenshot_name(name: &str, position: Duration) -> String {
    // By hand, not with `Path`, which on Windows reads "b:" as a drive.
    let base = name.rsplit(['/', '\\']).next().unwrap_or(name);
    let stem = match base.rsplit_once('.') {
        Some((stem, _)) if !stem.is_empty() => stem,
        _ => base,
    };
    let stem: String = stem
        .chars()
        .map(|character| {
            if "/\\:*?\"<>|".contains(character) {
                '_'
            } else {
                character
            }
        })
        .take(80)
        .collect();
    let seconds = position.as_secs();
    format!(
        "{stem} {}.{:02}.{:02}.png",
        seconds / 3600,
        seconds / 60 % 60,
        seconds % 60
    )
}

/// A tick every `TICK` from a thread of its own, as iced's executor has
/// no timers; it stops once the subscription goes.
fn ticks() -> impl iced::futures::Stream<Item = ()> {
    let (sender, receiver) = iced::futures::channel::mpsc::unbounded();
    let _ = std::thread::Builder::new()
        .name("plaa-tick".into())
        .spawn(move || {
            loop {
                std::thread::sleep(TICK);
                if sender.unbounded_send(()).is_err() {
                    return;
                }
            }
        });
    receiver
}

const ADDRESS_FIELD: &str = "address";

fn address_dialog(id: window::Id, text: &str) -> Element<'_, Message> {
    let field = component::text_field(
        crate::fl!("address-label"),
        text,
        Backdrop::ContainerHigh,
        |input| {
            input
                .id(ADDRESS_FIELD)
                .on_input(move |text| Message::AddressChanged(id, text))
                .on_submit(Message::AddressSubmitted(id))
        },
    );
    let actions = vec![
        scramble_ui::button(button::Kind::Text, crate::fl!("common-cancel"))
            .on_press(Message::AddressCancelled(id))
            .into(),
        scramble_ui::button(button::Kind::Filled, crate::fl!("address-open"))
            .on_press_maybe((!text.trim().is_empty()).then_some(Message::AddressSubmitted(id)))
            .into(),
    ];
    let body = column![
        scramble_ui::styled(crate::fl!("address-title"), Type::HeadlineSmall),
        scramble_ui::aligned(
            scramble_ui::styled(crate::fl!("address-note"), Type::BodyMedium)
                .style(style::on_surface_variant)
        ),
        field,
        container(scramble_ui::dir::row(actions).spacing(8)).align_right(Fill),
    ]
    .spacing(16);
    let card = container(body).padding(24).width(480).style(style::dialog);
    iced::widget::mouse_area(
        container(iced::widget::opaque(card))
            .center(Fill)
            .padding(24)
            .style(style::scrim),
    )
    .on_press(Message::AddressCancelled(id))
    .into()
}

fn start_view(id: window::Id, close_button: bool) -> Element<'static, Message> {
    let hints = [Action::Open, Action::OpenAddress, Action::Settings]
        .into_iter()
        .filter_map(|action| Some((action, shortcuts::label(action)?)))
        .map(|(action, label)| {
            row![
                scramble_ui::styled(action_name(action), Type::BodyMedium)
                    .style(style::on_surface_variant),
                scramble_ui::styled(label, Type::LabelLarge),
            ]
            .spacing(8)
            .into()
        });
    let content = container(
        column![
            icon::filled(Icon::Movie, 64).style(style::primary_text),
            scramble_ui::styled(crate::fl!("app-name"), Type::DisplaySmall),
            scramble_ui::styled(crate::fl!("start-hint"), Type::BodyLarge)
                .style(style::on_surface_variant),
            row![
                scramble_ui::with_icon(
                    button::Kind::Filled,
                    Icon::FolderOpen,
                    crate::fl!("start-open")
                )
                .size(button::Size::Medium)
                .on_press(Message::Perform(id, Action::Open)),
                scramble_ui::with_icon(
                    button::Kind::Tonal,
                    Icon::Link,
                    crate::fl!("start-open-address")
                )
                .size(button::Size::Medium)
                .on_press(Message::Perform(id, Action::OpenAddress)),
            ]
            .spacing(12),
            scramble_ui::dir::row(hints).spacing(24),
        ]
        .spacing(16)
        .align_x(Center),
    )
    .center(Fill)
    .style(style::surface);
    if close_button {
        stack![
            content,
            container(row![
                component::tool(
                    Icon::Settings,
                    crate::fl!("action-settings"),
                    Some(Message::Perform(id, Action::Settings))
                ),
                component::tool(
                    Icon::Close,
                    crate::fl!("common-close"),
                    Some(Message::CloseRequested(id))
                )
            ])
            .align_right(Fill)
            .padding(8)
        ]
        .into()
    } else {
        stack![
            content,
            container(component::tool(
                Icon::Settings,
                crate::fl!("action-settings"),
                Some(Message::Perform(id, Action::Settings))
            ))
            .align_right(Fill)
            .padding(8)
        ]
        .into()
    }
}

fn action_name(action: Action) -> String {
    match action {
        Action::Open => crate::fl!("action-open"),
        Action::OpenAddress => crate::fl!("action-open-address"),
        Action::Settings => crate::fl!("action-settings"),
        _ => String::new(),
    }
}

/// On Linux, the app id desktops match windows and launchers by.
#[cfg(target_os = "linux")]
fn platform_settings() -> PlatformSpecific {
    PlatformSpecific {
        application_id: APP_ID.to_owned(),
        ..PlatformSpecific::default()
    }
}

#[cfg(not(target_os = "linux"))]
fn platform_settings() -> PlatformSpecific {
    PlatformSpecific::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_fit_the_video_and_a_screen() {
        assert_eq!(
            fitted_window(640, 360, 16.0 / 9.0, 0, true),
            Size::new(640.0, 360.0)
        );
        let big = fitted_window(3840, 2160, 16.0 / 9.0, 0, true);
        assert!(big.width <= 1280.0 && big.height <= 800.0);
        let portrait = fitted_window(1920, 1080, 16.0 / 9.0, 1, true);
        assert!(portrait.height > portrait.width);
    }

    #[test]
    fn screenshot_names_are_safe() {
        assert_eq!(
            screenshot_name("b:c?.mkv", Duration::from_secs(3723)),
            "b_c_ 1.02.03.png"
        );
        assert_eq!(
            screenshot_name("https://example.com/live.m3u8", Duration::from_secs(5)),
            "live 0.00.05.png"
        );
    }
}
