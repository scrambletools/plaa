//! The bars that edit a movie in place of the playback controls: the
//! trim bar, a filmstrip with a handle at each end of the part kept, and
//! the clip strip, the clips side by side to choose, move and trim. Both
//! place the playhead from the player each time they draw.

use std::time::{Duration, Instant};

use iced::advanced::image::{self, Renderer as _};
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Renderer as _};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell};
use iced::{Color, Element, Event, Length, Point, Rectangle, Size, Theme, mouse, touch, window};
use plaa_media::Player;
use plaa_media::edit::SHORTEST;
use scramble_ui::Scheme;

use crate::editing::Editor;

/// A picture of the movie for a filmstrip: the file's time it was asked
/// for, and the picture as it shows, turned and flipped as edited.
#[derive(Debug, Clone)]
pub struct Still {
    pub seconds: f64,
    pub handle: image::Handle,
    /// Its width over its height.
    pub aspect: f32,
}

/// The trim handles' width, and the frame's thickness between them.
const GRIP: f32 = 14.0;
const FRAME: f32 = 3.0;
/// How near a handle a press takes it.
const GRIP_REACH: f32 = 6.0;
/// The gap between clips, and the narrowest a clip is drawn.
const CLIP_GAP: f32 = 8.0;
const NARROWEST_CLIP: f32 = 16.0;
/// How far a clip is dragged before it moves rather than is chosen.
const DRAG_START: f32 = 6.0;
/// Two presses on a clip within this make a double click.
const DOUBLE_CLICK: Duration = Duration::from_millis(400);
/// How often the playhead moves on while sound alone plays; pictures
/// redraw the window as they show.
const PLAYHEAD_REDRAW: Duration = Duration::from_millis(33);

/// The still nearest the file's time `seconds`, of stills in time order.
fn nearest(stills: &[Still], seconds: f64) -> Option<&Still> {
    let after = stills.partition_point(|still| still.seconds < seconds);
    let before = after.checked_sub(1).and_then(|index| stills.get(index));
    match (before, stills.get(after)) {
        (Some(before), Some(after)) => {
            Some(if seconds - before.seconds <= after.seconds - seconds {
                before
            } else {
                after
            })
        }
        (before, after) => before.or(after),
    }
}

/// Fills `bounds` with pictures of the edited time from `from` to `to`
/// seconds, each tile the still nearest its middle, or with `empty` while
/// there are none.
fn filmstrip(
    renderer: &mut iced::Renderer,
    bounds: Rectangle,
    (from, to): (f64, f64),
    stills: &[Still],
    editor: &Editor,
    empty: Color,
) {
    renderer.fill_quad(
        renderer::Quad {
            bounds,
            snap: true,
            ..renderer::Quad::default()
        },
        empty,
    );
    let Some(first) = stills.first() else {
        return;
    };
    if to <= from || bounds.width < 1.0 || bounds.height < 1.0 {
        return;
    }
    let aspect = if first.aspect.is_finite() {
        first.aspect.clamp(0.25, 4.0)
    } else {
        16.0 / 9.0
    };
    let tile = (bounds.height * aspect).max(8.0);
    let count = (bounds.width / tile).ceil() as usize;
    renderer.with_layer(bounds, |renderer| {
        for index in 0..count {
            let x = bounds.x + index as f32 * tile;
            let share = ((index as f32 + 0.5) * tile / bounds.width).min(1.0);
            let middle = from + (to - from) * f64::from(share);
            let Some(still) = nearest(stills, editor.source_of(middle)) else {
                continue;
            };
            renderer.draw_image(
                image::Image::new(still.handle.clone()),
                Rectangle {
                    x,
                    y: bounds.y,
                    width: tile,
                    height: bounds.height,
                },
                bounds,
            );
        }
    });
}

fn quad(bounds: Rectangle, radius: impl Into<iced::border::Radius>) -> renderer::Quad {
    renderer::Quad {
        bounds,
        border: iced::Border {
            radius: radius.into(),
            ..iced::Border::default()
        },
        snap: true,
        ..renderer::Quad::default()
    }
}

