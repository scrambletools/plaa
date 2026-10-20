//! Records the screen as the screen recording window does, with the
//! sound of an input device when one is given: on Linux a PipeWire video
//! node, as the ScreenCast portal shares one, and on Windows and macOS a
//! screen by its number.
//!
//! cargo run --release -p plaa-media --example screen -- [--format
//! h264|hevc|av1] [--software] [--rate <pictures a second>] [--sound
//! <device id>] <node or screen> <seconds> <to>

fn main() {
    record::main();
}

mod record {
    #[cfg(target_os = "linux")]
    use std::os::unix::net::UnixStream;
    use std::path::PathBuf;
    use std::sync::mpsc;
    use std::time::{Duration, Instant};

    use plaa_media::export::Format;
    use plaa_media::record::{Microphone, RecordEvent, input_devices};
    use plaa_media::screen::{ScreenCapture, ScreenEncoding};

    pub fn main() {
        let mut arguments = std::env::args().skip(1).peekable();
        let mut format = Format::H264;
        let mut hardware = true;
        let mut sound = None;
        let mut rate = None;
        while let Some(option) = arguments.next_if(|argument| argument.starts_with("--")) {
            match option.as_str() {
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
        let (Some(node), Some(seconds), Some(destination)) = (
            arguments.next().and_then(|text| text.parse::<u32>().ok()),
            arguments.next().and_then(|text| text.parse::<f64>().ok()),
            arguments.next(),
        ) else {
            eprintln!(
                "usage: screen [--format h264|hevc|av1] [--software] [--rate <per second>] \
                 [--sound <device id>] \
                 <node or screen> <seconds> <to>"
            );
            std::process::exit(2);
        };
        let mut capture = match open(node) {
            Ok(capture) => capture,
            Err(error) => {
                eprintln!("could not open {node}: {error}");
                std::process::exit(1);
            }
        };
        // As the window shows the capture a while before recording starts.
        std::thread::sleep(Duration::from_secs(1));
        // An id that is not listed would record the default input instead.
        if let Some(device) = &sound
            && !input_devices().iter().any(|listed| listed.id == *device)
        {
            eprintln!("no input {device}; the record example's --list shows them");
            std::process::exit(1);
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
            ScreenEncoding {
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
                println!("the stream ended: {reason}");
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

    /// PipeWire node `node`, through the sound server's own connection,
    /// which reaches every node; the portal hands over one of its own.
    #[cfg(target_os = "linux")]
    fn open(node: u32) -> Result<ScreenCapture, String> {
        let runtime = std::env::var_os("XDG_RUNTIME_DIR").unwrap_or_default();
        let socket = UnixStream::connect(PathBuf::from(runtime).join("pipewire-0"))
            .map_err(|error| format!("could not reach PipeWire: {error}"))?;
        ScreenCapture::open(socket.into(), node)
    }

    /// Screen number `screen`.
    #[cfg(any(windows, target_os = "macos"))]
    fn open(screen: u32) -> Result<ScreenCapture, String> {
        ScreenCapture::open(plaa_media::screen::Target::Screen(screen))
    }
}
