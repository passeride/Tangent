use tangent_core::music::*;
use tangent_midi::{ChannelMessage as M, EventKind};
/// MIDI is one source adapter. The application Song has no MIDI header/ticks.
pub fn midi(bytes: &[u8], title: String) -> Result<Song, String> {
    let source = tangent_midi::import(bytes).map_err(|e| format!("{e:?}"))?;
    let mut notes = Vec::new();
    let mut events = Vec::new();
    let mut parts = Vec::new();
    let warnings = source
        .diagnostics
        .iter()
        .map(|d| format!("{:?} in track {} at tick {}", d.kind, d.track + 1, d.tick))
        .collect();
    for track in source.tracks {
        let track_name = track
            .name
            .unwrap_or_else(|| format!("Track {}", track.index + 1));
        let mut lanes = std::collections::BTreeMap::new();
        for n in track.notes {
            let part = *lanes.entry((n.port, n.channel)).or_insert_with(|| {
                let id = parts.len();
                parts.push(Part {
                    name: format!("{} · channel {}", track_name, n.channel + 1),
                    percussion: n.is_percussion(),
                });
                id
            });
            notes.push(PerformanceNote {
                id: notes.len(),
                part,
                key: n.key,
                velocity: n.velocity,
                start_us: n.start_us,
                end_us: n.end_us,
                written: None,
            });
        }
        for e in track.events {
            if let EventKind::Channel {
                port,
                channel,
                message,
            } = e.kind
            {
                let message = match message {
                    M::NoteOn { key, velocity } => PerformanceEvent::On { key, velocity },
                    M::NoteOff { key, .. } => PerformanceEvent::Off { key },
                    M::Controller { controller, value } => {
                        PerformanceEvent::Control { controller, value }
                    }
                    M::PitchBend { value } => PerformanceEvent::Bend { value },
                    M::Program { program } => PerformanceEvent::Program { program },
                    _ => continue,
                };
                events.push(TimedEvent {
                    time_us: e.time_us,
                    port,
                    channel,
                    message,
                });
            }
        }
    }
    notes.sort_by_key(|n| (n.start_us, n.id));
    // Stable sort preserves track/source order when timestamps tie.
    events.sort_by_key(|e| e.time_us);
    Ok(Song {
        title,
        duration_us: source.duration_us,
        parts,
        notes,
        events,
        notation: None,
        warnings,
    })
}
pub fn study() -> Song {
    let mut notes = Vec::new();
    let mut events = Vec::new();
    for bar in 0..4 {
        for (beat, key) in [60, 64, 67, 72, 71, 67, 64, 62].into_iter().enumerate() {
            let start_us = 2_000_000 + (bar * 8 + beat) as u64 * 420_000;
            for (part, key, duration) in [(0, key, 330_000), (1, [48, 53, 55, 48][bar], 1_500_000)]
            {
                if part == 1 && beat % 4 != 0 {
                    continue;
                }
                notes.push(PerformanceNote {
                    id: notes.len(),
                    part,
                    key,
                    velocity: 85,
                    start_us,
                    end_us: start_us + duration,
                    written: None,
                });
                events.push(TimedEvent {
                    time_us: start_us,
                    port: 0,
                    channel: part as u8,
                    message: PerformanceEvent::On { key, velocity: 85 },
                });
                events.push(TimedEvent {
                    time_us: start_us + duration,
                    port: 0,
                    channel: part as u8,
                    message: PerformanceEvent::Off { key },
                });
            }
        }
    }
    notes.sort_by_key(|n| (n.start_us, n.id));
    events.sort_by_key(|e| e.time_us);
    Song {
        title: "First light · original study".into(),
        duration_us: 16_000_000,
        parts: vec![
            Part {
                name: "Melody".into(),
                percussion: false,
            },
            Part {
                name: "Foundation".into(),
                percussion: false,
            },
        ],
        notes,
        events,
        notation: None,
        warnings: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mixed_channel_track_keeps_drums_out_of_the_piano_lane() {
        // New independent type-0 sequence: C4 and a drum at t0, releases at 0.5s.
        let events = [
            0, 0x90, 60, 90, 0, 0x99, 35, 100, 0x83, 0x60, 0x80, 60, 0, 0, 0x89, 35, 0, 0, 0xff,
            0x2f, 0,
        ];
        let mut bytes = b"MThd\0\0\0\x06\0\0\0\x01\x01\xe0MTrk".to_vec();
        bytes.extend((events.len() as u32).to_be_bytes());
        bytes.extend(events);
        let song = midi(&bytes, "Mixed".into()).unwrap();
        assert_eq!(song.parts.len(), 2);
        assert_eq!(song.notes.len(), 2);
        assert_eq!(song.notes[0].end_us, 500_000);
        assert!(!song.parts[song.notes[0].part].percussion);
        assert!(song.parts[song.notes[1].part].percussion);
        assert!(song.notation.is_none());
        assert!(song.notes.iter().all(|n| n.written.is_none()));
    }
}
