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

export function migrateLibraryV4(db) {
  db.run(`ALTER TABLE library_songs ADD COLUMN composer TEXT NOT NULL DEFAULT '';
    ALTER TABLE library_songs ADD COLUMN search_key TEXT NOT NULL DEFAULT '';
    ALTER TABLE library_songs ADD COLUMN fingerprint_id TEXT;
    CREATE INDEX library_fingerprint ON library_songs(fingerprint_id);`);
  const rows = db.exec("SELECT id,title,composer FROM library_songs");
  if (rows.length) for (const [id,title,composer] of rows[0].values)
    db.run("UPDATE library_songs SET search_key=? WHERE id=?", [searchKey(title,composer),id]);
  db.run("PRAGMA user_version=4");
}
function searchKey(title, composer) { return `${title}\n${composer}`.toLowerCase(); }
const orderBy = {
  recent: "last_opened_at DESC,id", title: "title COLLATE NOCASE,id",
  composer: "composer COLLATE NOCASE,title COLLATE NOCASE,id", duration: "duration_us,title COLLATE NOCASE,id",
};

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
  db.run(`INSERT INTO library_songs(id,title,format,duration_us,note_count,source,imported_at,last_opened_at,search_key,fingerprint_id,composer) VALUES(?,?,?,?,?,?,?,?,?,?,?)
    ON CONFLICT(id) DO UPDATE SET format=excluded.format, fingerprint_id=COALESCE(excluded.fingerprint_id,library_songs.fingerprint_id),
      duration_us=excluded.duration_us, note_count=excluded.note_count,
      last_opened_at=excluded.last_opened_at`,
    [id, metadata.title, metadata.format, metadata.duration_us, metadata.note_count, source, now, now, searchKey(metadata.title,metadata.composer??""), metadata.fingerprint_id ?? null,metadata.composer??""]);
}

export function listLibrary(db, offset = 0, limit = 8, query = "", sort = "recent", format = "") {
  offset = Math.max(0, Math.floor(offset) || 0);
  limit = Math.max(1, Math.min(50, Math.floor(limit) || 8));
  const needle = String(query).trim().toLowerCase().slice(0, 256);
  const kind = ["MIDI","MusicXML","MXL"].includes(format) ? format : "";
  const where = "WHERE instr(search_key,?)>0 AND (?='' OR format=?)";
  const count = db.prepare(`SELECT COUNT(*) FROM library_songs ${where}`);
  let total;
  try { count.bind([needle,kind,kind]); count.step(); total = count.get()[0]; } finally { count.free(); }
  const stmt = db.prepare(`SELECT id,title,composer,format,duration_us,note_count FROM library_songs
    ${where} ORDER BY ${orderBy[sort] ?? orderBy.recent} LIMIT ? OFFSET ?`);
  const items = [];
  try {
    stmt.bind([needle,kind,kind,limit,offset]);
    while (stmt.step()) items.push(stmt.getAsObject());
  } finally { stmt.free(); }
  return { items, total };
}
export function editLibraryMetadata(db, id, title, composer) {
  title = String(title).trim(); composer = String(composer).trim();
  if (!title || [...title].length > 160 || [...composer].length > 160 || /[\x00-\x1f]/.test(title+composer))
    throw Error("Use a title and composer of at most 160 characters.");
  db.run("UPDATE library_songs SET title=?,composer=?,search_key=? WHERE id=?", [title,composer,searchKey(title,composer),id]);
  db.run(`UPDATE fingerprint_songs SET title=? WHERE id=(SELECT fingerprint_id FROM library_songs WHERE id=?)`, [title,id]);
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
  const statement = db.prepare("SELECT fingerprint_id FROM library_songs WHERE id=?");
  let fingerprintId;
  try { statement.bind([id]); if (statement.step()) fingerprintId = statement.get()[0]; } finally { statement.free(); }
  db.run("DELETE FROM library_songs WHERE id=?", [id]);
  if (fingerprintId) db.run(`DELETE FROM fingerprint_songs WHERE id=? AND NOT EXISTS
    (SELECT 1 FROM library_songs WHERE fingerprint_id=?)`, [fingerprintId,fingerprintId]);
}
