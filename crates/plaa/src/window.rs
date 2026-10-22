//! A window playing a file, or a list of them one at a time: the picture
//! or the cover art, the toolbar and the playback controls, docked or
//! floating over the picture, and the inspector and the list beside it.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use iced::widget::{container, image, mouse_area, slider, space, stack, text};
use iced::{Center, Element, Fill, Length, Theme};
use plaa_media::info::Chapter;
use plaa_media::subtitle::Cue;
use plaa_media::waveform::{Shape, Waveform};
use plaa_media::web::Page;
use plaa_media::{MediaInfo, Picture, Player, Quality, Source, Status, Thumbnails, Transform};
use plaa_store::settings::Visualizer;
use scramble_ui::component::{self, Backdrop};
use scramble_ui::dir::row;
use scramble_ui::dropdown::{self, Entry};
use scramble_ui::{Icon, Scheme, Type, button, column, icon, style};

use crate::edit_bar::{ClipStrip, Still, TrimBar};
use crate::editing::{Editor, Mode};
use crate::format;
use crate::playlist::Playlist;
use crate::shortcuts::Action;
use crate::timeline::{self, Timeline};

/// The speeds the speed menu offers.
pub const SPEEDS: [f64; 8] = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 2.0, 3.0];

/// How long opening may take before a spinner says it is under way.
const OPENING_SHOWN: Duration = Duration::from_millis(300);

/// How far the skip buttons and arrow keys move, in seconds.
pub const SKIP: f64 = 10.0;
pub const SKIP_SHORT: f64 = 5.0;

/// What can show in the sheet beside the picture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sheet {
    Inspector,
    Playlist,
    /// The AVB network's talkers and listeners.
    Network,
}

/// The menus a window can have open, which keep its overlay shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Menu {
    Open,
    Speed,
    Tracks,
    Chapters,
    Quality,
    Edit,
    Visualizer,
    /// The formats a listener and plaa share, to play there in.
    AvbFormats((plaa_avb::EntityId, u16)),
    /// The talkers on the AVB network to listen to.
    Connect,
    /// The renderers to cast to.
    Cast,
}

/// What a window casts to a renderer on the network, and how the renderer
/// last said it stands.
pub struct Casting {
    /// The renderer, by its id and name.
    pub id: String,
    pub name: String,
    pub session: plaa_cast::Session,
    pub status: Option<plaa_cast::Status>,
    /// Tells this cast's events from those of one before it.
    pub generation: u64,
}

impl Casting {
    /// Whether the renderer plays, or is about to.
    pub fn playing(&self) -> bool {
        self.status.is_none_or(|status| {
            matches!(
                status.state,
                plaa_cast::State::Playing | plaa_cast::State::Transitioning
            )
        })
    }
}

/// What the quality menu offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityChoice {
    /// A web page's picture at most this many lines tall.
    Lines(u32),
    /// A web page's sound alone.
    AudioOnly,
    /// One of a stream's variants, by its picture's stream.
    Variant(usize),
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
    /// The cover saved as a picture, for the desktop to show.
    pub art: Option<std::path::PathBuf>,
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
    pub sheet: Option<Sheet>,
    /// The renderer the window casts to, whose playback its controls
    /// drive and its timeline shows while it does.
    pub casting: Option<Casting>,
    /// The listener chosen in the Network Audio sheet, which it details.
    pub listener: Option<(plaa_avb::EntityId, u16)>,
    pub looping: bool,
    /// What the window plays, the player's source being the current item.
    pub playlist: Playlist,
    /// The window fits itself to the picture when it opens: not when a
    /// list moves on.
    pub fit_window: bool,
    /// Where a loop within the file starts, and once set, where it ends.
    pub ab_loop: Option<(f64, Option<f64>)>,
    /// Makes the pictures shown over the timeline, for files with video.
    pub thumbnails: Option<Thumbnails>,
    /// The latest of them, and its width over its height.
    pub thumbnail: Option<(image::Handle, f32)>,
    pub subtitle: Option<Cue>,
    pub volume: f32,
    pub muted: bool,
    /// When the player started opening its source.
    pub opened: Instant,
    /// The web page the source is, as yt-dlp read it.
    pub page: Option<Arc<Page>>,
    /// The quality asked for, where the source comes in several.
    pub quality: Quality,
    /// The edit of the file, its history and what is being edited.
    pub editor: Editor,
    /// How far the picture is zoomed in, and where.
    pub zoom: crate::video::Zoom,
    /// The file's chapters as they fall in the edit.
    pub chapters: Vec<Chapter>,
    /// Pictures along the file for the editing bars' filmstrips, by the
    /// file's time, as they come and as they show.
    pub strip: Vec<(f64, Arc<Picture>)>,
    pub stills: Vec<Still>,
    /// Reads a sound file's waveform, once it is to be shown.
    pub waveform: Option<Waveform>,
    /// The waveform read so far.
    pub shape: Option<Arc<Shape>>,
    /// What a sound file shows, as Settings have it.
    pub visualizer: Visualizer,
}

impl PlayerWindow {
    pub fn new(player: Player, playlist: Playlist, surface: u64, volume: f32, muted: bool) -> Self {
        Self {
            status: player.status(),
            player,
            surface,
            info: None,
            cover: None,
            art: None,
            decoding: None,
            error: None,
            no_sound: None,
            dragging: None,
            activity: Arc::new(Activity::new()),
            pointer_inside: true,
            overlay_shown: true,
            menu: None,
            sheet: None,
            listener: None,
            looping: false,
            playlist,
            fit_window: true,
            ab_loop: None,
            thumbnails: None,
            thumbnail: None,
            subtitle: None,
            volume,
            muted,
            opened: Instant::now(),
            page: None,
            quality: Quality::Best,
            editor: Editor::default(),
            zoom: crate::video::Zoom::default(),
            chapters: Vec::new(),
            strip: Vec::new(),
            stills: Vec::new(),
            waveform: None,
            shape: None,
            visualizer: Visualizer::default(),
            casting: None,
        }
    }

