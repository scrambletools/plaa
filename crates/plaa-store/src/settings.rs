//! User settings, stored as TOML in `$XDG_CONFIG_HOME/plaa.toml`
//! (`plaa-dev.toml` for development builds).

use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{atomic, paths};

/// The `language` and `input-language` setting that follows the system.
pub const SYSTEM_LANGUAGE: &str = "system";
/// M3's extra large corner, which dialogs use.
pub const DEFAULT_CORNER_RADIUS: f32 = 20.0;
/// How long the pointer rests before the overlay hides, in seconds.
pub const DEFAULT_OVERLAY_DELAY: f32 = 2.0;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Settings {
    pub appearance: Appearance,
    /// The interface's language, as a tag such as `he` or `pt-BR`, or
    /// `system` to follow the system's language.
    pub language: String,
    /// The language typed into text fields, as a tag, which sets the side
    /// an empty field starts on; or `system` to follow the keyboard layout
    /// in use.
    pub input_language: String,
    /// Build the colors from the system's accent: the Omarchy theme's on
    /// Omarchy, else the desktop's, Windows' or macOS's.
    pub system_accent: bool,
    /// The color picked for the scheme, as "#RRGGBB", used when the system
    /// accent is off or the system has none; plaa's own color when unset.
    pub accent_color: Option<String>,
    /// Float the toolbar and the playback controls over the picture of a
    /// video, hiding them once the pointer rests or leaves the window.
    /// Audio always has its controls docked.
    pub overlay: bool,
    /// Seconds the pointer rests over a playing video before the overlay
    /// hides, 1 to 10.
    pub overlay_delay: f32,
    /// Corner radius of dialogs and floating toolbars, 0 to 32 pixels.
    pub corner_radius: f32,
    /// How see-through floating toolbars are, in percent, 0 to 90.
    pub overlay_transparency: f32,
    /// Interface motion: springs, slides and growing dialogs. Off, or the
    /// system's reduced motion setting, makes changes happen at once.
    pub animations: bool,
    /// Decode video on the graphics card where it can, falling back to
    /// the processor.
    pub hardware_decoding: bool,
    /// The sound output device, by the id the system gives it; the
    /// system's default when none.
    pub audio_device: Option<String>,
    /// Volume, 0 to 1, as last set.
    pub volume: f32,
    pub muted: bool,
    /// Start a file where it was left off.
    pub resume: bool,
    /// When one file opens, list the other files in its folder with it,
    /// to play on to.
    pub folder_playlists: bool,
    /// Audio and subtitle languages to pick first, as ISO 639 codes such
    /// as `en` or `jpn`, most wanted first. Empty follows the file's own
    /// defaults.
    pub audio_languages: Vec<String>,
    pub subtitle_languages: Vec<String>,
    /// Where recordings are saved.
    #[serde(with = "home_path")]
    pub recordings: PathBuf,
    /// Where frames saved as pictures go.
    #[serde(with = "home_path")]
    pub screenshots: PathBuf,
    /// Keyboard shortcuts replacing the defaults: an action's name, such
    /// as `play-pause`, set to a shortcut such as "Space", a list of them,
    /// or an empty list for none; and the same in `linux`, `windows` and
    /// `macos` tables, which apply only on that system.
    pub keys: toml::Table,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            appearance: Appearance::System,
            language: SYSTEM_LANGUAGE.to_owned(),
            input_language: SYSTEM_LANGUAGE.to_owned(),
            system_accent: true,
            accent_color: None,
            overlay: true,
            overlay_delay: DEFAULT_OVERLAY_DELAY,
            corner_radius: DEFAULT_CORNER_RADIUS,
            overlay_transparency: 15.0,
            animations: true,
            hardware_decoding: true,
            audio_device: None,
            volume: 1.0,
            muted: false,
            resume: true,
            folder_playlists: false,
            audio_languages: Vec::new(),
            subtitle_languages: Vec::new(),
            recordings: paths::default_recordings(),
            screenshots: paths::default_screenshots(),
            keys: toml::Table::new(),
        }
    }
}

impl Settings {
    /// The chosen language's tag, or `None` to follow the system.
    pub fn chosen_language(&self) -> Option<&str> {
        let tag = self.language.trim();
        (!tag.is_empty() && tag != SYSTEM_LANGUAGE).then_some(tag)
    }

