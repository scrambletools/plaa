//! The timeline: a slider whose handle is placed from the player's
//! position each time the window draws, asking to draw again when the
//! handle reaches the next pixel, so it moves smoothly at any width
//! without the window being rebuilt. Chapters mark it, and pointing at it
//! shows the time and chapter there, and a picture from there.

use std::cell::Cell;
use std::time::{Duration, Instant};

use iced::advanced::image::{self, Renderer as _};
use iced::advanced::layout::{self, Layout};
use iced::advanced::overlay;
use iced::advanced::renderer::{self, Renderer as _};
use iced::advanced::text::{self, Renderer as _};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell};
use iced::widget::slider::{HandleShape, Status, Style};
use iced::{
    Border, Element, Event, Length, Pixels, Point, Rectangle, Size, Theme, Vector, mouse, touch,
    window,
};
use plaa_media::Player;
use plaa_media::info::Chapter;
use scramble_ui::Scheme;
use scramble_ui::font::Type;

use crate::format;

/// How long a released handle stays where it was let go while the
/// player seeks there.
const HOLD: Duration = Duration::from_secs(1);

/// How often the mark of what is read ahead moves on.
const BUFFERED_REDRAW: Duration = Duration::from_millis(250);

/// The gap in the rail where a chapter starts.
const CHAPTER_GAP: f32 = 2.0;

/// The bar under the rail that marks a loop: its thickness, its distance
/// from the rail, and its length when only the start is set.
const LOOP_BAR: f32 = 3.0;
const LOOP_GAP: f32 = 4.0;
const LOOP_START: f32 = 6.0;

/// The longer side of the picture over the timeline.
pub const THUMBNAIL_SIZE: f32 = 160.0;

/// What the timeline is told of where it is pointed at.
type HoverFn<'a, Message> = Box<dyn Fn(Option<f64>) -> Message + 'a>;

/// How the rail and handle look in each status, as for a slider.
type StyleFn<'a> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

pub struct Timeline<'a, Message> {
    player: &'a Player,
    duration: f64,
    /// The window's scale factor, as the handle moves by its pixels.
    scale: f32,
    on_change: Box<dyn Fn(f64) -> Message + 'a>,
    on_release: Message,
    height: f32,
    style: StyleFn<'a>,
    chapters: &'a [Chapter],
    /// Where an edit's clips meet.
    cuts: Vec<f64>,
    ab_loop: Option<(f64, Option<f64>)>,
    on_hover: Option<HoverFn<'a, Message>>,
    /// The picture near where it is pointed at, and its shape.
    thumbnail: Option<(image::Handle, f32)>,
    /// Marks how far ahead the source has been read, as for streams.
    buffered: bool,
}

impl<'a, Message> Timeline<'a, Message> {
    /// A timeline for `player`, `duration` seconds long, sending
    /// `on_change` with the seconds the handle is dragged to and
    /// `on_release` when it is let go.
    pub fn new(
        player: &'a Player,
        duration: f64,
        scale: f32,
        on_change: impl Fn(f64) -> Message + 'a,
        on_release: Message,
    ) -> Self {
        Self {
            player,
            duration,
            scale,
            on_change: Box::new(on_change),
            on_release,
            height: 16.0,
            style: Box::new(iced::widget::slider::default),
            chapters: &[],
            cuts: Vec::new(),
            ab_loop: None,
            on_hover: None,
            thumbnail: None,
            buffered: false,
        }
    }

    /// Sends the seconds pointed at as the pointer moves over the
    /// timeline or drags it, and none once it leaves.
    pub fn on_hover(mut self, on_hover: impl Fn(Option<f64>) -> Message + 'a) -> Self {
        self.on_hover = Some(Box::new(on_hover));
        self
    }

    /// The picture to show over where it is pointed at, and its width over
    /// its height.
    pub fn thumbnail(mut self, thumbnail: Option<(image::Handle, f32)>) -> Self {
        self.thumbnail = thumbnail;
        self
    }

    /// A loop's start, and its end once set, marked under the rail.
    pub fn ab_loop(mut self, ab_loop: Option<(f64, Option<f64>)>) -> Self {
        self.ab_loop = ab_loop;
        self
    }

    /// Marks how far ahead the player has read along the rail after the
    /// handle, as is worth knowing for streams.
    pub fn buffered(mut self, buffered: bool) -> Self {
        self.buffered = buffered;
        self
    }

