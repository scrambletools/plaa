//! Editing a window's file: trimming, splitting and arranging clips,
//! turning and flipping, and undoing; the player plays the edit as it
//! changes.

use std::time::Duration;

use iced::{Size, Task, window};
use plaa_media::Edit;

use super::{Message, Plaa, WINDOW_ROOM, fitted_window};
use crate::editing::Mode;
use crate::shortcuts::Action;
use crate::window::{Control, PlayerWindow};

/// How many pictures the editing bars' filmstrips ask for along the
/// movie.
const STRIP_STILLS: usize = 40;

impl Plaa {
    /// Does editing `action` in window `id`.
    pub(super) fn edit_action(&mut self, id: window::Id, action: Action) -> Task<Message> {
        let Some(player) = self.player_mut(id) else {
            return Task::none();
        };
        if !player.can_edit() {
            return Task::none();
        }
        let Some(duration) = player.file_duration() else {
            return Task::none();
        };
        player.menu = None;
        let position = player.player.position().as_secs_f64();
        match action {
            Action::Trim => {
                // The clip chosen trims by itself; else the whole movie.
                let clip = match player.editor.mode {
                    Some(Mode::Clips { selected }) => selected,
                    _ => None,
                };
                start_trim(player, clip);
            }
            Action::Split => {
                let mut edit = player.editor.working(duration);
                if edit.split(position) {
                    change(player, |editor| editor.commit(edit, duration), 0.0);
                }
                if player.editor.clips().is_some_and(|clips| clips.len() > 1)
                    && !matches!(player.editor.mode, Some(Mode::Clips { .. }))
                {
                    player.editor.mode = Some(Mode::Clips { selected: None });
                    request_strip(player);
                }
            }
            Action::RotateLeft | Action::RotateRight => {
                if !player.is_video() {
                    return Task::none();
                }
                let mut edit = player.editor.working(duration);
                edit.rotate(action == Action::RotateRight);
                change(player, |editor| editor.commit(edit, duration), position);
                return self.fit_turned(id);
            }
            Action::FlipHorizontal | Action::FlipVertical => {
                if !player.is_video() {
                    return Task::none();
                }
                let mut edit = player.editor.working(duration);
                edit.flip(action == Action::FlipHorizontal);
                change(player, |editor| editor.commit(edit, duration), position);
            }
            Action::Undo | Action::Redo | Action::Revert => {
                let turns = player.transform().turns;
                player.editor.mode = None;
                change(
                    player,
                    |editor| match action {
                        Action::Undo => editor.undo(),
                        Action::Redo => editor.redo(),
                        _ => editor.revert(),
                    },
                    0.0,
                );
                if player.transform().turns % 2 != turns % 2 {
                    return self.fit_turned(id);
                }
            }
            Action::Export => return self.open_export(id),
            _ => {}
        }
        Task::none()
    }

