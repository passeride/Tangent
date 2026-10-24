//! Bounded SMF import from bytes. No filesystem, device, rendering or audio APIs.
mod error;
mod model;
mod time;
pub use error::*;
use midly::{Format, MetaMessage, MidiMessage, Timing, TrackEventKind};
pub use model::*;
use std::collections::{BTreeMap, VecDeque};
pub use time::TempoMap;

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub bytes: usize,
    pub tracks: usize,
    pub events: usize,
    pub duration_us: u64,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            bytes: 16 * 1024 * 1024,
            tracks: 256,
            events: 250_000,
            duration_us: 24 * 60 * 60 * 1_000_000,
        }
    }
}
fn fail(code: ErrorCode, text: &str) -> ImportError {
    ImportError::new(code, text)
}

pub fn import(bytes: &[u8]) -> Result<Song, ImportError> {
    import_with_limits(bytes, Limits::default())
}

pub fn import_with_limits(bytes: &[u8], limits: Limits) -> Result<Song, ImportError> {
    if bytes.len() > limits.bytes {
        return Err(fail(
            ErrorCode::LimitExceeded,
            "MIDI input exceeds the byte limit",
        ));
    }
    // Lazy parsing lets us reject dense input before collecting all events.
    let (header, source) =
        midly::parse(bytes).map_err(|e| ImportError::new(ErrorCode::InvalidFile, e.to_string()))?;
    let format = match header.format {
        Format::SingleTrack => 0,
        Format::Parallel => 1,
        Format::Sequential => {
            return Err(fail(
                ErrorCode::UnsupportedFormat,
                "SMF type 2 contains independent sequences and is not supported yet",
            ));
        }
    };
    let division = match header.timing {
        Timing::Metrical(d) if d.as_int() > 0 => d.as_int(),
        _ => {
            return Err(fail(
                ErrorCode::UnsupportedTiming,
                "Only positive metrical MIDI timing is currently supported",
            ));
        }
    };
    let mut tracks = Vec::new();
    let mut tempos = BTreeMap::from([(0, 500_000)]);
    let mut authored = BTreeMap::new();
    let mut diagnostics = Vec::new();
    let mut count = 0usize;
    for (index, source_track) in source.enumerate() {
        if index >= limits.tracks {
            return Err(fail(ErrorCode::LimitExceeded, "MIDI track limit exceeded"));
        }
        let source_track = source_track
            .map_err(|e| ImportError::new(ErrorCode::InvalidFile, e.to_string()).at(index, 0))?;
        let mut track = Track {
            index,
            name: None,
            end_tick: 0,
            notes: Vec::new(),
            events: Vec::new(),
        };
        let mut pending: BTreeMap<(u8, u8, u8), VecDeque<Note>> = BTreeMap::new();
        let mut port = 0;
        let mut ended = false;
        for (event_index, event) in source_track.enumerate() {
            count += 1;
            if count > limits.events {
                return Err(fail(ErrorCode::LimitExceeded, "MIDI event limit exceeded")
                    .at(index, event_index));
            }
            let event = event.map_err(|e| {
                ImportError::new(ErrorCode::InvalidFile, e.to_string()).at(index, event_index)
            })?;
            if ended {
                return Err(fail(
                    ErrorCode::InvalidTrack,
                    "Events after EndOfTrack are not supported",
                )
                .at(index, event_index));
            }
            track.end_tick = track
                .end_tick
                .checked_add(u64::from(event.delta.as_int()))
                .ok_or_else(|| {
                    fail(ErrorCode::TimeOverflow, "Tick position overflow").at(index, event_index)
                })?;
            let tick = track.end_tick;
            let kind = match event.kind {
                TrackEventKind::Midi { channel, message } => {
                    let channel = channel.as_int();
                    let message = convert(message);
                    match message {
                        ChannelMessage::NoteOn { key, velocity } => {
                            let queue = pending.entry((port, channel, key)).or_default();
                            if !queue.is_empty() {
                                diagnostics.push(Diagnostic {
                                    kind: DiagnosticKind::OverlappingKey,
                                    track: index,
                                    source_event: event_index,
                                    tick,
                                    port,
                                    channel,
                                    key,
                                });
                            }
                            queue.push_back(Note {
                                source_event: event_index,
                                port,
                                channel,
                                key,
                                velocity,
                                release_velocity: 0,
                                start_tick: tick,
                                end_tick: tick,
                                start_us: 0,
                                end_us: 0,
                                ending: NoteEnding::Release,
                            });
                        }
                        ChannelMessage::NoteOff { key, velocity } => {
                            if let Some(note) = pending
                                .get_mut(&(port, channel, key))
                                .and_then(VecDeque::pop_front)
                            {
                                track
                                    .notes
                                    .push(close(note, tick, velocity, NoteEnding::Release));
                                if pending
                                    .get(&(port, channel, key))
                                    .is_some_and(VecDeque::is_empty)
                                {
                                    pending.remove(&(port, channel, key));
                                }
                            } else {
                                diagnostics.push(Diagnostic {
                                    kind: DiagnosticKind::OrphanRelease,
                                    track: index,
                                    source_event: event_index,
                                    tick,
                                    port,
                                    channel,
                                    key,
                                });
                            }
                        }
                        ChannelMessage::Controller {
                            controller: 120 | 123,
                            ..
                        } => {
                            pending.retain(|(p, c, _), queue| {
                                if *p == port && *c == channel {
                                    for note in queue.drain(..) {
                                        track.notes.push(close(
                                            note,
                                            tick,
                                            0,
                                            NoteEnding::ChannelMode,
                                        ));
                                    }
                                    false
                                } else {
                                    true
                                }
                            });
                        }
                        _ => {}
                    }
                    EventKind::Channel {
                        port,
                        channel,
                        message,
                    }
                }
                TrackEventKind::Meta(m) => {
                    match m {
                        MetaMessage::Tempo(value) => {
                            let value = value.as_int();
                            if value == 0 {
                                return Err(fail(ErrorCode::InvalidTempo, "Tempo zero is invalid")
                                    .at(index, event_index));
                            }
                            if authored.insert(tick, value).is_some_and(|old| old != value) {
                                return Err(fail(
                                    ErrorCode::ConflictingTempo,
                                    "Conflicting tempo declarations at one tick",
                                )
                                .at(index, event_index));
                            }
                            tempos.insert(tick, value);
                        }
                        MetaMessage::TrackName(name) => {
                            if track.name.is_none() {
                                track.name = Some(String::from_utf8_lossy(name).into_owned());
                            }
                        }
                        MetaMessage::MidiPort(p) => port = p.as_int(),
                        MetaMessage::EndOfTrack => ended = true,
                        _ => {}
                    }
                    metadata(m)
                }
                TrackEventKind::SysEx(data) => EventKind::SysEx {
                    escaped: false,
                    data: data.to_vec(),
                },
                TrackEventKind::Escape(data) => EventKind::SysEx {
                    escaped: true,
                    data: data.to_vec(),
                },
            };
            track.events.push(Event {
                source_event: event_index,
                tick,
                time_us: 0,
                kind,
            });
        }
        if !ended {
            return Err(
                fail(ErrorCode::InvalidTrack, "MIDI track is missing EndOfTrack")
                    .at(index, track.events.len()),
            );
        }
        for queue in pending.into_values() {
            for note in queue {
                diagnostics.push(Diagnostic {
                    kind: DiagnosticKind::UnreleasedNote,
                    track: index,
                    source_event: note.source_event,
                    tick: track.end_tick,
                    port: note.port,
                    channel: note.channel,
                    key: note.key,
                });
                track
                    .notes
                    .push(close(note, track.end_tick, 0, NoteEnding::TrackEnd));
            }
        }
        track.notes.sort_by_key(|n| (n.start_tick, n.source_event));
        tracks.push(track);
    }
    if tracks.is_empty() || format == 0 && tracks.len() != 1 {
        return Err(fail(
            ErrorCode::InvalidTrack,
            "SMF track count is inconsistent with its format",
        ));
    }
    // Check the declared count as well as what the lazy iterator actually yielded.
    let declared = bytes
        .get(10..12)
        .map(|b| usize::from(u16::from_be_bytes([b[0], b[1]])));
    if declared != Some(tracks.len()) {
        return Err(fail(
            ErrorCode::InvalidTrack,
            "Declared and actual MIDI track counts differ",
        ));
    }
    let tempos: Vec<_> = tempos
        .into_iter()
        .map(|(tick, microseconds_per_quarter)| Tempo {
            tick,
            microseconds_per_quarter,
        })
        .collect();
    let time = TempoMap::new(division, &tempos)?;
    let duration_ticks = tracks.iter().map(|t| t.end_tick).max().unwrap_or(0);
    let duration_us = time.microseconds_at(duration_ticks)?;
    if duration_us > limits.duration_us {
        return Err(fail(
            ErrorCode::LimitExceeded,
            "MIDI duration exceeds the supported limit",
        ));
    }
    for track in &mut tracks {
        for e in &mut track.events {
            e.time_us = time.microseconds_at(e.tick)?;
        }
        for n in &mut track.notes {
            n.start_us = time.microseconds_at(n.start_tick)?;
            n.end_us = time.microseconds_at(n.end_tick)?;
        }
    }
    Ok(Song {
        format,
        ticks_per_quarter: division,
        duration_ticks,
        duration_us,
        tracks,
        tempos,
        diagnostics,
    })
}
fn close(mut note: Note, end: u64, velocity: u8, ending: NoteEnding) -> Note {
    note.end_tick = end;
    note.release_velocity = velocity;
    note.ending = ending;
    note
}
fn convert(message: MidiMessage) -> ChannelMessage {
    match message {
        MidiMessage::NoteOn { key, vel } if vel.as_int() != 0 => ChannelMessage::NoteOn {
            key: key.as_int(),
            velocity: vel.as_int(),
        },
        MidiMessage::NoteOn { key, vel } | MidiMessage::NoteOff { key, vel } => {
            ChannelMessage::NoteOff {
                key: key.as_int(),
                velocity: vel.as_int(),
            }
        }
        MidiMessage::Aftertouch { key, vel } => ChannelMessage::PolyPressure {
            key: key.as_int(),
            pressure: vel.as_int(),
        },
        MidiMessage::Controller { controller, value } => ChannelMessage::Controller {
            controller: controller.as_int(),
            value: value.as_int(),
        },
        MidiMessage::ProgramChange { program } => ChannelMessage::Program {
            program: program.as_int(),
        },
        MidiMessage::ChannelAftertouch { vel } => ChannelMessage::Pressure {
            pressure: vel.as_int(),
        },
        MidiMessage::PitchBend { bend } => ChannelMessage::PitchBend {
            value: bend.0.as_int(),
        },
    }
}
fn text(category: &str, raw: &[u8]) -> EventKind {
    EventKind::Text {
        category: category.into(),
        raw: raw.to_vec(),
        display: String::from_utf8_lossy(raw).into_owned(),
    }
}
fn metadata(m: MetaMessage<'_>) -> EventKind {
    match m {
        MetaMessage::TrackName(b) => text("track_name", b),
        MetaMessage::InstrumentName(b) => text("instrument_name", b),
        MetaMessage::Text(b) => text("text", b),
        MetaMessage::Copyright(b) => text("copyright", b),
        MetaMessage::Lyric(b) => text("lyric", b),
        MetaMessage::Marker(b) => text("marker", b),
        MetaMessage::CuePoint(b) => text("cue", b),
        MetaMessage::ProgramName(b) => text("program_name", b),
        MetaMessage::DeviceName(b) => text("device_name", b),
        MetaMessage::Tempo(v) => EventKind::Tempo {
            microseconds_per_quarter: v.as_int(),
        },
        MetaMessage::TimeSignature(n, d, c, b) => EventKind::TimeSignature {
            numerator: n,
            denominator_power: d,
            clocks_per_click: c,
            thirty_seconds_per_quarter: b,
        },
        MetaMessage::KeySignature(sharps, minor) => EventKind::KeySignature { sharps, minor },
        MetaMessage::MidiPort(p) => EventKind::Port { port: p.as_int() },
        MetaMessage::EndOfTrack => EventKind::EndOfTrack,
        MetaMessage::TrackNumber(n) => EventKind::Meta {
            tag: 0,
            data: n.map(|n| n.to_be_bytes().to_vec()).unwrap_or_default(),
        },
        MetaMessage::MidiChannel(c) => EventKind::Meta {
            tag: 0x20,
            data: vec![c.as_int()],
        },
        MetaMessage::SequencerSpecific(data) => EventKind::Meta {
            tag: 0x7f,
            data: data.to_vec(),
        },
        MetaMessage::Unknown(tag, data) => EventKind::Meta {
            tag,
            data: data.to_vec(),
        },
        MetaMessage::SmpteOffset(t) => EventKind::Meta {
            tag: 0x54,
            data: vec![
                ((t.fps() as u8) << 5) | t.hour(),
                t.minute(),
                t.second(),
                t.frame(),
                t.subframe(),
            ],
        },
    }
}
