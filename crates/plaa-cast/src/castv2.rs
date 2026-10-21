//! Casting to a Chromecast: its CASTV2 protocol, JSON messages in
//! protobuf frames over TLS on port 8009. plaa launches the Default Media
//! Receiver, has it load the media's address, and drives it as a DLNA
//! renderer is driven: play, pause, seek, volume, and its status every
//! second.
//!
//! Chromecasts present certificates of their own making, so the
//! connection takes any; what it carries is media addresses on the local
//! network.

use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::Arc;
use std::time::{Duration, Instant};

use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::crypto::CryptoProvider;
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{ClientConfig, ClientConnection, DigitallySignedStruct, SignatureScheme, StreamOwned};
use serde_json::{Value, json};

use crate::upnp::State;
use crate::{Command, Event, Status};

/// The Default Media Receiver, which plays a media address.
const MEDIA_RECEIVER: &str = "CC1AD845";

const CONNECTION: &str = "urn:x-cast:com.google.cast.tp.connection";
const HEARTBEAT: &str = "urn:x-cast:com.google.cast.tp.heartbeat";
const RECEIVER: &str = "urn:x-cast:com.google.cast.receiver";
const MEDIA: &str = "urn:x-cast:com.google.cast.media";

const SENDER: &str = "sender-plaa";
const PLATFORM: &str = "receiver-0";

/// How long the device may take to answer a request.
const ANSWER: Duration = Duration::from_secs(10);
/// How often it is asked how it stands, and pinged.
const POLL: Duration = Duration::from_secs(1);
const PING: Duration = Duration::from_secs(5);

/// One message: its route, namespace and JSON.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Message {
    pub source: String,
    pub destination: String,
    pub namespace: String,
    pub payload: String,
}

impl Message {
    fn new(destination: &str, namespace: &str, payload: &Value) -> Message {
        Message {
            source: SENDER.to_owned(),
            destination: destination.to_owned(),
            namespace: namespace.to_owned(),
            payload: payload.to_string(),
        }
    }

    /// The message as CastMessage encodes it: protocol version 0, the
    /// route, the namespace, and a string payload.
    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        varint_field(&mut bytes, 1, 0);
        string_field(&mut bytes, 2, &self.source);
        string_field(&mut bytes, 3, &self.destination);
        string_field(&mut bytes, 4, &self.namespace);
        varint_field(&mut bytes, 5, 0);
        string_field(&mut bytes, 6, &self.payload);
        bytes
    }

    /// A CastMessage, its string fields read and the rest skipped.
    pub(crate) fn decode(mut bytes: &[u8]) -> Option<Message> {
        let mut message = Message {
            source: String::new(),
            destination: String::new(),
            namespace: String::new(),
            payload: String::new(),
        };
        while !bytes.is_empty() {
            let key = varint(&mut bytes)?;
            match key & 7 {
                0 => {
                    varint(&mut bytes)?;
                }
                2 => {
                    let length = usize::try_from(varint(&mut bytes)?).ok()?;
                    let value = bytes.get(..length)?;
                    bytes = &bytes[length..];
                    let text = String::from_utf8_lossy(value).into_owned();
                    match key >> 3 {
                        2 => message.source = text,
                        3 => message.destination = text,
                        4 => message.namespace = text,
                        6 => message.payload = text,
                        _ => {}
                    }
                }
                _ => return None,
            }
        }
        Some(message)
    }

    fn json(&self) -> Value {
        serde_json::from_str(&self.payload).unwrap_or(Value::Null)
    }
}

fn varint_field(bytes: &mut Vec<u8>, field: u64, value: u64) {
    put_varint(bytes, field << 3);
    put_varint(bytes, value);
}

fn string_field(bytes: &mut Vec<u8>, field: u64, value: &str) {
    put_varint(bytes, (field << 3) | 2);
    put_varint(bytes, value.len() as u64);
    bytes.extend_from_slice(value.as_bytes());
}