    pub fn chapters(mut self, chapters: &'a [Chapter]) -> Self {
        self.chapters = chapters;
        self
    }

    /// Where an edit's clips meet, in seconds, marked as chapters are.
    pub fn cuts(mut self, cuts: Vec<f64>) -> Self {
        self.cuts = cuts;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self {
        self.style = Box::new(style);
        self
    }

    /// Where `seconds` are along the rail, the handle's centre there.
    fn x_of(&self, bounds: Rectangle, handle: f32, seconds: f64) -> f32 {
        let travel = (bounds.width - handle).max(0.0);
        let along = if self.duration > 0.0 {
            (seconds / self.duration).clamp(0.0, 1.0) as f32
        } else {
            0.0
        };
        bounds.x + handle / 2.0 + travel * along
    }

    /// Where the player is, in seconds along the timeline.
    fn live(&self) -> f64 {
        self.player.position().as_secs_f64().min(self.duration)
    }

    /// The seconds under `x`, the handle's centre following the pointer.
    fn locate(&self, bounds: Rectangle, handle: f32, x: f32) -> f64 {
        let travel = (bounds.width - handle).max(1.0);
        let along = ((x - bounds.x - handle / 2.0) / travel).clamp(0.0, 1.0);
        f64::from(along) * self.duration
    }

    /// When the handle next lands on another of the screen's pixels as
    /// the player moves on: quads snap to the nearest pixel.
    fn next_move(&self, bounds: Rectangle, handle: f32) -> Option<Instant> {
        let pace = self.player.pace();
        let position = self.live();
        if pace <= 0.0 || self.duration <= 0.0 || position >= self.duration {
            return None;
        }
        let scale = f64::from(self.scale.max(0.25));
        let travel = f64::from((bounds.width - handle).max(0.0)) * scale;
        let speed = travel / self.duration * pace;
        if speed <= 0.0 {
            return None;
        }
        let edge = f64::from(bounds.x) * scale + travel * position / self.duration;
        let wait = until_next_pixel(edge, speed);
        Some(Instant::now() + Duration::from_secs_f64(wait.clamp(0.0, 3600.0)))
    }
}

/// `from` moved `share` of the way to `to`.
fn mix(from: iced::Color, to: iced::Color, share: f32) -> iced::Color {
    let toward = |from: f32, to: f32| from + (to - from) * share;
    iced::Color {
        r: toward(from.r, to.r),
        g: toward(from.g, to.g),
        b: toward(from.b, to.b),
        a: toward(from.a, to.a),
    }
}

/// Seconds until an edge at `edge` pixels, moving `speed` pixels a
/// second, is drawn a pixel further: snapping rounds, so it moves on as
/// it passes the next half pixel.
fn until_next_pixel(edge: f64, speed: f64) -> f64 {
    let next = (edge - 0.5).floor() + 1.5;
    (next - edge) / speed
}

#[derive(Default)]
struct State {
    /// The seconds the handle is held at while dragged.
    dragged: Option<f64>,
    /// Where it was let go and when, shown until the player is there.
    released: Option<(f64, Instant)>,
    hovered: bool,
    /// Where the pointer is along the timeline, while over it or dragging.
    pointer: Option<f32>,
    /// The handle's width as last drawn, which its style sets.
    handle: Cell<f32>,
}

impl State {
    fn shown(&self) -> Option<f64> {
        self.dragged.or(self.released.map(|(seconds, _)| seconds))
    }
}

impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for Timeline<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
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
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();
        let handle = state.handle.get();
        let pointed = state.pointer;
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(at) = cursor.position_over(bounds) {
                    let seconds = self.locate(bounds, handle, at.x);
                    state.dragged = Some(seconds);
                    state.released = None;
                    shell.publish((self.on_change)(seconds));
                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if state.dragged.is_some() {
                    if let Some(Point { x, .. }) = cursor.land().position() {
                        let seconds = self.locate(bounds, handle, x);
                        state.pointer = Some(x);
                        if state.dragged != Some(seconds) {
                            state.dragged = Some(seconds);
                            shell.publish((self.on_change)(seconds));
                            shell.request_redraw();
                        }
                    }
                    shell.capture_event();
                } else {
                    let over = cursor.position_over(bounds).map(|at| at.x);
                    if over != state.pointer {
                        state.pointer = over;
                        state.hovered = over.is_some();
                        shell.request_redraw();
                    }
                }
            }
            Event::Mouse(mouse::Event::CursorLeft) if state.dragged.is_none() => {
                if state.hovered || state.pointer.is_some() {
                    state.hovered = false;
                    state.pointer = None;
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                if let Some(seconds) = state.dragged.take() {
                    state.released = Some((seconds, Instant::now()));
                    state.pointer = cursor.position_over(bounds).map(|at| at.x);
                    state.hovered = state.pointer.is_some();
                    shell.publish(self.on_release.clone());
                    shell.request_redraw();
                }
            }
            Event::Window(window::Event::RedrawRequested(_)) => {
                if let Some((seconds, at)) = state.released
                    && ((self.live() - seconds).abs() < 0.05 || at.elapsed() > HOLD)
                {
                    state.released = None;
                }
                if state.released.is_some() {
                    shell.request_redraw();
                } else if state.dragged.is_none()
                    && let Some(at) = self.next_move(bounds, handle)
                {
                    shell.request_redraw_at(at);
                }
                // Reading ahead goes on while paused.
                if self.buffered
                    && self
                        .player
                        .buffered()
                        .is_some_and(|read| read.as_secs_f64() < self.duration)
                {
                    shell.request_redraw_at(Instant::now() + BUFFERED_REDRAW);
                }
            }
            _ => {}
        }
        if state.pointer != pointed
            && let Some(on_hover) = &self.on_hover
        {
            let seconds = state.pointer.map(|x| self.locate(bounds, handle, x));
            shell.publish(on_hover(seconds));
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
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let status = if state.dragged.is_some() {
            Status::Dragged
        } else if state.hovered {
            Status::Hovered
        } else {
            Status::Active
        };
        let style = (self.style)(theme, status);
        let (handle_width, handle_height, handle_radius) = match style.handle.shape {
            HandleShape::Circle { radius } => (radius * 2.0, radius * 2.0, radius.into()),
            HandleShape::Rectangle {
                width,
                border_radius,
            } => (f32::from(width), bounds.height, border_radius),
        };
        state.handle.set(handle_width);
        let seconds = state.shown().unwrap_or_else(|| self.live());
        let offset = if self.duration > 0.0 {
            (bounds.width - handle_width).max(0.0)
                * (seconds / self.duration).clamp(0.0, 1.0) as f32
        } else {
            0.0
        };
        let rail_y = bounds.y + bounds.height / 2.0 - style.rail.width / 2.0;
        let rail = |x: f32, width: f32| renderer::Quad {
            bounds: Rectangle {
                x,
                y: rail_y,
                width,
                height: style.rail.width,
            },
            border: style.rail.border,
            snap: true,
            ..renderer::Quad::default()
        };
        let middle = offset + handle_width / 2.0;
        renderer.fill_quad(rail(bounds.x, middle), style.rail.backgrounds.0);
        renderer.fill_quad(
            rail(bounds.x + middle, bounds.width - middle),
            style.rail.backgrounds.1,
        );
        // What is read ahead, between the rail's two colors.
        if self.buffered
            && let Some(read) = self.player.buffered()
        {
            let end = self.x_of(bounds, handle_width, read.as_secs_f64()) - bounds.x;
            if end > middle {
                let (played, ahead) = (style.rail.backgrounds.0, style.rail.backgrounds.1);
                let between = match (played, ahead) {
                    (iced::Background::Color(played), iced::Background::Color(ahead)) => {
                        iced::Background::Color(mix(ahead, played, 0.4))
                    }
                    (_, ahead) => ahead,
                };
                renderer.fill_quad(rail(bounds.x + middle, end - middle), between);
            }
        }
        if let Some((from, to)) = self.ab_loop {
            let start = self.x_of(bounds, handle_width, from);
            let end = to.map_or(start + LOOP_START, |to| self.x_of(bounds, handle_width, to));
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: start,
                        y: rail_y + style.rail.width + LOOP_GAP,
                        width: (end - start).max(LOOP_BAR),
                        height: LOOP_BAR,
                    },
                    border: iced::border::rounded(LOOP_BAR / 2.0),
                    snap: true,
                    ..renderer::Quad::default()
                },
                style.rail.backgrounds.0,
            );
        }
        // Chapters and cuts part the rail, in the color of the gap by the
        // handle.
        let starts = self
            .chapters
            .iter()
            .map(|chapter| chapter.start.as_secs_f64())
            .chain(self.cuts.iter().copied());
        for start in starts {
            if start <= 0.0 || start >= self.duration {
                continue;
            }
            let x = self.x_of(bounds, handle_width, start);
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: x - CHAPTER_GAP / 2.0,
                        y: rail_y,
                        width: CHAPTER_GAP,
                        height: style.rail.width,
                    },
                    snap: true,
                    ..renderer::Quad::default()
                },
                style.handle.border_color,
            );
        }
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: bounds.x + offset,
                    y: bounds.y + bounds.height / 2.0 - handle_height / 2.0,
                    width: handle_width,
                    height: handle_height,
                },
                border: Border {
                    radius: handle_radius,
                    width: style.handle.border_width,
                    color: style.handle.border_color,
                },
                snap: true,
                ..renderer::Quad::default()
            },
            style.handle.background,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        _renderer: &iced::Renderer,
        _viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        let state = tree.state.downcast_ref::<State>();
        let x = state.pointer?;
        let bounds = layout.bounds();
        let handle = state.handle.get();
        let seconds = self.locate(bounds, handle, x);
        let anchor = Point::new(self.x_of(bounds, handle, seconds), bounds.y) + translation;
        let below = bounds.y + bounds.height + translation.y;
        let chapter = chapter_at(self.chapters, seconds).and_then(|chapter| chapter.title.clone());
        Some(overlay::Element::new(Box::new(Preview {
            anchor,
            below,
            time: format::time(seconds, self.duration >= 3600.0),
            chapter,
            thumbnail: self.thumbnail.clone(),
        })))
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        // Windows has no grab cursors.
        if state.dragged.is_some() {
            if cfg!(windows) {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::Grabbing
            }
        } else if cursor.is_over(layout.bounds()) {
            if cfg!(windows) {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::Grab
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

/// The chapter `seconds` are in: the last to start by then.
pub fn chapter_at(chapters: &[Chapter], seconds: f64) -> Option<&Chapter> {
    chapters
        .iter()
        .rev()
        .find(|chapter| chapter.start.as_secs_f64() <= seconds + 1e-3)
}

/// Where the next or previous chapter starts from `seconds`. Going back
/// from more than `restart` into a chapter goes to its start.
pub fn chapter_step(
    chapters: &[Chapter],
    seconds: f64,
    forward: bool,
    restart: f64,
) -> Option<f64> {
    let starts = chapters.iter().map(|chapter| chapter.start.as_secs_f64());
    if forward {
        return starts.into_iter().find(|start| *start > seconds + 0.5);
    }
    let before: Vec<f64> = starts.filter(|start| *start <= seconds + 1e-3).collect();
    match before.as_slice() {
        [] => None,
        [.., current] if seconds - current > restart => Some(*current),
        [.., previous, _] => Some(*previous),
        [only] => Some(*only),
    }
}

/// What the timeline shows over the point the pointer is at: the time
/// there, and its chapter's title.
struct Preview {
    /// The top of the timeline at that point, which the preview stands
    /// over.
    anchor: Point,
    /// The timeline's bottom, which the preview hangs from when there is
    /// no room over it, as with the bars docked at the top.
    below: f32,
    time: String,
    chapter: Option<String>,
    thumbnail: Option<(image::Handle, f32)>,
}

/// Room around a preview's text, across and down, and between it and the
/// timeline.
const PREVIEW_PADDING: (f32, f32) = (8.0, 4.0);
const PREVIEW_GAP: f32 = 8.0;
/// The widest a preview gets; longer titles are cut.
const PREVIEW_WIDTH: f32 = 320.0;

impl Preview {
    /// The picture's size: its longer side `THUMBNAIL_SIZE`.
    fn thumbnail_size(&self) -> Option<Size> {
        let (_, aspect) = self.thumbnail.as_ref()?;
        let aspect = if aspect.is_finite() && *aspect > 0.0 {
            *aspect
        } else {
            16.0 / 9.0
        };
        Some(if aspect >= 1.0 {
            Size::new(THUMBNAIL_SIZE, (THUMBNAIL_SIZE / aspect).round())
        } else {
            Size::new((THUMBNAIL_SIZE * aspect).round(), THUMBNAIL_SIZE)
        })
    }

    fn lines(&self) -> impl Iterator<Item = (&str, Type)> {
        self.chapter
            .as_deref()
            .map(|chapter| (chapter, Type::BodySmall))
            .into_iter()
            .chain([(self.time.as_str(), Type::LabelLarge)])
    }
}

impl<Message> overlay::Overlay<Message, Theme, iced::Renderer> for Preview {
    fn layout(&mut self, _renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        let (across, down) = PREVIEW_PADDING;
        let mut width: f32 = 0.0;
        let mut height = 2.0 * down;
        if let Some(picture) = self.thumbnail_size() {
            width = picture.width;
            height += picture.height + down;
        }
        for (line, style) in self.lines() {
            width = width.max(scramble_ui::font::measure(line, style));
            height += style.metrics().1;
        }
        let width = (width + 2.0 * across).min(PREVIEW_WIDTH).min(bounds.width);
        let x = (self.anchor.x - width / 2.0).clamp(0.0, (bounds.width - width).max(0.0));
        let over = self.anchor.y - PREVIEW_GAP - height;
        let y = if over >= 0.0 {
            over
        } else {
            self.below + PREVIEW_GAP
        };
        layout::Node::new(Size::new(width, height)).move_to(Point::new(x, y))
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
    ) {
        let scheme = Scheme::of(theme);
        let bounds = layout.bounds();
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: iced::border::rounded(scramble_ui::shape::EXTRA_SMALL),
                ..renderer::Quad::default()
            },
            scheme.inverse_surface,
        );
        let (across, down) = PREVIEW_PADDING;
        let mut y = bounds.y + down;
        if let (Some((handle, _)), Some(size)) = (&self.thumbnail, self.thumbnail_size()) {
            let picture = Rectangle {
                x: bounds.center_x() - size.width / 2.0,
                y,
                width: size.width,
                height: size.height,
            };
            renderer.draw_image(
                image::Image {
                    border_radius: iced::border::radius(scramble_ui::shape::EXTRA_SMALL),
                    ..image::Image::new(handle.clone())
                },
                picture,
                bounds,
            );
            y += size.height + down;
        }
        for (line, style) in self.lines() {
            let (size, line_height) = style.metrics();
            renderer.fill_text(
                text::Text {
                    content: line.to_owned(),
                    bounds: Size::new(bounds.width - 2.0 * across, line_height),
                    size: Pixels(size),
                    line_height: text::LineHeight::Absolute(Pixels(line_height)),
                    font: style.font(false),
                    align_x: text::Alignment::Center,
                    align_y: iced::alignment::Vertical::Top,
                    shaping: text::Shaping::Advanced,
                    wrapping: text::Wrapping::None,
                },
                Point::new(bounds.center_x(), y),
                scheme.inverse_on_surface,
                bounds,
            );
            y += line_height;
        }
    }
}

