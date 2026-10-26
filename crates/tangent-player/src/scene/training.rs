//! Session lifecycle, retryable checkpoints and quiet progress/results UI.
use super::{controls::Action, draw::Drawing, *};
use tangent_core::progress::{Run, SESSION_LENGTH, Training};
#[derive(Default)]
pub(super) struct Sessions {
    pub model: Training,
    pub result: Option<Run>,
    pub pending: BTreeMap<String, Run>,
    pub status: String,
    timer: f32,
    serial: u64,
    pub page: usize,
    millis: f64,
}
fn merge_pending(state: &mut State) {
    for run in state.training.pending.values() {
        if let Some(old) = state
            .training
            .model
            .history
            .iter_mut()
            .find(|r| r.id == run.id)
        {
            *old = run.clone();
        } else {
            state.training.model.history.insert(0, run.clone());
        }
    }
    state.training.model.history.sort_by(|a, b| {
        b.started_unix_ms
            .cmp(&a.started_unix_ms)
            .then_with(|| b.id.cmp(&a.id))
    });
    state.training.model.history.truncate(200);
}
pub(super) fn start(state: &mut State) {
    state.training.serial += 1;
    let id = format!(
        "{}-{}-{}",
        platform::clock_us() as u64,
        state.training.serial,
        session_nonce()
    );
    merge_pending(state);
    state.training.model.start(id, &state.practice);
    if let Some(r) = &mut state.training.model.run {
        r.started_unix_ms = platform::wall_time_ms() as u64;
    }
    state.training.model.choose(&mut state.practice);
}
pub(super) fn start_song(state: &mut State) {
    state.training.serial += 1;
    let id = format!("song-{}-{}", session_nonce(), state.training.serial);
    let count = state
        .song
        .notes
        .iter()
        .filter(|n| state.song.parts.get(n.part).is_some_and(|p| !p.percussion))
        .count() as u32;
    state.training.model.run = Some(Run::song(id, state.song.title.clone(), count));
    if let Some(r) = &mut state.training.model.run {
        r.started_unix_ms = platform::wall_time_ms() as u64;
        r.piece_id = Some(state.piece_id.clone());
    }
    state.song_hits.reset();
}
fn session_nonce() -> u64 {
    // The platform supplies a wall-clock/random identifier on both targets.
    platform::session_nonce()
}
pub(super) fn checkpoint(state: &mut State) {
    if let Some(run) = &mut state.training.model.run
        && run.meaningful()
    {
        run.revision += 1;
        state.training.pending.insert(run.id.clone(), run.clone());
        platform::action("progress-save", &serde_json::to_string(run).unwrap());
    }
}
pub(super) fn end(state: &mut State, show: bool) {
    if let Some(mut run) = state.training.model.run.take() {
        let complete = if run.mode == "play" {
            state.transport.position_us >= state.transport.duration_us
        } else {
            run.trials.len() >= SESSION_LENGTH
        };
        run.close(complete);
        if run.meaningful() {
            state.training.pending.insert(run.id.clone(), run.clone());
            platform::action("progress-save", &serde_json::to_string(&run).unwrap());
        }
        if show {
            state.training.result = Some(run);
        }
    }
    if show {
        state.panic();
        state.screen = Screen::Results;
        state.help = false;
        state.settings_open = false;
    }
}
pub(super) fn grade(state: &mut State, grade: effects::Grade) {
    if state.mode == Mode::Play {
        if let Some(r) = &mut state.training.model.run
            && r.mode == "play"
        {
            r.song_input(grade);
        }
        return;
    }
    if !matches!(state.mode, Mode::Notes | Mode::Chords) {
        return;
    }
    match grade {
        effects::Grade::Correct => state.training.model.answer(&state.practice, false),
        effects::Grade::Wrong => state.training.model.mistake(),
        _ => return,
    }
    if grade == effects::Grade::Correct {
        checkpoint(state);
    } else if let Some(r) = &state.training.model.run {
        state.training.pending.insert(r.id.clone(), r.clone());
    }
}
pub(super) fn released(state: &mut State) {
    if state
        .training
        .model
        .run
        .as_ref()
        .is_some_and(|r| r.trials.len() >= SESSION_LENGTH)
    {
        end(state, true);
    } else {
        state.training.model.ready();
        state.training.model.choose(&mut state.practice);
    }
}
pub(super) fn tick(state: &mut State, dt: f32) {
    if state.screen == Screen::Session
        && !state.help
        && matches!(state.mode, Mode::Notes | Mode::Chords)
        && !state.practice.waiting_release
        && state.held.is_empty()
        && state
            .training
            .model
            .run
            .as_ref()
            .is_some_and(|r| r.trials.len() >= SESSION_LENGTH)
    {
        end(state, true);
        return;
    }

    if state.screen == Screen::Session && !state.help && !state.practice.waiting_release {
        state.training.millis += f64::from(dt) * 1000.;
        let ms = state.training.millis as u64;
        state.training.millis -= ms as f64;
        state.training.model.tick(ms);
    }
    state.training.timer -= dt;
    if state.training.timer <= 0. {
        if state.mode == Mode::Play && state.transport.playing {
            checkpoint(state);
        }
        state.training.timer = 3.;
        for run in state.training.pending.values().take(8) {
            platform::action("progress-save", &serde_json::to_string(run).unwrap());
        }
    }
}
pub(super) fn message(state: &mut State, m: &serde_json::Value) -> bool {
    match m["type"].as_str().unwrap_or("") {
        "progress" => {
            if let Ok(runs) = serde_json::from_value::<Vec<Run>>(m["runs"].clone()) {
                state.training.model.history =
                    runs.into_iter().filter(Run::validate).take(200).collect();
                merge_pending(state);
            }
        }
        "progress-saved" => {
            let id = m["id"].as_str().unwrap_or("");
            if state
                .training
                .pending
                .get(id)
                .is_some_and(|r| Some(r.revision as u64) == m["revision"].as_u64())
            {
                state.training.pending.remove(id);
            }
            state.training.status = if state.training.pending.is_empty() {
                "Saved locally"
            } else {
                "Saving..."
            }
            .into();
            platform::action("progress-list", "");
        }
        "progress-error" => {
            state.training.status = format!(
                "Not saved; retrying: {}",
                m["text"].as_str().unwrap_or("Storage unavailable")
            )
        }
        _ => return false,
    }
    true
}
pub(super) fn render(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let x = l.margin + 8.;
    let w = l.width - 2. * x;
    let result = state.screen == Screen::Results;
    d.text(
        x,
        22.,
        if result {
            "Session results"
        } else {
            "Your progress"
        },
        24.,
        t.ink,
    );
    d.button(
        state,
        l.width - l.margin - 100.,
        18.,
        100.,
        "Main menu",
        Action::Main,
        t,
    );
    if result {
        if let Some(r) = &state.training.result {
            d.text(x, 76., format!("{} / {}", r.mode, r.label), 14., t.muted);
            d.text(x, 112., format!("{} points", r.points), 28., t.colors[0]);
            d.text(
                x,
                156.,
                format!(
                    "{} / {} solved / {}% {}",
                    r.solved(),
                    if r.mode == "play" {
                        r.song_total as usize
                    } else {
                        r.trials.len()
                    },
                    r.accuracy().unwrap_or(0),
                    if r.mode == "play" {
                        "coverage"
                    } else {
                        "first try"
                    }
                ),
                14.,
                t.ink,
            );
            d.text(
                x,
                188.,
                if r.mode == "play" {
                    format!(
                        "{} wrong attacks / {} targets not hit",
                        r.errors, r.song_missed
                    )
                } else {
                    format!(
                        "Errors: {} / assisted: {} / skipped: {}",
                        r.errors,
                        r.trials.iter().filter(|t| t.assisted).count(),
                        r.trials.iter().filter(|t| t.skipped).count()
                    )
                },
                12.,
                t.muted,
            );
            d.text(
                x,
                218.,
                if r.finished && r.mode == "play" {
                    "Piece finished"
                } else if r.finished {
                    "12-task session complete"
                } else {
                    "Session ended early"
                },
                12.,
                t.muted,
            );
        }
        d.text(
            x,
            242.,
            if state.mode == Mode::Play {
                "100 per target / percentage is pitch coverage"
            } else {
                "100 first try / 40 corrected / 0 assisted or skipped"
            },
            if l.width < 600. { 10. } else { 12. },
            t.muted,
        );
        if let Some(r) = &state.training.result
            && r.mode == "play"
        {
            let best = state
                .training
                .model
                .history
                .iter()
                .filter(|h| h.mode == "play" && h.piece_id.is_some() && h.piece_id == r.piece_id)
                .map(|h| h.points)
                .chain(std::iter::once(r.points))
                .max()
                .unwrap_or(0);
            d.text(
                x,
                310.,
                format!("Best recent score for this file: {best}"),
                12.,
                t.colors[0],
            );
        }
        d.button(
            state,
            x,
            265.,
            (w / 2. - 6.).min(220.),
            if state.mode == Mode::Play {
                "Play again"
            } else {
                "Practice again"
            },
            Action::PracticeAgain,
            t,
        );
        d.button(
            state,
            x + (w / 2.).min(232.),
            265.,
            (w / 2. - 6.).min(220.),
            "View progress",
            Action::Progress,
            t,
        );
    } else {
        merge_pending(state);
        let history = &state.training.model.history;
        let points: u64 = history.iter().map(|r| u64::from(r.points)).sum();
        let trials: usize = history.iter().map(|r| r.trials.len()).sum();
        let clean: usize = history
            .iter()
            .filter(|r| r.mode != "play")
            .map(Run::clean)
            .sum();
        d.text(
            x,
            72.,
            format!("Recent sessions: {} / {} points", history.len(), points),
            16.,
            t.ink,
        );
        d.text(
            x,
            104.,
            format!(
                "{}% first try across {} tasks",
                (clean * 100).checked_div(trials).unwrap_or(0),
                trials
            ),
            13.,
            t.muted,
        );
        d.text(
            x,
            134.,
            "Latest 200 sessions; partial sessions included.",
            11.,
            t.muted,
        );
        if history.is_empty() {
            d.text(
                x,
                190.,
                "Complete a note or chord exercise to begin.",
                14.,
                t.ink,
            );
        }
        let focus = state
            .training
            .model
            .review_focus()
            .iter()
            .filter_map(|s| {
                let key = s
                    .rsplit(':')
                    .next()?
                    .split('-')
                    .next()?
                    .parse::<u8>()
                    .ok()?;
                if key > 127 {
                    return None;
                }
                Some(format!(
                    "{}{}{}",
                    [
                        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"
                    ][key as usize % 12],
                    i16::from(key) / 12 - 1,
                    if s.starts_with("chord") { " chord" } else { "" }
                ))
            })
            .collect::<Vec<_>>()
            .join(", ");
        if !focus.is_empty() {
            d.text(x, 154., format!("Review focus: {focus}"), 11., t.colors[0]);
        }
        let rows = ((l.height - 275.) / 60.).clamp(1., 10.) as usize;
        let last = history.len().saturating_sub(1) / rows;
        state.training.page = state.training.page.min(last);
        let page = state.training.page;
        for (i, r) in history.iter().skip(page * rows).take(rows).enumerate() {
            let y = 175. + i as f32 * 60.;
            d.rect(x, y, w, 50., t.panel, 7.);
            let seconds =
                (platform::wall_time_ms() as u64).saturating_sub(r.started_unix_ms) / 1000;
            let ago = if r.started_unix_ms == 0 {
                String::new()
            } else if seconds < 60 {
                "just now".into()
            } else if seconds < 3600 {
                format!("{} min ago", seconds / 60)
            } else if seconds < 86400 {
                format!("{} h ago", seconds / 3600)
            } else {
                format!("{} d ago", seconds / 86400)
            };
            let label = format!("{} / {} / {}", r.mode, r.label, ago);
            d.text(
                x + 10.,
                y + 6.,
                label.chars().take((w / 8.) as usize).collect::<String>(),
                13.,
                t.ink,
            );
            d.text(
                x + 10.,
                y + 28.,
                format!(
                    "{} pts / {} solved / {}% {} / {}",
                    r.points,
                    r.solved(),
                    r.accuracy().unwrap_or(0),
                    if r.mode == "play" {
                        "coverage"
                    } else {
                        "first try"
                    },
                    if state.training.pending.contains_key(&r.id) {
                        "unsaved"
                    } else if r.finished {
                        "complete"
                    } else {
                        "partial"
                    }
                ),
                11.,
                t.muted,
            );
        }
    }
    if !result {
        d.button(
            state,
            x,
            l.height - 80.,
            100.,
            "Previous",
            Action::ProgressPrevious,
            t,
        );
        d.button(
            state,
            x + 110.,
            l.height - 80.,
            100.,
            "Next",
            Action::ProgressNext,
            t,
        );
        d.text(
            x + 230.,
            l.height - 73.,
            format!("Page {}", state.training.page + 1),
            11.,
            t.muted,
        );
    }
    let status =
        if !state.training.pending.is_empty() && !state.training.status.starts_with("Not saved") {
            "Saving locally..."
        } else {
            &state.training.status
        };
    d.text(
        x,
        l.height - 35.,
        status.chars().take((w / 7.) as usize).collect::<String>(),
        11.,
        t.muted,
    );
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn on_exit(mut state: ResMut<State>, mut exits: MessageReader<bevy::app::AppExit>) {
    if exits.read().next().is_some() {
        end(&mut state, false);
    }
}
