use tangent_core::notation::{BeamValue, StemDirection};
use tangent_musicxml::import;
fn xml(content: &str) -> Vec<u8> {
    format!(r#"<score-partwise version="4.0"><part-list><score-part id="P"><part-name>Piano</part-name></score-part></part-list><part id="P"><measure number="1"><attributes><divisions>4</divisions><time><beats>4</beats><beat-type>4</beat-type></time></attributes>{content}</measure></part></score-partwise>"#).into_bytes()
}
fn note(step: &str, duration: u8, value: &str, extra: &str) -> String {
    format!(
        "<note><pitch><step>{step}</step><octave>4</octave></pitch><duration>{duration}</duration><type>{value}</type>{extra}</note>"
    )
}
#[test]
fn written_beams_survive_repeats_without_changing_sound_timing() {
    let content = note("C", 2, "eighth", "<beam>begin</beam><stem>down</stem>")
        + &note(
            "E",
            1,
            "16th",
            "<beam>continue</beam><beam number='2'>forward hook</beam><stem>down</stem>",
        )
        + &note(
            "D",
            1,
            "16th",
            "<beam>end</beam><beam number='2'>backward hook</beam><stem>down</stem>",
        );
    let song = import(
        &xml(&(content + "<barline><repeat direction='backward'/></barline>")),
        "Beams".into(),
    )
    .unwrap();
    assert_eq!(song.notes.len(), 6);
    assert_eq!(song.notes[1].start_us, 250_000);
    assert_eq!(song.notes[2].start_us, 375_000);
    let score = song.notation.unwrap();
    assert_eq!(score.playback.len(), 2);
    let m = &score.measures[0];
    assert_eq!(m.notes[0].beams[0].value, BeamValue::Begin);
    assert_eq!(m.notes[0].stem, Some(StemDirection::Down));
    assert_eq!(m.beam_groups.len(), 1);
    assert_eq!(m.beam_groups[0].chords.len(), 3);
    assert_eq!(m.beam_groups[0].spans.iter().filter(|s| s.hook).count(), 2);
}
#[test]
fn absent_marks_auto_group_but_malformed_marks_never_connect() {
    let s = import(
        &xml(&(note("C", 2, "eighth", "") + &note("D", 2, "eighth", ""))),
        "auto".into(),
    )
    .unwrap();
    assert_eq!(s.notation.unwrap().measures[0].beam_groups.len(), 1);
    let s = import(
        &xml(&(note("C", 2, "eighth", "<beam>begin</beam>") + &note("D", 2, "eighth", ""))),
        "orphan".into(),
    )
    .unwrap();
    assert!(s.notation.unwrap().measures[0].beam_groups.is_empty());
    for bad in [
        "<beam number='0'>begin</beam>",
        "<beam number='9'>end</beam>",
        "<beam>typo</beam>",
        "<beam>begin</beam><beam>end</beam>",
    ] {
        assert!(
            import(&xml(&note("C", 2, "eighth", bad)), "bad".into()).is_err(),
            "{bad}"
        );
    }
}
