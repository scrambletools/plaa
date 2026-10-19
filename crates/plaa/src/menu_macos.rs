//! The macOS menu bar: the app menu, then File, Edit, View, Playback and
//! Window. Each item runs the same action as its shortcut.
//!
//! A menu item's key equivalent takes the key before the window sees it.
//! Items therefore show only the shortcuts with ⌘ on a character, and the
//! function keys: a single key such as Space, or ⌘ and an arrow, would no
//! longer reach the address field, which types and moves with them. Cut,
//! Copy, Paste and Select All are the system's own items, which take their
//! keys only when something answers `copy:` and the like; plaa's windows
//! do not, so the keys reach text fields as before.

use std::cell::RefCell;
use std::sync::{Mutex, PoisonError};

use iced::keyboard::key::Named;
use muda::accelerator::{Accelerator, Code, Modifiers};
use muda::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use objc2::runtime::AnyObject;
use objc2_foundation::{NSDictionary, NSNumber, NSString, NSUserDefaults};

use crate::fl;
use crate::shortcuts::{self, Action};

thread_local! {
    /// The menu bar in use; AppKit only borrows it.
    static MENU: RefCell<Option<Menu>> = const { RefCell::new(None) };
}

type Handler = Box<dyn Fn(Action) + Send>;
static HANDLER: Mutex<Option<Handler>> = Mutex::new(None);

/// Menu item ids and their actions.
const ACTIONS: &[(&str, Action)] = &[
    ("settings", Action::Settings),
    ("quit", Action::Quit),
    ("open", Action::Open),
    ("open-address", Action::OpenAddress),
    ("close", Action::CloseWindow),
    ("inspector", Action::Info),
    ("playlist", Action::Playlist),
    ("picture-in-picture", Action::PictureInPicture),
    ("full-screen", Action::FullScreen),
    ("play-pause", Action::PlayPause),
    ("back", Action::Back),
    ("forward", Action::Forward),
    ("to-start", Action::ToStart),
    ("previous", Action::Previous),
    ("next", Action::Next),
    ("previous-chapter", Action::PreviousChapter),
    ("next-chapter", Action::NextChapter),
    ("faster", Action::Faster),
    ("slower", Action::Slower),
    ("normal-speed", Action::NormalSpeed),
    ("loop", Action::Loop),
    ("ab-loop", Action::AbLoop),
    ("volume-up", Action::VolumeUp),
    ("volume-down", Action::VolumeDown),
    ("mute", Action::Mute),
    ("subtitles", Action::Subtitles),
    ("audio-track", Action::AudioTrack),
    ("screenshot", Action::Screenshot),
];

/// Calls `handler` with the action of each menu item chosen.
pub fn set_handler(handler: impl Fn(Action) + Send + 'static) {
    *HANDLER.lock().unwrap_or_else(PoisonError::into_inner) = Some(Box::new(handler));
    MenuEvent::set_event_handler(Some(|event: MenuEvent| {
        let Some(action) = ACTIONS
            .iter()
            .find(|(id, _)| *id == event.id.as_ref())
            .map(|(_, action)| *action)
        else {
            return;
        };
        if let Some(handler) = &*HANDLER.lock().unwrap_or_else(PoisonError::into_inner) {
            handler(action);
        }
    }));
}

/// The key equivalent for the first of `action`'s shortcuts a menu item
/// may take.
fn accelerator(action: Action) -> Option<Accelerator> {
    shortcuts::shortcuts(action).iter().find_map(|shortcut| {
        let code = match (shortcut.character(), shortcut.named()) {
            (Some(character), _) if shortcut.command() => character_code(character)?,
            (None, Some(named)) => function_code(named)?,
            _ => return None,
        };
        let mut modifiers = Modifiers::empty();
        for (held, modifier) in [
            (shortcut.command(), Modifiers::META),
            (shortcut.shift(), Modifiers::SHIFT),
            (shortcut.alt(), Modifiers::ALT),
        ] {
            if held {
                modifiers |= modifier;
            }
        }
        Some(Accelerator::new(modifiers, code))
    })
}

