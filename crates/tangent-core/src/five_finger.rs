//! Original T01 major five-finger patterns. Fingering is advice, not MIDI evidence.
use crate::practice::Level;
const OFFSETS: [u8; 9] = [0, 2, 4, 5, 7, 5, 4, 2, 0];
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiveFingerTask {
    pub notes: Vec<u8>,
    pub left: bool,
}
impl FiveFingerTask {
    pub fn generate(level: Level, index: usize) -> Self {
        let root = match level {
            Level::Stepwise => 60,
            Level::Mixed => 48,
            Level::Chromatic => [60, 55, 65, 48, 67, 53][index % 6],
        };
        Self {
            notes: OFFSETS.iter().map(|n| root + n).collect(),
            left: root < 60,
        }
    }
    pub fn note_label(&self, index: usize) -> String {
        let key = self.notes[index];
        if self.notes[0] % 12 == 5 && key % 12 == 10 {
            format!("Bb{}", i16::from(key) / 12 - 1)
        } else {
            crate::explorer::note_name(key)
        }
    }
    pub fn fingers(&self) -> [u8; 9] {
        if self.left {
            [5, 4, 3, 2, 1, 2, 3, 4, 5]
        } else {
            [1, 2, 3, 4, 5, 4, 3, 2, 1]
        }
    }
    pub fn skill(&self) -> String {
        format!(
            "flow:1:{}:{}",
            if self.left { "L" } else { "R" },
            self.notes
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join("-")
        )
    }
    pub fn from_skill(s: &str) -> Option<Self> {
        let f: Vec<_> = s.split(':').collect();
        if f.len() != 4 || f[0] != "flow" || f[1] != "1" || !matches!(f[2], "L" | "R") {
            return None;
        }
        let tokens: Vec<_> = f[3].split('-').collect();
        if tokens.len() != 9
            || tokens
                .iter()
                .any(|v| v.is_empty() || !v.bytes().all(|c| c.is_ascii_digit()))
        {
            return None;
        }
        let notes: Vec<u8> = tokens
            .iter()
            .map(|n| n.parse())
            .collect::<Result<_, _>>()
            .ok()?;
        let root = notes[0];
        if root > 120
            || !notes
                .iter()
                .zip(OFFSETS)
                .all(|(&note, offset)| note == root + offset)
        {
            return None;
        }
        Some(Self {
            notes,
            left: f[2] == "L",
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plans_are_bounded_deterministic_and_retain_fingering_evidence() {
        for level in [Level::Stepwise, Level::Mixed, Level::Chromatic] {
            for i in 0..12 {
                let t = FiveFingerTask::generate(level, i);
                assert_eq!(t, FiveFingerTask::generate(level, i));
                assert_eq!(t.notes.len(), 9);
                assert!(t.notes.iter().all(|k| (36..=84).contains(k)));
                assert_eq!(FiveFingerTask::from_skill(&t.skill()), Some(t.clone()));
                assert_eq!(t.fingers()[0], if t.left { 5 } else { 1 });
            }
        }
        for bad in [
            "flow:2:R:60-62-64-65-67-65-64-62-60",
            "flow:1:R:60-62-64",
            "flow:1:R:125-127-129-130-132-130-129-127-125",
            "flow:1:X:60-62-64-65-67-65-64-62-60",
        ] {
            assert!(FiveFingerTask::from_skill(bad).is_none());
        }
    }
}

#[cfg(test)]
mod scoring_tests {
    use super::*;
    use crate::{
        practice::Practice,
        progress::{Training, skill_label},
    };
    fn complete(p: &mut Practice, t: &mut Training, mistake: bool, assist: bool) {
        let before = t.run.as_ref().unwrap().trials.len();
        for i in 0..9 {
            if i == 1 && mistake {
                assert!(!p.answer(&[127]));
                t.mistake();
                p.answer(&[]);
            }
            if i == 2 && assist {
                t.assist();
            }
            t.tick(10);
            assert!(p.answer(&p.target()));
            t.answer(p, false);
            assert_eq!(
                t.run.as_ref().unwrap().trials.len(),
                before + usize::from(i == 8)
            );
            assert!(!p.answer(&p.target()));
            assert!(p.waiting_release());
            p.answer(&[]);
        }
        t.ready();
    }
    #[test]
    fn points_are_whole_patterns_with_sticky_assistance_and_skip_resets() {
        let mut p = Practice::five_finger();
        p.level = Level::Chromatic;
        let mut t = Training::default();
        t.adaptive = true;
        t.start("flow-session".into(), &p);
        complete(&mut p, &mut t, true, false);
        assert_eq!(t.run.as_ref().unwrap().points, 40);
        complete(&mut p, &mut t, false, true);
        assert_eq!(t.run.as_ref().unwrap().points, 40);
        // A partially played pattern has no completed trial and can be skipped.
        for _ in 0..3 {
            p.answer(&p.target());
            t.answer(&p, false);
            p.answer(&[]);
        }
        assert_eq!(t.run.as_ref().unwrap().trials.len(), 2);
        t.answer(&p, true);
        p.skip();
        t.ready();
        t.choose(&mut p);
        assert_eq!(p.sequence_step(), 0);
        assert_eq!(p.completed, 3);
        assert!(p.review_target.is_none());
        assert_eq!(p.target(), vec![48]);
        complete(&mut p, &mut t, false, false);
        let mut run = t.run.unwrap();
        run.close(false);
        assert_eq!(run.mode, "five-finger-flow");
        assert_eq!(run.points, 140);
        assert_eq!(run.trials.len(), 4);
        assert!(run.trials[1].assisted);
        assert!(run.trials[2].skipped);
        assert_eq!(run.trials[0].response_ms, 90);
        assert!(run.validate());
        assert!(
            skill_label(&run.trials[0].skill)
                .unwrap()
                .contains("Right C4")
        );
        run.trials[0].skill = "flow:1:R:60-62".into();
        assert!(!run.validate());
    }
    #[test]
    fn spelling_and_reset_keep_the_selected_curriculum() {
        let f = FiveFingerTask::generate(Level::Chromatic, 2);
        assert_eq!(f.note_label(3), "Bb4");
        let mut p = Practice::five_finger();
        assert!(!p.answer(&[72])); // Wrong octave never accepts a step.
        p.answer(&[]);
        p.answer(&[60]);
        p.answer(&[]);
        assert_eq!(p.sequence_step(), 1);
        p.configure(crate::practice::ReadingClef::Bass, Level::Mixed);
        assert_eq!(p.sequence_step(), 0);
        assert_eq!(p.target(), vec![48]);
    }
}
