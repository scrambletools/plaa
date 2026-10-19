//! The iced application: a window per file, a start window when there is
//! none, and Settings over whichever window asked.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;
use std::time::{Duration, Instant};

use iced::keyboard::{self, Key, Modifiers};
use iced::widget::{container, image, space, stack};
use iced::window::{self, settings::PlatformSpecific};
use iced::{Center, Color, Element, Event, Fill, Size, Subscription, Task, Theme, event};
use plaa_media::web::Page;
use plaa_media::{Event as PlayerEvent, Options, Player, Quality, Source, Thumbnails};
use plaa_store::positions::{self, Positions};
use plaa_store::recent::{self, Recent};
use plaa_store::settings::{self, Appearance, Settings, StreamQuality};
use scramble_ui::component::{self, Backdrop};
use scramble_ui::{Icon, Type, button, column, icon, omarchy, row, style};

use crate::External;
use crate::editing::Mode;
use crate::playlist::{self, Playlist};
use crate::settings_view::{self, SettingsMessage, SettingsTab};
use crate::shortcuts::{self, Action};
use crate::window::{
    self as player_window, Control, Menu, PlayerWindow, QualityChoice, SPEEDS, Sheet,
};

mod edits;
mod export;

/// plaa's id, reversed domain style: its desktop entry on Linux, and how
/// it names itself to the system's services everywhere.
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
/// Going back this far into an item starts it over rather than going to
/// the item before.
const RESTART_WITHIN: Duration = Duration::from_secs(3);
/// The most a window opens to for a picture, and in picture in picture.
const WINDOW_ROOM: Size = Size::new(1280.0, 800.0);
const PIP_ROOM: Size = Size::new(400.0, 400.0);
/// How far picture in picture keeps from the screen's corner.
const PIP_MARGIN: f32 = 24.0;

/// How long the files of one drop take to arrive, at most.
const DROP_GATHER: Duration = Duration::from_millis(150);

pub struct Plaa {
    pub(crate) windows: BTreeMap<window::Id, Window>,
    pub(crate) settings: Settings,
    settings_path: Option<PathBuf>,
    pub(crate) settings_error: Option<String>,
    positions: Positions,
    positions_path: Option<PathBuf>,
    positions_saved: Instant,
    /// The addresses opened lately, which the Open Address dialog offers.
    recent: Recent,
    recent_path: Option<PathBuf>,
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
    /// Whether yt-dlp was installed when Settings last opened.
    pub(crate) yt_dlp_found: bool,
    /// The encoder each format and size of export would use, once found,
    /// and those being looked for.
    encoders: HashMap<export::Encoding, Option<plaa_media::export::Encoder>>,
    probing: HashSet<export::Encoding>,
    /// What the Export dialog chose last, which it starts with next.
    export_choice: Option<export::Choice>,
    /// What is kept awake while something plays, and the hold on it when
    /// the system gave one.
    awake: Option<(Awake, Option<keepawake::KeepAwake>)>,
    /// The window media keys and the desktop's controls act on: the one
    /// last played or used.
    media_window: Option<window::Id>,
    media: Option<crate::media::Service>,
    /// The window last focused, which the menu bar's items act on.
    #[cfg(target_os = "macos")]
    focused: Option<window::Id>,
    next_surface: u64,
}

/// Holds the screen on for video, or the computer for sound. Where the
/// screen saver's service is missing, as on some tiling desktops, the
/// computer's idle hold still keeps idle daemons such as hypridle off.
fn hold_awake(kind: Awake) -> Option<keepawake::KeepAwake> {
    let hold = |display: bool| {
        keepawake::Builder::default()
            .display(display)
            .idle(true)
            .reason(match kind {
                Awake::Screen => "Playing video",
                Awake::Computer => "Playing sound",
            })
            .app_name("plaa")
            .app_reverse_domain(APP_ID)
            .create()
    };
    let held = match kind {
        Awake::Screen => hold(true).or_else(|_| hold(false)),
        Awake::Computer => hold(false),
    };
    match held {
        Ok(held) => Some(held),
        Err(error) => {
            eprintln!("plaa: keeping the screen on: {error}");
            None
        }
    }
}

