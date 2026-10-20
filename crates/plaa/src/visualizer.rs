//! What a sound file shows under its cover as it plays: the spectrum of
//! the sound heard, its bars rising with it and falling back slowly, or
//! the file's waveform from start to end, the part played in the accent
//! color, which a click or a drag seeks along as the timeline does.

use std::time::{Duration, Instant};

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Renderer as _};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell};
use iced::{Color, Element, Event, Length, Rectangle, Size, Theme, mouse, touch, window};
use plaa_media::Player;
use plaa_media::spectrum::{self, FRAMES};
use plaa_media::waveform::Shape;
use scramble_ui::Scheme;

/// A bar's width and the gap between bars, as near as the width allows.
const BAR: f32 = 8.0;
const BAR_GAP: f32 = 3.0;

/// How quickly bars rise to the sound, as the share of the way they go
/// in a second's fraction, and how far they fall back in a second.
const RISE: f32 = 30.0;
const FALL: f32 = 1.4;

/// The least a bar or line shows, so the visualizer is seen at rest.
const LEAST: f32 = 3.0;

/// A line of the waveform and the gap between lines.
const LINE: f32 = 2.0;
const LINE_GAP: f32 = 1.0;

/// How long a released waveform shows where it was let go while the
/// player seeks there.
const HOLD: Duration = Duration::from_secs(1);

/// The spectrum of what `player` plays, `height` tall.
pub fn spectrum<'a, Message: 'a>(player: &'a Player, height: f32) -> Element<'a, Message> {
    Element::new(Spectrum { player, height })
}

struct Spectrum<'a> {
    player: &'a Player,
    height: f32,
}

/// The bars as last drawn, and the transform's room.
struct Bars {
    spectrum: spectrum::Spectrum,
    levels: Vec<f32>,
    /// When the levels were last worked out, while they move.
    at: Option<Instant>,
}

/// How many bars fit across `width`.
fn bar_count(width: f32) -> usize {
    (((width + BAR_GAP) / (BAR + BAR_GAP)) as usize).clamp(8, 96)
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Spectrum<'_> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Bars>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Bars {
            spectrum: spectrum::Spectrum::new(),
            levels: Vec::new(),
            at: None,
        })
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
        _cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let Event::Window(window::Event::RedrawRequested(now)) = event else {
            return;
        };
        let bars = tree.state.downcast_mut::<Bars>();
        let count = bar_count(layout.bounds().width);
        if bars.levels.len() != count {
            bars.levels = vec![0.0; count];
        }
        let heard = self.player.heard(FRAMES);
        let fresh = match &heard {
            Some((frames, rate)) => bars.spectrum.bands(frames, *rate, count),
            None => vec![0.0; count],
        };
        let elapsed = bars
            .at
            .map_or(0.0, |at| now.saturating_duration_since(at).as_secs_f32())
            .min(0.1);
        let rise = 1.0 - (-RISE * elapsed).exp();
        for (level, fresh) in bars.levels.iter_mut().zip(fresh) {
            *level = if fresh > *level {
                *level + (fresh - *level) * rise
            } else {
                (*level - FALL * elapsed).max(fresh)
            };
        }
        // On while there is sound or the bars have yet to fall back.
        if heard.is_some() || bars.levels.iter().any(|level| *level > 0.0) {
            bars.at = Some(*now);
            shell.request_redraw();
        } else {
            bars.at = None;
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
        let bars = tree.state.downcast_ref::<Bars>();
        let bounds = layout.bounds();
        let scheme = Scheme::of(theme);
        let count = bar_count(bounds.width);
        let width = (bounds.width - BAR_GAP * (count - 1) as f32) / count as f32;
        let radius = (width / 2.0).min(3.0);
        for index in 0..count {
            let level = bars.levels.get(index).copied().unwrap_or(0.0);
            let height = (level * bounds.height).max(LEAST);
            let color = if level > 0.0 {
                scheme.primary.scale_alpha(0.45 + 0.55 * level)
            } else {
                scheme.outline_variant
            };
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x + index as f32 * (width + BAR_GAP),
                        y: bounds.y + bounds.height - height,
                        width,
                        height,
                    },
                    border: iced::border::rounded(iced::border::Radius {
                        top_left: radius,
                        top_right: radius,
                        bottom_right: 0.0,
                        bottom_left: 0.0,
                    }),
                    snap: true,
                    ..renderer::Quad::default()
                },
                color,
            );
        }
    }
}

