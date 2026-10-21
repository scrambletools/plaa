//! Finding the renderers on the network: an SSDP search for UPnP media
//! renderers, then each one's description, for its name and where its
//! AVTransport and RenderingControl services take their calls.

use std::collections::BTreeSet;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

use crate::http::{self, Url};

/// Where SSDP searches go.
const SSDP: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(239, 255, 255, 250)), 1900);

/// What a renderer is, as a search finds it.
const RENDERER: &str = "urn:schemas-upnp-org:device:MediaRenderer:1";

const AV_TRANSPORT: &str = "urn:schemas-upnp-org:service:AVTransport";
const RENDERING_CONTROL: &str = "urn:schemas-upnp-org:service:RenderingControl";

/// A renderer that can play what plaa sends it: a TV, a speaker, or a
/// program such as Kodi.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Renderer {
    /// Its name, as it gives it.
    pub name: String,
    /// Its unique device name, which stays as it moves.
    pub id: String,
    pub(crate) av_transport: Service,
    pub(crate) rendering_control: Option<Service>,
}

/// A service of a renderer: its type, with its version, and where it takes
/// its calls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Service {
    pub kind: String,
    pub control: Url,
}

impl Renderer {
    /// The renderer's network address, which a server for it listens on
    /// the computer's own address toward.
    pub(crate) fn host(&self) -> &str {
        &self.av_transport.control.host
    }
}