    pub fn source(&self) -> &Source {
        self.player.source()
    }

    /// Whether the file shows a moving picture, rather than only sound.
    pub fn is_video(&self) -> bool {
        self.info.as_ref().is_some_and(|info| info.has_video())
    }

    /// Whether a picture fills the window, the bars floating over it: a
    /// video's, or the prism's for sound.
    pub fn fills_window(&self) -> bool {
        self.is_video()
            || (self.info.is_some() && self.error.is_none() && self.visualizer == Visualizer::Prism)
    }

    /// The picture's track that shows.
    pub fn picture(&self) -> Option<&plaa_media::info::VideoTrack> {
        self.info.as_ref()?.picture(self.status.video)
    }

    /// How the edit turns and flips the picture.
    pub fn transform(&self) -> Transform {
        self.editor.transform()
    }

    /// How long the file lasts, if known.
    pub fn file_duration(&self) -> Option<f64> {
        self.info
            .as_ref()
            .and_then(|info| info.duration)
            .map(|duration| duration.as_secs_f64())
            .filter(|seconds| *seconds > 0.0)
    }

    /// Whether the file can be edited: a file, not a stream, of a known
    /// length that plays.
    pub fn can_edit(&self) -> bool {
        matches!(self.source(), Source::File(_))
            && self.error.is_none()
            && self.info.as_ref().is_some_and(|info| info.seekable)
            && self.file_duration().is_some()
    }

    /// Takes the chapters again after the edit changed.
    pub fn refresh_chapters(&mut self) {
        let chapters = self
            .info
            .as_ref()
            .map_or(&[][..], |info| info.chapters.as_slice());
        self.chapters = self.editor.chapters(chapters);
    }

    /// Notes a picture for the filmstrips, as it shows.
    pub fn add_still(&mut self, seconds: f64, picture: Arc<Picture>) {
        let at = self.strip.partition_point(|(time, _)| *time < seconds);
        if self.strip.get(at).is_some_and(|(time, _)| *time == seconds) {
            return;
        }
        self.stills
            .insert(at, still(seconds, &picture, self.transform()));
        self.strip.insert(at, (seconds, picture));
    }

    /// Turns the filmstrips' pictures again after the edit turned or
    /// flipped the picture.
    pub fn refresh_stills(&mut self) {
        let transform = self.transform();
        self.stills = self
            .strip
            .iter()
            .map(|(seconds, picture)| still(*seconds, picture, transform))
            .collect();
    }

    /// The name to show: for a stream, its title as the page, the stream
    /// or the list says; for a sound file, its title tag; else the file's
    /// name.
    pub fn title(&self) -> String {
        let tagged = self.info.as_ref().and_then(|info| info.title.clone());
        match self.source() {
            Source::Url(_) => tagged
                .or_else(|| self.page.as_ref().and_then(|page| page.title.clone()))
                .or_else(|| {
                    self.playlist
                        .title(self.playlist.current())
                        .map(str::to_owned)
                })
                .unwrap_or_else(|| self.source().name()),
            Source::File(_) if self.info.is_some() && !self.is_video() => {
                tagged.unwrap_or_else(|| self.source().name())
            }
            Source::File(_) => self.source().name(),
        }
    }

    /// The window's title: the name, marked while it has changes no
    /// export has kept.
    pub fn window_title(&self) -> String {
        if self.editor.unexported() {
            crate::fl!("title-edited", name = self.title())
        } else {
            self.title()
        }
    }

