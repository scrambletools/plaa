//! Where plaa keeps its own files: the XDG base directories, on Windows
//! `%APPDATA%` and `%LOCALAPPDATA%`, and on macOS `~/Library`.

use std::env;
use std::path::{Path, PathBuf};

/// Set when built for a package. Every other build is a development
/// build, which keeps its own settings, data and instance socket so it
/// never disturbs the installed copy.
pub const PRODUCTION: bool = option_env!("PLAA_PRODUCTION").is_some();

const APP_DIR: &str = if PRODUCTION { "plaa" } else { "plaa-dev" };

#[cfg(not(any(windows, target_os = "macos")))]
fn xdg_dir(variable: &str, fallback_under_home: &str) -> Option<PathBuf> {
    env::var_os(variable)
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| home().map(|home| home.join(fallback_under_home)))
}

/// A folder named by a Windows environment variable, such as `APPDATA`.
#[cfg(windows)]
fn known_dir(variable: &str) -> Option<PathBuf> {
    env::var_os(variable)
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
}

/// The home folder: `$HOME`, or `%USERPROFILE%` on Windows.
pub fn home() -> Option<PathBuf> {
    let variable = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    env::var_os(variable).map(PathBuf::from)
}

/// `$XDG_CONFIG_HOME/plaa.toml`, the settings file (`plaa-dev.toml` for
/// development builds).
#[cfg(not(any(windows, target_os = "macos")))]
pub fn config_file() -> Option<PathBuf> {
    xdg_dir("XDG_CONFIG_HOME", ".config").map(|dir| dir.join(format!("{APP_DIR}.toml")))
}

/// `%APPDATA%\plaa\plaa.toml` (`plaa-dev` for development builds).
#[cfg(windows)]
pub fn config_file() -> Option<PathBuf> {
    known_dir("APPDATA").map(|dir| dir.join(APP_DIR).join(format!("{APP_DIR}.toml")))
}

/// `~/Library/Application Support/plaa/plaa.toml`.
#[cfg(target_os = "macos")]
pub fn config_file() -> Option<PathBuf> {
    application_support().map(|dir| dir.join(format!("{APP_DIR}.toml")))
}

/// `$XDG_STATE_HOME/plaa`, for where each file was left off.
#[cfg(not(any(windows, target_os = "macos")))]
pub fn state_dir() -> Option<PathBuf> {
    xdg_dir("XDG_STATE_HOME", ".local/state").map(|dir| dir.join(APP_DIR))
}

/// `%LOCALAPPDATA%\plaa`, kept on this computer.
#[cfg(windows)]
pub fn state_dir() -> Option<PathBuf> {
    known_dir("LOCALAPPDATA").map(|dir| dir.join(APP_DIR))
}

/// `~/Library/Application Support/plaa`.
#[cfg(target_os = "macos")]
pub fn state_dir() -> Option<PathBuf> {
    application_support()
}

/// `$XDG_CACHE_HOME/plaa`, for thumbnails and files made for other apps.
#[cfg(not(any(windows, target_os = "macos")))]
pub fn cache_dir() -> Option<PathBuf> {
    xdg_dir("XDG_CACHE_HOME", ".cache").map(|dir| dir.join(APP_DIR))
}

/// `%LOCALAPPDATA%\plaa\cache`.
#[cfg(windows)]
pub fn cache_dir() -> Option<PathBuf> {
    state_dir().map(|dir| dir.join("cache"))
}

/// `~/Library/Caches/plaa`.
#[cfg(target_os = "macos")]
pub fn cache_dir() -> Option<PathBuf> {
    home().map(|home| home.join("Library/Caches").join(APP_DIR))
}

/// `$XDG_RUNTIME_DIR/plaa`, for the single instance socket. macOS has
/// no runtime folder; there it is under `$TMPDIR`, which macOS gives each
/// user.
pub fn runtime_dir() -> Option<PathBuf> {
    env::var_os("XDG_RUNTIME_DIR")
        .or_else(|| {
            cfg!(target_os = "macos")
                .then(|| env::var_os("TMPDIR"))
                .flatten()
        })
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .map(|dir| dir.join(APP_DIR))
}

#[cfg(target_os = "macos")]
fn application_support() -> Option<PathBuf> {
    home().map(|home| home.join("Library/Application Support").join(APP_DIR))
}

/// The folder recordings go to by default: the user's videos folder
/// (`Movies` on macOS), else the home folder.
pub fn default_recordings() -> PathBuf {
    let videos = if cfg!(target_os = "macos") {
        "Movies"
    } else {
        "Videos"
    };
    user_folder("XDG_VIDEOS_DIR", videos)
}

