// Store source bytes, not renderer state. Import validation belongs to Rust.
export const libraryLimits = { fileBytes: 16 * 1024 * 1024, totalBytes: 128 * 1024 * 1024, songs: 500 };
export const libraryMigration = `
  CREATE TABLE library_songs (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    format TEXT NOT NULL,
    duration_us INTEGER NOT NULL,
    note_count INTEGER NOT NULL,
    source BLOB NOT NULL,
    imported_at TEXT NOT NULL,
    last_opened_at TEXT NOT NULL
  );
  CREATE INDEX library_recent ON library_songs(last_opened_at DESC, id);
  PRAGMA user_version = 3;`;

export function saveLibrarySong(db, id, source, metadata) {
  if (typeof id !== "string" || !/^[a-f0-9]{64}$/.test(id) || !(source instanceof Uint8Array) ||
      source.length === 0 || source.length > libraryLimits.fileBytes) throw Error("Invalid library source");
  if (typeof metadata.title !== "string" || !["MIDI", "MusicXML", "MXL"].includes(metadata.format) ||
      !Number.isSafeInteger(metadata.duration_us) || metadata.duration_us < 0 ||
      !Number.isSafeInteger(metadata.note_count) || metadata.note_count < 0) throw Error("Invalid library metadata");
  const usageQuery = db.prepare("SELECT COUNT(*), COALESCE(SUM(length(source)),0) FROM library_songs WHERE id <> ?");
  let usage;
  try { usageQuery.bind([id]); usageQuery.step(); usage = usageQuery.get(); }
  finally { usageQuery.free(); }
  if (usage[0] >= libraryLimits.songs || usage[1] + source.length > libraryLimits.totalBytes)
    throw Error("Library limit reached. Remove a piece before importing more.");
  const now = new Date().toISOString();
  db.run(`INSERT INTO library_songs VALUES(?,?,?,?,?,?,?,?)
    ON CONFLICT(id) DO UPDATE SET title=excluded.title, format=excluded.format,
      duration_us=excluded.duration_us, note_count=excluded.note_count,
      last_opened_at=excluded.last_opened_at`,
    [id, metadata.title, metadata.format, metadata.duration_us, metadata.note_count, source, now, now]);
}

export function listLibrary(db, offset = 0, limit = 8) {
  offset = Math.max(0, Math.floor(offset) || 0);
  limit = Math.max(1, Math.min(50, Math.floor(limit) || 8));
  const total = db.exec("SELECT COUNT(*) FROM library_songs")[0].values[0][0];
  const stmt = db.prepare(`SELECT id,title,format,duration_us,note_count FROM library_songs
    ORDER BY last_opened_at DESC,id LIMIT ? OFFSET ?`);
  const items = [];
  try {
    stmt.bind([limit, offset]);
    while (stmt.step()) items.push(stmt.getAsObject());
  } finally { stmt.free(); }
  return { items, total };
}
export function readLibrarySource(db, id) {
  const stmt = db.prepare("SELECT title,source FROM library_songs WHERE id=?");
  let result;
  try {
    stmt.bind([id]);
    if (!stmt.step()) throw Error("Piece is no longer in the library.");
    const row = stmt.getAsObject();
    result = { title: row.title, source: new Uint8Array(row.source) };
  } finally { stmt.free(); }
  db.run("UPDATE library_songs SET last_opened_at=? WHERE id=?", [new Date().toISOString(), id]);
  return result;
}
export function removeLibrarySong(db, id) {
  db.run("DELETE FROM library_songs WHERE id=?", [id]);
}
