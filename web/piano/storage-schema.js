// Portable SQLite schema. Musical matching stays in tangent-core (Rust).
export const schemaVersion = 2;

export function migrate(db) {
  let version = db.exec("PRAGMA user_version")[0].values[0][0];
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
    version = 1;
  }
  if (version === 1) {
    db.run(`BEGIN;
      CREATE TABLE fingerprint_songs (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        indexed_at TEXT NOT NULL
      );
      CREATE TABLE fingerprint_melodies (
        song_id TEXT NOT NULL REFERENCES fingerprint_songs(id) ON DELETE CASCADE,
        part INTEGER NOT NULL,
        keys_json TEXT NOT NULL,
        onsets_json TEXT NOT NULL,
        PRIMARY KEY(song_id, part)
      );
      CREATE TABLE fingerprint_ngrams (
        signature TEXT NOT NULL,
        song_id TEXT NOT NULL,
        part INTEGER NOT NULL,
        position INTEGER NOT NULL,
        FOREIGN KEY(song_id, part) REFERENCES fingerprint_melodies(song_id, part) ON DELETE CASCADE,
        PRIMARY KEY(signature, song_id, part, position)
      );
      CREATE INDEX fingerprint_signature_lookup ON fingerprint_ngrams(signature);
      PRAGMA user_version = 2;
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

/** Persist only the recognition representation, never full imported score files. */
export function writeFingerprintedSong(db, song) {
  if (typeof song.id !== "string" || !/^[0-9a-f]{64}$/.test(song.id)) throw Error("Invalid fingerprint song ID");
  if (typeof song.title !== "string" || !Array.isArray(song.melodies)) throw Error("Invalid fingerprint song");
  db.run("DELETE FROM fingerprint_songs WHERE id = ?", [song.id]);
  db.run("INSERT INTO fingerprint_songs(id,title,indexed_at) VALUES(?,?,?)",
    [song.id, song.title, new Date().toISOString()]);
  for (const melody of song.melodies) {
    if (!Number.isInteger(melody.part) || melody.part < 0 ||
        !Array.isArray(melody.keys) || !Array.isArray(melody.onsets_us) ||
        melody.keys.length !== melody.onsets_us.length || !Array.isArray(melody.fingerprints)) {
      throw Error("Invalid fingerprint lane");
    }
    db.run("INSERT INTO fingerprint_melodies(song_id,part,keys_json,onsets_json) VALUES(?,?,?,?)",
      [song.id, melody.part, JSON.stringify(melody.keys), JSON.stringify(melody.onsets_us)]);
    const insert = db.prepare("INSERT INTO fingerprint_ngrams(signature,song_id,part,position) VALUES(?,?,?,?)");
    try {
      for (const fp of melody.fingerprints) {
        if (typeof fp.signature !== "string" || !/^[0-9a-f]{8}$/.test(fp.signature) ||
            !Number.isInteger(fp.position) || fp.position < 0 ||
            fp.position + 5 > melody.keys.length) throw Error("Invalid melodic fingerprint");
        insert.run([fp.signature, song.id, melody.part, fp.position]);
      }
    } finally {
      insert.free();
    }
  }
}

/** Inverted index provides a shortlist. No match ranking is implemented in JS. */
export function findFingerprintCandidates(db, signatures, limit = 30) {
  if (!Array.isArray(signatures)) throw Error("Invalid fingerprint query");
  const unique = [...new Set(signatures.filter(s => typeof s === "string" && /^[0-9a-f]{8}$/.test(s)))].slice(0, 96);
  if (!unique.length) return [];
  const max = Math.max(1, Math.min(100, Math.floor(limit) || 30));
  const sql = `SELECT n.song_id, n.part, s.title, m.keys_json, m.onsets_json, COUNT(*) AS hits
    FROM fingerprint_ngrams n
    JOIN fingerprint_songs s ON s.id = n.song_id
    JOIN fingerprint_melodies m ON m.song_id = n.song_id AND m.part = n.part
    WHERE n.signature IN (${unique.map(() => "?").join(",")})
    GROUP BY n.song_id, n.part
    ORDER BY hits DESC, n.song_id, n.part
    LIMIT ${max}`;
  const rows = [];
  const stmt = db.prepare(sql);
  try {
    stmt.bind(unique);
    while (stmt.step()) rows.push(stmt.getAsObject());
  } finally {
    stmt.free();
  }
  // Fill spare slots even when an unrelated lane has an exact n-gram hit.
  // Otherwise that incidental hit prevents fuzzy matching the correct melody.
  // Small catalogs are fully covered; larger catalogs still need an approximate index.
  if (rows.length < max) {
    const fallback = db.exec(`SELECT m.song_id, m.part, s.title, m.keys_json, m.onsets_json, 0 AS hits
      FROM fingerprint_melodies m JOIN fingerprint_songs s ON s.id = m.song_id
      ORDER BY m.song_id, m.part LIMIT ${max}`);
    if (fallback.length) {
      const cols = fallback[0].columns;
      for (const values of fallback[0].values) {
        const row = Object.fromEntries(cols.map((k, i) => [k, values[i]]));
        if (rows.length >= max) break;
        if (!rows.some(existing => existing.song_id === row.song_id && existing.part === row.part)) rows.push(row);
      }
    }
  }
  return rows.map(row => ({
    song_id: row.song_id, part: row.part, title: row.title,
    keys: JSON.parse(row.keys_json), onsets_us: JSON.parse(row.onsets_json),
    hits: row.hits,
  }));
}
