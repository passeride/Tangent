//! Versioned portable appearance and source-specific accepted hand annotations.
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tokens {
    pub bg: [u8; 4],
    pub panel: [u8; 4],
    pub ink: [u8; 4],
    pub muted: [u8; 4],
    pub staff: [u8; 4],
    pub white: [u8; 4],
    pub black: [u8; 4],
    pub accent_1: [u8; 4],
    pub accent_2: [u8; 4],
    pub accent_3: [u8; 4],
    pub accent_4: [u8; 4],
    pub hand_left: [u8; 4],
    pub hand_right: [u8; 4],
    pub hand_either: [u8; 4],
    pub hand_unassigned: [u8; 4],
}
impl Tokens {
    pub const LABELS: [&'static str; 15] = [
        "Background",
        "Panels",
        "Text",
        "Secondary text",
        "Staff lines",
        "White keys",
        "Black keys",
        "Accent 1",
        "Accent 2",
        "Accent 3",
        "Accent 4",
        "Left hand",
        "Right hand",
        "Either hand",
        "Unassigned",
    ];
    pub fn values(&self) -> [[u8; 4]; 15] {
        [
            self.bg,
            self.panel,
            self.ink,
            self.muted,
            self.staff,
            self.white,
            self.black,
            self.accent_1,
            self.accent_2,
            self.accent_3,
            self.accent_4,
            self.hand_left,
            self.hand_right,
            self.hand_either,
            self.hand_unassigned,
        ]
    }
    pub fn from_values(v: [[u8; 4]; 15]) -> Self {
        Self {
            bg: v[0],
            panel: v[1],
            ink: v[2],
            muted: v[3],
            staff: v[4],
            white: v[5],
            black: v[6],
            accent_1: v[7],
            accent_2: v[8],
            accent_3: v[9],
            accent_4: v[10],
            hand_left: v[11],
            hand_right: v[12],
            hand_either: v[13],
            hand_unassigned: v[14],
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeFile {
    pub schema_version: u8,
    pub name: String,
    pub tokens: Tokens,
}
impl ThemeFile {
    pub fn validate(&self) -> bool {
        self.schema_version == 1
            && !self.name.trim().is_empty()
            && self.name.len() <= 128
            && !self.name.chars().any(char::is_control)
    }
    pub fn parse(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() > 65536 {
            return Err("Theme exceeds 64 KiB".into());
        }
        let v: Self = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
        if !v.validate() {
            return Err("Unsupported or invalid theme".into());
        }
        Ok(v)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Hand {
    Left,
    Right,
    Either,
    Unassigned,
}
impl Hand {
    pub fn next(self) -> Self {
        match self {
            Self::Unassigned => Self::Left,
            Self::Left => Self::Right,
            Self::Right => Self::Either,
            Self::Either => Self::Unassigned,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Left => "Left",
            Self::Right => "Right",
            Self::Either => "Either",
            Self::Unassigned => "Unassigned",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandProfile {
    pub mapping_version: u8,
    pub revision: u64,
    pub notes: BTreeMap<usize, Hand>,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Personalization {
    #[serde(default)]
    pub use_custom: bool,
    pub theme: Option<ThemeFile>,
    pub hands: BTreeMap<String, std::sync::Arc<HandProfile>>,
}
impl Personalization {
    pub fn validate(&self) -> bool {
        (!self.use_custom || self.theme.is_some())
            && self.theme.as_ref().is_none_or(ThemeFile::validate)
            && self.hands.len() <= 128
            && self.hands.iter().all(|(source, p)| {
                source.len() == 64
                    && source.bytes().all(|b| b.is_ascii_hexdigit())
                    && p.mapping_version == 1
                    && p.notes.len() <= 250000
            })
    }
}
/// Conservative demonstration alignment: accept only a uniquely compatible onset.
/// Ambiguous repeated/unison attacks remain unresolved, never assigned by pitch split.
pub fn match_attack(song: &tangent_core::music::Song, at: u64, key: u8) -> Option<usize> {
    let start = song
        .notes
        .partition_point(|n| n.start_us < at.saturating_sub(200000));
    let end = song
        .notes
        .partition_point(|n| n.start_us <= at.saturating_add(200000));
    let mut hits = song.notes[start..end].iter().filter(|n| {
        n.key == key && n.start_us.abs_diff(at) <= 200000 && !song.parts[n.part].percussion
    });
    let first = hits.next()?;
    hits.next().is_none().then_some(first.id)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strict_theme() {
        let v = Tokens::from_values([[123, 45, 67, 89]; 15]);
        let f = ThemeFile {
            schema_version: 1,
            name: "Custom".into(),
            tokens: v,
        };
        let bytes = serde_json::to_vec(&f).unwrap();
        assert_eq!(ThemeFile::parse(&bytes).unwrap(), f);
        assert!(ThemeFile::parse(&vec![b' '; 65537]).is_err());
        assert!(ThemeFile::parse(br#"{"schema_version":1,"schema_version":1}"#).is_err());
    }
    #[test]
    fn profile_source_validation() {
        let mut p = Personalization::default();
        p.hands.insert(
            "bad".into(),
            std::sync::Arc::new(HandProfile {
                mapping_version: 1,
                revision: 1,
                notes: BTreeMap::new(),
            }),
        );
        assert!(!p.validate());
    }
}

#[cfg(test)]
mod demonstration_tests {
    use super::*;
    #[test]
    fn repeated_pitch_abstains_and_profiles_round_trip() {
        let mut song = crate::import::song(
            include_bytes!("../../../web/piano/samples/first-light.musicxml"),
            "Test".into(),
        )
        .unwrap();
        let n = song.notes[0].clone();
        song.notes = vec![n.clone()];
        assert_eq!(match_attack(&song, n.start_us, n.key), Some(n.id));
        let mut duplicate = n.clone();
        duplicate.id += 1;
        song.notes.push(duplicate);
        assert_eq!(match_attack(&song, n.start_us, n.key), None);
        assert_eq!(match_attack(&song, n.start_us + 500000, n.key), None);
        let mut p = Personalization::default();
        p.hands.insert(
            "a".repeat(64),
            std::sync::Arc::new(HandProfile {
                mapping_version: 1,
                revision: 1,
                notes: BTreeMap::from([(n.id, Hand::Left)]),
            }),
        );
        let restored: Personalization =
            serde_json::from_str(&serde_json::to_string(&p).unwrap()).unwrap();
        assert_eq!(p, restored);
        assert!(restored.validate());
    }
}

pub fn contrast(ink: [u8; 4], bg: [u8; 4]) -> f32 {
    fn luminance(c: [f32; 3]) -> f32 {
        let c = c.map(|v| {
            if v <= 0.04045 {
                v / 12.92
            } else {
                ((v + 0.055) / 1.055).powf(2.4)
            }
        });
        c[0] * 0.2126 + c[1] * 0.7152 + c[2] * 0.0722
    }
    let base = [bg[0], bg[1], bg[2]].map(|v| v as f32 / 255.);
    let alpha = ink[3] as f32 / 255.;
    let fg = std::array::from_fn(|i| ink[i] as f32 / 255. * alpha + base[i] * (1. - alpha));
    let a = luminance(fg);
    let b = luminance(base);
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}
