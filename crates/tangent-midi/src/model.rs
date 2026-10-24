use serde::Serialize;

/// Source timing is retained exactly; this is performance data, not notation.
#[derive(Debug, Clone, Serialize)]
pub struct Song {
    pub format: u8,
    pub ticks_per_quarter: u16,
    pub duration_ticks: u64,
    pub duration_us: u64,
    pub tracks: Vec<Track>,
    pub tempos: Vec<Tempo>,
    pub diagnostics: Vec<Diagnostic>,
}
#[derive(Debug, Clone, Serialize)]
pub struct Tempo {
    pub tick: u64,
    pub microseconds_per_quarter: u32,
}
#[derive(Debug, Clone, Serialize)]
pub struct Track {
    pub index: usize,
    pub name: Option<String>,
    pub end_tick: u64,
    pub notes: Vec<Note>,
    pub events: Vec<Event>,
}
#[derive(Debug, Clone, Serialize)]
pub struct Note {
    pub source_event: usize,
    pub port: u8,
    pub channel: u8,
    pub key: u8,
    pub velocity: u8,
    pub release_velocity: u8,
    pub start_tick: u64,
    pub end_tick: u64,
    pub start_us: u64,
    pub end_us: u64,
    pub ending: NoteEnding,
}
impl Note {
    pub fn is_percussion(&self) -> bool {
        self.channel == 9
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum NoteEnding {
    Release,
    ChannelMode,
    TrackEnd,
}
#[derive(Debug, Clone, Serialize)]
pub struct Event {
    pub source_event: usize,
    pub tick: u64,
    pub time_us: u64,
    pub kind: EventKind,
}
#[derive(Debug, Clone, Serialize)]
pub enum EventKind {
    Channel {
        port: u8,
        channel: u8,
        message: ChannelMessage,
    },
    Text {
        category: String,
        raw: Vec<u8>,
        display: String,
    },
    Tempo {
        microseconds_per_quarter: u32,
    },
    TimeSignature {
        numerator: u8,
        denominator_power: u8,
        clocks_per_click: u8,
        thirty_seconds_per_quarter: u8,
    },
    KeySignature {
        sharps: i8,
        minor: bool,
    },
    Port {
        port: u8,
    },
    Meta {
        tag: u8,
        data: Vec<u8>,
    },
    SysEx {
        escaped: bool,
        data: Vec<u8>,
    },
    EndOfTrack,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ChannelMessage {
    NoteOn { key: u8, velocity: u8 },
    NoteOff { key: u8, velocity: u8 },
    PolyPressure { key: u8, pressure: u8 },
    Controller { controller: u8, value: u8 },
    Program { program: u8 },
    Pressure { pressure: u8 },
    PitchBend { value: u16 },
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum DiagnosticKind {
    OrphanRelease,
    UnreleasedNote,
    OverlappingKey,
}
#[derive(Debug, Clone, Serialize)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub track: usize,
    pub source_event: usize,
    pub tick: u64,
    pub port: u8,
    pub channel: u8,
    pub key: u8,
}
