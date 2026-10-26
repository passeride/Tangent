//! Stable input preferences and MIDI command grammar. No Bevy or platform types.
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyboardChord {
    pub key: String,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Binding {
    pub action: String,
    pub keyboard: Option<KeyboardChord>,
    pub piano: Option<u8>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bindings {
    pub version: u8,
    pub piano_enabled: bool,
    pub control: u8,
    pub rows: Vec<Binding>,
}
/// Stable IDs, labels and default physical key chords. PC piano notes are rows too.
pub const CATALOG: &[(&str, &str, &str, bool, bool)] = &[
    ("nav_next", "Next menu control", "Tab", false, false),
    ("nav_previous", "Previous menu control", "Tab", false, true),
    (
        "nav_activate",
        "Activate menu control",
        "Enter",
        false,
        false,
    ),
    (
        "nav_activate_alt",
        "Activate menu (numpad)",
        "NumpadEnter",
        false,
        false,
    ),
    ("nav_back", "Back / close / pause", "Escape", false, false),
    ("play", "Play / pause", "Space", false, false),
    ("slower", "Slower speed", "ArrowDown", false, false),
    ("faster", "Faster speed", "ArrowUp", false, false),
    ("back", "Jump back 5 seconds", "ArrowLeft", false, false),
    (
        "forward",
        "Jump forward 5 seconds",
        "ArrowRight",
        false,
        false,
    ),
    (
        "back_long",
        "Jump back 15 seconds",
        "ArrowLeft",
        false,
        true,
    ),
    (
        "forward_long",
        "Jump forward 15 seconds",
        "ArrowRight",
        false,
        true,
    ),
    ("restart", "Restart", "Home", false, false),
    ("end", "Jump to end", "End", false, false),
    ("fullscreen", "Fullscreen", "F11", false, false),
    ("view", "Next view", "KeyV", false, false),
    ("theme", "Next theme", "KeyB", false, false),
    ("hear", "Song sound", "KeyM", false, false),
    ("effects", "Particles", "KeyP", false, false),
    ("instrument", "Next instrument", "KeyX", false, false),
    ("range", "Keyboard range", "KeyR", false, false),
    ("pause", "Pause / settings", "F1", false, false),
    ("import", "Import music", "KeyO", true, false),
    ("sample", "Load sample", "KeyB", true, false),
    ("midi", "Rescan MIDI", "KeyM", true, false),
    ("search", "Library search", "KeyF", true, false),
    ("quieter", "Volume down", "", false, false),
    ("louder", "Volume up", "", false, false),
    ("speed_reset", "Reset speed", "", false, false),
    ("loop_a", "Set A", "", false, false),
    ("loop_b", "Set B", "", false, false),
    ("loop_clear", "Clear loop", "", false, false),
    ("loop_snap", "Snap bars", "", false, false),
    ("part", "Next target part", "", false, false),
    ("fit", "Fit keys", "", false, false),
    ("hint", "Exercise hint", "", false, false),
    ("listen", "Hear exercise", "", false, false),
    ("skip", "Skip exercise", "", false, false),
    ("note_60", "Play 60", "KeyA", false, false),
    ("note_61", "Play 61", "KeyW", false, false),
    ("note_62", "Play 62", "KeyS", false, false),
    ("note_63", "Play 63", "KeyE", false, false),
    ("note_64", "Play 64", "KeyD", false, false),
    ("note_65", "Play 65", "KeyF", false, false),
    ("note_66", "Play 66", "KeyT", false, false),
    ("note_67", "Play 67", "KeyG", false, false),
    ("note_68", "Play 68", "KeyY", false, false),
    ("note_69", "Play 69", "KeyH", false, false),
    ("note_70", "Play 70", "KeyU", false, false),
    ("note_71", "Play 71", "KeyJ", false, false),
    ("note_72", "Play 72", "KeyK", false, false),
];
impl Default for Bindings {
    fn default() -> Self {
        Self {
            version: 1,
            piano_enabled: false,
            control: 21,
            rows: CATALOG
                .iter()
                .map(|&(action, _, key, ctrl, shift)| Binding {
                    action: action.into(),
                    keyboard: (!key.is_empty()).then(|| KeyboardChord {
                        key: key.into(),
                        ctrl,
                        shift,
                        alt: false,
                    }),
                    piano: None,
                })
                .collect(),
        }
    }
}
pub fn valid_key(key: &str) -> bool {
    matches!(
        key,
        "NumpadEnter"
            | "Tab"
            | "Enter"
            | "Escape"
            | "Space"
            | "ArrowUp"
            | "ArrowDown"
            | "ArrowLeft"
            | "ArrowRight"
            | "Home"
            | "End"
            | "PageUp"
            | "PageDown"
            | "Insert"
            | "Delete"
            | "Backspace"
            | "Minus"
            | "Equal"
            | "BracketLeft"
            | "BracketRight"
            | "Backslash"
            | "Semicolon"
            | "Quote"
            | "Comma"
            | "Period"
            | "Slash"
            | "Backquote"
    ) || (key.starts_with("Key") && key.len() == 4 && key.as_bytes()[3].is_ascii_uppercase())
        || (key.starts_with("Digit") && key.len() == 6 && key.as_bytes()[5].is_ascii_digit())
        || key
            .strip_prefix('F')
            .and_then(|n| n.parse::<u8>().ok())
            .is_some_and(|n| (1..=12).contains(&n))
}
impl Bindings {
    /// Add newly introduced actions without replacing explicit unbound cells or
    /// stealing an existing user chord. Unknown action IDs remain invalid.
    pub fn fill_missing(&mut self) {
        for mut row in Self::default().rows {
            if self.rows.iter().any(|r| r.action == row.action) {
                continue;
            }
            if self
                .rows
                .iter()
                .any(|r| r.keyboard.is_some() && r.keyboard == row.keyboard)
            {
                row.keyboard = None;
            }
            self.rows.push(row);
        }
    }
    pub fn validate(&self) -> bool {
        self.version == 1
            && self.control < 128
            && self.rows.len() <= CATALOG.len()
            && self.rows.iter().all(|row| {
                CATALOG.iter().any(|(id, ..)| row.action == *id)
                    && self.rows.iter().filter(|r| r.action == row.action).count() == 1
            })
            && self.rows.iter().all(|r| {
                r.piano
                    .is_none_or(|n| n < 128 && n != self.control && !r.action.starts_with("note_"))
                    && r.keyboard.as_ref().is_none_or(|k| {
                        valid_key(&k.key) && (k.key != "Escape" || r.action == "nav_back")
                    })
            })
            && !self.rows.iter().enumerate().any(|(i, r)| {
                self.rows[i + 1..].iter().any(|other| {
                    (r.keyboard.is_some() && r.keyboard == other.keyboard)
                        || (r.piano.is_some() && r.piano == other.piano)
                })
            })
    }
}
pub fn pitch(key: u8) -> String {
    format!(
        "{}{}",
        [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"
        ][(key % 12) as usize],
        key as i16 / 12 - 1
    )
}
/// Prioritize modifier attacks only within consecutive attack groups. Releases
/// and non-note barriers preserve causal ordering for rapid repeated gestures.
pub fn batch_order(edges: &[(u8, bool)], control: u8) -> Vec<usize> {
    let mut order: Vec<_> = (0..edges.len()).collect();
    let mut start = 0;
    while start < edges.len() {
        if !edges[start].1 {
            start += 1;
            continue;
        }
        let mut end = start + 1;
        while end < edges.len() && edges[end].1 {
            end += 1;
        }
        order[start..end].sort_by_key(|&i| edges[i].0 != control);
        start = end;
    }
    order
}
#[derive(Default)]
pub struct PianoCommands {
    control_sources: BTreeSet<String>,
    consumed: BTreeSet<String>,
    armed: Option<(String, u64)>,
}
impl PianoCommands {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    pub fn armed(&self, now: u64) -> bool {
        !self.control_sources.is_empty()
            || self.armed.as_ref().is_some_and(|(_, until)| now <= *until)
    }
    /// Some(action) consumes a command; Some("") consumes a modifier/unmapped command.
    /// Same-device/channel identity is the MIDI source ID without its final key.
    pub fn event(
        &mut self,
        cfg: &Bindings,
        id: &str,
        key: u8,
        down: bool,
        now: u64,
    ) -> Option<String> {
        if !cfg.piano_enabled {
            return None;
        }
        let device = id.rsplit_once(':').map_or(id, |(prefix, _)| prefix);
        if !down {
            self.control_sources.remove(id);
            return self.consumed.remove(id).then(String::new);
        }
        if self.consumed.contains(id) {
            return Some(String::new());
        }
        if key == cfg.control {
            self.control_sources.insert(id.into());
            self.consumed.insert(id.into());
            self.armed = Some((device.into(), now.saturating_add(2_000_000)));
            return Some(String::new());
        }
        let held = self
            .control_sources
            .iter()
            .any(|s| s.rsplit_once(':').is_some_and(|(p, _)| p == device));
        let armed = self
            .armed
            .as_ref()
            .is_some_and(|(d, until)| d == device && now <= *until);
        if held || armed {
            self.armed = None;
            self.consumed.insert(id.into());
            return Some(
                cfg.rows
                    .iter()
                    .find(|r| r.piano == Some(key))
                    .map_or(String::new(), |r| r.action.clone()),
            );
        }
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn config() -> Bindings {
        let mut cfg = Bindings {
            piano_enabled: true,
            ..Default::default()
        };
        cfg.rows
            .iter_mut()
            .find(|r| r.action == "play")
            .unwrap()
            .piano = Some(60);
        cfg
    }
    #[test]
    fn validates_defaults_conflicts_and_control_collision() {
        let mut cfg = config();
        assert!(cfg.validate());
        cfg.rows[1].piano = Some(60);
        assert!(!cfg.validate());
        cfg.rows[1].piano = None;
        cfg.control = 60;
        assert!(!cfg.validate());
        cfg.control = 21;
        cfg.rows[1].keyboard = cfg.rows[0].keyboard.clone();
        assert!(!cfg.validate());
    }
    #[test]
    fn held_and_sequential_commands_consume_notes_once_and_isolate_devices() {
        let cfg = config();
        let mut state = PianoCommands::default();
        assert_eq!(
            state.event(&cfg, "midi:a:0:21", 21, true, 0),
            Some(String::new())
        );
        assert_eq!(state.event(&cfg, "midi:b:0:60", 60, true, 1), None);
        assert_eq!(
            state.event(&cfg, "midi:a:0:60", 60, true, 2),
            Some("play".into())
        );
        assert_eq!(
            state.event(&cfg, "midi:a:0:60", 60, true, 3),
            Some(String::new())
        );
        assert_eq!(
            state.event(&cfg, "midi:a:0:60", 60, false, 4),
            Some(String::new())
        );
        state.event(&cfg, "midi:a:0:21", 21, false, 5);
        assert_eq!(state.event(&cfg, "midi:a:0:60", 60, true, 6), None);
        state.event(&cfg, "midi:a:0:21", 21, true, 7);
        state.event(&cfg, "midi:a:0:21", 21, false, 8);
        assert_eq!(
            state.event(&cfg, "midi:a:0:60", 60, true, 9),
            Some("play".into())
        );
        state.reset();
        state.event(&cfg, "midi:a:0:21", 21, true, 10);
        state.event(&cfg, "midi:a:0:21", 21, false, 11);
        assert_eq!(state.event(&cfg, "midi:a:0:60", 60, true, 2_000_011), None);
    }
}

#[cfg(test)]
mod migration_tests {
    use super::*;
    #[test]
    fn adding_actions_preserves_unbound_cells_and_never_steals_keys() {
        let mut cfg = Bindings::default();
        let mut play = cfg
            .rows
            .iter()
            .find(|r| r.action == "play")
            .unwrap()
            .clone();
        play.keyboard = None;
        let mut speed = cfg
            .rows
            .iter()
            .find(|r| r.action == "faster")
            .unwrap()
            .clone();
        speed.keyboard = Some(KeyboardChord {
            key: "KeyV".into(),
            ctrl: false,
            shift: false,
            alt: false,
        });
        cfg.rows = vec![play, speed];
        assert!(cfg.validate());
        cfg.fill_missing();
        assert!(cfg.validate());
        assert!(
            cfg.rows
                .iter()
                .find(|r| r.action == "play")
                .unwrap()
                .keyboard
                .is_none()
        );
        assert!(
            cfg.rows
                .iter()
                .find(|r| r.action == "view")
                .unwrap()
                .keyboard
                .is_none()
        );
        cfg.rows[0].action = "unknown_action".into();
        assert!(!cfg.validate());
    }
    #[test]
    fn disabled_reset_and_unmapped_commands_never_leak_reserved_attacks() {
        let mut cfg = Bindings {
            piano_enabled: true,
            ..Default::default()
        };
        let mut input = PianoCommands::default();
        assert!(input.event(&cfg, "midi:a:0:21", 21, true, 1).is_some());
        assert_eq!(
            input.event(&cfg, "midi:a:0:61", 61, true, 2),
            Some(String::new())
        );
        assert!(input.event(&cfg, "midi:a:0:61", 61, false, 3).is_some());
        input.reset();
        assert!(!input.armed(4));
        assert_eq!(input.event(&cfg, "midi:a:0:61", 61, true, 5), None);
        cfg.piano_enabled = false;
        assert_eq!(input.event(&cfg, "midi:a:0:21", 21, true, 6), None);
        assert_eq!(pitch(60), "C4");
        assert_eq!(pitch(21), "A0");
    }
}

#[cfg(test)]
mod ordering_tests {
    #[test]
    fn simultaneous_reordering_never_crosses_note_offs() {
        let edges = [
            (60, true),
            (23, true),
            (60, false),
            (23, false),
            (23, true),
            (23, false),
            (60, true),
        ];
        assert_eq!(super::batch_order(&edges, 23), vec![1, 0, 2, 3, 4, 5, 6]);
    }
}
