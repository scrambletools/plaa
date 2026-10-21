//! Finding Chromecasts: an mDNS question for `_googlecast._tcp.local`,
//! asked from a port of its own so devices answer it directly (RFC 6762,
//! 6.7), and their answers read for each device's name, address and port.

use std::collections::HashMap;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

/// Where mDNS questions go.
const MDNS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(224, 0, 0, 251)), 5353);

const SERVICE: &str = "_googlecast._tcp.local";

const PTR: u16 = 12;
const TXT: u16 = 16;
const SRV: u16 = 33;
const A: u16 = 1;

/// A Chromecast, as its answers describe it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Device {
    /// Its name, as the user gave it: TXT's `fn`.
    pub name: String,
    /// Its id: TXT's `id`, else its instance name.
    pub id: String,
    pub address: SocketAddr,
}

/// Asks for Chromecasts for `wait`.
pub(crate) fn discover(wait: Duration) -> io::Result<Vec<Device>> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
    socket.set_multicast_ttl_v4(255)?;
    let question = question(SERVICE);
    for _ in 0..2 {
        socket.send_to(&question, MDNS)?;
    }
    let deadline = Instant::now() + wait;
    let mut records = Records::default();
    let mut buffer = [0u8; 9000];
    while let Some(left) = deadline.checked_duration_since(Instant::now()) {
        socket.set_read_timeout(Some(left.max(Duration::from_millis(1))))?;
        match socket.recv_from(&mut buffer) {
            Ok((length, from)) => records.read(&buffer[..length], from.ip()),
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                ) =>
            {
                break;
            }
            Err(error) => return Err(error),
        }
    }
    Ok(records.devices())
}

/// A question for the PTR records of `service`.
fn question(service: &str) -> Vec<u8> {
    // ID 0, no flags, one question.
    let mut packet = vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0];
    for label in service.split('.') {
        packet.push(label.len() as u8);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0);
    packet.extend_from_slice(&PTR.to_be_bytes());
    // Class IN.
    packet.extend_from_slice(&1u16.to_be_bytes());
    packet
}

/// What the answers said so far.
#[derive(Debug, Default)]
struct Records {
    /// The service's instances.
    instances: Vec<String>,
    /// Each instance's host and port.
    services: HashMap<String, (String, u16)>,
    texts: HashMap<String, HashMap<String, String>>,
    hosts: HashMap<String, Ipv4Addr>,
    /// Where each instance answered from, should no A record come.
    senders: HashMap<String, IpAddr>,
}

impl Records {
    fn read(&mut self, packet: &[u8], from: IpAddr) {
        let Some(records) = parse(packet) else {
            return;
        };
        for record in records {
            match record.kind {
                PTR if record.name.eq_ignore_ascii_case(SERVICE) => {
                    if let Some(instance) = name_at(packet, record.data_at) {
                        self.senders.insert(instance.clone(), from);
                        if !self.instances.contains(&instance) {
                            self.instances.push(instance);
                        }
                    }
                }
                SRV => {
                    let data = &packet[record.data_at..record.data_at + record.data.len()];
                    if data.len() > 6 {
                        let port = u16::from_be_bytes([data[4], data[5]]);
                        if let Some(target) = name_at(packet, record.data_at + 6) {
                            self.services.insert(record.name.clone(), (target, port));
                        }
                    }
                }
                TXT => {
                    self.texts.insert(record.name.clone(), text(record.data));
                }
                A if record.data.len() == 4 => {
                    let [a, b, c, d] = [
                        record.data[0],
                        record.data[1],
                        record.data[2],
                        record.data[3],
                    ];
                    self.hosts
                        .insert(record.name.clone(), Ipv4Addr::new(a, b, c, d));
                }
                _ => {}
            }
        }
    }

    fn devices(&self) -> Vec<Device> {
        let mut devices: Vec<Device> = self
            .instances
            .iter()
            .filter_map(|instance| {
                let (host, port) = self.services.get(instance)?;
                let ip = self
                    .hosts
                    .get(host)
                    .map(|ip| IpAddr::V4(*ip))
                    .or_else(|| self.senders.get(instance).copied())?;
                let text = self.texts.get(instance);
                let field = |key: &str| text.and_then(|text| text.get(key)).cloned();
                Some(Device {
                    name: field("fn").unwrap_or_else(|| {
                        instance
                            .strip_suffix(&format!(".{SERVICE}"))
                            .unwrap_or(instance)
                            .to_owned()
                    }),
                    id: field("id").unwrap_or_else(|| instance.clone()),
                    address: SocketAddr::new(ip, *port),
                })
            })
            .collect();
        devices.sort_by_key(|device| device.name.to_lowercase());
        devices.dedup_by(|left, right| left.id == right.id);
        devices
    }
}

/// A resource record: its name, type, and data, with where the data
/// starts in the packet, as names in it point back into the packet.
struct Record<'a> {
    name: String,
    kind: u16,
    data: &'a [u8],
    data_at: usize,
}

