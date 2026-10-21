//! The tools plaa offers agents. Each has a name, a kind, which the
//! approval switches go by, a description the model reads, an input
//! schema made from its input type, and a function that runs it against
//! a window. A tool answers through its [`Answer`], at once or once what
//! it waits on is done.

use std::sync::LazyLock;
use std::time::Duration;

use base64::Engine;
use iced::{Task, window};
use plaa_media::Source;
use plaa_store::settings::AskBefore;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use super::{Content, Message, Plaa};
use crate::control::{Call, Error, code};
use crate::window::{Control, PlayerWindow, SPEEDS};

/// What a tool does, which decides whether plaa asks first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Kind {
    /// Sees what plays: the windows, the files and the picture.
    Read,
    /// Plays, pauses and seeks, and changes the sound, the tracks and
    /// full screen.
    Play,
    /// Opens files and addresses, and closes or raises windows.
    Open,
}

impl Kind {
    pub(crate) const ALL: [Kind; 3] = [Kind::Read, Kind::Play, Kind::Open];

    /// Its switch in Settings: whether plaa asks before a tool of this
    /// kind runs.
    pub(crate) fn asks(self, ask: &AskBefore) -> bool {
        match self {
            Kind::Read => ask.reading,
            Kind::Play => ask.playing,
            Kind::Open => ask.opening,
        }
    }

    pub(crate) fn set_asks(self, ask: &mut AskBefore, asks: bool) {
        *match self {
            Kind::Read => &mut ask.reading,
            Kind::Play => &mut ask.playing,
            Kind::Open => &mut ask.opening,
        } = asks;
    }
}

/// What a tool gives back.
#[derive(Debug, Clone)]
pub(crate) enum Output {
    /// An object, as structured content and the same as text for agents
    /// that read only text.
    Json(Value),
    Text(String),
    /// A picture, with a line saying what it shows.
    Png(Vec<u8>, String),
}

impl Output {
    /// The output as an MCP tool result.
    fn into_result(self) -> Value {
        match self {
            Output::Json(value) => json!({
                "content": [{ "type": "text", "text": value.to_string() }],
                "structuredContent": value,
            }),
            Output::Text(text) => json!({ "content": [{ "type": "text", "text": text }] }),
            Output::Png(png, note) => json!({
                "content": [
                    {
                        "type": "image",
                        "data": base64::engine::general_purpose::STANDARD.encode(png),
                        "mimeType": "image/png",
                    },
                    { "type": "text", "text": note },
                ],
            }),
        }
    }
}

/// Where a tool's answer goes. A tool that waits on something keeps it
/// and answers later, with [`Answer::later`]; dropped unanswered, it tells
/// the agent plaa did not answer.
#[derive(Debug, Clone)]
pub(crate) struct Answer(Call);

impl Answer {
    pub(super) fn send(&self, result: Result<Output, Error>) {
        self.0.reply(result.map(Output::into_result));
    }

    /// Tells the agent what the call is waiting for, before its answer.
    pub(super) fn note(&self, message: &str) {
        self.0
            .note(super::agents::WAITING, json!({ "message": message }));
    }

    /// Answers with what `task` gives, once it finishes.
    fn later(self, task: Task<Result<Output, Error>>) -> Task<Message> {
        task.map(move |result| Message::ToolAnswer(self.clone(), result))
    }
}

type Run = Box<dyn Fn(&mut Plaa, Value, Answer) -> Task<Message> + Send + Sync>;

pub(super) struct Tool {
    pub(super) name: &'static str,
    pub(super) title: &'static str,
    pub(super) kind: Kind,
    description: &'static str,
    schema: fn() -> Value,
    run: Run,
}

impl Tool {
    /// Runs the tool on `arguments`; it answers through `answer`.
    pub(super) fn start(&self, app: &mut Plaa, arguments: Value, answer: Answer) -> Task<Message> {
        (self.run)(app, arguments, answer)
    }
}

/// A tool whose input is `I`, read from the call's arguments.
fn tool<I: DeserializeOwned + JsonSchema + 'static>(
    name: &'static str,
    title: &'static str,
    kind: Kind,
    description: &'static str,
    run: fn(&mut Plaa, I, &Answer) -> Task<Message>,
) -> Tool {
    Tool {
        name,
        title,
        kind,
        description,
        schema: || {
            let mut schema = serde_json::to_value(schemars::schema_for!(I)).unwrap_or_default();
            // The tool's own name and description say what these would.
            if let Some(schema) = schema.as_object_mut() {
                schema.remove("$schema");
                schema.remove("title");
                schema.remove("description");
                // Some clients, such as ollama's, need properties even when
                // there are none.
                schema.entry("properties").or_insert_with(|| json!({}));
            }
            compact(&mut schema);
            schema
        },
        run: Box::new(
            move |app, arguments, answer| match serde_json::from_value::<I>(arguments) {
                Ok(input) => run(app, input, &answer),
                Err(error) => {
                    answer.send(Err(Error::new(
                        code::INVALID_PARAMS,
                        format!("{name}: {error}"),
                    )));
                    Task::none()
                }
            },
        ),
    }
}

