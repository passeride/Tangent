//! Scored reading/chord sessions and recent-history review. No storage or UI.
use crate::practice::{Level, Practice};
use serde::{Deserialize, Serialize};
pub const SESSION_LENGTH: usize = 12;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Trial {
    pub skill: String,
    pub clean: bool,
    pub assisted: bool,
    pub skipped: bool,
    pub response_ms: u64,
}
fn run_version() -> u32 {
    1
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Run {
    #[serde(default = "run_version")]
    pub version: u32,
    pub id: String,
    pub revision: u32,
    pub mode: String,
    pub label: String,
    #[serde(default)]
    pub clef: u8,
    #[serde(default)]
    pub level: u8,
    pub points: u32,
    pub errors: u32,
    pub elapsed_ms: u64,
    #[serde(default)]
    pub started_unix_ms: u64,
    pub finished: bool,
    pub trials: Vec<Trial>,
    #[serde(default)]
    pub song_correct: u32,
    #[serde(default)]
    pub song_total: u32,
    #[serde(default)]
    pub song_missed: u32,
    #[serde(default)]
    pub piece_id: Option<String>,
}
impl Run {
    pub fn new(id: String, p: &Practice) -> Self {
        Self {
            version: 1,
            id,
            revision: 0,
            mode: if p.chords { "chords" } else { "notes" }.into(),
            label: if p.chords {
                p.curriculum_name().into()
            } else {
                format!("{} / {}", p.clef.name(), p.curriculum_name())
            },
            clef: p.clef as u8,
            level: p.level as u8,
            points: 0,
            errors: 0,
            elapsed_ms: 0,
            started_unix_ms: 0,
            finished: false,
            trials: vec![],
            song_correct: 0,
            song_total: 0,
            song_missed: 0,
            piece_id: None,
        }
    }
    pub fn song(id: String, title: String, total: u32) -> Self {
        let mut r = Self::new(id, &Practice::default());
        r.mode = "play".into();
        r.label = title.chars().take(24).collect();
        r.song_total = total;
        r
    }
    pub fn song_input(&mut self, grade: crate::practice::Grade) {
        if self.mode != "play" {
            return;
        }
        match grade {
            crate::practice::Grade::Correct if self.song_correct < self.song_total => {
                self.song_correct += 1;
                self.points += 100;
            }
            crate::practice::Grade::Wrong => self.errors = (self.errors + 1).min(100000),
            _ => return,
        }
        self.revision += 1;
    }
    pub fn close(&mut self, complete: bool) {
        self.finished = complete;
        if self.mode == "play" {
            self.song_missed = self.song_total.saturating_sub(self.song_correct);
        }
        self.revision += 1;
    }
    pub fn solved(&self) -> usize {
        if self.mode == "play" {
            return self.song_correct as usize;
        }
        self.trials.iter().filter(|t| !t.skipped).count()
    }
    pub fn clean(&self) -> usize {
        if self.mode == "play" {
            return self.song_correct as usize;
        }
        self.trials
            .iter()
            .filter(|t| t.clean && !t.assisted && !t.skipped)
            .count()
    }
    pub fn accuracy(&self) -> Option<u32> {
        if self.mode == "play" {
            return (self.song_total > 0).then(|| self.song_correct * 100 / self.song_total);
        }
        (!self.trials.is_empty()).then(|| self.clean() as u32 * 100 / self.trials.len() as u32)
    }
    pub fn meaningful(&self) -> bool {
        !self.trials.is_empty()
            || self.errors > 0
            || self.song_correct > 0
            || (self.mode == "play" && self.finished)
    }
    pub fn validate(&self) -> bool {
        self.version == 1
            && self.id.len() >= 8
            && self.id.len() <= 100
            && self
                .id
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-')
            && matches!(self.mode.as_str(), "notes" | "chords" | "play")
            && self
                .piece_id
                .as_ref()
                .is_none_or(|id| id.len() == 64 && id.bytes().all(|b| b.is_ascii_hexdigit()))
            && self.clef <= 2
            && self.level <= 2
            && self.label.len() <= 100
            && self.trials.len() <= SESSION_LENGTH
            && ((self.mode == "play" && self.trials.is_empty())
                || (self.mode != "play"
                    && self.song_total == 0
                    && self.song_correct == 0
                    && self.song_missed == 0))
            && self.song_total <= 1000000
            && self.song_correct <= self.song_total
            && self.song_missed <= self.song_total
            && (!self.finished
                || if self.mode == "play" {
                    self.song_correct + self.song_missed == self.song_total
                } else {
                    self.trials.len() == SESSION_LENGTH
                })
            && self.points == self.trials.iter().map(points).sum::<u32>() + self.song_correct * 100
            && self.errors <= 100000
            && self.elapsed_ms <= 604800000
            && self
                .trials
                .iter()
                .all(|t| t.skill.len() <= 100 && t.response_ms <= 604800000)
    }
}
fn points(t: &Trial) -> u32 {
    if t.skipped || t.assisted {
        0
    } else if t.clean {
        100
    } else {
        40
    }
}
#[derive(Default)]
pub struct Training {
    pub run: Option<Run>,
    pub history: Vec<Run>,
    pub adaptive: bool,
    failed: bool,
    assisted: bool,
    response_ms: u64,
}
impl Training {
    pub fn start(&mut self, id: String, p: &Practice) {
        self.run = Some(Run::new(id, p));
        self.reset_trial();
    }
    pub fn ready(&mut self) {
        self.reset_trial();
    }
    fn reset_trial(&mut self) {
        self.failed = false;
        self.assisted = false;
        self.response_ms = 0;
    }
    pub fn tick(&mut self, ms: u64) {
        if let Some(r) = &mut self.run {
            r.elapsed_ms = (r.elapsed_ms + ms).min(604800000);
            self.response_ms = (self.response_ms + ms).min(604800000);
        }
    }
    pub fn mistake(&mut self) {
        if let Some(r) = &mut self.run {
            r.errors = (r.errors + 1).min(100000);
            r.revision += 1;
            self.failed = true;
        }
    }
    pub fn assist(&mut self) {
        self.assisted = true;
    }
    pub fn answer(&mut self, p: &Practice, skipped: bool) {
        let skill = skill(p);
        if let Some(r) = &mut self.run {
            if r.trials.len() >= SESSION_LENGTH {
                return;
            }
            let trial = Trial {
                skill,
                clean: !self.failed,
                assisted: self.assisted,
                skipped,
                response_ms: self.response_ms,
            };
            r.points += points(&trial);
            r.trials.push(trial);
            r.revision += 1;
        }
        self.reset_trial();
    }
    pub fn review_focus(&self) -> Vec<String> {
        let mut stats = std::collections::BTreeMap::<String, (u32, u32)>::new();
        for t in self.history.iter().flat_map(|r| r.trials.iter()) {
            let entry = stats.entry(t.skill.clone()).or_default();
            entry.0 += 1;
            if t.clean && !t.assisted && !t.skipped {
                entry.1 += 1;
            }
        }
        let mut rows: Vec<_> = stats.into_iter().collect();
        rows.sort_by(|a, b| {
            let weight = |v: &(u32, u32)| (v.1 as f64 + 1.) / (v.0 as f64 + 2.);
            weight(&a.1)
                .total_cmp(&weight(&b.1))
                .then_with(|| b.1.0.cmp(&a.1.0))
        });
        rows.into_iter().take(3).map(|(s, _)| s).collect()
    }
    pub fn choose(&self, p: &mut Practice) {
        p.review_target = None;
        if !self.adaptive {
            return;
        }
        let original = p.completed;
        let count = if p.chords {
            if p.level == Level::Stepwise { 7 } else { 24 }
        } else if p.level == Level::Chromatic {
            13
        } else {
            8
        };
        let mut candidates = vec![];
        for i in 0..count {
            p.completed = if p.clef == crate::practice::ReadingClef::Mixed && !p.chords {
                i * 2 + original % 2
            } else {
                i
            };
            let keys = p.target();
            let tag = skill(p);
            if !candidates.iter().any(|(k, _)| k == &keys) {
                candidates.push((keys, tag));
            }
        }
        p.completed = original;
        let history = self
            .history
            .iter()
            .filter(|r| self.run.as_ref().is_none_or(|a| a.id != r.id))
            .chain(self.run.iter());
        let trials: Vec<_> = history.flat_map(|r| r.trials.iter()).collect();
        let recent: Vec<_> = self
            .run
            .iter()
            .flat_map(|r| r.trials.iter().rev().take(2))
            .map(|t| t.skill.as_str())
            .collect();
        let best = candidates.iter().min_by(|(_, a), (_, b)| {
            let weight = |s: &String| {
                let rows: Vec<_> = trials.iter().filter(|t| &t.skill == s).collect();
                let clean = rows
                    .iter()
                    .filter(|t| t.clean && !t.assisted && !t.skipped)
                    .count();
                (clean as f64 + 1.) / (rows.len() as f64 + 2.)
                    + if recent.contains(&s.as_str()) { 2. } else { 0. }
            };
            weight(a).total_cmp(&weight(b))
        });
        if let Some((keys, _)) = best {
            p.review_target = Some(keys.clone());
        }
    }
}
pub fn skill(p: &Practice) -> String {
    format!(
        "{}:{}:{}:{}",
        if p.chords { "chord" } else { "note" },
        p.clef as u8,
        p.level as u8,
        p.target()
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join("-")
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn song_coverage_and_misses_are_not_rhythm_accuracy() {
        let mut r = Run::song("song-session".into(), "Test".into(), 3);
        r.song_input(crate::practice::Grade::Correct);
        r.song_input(crate::practice::Grade::Neutral);
        r.song_input(crate::practice::Grade::Wrong);
        r.close(true);
        assert_eq!(r.points, 100);
        assert_eq!(r.song_missed, 2);
        assert_eq!(r.accuracy(), Some(33));
        assert!(r.validate());
        let mut empty = Run::song("empty-session".into(), "Test".into(), 3);
        empty.close(true);
        assert!(empty.meaningful());
        assert_eq!(empty.accuracy(), Some(0));
        empty.song_correct = u32::MAX;
        assert!(!empty.validate());
    }
    #[test]
    fn fair_points_assistance_and_skip() {
        let p = Practice::default();
        let mut t = Training::default();
        t.start("test-session".into(), &p);
        t.answer(&p, false);
        t.mistake();
        t.answer(&p, false);
        t.assist();
        t.answer(&p, false);
        t.answer(&p, true);
        let r = t.run.unwrap();
        assert_eq!(r.points, 140);
        assert_eq!(r.clean(), 1);
        assert_eq!(r.accuracy(), Some(25));
        assert!(r.validate());
    }
    #[test]
    fn adaptive_reviews_weak_keys_without_immediate_repetition() {
        let mut p = Practice::default();
        let mut t = Training {
            adaptive: true,
            ..Default::default()
        };
        t.start("test-session".into(), &p);
        t.mistake();
        t.answer(&p, false);
        t.choose(&mut p);
        assert_ne!(p.target(), vec![60]);
        t.answer(&p, false);
        t.choose(&mut p);
        assert_ne!(p.target(), vec![60]);
        t.answer(&p, false);
        t.choose(&mut p);
        assert_eq!(p.target(), vec![60]);
    }
    #[test]
    fn invalid_scores_and_unfinished_accuracy() {
        let p = Practice::default();
        let mut r = Run::new("test-session".into(), &p);
        assert_eq!(r.accuracy(), None);
        assert!(!r.meaningful());
        r.points = 100;
        assert!(!r.validate());
    }
}

/// Human-readable skill identity without exposing a target in the exercise HUD.
pub fn skill_label(tag: &str) -> Option<String> {
    let fields: Vec<_> = tag.split(':').collect();
    if fields.len() != 4 {
        return None;
    }
    let keys: Vec<u8> = fields[3]
        .split('-')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .ok()?;
    if keys.iter().any(|&k| k > 127) || !keys.windows(2).all(|p| p[0] < p[1]) {
        return None;
    }
    let chords = fields[0] == "chord";
    if (!chords && keys.len() != 1) || (chords && !(3..=4).contains(&keys.len())) {
        return None;
    }
    let p = Practice {
        chords,
        review_target: Some(keys),
        ..Default::default()
    };
    Some(p.label())
}
