use tangent_musicxml::import;
const PIANO: &[u8] = include_bytes!("fixtures/piano.musicxml");
fn wrap(content: &str) -> Vec<u8> {
    format!(r#"<score-partwise><part-list><score-part id="P"><part-name>Piano</part-name></score-part></part-list><part id="P">{content}</part></score-partwise>"#).into_bytes()
}
#[test]
fn piano_voices_chords_written_flats_ties_and_tempo_changes() {
    let s = import(PIANO, "fallback".into()).unwrap();
    assert_eq!(s.title, "Tangent XML study");
    assert_eq!(s.duration_us, 6_000_000);
    assert_eq!(s.notes.len(), 5);
    assert_eq!(s.notes[0].key, 61);
    assert_eq!(s.notes[0].written.as_ref().unwrap().alteration, -1);
    assert_eq!(s.notes[1].start_us, 0);
    let tied = s.notes.iter().find(|n| n.key == 60).unwrap();
    assert_eq!((tied.start_us, tied.end_us), (1_000_000, 4_500_000));
    let score = s.notation.unwrap();
    assert_eq!(score.staves.len(), 2);
    assert_eq!(score.measures.len(), 2);
    assert_eq!(score.measures[1].notes[0].performance_note, Some(tied.id));
    assert!(score.measures[1].notes[1].pitch.is_none());
    assert_eq!(score.measures[1].notes[1].duration.denominator.get(), 3);
    assert!(s.warnings.is_empty());
}
#[test]
fn divisions_changes_use_one_exact_grid() {
    let bytes = wrap(
        r#"<measure implicit="yes"><attributes><divisions>3</divisions></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration></note><attributes><divisions>2</divisions></attributes><note><pitch><step>D</step><octave>4</octave></pitch><duration>1</duration></note></measure>"#,
    );
    let s = import(&bytes, "t".into()).unwrap();
    assert_eq!(s.duration_us, 416_667);
    assert_eq!(s.notes[1].start_us, 166_667);
}
#[test]
fn invalid_inputs_are_rejected_without_panics_or_external_resolution() {
    for bytes in [b"bad".as_slice(),b"PK123",b"<score-timewise/>",b"<!DOCTYPE x [<!ENTITY e SYSTEM 'file:///etc/passwd'>]><score-partwise>&e;</score-partwise>"] {
        assert!(import(bytes,"bad".into()).is_err());
    }
    for fragment in [
        "<measure><attributes><divisions>0</divisions></attributes></measure>",
        "<measure><backup><duration>1</duration></backup></measure>",
        "<measure><attributes><divisions>1</divisions><transpose><chromatic>2</chromatic></transpose></attributes></measure>",
        "<measure><attributes><divisions>1</divisions></attributes><note><chord/><rest/><duration>1</duration></note></measure>",
        "<measure><attributes><divisions>1</divisions></attributes><note><pitch><step>C</step><alter>0.5</alter><octave>4</octave></pitch><duration>1</duration></note></measure>",
    ] {
        assert!(import(&wrap(fragment), "bad".into()).is_err(), "{fragment}");
    }
}
#[test]
fn unsupported_repeat_is_visible_as_a_warning() {
    let s = import(
        &wrap("<measure><barline><repeat direction='backward'/></barline></measure>"),
        "t".into(),
    )
    .unwrap();
    assert!(s.warnings[0].contains("written order"));
}
#[test]
fn parts_share_measure_offsets_and_conflicting_tempos_are_errors() {
    let base = String::from_utf8(wrap("<measure implicit='yes'><attributes><divisions>1</divisions></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration></note></measure><measure><note><pitch><step>D</step><octave>4</octave></pitch><duration>1</duration></note></measure>")).unwrap();
    let xml = base.replace("</part-list>", "<score-part id='Q'><part-name>Bass</part-name></score-part></part-list>").replace("</score-partwise>", "<part id='Q'><measure implicit='yes'><attributes><divisions>1</divisions></attributes><forward><duration>2</duration></forward></measure><measure><note><pitch><step>C</step><octave>3</octave></pitch><duration>1</duration></note></measure></part></score-partwise>");
    let s = import(xml.as_bytes(), "t".into()).unwrap();
    assert_eq!(
        s.notes.iter().find(|n| n.key == 62).unwrap().start_us,
        1_000_000
    );
    assert_eq!(
        s.notes.iter().find(|n| n.key == 48).unwrap().start_us,
        1_000_000
    );
    let conflict = wrap("<measure><sound tempo='60'/><sound tempo='90'/></measure>");
    assert!(
        import(&conflict, "t".into())
            .unwrap_err()
            .contains("Conflicting tempos")
    );
}
#[test]
fn truncations_fail_without_panics() {
    let lf = std::str::from_utf8(PIANO).unwrap().replace("\r\n", "\n");
    for bytes in [
        lf.as_bytes().to_vec(),
        lf.replace('\n', "\r\n").into_bytes(),
    ] {
        // Cutting trailing whitespace leaves valid XML. Check every truncation
        // of the actual document for both Unix and Windows checkout line endings.
        let score = bytes.trim_ascii_end();
        for i in 0..score.len() {
            assert!(import(&score[..i], "t".into()).is_err());
        }
    }
}

#[test]
fn overlapping_unisons_in_separate_voices_have_independent_audio_routes() {
    use tangent_core::music::PerformanceEvent;
    let xml = wrap(
        "<measure implicit='yes'><attributes><divisions>1</divisions></attributes><note><pitch><step>C</step><octave>4</octave></pitch><duration>2</duration><voice>1</voice></note><backup><duration>2</duration></backup><note><pitch><step>C</step><octave>4</octave></pitch><duration>1</duration><voice>2</voice></note></measure>",
    );
    let song = import(&xml, "t".into()).unwrap();
    let attacks: Vec<_> = song
        .events
        .iter()
        .filter(|e| matches!(e.message, PerformanceEvent::On { .. }))
        .collect();
    assert_ne!(
        (attacks[0].port, attacks[0].channel),
        (attacks[1].port, attacks[1].channel)
    );
}
#[test]
fn node_and_resolution_bounds_are_enforced_early() {
    let excessive =
        wrap("<measure><attributes><divisions>1000000001</divisions></attributes></measure>");
    assert!(
        import(&excessive, "t".into())
            .unwrap_err()
            .contains("resolution")
    );
    let excessive = vec![b' '; 16 * 1024 * 1024 + 1];
    assert!(
        import(&excessive, "t".into())
            .unwrap_err()
            .contains("16 MiB")
    );
}
