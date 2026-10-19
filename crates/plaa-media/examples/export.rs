//! Exports a file, as the Export dialog does, and prints how it went.
//!
//! cargo run --release -p plaa-media --example export -- [--format
//! h264|hevc|av1|aac|mp3|opus|flac|alac|wav] [--quality <kb/s or bits>]
//! [--lines <lines>] [--software] [--clips <from>-<to>,...] [--turns
//! <quarter turns>] [--mirror] [--streams <picture>,<sound>] <file> <to>
//!
//! The streams are the picture's and the sound's numbers in the file, 0
//! and 1 unless given, `-` for none.

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc;
use std::time::Instant;

use plaa_media::export::{Export, ExportEvent, Format, Job, Sound};
use plaa_media::{Clip, Edit};

fn main() {
    let mut arguments = std::env::args().skip(1).peekable();
    let mut format = Format::H264;
    let mut lines = None;
    let mut quality = None;
    let mut hardware = true;
    let mut clips: Option<Vec<Clip>> = None;
    let mut turns = 0;
    let mut mirror = false;
    let mut streams = (Some(0), Some(1));
    while let Some(option) = arguments.next_if(|argument| argument.starts_with("--")) {
        match option.as_str() {
            "--format" => {
                format = match arguments.next().as_deref() {
                    Some("hevc") => Format::Hevc,
                    Some("av1") => Format::Av1,
                    Some("aac") => Format::Sound(Sound::Aac),
                    Some("mp3") => Format::Sound(Sound::Mp3),
                    Some("opus") => Format::Sound(Sound::Opus),
                    Some("flac") => Format::Sound(Sound::Flac),
                    Some("alac") => Format::Sound(Sound::Alac),
                    Some("wav") => Format::Sound(Sound::Wav),
                    _ => Format::H264,
                }
            }
            "--lines" => lines = arguments.next().and_then(|text| text.parse().ok()),
            "--quality" => quality = arguments.next().and_then(|text| text.parse().ok()),
            "--software" => hardware = false,
            "--clips" => {
                clips = arguments.next().map(|text| {
                    text.split(',')
                        .filter_map(|clip| {
                            let (from, to) = clip.split_once('-')?;
                            Some(Clip {
                                from: from.parse().ok()?,
                                to: to.parse().ok()?,
                            })
                        })
                        .collect()
                })
            }
            "--turns" => {
                turns = arguments
                    .next()
                    .and_then(|text| text.parse().ok())
                    .unwrap_or(0)
            }
            "--mirror" => mirror = true,
            "--streams" => {
                if let Some((picture, sound)) = arguments
                    .next()
                    .as_deref()
                    .and_then(|text| text.split_once(','))
                {
                    streams = (picture.parse().ok(), sound.parse().ok());
                }
            }
            other => {
                eprintln!("unknown option {other}");
                std::process::exit(2);
            }
        }
    }
    let (Some(source), Some(destination)) = (arguments.next(), arguments.next()) else {
        eprintln!(
            "usage: export [--format h264|hevc|av1|aac|mp3|opus|flac|alac|wav] \
             [--quality <kb/s or bits>] [--lines <lines>] [--software] \
             [--clips <from>-<to>,...] [--turns <n>] [--mirror] \
             [--streams <picture>,<sound>] <file> <to>"
        );
        std::process::exit(2);
    };
    let edit = if clips.is_some() || turns > 0 || mirror {
        let mut edit = Edit::new(clips.unwrap_or_else(|| {
            vec![Clip {
                from: 0.0,
                to: f64::INFINITY,
            }]
        }));
        if mirror {
            edit.flip(true);
        }
        for _ in 0..turns {
            edit.rotate(true);
        }
        Some(Arc::new(edit))
    } else {
        None
    };
    let job = Job {
        source: PathBuf::from(source),
        edit,
        video: streams.0,
        audio: streams.1,
        format,
        lines,
        quality,
        hardware,
        destination: PathBuf::from(destination),
    };
    let started = Instant::now();
    let (sender, receiver) = mpsc::channel();
    let _export = Export::start(job, move |event| {
        let _ = sender.send(event);
    });
    let mut shown = -1;
    for event in receiver {
        match event {
            ExportEvent::Started(encoder) => println!("encoding with {encoder:?}"),
            ExportEvent::Progress(share) => {
                let percent = (share * 100.0) as i32;
                if percent / 10 != shown / 10 {
                    shown = percent;
                    println!("{percent}%");
                }
            }
            ExportEvent::Finished(result) => {
                println!(
                    "finished after {:.1} s: {result:?}",
                    started.elapsed().as_secs_f64()
                );
                break;
            }
        }
    }
}
