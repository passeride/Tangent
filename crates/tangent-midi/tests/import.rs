//! New synthetic SMFs generated here using the independent midly writer.
use midly::{
    Format, Header, MetaMessage as Meta, MidiMessage as Msg, Smf, Timing, TrackEvent,
    TrackEventKind as Kind,
};
use tangent_midi::*;
fn event(delta: u32, kind: Kind<'static>) -> TrackEvent<'static> {
    TrackEvent {
        delta: delta.into(),
        kind,
    }
}
fn on(ch: u8, key: u8, vel: u8) -> Kind<'static> {
    Kind::Midi {
        channel: ch.into(),
        message: Msg::NoteOn {
            key: key.into(),
            vel: vel.into(),
        },
    }
}
fn off(ch: u8, key: u8) -> Kind<'static> {
    Kind::Midi {
        channel: ch.into(),
        message: Msg::NoteOff {
            key: key.into(),
            vel: 40.into(),
        },
    }
}
fn cc(ch: u8, controller: u8, value: u8) -> Kind<'static> {
    Kind::Midi {
        channel: ch.into(),
        message: Msg::Controller {
            controller: controller.into(),
            value: value.into(),
        },
    }
}
fn end(delta: u32) -> TrackEvent<'static> {
    event(delta, Kind::Meta(Meta::EndOfTrack))
}
fn encode(format: Format, timing: Timing, tracks: Vec<Vec<TrackEvent<'static>>>) -> Vec<u8> {
    let smf = Smf {
        header: Header::new(format, timing),
        tracks,
    };
    let mut bytes = Vec::new();
    smf.write_std(&mut bytes).unwrap();
    bytes
}
fn single(events: Vec<TrackEvent<'static>>) -> Vec<u8> {
    encode(
        Format::SingleTrack,
        Timing::Metrical(480.into()),
        vec![events],
    )
}
fn basic() -> Vec<u8> {
    single(vec![
        event(0, on(0, 60, 96)),
        event(480, off(0, 60)),
        end(480),
    ])
}
#[test]
fn key_spans_default_tempo_and_trailing_silence() {
    let s = import(&basic()).unwrap();
    assert_eq!(s.format, 0);
    assert_eq!(s.ticks_per_quarter, 480);
    assert_eq!((s.duration_ticks, s.duration_us), (960, 1_000_000));
    let n = &s.tracks[0].notes[0];
    assert_eq!(
        (n.start_us, n.end_us, n.velocity, n.release_velocity),
        (0, 500_000, 96, 40)
    );
    assert_eq!(n.ending, NoteEnding::Release);
    assert!(s.diagnostics.is_empty());
}
#[test]
fn tempo_in_another_track_and_meter_metadata() {
    let b = encode(
        Format::Parallel,
        Timing::Metrical(480.into()),
        vec![
            vec![
                event(0, Kind::Meta(Meta::Tempo(1_000_000.into()))),
                event(480, Kind::Meta(Meta::Tempo(250_000.into()))),
                end(480),
            ],
            vec![
                event(0, Kind::Meta(Meta::TrackName(b"Piano"))),
                event(0, Kind::Meta(Meta::TimeSignature(3, 3, 24, 8))),
                event(0, Kind::Meta(Meta::KeySignature(-2, true))),
                event(0, on(2, 48, 70)),
                event(960, off(2, 48)),
                end(0),
            ],
        ],
    );
    let s = import(&b).unwrap();
    assert_eq!(s.tracks.len(), 2);
    assert_eq!(s.duration_us, 1_250_000);
    assert_eq!(s.tracks[1].name.as_deref(), Some("Piano"));
    assert!(matches!(
        s.tracks[1].events[1].kind,
        EventKind::TimeSignature {
            numerator: 3,
            denominator_power: 3,
            ..
        }
    ));
    assert!(matches!(
        s.tracks[1].events[2].kind,
        EventKind::KeySignature {
            sharps: -2,
            minor: true
        }
    ));
}
#[test]
fn fractional_time_is_carried_between_segments() {
    let b = encode(
        Format::SingleTrack,
        Timing::Metrical(3.into()),
        vec![vec![
            event(1, Kind::Meta(Meta::Tempo(500_001.into()))),
            event(1, Kind::Meta(Meta::Tempo(500_002.into()))),
            end(1),
        ]],
    );
    assert_eq!(import(&b).unwrap().duration_us, 500_001);
}
#[test]
fn tempo_lookup_after_the_last_change_and_before_the_first() {
    let m = TempoMap::new(
        480,
        &[Tempo {
            tick: 480,
            microseconds_per_quarter: 1_000_000,
        }],
    )
    .unwrap();
    assert_eq!(m.microseconds_at(240).unwrap(), 250_000);
    assert_eq!(m.microseconds_at(960).unwrap(), 1_500_000);
}
#[test]
fn zero_velocity_releases_and_same_tick_retrigger_preserve_order() {
    let s = import(&single(vec![
        event(0, on(0, 60, 80)),
        event(480, on(0, 60, 0)),
        event(0, on(0, 60, 90)),
        event(480, off(0, 60)),
        end(0),
    ]))
    .unwrap();
    assert_eq!(s.tracks[0].notes.len(), 2);
    assert_eq!(
        s.tracks[0].notes[0].end_tick,
        s.tracks[0].notes[1].start_tick
    );
    assert!(s.diagnostics.is_empty());
}
#[test]
fn channels_ports_and_percussion_are_not_merged() {
    let s = import(&single(vec![
        event(0, on(0, 60, 80)),
        event(0, on(9, 60, 80)),
        event(0, Kind::Meta(Meta::MidiPort(2.into()))),
        event(0, on(0, 60, 80)),
        event(480, off(0, 60)),
        event(0, Kind::Meta(Meta::MidiPort(0.into()))),
        event(0, off(9, 60)),
        event(0, off(0, 60)),
        end(0),
    ]))
    .unwrap();
    assert_eq!(s.tracks[0].notes.len(), 3);
    assert!(s.diagnostics.is_empty());
    assert!(s.tracks[0].notes.iter().any(|n| n.port == 2));
    assert_eq!(
        s.tracks[0]
            .notes
            .iter()
            .filter(|n| n.is_percussion())
            .count(),
        1
    );
}
#[test]
fn repeated_keys_use_fifo_and_report_ambiguity() {
    let s = import(&single(vec![
        event(0, on(0, 60, 80)),
        event(10, on(0, 60, 90)),
        event(10, off(0, 60)),
        event(10, off(0, 60)),
        end(0),
    ]))
    .unwrap();
    assert_eq!(
        s.tracks[0]
            .notes
            .iter()
            .map(|n| (n.start_tick, n.end_tick))
            .collect::<Vec<_>>(),
        [(0, 20), (10, 30)]
    );
    assert_eq!(s.diagnostics[0].kind, DiagnosticKind::OverlappingKey);
}
#[test]
fn missing_and_orphan_releases_are_diagnosed_without_discarding_notes() {
    let s = import(&single(vec![
        event(0, off(0, 61)),
        event(0, on(0, 60, 80)),
        end(480),
    ]))
    .unwrap();
    assert_eq!(s.tracks[0].notes[0].ending, NoteEnding::TrackEnd);
    assert_eq!(s.tracks[0].notes[0].end_tick, 480);
    assert_eq!(
        s.diagnostics
            .iter()
            .map(|d| d.kind.clone())
            .collect::<Vec<_>>(),
        [
            DiagnosticKind::OrphanRelease,
            DiagnosticKind::UnreleasedNote
        ]
    );
}
#[test]
fn sustain_is_preserved_without_extending_key_down_duration() {
    let s = import(&single(vec![
        event(0, on(0, 60, 80)),
        event(0, cc(0, 64, 127)),
        event(240, off(0, 60)),
        event(240, cc(0, 64, 0)),
        end(0),
    ]))
    .unwrap();
    assert_eq!(s.tracks[0].notes[0].end_tick, 240);
    assert!(matches!(
        s.tracks[0].events[1].kind,
        EventKind::Channel {
            message: ChannelMessage::Controller {
                controller: 64,
                value: 127
            },
            ..
        }
    ));
}
#[test]
fn channel_mode_closes_only_the_affected_channel() {
    let s = import(&single(vec![
        event(0, on(0, 60, 80)),
        event(0, on(1, 60, 80)),
        event(100, cc(0, 123, 0)),
        event(100, off(1, 60)),
        end(0),
    ]))
    .unwrap();
    assert_eq!(s.tracks[0].notes[0].ending, NoteEnding::ChannelMode);
    assert_eq!(s.tracks[0].notes[1].end_tick, 200);
    assert!(s.diagnostics.is_empty());
}
#[test]
fn raw_text_sysex_program_pitch_and_pressure_survive_import() {
    let events = vec![
        event(0, Kind::Meta(Meta::Text(&[0xff, b'a']))),
        event(0, Kind::SysEx(&[1, 2, 0xf7])),
        event(
            0,
            Kind::Midi {
                channel: 0.into(),
                message: Msg::ProgramChange { program: 4.into() },
            },
        ),
        event(
            0,
            Kind::Midi {
                channel: 0.into(),
                message: Msg::PitchBend {
                    bend: midly::PitchBend::mid_raw_value(),
                },
            },
        ),
        end(0),
    ];
    let s = import(&single(events)).unwrap();
    assert!(matches!(&s.tracks[0].events[0].kind,EventKind::Text{raw,..} if raw==&[0xff,b'a']));
    assert!(matches!(&s.tracks[0].events[1].kind,EventKind::SysEx{data,..} if data==&[1,2,0xf7]));
    assert!(matches!(
        s.tracks[0].events[2].kind,
        EventKind::Channel {
            message: ChannelMessage::Program { program: 4 },
            ..
        }
    ));
    assert!(matches!(
        s.tracks[0].events[3].kind,
        EventKind::Channel {
            message: ChannelMessage::PitchBend { value: 8192 },
            ..
        }
    ));
}
#[test]
fn independent_sequences_smpte_and_zero_division_are_explicit_errors() {
    let b = encode(
        Format::Sequential,
        Timing::Metrical(480.into()),
        vec![vec![end(0)]],
    );
    assert_eq!(import(&b).unwrap_err().code, ErrorCode::UnsupportedFormat);
    let mut b = basic();
    b[12] = 0xe7;
    b[13] = 40;
    assert_eq!(import(&b).unwrap_err().code, ErrorCode::UnsupportedTiming);
    let mut b = basic();
    b[12] = 0;
    b[13] = 0;
    assert_eq!(import(&b).unwrap_err().code, ErrorCode::UnsupportedTiming);
}
#[test]
fn tempo_conflicts_and_zero_are_not_silently_chosen() {
    let b = encode(
        Format::Parallel,
        Timing::Metrical(480.into()),
        vec![
            vec![event(0, Kind::Meta(Meta::Tempo(400_000.into()))), end(0)],
            vec![event(0, Kind::Meta(Meta::Tempo(600_000.into()))), end(0)],
        ],
    );
    assert_eq!(import(&b).unwrap_err().code, ErrorCode::ConflictingTempo);
    let b = single(vec![event(0, Kind::Meta(Meta::Tempo(0.into()))), end(0)]);
    assert_eq!(import(&b).unwrap_err().code, ErrorCode::InvalidTempo);
}
#[test]
fn duplicate_equal_tempos_are_harmless() {
    let b = single(vec![
        event(0, Kind::Meta(Meta::Tempo(600_000.into()))),
        event(0, Kind::Meta(Meta::Tempo(600_000.into()))),
        end(480),
    ]);
    assert_eq!(import(&b).unwrap().duration_us, 600_000);
}
#[test]
fn resource_limits_fail_with_structured_codes() {
    let b = basic();
    for l in [
        Limits {
            bytes: 1,
            ..Default::default()
        },
        Limits {
            tracks: 0,
            ..Default::default()
        },
        Limits {
            events: 1,
            ..Default::default()
        },
        Limits {
            duration_us: 1,
            ..Default::default()
        },
    ] {
        assert_eq!(
            import_with_limits(&b, l).unwrap_err().code,
            ErrorCode::LimitExceeded
        );
    }
}
#[test]
fn truncated_tracks_and_inconsistent_counts_fail() {
    let b = basic();
    assert!(import(&b[..b.len() - 1]).is_err());
    let mut b = basic();
    b[10] = 0;
    b[11] = 2;
    assert!(import(&b).is_err());
    assert!(import(b"not a MIDI").is_err());
    assert!(import(&single(vec![event(0, on(0, 60, 80))])).is_err());
    assert!(import(&single(vec![end(0), event(0, on(0, 60, 80)), end(0)])).is_err());
}
#[test]
fn empty_track_is_valid_and_no_instrument_is_inferred() {
    let s = import(&single(vec![end(480)])).unwrap();
    assert!(s.tracks[0].notes.is_empty());
    assert_eq!(s.duration_us, 500_000);
}
#[test]
fn all_truncations_and_small_arbitrary_inputs_fail_without_panics() {
    let b = basic();
    for i in 0..b.len() {
        assert!(import(&b[..i]).is_err());
    }
    for n in 0..256u16 {
        let bytes = vec![n as u8; 64];
        let _ = import(&bytes);
    }
}

