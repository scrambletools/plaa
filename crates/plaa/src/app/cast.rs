//! Casting a window's file or stream to a renderer on the network, a TV or
//! speaker that plays DLNA: the renderers are looked for as the Cast menu
//! opens; while a window casts, its own player waits, paused, and its
//! controls and timeline are the renderer's.

use std::sync::Arc;
use std::time::Duration;

use iced::{Task, window};
use plaa_cast::{Event, Media, Session, State};
use plaa_media::Source;

use super::{Message, Plaa};
use crate::External;
use crate::window::{Casting, Control};

/// How long a search for renderers waits for their answers.
const LOOK: Duration = Duration::from_secs(3);

impl Plaa {
    /// Looks for renderers, one search at a time, as the Cast menu opens;
    /// the menu shows those found last meanwhile.
    pub(super) fn look_for_renderers(&mut self) {
        if self.cast_looking {
            return;
        }
        self.cast_looking = true;
        let spawned = std::thread::Builder::new()
            .name("plaa-cast-look".into())
            .spawn(|| {
                let found = plaa_cast::discover(LOOK).unwrap_or_else(|error| {
                    eprintln!("plaa: looking for renderers: {error}");
                    Vec::new()
                });
                crate::post(External::CastFound(found));
            });
        if spawned.is_err() {
            self.cast_looking = false;
        }
    }

    pub(super) fn renderers_found(&mut self, found: Vec<plaa_cast::Renderer>) {
        self.cast_looking = false;
        self.renderers = Some(Arc::new(found));
    }

    /// Casts window `id`'s file or stream to the renderer with id
    /// `renderer`, from where it is, its own player paused meanwhile.
    pub(super) fn cast_to(&mut self, id: window::Id, renderer: &str) -> Task<Message> {
        let Some(renderer) = self
            .renderers
            .as_deref()
            .and_then(|found| found.iter().find(|found| found.id == renderer))
            .cloned()
        else {
            return Task::none();
        };
        let volume = if self.settings.muted {
            0
        } else {
            (self.settings.volume * 100.0).round() as u8
        };
        self.next_cast += 1;
        let generation = self.next_cast;
        let Some(player) = self.player_mut(id) else {
            return Task::none();
        };
        player.menu = None;
        // A web page plays from addresses yt-dlp found for this computer.
        if player.page.is_some() {
            self.notify(id, crate::fl!("cast-web-page"));
            return Task::none();
        }
        let media = match player.source() {
            Source::File(path) => Media::File(path.clone()),
            Source::Url(url) => Media::Url(url.clone()),
            // A stream plaa listens to, whose window offers no casting.
            Source::Live { .. } => return Task::none(),
        };
        let start = player.status.position.as_secs_f64();
        player.player.pause();
        // The cast before, if any, stops first.
        player.casting = None;
        let session = Session::start(
            renderer.clone(),
            media,
            player.title(),
            start,
            Some(volume),
            move |event| crate::post(External::Cast(id, generation, event)),
        );
        player.casting = Some(Casting {
            id: renderer.id.clone(),
            name: renderer.name.clone(),
            session,
            status: None,
            generation,
        });
        player.refresh();
        self.notify(
            id,
            crate::fl!("notice-casting", renderer = renderer.name.clone()),
        );
        Task::none()
    }

    /// Stops casting from window `id`: the renderer stops, and the window
    /// goes on from where it was, playing if it played.
    pub(super) fn stop_casting(&mut self, id: window::Id) -> Task<Message> {
        let Some(player) = self.player_mut(id) else {
            return Task::none();
        };
        player.menu = None;
        let Some(casting) = player.casting.take() else {
            return Task::none();
        };
        if let Some(position) = casting.status.and_then(|status| status.position) {
            player
                .player
                .seek(Duration::from_secs_f64(position.max(0.0)), true);
        }
        if casting.playing() {
            player.player.play();
        }
        player.refresh();
        Task::none()
    }

    /// What window `id`'s cast `generation` tells.
    pub(super) fn cast_event(
        &mut self,
        id: window::Id,
        generation: u64,
        event: Event,
    ) -> Task<Message> {
        let Some(player) = self.player_mut(id) else {
            return Task::none();
        };
        let Some(casting) = player
            .casting
            .as_mut()
            .filter(|casting| casting.generation == generation)
        else {
            return Task::none();
        };
        match event {
            Event::Started => {}
            Event::Status(status) => casting.status = Some(status),
            Event::Ended => {
                player.casting = None;
                player.player.seek(Duration::ZERO, true);
            }
            Event::Failed(why) => {
                let name = casting.name.clone();
                player.casting = None;
                player.refresh();
                self.notify(
                    id,
                    crate::fl!("notice-cast-failed", renderer = name, reason = why),
                );
                return Task::none();
            }
        }
        player.refresh();
        Task::none()
    }

    /// Carries `control` out on the renderer window `id` casts to, if it
    /// casts: whether that is all it takes, the window's own player left
    /// alone.
    pub(super) fn cast_control(&mut self, id: window::Id, control: &Control) -> bool {
        let Some(player) = self.player_mut(id) else {
            return false;
        };
        let position = player.status.position.as_secs_f64();
        let Some(casting) = player.casting.as_mut() else {
            return false;
        };
        let seek = |casting: &mut Casting, seconds: f64| {
            let seconds = seconds.max(0.0);
            casting.session.seek(seconds);
            if let Some(status) = &mut casting.status {
                status.position = Some(seconds);
            }
        };
        match control {
            Control::TogglePlay => {
                let playing = casting.playing();
                if playing {
                    casting.session.pause();
                } else {
                    casting.session.play();
                }
                if let Some(status) = &mut casting.status {
                    status.state = if playing {
                        State::Paused
                    } else {
                        State::Playing
                    };
                }
            }
            Control::SeekTo(seconds) => seek(casting, *seconds),
            Control::SkipBy(seconds) => seek(casting, position + seconds),
            Control::Scrub(seconds) => player.dragging = Some(*seconds),
            Control::ScrubEnded => {
                if let Some(seconds) = player.dragging.take() {
                    seek(casting, seconds);
                }
            }
            // The renderer's volume follows plaa's, which goes on too.
            Control::Volume(volume) => {
                casting
                    .session
                    .set_volume((volume * 100.0).round().clamp(0.0, 100.0) as u8);
                return false;
            }
            Control::ToggleMute => {
                let volume = if self.settings.muted {
                    (self.settings.volume * 100.0).round() as u8
                } else {
                    0
                };
                if let Some(casting) = self
                    .player_mut(id)
                    .and_then(|player| player.casting.as_ref())
                {
                    casting.session.set_volume(volume);
                }
                return false;
            }
            _ => return false,
        }
        if let Some(player) = self.player_mut(id) {
            player.refresh();
        }
        true
    }
}
