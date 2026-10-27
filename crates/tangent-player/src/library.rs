//! Portable library metadata; source bytes and queries belong to platform storage.
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Entry {
    #[serde(default)]
    pub enrichment: Option<tangent_core::metadata::Candidate>,
    #[serde(default)]
    pub has_cover: bool,
    pub id: String,
    pub title: String,
    pub composer: String,
    pub format: String,
    pub duration_us: u64,
    pub note_count: usize,
}
#[derive(Serialize, Deserialize)]
pub(crate) struct ImportMetadata {
    #[serde(default)]
    pub composer: String,
    #[serde(default)]
    pub fingerprint: Option<tangent_core::fingerprint::FingerprintedSong>,
    pub title: String,
    pub format: String,
    pub duration_us: u64,
    pub note_count: usize,
}
impl ImportMetadata {
    pub fn from_song(song: &tangent_core::music::Song, bytes: &[u8]) -> Self {
        Self {
            composer: song.composer.clone().unwrap_or_default(),
            fingerprint: Some(tangent_core::fingerprint::index_song(song)),
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

#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct SourceFile {
    pub bytes: Vec<u8>,
    pub title: String,
    pub from_library: bool,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct MetadataEdit {
    pub id: String,
    pub title: String,
    pub composer: String,
}
impl MetadataEdit {
    pub fn validate(&self) -> bool {
        !self.title.trim().is_empty()
            && self.title.chars().count() <= 160
            && self.composer.chars().count() <= 160
            && !self
                .title
                .chars()
                .chain(self.composer.chars())
                .any(char::is_control)
    }
}
