//! Bevy capture/dispatch and matrix presentation. Grammar/preferences live in core.
use super::{
    controls::{self, Action},
    draw::Drawing,
    *,
};
use tangent_core::input_bindings::{
    Bindings, CATALOG, KeyboardChord, PianoCommands, pitch, valid_key,
};
#[derive(Clone, Copy)]
pub(super) enum Capture {
    Keyboard(usize),
    Piano(usize),
    Control,
}
#[derive(Default)]
pub(super) struct Shortcuts {
    pub config: Bindings,
    pub open: bool,
    pub piano: PianoCommands,
    pub capture: Option<Capture>,
    pub page: usize,
    pub status: String,
    blocked: BTreeSet<String>,
}
fn modifiers(keys: &ButtonInput<KeyCode>) -> (bool, bool, bool) {
    let held = |key| keys.pressed(key) || keys.just_pressed(key);
    (
        held(KeyCode::ControlLeft)
            || held(KeyCode::ControlRight)
            || held(KeyCode::SuperLeft)
            || held(KeyCode::SuperRight),
        held(KeyCode::ShiftLeft) || held(KeyCode::ShiftRight),
        held(KeyCode::AltLeft) || held(KeyCode::AltRight),
    )
}
fn chord_label(k: &KeyboardChord) -> String {
    format!(
        "{}{}{}{}",
        if k.ctrl { "Ctrl+" } else { "" },
        if k.shift { "Shift+" } else { "" },
        if k.alt { "Alt+" } else { "" },
        k.key.strip_prefix("Key").unwrap_or(&k.key)
    )
}
pub(super) fn set_control(state: &mut State, note: u8) {
    if state
        .shortcuts
        .config
        .rows
        .iter()
        .any(|r| r.piano == Some(note))
    {
        state.shortcuts.status =
            "Control-key conflicts with a piano binding. Clear that cell first.".into();
        return;
    }
    state.shortcuts.config.control = note;
    state.shortcuts.piano.reset();
    state.shortcuts.status = format!("Control-key: {} (MIDI {note})", pitch(note));
    state.shortcuts.capture = None;
}
fn assign(
    state: &mut State,
    index: usize,
    keyboard: Option<KeyboardChord>,
    piano: Option<u8>,
    is_piano: bool,
) {
    let mut next = state.shortcuts.config.clone();
    let row = &mut next.rows[index];
    if is_piano {
        row.piano = piano;
    } else {
        row.keyboard = keyboard;
    }
    if !next.validate() {
        state.shortcuts.status =
            "Already assigned or reserved control-key. Clear the other binding first.".into();
        return;
    }
    state.shortcuts.config = next;
    state.shortcuts.piano.reset();
    state.shortcuts.capture = None;
    state.shortcuts.status = "Binding updated. Settings save automatically.".into();
}
pub(super) fn capture_keyboard(
    state: &mut State,
    events: &[bevy::input::keyboard::KeyboardInput],
    keys: &ButtonInput<KeyCode>,
) -> bool {
    let Some(capture) = state.shortcuts.capture else {
        return false;
    };
    let (ctrl, shift, alt) = modifiers(keys);
    for event in events.iter().filter(|e| e.state.is_pressed()) {
        if event.key_code == KeyCode::Escape {
            state.shortcuts.capture = None;
            state.shortcuts.status = "Binding unchanged.".into();
            return true;
        }
        if event.key_code == KeyCode::Delete {
            match capture {
                Capture::Keyboard(i) => assign(state, i, None, None, false),
                Capture::Piano(i) => assign(state, i, None, None, true),
                Capture::Control => {
                    state.shortcuts.status =
                        "Control-key cannot be cleared; switch piano shortcuts Off.".into();
                }
            }
            return true;
        }
        if let Capture::Keyboard(i) = capture {
            let key = format!("{:?}", event.key_code);
            if valid_key(&key) {
                assign(
                    state,
                    i,
                    Some(KeyboardChord {
                        key,
                        ctrl,
                        shift,
                        alt,
                    }),
                    None,
                    false,
                );
                return true;
            }
        }
    }
    true
}
fn execute(state: &mut State, command: &str, piano: bool) {
    if command.starts_with("nav_") {
        match command {
            "nav_next" | "nav_previous" if !state.buttons.is_empty() => {
                let count = state.buttons.len();
                state.focused_button =
                    Some(match (state.focused_button, command == "nav_previous") {
                        (None, false) => 0,
                        (None, true) => count - 1,
                        (Some(i), false) => (i + 1) % count,
                        (Some(i), true) => (i + count - 1) % count,
                    });
            }
            "nav_activate" | "nav_activate_alt" => {
                if let Some((_, action)) = state
                    .focused_button
                    .and_then(|i| state.buttons.get(i))
                    .copied()
                {
                    controls::activate(state, action, false);
                }
            }
            "nav_back" => {
                if state.screen == Screen::Library && state.library.edit.is_some() {
                    controls::activate(state, Action::LibraryCancelEdit, false);
                } else if state.screen == Screen::Library && state.library.focus.is_some() {
                    state.library.focus = None;
                } else {
                    let action = if state.screen == Screen::Session {
                        if state.settings_open {
                            Action::SettingsBack
                        } else {
                            Action::Help
                        }
                    } else if state.help {
                        Action::SettingsBack
                    } else {
                        Action::Main
                    };
                    controls::activate(state, action, false);
                }
            }
            _ => {}
        }
        return;
    }
    if let Some(key) = command
        .strip_prefix("note_")
        .and_then(|n| n.parse::<u8>().ok())
    {
        if !piano {
            state.source(format!("binding:{command}"), Some(key), 95);
        }
        // Piano column replays the mapped pitch through ordinary live input; the
        // command attack is still excluded from scoring and fingerprint input.
        else {
            state.shortcuts.status =
                "Live-note commands are keyboard-only; play piano notes directly.".into();
        }
        return;
    }
    if command == "search" {
        if state.screen == Screen::Library {
            state.library.focus(library::Field::Search);
        }
        return;
    }
    let global = matches!(
        command,
        "fullscreen" | "pause" | "import" | "midi" | "sample"
    );
    let paused_action = state.screen == Screen::Session
        && state.help
        && !state.settings_open
        && matches!(
            command,
            "play"
                | "slower"
                | "faster"
                | "back"
                | "forward"
                | "back_long"
                | "forward_long"
                | "restart"
                | "speed_reset"
                | "loop_a"
                | "loop_b"
                | "loop_clear"
                | "loop_snap"
                | "part"
                | "fit"
        );
    if !global && !paused_action && (state.screen != Screen::Session || state.help) {
        return;
    }
    if matches!(command, "hint" | "listen" | "skip")
        && !matches!(state.mode, Mode::Notes | Mode::Chords)
    {
        return;
    }
    let action = match command {
        "play" => Action::Play,
        "slower" => Action::Slower,
        "faster" => Action::Faster,
        "back" | "back_long" => Action::Back,
        "forward" | "forward_long" => Action::Forward,
        "restart" => Action::Restart,
        "fullscreen" => Action::Fullscreen,
        "view" => Action::View,
        "theme" => Action::Theme,
        "hear" => Action::Hear,
        "effects" => Action::Effects,
        "instrument" => Action::Instrument,
        "range" => Action::Range,
        "pause" => {
            if state.screen == Screen::Session {
                Action::Help
            } else if state.help {
                Action::SettingsBack
            } else {
                Action::Settings
            }
        }
        "import" => Action::Import,
        "sample" => Action::Sample,
        "midi" => Action::Midi,
        "quieter" => Action::Quieter,
        "louder" => Action::Louder,
        "speed_reset" => Action::NormalSpeed,
        "loop_a" => Action::LoopA,
        "loop_b" => Action::LoopB,
        "loop_clear" => Action::LoopClear,
        "loop_snap" => Action::LoopSnap,
        "part" => Action::Part,
        "fit" => Action::FitRange,
        "hint" => Action::PracticeHint,
        "listen" => Action::PracticeListen,
        "skip" => Action::PracticeSkip,
        "end" => {
            if state.mode == Mode::Play {
                training::end(state, false);
                let end = state.transport.duration_us;
                state.transport.seek(end);
                state.sync();
            }
            return;
        }
        _ => return,
    };
    controls::activate(
        state,
        action,
        matches!(command, "back_long" | "forward_long"),
    );
}
pub(super) fn keyboard(state: &mut State, keys: &ButtonInput<KeyCode>, typing: bool) -> bool {
    if keys.just_pressed(KeyCode::Escape) {
        execute(state, "nav_back", false);
        return false;
    }
    let modifiers = modifiers(keys);
    let pressed: BTreeSet<_> = keys.get_just_pressed().map(|k| format!("{k:?}")).collect();
    let released: BTreeSet<_> = keys.get_just_released().map(|k| format!("{k:?}")).collect();
    let events: Vec<_> = state
        .shortcuts
        .config
        .rows
        .iter()
        .filter_map(|row| {
            let k = row.keyboard.as_ref()?;
            let down = (!typing || matches!(row.action.as_str(), "import" | "search"))
                && pressed.contains(&k.key)
                && modifiers == (k.ctrl, k.shift, k.alt);
            let up = row.action.starts_with("note_") && released.contains(&k.key);
            (down || up).then(|| (row.action.clone(), down, up))
        })
        .collect();
    let mut navigation = false;
    for (action, down, up) in events {
        if up {
            state.source(format!("binding:{action}"), None, 0);
        }
        if down {
            execute(state, &action, false);
            navigation |= action.starts_with("nav_") && action != "nav_back";
            if up {
                state.source(format!("binding:{action}"), None, 0);
            }
        }
    }
    navigation
}
pub(super) fn midi(state: &mut State, id: &str, key: u8, down: bool, now: u64) -> bool {
    if down && state.shortcuts.blocked.contains(id) {
        return true;
    }
    if !down && state.shortcuts.blocked.remove(id) {
        return true;
    }
    if let Some(capture) = state.shortcuts.capture {
        if down {
            state.shortcuts.blocked.insert(id.into());
            match capture {
                Capture::Control => set_control(state, key),
                Capture::Piano(i) => {
                    if state.shortcuts.config.rows[i].action.starts_with("note_") {
                        state.shortcuts.status =
                            "Live notes use ordinary piano input; no modifier mapping needed."
                                .into();
                    } else {
                        assign(state, i, None, Some(key), true);
                    }
                }
                Capture::Keyboard(_) => {}
            }
        }
        return true;
    }
    let cfg = &state.shortcuts.config;
    if let Some(command) = state.shortcuts.piano.event(cfg, id, key, down, now) {
        if !command.is_empty() {
            state.shortcuts.status = format!("Piano: {command}");
            execute(state, &command, true);
        }
        return true;
    }
    false
}
pub(super) fn render(
    d: &mut Drawing,
    state: &mut State,
    l: &Layout,
    t: theme::Theme,
    x: f32,
    y: f32,
    w: f32,
) {
    d.text(x, y, "Keybindings", 21., t.ink);
    let gap = 6.;
    let third = (w - 2. * gap) / 3.;
    for (i, (label, action)) in [
        (
            format!(
                "Piano: {}",
                if state.shortcuts.config.piano_enabled {
                    "On"
                } else {
                    "Off"
                }
            ),
            Action::BindingToggle,
        ),
        (
            format!("Ctrl: {} / learn", pitch(state.shortcuts.config.control)),
            Action::BindingControl(2),
        ),
        ("Reset bindings".into(), Action::BindingReset),
    ]
    .into_iter()
    .enumerate()
    {
        d.button(
            state,
            x + i as f32 * (third + gap),
            y + 32.,
            third,
            label,
            action,
            t,
        );
    }
    if l.height >= 500. {
        let ctrl = format!(
            "Hold Ctrl + note, or Ctrl then note (2s). MIDI {}",
            state.shortcuts.config.control
        );
        d.text(
            x,
            y + 72.,
            ctrl.chars().take((w / 6.) as usize).collect::<String>(),
            10.,
            t.muted,
        );
        let half = (w - gap) / 2.;
        d.button(
            state,
            x,
            y + 91.,
            half,
            "Use lowest visible key",
            Action::BindingControl(0),
            t,
        );
        d.button(
            state,
            x + half + gap,
            y + 91.,
            half,
            "Use highest visible key",
            Action::BindingControl(1),
            t,
        );
    }
    let top = y + if l.height < 500. { 78. } else { 139. };
    let label_w = w * 0.40;
    let cell_w = (w - label_w - 12.) / 2.;
    d.text(x, top, "Action", 11., t.muted);
    d.text(x + label_w, top, "Keyboard", 11., t.muted);
    d.text(
        x + label_w + cell_w + 6.,
        top,
        "Piano (Ctrl first)",
        11.,
        t.muted,
    );
    let count = ((l.height - top - 130.) / 38.).floor().max(1.) as usize;
    let pages = CATALOG.len().div_ceil(count);
    state.shortcuts.page = state.shortcuts.page.min(pages - 1);
    let start = state.shortcuts.page * count;
    for (line, index) in (start..(start + count).min(CATALOG.len())).enumerate() {
        let id = CATALOG[index].0;
        let row = state
            .shortcuts
            .config
            .rows
            .iter()
            .position(|r| r.action == id)
            .unwrap();
        let binding = &state.shortcuts.config.rows[row];
        let key_label = binding
            .keyboard
            .as_ref()
            .map_or("Unbound".into(), chord_label);
        let piano_label = binding
            .piano
            .map_or("Unbound".into(), |n| format!("{} ({n})", pitch(n)));
        let label = if let Some(n) = id.strip_prefix("note_").and_then(|n| n.parse::<u8>().ok()) {
            format!("Play {}", pitch(n))
        } else {
            CATALOG[index].1.into()
        };
        let py = top + 23. + line as f32 * 38.;
        d.text(
            x,
            py + 8.,
            label
                .chars()
                .take((label_w / 6.) as usize)
                .collect::<String>(),
            10.,
            t.ink,
        );
        d.button(
            state,
            x + label_w,
            py,
            cell_w,
            key_label,
            Action::BindingCapture(row, false),
            t,
        );
        if !id.starts_with("note_") {
            d.button(
                state,
                x + label_w + cell_w + 6.,
                py,
                cell_w,
                piano_label,
                Action::BindingCapture(row, true),
                t,
            );
        } else {
            d.text(
                x + label_w + cell_w + 6.,
                py + 8.,
                "Direct input",
                10.,
                t.muted,
            );
        }
    }
    let footer = top + 23. + count as f32 * 38.;
    d.button(
        state,
        x,
        footer,
        70.,
        "Previous",
        Action::BindingPage(false),
        t,
    );
    d.text(
        x + 80.,
        footer + 8.,
        format!("{}/{}", state.shortcuts.page + 1, pages),
        11.,
        t.ink,
    );
    d.button(
        state,
        x + w - 70.,
        footer,
        70.,
        "Next",
        Action::BindingPage(true),
        t,
    );
    let status = if state.shortcuts.capture.is_some() {
        format!("Listening. {}", state.shortcuts.status)
    } else if state.shortcuts.status.is_empty() {
        "Click a cell to bind. Esc cancels; Delete clears. Tab/Enter/Esc stay available.".into()
    } else {
        state.shortcuts.status.clone()
    };
    d.text(
        x,
        footer + 42.,
        status.chars().take((w / 6.) as usize).collect::<String>(),
        10.,
        t.colors[0],
    );
    d.text(
        x,
        footer + 60.,
        state
            .storage_status
            .chars()
            .take((w / 6.) as usize)
            .collect::<String>(),
        10.,
        t.muted,
    );
}

