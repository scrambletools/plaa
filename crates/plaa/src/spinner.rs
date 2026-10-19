//! An M3 circular indeterminate progress indicator: an arc that grows
//! and shrinks as it turns, drawn as a chain of round dots close enough to
//! read as a stroke.

use std::f32::consts::TAU;
use std::time::{Duration, Instant};

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Renderer as _};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell};
use iced::{Element, Event, Length, Rectangle, Size, Theme, mouse, window};
use scramble_ui::Scheme;

/// One turn of the whole indicator, and one grow and shrink of its arc.
const TURN: f32 = 1.568;
const CYCLE: f32 = 1.333;
/// The arc's shortest and longest, as shares of a turn.
const SHORTEST: f32 = 10.0 / 360.0;
const LONGEST: f32 = 270.0 / 360.0;
/// How often it moves on.
const FRAME: Duration = Duration::from_millis(16);

pub struct Spinner {
    size: f32,
    stroke: f32,
}

/// A spinner `size` across.
pub fn spinner(size: f32) -> Spinner {
    Spinner {
        size,
        stroke: (size / 12.0).max(2.0),
    }
}

struct State {
    since: Instant,
}

/// M3's standard easing, near enough: slow out and slow in.
fn ease(share: f32) -> f32 {
    let share = share.clamp(0.0, 1.0);
    share * share * (3.0 - 2.0 * share)
}

/// Where the arc starts and ends after `elapsed` seconds, in turns.
fn arc(elapsed: f32) -> (f32, f32) {
    let cycles = elapsed / CYCLE;
    let phase = cycles.fract();
    // Each cycle leaves the arc a little further round.
    let advance = cycles.floor() * (LONGEST - SHORTEST);
    let head = ease(phase * 2.0) * (LONGEST - SHORTEST) + SHORTEST;
    let tail = ease(phase * 2.0 - 1.0) * (LONGEST - SHORTEST);
    let turned = elapsed / TURN + advance;
    (turned + tail, turned + head)
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Spinner {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            since: Instant::now(),
        })
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.size), Length::Fixed(self.size))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.size, self.size)
    }

    fn update(
        &mut self,
        _tree: &mut Tree,
        event: &Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            shell.request_redraw_at(*now + FRAME);
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
        let center = bounds.center();
        let radius = (bounds.width.min(bounds.height) - self.stroke) / 2.0;
        let (start, end) = arc(state.since.elapsed().as_secs_f32());
        // Dots a third of the stroke apart overlap into a smooth line.
        let length = (end - start) * TAU * radius;
        let dots = (length / (self.stroke / 3.0)).ceil().max(1.0) as usize;
        let color = Scheme::of(theme).primary;
        for dot in 0..=dots {
            let turn = start + (end - start) * dot as f32 / dots as f32;
            // A quarter turn back, so it starts at the top.
            let angle = (turn - 0.25) * TAU;
            let (x, y) = (
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            );
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: x - self.stroke / 2.0,
                        y: y - self.stroke / 2.0,
                        width: self.stroke,
                        height: self.stroke,
                    },
                    border: iced::border::rounded(self.stroke / 2.0),
                    ..renderer::Quad::default()
                },
                color,
            );
        }
    }
}

impl<'a, Message: 'a> From<Spinner> for Element<'a, Message> {
    fn from(spinner: Spinner) -> Self {
        Element::new(spinner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_arc_grows_and_shrinks_without_going_back() {
        let mut last_start = f32::MIN;
        let mut lengths = Vec::new();
        for step in 0..400 {
            let (start, end) = arc(step as f32 * 0.01);
            assert!(start >= last_start - 1e-4, "the tail goes back at {step}");
            last_start = start;
            lengths.push(end - start);
        }
        let shortest = lengths.iter().copied().fold(f32::MAX, f32::min);
        let longest = lengths.iter().copied().fold(f32::MIN, f32::max);
        assert!((shortest - SHORTEST).abs() < 0.01, "{shortest}");
        assert!((longest - LONGEST).abs() < 0.01, "{longest}");
    }
}
