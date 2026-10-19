//! Times, sizes and rates as the interface shows them.

use plaa_media::info::Track;

/// `seconds` as "1:02", or "1:01:02" when `long`.
pub fn time(seconds: f64, long: bool) -> String {
    let total = if seconds.is_finite() {
        seconds.max(0.0) as u64
    } else {
        0
    };
    let (hours, minutes, seconds) = (total / 3600, total / 60 % 60, total % 60);
    if long || hours > 0 {
        format!("{hours}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes}:{seconds:02}")
    }
}

/// A playback speed, such as "1.25×".
pub fn speed(speed: f64) -> String {
    let text = format!("{speed:.2}");
    let text = text.trim_end_matches('0').trim_end_matches('.');
    format!("{text}×")
}

/// A file size in bytes, in the largest unit under 1,000 of it.
pub fn size(bytes: u64) -> String {
    let units = ["B", "kB", "MB", "GB", "TB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1000.0 && unit + 1 < units.len() {
        value /= 1000.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes} B")
    } else {
        format!("{value:.1} {}", units[unit])
    }
}

/// Bits per second, as "5.2 Mb/s".
pub fn bit_rate(bits: u64) -> String {
    let value = bits as f64;
    if value >= 1_000_000.0 {
        format!("{:.1} Mb/s", value / 1_000_000.0)
    } else {
        format!("{:.0} kb/s", value / 1000.0)
    }
}

/// A track's name in a menu: its title, else its language, else its
/// number, with `detail`, such as the channel layout, after it.
pub fn track_name(track: &Track, number: usize, detail: Option<&str>) -> String {
    let base = match (&track.title, &track.language) {
        (Some(title), Some(language)) => format!("{title} ({language})"),
        (Some(title), None) => title.clone(),
        (None, Some(language)) => language.clone(),
        (None, None) => crate::fl!("tracks-numbered", number = number),
    };
    match detail.filter(|detail| !detail.is_empty()) {
        Some(detail) => format!("{base} · {detail}"),
        None => base,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn times_read_as_clocks() {
        assert_eq!(time(62.4, false), "1:02");
        assert_eq!(time(62.4, true), "0:01:02");
        assert_eq!(time(3723.0, false), "1:02:03");
        assert_eq!(time(f64::NAN, false), "0:00");
    }

    #[test]
    fn speeds_and_sizes() {
        assert_eq!(speed(1.0), "1×");
        assert_eq!(speed(1.25), "1.25×");
        assert_eq!(speed(0.5), "0.5×");
        assert_eq!(size(999), "999 B");
        assert_eq!(size(12_345_678), "12.3 MB");
        assert_eq!(bit_rate(5_200_000), "5.2 Mb/s");
    }
}
