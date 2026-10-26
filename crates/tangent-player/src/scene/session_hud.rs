//! Small, real-state session chrome following the approved UI v2 hierarchy.
use super::{
    controls::Action,
    draw::{Drawing, time},
    *,
};
pub(super) fn render(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    if state.mode != Mode::Play {
        return;
    }
    let label = if state.transport.playing {
        "Pause"
    } else {
        "Play"
    };
    d.button(state, l.margin, 12., 50., label, Action::Play, t);
    let x = l.margin + 62.;
    let max = ((l.width - x - 75.) / 8.).max(1.) as usize;
    let title: String = state.song.title.chars().take(max).collect();
    d.text(x, 10., title, 14., t.ink);
    let detail = format!(
        "{} / {} / {}%",
        time(state.transport.position_us),
        time(state.transport.duration_us),
        state.transport.rate().percent()
    );
    let output = if state.midi_output.is_empty() {
        "Computer"
    } else {
        "MIDI out"
    };
    let streak = state.feedback.streak();
    let detail = if l.width > 650. && streak >= 2 {
        format!(
            "{detail}  /  {output}  /  {streak} streak (best {})",
            state.feedback.best()
        )
    } else {
        format!("{detail}  /  {output}")
    };
    let detail = if l.width > 650. {
        if let Some(r) = &state.training.model.run {
            format!("{detail} / {} pts", r.points)
        } else {
            detail
        }
    } else {
        detail
    };
    d.text(x, 31., detail, 11., t.muted);
}