/// Leaves out what a schema says only for a validator, since every agent
/// reads the schemas on every turn and plaa checks the input itself: the
/// number formats and their bounds, `null` beside optional fields' types,
/// shared definitions (written where they are used instead) and the
/// line breaks of descriptions.
fn compact(schema: &mut Value) {
    let definitions = schema
        .as_object_mut()
        .and_then(|schema| schema.remove("$defs"))
        .unwrap_or_default();
    schema
        .as_object_mut()
        .map(|schema| schema.remove("additionalProperties"));
    tidy(schema, &definitions);
}

fn tidy(value: &mut Value, definitions: &Value) {
    match value {
        Value::Object(object) => {
            // An optional field is one of a type or null: the type alone.
            if let Some(Value::Array(any)) = object.get("anyOf")
                && any.len() == 2
                && let Some(other) = any.iter().find(|one| one["type"] != "null")
                && any.iter().any(|one| one["type"] == "null")
            {
                let other = other.clone();
                object.remove("anyOf");
                if let Value::Object(other) = other {
                    for (key, value) in other {
                        object.entry(key).or_insert(value);
                    }
                }
            }
            if let Some(name) = object
                .get("$ref")
                .and_then(Value::as_str)
                .and_then(|reference| reference.strip_prefix("#/$defs/"))
                && let Some(Value::Object(definition)) = definitions.get(name).cloned()
            {
                object.remove("$ref");
                for (key, value) in definition {
                    object.entry(key).or_insert(value);
                }
            }
            if let Some(Value::Array(types)) = object.get("type")
                && types.len() == 2
                && let Some(only) = types.iter().find(|one| *one != "null").cloned()
            {
                object.insert("type".into(), only);
            }
            if let Some(format) = object.remove("format")
                && format
                    .as_str()
                    .is_some_and(|format| format.starts_with("uint"))
                && object.get("minimum") == Some(&json!(0))
            {
                object.remove("minimum");
            }
            if let Some(Value::String(description)) = object.get_mut("description") {
                *description = description.replace('\n', " ");
            }
            for (key, value) in object.iter_mut() {
                // Field names are not schemas, but their values are.
                if key != "enum" && key != "required" {
                    tidy(value, definitions);
                }
            }
        }
        Value::Array(values) => values.iter_mut().for_each(|value| tidy(value, definitions)),
        _ => {}
    }
}

/// The input of a tool that takes none.
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Nothing {}

