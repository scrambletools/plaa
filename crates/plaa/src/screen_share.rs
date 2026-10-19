//! Asking the system to share a screen or a window for recording: the
//! ScreenCast portal shows the system's own picker, then hands over the
//! PipeWire stream of what was chosen.

use std::os::fd::OwnedFd;
use std::sync::{Arc, Mutex, PoisonError};

use ashpd::desktop::PersistMode;
use ashpd::desktop::Session;
use ashpd::desktop::screencast::{CursorMode, Screencast, SelectSourcesOptions, SourceType};

/// What the system shared, once: [`Shared::take`] hands it over to the
/// window that records it, as messages are copied.
#[derive(Clone)]
pub struct Shared(Arc<Mutex<Option<Sharing>>>);

impl std::fmt::Debug for Shared {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Shared")
    }
}

impl Shared {
    pub fn take(&self) -> Option<Sharing> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner).take()
    }
}

/// A screen or window being shared: the stream to read it from, and the
/// session that keeps it shared, which ends once this is dropped.
pub struct Sharing {
    fd: Option<OwnedFd>,
    node: u32,
    /// A window is shared, not a whole screen.
    pub window: bool,
    session: Option<Session<Screencast>>,
}

impl Sharing {
    /// The PipeWire remote to read the stream from, once, and the node
    /// that is the stream.
    pub fn stream(&mut self) -> Option<(OwnedFd, u32)> {
        Some((self.fd.take()?, self.node))
    }
}

impl Drop for Sharing {
    fn drop(&mut self) {
        // The compositor keeps copying the screen until told to stop.
        if let Some(session) = self.session.take() {
            std::thread::spawn(move || {
                let _ = iced::futures::executor::block_on(session.close());
            });
        }
    }
}

/// Asks the system for a screen or a window to record. None when the
/// person cancels.
pub async fn share() -> Result<Option<Shared>, String> {
    let failed = |error: ashpd::Error| error.to_string();
    let portal = Screencast::new().await.map_err(failed)?;
    let cursors = portal.available_cursor_modes().await.unwrap_or_default();
    // The pointer is recorded where it is, as the person sees it.
    let cursor = if cursors.contains(CursorMode::Embedded) {
        CursorMode::Embedded
    } else {
        CursorMode::Hidden
    };
    let session = portal
        .create_session(Default::default())
        .await
        .map_err(failed)?;
    let selected = portal
        .select_sources(
            &session,
            SelectSourcesOptions::default()
                .set_cursor_mode(cursor)
                .set_sources(SourceType::Monitor | SourceType::Window)
                .set_multiple(false)
                .set_persist_mode(PersistMode::DoNot),
        )
        .await
        .map_err(failed)?;
    if let Err(error) = selected.response() {
        let _ = session.close().await;
        return cancelled_or(error);
    }
    let started = portal
        .start(&session, None, Default::default())
        .await
        .map_err(failed)?;
    let streams = match started.response() {
        Ok(streams) => streams,
        Err(error) => {
            let _ = session.close().await;
            return cancelled_or(error);
        }
    };
    let Some(stream) = streams.streams().first() else {
        let _ = session.close().await;
        return Ok(None);
    };
    let (node, window) = (
        stream.pipe_wire_node_id(),
        stream.source_type() == Some(SourceType::Window),
    );
    let fd = match portal
        .open_pipe_wire_remote(&session, Default::default())
        .await
    {
        Ok(fd) => fd,
        Err(error) => {
            let _ = session.close().await;
            return Err(error.to_string());
        }
    };
    Ok(Some(Shared(Arc::new(Mutex::new(Some(Sharing {
        fd: Some(fd),
        node,
        window,
        session: Some(session),
    }))))))
}

/// Cancelling the picker, or closing it, chose nothing.
fn cancelled_or(error: ashpd::Error) -> Result<Option<Shared>, String> {
    match error {
        ashpd::Error::Response(_) => Ok(None),
        error => Err(error.to_string()),
    }
}
