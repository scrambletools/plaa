//! Makes plaa the system's default app for the video and sound files it
//! plays, as prev does for its own. This changes only the system's
//! setting, never plaa's.
//!
//! - Linux: the `[Default Applications]` of `mimeapps.list` in the user's
//!   config folder, as `xdg-mime default` writes it, for every MIME type
//!   plaa's desktop entry lists.
//! - macOS: Launch Services' default handler for the common types. macOS
//!   asks the user to confirm each one, so the rarer types are left to
//!   Finder's Get Info.
//! - Windows lets only the user choose default apps, in Settings; this
//!   opens plaa's page there, which its installer registers.
//!
//! `status` tells how many of those types plaa opens now.

/// What changing the default did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// plaa is now the default for this many file types.
    #[cfg_attr(any(windows, target_os = "macos"), expect(dead_code))]
    Set(usize),
    /// The system's own page for choosing opened, where the user chooses.
    #[cfg_attr(not(windows), expect(dead_code))]
    OpenedSettings,
    /// macOS: the change was asked for; the system asks the user to
    /// confirm each type.
    #[cfg_attr(not(target_os = "macos"), expect(dead_code))]
    Asked,
}

/// Why plaa could not become the default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Failure {
    /// Linux: the desktop entry is not installed, so the system has
    /// nothing to open files with.
    #[cfg_attr(any(windows, target_os = "macos"), expect(dead_code))]
    NoDesktopEntry,
    /// macOS: plaa is not running from plaa.app.
    #[cfg_attr(not(target_os = "macos"), expect(dead_code))]
    NoBundle,
    Other(String),
}

/// How many of the file types the button covers plaa opens now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Status {
    pub set: usize,
    pub total: usize,
}

/// The file name extensions plaa's installer offers plaa for, in Open
/// With and on its page in Default apps, as `packaging/windows/plaa.wxs`
/// lists them: video, sound, then playlists.
#[cfg(any(windows, test))]
const EXTENSIONS: &[&str] = &[
    ".3g2", ".3gp", ".avi", ".flv", ".m2ts", ".m4v", ".mkv", ".mov", ".mp4", ".mpeg", ".mpg",
    ".mts", ".ogv", ".ts", ".webm", ".wmv", ".aac", ".ac3", ".aif", ".aiff", ".ape", ".dts",
    ".flac", ".m4a", ".mka", ".mp3", ".mpc", ".oga", ".ogg", ".opus", ".wav", ".wma", ".wv",
    ".m3u", ".m3u8", ".pls",
];

/// Makes plaa the default app. Blocks briefly; run it off the interface
/// thread.
pub async fn make_default() -> Result<Outcome, Failure> {
    platform::make_default().await
}

/// How many of the file types plaa opens now; `None` when that cannot be
/// told, such as for a Mac build not run from plaa.app.
pub async fn status() -> Option<Status> {
    platform::status()
}

#[cfg(not(any(windows, target_os = "macos")))]
mod platform {
    use std::path::PathBuf;

    use super::{Failure, Outcome, Status};

    /// The desktop entry plaa's packages install, whose MIME types are the
    /// ones plaa becomes the default for.
    const DESKTOP_ENTRY: &str = include_str!("../../../data/run.plaa.desktop");
    const SECTION: &str = "[Default Applications]";

    /// The desktop entry every plaa package installs, or inside a Flatpak
    /// the one it exports, named for the Flatpak.
    fn desktop_id() -> String {
        match std::env::var("FLATPAK_ID") {
            Ok(id) if !id.is_empty() => format!("{id}.desktop"),
            _ => "run.plaa.desktop".to_owned(),
        }
    }

