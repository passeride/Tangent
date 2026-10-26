//! T01 pattern/fingering guidance. Input progression and score live in core.
use super::{controls::Action, draw::Drawing, *};
pub(super) fn render(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let plan = state.practice.flow_task().unwrap();
    let step = state.practice.sequence_step();
    let compact = l.width < 600.;
    let short = l.height < 500.;
    let x = l.margin + 8.;
    d.text(
        x,
        18.,
        "Train / Five Finger Flow",
        if compact { 18. } else { 22. },
        t.ink,
    );
    d.text(
        x,
        52.,
        format!(
            "{} position: play up, then down",
            if plan.left { "Left-hand" } else { "Right-hand" }
        ),
        if compact { 13. } else { 19. },
        t.ink,
    );
    let run = state.training.model.run.as_ref();
    d.text(
        x,
        80.,
        format!(
            "{} / 12 patterns / {} pts",
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
                    tangent_core::practice::Level::Stepwise => "Right C",
                    tangent_core::practice::Level::Mixed => "Left C",
                    tangent_core::practice::Level::Chromatic => "Moving",
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
        if compact || short {
            "Release each note. Fingers are suggestions."
        } else {
            "Release each note before the next. Finger numbers are suggestions, not measured."
        },
        if compact { 10. } else { 12. },
        t.muted,
    );
    let row_w = (l.width - 2. * l.margin).min(850.);
    let left = (l.width - row_w) * 0.5;
    let cell = row_w / 9.;
    let y = if short {
        172.
    } else {
        (l.strike * 0.38).max(195.)
    };
    let h = if short {
        (l.strike - 72. - y - 8.).clamp(24., 62.)
    } else {
        112.
    };
    let fingers = plan.fingers();
    for (i, finger) in fingers.iter().enumerate().take(plan.notes.len()) {
        let current = i == step;
        let done = i < step;
        let r = layout::Rect {
            x: left + i as f32 * cell + 2.,
            y,
            w: cell - 4.,
            h,
        };
        d.rect(
            r.x,
            r.y,
            r.w,
            r.h,
            if done {
                t.colors[0].with_alpha(0.14)
            } else {
                t.panel
            },
            5.,
        );
        if current {
            d.rect(r.x, r.y, 3., r.h, t.colors[0], 1.);
            d.rect(r.x, r.y + r.h - 3., r.w, 3., t.colors[0], 1.);
        }
        let font = if compact {
            11.
        } else if short {
            15.
        } else {
            21.
        };
        d.text(
            r.x + 5.,
            r.y + if short { 5. } else { 21. },
            plan.note_label(i),
            font,
            if current { t.colors[0] } else { t.ink },
        );
        if h >= 45. {
            d.text(
                r.x + 5.,
                r.y + h - 22.,
                finger.to_string(),
                if compact { 10. } else { 14. },
                t.muted,
            );
        }
    }
    let message = if state.practice.waiting_release() {
        if state.practice.sequence_accepted() {
            "Pattern complete! Release all keys"
        } else {
            "Correct step! Release all keys"
        }
    } else if state.practice.wrong() {
        "Try again: play the marked note, in the correct octave"
    } else {
        "Play the marked step. Pitch and order only; no timing score."
    };
    d.text(
        x,
        l.strike - 72.,
        message,
        if compact { 10. } else { 13. },
        if state.practice.wrong() {
            t.colors[2]
        } else {
            t.ink
        },
    );
    let assisted = state.practice.hint || state.practice_hear;
    d.text(
        x,
        l.strike - 45.,
        format!(
            "Step {} / 9  /  {}",
            step + 1,
            if assisted {
                "Hint/audio assistance"
            } else {
                "Whole-pattern scoring"
            }
        ),
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
}
