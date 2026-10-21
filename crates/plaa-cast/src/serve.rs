//! Serving the file cast to the renderer, which fetches it over HTTP and
//! seeks in it with ranges. Only the file, at an address with a random
//! part that only the renderer is told, on the computer's address toward
//! the renderer, while the cast lasts.

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::net::{IpAddr, Shutdown, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

/// How long a connection may sit idle before it is let go.
const IDLE: Duration = Duration::from_secs(30);

/// A file served until this is dropped.
pub(crate) struct Served {
    url: String,
    address: SocketAddr,
    stop: Arc<AtomicBool>,
}

impl Served {
    /// Where the renderer fetches it.
    pub(crate) fn url(&self) -> &str {
        &self.url
    }
}

impl Drop for Served {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        // Wakes the listener so it sees it should stop.
        let _ = TcpStream::connect_timeout(&self.address, Duration::from_millis(200));
    }
}

/// This computer's address toward `host`: the one its route to it leaves
/// from.
pub(crate) fn address_toward(host: &str) -> io::Result<IpAddr> {
    let socket = UdpSocket::bind(("0.0.0.0", 0))?;
    socket.connect((host, 9))?;
    Ok(socket.local_addr()?.ip())
}

/// Serves `path` as `mime` on `ip`.
pub(crate) fn serve(path: PathBuf, mime: &'static str, ip: IpAddr) -> io::Result<Served> {
    let listener = TcpListener::bind((ip, 0))?;
    let address = listener.local_addr()?;
    let token = token();
    let name = path
        .extension()
        .map(|extension| format!("media.{}", extension.to_string_lossy()))
        .unwrap_or_else(|| "media".to_owned());
    let route = format!("/{token}/{name}");
    let url = match ip {
        IpAddr::V4(_) => format!("http://{address}{route}"),
        IpAddr::V6(v6) => format!("http://[{v6}]:{}{route}", address.port()),
    };
    let stop = Arc::new(AtomicBool::new(false));
    let stopped = stop.clone();
    let path = Arc::new(path);
    std::thread::Builder::new()
        .name("plaa-cast-serve".into())
        .spawn(move || {
            for stream in listener.incoming() {
                if stopped.load(Ordering::Relaxed) {
                    return;
                }
                let Ok(stream) = stream else { continue };
                let path = path.clone();
                let route = route.clone();
                let _ = std::thread::Builder::new()
                    .name("plaa-cast-client".into())
                    .spawn(move || {
                        let _ = answer(stream, &path, &route, mime);
                    });
            }
        })?;
    Ok(Served { url, address, stop })
}

/// A part of the address no one guesses: from the clock and the process,
/// through the standard library's random hasher keys.
fn token() -> String {
    use std::hash::{BuildHasher, Hasher};
    let mut hasher = std::collections::hash_map::RandomState::new().build_hasher();
    hasher.write_u128(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos(),
    );
    hasher.write_u32(std::process::id());
    let first = hasher.finish();
    hasher.write_u64(first);
    format!("{first:016x}{:016x}", hasher.finish())
}

/// Answers one connection's requests for the file.
fn answer(stream: TcpStream, path: &Path, route: &str, mime: &str) -> io::Result<()> {
    stream.set_read_timeout(Some(IDLE))?;
    let mut writer = stream.try_clone()?;
    let mut reader = BufReader::new(stream);
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            return Ok(());
        }
        let mut parts = line.split_whitespace();
        let method = parts.next().unwrap_or_default().to_owned();
        let target = parts.next().unwrap_or_default().to_owned();
        let mut range = None;
        let mut close = false;
        loop {
            let mut header = String::new();
            if reader.read_line(&mut header)? == 0 {
                return Ok(());
            }
            let header = header.trim_end();
            if header.is_empty() {
                break;
            }
            if let Some((name, value)) = header.split_once(':') {
                if name.eq_ignore_ascii_case("range") {
                    range = Some(value.trim().to_owned());
                } else if name.eq_ignore_ascii_case("connection") {
                    close = value.trim().eq_ignore_ascii_case("close");
                }
            }
        }
        let head = method == "HEAD";
        if (method != "GET" && !head) || target != route {
            writer.write_all(
                b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            )?;
            return Ok(());
        }
        let mut file = File::open(path)?;
        let size = file.metadata()?.len();
        let (status, start, end) = match range.as_deref().and_then(|range| bytes(range, size)) {
            Some((start, end)) => ("206 Partial Content", start, end),
            None if range.is_some() => {
                writer.write_all(
                    format!(
                        "HTTP/1.1 416 Range Not Satisfiable\r\nContent-Range: bytes */{size}\r\nContent-Length: 0\r\n\r\n"
                    )
                    .as_bytes(),
                )?;
                continue;
            }
            None => ("200 OK", 0, size.saturating_sub(1)),
        };
        let length = if size == 0 { 0 } else { end - start + 1 };
        let mut head_text = format!(
            "HTTP/1.1 {status}\r\nContent-Type: {mime}\r\nContent-Length: {length}\r\nAccept-Ranges: bytes\r\ntransferMode.dlna.org: Streaming\r\ncontentFeatures.dlna.org: DLNA.ORG_OP=01;DLNA.ORG_CI=0;DLNA.ORG_FLAGS=01700000000000000000000000000000\r\n"
        );
        if status.starts_with("206") {
            head_text.push_str(&format!("Content-Range: bytes {start}-{end}/{size}\r\n"));
        }
        head_text.push_str("\r\n");
        writer.write_all(head_text.as_bytes())?;
        if !head && length > 0 {
            file.seek(SeekFrom::Start(start))?;
            io::copy(&mut (&mut file).take(length), &mut writer)?;
        }
        writer.flush()?;
        if close {
            let _ = writer.shutdown(Shutdown::Both);
            return Ok(());
        }
    }
}