    /// Does what the editing bars ask in window `id`.
    pub(super) fn edit_control(&mut self, id: window::Id, control: Control) -> Task<Message> {
        let Some(player) = self.player_mut(id) else {
            return Task::none();
        };
        let Some(duration) = player.file_duration() else {
            return Task::none();
        };
        match control {
            Control::TrimTo(from, to, shown) => {
                if let Some(Mode::Trim { clip, .. }) = player.editor.mode {
                    player.editor.mode = Some(Mode::Trim { clip, from, to });
                    preview_trim(player, from, to);
                    player.dragging = Some(shown);
                    player.player.scrub(Duration::from_secs_f64(shown.max(0.0)));
                }
            }
            Control::ApplyTrim => {
                let Some(Mode::Trim { clip, from, to }) = player.editor.mode else {
                    return Task::none();
                };
                let mut edit = player.editor.working(duration);
                let trimmed = match clip {
                    Some(clip) => edit.trim_clip(clip, from, to),
                    None => edit.trim(from, to),
                };
                end_trim(player, clip);
                if trimmed {
                    // From the start of what is kept, unless it plays on
                    // from where it is.
                    let start = match clip {
                        Some(clip) => edit.starts().get(clip).copied().unwrap_or(0.0),
                        None => 0.0,
                    };
                    change(player, |editor| editor.commit(edit, duration), start);
                }
            }
            Control::CancelTrim => {
                if let Some(Mode::Trim { clip, .. }) = player.editor.mode {
                    end_trim(player, clip);
                }
            }
            Control::SelectClip(selected) => {
                if matches!(player.editor.mode, Some(Mode::Clips { .. })) {
                    player.editor.mode = Some(Mode::Clips { selected });
                }
            }
            Control::MoveClip(from, to) => {
                let mut edit = player.editor.working(duration);
                if edit.move_clip(from, to) {
                    change(player, |editor| editor.commit(edit, duration), 0.0);
                    player.editor.mode = Some(Mode::Clips { selected: Some(to) });
                }
            }
            Control::RemoveClip(index) => {
                let mut edit = player.editor.working(duration);
                let start = edit.starts().get(index).copied().unwrap_or(0.0);
                if edit.remove(index) {
                    change(player, |editor| editor.commit(edit, duration), start);
                    player.editor.mode = Some(Mode::Clips { selected: None });
                }
            }
            Control::TrimClip(index) => start_trim(player, Some(index)),
            Control::DoneEditing => {
                if let Some(Mode::Trim { clip, .. }) = player.editor.mode {
                    end_trim(player, clip);
                }
                player.editor.mode = None;
            }
            _ => {}
        }
        Task::none()
    }

    /// Changes window `id`'s edit as an agent asks, with `apply`, which
    /// says whether it changed it; it plays on from where it was.
    pub(super) fn tool_edit(
        &mut self,
        id: window::Id,
        apply: impl FnOnce(&mut Edit) -> bool,
    ) -> Task<Message> {
        let Some(player) = self.player_mut(id) else {
            return Task::none();
        };
        let Some(duration) = player.file_duration() else {
            return Task::none();
        };
        let turns = player.transform().turns;
        let position = player.player.position().as_secs_f64();
        let mut edit = player.editor.working(duration);
        if !apply(&mut edit) {
            return Task::none();
        }
        // A part being trimmed is no longer where it was.
        if let Some(Mode::Trim { clip, .. }) = player.editor.mode {
            end_trim(player, clip);
        }
        let fallback = position.min(edit.duration());
        change(player, |editor| editor.commit(edit, duration), fallback);
        if player.transform().turns % 2 != turns % 2 {
            return self.fit_turned(id);
        }
        Task::none()
    }

    /// Fits window `id` to its picture once it turned on its side, unless
    /// it fills the screen or is in picture in picture.
    fn fit_turned(&mut self, id: window::Id) -> Task<Message> {
        let overlay = self.settings.overlay;
        let Some(window) = self.windows.get(&id) else {
            return Task::none();
        };
        if window.fullscreen || window.pip.is_some() {
            return Task::none();
        }
        let Some(player) = window.player() else {
            return Task::none();
        };
        let Some(track) = player.picture() else {
            return Task::none();
        };
        let turns = player.transform().after_turns(track.quarter_turns).turns;
        let size: Size = fitted_window(
            track.width,
            track.height,
            track.aspect,
            turns,
            overlay,
            WINDOW_ROOM,
        );
        window::resize(id, size)
    }
}

/// Starts trimming the whole movie, or clip `clip`, all of it kept to
/// begin with.
fn start_trim(player: &mut PlayerWindow, clip: Option<usize>) {
    let duration = edited_length(player);
    let (from, to) = clip
        .and_then(|clip| {
            let edit = player.editor.edit()?;
            let start = *edit.starts().get(clip)?;
            Some((start, start + edit.clips()[clip].length()))
        })
        .unwrap_or((0.0, duration));
    if to - from <= 0.0 {
        return;
    }
    player.editor.mode = Some(Mode::Trim { clip, from, to });
    preview_trim(player, from, to);
    request_strip(player);
}