fn character_code(character: &str) -> Option<Code> {
    const LETTERS: [Code; 26] = [
        Code::KeyA,
        Code::KeyB,
        Code::KeyC,
        Code::KeyD,
        Code::KeyE,
        Code::KeyF,
        Code::KeyG,
        Code::KeyH,
        Code::KeyI,
        Code::KeyJ,
        Code::KeyK,
        Code::KeyL,
        Code::KeyM,
        Code::KeyN,
        Code::KeyO,
        Code::KeyP,
        Code::KeyQ,
        Code::KeyR,
        Code::KeyS,
        Code::KeyT,
        Code::KeyU,
        Code::KeyV,
        Code::KeyW,
        Code::KeyX,
        Code::KeyY,
        Code::KeyZ,
    ];
    const DIGITS: [Code; 10] = [
        Code::Digit0,
        Code::Digit1,
        Code::Digit2,
        Code::Digit3,
        Code::Digit4,
        Code::Digit5,
        Code::Digit6,
        Code::Digit7,
        Code::Digit8,
        Code::Digit9,
    ];
    let mut characters = character.chars();
    let (Some(character), None) = (characters.next(), characters.next()) else {
        return None;
    };
    Some(match character {
        'a'..='z' => LETTERS[(character as u8 - b'a') as usize],
        '0'..='9' => DIGITS[(character as u8 - b'0') as usize],
        ',' => Code::Comma,
        '.' => Code::Period,
        '-' => Code::Minus,
        '=' => Code::Equal,
        '/' => Code::Slash,
        ';' => Code::Semicolon,
        '\'' => Code::Quote,
        '[' => Code::BracketLeft,
        ']' => Code::BracketRight,
        '\\' => Code::Backslash,
        '`' => Code::Backquote,
        _ => return None,
    })
}

fn function_code(named: Named) -> Option<Code> {
    Some(match named {
        Named::F1 => Code::F1,
        Named::F2 => Code::F2,
        Named::F3 => Code::F3,
        Named::F4 => Code::F4,
        Named::F5 => Code::F5,
        Named::F6 => Code::F6,
        Named::F7 => Code::F7,
        Named::F8 => Code::F8,
        Named::F9 => Code::F9,
        Named::F10 => Code::F10,
        Named::F11 => Code::F11,
        Named::F12 => Code::F12,
        _ => return None,
    })
}

/// The item for action `id`, with its label and shortcut.
fn item(id: &str, label: String) -> MenuItem {
    let key = ACTIONS
        .iter()
        .find(|(candidate, _)| *candidate == id)
        .and_then(|(_, action)| accelerator(*action));
    MenuItem::with_id(id, label, true, key)
}

/// Keeps AppKit from adding its own Enter Full Screen item to the View
/// menu, beside plaa's, which plaa's windows would not know of. Call it
/// before iced starts.
#[allow(unsafe_code)]
pub fn leave_out_system_full_screen() {
    let key = NSString::from_str("NSFullScreenMenuItemEverywhere");
    let off = NSNumber::new_bool(false);
    let defaults = NSDictionary::<NSString, AnyObject>::from_slices(&[&*key], &[&*off]);
    // The registration domain: only for this run, not saved.
    // SAFETY: the dictionary holds a string key and a number, both
    // property list values, as defaults require.
    unsafe { NSUserDefaults::standardUserDefaults().registerDefaults(&defaults) };
}

/// Builds the menu bar in the interface's language and puts it up.
/// Called at start and when the language changes; on the main thread.
pub fn install() {
    match build() {
        Ok(menu) => {
            menu.init_for_nsapp();
            MENU.with(|slot| *slot.borrow_mut() = Some(menu));
        }
        Err(error) => eprintln!("plaa: no menu bar: {error}"),
    }
}

