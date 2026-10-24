//! Native launcher for the current browser adapter, not a native Bevy renderer.
mod server;
include!(concat!(env!("OUT_DIR"), "/assets.rs"));
use std::{
    net::TcpListener,
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
fn open_browser(url: &str) -> std::io::Result<()> {
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut c = Command::new("rundll32.exe");
        c.args(["url.dll,FileProtocolHandler", url]);
        c
    };
    #[cfg(target_os = "macos")]
    let mut command = {
        let mut c = Command::new("open");
        c.arg(url);
        c
    };
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let mut command = {
        let mut c = Command::new("xdg-open");
        c.arg(url);
        c
    };
    command.spawn().map(|_| ())
}
fn main() -> std::io::Result<()> {
    if ASSETS.is_empty() {
        return Err(std::io::Error::other(
            "No packaged player. Build with TANGENT_WEB_DIST pointing to the prepared web distribution.",
        ));
    }
    let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0))?;
    let port = listener.local_addr()?.port();
    let url = format!("http://127.0.0.1:{port}/");
    println!("Tangent: {url}\nKeep this window open while playing. Ctrl+C closes Tangent.");
    if !std::env::args().any(|a| a == "--no-open")
        && let Err(error) = open_browser(&url)
    {
        eprintln!("Open {url} in your browser ({error})");
    }
    let active = Arc::new(AtomicUsize::new(0));
    for stream in listener.incoming() {
        let stream = stream?;
        if active.load(Ordering::Relaxed) >= 16 {
            continue;
        }
        active.fetch_add(1, Ordering::Relaxed);
        let active = active.clone();
        std::thread::spawn(move || {
            let _ = server::serve(stream, port, ASSETS);
            active.fetch_sub(1, Ordering::Relaxed);
        });
    }
    Ok(())
}
