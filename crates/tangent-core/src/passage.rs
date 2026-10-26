//! Source-time practice selection and honest comparison context, independent of UI.
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Passage {
    pub start_us: u64,
    pub end_us: u64,
}
impl Passage {
    pub fn new(start_us: u64, end_us: u64, duration_us: u64) -> Option<Self> {
        (start_us < end_us && end_us <= duration_us).then_some(Self { start_us, end_us })
    }
    pub fn contains(self, at: u64) -> bool {
        self.start_us <= at && at < self.end_us
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SongContext {
    pub passage: Passage,
    pub duration_us: u64,
    pub speed_percent: u16,
    /// Explicit imported part index; None means all parts, never a guessed hand.
    pub part: Option<usize>,
    pub evaluator: u16,
    pub song_audio: bool,
}
impl SongContext {
    pub fn validate(&self) -> bool {
        Passage::new(self.passage.start_us, self.passage.end_us, self.duration_us).is_some()
            && crate::transport::PlaybackRate::new(self.speed_percent).is_some()
            && self.evaluator == 1
            && self.part.is_none_or(|p| p < 100000)
    }
    pub fn includes(&self, n: &crate::music::PerformanceNote) -> bool {
        self.passage.contains(n.start_us) && self.part.is_none_or(|p| p == n.part)
    }
    pub fn target_count(&self, song: &crate::music::Song) -> u32 {
        song.notes
            .iter()
            .filter(|n| self.includes(n) && song.parts.get(n.part).is_some_and(|p| !p.percussion))
            .map(|n| (n.start_us, n.key))
            .collect::<std::collections::BTreeSet<_>>()
            .len() as u32
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn range_is_half_open_and_bounded() {
        assert!(Passage::new(5, 5, 10).is_none());
        assert!(Passage::new(5, 11, 10).is_none());
        let p = Passage::new(5, 10, 10).unwrap();
        assert!(p.contains(5));
        assert!(!p.contains(10));
    }
    #[test]
    fn context_rejects_unknown_evaluator_and_invalid_rate() {
        let mut c = SongContext {
            passage: Passage {
                start_us: 0,
                end_us: 10,
            },
            duration_us: 10,
            speed_percent: 70,
            part: None,
            evaluator: 1,
            song_audio: true,
        };
        assert!(c.validate());
        c.speed_percent = 0;
        assert!(!c.validate());
        c.speed_percent = 70;
        c.evaluator = 2;
        assert!(!c.validate());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::{
        music::{Part, PerformanceNote, Song},
        practice::{Grade, SongHits},
        progress::Run,
    };
    fn song() -> Song {
        Song {
            title: "Selection".into(),
            composer: None,
            duration_us: 30_000_000,
            parts: vec![
                Part {
                    name: "Upper".into(),
                    percussion: false,
                },
                Part {
                    name: "Lower".into(),
                    percussion: false,
                },
            ],
            notes: vec![
                (0, 0, 60, 1_000_000),
                (1, 1, 48, 1_000_000),
                (2, 0, 62, 11_000_000),
                (3, 0, 64, 21_000_000),
            ]
            .into_iter()
            .map(|(id, part, key, start_us)| PerformanceNote {
                id,
                part,
                key,
                start_us,
                end_us: start_us + 500_000,
                velocity: 90,
                written: None,
            })
            .collect(),
            notation: None,
            warnings: vec![],
            events: vec![],
        }
    }
    fn context() -> SongContext {
        SongContext {
            passage: Passage {
                start_us: 0,
                end_us: 30_000_000,
            },
            duration_us: 30_000_000,
            speed_percent: 70,
            part: Some(0),
            evaluator: 1,
            song_audio: false,
        }
    }
    #[test]
    fn selection_filters_pitch_judgment_and_denominator() {
        let song = song();
        let mut c = context();
        let mut hits = SongHits::default();
        assert_eq!(c.target_count(&song), 3);
        assert_eq!(
            hits.judge_selection(&song, 1_000_000, 48, Some(&c)),
            Grade::Wrong
        );
        assert_eq!(
            hits.judge_selection(&song, 1_000_000, 60, Some(&c)),
            Grade::Correct
        );
        assert_eq!(
            hits.judge_selection(&song, 1_000_000, 60, Some(&c)),
            Grade::Neutral
        );
        c.passage = Passage {
            start_us: 10_000_000,
            end_us: 21_000_000,
        };
        assert_eq!(c.target_count(&song), 1);
        assert_eq!(
            hits.judge_selection(&song, 21_000_000, 64, Some(&c)),
            Grade::Wrong
        );
    }
    #[test]
    fn comparisons_require_complete_known_identical_context() {
        let mut a = Run::song("test-session".into(), "Test".into(), 3);
        a.piece_id = Some("a".repeat(64));
        a.input_sources = Some(vec!["midi".into()]);
        a.context = Some(context());
        a.close(true);
        let mut b = a.clone();
        assert!(a.comparable_to(&b));
        assert_eq!(a.matched_change(&b), Some(0));
        b.song_correct = 1;
        b.song_missed = 2;
        b.points = 100;
        assert_eq!(a.matched_change(&b), Some(-1));
        b.context.as_mut().unwrap().speed_percent = 80;
        assert!(!a.comparable_to(&b));
        b = a.clone();
        b.finished = false;
        assert!(!a.comparable_to(&b));
        b = a.clone();
        b.context = None;
        assert!(!a.comparable_to(&b));
        b = a.clone();
        b.context.as_mut().unwrap().song_audio = true;
        assert!(!a.comparable_to(&b));
        b = a.clone();
        b.piece_id = Some("b".repeat(64));
        assert!(!a.comparable_to(&b));
    }
    #[test]
    fn weak_section_uses_actual_missing_targets_and_stable_ties() {
        let song = song();
        let mut run = Run::song("test-session".into(), "Test".into(), 3);
        run.context = Some(context());
        run.song_input(Grade::Correct);
        run.matched_targets = Some(vec![(1_000_000, 60)]);
        run.close(true);
        assert!(run.validate());
        assert_eq!(
            run.weak_passage(&song),
            Passage::new(10_000_000, 20_000_000, 30_000_000)
        );
        run.finished = false;
        assert!(run.weak_passage(&song).is_none());
        run.finished = true;
        run.matched_targets = None;
        assert!(run.weak_passage(&song).is_none());
        run.matched_targets = Some(vec![(1_000_000, 60), (1_000_000, 60)]);
        assert!(!run.validate());
    }
}

impl Passage {
    /// Snap to the particular expanded measure visits, never all written repeats.
    pub fn snap_to_measures(
        self,
        score: &crate::notation::NotatedScore,
        duration_us: u64,
    ) -> Option<Self> {
        Self::new(self.start_us, self.end_us, duration_us)?;
        let start = score
            .playback
            .iter()
            .find(|v| v.start_us <= self.start_us && self.start_us < v.end_us)?;
        let end = score
            .playback
            .iter()
            .find(|v| v.start_us < self.end_us && self.end_us <= v.end_us)?;
        Self::new(start.start_us, end.end_us.min(duration_us), duration_us)
    }
}
/// Optional session-local repetition target. This measures coverage, not mastery.
#[derive(Clone, Debug)]
pub struct RepetitionGoal {
    required: u8,
    minimum_coverage: u8,
    accepted: Vec<String>,
}
impl RepetitionGoal {
    pub fn new(required: u8, minimum_coverage: u8) -> Option<Self> {
        ((1..=20).contains(&required) && (1..=100).contains(&minimum_coverage)).then_some(Self {
            required,
            minimum_coverage,
            accepted: vec![],
        })
    }
    pub fn completed(&self) -> u8 {
        self.accepted.len() as u8
    }
    pub fn required(&self) -> u8 {
        self.required
    }
    pub fn reached(&self) -> bool {
        self.completed() >= self.required
    }
    /// Returns true only when this complete, evidenced attempt advances the goal.
    pub fn record(&mut self, run: &crate::progress::Run) -> bool {
        if self.reached()
            || !run.validate()
            || !run.finished
            || run.mode != "play"
            || run.context.is_none()
            || run.piece_id.is_none()
            || run.song_total == 0
            || run.input_sources.as_ref().is_none_or(Vec::is_empty)
            || run.accuracy().unwrap_or(0) < u32::from(self.minimum_coverage)
            || self.accepted.contains(&run.id)
        {
            return false;
        }
        self.accepted.push(run.id.clone());
        true
    }
}

#[cfg(test)]
mod goal_tests {
    use super::*;
    use crate::notation::{MeasureVisit, NotatedScore};
    #[test]
    fn snapping_selects_specific_repeat_visits_and_respects_exact_end() {
        let score = NotatedScore {
            staves: vec![],
            measures: vec![],
            playback: (0..4)
                .map(|i| MeasureVisit {
                    measure: i % 2,
                    pass: (i / 2 + 1) as u8,
                    start_us: i as u64 * 10,
                    end_us: (i + 1) as u64 * 10,
                    timing: vec![],
                })
                .collect(),
        };
        assert_eq!(
            Passage {
                start_us: 21,
                end_us: 29
            }
            .snap_to_measures(&score, 40),
            Passage::new(20, 30, 40)
        );
        assert_eq!(
            Passage {
                start_us: 11,
                end_us: 30
            }
            .snap_to_measures(&score, 40),
            Passage::new(10, 30, 40)
        );
        assert!(
            Passage {
                start_us: 40,
                end_us: 41
            }
            .snap_to_measures(&score, 40)
            .is_none()
        );
    }
    #[test]
    fn goal_needs_real_complete_input_and_deduplicates_attempts() {
        use crate::{practice::Grade, progress::Run};
        let mut r = Run::song("goal-attempt-one".into(), "Goal".into(), 1);
        r.context = Some(SongContext {
            passage: Passage {
                start_us: 0,
                end_us: 100,
            },
            duration_us: 100,
            speed_percent: 70,
            part: None,
            evaluator: 1,
            song_audio: true,
        });
        r.piece_id = Some("a".repeat(64));
        r.matched_targets = Some(vec![]);
        r.input_sources = Some(vec![]);
        r.close(true);
        let mut goal = RepetitionGoal::new(3, 80).unwrap();
        assert!(!goal.record(&r));
        r.song_input(Grade::Correct);
        r.matched_targets = Some(vec![(0, 60)]);
        r.input_sources = Some(vec!["midi".into()]);
        r.close(true);
        assert!(goal.record(&r));
        assert!(!goal.record(&r));
        r.id = "goal-attempt-two".into();
        r.finished = false;
        assert!(!goal.record(&r));
        r.close(true);
        assert!(goal.record(&r));
        r.id = "goal-attempt-three".into();
        assert!(goal.record(&r));
        assert!(goal.reached());
        assert_eq!(goal.completed(), 3);
        assert!(RepetitionGoal::new(0, 80).is_none());
        assert!(RepetitionGoal::new(3, 101).is_none());
    }
}

#[cfg(test)]
mod bounded_evidence_tests {
    use super::*;
    use crate::{
        practice::Grade,
        progress::{Run, TARGET_EVIDENCE_LIMIT},
    };
    #[test]
    fn long_attempt_keeps_totals_without_unbounded_detail_and_goals_use_real_counts() {
        let mut run = Run::song(
            "long-attempt-test".into(),
            "Synthetic".into(),
            TARGET_EVIDENCE_LIMIT as u32 + 1,
        );
        run.context = Some(SongContext {
            passage: Passage {
                start_us: 0,
                end_us: 30_000,
            },
            duration_us: 30_000,
            speed_percent: 70,
            part: None,
            evaluator: 1,
            song_audio: false,
        });
        run.matched_targets = Some(vec![]);
        run.piece_id = Some("a".repeat(64));
        run.input_sources = Some(vec!["computer".into()]);
        run.song_input_with_target(Grade::Correct, Some((0, 60)));
        run.song_input_with_target(Grade::Correct, Some((0, 60)));
        assert_eq!(run.song_correct, 1);
        for at in 1..=TARGET_EVIDENCE_LIMIT as u64 {
            run.song_input_with_target(Grade::Correct, Some((at, 60)));
        }
        run.close(true);
        assert!(run.matched_targets.is_none());
        assert_eq!(run.song_correct, TARGET_EVIDENCE_LIMIT as u32 + 1);
        assert!(run.validate());
        let mut goal = RepetitionGoal::new(3, 80).unwrap();
        assert!(goal.record(&run));
        // The new writer cap must not invalidate older, larger detailed records.
        run.matched_targets = Some(
            (0..=TARGET_EVIDENCE_LIMIT as u64)
                .map(|at| (at, 60))
                .collect(),
        );
        assert!(run.validate());
    }
}
