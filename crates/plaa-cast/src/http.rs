//! Just enough HTTP/1.1 for a renderer on the local network: plain
//! `http://` addresses, a request at a time, each on its own connection.

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

/// The longest a renderer may take to answer.
pub(crate) const TIMEOUT: Duration = Duration::from_secs(5);

/// The most of an answer read, as descriptions and SOAP answers are small.
const MOST: usize = 4 << 20;

/// An `http://` address, taken apart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Url {
    pub host: String,
    pub port: u16,
    /// The path and query, starting with `/`.
    pub path: String,
}

impl Url {
    pub(crate) fn parse(text: &str) -> Option<Url> {
        let rest = text.trim().strip_prefix("http://")?;
        let (authority, path) = match rest.find('/') {
            Some(at) => (&rest[..at], &rest[at..]),
            None => (rest, "/"),
        };
        // An IPv6 address is in brackets, its port after them.
        let (host, port) = match authority.strip_prefix('[') {
            Some(bracketed) => {
                let (host, after) = bracketed.split_once(']')?;
                match after.strip_prefix(':') {
                    Some(port) => (host, port.parse().ok()?),
                    None => (host, 80),
                }
            }
            None => match authority.split_once(':') {
                Some((host, port)) => (host, port.parse().ok()?),
                None => (authority, 80),
            },
        };
        if host.is_empty() {
            return None;
        }
        Some(Url {
            host: host.to_owned(),
            port,
            path: path.to_owned(),
        })
    }

    /// `reference` resolved against this address, as a description's
    /// relative control URLs are.
    pub(crate) fn join(&self, reference: &str) -> Option<Url> {
        let reference = reference.trim();
        if reference.starts_with("http://") {
            return Url::parse(reference);
        }
        let path = if reference.starts_with('/') {
            reference.to_owned()
        } else {
            let folder = self.path.rsplit_once('/').map_or("", |(folder, _)| folder);
            format!("{folder}/{reference}")
        };
        Some(Url {
            path,
            ..self.clone()
        })
    }

    fn address(&self) -> io::Result<SocketAddr> {
        (self.host.as_str(), self.port)
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| io::Error::other(format!("no address for {}", self.host)))
    }

    fn host_header(&self) -> String {
        if self.host.contains(':') {
            format!("[{}]:{}", self.host, self.port)
        } else {
            format!("{}:{}", self.host, self.port)
        }
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "http://{}{}", self.host_header(), self.path)
    }
}

/// What came back: the status code and the body.
#[derive(Debug)]
pub(crate) struct Response {
    pub status: u16,
    pub body: Vec<u8>,
}

/// Sends `method` to `url` with `headers` and `body`, and reads the
/// answer.
pub(crate) fn request(
    method: &str,
    url: &Url,
    headers: &[(&str, &str)],
    body: &[u8],
) -> io::Result<Response> {
    let mut stream = TcpStream::connect_timeout(&url.address()?, TIMEOUT)?;
    stream.set_read_timeout(Some(TIMEOUT))?;
    stream.set_write_timeout(Some(TIMEOUT))?;
    let mut head = format!(
        "{method} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nUser-Agent: plaa/{}\r\nContent-Length: {}\r\n",
        url.path,
        url.host_header(),
        env!("CARGO_PKG_VERSION"),
        body.len()
    );
    for (name, value) in headers {
        head.push_str(&format!("{name}: {value}\r\n"));
    }
    head.push_str("\r\n");
    stream.write_all(head.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()?;
    read_response(BufReader::new(stream))
}

pub(crate) fn get(url: &Url) -> io::Result<Response> {
    request("GET", url, &[], &[])
}

fn read_response(mut reader: impl BufRead) -> io::Result<Response> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let status = line
        .split_whitespace()
        .nth(1)
        .and_then(|code| code.parse().ok())
        .ok_or_else(|| io::Error::other(format!("not an HTTP answer: {}", line.trim())))?;
    let mut length = None;
    let mut chunked = false;
    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        let header = line.trim_end();
        if header.is_empty() {
            break;
        }
        if let Some((name, value)) = header.split_once(':') {
            let value = value.trim();
            if name.eq_ignore_ascii_case("content-length") {
                length = value.parse::<usize>().ok();
            } else if name.eq_ignore_ascii_case("transfer-encoding") {
                chunked = value.to_ascii_lowercase().contains("chunked");
            }
        }
    }
    let body = if chunked {
        read_chunks(&mut reader)?
    } else if let Some(length) = length {
        let mut body = vec![0; length.min(MOST)];
        reader.read_exact(&mut body)?;
        body
    } else {
        let mut body = Vec::new();
        reader.take(MOST as u64).read_to_end(&mut body)?;
        body
    };
    Ok(Response { status, body })
}

