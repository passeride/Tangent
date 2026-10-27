//! Native persistence for reviewed annotations and bounded cover bytes.
use rusqlite::{Connection, params};
use serde_json::{Value, json};
use tangent_core::metadata::Candidate;
pub(super) const MIGRATION: &str = "CREATE TABLE library_enrichment (song_id TEXT PRIMARY KEY NOT NULL REFERENCES library_songs(id) ON DELETE CASCADE, payload TEXT NOT NULL, cover BLOB NOT NULL, search_key TEXT NOT NULL, updated_at TEXT NOT NULL); PRAGMA user_version=6;";
pub(super) fn save(conn: &Connection, v: &Value) -> Result<(), String> {
    let candidate: Candidate =
        serde_json::from_value(v["candidate"].clone()).map_err(|e| e.to_string())?;
    let bytes: Vec<u8> = serde_json::from_value(v["bytes"].clone()).map_err(|e| e.to_string())?;
    let id = v["id"].as_str().ok_or("Missing source")?;
    if !candidate.validate() {
        return Err("Invalid external metadata.".into());
    }
    if !bytes.is_empty() {
        crate::cover::decode(&bytes)?;
    }
    let size: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(length(cover)),0) FROM library_enrichment WHERE song_id<>?1",
            [id],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;
    if size + bytes.len() as i64 > 32 * 1024 * 1024 {
        return Err("Cover storage is full. Remove an old cover first.".into());
    }
    let changed=conn.execute("INSERT INTO library_enrichment(song_id,payload,cover,search_key,updated_at) SELECT ?1,?2,?3,?4,datetime('now') WHERE EXISTS(SELECT 1 FROM library_songs WHERE id=?1) ON CONFLICT(song_id) DO UPDATE SET payload=excluded.payload,cover=excluded.cover,search_key=excluded.search_key,updated_at=excluded.updated_at",params![id,serde_json::to_string(&candidate).unwrap(),bytes,format!("{}\n{}\n{}",candidate.title,candidate.album,candidate.artists.join("\n")).to_lowercase()]).map_err(|e|e.to_string())?;
    if changed == 0 {
        return Err("Piece is no longer in the library.".into());
    }
    Ok(())
}
pub(super) fn cover(conn: &Connection, id: &str) -> Result<Value, String> {
    let bytes: Vec<u8> = conn
        .query_row(
            "SELECT cover FROM library_enrichment WHERE song_id=?1",
            [id],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;
    Ok(json!({"type":"library-cover","id":id,"bytes":bytes}))
}
pub(super) fn clear(conn: &Connection, id: &str) -> Result<(), String> {
    conn.execute("DELETE FROM library_enrichment WHERE song_id=?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn candidate() -> Value {
        json!({"provider":"musicbrainz","recording_id":"11111111-1111-1111-1111-111111111111","release_id":"22222222-2222-2222-2222-222222222222","title":"Morning lantern","artists":["Example pianist"],"album":"Quiet keys","date":"2001","country":"NO","disambiguation":"","length_ms":180000,"release_group_id":null,"isrcs":[],"provider_score":95,"title_match":true,"creator_match":false})
    }
    #[test]
    fn annotations_do_not_rewrite_originals_and_cascade_on_remove() {
        let conn = Connection::open_in_memory().unwrap();
        super::super::migrate(&conn).unwrap();
        let bytes = include_bytes!("../../../../../../web/piano/samples/first-light.musicxml");
        let song = crate::import::song(bytes, "Original".into()).unwrap();
        super::super::library::save(
            &conn,
            bytes,
            &serde_json::to_string(&crate::library::ImportMetadata::from_song(&song, bytes))
                .unwrap(),
        )
        .unwrap();
        let id = blake3::hash(bytes).to_hex().to_string();
        let cover_bytes = include_bytes!("../../../../tests/fixtures/metadata-cover.png");
        let r = json!({"id":id,"candidate":candidate(),"bytes":cover_bytes.to_vec()});
        save(&conn, &r).unwrap();
        assert_eq!(
            cover(&conn, &id).unwrap()["bytes"]
                .as_array()
                .unwrap()
                .len(),
            cover_bytes.len()
        );
        let before = super::super::library::list(&conn, &json!({})).unwrap();
        assert_eq!(before["items"][0]["title"], song.title);
        assert_eq!(before["items"][0]["enrichment"]["album"], "Quiet keys");
        let (restored, _) = super::super::library::open(&conn, &id).unwrap();
        assert_eq!(restored, bytes);
        let mut bad = r.clone();
        bad["candidate"]["release_id"] = json!("bad");
        assert!(save(&conn, &bad).is_err());
        clear(&conn, &id).unwrap();
        assert!(cover(&conn, &id).is_err());
        save(&conn, &r).unwrap();
        super::super::library::remove(&conn, &id).unwrap();
        assert!(cover(&conn, &id).is_err());
        assert!(save(&conn, &r).is_err());
    }
}
