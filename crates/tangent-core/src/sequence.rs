//! Clock-independent ordered note input. Each accepted step requires full release.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SequenceEvent {
    Idle,
    Wrong,
    StepAccepted,
    PatternAccepted,
    StepReleased,
    PatternReleased,
}
#[derive(Debug, Default)]
pub struct SequenceRunner {
    cursor: usize,
    waiting_release: bool,
    wrong: bool,
}
impl SequenceRunner {
    pub fn cursor(&self) -> usize {
        self.cursor
    }
    pub fn waiting_release(&self) -> bool {
        self.waiting_release
    }
    pub fn wrong(&self) -> bool {
        self.wrong
    }
    pub fn target(&self, notes: &[u8]) -> Option<u8> {
        notes.get(self.cursor).copied()
    }
    pub fn accepted(&self, notes: &[u8]) -> bool {
        self.waiting_release && self.cursor + 1 == notes.len()
    }
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    pub fn input(&mut self, notes: &[u8], held: &[u8]) -> SequenceEvent {
        if notes.is_empty() || notes.iter().any(|&key| key > 127) {
            self.reset();
            return SequenceEvent::Idle;
        }
        if self.cursor >= notes.len() {
            self.reset();
        }
        if held.is_empty() {
            self.wrong = false;
            if !self.waiting_release {
                return SequenceEvent::Idle;
            }
            if self.cursor + 1 == notes.len() {
                self.reset();
                return SequenceEvent::PatternReleased;
            }
            self.waiting_release = false;
            self.cursor += 1;
            return SequenceEvent::StepReleased;
        }
        if self.waiting_release {
            return SequenceEvent::Idle;
        }
        if held.iter().all(|&key| key == notes[self.cursor]) {
            self.waiting_release = true;
            self.wrong = false;
            if self.cursor + 1 == notes.len() {
                SequenceEvent::PatternAccepted
            } else {
                SequenceEvent::StepAccepted
            }
        } else {
            self.wrong = true;
            SequenceEvent::Wrong
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn order_full_release_and_repeated_targets_are_distinct_steps() {
        let mut r = SequenceRunner::default();
        let notes = [60, 60, 62];
        assert_eq!(r.input(&notes, &[62]), SequenceEvent::Wrong);
        assert_eq!(r.input(&notes, &[60, 61]), SequenceEvent::Wrong);
        assert_eq!(r.input(&notes, &[60, 60]), SequenceEvent::StepAccepted);
        assert_eq!(r.input(&notes, &[62]), SequenceEvent::Idle);
        assert_eq!(r.cursor(), 0);
        assert_eq!(r.input(&notes, &[]), SequenceEvent::StepReleased);
        assert_eq!(r.target(&notes), Some(60));
        assert_eq!(r.input(&notes, &[60]), SequenceEvent::StepAccepted);
        r.input(&notes, &[]);
        assert_eq!(r.input(&notes, &[62]), SequenceEvent::PatternAccepted);
        assert!(r.accepted(&notes));
        assert_eq!(r.input(&notes, &[]), SequenceEvent::PatternReleased);
        assert_eq!(r.cursor(), 0);
        assert!(!r.waiting_release());
        assert_eq!(r.input(&[], &[60]), SequenceEvent::Idle);
    }
}
