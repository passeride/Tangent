//! N02 presentation. Task generation and exact-pitch evaluation live in core.
use super::{controls::Action, draw::Drawing, *};
use tangent_core::explorer::note_name;

pub(super) fn render(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let task = state.practice.explorer_task().unwrap();
    let compact = l.width < 600.;
    let short = l.height < 500.;
    let x = l.margin + 8.;
    d.text(
        x,
        18.,
        "Train / Keyboard Explorer",
        if compact { 18. } else { 22. },
        t.ink,
    );
    d.text(
        x,
        52.,
        if task.higher() {
            "Find the same note one octave HIGHER"
        } else {
            "Find the same note one octave LOWER"
        },
        if compact { 13. } else { 19. },
        t.ink,
    );
    let run = state.training.model.run.as_ref();
    d.text(
        x,
        80.,
        format!(
            "{} / 12 tasks / {} pts",
            run.map_or(0, |r| r.trials.len()),
            run.map_or(0, |r| r.points)
        ),
        12.,
        t.muted,
    );
    d.button(
        state,
        l.width - l.margin - 86.,
        70.,
        86.,
        "Finish",
        Action::PracticeFinish,
        t,
    );
    let w = ((l.width - l.margin * 2. - 12.) / 3.).min(210.);
    for (i, (label, action)) in [
        (
            format!(
                "Level: {}",
                match state.practice.level {
                    tangent_core::practice::Level::Stepwise => "C/D/E",
                    tangent_core::practice::Level::Mixed => "White keys",
                    tangent_core::practice::Level::Chromatic => "All keys",
                }
            ),
            Action::PracticeLevel,
        ),
        (
            (if state.practice.hint {
                "Hide hint"
            } else {
                "Show hint"
            })
            .into(),
            Action::PracticeHint,
        ),
        (
            format!("Hear: {}", if state.practice_hear { "On" } else { "Off" }),
            Action::PracticeListen,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        d.button(
            state,
            l.margin + i as f32 * (w + 6.),
            104.,
            w,
            label,
            action,
            t,
        );
    }
    d.text(
        x,
        145.,
        if short {
            "An octave = 12 semitones"
        } else {
            "An octave is 12 semitones: the same name, a different register."
        },
        if compact { 10. } else { 13. },
        t.muted,
    );
    let card_w = (l.width - 2. * l.margin).min(590.);
    let card_x = (l.width - card_w) * 0.5;
    let card_y = if short {
        172.
    } else {
        (l.strike * 0.43).max(200.)
    };
    let card_h = if short {
        (l.strike - 72. - card_y - 8.).clamp(24., 63.)
    } else {
        120.
    };
    d.rect(card_x, card_y, card_w, card_h, t.panel, 12.);
    let destination = if state.practice.hint || state.practice.waiting_release {
        note_name(task.target)
    } else {
        "?".into()
    };
    if short {
        d.text(
            card_x + 12.,
            card_y + (card_h - 20.) * 0.5,
            format!("REF {}", note_name(task.source)),
            16.,
            t.ink,
        );
        d.text(
            card_x + card_w * 0.55,
            card_y + (card_h - 20.) * 0.5,
            format!("YOU {destination}"),
            16.,
            t.colors[0],
        );
    } else {
        d.text(card_x + 16., card_y + 10., "REFERENCE", 10., t.colors[2]);
        d.text(
            card_x + 16.,
            card_y + 38.,
            note_name(task.source),
            44.,
            t.ink,
        );
        d.text(
            card_x + card_w * 0.55,
            card_y + 10.,
            "YOUR OCTAVE",
            10.,
            t.muted,
        );
        d.text(
            card_x + card_w * 0.55,
            card_y + 38.,
            destination,
            44.,
            t.colors[0],
        );
    }
    let feedback: String = if state.practice.waiting_release {
        "Correct! Release all keys to continue".into()
    } else if state.practice.wrong {
        "Try again: same note name, exactly one octave away".into()
    } else if state.practice.hint {
        "Hint shown: this task is assisted".into()
    } else if state.practice_hear {
        "Target audio is on: this task is assisted".into()
    } else {
        "Use MIDI, your computer keyboard or the screen piano".into()
    };
    d.text(
        x,
        l.strike - 72.,
        feedback,
        if compact { 10. } else { 13. },
        if state.practice.wrong {
            t.colors[2]
        } else {
            t.ink
        },
    );
    let held = state
        .held
        .values()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(note_name)
        .collect::<Vec<_>>()
        .join(" / ");
    d.text(
        x,
        l.strike - 45.,
        if held.is_empty() {
            "REFERENCE marker identifies the starting key".into()
        } else {
            format!("Played {held}")
        },
        if compact { 10. } else { 12. },
        t.muted,
    );
    d.button(
        state,
        l.width - l.margin - 86.,
        l.strike - 34.,
        86.,
        "Skip",
        Action::PracticeSkip,
        t,
    );
    if let Some(r) = l.key(&state.piano, task.source) {
        // A labelled pointer is the task stimulus, never a solid played key or
        // the expected-key glow. Revealing the destination is an explicit hint.
        let marker = (r.x + r.w * 0.5).clamp(l.margin + 20., l.width - l.margin - 20.);
        d.text(marker - 9., l.strike - 20., "REF", 9., t.colors[2]);
        d.rect(marker - 2., l.strike - 6., 4., 6., t.colors[2], 1.);
    }
}
