//! Lists the sound output devices, as Settings offers them, and how long
//! asking the system took.
//!
//! cargo run --release -p plaa-media --example devices

fn main() {
    let asked = std::time::Instant::now();
    let devices = plaa_media::output_devices();
    println!("{} devices in {:?}", devices.len(), asked.elapsed());
    for device in devices {
        println!("{}  ({})", device.name, device.id);
    }
}