    /// Whether the floating overlay should show now: always until the file
    /// has opened, and while a menu is open or something is dragged; else
    /// while the pointer is inside the window and it is paused, being
    /// edited, shows a sheet, or the pointer moved lately.
    pub fn wants_overlay(&self, delay: Duration) -> bool {
        if self.info.is_none() || self.error.is_some() {
            return true;
        }
        if self.menu.is_some() || self.dragging.is_some() {
            return true;
        }
        if !self.pointer_inside {
            return false;
        }
        self.status.paused
            || self.sheet.is_some()
            || self.editor.mode.is_some()
            || self.activity.idle() < delay
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

    /// Opens `sheet`, in place of another, or closes it.
    pub fn toggle_sheet(&mut self, sheet: Sheet) {
        self.sheet = (self.sheet != Some(sheet)).then_some(sheet);
    }

    /// Takes the player's latest status and subtitle.
    pub fn refresh(&mut self) {
        self.status = self.player.status();
        self.subtitle = self.player.subtitle();
        // While casting, the renderer's playback shows.
        if let Some(casting) = &self.casting {
            self.status.paused = !casting.playing();
            self.status.ended = false;
            self.status.buffering = casting
                .status
                .is_some_and(|status| status.state == plaa_cast::State::Transitioning);
            if let Some(status) = casting.status {
                if let Some(position) = status.position {
                    self.status.position = Duration::from_secs_f64(position.max(0.0));
                }
                if self.status.duration.is_none() {
                    self.status.duration = status.duration.map(Duration::from_secs_f64);
                }
            }
        }
    }

    /// Whether the player is waiting for its source: opening it, or for a
    /// stream to read ahead.
    pub fn waiting(&self) -> bool {
        self.error.is_none()
            && (self.status.buffering
                || (self.info.is_none() && self.opened.elapsed() >= OPENING_SHOWN))
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
    /// The timeline is pointed at this many seconds, or no longer.
    Hover(Option<f64>),
    /// Goes to this many seconds exactly, as to a chapter.
    SeekTo(f64),
    ScrubEnded,
    SkipBy(f64),
    Volume(f32),
    VolumeReleased,
    ToggleMute,
    Speed(f64),
    Quality(QualityChoice),
    Audio(Option<usize>),
    Subtitles(Option<usize>),
    Menu(Menu, bool),
    ToggleFullScreen,
    ToggleInspector,
    TogglePlaylist,
    /// Shows or hides the AVB network's talkers and listeners.
    ToggleNetwork,
    /// Binds a listener's input to a talker's output on the AVB network,
    /// in a format chosen or the talker's.
    AvbConnect(
        (plaa_avb::EntityId, u16),
        (plaa_avb::EntityId, u16),
        Option<plaa_avb::StreamFormat>,
    ),
    /// Unbinds a listener's input.
    AvbDisconnect((plaa_avb::EntityId, u16)),
    /// Chooses a listener in the Network Audio sheet, to see its details.
    AvbChoose((plaa_avb::EntityId, u16)),
    /// Opens or closes the assistant's panel.
    ToggleAssistant,
    /// Casts to the renderer with this id.
    CastTo(String),
    StopCasting,
    TogglePip,
    /// Plays the list's item at this index.
    PlayItem(usize),
    Previous,
    Next,
    ToggleLoop,
    /// Floats the bars over pictures or docks them, for every window.
    ToggleOverlay,
    Screenshot,
    /// Shows this under a sound file's cover, in every window.
    Visualizer(Visualizer),
    Open,
    Settings,
    Close,
    /// The pointer moved over the picture while the overlay was hidden.
    Activity,
    /// The wheel turned over the picture, by lines.
    Wheel(f32),
    /// The command key and the wheel zoomed the picture, or a drag moved
    /// it while zoomed.
    Zoom(crate::video::ZoomChange),
    /// Does what a menu item or shortcut would.
    Action(Action),
    /// The trim bar keeps from and to these seconds, showing the third.
    TrimTo(f64, f64, f64),
    ApplyTrim,
    CancelTrim,
    SelectClip(Option<usize>),
    /// Moves a clip to another place among them.
    MoveClip(usize, usize),
    RemoveClip(usize),
    /// Trims one clip.
    TrimClip(usize),
    DoneEditing,
}

/// Everything the window's view needs besides its own state.
pub struct Context {
    pub fullscreen: bool,
    /// In picture in picture.
    pub pip: bool,
    pub width: f32,
    /// The window's scale factor.
    pub scale: f32,
    /// The window manager draws no title bar, so the toolbar ends with a
    /// close button.
    pub close_button: bool,
    /// The AVB network, while plaa is on one.
    pub avb: Option<Arc<plaa_avb::Snapshot>>,
    /// The renderers on the network, once looked for.
    pub renderers: Option<Arc<Vec<plaa_cast::Renderer>>>,
    /// The assistant's panel is open beside the window.
    pub assistant_open: bool,
}

/// The window: the picture with bars over it or around it, or for sound,
/// the cover with bars over it or docked above and below.
pub fn view<'a>(window: &'a PlayerWindow, context: &Context) -> Element<'a, Control> {
    if let Some(error) = &window.error {
        let body = container(component::empty_state(
            Icon::Error,
            window.title(),
            error.as_str(),
        ))
        .style(style::surface);
        return column![docked(toolbar(window, context)), body].into();
    }
    if window.info.is_some() && !window.is_video() {
        return if window.fills_window() {
            prism_view(window, context)
        } else {
            audio_view(window, context)
        };
    }
    let picture = mouse_area(crate::video::view(
        &window.player,
        window.surface,
        (window.transform(), window.zoom),
        window.activity.clone(),
        !window.overlay_shown,
        Control::Activity,
        Control::Zoom,
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
    let with_subtitles = with_spinner(with_subtitles, window);
    let body: Element<'a, Control> = match sheet(window, context) {
        Some(sheet) => {
            let sheet = component::between_bars(
                sheet,
                component::floating_room(true),
                component::floating_room(true),
            );
            iced::widget::row![with_subtitles, sheet].into()
        }
        None => with_subtitles,
    };
    component::window_bars(
        component::toolbar(toolbar(window, context)),
        Some(component::toolbar(transport(window, context))),
        body,
        window.overlay_shown,
    )
}

/// The toolbar: open, and connect while on an AVB network, and the
/// window's tools at the other end. The file's name is the window's
/// title, as tools would cover it in a narrow window.
fn toolbar<'a>(window: &'a PlayerWindow, context: &Context) -> Element<'a, Control> {
    let open = dropdown::icon_menu(
        Icon::FolderOpen,
        vec![
            Entry::item(crate::fl!("open-file"), Control::Open).icon(Icon::FileOpen),
            Entry::item(
                crate::fl!("open-address"),
                Control::Action(Action::OpenAddress),
            )
            .icon(Icon::Link),
            Entry::divider(),
            Entry::item(
                crate::fl!("open-record-sound"),
                Control::Action(Action::RecordSound),
            )
            .icon(Icon::Mic),
            Entry::item(
                crate::fl!("open-record-screen"),
                Control::Action(Action::RecordScreen),
            )
            .icon(Icon::ScreenRecord),
            Entry::item(
                crate::fl!("open-record-camera"),
                Control::Action(Action::RecordCamera),
            )
            .icon(Icon::Videocam),
        ],
    )
    .size(button::Size::Small)
    .tip(crate::fl!("action-open"))
    .open(window.menu == Some(Menu::Open), |open| {
        Control::Menu(Menu::Open, open)
    });
    let mut tools = vec![open.into()];
    if let Some(snapshot) = &context.avb {
        tools.push(connect_menu(window, snapshot));
    }
    tools.push(space().width(Fill).into());
    if let Some(menu) = edit_menu(window) {
        tools.push(menu);
    }
    if window.is_video() {
        tools.push(component::tool(
            Icon::Photo,
            crate::fl!("action-screenshot"),
            Some(Control::Screenshot),
        ));
    } else if window.info.is_some() {
        tools.push(visualizer_menu(window));
    }
    tools.push(component::toggle_tool(
        Icon::Repeat,
        crate::fl!("action-loop"),
        window.looping,
        Control::ToggleLoop,
    ));
    if window.playlist.is_list() {
        tools.push(component::toggle_tool(
            Icon::PlaylistPlay,
            crate::fl!("action-playlist"),
            window.sheet == Some(Sheet::Playlist),
            Control::TogglePlaylist,
        ));
    }
    tools.push(cast_menu(window, context));
    if context.avb.is_some() {
        tools.push(component::toggle_tool(
            Icon::Lan,
            crate::fl!("action-network"),
            window.sheet == Some(Sheet::Network),
            Control::ToggleNetwork,
        ));
    }
    tools.push(component::toggle_tool(
        Icon::ChatBubble,
        crate::fl!("action-assistant"),
        context.assistant_open,
        Control::ToggleAssistant,
    ));
    tools.push(component::toggle_tool(
        Icon::Info,
        crate::fl!("action-info"),
        window.sheet == Some(Sheet::Inspector),
        Control::ToggleInspector,
    ));
    if window.is_video() {
        tools.push(component::toggle_tool(
            Icon::PictureInPicture,
            crate::fl!("action-picture-in-picture"),
            context.pip,
            Control::TogglePip,
        ));
    }
    tools.push(component::floating_bars_toggle(Control::ToggleOverlay));
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

