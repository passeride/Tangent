//! SQLite persistence for the shared symbolic fingerprint models.
use rusqlite::{Connection, params, params_from_iter};
use serde_json::Value;
use tangent_core::fingerprint::{Candidate, FingerprintedSong};

pub(super) fn index(conn: &Connection, payload: &str) -> Result<(), String> {
    let song: FingerprintedSong = serde_json::from_str(payload).map_err(|e| e.to_string())?;
    let id = blake3::hash(payload.as_bytes()).to_hex().to_string();
    conn.execute_batch("SAVEPOINT fingerprint_index")
        .map_err(|e| e.to_string())?;
    let result = (|| -> rusqlite::Result<()> {
        conn.execute("DELETE FROM fingerprint_songs WHERE id=?1", [&id])?;
        conn.execute(
            "INSERT INTO fingerprint_songs VALUES(?1,?2,datetime('now'))",
            params![id, song.title],
        )?;
        for melody in song.melodies {
            conn.execute(
                "INSERT INTO fingerprint_melodies VALUES(?1,?2,?3,?4)",
                params![
                    id,
                    melody.part as i64,
                    serde_json::to_string(&melody.keys).unwrap(),
                    serde_json::to_string(&melody.onsets_us).unwrap()
                ],
            )?;
            let mut insert = conn.prepare("INSERT INTO fingerprint_ngrams VALUES(?1,?2,?3,?4)")?;
            for fp in melody.fingerprints {
                insert.execute(params![
                    fp.signature,
                    id,
                    melody.part as i64,
                    fp.position as i64
                ])?;
            }
        }
        Ok(())
    })();
    if result.is_err() {
        let _ = conn.execute_batch("ROLLBACK TO fingerprint_index");
    }
    conn.execute_batch("RELEASE fingerprint_index")
        .map_err(|e| e.to_string())?;
    result.map_err(|e| e.to_string())
}

pub(super) fn candidates(conn: &Connection, signatures: &Value) -> Result<Vec<Candidate>, String> {
    let signatures: Vec<&str> = signatures
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .filter(|s| s.len() == 8 && s.bytes().all(|c| c.is_ascii_hexdigit()))
        .take(96)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    if signatures.is_empty() {
        return Ok(vec![]);
    }
    let placeholders = vec!["?"; signatures.len()].join(",");
    let sql = format!(
        "SELECT m.song_id,m.part,s.title,m.keys_json,m.onsets_json,COUNT(n.signature) AS hits
        FROM fingerprint_melodies m JOIN fingerprint_songs s ON s.id=m.song_id
        LEFT JOIN fingerprint_ngrams n ON n.song_id=m.song_id AND n.part=m.part
            AND n.signature IN ({placeholders})
        GROUP BY m.song_id,m.part ORDER BY hits DESC,m.song_id,m.part LIMIT 30"
    );
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map(params_from_iter(signatures), |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, i64>(5)?,
            ))
        })
        .map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for row in rows {
        let (song_id, part, title, keys, onsets, hits) = row.map_err(|e| e.to_string())?;
        result.push(Candidate {
            song_id,
            part: part as usize,
            title,
            keys: serde_json::from_str(&keys).map_err(|e| e.to_string())?,
            onsets_us: serde_json::from_str(&onsets).map_err(|e| e.to_string())?,
            hits: hits as usize,
        });
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_catalog_deduplicates_and_keeps_fuzzy_fallback() {
        let conn = Connection::open_in_memory().unwrap();
        super::super::migrate(&conn).unwrap();
        let keys = vec![60, 62, 64, 65, 67, 69];
        let song = FingerprintedSong {
            title: "Native fixture".into(),
            melodies: vec![tangent_core::fingerprint::MelodyIndex {
                part: 0,
                onsets_us: vec![0, 1, 2, 3, 4, 5],
                fingerprints: tangent_core::fingerprint::fingerprints(&keys),
                keys,
            }],
        };
        let payload = serde_json::to_string(&song).unwrap();
        index(&conn, &payload).unwrap();
        index(&conn, &payload).unwrap();
        let count: i64 = conn
            .query_row("SELECT count(*) FROM fingerprint_songs", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1);
        let found = candidates(&conn, &serde_json::json!(["00000000"])).unwrap();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].hits, 0);
        let found = candidates(
            &conn,
            &serde_json::json!(tangent_core::fingerprint::signatures(&[
                48, 50, 52, 53, 55, 57
            ])),
        )
        .unwrap();
        assert!(found[0].hits > 0);
    }
}
