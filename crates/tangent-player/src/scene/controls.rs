use super::*;
#[derive(Clone, Copy, Debug)]
pub enum Action {
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
    Help,
    Credits,
}
impl Action {
    pub fn id(self) -> &'static str {
        match self {
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
            Self::Help => "help",
            Self::Credits => "credits",
        }
    }
}
fn activate(state: &mut State, action: Action, shift: bool) {
    let step = if shift { 15_000_000 } else { 5_000_000 };
    match action {
        Action::Play => {
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
        Action::Help => state.help = !state.help,
        Action::Import => platform::action("import", ""),
        Action::Sample => platform::action("sample", ""),
        Action::Midi => platform::action("midi", ""),
        Action::Credits => platform::action("credits", ""),
    }
}
pub fn input(
    mut state: ResMut<State>,
    windows: Query<&Window>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
    time: Res<Time>,
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
        state.help = false;
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
    if keys.just_pressed(KeyCode::End) {
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
        if !state.help && layout.progress.contains(pos.x, pos.y) {
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
