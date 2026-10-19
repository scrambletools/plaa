//! Keyboard shortcuts: the defaults, and the `[keys]` table of the
//! settings file that changes them.

use std::sync::RwLock;

use iced::keyboard::{self, Key, Modifiers, key::Named};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Open,
    OpenAddress,
    CloseWindow,
    Quit,
    Settings,
    Info,
    PlayPause,
    Back,
    Forward,
    BackMore,
    ForwardMore,
    PreviousFrame,
    NextFrame,
    VolumeUp,
    VolumeDown,
    Mute,
    FullScreen,
    Faster,
    Slower,
    NormalSpeed,
    Loop,
    Screenshot,
    Subtitles,
    AudioTrack,
    ToStart,
    Previous,
    Next,
    Playlist,
    PreviousChapter,
    NextChapter,
    AbLoop,
    PictureInPicture,
}

/// Every action, its name in the settings file and its default
/// shortcuts. `Ctrl` is the command key: ⌘ on macOS.
const ACTIONS: &[(Action, &str, &[&str])] = &[
    (Action::Open, "open", &["Ctrl+O"]),
    (Action::OpenAddress, "open-address", &["Ctrl+U"]),
    (Action::CloseWindow, "close-window", &["Ctrl+W"]),
    (Action::Quit, "quit", &["Ctrl+Q"]),
    (Action::Settings, "settings", &["Ctrl+,"]),
    (Action::Info, "info", &["Ctrl+I"]),
    (Action::PlayPause, "play-pause", &["Space", "K"]),
    (Action::Back, "back", &["Left"]),
    (Action::Forward, "forward", &["Right"]),
    (Action::BackMore, "back-more", &["J", "Shift+Left"]),
    (Action::ForwardMore, "forward-more", &["L", "Shift+Right"]),
    (Action::PreviousFrame, "previous-frame", &[","]),
    (Action::NextFrame, "next-frame", &["."]),
    (Action::VolumeUp, "volume-up", &["Up"]),
    (Action::VolumeDown, "volume-down", &["Down"]),
    (Action::Mute, "mute", &["M"]),
    (Action::FullScreen, "full-screen", &["F", "F11"]),
    (Action::Faster, "faster", &["]"]),
    (Action::Slower, "slower", &["["]),
    (Action::NormalSpeed, "normal-speed", &["Backspace"]),
    (Action::Loop, "loop", &["Ctrl+L"]),
    (Action::Screenshot, "screenshot", &["S"]),
    (Action::Subtitles, "subtitles", &["C"]),
    (Action::AudioTrack, "audio-track", &["A"]),
    (Action::ToStart, "to-start", &["Home"]),
    (Action::Previous, "previous", &["P", "Ctrl+Left"]),
    (Action::Next, "next", &["N", "Ctrl+Right"]),
    (Action::Playlist, "playlist", &["Ctrl+P"]),
    (Action::PreviousChapter, "previous-chapter", &["PageUp"]),
    (Action::NextChapter, "next-chapter", &["PageDown"]),
    (Action::AbLoop, "ab-loop", &["B"]),
    (
        Action::PictureInPicture,
        "picture-in-picture",
        &["Ctrl+Shift+P"],
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shortcut {
    command: bool,
    alt: bool,
    shift: bool,
    key: KeyName,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum KeyName {
    Character(String),
    Named(Named),
}

const NAMED: &[(&str, Named)] = &[
    ("Esc", Named::Escape),
    ("Enter", Named::Enter),
    ("Tab", Named::Tab),
    ("Space", Named::Space),
    ("Backspace", Named::Backspace),
    ("Delete", Named::Delete),
    ("Insert", Named::Insert),
    ("Home", Named::Home),
    ("End", Named::End),
    ("PageUp", Named::PageUp),
    ("PageDown", Named::PageDown),
    ("Up", Named::ArrowUp),
    ("Down", Named::ArrowDown),
    ("Left", Named::ArrowLeft),
    ("Right", Named::ArrowRight),
    ("F1", Named::F1),
    ("F2", Named::F2),
    ("F3", Named::F3),
    ("F4", Named::F4),
    ("F5", Named::F5),
    ("F6", Named::F6),
    ("F7", Named::F7),
    ("F8", Named::F8),
    ("F9", Named::F9),
    ("F10", Named::F10),
    ("F11", Named::F11),
    ("F12", Named::F12),
    ("MediaPlayPause", Named::MediaPlayPause),
    ("MediaStop", Named::MediaStop),
    ("MediaNext", Named::MediaTrackNext),
    ("MediaPrevious", Named::MediaTrackPrevious),
];

/// Reads a shortcut such as "Ctrl+Shift+O" or "Space".
pub fn parse(text: &str) -> Result<Shortcut, String> {
    let mut shortcut = Shortcut {
        command: false,
        alt: false,
        shift: false,
        key: KeyName::Character(String::new()),
    };
    let parts: Vec<&str> = text.split('+').map(str::trim).collect();
    // "Ctrl++" is Ctrl and the plus key.
    let (modifiers, key) = match parts.as_slice() {
        [.., "", ""] => (&parts[..parts.len() - 2], "+"),
        [rest @ .., key] => (rest, *key),
        [] => return Err(format!("“{text}” names no key")),
    };
    for modifier in modifiers {
        match modifier.to_ascii_lowercase().as_str() {
            "ctrl" | "cmd" | "command" => shortcut.command = true,
            "alt" | "option" => shortcut.alt = true,
            "shift" => shortcut.shift = true,
            other => return Err(format!("“{other}” in “{text}” is not Ctrl, Alt or Shift")),
        }
    }
    shortcut.key = if let Some((_, named)) = NAMED
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(key))
    {
        KeyName::Named(*named)
    } else if key.chars().count() == 1 {
        KeyName::Character(key.to_lowercase())
    } else {
        return Err(format!("“{key}” in “{text}” is not a key plaa knows"));
    };
    Ok(shortcut)
}

impl Shortcut {
    fn matches(&self, key: &Key, modifiers: Modifiers) -> bool {
        if self.command != modifiers.command() || self.alt != modifiers.alt() {
            return false;
        }
        match (&self.key, key) {
            (KeyName::Named(named), Key::Named(pressed)) => {
                named == pressed && self.shift == modifiers.shift()
            }
            (KeyName::Character(character), Key::Character(pressed)) => {
                // Shift may be what types a character such as "]" or "?".
                let pressed = pressed.to_lowercase();
                *character == pressed && (self.shift == modifiers.shift() || !is_letter(character))
            }
            _ => false,
        }
    }

    fn label(&self) -> String {
        let mut label = String::new();
        if self.command {
            label.push_str(if cfg!(target_os = "macos") {
                "⌘"
            } else {
                "Ctrl+"
            });
        }
        if self.alt {
            label.push_str(if cfg!(target_os = "macos") {
                "⌥"
            } else {
                "Alt+"
            });
        }
        if self.shift {
            label.push_str(if cfg!(target_os = "macos") {
                "⇧"
            } else {
                "Shift+"
            });
        }
        match &self.key {
            KeyName::Character(character) => label.push_str(&character.to_uppercase()),
            KeyName::Named(named) => label.push_str(
                NAMED
                    .iter()
                    .find(|(_, candidate)| candidate == named)
                    .map_or("?", |(name, _)| name),
            ),
        }
        label
    }
}

/// What the macOS menu bar reads to show a shortcut on its items.
#[cfg(target_os = "macos")]
impl Shortcut {
    pub fn command(&self) -> bool {
        self.command
    }

    pub fn alt(&self) -> bool {
        self.alt
    }

    pub fn shift(&self) -> bool {
        self.shift
    }

    pub fn character(&self) -> Option<&str> {
        match &self.key {
            KeyName::Character(character) => Some(character),
            KeyName::Named(_) => None,
        }
    }

    pub fn named(&self) -> Option<Named> {
        match &self.key {
            KeyName::Named(named) => Some(*named),
            KeyName::Character(_) => None,
        }
    }
}

fn is_letter(text: &str) -> bool {
    text.chars().all(char::is_alphabetic)
}

static BINDINGS: RwLock<Vec<(Shortcut, Action)>> = RwLock::new(Vec::new());

/// Sets the shortcuts from the defaults and the settings' `[keys]` table.
/// Returns what in the table could not be read; the rest applies.
pub fn configure(keys: &toml::Table) -> Vec<String> {
    let mut problems = Vec::new();
    let system = if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(windows) {
        "windows"
    } else {
        "linux"
    };
    // Each action's shortcuts, and whether the settings chose them.
    let mut chosen: Vec<(Action, Vec<String>, bool)> = ACTIONS
        .iter()
        .map(|(action, _, defaults)| {
            (
                *action,
                defaults.iter().map(|text| (*text).to_owned()).collect(),
                false,
            )
        })
        .collect();
    let shared = keys.iter().filter(|(_, value)| !value.is_table());
    let own = keys
        .get(system)
        .and_then(toml::Value::as_table)
        .into_iter()
        .flat_map(|table| table.iter());
    for (name, value) in shared.chain(own) {
        if ["linux", "windows", "macos"].contains(&name.as_str()) {
            continue;
        }
        let Some(slot) = ACTIONS
            .iter()
            .position(|(_, action_name, _)| action_name == name)
        else {
            problems.push(format!("[keys]: “{name}” is not an action plaa has"));
            continue;
        };
        let texts: Vec<String> = match value {
            toml::Value::String(text) => vec![text.clone()],
            toml::Value::Array(items) => items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_owned))
                .collect(),
            _ => {
                problems.push(format!(
                    "[keys]: “{name}” needs a shortcut or a list of them"
                ));
                continue;
            }
        };
        chosen[slot].1 = texts;
        chosen[slot].2 = true;
    }
    // The settings' shortcuts last, so a key they give one action is
    // taken from any default it had.
    chosen.sort_by_key(|(_, _, from_settings)| *from_settings);
    let mut bindings: Vec<(Shortcut, Action)> = Vec::new();
    for (action, texts, _) in chosen {
        for text in texts {
            match parse(&text) {
                Ok(shortcut) => {
                    // A shortcut given to one action is taken from others.
                    bindings.retain(|(existing, _)| *existing != shortcut);
                    bindings.push((shortcut, action));
                }
                Err(problem) => problems.push(format!("[keys]: {problem}")),
            }
        }
    }
    *BINDINGS
        .write()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = bindings;
    problems
}

