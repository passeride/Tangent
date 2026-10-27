use serde::Serialize;
/// Approximate sharp spelling for unscripted performance data. Not engraving.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct ReadingPitch {
    pub step: i16,
    pub sharp: bool,
    pub bass: bool,
}
pub fn reading_pitch(key: u8) -> ReadingPitch {
    const STEPS: [i16; 12] = [0, 0, 1, 1, 2, 3, 3, 4, 4, 5, 5, 6];
    ReadingPitch {
        step: (key as i16 / 12 - 1) * 7 + STEPS[key as usize % 12],
        sharp: matches!(key % 12, 1 | 3 | 6 | 8 | 10),
        bass: key < 60,
    }
}
/// Diatonic steps for short ledger lines outside a five-line staff.
pub fn ledger_steps(step: i16, bottom: i16) -> Vec<i16> {
    let top = bottom + 8;
    if step < bottom {
        ((step..bottom).filter(|s| (bottom - s) % 2 == 0)).collect()
    } else {
        ((top + 1..=step).filter(|s| (s - bottom) % 2 == 0)).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn middle_c_and_extreme_ledgers() {
        assert_eq!(reading_pitch(60).step, 28);
        assert_eq!(ledger_steps(28, 30), vec![28]); // treble middle C
        assert_eq!(ledger_steps(28, 18), vec![28]); // bass middle C
        assert_eq!(ledger_steps(35, 30), Vec::<i16>::new());
        assert_eq!(ledger_steps(24, 30), vec![24, 26, 28]);
    }
}

/// Optional authored notation; independent from absolute performance time.
/// MusicXML supplies this; MIDI projections use ReadingPitch instead.
#[derive(Debug, Clone, Serialize)]
pub struct NotatedScore {
    pub staves: Vec<NotatedStaff>,
    pub measures: Vec<NotatedMeasure>,
    /// Written measure -> performance visits, including repeats and endings.
    pub playback: Vec<MeasureVisit>,
}
#[derive(Debug, Clone, Serialize)]
pub enum Clef {
    Treble,
    Bass,
}
#[derive(Debug, Clone, Serialize)]
pub struct NotatedStaff {
    pub id: u16,
    pub part: usize,
    pub clef: Clef,
}
/// Exact quarter-note fractions, including tuplets. A denominator cannot be zero.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct QuarterFraction {
    pub numerator: u32,
    pub denominator: std::num::NonZeroU32,
}
#[derive(Debug, Clone, Serialize)]
pub struct NotatedMeasure {
    pub label: String,
    pub duration: QuarterFraction,
    pub notes: Vec<WrittenNote>,
    pub meter: TimeSignature,
    pub key_fifths: i8,
    pub initial_quarter_bpm: f64,
    pub tempos: Vec<TempoMark>,
    pub repeat_start: bool,
    pub repeat_end: Option<u8>,
    pub ending: Option<Ending>,
}
#[derive(Debug, Clone, Serialize)]
pub struct WrittenNote {
    pub performance_note: Option<usize>,
    pub staff: u16,
    pub voice: String,
    pub onset: QuarterFraction,
    pub duration: QuarterFraction,
    /// None denotes a rest. Simultaneous notes share an onset, not a block.
    pub pitch: Option<crate::music::WrittenPitch>,
    pub value: Option<NoteValue>,
    pub dots: u8,
    pub tie_from_previous: bool,
    pub tie_to_next: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TimeSignature {
    pub beats: u8,
    pub beat_type: u16,
}
impl Default for TimeSignature {
    fn default() -> Self {
        Self {
            beats: 4,
            beat_type: 4,
        }
    }
}
#[derive(Debug, Clone, Serialize)]
pub struct TempoMark {
    pub onset: QuarterFraction,
    pub quarter_bpm: f64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Ending {
    pub numbers: Vec<u8>,
    pub start: bool,
    pub end: bool,
    pub open_end: bool,
}
#[derive(Debug, Clone, Serialize)]
pub struct MeasureVisit {
    pub measure: usize,
    pub pass: u8,
    pub start_us: u64,
    pub end_us: u64,
    pub timing: Vec<BeatTime>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum NoteValue {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
}

/// A written value, or a conservative reading estimate from MIDI source beats.
/// This does not change playback, infer rests, or reconstruct authored notation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct NoteRhythm {
    pub value: NoteValue,
    pub dots: u8,
}
impl NoteRhythm {
    pub fn written(note: &WrittenNote) -> Option<Self> {
        note.value
            .map(|value| Self {
                value,
                dots: note.dots,
            })
            .or_else(|| Self::from_quarters(note.duration.quarters()))
    }

