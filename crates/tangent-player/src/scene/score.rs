//! Authored measure layout. MIDI retains its separate time-spaced reading aid.
use super::{draw::Drawing, layout::Layout, theme, *};
use tangent_core::notation::{ledger_steps, reading_pitch};
use tangent_core::{music::WrittenPitch, notation::*};
pub(super) fn render(d: &mut Drawing, state: &State, l: &Layout, t: theme::Theme) {
    if let Some(score) = &state.song.notation {
        authored(d, state, score, l, t);
    } else {
        legacy(d, state, l, t);
    }
}
fn pitch(p: &WrittenPitch, bass: bool) -> ReadingPitch {
    use tangent_core::music::PitchLetter::*;
    ReadingPitch {
        step: p.octave as i16 * 7
            + match p.letter {
                C => 0,
                D => 1,
                E => 2,
                F => 3,
                G => 4,
                A => 5,
                B => 6,
            },
        bass,
        sharp: p.alteration > 0,
    }
}
fn authored(d: &mut Drawing, state: &State, score: &NotatedScore, l: &Layout, t: theme::Theme) {
    if score.measures.is_empty() {
        return;
    }
    let now = state.transport.position_us;
    let visit_index = score
        .playback
        .partition_point(|v| v.end_us <= now)
        .min(score.playback.len().saturating_sub(1));
    let Some(visit) = score.playback.get(visit_index) else {
        return;
    };
    let count = if l.width < 700. { 2 } else { 4 };
    let page = visit.measure / count * count;
    let end = (page + count).min(score.measures.len());
    let gap = ((l.strike - l.top) / 24.).clamp(6., 14.);
    let treble = l.top + gap * 8.;
    let bass = treble + gap * 10.;
    let header = (gap * (7. + score.measures[page].key_fifths.unsigned_abs() as f32 * 0.7))
        .min(l.width * 0.35);
    let left = l.margin + header;
    let right = l.width - l.margin;
    let width = (right - left) / (end - page) as f32;
    for bottom in [treble, bass] {
        for i in 0..5 {
            d.line(
                l.margin,
                bottom - i as f32 * gap,
                right - l.margin,
                1.,
                t.staff,
            );
        }
    }
    d.glyph(l.margin + 3., treble - gap, '\u{e050}', gap * 4., t.ink);
    d.glyph(l.margin + 3., bass - gap * 3., '\u{e062}', gap * 4., t.ink);
    let first = &score.measures[page];
    if first.tempos.iter().all(|m| m.onset.numerator != 0) {
        d.glyph(
            left + gap,
            treble - gap * 8.6,
            '\u{e1d5}',
            gap * 2.,
            t.muted,
        );
        d.text(
            left + gap + 10.,
            treble - gap * 9.4,
            format!("= {}", first.initial_quarter_bpm.round()),
            11.,
            t.muted,
        );
    }
    for bottom in [treble, bass] {
        key_signature(
            d,
            l.margin + gap * 3.4,
            bottom,
            gap,
            first.key_fifths,
            bottom == bass,
            t.ink,
        );
        d.text(
            left - gap * 2.5,
            bottom - gap * 3.7,
            first.meter.beats.to_string(),
            gap * 1.45,
            t.ink,
        );
        d.text(
            left - gap * 2.5,
            bottom - gap * 1.9,
            first.meter.beat_type.to_string(),
            gap * 1.45,
            t.ink,
        );
    }
    for index in page..end {
        let m = &score.measures[index];
        let x = left + (index - page) as f32 * width;
        let changed_meter = index > page && m.meter != score.measures[index - 1].meter;
        let changed_key = index > page && m.key_fifths != score.measures[index - 1].key_fifths;
        let inset = (if m.repeat_start { gap * 2. } else { gap * 1.4 })
            + if changed_meter { gap * 2. } else { 0. }
            + if changed_key {
                gap * (1. + m.key_fifths.unsigned_abs() as f32 * 0.7)
            } else {
                0.
            };
        for bottom in [treble, bass] {
            if changed_meter {
                d.text(
                    x + gap * 0.4,
                    bottom - gap * 3.7,
                    m.meter.beats.to_string(),
                    gap * 1.4,
                    t.ink,
                );
                d.text(
                    x + gap * 0.4,
                    bottom - gap * 1.9,
                    m.meter.beat_type.to_string(),
                    gap * 1.4,
                    t.ink,
                );
            }
            if changed_key {
                key_signature(
                    d,
                    x + gap * 2.,
                    bottom,
                    gap,
                    m.key_fifths,
                    bottom == bass,
                    t.ink,
                );
            }
        }
        let space = (width - inset - gap * 1.5).max(12.);
        let q = m.duration.quarters().max(0.001);
        let at = |beat: f64| x + inset + space * (beat / q).clamp(0., 1.) as f32;
        d.text(x + 3., treble - gap * 5.3, &m.label, 10., t.muted);
        bar(d, x, treble, bass, gap, 1., t.staff);
        if m.repeat_start {
            bar(d, x + 3., treble, bass, gap, 3., t.ink);
            repeat_dots(d, x + gap * 0.8, treble, bass, gap, t.ink);
        }
        if let Some(times) = m.repeat_end {
            let rx = x + width - gap * 0.5;
            bar(d, rx, treble, bass, gap, 3., t.ink);
            bar(d, rx - gap * 0.45, treble, bass, gap, 1., t.ink);
            repeat_dots(d, rx - gap, treble, bass, gap, t.ink);
            if times > 2 {
                d.text(
                    rx - gap * 3.,
                    bass + gap * 2.,
                    format!("{times}x"),
                    10.,
                    t.muted,
                );
            }
        }
        if let Some(ending) = &m.ending {
            let y = treble - gap * 7.;
            d.line(x, y, width, 1.4, t.ink);
            if ending.start || index == page {
                d.line(x, y, 1.4, gap, t.ink);
            }
            if ending.end && !ending.open_end {
                d.line(x + width - 1., y, 1.4, gap, t.ink);
            }
            if ending.start || index == page {
                d.text(
                    x + 5.,
                    y + 3.,
                    ending
                        .numbers
                        .iter()
                        .map(|n| format!("{n}."))
                        .collect::<Vec<_>>()
                        .join(", "),
                    11.,
                    t.ink,
                );
            }
        }
        for mark in &m.tempos {
            let mx = at(mark.onset.quarters());
            d.glyph(mx, treble - gap * 8.6, '\u{e1d5}', gap * 2., t.muted);
            d.text(
                mx + 10.,
                treble - gap * 9.4,
                format!("= {}", mark.quarter_bpm.round()),
                11.,
                t.muted,
            );
        }
        // Accidental state is per staff/octave/letter and resets each measure.
        let mut accidentals = BTreeMap::new();
        for n in &m.notes {
            let is_bass = score
                .staves
                .iter()
                .find(|s| s.id == n.staff)
                .is_some_and(|s| matches!(s.clef, Clef::Bass));
            let bottom = if is_bass { bass } else { treble };
            let nx = at(n.onset.quarters());
            let color = score
                .staves
                .iter()
                .find(|s| s.id == n.staff)
                .map_or(t.colors[0], |s| t.colors[s.part % 4]);
            let Some(p) = &n.pitch else {
                d.glyph(
                    nx - 3.,
                    bottom - gap * 2.,
                    rest(n.value),
                    gap * 4.,
                    color.with_alpha(0.6),
                );
                continue;
            };
            let p = pitch(p, is_bass);
            let ny = bottom - (p.step - if is_bass { 18 } else { 30 }) as f32 * gap / 2.;
            for step in ledger_steps(p.step, if is_bass { 18 } else { 30 }) {
                d.line(
                    nx - gap * 0.8,
                    bottom - (step - if is_bass { 18 } else { 30 }) as f32 * gap / 2.,
                    gap * 1.6,
                    1.,
                    color,
                );
            }
            let written = n.pitch.as_ref().unwrap();
            let expected = accidentals
                .entry((n.staff, p.step))
                .or_insert_with(|| key_alteration(p.step.rem_euclid(7), m.key_fifths));
            if *expected != written.alteration {
                d.glyph(
                    nx - gap * 1.4,
                    ny,
                    match written.alteration {
                        -2 => '\u{e264}',
                        -1 => '\u{e260}',
                        0 => '\u{e261}',
                        1 => '\u{e262}',
                        _ => '\u{e263}',
                    },
                    gap * 3.5,
                    color,
                );
                *expected = written.alteration;
            }
            let duration = n.duration.quarters();
            let hollow = matches!(n.value, Some(NoteValue::Whole | NoteValue::Half))
                || n.value.is_none() && duration >= 2.;
            let flag = matches!(
                n.value,
                Some(
                    NoteValue::Eighth
                        | NoteValue::Sixteenth
                        | NoteValue::ThirtySecond
                        | NoteValue::SixtyFourth
                )
            ) || n.value.is_none() && duration < 1.;
            if matches!(n.value, Some(NoteValue::Whole)) || n.value.is_none() && duration >= 4. {
                d.glyph(nx - gap * 0.8, ny, '\u{e0a2}', gap * 4., color);
            } else {
                d.note(nx, ny, gap * 0.44, color, hollow, flag);
            }
            for dot in 0..n.dots {
                d.rect(
                    nx + gap * (0.9 + dot as f32 * 0.5),
                    ny - 1.5,
                    3.,
                    3.,
                    color,
                    2.,
                );
            }
            if n.tie_to_next {
                d.line(
                    nx + gap * 0.5,
                    ny + gap * 0.9,
                    (space * (duration / q) as f32 - gap).max(3.),
                    1.5,
                    color.with_alpha(0.5),
                );
            }
        }
        if index == visit.measure {
            let fraction = (now.saturating_sub(visit.start_us) as f64
                / (visit.end_us - visit.start_us).max(1) as f64)
                .clamp(0., 1.);
            // Piecewise timing follows tempo changes inside the measure.
            let beat = visit.quarter_at_time(now.saturating_sub(visit.start_us));
            let px = if fraction >= 1. { x + width } else { at(beat) };
            d.line(
                px,
                treble - gap * 4.8,
                2.,
                bass - treble + gap * 5.6,
                t.colors[0],
            );
        }
    }
    bar(d, right, treble, bass, gap, 1., t.staff);
}
fn bar(d: &mut Drawing, x: f32, treble: f32, bass: f32, gap: f32, width: f32, color: Color) {
    d.line(x, treble - gap * 4., width, bass - treble + gap * 4., color);
}
fn repeat_dots(d: &mut Drawing, x: f32, treble: f32, bass: f32, gap: f32, color: Color) {
    for bottom in [treble, bass] {
        for delta in [1.5, 2.5] {
            d.rect(x, bottom - gap * delta - 1.5, 3., 3., color, 2.);
        }
    }
}
fn rest(value: Option<NoteValue>) -> char {
    match value {
        Some(NoteValue::Whole) => '\u{e4e3}',
        Some(NoteValue::Half) => '\u{e4e4}',
        Some(NoteValue::Eighth) => '\u{e4e6}',
        Some(NoteValue::Sixteenth) => '\u{e4e7}',
        _ => '\u{e4e5}',
    }
}
fn key_alteration(letter: i16, fifths: i8) -> i8 {
    let order = if fifths >= 0 {
        [3, 0, 4, 1, 5, 2, 6]
    } else {
        [6, 2, 5, 1, 4, 0, 3]
    };
    if order[..fifths.unsigned_abs() as usize].contains(&letter) {
        fifths.signum()
    } else {
        0
    }
}
fn key_signature(
    d: &mut Drawing,
    x: f32,
    bottom: f32,
    gap: f32,
    fifths: i8,
    bass: bool,
    color: Color,
) {
    let steps = match (fifths >= 0, bass) {
        (true, false) => [8, 5, 9, 6, 3, 7, 4],
        (true, true) => [6, 3, 7, 4, 1, 5, 2],
        (false, false) => [4, 7, 3, 6, 2, 5, 1],
        (false, true) => [2, 5, 1, 4, 0, 3, -1],
    };
    for (i, step) in steps
        .into_iter()
        .take(fifths.unsigned_abs() as usize)
        .enumerate()
    {
        d.glyph(
            x + i as f32 * gap * 0.7,
            bottom - step as f32 * gap * 0.5,
            if fifths >= 0 { '\u{e262}' } else { '\u{e260}' },
            gap * 3.6,
            color,
        );
    }
}

