use super::*;
#[derive(Clone, Copy, Debug)]
pub enum Action {
    Main,
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
    Back,
    Forward,
    Quieter,
    Louder,
    Fullscreen,
    View,
    Theme,
    Hear,
    Effects,
    Instrument,
    Range,
    Import,
    Sample,
    Midi,
    MidiOutput,
    PracticeClef,
    PracticeLevel,
    PracticeHint,
    Help,
    Credits,
    ExportDatabase,
}
impl Action {
    #[cfg(target_arch = "wasm32")]
    pub fn id(self) -> &'static str {
        match self {
            Self::Main => "main",
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
            Self::Instrument => "instrument",
            Self::Range => "range",
            Self::Import => "import",
            Self::Sample => "sample",
            Self::Midi => "midi",
            Self::MidiOutput => "midi-output",
            Self::PracticeClef => "practice-clef",
            Self::PracticeLevel => "practice-level",
            Self::PracticeHint => "practice-hint",
            Self::Help => "help",
            Self::Credits => "credits",
            Self::ExportDatabase => "export-database",
        }
    }
}
fn activate(state: &mut State, action: Action, shift: bool) {
    if state.mode != Mode::Play
        && matches!(action, Action::Restart | Action::Back | Action::Forward)
    {
        return;
    }
    let step = if shift { 15_000_000 } else { 5_000_000 };
    match action {
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
            state.panic();
            state.help = false;
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
            state.panic();
            state.help = false;
            state.screen = if matches!(action, Action::Main) {
                Screen::Main
            } else {
                Screen::Train
            };
        }
        Action::StartPlay | Action::FreeForm | Action::TrainNotes | Action::TrainChords => {
            state.panic();
            state.help = false;
            state.screen = Screen::Session;
            state.mode = match action {
                Action::FreeForm => Mode::FreeForm,
                Action::TrainNotes => Mode::Notes,
                Action::TrainChords => Mode::Chords,
                _ => Mode::Play,
            };
            state.practice = tangent_core::practice::Practice::new(state.mode == Mode::Chords);
            state.particles.clear();
            state.live_bursts.clear();
        }
        Action::Play => {
            if state.screen != Screen::Session {
                return;
            }
            state.help = false;
            if state.mode != Mode::Play {
                return;
            }
            if state.transport.position_us >= state.transport.duration_us {
                state.transport.seek(0);
            }
            state.transport.playing = !state.transport.playing;
            state.clock = platform::clock_us();
            state.sync();
        }
        Action::Restart => {
            state.transport.seek(0);
            state.sync();
        }
        Action::Back => {
            let pos = state.transport.position_us.saturating_sub(step);
            state.transport.seek(pos);
            state.sync();
        }
        Action::Forward => {
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
        Action::Range => {
            state.piano = match state.piano.white_count {
                52 => PianoLayout::new(36, 84),
                29 => PianoLayout::new(36, 96),
                _ => PianoLayout::new(21, 108),
            }
            .unwrap()
        }
        Action::Help => {
            state.help = !state.help;
            if state.help {
                state.panic();
            }
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
            state.panic();
            state.practice.configure(clef, level);
        }
        Action::PracticeHint => state.practice.hint = !state.practice.hint,
        Action::MidiOutput => {
            let next = state
                .midi_outputs
                .iter()
                .position(|p| p.id == state.midi_output)
                .map_or(0, |i| i + 1);
            let id = state
                .midi_outputs
                .get(next)
                .map_or(String::new(), |p| p.id.clone());
            state.panic();
            platform::action("midi-output", &id);
        }
        Action::Midi => {
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
        if state.transport.playing || !state.held.is_empty() {
            state.panic();
        }
        state.library.clear_modifiers();
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
            activate(&mut state, Action::Help, false);
        }
        if keys.just_pressed(KeyCode::Escape) {
            if state.screen == Screen::Library && state.library.edit.is_some() {
                activate(&mut state, Action::LibraryCancelEdit, false);
            } else if state.screen == Screen::Library && state.library.focus.is_some() {
                state.library.focus = None;
            } else {
                let action = if state.help {
                    Action::Help
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
            if keys.just_pressed(key) {
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
        if keys.just_pressed(key) || (keys.pressed(key) && state.arrow_repeat <= 0.) {
            activate(&mut state, action, shift);
            state.arrow_repeat = 0.15;
        }
    }
    if state.mode == Mode::Play && keys.just_pressed(KeyCode::End) {
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
            state.source(id, Some(60 + i as u8), 95);
        }
    }
    let mut pointer = |id: String, pos: Option<Vec2>, down: bool, pressed: bool| {
        if !down {
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
        if state.mode == Mode::Play && !state.help && layout.progress.contains(pos.x, pos.y) {
            let at = ((pos.x - layout.progress.x) / layout.progress.w).clamp(0., 1.)
                * state.transport.duration_us as f32;
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
