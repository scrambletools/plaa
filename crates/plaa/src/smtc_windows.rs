//! What plays, told to Windows' System Media Transport Controls (the media
//! flyout and the lock screen), and the media keys and those controls
//! acting on plaa.

#![allow(unsafe_code)]

use std::cell::{Cell, RefCell};
use std::sync::Arc;
use std::time::{Duration, Instant};

use windows::Foundation::{TimeSpan, TypedEventHandler};
use windows::Media::{
    MediaPlaybackStatus, MediaPlaybackType, PlaybackPositionChangeRequestedEventArgs,
    SystemMediaTransportControls, SystemMediaTransportControlsButton,
    SystemMediaTransportControlsButtonPressedEventArgs,
    SystemMediaTransportControlsTimelineProperties,
};
use windows::Storage::StorageFile;
use windows::Storage::Streams::RandomAccessStreamReference;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::WinRT::ISystemMediaTransportControlsInterop;
use windows::core::{HSTRING, Ref, factory};

use crate::media::{Command, NowPlaying, Playback};

type Commands = Arc<dyn Fn(Command) + Send + Sync>;

/// How often the position is told again while playing: the media flyout
/// shows it as told rather than counting on.
const TIMELINE_EVERY: Duration = Duration::from_secs(5);

/// The transport controls, kept up to date while plaa runs.
pub struct Service {
    controls: SystemMediaTransportControls,
    told: RefCell<NowPlaying>,
    /// When the position was last told.
    timeline_told: Cell<Instant>,
}

impl Service {
    /// Takes the media keys and the system's controls, sending what is
    /// asked to `commands`. None when Windows has no controls to give.
    pub fn start(_name: &str, commands: impl Fn(Command) + Send + Sync + 'static) -> Option<Self> {
        let send: Commands = Arc::new(commands);
        let window = hidden_window()?;
        let interop =
            factory::<SystemMediaTransportControls, ISystemMediaTransportControlsInterop>().ok()?;
        // SAFETY: the window is plaa's own and stays open while it runs.
        let controls: SystemMediaTransportControls =
            unsafe { interop.GetForWindow(window) }.ok()?;
        let pressed = send.clone();
        controls
            .ButtonPressed(&TypedEventHandler::new(
                move |_, event: Ref<SystemMediaTransportControlsButtonPressedEventArgs>| {
                    let command = match event.ok()?.Button()? {
                        SystemMediaTransportControlsButton::Play => Command::Play,
                        SystemMediaTransportControlsButton::Pause => Command::Pause,
                        SystemMediaTransportControlsButton::Stop => Command::Stop,
                        SystemMediaTransportControlsButton::Next => Command::Next,
                        SystemMediaTransportControlsButton::Previous => Command::Previous,
                        _ => return Ok(()),
                    };
                    pressed(command);
                    Ok(())
                },
            ))
            .ok()?;
        controls
            .PlaybackPositionChangeRequested(&TypedEventHandler::new(
                move |_, event: Ref<PlaybackPositionChangeRequestedEventArgs>| {
                    let position = event.ok()?.RequestedPlaybackPosition()?;
                    send(Command::SeekTo(position.Duration as f64 / 10_000_000.0));
                    Ok(())
                },
            ))
            .ok()?;
        for enable in [
            SystemMediaTransportControls::SetIsPlayEnabled,
            SystemMediaTransportControls::SetIsPauseEnabled,
            SystemMediaTransportControls::SetIsStopEnabled,
        ] {
            enable(&controls, true).ok()?;
        }
        Some(Self {
            controls,
            told: RefCell::new(NowPlaying::default()),
            timeline_told: Cell::new(Instant::now()),
        })
    }

