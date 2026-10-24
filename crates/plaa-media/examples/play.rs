//! Plays a file without a window, drawing nothing but asking for pictures
//! as the interface does, and prints what happened.
//!
//! cargo run --release -p plaa-media --example play -- [--software]
//! [--at-most <lines>] [--clips <from>-<to>,...] [--insert <file>@<at>]
//! [--sound <file>@<from>-<to>] [--picture <file>@<from>-<to>]
//! [--picture-filter <filters>] [--sound-filter <filters>] <file>
//! [seconds] [seek-to]
//!
//! --insert, --sound and --picture change the edit as the export
//! example's do. With PLAY_TRACE set, it prints its status four times a
//! second, with the pitch heard, and each picture shown with the
//! brightness of its first pixel, which a test video can count frames
//! in.

use std::sync::Arc;
use std::time::{Duration, Instant};

use std::path::PathBuf;

use plaa_media::{
    Clip, Edit, Event, Feed, MediaInfo, Options, Other, Player, Quality, Source, Track,
};

/// `<file>@<from>-<to>`, `-` for no file.
fn replacement(text: &str) -> Option<(Option<PathBuf>, f64, f64)> {
    let (file, range) = text.rsplit_once('@')?;
    let (from, to) = range.split_once('-')?;
    Some((
        (file != "-").then(|| PathBuf::from(file)),
        from.parse().ok()?,
        to.parse().ok()?,
    ))
}

/// The pitch of `samples` at `rate`, by how often they cross zero.
fn pitch(samples: &[f32], rate: u32) -> f64 {
    let crossings = samples
        .windows(2)
        .filter(|pair| (pair[0] < 0.0) != (pair[1] < 0.0))
        .count();
    crossings as f64 / 2.0 / (samples.len() as f64 / f64::from(rate))
}

