//! Navigation and exercise presentation share the single Bevy drawing surface.
use super::{controls::Action, draw::Drawing, *};
pub(super) fn render(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let w = (l.width - 40.).clamp(120., 340.);
    let x = (l.width - w) * 0.5;
    let top = (l.height * 0.12).min(90.);
    // Original piano-key wordmark, built from geometry rather than an external asset.
    let logo_x = l.width * 0.5 - 49.;
    for i in 0..7 {
        d.rect(logo_x + i as f32 * 14., top, 12., 44., t.white, 2.);
    }
    for i in [0, 1, 3, 4, 5] {
        d.rect(logo_x + i as f32 * 14. + 9., top, 8., 27., t.black, 1.);
    }
    d.text(l.width * 0.5 - 84., top + 58., "TANGENT", 32., t.ink);
    d.text(
        x,
        top + 109.,
        if state.screen == Screen::Train {
            "Train / choose an exercise"
        } else {
            "Make time for music."
        },
        15.,
        t.muted,
    );
    let items: Vec<(&str, Action)> = if state.screen == Screen::Train {
        vec![
            ("Notes", Action::TrainNotes),
            ("Chords", Action::TrainChords),
            ("Back", Action::Main),
        ]
    } else {
        vec![
            ("Play", Action::StartPlay),
            ("Library", Action::Library),
            ("Train", Action::Train),
            ("FreeForm", Action::FreeForm),
            ("Settings", Action::Help),
        ]
    };
    let step = ((l.height - top - 160.) / items.len() as f32).clamp(35., 51.);
    for (i, (label, action)) in items.into_iter().enumerate() {
        d.button(state, x, top + 145. + i as f32 * step, w, label, action, t);
    }
}
pub(super) fn exercise(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
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
    // The same bottom-line anchors as the score renderer: E4=30, G2=18.
    // Independent bass placement fixes the old treble-relative positioning.
    let gap = ((l.strike - 165.) / 18.).clamp(4., 14.);
    let top = if chords { 140. } else { 160. };
    let treble = top + 4. * gap;
    let bass = treble + 10. * gap;
    let center = l.width * 0.5;
    let half = (l.width * 0.34).clamp(85., 260.);
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
    let is_bass = !chords && state.practice.bass();
    let bottom = if is_bass { bass } else { treble };
    let bottom_step = if is_bass { 18 } else { 30 };
    for key in state.practice.target() {
        let pitch = tangent_core::notation::reading_pitch(key);
        let y = bottom - (pitch.step - bottom_step) as f32 * gap * 0.5;
        for step in tangent_core::notation::ledger_steps(pitch.step, bottom_step) {
            d.line(
                center - gap,
                y + (pitch.step - step) as f32 * gap * 0.5,
                gap * 2.,
                1.,
                t.ink,
            );
        }
        if pitch.sharp {
            d.glyph(center - gap * 2., y, '\u{e262}', gap * 3.5, t.ink);
        }
        d.note(center, y, gap * 0.55, t.ink, false, false);
    }
    d.text(
        x,
        l.strike - 25.,
        "Solid = played   Glow = song / hint",
        11.,
        t.muted,
    );
}
