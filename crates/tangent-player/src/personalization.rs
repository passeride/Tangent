//! Versioned portable appearance and source-specific accepted hand annotations.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
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
/// A lightweight, non-persistent starting point for unconfigured hand colours.
/// Authored MusicXML staff clefs take priority; MIDI or unlinked performance
/// notes use middle C (MIDI 60). These are suggestions, not observed hands.
pub fn suggested_hands(song: &tangent_core::music::Song) -> BTreeMap<usize, Hand> {
    use tangent_core::notation::Clef;

    let mut authored = BTreeMap::new();
    let mut conflicts = BTreeSet::new();
    if let Some(score) = &song.notation {
        let staff_hands: BTreeMap<_, _> = score
            .staves
            .iter()
            .map(|s| {
                (
                    s.id,
                    match s.clef {
                        Clef::Treble => Hand::Right,
                        Clef::Bass => Hand::Left,
                    },
                )
            })
            .collect();
        for written in score.measures.iter().flat_map(|m| &m.notes) {
            if written.pitch.is_none() {
                continue;
            }
            if let (Some(id), Some(&hand)) =
                (written.performance_note, staff_hands.get(&written.staff))
            {
                if authored.insert(id, hand).is_some_and(|old| old != hand) {
                    conflicts.insert(id);
                }
            }
        }
    }

    song.notes
        .iter()
        .filter(|note| !song.parts.get(note.part).is_some_and(|p| p.percussion))
        .map(|note| {
            let hand = if conflicts.contains(&note.id) {
                None
            } else {
                authored.get(&note.id).copied()
            }
            .unwrap_or(if note.key >= 60 {
                Hand::Right
            } else {
                Hand::Left
            });
            (note.id, hand)
        })
        .collect()
}

/// Any explicit choice (including Unassigned) overrides the suggested hand.
pub fn resolved_hand(
    profile: Option<&HandProfile>,
    suggested: &BTreeMap<usize, Hand>,
    id: usize,
) -> Hand {
    profile
        .and_then(|p| p.notes.get(&id).copied())
        .or_else(|| suggested.get(&id).copied())
        .unwrap_or(Hand::Unassigned)
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

#[cfg(test)]
mod suggested_hand_tests {
    use super::*;
    use tangent_core::notation::Clef;

    fn sample() -> tangent_core::music::Song {
        crate::import::song(
            include_bytes!("../../../web/piano/samples/first-light.musicxml"),
            "Sample".into(),
        )
        .unwrap()
    }

    #[test]
    fn written_clefs_take_priority_over_pitch() {
        let mut song = sample();
        let score = song.notation.as_ref().unwrap();
        let staff: BTreeMap<_, _> = score
            .staves
            .iter()
            .map(|s| {
                (
                    s.id,
                    match s.clef {
                        Clef::Treble => Hand::Right,
                        Clef::Bass => Hand::Left,
                    },
                )
            })
            .collect();
        // Confirm both clefs are represented by linked performed notes.
        let mut checked = BTreeSet::new();
        let suggested = suggested_hands(&song);
        for written in score.measures.iter().flat_map(|m| &m.notes) {
            if let (Some(id), Some(&hand)) = (written.performance_note, staff.get(&written.staff))
                && written.pitch.is_some()
                && !score.measures.iter().flat_map(|m| &m.notes).any(|other| {
                    other.performance_note == Some(id)
                        && other.pitch.is_some()
                        && staff.get(&other.staff).is_some_and(|h| *h != hand)
                })
            {
                assert_eq!(suggested.get(&id), Some(&hand));
                checked.insert(hand.label());
            }
        }
        assert!(checked.contains("Left") && checked.contains("Right"));
        // Without notation, the same song must use a middle-C split.
        song.notation = None;
        let fallback = suggested_hands(&song);
        for note in &song.notes {
            assert_eq!(
                fallback.get(&note.id),
                Some(&if note.key >= 60 {
                    Hand::Right
                } else {
                    Hand::Left
                })
            );
        }
    }

    #[test]
    fn explicit_override_including_unassigned_wins() {
        let suggested = BTreeMap::from([(3, Hand::Right), (4, Hand::Left)]);
        let profile = HandProfile {
            mapping_version: 1,
            revision: 1,
            notes: BTreeMap::from([(3, Hand::Unassigned), (4, Hand::Right)]),
        };
        assert_eq!(resolved_hand(None, &suggested, 3), Hand::Right);
        assert_eq!(
            resolved_hand(Some(&profile), &suggested, 3),
            Hand::Unassigned
        );
        assert_eq!(resolved_hand(Some(&profile), &suggested, 4), Hand::Right);
        assert_eq!(
            resolved_hand(Some(&profile), &suggested, 99),
            Hand::Unassigned
        );
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
