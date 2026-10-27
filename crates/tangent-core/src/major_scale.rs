//! S03 one-octave major scales. Finger numbers are instructions, not measurements.
use crate::practice::Level;
const OFFSETS: [u8; 15] = [0, 2, 4, 5, 7, 9, 11, 12, 11, 9, 7, 5, 4, 2, 0];
// Circle order uses readable sharp/flat spellings, including E# in F# major.
const ROOTS: [u8; 12] = [60, 67, 62, 69, 64, 71, 66, 61, 68, 63, 70, 65];
const NAMES: [&str; 12] = [
    "C", "G", "D", "A", "E", "B", "F#", "Db", "Ab", "Eb", "Bb", "F",
];
const SPELLINGS: [[&str; 8]; 12] = [
    ["C", "D", "E", "F", "G", "A", "B", "C"],
    ["G", "A", "B", "C", "D", "E", "F#", "G"],
    ["D", "E", "F#", "G", "A", "B", "C#", "D"],
    ["A", "B", "C#", "D", "E", "F#", "G#", "A"],
    ["E", "F#", "G#", "A", "B", "C#", "D#", "E"],
    ["B", "C#", "D#", "E", "F#", "G#", "A#", "B"],
    ["F#", "G#", "A#", "B", "C#", "D#", "E#", "F#"],
    ["Db", "Eb", "F", "Gb", "Ab", "Bb", "C", "Db"],
    ["Ab", "Bb", "C", "Db", "Eb", "F", "G", "Ab"],
    ["Eb", "F", "G", "Ab", "Bb", "C", "D", "Eb"],
    ["Bb", "C", "D", "Eb", "F", "G", "A", "Bb"],
    ["F", "G", "A", "Bb", "C", "D", "E", "F"],
];
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MajorScaleTask {
    pub notes: Vec<u8>,
    pub left: bool,
    pub key: usize,
}
impl MajorScaleTask {
    pub fn generate(level: Level, index: usize) -> Self {
        let key = if level == Level::Chromatic {
            index % 12
        } else {
            0
        };
        let left = level == Level::Mixed || level == Level::Chromatic && index / 12 % 2 == 1;
        let root = ROOTS[key] - if left { 12 } else { 0 };
        Self {
            notes: OFFSETS.iter().map(|n| root + n).collect(),
            left,
            key,
        }
    }
    pub fn name(&self) -> &'static str {
        NAMES[self.key]
    }
    pub fn note_label(&self, index: usize) -> String {
        let degree = if index <= 7 { index } else { 14 - index };
        // MIDI's octave number changes at C even in flat/sharp keys.
        format!(
            "{}{}",
            SPELLINGS[self.key][degree],
            self.notes[index] / 12 - 1
        )
    }
    pub fn fingers(&self) -> Vec<u8> {
        let up = if self.left {
            match self.key {
                5 => [4, 3, 2, 1, 4, 3, 2, 1],
                6 => [4, 3, 2, 1, 3, 2, 1, 4],
                7..=10 => [3, 2, 1, 4, 3, 2, 1, 3],
                _ => [5, 4, 3, 2, 1, 3, 2, 1],
            }
        } else {
            match self.key {
                6 => [2, 3, 4, 1, 2, 3, 1, 2],
                7 => [2, 3, 1, 2, 3, 4, 1, 2],
                8 => [3, 4, 1, 2, 3, 1, 2, 3],
                9 => [3, 1, 2, 3, 4, 1, 2, 3],
                10 => [2, 1, 2, 3, 1, 2, 3, 4],
                11 => [1, 2, 3, 4, 1, 2, 3, 4],
                _ => [1, 2, 3, 1, 2, 3, 4, 5],
            }
        };
        up.into_iter()
            .chain(up[..7].iter().rev().copied())
            .collect()
    }
    pub fn skill(&self) -> String {
        format!("scale:1:{}:{}", if self.left { "L" } else { "R" }, self.key)
    }
    pub fn from_skill(s: &str) -> Option<Self> {
        let f: Vec<_> = s.split(':').collect();
        if f.len() != 4 || f[0] != "scale" || f[1] != "1" || !matches!(f[2], "L" | "R") {
            return None;
        }
        let key: usize = f[3].parse().ok()?;
        if key >= 12 || f[3] != key.to_string() {
            return None;
        }
        Some(Self::generate(
            Level::Chromatic,
            key + if f[2] == "L" { 12 } else { 0 },
        ))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{practice::Practice, progress::Training};
    #[test]
    fn all_keys_hands_spelling_and_fingers() {
        let mut roots = std::collections::BTreeSet::new();
        for i in 0..24 {
            let t = MajorScaleTask::generate(Level::Chromatic, i);
            roots.insert(t.notes[0] % 12);
            assert_eq!(t.notes.len(), 15);
            assert_eq!(t.notes[7] - t.notes[0], 12);
            assert_eq!(t.notes[0], t.notes[14]);
            assert_eq!(t.fingers().len(), 15);
            assert_eq!(MajorScaleTask::from_skill(&t.skill()), Some(t));
        }
        assert_eq!(roots.len(), 12);
        assert_eq!(
            MajorScaleTask::generate(Level::Chromatic, 6).note_label(6),
            "E#5"
        );
        assert_eq!(
            MajorScaleTask::generate(Level::Chromatic, 7).note_label(3),
            "Gb4"
        );
        for bad in ["scale:2:R:0", "scale:1:X:0", "scale:1:R:12", "scale:1:L:01"] {
            assert!(MajorScaleTask::from_skill(bad).is_none());
        }
    }
    #[test]
    fn every_key_can_be_selected_for_each_hand_and_scored_cleanly() {
        use crate::practice::ReadingClef;
        for clef in [ReadingClef::Treble, ReadingClef::Bass, ReadingClef::Mixed] {
            let mut p = Practice::major_scale();
            p.configure(clef, Level::Chromatic);
            let mut t = Training::default();
            t.adaptive = true;
            t.start("scale-clean-session".into(), &p);
            for index in 0..12 {
                let plan = p.scale_task().unwrap();
                assert_eq!(plan.key, index);
                assert_eq!(
                    plan.left,
                    clef == ReadingClef::Bass || clef == ReadingClef::Mixed && index % 2 == 1
                );
                for _ in 0..15 {
                    assert!(p.answer(&p.target()));
                    t.answer(&p, false);
                    p.answer(&[]);
                }
                t.ready();
                t.choose(&mut p);
                assert!(p.review_target.is_none());
            }
            let mut run = t.run.unwrap();
            run.close(true);
            assert!(run.validate());
            assert_eq!(run.points, 1200);
        }
    }
    #[test]
    fn whole_scale_scoring_release_skip_and_sticky_assistance() {
        let mut p = Practice::major_scale();
        let mut t = Training::default();
        t.start("scale-session".into(), &p);
        for i in 0..15 {
            if i == 2 {
                assert!(!p.answer(&[127]));
                t.mistake();
                p.answer(&[]);
            }
            if i == 4 {
                t.assist();
            }
            assert!(p.answer(&p.target()));
            t.answer(&p, false);
            assert_eq!(t.run.as_ref().unwrap().trials.len(), usize::from(i == 14));
            assert!(!p.answer(&p.target()));
            p.answer(&[]);
        }
        assert_eq!(p.completed, 1);
        assert_eq!(t.run.as_ref().unwrap().points, 0);
        assert!(t.run.as_ref().unwrap().validate());
        p.answer(&p.target());
        p.answer(&[]);
        t.answer(&p, true);
        p.skip();
        assert_eq!(p.sequence_step(), 0);
        assert_eq!(p.completed, 2);
    }
}