fn put_varint(bytes: &mut Vec<u8>, mut value: u64) {
    loop {
        let low = (value & 0x7f) as u8;
        value >>= 7;
        if value == 0 {
            bytes.push(low);
            return;
        }
        bytes.push(low | 0x80);
    }
}

fn varint(bytes: &mut &[u8]) -> Option<u64> {
    let mut value = 0u64;
    for shift in (0..64).step_by(7) {
        let (&byte, rest) = bytes.split_first()?;
        *bytes = rest;
        value |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Some(value);
        }
    }
    None
}

/// Takes the device's certificate, whatever signed it, while checking the
/// handshake's signatures with it.
#[derive(Debug)]
struct AnyCertificate(Arc<CryptoProvider>);

impl ServerCertVerifier for AnyCertificate {
    fn verify_server_cert(
        &self,
        _: &CertificateDer<'_>,
        _: &[CertificateDer<'_>],
        _: &ServerName<'_>,
        _: &[u8],
        _: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        certificate: &CertificateDer<'_>,
        signed: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(
            message,
            certificate,
            signed,
            &self.0.signature_verification_algorithms,
        )
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        certificate: &CertificateDer<'_>,
        signed: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            certificate,
            signed,
            &self.0.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.0.signature_verification_algorithms.supported_schemes()
    }
}

type Tls = StreamOwned<ClientConnection, TcpStream>;

/// A connection to a Chromecast.
struct Connection {
    stream: Tls,
    next_request: u64,
    pinged: Instant,
}

impl Connection {
    fn open(address: SocketAddr) -> io::Result<Connection> {
        let provider = Arc::new(rustls::crypto::aws_lc_rs::default_provider());
        let config = ClientConfig::builder_with_provider(provider.clone())
            .with_safe_default_protocol_versions()
            .map_err(io::Error::other)?
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(AnyCertificate(provider)))
            .with_no_client_auth();
        let name = ServerName::IpAddress(address.ip().into());
        let connection = ClientConnection::new(Arc::new(config), name).map_err(io::Error::other)?;
        let socket = TcpStream::connect_timeout(&address, ANSWER)?;
        socket.set_read_timeout(Some(POLL))?;
        let mut opened = Connection {
            stream: StreamOwned::new(connection, socket),
            next_request: 1,
            pinged: Instant::now(),
        };
        opened.send(&Message::new(
            PLATFORM,
            CONNECTION,
            &json!({ "type": "CONNECT" }),
        ))?;
        Ok(opened)
    }

    fn send(&mut self, message: &Message) -> io::Result<()> {
        let body = message.encode();
        let length = u32::try_from(body.len()).map_err(io::Error::other)?;
        self.stream.write_all(&length.to_be_bytes())?;
        self.stream.write_all(&body)?;
        self.stream.flush()
    }

    /// Sends `payload` to `destination` with a request id of its own,
    /// returning the id.
    fn request(
        &mut self,
        destination: &str,
        namespace: &str,
        mut payload: Value,
    ) -> io::Result<u64> {
        let id = self.next_request;
        self.next_request += 1;
        payload["requestId"] = json!(id);
        self.send(&Message::new(destination, namespace, &payload))?;
        Ok(id)
    }

