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
}
pub(super) fn start(state: &mut State) {
    state.training.serial += 1;
    let id = format!(
        "{}-{}-{}",
        platform::clock_us() as u64,
        state.training.serial,
        session_nonce()
    );
    state.training.model.start(id, &state.practice);
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
        run.finished = if run.mode == "play" {
            state.transport.position_us >= state.transport.duration_us
        } else {
            run.trials.len() >= SESSION_LENGTH
        };
        if run.mode == "play" {
            run.song_missed = run.song_total.saturating_sub(run.song_correct);
        }
        run.revision += 1;
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
            match grade {
                effects::Grade::Correct => {
                    r.song_correct += 1;
                    r.points += 100;
                }
                effects::Grade::Wrong => r.errors += 1,
                _ => return,
            }
            r.revision += 1;
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
    checkpoint(state);
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
    if state.screen == Screen::Session && !state.help && !state.practice.waiting_release {
        state.training.model.tick((dt * 1000.) as u64);
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
                    "{} / {} solved / {}%",
                    r.solved(),
                    if r.mode == "play" {
                        r.song_total as usize
                    } else {
                        r.trials.len()
                    },
                    r.accuracy().unwrap_or(0)
                ),
                14.,
                t.ink,
            );
            d.text(
                x,
                188.,
                format!(
                    "{} input errors / {} assisted / {} skipped",
                    r.errors,
                    r.trials.iter().filter(|t| t.assisted).count(),
                    r.trials.iter().filter(|t| t.skipped).count()
                ),
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
            "100 first try / 40 corrected / 0 helped or skipped",
            if l.width < 600. { 10. } else { 12. },
            t.muted,
        );
        d.button(
            state,
            x,
            265.,
            (w / 2. - 6.).min(220.),
            "Practice again",
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
        let history = &state.training.model.history;
        let points: u32 = history.iter().map(|r| r.points).sum();
        let trials: usize = history.iter().map(|r| r.trials.len()).sum();
        let clean: usize = history
            .iter()
            .filter(|r| r.mode != "play")
            .map(Run::clean)
            .sum();
        d.text(
            x,
            72.,
            format!("{} recent sessions / {} points", history.len(), points),
            16.,
            t.ink,
        );
        d.text(
            x,
            104.,
            format!(
                "{}% first try across {} tasks",
                if trials > 0 { clean * 100 / trials } else { 0 },
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
        let rows = ((l.height - 245.) / 60.).clamp(0., 10.) as usize;
        for (i, r) in history.iter().take(rows).enumerate() {
            let y = 175. + i as f32 * 60.;
            d.rect(x, y, w, 50., t.panel, 7.);
            let label = format!("{} / {}", r.mode, r.label);
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
                    "{} pts / {} solved / {}% first try / {}",
                    r.points,
                    r.solved(),
                    r.accuracy().unwrap_or(0),
                    if r.finished { "complete" } else { "partial" }
                ),
                11.,
                t.muted,
            );
        }
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