/// The playback controls: play, between the list's previous and next
/// items when there is a list, skip, the timeline, speed, volume, tracks
/// and full screen.
fn transport<'a>(window: &'a PlayerWindow, context: &Context) -> Element<'a, Control> {
    if let Some(mode) = window.editor.mode {
        return edit_bar(window, mode);
    }
    let status = &window.status;
    let play = play_button(window);
    let (previous, next) = if window.playlist.is_list() {
        (
            Some(component::tool(
                Icon::SkipPrevious,
                crate::fl!("action-previous"),
                Some(Control::Previous),
            )),
            Some(component::tool(
                Icon::SkipNext,
                crate::fl!("action-next"),
                window.playlist.next().map(|_| Control::Next),
            )),
        )
    } else {
        (None, None)
    };
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
        .chapters(&window.chapters)
        .cuts(window.editor.cuts())
        .ab_loop(window.ab_loop)
        .at(window
            .casting
            .as_ref()
            .map(|_| window.status.position.as_secs_f64()))
        .buffered(window.source().is_url() && window.casting.is_none())
        .on_hover(Control::Hover)
        .thumbnail(window.thumbnail.clone())
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
    let mut controls: Vec<Element<'a, Control>> = Vec::new();
    controls.extend(previous);
    controls.push(play);
    controls.extend(next);
    controls.extend([
        back,
        forward,
        container(time(elapsed)).padding([0, 4]).into(),
        timeline,
        container(time(remaining)).padding([0, 4]).into(),
        speed_menu(window),
        mute,
    ]);
    // Narrow windows keep the mute button and lose the slider.
    if context.width >= 640.0 {
        controls.push(volume.into());
    }
    if let Some(menu) = chapters_menu(window) {
        controls.push(menu);
    }
    if let Some(menu) = quality_menu(window) {
        controls.push(menu);
    }
    if let Some(menu) = tracks_menu(window) {
        controls.push(menu);
    }
    if window.fills_window() {
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

/// The play button, filled as the bars' main action.
fn play_button<'a>(window: &PlayerWindow) -> Element<'a, Control> {
    let status = &window.status;
    let playing = !status.paused;
    let glyph = if status.ended {
        Icon::Replay
    } else if playing {
        Icon::Pause
    } else {
        Icon::PlayArrow
    };
    let label = if playing {
        crate::fl!("action-pause")
    } else {
        crate::fl!("action-play")
    };
    component::tip(
        button::icon_button(glyph)
            .kind(button::Kind::Filled)
            .always_filled()
            .on_press(Control::TogglePlay),
        label,
    )
}

/// The bar in place of the playback controls while editing: the trim
/// bar and its times, or the clips.
fn edit_bar(window: &PlayerWindow, mode: Mode) -> Element<'_, Control> {
    let duration = window
        .status
        .duration
        .map_or(0.0, |duration| duration.as_secs_f64());
    let long = duration >= 3600.0;
    let time = |seconds: f64| -> Element<'_, Control> {
        container(
            text(format::precise_time(seconds, long))
                .size(Type::LabelLarge.size())
                .font(Type::LabelLarge.font(false))
                .wrapping(text::Wrapping::None),
        )
        .padding([0, 4])
        .into()
    };
    let height = style::SLIDER_HEIGHT;
    let mut items: Vec<Element<'_, Control>> = vec![play_button(window)];
    match mode {
        Mode::Trim { clip, from, to } => {
            let span = match clip.and_then(|clip| {
                let edit = window.editor.edit()?;
                let start = *edit.starts().get(clip)?;
                Some((start, start + edit.clips()[clip].length()))
            }) {
                Some(span) => span,
                None => (0.0, duration),
            };
            items.extend([
                time(from),
                TrimBar::new(
                    &window.player,
                    &window.editor,
                    &window.stills,
                    span,
                    (from, to),
                    Control::TrimTo,
                    Control::Scrub,
                    Control::ScrubEnded,
                )
                .height(height)
                .into(),
                time(to),
                scramble_ui::button(button::Kind::Text, crate::fl!("common-cancel"))
                    .on_press(Control::CancelTrim)
                    .into(),
                scramble_ui::button(button::Kind::Filled, crate::fl!("edit-trim-apply"))
                    .on_press(Control::ApplyTrim)
                    .into(),
            ]);
        }
        Mode::Clips { selected } => {
            let clips = window.editor.clips().map_or(1, <[_]>::len);
            items.push(
                ClipStrip::new(
                    &window.player,
                    &window.editor,
                    &window.stills,
                    selected,
                    Control::SelectClip,
                    Control::SeekTo,
                    Control::MoveClip,
                    Control::TrimClip,
                )
                .height(height)
                .into(),
            );
            items.push(component::tool(
                Icon::ContentCut,
                crate::fl!("edit-split"),
                Some(Control::Action(Action::Split)),
            ));
            items.push(component::tool(
                Icon::Delete,
                crate::fl!("edit-remove-clip"),
                selected.filter(|_| clips > 1).map(Control::RemoveClip),
            ));
            items.push(
                scramble_ui::button(button::Kind::Filled, crate::fl!("edit-done"))
                    .on_press(Control::DoneEditing)
                    .into(),
            );
        }
    }
    row(items).spacing(8).align_y(Center).width(Fill).into()
}