    /// The next message, answering pings on the way; none when nothing
    /// came within the poll.
    fn receive(&mut self) -> io::Result<Option<Message>> {
        if self.pinged.elapsed() >= PING {
            self.pinged = Instant::now();
            self.send(&Message::new(
                PLATFORM,
                HEARTBEAT,
                &json!({ "type": "PING" }),
            ))?;
        }
        let mut length = [0u8; 4];
        match self.stream.read_exact(&mut length) {
            Ok(()) => {}
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                ) =>
            {
                return Ok(None);
            }
            Err(error) => return Err(error),
        }
        let length = u32::from_be_bytes(length) as usize;
        if length > 1 << 20 {
            return Err(io::Error::other("a message too long"));
        }
        let mut body = vec![0u8; length];
        self.stream.read_exact(&mut body)?;
        let Some(message) = Message::decode(&body) else {
            return Ok(None);
        };
        if message.namespace == HEARTBEAT && message.json()["type"] == "PING" {
            self.send(&Message::new(
                &message.source,
                HEARTBEAT,
                &json!({ "type": "PONG" }),
            ))?;
            return Ok(None);
        }
        Ok(Some(message))
    }

    /// Waits for the answer to request `id`.
    fn answer(&mut self, id: u64) -> io::Result<Value> {
        let started = Instant::now();
        while started.elapsed() < ANSWER {
            if let Some(message) = self.receive()? {
                let value = message.json();
                if value["requestId"] == json!(id) {
                    return Ok(value);
                }
            }
        }
        Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "the Chromecast did not answer",
        ))
    }
}

/// The media session plaa's cast runs on the device.
struct Session {
    transport: String,
    session: String,
    media: Option<u64>,
}

