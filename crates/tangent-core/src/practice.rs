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
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Exercise {
    #[default]
    Notes,
    Chords,
    KeyboardExplorer,
    FiveFinger,
}
#[derive(Debug, Default)]
pub struct Practice {
    pub exercise: Exercise,
    pub completed: usize,
    pub clef: ReadingClef,
    pub level: Level,
    pub hint: bool,
    waiting_release: bool,
    wrong: bool,
    sequence: crate::sequence::SequenceRunner,
    pub review_target: Option<Vec<u8>>,
}
impl Practice {
    pub fn new(chords: bool) -> Self {
        Self {
            exercise: if chords {
                Exercise::Chords
            } else {
                Exercise::Notes
            },
            ..Default::default()
        }
    }
    pub fn explorer() -> Self {
        Self {
            exercise: Exercise::KeyboardExplorer,
            ..Default::default()
        }
    }
    pub fn five_finger() -> Self {
        Self {
            exercise: Exercise::FiveFinger,
            ..Default::default()
        }
    }
    pub fn flow_task(&self) -> Option<crate::five_finger::FiveFingerTask> {
        (self.exercise == Exercise::FiveFinger)
            .then(|| crate::five_finger::FiveFingerTask::generate(self.level, self.completed))
    }
    pub fn sequence_step(&self) -> usize {
        self.sequence.cursor()
    }
    pub fn sequence_accepted(&self) -> bool {
        self.flow_task()
            .is_some_and(|p| self.sequence.accepted(&p.notes))
    }
    pub fn waiting_release(&self) -> bool {
        if self.exercise == Exercise::FiveFinger {
            self.sequence.waiting_release()
        } else {
            self.waiting_release
        }
    }
    pub fn wrong(&self) -> bool {
        if self.exercise == Exercise::FiveFinger {
            self.sequence.wrong()
        } else {
            self.wrong
        }
    }
    pub fn skip(&mut self) {
        self.sequence.reset();
        self.waiting_release = false;
        self.wrong = false;
        self.completed += 1;
        self.hint = false;
        self.review_target = None;
    }
    pub fn is_chord(&self) -> bool {
        self.exercise == Exercise::Chords
    }
    pub fn explorer_task(&self) -> Option<crate::explorer::ExplorerTask> {
        (self.exercise == Exercise::KeyboardExplorer)
            .then(|| crate::explorer::ExplorerTask::generate(self.level, self.completed))
    }
    pub fn curriculum_name(&self) -> &'static str {
        if self.exercise == Exercise::FiveFinger {
            return match self.level {
                Level::Stepwise => "Right / C",
                Level::Mixed => "Left / C",
                Level::Chromatic => "Moving positions",
            };
        }
        if self.exercise == Exercise::KeyboardExplorer {
            return match self.level {
                Level::Stepwise => "C / D / E",
                Level::Mixed => "White keys",
                Level::Chromatic => "All keys / both directions",
            };
        }
        if self.is_chord() {
            match self.level {
                Level::Stepwise => "Diatonic triads",
                Level::Mixed => "Major/minor triads",
                Level::Chromatic => "Seventh chords",
            }
        } else {
            self.level.name()
        }
    }
    pub fn target(&self) -> Vec<u8> {
        if let Some(task) = self.flow_task() {
            return self.sequence.target(&task.notes).into_iter().collect();
        }
        if let Some(task) = self.explorer_task() {
            return vec![task.target];
        }
        if let Some(keys) = &self.review_target {
            return keys.clone();
        }
        if self.is_chord() && self.level != Level::Stepwise {
            let root = 60 + (self.completed % 12) as u8;
            let minor = (self.completed / 12) % 2 == 1;
            return if self.level == Level::Chromatic {
                vec![root, root + if minor { 3 } else { 4 }, root + 7, root + 10]
            } else {
                vec![root, root + if minor { 3 } else { 4 }, root + 7]
            };
        }
        let root = if self.is_chord() {
            [60, 62, 64, 65, 67, 69, 71][self.completed % 7]
        } else {
            self.note_target()
        };
        if self.is_chord() {
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
        if self.is_chord() && (self.level != Level::Stepwise || self.review_target.is_some()) {
            let keys = self.target();
            let name = [
                "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
            ][keys[0] as usize % 12];
            let kind = if keys.len() == 4 {
                if keys[1] - keys[0] == 3 {
                    "minor seventh"
                } else {
                    "dominant seventh"
                }
            } else if keys[2] - keys[0] == 6 {
                "diminished"
            } else if keys[1] - keys[0] == 3 {
                "minor"
            } else {
                "major"
            };
            format!("{name} {kind}")
        } else if self.is_chord() {
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
            let key = self.target()[0];
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
            exercise: self.exercise,
            ..Default::default()
        };
    }
    pub fn answer(&mut self, held: &[u8]) -> bool {
        if let Some(task) = self.flow_task() {
            use crate::sequence::SequenceEvent;
            let event = self.sequence.input(&task.notes, held);
            if event == SequenceEvent::PatternReleased {
                self.completed += 1;
                self.hint = false;
            }
            return matches!(
                event,
                SequenceEvent::StepAccepted | SequenceEvent::PatternAccepted
            );
        }
        if held.is_empty() {
            if self.waiting_release {
                self.completed += 1;
                self.hint = false;
                self.review_target = None;
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
            .map(|k| if self.is_chord() { k % 12 } else { *k })
            .collect();
        let mut expected: Vec<_> = self
            .target()
            .iter()
            .map(|k| if self.is_chord() { k % 12 } else { *k })
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
            // A chord can be assembled one finger at a time. A subset of
            // the expected pitch classes is incomplete, not an error.
            self.wrong = actual.iter().any(|key| !expected.contains(key));
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn releasing_wrong_finger_can_complete_held_target() {
        let mut p = Practice::default();
        assert!(!p.answer(&[60, 62]));
        assert!(p.answer(&[60]));
        assert_eq!(p.completed, 0);
        p.answer(&[]);
        assert_eq!(p.completed, 1);
    }
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
            exercise: Exercise::Chords,
            ..Default::default()
        };
        assert!(!p.answer(&[60, 64]));
        assert!(!p.wrong);
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

/// Cosmetic performance feedback; this is not calibrated scoring or a saved score.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Grade {
    Correct,
    Wrong,
    Neutral,
}
#[derive(Default)]
pub struct SongHits(std::collections::BTreeSet<(u64, u8)>, Option<(u64, u8)>);
impl SongHits {
    pub fn reset(&mut self) {
        self.0.clear();
        self.1 = None;
    }
    pub fn latest_target(&self) -> Option<(u64, u8)> {
        self.1
    }
    /// Award each target once per transport run. Reattacking a held target
    /// remains neutral instead of farming streak milestones.
    pub fn judge(&mut self, song: &crate::music::Song, position_us: u64, key: u8) -> Grade {
        self.judge_selection(song, position_us, key, None)
    }
    pub fn judge_selection(
        &mut self,
        song: &crate::music::Song,
        position_us: u64,
        key: u8,
        context: Option<&crate::passage::SongContext>,
    ) -> Grade {
        self.1 = None;
        let candidates: Vec<_> = song
            .notes
            .iter()
            .filter(|n| {
                n.key == key
                    && context.is_none_or(|c| c.includes(n))
                    && song.parts.get(n.part).is_some_and(|p| !p.percussion)
                    && n.start_us <= position_us.saturating_add(180_000)
                    && n.end_us.saturating_add(180_000) >= position_us
            })
            .collect();
        if let Some(note) = candidates
            .iter()
            .filter(|n| !self.0.contains(&(n.start_us, n.key)))
            .min_by_key(|n| n.start_us.abs_diff(position_us))
        {
            self.0.insert((note.start_us, note.key));
            self.1 = Some((note.start_us, note.key));
            Grade::Correct
        } else if candidates.is_empty() {
            Grade::Wrong
        } else {
            Grade::Neutral
        }
    }
}
#[cfg(test)]
mod hit_tests {
    use super::*;
    use crate::music::{Part, PerformanceNote, Song};
    #[test]
    fn simultaneous_unison_parts_are_one_physical_target() {
        let mut song = Song {
            title: String::new(),
            composer: None,
            duration_us: 2000000,
            parts: vec![Part {
                name: String::new(),
                percussion: false,
            }],
            notes: vec![],
            notation: None,
            warnings: vec![],
            events: vec![],
        };
        for id in 0..2 {
            song.notes.push(PerformanceNote {
                id,
                part: 0,
                key: 60,
                velocity: 90,
                start_us: 0,
                end_us: 1000000,
                written: None,
            });
        }
        let mut hits = SongHits::default();
        assert_eq!(hits.judge(&song, 0, 60), Grade::Correct);
        assert_eq!(hits.judge(&song, 10000, 60), Grade::Neutral);
        assert_eq!(crate::progress::song_target_count(&song), 1);
    }
    #[test]
    fn repeated_input_cannot_award_same_target_twice_and_seek_can_reset() {
        let mut song = Song {
            title: String::new(),
            composer: None,
            duration_us: 3_000_000,
            parts: vec![Part {
                name: String::new(),
                percussion: false,
            }],
            notes: vec![PerformanceNote {
                id: 0,
                part: 0,
                key: 60,
                velocity: 90,
                start_us: 1_000_000,
                end_us: 1_500_000,
                written: None,
            }],
            notation: None,
            warnings: vec![],
            events: vec![],
        };
        let mut hits = SongHits::default();
        assert_eq!(hits.judge(&song, 700_000, 60), Grade::Wrong);
        assert_eq!(hits.judge(&song, 900_000, 60), Grade::Correct);
        assert_eq!(hits.judge(&song, 1_200_000, 60), Grade::Neutral);
        assert_eq!(hits.judge(&song, 1_200_000, 61), Grade::Wrong);
        assert_eq!(hits.judge(&song, 1_700_001, 60), Grade::Wrong);
        hits.reset();
        assert_eq!(hits.judge(&song, 1_200_000, 60), Grade::Correct);
        song.parts[0].percussion = true;
        assert_eq!(hits.judge(&song, 1_200_000, 60), Grade::Wrong);
    }
}

#[cfg(test)]
mod curriculum_tests {
    use super::*;
    #[test]
    fn expanded_chord_pools_cover_every_root_and_quality() {
        for level in [Level::Mixed, Level::Chromatic] {
            let mut p = Practice::new(true);
            p.level = level;
            let mut seen = std::collections::BTreeSet::new();
            for i in 0..24 {
                p.completed = i;
                let target = p.target();
                assert_eq!(target.len(), if level == Level::Mixed { 3 } else { 4 });
                seen.insert(target);
            }
            assert_eq!(seen.len(), 24);
        }
    }
    #[test]
    fn seventh_inversions_match_and_review_labels_keep_quality() {
        let mut p = Practice::new(true);
        p.level = Level::Chromatic;
        assert_eq!(p.label(), "C dominant seventh");
        assert!(p.answer(&[52, 55, 58, 60]));
        p.answer(&[]);
        p.level = Level::Stepwise;
        p.review_target = Some(vec![71, 74, 77]);
        assert_eq!(p.label(), "B diminished");
    }
}
