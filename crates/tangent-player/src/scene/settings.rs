//! The paused session and settings pages are presentation-only; transport,
//! MIDI routing and saved preferences stay owned by the existing scene/actions.
use super::{controls::Action, draw::Drawing, *};

pub(super) fn pause(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let expanded = l.height >= 570.;
    let w = (l.width - 28.).min(570.);
    let h = (l.height - 24.).min(if state.mode == Mode::Play {
        if expanded {
            532.
        } else if l.height >= 420. {
            454.
        } else {
            412.
        }
    } else {
        370.
    });
    let x = (l.width - w) * 0.5;
    let y = (l.height - h) * 0.5;
    d.rect(x, y, w, h, t.panel, 14.);
    let inner = x + 18.;
    let usable = w - 36.;
    d.text(inner, y + 15., "Paused", 25., t.ink);
    if state.mode == Mode::Play {
        let title: String = state
            .song
            .title
            .chars()
            .take((usable / 11.).max(9.) as usize)
            .collect();
        d.text(
            inner,
            y + 50.,
            format!(
                "{} / {} / {}",
                title,
                super::draw::time(state.transport.position_us),
                super::draw::time(state.transport.duration_us)
            ),
            12.,
            t.muted,
        );
    } else {
        d.text(inner, y + 50., "Your session is paused.", 12., t.muted);
    }

    let resume_hint = super::shortcuts::hint(state, "play");
    let resume_label = if resume_hint.is_empty() {
        "Resume".into()
    } else {
        format!("Resume  [{resume_hint}]")
    };
    // Resume is the only action that can restart the transport.
    d.menu_card(
        state,
        inner,
        y + 76.,
        usable,
        45.,
        &resume_label,
        "",
        Action::Play,
        true,
        t,
    );

    let gap = 8.;
    let third = (usable - 2. * gap) / 3.;
    if state.mode == Mode::Play {
        for (i, (label, action)) in [
            ("Restart", Action::Restart),
            ("-5 seconds", Action::Back),
            ("+5 seconds", Action::Forward),
        ]
        .into_iter()
        .enumerate()
        {
            d.button(
                state,
                inner + i as f32 * (third + gap),
                y + 135.,
                third,
                label,
                action,
                t,
            );
        }
    } else {
        d.text(
            inner,
            y + 141.,
            "Play freely; transport seeking is unavailable.",
            11.,
            t.muted,
        );
    }

    let offset = if state.mode == Mode::Play {
        if expanded {
            162.
        } else if l.height >= 420. {
            84.
        } else {
            42.
        }
    } else {
        0.
    };
    if state.mode == Mode::Play {
        let label = format!("{}% / reset", state.transport.rate().percent());
        for (i, (label, action)) in [
            ("Slower".to_string(), Action::Slower),
            (label, Action::NormalSpeed),
            ("Faster".to_string(), Action::Faster),
        ]
        .into_iter()
        .enumerate()
        {
            d.button(
                state,
                inner + i as f32 * (third + gap),
                y + 178.,
                third,
                label,
                action,
                t,
            );
        }
    }
    if state.mode == Mode::Play && expanded {
        for (i, (label, action)) in [
            ("Set A", Action::LoopA),
            ("Set B", Action::LoopB),
            ("Clear loop", Action::LoopClear),
        ]
        .into_iter()
        .enumerate()
        {
            d.button(
                state,
                inner + i as f32 * (third + gap),
                y + 220.,
                third,
                label,
                action,
                t,
            );
        }
        d.text(inner, y + 260., passage::label(state), 11., t.colors[0]);
        let part = state
            .selected_part
            .and_then(|i| state.song.parts.get(i))
            .map_or("All parts", |p| p.name.as_str());
        let cols = if state.song.notation.is_some() {
            3.
        } else {
            2.
        };
        let width = (usable - (cols - 1.) * gap) / cols;
        let label = format!(
            "Targets: {}",
            part.chars()
                .take((((width - 18.) / (9. * 0.62)) as usize).saturating_sub(9))
                .collect::<String>()
        );
        d.button(state, inner, y + 284., width, label, Action::Part, t);
        d.button(
            state,
            inner + width + gap,
            y + 284.,
            width,
            passage::goal_label(state),
            Action::RepetitionGoal,
            t,
        );
        if cols == 3. {
            d.button(
                state,
                inner + 2. * (width + gap),
                y + 284.,
                width,
                "Snap bars",
                Action::LoopSnap,
                t,
            );
        }
    }
    if state.mode == Mode::Play && !expanded && l.height >= 420. {
        d.button(
            state,
            inner,
            y + 220.,
            usable,
            "Passage practice setup",
            Action::PassageSetup,
            t,
        );
    }
    let half = (usable - gap) * 0.5;
    let view_label = format!("View: {}", state.view.name());
    let sound_label = format!("Song audio: {}", if state.hear { "On" } else { "Off" });
    if state.training.model.run.is_some() {
        d.button(
            state,
            inner,
            y + 178. + offset,
            half,
            "Finish session",
            Action::PracticeFinish,
            t,
        );
    } else {
        d.button(
            state,
            inner,
            y + 178. + offset,
            half,
            view_label,
            Action::View,
            t,
        );
    }
    d.button(
        state,
        inner + half + gap,
        y + 178. + offset,
        half,
        sound_label,
        Action::Hear,
        t,
    );
    d.button(
        state,
        inner,
        y + 220. + offset,
        half,
        "Settings",
        Action::Settings,
        t,
    );
    d.button(
        state,
        inner + half + gap,
        y + 220. + offset,
        half,
        "Open library",
        Action::Library,
        t,
    );
    d.button(
        state,
        inner,
        y + 262. + offset,
        half,
        "Main menu",
        Action::Main,
        t,
    );
    d.button(
        state,
        inner + half + gap,
        y + 262. + offset,
        half,
        "Close  [Esc]",
        Action::Help,
        t,
    );

    if h > 330. + offset {
        d.text(
            inner,
            y + 320. + offset,
            if state.mode == Mode::Play {
                state
                    .status
                    .chars()
                    .take((usable / 6.) as usize)
                    .collect::<String>()
            } else {
                "Use Resume / Esc closes while staying paused".into()
            },
            11.,
            t.muted,
        );
    }
}

