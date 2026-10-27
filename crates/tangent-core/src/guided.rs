//! Guided song-time gates; independent of rendering, audio devices and wall clocks.
use crate::{music::Song, passage::SongContext, practice::Grade, transport::Transport};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuidedConfig {
    pub early_ms: u16,
}
impl Default for GuidedConfig {
    fn default() -> Self {
        Self { early_ms: 250 }
    }
}
impl GuidedConfig {
    pub fn validate(self) -> bool {
        self.early_ms <= 500
    }
}
#[derive(Debug)]
struct Gate {
    at: u64,
    keys: BTreeSet<u8>,
}
#[derive(Debug, Default)]
pub struct GuidedPlay {
    gates: Vec<Gate>,
    cursor: usize,
    credited: BTreeSet<u8>,
    position: u64,
}
#[derive(Debug)]
pub struct GuidedHit {
    pub grade: Grade,
    pub target: Option<(u64, u8)>,
    pub advanced: bool,
}
impl GuidedPlay {
    pub fn new(song: &Song, context: &SongContext, position: u64) -> Self {
        let mut groups: BTreeMap<u64, BTreeSet<u8>> = BTreeMap::new();
        for n in song.notes.iter().filter(|n| {
            context.includes(n)
                && n.key < 128
                && song.parts.get(n.part).is_some_and(|p| !p.percussion)
        }) {
            groups.entry(n.start_us).or_default().insert(n.key);
        }
        let gates: Vec<_> = groups
            .into_iter()
            .map(|(at, keys)| Gate { at, keys })
            .collect();
        let cursor = gates.partition_point(|g| g.at < position);
        Self {
            gates,
            cursor,
            position,
            ..Default::default()
        }
    }

