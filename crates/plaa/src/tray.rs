//! A red dot in the system tray while plaa records the screen hidden,
//! through the StatusNotifierItem protocol the desktop's panel shows
//! items by: clicking it stops the recording.

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use zbus::blocking::Connection;
use zbus::interface;

const PATH: &str = "/StatusNotifierItem";
const WATCHER: &str = "org.kde.StatusNotifierWatcher";

/// Pictures of an icon, each its width, height and ARGB pixels in network
/// byte order.
type Pixmap = Vec<(i32, i32, Vec<u8>)>;

/// The dot. Dropping it takes it away.
pub struct Tray {
    _connection: Connection,
}

/// Whether the desktop shows a tray to put the dot in.
pub fn available() -> bool {
    let has_watcher = || {
        let connection = Connection::session().ok()?;
        let bus = zbus::blocking::fdo::DBusProxy::new(&connection).ok()?;
        bus.name_has_owner(WATCHER.try_into().ok()?).ok()
    };
    has_watcher().unwrap_or(false)
}

impl Tray {
    /// Shows the dot, named `title` and saying `tip` where it is pointed
    /// at; clicking it calls `stop`. None without a tray.
    pub fn show(
        title: String,
        tip: String,
        stop: impl Fn() + Send + Sync + 'static,
    ) -> Option<Self> {
        static SHOWN: AtomicU32 = AtomicU32::new(0);
        let name = format!(
            "org.kde.StatusNotifierItem-{}-{}",
            std::process::id(),
            SHOWN.fetch_add(1, Ordering::Relaxed) + 1
        );
        let item = Item {
            title,
            tip,
            stop: Arc::new(stop),
        };
        let connection = zbus::blocking::connection::Builder::session()
            .ok()?
            .name(name.clone())
            .ok()?
            .serve_at(PATH, item)
            .ok()?
            .build()
            .ok()?;
        connection
            .call_method(
                Some(WATCHER),
                "/StatusNotifierWatcher",
                Some(WATCHER),
                "RegisterStatusNotifierItem",
                &(name.as_str(),),
            )
            .ok()?;
        Some(Self {
            _connection: connection,
        })
    }
}

struct Item {
    title: String,
    tip: String,
    stop: Arc<dyn Fn() + Send + Sync>,
}

#[interface(name = "org.kde.StatusNotifierItem")]
impl Item {
    // Any click stops: stopping is all the dot does.
    fn activate(&self, _x: i32, _y: i32) {
        (self.stop)();
    }

    fn secondary_activate(&self, _x: i32, _y: i32) {
        (self.stop)();
    }

    fn context_menu(&self, _x: i32, _y: i32) {
        (self.stop)();
    }

    fn scroll(&self, _delta: i32, _orientation: &str) {}

    #[zbus(property)]
    fn category(&self) -> &str {
        "ApplicationStatus"
    }

    #[zbus(property)]
    fn id(&self) -> &str {
        "plaa"
    }

    #[zbus(property)]
    fn title(&self) -> &str {
        &self.title
    }

    #[zbus(property)]
    fn status(&self) -> &str {
        "Active"
    }

    #[zbus(property)]
    fn window_id(&self) -> i32 {
        0
    }

    /// None: the dot is drawn here, as icon themes differ.
    #[zbus(property)]
    fn icon_name(&self) -> &str {
        ""
    }

    #[zbus(property)]
    fn icon_pixmap(&self) -> Pixmap {
        dot()
    }

    #[zbus(property)]
    fn tool_tip(&self) -> (String, Pixmap, String, String) {
        (
            String::new(),
            Vec::new(),
            self.title.clone(),
            self.tip.clone(),
        )
    }

    #[zbus(property)]
    fn item_is_menu(&self) -> bool {
        false
    }
}

/// A red dot, as a recording light, at the sizes panels use.
fn dot() -> Pixmap {
    const RED: [u8; 3] = [0xe5, 0x39, 0x35];
    [16, 22, 24, 32, 48, 64]
        .into_iter()
        .map(|size: i32| {
            let middle = size as f32 / 2.0;
            let radius = size as f32 * 0.4;
            let mut pixels = Vec::with_capacity((size * size * 4) as usize);
            for row in 0..size {
                for column in 0..size {
                    let across = column as f32 + 0.5 - middle;
                    let down = row as f32 + 0.5 - middle;
                    let distance = (across * across + down * down).sqrt();
                    // A pixel's worth of soft edge.
                    let alpha = (radius + 0.5 - distance).clamp(0.0, 1.0);
                    pixels.push((alpha * 255.0).round() as u8);
                    pixels.extend(RED);
                }
            }
            (size, size, pixels)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::dot;

    #[test]
    fn the_dot_is_round_and_red() {
        for (width, height, pixels) in dot() {
            assert_eq!(pixels.len(), (width * height * 4) as usize);
            let at = |column: i32, row: i32| {
                let start = ((row * width + column) * 4) as usize;
                &pixels[start..start + 4]
            };
            // Opaque red in the middle, clear in the corners.
            assert_eq!(at(width / 2, height / 2), [255, 0xe5, 0x39, 0x35]);
            assert_eq!(at(0, 0)[0], 0);
            assert_eq!(at(width - 1, height - 1)[0], 0);
        }
    }
}
