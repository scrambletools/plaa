//! The tools that read what a window plays, as edited, and edit it: its
//! pictures at any time, its sound and how loud it is, its subtitles'
//! text, its clips; trimming, cutting, splitting, moving clips, putting
//! in parts of other files, replacing the picture or the sound, turning
//! the picture, undoing, and exporting. Times are in seconds of the
//! edited movie, as the timeline counts them.

use std::path::{Path, PathBuf};

use iced::{Task, window};
use plaa_media::edit::Filters;
use plaa_media::filter::{self, PictureInput};
use plaa_media::read::{self, Edited};
use plaa_media::{Clip, Edit, Feed, MediaInfo, OWN, Other, Source, Track};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Value, json};

use super::{Answer, Kind, Output, Part, Progress, Tool, number, reply, round, tool};
use crate::app::{Message, Plaa};
use crate::control::{Error, code};
use crate::window::PlayerWindow;

/// The most pictures get_frames gives at once.
const MOST_PICTURES: usize = 24;

/// The most sound get_sound gives at once, in seconds.
const MOST_SOUND: f64 = 60.0;

/// The sound get_sound gives: one channel at this rate.
const SOUND_RATE: u32 = 16000;

/// The rate get_levels measures sound at.
const LEVELS_RATE: u32 = 8000;

/// The most steps get_levels gives.
const MOST_LEVELS: usize = 400;