/// Searches for renderers for `wait`, and reads each one's description.
pub fn discover(wait: Duration) -> io::Result<Vec<Renderer>> {
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
    socket.set_multicast_ttl_v4(2)?;
    let search = format!(
        "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nMAN: \"ssdp:discover\"\r\nMX: {}\r\nST: {RENDERER}\r\n\r\n",
        wait.as_secs().clamp(1, 5)
    );
    // Twice, as UDP may lose one.
    for _ in 0..2 {
        socket.send_to(search.as_bytes(), SSDP)?;
    }
    let deadline = Instant::now() + wait;
    let mut locations = BTreeSet::new();
    let mut buffer = [0u8; 2048];
    while let Some(left) = deadline.checked_duration_since(Instant::now()) {
        socket.set_read_timeout(Some(left.max(Duration::from_millis(1))))?;
        match socket.recv_from(&mut buffer) {
            Ok((length, _)) => {
                if let Some(location) = location(&String::from_utf8_lossy(&buffer[..length])) {
                    locations.insert(location);
                }
            }
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
    // Each description read on a thread of its own, as some answer slowly.
    let reads: Vec<_> = locations
        .into_iter()
        .filter_map(|location| Url::parse(&location))
        .map(|url| std::thread::spawn(move || describe(&url)))
        .collect();
    let mut found: Vec<Renderer> = reads
        .into_iter()
        .filter_map(|read| read.join().ok().flatten())
        .collect();
    found.sort_by_key(|renderer| renderer.name.to_lowercase());
    found.dedup_by(|left, right| left.id == right.id);
    Ok(found)
}

/// The LOCATION header of a search's answer for a renderer.
fn location(answer: &str) -> Option<String> {
    let mut lines = answer.lines();
    if !lines.next()?.starts_with("HTTP/1.1 200") {
        return None;
    }
    let mut location = None;
    let mut renderer = false;
    for line in lines {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        let value = value.trim();
        if name.eq_ignore_ascii_case("location") {
            location = Some(value.to_owned());
        } else if name.eq_ignore_ascii_case("st") {
            renderer = value.starts_with("urn:schemas-upnp-org:device:MediaRenderer:");
        }
    }
    location.filter(|_| renderer)
}

/// The renderer the description at `url` describes, if it has an
/// AVTransport service.
fn describe(url: &Url) -> Option<Renderer> {
    let response = http::get(url)
        .ok()
        .filter(|response| response.status == 200)?;
    parse_description(url, &String::from_utf8_lossy(&response.body))
}

/// The renderer in a description read from `url`: the device, or the one
/// within it, that has an AVTransport service.
pub(crate) fn parse_description(url: &Url, text: &str) -> Option<Renderer> {
    let document = roxmltree::Document::parse(text).ok()?;
    let base = document
        .descendants()
        .find(|node| node.has_tag_name("URLBase"))
        .and_then(|node| node.text())
        .and_then(Url::parse)
        .unwrap_or_else(|| url.clone());
    let child_text = |node: roxmltree::Node<'_, '_>, name: &str| {
        node.children()
            .find(|child| child.has_tag_name(name))
            .and_then(|child| child.text())
            .map(|text| text.trim().to_owned())
    };
    document
        .descendants()
        .filter(|node| node.has_tag_name("device"))
        .find_map(|device| {
            let services: Vec<Service> = device
                .children()
                .filter(|child| child.has_tag_name("serviceList"))
                .flat_map(|list| list.children().filter(|node| node.has_tag_name("service")))
                .filter_map(|service| {
                    Some(Service {
                        kind: child_text(service, "serviceType")?,
                        control: base.join(&child_text(service, "controlURL")?)?,
                    })
                })
                .collect();
            let find = |kind: &str| {
                services
                    .iter()
                    .find(|service| service.kind.starts_with(kind))
                    .cloned()
            };
            Some(Renderer {
                name: child_text(device, "friendlyName").unwrap_or_else(|| url.host.clone()),
                id: child_text(device, "UDN").unwrap_or_else(|| url.to_string()),
                av_transport: find(AV_TRANSPORT)?,
                rendering_control: find(RENDERING_CONTROL),
            })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_name_where_renderers_describe_themselves() {
        let answer = "HTTP/1.1 200 OK\r\nCACHE-CONTROL: max-age=1800\r\nLOCATION: http://10.0.0.5:1400/xml/device_description.xml\r\nST: urn:schemas-upnp-org:device:MediaRenderer:1\r\nUSN: uuid:RINCON_1::urn:schemas-upnp-org:device:MediaRenderer:1\r\n\r\n";
        assert_eq!(
            location(answer).as_deref(),
            Some("http://10.0.0.5:1400/xml/device_description.xml")
        );
        let server = answer.replace("MediaRenderer", "MediaServer");
        assert_eq!(location(&server), None);
        assert_eq!(location("NOTIFY * HTTP/1.1\r\n\r\n"), None);
    }

    /// A speaker that keeps its renderer within a root device, as Sonos
    /// does, its control URLs relative to the description.
    #[test]
    fn a_renderer_within_a_device_is_found() {
        let description = r#"<?xml version="1.0"?>
<root xmlns="urn:schemas-upnp-org:device-1-0">
  <device>
    <deviceType>urn:schemas-upnp-org:device:ZonePlayer:1</deviceType>
    <friendlyName>Kitchen</friendlyName>
    <UDN>uuid:RINCON_1</UDN>
    <serviceList><service>
      <serviceType>urn:schemas-upnp-org:service:AlarmClock:1</serviceType>
      <controlURL>/AlarmClock/Control</controlURL>
    </service></serviceList>
    <deviceList><device>
      <deviceType>urn:schemas-upnp-org:device:MediaRenderer:1</deviceType>
      <friendlyName>Kitchen - Sonos One Media Renderer</friendlyName>
      <UDN>uuid:RINCON_1_MR</UDN>
      <serviceList>
        <service>
          <serviceType>urn:schemas-upnp-org:service:RenderingControl:1</serviceType>
          <controlURL>/MediaRenderer/RenderingControl/Control</controlURL>
        </service>
        <service>
          <serviceType>urn:schemas-upnp-org:service:AVTransport:1</serviceType>
          <controlURL>MediaRenderer/AVTransport/Control</controlURL>
        </service>
      </serviceList>
    </device></deviceList>
  </device>
</root>"#;
        let url = Url::parse("http://10.0.0.5:1400/xml/device_description.xml").unwrap();
        let renderer = parse_description(&url, description).unwrap();
        assert_eq!(renderer.name, "Kitchen - Sonos One Media Renderer");
        assert_eq!(renderer.id, "uuid:RINCON_1_MR");
        assert_eq!(
            renderer.av_transport.control.to_string(),
            "http://10.0.0.5:1400/xml/MediaRenderer/AVTransport/Control"
        );
        assert_eq!(
            renderer.rendering_control.as_ref().unwrap().control.path,
            "/MediaRenderer/RenderingControl/Control"
        );
        assert_eq!(renderer.host(), "10.0.0.5");
        // A device with no AVTransport is no renderer.
        let server = description.replace("AVTransport", "ContentDirectory");
        assert!(parse_description(&url, &server).is_none());
    }
}
