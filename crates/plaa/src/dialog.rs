//! File dialogs, to open and to save: through the XDG desktop portal on
//! Linux, and the system's own dialogs (through rfd) elsewhere. Links open
//! in the browser the same ways.

#[cfg(target_os = "linux")]
use std::path::PathBuf;

#[cfg(target_os = "linux")]
pub use portal::*;

#[cfg(not(target_os = "linux"))]
pub use native::*;

/// Media types the open dialog offers. Wildcards cover what FFmpeg reads;
/// the rest are containers registered under other types.
#[cfg(target_os = "linux")]
const MIME_TYPES: &[&str] = &[
    "video/*",
    "audio/*",
    "application/ogg",
    "application/mxf",
    "application/vnd.rn-realmedia",
    "application/x-matroska",
    "application/vnd.apple.mpegurl",
    "application/x-mpegurl",
    "application/dash+xml",
];

/// File name extensions the open dialog offers where it filters by them.
#[cfg(not(target_os = "linux"))]
const EXTENSIONS: &[&str] = &[
    "3g2", "3gp", "aac", "ac3", "aif", "aiff", "alac", "amr", "ape", "asf", "avi", "caf", "dts",
    "dv", "eac3", "f4v", "flac", "flv", "m2ts", "m2v", "m3u8", "m4a", "m4b", "m4v", "mka", "mkv",
    "mov", "mp2", "mp3", "mp4", "mpc", "mpd", "mpeg", "mpg", "mts", "mxf", "nut", "oga", "ogg",
    "ogv", "opus", "ra", "rm", "rmvb", "spx", "tak", "ts", "tta", "vob", "wav", "weba", "webm",
    "wma", "wmv", "wv", "y4m",
];

#[cfg(target_os = "linux")]
mod portal {
    use std::path::PathBuf;

    use ashpd::desktop::file_chooser::{FileFilter, SelectedFiles};

    use super::{MIME_TYPES, file_uri_to_path};

    /// Asks the user for files to play. Cancelling yields an empty list.
    pub async fn open_files() -> Result<Vec<PathBuf>, String> {
        let media = MIME_TYPES.iter().fold(
            FileFilter::new(&crate::fl!("dialog-filter-media")),
            |filter, mime| filter.mimetype(mime),
        );
        let everything = FileFilter::new(&crate::fl!("dialog-filter-all")).glob("*");
        let title = crate::fl!("dialog-open");
        let request = SelectedFiles::open_file()
            .title(title.as_str())
            .multiple(true)
            .filters([media.clone(), everything])
            .current_filter(media)
            .send()
            .await
            .map_err(|error| error.to_string())?;
        match request.response() {
            Ok(selected) => Ok(selected
                .uris()
                .iter()
                .filter_map(|uri| file_uri_to_path(uri.as_str()))
                .collect()),
            // Cancelled, or closed some other way: either way the user
            // chose nothing.
            Err(ashpd::Error::Response(_)) => Ok(Vec::new()),
            Err(error) => Err(error.to_string()),
        }
    }

    /// Asks where to save a file, starting as `name` in `folder`.
    /// Cancelling yields `None`.
    pub async fn save_file(
        title: String,
        name: String,
        folder: Option<PathBuf>,
    ) -> Result<Option<PathBuf>, String> {
        let request = SelectedFiles::save_file()
            .title(title.as_str())
            .current_name(name.as_str())
            .current_folder::<&std::path::Path>(folder.as_deref())
            .map_err(|error| error.to_string())?
            .send()
            .await
            .map_err(|error| error.to_string())?;
        match request.response() {
            Ok(selected) => Ok(selected
                .uris()
                .first()
                .and_then(|uri| file_uri_to_path(uri.as_str()))),
            Err(ashpd::Error::Response(_)) => Ok(None),
            Err(error) => Err(error.to_string()),
        }
    }