    /// The MIME types the desktop entry lists.
    pub fn mime_types() -> Vec<&'static str> {
        DESKTOP_ENTRY
            .lines()
            .find_map(|line| line.strip_prefix("MimeType="))
            .unwrap_or_default()
            .split(';')
            .map(str::trim)
            .filter(|mime| !mime.is_empty())
            .collect()
    }

    pub async fn make_default() -> Result<Outcome, Failure> {
        let flatpak = std::env::var_os("FLATPAK_ID").is_some();
        // Inside a Flatpak the desktop entry is exported outside the
        // sandbox, where plaa cannot look.
        if !flatpak && !entry_installed(&desktop_id()) {
            return Err(Failure::NoDesktopEntry);
        }
        let types = mime_types();
        for list in lists(flatpak) {
            let text = match std::fs::read_to_string(&list) {
                Ok(text) => text,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
                Err(error) => return Err(Failure::Other(error.to_string())),
            };
            let updated = set_defaults(&text, &types, &desktop_id());
            if updated != text {
                if let Some(parent) = list.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|error| Failure::Other(error.to_string()))?;
                }
                plaa_store::atomic::write(&list, updated.as_bytes())
                    .map_err(|error| Failure::Other(error.to_string()))?;
            }
        }
        Ok(Outcome::Set(types.len()))
    }

    pub fn status() -> Option<Status> {
        let flatpak = std::env::var_os("FLATPAK_ID").is_some();
        let texts: Vec<String> = lookup_order(flatpak)
            .iter()
            .filter_map(|path| std::fs::read_to_string(path).ok())
            .collect();
        let types = mime_types();
        let ours = desktop_id();
        let set = types
            .iter()
            .filter(|mime| {
                texts
                    .iter()
                    .find_map(|text| default_in(text, mime))
                    .is_some_and(|desktop| desktop == ours)
            })
            .count();
        Some(Status {
            set,
            total: types.len(),
        })
    }

    /// The files the system reads defaults from, first one first: each
    /// config and data folder's desktop-specific list, then its
    /// `mimeapps.list`, then the older `defaults.list` in data folders.
    fn lookup_order(flatpak: bool) -> Vec<PathBuf> {
        let desktops: Vec<String> = std::env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_default()
            .split(':')
            .filter(|name| !name.is_empty())
            .map(str::to_lowercase)
            .collect();
        let dirs = |variable: &str, fallback: &str| -> Vec<PathBuf> {
            std::env::var(variable)
                .ok()
                .filter(|dirs| !dirs.is_empty())
                .unwrap_or_else(|| fallback.into())
                .split(':')
                .map(PathBuf::from)
                .collect()
        };
        let mut config = vec![config_home(flatpak)];
        config.extend(dirs("XDG_CONFIG_DIRS", "/etc/xdg"));
        let mut data = vec![data_home()];
        data.extend(dirs("XDG_DATA_DIRS", "/usr/local/share:/usr/share"));
        let mut order = Vec::new();
        for dir in config
            .into_iter()
            .chain(data.iter().map(|dir| dir.join("applications")))
        {
            for desktop in &desktops {
                order.push(dir.join(format!("{desktop}-mimeapps.list")));
            }
            order.push(dir.join("mimeapps.list"));
        }
        order.extend(
            data.iter()
                .map(|dir| dir.join("applications/defaults.list")),
        );
        order
    }

    /// The default app `text`, a `mimeapps.list`, names for `mime`: the
    /// first in its list under `[Default Applications]`.
    pub fn default_in(text: &str, mime: &str) -> Option<String> {
        let mut in_section = false;
        for line in text.lines() {
            let line = line.trim();
            if line.starts_with('[') {
                in_section = line == SECTION;
                continue;
            }
            if !in_section {
                continue;
            }
            if let Some((key, value)) = line.split_once('=')
                && key.trim() == mime
            {
                return value
                    .split(';')
                    .map(str::trim)
                    .find(|desktop| !desktop.is_empty())
                    .map(str::to_owned);
            }
        }
        None
    }

    /// The user's `mimeapps.list`, and the desktop's own one beside it
    /// when there is one, such as `hyprland-mimeapps.list`, which the
    /// system reads first.
    fn lists(flatpak: bool) -> Vec<PathBuf> {
        let config = config_home(flatpak);
        let mut lists = vec![config.join("mimeapps.list")];
        if let Ok(desktops) = std::env::var("XDG_CURRENT_DESKTOP") {
            for desktop in desktops.split(':').filter(|name| !name.is_empty()) {
                let own = config.join(format!("{}-mimeapps.list", desktop.to_lowercase()));
                if own.exists() {
                    lists.push(own);
                }
            }
        }
        lists
    }

    /// `$XDG_CONFIG_HOME`, or `~/.config`. A Flatpak's own points into its
    /// sandbox, so there it is always the home folder's.
    fn config_home(flatpak: bool) -> PathBuf {
        let home = plaa_store::paths::home().unwrap_or_else(|| PathBuf::from("/"));
        if flatpak {
            return home.join(".config");
        }
        std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
            .unwrap_or_else(|| home.join(".config"))
    }

    /// `$XDG_DATA_HOME`, or `~/.local/share`.
    fn data_home() -> PathBuf {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
            .unwrap_or_else(|| {
                plaa_store::paths::home()
                    .unwrap_or_default()
                    .join(".local/share")
            })
    }

    /// Whether a desktop entry named `id` is installed: in the data
    /// folders, and in `~/.local/share` and Flatpak's exports, which
    /// launchers read even where the variables leave them out.
    fn entry_installed(id: &str) -> bool {
        let home = plaa_store::paths::home().unwrap_or_default();
        let data_dirs = std::env::var("XDG_DATA_DIRS")
            .ok()
            .filter(|dirs| !dirs.is_empty())
            .unwrap_or_else(|| "/usr/local/share:/usr/share".into());
        [
            data_home(),
            home.join(".local/share"),
            home.join(".local/share/flatpak/exports/share"),
            PathBuf::from("/var/lib/flatpak/exports/share"),
        ]
        .into_iter()
        .chain(data_dirs.split(':').map(PathBuf::from))
        .any(|dir| dir.join("applications").join(id).exists())
    }

    /// `text`, a `mimeapps.list`, with `desktop` the default for each of
    /// `types`: existing lines for them replaced, missing ones added to
    /// `[Default Applications]`, which is made when missing. Everything
    /// else stays as it was.
    pub fn set_defaults(text: &str, types: &[&str], desktop: &str) -> String {
        let mut lines: Vec<String> = text.lines().map(str::to_owned).collect();
        let start = match lines.iter().position(|line| line.trim() == SECTION) {
            Some(start) => start,
            None => {
                if lines.last().is_some_and(|line| !line.trim().is_empty()) {
                    lines.push(String::new());
                }
                lines.push(SECTION.to_owned());
                lines.len() - 1
            }
        };
        let end = lines[start + 1..]
            .iter()
            .position(|line| line.trim_start().starts_with('['))
            .map_or(lines.len(), |offset| start + 1 + offset);
        let mut section: Vec<String> = lines[start + 1..end]
            .iter()
            .filter(|line| {
                let key = line.split('=').next().unwrap_or("").trim();
                !types.contains(&key)
            })
            .cloned()
            .collect();
        // Keep a blank line before the next section, if there was one.
        let trailing_blank = section.last().is_some_and(|line| line.trim().is_empty());
        if trailing_blank {
            section.pop();
        }
        section.extend(types.iter().map(|mime| format!("{mime}={desktop}")));
        if trailing_blank || end < lines.len() {
            section.push(String::new());
        }
        lines.splice(start + 1..end, section);
        let mut out = lines.join("\n");
        out.push('\n');
        out
    }
}

