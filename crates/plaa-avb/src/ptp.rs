//! ptp4l as plaa runs it where nothing else does: its service,
//! plaa-ptp4l@<interface>, which plaa's packages install with a polkit
//! rule letting the person at the computer start it, as triib's do
//! theirs, and plaa-ptp4l-lite@<interface>, the AVB Lite PTP profile's,
//! which plaa moves it to as its entity falls back to AVB Lite. They give
//! way to triib's, which conflict with them.

use std::path::Path;
use std::process::{Command, Stdio};

/// The service's unit on `interface`, gPTP's or, `lite`, the AVB Lite
/// PTP profile's, where the name needs no escaping.
fn unit(interface: &str, lite: bool) -> Option<String> {
    let nameable = !interface.is_empty()
        && interface
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'));
    let profile = if lite { "-lite" } else { "" };
    nameable.then(|| format!("plaa-ptp4l{profile}@{interface}.service"))
}

/// Whether plaa's package installed the service.
pub(crate) fn installed() -> bool {
    [
        "/etc/systemd/system",
        "/usr/local/lib/systemd/system",
        "/usr/lib/systemd/system",
        "/lib/systemd/system",
    ]
    .iter()
    .any(|folder| Path::new(folder).join("plaa-ptp4l@.service").exists())
}

/// Whether a ptp4l process names `interface` on its command line, as one
/// run by hand or by another unit does.
pub(crate) fn running_on(interface: &str) -> bool {
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return false;
    };
    entries.flatten().any(|entry| {
        std::fs::read(entry.path().join("cmdline"))
            .is_ok_and(|line| runs_ptp4l_on(&line, interface))
    })
}

/// Whether a process's command line, its arguments each ended by a zero,
/// runs ptp4l on `interface`.
fn runs_ptp4l_on(line: &[u8], interface: &str) -> bool {
    let args: Vec<&[u8]> = line.split(|&byte| byte == 0).collect();
    let program = args
        .first()
        .and_then(|program| program.rsplit(|&byte| byte == b'/').next());
    program == Some(b"ptp4l".as_slice())
        && args
            .windows(2)
            .any(|pair| pair[0] == b"-i" && pair[1] == interface.as_bytes())
}

/// Starts the service on `interface`; why it could not, where it could
/// not.
pub(crate) fn start(interface: &str) -> Result<(), String> {
    start_unit(interface, false)
}

/// Moves plaa's ptp4l on `interface` to the AVB Lite PTP profile, `lite`,
/// or back to gPTP, where plaa's other service runs it: the two conflict,
/// so starting one stops the other. A ptp4l run by hand or by another
/// unit plaa leaves as it is.
pub(crate) fn switch(interface: &str, lite: bool) -> Result<(), String> {
    let Some(from) = unit(interface, !lite) else {
        return Ok(());
    };
    let active = Command::new("systemctl")
        .args(["is-active", "--quiet", &from])
        .stdin(Stdio::null())
        .status()
        .is_ok_and(|status| status.success());
    if active {
        start_unit(interface, lite)
    } else {
        Ok(())
    }
}

fn start_unit(interface: &str, lite: bool) -> Result<(), String> {
    let unit = unit(interface, lite).ok_or_else(|| format!("{interface} cannot name a service"))?;
    let output = Command::new("systemctl")
        .args(["start", "--no-block", "--no-ask-password", &unit])
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
            unit("enp2s0", false).as_deref(),
            Some("plaa-ptp4l@enp2s0.service")
        );
        assert_eq!(
            unit("eth0.2", true).as_deref(),
            Some("plaa-ptp4l-lite@eth0.2.service")
        );
        assert_eq!(unit("", false), None);
        assert_eq!(unit("we ird", true), None);
    }

    #[test]
    fn a_ptp4l_is_found_on_its_interface() {
        let line = b"/usr/sbin/ptp4l\0-f\0/etc/triib/ptp4l-gptp.cfg\0-i\0enp2s0\0";
        assert!(runs_ptp4l_on(line, "enp2s0"));
        assert!(!runs_ptp4l_on(line, "eth1"));
        assert!(!runs_ptp4l_on(b"/usr/bin/pmc\0-i\0enp2s0\0", "enp2s0"));
    }
}
