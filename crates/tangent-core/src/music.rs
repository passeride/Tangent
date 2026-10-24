use serde::Serialize;
/// Stable within a loaded song; never a visual entity or a file offset.
#[derive(Debug, Clone, Serialize)]
pub struct PerformanceNote {
    pub id: usize,
    pub part: usize,
    pub key: u8,
    pub velocity: u8,
    pub start_us: u64,
    pub end_us: u64,
    /// Authored spelling is optional. MIDI adapters must not fabricate it.
    pub written: Option<WrittenPitch>,
}
#[derive(Debug, Clone, Serialize)]
pub struct WrittenPitch {
    pub letter: PitchLetter,
    pub alteration: i8,
    pub octave: i8,
}
#[derive(Debug, Clone, Copy, Serialize)]
pub enum PitchLetter {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}
#[derive(Debug, Clone, Serialize)]
pub struct Part {
    pub name: String,
    pub percussion: bool,
}
#[derive(Debug, Clone, Serialize)]
pub struct Song {
    pub title: String,
    pub duration_us: u64,
    pub parts: Vec<Part>,
    pub notes: Vec<PerformanceNote>,
    /// Timing and notation are distinct: absence is meaningful.
    pub notation: Option<crate::notation::NotatedScore>,
    pub warnings: Vec<String>,
    pub events: Vec<TimedEvent>,
}
#[derive(Debug, Clone, Serialize)]
pub struct TimedEvent {
    pub time_us: u64,
    pub port: u8,
    pub channel: u8,
    pub message: PerformanceEvent,
}
#[derive(Debug, Clone, Serialize)]
pub enum PerformanceEvent {
    On { key: u8, velocity: u8 },
    Off { key: u8 },
    Control { controller: u8, value: u8 },
    Bend { value: u16 },
    Program { program: u8 },
}