#[cfg(target_os = "macos")]
mod platform {
    #![allow(unsafe_code)]

    use std::ffi::c_void;

    use objc2::rc::Retained;
    use objc2_foundation::{NSBundle, NSString};

    use super::{Failure, Outcome, Status};

    type CFStringRef = *const c_void;

    #[link(name = "CoreServices", kind = "framework")]
    unsafe extern "C" {
        fn LSSetDefaultRoleHandlerForContentType(
            content_type: CFStringRef,
            role: u32,
            bundle_id: CFStringRef,
        ) -> i32;
        fn LSCopyDefaultRoleHandlerForContentType(
            content_type: CFStringRef,
            role: u32,
        ) -> CFStringRef;
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    unsafe extern "C" {
        fn CFRelease(object: *const c_void);
    }

    /// Every role: viewing and editing.
    const ROLES_ALL: u32 = 0xFFFF_FFFF;

    /// The content types the button asks for: the usual video and sound
    /// files. macOS asks the user about each, so the list stays short.
    pub const TYPES: &[&str] = &[
        "public.mpeg-4",
        "com.apple.quicktime-movie",
        "org.matroska.mkv",
        "org.webmproject.webm",
        "public.mp3",
        "com.apple.m4a-audio",
        "org.xiph.flac",
        "com.microsoft.waveform-audio",
    ];

    fn bundle() -> Option<Retained<NSString>> {
        NSBundle::mainBundle().bundleIdentifier()
    }

    pub async fn make_default() -> Result<Outcome, Failure> {
        let Some(bundle) = bundle() else {
            return Err(Failure::NoBundle);
        };
        let mut asked = 0;
        let mut last_error = None;
        for identifier in TYPES {
            let status = set_handler(identifier, &bundle);
            if status == 0 {
                asked += 1;
            } else {
                last_error = Some(status);
            }
        }
        match (asked, last_error) {
            (0, Some(status)) => Err(Failure::Other(format!("Launch Services error {status}"))),
            _ => Ok(Outcome::Asked),
        }
    }

    pub fn status() -> Option<Status> {
        let bundle = bundle()?.to_string();
        let set = TYPES
            .iter()
            .filter(|identifier| {
                handler(identifier).is_some_and(|name| name.eq_ignore_ascii_case(&bundle))
            })
            .count();
        Some(Status {
            set,
            total: TYPES.len(),
        })
    }

    /// The bundle identifier of the default app for a content type.
    fn handler(identifier: &str) -> Option<String> {
        let content_type = NSString::from_str(identifier);
        // SAFETY: a live NSString is a CFString; the result is owned,
        // copied, then released.
        let handler = unsafe {
            LSCopyDefaultRoleHandlerForContentType((&raw const *content_type).cast(), ROLES_ALL)
        };
        if handler.is_null() {
            return None;
        }
        let name = unsafe { &*handler.cast::<NSString>() }.to_string();
        unsafe { CFRelease(handler) };
        Some(name)
    }

    /// Makes `bundle` the default app for a content type, which macOS asks
    /// the user to confirm; Launch Services' status, 0 when asked.
    fn set_handler(identifier: &str, bundle: &NSString) -> i32 {
        let content_type = NSString::from_str(identifier);
        // SAFETY: both strings are live NSStrings, which are CFStrings.
        unsafe {
            LSSetDefaultRoleHandlerForContentType(
                (&raw const *content_type).cast(),
                ROLES_ALL,
                (&raw const *bundle).cast(),
            )
        }
    }
}

#[cfg(windows)]
mod platform {
    #![allow(unsafe_code)]

