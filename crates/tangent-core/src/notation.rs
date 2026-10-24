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
/// No importer supplies this yet. MIDI projections use ReadingPitch instead.
#[derive(Debug, Clone, Serialize)]
pub struct NotatedScore {
    pub staves: Vec<NotatedStaff>,
    pub measures: Vec<NotatedMeasure>,
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
    pub tie_from_previous: bool,
    pub tie_to_next: bool,
}