/// The most subtitle lines get_subtitles gives.
const MOST_LINES: usize = 2000;

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Frames {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The times to show, in seconds of the edited movie.
    at: Option<Vec<f64>>,
    /// Without `at`, pictures spread evenly from here, in seconds; the
    /// start if left out.
    from: Option<f64>,
    /// Without `at`, up to here, in seconds; the end if left out.
    to: Option<f64>,
    /// Without `at`, how many pictures to spread, 1 to 24; 6 if left out.
    count: Option<usize>,
    /// Each picture's longest side, from 160 to 1280 pixels; 512 if left
    /// out.
    size: Option<u32>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Range {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// Where it starts, in seconds of the edited movie; the start if left
    /// out.
    from: Option<f64>,
    /// Where it ends, in seconds; the end if left out.
    to: Option<f64>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Levels {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// Where to start, in seconds of the edited movie; the start if left
    /// out.
    from: Option<f64>,
    /// Where to end, in seconds; the end if left out.
    to: Option<f64>,
    /// How long each step measured is, in seconds; chosen to give at most
    /// 200 if left out.
    step: Option<f64>,
    /// Below this loudness, in dBFS, the sound counts as silence; -45 if
    /// left out.
    silence: Option<f64>,
    /// The shortest silence to list, in seconds; 0.5 if left out.
    shortest: Option<f64>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Subtitles {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The subtitles, by their stream number from get_info; those
    /// showing, else the first with text, if left out.
    stream: Option<usize>,
    /// Where to start, in seconds of the edited movie; the start if left
    /// out.
    from: Option<f64>,
    /// Where to end, in seconds; the end if left out.
    to: Option<f64>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Between {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// Where the part starts, in seconds of the edited movie.
    from: f64,
    /// Where it ends, in seconds.
    to: f64,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Split {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// Where to split, in seconds of the edited movie.
    at: f64,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct MoveClip {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The clip to move, by its number from get_edit, from 1.
    clip: usize,
    /// The place to move it to, from 1 for the first.
    to: usize,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Insert {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The full path of the file to put in; the window's own file to
    /// repeat a part of it.
    file: String,
    /// Where to put it, in seconds of the edited movie; the end if left
    /// out.
    at: Option<f64>,
    /// The part of the file to put in, from this many seconds of it; its
    /// start if left out.
    from: Option<f64>,
    /// To this many seconds of it; its end if left out.
    to: Option<f64>,
}

/// A clip's picture or sound, as agents name them.
#[derive(Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum Which {
    Picture,
    Sound,
}

impl Which {
    fn track(self) -> Track {
        match self {
            Which::Picture => Track::Picture,
            Which::Sound => Track::Sound,
        }
    }
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Replace {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// What to replace: the picture or the sound.
    track: Which,
    /// Where the part replaced starts, in seconds of the edited movie.
    from: f64,
    /// Where it ends, in seconds.
    to: f64,
    /// The full path of the file to play there instead; silence or black
    /// if left out.
    file: Option<String>,
    /// Where in that file to play from, in seconds; its start if left out.
    file_from: Option<f64>,
    /// Puts the clips' own picture or sound back there instead.
    original: Option<bool>,
}

/// Which way to turn a picture.
#[derive(Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum Turn {
    Left,
    Right,
}

/// Which way to flip a picture.
#[derive(Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum Flip {
    Horizontal,
    Vertical,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct TurnPicture {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// Turns the picture a quarter turn, left or right.
    turn: Option<Turn>,
    /// Flips the picture, left to right (horizontal) or top to bottom
    /// (vertical).
    flip: Option<Flip>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Undo {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// Redoes the change last undone instead.
    redo: Option<bool>,
    /// Undoes every change, so the file plays as it is.
    all: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ExportTo {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The full path of the file to write.
    file: String,
    /// h264, hevc or av1 for a video, as an MP4; or aac, mp3, opus, flac,
    /// alac or wav for the sound alone. h264 if left out, or for sound
    /// alone, aac.
    format: Option<String>,
    /// The picture's shorter side in pixels, when smaller than the file's.
    lines: Option<u32>,
    /// The sound alone's quality: kilobits a second, or bits a sample for
    /// flac, alac and wav.
    quality: Option<u32>,
    /// Writes over a file that is there already.
    replace: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct ApplyFilter {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// FFmpeg filters for the picture, as -vf takes them, such as
    /// "hue=s=0" for black and white.
    picture: Option<String>,
    /// FFmpeg filters for the sound, as -af takes them, such as
    /// "highpass=f=80,loudnorm".
    sound: Option<String>,
    /// Where they start, in seconds of the edited movie; the start if
    /// left out.
    from: Option<f64>,
    /// Where they end, in seconds; the end if left out.
    to: Option<f64>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct MixIn {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The full path of the file whose sound to mix in.
    file: String,
    /// Where it starts, in seconds of the edited movie; the start if left
    /// out.
    at: Option<f64>,
    /// The part of the file to mix in, from this many seconds of it; its
    /// start if left out.
    from: Option<f64>,
    /// To this many seconds of it; its end if left out.
    to: Option<f64>,
    /// How loud it is, in decibels: 0 as it is, negative quieter, such as
    /// -12 for music under speech; 0 if left out.
    volume: Option<f64>,
    /// Seconds it fades in over; none if left out.
    fade_in: Option<f64>,
    /// Seconds it fades out over at its end; none if left out.
    fade_out: Option<f64>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct RemoveMix {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The sound mixed in to take out, by its number from get_edit.
    mix: usize,
}

/// A change to a window's edit waiting on what another file holds.
#[derive(Debug, Clone)]
pub(crate) enum FileChange {
    Insert {
        window: window::Id,
        at: Option<f64>,
        from: Option<f64>,
        to: Option<f64>,
    },
    Replace {
        window: window::Id,
        track: Track,
        from: f64,
        to: f64,
        file_from: f64,
    },
    Mix {
        window: window::Id,
        at: Option<f64>,
        from: Option<f64>,
        to: Option<f64>,
        volume: f64,
        fades: (f64, f64),
    },
}

/// The tools of this module, in the order agents see them.
pub(super) fn tools() -> Vec<Tool> {
    vec![
        tool(
            "get_frames",
            "Get pictures",
            Kind::Read,
            "Gives pictures of what a window plays, as edited, at any time: at the times `at` \
             gives, or `count` of them spread evenly from `from` to `to`, the whole movie if \
             left out. Each is a JPEG at most `size` pixels on its long side, upright and as \
             edited, with the time it shows; parts that are black give none. Use it to see \
             a video's content, find a scene, or check an edit.",
            |app, frames: Frames, answer| get_frames(app, frames, answer),
        ),
        tool(
            "get_sound",
            "Get the sound",
            Kind::Read,
            "Gives the sound of what a window plays, as edited, from `from` to `to` seconds, \
             60 seconds at most at a time, as a WAV: one channel at 16 kHz, silence where \
             there is none. Only for models that hear sound; others measure it with \
             get_levels and read what is said with get_subtitles.",
            |app, range: Range, answer| get_sound(app, range, answer),
        ),
        tool(
            "get_levels",
            "Measure the sound",
            Kind::Read,
            "Measures how loud a window's sound is, as edited, from `from` to `to`, the whole \
             movie if left out: the loudness and peak of each step, in dBFS, of the whole, \
             and the silences, where it stays below `silence` dBFS for `shortest` seconds or \
             more. Use it to find pauses to cut, loud and quiet parts, and where music or \
             speech starts.",
            |app, levels: Levels, answer| get_levels(app, levels, answer),
        ),
        tool(
            "get_subtitles",
            "Read the subtitles",
            Kind::Read,
            "Gives the text of a window's subtitles, as they show in the edit, from `from` to \
             `to`, all of it if left out: each line's start and end in seconds, and its text. \
             Subtitles that are pictures have no text to give.",
            |app, subtitles: Subtitles, answer| get_subtitles(app, subtitles, answer),
        ),
        tool(
            "get_edit",
            "Get the edit",
            Kind::Read,
            "Says how a window's file is edited: the edited movie's length; its clips in the \
             order they play, numbered from 1, each with where it starts and ends in the \
             edited movie, the file and the part of it it plays, and what plays in place of \
             its picture or sound; how the picture is turned and flipped; whether there are \
             changes to undo or redo, and changes not exported yet.",
            |app, on: super::On, answer| match edit_of(app, on.window) {
                Ok(value) => reply(answer, Ok(Output::Json(value))),
                Err(error) => reply(answer, Err(error)),
            },
        ),
        tool(
            "trim",
            "Trim",
            Kind::Edit,
            "Keeps only the part of a window's movie from `from` to `to` seconds, as Trim \
             does. Times after it move up. Undo puts it back.",
            |app, part: Between, answer| {
                edit(app, part.window, answer, |edit| {
                    edit.trim(part.from, part.to)
                        .then_some("Trimmed")
                        .ok_or("trim: the part must be at least 0.1 seconds long, within the movie")
                })
            },
        ),
        tool(
            "cut",
            "Cut out",
            Kind::Edit,
            "Takes the part of a window's movie from `from` to `to` seconds out; what comes \
             after moves up to close the gap. Undo puts it back.",
            |app, part: Between, answer| {
                edit(app, part.window, answer, |edit| {
                    edit.cut(part.from, part.to).then_some("Cut").ok_or(
                        "cut: the part must be within the movie, and leave at least 0.1 seconds",
                    )
                })
            },
        ),
        tool(
            "split",
            "Split",
            Kind::Edit,
            "Splits the clip playing at `at` seconds of a window's movie in two, as Split \
             does, so each part can be moved or taken out on its own.",
            |app, split: Split, answer| {
                edit(app, split.window, answer, |edit| {
                    edit.split(split.at)
                        .then_some("Split")
                        .ok_or("split: too close to a clip's start or end")
                })
            },
        ),
        tool(
            "move_clip",
            "Move a clip",
            Kind::Edit,
            "Moves clip `clip` of a window's movie, by its number from get_edit, to place \
             `to`, so the clips play in another order.",
            |app, moved: MoveClip, answer| {
                edit(app, moved.window, answer, |edit| {
                    let count = edit.clips().len();
                    if !(1..=count).contains(&moved.clip) || !(1..=count).contains(&moved.to) {
                        return Err("move_clip: there is no such clip or place; see get_edit");
                    }
                    edit.move_clip(moved.clip - 1, moved.to - 1)
                        .then_some("Moved")
                        .ok_or("move_clip: the clip is there already")
                })
            },
        ),
        tool(
            "insert",
            "Insert",
            Kind::Edit,
            "Puts part of a file, `from` to `to` seconds of it, all of it if left out, into a \
             window's movie at `at` seconds, the end if left out, splitting the clip there. \
             Its picture shows fitted in the movie's; a file with no picture shows black, \
             one with no sound is silent. The window's own file repeats a part of it.",
            |app, insert: Insert, answer| start_insert(app, insert, answer),
        ),
        tool(
            "replace",
            "Replace the picture or sound",
            Kind::Edit,
            "Plays another file's picture or sound in place of a window's own, from `from` to \
             `to` seconds of the movie, from `file_from` seconds of the file on: for music or \
             a voice over a video, or other pictures over its sound. Without a file, silence \
             or black; with `original`, the clips' own back. Where the file runs out, \
             silence or black.",
            |app, replace: Replace, answer| start_replace(app, replace, answer),
        ),
        tool(
            "mix_in",
            "Mix in sound",
            Kind::Edit,
            "Mixes a file's sound into a window's, heard together with what plays: music \
             under a video, a voice over it, or a sound effect. Part of the file, `from` to \
             `to` seconds of it, all of it if left out, starts at `at` seconds of the movie, \
             the start if left out, at `volume` decibels, with fades. It stays at its time as \
             clips change, and stops at the movie's end. To lower the window's own sound under \
             it, add a volume filter there with apply_filter. remove_mix takes it out.",
            |app, mix: MixIn, answer| start_mix(app, mix, answer),
        ),
        tool(
            "remove_mix",
            "Remove mixed-in sound",
            Kind::Edit,
            "Takes sound mixed in with mix_in out of a window's movie, by its number from \
             get_edit.",
            |app, remove: RemoveMix, answer| {
                edit(app, remove.window, answer, |edit| {
                    remove
                        .mix
                        .checked_sub(1)
                        .is_some_and(|index| edit.remove_layer(index))
                        .then_some("Took the mixed-in sound out of")
                        .ok_or("remove_mix: there is no such sound mixed in; see get_edit")
                })
            },
        ),
        tool(
            "turn_picture",
            "Turn the picture",
            Kind::Edit,
            "Turns a window's picture a quarter turn left or right, or flips it horizontally \
             or vertically, as it plays and exports, as the Edit menu does.",
            |app, turn: TurnPicture, answer| {
                if turn.turn.is_none() && turn.flip.is_none() {
                    return reply(answer, Err(invalid("turn_picture: give `turn` or `flip`")));
                }
                if let Ok((_, player)) = app.player_window(turn.window)
                    && !player.is_video()
                {
                    return reply(
                        answer,
                        Err(invalid("turn_picture: this window plays sound alone")),
                    );
                }
                edit(app, turn.window, answer, |edit| {
                    if let Some(way) = turn.turn {
                        edit.rotate(way == Turn::Right);
                    }
                    if let Some(way) = turn.flip {
                        edit.flip(way == Flip::Horizontal);
                    }
                    Ok("Turned")
                })
            },
        ),
        tool(
            "apply_filter",
            "Apply filters",
            Kind::Edit,
            "Puts a window's picture, sound or both through FFmpeg filters from `from` to `to` \
             seconds, the whole movie if left out, as an edit that plays at once and that undo \
             takes back. `picture` and `sound` are filter graphs as -vf and -af take them: \
             \"hue=s=0\" makes it black and white, \"eq=contrast=1.2:brightness=0.05:saturation=1.3\", \
             \"colortemperature=temperature=5000\", \"crop=ih:ih\", \"unsharp\", \"hqdn3d\", \
             \"vignette\", \"drawtext=text='Title':fontsize=64:fontcolor=white:x=(w-tw)/2:y=h/8\", \
             \"fade=t=in:st=0:d=2\"; \"highpass=f=80,acompressor,loudnorm\", \"afftdn\", \
             \"volume=0.5\", \"afade=t=out:st=55:d=5\". They add to filters already there. Times in \
             filters are seconds of the edited movie; the picture is upright. Only filters that \
             change each frame as it comes are allowed: none that read or write files, reach \
             devices or the network, or change timing, which trim, cut and the other tools do. \
             plaa checks them first and says why it will not run one.",
            |app, apply: ApplyFilter, answer| apply_filter(app, apply, answer),
        ),
        tool(
            "remove_filters",
            "Remove filters",
            Kind::Edit,
            "Takes every filter off a window's picture and sound from `from` to `to` seconds, the \
             whole movie if left out.",
            |app, range: Range, answer| {
                let length = match reading(app, range.window) {
                    Ok((_, reading)) => reading.length,
                    Err(error) => return reply(answer, Err(error)),
                };
                let (from, to) = match span("remove_filters", range.from, range.to, length) {
                    Ok(span) => span,
                    Err(error) => return reply(answer, Err(error)),
                };
                edit(app, range.window, answer, |edit| {
                    edit.remove_filters(from, to)
                        .then_some("Took the filters off")
                        .ok_or("remove_filters: there are no filters there")
                })
            },
        ),
        tool(
            "undo",
            "Undo",
            Kind::Edit,
            "Undoes a window's last change to its edit; with `redo`, redoes the change last \
             undone; with `all`, undoes every change, so the file plays as it is.",
            |app, undo: Undo, answer| undo_edit(app, undo, answer),
        ),
        tool(
            "export",
            "Export",
            Kind::Edit,
            "Exports a window's movie, as edited, to a new file at the full path `file`: an \
             MP4 in `format` h264, hevc or av1, its picture made smaller when `lines` says; \
             or the sound alone in aac, mp3, opus, flac, alac or wav, at `quality`. Answers \
             once the export is done, with the file's path, as it can take a while.",
            |app, export: ExportTo, answer| start_export(app, export, answer),
        ),
    ]
}

fn invalid(message: impl Into<String>) -> Error {
    Error::new(code::INVALID_PARAMS, message)
}

/// The file window `id` plays, as read needs it.
struct Reading {
    path: PathBuf,
    edit: Option<Edit>,
    video: Option<usize>,
    audio: Option<usize>,
    /// How long the edited movie lasts.
    length: f64,
}

impl Reading {
    fn edited(&self) -> Edited<'_> {
        Edited {
            path: &self.path,
            edit: self.edit.as_ref(),
            video: self.video,
            audio: self.audio,
        }
    }
}

/// What window `window`'s player plays, for reading: a file whose length
/// is known.
fn reading(app: &Plaa, window: Option<u64>) -> Result<(window::Id, Reading), Error> {
    let (id, player) = app.player_window(window)?;
    let Source::File(path) = player.source() else {
        return Err(invalid(format!(
            "Window {} plays a stream; only files can be read this way.",
            number(id)
        )));
    };
    let Some(duration) = player.file_duration() else {
        return Err(invalid(format!(
            "Window {} is still opening its file.",
            number(id)
        )));
    };
    let edit = player.editor.edit().cloned();
    let length = edit.as_ref().map_or(duration, Edit::duration);
    Ok((
        id,
        Reading {
            path: path.clone(),
            edit,
            video: player.status.video,
            audio: player.status.audio,
            length,
        },
    ))
}

/// `from` and `to` within a movie `length` seconds long, the whole of it
/// where left out.
fn span(tool: &str, from: Option<f64>, to: Option<f64>, length: f64) -> Result<(f64, f64), Error> {
    let from = from.unwrap_or(0.0);
    let to = to.unwrap_or(length).min(length);
    if !from.is_finite() || !to.is_finite() || from < 0.0 || to <= from {
        return Err(invalid(format!(
            "{tool}: `from` must come before `to`, within the movie's {} seconds",
            round(length)
        )));
    }
    Ok((from, to))
}

/// Does `work` on a thread of its own for window `id`, which tells the
/// agent, saying `message`, and the window how far it has come once it
/// takes a while; answers with what it gives.
fn on_thread(
    app: &mut Plaa,
    id: window::Id,
    answer: &Answer,
    message: &'static str,
    work: impl FnOnce(&Progress) -> Result<Output, Error> + Send + 'static,
) -> Task<Message> {
    let (progress, shown) = app.begin_work(id, answer, message);
    let task = answer.clone().later(Task::perform(
        async move {
            let (sender, receiver) = iced::futures::channel::oneshot::channel();
            let spawned = std::thread::Builder::new()
                .name("plaa-tool".into())
                .spawn(move || {
                    let result = work(&progress);
                    crate::post(crate::External::Work(id, None));
                    let _ = sender.send(result);
                });
            if let Err(error) = spawned {
                return Err(Error::new(code::INTERNAL_ERROR, error.to_string()));
            }
            receiver
                .await
                .unwrap_or_else(|_| Err(Error::new(code::INTERNAL_ERROR, "the tool stopped")))
        },
        |result| result,
    ));
    Task::batch([task, shown])
}

fn get_frames(app: &mut Plaa, frames: Frames, answer: &Answer) -> Task<Message> {
    let (id, reading) = match reading(app, frames.window) {
        Ok(found) => found,
        Err(error) => return reply(answer, Err(error)),
    };
    if reading.video.is_none() {
        return reply(
            answer,
            Err(invalid(format!(
                "Window {} plays sound alone: there are no pictures.",
                number(id)
            ))),
        );
    }
    let times: Vec<f64> = match frames.at {
        Some(at) => {
            if at.is_empty() || at.len() > MOST_PICTURES {
                return reply(
                    answer,
                    Err(invalid(format!(
                        "get_frames: give 1 to {MOST_PICTURES} times"
                    ))),
                );
            }
            if at.iter().any(|time| !time.is_finite() || *time < 0.0) {
                return reply(answer, Err(invalid("get_frames: times are seconds from 0")));
            }
            at.into_iter()
                .map(|time| time.min((reading.length - 0.001).max(0.0)))
                .collect()
        }
        None => {
            let (from, to) = match span("get_frames", frames.from, frames.to, reading.length) {
                Ok(span) => span,
                Err(error) => return reply(answer, Err(error)),
            };
            let count = frames.count.unwrap_or(6).clamp(1, MOST_PICTURES);
            (0..count)
                .map(|index| from + (index as f64 + 0.5) * (to - from) / count as f64)
                .collect()
        }
    };
    let side = frames.size.unwrap_or(512).clamp(160, 1280);
    let window = number(id);
    on_thread(app, id, answer, "Reading the pictures.", move |progress| {
        let pictures = read::pictures(&reading.edited(), &times, side, &|share| {
            progress.report(share);
        });
        let mut parts = Vec::new();
        for (time, picture) in times.iter().zip(pictures) {
            match picture.map(|picture| read::jpeg(&picture, 4)) {
                Some(Ok(jpeg)) => {
                    parts.push(Part::Text(format!("Window {window} at {:.2} s:", time)));
                    parts.push(Part::Jpeg(jpeg));
                }
                Some(Err(error)) => {
                    parts.push(Part::Text(format!("At {:.2} s, no picture: {error}", time)));
                }
                None => parts.push(Part::Text(format!("At {:.2} s: black.", time))),
            }
        }
        Ok(Output::Parts(parts))
    })
}

fn get_sound(app: &mut Plaa, range: Range, answer: &Answer) -> Task<Message> {
    let (id, reading) = match reading(app, range.window) {
        Ok(found) => found,
        Err(error) => return reply(answer, Err(error)),
    };
    let (from, to) = match span("get_sound", range.from, range.to, reading.length) {
        Ok(span) => span,
        Err(error) => return reply(answer, Err(error)),
    };
    if to - from > MOST_SOUND + 1e-6 {
        return reply(
            answer,
            Err(invalid(format!(
                "get_sound: {MOST_SOUND:.0} seconds at most at a time; the movie lasts {} s",
                round(reading.length)
            ))),
        );
    }
    let window = number(id);
    on_thread(app, id, answer, "Reading the sound.", move |progress| {
        let mut samples = Vec::new();
        let total = ((to - from) * f64::from(SOUND_RATE)).max(1.0);
        read::sound(&reading.edited(), from, to, SOUND_RATE, &mut |chunk| {
            samples.extend_from_slice(chunk);
            progress.report(samples.len() as f64 / total);
        })
        .map_err(|error| Error::new(code::INTERNAL_ERROR, error))?;
        Ok(Output::Parts(vec![
            Part::Text(format!(
                "Window {window}'s sound from {:.2} to {:.2} s, one channel at {} Hz:",
                from, to, SOUND_RATE
            )),
            Part::Wav(wav(&samples, SOUND_RATE)),
        ]))
    })
}

/// `samples` of one channel at `rate`, as a 16-bit WAV file.
fn wav(samples: &[f32], rate: u32) -> Vec<u8> {
    let data = (samples.len() * 2) as u32;
    let mut bytes = Vec::with_capacity(44 + data as usize);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36 + data).to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&rate.to_le_bytes());
    bytes.extend_from_slice(&(rate * 2).to_le_bytes());
    bytes.extend_from_slice(&2u16.to_le_bytes());
    bytes.extend_from_slice(&16u16.to_le_bytes());
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data.to_le_bytes());
    for sample in samples {
        let value = (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16;
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes
}

/// Decibels of full scale for an amplitude, with silence at -120.
fn decibels(amplitude: f64) -> f64 {
    if amplitude <= 1e-6 {
        -120.0
    } else {
        (20.0 * amplitude.log10() * 10.0).round() / 10.0
    }
}

/// How loud sound is over a stretch: its sum of squares, its peak and how
/// many samples.
#[derive(Default, Clone, Copy)]
struct Loudness {
    squares: f64,
    peak: f64,
    count: usize,
}

impl Loudness {
    fn add(&mut self, sample: f32) {
        let value = f64::from(sample);
        self.squares += value * value;
        self.peak = self.peak.max(value.abs());
        self.count += 1;
    }

    fn rms(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            (self.squares / self.count as f64).sqrt()
        }
    }
}

/// The levels of each step and where it starts, the whole's, and the
/// silences, from and to.
type Measured = (Vec<(f64, Loudness)>, Loudness, Vec<(f64, f64)>);

/// Measures sound as it comes: the levels of each step, the whole's, and
/// the silences.
struct Meter {
    rate: u32,
    from: f64,
    per_step: usize,
    /// Silence is judged over blocks of a twentieth of a second.
    per_block: usize,
    /// The amplitude below which sound counts as silence.
    limit: f64,
    shortest: f64,
    steps: Vec<(f64, Loudness)>,
    whole: Loudness,
    current: Loudness,
    block: Loudness,
    silences: Vec<(f64, f64)>,
    quiet_since: Option<f64>,
    count: usize,
}

impl Meter {
    /// A meter for sound at `rate` from `from` seconds, in steps of `step`
    /// seconds, with silence below `threshold` dBFS for `shortest`
    /// seconds or more.
    fn new(rate: u32, from: f64, step: f64, threshold: f64, shortest: f64) -> Self {
        Self {
            rate,
            from,
            per_step: ((step * f64::from(rate)).round() as usize).max(1),
            per_block: (rate as usize / 20).max(1),
            limit: 10f64.powf(threshold / 20.0),
            shortest,
            steps: Vec::new(),
            whole: Loudness::default(),
            current: Loudness::default(),
            block: Loudness::default(),
            silences: Vec::new(),
            quiet_since: None,
            count: 0,
        }
    }

    fn at(&self, count: usize) -> f64 {
        self.from + count as f64 / f64::from(self.rate)
    }

    fn feed(&mut self, samples: &[f32]) {
        for sample in samples {
            self.current.add(*sample);
            self.block.add(*sample);
            self.whole.add(*sample);
            self.count += 1;
            if self.current.count == self.per_step {
                self.steps
                    .push((self.at(self.count - self.per_step), self.current));
                self.current = Loudness::default();
            }
            if self.block.count == self.per_block {
                let start = self.at(self.count - self.per_block);
                if self.block.rms() < self.limit {
                    self.quiet_since.get_or_insert(start);
                } else if let Some(since) = self.quiet_since.take()
                    && start - since >= self.shortest
                {
                    self.silences.push((since, start));
                }
                self.block = Loudness::default();
            }
        }
    }

    /// The levels of each step, the whole's, and the silences.
    fn finish(mut self) -> Measured {
        if self.current.count > 0 {
            self.steps
                .push((self.at(self.count - self.current.count), self.current));
        }
        let end = self.at(self.count);
        if let Some(since) = self.quiet_since
            && end - since >= self.shortest
        {
            self.silences.push((since, end));
        }
        (self.steps, self.whole, self.silences)
    }
}

fn get_levels(app: &mut Plaa, levels: Levels, answer: &Answer) -> Task<Message> {
    let (id, reading) = match reading(app, levels.window) {
        Ok(found) => found,
        Err(error) => return reply(answer, Err(error)),
    };
    let (from, to) = match span("get_levels", levels.from, levels.to, reading.length) {
        Ok(span) => span,
        Err(error) => return reply(answer, Err(error)),
    };
    let wanted = levels.step.filter(|step| step.is_finite() && *step > 0.0);
    let step = wanted
        .unwrap_or((to - from) / 200.0)
        .max((to - from) / MOST_LEVELS as f64)
        .max(0.05);
    let threshold = levels.silence.unwrap_or(-45.0).clamp(-100.0, 0.0);
    let shortest = levels.shortest.unwrap_or(0.5).max(0.05);
    let window = number(id);
    on_thread(app, id, answer, "Measuring the sound.", move |progress| {
        let mut meter = Meter::new(LEVELS_RATE, from, step, threshold, shortest);
        let total = ((to - from) * f64::from(LEVELS_RATE)).max(1.0);
        let mut read_so_far = 0usize;
        read::sound(&reading.edited(), from, to, LEVELS_RATE, &mut |chunk| {
            meter.feed(chunk);
            read_so_far += chunk.len();
            progress.report(read_so_far as f64 / total);
        })
        .map_err(|error| Error::new(code::INTERNAL_ERROR, error))?;
        let (steps, whole, silences) = meter.finish();
        Ok(Output::Json(json!({
            "window": window,
            "from": round(from),
            "to": round(to),
            "step": round(step),
            "loudness": decibels(whole.rms()),
            "peak": decibels(whole.peak),
            "levels_are": "[start, loudness, peak] in seconds and dBFS",
            "levels": steps
                .iter()
                .map(|(at, loudness)| json!([round(*at), decibels(loudness.rms()), decibels(loudness.peak)]))
                .collect::<Vec<_>>(),
            "silence_below": threshold,
            "silences": silences
                .iter()
                .map(|(start, end)| json!([round(*start), round(*end)]))
                .collect::<Vec<_>>(),
        })))
    })
}

fn get_subtitles(app: &mut Plaa, subtitles: Subtitles, answer: &Answer) -> Task<Message> {
    let (id, reading) = match reading(app, subtitles.window) {
        Ok(found) => found,
        Err(error) => return reply(answer, Err(error)),
    };
    let (from, to) = match span(
        "get_subtitles",
        subtitles.from,
        subtitles.to,
        reading.length,
    ) {
        Ok(span) => span,
        Err(error) => return reply(answer, Err(error)),
    };
    let Ok((_, player)) = app.player_window(Some(number(id))) else {
        return reply(answer, Err(super::no_window()));
    };
    let Some(info) = player.info.clone() else {
        return reply(answer, Err(invalid("The file is still opening.")));
    };
    let chosen = subtitles.stream.or(player.status.subtitles).or_else(|| {
        info.subtitles
            .iter()
            .find(|track| track.text)
            .map(|track| track.track.index)
    });
    let Some(stream) = chosen else {
        return reply(
            answer,
            Err(invalid(format!("Window {} has no subtitles.", number(id)))),
        );
    };
    let Some(track) = info
        .subtitles
        .iter()
        .find(|track| track.track.index == stream)
        .cloned()
    else {
        return reply(answer, Err(super::no_stream(id, stream, "subtitles")));
    };
    let window = number(id);
    on_thread(app, id, answer, "Reading the subtitles.", move |progress| {
        let report = |share: f64| progress.report(share);
        let cues = match &track.file {
            Some(file) => read::subtitle_text(file, None, &report),
            None => read::subtitle_text(&reading.path, Some(stream), &report),
        }
        .map_err(|error| Error::new(code::INVALID_PARAMS, error))?;
        let lines: Vec<Value> = read::edited_cues(reading.edit.as_ref(), &cues)
            .into_iter()
            .filter(|cue| cue.end > from && cue.start < to)
            .take(MOST_LINES)
            .map(|cue| {
                json!({
                    "start": round(cue.start),
                    "end": round(cue.end),
                    "text": cue.text,
                })
            })
            .collect();
        Ok(Output::Json(json!({
            "window": window,
            "stream": stream,
            "language": track.track.language,
            "lines": lines,
        })))
    })
}

/// How window `window`'s file is edited, for get_edit.
fn edit_of(app: &Plaa, window: Option<u64>) -> Result<Value, Error> {
    let (id, player) = app.player_window(window)?;
    let Source::File(path) = player.source() else {
        return Err(invalid(format!(
            "Window {} plays a stream, which cannot be edited.",
            number(id)
        )));
    };
    let duration = player.file_duration().unwrap_or(0.0);
    let edit = player
        .editor
        .edit()
        .cloned()
        .unwrap_or_else(|| Edit::whole(duration));
    let file_of = |source: usize| match edit.other(source) {
        Some(other) => other.path.display().to_string(),
        None => path.display().to_string(),
    };
    let filters = |clip: &Clip| match edit.filters_of(clip) {
        Some(filters) => json!({ "picture": filters.picture, "sound": filters.sound }),
        None => Value::Null,
    };
    let feed = |feed: Option<Feed>, nothing: &str| match feed {
        None => Value::Null,
        Some(Feed::Blank) => json!(nothing),
        Some(Feed::File { source, from }) => json!({
            "file": file_of(source),
            "from": round(from),
        }),
    };
    let clips: Vec<Value> = edit
        .clips()
        .iter()
        .zip(edit.starts())
        .enumerate()
        .map(|(index, (clip, start))| {
            json!({
                "clip": index + 1,
                "start": round(start),
                "end": round(start + clip.length()),
                "file": file_of(clip.source),
                "from": round(clip.from),
                "to": round(clip.to),
                "picture": feed(clip.picture, "black"),
                "sound": feed(clip.sound, "silence"),
                "filters": filters(clip),
            })
        })
        .collect();
    let mixed: Vec<Value> = edit
        .layers()
        .iter()
        .enumerate()
        .map(|(index, layer)| {
            json!({
                "mix": index + 1,
                "file": file_of(layer.source),
                "at": round(layer.at),
                "end": round(layer.end()),
                "from": round(layer.from),
                "to": round(layer.to),
                "volume": round(layer.volume),
                "fade_in": round(layer.fade_in),
                "fade_out": round(layer.fade_out),
            })
        })
        .collect();
    let transform = edit.transform();
    Ok(json!({
        "window": number(id),
        "file": path.display().to_string(),
        "edited": player.editor.is_edited(),
        "duration": round(edit.duration()),
        "clips": clips,
        "mixed_in": mixed,
        "quarter_turns": transform.turns,
        "mirrored": transform.mirrored,
        "can_undo": player.editor.can_undo(),
        "can_redo": player.editor.can_redo(),
        "unexported": player.editor.unexported(),
        "editable": player.can_edit(),
    }))
}

/// The window an edit tool acts on: one whose file can be edited.
fn editable(app: &Plaa, window: Option<u64>) -> Result<window::Id, Error> {
    let (id, player) = app.player_window(window)?;
    if !player.can_edit() {
        return Err(invalid(format!(
            "Window {} cannot be edited: it plays a stream, or a file still opening or that \
             could not play.",
            number(id)
        )));
    }
    Ok(id)
}

/// Changes window `window`'s edit with `change`, which says what it did
/// or why it could not, and answers with the edit as it is then.
fn edit(
    app: &mut Plaa,
    window: Option<u64>,
    answer: &Answer,
    change: impl FnOnce(&mut Edit) -> Result<&'static str, &'static str>,
) -> Task<Message> {
    let id = match editable(app, window) {
        Ok(id) => id,
        Err(error) => return reply(answer, Err(error)),
    };
    let mut said = Ok("");
    let task = app.tool_edit(id, |edit| {
        said = change(edit);
        said.is_ok()
    });
    match said {
        Ok(done) => {
            let edit = edit_of(app, Some(number(id))).unwrap_or_default();
            answer.send(Ok(Output::Json(json!({
                "done": format!("{done} window {}.", number(id)),
                "duration": edit["duration"],
                "clips": edit["clips"].as_array().map_or(0, Vec::len),
            }))));
            task
        }
        Err(why) => reply(answer, Err(invalid(why))),
    }
}

/// A file's full path, as a tool's argument, which must be there.
fn existing(tool: &str, file: &str) -> Result<PathBuf, Error> {
    let path = PathBuf::from(file.trim());
    if !path.is_absolute() {
        return Err(invalid(format!("{tool}: give the file's full path")));
    }
    if !path.is_file() {
        return Err(invalid(format!(
            "{tool}: there is no file {}",
            path.display()
        )));
    }
    Ok(path)
}

/// Reads what the file at `path` holds on a thread of its own, then makes
/// `change` with it.
fn with_file(path: PathBuf, change: FileChange, answer: &Answer) -> Task<Message> {
    let answer = answer.clone();
    Task::perform(
        async move {
            let (sender, receiver) = iced::futures::channel::oneshot::channel();
            let file = path.clone();
            let spawned = std::thread::Builder::new()
                .name("plaa-probe".into())
                .spawn(move || {
                    let _ = sender.send(MediaInfo::probe(&file));
                });
            let info = match spawned {
                Ok(_) => receiver
                    .await
                    .unwrap_or_else(|_| Err("plaa could not read it".to_owned())),
                Err(error) => Err(error.to_string()),
            };
            (path, info)
        },
        move |(path, info)| Message::ToolFile(answer.clone(), change.clone(), path, Box::new(info)),
    )
}

fn start_insert(app: &mut Plaa, insert: Insert, answer: &Answer) -> Task<Message> {
    let id = match editable(app, insert.window) {
        Ok(id) => id,
        Err(error) => return reply(answer, Err(error)),
    };
    let path = match existing("insert", &insert.file) {
        Ok(path) => path,
        Err(error) => return reply(answer, Err(error)),
    };
    with_file(
        path,
        FileChange::Insert {
            window: id,
            at: insert.at,
            from: insert.from,
            to: insert.to,
        },
        answer,
    )
}

fn start_replace(app: &mut Plaa, replace: Replace, answer: &Answer) -> Task<Message> {
    let id = match editable(app, replace.window) {
        Ok(id) => id,
        Err(error) => return reply(answer, Err(error)),
    };
    let track = replace.track.track();
    if track == Track::Picture
        && let Ok((_, player)) = app.player_window(Some(number(id)))
        && !player.is_video()
    {
        return reply(
            answer,
            Err(invalid(
                "replace: this window plays sound alone, with no picture",
            )),
        );
    }
    let (from, to) = (replace.from, replace.to);
    if replace.original == Some(true) || replace.file.is_none() {
        let feed = (replace.original != Some(true)).then_some(Feed::Blank);
        let done = match (feed, track) {
            (None, _) => "Put the clips' own back in",
            (Some(_), Track::Sound) => "Made silent part of",
            (Some(_), Track::Picture) => "Made black part of",
        };
        return edit(app, Some(number(id)), answer, |edit| {
            edit.replace_track(track, from, to, feed)
                .then_some(done)
                .ok_or("replace: nothing changed; the part must be within the movie")
        });
    }
    let path = match existing("replace", replace.file.as_deref().unwrap_or_default()) {
        Ok(path) => path,
        Err(error) => return reply(answer, Err(error)),
    };
    with_file(
        path,
        FileChange::Replace {
            window: id,
            track,
            from,
            to,
            file_from: replace.file_from.unwrap_or(0.0).max(0.0),
        },
        answer,
    )
}

fn start_mix(app: &mut Plaa, mix: MixIn, answer: &Answer) -> Task<Message> {
    let id = match editable(app, mix.window) {
        Ok(id) => id,
        Err(error) => return reply(answer, Err(error)),
    };
    let path = match existing("mix_in", &mix.file) {
        Ok(path) => path,
        Err(error) => return reply(answer, Err(error)),
    };
    let volume = mix.volume.unwrap_or(0.0);
    if !(-60.0..=24.0).contains(&volume) {
        return reply(
            answer,
            Err(invalid("mix_in: the volume is from -60 to 24 decibels")),
        );
    }
    let fade = |seconds: Option<f64>| seconds.unwrap_or(0.0).clamp(0.0, 60.0);
    with_file(
        path,
        FileChange::Mix {
            window: id,
            at: mix.at,
            from: mix.from,
            to: mix.to,
            volume,
            fades: (fade(mix.fade_in), fade(mix.fade_out)),
        },
        answer,
    )
}

impl Plaa {
    /// Makes `change` to a window's edit, with what the file at `path`
    /// holds, once read.
    pub(crate) fn file_change(
        &mut self,
        answer: Answer,
        change: FileChange,
        path: PathBuf,
        info: Result<MediaInfo, String>,
    ) -> Task<Message> {
        let info = match info {
            Ok(info) => info,
            Err(error) => {
                return reply(
                    &answer,
                    Err(invalid(format!("{}: {error}", path.display()))),
                );
            }
        };
        let window = match &change {
            FileChange::Insert { window, .. }
            | FileChange::Replace { window, .. }
            | FileChange::Mix { window, .. } => *window,
        };
        let Some(player) = self.player_mut(window) else {
            return reply(&answer, Err(super::no_window()));
        };
        let own = is_own(player, &path);
        let length = info.duration.map_or(0.0, |duration| duration.as_secs_f64());
        let other = Other::new(path.clone(), &info);
        match change {
            FileChange::Insert { at, from, to, .. } => {
                if !other.picture && !other.sound {
                    return reply(
                        &answer,
                        Err(invalid("insert: the file has no picture or sound")),
                    );
                }
                let from = from.unwrap_or(0.0).max(0.0);
                let to = to.unwrap_or(length).min(length);
                if to - from < plaa_media::edit::SHORTEST {
                    return reply(
                        &answer,
                        Err(invalid(format!(
                            "insert: the part must be at least 0.1 seconds, within the file's {} seconds",
                            round(length)
                        ))),
                    );
                }
                edit(self, Some(number(window)), &answer, |edit| {
                    let source = if own { OWN } else { edit.add_other(other) };
                    let at = at.unwrap_or(edit.duration()).clamp(0.0, edit.duration());
                    let clip = Clip {
                        source,
                        ..Clip::new(from, to)
                    };
                    edit.insert(at, clip)
                        .then_some("Inserted in")
                        .ok_or("insert: too short")
                })
            }
            FileChange::Mix {
                at,
                from,
                to,
                volume,
                fades: (fade_in, fade_out),
                ..
            } => {
                if !other.sound {
                    return reply(&answer, Err(invalid("mix_in: the file has no sound")));
                }
                let from = from.unwrap_or(0.0).max(0.0);
                let to = to.unwrap_or(length).min(length);
                if to - from < plaa_media::edit::SHORTEST {
                    return reply(
                        &answer,
                        Err(invalid(format!(
                            "mix_in: the part must be at least 0.1 seconds, within the file's {} seconds",
                            round(length)
                        ))),
                    );
                }
                edit(self, Some(number(window)), &answer, |edit| {
                    let source = if own { OWN } else { edit.add_other(other) };
                    let layer = plaa_media::edit::Layer {
                        source,
                        at: at.unwrap_or(0.0),
                        from,
                        to,
                        volume,
                        fade_in,
                        fade_out,
                    };
                    edit.add_layer(layer)
                        .then_some("Mixed sound into")
                        .ok_or("mix_in: `at` must be within the movie")
                })
            }
            FileChange::Replace {
                track,
                from,
                to,
                file_from,
                ..
            } => {
                let has = match track {
                    Track::Picture => other.picture,
                    Track::Sound => other.sound,
                };
                if !has {
                    return reply(
                        &answer,
                        Err(invalid(match track {
                            Track::Picture => "replace: the file has no picture",
                            Track::Sound => "replace: the file has no sound",
                        })),
                    );
                }
                edit(self, Some(number(window)), &answer, |edit| {
                    let source = if own { OWN } else { edit.add_other(other) };
                    edit.replace_track(
                        track,
                        from,
                        to,
                        Some(Feed::File {
                            source,
                            from: file_from,
                        }),
                    )
                    .then_some("Replaced part of")
                    .ok_or("replace: nothing changed; the part must be within the movie")
                })
            }
        }
    }
}

/// Whether `path` is the file `player` plays.
fn is_own(player: &PlayerWindow, path: &Path) -> bool {
    let Source::File(own) = player.source() else {
        return false;
    };
    match (own.canonicalize(), path.canonicalize()) {
        (Ok(own), Ok(path)) => own == path,
        _ => own == path,
    }
}

fn apply_filter(app: &mut Plaa, apply: ApplyFilter, answer: &Answer) -> Task<Message> {
    let id = match editable(app, apply.window) {
        Ok(id) => id,
        Err(error) => return reply(answer, Err(error)),
    };
    let Ok((_, player)) = app.player_window(Some(number(id))) else {
        return reply(answer, Err(super::no_window()));
    };
    let given = |chain: Option<String>| {
        chain
            .map(|chain| chain.trim().to_owned())
            .filter(|chain| !chain.is_empty())
    };
    let added = Filters {
        picture: given(apply.picture),
        sound: given(apply.sound),
    };
    if added.is_empty() {
        return reply(
            answer,
            Err(invalid("apply_filter: give `picture` or `sound` filters")),
        );
    }
    let Some(info) = player.info.clone() else {
        return reply(answer, Err(invalid("The file is still opening.")));
    };
    let picture = info
        .picture(player.status.video)
        .filter(|_| player.is_video());
    if added.picture.is_some() && picture.is_none() {
        return reply(
            answer,
            Err(invalid(
                "apply_filter: this window plays sound alone, with no picture",
            )),
        );
    }
    let sound = info
        .audio
        .iter()
        .find(|track| Some(track.track.index) == player.status.audio)
        .or(info.audio.first());
    let mixed = player.editor.edit().is_some_and(Edit::is_mixed);
    if added.sound.is_some() && sound.is_none() && !mixed {
        return reply(
            answer,
            Err(invalid("apply_filter: this window has no sound")),
        );
    }
    let edit_now = player.editor.working(player.file_duration().unwrap_or(0.0));
    let (from, to) = match span("apply_filter", apply.from, apply.to, edit_now.duration()) {
        Ok(span) => span,
        Err(error) => return reply(answer, Err(error)),
    };
    // Each set of filters a part would then go through, checked first.
    let (rate, channels) = sound.map_or((48000, 2), |track| (track.sample_rate, track.channels));
    for combined in edit_now.filters_with(from, to, &added) {
        if let (Some(chain), Some(track)) = (&combined.picture, picture) {
            let checked =
                filter::check_picture(chain, track.quarter_turns, PictureInput::of_track(track));
            if let Err(why) = checked {
                return reply(answer, Err(invalid(format!("apply_filter: {why}"))));
            }
        }
        if let Some(chain) = &combined.sound
            && let Err(why) = filter::check_sound(chain, rate, channels)
        {
            return reply(answer, Err(invalid(format!("apply_filter: {why}"))));
        }
    }
    edit(app, Some(number(id)), answer, |edit| {
        edit.add_filters(from, to, &added)
            .then_some("Filtered")
            .ok_or("apply_filter: the part must be within the movie")
    })
}

fn undo_edit(app: &mut Plaa, undo: Undo, answer: &Answer) -> Task<Message> {
    let id = match editable(app, undo.window) {
        Ok(id) => id,
        Err(error) => return reply(answer, Err(error)),
    };
    let Ok((_, player)) = app.player_window(Some(number(id))) else {
        return reply(answer, Err(super::no_window()));
    };
    let (action, possible, done) = if undo.all == Some(true) {
        (
            crate::shortcuts::Action::Revert,
            player.editor.is_edited(),
            "Undid every change to",
        )
    } else if undo.redo == Some(true) {
        (
            crate::shortcuts::Action::Redo,
            player.editor.can_redo(),
            "Redid a change to",
        )
    } else {
        (
            crate::shortcuts::Action::Undo,
            player.editor.can_undo(),
            "Undid a change to",
        )
    };
    if !possible {
        return reply(
            answer,
            Err(invalid("undo: there is nothing to undo or redo")),
        );
    }
    let task = app.edit_action(id, action);
    let edit = edit_of(app, Some(number(id))).unwrap_or_default();
    answer.send(Ok(Output::Json(json!({
        "done": format!("{done} window {}.", number(id)),
        "duration": edit["duration"],
        "clips": edit["clips"].as_array().map_or(0, Vec::len),
    }))));
    task
}

fn start_export(app: &mut Plaa, export: ExportTo, answer: &Answer) -> Task<Message> {
    let id = match editable(app, export.window) {
        Ok(id) => id,
        Err(error) => return reply(answer, Err(error)),
    };
    let destination = PathBuf::from(export.file.trim());
    if !destination.is_absolute() {
        return reply(answer, Err(invalid("export: give the file's full path")));
    }
    if destination.exists() && export.replace != Some(true) {
        return reply(
            answer,
            Err(invalid(format!(
                "export: {} is there already; give `replace` to write over it",
                destination.display()
            ))),
        );
    }
    if !destination.parent().is_some_and(Path::is_dir) {
        return reply(answer, Err(invalid("export: there is no such folder")));
    }
    let format = match export.format.as_deref() {
        Some(key) => match plaa_media::export::Format::from_key(key) {
            Some(format) => format,
            None => {
                return reply(
                    answer,
                    Err(invalid(format!("export: plaa does not export {key}"))),
                );
            }
        },
        None => plaa_media::export::Format::H264,
    };
    match app.export_for_agent(
        id,
        destination,
        format,
        export.lines,
        export.quality,
        answer,
    ) {
        Ok(()) => {
            answer.note("Exporting, which can take a while.");
            Task::none()
        }
        Err(error) => reply(answer, Err(invalid(format!("export: {error}")))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_find_steps_and_silences() {
        // A second of sound, a second of silence, half a second of sound.
        let rate = 1000;
        let samples: Vec<f32> = (0..2500)
            .map(|index| {
                let loud = !(1000..2000).contains(&index);
                if loud && index % 2 == 0 {
                    0.5
                } else if loud {
                    -0.5
                } else {
                    0.0
                }
            })
            .collect();
        let mut meter = Meter::new(rate, 10.0, 0.5, -45.0, 0.5);
        meter.feed(&samples[..1234]);
        meter.feed(&samples[1234..]);
        let (steps, whole, silences) = meter.finish();
        assert_eq!(steps.len(), 5);
        assert_eq!(steps[0].0, 10.0);
        assert!((decibels(steps[0].1.rms()) - -6.0).abs() < 0.1);
        assert_eq!(decibels(steps[2].1.rms()), -120.0);
        assert!(whole.peak > 0.49);
        assert_eq!(silences, [(11.0, 12.0)]);
    }

    #[test]
    fn sound_is_a_wav() {
        let bytes = wav(&[0.0, 1.0, -1.0], 16000);
        assert_eq!(&bytes[..4], b"RIFF");
        assert_eq!(bytes.len(), 44 + 6);
        assert_eq!(&bytes[46..48], &i16::MAX.to_le_bytes());
    }
}
