//! What plays, told to macOS's Now Playing (Control Center and the menu
//! bar's widget), and the media keys and those controls acting on plaa.
//! AppKit calls the handlers on the main thread, where iced runs.

#![allow(unsafe_code)]

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::ptr::NonNull;
use std::sync::Arc;

use block2::RcBlock;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{AllocAnyThread, MainThreadMarker};
use objc2_app_kit::NSImage;
use objc2_core_foundation::CGSize;
use objc2_foundation::{NSDictionary, NSNumber, NSString};
use objc2_media_player::{
    MPChangePlaybackPositionCommandEvent, MPMediaItemArtwork, MPMediaItemPropertyAlbumTitle,
    MPMediaItemPropertyArtist, MPMediaItemPropertyArtwork, MPMediaItemPropertyPlaybackDuration,
    MPMediaItemPropertyTitle, MPNowPlayingInfoCenter, MPNowPlayingInfoPropertyElapsedPlaybackTime,
    MPNowPlayingInfoPropertyPlaybackRate, MPNowPlayingPlaybackState, MPRemoteCommand,
    MPRemoteCommandCenter, MPRemoteCommandEvent, MPRemoteCommandHandlerStatus,
};

use crate::media::{Command, NowPlaying, Playback};

type Commands = Arc<dyn Fn(Command) + Send + Sync>;

/// Now Playing, kept up to date while plaa runs.
pub struct Service {
    told: RefCell<NowPlaying>,
    /// The cover shown, and the file it came from.
    artwork: RefCell<Option<(PathBuf, Retained<MPMediaItemArtwork>)>>,
}

impl Service {
    /// Takes the media keys and Now Playing's controls, sending what is
    /// asked to `commands`. Call it on the main thread.
    pub fn start(_name: &str, commands: impl Fn(Command) + Send + Sync + 'static) -> Option<Self> {
        MainThreadMarker::new()?;
        let send: Commands = Arc::new(commands);
        // SAFETY: the command center is shared and lives as long as plaa;
        // each handler only sends a command on.
        unsafe {
            let center = MPRemoteCommandCenter::sharedCommandCenter();
            for (command, asked) in [
                (center.playCommand(), Command::Play),
                (center.pauseCommand(), Command::Pause),
                (center.togglePlayPauseCommand(), Command::PlayPause),
                (center.stopCommand(), Command::Stop),
                (center.nextTrackCommand(), Command::Next),
                (center.previousTrackCommand(), Command::Previous),
            ] {
                let send = send.clone();
                handle(&command, move |_| {
                    send(asked.clone());
                    MPRemoteCommandHandlerStatus::Success
                });
            }
            let send = send.clone();
            handle(&center.changePlaybackPositionCommand(), move |event| {
                // SAFETY: this command's events are position changes.
                let event = &*(event as *const MPRemoteCommandEvent)
                    .cast::<MPChangePlaybackPositionCommandEvent>();
                send(Command::SeekTo(event.positionTime()));
                MPRemoteCommandHandlerStatus::Success
            });
        }
        Some(Self {
            told: RefCell::new(NowPlaying::default()),
            artwork: RefCell::new(None),
        })
    }

    /// Tells Now Playing what plays now, when that changed in more than
    /// the time going on, which it counts itself.
    pub fn update(&self, now: NowPlaying) {
        let before = self.told.replace(now.clone());
        let changed = now.item_changed(&before)
            || now.playback != before.playback
            || now.rate != before.rate
            || now.jumped(&before);
        if changed {
            self.tell(&now);
        }
        if now.can_next != before.can_next
            || now.can_previous != before.can_previous
            || now.can_seek != before.can_seek
        {
            // SAFETY: as in `start`.
            unsafe {
                let center = MPRemoteCommandCenter::sharedCommandCenter();
                center.nextTrackCommand().setEnabled(now.can_next);
                center.previousTrackCommand().setEnabled(now.can_previous);
                center
                    .changePlaybackPositionCommand()
                    .setEnabled(now.can_seek);
            }
        }
    }

