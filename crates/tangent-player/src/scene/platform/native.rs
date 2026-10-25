//! Desktop capability adapter. No browser, loopback HTTP or webview.
//! The Bevy/render thread never opens devices, blocks on SQLite, or parses files.
#[path = "native/fingerprints.rs"]
mod fingerprints;
#[path = "native/library.rs"]
mod library;
#[path = "native/midi_output.rs"]
mod midi_output;
use crate::library::SourceFile;
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
static QUIT: AtomicBool = AtomicBool::new(false);
static SHUTDOWN_DONE: AtomicBool = AtomicBool::new(false);

struct Bus {
    actions: Sender<(String, String)>,
    updates: Mutex<Receiver<Value>>,
    files: Mutex<Receiver<SourceFile>>,
    imports: Sender<(Vec<u8>, String)>,
}
fn bus() -> &'static Bus {
    BUS.get_or_init(|| {
        let (actions, rx) = mpsc::channel();
        let (updates, messages) = mpsc::channel();
        let (file_tx, files) = mpsc::sync_channel(2);
        let (imports, import_rx) = mpsc::channel();
        thread::Builder::new()
            .name("tangent-native-io".into())
            .spawn(move || backend(rx, updates, file_tx, import_rx))
            .expect("spawn native audio/MIDI/storage worker");
        Bus {
            actions,
            updates: Mutex::new(messages),
            files: Mutex::new(files),
            imports,
        }
    })
}
pub fn clock_us() -> f64 {
    START.get_or_init(Instant::now).elapsed().as_secs_f64() * 1_000_000.0
}
pub fn action(name: &str, data: &str) {
    if name == "quit" {
        QUIT.store(true, Ordering::Release);
    } else if name == "fullscreen" {
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
pub fn store_import(bytes: &[u8], metadata: &str) {
    let _ = bus().imports.send((bytes.to_vec(), metadata.into()));
}
pub fn take_file() -> Option<SourceFile> {
    bus().files.lock().ok()?.try_recv().ok()
}
pub fn take_quit_request() -> bool {
    QUIT.swap(false, Ordering::AcqRel)
}
pub fn take_fullscreen_request() -> bool {
    FULLSCREEN.swap(false, Ordering::AcqRel)
}
/// Called after Bevy returns, while the I/O worker can still silence hardware.
pub fn shutdown() {
    if BUS.get().is_none() {
        return;
    }
    action("shutdown", "");
    let deadline = Instant::now() + Duration::from_secs(1);
    while !SHUTDOWN_DONE.load(Ordering::Acquire) && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(5));
    }
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
    output: midi_output::Output,
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
            output: midi_output::Output::new(out.clone()),
            stream,
            voices: HashMap::new(),
            song: Song::default(),
            transport: Transport::default(),
            anchor: Instant::now(),
            next: 0,
        }
    }
    fn silence(&mut self) {
        self.output.panic();
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
        self.output.reset(
            self.transport.position,
            self.transport.playing && self.transport.hear,
        );
    }
    fn play_note(
        &mut self,
        id: String,
        key: u8,
        velocity: u8,
        duration: Duration,
        delay: Duration,
    ) {
        if self.output.active() {
            return;
        }
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
        if self.output.active() {
            self.output.tick(position);
            return;
        }
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
            "midi-output" => {
                self.silence();
                self.output.select(value);
                self.transport.playing = false;
            }
            "midi-output-scan" => {
                self.silence();
                self.output.scan();
                self.transport.playing = false;
            }

            "song" => {
                if let Ok(song) = serde_json::from_str::<Song>(value) {
                    self.output.song(value);
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
                    if self.output.active() {
                        self.output.on(id, key as u8, velocity as u8);
                        return;
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
                self.output.off(value);
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
    if version > 4 {
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
    if version < 3 {
        conn.execute_batch("BEGIN;")?;
        conn.execute_batch(library::MIGRATION)?;
        conn.execute_batch("COMMIT;")?;
    }
    if version < 4 {
        conn.execute_batch("BEGIN;")?;
        library::migrate_v4(conn)?;
        conn.execute_batch("COMMIT;")?;
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
fn import_path(path: &Path, files: &mpsc::SyncSender<SourceFile>, out: &Sender<Value>) {
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
        files
            .send(SourceFile {
                bytes,
                title,
                from_library: false,
            })
            .map_err(|e| e.to_string())
    })();
    if let Err(e) = result {
        let _ = out.send(json!({"type":"library-status","text":format!("Import failed: {e}")}));
    }
}
fn pick_files() -> Result<Vec<PathBuf>, String> {
    #[cfg(target_os = "windows")]
    let output=Command::new("powershell").args(["-NoProfile","-STA","-Command",
        "[Console]::OutputEncoding=[System.Text.UTF8Encoding]::new($false); Add-Type -AssemblyName System.Windows.Forms; $d=New-Object System.Windows.Forms.OpenFileDialog; $d.Multiselect=$true; $d.Filter='Music files|*.mid;*.midi;*.xml;*.musicxml;*.mxl|All files|*.*'; if($d.ShowDialog() -eq 'OK'){ConvertTo-Json -Compress -InputObject @($d.FileNames)}"])
        .output().map_err(|e|format!("Could not open file picker: {e}"))?;
    #[cfg(target_os = "macos")]
    let output=Command::new("osascript").args(["-e", "set picked to choose file with prompt \"Open music scores\" with multiple selections allowed\nset paths to \"\"\nrepeat with f in picked\nset paths to paths & POSIX path of f & linefeed\nend repeat\nreturn paths"])
        .output().map_err(|e|format!("Could not open file picker: {e}"))?;
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let output = Command::new("zenity")
        .args([
            "--file-selection",
            "--multiple",
            "--separator=\n",
            "--title=Open music scores",
            "--file-filter=Music | *.mid *.midi *.xml *.musicxml *.mxl",
        ])
        .output()
        .map_err(|e| format!("Could not open file picker (install Zenity): {e}"))?;
    if !output.status.success() {
        return Ok(vec![]);
    }
    let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let text = text.trim().trim_start_matches('\u{feff}');
    if text.is_empty() {
        return Ok(vec![]);
    }
    #[cfg(target_os = "windows")]
    let paths: Vec<PathBuf> =
        serde_json::from_str(text).map_err(|e| format!("Invalid file picker result: {e}"))?;
    #[cfg(not(target_os = "windows"))]
    let paths: Vec<PathBuf> = text
        .lines()
        .filter(|p| !p.is_empty())
        .map(PathBuf::from)
        .collect();
    if paths.len() > 20 {
        return Err("Choose at most 20 files at once.".into());
    }
    Ok(paths)
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
fn backend(
    rx: Receiver<(String, String)>,
    out: Sender<Value>,
    files: mpsc::SyncSender<SourceFile>,
    imports: Receiver<(Vec<u8>, String)>,
) {
    let mut audio = Audio::new(&out);
    let db = storage(&out);
    let mut midi = connect_midi(&out);
    loop {
        for (bytes, metadata) in imports.try_iter().take(1) {
            let result = db
                .as_ref()
                .ok_or_else(|| "Local library unavailable".to_string())
                .and_then(|(conn, _)| library::save(conn, &bytes, &metadata));
            let message = match result {
                Ok(()) => json!({"type":"library-updated","text":"Saved in library."}),
                Err(e) => {
                    json!({"type":"library-status","text":format!("Could not save piece: {e}")})
                }
            };
            let _ = out.send(message);
        }
        match rx.recv_timeout(Duration::from_millis(4)) {
            Ok((name, value)) => {
                match name.as_str() {
                    "shutdown" => {
                        audio.silence();
                        midi.clear();
                        SHUTDOWN_DONE.store(true, Ordering::Release);
                        break;
                    }
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
                    "library-list" => {
                        if let Some((conn, _)) = &db {
                            if let Ok(request) = serde_json::from_str::<Value>(&value) {
                                match library::list(conn, &request) {
                                    Ok(message) => {
                                        let _ = out.send(message);
                                    }
                                    Err(e) => {
                                        let _ = out.send(json!({"type":"library-status","text":e}));
                                    }
                                }
                            }
                        } else {
                            let _ = out.send(
                                json!({"type":"library-status","text":"Local library unavailable"}),
                            );
                        }
                    }
                    "library-edit" => {
                        if let Some((conn, _)) = &db {
                            let message = match library::edit(conn, &value) {
                                Ok(()) => {
                                    json!({"type":"library-updated","text":"Metadata saved.","metadata":true})
                                }
                                Err(e) => json!({"type":"library-status","text":e}),
                            };
                            let _ = out.send(message);
                        }
                    }
                    "library-open" => {
                        if let Some((conn, _)) = &db {
                            match library::open(conn, &value) {
                                Ok((bytes, title)) => {
                                    let _ = files.send(SourceFile {
                                        bytes,
                                        title,
                                        from_library: true,
                                    });
                                }
                                Err(e) => {
                                    let _ = out.send(json!({"type":"library-status","text":format!("Could not open piece: {e}")}));
                                }
                            }
                        }
                    }
                    "library-remove" => {
                        if let Some((conn, _)) = &db {
                            let message = match library::remove(conn, &value) {
                                Ok(()) => {
                                    json!({"type":"library-updated","text":"Piece removed from library."})
                                }
                                Err(e) => json!({"type":"library-status","text":e}),
                            };
                            let _ = out.send(message);
                        }
                    }
                    "fingerprint-index" => {
                        if let Some((conn, _)) = &db
                            && let Err(e) = fingerprints::index(conn, &value)
                        {
                            let _ = out.send(json!({"type":"storage-status","text":format!("Index failed: {e}")}));
                        }
                    }
                    "fingerprint-query" => {
                        if let Some((conn, _)) = &db
                            && let Ok(request) = serde_json::from_str::<Value>(&value)
                        {
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
                    "midi" => {
                        // Release exclusive OS ports before attempting to reopen them.
                        // Clear held keys because a note-off may be lost during rescan.
                        midi.clear();
                        audio.handle("midi-output-scan", "");
                        audio.silence();
                        let _ = out.send(json!({"type":"pause"}));
                        midi = connect_midi(&out);
                    }
                    "import" => {
                        let tx = rx_sender_file(files.clone(), out.clone());
                        let picker_out = out.clone();
                        thread::spawn(move || match pick_files() {
                            Ok(paths) => {
                                for path in paths {
                                    tx(&path);
                                }
                            }
                            Err(error) => {
                                let _ =
                                    picker_out.send(json!({"type":"library-status","text":error}));
                            }
                        });
                    }
                    "path" => import_path(Path::new(&value), &files, &out),
                    "sample" => {
                        let _ = files.send(SourceFile {
                            bytes: include_bytes!(
                                "../../../../../web/piano/samples/first-light.musicxml"
                            )
                            .to_vec(),
                            title: "First light".into(),
                            from_library: false,
                        });
                    }
                    "credits" => {
                        let _ = out.send(json!({"type":"status","text":"First light is CC0 1.0; see web/piano/samples/ATTRIBUTION.md"}));
                    }
                    _ => audio.handle(&name, &value),
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                audio.silence();
                break;
            }
        }
        audio.tick();
        // Keep the connections alive even when hardware is temporarily idle.
        let _ = &midi;
    }
}
fn rx_sender_file(
    files: mpsc::SyncSender<SourceFile>,
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
        assert_eq!(version, 4);
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
        let (tx, rx) = mpsc::sync_channel(2);
        let (status_tx, _) = mpsc::channel();
        import_path(Path::new("/missing-tangent-score.mid"), &tx, &status_tx);
        assert!(rx.try_recv().is_err());
    }
}
