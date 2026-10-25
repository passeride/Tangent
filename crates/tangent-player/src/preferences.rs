//! Versioned user preferences; no platform/storage dependency.
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    pub version: u32,
    pub view: u8,
    pub theme: usize,
    pub effects: bool,
    pub volume: f32,
    pub instrument: usize,
    pub hear: bool,
    pub first_key: u8,
    pub last_key: u8,
    /// Stable device name rather than a transient OS port index; optional for v1 data.
    #[serde(default)]
    pub midi_output_name: Option<String>,
}
impl Preferences {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.version != 1 {
            return Err("Unsupported settings version");
        }
        if self.view > 3 || self.theme > 2 || self.instrument > 1 {
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
            view: 3,
            theme: 2,
            effects: false,
            volume: 0.25,
            instrument: 1,
            hear: false,
            first_key: 36,
            last_key: 84,
            midi_output_name: Some("Yamaha P-45".into()),
        }
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
        let old: Preferences = serde_json::from_value(legacy).unwrap();
        assert_eq!(old.midi_output_name, None);
        assert!(old.validate().is_ok());
        for bad in [
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
