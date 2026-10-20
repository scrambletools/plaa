//! Rebuilds plaa when its translations change, and records the commit
//! and time it is built from for its version line.

/// The commit plaa is built from, with `+` when the working tree has
/// changes: from git, or for copies of the source without `.git` from the
/// `BUILD` file beside the workspace, where `git archive` and GitHub's
/// source archives write the commit (`.gitattributes`).
fn commit() -> String {
    let git = |args: &[&str]| {
        std::process::Command::new("git")
            .args(args)
            .current_dir("../..")
            .output()
            .ok()
            .filter(|output| output.status.success())
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
    };
    match git(&["rev-parse", "--short=7", "HEAD"]) {
        Some(hash) => {
            let dirty = git(&["status", "--porcelain", "--untracked-files=no"])
                .is_some_and(|status| !status.is_empty());
            format!("{hash}{}", if dirty { "+" } else { "" })
        }
        None => std::fs::read_to_string("../../BUILD")
            .map(|text| text.trim().to_owned())
            .ok()
            .filter(|text| !text.is_empty() && !text.starts_with("$Format"))
            .unwrap_or_else(|| "unknown".to_owned()),
    }
}

/// The time of the build, in UTC, as "YYYY-MM-DD HH:MM".
fn built_at() -> String {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |since| since.as_secs() as i64);
    let (days, rest) = (seconds.div_euclid(86_400), seconds.rem_euclid(86_400));
    // Days since 1970 to a civil date (Howard Hinnant's algorithm).
    let shifted = days + 719_468;
    let era = shifted.div_euclid(146_097);
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02} {:02}:{:02}",
        rest / 3600,
        rest % 3600 / 60
    )
}

fn main() {
    println!("cargo:rustc-env=PLAA_COMMIT={}", commit());
    println!("cargo:rustc-env=PLAA_BUILT={}", built_at());
    for path in [
        "../../.git/HEAD",
        "../../.git/index",
        "../../BUILD",
        "../../crates",
    ] {
        println!("cargo:rerun-if-changed={path}");
    }
    // The translations are embedded when plaa compiles; a new or changed
    // language file must rebuild it.
    println!("cargo:rerun-if-changed=../../i18n");
}
