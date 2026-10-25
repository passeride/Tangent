//! Renderer-independent reading exercises. Exact octaves; chords accept inversions.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ReadingClef {
    #[default]
    Treble,
    Bass,
    Mixed,
}
impl ReadingClef {
    pub fn next(self) -> Self {
        match self {
            Self::Treble => Self::Bass,
            Self::Bass => Self::Mixed,
            Self::Mixed => Self::Treble,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Treble => "Treble",
            Self::Bass => "Bass",
            Self::Mixed => "Both clefs",
        }
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    #[default]
    Stepwise,
    Mixed,
    Chromatic,
}
impl Level {
    pub fn next(self) -> Self {
        match self {
            Self::Stepwise => Self::Mixed,
            Self::Mixed => Self::Chromatic,
            Self::Chromatic => Self::Stepwise,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Stepwise => "Stepwise",
            Self::Mixed => "Mixed notes",
            Self::Chromatic => "Chromatic",
        }
    }
}
#[derive(Debug, Default)]
pub struct Practice {
    pub chords: bool,
    pub completed: usize,
    pub clef: ReadingClef,
    pub level: Level,
    pub hint: bool,
    pub waiting_release: bool,
    pub wrong: bool,
}
impl Practice {
    pub fn new(chords: bool) -> Self {
        Self {
            chords,
            ..Default::default()
        }
    }
    pub fn target(&self) -> Vec<u8> {
        let root = if self.chords {
            [60, 62, 64, 65, 67, 69, 71][self.completed % 7]
        } else {
            self.note_target()
        };
        if self.chords {
            vec![
                root,
                root + if [62, 64, 69, 71].contains(&root) {
                    3
                } else {
                    4
                },
                root + if root == 71 { 6 } else { 7 },
            ]
        } else {
            vec![root]
        }
    }
    pub fn bass(&self) -> bool {
        match self.clef {
            ReadingClef::Treble => false,
            ReadingClef::Bass => true,
            ReadingClef::Mixed => self.completed % 2 == 1,
        }
    }
    fn note_target(&self) -> u8 {
        // Fixed permutations are reproducible and cover every pitch before repeating.
        // No random state in the renderer; mixed clefs each traverse their own range.
        let i = if self.clef == ReadingClef::Mixed {
            self.completed / 2
        } else {
            self.completed
        };
        let base = if self.bass() { 48 } else { 60 };
        base + match self.level {
            Level::Stepwise => [0, 2, 4, 5, 7, 9, 11, 12][i % 8],
            Level::Mixed => [0, 7, 2, 11, 4, 12, 5, 9][i % 8],
            Level::Chromatic => ((i * 5) % 13) as u8,
        }
    }
    pub fn label(&self) -> String {
        if self.chords {
            format!(
                "{} {}",
                ["C", "D", "E", "F", "G", "A", "B"][self.completed % 7],
                [
                    "major",
                    "minor",
                    "minor",
                    "major",
                    "major",
                    "minor",
                    "diminished"
                ][self.completed % 7]
            )
        } else {
            let key = self.note_target();
            format!(
                "{}{}",
                [
                    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"
                ][key as usize % 12],
                key / 12 - 1
            )
        }
    }
    pub fn configure(&mut self, clef: ReadingClef, level: Level) {
        *self = Self {
            clef,
            level,
            chords: self.chords,
            ..Default::default()
        };
    }
    pub fn answer(&mut self, held: &[u8]) -> bool {
        if held.is_empty() {
            if self.waiting_release {
                self.completed += 1;
                self.hint = false;
            }
            self.waiting_release = false;
            self.wrong = false;
            return false;
        }
        if self.waiting_release {
            return false;
        }
        let mut actual: Vec<_> = held
            .iter()
            .map(|k| if self.chords { k % 12 } else { *k })
            .collect();
        let mut expected: Vec<_> = self
            .target()
            .iter()
            .map(|k| if self.chords { k % 12 } else { *k })
            .collect();
        actual.sort_unstable();
        actual.dedup();
        expected.sort_unstable();
        expected.dedup();
        if actual == expected {
            self.waiting_release = true;
            self.wrong = false;
            true
        } else {
            self.wrong = true;
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn notes_require_correct_octave_and_release() {
        let mut p = Practice::default();
        assert!(!p.answer(&[72]));
        assert!(p.answer(&[60]));
        assert!(!p.answer(&[62]));
        p.answer(&[]);
        assert!(p.answer(&[62]));
    }
    #[test]
    fn chords_require_all_and_only_target_pitch_classes() {
        let mut p = Practice {
            chords: true,
            ..Default::default()
        };
        assert!(!p.answer(&[60, 64]));
        assert!(!p.answer(&[60, 64, 67, 70]));
        assert!(p.answer(&[64, 67, 72]));
        p.answer(&[]);
        assert!(!p.answer(&[62, 66, 69]));
        assert!(p.answer(&[62, 65, 69]));
    }
}

#[cfg(test)]
mod reading_tests {
    use super::*;
    #[test]
    fn correct_target_remains_until_full_release() {
        let mut p = Practice::default();
        assert!(p.answer(&[60]));
        assert_eq!(p.target(), vec![60]);
        assert_eq!(p.completed, 0);
        assert!(!p.answer(&[60, 62]));
        assert_eq!(p.completed, 0);
        p.answer(&[]);
        assert_eq!(p.target(), vec![62]);
        assert_eq!(p.completed, 1);
    }
    #[test]
    fn bass_mixed_and_chromatic_cover_the_selected_range() {
        let mut p = Practice::default();
        p.configure(ReadingClef::Bass, Level::Chromatic);
        let mut keys = std::collections::BTreeSet::new();
        for i in 0..13 {
            p.completed = i;
            keys.insert(p.target()[0]);
            assert!(p.bass());
        }
        assert_eq!(keys, (48..=60).collect());
        p.configure(ReadingClef::Mixed, Level::Stepwise);
        assert_eq!(p.target(), vec![60]);
        p.completed = 1;
        assert_eq!(p.target(), vec![48]);
        p.completed = 2;
        assert_eq!(p.target(), vec![62]);
        p.completed = 3;
        assert_eq!(p.target(), vec![50]);
    }
    #[test]
    fn wrong_octave_extra_keys_and_repeated_held_answers_do_not_advance() {
        let mut p = Practice::default();
        assert!(!p.answer(&[48]));
        assert!(p.wrong);
        assert!(!p.answer(&[60, 61]));
        assert_eq!(p.completed, 0);
        p.answer(&[]);
        assert!(p.answer(&[60]));
        assert!(!p.answer(&[60]));
        p.answer(&[]);
        assert_eq!(p.completed, 1);
    }
}