/// The first and last byte a Range header asks of a file of `size`
/// bytes: one range, from a start, to an end or the file's end, or its
/// last bytes.
fn bytes(range: &str, size: u64) -> Option<(u64, u64)> {
    let spec = range.strip_prefix("bytes=")?.split(',').next()?.trim();
    let (first, last) = spec.split_once('-')?;
    let last_byte = size.checked_sub(1)?;
    let (start, end) = match (first.trim(), last.trim()) {
        ("", suffix) => {
            let suffix: u64 = suffix.parse().ok()?;
            (size.saturating_sub(suffix), last_byte)
        }
        (first, "") => (first.parse().ok()?, last_byte),
        (first, last) => (
            first.parse().ok()?,
            last.parse::<u64>().ok()?.min(last_byte),
        ),
    };
    (start <= end && start < size).then_some((start, end))
}

/// The type a renderer is told the file is, by its ending.
pub(crate) fn mime(path: &Path) -> &'static str {
    let extension = path
        .extension()
        .map(|extension| extension.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    match extension.as_str() {
        "mp4" | "m4v" => "video/mp4",
        "mkv" => "video/x-matroska",
        "webm" => "video/webm",
        "mov" => "video/quicktime",
        "avi" => "video/x-msvideo",
        "ts" | "m2ts" | "mts" => "video/mp2t",
        "mpg" | "mpeg" => "video/mpeg",
        "wmv" => "video/x-ms-wmv",
        "mp3" => "audio/mpeg",
        "m4a" | "aac" => "audio/mp4",
        "flac" => "audio/flac",
        "wav" => "audio/wav",
        "ogg" | "oga" => "audio/ogg",
        "opus" => "audio/opus",
        "wma" => "audio/x-ms-wma",
        "aiff" | "aif" => "audio/aiff",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges_are_read_as_rfc_9110_has_them() {
        assert_eq!(bytes("bytes=0-99", 1000), Some((0, 99)));
        assert_eq!(bytes("bytes=500-", 1000), Some((500, 999)));
        assert_eq!(bytes("bytes=-100", 1000), Some((900, 999)));
        assert_eq!(bytes("bytes=900-5000", 1000), Some((900, 999)));
        assert_eq!(bytes("bytes=1000-", 1000), None);
        assert_eq!(bytes("bytes=5-1", 1000), None);
        assert_eq!(bytes("lines=1-2", 1000), None);
        assert_eq!(bytes("bytes=0-0", 0), None);
    }

    fn fetch(url: &str, range: Option<&str>) -> (String, Vec<u8>) {
        let url = crate::http::Url::parse(url).unwrap();
        let headers: Vec<(&str, &str)> = range.map(|range| ("Range", range)).into_iter().collect();
        let response = crate::http::request("GET", &url, &headers, &[]).unwrap();
        (response.status.to_string(), response.body)
    }

    #[test]
    fn the_file_is_served_whole_in_ranges_and_alone() {
        let folder = std::env::temp_dir().join(format!("plaa-cast-{}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        let path = folder.join("clip.mp4");
        let content: Vec<u8> = (0..=255u8).cycle().take(10_000).collect();
        std::fs::write(&path, &content).unwrap();
        let served = serve(path.clone(), mime(&path), "127.0.0.1".parse().unwrap()).unwrap();
        assert!(served.url().ends_with("/media.mp4"));
        let (status, body) = fetch(served.url(), None);
        assert_eq!((status.as_str(), body.len()), ("200", 10_000));
        assert_eq!(body, content);
        let (status, body) = fetch(served.url(), Some("bytes=9990-"));
        assert_eq!(status, "206");
        assert_eq!(body, &content[9990..]);
        let (status, _) = fetch(served.url(), Some("bytes=20000-"));
        assert_eq!(status, "416");
        // Another path on the same server finds nothing.
        let elsewhere = served.url().rsplit_once('/').unwrap().0.to_owned() + "/../secret";
        assert_eq!(fetch(&elsewhere, None).0, "404");
        drop(served);
        let _ = std::fs::remove_dir_all(&folder);
    }

    #[test]
    fn types_follow_the_ending() {
        assert_eq!(mime(Path::new("a/b.MKV")), "video/x-matroska");
        assert_eq!(mime(Path::new("song.flac")), "audio/flac");
        assert_eq!(mime(Path::new("noending")), "application/octet-stream");
    }
}