    pub fn position(&self) -> u64 {
        self.position
    }
    pub fn barrier(&self) -> Option<u64> {
        self.gates.get(self.cursor).map(|g| g.at)
    }
    pub fn targets(&self) -> Vec<u8> {
        self.gates
            .get(self.cursor)
            .map_or_else(Vec::new, |g| g.keys.iter().copied().collect())
    }
    pub fn waiting(&self) -> bool {
        self.barrier().is_some_and(|at| self.position >= at)
    }
    pub fn remaining(&self, held: &BTreeSet<u8>) -> Vec<u8> {
        self.targets()
            .into_iter()
            .filter(|k| !self.credited.contains(k) || !held.contains(k))
            .collect()
    }
    /// Stops exactly at the first unfulfilled onset, even after a very large frame.
    /// At a loop boundary discard overshoot: every repeat requires fresh input.
    pub fn advance(&mut self, t: &mut Transport, elapsed: u64) -> u64 {
        let boundary = self.barrier().or_else(|| t.passage().map(|p| p.end_us));
        let wraps = t.advance_until(elapsed, boundary);
        if t.playing
            && self.barrier().is_none()
            && let Some(p) = t.passage()
            && t.position_us == p.end_us
        {
            t.seek(p.start_us);
            self.cursor = 0;
            self.credited.clear();
            self.position = t.position_us;
            return 1;
        }
        self.position = t.position_us;
        wraps
    }
    pub fn attack(
        &mut self,
        t: &Transport,
        config: GuidedConfig,
        key: u8,
        fresh: bool,
        held: &BTreeSet<u8>,
    ) -> GuidedHit {
        let neutral = || GuidedHit {
            grade: Grade::Neutral,
            target: None,
            advanced: false,
        };
        if !t.playing || !fresh || !config.validate() {
            return neutral();
        }
        let Some(gate) = self.gates.get(self.cursor) else {
            return neutral();
        };
        let window = t.rate().song_time(u64::from(config.early_ms) * 1000);
        if t.position_us.saturating_add(window) < gate.at || !gate.keys.contains(&key) {
            return GuidedHit {
                grade: Grade::Wrong,
                target: None,
                advanced: false,
            };
        }
        let new = self.credited.insert(key);
        let target = new.then_some((gate.at, key));
        let complete = gate.keys.is_subset(&self.credited) && gate.keys.is_subset(held);
        if complete {
            self.cursor += 1;
            self.credited.clear();
        }
        GuidedHit {
            grade: if new { Grade::Correct } else { Grade::Neutral },
            target,
            advanced: complete,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        music::{Part, PerformanceNote},
        passage::Passage,
        transport::PlaybackRate,
    };
    fn song() -> Song {
        Song {
            title: "Guided".into(),
            composer: None,
            duration_us: 4_000_000,
            parts: vec![
                Part {
                    name: "Piano".into(),
                    percussion: false,
                },
                Part {
                    name: "Other".into(),
                    percussion: false,
                },
            ],
            notes: [
                (0, 60, 1_000_000),
                (0, 64, 1_000_000),
                (0, 60, 2_000_000),
                (1, 48, 1_500_000),
            ]
            .into_iter()
            .enumerate()
            .map(|(id, (part, key, start_us))| PerformanceNote {
                id,
                part,
                key,
                velocity: 90,
                start_us,
                end_us: start_us + 500_000,
                written: None,
                rhythm: None,
            })
            .collect(),
            events: vec![],
            notation: None,
            warnings: vec![],
        }
    }
    fn context() -> SongContext {
        SongContext {
            passage: Passage {
                start_us: 0,
                end_us: 4_000_000,
            },
            duration_us: 4_000_000,
            speed_percent: 100,
            part: Some(0),
            evaluator: 1,
            song_audio: false,
            guided: Some(GuidedConfig::default()),
        }
    }
    #[test]
    fn chord_gates_reject_wrong_early_and_preheld_input() {
        let s = song();
        let mut t = Transport::new(s.duration_us);
        t.playing = true;
        let mut g = GuidedPlay::new(&s, &context(), 0);
        let c = GuidedConfig::default();
        assert_eq!(g.attack(&t, c, 60, true, &[60].into()).grade, Grade::Wrong);
        g.advance(&mut t, u64::MAX);
        assert_eq!(t.position_us, 1_000_000);
        assert!(g.waiting());
        assert!(t.playing);
        assert_eq!(g.attack(&t, c, 72, true, &[72].into()).grade, Grade::Wrong);
        assert!(!g.attack(&t, c, 60, true, &[60].into()).advanced);
        assert!(
            !g.attack(&t, c, 64, true, &[64].into()).advanced,
            "Chord must be held together"
        );
        assert!(g.attack(&t, c, 60, true, &[60, 64].into()).advanced);
        g.advance(&mut t, 10_000_000);
        assert_eq!(t.position_us, 2_000_000);
        assert_eq!(
            g.attack(&t, c, 60, false, &[60].into()).grade,
            Grade::Neutral
        );
        assert!(g.attack(&t, c, 60, true, &[60].into()).advanced);
        g.advance(&mut t, 10_000_000);
        assert_eq!(t.position_us, 4_000_000);
        assert!(!t.playing);
    }
    #[test]
    fn early_window_is_wall_time_and_loops_cannot_skip_targets() {
        let s = song();
        let c = context();
        let mut t = Transport::new(s.duration_us);
        t.playing = true;
        t.set_rate(PlaybackRate::new(50).unwrap());
        let mut g = GuidedPlay::new(&s, &c, 0);
        g.advance(&mut t, 1_750_000);
        assert_eq!(t.position_us, 875_000);
        assert_eq!(
            g.attack(&t, GuidedConfig { early_ms: 249 }, 60, true, &[60].into())
                .grade,
            Grade::Wrong
        );
        assert_eq!(
            g.attack(&t, GuidedConfig { early_ms: 250 }, 60, true, &[60].into())
                .grade,
            Grade::Correct
        );
        assert!(
            g.attack(
                &t,
                GuidedConfig { early_ms: 250 },
                64,
                true,
                &[60, 64].into()
            )
            .advanced
        );
        assert!(t.set_passage(Some(c.passage)));
        g.advance(&mut t, u64::MAX);
        assert_eq!(t.position_us, 2_000_000);
        g.attack(&t, GuidedConfig::default(), 60, true, &[60].into());
        assert_eq!(g.advance(&mut t, u64::MAX), 1);
        assert_eq!(t.position_us, 0);
        g.advance(&mut t, u64::MAX);
        assert_eq!(t.position_us, 1_000_000);
        assert!(!GuidedConfig { early_ms: 501 }.validate());
    }
    #[test]
    fn loops_started_after_a_seek_restore_earlier_targets() {
        let s = song();
        let c = context();
        let mut t = Transport::new(s.duration_us);
        t.set_passage(Some(c.passage));
        t.seek(1_100_000);
        t.playing = true;
        let mut g = GuidedPlay::new(&s, &c, t.position_us);
        g.advance(&mut t, u64::MAX);
        assert_eq!(t.position_us, 2_000_000);
        g.attack(&t, GuidedConfig::default(), 60, true, &[60].into());
        assert_eq!(g.advance(&mut t, u64::MAX), 1);
        g.advance(&mut t, u64::MAX);
        assert_eq!(t.position_us, 1_000_000);
        assert_eq!(g.targets(), vec![60, 64]);
    }
    #[test]
    fn normal_and_guided_scores_are_not_comparable() {
        use crate::progress::Run;
        let mut a = Run::song("guided-score-test".into(), "Test".into(), 3);
        a.piece_id = Some("a".repeat(64));
        a.context = Some(context());
        a.input_sources = Some(vec!["midi".into()]);
        a.close(true);
        let mut b = a.clone();
        assert!(a.comparable_to(&b));
        b.context.as_mut().unwrap().guided = None;
        assert!(!a.comparable_to(&b));
        b = a.clone();
        b.context.as_mut().unwrap().guided = Some(GuidedConfig { early_ms: 100 });
        assert!(!a.comparable_to(&b));
    }
    #[test]
    fn selection_seek_and_unisons_are_explicit() {
        let mut s = song();
        s.notes.push(s.notes[0].clone());
        let c = context();
        let g = GuidedPlay::new(&s, &c, 0);
        assert_eq!(g.targets(), vec![60, 64]);
        let g = GuidedPlay::new(&s, &c, 1_100_000);
        assert_eq!(g.barrier(), Some(2_000_000));
        let mut c = c;
        c.part = Some(1);
        assert_eq!(GuidedPlay::new(&s, &c, 0).targets(), vec![48]);
    }
}