    /// Asks for a folder, starting in `current`. Cancelling yields `None`.
    pub async fn choose_folder(
        title: String,
        current: Option<PathBuf>,
    ) -> Result<Option<PathBuf>, String> {
        let request = SelectedFiles::open_file()
            .title(title.as_str())
            .directory(true)
            .current_folder::<&std::path::Path>(current.as_deref())
            .map_err(|error| error.to_string())?
            .send()
            .await
            .map_err(|error| error.to_string())?;
        match request.response() {
            Ok(selected) => Ok(selected
                .uris()
                .first()
                .and_then(|uri| file_uri_to_path(uri.as_str()))),
            Err(ashpd::Error::Response(_)) => Ok(None),
            Err(error) => Err(error.to_string()),
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod native {
    use std::path::PathBuf;

    use rfd::AsyncFileDialog;

    use super::EXTENSIONS;

    /// Asks the user for files to play. Cancelling yields an empty list.
    pub async fn open_files() -> Result<Vec<PathBuf>, String> {
        let files = AsyncFileDialog::new()
            .set_title(crate::fl!("dialog-open"))
            .add_filter(crate::fl!("dialog-filter-media"), EXTENSIONS)
            .add_filter(crate::fl!("dialog-filter-all"), &["*"])
            .pick_files()
            .await
            .unwrap_or_default();
        Ok(files
            .into_iter()
            .map(|file| file.path().to_path_buf())
            .collect())
    }

    /// Asks where to save a file, starting as `name` in `folder`.
    /// Cancelling yields `None`.
    pub async fn save_file(
        title: String,
        name: String,
        folder: Option<PathBuf>,
    ) -> Result<Option<PathBuf>, String> {
        let mut dialog = AsyncFileDialog::new().set_title(title).set_file_name(name);
        if let Some(folder) = folder {
            dialog = dialog.set_directory(folder);
        }
        Ok(dialog
            .save_file()
            .await
            .map(|file| file.path().to_path_buf()))
    }

    /// Asks for a folder, starting in `current`. Cancelling yields `None`.
    pub async fn choose_folder(
        title: String,
        current: Option<PathBuf>,
    ) -> Result<Option<PathBuf>, String> {
        let mut dialog = AsyncFileDialog::new().set_title(title);
        if let Some(current) = current {
            dialog = dialog.set_directory(current);
        }
        Ok(dialog
            .pick_folder()
            .await
            .map(|folder| folder.path().to_path_buf()))
    }
}

/// Converts a local `file://` URI, as the portal gives, to a path,
/// decoding percent escapes.
#[cfg(target_os = "linux")]
pub fn file_uri_to_path(uri: &str) -> Option<PathBuf> {
    use std::os::unix::ffi::OsStringExt;

    let rest = uri.strip_prefix("file://")?;
    let path = rest.strip_prefix("localhost").unwrap_or(rest);
    if !path.starts_with('/') {
        return None;
    }
    let mut bytes = Vec::with_capacity(path.len());
    let mut input = path.bytes();
    while let Some(byte) = input.next() {
        if byte == b'%' {
            let high = input
                .next()
                .and_then(|digit| (digit as char).to_digit(16))?;
            let low = input
                .next()
                .and_then(|digit| (digit as char).to_digit(16))?;
            bytes.push((high * 16 + low) as u8);
        } else {
            bytes.push(byte);
        }
    }
    Some(PathBuf::from(std::ffi::OsString::from_vec(bytes)))
}

/// Opens `uri`, such as a link in the assistant's answer, in the browser
/// or the program the system has for it.
pub async fn open_link(uri: String) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        let parsed = ashpd::Uri::parse(&uri).map_err(|error| error.to_string())?;
        ashpd::desktop::open_uri::OpenFileRequest::default()
            .send_uri(&parsed)
            .await
            .map(drop)
            .map_err(|error| error.to_string())
    }
    #[cfg(not(target_os = "linux"))]
    {
        #[cfg(windows)]
        let mut command = std::process::Command::new("rundll32");
        #[cfg(windows)]
        command.args(["url.dll,FileProtocolHandler", &uri]);
        #[cfg(not(windows))]
        let mut command = std::process::Command::new("open");
        #[cfg(not(windows))]
        command.arg(&uri);
        command.spawn().map(drop).map_err(|error| error.to_string())
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    #[test]
    fn decodes_file_uris() {
        assert_eq!(
            file_uri_to_path("file:///home/me/a%20b.mkv"),
            Some(PathBuf::from("/home/me/a b.mkv"))
        );
        assert_eq!(
            file_uri_to_path("file://localhost/tmp/x.mp3"),
            Some(PathBuf::from("/tmp/x.mp3"))
        );
        assert_eq!(file_uri_to_path("file://server/share/x.mkv"), None);
        assert_eq!(file_uri_to_path("https://example.com/x.mp4"), None);
        assert_eq!(file_uri_to_path("file:///bad%zz"), None);
    }
}