    /// Tells the controls what plays now, when that changed in more than
    /// the time going on, which they count themselves.
    pub fn update(&self, now: NowPlaying) {
        let before = self.told.replace(now.clone());
        // Failures leave the controls as they were; there is nothing else
        // to do with them.
        if now.item_changed(&before) {
            let _ = self.show_item(&now);
        }
        if now.playback != before.playback || now.title.is_empty() != before.title.is_empty() {
            let _ = self.controls.SetIsEnabled(!now.title.is_empty());
            let _ = self.controls.SetPlaybackStatus(match now.playback {
                Playback::Playing => MediaPlaybackStatus::Playing,
                Playback::Paused => MediaPlaybackStatus::Paused,
                Playback::Stopped => MediaPlaybackStatus::Stopped,
            });
        }
        if now.can_next != before.can_next || now.can_previous != before.can_previous {
            let _ = self.controls.SetIsNextEnabled(now.can_next);
            let _ = self.controls.SetIsPreviousEnabled(now.can_previous);
        }
        if now.rate != before.rate {
            let _ = self.controls.SetPlaybackRate(now.rate);
        }
        let stale = now.playback == Playback::Playing
            && self.timeline_told.get().elapsed() >= TIMELINE_EVERY;
        if now.item_changed(&before)
            || now.playback != before.playback
            || now.jumped(&before)
            || stale
        {
            let _ = self.show_timeline(&now);
            self.timeline_told.set(Instant::now());
        }
    }

    fn show_item(&self, now: &NowPlaying) -> windows::core::Result<()> {
        let display = self.controls.DisplayUpdater()?;
        display.ClearAll()?;
        if now.title.is_empty() {
            return display.Update();
        }
        // Music shows the artist and album; anything else shows as video.
        if now.artist.is_some() || now.album.is_some() {
            display.SetType(MediaPlaybackType::Music)?;
            let music = display.MusicProperties()?;
            music.SetTitle(&HSTRING::from(&now.title))?;
            if let Some(artist) = &now.artist {
                music.SetArtist(&HSTRING::from(artist))?;
            }
            if let Some(album) = &now.album {
                music.SetAlbumTitle(&HSTRING::from(album))?;
            }
        } else {
            display.SetType(MediaPlaybackType::Video)?;
            display
                .VideoProperties()?
                .SetTitle(&HSTRING::from(&now.title))?;
        }
        if let Some(art) = &now.art {
            let file = StorageFile::GetFileFromPathAsync(&HSTRING::from(art.as_path()))?.join()?;
            display.SetThumbnail(&RandomAccessStreamReference::CreateFromFile(&file)?)?;
        }
        display.Update()
    }

    fn show_timeline(&self, now: &NowPlaying) -> windows::core::Result<()> {
        let length = now.length.unwrap_or_default();
        let timeline = SystemMediaTransportControlsTimelineProperties::new()?;
        timeline.SetStartTime(time_span(Duration::ZERO))?;
        timeline.SetEndTime(time_span(length))?;
        timeline.SetMinSeekTime(time_span(Duration::ZERO))?;
        timeline.SetMaxSeekTime(time_span(if now.can_seek {
            length
        } else {
            Duration::ZERO
        }))?;
        timeline.SetPosition(time_span(now.position.min(length)))?;
        self.controls.UpdateTimelineProperties(&timeline)
    }
}

/// `duration` in the hundreds of nanoseconds Windows counts in.
fn time_span(duration: Duration) -> TimeSpan {
    TimeSpan {
        Duration: (duration.as_nanos() / 100) as i64,
    }
}

/// A window of plaa's that is never shown, which the controls belong to:
/// they need one, and plaa's own windows come and go.
fn hidden_window() -> Option<HWND> {
    use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        CreateWindowExW, DefWindowProcW, RegisterClassW, WNDCLASSW,
    };
    let class: Vec<u16> = "plaa media controls\0".encode_utf16().collect();
    // SAFETY: the class name outlives both calls, and the window procedure
    // is the system's default.
    unsafe {
        let instance = GetModuleHandleW(std::ptr::null());
        let window_class = WNDCLASSW {
            lpfnWndProc: Some(DefWindowProcW),
            hInstance: instance,
            lpszClassName: class.as_ptr(),
            ..std::mem::zeroed()
        };
        RegisterClassW(&window_class);
        let window = CreateWindowExW(
            0,
            class.as_ptr(),
            class.as_ptr(),
            0,
            0,
            0,
            0,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            instance,
            std::ptr::null(),
        );
        (!window.is_null()).then_some(HWND(window))
    }
}