/// The input of a tool that acts on a window and takes nothing else.
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct On {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Seek {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// Where to go, in seconds from the start.
    to: Option<f64>,
    /// How far to move instead, in seconds, back when negative.
    by: Option<f64>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Speed {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// How fast to play, 1 for normal speed, from 0.25 to 3.
    speed: f64,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Volume {
    /// The volume, from 0 to 100.
    volume: Option<f64>,
    /// Whether the sound is muted.
    muted: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Switch {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    on: bool,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Tracks {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
    /// The sound track to play, by its stream number from get_info.
    audio: Option<usize>,
    /// The subtitles to show, by their stream number from get_info.
    subtitles: Option<usize>,
    /// Hides the subtitles.
    subtitles_off: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Open {
    /// A file's full path, or the address of a stream or a web page.
    source: String,
}

/// Every tool, in the order agents see them.
static TOOLS: LazyLock<Vec<Tool>> = LazyLock::new(|| {
    vec![
        tool(
            "list_windows",
            "List windows",
            Kind::Read,
            "Lists plaa's open windows: each one's number, title, kind (player, start for a \
             window with nothing open, or recorder), what it plays (a file's path or an \
             address), whether it plays, is paused, has ended or failed, and whether it is \
             the one in front. Other tools take the number as `window`.",
            |app, _: Nothing, answer| {
                reply(
                    answer,
                    Ok(Output::Json(json!({ "windows": app.window_list() }))),
                )
            },
        ),
        tool(
            "get_status",
            "Get the status",
            Kind::Read,
            "Says how a window plays: playing, paused, ended or failed, the position and \
             length in seconds, the speed, the volume (0 to 100, the same in every window) \
             and whether it is muted, whether it loops, the sound and subtitle streams chosen, \
             and the item it plays of its list.",
            |app, on: On, answer| {
                let status = app.player_window(on.window).map(|(id, player)| {
                    let window = &app.windows[&id];
                    status(id, player, window.fullscreen)
                });
                reply(answer, status.map(Output::Json))
            },
        ),
        tool(
            "get_info",
            "Get file info",
            Kind::Read,
            "Describes what a window plays: its source, container, length, bit rate, title, \
             artist and album, its tags and chapters, its picture, sound and subtitle streams \
             with their stream numbers, codecs, languages and titles, and the items of its \
             list.",
            |app, on: On, answer| {
                let info = app
                    .player_window(on.window)
                    .and_then(|(id, player)| info(id, player));
                reply(answer, info.map(Output::Json))
            },
        ),
        tool(
            "get_frame",
            "Get the picture",
            Kind::Read,
            "Gives the picture a window shows now, as a PNG at most 1280 pixels wide or tall, \
             upright and as edited, without the subtitles or plaa's controls.",
            |app, on: On, answer| {
                let frame = app.player_window(on.window).and_then(|(id, player)| {
                    let (width, height, rgba) = player.player.snapshot().ok_or_else(|| {
                        Error::new(
                            code::INVALID_PARAMS,
                            format!("Window {} shows no picture yet.", number(id)),
                        )
                    })?;
                    let own = player.picture().map_or(0, |track| track.quarter_turns);
                    let shown = plaa_media::Picture {
                        width,
                        height,
                        rgba,
                    }
                    .transformed(player.transform().after_turns(own));
                    let note = format!(
                        "Window {} at {:.1} s, {} by {} pixels as decoded.",
                        number(id),
                        player.status.position.as_secs_f64(),
                        shown.width,
                        shown.height,
                    );
                    Ok((shown, note))
                });
                match frame {
                    Ok((shown, note)) => answer
                        .clone()
                        .later(Task::perform(encode(shown, FRAME_SIDE), move |png| {
                            png.map(|png| Output::Png(png, note.clone()))
                        })),
                    Err(error) => reply(answer, Err(error)),
                }
            },
        ),
        tool(
            "play",
            "Play",
            Kind::Play,
            "Plays a window, from where it is; one that has ended starts over.",
            |app, on: On, answer| {
                playing(app, on.window, answer, true);
                Task::none()
            },
        ),
        tool(
            "pause",
            "Pause",
            Kind::Play,
            "Pauses a window.",
            |app, on: On, answer| {
                playing(app, on.window, answer, false);
                Task::none()
            },
        ),
        tool(
            "seek",
            "Seek",
            Kind::Play,
            "Moves a window's position: to `to` seconds from the start, or by `by` seconds, \
             back when negative.",
            |app, seek: Seek, answer| {
                let (id, player) = match app.player_window(seek.window) {
                    Ok(found) => found,
                    Err(error) => return reply(answer, Err(error)),
                };
                if !player.info.as_ref().is_some_and(|info| info.seekable) {
                    return reply(
                        answer,
                        Err(Error::new(
                            code::INVALID_PARAMS,
                            format!("Window {} plays something that cannot seek.", number(id)),
                        )),
                    );
                }
                let control = match (seek.to, seek.by) {
                    (Some(to), None) if to.is_finite() => Control::SeekTo(to.max(0.0)),
                    (None, Some(by)) if by.is_finite() => Control::SkipBy(by),
                    _ => {
                        return reply(
                            answer,
                            Err(Error::new(
                                code::INVALID_PARAMS,
                                "seek: give `to` or `by`, a number of seconds",
                            )),
                        );
                    }
                };
                answer.send(Ok(Output::Text(format!(
                    "Seeking in window {}.",
                    number(id)
                ))));
                app.control(id, control)
            },
        ),
        tool(
            "set_speed",
            "Set the speed",
            Kind::Play,
            "Sets how fast a window plays, 1 for normal speed, from 0.25 to 3.",
            |app, speed: Speed, answer| {
                let (lowest, highest) = (SPEEDS[0], SPEEDS[SPEEDS.len() - 1]);
                if !(lowest..=highest).contains(&speed.speed) {
                    return reply(
                        answer,
                        Err(Error::new(
                            code::INVALID_PARAMS,
                            format!("set_speed: the speed is from {lowest} to {highest}"),
                        )),
                    );
                }
                match app.player_window(speed.window) {
                    Ok((id, _)) => {
                        answer.send(Ok(Output::Text(format!(
                            "Window {} plays at {}x.",
                            number(id),
                            speed.speed
                        ))));
                        app.control(id, Control::Speed(speed.speed))
                    }
                    Err(error) => reply(answer, Err(error)),
                }
            },
        ),
        tool(
            "set_volume",
            "Set the volume",
            Kind::Play,
            "Sets the volume, from 0 to 100, or mutes and unmutes, in every window, as \
             plaa's volume is one for all.",
            |app, volume: Volume, answer| {
                if volume
                    .volume
                    .is_some_and(|level| !(0.0..=100.0).contains(&level))
                {
                    return reply(
                        answer,
                        Err(Error::new(
                            code::INVALID_PARAMS,
                            "set_volume: the volume is from 0 to 100",
                        )),
                    );
                }
                let Ok(id) = app.tool_window(None) else {
                    return reply(answer, Err(no_window()));
                };
                let mut tasks = Vec::new();
                if let Some(level) = volume.volume {
                    tasks.push(app.control(id, Control::Volume((level / 100.0) as f32)));
                    tasks.push(app.control(id, Control::VolumeReleased));
                }
                if volume
                    .muted
                    .is_some_and(|muted| muted != app.settings.muted)
                {
                    tasks.push(app.control(id, Control::ToggleMute));
                }
                answer.send(Ok(Output::Json(json!({
                    "volume": (app.settings.volume * 100.0).round(),
                    "muted": app.settings.muted,
                }))));
                Task::batch(tasks)
            },
        ),
        tool(
            "set_loop",
            "Loop",
            Kind::Play,
            "Turns looping on or off: a window that loops plays its file, or its list, again \
             from the start when it ends.",
            |app, switch: Switch, answer| match app.player_window(switch.window) {
                Ok((id, player)) => {
                    let toggle = player.looping != switch.on;
                    answer.send(Ok(Output::Text(format!(
                        "Window {} {}.",
                        number(id),
                        if switch.on { "loops" } else { "does not loop" }
                    ))));
                    if toggle {
                        app.control(id, Control::ToggleLoop)
                    } else {
                        Task::none()
                    }
                }
                Err(error) => reply(answer, Err(error)),
            },
        ),
        tool(
            "choose_tracks",
            "Choose tracks",
            Kind::Play,
            "Chooses the sound track a window plays and the subtitles it shows, by their \
             stream numbers from get_info, or hides the subtitles.",
            |app, tracks: Tracks, answer| {
                let (id, player) = match app.player_window(tracks.window) {
                    Ok(found) => found,
                    Err(error) => return reply(answer, Err(error)),
                };
                let info = player.info.clone();
                let has = |index: usize, audio: bool| {
                    info.as_ref().is_some_and(|info| {
                        if audio {
                            info.audio.iter().any(|track| track.track.index == index)
                        } else {
                            info.subtitles
                                .iter()
                                .any(|track| track.track.index == index)
                        }
                    })
                };
                if let Some(index) = tracks.audio
                    && !has(index, true)
                {
                    return reply(answer, Err(no_stream(id, index, "sound track")));
                }
                if let Some(index) = tracks.subtitles
                    && !has(index, false)
                {
                    return reply(answer, Err(no_stream(id, index, "subtitles")));
                }
                let mut tasks = Vec::new();
                if let Some(index) = tracks.audio {
                    tasks.push(app.control(id, Control::Audio(Some(index))));
                }
                if let Some(index) = tracks.subtitles {
                    tasks.push(app.control(id, Control::Subtitles(Some(index))));
                } else if tracks.subtitles_off == Some(true) {
                    tasks.push(app.control(id, Control::Subtitles(None)));
                }
                answer.send(Ok(Output::Text(format!(
                    "Chose the tracks of window {}.",
                    number(id)
                ))));
                Task::batch(tasks)
            },
        ),
        tool(
            "next_item",
            "Next item",
            Kind::Play,
            "Plays the next item of a window's list, as of a folder's files.",
            |app, on: On, answer| match app.player_window(on.window) {
                Ok((id, player)) if player.playlist.next().is_some() => {
                    answer.send(Ok(Output::Text(format!(
                        "Window {} plays its next item.",
                        number(id)
                    ))));
                    app.next(id)
                }
                Ok((id, _)) => reply(
                    answer,
                    Err(Error::new(
                        code::INVALID_PARAMS,
                        format!("Window {} plays its last item.", number(id)),
                    )),
                ),
                Err(error) => reply(answer, Err(error)),
            },
        ),
        tool(
            "previous_item",
            "Previous item",
            Kind::Play,
            "Plays the item before in a window's list; a few seconds into an item, starts it \
             over instead, as the Previous button does.",
            |app, on: On, answer| match app.player_window(on.window) {
                Ok((id, _)) => {
                    answer.send(Ok(Output::Text(format!(
                        "Window {} goes back.",
                        number(id)
                    ))));
                    app.previous(id)
                }
                Err(error) => reply(answer, Err(error)),
            },
        ),
        tool(
            "set_fullscreen",
            "Full screen",
            Kind::Play,
            "Puts a window in full screen, or takes it out.",
            |app, switch: Switch, answer| match app.tool_window(switch.window) {
                Ok(id) => {
                    answer.send(Ok(Output::Text(format!(
                        "Window {} is {}.",
                        number(id),
                        if switch.on {
                            "in full screen"
                        } else {
                            "out of full screen"
                        }
                    ))));
                    app.set_fullscreen(id, switch.on)
                }
                Err(error) => reply(answer, Err(error)),
            },
        ),
        tool(
            "open",
            "Open",
            Kind::Open,
            "Opens a file, by its full path, or a stream or web page, by its address, in a \
             window of its own, and plays it; a file that plays already brings its window to \
             the front. Gives the window's number.",
            |app, open: Open, answer| {
                let source = Source::parse(open.source.trim());
                if let Source::File(path) = &source {
                    if !path.is_absolute() {
                        return reply(
                            answer,
                            Err(Error::new(
                                code::INVALID_PARAMS,
                                "open: give the file's full path",
                            )),
                        );
                    }
                    if !path.exists() {
                        return reply(
                            answer,
                            Err(Error::new(
                                code::INVALID_PARAMS,
                                format!("open: there is no file {}", path.display()),
                            )),
                        );
                    }
                }
                let before: Vec<window::Id> = app.windows.keys().copied().collect();
                let into = app.lone_start();
                let task = app.open_sources(vec![source.clone()], into);
                let opened = into
                    .or_else(|| app.windows.keys().copied().find(|id| !before.contains(id)))
                    .or_else(|| app.window_playing(&super::canonical(source)));
                answer.send(match opened {
                    Some(id) => Ok(Output::Json(json!({ "window": number(id) }))),
                    None => Err(Error::new(
                        code::INTERNAL_ERROR,
                        "open: plaa could not open it",
                    )),
                });
                task
            },
        ),
        tool(
            "close_window",
            "Close a window",
            Kind::Open,
            "Closes a window, which stops what it plays. A window with edits not yet exported \
             asks the user first.",
            |app, on: On, answer| match app.tool_window(on.window) {
                Ok(id) => {
                    if app.ask_before_closing(id, super::export::Closing::Window) {
                        answer.send(Ok(Output::Text(format!(
                            "Window {} has edits not exported: plaa asks the user whether to \
                             close it.",
                            number(id)
                        ))));
                        Task::none()
                    } else {
                        answer.send(Ok(Output::Text(format!("Closed window {}.", number(id)))));
                        window::close(id)
                    }
                }
                Err(error) => reply(answer, Err(error)),
            },
        ),
        tool(
            "focus_window",
            "Focus a window",
            Kind::Open,
            "Brings a plaa window to the front and gives it the keyboard focus. Some \
             desktops, such as Wayland ones, only flag the window for the user instead.",
            |app, on: On, answer| match app.tool_window(on.window) {
                Ok(id) => {
                    answer.send(Ok(Output::Text(format!(
                        "Asked to bring window {} to the front. Some desktops flag the \
                         window for the user instead.",
                        number(id)
                    ))));
                    bring_forward(id)
                }
                Err(error) => reply(answer, Err(error)),
            },
        ),
    ]
});

/// The longest side of the pictures get_frame gives.
const FRAME_SIDE: u32 = 1280;

/// Answers `answer` with `result` at once.
fn reply(answer: &Answer, result: Result<Output, Error>) -> Task<Message> {
    answer.send(result);
    Task::none()
}

fn no_window() -> Error {
    Error::new(code::INVALID_PARAMS, "plaa has no window open")
}

fn no_stream(id: window::Id, index: usize, what: &str) -> Error {
    Error::new(
        code::INVALID_PARAMS,
        format!(
            "Window {} has no {what} at stream {index}; see get_info.",
            number(id)
        ),
    )
}

/// Plays or pauses window `window`'s player, answering `answer`.
fn playing(app: &mut Plaa, window: Option<u64>, answer: &Answer, play: bool) {
    let id = match app.player_window(window) {
        Ok((id, _)) => id,
        Err(error) => {
            answer.send(Err(error));
            return;
        }
    };
    app.media_window = Some(id);
    if let Some(player) = app.player_mut(id) {
        if play {
            player.player.play();
        } else {
            player.player.pause();
        }
        player.refresh();
    }
    answer.send(Ok(Output::Text(format!(
        "Window {} {}.",
        number(id),
        if play { "plays" } else { "is paused" }
    ))));
}

/// How window `id`'s player stands, for get_status.
fn status(id: window::Id, player: &PlayerWindow, fullscreen: bool) -> Value {
    let status = &player.status;
    let state = if player.error.is_some() {
        "failed"
    } else if player.info.is_none() {
        "opening"
    } else if status.ended {
        "ended"
    } else if status.paused {
        "paused"
    } else {
        "playing"
    };
    json!({
        "window": number(id),
        "title": player.title(),
        "state": state,
        "error": player.error,
        "position": round(status.position.as_secs_f64()),
        "duration": status.duration.map(|duration| round(duration.as_secs_f64())),
        "speed": status.speed,
        "buffering": status.buffering,
        "volume": (player.volume * 100.0).round(),
        "muted": player.muted,
        "looping": player.looping,
        "fullscreen": fullscreen,
        "audio_stream": status.audio,
        "subtitle_stream": status.subtitles,
        "item": player.playlist.current() + 1,
        "items": player.playlist.items().len(),
    })
}

/// What window `id` plays, for get_info.
fn info(id: window::Id, player: &PlayerWindow) -> Result<Value, Error> {
    let Some(info) = player.info.as_deref() else {
        return Err(Error::new(
            code::INVALID_PARAMS,
            match &player.error {
                Some(error) => format!("Window {} could not play: {error}", number(id)),
                None => format!("Window {} is still opening its file.", number(id)),
            },
        ));
    };
    let track = |track: &plaa_media::info::Track| {
        json!({
            "stream": track.index,
            "codec": track.codec,
            "codec_name": track.codec_name,
            "language": track.language,
            "title": track.title,
            "default": track.default,
            "forced": track.forced,
            "bit_rate": track.bit_rate,
        })
    };
    let merged = |mut base: Value, more: Value| {
        if let (Some(base), Value::Object(more)) = (base.as_object_mut(), more) {
            base.extend(more);
        }
        base
    };
    let video: Vec<Value> = info
        .video
        .iter()
        .map(|video| {
            merged(
                track(&video.track),
                json!({
                    "width": video.width,
                    "height": video.height,
                    "frame_rate": video.frame_rate,
                    "cover": video.cover,
                    "pixel_format": video.pixel_format,
                    "hdr": video.hdr,
                }),
            )
        })
        .collect();
    let audio: Vec<Value> = info
        .audio
        .iter()
        .map(|audio| {
            merged(
                track(&audio.track),
                json!({
                    "sample_rate": audio.sample_rate,
                    "channels": audio.channels,
                    "layout": audio.layout,
                    "bits": audio.bits,
                }),
            )
        })
        .collect();
    let subtitles: Vec<Value> = info
        .subtitles
        .iter()
        .map(|subtitles| {
            merged(
                track(&subtitles.track),
                json!({
                    "text": subtitles.text,
                    "file": subtitles.file.as_ref().map(|file| file.display().to_string()),
                }),
            )
        })
        .collect();
    let chapters: Vec<Value> = info
        .chapters
        .iter()
        .map(|chapter| {
            json!({
                "title": chapter.title,
                "start": round(chapter.start.as_secs_f64()),
                "end": round(chapter.end.as_secs_f64()),
            })
        })
        .collect();
    let items: Vec<String> = player.playlist.items().iter().map(source_text).collect();
    Ok(json!({
        "window": number(id),
        "source": source_text(player.source()),
        "format": info.format,
        "duration": info.duration.map(|duration| round(duration.as_secs_f64())),
        "bit_rate": info.bit_rate,
        "seekable": info.seekable,
        "title": info.title,
        "artist": info.artist,
        "album": info.album,
        "tags": info.tags.iter().map(|(key, value)| json!([key, value])).collect::<Vec<_>>(),
        "chapters": chapters,
        "video": video,
        "audio": audio,
        "subtitles": subtitles,
        "items": items,
        "item": player.playlist.current() + 1,
    }))
}

/// Seconds to the millisecond, as agents need no more.
fn round(seconds: f64) -> f64 {
    (seconds * 1000.0).round() / 1000.0
}

/// A file's path or a stream's address.
fn source_text(source: &Source) -> String {
    match source {
        Source::File(path) => path.display().to_string(),
        Source::Url(url) => url.clone(),
    }
}

/// `picture` as a PNG, shrunk to at most `side` pixels wide and tall, on a
/// thread of its own.
async fn encode(picture: plaa_media::Picture, side: u32) -> Result<Vec<u8>, Error> {
    let (sender, receiver) = iced::futures::channel::oneshot::channel();
    std::thread::Builder::new()
        .name("plaa-frame".into())
        .spawn(move || {
            let _ = sender.send(png(&shrunk(picture, side)));
        })
        .map_err(|error| Error::new(code::INTERNAL_ERROR, error.to_string()))?;
    receiver
        .await
        .unwrap_or_else(|_| Err(Error::new(code::INTERNAL_ERROR, "the picture was lost")))
}

/// `picture`, averaged down to fit `side` pixels each way.
fn shrunk(picture: plaa_media::Picture, side: u32) -> plaa_media::Picture {
    let longest = picture.width.max(picture.height);
    if longest <= side || picture.width == 0 || picture.height == 0 {
        return picture;
    }
    let width = (u64::from(picture.width) * u64::from(side) / u64::from(longest)).max(1) as u32;
    let height = (u64::from(picture.height) * u64::from(side) / u64::from(longest)).max(1) as u32;
    let mut rgba = vec![0u8; (width * height * 4) as usize];
    for row in 0..height {
        let top = row * picture.height / height;
        let bottom = ((row + 1) * picture.height / height).max(top + 1);
        for column in 0..width {
            let left = column * picture.width / width;
            let right = ((column + 1) * picture.width / width).max(left + 1);
            let mut sums = [0u32; 4];
            for source_row in top..bottom {
                let start = ((source_row * picture.width + left) * 4) as usize;
                let end = ((source_row * picture.width + right) * 4) as usize;
                for pixel in picture.rgba[start..end].as_chunks::<4>().0 {
                    for (sum, value) in sums.iter_mut().zip(pixel) {
                        *sum += u32::from(*value);
                    }
                }
            }
            let count = (bottom - top) * (right - left);
            let at = ((row * width + column) * 4) as usize;
            for (channel, sum) in sums.iter().enumerate() {
                rgba[at + channel] = (sum / count) as u8;
            }
        }
    }
    plaa_media::Picture {
        width,
        height,
        rgba,
    }
}

fn png(picture: &plaa_media::Picture) -> Result<Vec<u8>, Error> {
    let failed = |error: png::EncodingError| Error::new(code::INTERNAL_ERROR, error.to_string());
    let mut bytes = Vec::new();
    let mut encoder = png::Encoder::new(&mut bytes, picture.width, picture.height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(failed)?;
    writer.write_image_data(&picture.rgba).map_err(failed)?;
    writer.finish().map_err(failed)?;
    Ok(bytes)
}

/// Waits `time` on a thread of its own.
pub(super) async fn after(time: Duration) {
    let (sender, receiver) = iced::futures::channel::oneshot::channel();
    std::thread::spawn(move || {
        std::thread::sleep(time);
        let _ = sender.send(());
    });
    let _ = receiver.await;
}

/// The tools as MCP lists them; those whose kind `ask` makes plaa ask
/// about say so, so agents know a call may wait for the user.
pub(super) fn list(ask: &AskBefore) -> Value {
    TOOLS
        .iter()
        .map(|tool| {
            let description = if tool.kind.asks(ask) {
                format!(
                    "{} plaa asks the user to allow it first, in the window it acts on, so the \
                     call waits for their answer: tell them to look at plaa.",
                    tool.description
                )
            } else {
                tool.description.to_owned()
            };
            json!({
                "name": tool.name,
                "title": tool.title,
                "description": description,
                "inputSchema": (tool.schema)(),
                "annotations": annotations(tool.kind),
            })
        })
        .collect()
}

/// MCP's hints for clients: what only reads.
fn annotations(kind: Kind) -> Value {
    match kind {
        Kind::Read => json!({ "readOnlyHint": true }),
        Kind::Play | Kind::Open => json!({ "readOnlyHint": false, "destructiveHint": false }),
    }
}

/// The tool named `name`.
pub(super) fn find(name: &str) -> Option<&'static Tool> {
    TOOLS.iter().find(|tool| tool.name == name)
}

/// Brings window `id` to the front. Wayland lets a window ask only for
/// attention, which some compositors answer by focusing it.
fn bring_forward(id: window::Id) -> Task<Message> {
    let focus = window::gain_focus(id);
    if cfg!(target_os = "linux") {
        Task::batch([
            focus,
            window::request_user_attention(id, Some(window::UserAttention::Informational)),
        ])
    } else {
        focus
    }
}

/// The number agents know window `id` by.
pub(super) fn number(id: window::Id) -> u64 {
    id.to_string().parse().unwrap_or_default()
}

impl Plaa {
    /// Runs the tool a `tools/call` names, which answers `call`, once
    /// the user allows it when its kind asks first. `agent` is the name
    /// the user knows the agent by.
    pub(super) fn run_tool(&mut self, call: Call, agent: String) -> Task<Message> {
        let name = call.params["name"].as_str().unwrap_or_default().to_owned();
        let arguments = match &call.params["arguments"] {
            Value::Null => json!({}),
            arguments => arguments.clone(),
        };
        match find(&name) {
            Some(tool) if tool.kind.asks(&self.settings.ask_before) => {
                // A window that cannot be found is the tool's to report.
                match self.tool_window(arguments["window"].as_u64()) {
                    Ok(id) => self.ask_to_run(id, agent, tool, arguments, Answer(call)),
                    Err(_) => tool.start(self, arguments, Answer(call)),
                }
            }
            Some(tool) => tool.start(self, arguments, Answer(call)),
            None => {
                call.reply(Err(Error::new(
                    code::INVALID_PARAMS,
                    format!("plaa has no tool {name}"),
                )));
                Task::none()
            }
        }
    }

    /// The window a tool acts on: the one numbered `window`, else the one
    /// in front, else the one last played.
    pub(super) fn tool_window(&self, window: Option<u64>) -> Result<window::Id, Error> {
        let found = match window {
            Some(wanted) => self
                .windows
                .keys()
                .copied()
                .find(|id| number(*id) == wanted),
            None => self
                .focused
                .filter(|id| self.windows.contains_key(id))
                .or_else(|| self.media_window())
                .or_else(|| self.windows.keys().next().copied()),
        };
        found.ok_or_else(|| match window {
            Some(wanted) => Error::new(
                code::INVALID_PARAMS,
                format!("plaa has no window {wanted}; see list_windows"),
            ),
            None => no_window(),
        })
    }

    /// The window a tool acts on, as [`Plaa::tool_window`] finds it, and
    /// its player; a window without one is an error.
    fn player_window(&self, window: Option<u64>) -> Result<(window::Id, &PlayerWindow), Error> {
        let id = match window {
            Some(_) => self.tool_window(window)?,
            // The one in front, unless it plays nothing and another does.
            None => {
                let id = self.tool_window(None)?;
                if self.windows[&id].player().is_some() {
                    id
                } else {
                    self.media_window().unwrap_or(id)
                }
            }
        };
        match self.windows.get(&id).and_then(|window| window.player()) {
            Some(player) => Ok((id, player)),
            None => Err(Error::new(
                code::INVALID_PARAMS,
                format!("Window {} plays nothing; see list_windows.", number(id)),
            )),
        }
    }

    fn window_list(&self) -> Vec<Value> {
        self.windows
            .iter()
            .map(|(id, window)| {
                let (kind, source, state) = match &window.content {
                    Content::Start => ("start", None, None),
                    Content::Recorder(_) => ("recorder", None, None),
                    Content::Player(player) => {
                        let state = status(*id, player, window.fullscreen)["state"].clone();
                        ("player", Some(source_text(player.source())), Some(state))
                    }
                };
                json!({
                    "window": number(*id),
                    "title": self.title(*id),
                    "kind": kind,
                    "source": source,
                    "state": state,
                    "focused": self.focused == Some(*id),
                    "fullscreen": window.fullscreen,
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schemas_leave_out_what_only_a_validator_needs() {
        for tool in TOOLS.iter() {
            let schema = (tool.schema)();
            let text = schema.to_string();
            for left_out in [
                "\"$defs\"",
                "\"$ref\"",
                "\"anyOf\"",
                "\"format\"",
                "\"null\"",
                "\\n",
            ] {
                assert!(
                    !text.contains(left_out),
                    "{}: {left_out} in {text}",
                    tool.name
                );
            }
        }
        let seek = (find("seek").expect("seek").schema)();
        assert_eq!(seek["properties"]["to"]["type"], "number");
        assert_eq!(seek["properties"]["window"]["type"], "integer");
    }

    #[test]
    fn names_are_unique_and_snake_case() {
        let mut names: Vec<_> = TOOLS.iter().map(|tool| tool.name).collect();
        assert!(names.iter().all(|name| {
            !name.is_empty()
                && name
                    .chars()
                    .all(|character| character.is_ascii_lowercase() || character == '_')
        }));
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), TOOLS.len());
    }

    #[test]
    fn every_schema_is_an_object() {
        for tool in TOOLS.iter() {
            let schema = (tool.schema)();
            assert_eq!(schema["type"], "object", "{}: {schema}", tool.name);
            assert!(schema["properties"].is_object(), "{}: {schema}", tool.name);
            assert!(schema.get("$schema").is_none(), "{}", tool.name);
        }
    }

    #[test]
    fn window_input_is_described() {
        let schema = (find("get_status").unwrap().schema)();
        let window = &schema["properties"]["window"];
        assert!(
            window["description"]
                .as_str()
                .is_some_and(|text| text.contains("list_windows")),
            "{schema}"
        );
    }

    #[test]
    fn list_has_what_mcp_needs() {
        let asking = AskBefore {
            opening: true,
            ..AskBefore::default()
        };
        let list = list(&asking);
        for tool in list.as_array().unwrap() {
            for field in ["name", "title", "description", "inputSchema", "annotations"] {
                assert!(tool.get(field).is_some(), "{field} in {tool}");
            }
        }
        assert_eq!(list[0]["annotations"]["readOnlyHint"], true);
        let described = |name: &str| {
            list.as_array()
                .unwrap()
                .iter()
                .find(|tool| tool["name"] == name)
                .unwrap()["description"]
                .as_str()
                .unwrap()
                .to_owned()
        };
        assert!(described("open").contains("asks the user"));
        assert!(!described("list_windows").contains("asks the user"));
    }

    #[test]
    fn outputs_become_mcp_results() {
        let json = Output::Json(json!({ "a": 1 })).into_result();
        assert_eq!(json["structuredContent"]["a"], 1);
        assert_eq!(json["content"][0]["text"], r#"{"a":1}"#);
        let png = Output::Png(vec![1, 2, 3], "note".to_owned()).into_result();
        assert_eq!(png["content"][0]["type"], "image");
        assert_eq!(png["content"][0]["data"], "AQID");
        assert_eq!(png["content"][0]["mimeType"], "image/png");
        assert_eq!(png["content"][1]["text"], "note");
    }

    #[test]
    fn pictures_shrink_to_fit() {
        let picture = plaa_media::Picture {
            width: 4,
            height: 2,
            rgba: [[255, 0, 0, 255], [0, 0, 255, 255]]
                .repeat(4)
                .into_iter()
                .flatten()
                .collect(),
        };
        let small = shrunk(picture, 2);
        assert_eq!((small.width, small.height), (2, 1));
        // Each pixel is the average of the two it takes the place of.
        assert_eq!(&small.rgba[..4], &[127, 0, 127, 255]);
        let png = png(&small).unwrap();
        assert_eq!(&png[1..4], b"PNG");
    }
}
