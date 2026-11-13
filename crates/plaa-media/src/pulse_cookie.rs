//! The PulseAudio cookie, made where none is yet, as libpulse makes it on
//! first use.
//!
//! cpal reaches the sound server through its own PulseAudio client, which
//! offers the cookie at `$PULSE_COOKIE`, `~/.config/pulse/cookie` or
//! `~/.pulse-cookie`, and an empty one where there is none. PipeWire's
//! PulseAudio server refuses an empty cookie, and cpal then falls back to
//! ALSA, where the sound server's devices, the ones avb-endpointd makes
//! among them, have no names to find. That happens on a computer where no
//! libpulse program has run yet. PipeWire ignores what the cookie holds,
//! and PulseAudio itself makes its cookie when it starts, so plaa makes a
//! random one only where none exists.

use std::fs::{DirBuilder, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

/// libpulse's cookie length, which PulseAudio's servers expect.
const LENGTH: usize = 256;

/// Makes the cookie in the person's home folder where they have none and
/// `$PULSE_COOKIE` names none. Call it before anything opens a sound
/// device. Says on standard error where it could not.
pub fn ensure() {
    if std::env::var_os("PULSE_COOKIE").is_some() {
        return;
    }
    let Some(home) = std::env::var_os("HOME").filter(|home| !home.is_empty()) else {
        return;
    };
    if let Err(error) = ensure_in(Path::new(&home)) {
        eprintln!("plaa: could not make a PulseAudio cookie: {error}");
    }
}

/// Makes `home`'s cookie where it has none, giving the path it made.
fn ensure_in(home: &Path) -> io::Result<Option<PathBuf>> {
    let path = home.join(".config/pulse/cookie");
    if path.exists() || home.join(".pulse-cookie").exists() {
        return Ok(None);
    }
    let mut cookie = [0u8; LENGTH];
    std::fs::File::open("/dev/urandom")?.read_exact(&mut cookie)?;
    if let Some(folder) = path.parent() {
        DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(folder)?;
    }
    let mut file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&path)
    {
        Ok(file) => file,
        // Another program made it first.
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => return Ok(None),
        Err(error) => return Err(error),
    };
    if let Err(error) = file.write_all(&cookie).and_then(|()| file.sync_all()) {
        // A short cookie would be refused as an empty one is.
        let _ = std::fs::remove_file(&path);
        return Err(error);
    }
    Ok(Some(path))
}

#[cfg(test)]
mod tests {
    use std::os::unix::fs::PermissionsExt;

    use super::*;

    #[test]
    fn makes_a_private_cookie_of_libpulses_length() {
        let home = tempfile::tempdir().unwrap();
        let made = ensure_in(home.path()).unwrap().unwrap();
        assert_eq!(made, home.path().join(".config/pulse/cookie"));
        let bytes = std::fs::read(&made).unwrap();
        assert_eq!(bytes.len(), LENGTH);
        assert!(bytes.iter().any(|&byte| byte != 0));
        let mode = std::fs::metadata(&made).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o600);
    }

    #[test]
    fn keeps_a_cookie_already_there() {
        let home = tempfile::tempdir().unwrap();
        let folder = home.path().join(".config/pulse");
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(folder.join("cookie"), b"theirs").unwrap();
        assert_eq!(ensure_in(home.path()).unwrap(), None);
        assert_eq!(std::fs::read(folder.join("cookie")).unwrap(), b"theirs");
    }

    #[test]
    fn takes_the_old_cookie_place_as_one() {
        let home = tempfile::tempdir().unwrap();
        std::fs::write(home.path().join(".pulse-cookie"), b"old").unwrap();
        assert_eq!(ensure_in(home.path()).unwrap(), None);
        assert!(!home.path().join(".config/pulse/cookie").exists());
    }
}
