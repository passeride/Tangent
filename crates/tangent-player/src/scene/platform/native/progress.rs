//! SQLite checkpoints shared with web schema v5. No scoring decisions here.
use rusqlite::{Connection, params};
use tangent_core::progress::Run;
pub const MIGRATION: &str = "CREATE TABLE practice_details (
 session_id TEXT PRIMARY KEY NOT NULL REFERENCES practice_sessions(id) ON DELETE CASCADE,
 revision INTEGER NOT NULL, payload TEXT NOT NULL, updated_at INTEGER NOT NULL);
 CREATE INDEX practice_recent ON practice_details(updated_at DESC); PRAGMA user_version=5;";
pub fn save(conn: &Connection, run: &Run) -> Result<(), String> {
    if !run.validate() {
        return Err("Invalid practice checkpoint".into());
    }
    let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
    let result = (|| -> rusqlite::Result<()> {
        use rusqlite::OptionalExtension;
        let old: Option<u32> = tx
            .query_row(
                "SELECT revision FROM practice_details WHERE session_id=?",
                [&run.id],
                |r| r.get(0),
            )
            .optional()?;
        if old.is_some_and(|v| v > run.revision) {
            return Ok(());
        }
        let at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;
        tx.execute("INSERT INTO practice_sessions VALUES(?,?,?,?) ON CONFLICT(id) DO UPDATE SET ended_at=excluded.ended_at",params![run.id,run.mode,at.to_string(),run.finished.then(||at.to_string())])?;
        tx.execute("INSERT INTO practice_details VALUES(?,?,?,?) ON CONFLICT(session_id) DO UPDATE SET revision=excluded.revision,payload=excluded.payload,updated_at=excluded.updated_at",params![run.id,run.revision,serde_json::to_string(run).unwrap(),at])?;
        tx.execute("DELETE FROM scores WHERE session_id=?", [&run.id])?;
        for (metric, value) in [
            ("points", run.points as u64),
            ("solved", run.solved() as u64),
            ("clean", run.clean() as u64),
            ("errors", run.errors as u64),
            ("elapsed_ms", run.elapsed_ms),
        ] {
            tx.execute(
                "INSERT INTO scores VALUES(?,?,?,?,?)",
                params![
                    format!("{}:{metric}", run.id),
                    run.id,
                    metric,
                    value as i64,
                    at.to_string()
                ],
            )?;
        }
        Ok(())
    })();
    result.map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())
}
pub fn read(conn: &Connection) -> Result<Vec<Run>, String> {
    let mut query=conn.prepare("SELECT payload FROM practice_details ORDER BY updated_at DESC,session_id DESC LIMIT 200").map_err(|e|e.to_string())?;
    let rows = query
        .query_map([], |r| r.get::<_, String>(0))
        .map_err(|e| e.to_string())?;
    rows.map(|r| {
        serde_json::from_str::<Run>(&r.map_err(|e| e.to_string())?).map_err(|e| e.to_string())
    })
    .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn checkpoints_are_idempotent_and_reject_older_revisions() {
        let db = Connection::open_in_memory().unwrap();
        super::super::migrate(&db).unwrap();
        let mut run = Run::new(
            "test-session".into(),
            &tangent_core::practice::Practice::default(),
        );
        run.revision = 2;
        run.errors = 1;
        save(&db, &run).unwrap();
        save(&db, &run).unwrap();
        run.revision = 1;
        run.errors = 0;
        save(&db, &run).unwrap();
        let rows = read(&db).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].errors, 1);
        assert_eq!(
            db.query_row("SELECT count(*) FROM scores", [], |r| r.get::<_, i64>(0))
                .unwrap(),
            5
        );
    }
    #[test]
    fn passage_context_and_evidence_survive_native_checkpoints() {
        use tangent_core::passage::{Passage, SongContext};
        let db = Connection::open_in_memory().unwrap();
        super::super::migrate(&db).unwrap();
        let mut run = Run::song("passage-native-test".into(), "Synthetic".into(), 1);
        run.context = Some(SongContext {
            passage: Passage {
                start_us: 100,
                end_us: 200,
            },
            duration_us: 300,
            speed_percent: 70,
            part: Some(0),
            evaluator: 1,
            song_audio: false,
        });
        run.piece_id = Some("a".repeat(64));
        run.input_sources = Some(vec!["midi".into()]);
        run.song_input(tangent_core::practice::Grade::Correct);
        run.matched_targets = Some(vec![(100, 60)]);
        run.close(true);
        save(&db, &run).unwrap();
        let restored = read(&db).unwrap().remove(0);
        assert_eq!(restored.context, run.context);
        assert_eq!(restored.matched_targets, run.matched_targets);
        assert!(restored.comparable_to(&run));
        run.context.as_mut().unwrap().speed_percent = 0;
        assert!(save(&db, &run).is_err());
        assert_eq!(
            read(&db).unwrap()[0]
                .context
                .as_ref()
                .unwrap()
                .speed_percent,
            70
        );
    }
}