/// The records of every section of a DNS packet.
fn parse(packet: &[u8]) -> Option<Vec<Record<'_>>> {
    let count = |at: usize| Some(u16::from_be_bytes([*packet.get(at)?, *packet.get(at + 1)?]));
    let questions = count(4)?;
    let records = u32::from(count(6)?) + u32::from(count(8)?) + u32::from(count(10)?);
    let mut at = 12;
    for _ in 0..questions {
        at = skip_name(packet, at)? + 4;
    }
    let mut found = Vec::new();
    for _ in 0..records {
        let name = name_at(packet, at)?;
        at = skip_name(packet, at)?;
        let kind = count(at)?;
        let length = usize::from(count(at + 8)?);
        let data_at = at + 10;
        let data = packet.get(data_at..data_at + length)?;
        found.push(Record {
            name,
            kind,
            data,
            data_at,
        });
        at = data_at + length;
    }
    Some(found)
}

/// Where the name at `at` ends.
fn skip_name(packet: &[u8], mut at: usize) -> Option<usize> {
    loop {
        let length = *packet.get(at)?;
        if length == 0 {
            return Some(at + 1);
        }
        if length & 0xc0 == 0xc0 {
            return Some(at + 2);
        }
        at += 1 + usize::from(length);
    }
}

/// The name at `at`, following pointers back into the packet.
fn name_at(packet: &[u8], mut at: usize) -> Option<String> {
    let mut labels = Vec::new();
    // Pointers only go back; a bound stops a loop of them.
    for _ in 0..128 {
        let length = *packet.get(at)?;
        if length == 0 {
            return Some(labels.join("."));
        }
        if length & 0xc0 == 0xc0 {
            at = usize::from(u16::from_be_bytes([length & 0x3f, *packet.get(at + 1)?]));
            continue;
        }
        let label = packet.get(at + 1..at + 1 + usize::from(length))?;
        labels.push(String::from_utf8_lossy(label).into_owned());
        at += 1 + usize::from(length);
    }
    None
}

/// A TXT record's `key=value` strings.
fn text(data: &[u8]) -> HashMap<String, String> {
    let mut fields = HashMap::new();
    let mut at = 0;
    while let Some(&length) = data.get(at) {
        let Some(entry) = data.get(at + 1..at + 1 + usize::from(length)) else {
            break;
        };
        let entry = String::from_utf8_lossy(entry);
        if let Some((key, value)) = entry.split_once('=') {
            fields.insert(key.to_owned(), value.to_owned());
        }
        at += 1 + usize::from(length);
    }
    fields
}

#[cfg(test)]
mod tests {
    use super::*;

    fn name(packet: &mut Vec<u8>, name: &str) {
        for label in name.split('.') {
            packet.push(label.len() as u8);
            packet.extend_from_slice(label.as_bytes());
        }
        packet.push(0);
    }

    fn record(packet: &mut Vec<u8>, owner: &str, kind: u16, data: &[u8]) {
        name(packet, owner);
        packet.extend_from_slice(&kind.to_be_bytes());
        packet.extend_from_slice(&[0x80, 1, 0, 0, 0x11, 0x94]);
        packet.extend_from_slice(&(data.len() as u16).to_be_bytes());
        packet.extend_from_slice(data);
    }

    #[test]
    fn questions_ask_for_the_service() {
        let packet = question(SERVICE);
        assert_eq!(&packet[4..6], &[0, 1]);
        assert_eq!(packet[12], 11);
        assert_eq!(&packet[13..24], b"_googlecast");
        assert_eq!(&packet[packet.len() - 4..], &[0, 12, 0, 1]);
    }

    /// A Chromecast's answer: the PTR to its instance, which a pointer
    /// names, and the instance's SRV, TXT and A.
    #[test]
    fn answers_give_name_address_and_port() {
        let instance = "Chromecast-abc123._googlecast._tcp.local";
        let mut packet = vec![0, 0, 0x84, 0, 0, 0, 0, 4, 0, 0, 0, 0];
        // The PTR's data names the instance in full; the records after
        // name it by a pointer to that, as devices do.
        name(&mut packet, SERVICE);
        packet.extend_from_slice(&PTR.to_be_bytes());
        packet.extend_from_slice(&[0, 1, 0, 0, 0x11, 0x94]);
        let mut target = Vec::new();
        name(&mut target, instance);
        packet.extend_from_slice(&(target.len() as u16).to_be_bytes());
        let instance_at = packet.len() as u16;
        packet.extend_from_slice(&target);
        let pointer = [0xc0 | (instance_at >> 8) as u8, instance_at as u8];
        let mut srv = vec![0, 0, 0, 0, 0x1f, 0x49];
        name(&mut srv, "abc123.local");
        packet.extend_from_slice(&pointer);
        packet.extend_from_slice(&SRV.to_be_bytes());
        packet.extend_from_slice(&[0x80, 1, 0, 0, 0, 120]);
        packet.extend_from_slice(&(srv.len() as u16).to_be_bytes());
        packet.extend_from_slice(&srv);
        let mut txt = Vec::new();
        for field in ["id=abc123", "fn=Living Room TV", "md=Chromecast"] {
            txt.push(field.len() as u8);
            txt.extend_from_slice(field.as_bytes());
        }
        record(&mut packet, instance, TXT, &txt);
        record(&mut packet, "abc123.local", A, &[192, 168, 1, 40]);
        let mut records = Records::default();
        records.read(&packet, "192.168.1.40".parse().unwrap());
        assert_eq!(
            records.devices(),
            vec![Device {
                name: "Living Room TV".to_owned(),
                id: "abc123".to_owned(),
                address: "192.168.1.40:8009".parse().unwrap(),
            }]
        );
        // A packet cut short is skipped.
        records.read(&packet[..20], "192.168.1.40".parse().unwrap());
        assert_eq!(records.devices().len(), 1);
    }
}