fn main() {
    let mut arguments = std::env::args().skip(1).peekable();
    let software = arguments.next_if_eq("--software").is_some();
    let quality = match arguments.next_if_eq("--at-most") {
        Some(_) => match arguments.next().and_then(|lines| lines.parse().ok()) {
            Some(lines) => Quality::AtMost(lines),
            None => Quality::Best,
        },
        None => Quality::Best,
    };
    let mut clips_given = arguments.next_if_eq("--clips");
    let mut changes = Vec::new();
    while let Some(option) = arguments.next_if(|argument| {
        matches!(
            argument.as_str(),
            "--insert" | "--sound" | "--picture" | "--picture-filter" | "--sound-filter" | "--mix"
        )
    }) {
        if let Some(value) = arguments.next() {
            changes.push((option, value));
        }
    }
    let edit = clips_given.take().and_then(|_| {
        let clips: Vec<Clip> = arguments
            .next()?
            .split(',')
            .filter_map(|clip| {
                let (from, to) = clip.split_once('-')?;
                Some(Clip::new(from.parse().ok()?, to.parse().ok()?))
            })
            .collect();
        Some(Arc::new(Edit::new(clips)))
    });
    let edit = if changes.is_empty() {
        edit
    } else {
        let source = arguments.peek().cloned().unwrap_or_default();
        let duration = MediaInfo::probe(source.as_ref())
            .ok()
            .and_then(|info| info.duration)
            .map_or(0.0, |duration| duration.as_secs_f64());
        let mut edit = edit.map_or_else(|| Edit::whole(duration), |edit| (*edit).clone());
        for (option, value) in changes {
            let other = |edit: &mut Edit, path: PathBuf| {
                let info = MediaInfo::probe(&path).expect("a file plaa reads");
                edit.add_other(Other::new(path, &info))
            };
            if option == "--mix" {
                let Some((file, at)) = value.rsplit_once('@') else {
                    continue;
                };
                let source = other(&mut edit, PathBuf::from(file));
                let length = edit.other(source).map_or(0.0, |other| other.duration);
                edit.add_layer(plaa_media::edit::Layer {
                    source,
                    at: at.parse().unwrap_or(0.0),
                    from: 0.0,
                    to: length,
                    volume: -6.0,
                    fade_in: 0.5,
                    fade_out: 0.5,
                });
            } else if option == "--picture-filter" || option == "--sound-filter" {
                let duration = edit.duration();
                let filters = Some(value.clone());
                let added = if option == "--picture-filter" {
                    plaa_media::edit::Filters {
                        picture: filters,
                        sound: None,
                    }
                } else {
                    plaa_media::edit::Filters {
                        picture: None,
                        sound: filters,
                    }
                };
                edit.add_filters(0.0, duration, &added);
            } else if option == "--insert" {
                let Some((file, at)) = value.rsplit_once('@') else {
                    continue;
                };
                let source = other(&mut edit, PathBuf::from(file));
                let length = edit.other(source).map_or(0.0, |other| other.duration);
                let clip = Clip {
                    source,
                    ..Clip::new(0.0, length)
                };
                edit.insert(at.parse().unwrap_or(0.0), clip);
            } else if let Some((file, from, to)) = replacement(&value) {
                let track = if option == "--sound" {
                    Track::Sound
                } else {
                    Track::Picture
                };
                let feed = match file {
                    Some(file) => Feed::File {
                        source: other(&mut edit, file),
                        from: 0.0,
                    },
                    None => Feed::Blank,
                };
                edit.replace_track(track, from, to, Some(feed));
            }
        }
        Some(Arc::new(edit))
    };
    let Some(source) = arguments.next() else {
        eprintln!(
            "usage: play [--software] [--at-most <lines>] [--clips <from>-<to>,...] \
             <file or address> [seconds] [seek-to]"
        );
        std::process::exit(2);
    };
    let seconds: f64 = arguments
        .next()
        .and_then(|text| text.parse().ok())
        .unwrap_or(5.0);
    let seek_to: Option<f64> = arguments.next().and_then(|text| text.parse().ok());
    let player = Player::open(
        Source::parse(&source),
        Options {
            hardware: !software,
            muted: true,
            quality,
            ..Options::default()
        },
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
                for variant in &info.variants {
                    println!(
                        "  variant: video {} {}x{}, {:?} bits a second, streams {:?}",
                        variant.video,
                        variant.width,
                        variant.height,
                        variant.bit_rate,
                        variant.streams
                    );
                }
            }
            Event::Page(page) => println!(
                "page: {:?} by {:?}, {} formats, heights {:?}, live {}",
                page.title,
                page.artist,
                page.formats.len(),
                page.heights(),
                page.live
            ),
            Event::List(title, entries) => {
                println!("list {title:?} of {}:", entries.len());
                for entry in entries {
                    println!("  {} {:?}", entry.url, entry.title);
                }
            }
            other => println!("event: {other:?}"),
        },
    );
    if let Some(edit) = &edit {
        println!("clips {:?}, {:.3} s", edit.clips(), edit.duration());
        player.set_edit(Some(edit.clone()), Duration::ZERO);
    }
    let trace = std::env::var_os("PLAY_TRACE").is_some();
    let start = Instant::now();
    let mut shown = 0;
    let mut last_id = 0;
    let mut first_position = None;
    let mut sought = false;
    let mut buffering = false;
    let mut traced = u64::MAX;
    while start.elapsed().as_secs_f64() < seconds {
        if let Some(frame) = player.picture()
            && frame.id != last_id
        {
            shown += 1;
            last_id = frame.id;
            if trace {
                let (plane, _) = frame.plane(0);
                eprintln!(
                    "picture {:.3} value {}",
                    frame.position,
                    plane.first().copied().unwrap_or(0)
                );
            }
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
        let step = (start.elapsed().as_secs_f64() * 4.0) as u64;
        if trace && step != traced {
            traced = step;
            let (heard, level) = player.heard(2048).map_or((0.0, -120.0), |(samples, rate)| {
                let power = samples.iter().map(|sample| sample * sample).sum::<f32>()
                    / samples.len().max(1) as f32;
                (
                    pitch(&samples, rate),
                    10.0 * f64::from(power.max(1e-12)).log10(),
                )
            });
            eprintln!(
                "{:.2} {:?} {:?} {:?} pitch {heard:.0} level {level:.1}",
                start.elapsed().as_secs_f64(),
                status.position,
                status.buffering,
                status.buffered
            );
        }
        if status.buffering != buffering {
            buffering = status.buffering;
            println!(
                "{:.2} s: {} at {:.2} s, read to {:?}",
                start.elapsed().as_secs_f64(),
                if buffering { "buffering" } else { "playing" },
                status.position.as_secs_f64(),
                status.buffered
            );
        }
        if status.ended {
            println!("ended at {:?}", status.position);
            break;
        }
        std::thread::sleep(Duration::from_millis(8));
    }
    let status = player.status();
    println!(
        "after {:.2} s: position {:.3} s, {shown} pictures, first {:?}, paused {}, ended {}, \
         video {:?}, audio {:?}",
        start.elapsed().as_secs_f64(),
        status.position.as_secs_f64(),
        first_position,
        status.paused,
        status.ended,
        status.video,
        status.audio
    );
}
