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

pub(super) fn migrate_v4(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "ALTER TABLE library_songs ADD COLUMN composer TEXT NOT NULL DEFAULT '';
        ALTER TABLE library_songs ADD COLUMN search_key TEXT NOT NULL DEFAULT '';
        ALTER TABLE library_songs ADD COLUMN fingerprint_id TEXT;
        CREATE INDEX library_fingerprint ON library_songs(fingerprint_id);",
    )?;
    let entries = {
        let mut stmt = conn.prepare("SELECT id,title FROM library_songs")?;
        stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?
    };
    for (id, title) in entries {
        conn.execute(
            "UPDATE library_songs SET search_key=?1 WHERE id=?2",
            params![format!("{title}\n").to_lowercase(), id],
        )?;
    }
    conn.execute_batch("PRAGMA user_version=4")
}

pub(super) fn save(conn: &Connection, bytes: &[u8], metadata: &str) -> Result<(), String> {
    let meta: ImportMetadata = serde_json::from_str(metadata).map_err(|e| e.to_string())?;
    if bytes.is_empty() || bytes.len() > 16 * 1024 * 1024 {
        return Err("Invalid library source size".into());
    }
    let id = blake3::hash(bytes).to_hex().to_string();
    let fingerprint_payload = meta
        .fingerprint
        .as_ref()
        .map(|fp| serde_json::to_string(fp).unwrap());
    let fingerprint_id = fingerprint_payload
        .as_ref()
        .map(|p| blake3::hash(p.as_bytes()).to_hex().to_string());
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
        if let Some(payload) = &fingerprint_payload {
            super::fingerprints::index(conn, payload)?;
        }
        conn.execute("INSERT INTO library_songs(id,title,format,duration_us,note_count,source,imported_at,last_opened_at,search_key,fingerprint_id,composer)
            VALUES(?1,?2,?3,?4,?5,?6,datetime('now'),datetime('now'),?7,?8,?9)
            ON CONFLICT(id) DO UPDATE SET format=excluded.format,fingerprint_id=COALESCE(excluded.fingerprint_id,library_songs.fingerprint_id),
            duration_us=excluded.duration_us,note_count=excluded.note_count,last_opened_at=excluded.last_opened_at",
            params![id, meta.title, meta.format, meta.duration_us as i64, meta.note_count as i64, bytes, format!("{}\n{}",meta.title,meta.composer).to_lowercase(),fingerprint_id,meta.composer])
            .map_err(|e| e.to_string())?;
        conn.execute("UPDATE fingerprint_songs SET title=(SELECT title FROM library_songs WHERE id=?1) WHERE id=?2",params![id,fingerprint_id])
            .map_err(|e|e.to_string())?;
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
    let query = request["query"]
        .as_str()
        .unwrap_or("")
        .trim()
        .to_lowercase()
        .chars()
        .take(256)
        .collect::<String>();
    let format = match request["format"].as_str().unwrap_or("") {
        "MIDI" => "MIDI",
        "MusicXML" => "MusicXML",
        "MXL" => "MXL",
        _ => "",
    };
    let order = match request["sort"].as_str().unwrap_or("recent") {
        "title" => "title COLLATE NOCASE,id",
        "composer" => "composer COLLATE NOCASE,title COLLATE NOCASE,id",
        "duration" => "duration_us,title COLLATE NOCASE,id",
        _ => "last_opened_at DESC,id",
    };
    let total:i64=conn.query_row("SELECT COUNT(*) FROM library_songs WHERE instr(search_key,?1)>0 AND (?2='' OR format=?2)",
        params![query,format],|row|row.get(0)).map_err(|e|e.to_string())?;
    let mut stmt = conn
        .prepare(&format!(
            "SELECT id,title,composer,format,duration_us,note_count FROM library_songs
        WHERE instr(search_key,?1)>0 AND (?2='' OR format=?2) ORDER BY {order} LIMIT ?3 OFFSET ?4"
        ))
        .map_err(|e| e.to_string())?;
    let items = stmt
        .query_map(params![query, format, limit, offset], |row| {
            Ok(Entry {
                id: row.get(0)?,
                title: row.get(1)?,
                composer: row.get(2)?,
                format: row.get(3)?,
                duration_us: row.get::<_, i64>(4)? as u64,
                note_count: row.get::<_, i64>(5)? as usize,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|e| e.to_string())?;
    Ok(json!({"type":"library-list","ticket":request["ticket"],"items":items,"total":total}))
}
pub(super) fn edit(conn: &Connection, payload: &str) -> Result<(), String> {
    let edit: crate::library::MetadataEdit =
        serde_json::from_str(payload).map_err(|e| e.to_string())?;
    if !edit.validate() {
        return Err("Enter a title; fields may contain up to 160 characters.".into());
    }
    let title = edit.title.trim();
    let composer = edit.composer.trim();
    conn.execute_batch("SAVEPOINT library_edit")
        .map_err(|e| e.to_string())?;
    let result = (|| -> rusqlite::Result<()> {
        conn.execute(
            "UPDATE library_songs SET title=?1,composer=?2,search_key=?3 WHERE id=?4",
            params![
                title,
                composer,
                format!("{title}\n{composer}").to_lowercase(),
                edit.id
            ],
        )?;
        conn.execute("UPDATE fingerprint_songs SET title=?1 WHERE id=(SELECT fingerprint_id FROM library_songs WHERE id=?2)",params![title,edit.id])?;
        Ok(())
    })();
    if result.is_err() {
        let _ = conn.execute_batch("ROLLBACK TO library_edit");
    }
    conn.execute_batch("RELEASE library_edit")
        .map_err(|e| e.to_string())?;
    result.map_err(|e| e.to_string())
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
    conn.execute_batch("SAVEPOINT library_remove")
        .map_err(|e| e.to_string())?;
    let result = (|| -> rusqlite::Result<()> {
        conn.execute("DELETE FROM fingerprint_songs WHERE id=(SELECT fingerprint_id FROM library_songs WHERE id=?1)
            AND NOT EXISTS (SELECT 1 FROM library_songs other WHERE other.id<>?1 AND other.fingerprint_id=fingerprint_songs.id)",[id])?;
        conn.execute("DELETE FROM library_songs WHERE id=?1", [id])?;
        Ok(())
    })();
    if result.is_err() {
        let _ = conn.execute_batch("ROLLBACK TO library_remove");
    }
    conn.execute_batch("RELEASE library_remove")
        .map_err(|e| e.to_string())?;
    result.map_err(|e| e.to_string())
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
        edit(
            &conn,
            &json!({"id":id,"title":"Øvelse 100%_","composer":"Bach"}).to_string(),
        )
        .unwrap();
        let found = list(&conn, &json!({"query":"ØVELSE"})).unwrap();
        assert_eq!(found["total"], 1);
        assert_eq!(found["items"][0]["composer"], "Bach");
        assert_eq!(list(&conn, &json!({"query":"%_"})).unwrap()["total"], 1);
        assert_eq!(
            list(&conn, &json!({"query":"Bach","format":"MIDI"})).unwrap()["total"],
            0
        );
        assert_eq!(
            list(&conn, &json!({"query":"' OR 1=1 --"})).unwrap()["total"],
            0
        );
        save(&conn, bytes, &meta).unwrap();
        assert_eq!(
            list(&conn, &json!({"query":"Bach"})).unwrap()["items"][0]["title"],
            "Øvelse 100%_"
        );
        let (restored, title) = open(&conn, id).unwrap();
        assert_eq!(title, "Øvelse 100%_");
        assert_eq!(restored, bytes);
        assert_eq!(
            crate::import::song(&restored, title).unwrap().notes.len(),
            song.notes.len()
        );
        remove(&conn, id).unwrap();
        assert!(open(&conn, id).is_err());
        let remaining: i64 = conn
            .query_row("SELECT count(*) FROM fingerprint_songs", [], |r| r.get(0))
            .unwrap();
        assert_eq!(remaining, 0);
        assert_eq!(list(&conn, &json!({})).unwrap()["total"], 0);
        drop(conn);
        std::fs::remove_file(path).unwrap();
    }
}

#[cfg(test)]
mod migration_tests {
    use super::*;
    #[test]
    fn v3_source_metadata_survives_search_migration() {
        let conn = Connection::open_in_memory().unwrap();
        super::super::migrate(&conn).unwrap();
        conn.execute_batch("DROP TABLE practice_details; DROP TABLE library_songs;")
            .unwrap();
        conn.execute_batch(MIGRATION).unwrap();
        conn.execute(
            "INSERT INTO library_songs VALUES('legacy','ØVING','MIDI',1,1,?1,'now','now')",
            [b"source".as_slice()],
        )
        .unwrap();
        super::super::migrate(&conn).unwrap();
        assert_eq!(list(&conn, &json!({"query":"øving"})).unwrap()["total"], 1);
        assert_eq!(open(&conn, "legacy").unwrap().0, b"source");
    }
}
