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
pub(super) fn exercise(d: &mut Drawing, state: &State, l: &Layout, t: theme::Theme) {
    let x = l.margin + 15.;
    d.text(
        x,
        22.,
        if state.mode == Mode::Chords {
            "Train / chords"
        } else {
            "Train / notes"
        },
        20.,
        t.ink,
    );
    d.text(
        x,
        60.,
        format!("Play {}", state.practice.label()),
        24.,
        t.colors[0],
    );
    d.text(
        x,
        100.,
        format!("Completed: {}", state.practice.completed),
        12.,
        t.muted,
    );
    d.text(x, 120., "Release all keys between answers", 11., t.muted);
    // A target grand staff provides a reading exercise, with no timing pressure.
    let center = l.width * 0.5;
    let gap = ((l.strike - 170.) / 18.).clamp(4., 10.);
    let top = 155.;
    for base in [0., 8.] {
        for i in 0..5 {
            d.line(
                center - 95.,
                top + (base + i as f32) * gap,
                190.,
                1.,
                t.staff,
            );
        }
    }
    d.glyph(center - 90., top + 3. * gap, '\u{e050}', gap * 4., t.ink);
    d.glyph(center - 90., top + 9. * gap, '\u{e062}', gap * 4., t.ink);
    for key in state.practice.target() {
        let pitch = tangent_core::notation::reading_pitch(key);
        let y = top + 5. * gap
            - (pitch.step - tangent_core::notation::reading_pitch(60).step) as f32 * gap * 0.5;
        if key == 60 {
            d.line(center - 12., y, 24., 1., t.ink);
        }
        d.note(center, y, gap * 0.6, t.colors[0], false, false);
    }
}