/// Plays the part kept over and over while trimming.
fn preview_trim(player: &mut PlayerWindow, from: f64, to: f64) {
    player.player.set_ab_loop(Some((
        Duration::from_secs_f64(from),
        Duration::from_secs_f64(to),
    )));
}

/// Leaves trimming: back to the clips when a clip was trimmed, and the
/// window's own loop again.
fn end_trim(player: &mut PlayerWindow, clip: Option<usize>) {
    player.editor.mode = clip.map(|clip| Mode::Clips {
        selected: Some(clip),
    });
    player.dragging = None;
    let between = match player.ab_loop {
        Some((from, Some(to))) => {
            Some((Duration::from_secs_f64(from), Duration::from_secs_f64(to)))
        }
        _ => None,
    };
    player.player.set_ab_loop(between);
}

/// Changes the edit with `apply`, then plays it from the frame that was
/// showing, when it is kept, else from `fallback` seconds, and takes the
/// chapters and filmstrip again.
fn change(
    player: &mut PlayerWindow,
    apply: impl FnOnce(&mut crate::editing::Editor) -> bool,
    fallback: f64,
) {
    let Some(duration) = player.file_duration() else {
        return;
    };
    let before = player.editor.for_player(duration);
    let source = player
        .editor
        .source_of(player.player.position().as_secs_f64());
    if !apply(&mut player.editor) {
        return;
    }
    let after = player.editor.for_player(duration);
    // The player plays the edit again when anything it hears or shows
    // changed, as sound mixed in or filters, not only the clips.
    let plays_differently = match (before.as_deref(), after.as_deref()) {
        (Some(before), Some(after)) => !before.plays_like(after),
        (None, None) => false,
        _ => true,
    };
    if plays_differently {
        let clips_changed = after.as_deref().map(Edit::clips) != before.as_deref().map(Edit::clips);
        let length = after.as_deref().map_or(duration, Edit::duration);
        let position = player
            .editor
            .edited_of(source)
            .unwrap_or(fallback)
            .clamp(0.0, length);
        player
            .player
            .set_edit(after, Duration::from_secs_f64(position));
        // A loop no longer falls where it was set once the clips move.
        if clips_changed {
            player.ab_loop = None;
            player.player.set_ab_loop(None);
        }
    }
    // Clips mode needs clips, and the clip chosen to be there.
    let clips = player.editor.clips().map_or(1, <[_]>::len);
    if let Some(Mode::Clips { selected }) = player.editor.mode {
        player.editor.mode = (clips > 1).then_some(Mode::Clips {
            selected: selected.filter(|selected| *selected < clips),
        });
    }
    player.refresh_chapters();
    player.refresh_stills();
    if player.editor.mode.is_some() {
        request_strip(player);
    }
}

/// Asks for pictures along the edited movie for the filmstrips, of each
/// file they show.
fn request_strip(player: &mut PlayerWindow) {
    if player.thumbnails.is_none() {
        return;
    }
    let length = edited_length(player);
    if length <= 0.0 {
        return;
    }
    let mut wanted: Vec<(usize, f64)> = (0..STRIP_STILLS)
        .filter_map(|index| {
            let seconds = (index as f64 + 0.5) * length / STRIP_STILLS as f64;
            let (source, time) = player.editor.picture_at(seconds)?;
            Some((player.still_file(source)?, time))
        })
        .collect();
    wanted.sort_by_key(|(file, _)| *file);
    for group in wanted.chunk_by(|first, second| first.0 == second.0) {
        if let Some(thumbnails) = player.thumbnails_of(group[0].0) {
            thumbnails.request_strip(group.iter().map(|(_, time)| *time));
        }
    }
}

/// How long the edited movie lasts, or the file when it is not edited.
fn edited_length(player: &PlayerWindow) -> f64 {
    match player.editor.edit() {
        Some(edit) => edit.duration(),
        None => player.file_duration().unwrap_or(0.0),
    }
}
