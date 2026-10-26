use super::*;
#[derive(Clone, Copy, Debug)]
pub enum Action {
    #[cfg(not(target_arch = "wasm32"))]
    Exit,
    Main,
    Progress,
    ProgressPrevious,
    ProgressNext,
    ProgressOpen(usize),
    PracticeFinish,
    PracticeAgain,
    PracticeSection,
    PracticeSkip,
    PracticeAdaptive,
    Library,
    LibraryOpen(usize),
    LibrarySearch,
    LibraryClearSearch,
    LibrarySort,
    LibraryFormat,
    LibraryEdit(usize),
    LibraryTitle,
    LibraryComposer,
    LibrarySaveMetadata,
    LibraryCancelEdit,
    LibraryRemove(usize),
    LibraryPrevious,
    LibraryNext,
    Train,
    StartPlay,
    FreeForm,
    TrainNotes,
    TrainChords,
    Play,
    Restart,
    Slower,
    Faster,
    NormalSpeed,
    LoopA,
    LoopB,
    LoopClear,
    LoopSnap,
    RepetitionGoal,
    Part,
    Back,
    Forward,
    Quieter,
    Louder,
    Fullscreen,
    View,
    Theme,
    Hear,
    Effects,
    EffectStrength,
    ReducedMotion,
    Instrument,
    Range,
    FitRange,
    Hud,
    PassageSetup,
    Import,
    Sample,
    Midi,
    MidiOutput,
    MidiOutputSelect(Option<usize>),
    MidiOutputPage(bool),
    PracticeClef,
    PracticeDefaults,
    PracticeLevel,
    PracticeHint,
    PracticeListen,
    Help,
    Settings,
    SettingsBack,
    SettingsTab(SettingsTab),
    Credits,
    ExportDatabase,
}
impl Action {
    #[cfg(target_arch = "wasm32")]
    pub fn id(self) -> &'static str {
        match self {
            Self::Main => "main",
            Self::Progress => "progress",
            Self::ProgressPrevious => "progress-previous",
            Self::ProgressNext => "progress-next",
            Self::ProgressOpen(_) => "progress-open",
            Self::PracticeFinish => "practice-finish",
            Self::PracticeAgain => "practice-again",
            Self::PracticeSection => "practice-section",
            Self::PracticeSkip => "practice-skip",
            Self::PracticeAdaptive => "practice-adaptive",
            Self::Library => "library",
            Self::LibraryOpen(_) => "library-open",
            Self::LibrarySearch => "library-search",
            Self::LibraryClearSearch => "library-clear-search",
            Self::LibrarySort => "library-sort",
            Self::LibraryFormat => "library-format",
            Self::LibraryEdit(_) => "library-edit",
            Self::LibraryTitle => "library-title",
            Self::LibraryComposer => "library-composer",
            Self::LibrarySaveMetadata => "library-save-metadata",
            Self::LibraryCancelEdit => "library-cancel-edit",
            Self::LibraryRemove(_) => "library-remove",
            Self::LibraryPrevious => "library-previous",
            Self::LibraryNext => "library-next",
            Self::Train => "train",
            Self::StartPlay => "start-play",
            Self::FreeForm => "freeform",
            Self::TrainNotes => "train-notes",
            Self::TrainChords => "train-chords",
            Self::Play => "play",
            Self::Slower => "speed-slower",
            Self::Faster => "speed-faster",
            Self::NormalSpeed => "speed-reset",
            Self::LoopA => "loop-a",
            Self::LoopB => "loop-b",
            Self::LoopClear => "loop-clear",
            Self::LoopSnap => "loop-snap",
            Self::RepetitionGoal => "repetition-goal",
            Self::Part => "practice-part",
            Self::Restart => "restart",
            Self::Back => "back",
            Self::Forward => "forward",
            Self::Quieter => "quieter",
            Self::Louder => "louder",
            Self::Fullscreen => "fullscreen",
            Self::View => "view",
            Self::Theme => "theme",
            Self::Hear => "hear",
            Self::Effects => "effects",
            Self::EffectStrength => "effect-strength",
            Self::ReducedMotion => "reduced-motion",
            Self::Instrument => "instrument",
            Self::Range => "range",
            Self::FitRange => "fit-range",
            Self::Hud => "hud",
            Self::PassageSetup => "passage-setup",
            Self::Import => "import",
            Self::Sample => "sample",
            Self::Midi => "midi",
            Self::MidiOutput => "midi-output",
            Self::MidiOutputSelect(Some(_)) => "midi-output-choice",
            Self::MidiOutputSelect(None) => "midi-output-computer",
            Self::MidiOutputPage(true) => "midi-output-next",
            Self::MidiOutputPage(false) => "midi-output-prev",
            Self::PracticeClef => "practice-clef",
            Self::PracticeDefaults => "practice-defaults",
            Self::PracticeLevel => "practice-level",
            Self::PracticeHint => "practice-hint",
            Self::PracticeListen => "practice-listen",
            Self::Help => "help",
            Self::Settings => "settings",
            Self::SettingsBack => "settings-back",
            Self::SettingsTab(tab) => match tab {
                SettingsTab::AudioMidi => "settings-audio",
                SettingsTab::Visuals => "settings-visuals",
                SettingsTab::Practice => "settings-practice",
                SettingsTab::Effects => "settings-effects",
                SettingsTab::Accessibility => "settings-accessibility",
                SettingsTab::Data => "settings-data",
            },
            Self::Credits => "credits",
            Self::ExportDatabase => "export-database",
        }
    }
}
fn activate(state: &mut State, action: Action, shift: bool) {
    state.focused_button = None;
    if state.mode != Mode::Play
        && matches!(action, Action::Restart | Action::Back | Action::Forward)
    {
        return;
    }
    let step = if shift { 15_000_000 } else { 5_000_000 };
    match action {
        #[cfg(not(target_arch = "wasm32"))]
        Action::Exit => {
            training::end(state, false);
            state.panic();
            platform::action("quit", "");
        }
        Action::ProgressOpen(index) => {
            if let Some(run) = state.training.model.history.get(index).cloned() {
                state.training.status = if state.training.pending.contains_key(&run.id) {
                    "Saving locally..."
                } else {
                    "Saved locally"
                }
                .into();
                state.training.result = Some(run);
                state.screen = Screen::Results;
            }
        }
        Action::ProgressPrevious => state.training.page = state.training.page.saturating_sub(1),
        Action::ProgressNext => state.training.page += 1,
        Action::Progress => {
            state.training.page = 0;
            training::end(state, false);
            state.panic();
            state.help = false;
            state.settings_open = false;
            state.screen = Screen::Progress;
            platform::action("progress-list", "");
        }
        Action::PracticeFinish => training::end(state, true),
        Action::PracticeSection | Action::PracticeAgain => {
            if let Some(run) = state.training.result.clone() {
                if run.mode == "play" {
                    if run.piece_id.as_ref() != Some(&state.piece_id) {
                        activate(state, Action::Library, false);
                        return;
                    }
                    activate(state, Action::StartPlay, false);
                    state.transport.set_passage(None);
                    if let Some(c) = run.context.as_ref() {
                        state.selected_part = c.part;
                        state.hear = c.song_audio;
                        if let Some(rate) =
                            tangent_core::transport::PlaybackRate::new(c.speed_percent)
                        {
                            state.transport.set_rate(rate);
                        }
                        let range = if matches!(action, Action::PracticeSection) {
                            run.weak_passage(&state.song)
                        } else {
                            Some(c.passage)
                        };
                        if let Some(p) = range {
                            state.transport.set_passage(Some(p));
                            state.loop_a = Some(p.start_us);
                        }
                    }
                    activate(state, Action::Restart, false);
                    // Retry is deliberately paused so the user can inspect range/part/rate.
                    state.help = true;
                } else {
                    use tangent_core::practice::{Level, ReadingClef};
                    let action = if run.mode == "chords" {
                        Action::TrainChords
                    } else {
                        Action::TrainNotes
                    };
                    state.practice.configure(
                        [ReadingClef::Treble, ReadingClef::Bass, ReadingClef::Mixed]
                            [run.clef as usize],
                        [Level::Stepwise, Level::Mixed, Level::Chromatic][run.level as usize],
                    );
                    activate(state, action, false);
                }
            }
        }
        Action::PracticeSkip => {
            if !state.practice.waiting_release && state.training.model.run.is_some() {
                state.training.model.answer(&state.practice, true);
                training::checkpoint(state);
                state.panic();
                state.practice.completed += 1;
                state.practice.hint = false;
                training::released(state);
            }
        }
        Action::PracticeAdaptive => {
            state.training.model.adaptive = !state.training.model.adaptive;
        }
        Action::LibrarySearch => state.library.focus(library::Field::Search),
        Action::LibraryClearSearch => {
            state.library.query.clear();
            state.library.focus(library::Field::Search);
            state.library.reset_page();
        }
        Action::LibrarySort => {
            state.library.sort = (state.library.sort + 1) % library::SORTS.len();
            state.library.reset_page();
        }
        Action::LibraryFormat => {
            state.library.format = (state.library.format + 1) % library::FORMATS.len();
            state.library.reset_page();
        }
        Action::LibraryEdit(index) => {
            if let Some(entry) = state.library.items.get(index) {
                state.library.edit = Some(library::Edit {
                    id: entry.id.clone(),
                    title: entry.title.clone(),
                    composer: entry.composer.clone(),
                });
                state.library.focus(library::Field::Title);
                state.library.pending_delete = None;
                state.library.status = "Edit title or composer, then Save.".into();
            }
        }
        Action::LibraryTitle => state.library.focus(library::Field::Title),
        Action::LibraryComposer => state.library.focus(library::Field::Composer),
        Action::LibrarySaveMetadata => {
            if let Some(edit) = &state.library.edit {
                if edit.validate() {
                    platform::action("library-edit", &serde_json::to_string(edit).unwrap());
                    state.library.status = "Saving metadata…".into();
                } else {
                    state.library.status =
                        "Enter a title; fields may contain up to 160 characters.".into();
                }
            }
        }
        Action::LibraryCancelEdit => {
            state.library.edit = None;
            state.library.focus = None;
        }
        Action::Library => {
            training::end(state, false);
            state.panic();
            state.help = false;
            state.settings_open = false;
            state.screen = Screen::Library;
            state.library.pending_delete = None;
            state.library.focus = None;
            state.library.edit = None;
            state.library.refresh();
        }
        Action::LibraryPrevious | Action::LibraryNext => {
            let page_size = state.library.page_size;
            if matches!(action, Action::LibraryPrevious) {
                state.library.offset = state.library.offset.saturating_sub(page_size);
            } else if state.library.offset + page_size < state.library.total {
                state.library.offset += page_size;
            }
            state.library.pending_delete = None;
            state.library.refresh();
        }
        Action::LibraryOpen(index) => {
            if let Some(entry) = state.library.items.get(index) {
                let id = entry.id.clone();
                state.panic();
                state.help = false;
                state.settings_open = false;
                state.screen = Screen::Session;
                state.mode = Mode::Play;
                state.status = "Opening piece…".into();
                platform::action("library-open", &id);
            }
        }
        Action::LibraryRemove(index) => {
            if let Some(entry) = state.library.items.get(index) {
                let id = entry.id.clone();
                if state.library.pending_delete.as_ref() == Some(&id) {
                    platform::action("library-remove", &id);
                    state.library.pending_delete = None;
                } else {
                    state.library.pending_delete = Some(id);
                }
            }
        }
        Action::Main | Action::Train => {
            training::end(state, false);
            state.panic();
            state.help = false;
            state.settings_open = false;
            state.screen = if matches!(action, Action::Main) {
                Screen::Main
            } else {
                Screen::Train
            };
        }
        Action::StartPlay | Action::FreeForm | Action::TrainNotes | Action::TrainChords => {
            training::end(state, false);
            state.panic();
            state.help = false;
            state.settings_open = false;
            state.screen = Screen::Session;
            state.mode = match action {
                Action::FreeForm => Mode::FreeForm,
                Action::TrainNotes => Mode::Notes,
                Action::TrainChords => Mode::Chords,
                _ => Mode::Play,
            };
            let (clef, level) = (state.practice.clef, state.practice.level);
            state.practice = tangent_core::practice::Practice::new(state.mode == Mode::Chords);
            state.practice.configure(clef, level);
            if matches!(state.mode, Mode::Notes | Mode::Chords) {
                training::start(state);
            }
            state.particles.clear();
            state.live_bursts.clear();
        }
        Action::Play => {
            if state.screen != Screen::Session || state.settings_open {
                return;
            }
            state.help = false;
            if state.mode != Mode::Play {
                return;
            }
            if state.transport.position_us >= state.transport.duration_us {
                state.transport.seek(0);
            }
            if !state.transport.playing
                && state.transport.position_us
                    == state.transport.passage().map_or(0, |p| p.start_us)
                && state.training.model.run.is_none()
            {
                training::start_song(state);
            }
            if state.transport.playing {
                training::checkpoint(state);
            }
            state.transport.playing = !state.transport.playing;
            state.clock = platform::clock_us();
            state.sync();
        }
        Action::LoopA
        | Action::LoopB
        | Action::LoopClear
        | Action::LoopSnap
        | Action::RepetitionGoal
        | Action::Part => passage::change(state, action),
        Action::Slower | Action::Faster | Action::NormalSpeed => {
            training::end(state, false);
            use tangent_core::transport::PlaybackRate;
            let now = platform::clock_us();
            state.transport.advance((now - state.clock).max(0.) as u64);
            state.clock = now;
            let current = state.transport.rate().percent();
            let percent = match action {
                Action::Slower => current.saturating_sub(5).max(PlaybackRate::MIN),
                Action::Faster => (current + 5).min(PlaybackRate::MAX),
                _ => 100,
            };
            state
                .transport
                .set_rate(PlaybackRate::new(percent).unwrap());
            state.sync();
        }
        Action::Restart => {
            training::end(state, false);
            state.song_hits.reset();
            state.feedback.reset();
            let start = state.transport.passage().map_or(0, |p| p.start_us);
            state.transport.seek(start);
            state.clock = platform::clock_us();
            if state.transport.playing {
                training::start_song(state);
            }
            state.sync();
        }
        Action::Back => {
            training::end(state, false);
            state.song_hits.reset();
            state.feedback.reset();
            let pos = state.transport.position_us.saturating_sub(step);
            state.transport.seek(pos);
            state.sync();
        }
        Action::Forward => {
            training::end(state, false);
            state.song_hits.reset();
            state.feedback.reset();
            let pos = state.transport.position_us.saturating_add(step);
            state.transport.seek(pos);
            state.sync();
        }
        Action::Quieter => {
            state.volume = (state.volume - 0.05).max(0.);
            platform::action("volume", &state.volume.to_string());
        }
        Action::Louder => {
            state.volume = (state.volume + 0.05).min(1.);
            platform::action("volume", &state.volume.to_string());
        }
        Action::Fullscreen => platform::action("fullscreen", ""),
        Action::View => {
            state.view = match state.view {
                View::Blocks => View::Staff,
                View::Staff => View::Notes,
                View::Notes => View::Score,
                View::Score => View::Blocks,
            }
        }
        Action::Theme => state.theme = (state.theme + 1) % 3,
        Action::Hear => {
            training::end(state, false);
            state.hear = !state.hear;
            state.sync();
        }
        Action::Effects => {
            state.effects = !state.effects;
            state.particles.clear();
            state.ripples.clear();
            #[cfg(not(target_arch = "wasm32"))]
            state.gpu_bursts.clear();
        }
        Action::Instrument => {
            state.instrument = (state.instrument + 1) % 2;
            state.sync();
        }
        Action::PassageSetup => {
            state.practice_defaults = false;
            state.panic();
            state.help = true;
            state.settings_open = true;
            state.settings_tab = SettingsTab::Practice;
        }
        Action::Hud => {
            state.hud = (state.hud + 1) % 3;
        }
        Action::FitRange => {
            let mut keys: Vec<u8> = state.held.values().copied().collect();
            if state.mode == Mode::Play {
                let c = passage::context(state);
                keys.extend(
                    state
                        .song
                        .notes
                        .iter()
                        .filter(|n| c.includes(n) && !state.song.parts[n.part].percussion)
                        .map(|n| n.key),
                );
            } else if matches!(state.mode, Mode::Notes | Mode::Chords) {
                keys.extend(state.practice.target());
            }
            let (first, last) = tangent_core::piano::fit_range(keys);
            state.piano = PianoLayout::new(first, last).unwrap();
            state.status="Keyboard fitted to selected music and played notes. Out-of-piano pitches remain flagged.".into();
        }
        Action::Range => {
            state.piano = match state.piano.white_count {
                52 => PianoLayout::new(36, 84),
                29 => PianoLayout::new(36, 96),
                _ => PianoLayout::new(21, 108),
            }
            .unwrap()
        }
        Action::Help => {
            if state.settings_open {
                state.settings_open = false;
                state.help = state.screen == Screen::Session;
            } else {
                state.help = !state.help;
                if state.help {
                    state.panic();
                }
            }
        }
        Action::Settings => {
            state.practice_defaults = false;
            if !state.help {
                state.panic();
            }
            state.help = true;
            state.settings_open = true;
            state.midi_dropdown_open = false;
            state.settings_tab = SettingsTab::AudioMidi;
        }
        Action::SettingsBack => {
            state.midi_dropdown_open = false;
            state.settings_open = false;
            state.help = state.screen == Screen::Session;
        }
        Action::SettingsTab(tab) => {
            state.midi_dropdown_open = false;
            state.settings_tab = tab;
        }
        Action::Import | Action::Sample => {
            state.help = state.screen != Screen::Library;
            state.panic();
            platform::action(
                if matches!(action, Action::Import) {
                    "import"
                } else {
                    "sample"
                },
                "",
            );
        }
        Action::PracticeDefaults => {
            state.practice_defaults = !state.practice_defaults;
        }
        Action::PracticeClef | Action::PracticeLevel => {
            let clef = if matches!(action, Action::PracticeClef) {
                state.practice.clef.next()
            } else {
                state.practice.clef
            };
            let level = if matches!(action, Action::PracticeLevel) {
                state.practice.level.next()
            } else {
                state.practice.level
            };
            let active_practice = matches!(state.mode, Mode::Notes | Mode::Chords);
            if active_practice {
                training::end(state, false);
            }
            state.panic();
            state.practice.configure(clef, level);
            if state.screen == Screen::Session && active_practice {
                training::start(state);
            }
        }
        Action::EffectStrength => state.effect_strength = (state.effect_strength + 1) % 3,
        Action::ReducedMotion => {
            state.reduced_motion = !state.reduced_motion;
            state.particles.clear();
            state.ripples.clear();
            state.live_bursts.clear();
        }
        Action::PracticeListen => {
            state.practice_hear = !state.practice_hear;
            state.training.heard_target = None;
            if !state.practice_hear {
                state.stop_reference();
            }
        }
        Action::PracticeHint => {
            state.practice.hint = !state.practice.hint;
            if state.practice.hint && !state.practice.waiting_release {
                state.training.model.assist();
            }
        }
        Action::MidiOutput => {
            state.midi_dropdown_open = !state.midi_dropdown_open;
            state.midi_dropdown_page = 0;
        }
        Action::MidiOutputSelect(index) => {
            // Persist a human-stable device name, never the scan-local port ID.
            // A missing device is not silently mapped to a different output.
            let selection = match index {
                None => Some((String::new(), None)),
                Some(i) => state
                    .midi_outputs
                    .get(i)
                    .map(|p| (p.id.clone(), Some(p.name.clone()))),
            };
            if let Some((id, name)) = selection {
                state.midi_dropdown_open = false;
                state.preferred_midi_output = name;
                state.midi_restore_pending = false;
                state.panic();
                platform::action("midi-output", &id);
            }
        }
        Action::MidiOutputPage(next) => {
            let pages = state.midi_outputs.len().div_ceil(5).max(1);
            if next {
                state.midi_dropdown_page = (state.midi_dropdown_page + 1).min(pages - 1);
            } else {
                state.midi_dropdown_page = state.midi_dropdown_page.saturating_sub(1);
            }
        }
        Action::Midi => {
            state.midi_dropdown_open = false;
            state.midi_restore_pending = state.preferred_midi_output.is_some();
            state.panic();
            platform::action("midi", "");
        }
        Action::Credits => platform::action("credits", ""),
        Action::ExportDatabase => platform::action("export-database", ""),
    }
}
pub fn input(
    mut state: ResMut<State>,
    windows: Query<&Window>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    time: Res<Time>,
    mut text_events: MessageReader<bevy::input::keyboard::KeyboardInput>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    if !state.viewport_set {
        if window.width() < 650. {
            state.piano = PianoLayout::new(36, 84).unwrap();
        }
        state.viewport_set = true;
    }
    let layout = Layout::new(window.width(), window.height(), &state.piano);
    let shift = keys.pressed(KeyCode::ShiftLeft)
        || keys.pressed(KeyCode::ShiftRight)
        || keys.just_pressed(KeyCode::ShiftLeft)
        || keys.just_pressed(KeyCode::ShiftRight);
    let ctrl = keys.pressed(KeyCode::ControlLeft)
        || keys.pressed(KeyCode::ControlRight)
        || keys.pressed(KeyCode::SuperLeft)
        || keys.pressed(KeyCode::SuperRight)
        || keys.just_pressed(KeyCode::ControlLeft)
        || keys.just_pressed(KeyCode::ControlRight)
        || keys.just_pressed(KeyCode::SuperLeft)
        || keys.just_pressed(KeyCode::SuperRight);
    let typing = state.screen == Screen::Library && state.library.focus.is_some();
    for event in text_events.read() {
        state.library.input_event(
            event.key_code,
            event.text.as_deref(),
            event.state.is_pressed(),
            typing,
        );
    }
    if !window.focused {
        if state.transport.playing || !state.held.is_empty() || state.reference_remaining > 0. {
            state.panic();
        }
        state.library.clear_modifiers();
    }
    if !typing && keys.just_pressed(KeyCode::Tab) && !state.buttons.is_empty() {
        let count = state.buttons.len();
        state.focused_button = Some(match (state.focused_button, shift) {
            (None, false) => 0,
            (None, true) => count - 1,
            (Some(i), false) => (i + 1) % count,
            (Some(i), true) => (i + count - 1) % count,
        });
        return;
    }
    if !typing
        && (keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter))
        && let Some((_, action)) = state
            .focused_button
            .and_then(|i| state.buttons.get(i))
            .copied()
    {
        activate(&mut state, action, shift);
        return;
    }
    if state.screen == Screen::Library {
        state.library.tick(time.delta_secs());
        if ctrl && keys.just_pressed(KeyCode::KeyF) {
            state.library.focus(library::Field::Search);
        }
        if ctrl && keys.just_pressed(KeyCode::KeyO) {
            activate(&mut state, Action::Import, false);
        }
        if keys.just_pressed(KeyCode::Tab) && state.library.edit.is_some() {
            let field = if state.library.focus == Some(library::Field::Title) {
                library::Field::Composer
            } else {
                library::Field::Title
            };
            state.library.focus(field);
        }
        if keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::NumpadEnter) {
            if state.library.edit.is_some() {
                activate(&mut state, Action::LibrarySaveMetadata, false);
            } else {
                state.library.focus = None;
                state.library.reset_page();
            }
        }
    }
    if state.screen != Screen::Session {
        if keys.just_pressed(KeyCode::F1) {
            let action = if state.help {
                Action::SettingsBack
            } else {
                Action::Settings
            };
            activate(&mut state, action, false);
        }
        if keys.just_pressed(KeyCode::Escape) {
            if state.screen == Screen::Library && state.library.edit.is_some() {
                activate(&mut state, Action::LibraryCancelEdit, false);
            } else if state.screen == Screen::Library && state.library.focus.is_some() {
                state.library.focus = None;
            } else {
                let action = if state.help {
                    Action::SettingsBack
                } else {
                    Action::Main
                };
                activate(&mut state, action, false);
            }
        }
        if keys.just_pressed(KeyCode::F11) {
            activate(&mut state, Action::Fullscreen, false);
        }
        let pos = window.cursor_position();
        let touch = touches.iter_just_pressed().next();
        let pressed = mouse.just_pressed(MouseButton::Left) || touch.is_some();
        let pos = touch.map(|t| t.position()).or(pos);
        if pressed
            && let Some(pos) = pos
            && let Some((_, action)) = state
                .buttons
                .iter()
                .find(|(r, _)| r.contains(pos.x, pos.y))
                .copied()
        {
            activate(&mut state, action, false);
        }
        return;
    }
    if ctrl {
        for (key, action) in [
            (KeyCode::KeyO, Action::Import),
            (KeyCode::KeyB, Action::Sample),
            (KeyCode::KeyM, Action::Midi),
        ] {
            if keys.just_pressed(key) {
                activate(&mut state, action, shift);
            }
        }
    }
    if !ctrl {
        for (key, action) in [
            (KeyCode::Space, Action::Play),
            (KeyCode::Home, Action::Restart),
            (KeyCode::F11, Action::Fullscreen),
            (KeyCode::KeyV, Action::View),
            (KeyCode::KeyB, Action::Theme),
            (KeyCode::KeyM, Action::Hear),
            (KeyCode::KeyP, Action::Effects),
            (KeyCode::KeyX, Action::Instrument),
            (KeyCode::KeyR, Action::Range),
            (KeyCode::F1, Action::Help),
        ] {
            if keys.just_pressed(key)
                && (!state.help
                    || matches!(action, Action::Help | Action::Fullscreen)
                    || (matches!(action, Action::Play) && !state.settings_open))
            {
                activate(&mut state, action, shift);
            }
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        activate(&mut state, Action::Help, false);
    }
    state.arrow_repeat -= time.delta_secs();
    for (key, action) in [
        (KeyCode::ArrowLeft, Action::Back),
        (KeyCode::ArrowRight, Action::Forward),
        (KeyCode::ArrowDown, Action::Quieter),
        (KeyCode::ArrowUp, Action::Louder),
    ] {
        if !state.help
            && (keys.just_pressed(key) || (keys.pressed(key) && state.arrow_repeat <= 0.))
        {
            activate(&mut state, action, shift);
            state.arrow_repeat = 0.15;
        }
    }
    if state.mode == Mode::Play && !state.help && keys.just_pressed(KeyCode::End) {
        training::end(&mut state, false);
        let end = state.transport.duration_us;
        state.transport.seek(end);
        state.sync();
    }
    let note_keys = [
        KeyCode::KeyA,
        KeyCode::KeyW,
        KeyCode::KeyS,
        KeyCode::KeyE,
        KeyCode::KeyD,
        KeyCode::KeyF,
        KeyCode::KeyT,
        KeyCode::KeyG,
        KeyCode::KeyY,
        KeyCode::KeyH,
        KeyCode::KeyU,
        KeyCode::KeyJ,
        KeyCode::KeyK,
    ];
    for (i, key) in note_keys.into_iter().enumerate() {
        let id = format!("key:{i}");
        if keys.just_released(key) {
            state.source(id.clone(), None, 0);
        }
        if keys.just_pressed(key) && !ctrl && !state.help {
            state.source(id.clone(), Some(60 + i as u8), 95);
        }
        // A rapid tap may press and release between two rendered frames.
        // Judge its attack, then preserve the final released state.
        if keys.just_released(key) && !keys.pressed(key) {
            state.source(id, None, 0);
        }
    }
    let mut pointer = |id: String, pos: Option<Vec2>, down: bool, pressed: bool| {
        if !down {
            if state
                .loop_drag
                .as_ref()
                .is_some_and(|(source, _)| source == &id)
            {
                state.loop_drag = None;
            }
            state.source(id, None, 0);
            return;
        }
        let Some(pos) = pos else {
            return;
        };
        if pressed
            && let Some((_, action)) = state
                .buttons
                .iter()
                .find(|(r, _)| r.contains(pos.x, pos.y))
                .copied()
        {
            activate(&mut state, action, shift);
            return;
        }
        if state.mode == Mode::Play
            && !state.help
            && let Some(range) = state.transport.passage()
        {
            if pressed && (pos.y - (layout.progress.y + 5.)).abs() < 18. {
                let px = |at: u64| {
                    layout.progress.x
                        + layout.progress.w * at as f32 / state.song.duration_us.max(1) as f32
                };
                let is_a = (pos.x - px(range.start_us)).abs() < 14.;
                let is_b = (pos.x - px(range.end_us)).abs() < 14.;
                if is_a || is_b {
                    training::end(&mut state, false);
                    state.panic();
                    state.loop_drag = Some((id.clone(), is_a));
                }
            }
            if let Some((source, is_a)) = state.loop_drag.clone()
                && source == id
            {
                let at = (((pos.x - layout.progress.x) / layout.progress.w).clamp(0., 1.)
                    * state.song.duration_us as f32) as u64;
                let next = if is_a {
                    tangent_core::passage::Passage::new(
                        at.min(range.end_us - 1),
                        range.end_us,
                        state.song.duration_us,
                    )
                } else {
                    tangent_core::passage::Passage::new(
                        range.start_us,
                        at.max(range.start_us + 1),
                        state.song.duration_us,
                    )
                };
                if let Some(p) = next {
                    state.transport.set_passage(Some(p));
                    state.transport.seek(p.start_us);
                    state.loop_a = Some(p.start_us);
                    state.sync();
                }
                return;
            }
        }
        if state.mode == Mode::Play && !state.help && layout.progress.contains(pos.x, pos.y) {
            let at = ((pos.x - layout.progress.x) / layout.progress.w).clamp(0., 1.)
                * state.transport.duration_us as f32;
            training::end(&mut state, false);
            state.song_hits.reset();
            state.feedback.reset();
            state.transport.seek(at as u64);
            state.sync();
            state.source(id, None, 0);
            return;
        }
        let key = if state.help {
            None
        } else {
            layout.hit(&state.piano, pos.x, pos.y)
        };
        state.source(id, key, 95);
    };
    pointer(
        "pointer".into(),
        window.cursor_position(),
        mouse.pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Left),
        mouse.just_pressed(MouseButton::Left),
    );
    for touch in touches.iter() {
        pointer(
            format!("touch:{}", touch.id()),
            Some(touch.position()),
            true,
            touches.just_pressed(touch.id()),
        );
    }
    for touch in touches.iter_just_released() {
        pointer(format!("touch:{}", touch.id()), None, false, false);
    }
    for touch in touches.iter_just_canceled() {
        pointer(format!("touch:{}", touch.id()), None, false, false);
    }
}
