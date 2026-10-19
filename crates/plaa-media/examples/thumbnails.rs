//! Asks for thumbnails of a file as the timeline would, and reports how
//! long each took and what came back.
//!
//! cargo run --release -p plaa-media --example thumbnails -- <file> [width]

use std::sync::mpsc;
use std::time::{Duration, Instant};

use plaa_media::Thumbnails;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let Some(path) = arguments.next() else {
        eprintln!("usage: thumbnails <file> [width]");
        std::process::exit(2);
    };
    let width = arguments
        .next()
        .and_then(|text| text.parse().ok())
        .unwrap_or(320);
    let (sender, receiver) = mpsc::channel();
    let thumbnails = Thumbnails::open(path.into(), width, move |thumbnail| {
        let _ = sender.send(thumbnail);
    });
    // Each time twice: the second comes from those kept.
    for seconds in [1.0, 8.0, 15.0, 3.0, 8.5, 1.0] {
        let asked = Instant::now();
        thumbnails.request(seconds);
        match receiver.recv_timeout(Duration::from_secs(10)) {
            Ok(thumbnail) => println!(
                "{seconds:>5.1} s: picture at {:.2} s, {}x{}, in {:?}",
                thumbnail.seconds,
                thumbnail.picture.width,
                thumbnail.picture.height,
                asked.elapsed()
            ),
            Err(_) => println!("{seconds:>5.1} s: nothing"),
        }
    }
}