/// The waveform of the file `player` plays, as `shape` has it so far,
/// along `duration` seconds of the edit: `source_of` gives the file's
/// time at a time of the edit. Clicking and dragging sends `on_change`
/// with the seconds there, and letting go `on_release`.
pub struct Waveform<'a, Message> {
    player: &'a Player,
    shape: Option<&'a Shape>,
    duration: f64,
    source_of: Box<dyn Fn(f64) -> f64 + 'a>,
    /// The window's scale factor, as the playhead moves by its pixels.
    scale: f32,
    on_change: fn(f64) -> Message,
    on_release: Message,
    height: f32,
}

impl<'a, Message> Waveform<'a, Message> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        player: &'a Player,
        shape: Option<&'a Shape>,
        duration: f64,
        source_of: impl Fn(f64) -> f64 + 'a,
        scale: f32,
        on_change: fn(f64) -> Message,
        on_release: Message,
        height: f32,
    ) -> Self {
        Self {
            player,
            shape,
            duration,
            source_of: Box::new(source_of),
            scale,
            on_change,
            on_release,
            height,
        }
    }

    fn live(&self) -> f64 {
        self.player.position().as_secs_f64().min(self.duration)
    }

    /// The seconds under `x`.
    fn locate(&self, bounds: Rectangle, x: f32) -> f64 {
        let along = ((x - bounds.x) / bounds.width.max(1.0)).clamp(0.0, 1.0);
        f64::from(along) * self.duration
    }

    /// When the playhead next lands on another of the screen's pixels.
    fn next_move(&self, bounds: Rectangle) -> Option<Instant> {
        let pace = self.player.pace();
        let position = self.live();
        if pace <= 0.0 || self.duration <= 0.0 || position >= self.duration {
            return None;
        }
        let scale = f64::from(self.scale.max(0.25));
        let width = f64::from(bounds.width) * scale;
        let speed = width / self.duration * pace;
        let edge = f64::from(bounds.x) * scale + width * position / self.duration;
        let wait = crate::timeline::until_next_pixel(edge, speed);
        Some(Instant::now() + Duration::from_secs_f64(wait.clamp(0.0, 3600.0)))
    }
}

#[derive(Default)]
struct Seeking {
    /// The seconds held at while dragged.
    dragged: Option<f64>,
    /// Where it was let go and when, shown until the player is there.
    released: Option<(f64, Instant)>,
}

/// How many lines fit across `width`.
fn line_count(width: f32) -> usize {
    (((width + LINE_GAP) / (LINE + LINE_GAP)) as usize).max(1)
}

/// How tall loudness shows: by its square root, so quiet sound is seen.
fn shown(loudness: f32) -> f32 {
    loudness.clamp(0.0, 1.0).sqrt()
}

impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for Waveform<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Seeking>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Seeking::default())
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
        let seeking = tree.state.downcast_mut::<Seeking>();
        let bounds = layout.bounds();
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(at) = cursor.position_over(bounds) {
                    let seconds = self.locate(bounds, at.x);
                    seeking.dragged = Some(seconds);
                    seeking.released = None;
                    shell.publish((self.on_change)(seconds));
                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if seeking.dragged.is_some() {
                    if let Some(at) = cursor.land().position() {
                        let seconds = self.locate(bounds, at.x);
                        if seeking.dragged != Some(seconds) {
                            seeking.dragged = Some(seconds);
                            shell.publish((self.on_change)(seconds));
                            shell.request_redraw();
                        }
                    }
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                if let Some(seconds) = seeking.dragged.take() {
                    seeking.released = Some((seconds, Instant::now()));
                    shell.publish(self.on_release.clone());
                    shell.request_redraw();
                }
            }
            Event::Window(window::Event::RedrawRequested(_)) => {
                if let Some((seconds, at)) = seeking.released
                    && ((self.live() - seconds).abs() < 0.05 || at.elapsed() > HOLD)
                {
                    seeking.released = None;
                }
                if seeking.released.is_some() {
                    shell.request_redraw();
                } else if seeking.dragged.is_none()
                    && let Some(at) = self.next_move(bounds)
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
        let seeking = tree.state.downcast_ref::<Seeking>();
        let bounds = layout.bounds();
        let scheme = Scheme::of(theme);
        let position = seeking
            .dragged
            .or(seeking.released.map(|(seconds, _)| seconds))
            .unwrap_or_else(|| self.live());
        let count = line_count(bounds.width);
        let width = (bounds.width - LINE_GAP * (count - 1) as f32) / count as f32;
        let middle = bounds.y + bounds.height / 2.0;
        let line = |renderer: &mut iced::Renderer, x: f32, height: f32, color: Color| {
            let height = height.max(LEAST.min(bounds.height));
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x,
                        y: middle - height / 2.0,
                        width,
                        height,
                    },
                    border: iced::border::rounded(width / 2.0),
                    snap: true,
                    ..renderer::Quad::default()
                },
                color,
            );
        };
        for index in 0..count {
            let from = self.duration * index as f64 / count as f64;
            let to = self.duration * (index + 1) as f64 / count as f64;
            let (start, end) = ((self.source_of)(from), (self.source_of)(to));
            let loudness = self
                .shape
                .and_then(|shape| shape.between(start.min(end), start.max(end)));
            let x = bounds.x + index as f32 * (width + LINE_GAP);
            let played = to <= position;
            let Some(loudness) = loudness else {
                line(renderer, x, 0.0, scheme.outline_variant);
                continue;
            };
            let (peak, average) = if played {
                (scheme.primary.scale_alpha(0.45), scheme.primary)
            } else {
                (
                    scheme.on_surface_variant.scale_alpha(0.3),
                    scheme.on_surface_variant.scale_alpha(0.7),
                )
            };
            line(renderer, x, shown(loudness.peak) * bounds.height, peak);
            line(
                renderer,
                x,
                shown(loudness.average) * bounds.height,
                average,
            );
        }
        if self.duration > 0.0 {
            let x = bounds.x + bounds.width * (position / self.duration).clamp(0.0, 1.0) as f32;
            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: x - 1.0,
                        y: bounds.y,
                        width: 2.0,
                        height: bounds.height,
                    },
                    border: iced::border::rounded(1.0),
                    snap: true,
                    ..renderer::Quad::default()
                },
                scheme.primary,
            );
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let seeking = tree.state.downcast_ref::<Seeking>();
        if seeking.dragged.is_some() || cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message: Clone + 'a> From<Waveform<'a, Message>> for Element<'a, Message> {
    fn from(waveform: Waveform<'a, Message>) -> Self {
        Element::new(waveform)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bars_and_lines_fit_their_width() {
        for width in [100.0, 392.0, 640.0, 2000.0] {
            let bars = bar_count(width);
            assert!(bars as f32 * BAR + (bars - 1) as f32 * BAR_GAP <= width.max(8.0 * 11.0));
            let lines = line_count(width);
            assert!(lines as f32 * LINE + (lines - 1) as f32 * LINE_GAP <= width);
        }
        // Quiet sound still shows, full scale fills.
        assert!(shown(0.01) >= 0.1);
        assert_eq!(shown(1.0), 1.0);
    }
}
