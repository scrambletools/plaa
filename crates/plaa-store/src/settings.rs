//! User settings, stored as TOML in `$XDG_CONFIG_HOME/plaa.toml`
//! (`plaa-dev.toml` for development builds).

use std::collections::BTreeMap;
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
    /// What a sound file shows as it plays: `spectrum` or `waveform`
    /// under its cover, `prism` for a scene filling the window, or `cover`
    /// for the cover alone.
    pub visualizer: Visualizer,
    /// Start a file where it was left off.
    pub resume: bool,
    /// When one file opens, list the other files in its folder with it,
    /// to play on to.
    pub folder_playlists: bool,
    /// Which quality streams that come in several play in: `auto` for the
    /// tallest that fits the screen, `best`, or the tallest to play, such
    /// as `1080p`.
    pub stream_quality: StreamQuality,
    /// Play web pages, such as YouTube's, through yt-dlp when it is
    /// installed.
    pub web_pages: bool,
    /// Audio and subtitle languages to pick first, as ISO 639 codes such
    /// as `en` or `jpn`, most wanted first. Empty follows the file's own
    /// defaults.
    pub audio_languages: Vec<String>,
    pub subtitle_languages: Vec<String>,
    /// Where sound recordings are saved.
    #[serde(with = "home_path")]
    pub sound_recordings: PathBuf,
    /// Where screen and camera recordings are saved. `recordings` named
    /// the one folder all of them went to before.
    #[serde(with = "home_path", alias = "recordings")]
    pub video_recordings: PathBuf,
    /// The sound input device recordings come from, by the id the system
    /// gives it; the system's default when none.
    pub microphone: Option<String>,
    /// What sound is recorded as: `aac`, `mp3`, `opus`, `flac`, `alac`
    /// or `wav`.
    pub recording_format: String,
    /// Its quality: kb/s for compressed sound, bits for lossless.
    pub recording_quality: u32,
    /// Whether screen recordings take the sound of the recording input.
    pub screen_sound: bool,
    /// What screen recordings are recorded as: `h264`, `hevc` or `av1`.
    pub screen_format: String,
    /// The most pictures a second screen and camera recordings keep; 0
    /// keeps every picture.
    pub screen_rate: u32,
    /// The camera recordings come from, by the id FFmpeg opens it by; the
    /// first there is when none, or when it is gone.
    pub camera: Option<String>,
    /// Whether camera recordings take the sound of the recording input.
    pub camera_sound: bool,
    /// Whether recordings even out their sound's level.
    pub level_automatically: bool,
    /// The wired interface plaa looks for AVB talkers and listeners on, by
    /// name; none keeps plaa off the AVB network.
    pub avb_interface: Option<String>,
    /// Whether plaa sends what it plays from a Milan entity of its own.
    pub avb_talker: bool,
    /// That entity's name, as controllers show it; empty for plaa's own
    /// with the computer's name.
    pub avb_name: String,
    /// Where frames saved as pictures go.
    #[serde(with = "home_path")]
    pub screenshots: PathBuf,
    /// Whether other programs, such as AI agents through `plaa --mcp`,
    /// may see what plays and control it.
    pub outside_control: bool,
    /// The agents, by the names they give, that the user allowed to
    /// control plaa. Others are asked about on their first connection.
    pub allowed_agents: Vec<String>,
    /// What plaa asks about before an agent's tool runs.
    pub ask_before: AskBefore,
    /// The models the assistant panel can use. Their keys are in the
    /// system keychain, not here.
    pub assistant_models: Vec<AssistantModel>,
    /// The id of the model the panel uses.
    pub assistant_model: Option<String>,
    /// How much louder each input is taken, in decibels, by the id the
    /// system gives it; "" for the system's default.
    pub input_gains: BTreeMap<String, f32>,
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
            visualizer: Visualizer::Spectrum,
            resume: true,
            folder_playlists: false,
            stream_quality: StreamQuality::Auto,
            web_pages: true,
            audio_languages: Vec::new(),
            subtitle_languages: Vec::new(),
            sound_recordings: paths::default_sound_recordings(),
            video_recordings: paths::default_video_recordings(),
            microphone: None,
            recording_format: "aac".to_owned(),
            recording_quality: 256,
            screen_sound: false,
            screen_format: "h264".to_owned(),
            screen_rate: 60,
            camera: None,
            camera_sound: true,
            level_automatically: false,
            input_gains: BTreeMap::new(),
            avb_interface: None,
            avb_talker: false,
            avb_name: String::new(),
            screenshots: paths::default_screenshots(),
            outside_control: true,
            allowed_agents: Vec::new(),
            ask_before: AskBefore::default(),
            assistant_models: Vec::new(),
            assistant_model: None,
            keys: toml::Table::new(),
        }
    }
}