#[test]
fn pressure_and_smpte_offset_are_preserved_as_data() {
    let offset = midly::SmpteTime::new(1, 2, 3, 4, 5, midly::Fps::Fps25).unwrap();
    let s = import(&single(vec![
        event(0, Kind::Meta(Meta::SmpteOffset(offset))),
        event(
            0,
            Kind::Midi {
                channel: 2.into(),
                message: Msg::Aftertouch {
                    key: 60.into(),
                    vel: 42.into(),
                },
            },
        ),
        event(
            0,
            Kind::Midi {
                channel: 2.into(),
                message: Msg::ChannelAftertouch { vel: 31.into() },
            },
        ),
        end(0),
    ]))
    .unwrap();
    assert!(
        matches!(&s.tracks[0].events[0].kind,EventKind::Meta{tag:0x54,data} if data==&[33,2,3,4,5])
    );
    assert!(matches!(
        s.tracks[0].events[1].kind,
        EventKind::Channel {
            message: ChannelMessage::PolyPressure {
                key: 60,
                pressure: 42
            },
            ..
        }
    ));
    assert!(matches!(
        s.tracks[0].events[2].kind,
        EventKind::Channel {
            message: ChannelMessage::Pressure { pressure: 31 },
            ..
        }
    ));
}
#[test]
fn tempo_api_rejects_invalid_maps_and_timestamp_overflow() {
    assert!(TempoMap::new(0, &[]).is_err());
    assert!(
        TempoMap::new(
            480,
            &[Tempo {
                tick: 0,
                microseconds_per_quarter: 0
            }]
        )
        .is_err()
    );
    assert!(
        TempoMap::new(
            480,
            &[
                Tempo {
                    tick: 0,
                    microseconds_per_quarter: 100
                },
                Tempo {
                    tick: 0,
                    microseconds_per_quarter: 200
                }
            ]
        )
        .is_err()
    );
    let map = TempoMap::new(1, &[]).unwrap();
    assert_eq!(
        map.microseconds_at(u64::MAX).unwrap_err().code,
        ErrorCode::TimeOverflow
    );
}
