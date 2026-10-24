//! Exports a file, as the Export dialog does, and prints how it went.
//!
//! cargo run --release -p plaa-media --example export -- [--format
//! h264|hevc|av1|aac|mp3|opus|flac|alac|wav] [--quality <kb/s or bits>]
//! [--lines <lines>] [--software] [--clips <from>-<to>,...] [--turns
//! <quarter turns>] [--mirror] [--streams <picture>,<sound>] [--insert
//! <file>@<at>] [--sound <file>@<from>-<to>] [--picture <file>@<from>-<to>]
//! [--picture-filter <filters>[@<from>-<to>]] [--sound-filter
//! <filters>[@<from>-<to>]] <file> <to>
//!
//! The streams are the picture's and the sound's numbers in the file, 0
//! and 1 unless given, `-` for none. --insert puts another file in at a
//! time of the edit; --sound and --picture play another file's, from its
//! start, between two times of the edit, or silence or black for `-`.

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc;
use std::time::Instant;

use plaa_media::edit::Filters;
use plaa_media::export::{Export, ExportEvent, Format, Job, Sound};
use plaa_media::{Clip, Edit, Feed, MediaInfo, Other, Track};

/// A change to make to the edit, as an option asks.
enum Change {
    Insert(PathBuf, f64),
    Replace(Track, Option<PathBuf>, f64, f64),
    Filter(Track, String, Option<(f64, f64)>),
}

/// `<filters>[@<from>-<to>]`.
fn filtering(track: Track, text: &str) -> Change {
    let range = text.rsplit_once('@').and_then(|(filters, range)| {
        let (from, to) = range.split_once('-')?;
        Some((filters.to_owned(), (from.parse().ok()?, to.parse().ok()?)))
    });
    match range {
        Some((filters, range)) => Change::Filter(track, filters, Some(range)),
        None => Change::Filter(track, text.to_owned(), None),
    }
}

/// `<file>@<from>-<to>`, `-` for no file.
fn replacement(track: Track, text: &str) -> Option<Change> {
    let (file, range) = text.rsplit_once('@')?;
    let (from, to) = range.split_once('-')?;
    let file = (file != "-").then(|| PathBuf::from(file));
    Some(Change::Replace(
        track,
        file,
        from.parse().ok()?,
        to.parse().ok()?,
    ))
}

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
    let mut changes = Vec::new();
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
                            Some(Clip::new(from.parse().ok()?, to.parse().ok()?))
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
            "--insert" => {
                if let Some((file, at)) = arguments.next().as_deref().and_then(|text| {
                    let (file, at) = text.rsplit_once('@')?;
                    Some((PathBuf::from(file), at.parse().ok()?))
                }) {
                    changes.push(Change::Insert(file, at));
                }
            }
            "--picture-filter" | "--sound-filter" => {
                let track = if option == "--sound-filter" {
                    Track::Sound
                } else {
                    Track::Picture
                };
                changes.extend(arguments.next().map(|text| filtering(track, &text)));
            }
            "--sound" | "--picture" => {
                let track = if option == "--sound" {
                    Track::Sound
                } else {
                    Track::Picture
                };
                changes.extend(arguments.next().and_then(|text| replacement(track, &text)));
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
    let edit = if clips.is_some() || turns > 0 || mirror || !changes.is_empty() {
        let duration = MediaInfo::probe(source.as_ref())
            .ok()
            .and_then(|info| info.duration)
            .map_or(f64::INFINITY, |duration| duration.as_secs_f64());
        let mut edit = Edit::new(clips.unwrap_or_else(|| vec![Clip::new(0.0, duration)]));
        for change in changes {
            let other = |edit: &mut Edit, path: PathBuf| match MediaInfo::probe(&path) {
                Ok(info) => Some(edit.add_other(Other::new(path, &info))),
                Err(error) => {
                    eprintln!("{}: {error}", path.display());
                    std::process::exit(2);
                }
            };
            match change {
                Change::Insert(path, at) => {
                    let source = other(&mut edit, path).unwrap_or_default();
                    let length = edit.other(source).map_or(0.0, |other| other.duration);
                    let clip = Clip {
                        source,
                        ..Clip::new(0.0, length)
                    };
                    println!("insert at {at}: {}", edit.insert(at, clip));
                }
                Change::Filter(track, filters, range) => {
                    let (from, to) = range.unwrap_or((0.0, edit.duration()));
                    let added = match track {
                        Track::Picture => Filters {
                            picture: Some(filters),
                            sound: None,
                        },
                        Track::Sound => Filters {
                            picture: None,
                            sound: Some(filters),
                        },
                    };
                    println!(
                        "filter {track:?} {from}-{to}: {}",
                        edit.add_filters(from, to, &added)
                    );
                }
                Change::Replace(track, path, from, to) => {
                    let feed = match path {
                        Some(path) => Feed::File {
                            source: other(&mut edit, path).unwrap_or_default(),
                            from: 0.0,
                        },
                        None => Feed::Blank,
                    };
                    println!(
                        "replace {track:?} {from}-{to}: {}",
                        edit.replace_track(track, from, to, Some(feed))
                    );
                }
            }
        }
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