fn read_chunks(reader: &mut impl BufRead) -> io::Result<Vec<u8>> {
    let mut body = Vec::new();
    let mut line = String::new();
    loop {
        line.clear();
        reader.read_line(&mut line)?;
        let size = line.trim().split(';').next().unwrap_or_default();
        let size = usize::from_str_radix(size, 16)
            .map_err(|_| io::Error::other("a chunk without its size"))?;
        if size == 0 || body.len() + size > MOST {
            return Ok(body);
        }
        let start = body.len();
        body.resize(start + size, 0);
        reader.read_exact(&mut body[start..])?;
        line.clear();
        reader.read_line(&mut line)?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addresses_are_taken_apart() {
        let url = Url::parse("http://192.168.1.20:49152/description.xml").unwrap();
        assert_eq!(url.host, "192.168.1.20");
        assert_eq!(url.port, 49152);
        assert_eq!(url.path, "/description.xml");
        assert_eq!(Url::parse("http://tv.local").unwrap().port, 80);
        assert_eq!(Url::parse("http://tv.local").unwrap().path, "/");
        assert!(Url::parse("https://tv.local/").is_none());
        assert!(Url::parse("http:///x").is_none());
        let six = Url::parse("http://[fe80::1]:8080/a").unwrap();
        assert_eq!((six.host.as_str(), six.port), ("fe80::1", 8080));
        assert_eq!(six.to_string(), "http://[fe80::1]:8080/a");
    }

    #[test]
    fn control_addresses_resolve_against_the_description() {
        let base = Url::parse("http://10.0.0.5:1400/xml/device_description.xml").unwrap();
        assert_eq!(
            base.join("/MediaRenderer/AVTransport/Control")
                .unwrap()
                .to_string(),
            "http://10.0.0.5:1400/MediaRenderer/AVTransport/Control"
        );
        assert_eq!(
            base.join("AVTransport/control").unwrap().to_string(),
            "http://10.0.0.5:1400/xml/AVTransport/control"
        );
        assert_eq!(
            base.join("http://10.0.0.6/ctl").unwrap().to_string(),
            "http://10.0.0.6:80/ctl"
        );
    }

    #[test]
    fn answers_are_read_whole_and_in_chunks() {
        let plain = b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nhello";
        let response = read_response(&plain[..]).unwrap();
        assert_eq!(
            (response.status, response.body.as_slice()),
            (200, &b"hello"[..])
        );
        let chunked =
            b"HTTP/1.1 500 Error\r\nTransfer-Encoding: chunked\r\n\r\n3\r\nabc\r\n2;x\r\nde\r\n0\r\n\r\n";
        let response = read_response(&chunked[..]).unwrap();
        assert_eq!(
            (response.status, response.body.as_slice()),
            (500, &b"abcde"[..])
        );
        let closed = b"HTTP/1.0 200 OK\r\n\r\nto the end";
        assert_eq!(read_response(&closed[..]).unwrap().body, b"to the end");
        assert!(read_response(&b"SSH-2.0\r\n"[..]).is_err());
    }
}
