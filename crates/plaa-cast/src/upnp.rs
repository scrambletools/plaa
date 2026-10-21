//! The UPnP calls plaa makes of a renderer: AVTransport's to give it the
//! media and play, pause, seek and stop it, and to ask where it is;
//! RenderingControl's to set its volume. Each is a SOAP request.

use std::io;

use crate::discovery::{Dlna, Service};
use crate::http;

/// What the renderer is doing, as AVTransport says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Playing,
    Paused,
    Stopped,
    /// Loading or seeking.
    Transitioning,
    /// Nothing given to play.
    NoMedia,
}

impl State {
    fn parse(text: &str) -> State {
        match text.trim() {
            "PLAYING" => State::Playing,
            "PAUSED_PLAYBACK" | "PAUSED_RECORDING" => State::Paused,
            "TRANSITIONING" => State::Transitioning,
            "NO_MEDIA_PRESENT" => State::NoMedia,
            _ => State::Stopped,
        }
    }
}

/// Calls `action` of `service` with `arguments`, returning the answer's
/// body; a SOAP fault is an error with its description.
fn call(service: &Service, action: &str, arguments: &[(&str, String)]) -> io::Result<String> {
    let mut body = format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\
<s:Envelope xmlns:s=\"http://schemas.xmlsoap.org/soap/envelope/\" s:encodingStyle=\"http://schemas.xmlsoap.org/soap/encoding/\">\
<s:Body><u:{action} xmlns:u=\"{}\">",
        service.kind
    );
    for (name, value) in arguments {
        body.push_str(&format!("<{name}>{}</{name}>", escape(value)));
    }
    body.push_str(&format!("</u:{action}></s:Body></s:Envelope>"));
    let soap_action = format!("\"{}#{action}\"", service.kind);
    let response = http::request(
        "POST",
        &service.control,
        &[
            ("Content-Type", "text/xml; charset=\"utf-8\""),
            ("SOAPACTION", &soap_action),
        ],
        body.as_bytes(),
    )?;
    let text = String::from_utf8_lossy(&response.body).into_owned();
    if response.status != 200 {
        let fault = value(&text, "errorDescription")
            .or_else(|| value(&text, "faultstring"))
            .unwrap_or_else(|| format!("HTTP {}", response.status));
        return Err(io::Error::other(format!("{action}: {fault}")));
    }
    Ok(text)
}

/// The text of the first element named `name` in `xml`.
fn value(xml: &str, name: &str) -> Option<String> {
    let document = roxmltree::Document::parse(xml).ok()?;
    document
        .descendants()
        .find(|node| node.tag_name().name() == name)
        .map(|node| node.text().unwrap_or_default().trim().to_owned())
}

pub(crate) fn escape(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            _ => escaped.push(character),
        }
    }
    escaped
}

fn instance() -> (&'static str, String) {
    ("InstanceID", "0".to_owned())
}

/// The DIDL-Lite that describes the media: its title, kind and where it
/// is, as renderers want beside the address.
pub(crate) fn metadata(url: &str, title: &str, mime: &str) -> String {
    let class = if mime.starts_with("audio/") {
        "object.item.audioItem.musicTrack"
    } else {
        "object.item.videoItem"
    };
    format!(
        "<DIDL-Lite xmlns=\"urn:schemas-upnp-org:metadata-1-0/DIDL-Lite/\" xmlns:dc=\"http://purl.org/dc/elements/1.1/\" xmlns:upnp=\"urn:schemas-upnp-org:metadata-1-0/upnp/\">\
<item id=\"0\" parentID=\"-1\" restricted=\"1\"><dc:title>{}</dc:title><upnp:class>{class}</upnp:class>\
<res protocolInfo=\"http-get:*:{mime}:*\">{}</res></item></DIDL-Lite>",
        escape(title),
        escape(url)
    )
}

pub(crate) fn set_uri(renderer: &Dlna, url: &str, metadata: &str) -> io::Result<()> {
    call(
        &renderer.av_transport,
        "SetAVTransportURI",
        &[
            instance(),
            ("CurrentURI", url.to_owned()),
            ("CurrentURIMetaData", metadata.to_owned()),
        ],
    )
    .map(drop)
}

pub(crate) fn play(renderer: &Dlna) -> io::Result<()> {
    call(
        &renderer.av_transport,
        "Play",
        &[instance(), ("Speed", "1".to_owned())],
    )
    .map(drop)
}

pub(crate) fn pause(renderer: &Dlna) -> io::Result<()> {
    call(&renderer.av_transport, "Pause", &[instance()]).map(drop)
}

pub(crate) fn stop(renderer: &Dlna) -> io::Result<()> {
    call(&renderer.av_transport, "Stop", &[instance()]).map(drop)
}

pub(crate) fn seek(renderer: &Dlna, seconds: f64) -> io::Result<()> {
    call(
        &renderer.av_transport,
        "Seek",
        &[
            instance(),
            ("Unit", "REL_TIME".to_owned()),
            ("Target", clock(seconds)),
        ],
    )
    .map(drop)
}