/// The edit menu: trimming and splitting, turning and flipping, undoing,
/// and exporting.
fn edit_menu(window: &PlayerWindow) -> Option<Element<'_, Control>> {
    if !window.can_edit() {
        return None;
    }
    let item = |label: String, action: Action| Entry::item(label, Control::Action(action));
    let mut entries = vec![
        item(crate::fl!("edit-trim"), Action::Trim).icon(Icon::ContentCut),
        item(crate::fl!("edit-split"), Action::Split).icon(Icon::ViewDay),
    ];
    if window.is_video() {
        entries.extend([
            Entry::divider(),
            item(crate::fl!("edit-rotate-left"), Action::RotateLeft).icon(Icon::RotateLeft),
            item(crate::fl!("edit-rotate-right"), Action::RotateRight).icon(Icon::RotateRight),
            item(crate::fl!("edit-flip-horizontal"), Action::FlipHorizontal).icon(Icon::Flip),
            item(crate::fl!("edit-flip-vertical"), Action::FlipVertical).icon(Icon::FlipVertical),
        ]);
    }
    let editor = &window.editor;
    if editor.can_undo() || editor.can_redo() || editor.is_edited() {
        entries.push(Entry::divider());
    }
    if editor.can_undo() {
        entries.push(item(crate::fl!("edit-undo"), Action::Undo).icon(Icon::Undo));
    }
    if editor.can_redo() {
        entries.push(item(crate::fl!("edit-redo"), Action::Redo).icon(Icon::Redo));
    }
    if editor.is_edited() {
        entries.push(item(crate::fl!("edit-revert"), Action::Revert).icon(Icon::ResetAll));
    }
    entries.extend([
        Entry::divider(),
        item(crate::fl!("edit-export"), Action::Export).icon(Icon::FileExport),
    ]);
    Some(
        dropdown::icon_menu(Icon::MovieEdit, entries)
            .size(button::Size::Small)
            .tip(crate::fl!("action-edit"))
            .selected(editor.is_edited())
            .open(window.menu == Some(Menu::Edit), |open| {
                Control::Menu(Menu::Edit, open)
            })
            .into(),
    )
}

/// The color behind the sliders' handles: the bars' own.
fn timeline_backdrop(theme: &Theme) -> iced::Color {
    Backdrop::ContainerLow.color(&Scheme::of(theme))
}

/// What shows under a sound file's cover.
fn visualizer_menu(window: &PlayerWindow) -> Element<'_, Control> {
    let current = window.visualizer;
    let choices = [
        (
            Visualizer::Spectrum,
            Icon::Equalizer,
            crate::fl!("visualizer-spectrum"),
        ),
        (
            Visualizer::Waveform,
            Icon::GraphicEq,
            crate::fl!("visualizer-waveform"),
        ),
        // Drawn by a shader of plaa's own on the graphics card, as the
        // flare marks every such visualizer.
        (
            Visualizer::Prism,
            Icon::Flare,
            crate::fl!("visualizer-prism"),
        ),
        (
            Visualizer::Cover,
            Icon::Album,
            crate::fl!("visualizer-cover"),
        ),
    ];
    let glyph = choices
        .iter()
        .find(|(choice, _, _)| *choice == current)
        .map_or(Icon::Equalizer, |(_, glyph, _)| *glyph);
    let entries = choices
        .into_iter()
        .map(|(choice, glyph, label)| {
            Entry::item(label, Control::Visualizer(choice))
                .icon(glyph)
                .checked(choice == current)
        })
        .collect();
    dropdown::icon_menu(glyph, entries)
        .size(button::Size::Small)
        .tip(crate::fl!("action-visualizer"))
        .open(window.menu == Some(Menu::Visualizer), |open| {
            Control::Menu(Menu::Visualizer, open)
        })
        .into()
}

/// The talkers on the AVB network plaa can listen to, the one it listens
/// to checked, and stopping.
fn connect_menu<'a>(
    window: &'a PlayerWindow,
    snapshot: &plaa_avb::Snapshot,
) -> Element<'a, Control> {
    dropdown::icon_menu(Icon::Cable, crate::network::connect_entries(snapshot))
        .size(button::Size::Small)
        .tip(crate::fl!("action-connect"))
        .selected(crate::network::listening_here(snapshot))
        .open(window.menu == Some(Menu::Connect), |open| {
            Control::Menu(Menu::Connect, open)
        })
        .into()
}

