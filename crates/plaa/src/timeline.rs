//! The timeline: a slider whose handle is placed from the player's
//! position each time the window draws, asking to draw again when the
//! handle reaches the next pixel, so it moves smoothly at any width
//! without the window being rebuilt.

use std::cell::Cell;
use std::time::{Duration, Instant};

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Renderer as _};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell};
use iced::widget::slider::{HandleShape, Status, Style};
use iced::{Border, Element, Event, Length, Point, Rectangle, Size, Theme, mouse, touch, window};
use plaa_media::Player;

/// How long a released handle stays where it was let go while the
/// player seeks there.
const HOLD: Duration = Duration::from_secs(1);

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
        }
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self {
        self.style = Box::new(style);
        self
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
                        if state.dragged != Some(seconds) {
                            state.dragged = Some(seconds);
                            shell.publish((self.on_change)(seconds));
                            shell.request_redraw();
                        }
                    }
                    shell.capture_event();
                } else if cursor.is_over(bounds) != state.hovered {
                    state.hovered = !state.hovered;
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorLeft) if state.hovered => {
                state.hovered = false;
                shell.request_redraw();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                if let Some(seconds) = state.dragged.take() {
                    state.released = Some((seconds, Instant::now()));
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

impl<'a, Message: Clone + 'a> From<Timeline<'a, Message>> for Element<'a, Message> {
    fn from(timeline: Timeline<'a, Message>) -> Self {
        Element::new(timeline)
    }
}

#[cfg(test)]
mod tests {
    use super::until_next_pixel;

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
