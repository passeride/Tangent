//! Original source-byte library. SQL owns persistence; Rust import owns validity.
use crate::library::{Entry, ImportMetadata};
use rusqlite::{Connection, params};
use serde_json::{Value, json};
pub(super) const MIGRATION: &str = "
CREATE TABLE library_songs (
 id TEXT PRIMARY KEY NOT NULL, title TEXT NOT NULL, format TEXT NOT NULL,
 duration_us INTEGER NOT NULL, note_count INTEGER NOT NULL, source BLOB NOT NULL,
 imported_at TEXT NOT NULL, last_opened_at TEXT NOT NULL
);
CREATE INDEX library_recent ON library_songs(last_opened_at DESC, id);
PRAGMA user_version = 3;";

pub(super) fn save(conn: &Connection, bytes: &[u8], metadata: &str) -> Result<(), String> {
    let meta: ImportMetadata = serde_json::from_str(metadata).map_err(|e| e.to_string())?;
    if bytes.is_empty() || bytes.len() > 16 * 1024 * 1024 {
        return Err("Invalid library source size".into());
    }
    let id = blake3::hash(bytes).to_hex().to_string();
    conn.execute_batch("SAVEPOINT library_save")
        .map_err(|e| e.to_string())?;
    let result = (|| -> Result<(), String> {
        let (count, size): (i64, i64) = conn
            .query_row(
                "SELECT COUNT(*),COALESCE(SUM(length(source)),0) FROM library_songs WHERE id<>?1",
                [&id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .map_err(|e| e.to_string())?;
        if count >= 500 || size + bytes.len() as i64 > 128 * 1024 * 1024 {
            return Err("Library limit reached. Remove a piece before importing more.".into());
        }
        conn.execute("INSERT INTO library_songs VALUES(?1,?2,?3,?4,?5,?6,datetime('now'),datetime('now'))
            ON CONFLICT(id) DO UPDATE SET title=excluded.title,format=excluded.format,
            duration_us=excluded.duration_us,note_count=excluded.note_count,last_opened_at=excluded.last_opened_at",
            params![id, meta.title, meta.format, meta.duration_us as i64, meta.note_count as i64, bytes])
            .map_err(|e| e.to_string())?;
        Ok(())
    })();
    if result.is_err() {
        let _ = conn.execute_batch("ROLLBACK TO library_save");
    }
    conn.execute_batch("RELEASE library_save")
        .map_err(|e| e.to_string())?;
    result
}
pub(super) fn list(conn: &Connection, request: &Value) -> Result<Value, String> {
    let offset = request["offset"].as_u64().unwrap_or(0).min(i64::MAX as u64) as i64;
    let limit = request["limit"].as_u64().unwrap_or(6).clamp(1, 50) as i64;
    let total: i64 = conn
        .query_row("SELECT COUNT(*) FROM library_songs", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT id,title,format,duration_us,note_count FROM library_songs ORDER BY last_opened_at DESC,id LIMIT ?1 OFFSET ?2").map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([limit, offset], |row| {
            Ok(Entry {
                id: row.get(0)?,
                title: row.get(1)?,
                format: row.get(2)?,
                duration_us: row.get::<_, i64>(3)? as u64,
                note_count: row.get::<_, i64>(4)? as usize,
            })
        })
        .map_err(|e| e.to_string())?;
    let items = rows
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|e| e.to_string())?;
    Ok(json!({"type":"library-list","ticket":request["ticket"],"items":items,"total":total}))
}
pub(super) fn open(conn: &Connection, id: &str) -> Result<(Vec<u8>, String), String> {
    let entry = conn
        .query_row(
            "SELECT source,title FROM library_songs WHERE id=?1",
            [id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE library_songs SET last_opened_at=datetime('now') WHERE id=?1",
        [id],
    )
    .map_err(|e| e.to_string())?;
    Ok(entry)
}
pub(super) fn remove(conn: &Connection, id: &str) -> Result<(), String> {
    conn.execute("DELETE FROM library_songs WHERE id=?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_survives_reopen_and_can_be_imported_again() {
        let path = std::env::temp_dir().join(format!(
            "tangent-library-{}-{}.sqlite",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let conn = Connection::open(&path).unwrap();
        super::super::migrate(&conn).unwrap();
        let bytes = include_bytes!("../../../../../../web/piano/samples/first-light.musicxml");
        let song = crate::import::song(bytes, "Study".into()).unwrap();
        let meta = serde_json::to_string(&ImportMetadata::from_song(&song, bytes)).unwrap();
        save(&conn, bytes, &meta).unwrap();
        save(&conn, bytes, &meta).unwrap();
        drop(conn);
        let conn = Connection::open(&path).unwrap();
        super::super::migrate(&conn).unwrap();
        let page = list(&conn, &json!({"ticket":1})).unwrap();
        assert_eq!(page["total"], 1);
        let id = page["items"][0]["id"].as_str().unwrap();
        let (restored, title) = open(&conn, id).unwrap();
        assert_eq!(restored, bytes);
        assert_eq!(
            crate::import::song(&restored, title).unwrap().notes.len(),
            song.notes.len()
        );
        remove(&conn, id).unwrap();
        assert!(open(&conn, id).is_err());
        assert_eq!(list(&conn, &json!({})).unwrap()["total"], 0);
        drop(conn);
        std::fs::remove_file(path).unwrap();
    }
}
