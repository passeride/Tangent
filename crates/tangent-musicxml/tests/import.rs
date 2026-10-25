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
fn backward_repeat_without_forward_repeats_from_the_beginning() {
    let s = import(
        &wrap("<measure><barline><repeat direction='backward'/></barline></measure>"),
        "t".into(),
    )
    .unwrap();
    assert!(s.warnings.is_empty());
    assert_eq!(s.notation.unwrap().playback.len(), 2);
    assert_eq!(s.duration_us, 4_000_000);
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
fn input_size_and_resolution_bounds_are_enforced_early() {
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

#[test]
fn complete_piano_demo_has_measures_endings_meter_tempo_and_correct_repeat_order() {
    let song = import(
        include_bytes!("../../../web/piano/samples/first-light.musicxml"),
        "demo".into(),
    )
    .unwrap();
    assert_eq!(song.notes.len(), 130);
    assert_eq!(song.duration_us, 52_222_222);
    assert!(song.warnings.is_empty());
    let score = song.notation.unwrap();
    assert_eq!(score.measures.len(), 16);
    assert_eq!(score.measures[11].meter.beats, 3);
    assert_eq!(score.measures[8].tempos[0].quarter_bpm, 108.);
    assert_eq!(
        score
            .playback
            .iter()
            .map(|v| v.measure + 1)
            .collect::<Vec<_>>(),
        vec![
            1, 2, 3, 4, 5, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 7, 8, 9, 12, 13, 14, 15, 16
        ]
    );
    assert_eq!(score.measures[4].ending.as_ref().unwrap().numbers, vec![1]);
    assert_eq!(score.measures[5].ending.as_ref().unwrap().numbers, vec![2]);
    // Jumping back restores the earlier tempo, rather than carrying the faster section.
    let repeat = &score.playback[15];
    assert_eq!(repeat.end_us - repeat.start_us, 2_500_000);
}
#[test]
fn malformed_and_excessive_repeat_structures_are_explicit_errors() {
    for measures in [
        "<measure><barline><repeat direction='forward'/></barline></measure>",
        "<measure><barline><repeat direction='forward'/></barline></measure><measure><barline><repeat direction='forward'/></barline></measure>",
        "<measure><barline><repeat direction='backward' times='100'/></barline></measure>",
        "<measure><barline><ending number='1' type='stop'/></barline></measure>",
    ] {
        assert!(import(&wrap(measures), "bad".into()).is_err());
    }
}

#[test]
fn tempo_change_inside_measure_maps_beats_and_time_both_ways() {
    let xml = wrap(
        "<measure><attributes><divisions>1</divisions></attributes><direction><sound tempo='60'/></direction><forward><duration>2</duration></forward><direction><sound tempo='120'/></direction><forward><duration>2</duration></forward></measure>",
    );
    let song = import(&xml, "tempo".into()).unwrap();
    assert_eq!(song.duration_us, 3_000_000);
    let score = song.notation.unwrap();
    let v = &score.playback[0];
    assert_eq!(v.time_at_quarter(1.), 1_000_000);
    assert_eq!(v.time_at_quarter(3.), 2_500_000);
    assert_eq!(v.quarter_at_time(2_500_000), 3.);
    assert_eq!(v.quarter_at_time(99_000_000), 4.);
}

#[test]
fn visual_metronome_beat_units_and_dots_set_quarter_tempo_without_sound() {
    for (unit, dots, bpm, duration) in [
        ("half", "", 120., 2_000_000),
        ("quarter", "<beat-unit-dot/>", 90., 2_666_667),
    ] {
        let xml = wrap(&format!(
            "<measure><direction><direction-type><metronome><beat-unit>{unit}</beat-unit>{dots}<per-minute>60</per-minute></metronome></direction-type></direction></measure>"
        ));
        let song = import(&xml, "metronome".into()).unwrap();
        assert_eq!(song.duration_us, duration);
        assert_eq!(song.notation.unwrap().measures[0].initial_quarter_bpm, bpm);
        assert!(song.warnings.is_empty());
    }
}

// Construct synthetic ZIP archives here instead of importing a fixture from
// another project or depending on browser-only decompression behavior.
fn put16(out: &mut Vec<u8>, n: u16) {
    out.extend_from_slice(&n.to_le_bytes());
}
fn put32(out: &mut Vec<u8>, n: u32) {
    out.extend_from_slice(&n.to_le_bytes());
}
fn mxl_zip(files: &[(&str, &[u8], bool)]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut directory = Vec::new();
    for &(name, data, deflate) in files {
        let method = if deflate { 8 } else { 0 };
        let packed = if deflate {
            miniz_oxide::deflate::compress_to_vec(data, 6)
        } else {
            data.to_vec()
        };
        let offset = out.len() as u32;
        let crc = crc32fast::hash(data);
        put32(&mut out, 0x04034b50);
        put16(&mut out, 20);
        put16(&mut out, 0);
        put16(&mut out, method);
        put16(&mut out, 0);
        put16(&mut out, 0);
        put32(&mut out, crc);
        put32(&mut out, packed.len() as u32);
        put32(&mut out, data.len() as u32);
        put16(&mut out, name.len() as u16);
        put16(&mut out, 0);
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(&packed);

        put32(&mut directory, 0x02014b50);
        put16(&mut directory, 20);
        put16(&mut directory, 20);
        put16(&mut directory, 0);
        put16(&mut directory, method);
        put16(&mut directory, 0);
        put16(&mut directory, 0);
        put32(&mut directory, crc);
        put32(&mut directory, packed.len() as u32);
        put32(&mut directory, data.len() as u32);
        put16(&mut directory, name.len() as u16);
        put16(&mut directory, 0);
        put16(&mut directory, 0);
        put16(&mut directory, 0);
        put16(&mut directory, 0);
        put32(&mut directory, 0);
        put32(&mut directory, offset);
        directory.extend_from_slice(name.as_bytes());
    }
    let dir_offset = out.len() as u32;
    out.extend_from_slice(&directory);
    put32(&mut out, 0x06054b50);
    put16(&mut out, 0);
    put16(&mut out, 0);
    put16(&mut out, files.len() as u16);
    put16(&mut out, files.len() as u16);
    put32(&mut out, directory.len() as u32);
    put32(&mut out, dir_offset);
    put16(&mut out, 0);
    out
}

const ROOT: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<container><rootfiles><rootfile full-path="scores/piano.musicxml"
media-type="application/vnd.recordare.musicxml+xml"/></rootfiles></container>"#;

#[test]
fn compressed_and_stored_mxl_import_through_the_normal_musicxml_adapter() {
    for deflate in [false, true] {
        let archive = mxl_zip(&[
            ("mimetype", b"application/vnd.recordare.musicxml", false),
            ("META-INF/container.xml", ROOT.as_bytes(), deflate),
            ("scores/piano.musicxml", PIANO, deflate),
            (
                "images/ignored.png",
                b"non-XML resource is never extracted",
                true,
            ),
        ]);
        let compressed = import(&archive, "fallback".into()).unwrap();
        let plain = import(PIANO, "fallback".into()).unwrap();
        assert_eq!(compressed.title, plain.title);
        assert_eq!(compressed.duration_us, plain.duration_us);
        assert_eq!(compressed.notes.len(), plain.notes.len());
        assert_eq!(
            compressed.notation.unwrap().measures.len(),
            plain.notation.unwrap().measures.len()
        );
    }
}

#[test]
fn old_mxl_without_mimetype_or_media_type_is_supported() {
    let manifest = "<container><rootfiles><rootfile full-path='p.xml'/></rootfiles></container>";
    let zip = mxl_zip(&[
        ("META-INF/container.xml", manifest.as_bytes(), false),
        ("p.xml", PIANO, true),
    ]);
    assert_eq!(import(&zip, "fallback".into()).unwrap().notes.len(), 5);
}

#[test]
fn malformed_and_unsafe_mxl_files_are_rejected() {
    let missing_manifest = mxl_zip(&[("p.xml", PIANO, false)]);
    assert!(import(&missing_manifest, "x".into()).is_err());

    for root in [
        "/absolute.xml",
        "../outside.xml",
        "sub/../outside.xml",
        "C:/windows.xml",
        "sub\\outside.xml",
    ] {
        let manifest =
            format!("<container><rootfiles><rootfile full-path='{root}'/></rootfiles></container>");
        let zip = mxl_zip(&[
            ("META-INF/container.xml", manifest.as_bytes(), false),
            ("p.xml", PIANO, false),
        ]);
        assert!(import(&zip, "x".into()).is_err(), "{root}");
    }

    let alternate = ROOT.replace("application/vnd.recordare.musicxml+xml", "application/pdf");
    let wrong_type = mxl_zip(&[
        ("META-INF/container.xml", alternate.as_bytes(), false),
        ("scores/piano.musicxml", PIANO, false),
    ]);
    assert!(import(&wrong_type, "x".into()).is_err());

    let missing_root = mxl_zip(&[("META-INF/container.xml", ROOT.as_bytes(), false)]);
    assert!(import(&missing_root, "x".into()).is_err());

    let duplicate = mxl_zip(&[
        ("META-INF/container.xml", ROOT.as_bytes(), false),
        ("META-INF/container.xml", ROOT.as_bytes(), false),
        ("scores/piano.musicxml", PIANO, false),
    ]);
    assert!(import(&duplicate, "x".into()).is_err());

    let mut broken_crc = mxl_zip(&[
        ("META-INF/container.xml", ROOT.as_bytes(), false),
        ("scores/piano.musicxml", PIANO, false),
    ]);
    let position = broken_crc
        .windows(PIANO.len())
        .position(|w| w == PIANO)
        .unwrap();
    broken_crc[position] ^= 1;
    assert!(import(&broken_crc, "x".into()).is_err());

    let valid = mxl_zip(&[
        ("META-INF/container.xml", ROOT.as_bytes(), false),
        ("scores/piano.musicxml", PIANO, true),
    ]);
    for end in 0..valid.len() {
        if end % 37 == 0 {
            assert!(import(&valid[..end], "x".into()).is_err());
        }
    }
}

#[test]
fn mxl_uncompressed_size_limit_is_enforced_before_decompression() {
    let mut zip = mxl_zip(&[
        ("META-INF/container.xml", ROOT.as_bytes(), false),
        ("scores/piano.musicxml", PIANO, true),
    ]);
    let score_directory = zip
        .windows(4)
        .enumerate()
        .filter(|(_, w)| *w == b"PK\x01\x02")
        .map(|(i, _)| i)
        .next_back()
        .unwrap();
    zip[score_directory + 24..score_directory + 28]
        .copy_from_slice(&(16_u32 * 1024 * 1024 + 1).to_le_bytes());
    assert!(import(&zip, "x".into()).unwrap_err().contains("size limit"));
}
