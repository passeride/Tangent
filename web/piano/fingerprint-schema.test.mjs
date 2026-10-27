import assert from "node:assert/strict";
import initSqlJs from "sql.js";
import {
  schemaVersion, migrate, readSettings, writeSettings,
  writeFingerprintedSong, findFingerprintCandidates,
} from "./storage-schema.js";

const SQL = await initSqlJs();
const db = new SQL.Database();
migrate(db);
assert.equal(schemaVersion, 7);
assert.equal(db.exec("PRAGMA user_version")[0].values[0][0], 7);
writeSettings(db, { version: 1, theme: 1 });
db.run("INSERT INTO practice_sessions VALUES ('s','notes','2026-10-25T12:00:00Z',NULL)");
db.run("INSERT INTO scores VALUES ('score','s','accuracy',0.75,'2026-10-25T12:01:00Z')");

const keys = [76, 75, 76, 75, 76, 71, 74, 72, 69];
const signatures = [];
for (let i = 0; i + 4 < keys.length; i++) {
  let signature = "";
  for (let j = i; j < i + 4; j++)
    signature += (keys[j + 1] - keys[j] + 127).toString(16).padStart(2, "0");
  signatures.push({ signature, position: i });
}
const entry = {
  id: "a".repeat(64), title: "Für Elise",
  melodies: [{ part: 0, keys, onsets_us: keys.map((_, i) => i * 200000),
    fingerprints: signatures }],
};
writeFingerprintedSong(db, entry);
const candidate = findFingerprintCandidates(db, signatures.map(s => s.signature));
assert.equal(candidate.length, 1);
assert.equal(candidate[0].title, "Für Elise");
assert.deepEqual(candidate[0].keys, keys);
assert.equal(candidate[0].hits, signatures.length);

// A wrong note can erase the target's exact windows while a decoy has one hit.
// Both lanes must reach Rust's fuzzy reranker in a small catalog.
writeFingerprintedSong(db, { ...entry, id: "b".repeat(64), title: "Decoy" });
const decoySignature = "80818283";
db.run("UPDATE fingerprint_ngrams SET signature=? WHERE song_id=?", [decoySignature, "b".repeat(64)]);
const mixed = findFingerprintCandidates(db, [decoySignature]);
assert.equal(mixed.length, 2);
assert.equal(mixed[0].title, "Decoy");
assert.equal(mixed.find(row => row.title === "Für Elise").hits, 0);
assert.equal(findFingerprintCandidates(db, [decoySignature], 1).length, 1);
db.run("DELETE FROM fingerprint_songs WHERE id=?", ["b".repeat(64)]);

// Repeated imports replace the same content without orphaning n-grams.
writeFingerprintedSong(db, entry);
assert.equal(db.exec("SELECT count(*) FROM fingerprint_ngrams")[0].values[0][0], signatures.length);
assert.equal(findFingerprintCandidates(db, ["ffffffff"])[0].hits, 0); // bounded fuzzy fallback
assert.deepEqual(findFingerprintCandidates(db, []), []);
assert.throws(() => writeFingerprintedSong(db, { ...entry, id: "not-a-hash" }), /Invalid fingerprint song ID/);
const bytes = db.export();
db.close();
const reopened = new SQL.Database(bytes);
migrate(reopened);
assert.equal(reopened.exec("PRAGMA integrity_check")[0].values[0][0], "ok");
assert.deepEqual(readSettings(reopened), { version: 1, theme: 1 });
assert.equal(reopened.exec("SELECT value FROM scores")[0].values[0][0], 0.75);
assert.equal(findFingerprintCandidates(reopened, signatures.map(s => s.signature)).length, 1);
reopened.close();

// A real schema-v1 image migrates forward without losing scores/settings.
const legacy = new SQL.Database();
legacy.run(`CREATE TABLE settings (id INTEGER PRIMARY KEY CHECK(id=1), payload TEXT NOT NULL, updated_at TEXT NOT NULL);
CREATE TABLE practice_sessions (id TEXT PRIMARY KEY NOT NULL, mode TEXT NOT NULL, started_at TEXT NOT NULL, ended_at TEXT);
CREATE TABLE scores (id TEXT PRIMARY KEY NOT NULL, session_id TEXT NOT NULL REFERENCES practice_sessions(id) ON DELETE CASCADE, metric TEXT NOT NULL, value REAL NOT NULL, created_at TEXT NOT NULL);
CREATE INDEX scores_by_session ON scores(session_id);
PRAGMA user_version = 1;`);
migrate(legacy);
migrate(legacy);
assert.equal(legacy.exec("PRAGMA user_version")[0].values[0][0], 7);
writeSettings(legacy, { version: 1, theme: 2 });
assert.deepEqual(readSettings(legacy), { version: 1, theme: 2 });
legacy.close();
console.log("Fingerprint SQLite passed: v1 migration, indexed lookup, deduplication, fallback and export.");
