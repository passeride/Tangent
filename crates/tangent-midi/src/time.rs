use crate::{ErrorCode, ImportError, Tempo};

struct Segment {
    tick: u64,
    scaled_us: u128,
    tempo: u32,
}
/// Integer integration retains the fractional remainder across tempo changes.
/// Exact source ticks stay in the model; microsecond timestamps round down once.
pub struct TempoMap {
    division: u16,
    segments: Vec<Segment>,
}
impl TempoMap {
    pub fn new(division: u16, tempos: &[Tempo]) -> Result<Self, ImportError> {
        if division == 0 {
            return Err(ImportError::new(
                ErrorCode::UnsupportedTiming,
                "Metrical division must be positive",
            ));
        }
        if tempos.windows(2).any(|pair| pair[0].tick >= pair[1].tick) {
            return Err(ImportError::new(
                ErrorCode::InvalidTempo,
                "Tempo positions must be strictly increasing",
            ));
        }
        let mut segments = vec![Segment {
            tick: 0,
            scaled_us: 0,
            tempo: 500_000,
        }];
        for t in tempos {
            if t.microseconds_per_quarter == 0 {
                return Err(ImportError::new(
                    ErrorCode::InvalidTempo,
                    "Tempo must be positive",
                ));
            }
            let last = segments.last().unwrap();
            if t.tick < last.tick {
                return Err(ImportError::new(
                    ErrorCode::InvalidTempo,
                    "Tempo map must be sorted",
                ));
            }
            let scaled = last.scaled_us + u128::from(t.tick - last.tick) * u128::from(last.tempo);
            if t.tick == last.tick {
                if t.tick != 0 {
                    return Err(ImportError::new(
                        ErrorCode::ConflictingTempo,
                        "Duplicate tempo position",
                    ));
                }
                segments.pop();
            }
            segments.push(Segment {
                tick: t.tick,
                scaled_us: scaled,
                tempo: t.microseconds_per_quarter,
            });
        }
        Ok(Self { division, segments })
    }
    pub fn microseconds_at(&self, tick: u64) -> Result<u64, ImportError> {
        let i = self.segments.partition_point(|s| s.tick <= tick) - 1;
        let s = &self.segments[i];
        let value = (s.scaled_us + u128::from(tick - s.tick) * u128::from(s.tempo))
            / u128::from(self.division);
        u64::try_from(value).map_err(|_| {
            ImportError::new(
                ErrorCode::TimeOverflow,
                "MIDI duration cannot fit microsecond timestamps",
            )
        })
    }
}
