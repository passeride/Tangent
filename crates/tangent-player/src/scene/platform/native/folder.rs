//! Background library ingestion. Never replace the active piece or block audio.
use rusqlite::{Connection, OptionalExtension};
use serde_json::{Value, json};
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
    sync::mpsc::{Sender, SyncSender, sync_channel},
    time::Duration,
};
pub(super) const MIGRATION: &str = "CREATE TABLE library_folder(id INTEGER PRIMARY KEY CHECK(id=1),path TEXT NOT NULL); PRAGMA user_version=7;";
const MAX_ENTRIES: usize = 10_000;
fn music(path: &Path) -> bool {
    path.extension().and_then(|s| s.to_str()).is_some_and(|s| {
        matches!(
            s.to_ascii_lowercase().as_str(),
            "mid" | "midi" | "xml" | "musicxml" | "mxl"
        )
    })
}
fn paths(root: &Path) -> Result<Vec<PathBuf>, String> {
    if !root.is_dir() {
        return Err("Library folder is missing or unavailable. Saved pieces are retained.".into());
    }
    let mut pending = vec![(root.to_owned(), 0)];
    let mut files = vec![];
    let mut visited = 0;
    while let Some((dir, depth)) = pending.pop() {
        for e in fs::read_dir(&dir).map_err(|e| format!("Cannot scan {}: {e}", dir.display()))? {
            visited += 1;
            if visited > MAX_ENTRIES {
                return Err(
                    "Folder exceeds the 10,000-entry scan limit. Choose a smaller folder.".into(),
                );
            }
            let e = e.map_err(|e| e.to_string())?;
            let kind = e.file_type().map_err(|e| e.to_string())?;
            if kind.is_symlink() {
                continue;
            }
            if kind.is_dir() {
                if depth >= 32 {
                    return Err("Folder nesting exceeds 32 levels.".into());
                }
                pending.push((e.path(), depth + 1));
            } else if kind.is_file() && music(&e.path()) {
                files.push(e.path());
            }
        }
    }
    files.sort();
    Ok(files)
}
#[derive(Default, Debug)]
struct Counts {
    added: usize,
    skipped: usize,
    failed: usize,
}
fn ingest(conn: &Connection, files: &[PathBuf], out: &Sender<Value>) -> Counts {
    let mut counts = Counts::default();
    let mut last_error = String::new();
    for (i, path) in files.iter().enumerate() {
        let result = (|| -> Result<bool, String> {
            if !music(path) {
                return Err("Unsupported file type".into());
            }
            let stat = fs::metadata(path).map_err(|e| e.to_string())?;
            if !stat.is_file() || stat.len() > 16 * 1024 * 1024 {
                return Err("File exceeds 16 MiB or is not a regular file".into());
            }
            let mut bytes = vec![];
            fs::File::open(path)
                .map_err(|e| e.to_string())?
                .take(16 * 1024 * 1024 + 1)
                .read_to_end(&mut bytes)
                .map_err(|e| e.to_string())?;
            if bytes.len() > 16 * 1024 * 1024 {
                return Err("File grew beyond 16 MiB while reading".into());
            }
            let id = blake3::hash(&bytes).to_hex().to_string();
            let exists: bool = conn
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM library_songs WHERE id=?1)",
                    [&id],
                    |r| r.get(0),
                )
                .map_err(|e| e.to_string())?;
            if exists {
                return Ok(false);
            }
            let title = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Score")
                .to_string();
            let song = crate::import::song(&bytes, title)?;
            super::library::save(
                conn,
                &bytes,
                &serde_json::to_string(&crate::library::ImportMetadata::from_song(&song, &bytes))
                    .map_err(|e| e.to_string())?,
            )?;
            Ok(true)
        })();
        match result {
            Ok(true) => counts.added += 1,
            Ok(false) => counts.skipped += 1,
            Err(e) => {
                counts.failed += 1;
                last_error = format!(
                    "{}: {e}",
                    path.file_name().unwrap_or_default().to_string_lossy()
                );
            }
        }
        let _=out.send(json!({"type":"library-scan","busy":true,"text":format!("Scanned {}/{} · {} new · {} unchanged · {} failed",i+1,files.len(),counts.added,counts.skipped,counts.failed)}));
    }
    let text = format!(
        "{} new · {} unchanged · {} failed{}",
        counts.added,
        counts.skipped,
        counts.failed,
        if last_error.is_empty() {
            String::new()
        } else {
            format!(" · Last error: {last_error}")
        }
    );
    let _ = out.send(json!({"type":"library-scan","busy":false,"text":text}));
    let _ = out.send(json!({"type":"library-updated","text":text}));
    counts
}
fn choose() -> Result<Option<PathBuf>, String> {
    use std::process::Command;
    #[cfg(target_os="windows")]
 let output=Command::new("powershell").args(["-NoProfile","-STA","-Command","[Console]::OutputEncoding=[System.Text.UTF8Encoding]::new($false);Add-Type -AssemblyName System.Windows.Forms;$d=New-Object System.Windows.Forms.FolderBrowserDialog;$d.Description='Choose Tangent library folder';if($d.ShowDialog() -eq 'OK'){$d.SelectedPath}"]).output();
    #[cfg(target_os = "macos")]
    let output = Command::new("osascript")
        .args([
            "-e",
            "POSIX path of (choose folder with prompt \"Choose Tangent library folder\")",
        ])
        .output();
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let output = Command::new("zenity")
        .args([
            "--file-selection",
            "--directory",
            "--title=Choose Tangent library folder",
        ])
        .output();
    let output = output.map_err(|e| format!("Folder picker unavailable: {e}"))?;
    if !output.status.success() {
        return Ok(None);
    }
    let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let text = text.trim().trim_start_matches('\u{feff}');
    Ok((!text.is_empty()).then(|| PathBuf::from(text)))
}
pub(super) fn start(
    db_path: Option<PathBuf>,
    out: Sender<Value>,
    files_tx: SyncSender<crate::library::SourceFile>,
) -> SyncSender<String> {
    let (tx, rx) = sync_channel::<String>(4);
    std::thread::spawn(move || {
        let open = || -> Result<Connection, String> {
            let conn = Connection::open(db_path.as_ref().ok_or("Local library unavailable")?)
                .map_err(|e| e.to_string())?;
            conn.busy_timeout(Duration::from_secs(5))
                .map_err(|e| e.to_string())?;
            conn.execute_batch("PRAGMA foreign_keys=ON;")
                .map_err(|e| e.to_string())?;
            Ok(conn)
        };
        let mut saved = open()
            .and_then(|c| {
                c.query_row("SELECT path FROM library_folder WHERE id=1", [], |r| {
                    r.get::<_, String>(0)
                })
                .optional()
                .map_err(|e| e.to_string())
            })
            .ok()
            .flatten();
        let send_path = |path: &Option<String>| {
            let _ = out.send(
                json!({"type":"library-folder","path":path.as_deref().unwrap_or(""),"auto":path.is_some()}),
            );
        };
        send_path(&saved);
        let mut automatic = saved.is_some();
        loop {
            let command = if automatic {
                automatic = false;
                "scan".to_string()
            } else {
                match rx.recv() {
                    Ok(c) => c,
                    Err(_) => break,
                }
            };
            let result = (|| -> Result<(), String> {
                // A single explicit open remains usable without local storage.
                if command == "import" {
                    let selected = super::pick_files()?;
                    if selected.len() == 1 {
                        super::import_path(&selected[0], &files_tx, &out);
                        return Ok(());
                    }
                    if !selected.is_empty() {
                        let conn = open()?;
                        let _=out.send(json!({"type":"library-scan","busy":true,"text":"Importing selected files…"}));
                        ingest(&conn, &selected, &out);
                    }
                    return Ok(());
                }
                let conn = open()?;
                if command == "clear" {
                    conn.execute("DELETE FROM library_folder", [])
                        .map_err(|e| e.to_string())?;
                    saved = None;
                    send_path(&saved);
                    return Ok(());
                }
                if command == "choose" {
                    let Some(path) = choose()? else {
                        return Ok(());
                    };
                    let path = path
                        .to_str()
                        .ok_or("Folder path must be valid Unicode")?
                        .to_string();
                    conn.execute("INSERT INTO library_folder(id,path) VALUES(1,?1) ON CONFLICT(id) DO UPDATE SET path=excluded.path",[&path]).map_err(|e|e.to_string())?;
                    saved = Some(path);
                    send_path(&saved);
                }
                let _ =
                    out.send(json!({"type":"library-scan","busy":true,"text":"Reading folder…"}));
                let root = saved.as_ref().ok_or("Choose a library folder first")?;
                let selected = paths(Path::new(root))?;
                ingest(&conn, &selected, &out);
                Ok(())
            })();
            if let Err(e) = result {
                let _ = out.send(json!({"type":"library-scan","busy":false,"text":e}));
            }
        }
    });
    tx
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recursive_scan_deduplicates_and_preserves_library() {
        let root = std::env::temp_dir().join(format!(
            "tangent-folder-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join("sub")).unwrap();
        let sample = include_bytes!("../../../../../../web/piano/samples/first-light.musicxml");
        fs::write(root.join("one.musicxml"), sample).unwrap();
        fs::write(root.join("sub/two.MXL"), sample).unwrap();
        fs::write(root.join("ignored.txt"), b"ignored").unwrap();
        fs::write(root.join("bad.mid"), b"bad").unwrap();
        let paths = paths(&root).unwrap();
        assert_eq!(paths.len(), 3);
        let db = Connection::open_in_memory().unwrap();
        super::super::migrate(&db).unwrap();
        let (out, _) = std::sync::mpsc::channel();
        let counts = ingest(&db, &paths, &out);
        assert_eq!((counts.added, counts.skipped, counts.failed), (1, 1, 1));
        let id = blake3::hash(sample).to_hex().to_string();
        super::super::library::edit(
            &db,
            &json!({"id":id,"title":"Mine","composer":"Me"}).to_string(),
        )
        .unwrap();
        let counts = ingest(&db, &paths, &out);
        assert_eq!((counts.added, counts.skipped), (0, 2));
        assert_eq!(
            super::super::library::list(&db, &json!({})).unwrap()["items"][0]["title"],
            "Mine"
        );
        fs::remove_dir_all(root).unwrap();
    }
}