/// A model the assistant panel can use.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssistantModel {
    /// Names the model's key in the keychain, and the model here.
    pub id: String,
    /// `anthropic`, `open-ai`, `gemini`, `ollama` or `open-ai-compatible`.
    pub provider: String,
    pub model: String,
    /// The server's address, for a local or compatible server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// How much a local model reads at once, in tokens, when plaa sets it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<u32>,
    /// The provider's name for the model, when it gives one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the model sees pictures, when the provider said.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vision: Option<bool>,
}

/// Whether plaa asks before an agent's tool of each kind runs. Nothing
/// asks at first: an agent is asked about once, as it connects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct AskBefore {
    /// Seeing what plays: the windows, the files, the picture shown.
    pub reading: bool,
    /// Playing, pausing, seeking, the volume and the tracks.
    pub playing: bool,
    /// Opening files and addresses, and closing windows.
    pub opening: bool,
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

/// What a sound file shows as it plays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Visualizer {
    /// The pitches heard, as bars.
    #[default]
    Spectrum,
    /// The file's loudness from start to end.
    Waveform,
    /// A scene in three dimensions filling the window.
    Prism,
    /// The cover alone.
    Cover,
}

/// Which of a stream's qualities to play.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(from = "String", into = "String")]
pub enum StreamQuality {
    /// The tallest picture that fits the screen.
    #[default]
    Auto,
    Best,
    /// The tallest picture at most this many lines tall.
    AtMost(u32),
}

impl From<String> for StreamQuality {
    fn from(text: String) -> Self {
        let text = text.trim().to_ascii_lowercase();
        match text.as_str() {
            "best" => Self::Best,
            _ => text
                .trim_end_matches('p')
                .parse()
                .ok()
                .filter(|lines| *lines > 0)
                .map_or(Self::Auto, Self::AtMost),
        }
    }
}

impl From<StreamQuality> for String {
    fn from(quality: StreamQuality) -> Self {
        match quality {
            StreamQuality::Auto => "auto".to_owned(),
            StreamQuality::Best => "best".to_owned(),
            StreamQuality::AtMost(lines) => format!("{lines}p"),
        }
    }
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
        let mut settings: Self = toml::from_str(&text).map_err(LoadError::Parse)?;
        let present = toml::from_str::<toml::Table>(&text).map_err(LoadError::Parse)?;
        settings.split_recordings(&present);
        let all = toml::Table::try_from(&settings)
            .map_err(|error| LoadError::Io(io::Error::other(error)))?;
        if all.keys().any(|key| !present.contains_key(key)) {
            settings.save_to(path).map_err(LoadError::Io)?;
        }
        Ok(settings)
    }

    /// A file from before sound recordings had a folder of their own goes
    /// on saving them where it saved every recording, unless that was the
    /// default videos folder.
    fn split_recordings(&mut self, present: &toml::Table) {
        if present.contains_key("recordings")
            && !present.contains_key("sound-recordings")
            && self.video_recordings != paths::default_video_recordings()
        {
            self.sound_recordings = self.video_recordings.clone();
        }
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
            sound_recordings: PathBuf::from("/srv/sound"),
            video_recordings: PathBuf::from("/srv/video"),
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
    fn one_recordings_folder_becomes_two() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("plaa.toml");
        std::fs::write(&path, "recordings = \"/srv/recordings\"\n").unwrap();
        let loaded = Settings::load_or_create(&path).unwrap();
        assert_eq!(loaded.video_recordings, PathBuf::from("/srv/recordings"));
        assert_eq!(loaded.sound_recordings, loaded.video_recordings);
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("video-recordings"));
        assert!(!text.lines().any(|line| line.starts_with("recordings")));

        let mut table = toml::Table::new();
        let videos = paths::default_video_recordings();
        table.insert("recordings".into(), videos.display().to_string().into());
        std::fs::write(&path, toml::to_string(&table).unwrap()).unwrap();
        let loaded = Settings::load_or_create(&path).unwrap();
        assert_eq!(loaded.video_recordings, videos);
        assert_eq!(loaded.sound_recordings, paths::default_sound_recordings());
    }

    #[test]
    fn stream_quality_reads_as_written() {
        for quality in [
            StreamQuality::Auto,
            StreamQuality::Best,
            StreamQuality::AtMost(1080),
        ] {
            assert_eq!(StreamQuality::from(String::from(quality)), quality);
        }
        assert_eq!(
            StreamQuality::from("720".to_owned()),
            StreamQuality::AtMost(720)
        );
        assert_eq!(
            StreamQuality::from("nonsense".to_owned()),
            StreamQuality::Auto
        );
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
