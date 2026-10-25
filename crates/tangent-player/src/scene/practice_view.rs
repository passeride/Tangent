//! Practice notation and live pitch pointers; no musical evaluation rules.
use super::{controls::Action, draw::Drawing, *};
pub(super) fn render(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let x = l.margin + 8.;
    let chords = state.mode == Mode::Chords;
    d.text(
        x,
        18.,
        if chords {
            "Train / chords"
        } else {
            "Train / note reading"
        },
        20.,
        t.ink,
    );
    let message = if state.practice.waiting_release {
        "Correct! Release all keys to continue".to_string()
    } else if state.practice.wrong {
        "Try again - check the clef and octave".to_string()
    } else if chords || state.practice.hint {
        format!("Play {}", state.practice.label())
    } else {
        "Read the note and play it".into()
    };
    d.text(
        x,
        52.,
        message,
        if l.width < 600. { 14. } else { 20. },
        t.ink,
    );
    d.text(
        x,
        80.,
        format!("Completed: {}", state.practice.completed),
        12.,
        t.muted,
    );
    let button_w = ((l.width - l.margin * 2. - 12.) / 3.).min(190.);
    if !chords {
        for (i, (label, action)) in [
            (
                format!("Clef: {}", state.practice.clef.name()),
                Action::PracticeClef,
            ),
            (
                format!("Level: {}", state.practice.level.name()),
                Action::PracticeLevel,
            ),
            (
                if state.practice.hint {
                    "Hide hint"
                } else {
                    "Show hint"
                }
                .into(),
                Action::PracticeHint,
            ),
        ]
        .into_iter()
        .enumerate()
        {
            d.button(
                state,
                l.margin + i as f32 * (button_w + 6.),
                104.,
                button_w,
                label,
                action,
                t,
            );
        }
    }
    staff(d, state, l, t, chords);
}

fn key_label(key: u8) -> String {
    format!(
        "{}{}",
        [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"
        ][key as usize % 12],
        i16::from(key) / 12 - 1
    )
}
fn staff(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme, chords: bool) {
    d.button(
        state,
        l.margin,
        144.,
        150.,
        "Hear target",
        Action::PracticeListen,
        t,
    );
    let center = l.width * 0.5;
    let half = (l.width * 0.34).clamp(85., 260.);
    let bottom_step = if !chords && state.practice.bass() {
        18
    } else {
        30
    };
    let staff_bottom = if bottom_step == 18 { 14. } else { 4. };
    let unit = |key| {
        staff_bottom - (tangent_core::notation::reading_pitch(key).step - bottom_step) as f32 * 0.5
    };
    let held: BTreeSet<_> = state.held.values().copied().collect();
    let targets = state.practice.target();
    // Include every played pitch in the vertical extent, even outside the
    // visible piano range. Never clamp a wrong octave onto another staff line.
    let mut lo = -1.5_f32;
    let mut hi = 16._f32;
    for &key in targets.iter().chain(held.iter()) {
        lo = lo.min(unit(key) - 4.);
        hi = hi.max(unit(key) + 1.);
    }
    let top = 193.;
    let gap = ((l.strike - 65. - top) / (hi - lo)).clamp(2., 14.);
    let origin = top - lo * gap;
    let treble = origin + 4. * gap;
    let bass = origin + 14. * gap;
    for bottom in [treble, bass] {
        for i in 0..5 {
            d.line(
                center - half,
                bottom - i as f32 * gap,
                half * 2.,
                1.,
                t.staff,
            );
        }
    }
    d.glyph(
        center - half + 3.,
        treble - gap,
        '\u{e050}',
        gap * 4.,
        t.ink,
    );
    d.glyph(
        center - half + 3.,
        bass - gap * 3.,
        '\u{e062}',
        gap * 4.,
        t.ink,
    );
    let bottom = if bottom_step == 18 { bass } else { treble };
    let marker_x = center + (half * 0.52).min(90.);
    for (keys, x, color) in [
        (&targets[..], center, t.ink),
        (
            &held.iter().copied().collect::<Vec<_>>()[..],
            marker_x,
            t.colors[0],
        ),
    ] {
        for &key in keys {
            let pitch = tangent_core::notation::reading_pitch(key);
            let y = bottom - (pitch.step - bottom_step) as f32 * gap * 0.5;
            for step in tangent_core::notation::ledger_steps(pitch.step, bottom_step) {
                d.line(
                    x - gap,
                    bottom - (step - bottom_step) as f32 * gap * 0.5,
                    gap * 2.,
                    1.,
                    color,
                );
            }
            if pitch.sharp {
                d.glyph(x - gap * 2., y, '\u{e262}', gap * 3.5, color);
            }
            d.note(x, y, gap * 0.55, color, false, false);
        }
    }
    let played = held
        .iter()
        .map(|&k| key_label(k))
        .collect::<Vec<_>>()
        .join(" / ");
    let caption = if held.is_empty() {
        "Play a key to see its position".into()
    } else if !chords && held.len() == 1 {
        let distance = i16::from(*held.first().unwrap()) - i16::from(targets[0]);
        if distance == 0 {
            format!("Played {played} · on target")
        } else {
            format!(
                "Played {played} · {} semitones {}",
                distance.abs(),
                if distance > 0 { "above" } else { "below" }
            )
        }
    } else {
        format!("Played {played}")
    };
    d.text(
        l.margin + 8.,
        l.strike - 48.,
        caption,
        if l.width < 600. { 11. } else { 14. },
        t.colors[0],
    );
    d.text(
        l.margin + 8.,
        l.strike - 25.,
        "Filled colour = you   Neutral note = target",
        11.,
        t.muted,
    );
}