/// The action a key press asks for.
pub fn action_for(key: &Key, modifiers: Modifiers) -> Option<Action> {
    BINDINGS
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .iter()
        .find(|(shortcut, _)| shortcut.matches(key, modifiers))
        .map(|(_, action)| *action)
}

/// The first shortcut for `action`, as the interface names it.
pub fn label(action: Action) -> Option<String> {
    BINDINGS
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .iter()
        .find(|(_, bound)| *bound == action)
        .map(|(shortcut, _)| shortcut.label())
}

/// The shortcuts for `action` in effect, in the order they were given.
#[cfg(target_os = "macos")]
pub fn shortcuts(action: Action) -> Vec<Shortcut> {
    BINDINGS
        .read()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .iter()
        .filter(|(_, bound)| *bound == action)
        .map(|(shortcut, _)| shortcut.clone())
        .collect()
}

/// Whether `key` is Escape, which always closes and cancels.
pub fn is_escape(key: &Key) -> bool {
    matches!(key, Key::Named(keyboard::key::Named::Escape))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_shortcuts() {
        let shortcut = parse("Ctrl+Shift+O").unwrap();
        assert!(shortcut.command && shortcut.shift && !shortcut.alt);
        assert_eq!(shortcut.key, KeyName::Character("o".into()));
        assert_eq!(parse("Space").unwrap().key, KeyName::Named(Named::Space));
        assert_eq!(parse("Ctrl++").unwrap().key, KeyName::Character("+".into()));
        assert!(parse("Hyper+X").is_err());
        assert!(parse("Ctrl+Banana").is_err());
    }

    #[test]
    // One test, as the shortcuts are shared by the whole program.
    fn settings_replace_and_take_shortcuts() {
        let keys: toml::Table =
            toml::from_str("play-pause = \"P\"\nmute = []\nscreenshot = \"M\"\nbogus = \"X\"")
                .unwrap();
        let problems = configure(&keys);
        assert_eq!(problems.len(), 1);
        let none = Modifiers::empty();
        assert_eq!(
            action_for(&Key::Character("p".into()), none),
            Some(Action::PlayPause)
        );
        assert_eq!(action_for(&Key::Named(Named::Space), none), None);
        assert_eq!(
            action_for(&Key::Character("m".into()), none),
            Some(Action::Screenshot)
        );
        configure(&toml::Table::new());
        assert_eq!(
            action_for(&Key::Named(Named::Space), none),
            Some(Action::PlayPause)
        );
        // Shift may be what types a symbol.
        assert_eq!(
            action_for(&Key::Character("]".into()), Modifiers::SHIFT),
            Some(Action::Faster)
        );
        assert_eq!(
            action_for(&Key::Named(Named::ArrowLeft), Modifiers::SHIFT),
            Some(Action::BackMore)
        );
    }
}