    use windows::Win32::UI::Shell::{ASSOCF_NONE, ASSOCSTR_EXECUTABLE, AssocQueryStringW};
    use windows::core::{HSTRING, PWSTR};

    use super::{EXTENSIONS, Failure, Outcome, Status};

    /// Settings' Default apps page for plaa, as the installer registers it
    /// under RegisteredApplications. Windows versions without the
    /// per-app page show the Default apps list.
    const PAGE: &str = "ms-settings:defaultapps?registeredAppUser=plaa";

    pub async fn make_default() -> Result<Outcome, Failure> {
        crate::dialog::open_link(PAGE.to_owned())
            .await
            .map(|()| Outcome::OpenedSettings)
            .map_err(Failure::Other)
    }

    /// Counts the extensions whose default app is this plaa.exe.
    pub fn status() -> Option<Status> {
        let exe = std::env::current_exe().ok()?;
        let name = exe.file_name()?.to_string_lossy().to_lowercase();
        let open = HSTRING::from("open");
        let set = EXTENSIONS
            .iter()
            .filter(|extension| {
                let extension = HSTRING::from(**extension);
                let mut buffer = [0u16; 1024];
                let mut length = buffer.len() as u32;
                // SAFETY: the buffer and its length are valid for the call,
                // which writes at most `length` characters.
                let result = unsafe {
                    AssocQueryStringW(
                        ASSOCF_NONE,
                        ASSOCSTR_EXECUTABLE,
                        &extension,
                        &open,
                        Some(PWSTR(buffer.as_mut_ptr())),
                        &mut length,
                    )
                };
                if result.is_err() {
                    return false;
                }
                let end = buffer.iter().position(|&unit| unit == 0).unwrap_or(0);
                let path = String::from_utf16_lossy(&buffer[..end]);
                std::path::Path::new(&path)
                    .file_name()
                    .is_some_and(|file| file.to_string_lossy().to_lowercase() == name)
            })
            .count();
        Some(Status {
            set,
            total: EXTENSIONS.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::EXTENSIONS;

    /// The extensions of one of the installer's lists, by its name.
    fn installer_list(name: &str) -> Vec<String> {
        let wxs = include_str!("../../../packaging/windows/plaa.wxs");
        let start = format!("<?define {name} = ");
        let line = wxs
            .lines()
            .find_map(|line| line.trim().strip_prefix(start.as_str()))
            .unwrap_or_else(|| panic!("plaa.wxs defines no {name}"));
        line.trim_end_matches("?>")
            .split(';')
            .map(|extension| extension.trim().to_owned())
            .filter(|extension| !extension.is_empty())
            .collect()
    }

    #[test]
    fn the_installer_registers_the_extensions_status_counts() {
        let registered: Vec<String> = ["VideoExtensions", "AudioExtensions", "PlaylistExtensions"]
            .into_iter()
            .flat_map(installer_list)
            .collect();
        assert_eq!(registered, EXTENSIONS);
    }

    #[cfg(not(any(windows, target_os = "macos")))]
    mod linux {
        use super::super::platform::{default_in, mime_types, set_defaults};

        const ID: &str = "plaa.desktop";

        #[test]
        fn the_desktop_entry_names_the_types() {
            let types = mime_types();
            assert!(types.contains(&"video/mp4"));
            assert!(types.contains(&"audio/flac"));
            assert!(types.iter().all(|mime| mime.contains('/')));
        }

        #[test]
        fn an_empty_list_gets_the_section() {
            assert_eq!(
                set_defaults("", &["video/mp4", "audio/flac"], ID),
                "[Default Applications]\nvideo/mp4=plaa.desktop\naudio/flac=plaa.desktop\n"
            );
        }

        #[test]
        fn other_entries_and_sections_stay() {
            let text = "[Added Associations]\nvideo/mp4=vlc.desktop;\n\n[Default Applications]\nvideo/mp4=mpv.desktop\ntext/html=firefox.desktop\n\n[Removed Associations]\naudio/flac=rhythmbox.desktop;\n";
            assert_eq!(
                set_defaults(text, &["video/mp4", "audio/flac"], ID),
                "[Added Associations]\nvideo/mp4=vlc.desktop;\n\n[Default Applications]\ntext/html=firefox.desktop\nvideo/mp4=plaa.desktop\naudio/flac=plaa.desktop\n\n[Removed Associations]\naudio/flac=rhythmbox.desktop;\n"
            );
        }

        #[test]
        fn a_list_without_the_section_gets_it_at_the_end() {
            assert_eq!(
                set_defaults(
                    "[Added Associations]\nvideo/mp4=vlc.desktop;\n",
                    &["video/mp4"],
                    ID
                ),
                "[Added Associations]\nvideo/mp4=vlc.desktop;\n\n[Default Applications]\nvideo/mp4=plaa.desktop\n"
            );
        }

        #[test]
        fn the_first_listed_default_counts() {
            let text = "[Added Associations]\nvideo/mp4=vlc.desktop;\n[Default Applications]\nvideo/mp4=plaa.desktop;vlc.desktop;\n";
            assert_eq!(
                default_in(text, "video/mp4").as_deref(),
                Some("plaa.desktop")
            );
            assert_eq!(default_in(text, "audio/flac"), None);
        }

        #[test]
        fn setting_twice_changes_nothing() {
            let once = set_defaults("", &["video/mp4"], ID);
            assert_eq!(set_defaults(&once, &["video/mp4"], ID), once);
        }
    }
}