    /// The chosen input language's tag, or `None` to follow the keyboard
    /// layout.
    pub fn chosen_input_language(&self) -> Option<&str> {
        let tag = self.input_language.trim();
        (!tag.is_empty() && tag != SYSTEM_LANGUAGE).then_some(tag)
    }

    /// The overlay's delay, kept in its range whatever the file says.
    pub fn overlay_delay(&self) -> std::time::Duration {
        let seconds = if self.overlay_delay.is_finite() {
            self.overlay_delay.clamp(1.0, 10.0)
        } else {
            DEFAULT_OVERLAY_DELAY
        };
        std::time::Duration::from_secs_f32(seconds)
    }
}

/// Paths written with `~` for the home folder, and read back expanded.
mod home_path {
    use std::path::PathBuf;

    use serde::{Deserialize, Deserializer, Serializer};

    use crate::paths;

    pub fn serialize<S: Serializer>(
        path: &std::path::Path,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&paths::abbreviate_home(path).to_string_lossy())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<PathBuf, D::Error> {
        let text = String::deserialize(deserializer)?;
        Ok(paths::expand_home(&PathBuf::from(text)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Appearance {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Parse(toml::de::Error),
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "cannot read settings: {error}"),
            Self::Parse(error) => write!(formatter, "invalid settings: {error}"),
        }
    }
}

impl std::error::Error for LoadError {}

pub fn default_path() -> Option<PathBuf> {
    paths::config_file()
}

impl Settings {
    /// Loads `path`, or the defaults when it does not exist yet, then
    /// writes it back if it lacks any setting, so the file lists them all.
    pub fn load_or_create(path: &Path) -> Result<Self, LoadError> {
        if !path.exists() {
            let settings = Self::default();
            settings.save_to(path).map_err(LoadError::Io)?;
            return Ok(settings);
        }
        let text = std::fs::read_to_string(path).map_err(LoadError::Io)?;
        let settings: Self = toml::from_str(&text).map_err(LoadError::Parse)?;
        let present = toml::from_str::<toml::Table>(&text).map_err(LoadError::Parse)?;
        let all = toml::Table::try_from(&settings)
            .map_err(|error| LoadError::Io(io::Error::other(error)))?;
        if all.keys().any(|key| !present.contains_key(key)) {
            settings.save_to(path).map_err(LoadError::Io)?;
        }
        Ok(settings)
    }

    /// A missing file yields the defaults.
    pub fn load_from(path: &Path) -> Result<Self, LoadError> {
        match std::fs::read_to_string(path) {
            Ok(text) => toml::from_str(&text).map_err(LoadError::Parse),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Self::default()),
            Err(error) => Err(LoadError::Io(error)),
        }
    }

    pub fn save_to(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let text = toml::to_string(self).map_err(io::Error::other)?;
        atomic::write(path, text.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_file_gives_defaults() {
        let dir = tempfile::tempdir().unwrap();
        let loaded = Settings::load_from(&dir.path().join("plaa.toml")).unwrap();
        assert_eq!(loaded, Settings::default());
    }

    #[test]
    fn round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("plaa.toml");
        let settings = Settings {
            appearance: Appearance::Dark,
            overlay: false,
            volume: 0.5,
            subtitle_languages: vec!["en".to_owned(), "fr".to_owned()],
            recordings: PathBuf::from("/srv/recordings"),
            ..Settings::default()
        };
        settings.save_to(&path).unwrap();
        assert_eq!(Settings::load_from(&path).unwrap(), settings);
    }

    #[test]
    fn missing_settings_are_written_back() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plaa.toml");
        std::fs::write(&path, "overlay = false\n").unwrap();
        let loaded = Settings::load_or_create(&path).unwrap();
        assert!(!loaded.overlay);
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("hardware-decoding"));
        assert!(text.contains("overlay = false"));
    }

    #[test]
    fn overlay_delay_stays_in_range() {
        let settings = Settings {
            overlay_delay: 60.0,
            ..Settings::default()
        };
        assert_eq!(settings.overlay_delay().as_secs(), 10);
        let settings = Settings {
            overlay_delay: f32::NAN,
            ..Settings::default()
        };
        assert_eq!(
            settings.overlay_delay().as_secs_f32(),
            DEFAULT_OVERLAY_DELAY
        );
    }
}
