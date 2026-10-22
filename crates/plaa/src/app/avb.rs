//! plaa on the AVB network: started on the interface Settings name, its
//! controller's view of the talkers and listeners kept for the windows to
//! show, and what the media window plays handed to plaa's own talker.

use std::sync::Arc;

use iced::window;
use plaa_avb::{EntityId, Network, Snapshot};

use super::{Content, Plaa};
use crate::External;
use crate::window::{Menu, Sheet};

impl Plaa {
    /// Starts plaa on the interface Settings name, or stops it where they
    /// name none, as at start and whenever they change.
    pub(crate) fn apply_avb(&mut self) {
        // The old one says goodbye before the new one says hello.
        self.stop_feeding();
        self.avb = None;
        self.avb_error = None;
        self.avb_snapshot = Arc::new(Snapshot::default());
        self.avb_watched = (false, Vec::new());
        let Some(interface) = self.settings.avb_interface.clone() else {
            return;
        };
        let config = plaa_avb::Config {
            interface: interface.clone(),
            talker: self.settings.avb_talker,
            name: self.avb_name(),
            address_file: plaa_store::paths::state_dir().map(|dir| dir.join("avb-address")),
        };
        match Network::start(config, || crate::post(External::Avb)) {
            Ok(network) => self.avb = Some(network),
            Err(error) => {
                self.avb_error = Some(crate::fl!(
                    "avb-could-not-start",
                    interface = interface,
                    error = error.to_string()
                ));
            }
        }
        self.feed_avb();
    }

    /// The entity's name: the one Settings give, or plaa's with the
    /// computer's.
    pub(crate) fn avb_name(&self) -> String {
        let name = self.settings.avb_name.trim();
        if name.is_empty() {
            default_name()
        } else {
            name.to_owned()
        }
    }

    /// The network changed: the windows show it anew.
    pub(crate) fn avb_changed(&mut self) {
        if let Some(network) = &self.avb {
            self.avb_snapshot = Arc::new(network.snapshot());
        }
    }

    pub(crate) fn avb_connect(
        &self,
        talker: (EntityId, u16),
        listener: (EntityId, u16),
        format: Option<plaa_avb::StreamFormat>,
    ) {
        if let Some(network) = &self.avb {
            network.connect(talker, listener, format);
        }
    }

    pub(crate) fn avb_disconnect(&self, listener: (EntityId, u16)) {
        if let Some(network) = &self.avb {
            network.disconnect(listener);
        }
    }

    /// Tells plaa's AVB side what the windows show of the network, which
    /// it keeps up to date: whether any lists the talkers and listeners,
    /// in the Network Audio sheet or a Connect menu, and the listeners
    /// chosen in the sheet.
    pub(crate) fn watch_avb(&mut self) {
        let Some(network) = &self.avb else {
            return;
        };
        let mut listed = self.start_connect.is_some_and(|id| {
            self.windows
                .get(&id)
                .is_some_and(|window| matches!(window.content, Content::Start))
        });
        let mut chosen = Vec::new();
        for window in self.windows.values() {
            let Some(player) = window.player() else {
                continue;
            };
            if player.sheet == Some(Sheet::Network) {
                listed = true;
                chosen.extend(player.listener);
            }
            listed |= player.menu == Some(Menu::Connect);
        }
        let watched = (listed, chosen);
        if self.avb_watched != watched {
            network.watch(watched.0, watched.1.clone());
            self.avb_watched = watched;
        }
    }

    /// Hands what the media window plays to plaa's talker, and no other
    /// window's: the window last played or used, as for the media keys.
    pub(crate) fn feed_avb(&mut self) {
        let sending = self.avb.is_some() && self.settings.avb_talker;
        let wanted = self
            .media_window()
            .filter(|_| sending)
            .and_then(|id| Some((id, self.windows.get(&id)?.player()?.surface)));
        if wanted == self.avb_feeding {
            return;
        }
        self.stop_feeding();
        let Some((id, _)) = wanted else {
            return;
        };
        let Some(sound) = self.avb.as_ref().map(Network::sound) else {
            return;
        };
        if let Some(player) = self.windows.get(&id).and_then(|window| window.player()) {
            player
                .player
                .set_copy(Some(Arc::new(move |frames: &[f32], rate| {
                    sound.take(frames, rate);
                })));
            self.avb_feeding = wanted;
        }
    }

    /// The window being sent stops handing its sound over.
    fn stop_feeding(&mut self) {
        if let Some((id, surface)) = self.avb_feeding.take()
            && let Some(window) = self.windows.get(&id)
            && let Content::Player(player) = &window.content
            && player.surface == surface
        {
            player.player.set_copy(None);
        }
    }

    /// The window `id` closed: it was perhaps the one being sent, or the
    /// one with its Connect menu open.
    pub(crate) fn avb_window_closed(&mut self, id: window::Id) {
        if self.avb_feeding.is_some_and(|(feeding, _)| feeding == id) {
            self.avb_feeding = None;
        }
        if self.start_connect == Some(id) {
            self.start_connect = None;
        }
    }
}

/// plaa's entity name, with the computer's name where it can be read.
fn default_name() -> String {
    let computer = std::fs::read_to_string("/proc/sys/kernel/hostname")
        .ok()
        .or_else(|| std::env::var("COMPUTERNAME").ok())
        .map(|name| name.trim().to_owned())
        .filter(|name| !name.is_empty());
    match computer {
        Some(computer) => crate::fl!("avb-default-name", computer = computer),
        None => "plaa".to_owned(),
    }
}