    /// Accept at most 8% articulation/rounding error. Irregular spans and tuplets
    /// without an authored type stay unknown instead of acquiring a false flag.
    pub fn from_quarters(quarters: f64) -> Option<Self> {
        if !quarters.is_finite() || quarters <= 0. {
            return None;
        }
        let values = [
            NoteValue::Whole,
            NoteValue::Half,
            NoteValue::Quarter,
            NoteValue::Eighth,
            NoteValue::Sixteenth,
            NoteValue::ThirtySecond,
            NoteValue::SixtyFourth,
        ];
        let mut best = None;
        let mut error = f64::INFINITY;
        for (index, value) in values.into_iter().enumerate() {
            for dots in 0..=3_u8 {
                let expected = 4. / 2_f64.powi(index as i32) * (2. - 2_f64.powi(-i32::from(dots)));
                let distance = (quarters - expected).abs() / expected;
                // Prefer a simple articulated value over a nearby multiply
                // dotted value (e.g. 0.96 quarters is not a triple-dotted eighth).
                let cost = distance + f64::from(dots) * 0.02;
                if distance <= 0.08 && cost < error {
                    error = cost;
                    best = Some(Self { value, dots });
                }
            }
        }
        best
    }
}

#[cfg(test)]
mod rhythm_tests {
    use super::*;
    #[test]
    fn beat_values_and_dots_do_not_depend_on_tempo_or_meter_denominator() {
        for (q, value, dots) in [
            (4., NoteValue::Whole, 0),
            (3., NoteValue::Half, 1),
            (1., NoteValue::Quarter, 0),
            (0.75, NoteValue::Eighth, 1),
            (0.25, NoteValue::Sixteenth, 0),
            (0.125, NoteValue::ThirtySecond, 0),
            (0.0625, NoteValue::SixtyFourth, 0),
            (1.75, NoteValue::Quarter, 2),
        ] {
            assert_eq!(
                NoteRhythm::from_quarters(q),
                Some(NoteRhythm { value, dots })
            );
        }
        assert_eq!(
            NoteRhythm::from_quarters(0.96).unwrap().value,
            NoteValue::Quarter
        );
        for q in [0., -1., f64::NAN, f64::INFINITY, 1. / 3., 0.6, 20.] {
            assert!(NoteRhythm::from_quarters(q).is_none());
        }
    }
}
impl QuarterFraction {
    pub fn quarters(self) -> f64 {
        self.numerator as f64 / self.denominator.get() as f64
    }
}

/// Piecewise linear timing within one performance visit, after tempo integration.
#[derive(Debug, Clone, Serialize)]
pub struct BeatTime {
    pub quarter: QuarterFraction,
    pub offset_us: u64,
}
impl MeasureVisit {
    pub fn quarter_at_time(&self, offset_us: u64) -> f64 {
        let end = self.timing.partition_point(|p| p.offset_us <= offset_us);
        if end == 0 {
            return 0.;
        }
        if end >= self.timing.len() {
            return self.timing.last().map_or(0., |p| p.quarter.quarters());
        }
        let a = &self.timing[end - 1];
        let b = &self.timing[end];
        let fraction = (offset_us - a.offset_us) as f64 / (b.offset_us - a.offset_us).max(1) as f64;
        a.quarter.quarters() + (b.quarter.quarters() - a.quarter.quarters()) * fraction
    }
    pub fn time_at_quarter(&self, quarter: f64) -> u64 {
        if !quarter.is_finite() || quarter <= 0. {
            return 0;
        }
        let end = self
            .timing
            .partition_point(|p| p.quarter.quarters() <= quarter);
        if end == 0 {
            return 0;
        }
        if end >= self.timing.len() {
            return self.timing.last().map_or(0, |p| p.offset_us);
        }
        let a = &self.timing[end - 1];
        let b = &self.timing[end];
        let fraction =
            (quarter - a.quarter.quarters()) / (b.quarter.quarters() - a.quarter.quarters());
        a.offset_us + ((b.offset_us - a.offset_us) as f64 * fraction).round() as u64
    }
}
