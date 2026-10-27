/// Detect the format from bytes, never a filename extension.
pub fn song(bytes: &[u8], title: String) -> Result<tangent_core::music::Song, String> {
    if bytes.starts_with(b"MThd") {
        midi(bytes, title)
    } else {
        tangent_musicxml::import(bytes, title)
    }
}
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
                // PPQ already expresses musical time across every tempo change.
                rhythm: tangent_core::notation::NoteRhythm::from_quarters(
                    (n.end_tick - n.start_tick) as f64 / f64::from(source.ticks_per_quarter),
                ),
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
        composer: None,
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
    tangent_musicxml::import(
        include_bytes!("../../../web/piano/samples/first-light.musicxml"),
        "First light".into(),
    )
    .expect("bundled MusicXML study must be valid")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tangent_core::notation::{NoteRhythm, NoteValue};

    #[test]
    fn midi_rhythm_uses_ticks_across_tempo_changes() {
        // 6/8, 120 BPM; C lasts a quarter with a mid-note change to 240 BPM.
        // D lasts an eighth at 240 BPM. Playback speed never changes these values.
        let events = [
            0, 0xff, 0x58, 4, 6, 3, 24, 8, 0, 0xff, 0x51, 3, 7, 0xa1, 0x20, 0, 0x90, 60, 90, 0x81,
            0x70, 0xff, 0x51, 3, 3, 0xd0, 0x90, 0x81, 0x70, 0x80, 60, 0, 0, 0x90, 62, 90, 0x81,
            0x70, 0x80, 62, 0, 0, 0xff, 0x2f, 0,
        ];
        let mut bytes = b"MThd\0\0\0\x06\0\0\0\x01\x01\xe0MTrk".to_vec();
        bytes.extend((events.len() as u32).to_be_bytes());
        bytes.extend(events);
        let song = midi(&bytes, "Tempo changes".into()).unwrap();
        assert_eq!(song.notes[0].end_us, 375_000);
        assert_eq!(song.notes[1].end_us - song.notes[1].start_us, 125_000);
        assert_eq!(
            song.notes[0].rhythm,
            Some(NoteRhythm {
                value: NoteValue::Quarter,
                dots: 0
            })
        );
        assert_eq!(
            song.notes[1].rhythm,
            Some(NoteRhythm {
                value: NoteValue::Eighth,
                dots: 0
            })
        );
    }

    #[test]
    fn musicxml_falling_rhythm_preserves_authored_type_on_repeat_visits() {
        let song = study();
        assert!(song.notes.iter().all(|n| n.rhythm.is_some()));
        let score = song.notation.as_ref().unwrap();
        for n in score.measures.iter().flat_map(|m| &m.notes) {
            if let Some(id) = n.performance_note
                && !n.tie_from_previous
            {
                assert_eq!(
                    song.notes.iter().find(|p| p.id == id).unwrap().rhythm,
                    NoteRhythm::written(n)
                );
            }
        }
    }
    #[test]
    fn written_tuplet_type_wins_over_duration_and_missing_type_is_inferred() {
        let xml = br#"<score-partwise version="4.0"><part-list>
            <score-part id="P1"><part-name>Piano</part-name></score-part>
            </part-list><part id="P1"><measure number="1"><attributes>
            <divisions>3</divisions><time><beats>4</beats><beat-type>4</beat-type></time>
            <clef><sign>G</sign><line>2</line></clef></attributes>
            <note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration>
            <type>eighth</type><time-modification><actual-notes>3</actual-notes>
            <normal-notes>2</normal-notes></time-modification></note>
            <note><pitch><step>D</step><octave>4</octave></pitch><duration>9</duration></note>
            </measure></part></score-partwise>"#;
        let song = song(xml, "Rhythm".into()).unwrap();
        assert_eq!(
            song.notes[0].rhythm,
            Some(NoteRhythm {
                value: NoteValue::Eighth,
                dots: 0
            })
        );
        assert_eq!(
            song.notes[1].rhythm,
            Some(NoteRhythm {
                value: NoteValue::Half,
                dots: 1
            })
        );
    }

    #[test]
    fn musicxml_preserves_composer_metadata() {
        let song = study();
        assert_eq!(song.composer.as_deref(), Some("Tangent contributors"));
    }
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
