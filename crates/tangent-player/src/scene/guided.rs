//! Shared guided-play integration. Domain gates live in tangent-core.
use super::*;
use tangent_core::{
    guided::{GuidedConfig, GuidedPlay},
    passage::SongContext,
    practice::Grade,
};
#[derive(Default)]
pub(super) struct Session {
    pub enabled: bool,
    pub early_ms: u16,
    pub engine: GuidedPlay,
    key: Option<(String, SongContext)>,
}
impl Session {
    pub fn new() -> Self {
        Self {
            early_ms: 250,
            ..Default::default()
        }
    }
    pub fn invalidate(&mut self) {
        self.key = None;
    }
}
pub(super) fn active(state: &State) -> bool {
    state.guidance.enabled && state.mode == Mode::Play && state.screen == Screen::Session
}
pub(super) fn ensure(state: &mut State) {
    if !active(state) {
        state.guidance.key = None;
        state.guidance.engine = GuidedPlay::default();
        return;
    }
    let context = passage::context(state);
    if state
        .guidance
        .key
        .as_ref()
        .is_none_or(|(id, c)| id != &state.piece_id || c != &context)
        || state.guidance.engine.position() != state.transport.position_us
    {
        state.guidance.engine = GuidedPlay::new(&state.song, &context, state.transport.position_us);
        state.guidance.key = Some((state.piece_id.clone(), context));
    }
}
pub(super) fn advance(state: &mut State, elapsed: u64) -> u64 {
    ensure(state);
    if !active(state) {
        return state.transport.advance(elapsed);
    }
    let was = state.guidance.engine.waiting();
    let wraps = state.guidance.engine.advance(&mut state.transport, elapsed);
    if was != state.guidance.engine.waiting() || wraps > 0 {
        state.sync();
    }
    wraps
}
pub(super) fn judge(state: &mut State, key: u8, fresh: bool) -> Grade {
    ensure(state);
    let held = state.held.values().copied().collect();
    let result = state.guidance.engine.attack(
        &state.transport,
        GuidedConfig {
            early_ms: state.guidance.early_ms,
        },
        key,
        fresh,
        &held,
    );
    let grade = if let Some(target) = result.target {
        state.song_hits.accept_target(target)
    } else {
        result.grade
    };
    if result.advanced {
        state.clock = platform::clock_us();
        state.sync();
    }
    grade
}
pub(super) fn waiting(state: &State) -> bool {
    active(state) && state.transport.playing && state.guidance.engine.waiting()
}
pub(super) fn label(state: &State) -> String {
    format!(
        "Sequential: {}",
        if state.guidance.enabled { "On" } else { "Off" }
    )
}
pub(super) fn render(d: &mut draw::Drawing, state: &State, l: &Layout, t: theme::Theme) {
    if !active(state) {
        return;
    }
    let notes = state
        .guidance
        .engine
        .remaining(&state.held.values().copied().collect());
    let names = notes
        .iter()
        .take(5)
        .map(|&k| tangent_core::explorer::note_name(k))
        .collect::<Vec<_>>()
        .join(" + ");
    let preheld = state.held.values().any(|key| notes.contains(key));
    let text = if !state.transport.playing {
        "Sequential / paused - press Play to continue".into()
    } else if waiting(state) && preheld {
        format!("Release and play again / {names}")
    } else if waiting(state) {
        format!(
            "Waiting / play {names}{}",
            if notes.len() > 5 { " + more" } else { "" }
        )
    } else if state.guidance.engine.barrier().is_some() {
        "Sequential / play each note or chord to continue".into()
    } else {
        "Sequential / selection complete".into()
    };
    let width = (l.width - 2. * l.margin).min(500.);
    d.rect(l.margin, 62., width, 26., t.panel.with_alpha(0.93), 6.);
    d.text(
        l.margin + 8.,
        68.,
        text.chars()
            .take(((width - 16.) / 6.) as usize)
            .collect::<String>(),
        11.,
        if waiting(state) { t.colors[0] } else { t.muted },
    );
}
