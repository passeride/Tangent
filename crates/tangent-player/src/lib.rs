//! Bevy application state, with explicit browser rendering/audio adapters.
mod import;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod preferences;
#[cfg(target_arch = "wasm32")]
mod scene;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_player() {
    scene::run();
}
use bevy_app::{App, Update};
use bevy_ecs::prelude::*;
use serde::Serialize;
use tangent_core::{music::Song, piano::PianoLayout, transport::Transport};
#[derive(Resource)]
struct Playback {
    song: Song,
    transport: Transport,
    elapsed: u64,
}
fn advance(mut state: ResMut<Playback>) {
    let elapsed = state.elapsed;
    state.transport.advance(elapsed);
    state.elapsed = 0;
}
#[derive(Serialize)]
struct Frame<'a> {
    position_us: u64,
    playing: bool,
    duration_us: u64,
    notes: Vec<&'a tangent_core::music::PerformanceNote>,
}
pub struct Player {
    app: App,
    layout: PianoLayout,
}
impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}
impl Player {
    pub fn new() -> Self {
        let song = import::study();
        let transport = Transport {
            duration_us: song.duration_us,
            ..Default::default()
        };
        let mut app = App::new();
        app.insert_resource(Playback {
            song,
            transport,
            elapsed: 0,
        });
        app.add_systems(Update, advance);
        Self {
            app,
            layout: PianoLayout::new(21, 108).unwrap(),
        }
    }
    pub fn load_midi(&mut self, bytes: &[u8], title: String) -> Result<String, String> {
        self.load_music(bytes, title)
    }
    pub fn load_music(&mut self, bytes: &[u8], title: String) -> Result<String, String> {
        let song = import::song(bytes, title)?;
        let json = serde_json::to_string(&song).unwrap();
        let transport = Transport {
            duration_us: song.duration_us,
            ..Default::default()
        };
        self.app.insert_resource(Playback {
            song,
            transport,
            elapsed: 0,
        });
        Ok(json)
    }
    pub fn song_json(&self) -> String {
        serde_json::to_string(&self.app.world().resource::<Playback>().song).unwrap()
    }
    pub fn layout_json(&self) -> String {
        serde_json::to_string(&self.layout).unwrap()
    }
    pub fn reading_json(&self) -> String {
        let values:Vec<_>=(0..=127).map(|key| {
            let p=tangent_core::notation::reading_pitch(key);
            serde_json::json!({"key":key,"step":p.step,"sharp":p.sharp,"bass":p.bass,"ledgers":tangent_core::notation::ledger_steps(p.step,if p.bass {18} else {30})})
        }).collect();
        serde_json::to_string(&values).unwrap()
    }
    pub fn set_range(&mut self, first: u8, last: u8) -> bool {
        if let Some(layout) = PianoLayout::new(first, last) {
            self.layout = layout;
            true
        } else {
            false
        }
    }
    pub fn hit_key(&self, x: f32, depth: f32) -> i16 {
        self.layout.hit(x, depth).map(i16::from).unwrap_or(-1)
    }
    pub fn set_playing(&mut self, playing: bool) {
        self.app
            .world_mut()
            .resource_mut::<Playback>()
            .transport
            .playing = playing;
    }
    pub fn seek(&mut self, us: f64) {
        self.app
            .world_mut()
            .resource_mut::<Playback>()
            .transport
            .seek(us.max(0.0) as u64);
    }
    /// JS uses numbers (safe through the importer's 24h limit), not wasm i64/BigInt.
    pub fn frame(&mut self, elapsed_us: f64) -> String {
        self.app.world_mut().resource_mut::<Playback>().elapsed = elapsed_us.max(0.0) as u64;
        self.app.update();
        let state = self.app.world().resource::<Playback>();
        let now = state.transport.position_us;
        let end = state
            .song
            .notes
            .partition_point(|n| n.start_us <= now + 8_000_000);
        let notes = state.song.notes[..end]
            .iter()
            .filter(|n| {
                n.end_us.saturating_add(8_000_000) >= now && !state.song.parts[n.part].percussion
            })
            .collect();
        serde_json::to_string(&Frame {
            position_us: now,
            playing: state.transport.playing,
            duration_us: state.song.duration_us,
            notes,
        })
        .unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn real_bevy_schedule_moves_and_pauses() {
        let mut p = Player::new();
        p.set_playing(true);
        let f: serde_json::Value = serde_json::from_str(&p.frame(2_100_000.0)).unwrap();
        assert_eq!(f["position_us"], 2_100_000);
        assert!(!f["notes"].as_array().unwrap().is_empty());
        p.set_playing(false);
        let f: serde_json::Value = serde_json::from_str(&p.frame(900_000.0)).unwrap();
        assert_eq!(f["position_us"], 2_100_000);
    }
    #[test]
    fn musicxml_uses_the_same_song_and_playback_boundary() {
        let mut p = Player::new();
        let song = p
            .load_music(
                include_bytes!("../../tangent-musicxml/tests/fixtures/piano.musicxml"),
                "XML".into(),
            )
            .unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&song).unwrap();
        assert_eq!(parsed["notes"].as_array().unwrap().len(), 5);
        assert!(parsed["notation"].is_object());
        p.set_playing(true);
        let frame: serde_json::Value = serde_json::from_str(&p.frame(1_000_000.)).unwrap();
        assert_eq!(frame["position_us"], 1_000_000);
        assert!(p.load_music(b"<broken>", "bad".into()).is_err());
        assert_eq!(p.song_json(), song);
    }
    #[test]
    fn failed_import_preserves_loaded_song() {
        let mut p = Player::new();
        let before = p.song_json();
        assert!(p.load_midi(b"bad", "bad".into()).is_err());
        assert_eq!(p.song_json(), before);
    }
}
