//! Readability presets never change evaluation or cosmetic effect preferences.
use super::{
    controls::Action,
    draw::{Drawing, time},
    *,
};
pub(super) struct RangeSummary {
    source: String,
    context: tangent_core::passage::SongContext,
    first: u8,
    last: u8,
    counts: (usize, usize),
}
fn refresh(state: &mut State) {
    let c = passage::context(state);
    if state.range_summary.as_ref().is_some_and(|r| {
        r.source == state.piece_id
            && r.context == c
            && r.first == state.piano.first
            && r.last == state.piano.last
    }) {
        return;
    }
    let mut counts = (0, 0);
    for n in state
        .song
        .notes
        .iter()
        .filter(|n| c.includes(n) && !state.song.parts[n.part].percussion)
    {
        if n.key < state.piano.first {
            counts.0 += 1;
        } else if n.key > state.piano.last {
            counts.1 += 1;
        }
    }
    state.range_summary = Some(RangeSummary {
        source: state.piece_id.clone(),
        context: c,
        first: state.piano.first,
        last: state.piano.last,
        counts,
    });
}
pub(super) fn render(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    if state.mode != Mode::Play {
        return;
    }
    refresh(state);
    let (below, above) = offscreen(state);
    let right = if below + above > 0 { 160. } else { 75. };
    d.button(
        state,
        l.margin,
        12.,
        50.,
        if state.transport.playing {
            "Pause"
        } else {
            "Play"
        },
        Action::Play,
        t,
    );
    let x = l.margin + 62.;
    let max = ((l.width - x - right) / 8.).max(1.) as usize;
    if state.hud != 0 {
        d.text(
            x,
            10.,
            state.song.title.chars().take(max).collect::<String>(),
            14.,
            t.ink,
        );
    }
    let output = if state.midi_output.is_empty() {
        "Computer audio"
    } else {
        "MIDI out"
    };
    let mut detail = if state.hud == 0 {
        format!("{output} / {}%", state.transport.rate().percent())
    } else {
        format!(
            "{output} / {}% / {} / {}",
            state.transport.rate().percent(),
            time(state.transport.position_us),
            time(state.transport.duration_us)
        )
    };
    if state.hud == 2 && l.width > 650. {
        if let Some(r) = &state.training.model.run {
            detail.push_str(&format!(" / {} pts", r.points));
        }
        let streak = state.feedback.streak();
        if streak >= 2 {
            detail.push_str(&format!(
                " / {streak} streak (best {})",
                state.feedback.best()
            ));
        }
    }
    if state.transport.passage().is_some() {
        detail.push_str(&format!(" / {}", passage::label(state)));
    }
    d.text(
        x,
        if state.hud == 0 { 20. } else { 31. },
        detail
            .chars()
            .take(((l.width - x - right) / 6.).max(1.) as usize)
            .collect::<String>(),
        11.,
        t.muted,
    );
    if below + above > 0 {
        let label = if l.width < 600. {
            format!("Outside view: {below} low / {above} high")
        } else {
            format!("Selected music outside keyboard: {below} lower / {above} higher")
        };
        d.text(l.margin, 49., label, 10., t.colors[1]);
        d.button(
            state,
            l.width - l.margin - 132.,
            12.,
            85.,
            "Fit keys",
            Action::FitRange,
            t,
        );
    }
}
pub(super) fn offscreen(state: &State) -> (usize, usize) {
    let (mut low, mut high) = state.range_summary.as_ref().map_or((0, 0), |r| r.counts);
    for key in state.held.values() {
        if *key < state.piano.first {
            low += 1;
        } else if *key > state.piano.last {
            high += 1;
        }
    }
    (low, high)
}
