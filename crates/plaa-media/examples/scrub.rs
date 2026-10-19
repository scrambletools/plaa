//! Drags along the timeline without a window, as a pointer would, and
//! reports how the pictures shown followed: how often they went the wrong
//! way, and how far behind the pointer they were.
//!
//! cargo run --release -p plaa-media --example scrub -- <file>

use std::time::{Duration, Instant};

use plaa_media::{Options, Player, Source};

/// How often the pointer moves and the window draws.
const FRAME: Duration = Duration::from_millis(8);

fn main() {
    let Some(source) = std::env::args().nth(1) else {
        eprintln!("usage: scrub <file>");
        std::process::exit(2);
    };
    let player = Player::open(
        Source::parse(&source),
        Options {
            muted: true,
            ..Options::default()
        },
        |_| {},
    );
    std::thread::sleep(Duration::from_millis(500));
    // Forward inside one long run of pictures between keyframes, forward
    // across several, then back.
    for (from, to, seconds) in [(1.0, 7.5, 3.0), (9.0, 21.0, 3.0), (20.0, 12.0, 2.0)] {
        drag(&player, from, to, seconds);
    }
    // Playing on after the last drag, drawing as a window does.
    let before = player.position().as_secs_f64();
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(500) {
        player.picture();
        std::thread::sleep(FRAME);
    }
    println!(
        "then plays on: {:.2} s in 0.5 s, paused {}",
        player.position().as_secs_f64() - before,
        player.paused()
    );
}

fn drag(player: &Player, from: f64, to: f64, seconds: f64) {
    let start = Instant::now();
    let mut shown: Vec<f64> = Vec::new();
    let mut behind = Vec::new();
    let forward = to > from;
    loop {
        let along = (start.elapsed().as_secs_f64() / seconds).min(1.0);
        let pointer = from + (to - from) * along;
        player.scrub(Duration::from_secs_f64(pointer));
        if let Some(frame) = player.picture()
            && shown.last() != Some(&frame.position)
        {
            shown.push(frame.position);
        }
        if let Some(&last) = shown.last() {
            behind.push((pointer - last).abs());
        }
        if along >= 1.0 {
            break;
        }
        std::thread::sleep(FRAME);
    }
    player.end_scrub();
    let settle = Instant::now();
    let mut settled = None;
    while settle.elapsed() < Duration::from_secs(2) {
        if let Some(frame) = player.picture()
            && (frame.position - to).abs() < 0.05
        {
            settled = Some(settle.elapsed());
            break;
        }
        std::thread::sleep(FRAME);
    }
    let wrong_way = shown
        .windows(2)
        .filter(|pair| (pair[1] < pair[0]) == forward && pair[1] != pair[0])
        .count();
    behind.sort_by(f64::total_cmp);
    let median = behind.get(behind.len() / 2).copied().unwrap_or(f64::NAN);
    println!(
        "{from:>5.1} → {to:>5.1} s: {} pictures, {wrong_way} the wrong way, \
         behind the pointer {median:.2} s (median), landed after {:?}",
        shown.len(),
        settled,
    );
}