fn build() -> muda::Result<Menu> {
    let separator = PredefinedMenuItem::separator;
    let about = AboutMetadata {
        name: Some("plaa".into()),
        version: Some(env!("CARGO_PKG_VERSION").into()),
        license: Some(env!("CARGO_PKG_LICENSE").into()),
        website: Some(env!("CARGO_PKG_REPOSITORY").into()),
        ..AboutMetadata::default()
    };
    let app = Submenu::with_items(
        "plaa",
        true,
        &[
            &PredefinedMenuItem::about(Some(&fl!("menu-about")), Some(about)),
            &separator(),
            &item("settings", fl!("menu-settings")),
            &separator(),
            &PredefinedMenuItem::services(Some(&fl!("menu-services"))),
            &separator(),
            &PredefinedMenuItem::hide(Some(&fl!("menu-hide"))),
            &PredefinedMenuItem::hide_others(Some(&fl!("menu-hide-others"))),
            &PredefinedMenuItem::show_all(Some(&fl!("menu-show-all"))),
            &separator(),
            &item("quit", fl!("menu-quit")),
        ],
    )?;
    let file = Submenu::with_items(
        fl!("menu-file"),
        true,
        &[
            &item("open", fl!("menu-open")),
            &item("open-address", fl!("menu-open-address")),
            &separator(),
            &item("close", fl!("menu-close")),
        ],
    )?;
    let edit = Submenu::with_items(
        fl!("menu-edit"),
        true,
        &[
            &PredefinedMenuItem::cut(Some(&fl!("menu-cut"))),
            &PredefinedMenuItem::copy(Some(&fl!("menu-copy"))),
            &PredefinedMenuItem::paste(Some(&fl!("menu-paste"))),
            &PredefinedMenuItem::select_all(Some(&fl!("menu-select-all"))),
        ],
    )?;
    let view = Submenu::with_items(
        fl!("menu-view"),
        true,
        &[
            &item("inspector", fl!("action-info")),
            &item("playlist", fl!("action-playlist")),
            &separator(),
            &item("picture-in-picture", fl!("action-picture-in-picture")),
            &item("full-screen", fl!("action-full-screen")),
        ],
    )?;
    let skip = crate::window::SKIP_SHORT;
    let playback = Submenu::with_items(
        fl!("menu-playback"),
        true,
        &[
            &item("play-pause", fl!("menu-play-pause")),
            &item("back", fl!("action-back", seconds = skip)),
            &item("forward", fl!("action-forward", seconds = skip)),
            &item("to-start", fl!("menu-to-start")),
            &separator(),
            &item("previous", fl!("action-previous")),
            &item("next", fl!("action-next")),
            &item("previous-chapter", fl!("menu-previous-chapter")),
            &item("next-chapter", fl!("menu-next-chapter")),
            &separator(),
            &item("faster", fl!("menu-faster")),
            &item("slower", fl!("menu-slower")),
            &item("normal-speed", fl!("menu-normal-speed")),
            &separator(),
            &item("loop", fl!("action-loop")),
            &item("ab-loop", fl!("menu-ab-loop")),
            &separator(),
            &item("volume-up", fl!("menu-volume-up")),
            &item("volume-down", fl!("menu-volume-down")),
            &item("mute", fl!("menu-mute")),
            &separator(),
            &item("subtitles", fl!("menu-next-subtitles")),
            &item("audio-track", fl!("menu-next-audio")),
            &separator(),
            &item("screenshot", fl!("action-screenshot")),
        ],
    )?;
    let window = Submenu::with_items(
        fl!("menu-window"),
        true,
        &[
            &PredefinedMenuItem::minimize(Some(&fl!("menu-minimize"))),
            &PredefinedMenuItem::maximize(Some(&fl!("menu-zoom"))),
            &separator(),
            &PredefinedMenuItem::bring_all_to_front(Some(&fl!("menu-bring-all-to-front"))),
        ],
    )?;
    let menu = Menu::new();
    for submenu in [&app, &file, &edit, &view, &playback, &window] {
        menu.append(submenu)?;
    }
    window.set_as_windows_menu_for_nsapp();
    Ok(menu)
}
