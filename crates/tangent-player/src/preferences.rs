//! Versioned user preferences; no platform/storage dependency.
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    pub version: u32,
    #[serde(default)]
    pub personalization: crate::personalization::Personalization,
    #[serde(default)]
    pub bindings: tangent_core::input_bindings::Bindings,
    pub view: u8,
    #[serde(default = "default_hud")]
    pub hud: u8,
    pub theme: usize,
    pub effects: bool,
    #[serde(default)]
    pub reduced_motion: bool,
    #[serde(default = "default_effect_strength")]
    pub effect_strength: u8,
    #[serde(default)]
    pub practice_clef: u8,
    #[serde(default)]
    pub practice_level: u8,
    #[serde(default)]
    pub adaptive_practice: bool,
    pub volume: f32,
    pub instrument: usize,
    pub hear: bool,
    #[serde(default = "default_speed")]
    pub speed_percent: u16,
    #[serde(default)]
    pub practice_hear: bool,
    pub first_key: u8,
    pub last_key: u8,
    /// Stable device name rather than a transient OS port index; optional for v1 data.
    #[serde(default)]
    pub midi_output_name: Option<String>,
}
fn default_hud() -> u8 {
    1
}
fn default_speed() -> u16 {
    100
}
fn default_effect_strength() -> u8 {
    1
}
impl Preferences {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.personalization.validate() {
            return Err("Invalid personal appearance/hand data");
        }
        if !self.bindings.validate() {
            return Err("Invalid input bindings");
        }
        if self.version != 1 {
            return Err("Unsupported settings version");
        }
        if tangent_core::transport::PlaybackRate::new(self.speed_percent).is_none() {
            return Err("Invalid playback speed");
        }
        if self.hud > 2
            || self.view > 3
            || self.theme > 2
            || self.instrument > 1
            || self.effect_strength > 2
            || self.practice_clef > 2
            || self.practice_level > 2
        {
            return Err("Invalid settings choice");
        }
        if !self.volume.is_finite() || !(0. ..=1.).contains(&self.volume) {
            return Err("Invalid volume");
        }
        if self.midi_output_name.as_ref().is_some_and(|name| {
            name.trim().is_empty() || name.len() > 256 || name.chars().any(char::is_control)
        }) {
            return Err("Invalid MIDI output preference");
        }
        if !matches!(
            (self.first_key, self.last_key),
            (21, 108) | (36, 84) | (36, 96)
        ) {
            return Err("Invalid piano range");
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn settings() -> Preferences {
        Preferences {
            version: 1,
            personalization: Default::default(),
            bindings: Default::default(),
            view: 3,
            hud: 2,
            theme: 2,
            effects: false,
            reduced_motion: false,
            effect_strength: 1,
            practice_clef: 0,
            practice_level: 0,
            adaptive_practice: false,
            volume: 0.25,
            instrument: 1,
            hear: false,
            speed_percent: 70,
            practice_hear: true,
            first_key: 36,
            last_key: 84,
            midi_output_name: Some("Yamaha P-45".into()),
        }
    }
    #[test]
    fn bindings_round_trip_and_conflicts_are_rejected() {
        let mut p = settings();
        p.bindings.piano_enabled = true;
        p.bindings.control = 108;
        p.bindings
            .rows
            .iter_mut()
            .find(|r| r.action == "play")
            .unwrap()
            .piano = Some(60);
        let restored: Preferences =
            serde_json::from_str(&serde_json::to_string(&p).unwrap()).unwrap();
        assert_eq!(p, restored);
        assert!(restored.validate().is_ok());
        p.bindings
            .rows
            .iter_mut()
            .find(|r| r.action == "faster")
            .unwrap()
            .piano = Some(60);
        assert!(p.validate().is_err());
    }
    #[test]
    fn settings_round_trip_and_invalid_values() {
        let p = settings();
        let restored: Preferences =
            serde_json::from_str(&serde_json::to_string(&p).unwrap()).unwrap();
        assert_eq!(p, restored);
        assert!(restored.validate().is_ok());
        // Legacy JSON without MIDI output preference is valid.
        let mut legacy = serde_json::to_value(settings()).unwrap();
        legacy.as_object_mut().unwrap().remove("midi_output_name");
        legacy.as_object_mut().unwrap().remove("reduced_motion");
        legacy.as_object_mut().unwrap().remove("effect_strength");
        legacy.as_object_mut().unwrap().remove("practice_clef");
        legacy.as_object_mut().unwrap().remove("practice_level");
        legacy.as_object_mut().unwrap().remove("adaptive_practice");
        legacy.as_object_mut().unwrap().remove("practice_hear");
        legacy.as_object_mut().unwrap().remove("speed_percent");
        legacy.as_object_mut().unwrap().remove("hud");
        legacy.as_object_mut().unwrap().remove("bindings");
        legacy.as_object_mut().unwrap().remove("personalization");
        let old: Preferences = serde_json::from_value(legacy).unwrap();
        assert_eq!(old.midi_output_name, None);
        assert!(!old.reduced_motion);
        assert!(!old.practice_hear);
        assert_eq!(old.speed_percent, 100);
        assert_eq!(old.hud, 1);
        assert_eq!(old.bindings, Default::default());
        assert_eq!(old.effect_strength, 1);
        assert_eq!(old.practice_clef, 0);
        assert_eq!(old.practice_level, 0);
        assert!(old.validate().is_ok());
        for bad in [
            Preferences {
                hud: 3,
                ..settings()
            },
            Preferences {
                speed_percent: 0,
                ..settings()
            },
            Preferences {
                speed_percent: 201,
                ..settings()
            },
            Preferences {
                practice_clef: 3,
                ..settings()
            },
            Preferences {
                practice_level: 3,
                ..settings()
            },
            Preferences {
                effect_strength: 3,
                ..settings()
            },
            Preferences {
                midi_output_name: Some("".into()),
                ..settings()
            },
            Preferences {
                version: 2,
                ..settings()
            },
            Preferences {
                volume: f32::NAN,
                ..settings()
            },
            Preferences {
                volume: 2.,
                ..settings()
            },
            Preferences {
                theme: 3,
                ..settings()
            },
            Preferences {
                view: 4,
                ..settings()
            },
            Preferences {
                instrument: 2,
                ..settings()
            },
            Preferences {
                first_key: 0,
                ..settings()
            },
        ] {
            assert!(bad.validate().is_err());
        }
    }
}