fn score_pitch(
    n: &tangent_core::music::PerformanceNote,
    bass: Option<bool>,
) -> tangent_core::notation::ReadingPitch {
    use tangent_core::music::PitchLetter::*;
    let mut p = reading_pitch(n.key);
    if let Some(w) = &n.written {
        p.step = i16::from(w.octave) * 7
            + match w.letter {
                C => 0,
                D => 1,
                E => 2,
                F => 3,
                G => 4,
                A => 5,
                B => 6,
            };
        p.bass = bass.unwrap_or(p.bass);
    }
    p
}
fn legacy(d: &mut Drawing, state: &State, l: &Layout, t: theme::Theme) {
    let gap = ((l.strike - l.top) / 22.).clamp(5., 12.);
    let treble = l.top + gap * 7.;
    let bass = treble + gap * 10.;
    let left = l.margin + 38.;
    let right = l.width - l.margin;
    let span = 8_000_000;
    let start = state.transport.position_us / span * span;
    for bottom in [treble, bass] {
        for i in 0..5 {
            d.line(
                l.margin,
                bottom - i as f32 * gap,
                l.width - 2. * l.margin,
                1.,
                t.staff,
            );
        }
    }
    d.glyph(l.margin, treble - gap, '\u{e050}', gap * 4., t.ink);
    d.glyph(l.margin, bass - gap * 3., '\u{e062}', gap * 4., t.ink);
    let py = |p: tangent_core::notation::ReadingPitch| {
        if p.bass {
            bass - (p.step - 18) as f32 * gap / 2.
        } else {
            treble - (p.step - 30) as f32 * gap / 2.
        }
    };
    let mut clefs = BTreeMap::new();
    if let Some(score) = &state.song.notation {
        let staff_clefs: BTreeMap<_, _> = score
            .staves
            .iter()
            .map(|s| (s.id, matches!(s.clef, tangent_core::notation::Clef::Bass)))
            .collect();
        for note in score.measures.iter().flat_map(|m| &m.notes) {
            if let Some(id) = note.performance_note
                && let Some(bass) = staff_clefs.get(&note.staff)
            {
                clefs.entry(id).or_insert(*bass);
            }
        }
    }
    for n in &state.song.notes {
        if n.start_us < start || n.start_us >= start + span || state.song.parts[n.part].percussion {
            continue;
        }
        let p = score_pitch(n, clefs.get(&n.id).copied());
        let x = left + (right - left) * (n.start_us - start) as f32 / span as f32;
        let y = py(p);
        let color = t.colors[n.part % 4];
        if y < l.top + 10. || y > l.strike - 25. {
            continue;
        }
        for step in ledger_steps(p.step, if p.bass { 18 } else { 30 }) {
            d.line(
                x - 9.,
                py(tangent_core::notation::ReadingPitch { step, ..p }),
                18.,
                1.,
                color,
            );
        }
        let accidental = n
            .written
            .as_ref()
            .map(|p| match p.alteration {
                -2 => "bb",
                -1 => "b",
                1 => "#",
                2 => "x",
                _ => "",
            })
            .unwrap_or(if p.sharp { "#" } else { "" });
        if !accidental.is_empty() {
            d.text(x - 20., y - 11., accidental, 14., color);
        }
        d.note(
            x,
            y,
            5.,
            color,
            n.end_us - n.start_us >= 800_000,
            n.end_us - n.start_us < 300_000,
        );
        d.line(
            x,
            y + 9.,
            ((right - left) * (n.end_us - n.start_us) as f32 / span as f32).min(right - x),
            2.,
            color.with_alpha(0.3),
        );
    }
    let x = left + (right - left) * (state.transport.position_us - start) as f32 / span as f32;
    d.line(
        x,
        l.top + 8.,
        2.,
        (bass + gap * 2. - l.top).min(l.strike - l.top),
        t.colors[0],
    );
}

