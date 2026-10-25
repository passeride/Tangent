//! Small, renderer-independent note/chord exercises. Exact keys, any inversion.
#[derive(Debug, Default)]
pub struct Practice {
    pub chords: bool,
    pub completed: usize,
    waiting_release: bool,
}
impl Practice {
    pub fn new(chords: bool) -> Self {
        Self {
            chords,
            ..Default::default()
        }
    }
    pub fn target(&self) -> Vec<u8> {
        let root = [60, 62, 64, 65, 67, 69, 71][self.completed % 7];
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
    pub fn label(&self) -> String {
        let name = ["C", "D", "E", "F", "G", "A", "B"][self.completed % 7];
        if self.chords {
            format!(
                "{} {}",
                name,
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
            format!("{}4", name)
        }
    }
    pub fn answer(&mut self, held: &[u8]) -> bool {
        if held.is_empty() {
            self.waiting_release = false;
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
            self.completed += 1;
            self.waiting_release = true;
            true
        } else {
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