/// The renderers on the network to cast to, the one cast to checked, and
/// stopping; found as the menu opens.
fn cast_menu<'a>(window: &'a PlayerWindow, context: &Context) -> Element<'a, Control> {
    let mut entries = Vec::new();
    if let Some(casting) = &window.casting {
        entries.push(
            Entry::item(
                crate::fl!("cast-stop", renderer = casting.name.clone()),
                Control::StopCasting,
            )
            .icon(Icon::Stop),
        );
        entries.push(Entry::divider());
    }
    match context.renderers.as_deref().map(Vec::as_slice) {
        None => entries.push(Entry::note(crate::fl!("cast-looking"))),
        Some([]) => entries.push(Entry::note(crate::fl!("cast-none"))),
        Some(renderers) => {
            for renderer in renderers {
                let current = window
                    .casting
                    .as_ref()
                    .is_some_and(|casting| casting.id == renderer.id);
                entries.push(
                    Entry::item(renderer.name.clone(), Control::CastTo(renderer.id.clone()))
                        .checked(current),
                );
            }
        }
    }
    let glyph = if window.casting.is_some() {
        Icon::CastConnected
    } else {
        Icon::Cast
    };
    dropdown::icon_menu(glyph, entries)
        .size(button::Size::Small)
        .tip(crate::fl!("action-cast"))
        .selected(window.casting.is_some())
        .open(window.menu == Some(Menu::Cast), |open| {
            Control::Menu(Menu::Cast, open)
        })
        .into()
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
        .size(button::Size::Small)
        .tip(crate::fl!("action-speed"))
        .selected((current - 1.0).abs() > 1e-6)
        .open(window.menu == Some(Menu::Speed), |open| {
            Control::Menu(Menu::Speed, open)
        })
        .into()
}

/// The chapters to go to, when there are some.
fn chapters_menu(window: &PlayerWindow) -> Option<Element<'_, Control>> {
    let chapters = &window.chapters;
    if chapters.len() < 2 {
        return None;
    }
    let long = window
        .status
        .duration
        .is_some_and(|duration| duration.as_secs() >= 3600);
    let current =
        timeline::chapter_at(chapters, window.shown_position()).map(|chapter| chapter.start);
    let entries = chapters
        .iter()
        .enumerate()
        .map(|(number, chapter)| {
            let title = chapter
                .title
                .clone()
                .unwrap_or_else(|| crate::fl!("chapters-numbered", number = (number + 1)));
            let start = chapter.start.as_secs_f64();
            Entry::item(
                format!("{title} · {}", format::time(start, long)),
                Control::SeekTo(start),
            )
            .checked(current == Some(chapter.start))
        })
        .collect();
    Some(
        dropdown::icon_menu(Icon::Bookmarks, entries)
            .size(button::Size::Small)
            .tip(crate::fl!("action-chapters"))
            .open(window.menu == Some(Menu::Chapters), |open| {
                Control::Menu(Menu::Chapters, open)
            })
            .into(),
    )
}

/// The qualities a web page or a stream comes in, when there is a choice:
/// a page's picture heights and its sound alone, or a stream's variants.
fn quality_menu(window: &PlayerWindow) -> Option<Element<'_, Control>> {
    let showing = window.picture().map(|track| track.height);
    let mut entries = Vec::new();
    if let Some(page) = &window.page {
        let sound_alone = window.quality == Quality::AudioOnly;
        for lines in page.heights() {
            entries.push(
                Entry::item(
                    crate::fl!("quality-lines", lines = lines.to_string()),
                    Control::Quality(QualityChoice::Lines(lines)),
                )
                .checked(!sound_alone && showing == Some(lines)),
            );
        }
        if page.has_audio() && !entries.is_empty() {
            entries.push(
                Entry::item(
                    crate::fl!("quality-audio-only"),
                    Control::Quality(QualityChoice::AudioOnly),
                )
                .checked(sound_alone),
            );
        }
    } else if let Some(info) = &window.info {
        for variant in &info.variants {
            // Two of the same height are told apart by their bit rates.
            let twin = info
                .variants
                .iter()
                .filter(|other| other.height == variant.height)
                .count()
                > 1;
            let lines = crate::fl!("quality-lines", lines = variant.height.to_string());
            let label = match variant.bit_rate.filter(|_| twin) {
                Some(rate) => format!("{lines} · {}", format::bit_rate(rate)),
                None => lines,
            };
            entries.push(
                Entry::item(
                    label,
                    Control::Quality(QualityChoice::Variant(variant.video)),
                )
                .checked(window.status.video == Some(variant.video)),
            );
        }
    }
    if entries.len() < 2 {
        return None;
    }
    Some(
        dropdown::icon_menu(Icon::Tune, entries)
            .size(button::Size::Small)
            .tip(crate::fl!("action-quality"))
            .open(window.menu == Some(Menu::Quality), |open| {
                Control::Menu(Menu::Quality, open)
            })
            .into(),
    )
}