/// Where the renderer is in the media, and how long the media is, in
/// seconds, where it says.
pub(crate) fn position(renderer: &Dlna) -> io::Result<(Option<f64>, Option<f64>)> {
    let answer = call(&renderer.av_transport, "GetPositionInfo", &[instance()])?;
    Ok((
        value(&answer, "RelTime").as_deref().and_then(seconds),
        value(&answer, "TrackDuration").as_deref().and_then(seconds),
    ))
}

pub(crate) fn state(renderer: &Dlna) -> io::Result<State> {
    let answer = call(&renderer.av_transport, "GetTransportInfo", &[instance()])?;
    Ok(State::parse(
        &value(&answer, "CurrentTransportState").unwrap_or_default(),
    ))
}

/// Sets the renderer's volume, 0 to 100, where it has RenderingControl.
pub(crate) fn set_volume(renderer: &Dlna, volume: u8) -> io::Result<()> {
    let Some(service) = &renderer.rendering_control else {
        return Ok(());
    };
    call(
        service,
        "SetVolume",
        &[
            instance(),
            ("Channel", "Master".to_owned()),
            ("DesiredVolume", volume.min(100).to_string()),
        ],
    )
    .map(drop)
}

/// `seconds` as H:MM:SS, as REL_TIME takes it.
pub(crate) fn clock(seconds: f64) -> String {
    let whole = seconds.max(0.0).round() as u64;
    format!("{}:{:02}:{:02}", whole / 3600, whole / 60 % 60, whole % 60)
}

/// H:MM:SS, with or without fractions, as seconds; none for NOT_IMPLEMENTED
/// and the like.
pub(crate) fn seconds(text: &str) -> Option<f64> {
    let mut parts = text.trim().split(':');
    let hours: f64 = parts.next()?.parse().ok()?;
    let minutes: f64 = parts.next()?.parse().ok()?;
    let rest = parts.next()?;
    // Some write fractions as "SS.mmm", some as "SS.F/F".
    let seconds: f64 = match rest.split_once('/') {
        Some((whole, _)) => whole.rsplit_once('.').map_or(whole, |(whole, _)| whole),
        None => rest,
    }
    .parse()
    .ok()?;
    parts
        .next()
        .is_none()
        .then_some(hours * 3600.0 + minutes * 60.0 + seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn times_go_both_ways() {
        assert_eq!(clock(3725.4), "1:02:05");
        assert_eq!(clock(-3.0), "0:00:00");
        assert_eq!(seconds("1:02:05"), Some(3725.0));
        assert_eq!(seconds("00:00:07.500"), Some(7.5));
        assert_eq!(seconds("0:00:07.5/10"), Some(7.0));
        assert_eq!(seconds("NOT_IMPLEMENTED"), None);
        assert_eq!(seconds("1:2:3:4"), None);
    }

    #[test]
    fn states_are_read() {
        assert_eq!(State::parse("PLAYING"), State::Playing);
        assert_eq!(State::parse("PAUSED_PLAYBACK"), State::Paused);
        assert_eq!(State::parse("STOPPED"), State::Stopped);
        assert_eq!(State::parse("NO_MEDIA_PRESENT"), State::NoMedia);
    }

    #[test]
    fn metadata_describes_and_escapes() {
        let didl = metadata(
            "http://10.0.0.2:5000/a/media.mp3?x=1&y=2",
            "Rock & Roll",
            "audio/mpeg",
        );
        assert!(didl.contains("<dc:title>Rock &amp; Roll</dc:title>"));
        assert!(didl.contains("object.item.audioItem.musicTrack"));
        assert!(didl.contains("http-get:*:audio/mpeg:*"));
        assert!(didl.contains("x=1&amp;y=2"));
        assert!(roxmltree::Document::parse(&didl).is_ok());
        assert!(metadata("http://a/b", "t", "video/mp4").contains("videoItem"));
    }

    #[test]
    fn answers_and_faults_are_read() {
        let answer = r#"<?xml version="1.0"?><s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/"><s:Body><u:GetPositionInfoResponse xmlns:u="urn:schemas-upnp-org:service:AVTransport:1"><Track>1</Track><TrackDuration>0:03:30</TrackDuration><RelTime>0:01:15</RelTime></u:GetPositionInfoResponse></s:Body></s:Envelope>"#;
        assert_eq!(value(answer, "RelTime").as_deref(), Some("0:01:15"));
        assert_eq!(value(answer, "TrackDuration").as_deref(), Some("0:03:30"));
        let fault = r#"<s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/"><s:Body><s:Fault><faultcode>s:Client</faultcode><faultstring>UPnPError</faultstring><detail><UPnPError xmlns="urn:schemas-upnp-org:control-1-0"><errorCode>714</errorCode><errorDescription>Illegal MIME-type</errorDescription></UPnPError></detail></s:Fault></s:Body></s:Envelope>"#;
        assert_eq!(
            value(fault, "errorDescription").as_deref(),
            Some("Illegal MIME-type")
        );
    }
}