/// What playback keeps awake.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Awake {
    /// The screen, as video plays.
    Screen,
    /// The computer, not the screen, as sound plays.
    Computer,
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
    /// Files dropped on the window, gathering while a drop arrives.
    dropped: Vec<Source>,
    /// In picture in picture: the size to go back to.
    pip: Option<Size>,
    /// The height of the window's screen in its pixels, once known.
    screen_lines: Option<u32>,
    /// The Export dialog's choices, while it shows.
    export: Option<export::Choice>,
    /// An export under way from the window.
    exporting: Option<export::Exporting>,
    /// Closing waits for an answer, as it would lose changes.
    closing: Option<export::Closing>,
    /// The window's Wayland surface, which drops name it by, or its
    /// AppKit view.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    surface: Option<usize>,
    /// The transparent title bar's height on macOS, which plaa's chrome
    /// fills; zero in full screen.
    #[cfg(target_os = "macos")]
    title_bar: f32,
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
    /// The height of a window's screen in its pixels, if the system says.
    Screen(window::Id, Option<u32>),
    /// A window's Wayland surface or AppKit view, once it shows.
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    Surface(window::Id, Option<usize>),
    #[cfg(target_os = "macos")]
    Focused(window::Id),
    #[cfg(target_os = "macos")]
    TitleBarDoubleClicked(window::Id),
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
    /// A recent address was chosen in the Open Address dialog.
    AddressChosen(window::Id, String),
    /// A recent address is to be forgotten.
    AddressForgotten(String),
    DismissNotice(window::Id),
    ExportFormat(window::Id, plaa_media::export::Format),
    /// The Export dialog's size: the picture's shorter side, or its own.
    ExportSize(window::Id, Option<u32>),
    /// The Export dialog's format for the sound alone.
    ExportSound(window::Id, plaa_media::export::Sound),
    /// The sound's quality: kb/s, or bits for lossless sound.
    ExportQuality(window::Id, u32),
    ExportDismissed(window::Id),
    /// Export was chosen in the dialog: on to where to save it.
    ExportChosen(window::Id),
    /// Where to export to, or none when the save dialog was cancelled.
    ExportTo(window::Id, Result<Option<PathBuf>, String>),
    ExportStop(window::Id),
    CloseConfirmed(window::Id),
    CloseDismissed(window::Id),
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
        let recent_path = recent::default_path();
        let recent = recent_path.as_deref().map(Recent::load).unwrap_or_default();
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
            recent,
            recent_path,
            omarchy_dir,
            omarchy: None,
            system_accent: None,
            system_mode: iced::theme::Mode::None,
            theme: Theme::Dark,
            system_animations: true,
            settings_tab: SettingsTab::default(),
            language_drafts,
            audio_devices: Vec::new(),
            yt_dlp_found: false,
            encoders: HashMap::new(),
            probing: HashSet::new(),
            export_choice: None,
            awake: None,
            media_window: None,
            media: crate::media::Service::start("plaa", |command| {
                crate::post(External::Media(command));
            }),
            #[cfg(target_os = "macos")]
            focused: None,
            next_surface: 1,
        };
        plaa.apply_looks();
        plaa.reload_omarchy();
        #[cfg(target_os = "macos")]
        crate::menu_macos::install();
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
        self.remember_addresses(&sources);
        if sources.is_empty() {
            if into.is_some() {
                return Task::none();
            }
            return self.open_window(None);
        }
        let sources: Vec<Source> = sources.into_iter().map(canonical).collect();
        // A file playing already: its window comes forward.
        if let [source] = sources.as_slice()
            && let Some(id) = self.window_playing(source)
        {
            return window::gain_focus(id);
        }
        let Some(list) = self.playlist_for(sources) else {
            return Task::none();
        };
        let into = into.filter(|id| {
            self.windows
                .get(id)
                .is_some_and(|window| matches!(window.content, Content::Start))
        });
        match into {
            Some(id) => {
                self.play_in(id, list, false);
                Task::none()
            }
            None => self.open_window(Some(list)),
        }
    }

    /// Plays what was dropped on a window on Wayland: its files, or the
    /// addresses of streams, as one list.
    #[cfg(target_os = "linux")]
    fn dragged(&mut self, event: smithay_clipboard::dnd::DragEvent) -> Task<Message> {
        use smithay_clipboard::dnd::{DragEvent, URI_LIST_MIME, parse_uri_list};
        let DragEvent::Dropped {
            surface,
            mime,
            data,
            ..
        } = event
        else {
            return Task::none();
        };
        let Some(id) = self
            .windows
            .iter()
            .find_map(|(id, window)| (window.surface == Some(surface)).then_some(*id))
        else {
            return Task::none();
        };
        if mime != URI_LIST_MIME {
            return Task::none();
        }
        let sources: Vec<Source> = parse_uri_list(&String::from_utf8_lossy(&data))
            .iter()
            .map(|uri| match crate::dialog::file_uri_to_path(uri) {
                Some(path) => canonical(Source::File(path)),
                None => Source::parse(uri),
            })
            .collect();
        self.remember_addresses(&sources);
        if let Some(list) = self.playlist_for(sources) {
            self.play_in(id, list, false);
        }
        window::gain_focus(id)
    }

    /// Puts the addresses among `sources` first among the recent ones.
    fn remember_addresses(&mut self, sources: &[Source]) {
        let mut changed = false;
        for source in sources.iter().rev() {
            if let Source::Url(url) = source {
                self.recent.add(url);
                changed = true;
            }
        }
        if changed {
            self.save_recent();
        }
    }

    /// Names a recent address, once what it is called is known.
    fn name_address(&mut self, source: &Source, title: Option<&str>) {
        if let (Source::Url(url), Some(title)) = (source, title)
            && title != url
            && self.recent.name(url, title)
        {
            self.save_recent();
        }
    }

    fn save_recent(&self) {
        if let Some(path) = &self.recent_path
            && let Err(error) = self.recent.save(path)
        {
            eprintln!("plaa: keeping recent addresses: {error}");
        }
    }

    /// The list `sources` make together: list files' entries in their
    /// place, or for one file alone, the files in its folder when Settings
    /// asks for them.
    fn playlist_for(&self, sources: Vec<Source>) -> Option<Playlist> {
        let items: Vec<Source> = playlist::expand(sources)
            .into_iter()
            .map(canonical)
            .collect();
        if self.settings.folder_playlists
            && let [Source::File(path)] = items.as_slice()
            && let Some((files, at)) = playlist::folder(path)
        {
            return Playlist::new(files.into_iter().map(Source::File).collect(), at);
        }
        Playlist::new(items, 0)
    }

    /// Shrinks a video's window to a corner and keeps it over others, or
    /// brings it back. Wayland lets apps do neither of the last two: there
    /// it only shrinks, and a compositor rule can pin it.
    fn toggle_pip(&mut self, id: window::Id) -> Task<Message> {
        let overlay = self.settings.overlay;
        let Some(window) = self.windows.get_mut(&id) else {
            return Task::none();
        };
        if let Some(size) = window.pip.take() {
            return Task::batch([
                window::set_level(id, window::Level::Normal),
                window::resize(id, size),
            ]);
        }
        let Some(video) = window.player().and_then(|player| {
            let track = player.picture()?;
            let turns = player.transform().after_turns(track.quarter_turns).turns;
            Some((track.width, track.height, track.aspect, turns))
        }) else {
            return Task::none();
        };
        window.pip = Some(window.size);
        let leave_fullscreen = if window.fullscreen {
            self.set_fullscreen(id, false)
        } else {
            Task::none()
        };
        let (width, height, aspect, turns) = video;
        let size = fitted_window(width, height, aspect, turns, overlay, PIP_ROOM);
        let corner = window::monitor_size(id).then(move |monitor| match monitor {
            Some(monitor) => window::move_to(
                id,
                iced::Point::new(
                    monitor.width - size.width - PIP_MARGIN,
                    monitor.height - size.height - 2.0 * PIP_MARGIN,
                ),
            ),
            None => Task::none(),
        });
        Task::batch([
            leave_fullscreen,
            window::set_level(id, window::Level::AlwaysOnTop),
            window::resize(id, size),
            corner,
        ])
    }

    /// Plays item `index` of window `id`'s list.
    fn play_item(&mut self, id: window::Id, index: usize) -> Task<Message> {
        let Some(player) = self.player_mut(id) else {
            return Task::none();
        };
        let mut list = player.playlist.clone();
        if list.select(index) {
            self.play_in(id, list, true);
        }
        Task::none()
    }

    fn next(&mut self, id: window::Id) -> Task<Message> {
        match self
            .player_mut(id)
            .and_then(|player| player.playlist.next())
        {
            Some(next) => self.play_item(id, next),
            None => Task::none(),
        }
    }

    /// The previous item, or the start of this one once it has played a
    /// few seconds.
    fn previous(&mut self, id: window::Id) -> Task<Message> {
        let Some(player) = self.player_mut(id) else {
            return Task::none();
        };
        match player.playlist.previous() {
            Some(previous) if player.player.position() < RESTART_WITHIN => {
                self.play_item(id, previous)
            }
            _ => {
                player.player.seek(Duration::ZERO, true);
                Task::none()
            }
        }
    }

    fn window_playing(&self, source: &Source) -> Option<window::Id> {
        self.windows
            .iter()
            .find_map(|(id, window)| match &window.content {
                Content::Player(player) if player.source() == source => Some(*id),
                _ => None,
            })
    }

    fn open_window(&mut self, list: Option<Playlist>) -> Task<Message> {
        let size = match list {
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
                dropped: Vec::new(),
                pip: None,
                screen_lines: None,
                export: None,
                exporting: None,
                closing: None,
                #[cfg(any(target_os = "linux", target_os = "macos"))]
                surface: None,
                #[cfg(target_os = "macos")]
                title_bar: 0.0,
            },
        );
        if let Some(list) = list {
            self.play_in(id, list, false);
        }
        opened.then(move |_| {
            Task::batch([
                Task::done(Message::WindowOpened),
                window::scale_factor(id).map(move |scale| Message::Rescaled(id, scale)),
                screen_lines(id),
                #[cfg(any(target_os = "linux", target_os = "macos"))]
                window::run(id, window_surface).map(move |surface| Message::Surface(id, surface)),
            ])
        })
    }

    /// Plays `list`'s current item in window `id`, in place of what it
    /// showed. Moving on through a list keeps the window's size and modes.
    fn play_in(&mut self, id: window::Id, list: Playlist, moving_on: bool) {
        let start = self
            .settings
            .resume
            .then(|| self.positions.get(&position_key(list.source())))
            .flatten();
        let lines = self.windows.get(&id).and_then(|window| window.screen_lines);
        self.open_player(
            id,
            list,
            Opening {
                quality: self.quality(lines),
                page: None,
                start,
                paused: false,
                moving_on,
            },
        );
    }

    /// Opens window `id`'s web page again in `quality`, where it was, as
    /// it was, playing or paused.
    fn reopen(&mut self, id: window::Id, quality: Quality) {
        let Some(player) = self.player_mut(id) else {
            return;
        };
        let Some(page) = player.page.clone() else {
            return;
        };
        let opening = Opening {
            quality,
            page: Some(page),
            start: Some(player.player.position()),
            paused: player.status.paused,
            moving_on: true,
        };
        let (list, ab_loop) = (player.playlist.clone(), player.ab_loop);
        self.open_player(id, list, opening);
        if let Some(player) = self.player_mut(id) {
            player.ab_loop = ab_loop;
            if let Some((from, Some(to))) = ab_loop {
                player.player.set_ab_loop(Some((
                    Duration::from_secs_f64(from),
                    Duration::from_secs_f64(to),
                )));
            }
        }
    }

    /// Starts a player for `list`'s current item in window `id`.
    fn open_player(&mut self, id: window::Id, list: Playlist, opening: Opening) {
        self.media_window = Some(id);
        let options = Options {
            hardware: self.settings.hardware_decoding,
            volume: self.settings.volume,
            muted: self.settings.muted,
            start: opening.start,
            paused: opening.paused,
            audio_languages: self.settings.audio_languages.clone(),
            subtitle_languages: self.settings.subtitle_languages.clone(),
            audio_device: self.settings.audio_device.clone(),
            quality: opening.quality,
            web: self.settings.web_pages,
            page: opening.page.clone(),
        };
        let player = Player::open(list.source().clone(), options, move |event| {
            crate::post(External::Player(id, event));
        });
        let surface = self.next_surface;
        self.next_surface += 1;
        if let Some(window) = self.windows.get_mut(&id) {
            let mut playing = PlayerWindow::new(
                player,
                list,
                surface,
                self.settings.volume,
                self.settings.muted,
            );
            playing.fit_window = !opening.moving_on;
            playing.quality = opening.quality;
            playing.page = opening.page;
            if let Content::Player(previous) = &window.content {
                remember_position(&mut self.positions, previous);
                if opening.moving_on {
                    playing.looping = previous.looping;
                    playing.sheet = previous.sheet;
                }
            }
            window.content = Content::Player(Box::new(playing));
        }
    }

    /// The quality to ask for: Settings' choice; the automatic one is the
    /// height of the window's screen, `lines` tall when known.
    fn quality(&self, lines: Option<u32>) -> Quality {
        match self.settings.stream_quality {
            StreamQuality::Best => Quality::Best,
            StreamQuality::AtMost(lines) => Quality::AtMost(lines),
            StreamQuality::Auto => Quality::AtMost(lines.unwrap_or(1080).max(360)),
        }
    }

    fn player_mut(&mut self, id: window::Id) -> Option<&mut PlayerWindow> {
        match &mut self.windows.get_mut(&id)?.content {
            Content::Player(player) => Some(player),
            Content::Start => None,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let task = self.handle(message);
        self.keep_awake();
        self.tell_desktop();
        task
    }

    /// The window media keys act on: the one last played or used, or any
    /// that plays.
    fn media_window(&self) -> Option<window::Id> {
        self.media_window
            .filter(|id| {
                self.windows
                    .get(id)
                    .is_some_and(|window| window.player().is_some())
            })
            .or_else(|| {
                self.windows
                    .iter()
                    .find_map(|(id, window)| window.player().map(|_| *id))
            })
    }

    /// Tells the system what the media window plays.
    fn tell_desktop(&self) {
        use crate::media::{NowPlaying, Playback};
        let Some(service) = &self.media else {
            return;
        };
        let now = self
            .media_window()
            .and_then(|id| self.windows.get(&id)?.player())
            .map(|player| {
                let info = player.info.as_deref();
                let status = &player.status;
                NowPlaying {
                    playback: if status.ended {
                        Playback::Stopped
                    } else if status.paused {
                        Playback::Paused
                    } else {
                        Playback::Playing
                    },
                    track: player.surface,
                    title: player.title(),
                    artist: info.and_then(|info| info.artist.clone()),
                    album: info.and_then(|info| info.album.clone()),
                    length: status.duration,
                    #[cfg(target_os = "linux")]
                    url: match player.source() {
                        Source::File(path) => Some(crate::mpris::file_uri(path)),
                        Source::Url(url) => Some(url.clone()),
                    },
                    art: player.art.clone(),
                    position: status.position,
                    rate: status.speed,
                    can_previous: player.playlist.is_list(),
                    can_next: player.playlist.next().is_some(),
                    can_seek: info.is_some_and(|info| info.seekable),
                }
            })
            .unwrap_or_default();
        service.update(now);
    }

    /// Does what media keys or the system asked of the media window.
    fn media(&mut self, command: crate::media::Command) -> Task<Message> {
        use crate::media::Command;
        let Some(id) = self.media_window() else {
            return Task::none();
        };
        match command {
            Command::Next => return self.next(id),
            Command::Previous => return self.previous(id),
            #[cfg(target_os = "linux")]
            Command::Raise => return window::gain_focus(id),
            #[cfg(target_os = "linux")]
            Command::Quit => return self.perform(id, Action::Quit),
            Command::SeekTo(seconds) => return self.control(id, Control::SeekTo(seconds)),
            _ => {}
        }
        if let Some(player) = self.player_mut(id) {
            match command {
                Command::PlayPause => player.player.toggle(),
                Command::Play => player.player.play(),
                Command::Pause => player.player.pause(),
                Command::Stop => {
                    player.player.pause();
                    player.player.seek(Duration::ZERO, true);
                }
                Command::SeekBy(seconds) => player.player.seek_by(seconds),
                _ => {}
            }
            player.refresh();
        }
        Task::none()
    }

    /// Keeps the screen on while video plays, and the computer from
    /// sleeping while sound does; lets go once nothing plays.
    fn keep_awake(&mut self) {
        let playing = |video: bool| {
            self.windows.values().any(|window| {
                window.player().is_some_and(|player| {
                    !player.status.paused && !player.status.ended && player.is_video() == video
                })
            })
        };
        let wanted = if playing(true) {
            Some(Awake::Screen)
        } else if playing(false) {
            Some(Awake::Computer)
        } else {
            None
        };
        if self.awake.as_ref().map(|(kind, _)| *kind) == wanted {
            return;
        }
        // Tried once for each change, held or not, so a system without the
        // services is not asked again on every message.
        self.awake = wanted.map(|kind| (kind, hold_awake(kind)));
    }

    fn handle(&mut self, message: Message) -> Task<Message> {
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
                    return exit();
                }
                Task::none()
            }
            Message::CloseRequested(id) => {
                if self.ask_before_closing(id, export::Closing::Window) {
                    Task::none()
                } else {
                    window::close(id)
                }
            }
            Message::External(External::Export(id, event)) => {
                self.export_event(id, event);
                Task::none()
            }
            Message::External(External::Encoder(encoding, found)) => {
                self.probing.remove(&encoding);
                self.encoders.insert(encoding, found);
                Task::none()
            }
            Message::ExportFormat(id, format) => {
                if let Some(choice) = self
                    .windows
                    .get_mut(&id)
                    .and_then(|window| window.export.as_mut())
                {
                    choice.format = format;
                }
                self.find_encoder(id);
                Task::none()
            }
            Message::ExportSize(id, lines) => {
                if let Some(choice) = self
                    .windows
                    .get_mut(&id)
                    .and_then(|window| window.export.as_mut())
                {
                    choice.lines = lines;
                }
                self.find_encoder(id);
                Task::none()
            }
            Message::ExportSound(id, sound) => {
                self.choose_sound(id, sound);
                Task::none()
            }
            Message::ExportQuality(id, quality) => {
                if let Some(choice) = self
                    .windows
                    .get_mut(&id)
                    .and_then(|window| window.export.as_mut())
                {
                    choice.quality = quality;
                }
                Task::none()
            }
            Message::ExportDismissed(id) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.export = None;
                }
                Task::none()
            }
            Message::ExportChosen(id) => self.choose_export_file(id),
            Message::ExportTo(id, result) => {
                match result {
                    Ok(Some(destination)) => self.start_export(id, destination),
                    // Cancelled: back to the dialog.
                    Ok(None) => {}
                    Err(error) => {
                        self.notify(id, crate::fl!("notice-dialog-failed", error = error));
                    }
                }
                Task::none()
            }
            Message::ExportStop(id) => {
                self.stop_export(id);
                Task::none()
            }
            Message::CloseConfirmed(id) => self.confirm_close(id),
            Message::CloseDismissed(id) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.closing = None;
                }
                Task::none()
            }
            Message::Resized(id, size) => {
                // A window shows on its screen only once it has a size, so
                // the screen may only now be known.
                let screen = match self.windows.get(&id) {
                    Some(window) if window.screen_lines.is_none() => screen_lines(id),
                    _ => Task::none(),
                };
                if let Some(window) = self.windows.get_mut(&id) {
                    window.size = size;
                    // Entering or leaving full screen resizes the window,
                    // and the bar leaves or comes back.
                    #[cfg(target_os = "macos")]
                    {
                        window.title_bar =
                            crate::title_bar_macos::height(window.surface.unwrap_or(0));
                    }
                }
                screen
            }
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            Message::Surface(id, surface) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.surface = surface;
                    #[cfg(target_os = "macos")]
                    {
                        window.title_bar = crate::title_bar_macos::height(surface.unwrap_or(0));
                    }
                }
                Task::none()
            }
            #[cfg(target_os = "macos")]
            Message::Focused(id) => {
                self.focused = Some(id);
                Task::none()
            }
            #[cfg(target_os = "macos")]
            Message::TitleBarDoubleClicked(id) => {
                if let Some(view) = self.windows.get(&id).and_then(|window| window.surface) {
                    crate::title_bar_macos::double_clicked(view);
                }
                Task::none()
            }
            #[cfg(target_os = "macos")]
            Message::External(External::Menu(action)) => {
                let target = self
                    .focused
                    .filter(|id| self.windows.contains_key(id))
                    .or_else(|| self.windows.keys().next().copied());
                match (target, action) {
                    (Some(id), action) => self.perform(id, action),
                    (None, Action::Quit) => exit(),
                    (None, _) => Task::none(),
                }
            }
            #[cfg(target_os = "linux")]
            Message::External(External::Drag(event)) => self.dragged(event),
            Message::Rescaled(id, scale) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.scale = scale;
                }
                Task::none()
            }
            Message::Screen(id, lines) => {
                let quality = self.quality(lines);
                if let Some(window) = self.windows.get_mut(&id) {
                    window.screen_lines = lines;
                    // A player still opening chooses for this screen.
                    if let Content::Player(player) = &mut window.content
                        && player.info.is_none()
                        && player.quality != quality
                        && self.settings.stream_quality == StreamQuality::Auto
                    {
                        player.quality = quality;
                        player.player.prefer(quality);
                    }
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
                // A drop of several files arrives a file at a time; they
                // play as one list once the last has come.
                if let Some(window) = self.windows.get_mut(&id) {
                    if window.dropped.is_empty() {
                        std::thread::spawn(move || {
                            std::thread::sleep(DROP_GATHER);
                            crate::post(External::DropsGathered(id));
                        });
                    }
                    window.dropped.push(Source::File(path));
                }
                Task::none()
            }
            Message::External(External::Thumbnail(id, thumbnail)) => {
                if let Some(player) = self.player_mut(id) {
                    if thumbnail.strip {
                        player.add_still(thumbnail.asked, thumbnail.picture);
                    } else {
                        let picture = (*thumbnail.picture).clone().transformed(player.transform());
                        let aspect = picture.width as f32 / picture.height.max(1) as f32;
                        player.thumbnail = Some((
                            image::Handle::from_rgba(picture.width, picture.height, picture.rgba),
                            aspect,
                        ));
                    }
                }
                Task::none()
            }
            Message::External(External::Media(command)) => self.media(command),
            Message::External(External::DropsGathered(id)) => {
                let Some(window) = self.windows.get_mut(&id) else {
                    return Task::none();
                };
                let dropped = std::mem::take(&mut window.dropped);
                let sources = dropped.into_iter().map(canonical).collect();
                if let Some(list) = self.playlist_for(sources) {
                    self.play_in(id, list, false);
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
                // Into this window when it shows nothing yet.
                self.open_sources(vec![Source::parse(text)], Some(id))
            }
            Message::AddressCancelled(id) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.address = None;
                }
                Task::none()
            }
            Message::AddressChosen(id, url) => {
                if let Some(window) = self.windows.get_mut(&id) {
                    window.address = None;
                }
                self.open_sources(vec![Source::parse(&url)], Some(id))
            }
            Message::AddressForgotten(url) => {
                self.recent.remove(&url);
                self.save_recent();
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
        let overlay = self.settings.overlay;
        let scale = self.windows.get(&id).map_or(1.0, |window| window.scale);
        // Recent addresses take their names from what opens.
        let named = match (&event, self.player_mut(id)) {
            (PlayerEvent::Page(page), Some(player)) => {
                Some((player.source().clone(), page.title.clone()))
            }
            (PlayerEvent::List(title, _), Some(player)) => {
                Some((player.source().clone(), title.clone()))
            }
            (PlayerEvent::Opened { info, .. }, Some(player)) if player.page.is_none() => {
                Some((player.source().clone(), info.title.clone()))
            }
            _ => None,
        };
        if let Some((source, title)) = named {
            self.name_address(&source, title.as_deref());
        }
        let Some(player) = self.player_mut(id) else {
            return Task::none();
        };
        match event {
            PlayerEvent::Opened { info, cover } => {
                player.art = cover
                    .as_deref()
                    .and_then(|cover| save_cover(player.source(), cover));
                player.cover = cover.map(|cover| {
                    image::Handle::from_rgba(cover.width, cover.height, cover.rgba.clone())
                });
                player.refresh();
                let video = info
                    .picture(player.status.video)
                    .map(|track| (track.width, track.height, track.aspect, track.quarter_turns));
                // Pictures over the timeline, for files: a stream would be
                // read a second time over the network.
                if let Source::File(path) = player.source()
                    && info.has_video()
                    && info.seekable
                {
                    let width = (crate::timeline::THUMBNAIL_SIZE * scale).round() as u32;
                    player.thumbnails =
                        Some(Thumbnails::open(path.clone(), width, move |thumbnail| {
                            crate::post(External::Thumbnail(id, thumbnail));
                        }));
                }
                player.info = Some(info);
                player.refresh_chapters();
                player.refresh();
                let size = match video {
                    Some((width, height, aspect, turns)) => {
                        fitted_window(width, height, aspect, turns, overlay, WINDOW_ROOM)
                    }
                    None => Size::new(440.0, 600.0),
                };
                if fullscreen || !player.fit_window {
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
            PlayerEvent::Page(page) => {
                player.page = Some(page);
                Task::none()
            }
            PlayerEvent::List(_, entries) => {
                let items = entries
                    .into_iter()
                    .map(|entry| (Source::Url(entry.url), entry.title))
                    .collect();
                match Playlist::titled(items, 0) {
                    Some(list) => self.play_in(id, list, false),
                    None => player.error = Some(crate::fl!("error-empty-list")),
                }
                Task::none()
            }
            PlayerEvent::NeedsYtDlp => {
                player.error = Some(crate::fl!("error-needs-yt-dlp"));
                Task::none()
            }
            PlayerEvent::Ended => {
                if player.looping {
                    player.player.play();
                }
                player.refresh();
                if !player.looping && player.playlist.next().is_some() {
                    return self.next(id);
                }
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
            if window.closing.take().is_some() || window.export.take().is_some() {
                return Task::none();
            }
            if let Content::Player(player) = &mut window.content
                && (player.menu.is_some() || player.sheet.is_some())
            {
                player.menu = None;
                player.sheet = None;
                return Task::none();
            }
            if let Some(mode) = window.player().and_then(|player| player.editor.mode) {
                let control = match mode {
                    Mode::Trim { .. } => Control::CancelTrim,
                    Mode::Clips { .. } => Control::DoneEditing,
                };
                return self.control(id, control);
            }
            if window.fullscreen {
                return self.set_fullscreen(id, false);
            }
            return Task::none();
        }
        if window.settings_open
            || window.address.is_some()
            || window.export.is_some()
            || window.closing.is_some()
        {
            return Task::none();
        }
        // Keys of their own while editing.
        if let Some(player) = window.player()
            && let Some(mode) = player.editor.mode
            && !modifiers.command()
        {
            let position = player.player.position().as_secs_f64();
            let control = match (mode, &key) {
                (Mode::Trim { .. }, Key::Named(keyboard::key::Named::Enter)) => {
                    Some(Control::ApplyTrim)
                }
                (Mode::Clips { .. }, Key::Named(keyboard::key::Named::Enter)) => {
                    Some(Control::DoneEditing)
                }
                (
                    Mode::Clips {
                        selected: Some(clip),
                    },
                    Key::Named(keyboard::key::Named::Delete | keyboard::key::Named::Backspace),
                ) => Some(Control::RemoveClip(clip)),
                // The part kept starts or ends where the playhead is.
                (Mode::Trim { to, .. }, Key::Character(character))
                    if character.eq_ignore_ascii_case("i") && position < to =>
                {
                    Some(Control::TrimTo(position, to, position))
                }
                (Mode::Trim { from, .. }, Key::Character(character))
                    if character.eq_ignore_ascii_case("o") && position > from =>
                {
                    Some(Control::TrimTo(from, position, position))
                }
                _ => None,
            };
            if let Some(control) = control {
                return self
                    .control(id, control)
                    .chain(Task::done(Message::Control(id, Control::ScrubEnded)));
            }
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
            Action::CloseWindow => {
                if self.ask_before_closing(id, export::Closing::Window) {
                    return Task::none();
                }
                return window::close(id);
            }
            Action::Quit => {
                // Asked in the window that would lose something, if any.
                let losing = self
                    .windows
                    .keys()
                    .copied()
                    .find(|window| self.would_lose(*window));
                if let Some(losing) = losing
                    && self.ask_before_closing(losing, export::Closing::Quit)
                {
                    return window::gain_focus(losing);
                }
                return self.quit();
            }
            Action::Settings => {
                self.audio_devices = plaa_media::output_devices();
                self.yt_dlp_found = plaa_media::web::program().is_some();
                if let Some(window) = self.windows.get_mut(&id) {
                    window.settings_open = true;
                }
                return Task::none();
            }
            Action::PictureInPicture => return self.toggle_pip(id),
            Action::Previous => return self.previous(id),
            Action::Next => return self.next(id),
            Action::FullScreen => {
                let fullscreen = self
                    .windows
                    .get(&id)
                    .is_some_and(|window| window.fullscreen);
                return self.set_fullscreen(id, !fullscreen);
            }
            Action::Trim
            | Action::Split
            | Action::RotateLeft
            | Action::RotateRight
            | Action::FlipHorizontal
            | Action::FlipVertical
            | Action::Undo
            | Action::Redo
            | Action::Revert
            | Action::Export => {
                let task = self.edit_action(id, action);
                self.touch(id);
                return task;
            }
            _ => {}
        }
        let delay = self.settings.overlay_delay();
        let volume = self.settings.volume;
        let Some(player) = self.player_mut(id) else {
            return Task::none();
        };
        let frame = player
            .picture()
            .and_then(|track| track.frame_rate)
            .map_or(1.0 / 25.0, |rate| 1.0 / rate);
        match action {
            Action::Info => player.toggle_sheet(Sheet::Inspector),
            Action::Playlist => {
                if player.playlist.is_list() {
                    player.toggle_sheet(Sheet::Playlist);
                }
            }
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
                let tracks: Vec<usize> = info
                    .audio_with(player.status.video)
                    .iter()
                    .map(|track| track.track.index)
                    .collect();
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
            Action::AbLoop => {
                let position = player.player.position().as_secs_f64();
                let long = player
                    .status
                    .duration
                    .is_some_and(|duration| duration.as_secs() >= 3600);
                let time = |seconds: f64| crate::format::time(seconds, long);
                let notice = match player.ab_loop {
                    Some((from, None)) if position > from + 0.1 => {
                        player.player.set_ab_loop(Some((
                            Duration::from_secs_f64(from),
                            Duration::from_secs_f64(position),
                        )));
                        player.ab_loop = Some((from, Some(position)));
                        crate::fl!(
                            "notice-loop-between",
                            from = time(from),
                            to = time(position)
                        )
                    }
                    Some((_, Some(_))) => {
                        player.player.set_ab_loop(None);
                        player.ab_loop = None;
                        crate::fl!("notice-loop-off")
                    }
                    // A start, or a new one before the last.
                    _ => {
                        player.ab_loop = Some((position, None));
                        crate::fl!("notice-loop-from", time = time(position))
                    }
                };
                self.notify(id, notice);
                return Task::none();
            }
            Action::PreviousChapter | Action::NextChapter => {
                if let Some(info) = &player.info
                    && let Some(target) = crate::timeline::chapter_step(
                        &info.chapters,
                        player.player.position().as_secs_f64(),
                        action == Action::NextChapter,
                        RESTART_WITHIN.as_secs_f64(),
                    )
                {
                    player.player.seek(Duration::from_secs_f64(target), true);
                }
            }
            _ => {}
        }
        player.refresh();
        player.activity.touch();
        player.refresh_overlay(delay);
        Task::none()
    }

    /// Keeps where each file was left, and lets plaa go.
    fn quit(&mut self) -> Task<Message> {
        self.save_positions();
        exit()
    }

    /// Takes window `id`'s status after a change, its overlay showing.
    fn touch(&mut self, id: window::Id) {
        let delay = self.settings.overlay_delay();
        if let Some(player) = self.player_mut(id) {
            player.refresh();
            player.activity.touch();
            player.refresh_overlay(delay);
        }
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
        if !matches!(control, Control::Hover(_) | Control::Activity) {
            self.media_window = Some(id);
        }
        // What a menu chose closes it.
        if matches!(control, Control::Open | Control::Action(_))
            && let Some(player) = self.player_mut(id)
        {
            player.menu = None;
        }
        let delay = self.settings.overlay_delay();
        match control {
            Control::Open => return self.perform(id, Action::Open),
            Control::Settings => return self.perform(id, Action::Settings),
            Control::Close => return window::close(id),
            Control::ToggleFullScreen => return self.perform(id, Action::FullScreen),
            Control::Action(action) => return self.perform(id, action),
            Control::TrimTo(..)
            | Control::ApplyTrim
            | Control::CancelTrim
            | Control::SelectClip(_)
            | Control::MoveClip(..)
            | Control::RemoveClip(_)
            | Control::TrimClip(_)
            | Control::DoneEditing => {
                let task = self.edit_control(id, control);
                self.touch(id);
                return task;
            }
            Control::PlayItem(index) => return self.play_item(id, index),
            Control::TogglePip => return self.toggle_pip(id),
            Control::Previous => return self.previous(id),
            Control::Next => return self.next(id),
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
                let Some((width, height, rgba)) = player.player.snapshot() else {
                    return Task::none();
                };
                // As it shows: upright, and as edited.
                let own = player.picture().map_or(0, |track| track.quarter_turns);
                let shown = plaa_media::Picture {
                    width,
                    height,
                    rgba,
                }
                .transformed(player.transform().after_turns(own));
                let picture = (shown.width, shown.height, shown.rgba);
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
            Control::Hover(seconds) => {
                match (seconds, &player.thumbnails) {
                    (Some(seconds), Some(thumbnails)) => {
                        thumbnails.request(player.editor.source_of(seconds));
                    }
                    // Gone, so pointing again shows no picture from before.
                    (None, _) => player.thumbnail = None,
                    _ => {}
                }
                // The overlay stays while the timeline is pointed at.
                return Task::none();
            }
            Control::SeekTo(seconds) => {
                player
                    .player
                    .seek(Duration::from_secs_f64(seconds.max(0.0)), true);
                player.menu = None;
            }
            Control::Speed(speed) => {
                player.player.set_speed(speed);
                player.menu = None;
            }
            Control::Quality(choice) => {
                player.menu = None;
                match choice {
                    QualityChoice::Variant(video) => player.player.select_video(video),
                    QualityChoice::Lines(lines) => self.reopen(id, Quality::AtMost(lines)),
                    QualityChoice::AudioOnly => self.reopen(id, Quality::AudioOnly),
                }
                return Task::none();
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
            Control::ToggleInspector => player.toggle_sheet(Sheet::Inspector),
            Control::TogglePlaylist => player.toggle_sheet(Sheet::Playlist),
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
            Some(Content::Player(player)) => player.window_title(),
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
        // Status and overlays need looking at while something plays, an
        // overlay or notice waits to go, or a seek is about to land.
        let ticking = self.windows.values().any(|window| {
            window.notice.is_some()
                || matches!(&window.content, Content::Player(player)
                    if !player.status.paused
                        || player.status.seeking
                        || player.overlay_shown != !player.is_video()
                        || player.info.is_none())
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
                #[cfg(target_os = "macos")]
                (Event::Window(window::Event::Focused), _) => Some(Message::Focused(id)),
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
                    pip: window.pip.is_some(),
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
            Some((notice, _)) => {
                // Above the playback controls: floating over a video, or
                // docked along the bottom.
                let player = window.player().filter(|player| player.error.is_none());
                let floating = player.is_some_and(|player| player.is_video())
                    && component::floating_bars_enabled();
                let docked = player.is_some() && !floating;
                let snackbar = component::snackbar_above_bar(
                    full(),
                    notice,
                    Message::DismissNotice(id),
                    floating,
                );
                container(snackbar)
                    .padding(iced::Padding {
                        bottom: if docked {
                            component::TOOLBAR_HEIGHT
                        } else {
                            0.0
                        },
                        ..iced::Padding::ZERO
                    })
                    .into()
            }
            None => full(),
        };
        let address = match &window.address {
            Some(text) => address_dialog(id, text, self.recent.addresses()),
            None => full(),
        };
        let settings = if window.settings_open {
            self.settings_dialog(id).map(Message::Settings)
        } else {
            full()
        };
        let exporting = match &window.exporting {
            Some(exporting) => export::progress(id, exporting),
            None => full(),
        };
        let export_dialog = match (window.export, window.player()) {
            (Some(choice), Some(player)) => export::dialog(id, choice, player, &self.encoders),
            _ => full(),
        };
        let closing = match window.closing {
            Some(closing) => export::close_dialog(
                id,
                closing,
                window.exporting.is_some(),
                window
                    .player()
                    .map_or_else(String::new, PlayerWindow::title),
            ),
            None => full(),
        };
        // Room for the transparent title bar on macOS, in the chrome
        // color, so the bar shows plaa's toolbar color behind its buttons.
        #[cfg(target_os = "macos")]
        let body: Element<'_, Message> = if window.title_bar > 0.0 {
            column![
                iced::widget::mouse_area(
                    container(space())
                        .width(Fill)
                        .height(window.title_bar)
                        .style(style::chrome)
                )
                .on_double_click(Message::TitleBarDoubleClicked(id)),
                body
            ]
            .into()
        } else {
            body
        };
        scramble_ui::smooth::smooth(stack![
            body,
            exporting,
            notice,
            address,
            export_dialog,
            settings,
            closing
        ])
    }
}

/// Saves a file's cover as a picture in the cache folder, for the system
/// to show with what plays. Returns where.
fn save_cover(source: &Source, cover: &plaa_media::Picture) -> Option<PathBuf> {
    use std::hash::{Hash, Hasher};
    let folder = plaa_store::paths::cache_dir()?.join("covers");
    std::fs::create_dir_all(&folder).ok()?;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    source.hash(&mut hasher);
    let path = folder.join(format!("{:016x}.png", hasher.finish()));
    if path.exists() {
        return Some(path);
    }
    let mut bytes = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut bytes, cover.width, cover.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().ok()?;
        writer.write_image_data(&cover.rgba).ok()?;
    }
    plaa_store::atomic::write(&path, &bytes).ok()?;
    Some(path)
}

/// The window's `wl_surface` pointer on Wayland, which drag and drop
/// events name the window by, or its `NSView` on macOS.
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn window_surface(window: &dyn window::Window) -> Option<usize> {
    use raw_window_handle::RawWindowHandle;
    match window.window_handle().ok()?.as_raw() {
        RawWindowHandle::Wayland(handle) => Some(handle.surface.as_ptr() as usize),
        RawWindowHandle::AppKit(handle) => Some(handle.ns_view.as_ptr() as usize),
        _ => None,
    }
}

/// Lets plaa go, the clipboard's worker first, while the Wayland
/// connection it uses is still open.
fn exit() -> Task<Message> {
    #[cfg(target_os = "linux")]
    smithay_clipboard::shutdown();
    iced::exit()
}

/// A file by its canonical path, so the same file is known however it was
/// named.
fn canonical(source: Source) -> Source {
    match source {
        Source::File(path) => Source::File(plaa_store::paths::canonical(&path)),
        other => other,
    }
}

impl Window {
    /// What the window plays, if anything.
    pub(crate) fn player(&self) -> Option<&PlayerWindow> {
        match &self.content {
            Content::Player(player) => Some(player),
            Content::Start => None,
        }
    }
}

/// Remembers where `player` is, for its file or stream to start there
/// next time. Live streams have nowhere to go back to.
fn remember_position(positions: &mut Positions, player: &PlayerWindow) {
    if let Some(info) = player.info.as_ref().filter(|info| info.seekable) {
        // In the file's own time, as edits are not kept.
        let position = player
            .editor
            .source_of(player.player.position().as_secs_f64());
        positions.set(
            &position_key(player.source()),
            Duration::from_secs_f64(position.max(0.0)),
            info.duration,
        );
    }
}

/// What a source's position is kept by: its path or its address.
fn position_key(source: &Source) -> String {
    match source {
        Source::File(path) => path.to_string_lossy().into_owned(),
        Source::Url(url) => url.clone(),
    }
}

/// How a window's player opens.
struct Opening {
    quality: Quality,
    /// The web page yt-dlp read already, when opening it again.
    page: Option<std::sync::Arc<Page>>,
    start: Option<Duration>,
    paused: bool,
    /// Moving on through a list, or to another quality: the window keeps
    /// its size and modes.
    moving_on: bool,
}

/// Asks for the height of window `id`'s screen in its own pixels.
fn screen_lines(id: window::Id) -> Task<Message> {
    window::monitor_size(id).then(move |size| {
        window::scale_factor(id).map(move |scale| {
            Message::Screen(
                id,
                size.map(|size| (size.height * scale).round() as u32)
                    .filter(|lines| *lines > 0),
            )
        })
    })
}

/// A window the video's size, made smaller to fit a typical screen, with
/// room for docked bars.
/// A window that shows a `width` by `height` picture whole, at most
/// `room` big, and the bars when they dock.
fn fitted_window(
    width: u32,
    height: u32,
    aspect: f64,
    turns: u8,
    overlay: bool,
    room: Size,
) -> Size {
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
    let scale = (room.width / width).min(room.height / height).min(1.0);
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

fn address_dialog<'a>(
    id: window::Id,
    text: &'a str,
    recent: &'a [recent::Address],
) -> Element<'a, Message> {
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
    let mut body = column![
        scramble_ui::styled(crate::fl!("address-title"), Type::HeadlineSmall),
        scramble_ui::aligned(
            scramble_ui::styled(crate::fl!("address-note"), Type::BodyMedium)
                .style(style::on_surface_variant)
        ),
        field,
    ]
    .spacing(16);
    if let Some(list) = recent_addresses(id, text, recent) {
        body = body.push(list);
    }
    let body = body.push(container(scramble_ui::dir::row(actions).spacing(8)).align_right(Fill));
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

/// How many recent addresses show before the list scrolls.
const RECENT_SHOWN: f32 = 5.0;
/// A recent address's row: its name over its address.
const RECENT_ROW_HEIGHT: f32 = 56.0;

/// The recent addresses that match what is typed, latest first, each
/// opened when chosen and forgotten by its own button. None when none
/// match.
fn recent_addresses<'a>(
    id: window::Id,
    typed: &str,
    recent: &'a [recent::Address],
) -> Option<Element<'a, Message>> {
    let typed = typed.trim().to_lowercase();
    let matching: Vec<&recent::Address> = recent
        .iter()
        .filter(|address| {
            typed.is_empty()
                || address.url.to_lowercase().contains(&typed)
                || address
                    .title
                    .as_ref()
                    .is_some_and(|title| title.to_lowercase().contains(&typed))
        })
        .collect();
    if matching.is_empty() {
        return None;
    }
    // Long names and addresses are cut at the row's end.
    let one_line = |text: &'a str, kind: Type, dim: bool| {
        let line = scramble_ui::styled(text, kind).wrapping(iced::widget::text::Wrapping::None);
        let line = if dim {
            line.style(style::on_surface_variant)
        } else {
            line
        };
        container(line).width(Fill).clip(true)
    };
    let shown = matching.len() as f32;
    let rows = matching.into_iter().map(|address| {
        let lines: Element<'a, Message> = match &address.title {
            Some(title) => column![
                one_line(title, Type::BodyLarge, false),
                one_line(&address.url, Type::BodySmall, true),
            ]
            .spacing(2)
            .width(Fill)
            .into(),
            None => one_line(&address.url, Type::BodyLarge, false).into(),
        };
        let forget = component::tip(
            scramble_ui::icon_button(Icon::Close)
                .size(button::Size::ExtraSmall)
                .on_press(Message::AddressForgotten(address.url.clone())),
            crate::fl!("address-forget"),
        );
        iced::widget::button(
            row![
                icon::icon(Icon::History, 20).style(style::on_surface_variant),
                lines,
                forget
            ]
            .spacing(12)
            .align_y(Center),
        )
        .padding([4, 8])
        .height(RECENT_ROW_HEIGHT)
        .width(Fill)
        .style(|theme, status| crate::window::list_row_style(theme, status, false))
        .on_press(Message::AddressChosen(id, address.url.clone()))
        .into()
    });
    Some(
        column![
            component::section(crate::fl!("address-recent")),
            component::scroll(iced::widget::Column::with_children(rows).spacing(2))
                .height(shown.min(RECENT_SHOWN) * (RECENT_ROW_HEIGHT + 2.0)),
        ]
        .spacing(4)
        .into(),
    )
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

/// On macOS, the title bar is transparent over plaa's own chrome, which
/// runs up behind it.
#[cfg(target_os = "macos")]
fn platform_settings() -> PlatformSpecific {
    PlatformSpecific {
        titlebar_transparent: true,
        fullsize_content_view: true,
        ..PlatformSpecific::default()
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn platform_settings() -> PlatformSpecific {
    PlatformSpecific::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_fit_the_video_and_a_screen() {
        assert_eq!(
            fitted_window(640, 360, 16.0 / 9.0, 0, true, WINDOW_ROOM),
            Size::new(640.0, 360.0)
        );
        let big = fitted_window(3840, 2160, 16.0 / 9.0, 0, true, WINDOW_ROOM);
        assert!(big.width <= 1280.0 && big.height <= 800.0);
        let portrait = fitted_window(1920, 1080, 16.0 / 9.0, 1, true, WINDOW_ROOM);
        assert!(portrait.height > portrait.width);
        // Picture in picture is small, yet no smaller than a window gets.
        let pip = fitted_window(3840, 2160, 16.0 / 9.0, 0, true, PIP_ROOM);
        assert_eq!(pip, Size::new(400.0, 240.0));
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