/// The folder screenshots of a frame go to by default: the user's
/// pictures folder, else the home folder.
pub fn default_screenshots() -> PathBuf {
    user_folder("XDG_PICTURES_DIR", "Pictures")
}

/// A folder from `user-dirs.dirs` on Linux, or `name` under the home
/// folder when it exists, else the home folder itself.
fn user_folder(variable: &str, name: &str) -> PathBuf {
    let home = home().unwrap_or_else(|| PathBuf::from("."));
    #[cfg(not(any(windows, target_os = "macos")))]
    if let Some(folder) = user_dirs_entry(&home, variable) {
        return folder;
    }
    #[cfg(any(windows, target_os = "macos"))]
    let _ = variable;
    let folder = home.join(name);
    if folder.is_dir() { folder } else { home }
}

/// Reads `$XDG_CONFIG_HOME/user-dirs.dirs`, where desktops say which
/// folder holds videos or pictures in the user's language.
#[cfg(not(any(windows, target_os = "macos")))]
fn user_dirs_entry(home: &Path, variable: &str) -> Option<PathBuf> {
    let file = xdg_dir("XDG_CONFIG_HOME", ".config")?.join("user-dirs.dirs");
    let text = std::fs::read_to_string(file).ok()?;
    parse_user_dirs(&text, home, variable).filter(|folder| folder.is_dir())
}

#[cfg(not(any(windows, target_os = "macos")))]
fn parse_user_dirs(text: &str, home: &Path, variable: &str) -> Option<PathBuf> {
    text.lines().find_map(|line| {
        let value = line.trim().strip_prefix(variable)?.trim_start();
        let value = value.strip_prefix('=')?.trim().trim_matches('"');
        Some(match value.strip_prefix("$HOME") {
            Some(rest) => home.join(rest.trim_start_matches('/')),
            None => PathBuf::from(value),
        })
    })
}

/// Replaces a leading `~` with the home folder.
pub fn expand_home(path: &Path) -> PathBuf {
    match (path.strip_prefix("~"), home()) {
        (Ok(rest), Some(home)) => home.join(rest),
        _ => path.to_path_buf(),
    }
}

/// Writes paths under the home folder with `~`, for a readable file.
pub fn abbreviate_home(path: &Path) -> PathBuf {
    match home() {
        Some(home) if path.starts_with(&home) && home != Path::new("/") => {
            Path::new("~").join(path.strip_prefix(&home).unwrap_or(path))
        }
        _ => path.to_path_buf(),
    }
}

/// `path` made absolute with links resolved, as `std::fs::canonicalize`
/// does, but in the form people know: on Windows without the `\\?\`
/// prefix it adds. `path` itself when it cannot be resolved.
pub fn canonical(path: &Path) -> PathBuf {
    let resolved = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    simplify(resolved)
}

#[cfg(windows)]
fn simplify(path: PathBuf) -> PathBuf {
    let text = path.to_string_lossy();
    if let Some(share) = text.strip_prefix(r"\\?\UNC\") {
        PathBuf::from(format!(r"\\{share}"))
    } else if let Some(local) = text.strip_prefix(r"\\?\") {
        PathBuf::from(local)
    } else {
        path
    }
}

#[cfg(not(windows))]
fn simplify(path: PathBuf) -> PathBuf {
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_is_written_as_a_tilde_and_read_back() {
        let Some(home) = home() else {
            return;
        };
        let inside = home.join("Videos").join("a.mkv");
        let short = abbreviate_home(&inside);
        assert!(short.starts_with("~"));
        assert_eq!(expand_home(&short), inside);
    }

    #[cfg(not(any(windows, target_os = "macos")))]
    #[test]
    fn user_dirs_are_read_relative_to_home() {
        let text = "# comment\nXDG_VIDEOS_DIR=\"$HOME/Vidéos\"\nXDG_PICTURES_DIR=\"/srv/pics\"\n";
        let home = Path::new("/home/someone");
        assert_eq!(
            parse_user_dirs(text, home, "XDG_VIDEOS_DIR"),
            Some(PathBuf::from("/home/someone/Vidéos"))
        );
        assert_eq!(
            parse_user_dirs(text, home, "XDG_PICTURES_DIR"),
            Some(PathBuf::from("/srv/pics"))
        );
        assert_eq!(parse_user_dirs(text, home, "XDG_MUSIC_DIR"), None);
    }

    #[cfg(windows)]
    #[test]
    fn canonical_paths_drop_the_verbatim_prefix() {
        assert_eq!(
            simplify(PathBuf::from(r"\\?\UNC\server\share\a.mkv")),
            PathBuf::from(r"\\server\share\a.mkv")
        );
        assert_eq!(
            simplify(PathBuf::from(r"\\?\C:\Users\me\a.mkv")),
            PathBuf::from(r"C:\Users\me\a.mkv")
        );
    }
}
