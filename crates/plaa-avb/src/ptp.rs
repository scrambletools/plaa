//! gptpd as plaa runs it where nothing else does: its service,
//! gptpd@<interface>, which gptpd's package installs with a polkit rule
//! letting the person at the computer start it. triib's entity service
//! starts the same one, so the two share gPTP on the interface, and its
//! profile, gPTP's or AVB Lite's, is gptpd's to choose. Beside it
//! plaa-link@<interface>, from plaa's own package, keeps the frames plaa
//! sends on one transmit queue, that queue moving, and EEE and PAUSE off
//! the link.

use std::path::Path;
use std::process::{Command, Stdio};

/// gptpd's service.
const GPTPD: &str = "gptpd";
/// plaa's own, keeping its frames on one transmit queue, it moving, and
/// EEE and PAUSE off the link.
const LINK: &str = "plaa-link";

/// The `service`'s unit on `interface`, where the name needs no escaping.
fn unit(service: &str, interface: &str) -> Option<String> {
    let nameable = !interface.is_empty()
        && interface
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'));
    nameable.then(|| format!("{service}@{interface}.service"))
}

/// Whether a package, or a dev install, put the `service` in.
fn has(service: &str) -> bool {
    [
        "/etc/systemd/system",
        "/usr/local/lib/systemd/system",
        "/usr/lib/systemd/system",
        "/lib/systemd/system",
    ]
    .iter()
    .any(|folder| {
        Path::new(folder)
            .join(format!("{service}@.service"))
            .exists()
    })
}

/// Whether gptpd's package, or a dev install, put its service in.
pub(crate) fn installed() -> bool {
    has(GPTPD)
}

/// gptpd's configuration for `interface`, which sets its priority1.
pub(crate) fn config(interface: &str) -> String {
    format!("/etc/gptpd/{interface}.conf")
}

/// Why gptpd refused to run on `interface`, as another PTP daemon holding
/// the port, where it said.
pub(crate) fn refused(interface: &str) -> Option<String> {
    let path = Path::new("/run/gptpd").join(format!("{interface}.refused"));
    let reason = std::fs::read_to_string(path).ok()?;
    let reason = reason.trim();
    (!reason.is_empty()).then(|| reason.to_owned())
}

/// Starts gptpd's service on `interface`; why it could not, where it
/// could not.
pub(crate) fn start(interface: &str) -> Result<(), String> {
    systemctl("start", GPTPD, interface)
}

/// Keeps the frames plaa sends on `interface` on its first transmit
/// queue, that queue moving, and EEE and PAUSE off the link, where plaa's
/// package installed plaa-link@: run again each time, as a driver
/// starting afresh forgets them. The link goes down and up once where
/// EEE or PAUSE was on.
pub(crate) fn steer(interface: &str) {
    if has(LINK)
        && let Err(why) = systemctl("restart", LINK, interface)
    {
        eprintln!("plaa: {interface}'s frames may leave out of order: {why}");
    }
}

fn systemctl(verb: &str, service: &str, interface: &str) -> Result<(), String> {
    let unit =
        unit(service, interface).ok_or_else(|| format!("{interface} cannot name a service"))?;
    let output = Command::new("systemctl")
        .args([verb, "--no-block", "--no-ask-password", &unit])
        .stdin(Stdio::null())
        .output()
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units_are_named_for_plain_interfaces_only() {
        assert_eq!(
            unit(GPTPD, "enp2s0").as_deref(),
            Some("gptpd@enp2s0.service")
        );
        assert_eq!(
            unit(LINK, "eth0.2").as_deref(),
            Some("plaa-link@eth0.2.service")
        );
        assert_eq!(unit(GPTPD, ""), None);
        assert_eq!(unit(LINK, "we ird"), None);
    }
}
