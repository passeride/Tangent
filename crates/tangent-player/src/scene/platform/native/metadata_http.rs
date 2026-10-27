//! Dedicated bounded HTTP worker; never block the audio/device/storage loop.
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    io::Read,
    sync::mpsc::{Sender, SyncSender, sync_channel},
    time::{Duration, Instant},
};
pub(super) fn start(out: Sender<Value>) -> SyncSender<Value> {
    let (tx, rx) = sync_channel::<Value>(8);
    std::thread::spawn(move || {
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .https_only(true)
            .timeout_global(Some(Duration::from_secs(20)))
            .build()
            .into();
        let mut next = Instant::now();
        let mut cache: VecDeque<(String, Instant, Vec<u8>)> = VecDeque::new();
        for r in rx {
            let url = r["url"].as_str().unwrap_or("");
            let cover = r["kind"] == "cover";
            let result = (|| -> Result<Value, String> {
                if !tangent_core::metadata::allowed_url(url, cover)
                    || !(r["kind"] == "cover" || r["kind"] == "search")
                {
                    return Err("Unsupported metadata request.".into());
                }
                if let Some((_, _, v)) = cache
                    .iter()
                    .find(|(u, t, _)| u == url && t.elapsed() < Duration::from_secs(600))
                {
                    return payload(v, cover);
                }
                if !cover {
                    std::thread::sleep(next.saturating_duration_since(Instant::now()));
                    next = Instant::now() + Duration::from_millis(1100);
                }
                let mut response = agent
                    .get(url)
                    .header(
                        "User-Agent",
                        "Tangent/0.1.0 (https://github.com/passeride/Tangent)",
                    )
                    .header(
                        "Accept",
                        if cover {
                            "image/jpeg,image/png"
                        } else {
                            "application/json"
                        },
                    )
                    .call()
                    .map_err(|e| format!("Provider unavailable: {e}"))?;
                let mut bytes = vec![];
                response
                    .body_mut()
                    .as_reader()
                    .take((crate::cover::MAX_COVER_BYTES + 1) as u64)
                    .read_to_end(&mut bytes)
                    .map_err(|e| e.to_string())?;
                if bytes.len() > crate::cover::MAX_COVER_BYTES {
                    return Err("Provider response is too large.".into());
                }
                if cache.len() >= 32 {
                    cache.pop_front();
                }
                let value = payload(&bytes, cover)?;
                cache.push_back((url.into(), Instant::now(), bytes));
                Ok(value)
            })();
            let mut reply = match result {
                Ok(v) => v,
                Err(e) => json!({"error":e}),
            };
            reply["type"] = json!("metadata-response");
            reply["ticket"] = r["ticket"].clone();
            reply["kind"] = r["kind"].clone();
            let _ = out.send(reply);
        }
    });
    tx
}

fn payload(bytes: &[u8], cover: bool) -> Result<Value, String> {
    if cover {
        Ok(json!({"bytes":bytes}))
    } else {
        let body = std::str::from_utf8(bytes).map_err(|_| "Provider response is not UTF-8")?;
        serde_json::from_str::<Value>(body)
            .map_err(|_| "Provider returned unreadable data. Try again later.")?;
        Ok(json!({"body":body}))
    }
}