/// Measure and beat guides follow expanded performance visits in falling modes.
pub(super) fn falling_structure(d: &mut Drawing, state: &State, l: &Layout, t: theme::Theme) {
    let Some(score) = &state.song.notation else {
        return;
    };
    let now = state.transport.position_us;
    let pps = (l.strike - l.top) / 5.5;
    let y = |us: u64| l.strike - (us as f64 - now as f64) as f32 / 1e6 * pps;
    for visit in &score.playback {
        if visit.end_us < now || visit.start_us > now + 6_000_000 {
            continue;
        }
        let m = &score.measures[visit.measure];
        let top = y(visit.end_us).max(l.top + 12.);
        let bottom = y(visit.start_us).min(l.strike);
        if bottom <= l.top || top >= l.strike {
            continue;
        }
        let bar = y(visit.start_us);
        if bar >= l.top + 12. && bar <= l.strike {
            d.line(
                l.margin,
                bar,
                l.width - 2. * l.margin,
                1.2,
                t.ink.with_alpha(0.22),
            );
            d.text(
                l.margin + 4.,
                bar - 17.,
                format!("{}  {}/{}", m.label, m.meter.beats, m.meter.beat_type),
                10.,
                t.muted,
            );
        }
        let beat = 4. / m.meter.beat_type as f64;
        for i in 1..m.meter.beats {
            let at = visit.time_at_quarter(i as f64 * beat);
            let by = y(visit.start_us + at);
            if by > l.top + 12. && by < l.strike {
                d.line(
                    l.margin,
                    by,
                    l.width - 2. * l.margin,
                    1.,
                    t.ink.with_alpha(0.055),
                );
            }
        }
        if let Some(ending) = &m.ending {
            let x = l.width - l.margin - 8.;
            d.line(x, top, 1.5, bottom - top, t.muted.with_alpha(0.65));
            if ending.start {
                d.line(x - 14., bottom, 14., 1.5, t.muted);
            }
            if ending.end && !ending.open_end {
                d.line(x - 14., top, 14., 1.5, t.muted);
            }
            d.text(
                x - 27.,
                top + 5.,
                ending
                    .numbers
                    .iter()
                    .map(|n| format!("{n}."))
                    .collect::<Vec<_>>()
                    .join(","),
                12.,
                t.muted,
            );
        }
    }
}
