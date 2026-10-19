//! Plays a file with a subtitle track on, without a window, and saves the
//! picture subtitle showing at a time as a PNG, or prints the text, and
//! the video's picture then.
//!
//! cargo run --release -p plaa-media --example subtitles -- <file> <track> <seconds> [subtitle.png] [frame.png]

use std::time::{Duration, Instant};

use plaa_media::{Options, Player, Source};

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [file, track, seconds, rest @ ..] = arguments.as_slice() else {
        eprintln!("usage: subtitles <file> <track> <seconds> [out.png]");
        std::process::exit(2);
    };
    let track: usize = track.parse().expect("a track number");
    let seconds: f64 = seconds.parse().expect("seconds");
    let player = Player::open(
        Source::parse(file),
        Options {
            muted: true,
            start: (seconds > 1.5).then(|| Duration::from_secs_f64(seconds - 1.0)),
            ..Options::default()
        },
        |_| {},
    );
    std::thread::sleep(Duration::from_millis(300));
    player.select_subtitles(Some(track));
    let begun = Instant::now();
    while player.position().as_secs_f64() < seconds && begun.elapsed() < Duration::from_secs(10) {
        player.picture();
        std::thread::sleep(Duration::from_millis(10));
    }
    println!("at {:.2} s", player.position().as_secs_f64());
    if let Some(cue) = player.subtitle() {
        println!("text: {}", cue.text);
    }
    match player.subtitle_picture() {
        Some(picture) => {
            println!(
                "picture {}x{} at {},{} on a {}x{} frame",
                picture.width,
                picture.height,
                picture.x,
                picture.y,
                picture.canvas.0,
                picture.canvas.1
            );
            if let Some(out) = rest.first() {
                let file = std::fs::File::create(out).expect("a file to write");
                let mut encoder = png::Encoder::new(file, picture.width, picture.height);
                encoder.set_color(png::ColorType::Rgba);
                encoder.set_depth(png::BitDepth::Eight);
                encoder
                    .write_header()
                    .and_then(|mut writer| writer.write_image_data(&picture.rgba))
                    .expect("the picture written");
                println!("saved {out}");
            }
        }
        None => println!("no picture subtitle"),
    }
    // The video's own picture then, beside it.
    if let (Some(out), Some((width, height, rgba))) = (rest.get(1), player.snapshot()) {
        let file = std::fs::File::create(out).expect("a file to write");
        let mut encoder = png::Encoder::new(file, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder
            .write_header()
            .and_then(|mut writer| writer.write_image_data(&rgba))
            .expect("the frame written");
        println!("saved {out}");
    }
}
