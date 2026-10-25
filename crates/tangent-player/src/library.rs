//! Portable library metadata; source bytes and queries belong to platform storage.
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Entry {
    pub id: String,
    pub title: String,
    pub format: String,
    pub duration_us: u64,
    pub note_count: usize,
}
#[derive(Serialize, Deserialize)]
pub(crate) struct ImportMetadata {
    pub title: String,
    pub format: String,
    pub duration_us: u64,
    pub note_count: usize,
}
impl ImportMetadata {
    pub fn from_song(song: &tangent_core::music::Song, bytes: &[u8]) -> Self {
        Self {
            title: song.title.clone(),
            duration_us: song.duration_us,
            note_count: song.notes.len(),
            format: if bytes.starts_with(b"MThd") {
                "MIDI"
            } else if bytes.starts_with(b"PK") {
                "MXL"
            } else {
                "MusicXML"
            }
            .into(),
        }
    }
}