/// The playhead: a light line with a dark edge, so it shows on any
/// picture.
fn playhead(renderer: &mut iced::Renderer, x: f32, bounds: Rectangle) {
    renderer.fill_quad(
        quad(
            Rectangle {
                x: x - 2.0,
                y: bounds.y,
                width: 4.0,
                height: bounds.height,
            },
            2.0,
        ),
        Color::from_rgba(0.0, 0.0, 0.0, 0.5),
    );
    renderer.fill_quad(
        quad(
            Rectangle {
                x: x - 1.0,
                y: bounds.y + 1.0,
                width: 2.0,
                height: bounds.height - 2.0,
            },
            1.0,
        ),
        Color::WHITE,
    );
}

/// Asks to draw again soon while playing, for the playhead.
fn follow_playhead<Message>(player: &Player, shell: &mut Shell<'_, Message>) {
    if player.pace() > 0.0 {
        shell.request_redraw_at(Instant::now() + PLAYHEAD_REDRAW);
    }
}

/// What a press on the trim bar took hold of.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Grip {
    From,
    To,
    Playhead,
}

#[derive(Default)]
struct TrimState {
    held: Option<Grip>,
    /// Where on the handle it was taken, so it does not jump to the
    /// pointer.
    offset: f32,
}

/// The trim bar: the edited time from `start` to `end` seconds as a
/// filmstrip, the part from `from` to `to` kept between two handles.
pub struct TrimBar<'a, Message> {
    player: &'a Player,
    editor: &'a Editor,
    stills: &'a [Still],
    span: (f64, f64),
    kept: (f64, f64),
    on_trim: Box<dyn Fn(f64, f64, f64) -> Message + 'a>,
    on_scrub: Box<dyn Fn(f64) -> Message + 'a>,
    on_release: Message,
    height: f32,
}

impl<'a, Message> TrimBar<'a, Message> {
    /// A trim bar over `span` of the edited time keeping `kept` of it. A
    /// handle dragged sends `on_trim` with the part kept and the time to
    /// show; the playhead dragged within it, `on_scrub`; letting go,
    /// `on_release`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        player: &'a Player,
        editor: &'a Editor,
        stills: &'a [Still],
        span: (f64, f64),
        kept: (f64, f64),
        on_trim: impl Fn(f64, f64, f64) -> Message + 'a,
        on_scrub: impl Fn(f64) -> Message + 'a,
        on_release: Message,
    ) -> Self {
        Self {
            player,
            editor,
            stills,
            span,
            kept,
            on_trim: Box::new(on_trim),
            on_scrub: Box::new(on_scrub),
            on_release,
            height: 44.0,
        }
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// The filmstrip's place: the bar less room for a handle each side.
    fn track(&self, bounds: Rectangle) -> Rectangle {
        Rectangle {
            x: bounds.x + GRIP,
            y: bounds.y + FRAME,
            width: (bounds.width - 2.0 * GRIP).max(1.0),
            height: (bounds.height - 2.0 * FRAME).max(1.0),
        }
    }

    fn x_of(&self, track: Rectangle, seconds: f64) -> f32 {
        let (start, end) = self.span;
        let share = if end > start {
            ((seconds - start) / (end - start)).clamp(0.0, 1.0)
        } else {
            0.0
        };
        track.x + track.width * share as f32
    }

    fn seconds_at(&self, track: Rectangle, x: f32) -> f64 {
        let (start, end) = self.span;
        let share = ((x - track.x) / track.width).clamp(0.0, 1.0);
        start + (end - start) * f64::from(share)
    }

    fn grip_at(&self, track: Rectangle, x: f32) -> Grip {
        let (from, to) = self.kept;
        let (from_x, to_x) = (self.x_of(track, from), self.x_of(track, to));
        if x <= from_x + GRIP_REACH && x >= from_x - GRIP - GRIP_REACH {
            Grip::From
        } else if x >= to_x - GRIP_REACH && x <= to_x + GRIP + GRIP_REACH {
            Grip::To
        } else {
            Grip::Playhead
        }
    }
}

impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for TrimBar<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<TrimState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(TrimState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fixed(self.height))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fill, self.height)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<TrimState>();
        let bounds = layout.bounds();
        let track = self.track(bounds);
        let (from, to) = self.kept;
        let (start, end) = self.span;
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let Some(at) = cursor.position_over(bounds) else {
                    return;
                };
                let grip = self.grip_at(track, at.x);
                state.held = Some(grip);
                state.offset = match grip {
                    Grip::From => at.x - self.x_of(track, from),
                    Grip::To => at.x - self.x_of(track, to),
                    Grip::Playhead => 0.0,
                };
                if grip == Grip::Playhead {
                    let seconds = self.seconds_at(track, at.x).clamp(from, to);
                    shell.publish((self.on_scrub)(seconds));
                }
                shell.capture_event();
                shell.request_redraw();
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                let (Some(grip), Some(Point { x, .. })) = (state.held, cursor.land().position())
                else {
                    return;
                };
                let seconds = self.seconds_at(track, x - state.offset);
                match grip {
                    Grip::From => {
                        let moved = seconds.clamp(start, (to - SHORTEST).max(start));
                        shell.publish((self.on_trim)(moved, to, moved));
                    }
                    Grip::To => {
                        let moved = seconds.clamp((from + SHORTEST).min(end), end);
                        shell.publish((self.on_trim)(from, moved, moved));
                    }
                    Grip::Playhead => shell.publish((self.on_scrub)(seconds.clamp(from, to))),
                }
                shell.capture_event();
                shell.request_redraw();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                if state.held.take().is_some() {
                    shell.publish(self.on_release.clone());
                    shell.request_redraw();
                }
            }
            Event::Window(window::Event::RedrawRequested(_)) => {
                follow_playhead(self.player, shell);
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<TrimState>();
        let scheme = Scheme::of(theme);
        let bounds = layout.bounds();
        let track = self.track(bounds);
        filmstrip(
            renderer,
            track,
            self.span,
            self.stills,
            self.editor,
            scheme.surface_container_highest,
        );
        // Over the pictures, which a layer draws over its own quads.
        renderer.with_layer(bounds, |renderer| {
            self.draw_over(state, renderer, &scheme, bounds)
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<TrimState>();
        let bounds = layout.bounds();
        let grip = match (state.held, cursor.position_over(bounds)) {
            (Some(grip), _) => grip,
            (None, Some(at)) => self.grip_at(self.track(bounds), at.x),
            (None, None) => return mouse::Interaction::default(),
        };
        match grip {
            Grip::From | Grip::To => mouse::Interaction::ResizingHorizontally,
            Grip::Playhead => mouse::Interaction::Pointer,
        }
    }
}

impl<Message> TrimBar<'_, Message> {
    /// What goes over the filmstrip: the part cut away dimmed, the frame
    /// and handles round what is kept, and the playhead.
    fn draw_over(
        &self,
        state: &TrimState,
        renderer: &mut iced::Renderer,
        scheme: &Scheme,
        bounds: Rectangle,
    ) {
        let track = self.track(bounds);
        let (from, to) = self.kept;
        let (from_x, to_x) = (self.x_of(track, from), self.x_of(track, to));
        // What is cut away is dimmed.
        let dim = Color::from_rgba(0.0, 0.0, 0.0, 0.6);
        for (left, right) in [(track.x, from_x), (to_x, track.x + track.width)] {
            if right > left {
                renderer.fill_quad(
                    quad(
                        Rectangle {
                            x: left,
                            y: track.y,
                            width: right - left,
                            height: track.height,
                        },
                        0.0,
                    ),
                    dim,
                );
            }
        }
        // The frame round what is kept, a handle at each end.
        let held = state.held;
        let color = scheme.primary;
        for y in [bounds.y, bounds.y + bounds.height - FRAME] {
            renderer.fill_quad(
                quad(
                    Rectangle {
                        x: from_x,
                        y,
                        width: (to_x - from_x).max(0.0),
                        height: FRAME,
                    },
                    0.0,
                ),
                color,
            );
        }
        let radius = 6.0;
        for (grip, x, corners) in [
            (
                Grip::From,
                from_x - GRIP,
                iced::border::Radius {
                    top_left: radius,
                    bottom_left: radius,
                    ..iced::border::Radius::default()
                },
            ),
            (
                Grip::To,
                to_x,
                iced::border::Radius {
                    top_right: radius,
                    bottom_right: radius,
                    ..iced::border::Radius::default()
                },
            ),
        ] {
            let handle = Rectangle {
                x,
                y: bounds.y,
                width: GRIP,
                height: bounds.height,
            };
            renderer.fill_quad(quad(handle, corners), color);
            // A line down the middle, longer while held.
            let length = if held == Some(grip) { 0.5 } else { 0.35 };
            renderer.fill_quad(
                quad(
                    Rectangle {
                        x: handle.center_x() - 1.0,
                        y: handle.center_y() - handle.height * length / 2.0,
                        width: 2.0,
                        height: handle.height * length,
                    },
                    1.0,
                ),
                scheme.on_primary,
            );
        }
        let position = self.player.position().as_secs_f64();
        let (start, end) = self.span;
        if position >= start && position <= end {
            playhead(renderer, self.x_of(track, position), track);
        }
    }
}

