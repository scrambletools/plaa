//! plaa's AVB side on its own: lists the entities on an interface, or runs
//! plaa's entity sending a quiet tone or silence, or connects and
//! disconnects streams as plaa's controller would. Raw Ethernet needs
//! CAP_NET_RAW on Linux (`sudo setcap cap_net_raw+ep <this program>`).
//!
//! cargo run -p plaa-avb --example avb -- list <interface>
//! cargo run -p plaa-avb --example avb -- talk <interface> <seconds> [--tone]
//! cargo run -p plaa-avb --example avb -- connect <interface> <talker>:<output> <listener>:<input>
//! cargo run -p plaa-avb --example avb -- disconnect <interface> <listener>:<input>

use std::time::{Duration, Instant};

use plaa_avb::{Config, EntityId, Network, Snapshot};

fn usage() -> ! {
    eprintln!(
        "usage: avb list <interface> | talk <interface> <seconds> [--tone] | \
         connect <interface> <talker>:<output> <listener>:<input> | \
         disconnect <interface> <listener>:<input>"
    );
    std::process::exit(2);
}

/// An entity's stream, as `0x...:index`.
fn end(text: &str) -> Option<(EntityId, u16)> {
    let (id, index) = text.split_once(':')?;
    let id = u64::from_str_radix(id.trim_start_matches("0x"), 16).ok()?;
    Some((EntityId(id), index.parse().ok()?))
}

fn print(snapshot: &Snapshot) {
    println!("talker: {:?}", snapshot.talker);
    if let Some(failure) = &snapshot.failure {
        println!("failed: {failure}");
    }
    for found in &snapshot.entities {
        let own = if found.own { "  (plaa)" } else { "" };
        println!("{} \"{}\"{own}", found.id, found.name);
        for output in &found.outputs {
            println!(
                "  out {} \"{}\"  {}",
                output.index, output.name, output.format
            );
        }
        for input in &found.inputs {
            let bound = input.bound.map_or_else(
                || "not bound".to_owned(),
                |(id, index)| format!("bound to {id}:{index}"),
            );
            println!(
                "  in  {} \"{}\"  {}  {bound}",
                input.index, input.name, input.format
            );
        }
    }
}

fn open(interface: &str, talker: bool) -> Network {
    let config = Config {
        interface: interface.to_owned(),
        talker,
        name: "plaa test".into(),
        address_file: None,
    };
    Network::start(config, || {}).unwrap_or_else(|error| {
        eprintln!("could not start on {interface}: {error}");
        std::process::exit(1);
    })
}

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let words: Vec<&str> = arguments.iter().map(String::as_str).collect();
    match words.as_slice() {
        ["list", interface] => {
            let network = open(interface, false);
            std::thread::sleep(Duration::from_secs(5));
            print(&network.snapshot());
        }
        ["talk", interface, seconds, rest @ ..] => {
            let seconds: f64 = seconds.parse().unwrap_or_else(|_| usage());
            let tone = rest.contains(&"--tone");
            let network = open(interface, true);
            let sound = network.sound();
            let started = Instant::now();
            let mut last = None;
            let mut phase = 0.0f64;
            // Ten milliseconds at a time, as an output's device would take
            // them: a quiet 440 Hz, or silence.
            while started.elapsed().as_secs_f64() < seconds {
                let mut frames = [0.0f32; 960];
                if tone {
                    for frame in frames.as_chunks_mut::<2>().0 {
                        let sample = (0.05 * phase.sin()) as f32;
                        frame.fill(sample);
                        phase += std::f64::consts::TAU * 440.0 / 48_000.0;
                    }
                }
                sound.take(&frames, 48_000);
                std::thread::sleep(Duration::from_millis(10));
                let snapshot = network.snapshot();
                if last.as_ref() != Some(&snapshot.talker) {
                    println!(
                        "{:.1} s: {:?}",
                        started.elapsed().as_secs_f64(),
                        snapshot.talker
                    );
                    last = Some(snapshot.talker);
                }
            }
            print(&network.snapshot());
        }
        ["connect", interface, talker, listener] => {
            let (Some(talker), Some(listener)) = (end(talker), end(listener)) else {
                usage();
            };
            let network = open(interface, false);
            std::thread::sleep(Duration::from_secs(5));
            network.connect(talker, listener, None);
            std::thread::sleep(Duration::from_secs(3));
            print(&network.snapshot());
        }
        ["disconnect", interface, listener] => {
            let Some(listener) = end(listener) else {
                usage();
            };
            let network = open(interface, false);
            std::thread::sleep(Duration::from_secs(5));
            network.disconnect(listener);
            std::thread::sleep(Duration::from_secs(3));
            print(&network.snapshot());
        }
        _ => usage(),
    }
}
