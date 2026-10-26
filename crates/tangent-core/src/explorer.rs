//! N02 keyboard orientation: exact octave transfer, not pitch-class matching.
use crate::practice::Level;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExplorerTask {
    pub source: u8,
    pub target: u8,
}
impl ExplorerTask {
    /// Deterministic curriculum: C/D/E, all white keys, then chromatic
    /// transfers in both directions. Every task fits the 49-key C2–C6 range.
    pub fn generate(level: Level, index: usize) -> Self {
        match level {
            Level::Stepwise => {
                let source = [48, 50, 52][index % 3];
                Self {
                    source,
                    target: source + 12,
                }
            }
            Level::Mixed => {
                let source = [48, 55, 50, 59, 52, 57, 53][index % 7];
                Self {
                    source,
                    target: source + 12,
                }
            }
            Level::Chromatic => {
                let source = 60 + (((index / 2 % 12) * 5) % 12) as u8;
                Self {
                    source,
                    target: if index.is_multiple_of(2) {
                        source + 12
                    } else {
                        source - 12
                    },
                }
            }
        }
    }
    pub fn higher(self) -> bool {
        self.target > self.source
    }
    /// Canonical evidence records the exact source and target, with version 1.
    pub fn skill(self) -> String {
        format!("explorer:1:{}:{}", self.source, self.target)
    }
    pub fn from_skill(s: &str) -> Option<Self> {
        let fields: Vec<_> = s.split(':').collect();
        if fields.len() != 4 || fields[0] != "explorer" || fields[1] != "1" {
            return None;
        }
        if fields[2..]
            .iter()
            .any(|field| field.is_empty() || !field.bytes().all(|c| c.is_ascii_digit()))
        {
            return None;
        }
        let task = Self {
            source: fields[2].parse().ok()?,
            target: fields[3].parse().ok()?,
        };
        (task.source < 128 && task.target < 128 && task.source.abs_diff(task.target) == 12)
            .then_some(task)
    }
}
pub fn note_name(key: u8) -> String {
    format!(
        "{}{}",
        [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"
        ][key as usize % 12],
        i16::from(key) / 12 - 1
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn levels_cover_intended_keys_and_directions_reproducibly() {
        for level in [Level::Stepwise, Level::Mixed, Level::Chromatic] {
            let mut pitches = std::collections::BTreeSet::new();
            for i in 0..24 {
                let task = ExplorerTask::generate(level, i);
                assert_eq!(task, ExplorerTask::generate(level, i));
                assert_eq!(task.source.abs_diff(task.target), 12);
                assert!((36..=84).contains(&task.target));
                assert_eq!(ExplorerTask::from_skill(&task.skill()), Some(task));
                pitches.insert(task.source % 12);
                assert_eq!(
                    task.higher(),
                    level != Level::Chromatic || i.is_multiple_of(2)
                );
            }
            assert_eq!(
                pitches.len(),
                match level {
                    Level::Stepwise => 3,
                    Level::Mixed => 7,
                    Level::Chromatic => 12,
                }
            );
        }
        assert!(ExplorerTask::generate(Level::Chromatic, usize::MAX).target < 128);
        for bad in [
            "explorer:2:48:60",
            "explorer:1:48:72",
            "explorer:1:128:116",
            "explorer:1:48:60:extra",
            "explorer:1:+48:60",
        ] {
            assert!(ExplorerTask::from_skill(bad).is_none());
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::{
        practice::{Exercise, Practice},
        progress::{Training, skill, skill_label},
    };
    #[test]
    fn exact_octave_full_release_and_extra_keys_keep_task_identity() {
        let mut p = Practice::explorer();
        assert_eq!(p.explorer_task().unwrap().source, 48);
        assert!(!p.answer(&[48])); // Source is not the answer.
        assert!(!p.answer(&[72])); // Same pitch class, wrong octave.
        assert!(!p.answer(&[60, 61]));
        assert!(p.answer(&[60]));
        assert!(!p.answer(&[60]));
        assert_eq!(p.target(), vec![60]);
        p.answer(&[]);
        assert_eq!(p.target(), vec![62]);
        p.configure(crate::practice::ReadingClef::Bass, Level::Chromatic);
        assert_eq!(p.exercise, Exercise::KeyboardExplorer);
        assert_eq!(p.target(), vec![72]);
        p.completed = 1;
        assert_eq!(p.target(), vec![48]);
    }
    #[test]
    fn scored_evidence_is_exact_and_never_enters_reading_adaptive_pool() {
        let mut p = Practice::explorer();
        let mut training = Training::default();
        training.adaptive = true;
        training.start("explorer-session".into(), &p);
        for i in 0..12 {
            training.choose(&mut p);
            assert!(p.review_target.is_none());
            let tag = skill(&p);
            assert_eq!(tag, p.explorer_task().unwrap().skill());
            assert!(skill_label(&tag).unwrap().contains("->"));
            if i == 1 {
                training.mistake();
            }
            if i == 2 {
                training.assist();
            }
            training.answer(&p, i == 3);
            assert!(p.answer(&p.target()));
            p.answer(&[]);
            training.ready();
        }
        let mut run = training.run.unwrap();
        run.close(true);
        assert_eq!(run.mode, "keyboard-explorer");
        assert_eq!(run.points, 940);
        assert!(run.validate());
        assert_eq!(run.trials[0].skill, "explorer:1:48:60");
        run.trials[0].skill = "note:0:0:60".into();
        assert!(!run.validate());
    }
}