impl<'a, Message: Clone + 'a> From<TrimBar<'a, Message>> for Element<'a, Message> {
    fn from(bar: TrimBar<'a, Message>) -> Self {
        Element::new(bar)
    }
}

/// The width of each clip, `lengths` long, in `available` pixels: as
/// long as it lasts, though none narrower than `narrowest`.
pub fn clip_widths(lengths: &[f64], available: f32, narrowest: f32) -> Vec<f32> {
    let count = lengths.len();
    if count == 0 {
        return Vec::new();
    }
    let narrowest = narrowest.min(available / count as f32).max(0.0);
    let mut widths = vec![0.0; count];
    // Clips too short for the narrowest width get it; the rest share
    // what is left as they last, until none more falls short.
    let mut fixed = vec![false; count];
    loop {
        let left = available - narrowest * fixed.iter().filter(|fixed| **fixed).count() as f32;
        let total: f64 = lengths
            .iter()
            .zip(&fixed)
            .filter(|(_, fixed)| !**fixed)
            .map(|(length, _)| length.max(0.0))
            .sum();
        let mut changed = false;
        for index in 0..count {
            if fixed[index] {
                widths[index] = narrowest;
                continue;
            }
            let share = if total > 0.0 {
                (lengths[index].max(0.0) / total) as f32
            } else {
                1.0 / count as f32
            };
            widths[index] = left.max(0.0) * share;
            if widths[index] < narrowest {
                fixed[index] = true;
                changed = true;
            }
        }
        if !changed {
            return widths;
        }
    }
}

/// What a press on the clip strip is doing.
#[derive(Debug, Clone, Copy)]
enum Press {
    /// On clip `clip` at `x`, not moved far yet.
    Down { clip: usize, x: f32 },
    /// Dragging clip `clip`, its pointer at `x`, from `grabbed` pixels
    /// into it.
    Dragging { clip: usize, x: f32, grabbed: f32 },
}

#[derive(Default)]
struct ClipState {
    press: Option<Press>,
    /// The last clip pressed and when, for double clicks.
    last: Option<(usize, Instant)>,
}

/// The clips side by side, each as a filmstrip as wide as it is long:
/// pressing one chooses it and moves the playhead there, dragging it
/// moves it among the others, and a double click trims it.
pub struct ClipStrip<'a, Message> {
    player: &'a Player,
    editor: &'a Editor,
    stills: &'a [Still],
    selected: Option<usize>,
    on_select: Box<dyn Fn(Option<usize>) -> Message + 'a>,
    on_seek: Box<dyn Fn(f64) -> Message + 'a>,
    on_move: Box<dyn Fn(usize, usize) -> Message + 'a>,
    on_open: Box<dyn Fn(usize) -> Message + 'a>,
    height: f32,
}

