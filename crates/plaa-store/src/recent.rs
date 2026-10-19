//! The addresses opened lately, latest first, with what each is called,
//! for the Open Address dialog to offer again. Kept in `recent.toml` in
//! the state folder.

use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{atomic, paths};

/// How many addresses are remembered.
const KEPT: usize = 20;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Address {
    pub url: String,
    /// What it is called, as its page or stream says, once it has opened.
    pub title: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Recent {
    addresses: Vec<Address>,
}

pub fn default_path() -> Option<PathBuf> {
    paths::state_dir().map(|dir| dir.join("recent.toml"))
}

impl Recent {
    /// A missing or unreadable file yields none.
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

    /// The addresses, latest first.
    pub fn addresses(&self) -> &[Address] {
        &self.addresses
    }

    /// Puts `url` first, keeping the title it had until it has another.
    pub fn add(&mut self, url: &str) {
        let title = self
            .addresses
            .iter()
            .position(|address| address.url == url)
            .and_then(|at| self.addresses.remove(at).title);
        self.addresses.insert(
            0,
            Address {
                url: url.to_owned(),
                title,
            },
        );
        self.addresses.truncate(KEPT);
    }

    /// Names `url`, if it is among them. Returns whether that changed it.
    pub fn name(&mut self, url: &str, title: &str) -> bool {
        let title = title.trim();
        match self.addresses.iter_mut().find(|address| address.url == url) {
            Some(address) if !title.is_empty() && address.title.as_deref() != Some(title) => {
                address.title = Some(title.to_owned());
                true
            }
            _ => false,
        }
    }

    pub fn remove(&mut self, url: &str) {
        self.addresses.retain(|address| address.url != url);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_latest_comes_first_once() {
        let mut recent = Recent::default();
        recent.add("https://a.example/live.m3u8");
        recent.add("https://b.example/watch");
        assert!(recent.name("https://a.example/live.m3u8", "Live"));
        assert!(!recent.name("https://a.example/live.m3u8", "Live"));
        recent.add("https://a.example/live.m3u8");
        let urls: Vec<_> = recent.addresses().iter().map(|a| a.url.as_str()).collect();
        assert_eq!(
            urls,
            ["https://a.example/live.m3u8", "https://b.example/watch"]
        );
        // Opening it again keeps its name.
        assert_eq!(recent.addresses()[0].title.as_deref(), Some("Live"));
        recent.remove("https://b.example/watch");
        assert_eq!(recent.addresses().len(), 1);
    }

    #[test]
    fn keeps_only_the_latest() {
        let mut recent = Recent::default();
        for index in 0..KEPT + 3 {
            recent.add(&format!("https://example.com/{index}"));
        }
        assert_eq!(recent.addresses().len(), KEPT);
        assert_eq!(
            recent.addresses()[0].url,
            format!("https://example.com/{}", KEPT + 2)
        );
    }

    #[test]
    fn round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("recent.toml");
        let mut recent = Recent::default();
        recent.add("https://example.com/a");
        recent.name("https://example.com/a", "A");
        recent.save(&path).unwrap();
        assert_eq!(Recent::load(&path), recent);
    }
}