/// Casts `url` to the Chromecast at `address`, as `mime`, titled `title`,
/// from `start` seconds, at `volume`, until `commands` closes.
#[allow(clippy::too_many_arguments)]
pub(crate) fn run(
    address: SocketAddr,
    url: &str,
    mime: &str,
    title: &str,
    start: f64,
    volume: Option<u8>,
    commands: &std::sync::mpsc::Receiver<Command>,
    events: &impl Fn(Event),
) -> Result<(), String> {
    let failed = |error: io::Error| error.to_string();
    let mut connection = Connection::open(address).map_err(failed)?;
    let id = connection
        .request(
            PLATFORM,
            RECEIVER,
            json!({ "type": "LAUNCH", "appId": MEDIA_RECEIVER }),
        )
        .map_err(failed)?;
    let status = connection.answer(id).map_err(failed)?;
    let application = status["status"]["applications"]
        .as_array()
        .and_then(|applications| {
            applications
                .iter()
                .find(|application| application["appId"] == MEDIA_RECEIVER)
        })
        .ok_or_else(|| "the Chromecast did not start its media player".to_owned())?;
    let mut session = Session {
        transport: application["transportId"]
            .as_str()
            .unwrap_or_default()
            .to_owned(),
        session: application["sessionId"]
            .as_str()
            .unwrap_or_default()
            .to_owned(),
        media: None,
    };
    connection
        .send(&Message::new(
            &session.transport,
            CONNECTION,
            &json!({ "type": "CONNECT" }),
        ))
        .map_err(failed)?;
    if let Some(volume) = volume {
        let _ = connection.request(
            PLATFORM,
            RECEIVER,
            json!({ "type": "SET_VOLUME", "volume": { "level": f64::from(volume.min(100)) / 100.0 } }),
        );
    }
    let load = json!({
        "type": "LOAD",
        "sessionId": session.session,
        "autoplay": true,
        "currentTime": start.max(0.0),
        "media": {
            "contentId": url,
            "streamType": "BUFFERED",
            "contentType": mime,
            "metadata": { "metadataType": 0, "title": title },
        },
    });
    let id = connection
        .request(&session.transport, MEDIA, load)
        .map_err(failed)?;
    let loaded = connection.answer(id).map_err(failed)?;
    if loaded["type"] == "LOAD_FAILED" || loaded["type"] == "LOAD_CANCELLED" {
        return Err(
            "the Chromecast could not play it; it plays MP4, WebM, MP3 and the like".into(),
        );
    }
    session.media = media_session(&loaded);
    events(Event::Started);
    let mut played = false;
    let mut polled = Instant::now();
    loop {
        match commands.try_recv() {
            Ok(command) => {
                if let Some(media) = session.media {
                    let payload = match command {
                        Command::Play => Some(json!({ "type": "PLAY", "mediaSessionId": media })),
                        Command::Pause => Some(json!({ "type": "PAUSE", "mediaSessionId": media })),
                        Command::Seek(seconds) => Some(json!({
                            "type": "SEEK",
                            "mediaSessionId": media,
                            "currentTime": seconds.max(0.0),
                        })),
                        Command::Volume(volume) => {
                            let _ = connection.request(
                                PLATFORM,
                                RECEIVER,
                                json!({ "type": "SET_VOLUME", "volume": { "level": f64::from(volume.min(100)) / 100.0 } }),
                            );
                            None
                        }
                    };
                    if let Some(payload) = payload {
                        connection
                            .request(&session.transport, MEDIA, payload)
                            .map_err(failed)?;
                    }
                }
                continue;
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                // The cast ends: the device stops its player.
                let _ = connection.request(
                    PLATFORM,
                    RECEIVER,
                    json!({ "type": "STOP", "sessionId": session.session }),
                );
                return Ok(());
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
        }
        if polled.elapsed() >= POLL {
            polled = Instant::now();
            connection
                .request(&session.transport, MEDIA, json!({ "type": "GET_STATUS" }))
                .map_err(failed)?;
        }
        let Some(message) = connection.receive().map_err(failed)? else {
            continue;
        };
        let value = message.json();
        if message.namespace == RECEIVER
            && value["type"] == "RECEIVER_STATUS"
            && !value["status"]["applications"]
                .as_array()
                .is_some_and(|applications| {
                    applications
                        .iter()
                        .any(|application| application["sessionId"] == json!(session.session))
                })
        {
            // Another sender took the device over, or its user stopped it.
            events(Event::Ended);
            return Ok(());
        }
        if message.namespace != MEDIA || value["type"] != "MEDIA_STATUS" {
            continue;
        }
        if let Some(media) = media_session(&value) {
            session.media = Some(media);
        }
        let Some(status) = value["status"]
            .as_array()
            .and_then(|statuses| statuses.first())
        else {
            continue;
        };
        let state = match status["playerState"].as_str() {
            Some("PLAYING") => State::Playing,
            Some("PAUSED") => State::Paused,
            Some("BUFFERING") => State::Transitioning,
            _ => State::Stopped,
        };
        played |= state == State::Playing;
        events(Event::Status(Status {
            state,
            position: status["currentTime"].as_f64(),
            duration: status["media"]["duration"].as_f64(),
        }));
        if played && status["playerState"] == "IDLE" && status["idleReason"] == "FINISHED" {
            events(Event::Ended);
            return Ok(());
        }
    }
}

/// The media session a MEDIA_STATUS names.
fn media_session(value: &Value) -> Option<u64> {
    value["status"]
        .as_array()?
        .first()?
        .get("mediaSessionId")?
        .as_u64()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_go_both_ways() {
        let message = Message::new(
            "web-5",
            MEDIA,
            &json!({ "type": "PLAY", "mediaSessionId": 1 }),
        );
        let bytes = message.encode();
        // Protocol version 0 first, as the field CastMessage has.
        assert_eq!(&bytes[..2], &[0x08, 0x00]);
        assert_eq!(Message::decode(&bytes), Some(message));
        assert_eq!(Message::decode(&[0x12, 0x05, b'a']), None, "cut short");
    }

    #[test]
    fn varints_carry_long_lengths() {
        let mut bytes = Vec::new();
        put_varint(&mut bytes, 300);
        assert_eq!(bytes, [0xac, 0x02]);
        assert_eq!(varint(&mut bytes.as_slice()), Some(300));
        let long = "x".repeat(1000);
        let message = Message {
            payload: long.clone(),
            ..Message::new("a", "b", &Value::Null)
        };
        assert_eq!(Message::decode(&message.encode()).unwrap().payload, long);
    }

    #[test]
    fn media_sessions_are_read() {
        let status = json!({ "type": "MEDIA_STATUS", "status": [{ "mediaSessionId": 7, "playerState": "PLAYING" }] });
        assert_eq!(media_session(&status), Some(7));
        assert_eq!(media_session(&json!({ "status": [] })), None);
    }
}
