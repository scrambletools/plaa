//! Where each file was left off, so it can start there next time. Kept in
//! `positions.toml` in the state folder, the most recent files only.

use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::{atomic, paths};

/// How many files are remembered; the ones left longest ago go first.
const KEPT: usize = 1000;
/// Positions this close to the start are not worth resuming from.
const NEAR_START: Duration = Duration::from_secs(10);
/// Positions this close to the end count as finished.
const NEAR_END: Duration = Duration::from_secs(15);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Entry {
    /// Seconds into the file.
    position: f64,
    /// When it was left, in seconds since 1970.
    left: u64,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Positions {
    #[serde(flatten)]
    entries: BTreeMap<String, Entry>,
}

pub fn default_path() -> Option<PathBuf> {
    paths::state_dir().map(|dir| dir.join("positions.toml"))
}

impl Positions {
    /// A missing or unreadable file yields no positions: losing them is
    /// better than refusing to play.
    pub fn load(path: &Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|text| toml::from_str(&text).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, path: &Path) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let text = toml::to_string(self).map_err(io::Error::other)?;
        atomic::write(path, text.as_bytes())
    }

    /// Where `file` was left off, if it is worth resuming.
    pub fn get(&self, file: &str) -> Option<Duration> {
        let entry = self.entries.get(file)?;
        (entry.position.is_finite() && entry.position > 0.0)
            .then(|| Duration::from_secs_f64(entry.position))
    }

    /// Remembers `position` in `file`, or forgets the file when the
    /// position is near its start or end.
    pub fn set(&mut self, file: &str, position: Duration, duration: Option<Duration>) {
        let finished = duration.is_some_and(|duration| position + NEAR_END >= duration);
        if position < NEAR_START || finished {
            self.entries.remove(file);
            return;
        }
        let left = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| since.as_secs());
        self.entries.insert(
            file.to_owned(),
            Entry {
                position: position.as_secs_f64(),
                left,
            },
        );
        if self.entries.len() > KEPT {
            let mut by_age: Vec<(u64, String)> = self
                .entries
                .iter()
                .map(|(file, entry)| (entry.left, file.clone()))
                .collect();
            by_age.sort();
            for (_, file) in by_age.into_iter().take(self.entries.len() - KEPT) {
                self.entries.remove(&file);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remembers_positions_away_from_the_ends() {
        let mut positions = Positions::default();
        let minute = Duration::from_secs(60);
        positions.set("/a.mkv", minute, Some(minute * 10));
        assert_eq!(positions.get("/a.mkv"), Some(minute));
        positions.set("/a.mkv", Duration::from_secs(3), Some(minute * 10));
        assert_eq!(positions.get("/a.mkv"), None);
        positions.set(
            "/b.mkv",
            minute * 10 - Duration::from_secs(5),
            Some(minute * 10),
        );
        assert_eq!(positions.get("/b.mkv"), None);
        positions.set("/live", minute, None);
        assert_eq!(positions.get("/live"), Some(minute));
    }

    #[test]
    fn round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("positions.toml");
        let mut positions = Positions::default();
        positions.set("/home/me/a film.mkv", Duration::from_secs(95), None);
        positions.save(&path).unwrap();
        assert_eq!(Positions::load(&path), positions);
    }

    #[test]
    fn keeps_only_the_most_recent() {
        let mut positions = Positions::default();
        for index in 0..KEPT + 5 {
            positions.set(&format!("/{index}"), Duration::from_secs(60), None);
        }
        assert_eq!(positions.entries.len(), KEPT);
    }
}