    fn tell(&self, now: &NowPlaying) {
        // SAFETY: the keys are MediaPlayer's own, and each value has the
        // type its key asks for.
        unsafe {
            let center = MPNowPlayingInfoCenter::defaultCenter();
            if now.title.is_empty() {
                center.setNowPlayingInfo(None);
                center.setPlaybackState(MPNowPlayingPlaybackState::Stopped);
                return;
            }
            let title = NSString::from_str(&now.title);
            let artist = now.artist.as_deref().map(NSString::from_str);
            let album = now.album.as_deref().map(NSString::from_str);
            let length = now
                .length
                .map(|length| NSNumber::new_f64(length.as_secs_f64()));
            let elapsed = NSNumber::new_f64(now.position.as_secs_f64());
            let playing = now.playback == Playback::Playing;
            let rate = NSNumber::new_f64(if playing { now.rate } else { 0.0 });
            let artwork = self.artwork(now.art.as_deref());
            let mut keys: Vec<&NSString> = vec![
                MPMediaItemPropertyTitle,
                MPNowPlayingInfoPropertyElapsedPlaybackTime,
                MPNowPlayingInfoPropertyPlaybackRate,
            ];
            let mut values: Vec<&AnyObject> =
                vec![as_object(&*title), as_object(&*elapsed), as_object(&*rate)];
            for (key, value) in [
                (MPMediaItemPropertyArtist, artist.as_deref().map(as_object)),
                (
                    MPMediaItemPropertyAlbumTitle,
                    album.as_deref().map(as_object),
                ),
                (
                    MPMediaItemPropertyPlaybackDuration,
                    length.as_deref().map(as_object),
                ),
                (
                    MPMediaItemPropertyArtwork,
                    artwork.as_deref().map(as_object),
                ),
            ] {
                if let Some(value) = value {
                    keys.push(key);
                    values.push(value);
                }
            }
            let info = NSDictionary::from_slices(&keys, &values);
            center.setNowPlayingInfo(Some(&info));
            center.setPlaybackState(match now.playback {
                Playback::Playing => MPNowPlayingPlaybackState::Playing,
                Playback::Paused => MPNowPlayingPlaybackState::Paused,
                Playback::Stopped => MPNowPlayingPlaybackState::Stopped,
            });
        }
    }

    /// The artwork for the cover saved at `path`, made once per cover.
    fn artwork(&self, path: Option<&Path>) -> Option<Retained<MPMediaItemArtwork>> {
        let path = path?;
        let mut kept = self.artwork.borrow_mut();
        if let Some((shown, artwork)) = &*kept
            && shown == path
        {
            return Some(artwork.clone());
        }
        let file = NSString::from_str(&path.to_string_lossy());
        let image = NSImage::initWithContentsOfFile(NSImage::alloc(), &file)?;
        let size: CGSize = image.size();
        let handler = RcBlock::new(move |_: CGSize| NonNull::from(&*image));
        // SAFETY: the block keeps the image, so the pointer it returns
        // stays good while the artwork asks for it.
        let artwork = unsafe {
            MPMediaItemArtwork::initWithBoundsSize_requestHandler(
                MPMediaItemArtwork::alloc(),
                size,
                &handler,
            )
        };
        *kept = Some((path.to_owned(), artwork.clone()));
        Some(artwork)
    }
}

fn as_object<T: AsRef<AnyObject>>(value: &T) -> &AnyObject {
    value.as_ref()
}

/// Runs `handler` for each of `command`'s events, and enables it.
///
/// # Safety
///
/// Call on the main thread.
unsafe fn handle(
    command: &MPRemoteCommand,
    handler: impl Fn(&MPRemoteCommandEvent) -> MPRemoteCommandHandlerStatus + 'static,
) {
    let block = RcBlock::new(move |event: NonNull<MPRemoteCommandEvent>| {
        // SAFETY: MediaPlayer passes an event that lives for the call.
        handler(unsafe { event.as_ref() })
    });
    // SAFETY: as the caller ensures; the center keeps the block.
    unsafe {
        command.addTargetWithHandler(&block);
        command.setEnabled(true);
    }
}
