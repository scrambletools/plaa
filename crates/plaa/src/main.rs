// Release builds on Windows open no console window.
#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod app;
mod dialog;
mod format;
mod i18n;
#[cfg(unix)]
mod instance;
#[cfg(windows)]
#[path = "instance_windows.rs"]
mod instance;
mod screenshot;
mod settings_view;
mod shortcuts;
mod timeline;
mod video;
mod window;

use std::sync::{Mutex, OnceLock, PoisonError};

use iced::futures::StreamExt;
use iced::futures::channel::mpsc::{self, UnboundedReceiver, UnboundedSender};
use plaa_media::Source;
use scramble_ui::{font, omarchy};

/// Events from other threads, delivered through a subscription.
#[derive(Debug, Clone)]
pub enum External {
    /// Another launch of plaa, or the system, asked to play these.
    OpenSources(Vec<Source>),
    OmarchyThemeChanged,
    /// What a window's player says.
    Player(iced::window::Id, plaa_media::Event),
}

static EXTERNAL_EVENTS: Mutex<Option<UnboundedReceiver<External>>> = Mutex::new(None);
static EXTERNAL_SENDER: OnceLock<UnboundedSender<External>> = OnceLock::new();

/// Sends `event` to the app from any thread.
pub fn post(event: External) {
    if let Some(sender) = EXTERNAL_SENDER.get() {
        let _ = sender.unbounded_send(event);
    }
}

/// Hands the receiver to the subscription once.
fn external_events() -> impl iced::futures::Stream<Item = External> {
    let receiver = EXTERNAL_EVENTS
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .take();
    iced::futures::stream::iter(receiver).flatten()
}

fn version() -> String {
    let build = if plaa_store::paths::PRODUCTION {
        concat!(" (", env!("PLAA_COMMIT"), ")")
    } else {
        concat!(
            " (dev build, ",
            env!("PLAA_COMMIT"),
            ", ",
            env!("PLAA_BUILT"),
            " UTC)"
        )
    };
    format!(
        "plaa {}{build}, FFmpeg {}",
        env!("CARGO_PKG_VERSION"),
        plaa_media::ffmpeg_version()
    )
}

fn main() -> iced::Result {
    #[cfg(windows)]
    attach_console();
    scramble_ui::input::remember_interface_thread();
    let mut sources = Vec::new();
    let mut options_done = false;
    for argument in std::env::args_os().skip(1) {
        let text = argument.to_string_lossy().into_owned();
        if !options_done {
            match text.as_str() {
                "-h" | "--help" => {
                    println!("{}", fl!("usage-help"));
                    return Ok(());
                }
                "-V" | "--version" => {
                    println!("{}", version());
                    return Ok(());
                }
                "--" => {
                    options_done = true;
                    continue;
                }
                option if option.starts_with('-') && option.len() > 1 => {
                    eprintln!("plaa: unknown option {option}\n\n{}\n", fl!("usage-help"));
                    std::process::exit(2);
                }
                _ => {}
            }
        }
        sources.push(match argument.to_str() {
            Some(text) => Source::parse(text),
            None => Source::File(argument.into()),
        });
    }
    let (sender, receiver) = mpsc::unbounded();
    let _ = EXTERNAL_SENDER.set(sender);

    #[cfg(unix)]
    if let Some(socket) =
        plaa_store::paths::runtime_dir().map(|dir| dir.join(instance::SOCKET_NAME))
    {
        match instance::claim_or_forward(&socket, &sources) {
            Ok(instance::Role::Forwarded) => return Ok(()),
            Ok(instance::Role::Primary(listener)) => {
                listener.spawn(|sources| post(External::OpenSources(sources)));
            }
            Err(error) => eprintln!("plaa: running without single instance: {error}"),
        }
    }
    #[cfg(windows)]
    match instance::claim_or_forward(&instance::pipe_name(), &sources) {
        Ok(instance::Role::Forwarded) => return Ok(()),
        Ok(instance::Role::Primary(listener)) => {
            listener.spawn(|sources| post(External::OpenSources(sources)));
        }
        Err(error) => eprintln!("plaa: running without single instance: {error}"),
    }

    let omarchy_dir = omarchy::current_theme_dir();
    if let Some(dir) = omarchy_dir.clone() {
        omarchy::watch(dir, || post(External::OmarchyThemeChanged));
    }
    *EXTERNAL_EVENTS
        .lock()
        .unwrap_or_else(PoisonError::into_inner) = Some(receiver);

    iced::daemon(
        move || app::Plaa::boot(sources.clone(), omarchy_dir.clone()),
        app::Plaa::update,
        app::Plaa::view,
    )
    .settings(iced::Settings {
        fonts: font::files().collect(),
        default_font: font::TEXT,
        default_text_size: font::DEFAULT_SIZE.into(),
        antialiasing: true,
        ..iced::Settings::default()
    })
    .title(app::Plaa::title)
    .theme(app::Plaa::theme)
    .subscription(app::Plaa::subscription)
    .run()
}

/// Writes `--help` and `--version` output to the terminal plaa was
/// started from: an app without a console window has nowhere else to
/// print.
#[cfg(windows)]
#[allow(unsafe_code)]
fn attach_console() {
    use windows_sys::Win32::System::Console::{ATTACH_PARENT_PROCESS, AttachConsole};
    // SAFETY: takes no pointers; fails harmlessly when there is no parent
    // console or one is already attached.
    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
    }
}
