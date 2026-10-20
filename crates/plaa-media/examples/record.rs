//! Records from a sound input device, as the recording window does, and
//! prints its levels as it goes.
//!
//! cargo run --release -p plaa-media --example record -- [--list]
//! [--device <id>] [--format aac|mp3|opus|flac|alac|wav] [--quality
//! <kb/s or bits>] [--gain <dB>] [--level] <seconds> <to>

use std::path::PathBuf;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use plaa_media::export::Sound;
use plaa_media::record::{Microphone, RecordEvent, input_devices};

fn main() {
    let mut arguments = std::env::args().skip(1).peekable();
    let mut device = None;
    let mut sound = Sound::Aac;
    let mut quality = None;
    let mut gain = 0.0;
    let mut leveled = false;
    while let Some(option) = arguments.next_if(|argument| argument.starts_with("--")) {
        match option.as_str() {
            "--list" => {
                for device in input_devices() {
                    println!("{}  {}", device.id, device.name);
                }
                return;
            }
            "--device" => device = arguments.next(),
            "--format" => {
                sound = arguments
                    .next()
                    .and_then(|key| Sound::from_key(&key))
                    .unwrap_or(Sound::Aac)
            }
            "--quality" => quality = arguments.next().and_then(|text| text.parse().ok()),
            "--gain" => {
                gain = arguments
                    .next()
                    .and_then(|text| text.parse().ok())
                    .unwrap_or(0.0)
            }
            "--level" => leveled = true,
            other => {
                eprintln!("unknown option {other}");
                std::process::exit(2);
            }
        }
    }
    let (Some(seconds), Some(destination)) = (
        arguments.next().and_then(|text| text.parse::<f64>().ok()),
        arguments.next(),
    ) else {
        eprintln!(
            "usage: record [--list] [--device <id>] [--format aac|mp3|opus|flac|alac|wav] \
             [--quality <kb/s or bits>] [--gain <dB>] [--level] <seconds> <to>"
        );
        std::process::exit(2);
    };
    // An id that is not listed would record the default input instead.
    if let Some(device) = &device
        && !input_devices().iter().any(|listed| listed.id == *device)
    {
        eprintln!("no input {device}; --list shows them");
        std::process::exit(1);
    }
    let mut microphone = match Microphone::open(device.as_deref()) {
        Ok(microphone) => microphone,
        Err(error) => {
            eprintln!("could not open the input: {error}");
            std::process::exit(1);
        }
    };
    println!("recording from {}", microphone.name());
    microphone.set_gain(gain);
    microphone.set_leveled(leveled);
    let (sender, receiver) = mpsc::channel();
    let quality = quality.unwrap_or(sound.usual_quality());
    if let Err(error) =
        microphone.record(PathBuf::from(destination), (sound, quality), move |event| {
            let _ = sender.send(event);
        })
    {
        eprintln!("could not record: {error}");
        std::process::exit(1);
    }
    let started = Instant::now();
    while started.elapsed().as_secs_f64() < seconds {
        std::thread::sleep(Duration::from_millis(500));
        let levels: Vec<String> = microphone
            .levels()
            .iter()
            .map(|level| format!("{level:.3}"))
            .collect();
        let (recorded, written) = microphone.recorded();
        println!(
            "{:.1} s recorded, {written} bytes, levels {}",
            recorded.as_secs_f64(),
            levels.join(" ")
        );
    }
    microphone.stop();
    if let Ok(RecordEvent::Finished(result)) = receiver.recv_timeout(Duration::from_secs(10)) {
        println!("finished: {result:?}");
    }
}