impl<'a, Message: Clone + 'a> From<Timeline<'a, Message>> for Element<'a, Message> {
    fn from(timeline: Timeline<'a, Message>) -> Self {
        Element::new(timeline)
    }
}

#[cfg(test)]
mod tests {
    use super::{chapter_at, chapter_step, until_next_pixel};
    use plaa_media::info::Chapter;
    use std::time::Duration;

    fn chapters() -> Vec<Chapter> {
        [0, 60, 120]
            .into_iter()
            .map(|start| Chapter {
                title: Some(format!("At {start}")),
                start: Duration::from_secs(start),
                end: Duration::from_secs(start + 60),
            })
            .collect()
    }

    #[test]
    fn chapters_are_found_and_stepped_through() {
        let chapters = chapters();
        assert_eq!(chapter_at(&chapters, 59.0).unwrap().start.as_secs(), 0);
        assert_eq!(chapter_at(&chapters, 60.0).unwrap().start.as_secs(), 60);
        assert_eq!(chapter_step(&chapters, 10.0, true, 3.0), Some(60.0));
        assert_eq!(chapter_step(&chapters, 130.0, true, 3.0), None);
        // Well into a chapter, back goes to its start; just after, to the
        // one before.
        assert_eq!(chapter_step(&chapters, 90.0, false, 3.0), Some(60.0));
        assert_eq!(chapter_step(&chapters, 61.0, false, 3.0), Some(0.0));
        assert_eq!(chapter_step(&chapters, 1.0, false, 3.0), Some(0.0));
    }

    #[test]
    fn the_handle_redraws_as_it_reaches_each_pixel() {
        let close = |seconds: f64, expected: f64| (seconds - expected).abs() < 1e-9;
        // Drawn at pixel 10 until the edge passes 10.5.
        assert!(close(until_next_pixel(10.2, 10.0), 0.03));
        // Just drawn at 11, so a whole pixel to go.
        assert!(close(until_next_pixel(10.5, 10.0), 0.1));
        assert!(close(until_next_pixel(10.9, 40.0), 0.015));
    }
}
