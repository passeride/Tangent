//! Bevy passage controls; validation and timing remain in tangent-core.
use super::*;
use tangent_core::passage::{Passage, SongContext};
pub(super) fn context(state: &State) -> SongContext {
    SongContext {
        passage: state.transport.passage().unwrap_or(Passage {
            start_us: 0,
            end_us: state.song.duration_us,
        }),
        duration_us: state.song.duration_us,
        speed_percent: state.transport.rate().percent(),
        part: state.selected_part,
        evaluator: 1,
        song_audio: state.hear,
    }
}
pub(super) fn change(state: &mut State, action: controls::Action) {
    use controls::Action;
    if state.mode != Mode::Play {
        return;
    }
    training::end(state, false);
    state.panic();
    match action {
        Action::LoopA => {
            state.transport.set_passage(None);
            state.loop_a = Some(state.transport.position_us);
            state.status = "A set. Seek forward, pause, then Set B.".into();
        }
        Action::LoopB => {
            let a = state.loop_a.unwrap_or(0);
            if let Some(p) = Passage::new(a, state.transport.position_us, state.song.duration_us) {
                state.transport.set_passage(Some(p));
                state.transport.seek(a);
                state.status =
                    "Loop ready. Resume to repeat; each repeat is a separate attempt.".into();
            } else {
                state.status = "Set B after A. Use +5 seconds or the timeline.".into();
            }
        }
        Action::LoopClear => {
            state.transport.set_passage(None);
            state.loop_a = None;
            state.status = "Loop cleared.".into();
        }
        Action::Part => {
            state.selected_part = match state.selected_part {
                None => state.song.parts.iter().position(|p| !p.percussion),
                Some(i) => state
                    .song
                    .parts
                    .iter()
                    .enumerate()
                    .skip(i + 1)
                    .find(|(_, p)| !p.percussion)
                    .map(|(i, _)| i),
            };
            state.status =
                "Selected part controls targets; song audio remains optional backing.".into();
        }
        _ => {}
    }
    state.song_hits.reset();
    state.feedback.reset();
    state.sync();
}
pub(super) fn label(state: &State) -> String {
    if let Some(p) = state.transport.passage() {
        format!("Loop {}-{}", draw::time(p.start_us), draw::time(p.end_us))
    } else if let Some(a) = state.loop_a {
        format!("A: {} / choose B", draw::time(a))
    } else {
        "Set A, seek forward, then Set B".into()
    }
}