/// The audio and subtitle tracks to choose from, when there is a choice.
fn tracks_menu(window: &PlayerWindow) -> Option<Element<'_, Control>> {
    let info = window.info.as_ref()?;
    let subtitles: Vec<_> = info.subtitles.iter().collect();
    let audio = info.audio_with(window.status.video);
    if audio.len() < 2 && subtitles.is_empty() {
        return None;
    }
    let mut entries = Vec::new();
    if audio.len() > 1 {
        entries.push(Entry::heading(crate::fl!("tracks-audio")));
        for (number, track) in audio.iter().enumerate() {
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
            // A file beside the video is known by its name.
            let file = track
                .file
                .as_ref()
                .and_then(|file| file.file_name())
                .map(|name| name.to_string_lossy().into_owned());
            entries.push(
                Entry::item(
                    format::track_name(&track.track, number + 1, file.as_deref()),
                    Control::Subtitles(Some(track.track.index)),
                )
                .checked(window.status.subtitles == Some(track.track.index)),
            );
        }
    }
    Some(
        dropdown::icon_menu(Icon::Subtitles, entries)
            .size(button::Size::Small)
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
/// album, the visualizer chosen under them, and the bars floating over
/// them or docked above and below.
fn audio_view<'a>(window: &'a PlayerWindow, context: &Context) -> Element<'a, Control> {
    let info = window.info.as_deref();
    let visualizer = visualizer(window, context);
    // With a visualizer under it, the cover gives way in a short window.
    let art_height = if visualizer.is_some() {
        Length::Fill
    } else {
        Length::Shrink
    };
    let (art, art_most): (Element<'a, Control>, f32) = match &window.cover {
        Some(cover) => (
            container(image(cover.clone()).content_fit(iced::ContentFit::Contain))
                .max_width(COVER_MOST)
                .max_height(COVER_MOST)
                .height(art_height)
                .align_y(Center)
                .into(),
            COVER_MOST,
        ),
        None => (
            container(icon::filled(Icon::MusicNote, 96).style(style::primary_text))
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
            192.0,
        ),
    };
    // The lines' height, about, by their type's.
    let mut lines_height = 32.0;
    let mut lines = column![scramble_ui::styled(window.title(), Type::HeadlineSmall).center()]
        .spacing(4)
        .align_x(Center);
    if let Some(artist) = info.and_then(|info| info.artist.as_deref()) {
        lines = lines
            .push(scramble_ui::styled(artist, Type::TitleMedium).style(style::on_surface_variant));
        lines_height += 28.0;
    }
    if let Some(album) = info.and_then(|info| info.album.as_deref()) {
        lines = lines
            .push(scramble_ui::styled(album, Type::BodyMedium).style(style::on_surface_variant));
        lines_height += 24.0;
    }
    let middle = column![art, lines]
        .spacing(SPACING)
        .align_x(Center)
        .padding(SPACING);
    let middle: Element<'a, Control> = match visualizer {
        // Filling the height, the column lays the cover out after the
        // rest, in the room they leave; and it is no taller than the
        // cover at its largest needs, so it stays in the middle.
        Some(visualizer) => {
            let most = art_most + lines_height + VISUALIZER_HEIGHT + SPACING * 4.0;
            container(
                middle
                    .push(container(visualizer).max_width(VISUALIZER_WIDTH))
                    .height(Fill),
            )
            .max_height(most)
            .into()
        }
        None => middle.into(),
    };
    let body = container(middle).center(Fill).style(style::surface);
    let body = with_spinner(body.into(), window);
    if !component::floating_bars_enabled() {
        let body: Element<'a, Control> = match sheet(window, context) {
            Some(sheet) => iced::widget::row![body, sheet].into(),
            None => body,
        };
        return column![
            docked(toolbar(window, context)),
            body,
            docked(transport(window, context))
        ]
        .into();
    }
    // Floating, the bars come and go as over a picture.
    let body = crate::watch::watch(body, window.activity.clone(), Control::Activity);
    let body: Element<'a, Control> = match sheet(window, context) {
        Some(sheet) => {
            let sheet = component::between_bars(
                sheet,
                component::floating_room(true),
                component::floating_room(true),
            );
            iced::widget::row![body, sheet].into()
        }
        None => body,
    };
    component::window_bars(
        component::toolbar(toolbar(window, context)),
        Some(component::toolbar(transport(window, context))),
        body,
        window.overlay_shown,
    )
}

/// A sound file with the prism filling the window, the bars floating over
/// it as over a video, and its cover, title, artist and album over the
/// scene's bottom corner, above the controls while they show.
fn prism_view<'a>(window: &'a PlayerWindow, context: &Context) -> Element<'a, Control> {
    let scene = mouse_area(crate::prism::view(
        &window.player,
        window.surface,
        window.activity.clone(),
        !window.overlay_shown,
        Control::Activity,
    ))
    .on_double_click(Control::ToggleFullScreen);
    let info = window.info.as_deref();
    let mut lines =
        column![scramble_ui::styled(window.title(), Type::TitleLarge).color(iced::Color::WHITE)]
            .spacing(2);
    let detail: Vec<&str> = [
        info.and_then(|info| info.artist.as_deref()),
        info.and_then(|info| info.album.as_deref()),
    ]
    .into_iter()
    .flatten()
    .collect();
    if !detail.is_empty() {
        lines = lines.push(
            scramble_ui::styled(detail.join(" · "), Type::BodyMedium)
                .color(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.8)),
        );
    }
    let mut tag: Vec<Element<'a, Control>> = Vec::new();
    if let Some(cover) = &window.cover {
        tag.push(
            image(cover.clone())
                .width(TAG_COVER)
                .height(TAG_COVER)
                .content_fit(iced::ContentFit::Cover)
                .border_radius(10)
                .into(),
        );
    }
    tag.push(lines.into());
    let tag = container(row(tag).spacing(12).align_y(Center))
        .padding(10)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Color::from_rgba(0.0, 0.0, 0.0, 0.35).into()),
            border: iced::border::rounded(16),
            ..container::Style::default()
        });
    let lifted = if window.overlay_shown {
        component::floating_room(true) + 8.0
    } else {
        24.0
    };
    let tag = container(tag)
        .padding(iced::Padding {
            top: 0.0,
            right: 24.0,
            bottom: lifted,
            left: 24.0,
        })
        .width(Fill)
        .height(Fill)
        .align_bottom(Fill);
    let body = with_spinner(stack![scene, tag].into(), window);
    let body: Element<'a, Control> = match sheet(window, context) {
        Some(sheet) => {
            let sheet = component::between_bars(
                sheet,
                component::floating_room(true),
                component::floating_room(true),
            );
            iced::widget::row![body, sheet].into()
        }
        None => body,
    };
    component::window_bars(
        component::toolbar(toolbar(window, context)),
        Some(component::toolbar(transport(window, context))),
        body,
        window.overlay_shown,
    )
}

/// The cover's size beside the title over the prism.
const TAG_COVER: f32 = 56.0;

/// The largest a sound's cover shows, and the room around it.
const COVER_MOST: f32 = 480.0;
const SPACING: f32 = 24.0;