pub(super) fn hint(state: &State, id: &str) -> String {
    let Some(row) = state.shortcuts.config.rows.iter().find(|r| r.action == id) else {
        return String::new();
    };
    let mut labels = Vec::new();
    if let Some(key) = &row.keyboard {
        labels.push(chord_label(key));
    }
    if let Some(note) = row.piano {
        labels.push(format!(
            "Piano Ctrl+{}{}",
            pitch(note),
            if state.shortcuts.config.piano_enabled {
                ""
            } else {
                " (Off)"
            }
        ));
    }
    labels.join(" / ")
}
pub(super) fn action_hint(state: &State, action: Action) -> String {
    let id = match action {
        Action::Play => "play",
        Action::Slower => "slower",
        Action::Faster => "faster",
        Action::Back => "back",
        Action::Forward => "forward",
        Action::Restart => "restart",
        Action::Quieter => "quieter",
        Action::Louder => "louder",
        Action::Fullscreen => "fullscreen",
        Action::View => "view",
        Action::Theme => "theme",
        Action::Hear => "hear",
        Action::Effects => "effects",
        Action::Instrument => "instrument",
        Action::Range => "range",
        Action::FitRange => "fit",
        Action::Import => "import",
        Action::Sample => "sample",
        Action::Midi => "midi",
        Action::PracticeHint => "hint",
        Action::PracticeListen => "listen",
        Action::PracticeSkip => "skip",
        Action::Part => "part",
        Action::LoopA => "loop_a",
        Action::LoopB => "loop_b",
        Action::LoopClear => "loop_clear",
        Action::LoopSnap => "loop_snap",
        _ => return String::new(),
    };
    hint(state, id)
}
