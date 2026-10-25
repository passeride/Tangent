// Portable SQLite schema. Keep musical/scoring rules out of the storage adapter.
export const schemaVersion = 1;
export function migrate(db) {
  const version = db.exec("PRAGMA user_version")[0].values[0][0];
  if (version > schemaVersion) throw Error("Database is from a newer Tangent version; preserved without changes.");
  db.run("PRAGMA foreign_keys = ON");
  if (version === 0) {
    db.run(`BEGIN;
      CREATE TABLE settings (
        id INTEGER PRIMARY KEY CHECK (id = 1),
        payload TEXT NOT NULL,
        updated_at TEXT NOT NULL
      );
      CREATE TABLE practice_sessions (
        id TEXT PRIMARY KEY NOT NULL,
        mode TEXT NOT NULL,
        started_at TEXT NOT NULL,
        ended_at TEXT
      );
      CREATE TABLE scores (
        id TEXT PRIMARY KEY NOT NULL,
        session_id TEXT NOT NULL REFERENCES practice_sessions(id) ON DELETE CASCADE,
        metric TEXT NOT NULL,
        value REAL NOT NULL,
        created_at TEXT NOT NULL
      );
      CREATE INDEX scores_by_session ON scores(session_id);
      PRAGMA user_version = 1;
      COMMIT;`);
  }
}
export function readSettings(db) {
  const result = db.exec("SELECT payload FROM settings WHERE id = 1");
  return result.length ? JSON.parse(result[0].values[0][0]) : null;
}
export function writeSettings(db, settings) {
  db.run(`INSERT INTO settings(id, payload, updated_at) VALUES (1, ?, ?)
    ON CONFLICT(id) DO UPDATE SET payload=excluded.payload, updated_at=excluded.updated_at`,
    [JSON.stringify(settings), new Date().toISOString()]);
}
