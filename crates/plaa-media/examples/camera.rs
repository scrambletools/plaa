//! Lists the cameras, or records one as the camera recording window does,
//! with the sound of an input device when one is given. With `--input
//! lavfi`, the camera is FFmpeg's test picture of that name, as
//! `testsrc2=s=1280x720:r=30`.
//!
//! cargo run --release -p plaa-media --example camera -- --list
//! cargo run --release -p plaa-media --example camera -- [--input
//! v4l2|dshow|avfoundation|lavfi] [--format h264|hevc|av1] [--software]
//! [--rate <per second>] [--sound <device id>] <camera> <seconds> <to>

use std::path::PathBuf;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use plaa_media::capture::{Camera, Capture, VideoEncoding, cameras};
use plaa_media::export::Format;
use plaa_media::record::{Microphone, RecordEvent, input_devices};

fn main() {
    let mut arguments = std::env::args().skip(1).peekable();
    let mut input = None;
    let mut format = Format::H264;
    let mut hardware = true;
    let mut rate = None;
    let mut sound = None;
    while let Some(option) = arguments.next_if(|argument| argument.starts_with("--")) {
        match option.as_str() {
            "--list" => {
                for camera in cameras() {
                    println!("{}  {}  {}", camera.input, camera.id, camera.name);
                }
                return;
            }
            "--input" => input = arguments.next(),
            "--format" => {
                format = match arguments.next().as_deref() {
                    Some("hevc") => Format::Hevc,
                    Some("av1") => Format::Av1,
                    _ => Format::H264,
                }
            }
            "--software" => hardware = false,
            "--rate" => rate = arguments.next().and_then(|text| text.parse().ok()),
            "--sound" => sound = arguments.next(),
            other => {
                eprintln!("unknown option {other}");
                std::process::exit(2);
            }
        }
    }
    let (Some(id), Some(seconds), Some(destination)) = (
        arguments.next(),
        arguments.next().and_then(|text| text.parse::<f64>().ok()),
        arguments.next(),
    ) else {
        eprintln!(
            "usage: camera --list | camera [--input <device input>] [--format h264|hevc|av1] \
             [--software] [--rate <per second>] [--sound <device id>] <camera> <seconds> <to>"
        );
        std::process::exit(2);
    };
    // A camera listed, or one of the input named.
    let camera = match input {
        Some(input) => Camera {
            input,
            name: id.clone(),
            id,
        },
        None => match cameras().into_iter().find(|camera| camera.id == id) {
            Some(camera) => camera,
            None => {
                eprintln!("no camera {id}; --list shows them");
                std::process::exit(1);
            }
        },
    };
    // An id that is not listed would record the default input instead.
    if let Some(device) = &sound
        && !input_devices().iter().any(|listed| listed.id == *device)
    {
        eprintln!("no input {device}; the record example's --list shows them");
        std::process::exit(1);
    }
    let mut capture = match Capture::camera(&camera) {
        Ok(capture) => capture,
        Err(error) => {
            eprintln!("could not open {}: {error}", camera.name);
            std::process::exit(1);
        }
    };
    // As the window shows the camera a while before recording starts.
    std::thread::sleep(Duration::from_secs(1));
    if let Some(preview) = capture.preview() {
        println!("preview {}x{}", preview.width, preview.height);
    }
    let microphone = sound.map(|device| match Microphone::open(Some(&device)) {
        Ok(microphone) => microphone,
        Err(error) => {
            eprintln!("could not open the input: {error}");
            std::process::exit(1);
        }
    });
    let (sender, receiver) = mpsc::channel();
    if let Err(error) = capture.record(
        microphone.as_ref(),
        VideoEncoding {
            format,
            hardware,
            rate,
        },
        PathBuf::from(destination),
        Duration::ZERO,
        move |event| {
            let _ = sender.send(event);
        },
    ) {
        eprintln!("could not record: {error}");
        std::process::exit(1);
    }
    let started = Instant::now();
    while started.elapsed().as_secs_f64() < seconds {
        std::thread::sleep(Duration::from_millis(500));
        let (recorded, written) = capture.recorded();
        println!(
            "{:.1} s recorded, {written} bytes, {:?}, {} dropped",
            recorded.as_secs_f64(),
            capture.size(),
            capture.dropped()
        );
        if let Some(reason) = capture.ended_because() {
            println!("the camera stopped: {reason}");
            break;
        }
    }
    capture.stop();
    match receiver.recv_timeout(Duration::from_secs(10)) {
        Ok(RecordEvent::Finished(Ok(path))) => println!("saved {}", path.display()),
        Ok(RecordEvent::Finished(Err(error))) => {
            eprintln!("failed: {error}");
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("never finished");
            std::process::exit(1);
        }
    }
}