impl<'a, Message> ClipStrip<'a, Message> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        player: &'a Player,
        editor: &'a Editor,
        stills: &'a [Still],
        selected: Option<usize>,
        on_select: impl Fn(Option<usize>) -> Message + 'a,
        on_seek: impl Fn(f64) -> Message + 'a,
        on_move: impl Fn(usize, usize) -> Message + 'a,
        on_open: impl Fn(usize) -> Message + 'a,
    ) -> Self {
        Self {
            player,
            editor,
            stills,
            selected,
            on_select: Box::new(on_select),
            on_seek: Box::new(on_seek),
            on_move: Box::new(on_move),
            on_open: Box::new(on_open),
            height: 44.0,
        }
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Each clip's place, with where it starts and ends in the edited
    /// time.
    fn places(&self, bounds: Rectangle) -> Vec<(Rectangle, f64, f64)> {
        let Some(edit) = self.editor.edit() else {
            return Vec::new();
        };
        let lengths: Vec<f64> = edit.clips().iter().map(|clip| clip.length()).collect();
        let gaps = CLIP_GAP * lengths.len().saturating_sub(1) as f32;
        let widths = clip_widths(&lengths, (bounds.width - gaps).max(0.0), NARROWEST_CLIP);
        let mut x = bounds.x;
        widths
            .iter()
            .zip(edit.starts())
            .zip(&lengths)
            .map(|((width, start), length)| {
                let place = Rectangle {
                    x,
                    y: bounds.y,
                    width: *width,
                    height: bounds.height,
                };
                x += width + CLIP_GAP;
                (place, start, start + length)
            })
            .collect()
    }

    /// The clip at `x`, if any.
    fn clip_at(places: &[(Rectangle, f64, f64)], x: f32) -> Option<usize> {
        places
            .iter()
            .position(|(place, ..)| x >= place.x && x < place.x + place.width + CLIP_GAP)
    }

    /// The edited time at `x` within clip `index`.
    fn seconds_at(places: &[(Rectangle, f64, f64)], index: usize, x: f32) -> f64 {
        let (place, start, end) = places[index];
        let share = ((x - place.x) / place.width.max(1.0)).clamp(0.0, 1.0);
        start + (end - start) * f64::from(share)
    }

    /// Where a clip dragged to `x` lands: its place among the others.
    fn drop_place(places: &[(Rectangle, f64, f64)], dragged: usize, x: f32) -> usize {
        places
            .iter()
            .enumerate()
            .filter(|(index, (place, ..))| *index != dragged && place.center_x() < x)
            .count()
    }
}

impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for ClipStrip<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ClipState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ClipState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fixed(self.height))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fill, self.height)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<ClipState>();
        let bounds = layout.bounds();
        let places = self.places(bounds);
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let Some(at) = cursor.position_over(bounds) else {
                    return;
                };
                let Some(clip) = Self::clip_at(&places, at.x) else {
                    shell.publish((self.on_select)(None));
                    shell.capture_event();
                    return;
                };
                let now = Instant::now();
                let double = state
                    .last
                    .is_some_and(|(last, when)| last == clip && now - when < DOUBLE_CLICK);
                if double {
                    state.last = None;
                    state.press = None;
                    shell.publish((self.on_open)(clip));
                } else {
                    state.last = Some((clip, now));
                    state.press = Some(Press::Down { clip, x: at.x });
                    shell.publish((self.on_select)(Some(clip)));
                }
                shell.capture_event();
                shell.request_redraw();
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                let Some(Point { x, .. }) = cursor.land().position() else {
                    return;
                };
                match state.press {
                    Some(Press::Down { clip, x: pressed }) if (x - pressed).abs() >= DRAG_START => {
                        let grabbed = pressed - places.get(clip).map_or(pressed, |place| place.0.x);
                        state.press = Some(Press::Dragging { clip, x, grabbed });
                        state.last = None;
                        shell.request_redraw();
                    }
                    Some(Press::Dragging { clip, grabbed, .. }) => {
                        state.press = Some(Press::Dragging { clip, x, grabbed });
                        shell.request_redraw();
                    }
                    _ => return,
                }
                shell.capture_event();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                match state.press.take() {
                    Some(Press::Down { clip, x }) if clip < places.len() => {
                        shell.publish((self.on_seek)(Self::seconds_at(&places, clip, x)));
                    }
                    Some(Press::Dragging { clip, x, .. }) => {
                        let to = Self::drop_place(&places, clip, x);
                        if to != clip {
                            shell.publish((self.on_move)(clip, to));
                        }
                    }
                    _ => return,
                }
                shell.request_redraw();
            }
            Event::Window(window::Event::RedrawRequested(_)) => {
                follow_playhead(self.player, shell);
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<ClipState>();
        let scheme = Scheme::of(theme);
        let bounds = layout.bounds();
        let places = self.places(bounds);
        let dragged = match state.press {
            Some(Press::Dragging { clip, x, grabbed }) => Some((clip, x, grabbed)),
            _ => None,
        };
        let inner = |place: &Rectangle| Rectangle {
            y: place.y + FRAME,
            height: (place.height - 2.0 * FRAME).max(1.0),
            ..*place
        };
        for (place, start, end) in &places {
            filmstrip(
                renderer,
                inner(place),
                (*start, *end),
                self.stills,
                self.editor,
                scheme.surface_container_highest,
            );
        }
        // Over the pictures, which a layer draws over its own quads.
        renderer.with_layer(bounds, |renderer| {
            self.draw_over(renderer, &scheme, bounds, &places, dragged);
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<ClipState>();
        if matches!(state.press, Some(Press::Dragging { .. })) {
            if cfg!(windows) {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::Grabbing
            }
        } else if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<Message> ClipStrip<'_, Message> {
    /// What goes over the filmstrips: the clip dragged dimmed, the one
    /// chosen framed, where a drag would drop it, and the playhead.
    fn draw_over(
        &self,
        renderer: &mut iced::Renderer,
        scheme: &Scheme,
        bounds: Rectangle,
        places: &[(Rectangle, f64, f64)],
        dragged: Option<(usize, f32, f32)>,
    ) {
        let inner = |place: &Rectangle| Rectangle {
            y: place.y + FRAME,
            height: (place.height - 2.0 * FRAME).max(1.0),
            ..*place
        };
        for (index, (place, ..)) in places.iter().enumerate() {
            if dragged.is_some_and(|(clip, ..)| clip == index) {
                renderer.fill_quad(
                    quad(inner(place), 0.0),
                    Color::from_rgba(0.0, 0.0, 0.0, 0.6),
                );
            }
            // Each clip framed, the one chosen boldly.
            let (width, color) = if self.selected == Some(index) {
                (FRAME, scheme.primary)
            } else {
                (1.0, scheme.outline)
            };
            renderer.fill_quad(
                renderer::Quad {
                    bounds: *place,
                    border: iced::Border {
                        radius: 6.0.into(),
                        width,
                        color,
                    },
                    snap: true,
                    ..renderer::Quad::default()
                },
                Color::TRANSPARENT,
            );
        }
        if let Some((clip, x, grabbed)) = dragged
            && let Some((place, ..)) = places.get(clip)
        {
            // Where it would land, and an outline following the pointer.
            let to = Self::drop_place(places, clip, x);
            let others: Vec<&Rectangle> = places
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != clip)
                .map(|(_, (place, ..))| place)
                .collect();
            let marker = match others.get(to) {
                Some(next) => next.x - CLIP_GAP / 2.0,
                None => others
                    .last()
                    .map_or(bounds.x, |last| last.x + last.width + CLIP_GAP / 2.0),
            };
            renderer.fill_quad(
                quad(
                    Rectangle {
                        x: marker - 2.0,
                        y: bounds.y,
                        width: 4.0,
                        height: bounds.height,
                    },
                    2.0,
                ),
                scheme.primary,
            );
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: (x - grabbed).clamp(bounds.x, bounds.x + bounds.width - place.width),
                        ..*place
                    },
                    border: iced::Border {
                        radius: 6.0.into(),
                        width: 2.0,
                        color: scheme.primary,
                    },
                    snap: true,
                    ..renderer::Quad::default()
                },
                scheme.primary.scale_alpha(0.25),
            );
        }
        let position = self.player.position().as_secs_f64();
        if let Some((index, _)) = self.editor.edit().and_then(|edit| edit.clip_at(position))
            && let Some((place, start, end)) = places.get(index)
        {
            let share = if end > start {
                ((position - start) / (end - start)).clamp(0.0, 1.0)
            } else {
                0.0
            };
            playhead(renderer, place.x + place.width * share as f32, *place);
        }
    }
}

impl<'a, Message: Clone + 'a> From<ClipStrip<'a, Message>> for Element<'a, Message> {
    fn from(strip: ClipStrip<'a, Message>) -> Self {
        Element::new(strip)
    }
}

#[cfg(test)]
mod tests {
    use super::clip_widths;

    #[test]
    fn clips_are_as_wide_as_they_last_yet_never_too_narrow() {
        let widths = clip_widths(&[10.0, 30.0], 400.0, 16.0);
        assert_eq!(widths, [100.0, 300.0]);
        // A clip far shorter than the rest still shows.
        let widths = clip_widths(&[0.1, 100.0, 100.0], 416.0, 16.0);
        assert_eq!(widths, [16.0, 200.0, 200.0]);
        let total: f32 = clip_widths(&[1.0, 2.0, 3.0, 0.01], 300.0, 16.0)
            .iter()
            .sum();
        assert!((total - 300.0).abs() < 0.01);
    }
}
