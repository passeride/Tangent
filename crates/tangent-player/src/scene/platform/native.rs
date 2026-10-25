//! Desktop capability adapter. No browser, loopback HTTP or webview.
//! The Bevy/render thread never opens devices, blocks on SQLite, or parses files.
#[path = "native/fingerprints.rs"]
mod fingerprints;
use midir::{Ignore, MidiInput, MidiInputConnection};
use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player, Source, source::SineWave};
use rusqlite::{Connection, OptionalExtension};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::HashMap,
    env, fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
    sync::{
        Mutex, OnceLock,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, Sender},
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_VOICES: usize = 64;
const LOOKAHEAD_US: u64 = 32_000;
static BUS: OnceLock<Bus> = OnceLock::new();
static START: OnceLock<Instant> = OnceLock::new();
static FULLSCREEN: AtomicBool = AtomicBool::new(false);

struct Bus {
    actions: Sender<(String, String)>,
    updates: Mutex<Receiver<Value>>,
    files: Mutex<Receiver<(Vec<u8>, String)>>,
}
fn bus() -> &'static Bus {
    BUS.get_or_init(|| {
        let (actions, rx) = mpsc::channel();
        let (updates, messages) = mpsc::channel();
        let (file_tx, files) = mpsc::channel();
        thread::Builder::new()
            .name("tangent-native-io".into())
            .spawn(move || backend(rx, updates, file_tx))
            .expect("spawn native audio/MIDI/storage worker");
        Bus {
            actions,
            updates: Mutex::new(messages),
            files: Mutex::new(files),
        }
    })
}
pub fn clock_us() -> f64 {
    START.get_or_init(Instant::now).elapsed().as_secs_f64() * 1_000_000.0
}
pub fn action(name: &str, data: &str) {
    if name == "fullscreen" {
        FULLSCREEN.store(true, Ordering::Release);
    } else {
        let _ = bus().actions.send((name.into(), data.into()));
    }
}
pub fn pull() -> String {
    let mut messages = Vec::new();
    if let Ok(rx) = bus().updates.lock() {
        for event in rx.try_iter().take(512) {
            messages.push(event);
        }
    }
    serde_json::to_string(&messages).unwrap_or_else(|_| "[]".into())
}
pub fn take_file() -> Option<(Vec<u8>, String)> {
    bus().files.lock().ok()?.try_recv().ok()
}
pub fn take_fullscreen_request() -> bool {
    FULLSCREEN.swap(false, Ordering::AcqRel)
}
#[derive(Deserialize, Default)]
struct Note {
    key: u8,
    velocity: u8,
    part: usize,
    start_us: u64,
    end_us: u64,
}
#[derive(Deserialize, Default)]
struct Part {
    percussion: bool,
}
#[derive(Deserialize, Default)]
struct Song {
    notes: Vec<Note>,
    parts: Vec<Part>,
}
#[derive(Deserialize, Default)]
struct Transport {
    position: u64,
    playing: bool,
    volume: f32,
    hear: bool,
    instrument: usize,
}
struct Audio {
    stream: Option<MixerDeviceSink>,
    voices: HashMap<String, Player>,
    song: Song,
    transport: Transport,
    anchor: Instant,
    next: usize,
}
impl Audio {
    fn new(out: &Sender<Value>) -> Self {
        let stream = DeviceSinkBuilder::open_default_sink().ok();
        if stream.is_none() {
            let _ = out.send(
                json!({"type":"status","text":"No audio device; visual player still available."}),
            );
        }
        Self {
            stream,
            voices: HashMap::new(),
            song: Song::default(),
            transport: Transport::default(),
            anchor: Instant::now(),
            next: 0,
        }
    }
    fn silence(&mut self) {
        for (_, voice) in self.voices.drain() {
            voice.stop();
        }
    }
    fn reset(&mut self) {
        self.silence();
        self.next = self
            .song
            .notes
            .partition_point(|n| n.start_us < self.transport.position);
        self.anchor = Instant::now();
    }
    fn play_note(
        &mut self,
        id: String,
        key: u8,
        velocity: u8,
        duration: Duration,
        delay: Duration,
    ) {
        let Some(stream) = &self.stream else { return };
        if self.voices.len() >= MAX_VOICES {
            return;
        }
        let freq = 440.0 * 2.0_f32.powf((f32::from(key) - 69.0) / 12.0);
        let p = Player::connect_new(stream.mixer());
        let level = (f32::from(velocity) / 127.) * 0.16;
        p.set_volume(self.transport.volume.clamp(0., 1.));
        // Soft harmonic voice. All DSP is performed by the audio device's mixer,
        // never by the Bevy rendering thread.
        let source = SineWave::new(freq)
            .mix(
                SineWave::new(freq * 2.).amplify(if self.transport.instrument == 0 {
                    0.22
                } else {
                    0.36
                }),
            )
            .mix(SineWave::new(freq * 3.).amplify(0.08))
            .take_duration(duration)
            .fade_in(Duration::from_millis(6))
            .fade_out(Duration::from_millis(100))
            .amplify(level)
            .delay(delay);
        p.append(source);
        self.voices.insert(id, p);
    }
    fn tick(&mut self) {
        self.voices.retain(|_, p| !p.empty());
        if !self.transport.playing || !self.transport.hear {
            return;
        }
        let position = self
            .transport
            .position
            .saturating_add(self.anchor.elapsed().as_micros() as u64);
        let horizon = position.saturating_add(LOOKAHEAD_US);
        while let Some(note) = self.song.notes.get(self.next) {
            if note.start_us > horizon {
                break;
            }
            let (key, velocity, part, begin, end) = (
                note.key,
                note.velocity,
                note.part,
                note.start_us,
                note.end_us,
            );
            self.next += 1;
            if self.song.parts.get(part).is_some_and(|p| p.percussion) || end <= position {
                continue;
            }
            let delay = Duration::from_micros(begin.saturating_sub(position));
            let length = Duration::from_micros(end.saturating_sub(begin.max(position)).max(50_000));
            self.play_note(format!("song:{}", self.next), key, velocity, length, delay);
        }
    }
    fn handle(&mut self, name: &str, value: &str) {
        match name {
            "song" => {
                if let Ok(song) = serde_json::from_str::<Song>(value) {
                    self.song = song;
                    self.reset();
                }
            }
            "transport" => {
                if let Ok(t) = serde_json::from_str::<Transport>(value) {
                    self.transport = t;
                    self.reset();
                }
            }
            "volume" => {
                if let Ok(v) = value.parse::<f32>() {
                    self.transport.volume = v;
                    for p in self.voices.values() {
                        p.set_volume(v.clamp(0., 1.));
                    }
                }
            }
            "on" => {
                if let Ok(n) = serde_json::from_str::<Value>(value)
                    && let (Some(id), Some(key), Some(velocity)) =
                        (n["id"].as_str(), n["key"].as_u64(), n["velocity"].as_u64())
                {
                    if let Some(old) = self.voices.remove(id) {
                        old.stop();
                    }
                    self.play_note(
                        id.into(),
                        key as u8,
                        velocity as u8,
                        Duration::from_secs(10),
                        Duration::ZERO,
                    );
                }
            }
            "off" => {
                if let Some(voice) = self.voices.remove(value) {
                    voice.stop();
                }
            }
            "panic" => self.silence(),
            _ => {}
        }
    }
}
fn data_dir() -> Result<PathBuf, String> {
    let folder = if cfg!(target_os = "windows") {
        PathBuf::from(env::var_os("LOCALAPPDATA").ok_or("LOCALAPPDATA not set")?).join("Tangent")
    } else if cfg!(target_os = "macos") {
        PathBuf::from(env::var_os("HOME").ok_or("HOME not set")?)
            .join("Library/Application Support/Tangent")
    } else if let Some(xdg) = env::var_os("XDG_DATA_HOME") {
        PathBuf::from(xdg).join("tangent")
    } else {
        PathBuf::from(env::var_os("HOME").ok_or("HOME not set")?).join(".local/share/tangent")
    };
    fs::create_dir_all(&folder).map_err(|e| e.to_string())?;
    Ok(folder)
}
fn migrate(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    let version: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
    if version > 2 {
        return Err(rusqlite::Error::InvalidQuery);
    }
    if version == 0 {
        conn.execute_batch(
            "BEGIN;
            CREATE TABLE settings (id INTEGER PRIMARY KEY CHECK (id = 1),
                payload TEXT NOT NULL, updated_at TEXT NOT NULL);
            CREATE TABLE practice_sessions (id TEXT PRIMARY KEY NOT NULL,
                mode TEXT NOT NULL, started_at TEXT NOT NULL, ended_at TEXT);
            CREATE TABLE scores (id TEXT PRIMARY KEY NOT NULL,
                session_id TEXT NOT NULL REFERENCES practice_sessions(id) ON DELETE CASCADE,
                metric TEXT NOT NULL, value REAL NOT NULL, created_at TEXT NOT NULL);
            CREATE INDEX scores_by_session ON scores(session_id);
            PRAGMA user_version = 1; COMMIT;",
        )?;
    }
    if version < 2 {
        conn.execute_batch(
            "BEGIN;
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
      COMMIT;",
        )?;
    }
    Ok(())
}
fn storage(out: &Sender<Value>) -> Option<(Connection, PathBuf)> {
    let res = (|| -> Result<_, String> {
        let path = data_dir()?.join("tangent.sqlite");
        let conn = Connection::open(&path).map_err(|e| e.to_string())?;
        migrate(&conn).map_err(|e| e.to_string())?;
        let payload: Option<String> = conn
            .query_row("SELECT payload FROM settings WHERE id=1", [], |r| r.get(0))
            .optional()
            .map_err(|e| e.to_string())?;
        let data = payload.and_then(|s| serde_json::from_str::<Value>(&s).ok());
        let _ = out.send(json!({"type":"preferences","data":data}));
        let _ = out.send(
            json!({"type":"storage-status","text":format!("Local SQLite: {}",path.display())}),
        );
        Ok((conn, path))
    })();
    match res {
        Ok(db) => Some(db),
        Err(e) => {
            let _ = out
                .send(json!({"type":"storage-status","text":format!("SQLite unavailable: {e}")}));
            // Let the UI continue with defaults, without pretending a save succeeded.
            let _ = out.send(json!({"type":"preferences","data":null}));
            None
        }
    }
}
fn save(conn: &Connection, json: &str) -> rusqlite::Result<()> {
    let _: Value = serde_json::from_str(json).map_err(|_| rusqlite::Error::InvalidQuery)?;
    conn.execute(
        "INSERT INTO settings (id,payload,updated_at) VALUES(1,?1,datetime('now'))
         ON CONFLICT(id) DO UPDATE SET payload=excluded.payload,updated_at=excluded.updated_at",
        [json],
    )?;
    Ok(())
}
fn export(conn: &Connection, path: &Path) -> Result<PathBuf, String> {
    let base = env::var_os("USERPROFILE")
        .or_else(|| env::var_os("HOME"))
        .map(PathBuf::from)
        .unwrap_or_else(|| path.to_owned());
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let dest = base
        .join("Downloads")
        .join(format!("tangent-{seconds}.sqlite"));
    conn.execute("VACUUM INTO ?1", [dest.to_string_lossy().as_ref()])
        .map_err(|e| e.to_string())?;
    Ok(dest)
}
fn read_music(reader: impl Read) -> Result<Vec<u8>, String> {
    // Enforce the limit while reading: metadata can be stale if a file grows.
    let mut bytes = Vec::new();
    reader
        .take(MAX_FILE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MAX_FILE_BYTES {
        return Err("Music file exceeds 16 MiB".into());
    }
    Ok(bytes)
}
fn import_path(path: &Path, files: &Sender<(Vec<u8>, String)>, out: &Sender<Value>) {
    let result = (|| -> Result<(), String> {
        let file = fs::File::open(path).map_err(|e| e.to_string())?;
        let metadata = file.metadata().map_err(|e| e.to_string())?;
        if !metadata.is_file() {
            return Err("Select a regular music file".into());
        }
        if metadata.len() > MAX_FILE_BYTES {
            return Err("Music file exceeds 16 MiB".into());
        }
        let bytes = read_music(file)?;
        let title = path
            .file_stem()
            .and_then(|x| x.to_str())
            .unwrap_or("Score")
            .into();
        files.send((bytes, title)).map_err(|e| e.to_string())
    })();
    if let Err(e) = result {
        let _ = out.send(json!({"type":"status","text":format!("Import failed: {e}")}));
    }
}
fn pick_file() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    let output = Command::new("powershell").args(["-NoProfile","-Command",
        "Add-Type -AssemblyName System.Windows.Forms; $d=New-Object System.Windows.Forms.OpenFileDialog; $d.Filter='Music files|*.mid;*.midi;*.xml;*.musicxml;*.mxl|All files|*.*'; if($d.ShowDialog() -eq 'OK'){$d.FileName}"]).output().ok()?;
    #[cfg(target_os = "macos")]
    let output = Command::new("osascript")
        .args([
            "-e",
            "POSIX path of (choose file with prompt \"Open music score\")",
        ])
        .output()
        .ok()?;
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let output = Command::new("zenity")
        .args([
            "--file-selection",
            "--title=Open music score",
            "--file-filter=Music | *.mid *.midi *.xml *.musicxml *.mxl",
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let path = String::from_utf8(output.stdout).ok()?;
    let path = path.trim();
    (!path.is_empty()).then(|| PathBuf::from(path))
}
fn connect_midi(out: &Sender<Value>) -> Vec<MidiInputConnection<()>> {
    let count = match MidiInput::new("Tangent probe") {
        Ok(input) => input.ports().len(),
        Err(e) => {
            let _ = out.send(json!({"type":"status","text":format!("MIDI unavailable: {e}")}));
            return Vec::new();
        }
    };
    let mut connections = Vec::new();
    for index in 0..count {
        let Ok(mut input) = MidiInput::new("Tangent") else {
            continue;
        };
        input.ignore(Ignore::None);
        let ports = input.ports();
        let Some(port) = ports.get(index) else {
            continue;
        };
        let name = input
            .port_name(port)
            .unwrap_or_else(|_| format!("Port {index}"));
        let tx = out.clone();
        let connect = input.connect(
            port,
            "Tangent piano input",
            move |_, bytes, _| {
                if bytes.len() < 3 {
                    return;
                }
                let kind = bytes[0] & 0xf0;
                let channel = bytes[0] & 0x0f;
                let key = bytes[1];
                let velocity = bytes[2];
                if kind == 0x90 || kind == 0x80 {
                    let _ = tx.send(json!({"type":"note",
                    "id":format!("midi:{index}:{channel}:{key}"),"key":key,
                    "velocity":velocity,"down":kind==0x90 && velocity>0}));
                } else if kind == 0xb0 && (key == 120 || key == 123) {
                    let _ = tx.send(json!({"type":"pause"}));
                }
            },
            (),
        );
        if let Ok(conn) = connect {
            connections.push(conn);
            let _ = out.send(json!({"type":"status","text":format!("MIDI connected: {name}")}));
        }
    }
    if connections.is_empty() {
        let _ = out.send(
            json!({"type":"status","text":"No MIDI inputs; use keyboard or Ctrl+M to rescan."}),
        );
    }
    connections
}
fn backend(rx: Receiver<(String, String)>, out: Sender<Value>, files: Sender<(Vec<u8>, String)>) {
    let mut audio = Audio::new(&out);
    let db = storage(&out);
    let mut midi = connect_midi(&out);
    loop {
        match rx.recv_timeout(Duration::from_millis(4)) {
            Ok((name, value)) => match name.as_str() {
                "save-settings" => {
                    if let Some((conn, _)) = &db
                        && let Err(e) = save(conn, &value)
                    {
                        let _ = out.send(
                            json!({"type":"storage-status","text":format!("Save failed: {e}")}),
                        );
                    }
                }
                "export-database" => {
                    if let Some((conn, path)) = &db {
                        let info = match export(conn, path) {
                            Ok(p) => format!("SQLite backup: {}", p.display()),
                            Err(e) => format!("Export failed: {e}"),
                        };
                        let _ = out.send(json!({"type":"storage-status","text":info}));
                    }
                }
                "fingerprint-index" => {
                    if let Some((conn, _)) = &db {
                        if let Err(e) = fingerprints::index(conn, &value) {
                            let _ = out.send(json!({"type":"storage-status","text":format!("Index failed: {e}")}));
                        }
                    }
                }
                "fingerprint-query" => {
                    if let Some((conn, _)) = &db {
                        if let Ok(request) = serde_json::from_str::<Value>(&value) {
                            match fingerprints::candidates(conn, &request["signatures"]) {
                                Ok(candidates) => {
                                    let _ = out.send(json!({"type":"fingerprint-candidates",
                                "ticket":request["ticket"],"keys":request["keys"],"candidates":candidates}));
                                }
                                Err(e) => {
                                    let _ = out.send(json!({"type":"status","text":format!("Search failed: {e}")}));
                                }
                            }
                        }
                    }
                }
                "midi" => {
                    // Release exclusive OS ports before attempting to reopen them.
                    // Clear held keys because a note-off may be lost during rescan.
                    midi.clear();
                    audio.silence();
                    let _ = out.send(json!({"type":"pause"}));
                    midi = connect_midi(&out);
                }
                "import" => {
                    let tx = rx_sender_file(files.clone(), out.clone());
                    thread::spawn(move || {
                        if let Some(path) = pick_file() {
                            tx(&path);
                        }
                    });
                }
                "path" => import_path(Path::new(&value), &files, &out),
                "sample" => {
                    let _ = files.send((
                        include_bytes!("../../../../../web/piano/samples/first-light.musicxml")
                            .to_vec(),
                        "First light".into(),
                    ));
                }
                "credits" => {
                    let _ = out.send(json!({"type":"status","text":"First light is CC0 1.0; see web/piano/samples/ATTRIBUTION.md"}));
                }
                _ => audio.handle(&name, &value),
            },
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
        audio.tick();
        // Keep the connections alive even when hardware is temporarily idle.
        let _ = &midi;
    }
}
fn rx_sender_file(
    files: Sender<(Vec<u8>, String)>,
    out: Sender<Value>,
) -> impl Fn(&Path) + Send + 'static {
    move |path| import_path(path, &files, &out)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn desktop_schema_is_portable_with_browser_sqlite() {
        let db = Connection::open_in_memory().unwrap();
        migrate(&db).unwrap();
        migrate(&db).unwrap();
        save(&db, r#"{"version":1,"view":0}"#).unwrap();
        let value: String = db
            .query_row("SELECT payload FROM settings", [], |r| r.get(0))
            .unwrap();
        assert_eq!(serde_json::from_str::<Value>(&value).unwrap()["version"], 1);
        let version: i64 = db
            .query_row("PRAGMA user_version", [], |r| r.get(0))
            .unwrap();
        assert_eq!(version, 2);
        let rows: i64 = db
            .query_row("SELECT count(*) FROM scores", [], |r| r.get(0))
            .unwrap();
        assert_eq!(rows, 0);
    }
    #[test]
    fn import_read_is_bounded_even_without_file_metadata() {
        let bytes = read_music(&b"score bytes"[..]).unwrap();
        assert_eq!(bytes, b"score bytes");
        let error = read_music(std::io::repeat(0)).unwrap_err();
        assert!(error.contains("16 MiB"));
    }
    #[test]
    fn missing_import_does_not_enqueue_a_song() {
        let (tx, rx) = mpsc::channel();
        let (status_tx, _) = mpsc::channel();
        import_path(Path::new("/missing-tangent-score.mid"), &tx, &status_tx);
        assert!(rx.try_recv().is_err());
    }
}