/// The widest and the height of the visualizer under a sound's cover.
const VISUALIZER_WIDTH: f32 = 640.0;
const VISUALIZER_HEIGHT: f32 = 88.0;

/// The spectrum or waveform under a sound's cover, as chosen; a waveform
/// only for files, which it is read from.
fn visualizer<'a>(window: &'a PlayerWindow, context: &Context) -> Option<Element<'a, Control>> {
    match window.visualizer {
        Visualizer::Spectrum => Some(crate::visualizer::spectrum(
            &window.player,
            VISUALIZER_HEIGHT,
        )),
        Visualizer::Waveform => {
            window.waveform.as_ref()?;
            let duration = window.status.duration?.as_secs_f64();
            let editor = &window.editor;
            Some(
                crate::visualizer::Waveform::new(
                    &window.player,
                    window.shape.as_deref(),
                    duration,
                    move |seconds| editor.source_of(seconds),
                    context.scale,
                    Control::Scrub,
                    Control::ScrubEnded,
                    VISUALIZER_HEIGHT,
                )
                .into(),
            )
        }
        Visualizer::Prism | Visualizer::Cover => None,
    }
}

/// `content` with a spinner over its middle while the player waits for
/// its source.
fn with_spinner<'a>(content: Element<'a, Control>, window: &PlayerWindow) -> Element<'a, Control> {
    if !window.waiting() {
        return content;
    }
    let spinner = container(crate::spinner::spinner(40.0))
        .padding(12)
        .style(|theme: &Theme| {
            let scheme = Scheme::of(theme);
            container::Style {
                background: Some(scheme.surface_container_high.scale_alpha(0.85).into()),
                border: iced::border::rounded(32),
                ..container::Style::default()
            }
        });
    stack![content, container(spinner).center(Fill)].into()
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

/// The sheet beside the picture, if one is open.
fn sheet<'a>(window: &'a PlayerWindow, context: &Context) -> Option<Element<'a, Control>> {
    match window.sheet? {
        Sheet::Inspector => Some(inspector(window)),
        Sheet::Playlist => Some(playlist_sheet(window)),
        Sheet::Network => Some(crate::network::sheet(
            context.avb.clone()?,
            window.menu,
            window.listener,
        )),
    }
}

/// The list: each item, the one playing marked, played when chosen.
fn playlist_sheet(window: &PlayerWindow) -> Element<'_, Control> {
    let current = window.playlist.current();
    let rows = window
        .playlist
        .items()
        .iter()
        .enumerate()
        .map(|(index, source)| {
            let playing = index == current;
            let name = window
                .playlist
                .title(index)
                .map_or_else(|| source.name(), str::to_owned);
            // The playing row's icon, in the room the others keep for it.
            let leading: Element<'_, Control> = if playing {
                icon::icon(Icon::PlayArrow, 20).into()
            } else {
                space().width(20).into()
            };
            // Long titles wrap, as items of a list often start alike, so
            // the row grows rather than keeping a list row's height.
            let label = scramble_ui::aligned(scramble_ui::styled(name, Type::LabelLarge));
            iced::widget::button(
                row(vec![leading, label])
                    .spacing(12)
                    .align_y(Center)
                    .width(Fill),
            )
            .padding([10, 16])
            .width(Fill)
            .style(move |theme, status| list_row_style(theme, status, playing))
            .on_press(Control::PlayItem(index))
            .into()
        });
    component::side_sheet_sized(
        crate::fl!("playlist-title"),
        Control::TogglePlaylist,
        iced::widget::Column::with_children(rows).spacing(2),
        Length::Fixed(component::SIDE_SHEET_WIDTH),
    )
}

/// A list row as scramble-ui draws one, for rows that grow with their
/// content: the chosen row in a tonal container, and a state layer while
/// pointed at or pressed.
pub(crate) fn list_row_style(
    theme: &Theme,
    status: iced::widget::button::Status,
    chosen: bool,
) -> iced::widget::button::Style {
    use iced::widget::button::Status;
    let scheme = Scheme::of(theme);
    let (background, content) = if chosen {
        (scheme.secondary_container, scheme.on_secondary_container)
    } else {
        (iced::Color::TRANSPARENT, scheme.on_surface_variant)
    };
    let layer = match status {
        Status::Hovered => scramble_ui::state_layer::HOVERED,
        Status::Pressed => scramble_ui::state_layer::PRESSED,
        Status::Active | Status::Disabled => 0.0,
    };
    // The content's color laid over the background at the layer's share.
    let over = |base: f32, top: f32| base * (1.0 - layer) + top * layer;
    let shown = if background.a == 0.0 {
        content.scale_alpha(layer)
    } else {
        iced::Color::from_rgb(
            over(background.r, content.r),
            over(background.g, content.g),
            over(background.b, content.b),
        )
    };
    iced::widget::button::Style {
        background: Some(shown.into()),
        text_color: content,
        border: iced::border::rounded(20),
        ..iced::widget::button::Style::default()
    }
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
    content = content.push(match window.source() {
        Source::Url(url) => field(crate::fl!("info-address"), url.clone()),
        Source::File(_) => field(crate::fl!("info-name"), window.source().name()),
    });
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
        if let Some(track) = info.picture(window.status.video) {
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
        for track in info.audio_with(window.status.video) {
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

/// A filmstrip's picture, turned and flipped as `transform` says.
fn still(seconds: f64, picture: &Picture, transform: Transform) -> Still {
    let shown = picture.clone().transformed(transform);
    Still {
        seconds,
        aspect: shown.width as f32 / shown.height.max(1) as f32,
        handle: image::Handle::from_rgba(shown.width, shown.height, shown.rgba),
    }
}

fn codec_label(track: &plaa_media::info::Track) -> String {
    if track.codec_name.is_empty() {
        track.codec.clone()
    } else {
        track.codec_name.clone()
    }
}
