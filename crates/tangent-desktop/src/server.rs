//! Minimal loopback-only, read-only asset delivery. No paths are read from HTTP.
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::time::Duration;
pub type Assets = &'static [(&'static str, &'static [u8])];
fn mime(path: &str) -> &'static str {
    match path.rsplit('.').next().unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "wasm" => "application/wasm",
        "mid" => "audio/midi",
        "json" => "application/json",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        _ => "text/plain; charset=utf-8",
    }
}
pub fn response(request: &str, port: u16, assets: Assets) -> Vec<u8> {
    let mut lines = request.split("\r\n");
    let first = lines.next().unwrap_or("");
    let fields: Vec<_> = first.split_whitespace().collect();
    let host = lines
        .filter_map(|line| line.split_once(':'))
        .find(|(key, _)| key.eq_ignore_ascii_case("host"))
        .map(|(_, v)| v.trim());
    let expected = format!("127.0.0.1:{port}");
    let localhost = format!("localhost:{port}");
    let (status, body, content_type) =
        if host != Some(expected.as_str()) && host != Some(localhost.as_str()) {
            (
                "403 Forbidden",
                b"Invalid local host".as_slice(),
                "text/plain",
            )
        } else if fields.len() != 3 || !matches!(fields[0], "GET" | "HEAD") {
            (
                "405 Method Not Allowed",
                b"Read-only local player".as_slice(),
                "text/plain",
            )
        } else {
            let path = fields[1].split('?').next().unwrap_or("");
            let path = if path == "/" { "/index.html" } else { path };
            match assets.iter().find(|(name, _)| *name == path) {
                Some((name, data)) => ("200 OK", *data, mime(name)),
                None => ("404 Not Found", b"Not found".as_slice(), "text/plain"),
            }
        };
    let header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\nX-Content-Type-Options: nosniff\r\nCache-Control: no-store\r\n\r\n",
        body.len()
    );
    let mut bytes = header.into_bytes();
    if fields.first() != Some(&"HEAD") {
        bytes.extend(body);
    }
    bytes
}
pub fn serve(mut stream: TcpStream, port: u16, assets: Assets) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;
    let mut bytes = Vec::new();
    let mut buffer = [0; 1024];
    while bytes.len() < 8192 {
        let n = stream.read(&mut buffer)?;
        if n == 0 {
            return Ok(());
        }
        bytes.extend_from_slice(&buffer[..n]);
        if bytes.windows(4).any(|w| w == b"\r\n\r\n") {
            return stream.write_all(&response(&String::from_utf8_lossy(&bytes), port, assets));
        }
    }
    stream.write_all(b"HTTP/1.1 431 Request Header Fields Too Large\r\nConnection: close\r\nContent-Length: 0\r\n\r\n")
}
#[cfg(test)]
mod tests {
    use super::*;
    const FILES: Assets = &[("/index.html", b"hello"), ("/pkg/app.wasm", b"\0asm")];
    #[test]
    fn serves_only_exact_packaged_assets_with_wasm_mime() {
        let ok = String::from_utf8(response(
            "GET /pkg/app.wasm HTTP/1.1\r\nHost: 127.0.0.1:1234\r\n\r\n",
            1234,
            FILES,
        ))
        .unwrap();
        assert!(ok.contains("application/wasm"));
        let denied = String::from_utf8(response(
            "GET /../secret HTTP/1.1\r\nHost: localhost:1234\r\n\r\n",
            1234,
            FILES,
        ))
        .unwrap();
        assert!(denied.starts_with("HTTP/1.1 404"));
        let hostile = String::from_utf8(response(
            "GET / HTTP/1.1\r\nHost: attacker.example\r\n\r\n",
            1234,
            FILES,
        ))
        .unwrap();
        assert!(hostile.starts_with("HTTP/1.1 403"));
    }
    #[test]
    fn head_has_length_but_no_body_and_posts_are_refused() {
        let head = String::from_utf8(response(
            "HEAD / HTTP/1.1\r\nHost: localhost:1234\r\n\r\n",
            1234,
            FILES,
        ))
        .unwrap();
        assert!(head.contains("Content-Length: 5"));
        assert!(head.ends_with("\r\n\r\n"));
        let post = String::from_utf8(response(
            "POST / HTTP/1.1\r\nHost: localhost:1234\r\n\r\n",
            1234,
            FILES,
        ))
        .unwrap();
        assert!(post.starts_with("HTTP/1.1 405"));
    }
}
