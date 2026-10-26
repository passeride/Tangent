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
        guided: state
            .guidance
            .enabled
            .then_some(tangent_core::guided::GuidedConfig {
                early_ms: state.guidance.early_ms,
            }),
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
        Action::LoopSnap => {
            if let Some(score) = &state.song.notation {
                let at = state
                    .transport
                    .position_us
                    .min(state.song.duration_us.saturating_sub(1));
                let range = state.transport.passage().unwrap_or(Passage {
                    start_us: at,
                    end_us: at.saturating_add(1),
                });
                if let Some(p) = range.snap_to_measures(score, state.song.duration_us) {
                    state.transport.set_passage(Some(p));
                    state.transport.seek(p.start_us);
                    state.loop_a = Some(p.start_us);
                    state.status =
                        "Snapped to these measure visits; other repeats are unchanged.".into();
                }
            }
        }
        Action::RepetitionGoal => {
            if state.goal.is_some() {
                state.goal = None;
                state.status = "Repetition goal off.".into();
            } else if state.transport.passage().is_some() {
                state.status = "Goal: 3 repeats at 80% pitch coverage; backing allowed.".into();
                state.goal = Some(Goal {
                    source: state.piece_id.clone(),
                    context: context(state),
                    model: tangent_core::passage::RepetitionGoal::new(3, 80).unwrap(),
                });
            } else {
                state.status = "Set an A-B loop before choosing a repetition goal.".into();
            }
        }
        Action::LoopClear => {
            state.goal = None;
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
        if let Some(score) = &state.song.notation
            && let Some((i, a)) = score
                .playback
                .iter()
                .enumerate()
                .find(|(_, v)| v.start_us == p.start_us)
            && let Some((j, b)) = score
                .playback
                .iter()
                .enumerate()
                .find(|(_, v)| v.end_us == p.end_us)
            && let (Some(ma), Some(mb)) =
                (score.measures.get(a.measure), score.measures.get(b.measure))
        {
            return format!(
                "Bars {}-{} / visits {}-{}",
                ma.label,
                mb.label,
                i + 1,
                j + 1
            );
        }
        format!("Loop {}-{}", draw::time(p.start_us), draw::time(p.end_us))
    } else if let Some(a) = state.loop_a {
        format!("A: {} / choose B", draw::time(a))
    } else {
        "Set A, seek forward, then Set B".into()
    }
}

pub(super) struct Goal {
    source: String,
    context: SongContext,
    pub model: tangent_core::passage::RepetitionGoal,
}
pub(super) fn goal_tick(state: &mut State) {
    let c = context(state);
    if let Some(g) = &mut state.goal
        && (g.context != c || g.source != state.piece_id)
    {
        g.source = state.piece_id.clone();
        g.context = c;
        g.model = tangent_core::passage::RepetitionGoal::new(3, 80).unwrap();
    }
}
pub(super) fn record_goal(state: &mut State, run: &tangent_core::progress::Run) {
    if let Some(g) = &mut state.goal
        && run.piece_id.as_ref() == Some(&g.source)
        && run.context.as_ref() == Some(&g.context)
    {
        g.model.record(run);
    }
}
pub(super) fn goal_label(state: &State) -> String {
    state.goal.as_ref().map_or("Goal: off".into(), |g| {
        format!("80% pitch: {}/{}", g.model.completed(), g.model.required())
    })
}
