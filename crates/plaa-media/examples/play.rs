//! Plays a file without a window, drawing nothing but asking for pictures
//! as the interface does, and prints what happened.
//!
//! cargo run --release -p plaa-media --example play -- <file> [seconds] [seek-to]

use std::time::{Duration, Instant};

use plaa_media::{Event, Options, Player, Source};

fn main() {
    let mut arguments = std::env::args().skip(1);
    let Some(source) = arguments.next() else {
        eprintln!("usage: play <file or address> [seconds] [seek-to]");
        std::process::exit(2);
    };
    let seconds: f64 = arguments
        .next()
        .and_then(|text| text.parse().ok())
        .unwrap_or(5.0);
    let seek_to: Option<f64> = arguments.next().and_then(|text| text.parse().ok());
    let player = Player::open(
        Source::parse(&source),
        Options::default(),
        |event| match event {
            Event::Opened { info, cover } => {
                println!(
                    "opened: {} {:?}, {} video, {} audio, {} subtitles, cover {:?}",
                    info.format,
                    info.duration,
                    info.video.len(),
                    info.audio.len(),
                    info.subtitles.len(),
                    cover
                );
                for track in &info.video {
                    println!(
                        "  video {}: {} {}x{} {:?} fps, aspect {:.3}, {} turns, {}",
                        track.track.index,
                        track.track.codec,
                        track.width,
                        track.height,
                        track.frame_rate,
                        track.aspect,
                        track.quarter_turns,
                        track.pixel_format
                    );
                }
                for track in &info.audio {
                    println!(
                        "  audio {}: {} {} Hz {}",
                        track.track.index, track.track.codec, track.sample_rate, track.layout
                    );
                }
            }
            other => println!("event: {other:?}"),
        },
    );
    let start = Instant::now();
    let mut shown = 0;
    let mut last_id = 0;
    let mut first_position = None;
    let mut sought = false;
    while start.elapsed().as_secs_f64() < seconds {
        if let Some(frame) = player.picture()
            && frame.id != last_id
        {
            shown += 1;
            last_id = frame.id;
            first_position.get_or_insert((frame.position, frame.layout, frame.colors));
        }
        if let Some(target) = seek_to
            && !sought
            && start.elapsed() > Duration::from_secs(1)
        {
            sought = true;
            player.seek(Duration::from_secs_f64(target), true);
            println!("seek to {target}");
        }
        let status = player.status();
        if status.ended {
            println!("ended at {:?}", status.position);
            break;
        }
        std::thread::sleep(Duration::from_millis(8));
    }
    let status = player.status();
    println!(
        "after {:.2} s: position {:.3} s, {shown} pictures, first {:?}, paused {}, ended {}",
        start.elapsed().as_secs_f64(),
        status.position.as_secs_f64(),
        first_position,
        status.paused,
        status.ended
    );
}