pub(super) fn render(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let compact = l.width < 740. || l.height < 550.;
    let pad = if compact { 12. } else { 28. };
    let full_w = (l.width - 2. * pad).max(0.);
    d.text(pad, 18., "Settings", 25., t.ink);
    let back_label = if state.screen == Screen::Session {
        "Back to pause"
    } else {
        "Close"
    };
    d.button(
        state,
        l.width - pad - 106.,
        14.,
        106.,
        back_label,
        Action::SettingsBack,
        t,
    );

    let content_x;
    let content_y;
    let content_w;
    if compact {
        let cols = 3;
        let gap = 6.;
        let tab_w = (full_w - (cols - 1) as f32 * gap) / cols as f32;
        for (i, tab) in SettingsTab::ALL.into_iter().enumerate() {
            let x = pad + (i % cols) as f32 * (tab_w + gap);
            let y = 58. + (i / cols) as f32 * 39.;
            d.button(
                state,
                x,
                y,
                tab_w,
                if tab == SettingsTab::AudioMidi {
                    "Audio/MIDI"
                } else {
                    tab.title()
                },
                Action::SettingsTab(tab),
                t,
            );
            if tab == state.settings_tab {
                d.rect(x, y + 30., tab_w, 2., t.colors[0], 0.);
            }
        }
        content_x = pad;
        content_y = 155.;
        content_w = full_w;
    } else {
        let nav_w = 212.;
        d.rect(pad, 64., nav_w, (l.height - 90.).max(0.), t.panel, 10.);
        for (i, tab) in SettingsTab::ALL.into_iter().enumerate() {
            let x = pad + 10.;
            let y = 78. + i as f32 * 44.;
            d.button(
                state,
                x,
                y,
                nav_w - 20.,
                tab.title(),
                Action::SettingsTab(tab),
                t,
            );
            if tab == state.settings_tab {
                d.rect(x, y + 29., nav_w - 20., 2., t.colors[0], 0.);
            }
        }
        content_x = pad + nav_w + 30.;
        content_y = 84.;
        content_w = (l.width - content_x - pad).max(0.);
    }

    if state.personal.open != 0 {
        super::personal::render(d, state, l, t, content_x, content_y, content_w);
        return;
    }
    if state.settings_tab == SettingsTab::Accessibility && state.shortcuts.open {
        super::shortcuts::render(d, state, l, t, content_x, content_y, content_w);
        return;
    }
    d.text(content_x, content_y, state.settings_tab.title(), 21., t.ink);
    let mut entries: Vec<(String, Action)> = vec![];
    let explanation = match state.settings_tab {
        SettingsTab::AudioMidi => {
            entries.extend([
                ("Rescan / connect MIDI".into(), Action::Midi),
                (
                    format!("Volume - ({}%)", (state.volume * 100.).round()),
                    Action::Quieter,
                ),
                ("Volume +".into(), Action::Louder),
                (
                    format!(
                        "Piano sound: {}",
                        ["Soft felt", "Bright keys"][state.instrument]
                    ),
                    Action::Instrument,
                ),
                (
                    format!("Song audio: {}", if state.hear { "On" } else { "Off" }),
                    Action::Hear,
                ),
            ]);
            "External MIDI output replaces computer audio; your piano controls its volume."
        }
        SettingsTab::Visuals => {
            entries.push(("Edit theme / Import / Export".into(), Action::Personal(0)));
            entries.push((
                "Hands: teach / review / colours".into(),
                Action::Personal(1),
            ));
            entries.extend([
                (format!("Notation: {}", state.view.name()), Action::View),
                (
                    format!("Theme: {}", ["Aurora", "Ember", "Paper"][state.theme]),
                    Action::Theme,
                ),
                (
                    format!("Visible keys: {}", state.piano.keys.len()),
                    Action::Range,
                ),
                ("Fit keyboard to selected music".into(), Action::FitRange),
                (
                    format!(
                        "HUD: {}",
                        ["Focus", "Standard", "Arcade"][state.hud as usize]
                    ),
                    Action::Hud,
                ),
                ("Toggle fullscreen".into(), Action::Fullscreen),
            ]);
            "Changes are saved locally. Sheet notation is most accurate with MusicXML."
        }
        SettingsTab::Practice
            if state.screen == Screen::Session
                && state.mode == Mode::Play
                && !state.practice_defaults =>
        {
            entries.extend([
                ("Set A here".into(), Action::LoopA),
                ("Set B here".into(), Action::LoopB),
                ("Clear loop".into(), Action::LoopClear),
                ("Seek -5 seconds".into(), Action::Back),
                ("Seek +5 seconds".into(), Action::Forward),
                (
                    format!(
                        "Targets: {}",
                        state
                            .selected_part
                            .and_then(|i| state.song.parts.get(i))
                            .map_or("All parts", |p| p.name.as_str())
                    ),
                    Action::Part,
                ),
            ]);
            entries.push((passage::goal_label(state), Action::RepetitionGoal));
            if state.song.notation.is_some() {
                entries.push(("Snap to measure visits".into(), Action::LoopSnap));
            }
            entries.push((
                "Note/chord exercise defaults".into(),
                Action::PracticeDefaults,
            ));
            "Goal: 3 complete repeats at 80% pitch coverage. Backing audio is allowed."
        }
        SettingsTab::Practice => {
            entries.extend([
                (
                    format!("Clef: {}", state.practice.clef.name()),
                    Action::PracticeClef,
                ),
                (
                    format!("Difficulty: {}", state.practice.level.name()),
                    Action::PracticeLevel,
                ),
                (
                    if state.practice.hint {
                        "Hide hints"
                    } else {
                        "Show hints"
                    }
                    .into(),
                    Action::PracticeHint,
                ),
            ]);
            if state.mode == Mode::Explorer {
                entries.retain(|(_, action)| !matches!(action, Action::PracticeClef));
            }
            if state.screen == Screen::Session && state.mode == Mode::Play {
                entries.push((
                    "Back to song passage setup".into(),
                    Action::PracticeDefaults,
                ));
            }
            "Hints reveal answers; note and chord exercises stay untimed."
        }
        SettingsTab::Effects => {
            entries.push((
                format!("Sparkles: {}", if state.effects { "On" } else { "Off" }),
                Action::Effects,
            ));
            entries.push((
                format!(
                    "Intensity: {}",
                    ["Soft", "Full", "Punchy"][state.effect_strength as usize]
                ),
                Action::EffectStrength,
            ));
            "Cosmetic hit feedback; reduced motion disables bursts. Key glow stays visible."
        }
        SettingsTab::Accessibility => {
            entries.push(("Keybindings: keyboard & piano".into(), Action::Keybindings));
            entries.push((
                format!(
                    "Reduced motion: {}",
                    if state.reduced_motion { "On" } else { "Off" }
                ),
                Action::ReducedMotion,
            ));
            "Reduced motion removes bursts and expanding rings. Note and key feedback stays visible."
        }
        SettingsTab::Data => {
            entries.extend([
                ("Export SQLite database".into(), Action::ExportDatabase),
                ("Music credits".into(), Action::Credits),
                ("Import music".into(), Action::Import),
                ("Load sample".into(), Action::Sample),
            ]);
            "Your library and preferences are stored locally. Database restore is not yet available."
        }
    };
    let desc_y = content_y + 34.;
    let max_text = ((content_w / 7.0).max(10.)) as usize;
    let text: String = explanation.chars().take(max_text).collect();
    d.text(content_x, desc_y, text, 11., t.muted);

    let cols = if l.height < 420. && entries.len() > 6 {
        3
    } else if content_w >= 500. || l.height < 500. {
        2
    } else {
        1
    };
    let gap = 10.;
    let btn_w = (content_w - (cols - 1) as f32 * gap) / cols as f32;
    let mut start = content_y + 72.;
    if state.settings_tab == SettingsTab::AudioMidi {
        let selected_name = state
            .midi_outputs
            .iter()
            .find(|p| p.id == state.midi_output)
            .map(|p| p.name.as_str());
        let label = if let Some(name) = selected_name {
            format!("MIDI output: {name}  v")
        } else if let Some(name) = &state.preferred_midi_output {
            format!("Computer audio  (saved: {name} unavailable)  v")
        } else {
            "MIDI output: Computer audio  v".to_string()
        };
        let max_chars = ((content_w - 24.) / 7.).max(12.) as usize;
        let label: String = label.chars().take(max_chars).collect();
        d.button(
            state,
            content_x,
            start,
            content_w,
            label,
            Action::MidiOutput,
            t,
        );
        start += 46.;
        if state.midi_dropdown_open {
            // The dropdown lives entirely inside Bevy and replaces other audio
            // controls while open; it never leaks over/behind adjacent widgets.
            let panel_h = 46. + 5. * 36. + 42.;
            d.rect(content_x, start - 6., content_w, panel_h, t.panel, 10.);
            d.button(
                state,
                content_x + 8.,
                start,
                content_w - 16.,
                if state.midi_output.is_empty() {
                    "Selected: Computer audio"
                } else {
                    "Computer audio"
                },
                Action::MidiOutputSelect(None),
                t,
            );
            let page = state.midi_dropdown_page;
            // Snapshot labels first: drawing buttons needs a mutable State.
            let ports: Vec<(usize, String)> = state
                .midi_outputs
                .iter()
                .enumerate()
                .skip(page * 5)
                .take(5)
                .map(|(index, port)| {
                    let max_chars = ((content_w - 110.) / 7.).max(10.) as usize;
                    let name: String = port.name.chars().take(max_chars).collect();
                    let marker = if port.id == state.midi_output {
                        "[selected] "
                    } else if state.preferred_midi_output.as_deref() == Some(port.name.as_str()) {
                        "[saved] "
                    } else {
                        ""
                    };
                    (index, format!("{marker}{name}"))
                })
                .collect();
            for (offset, label) in ports {
                d.button(
                    state,
                    content_x + 8.,
                    start + 36. * (offset - page * 5 + 1) as f32,
                    content_w - 16.,
                    label,
                    Action::MidiOutputSelect(Some(offset)),
                    t,
                );
            }
            if state.midi_outputs.is_empty() {
                d.text(
                    content_x + 12.,
                    start + 43.,
                    "No MIDI outputs found. Select Rescan after closing.",
                    11.,
                    t.muted,
                );
            }
            if state.midi_outputs.len() > 5 {
                let nav_y = start + 36. * 6.;
                let half = (content_w - 26.) / 2.;
                if page > 0 {
                    d.button(
                        state,
                        content_x + 8.,
                        nav_y,
                        half,
                        "Previous",
                        Action::MidiOutputPage(false),
                        t,
                    );
                }
                if (page + 1) * 5 < state.midi_outputs.len() {
                    d.button(
                        state,
                        content_x + content_w - half - 8.,
                        nav_y,
                        half,
                        "Next",
                        Action::MidiOutputPage(true),
                        t,
                    );
                }
            }
            return;
        }
    }
    let step = if l.height < 430. { 36. } else { 44. };
    for (i, (label, action)) in entries.into_iter().enumerate() {
        let x = content_x + (i % cols) as f32 * (btn_w + gap);
        let y = start + (i / cols) as f32 * step;
        if y + 30. < l.height - 16. {
            let hint = super::shortcuts::action_hint(state, action);
            let label = if hint.is_empty() {
                label
            } else {
                format!("{label} [{hint}]")
            };
            d.button(state, x, y, btn_w, label, action, t);
        }
    }

    // Storage and platform status are informational, not clickable actions.
    if !compact && l.height > 550. {
        d.text(content_x, l.height - 63., &state.status, 11., t.muted);
        d.text(
            content_x,
            l.height - 39.,
            &state.storage_status,
            11.,
            t.muted,
        );
    }
}
