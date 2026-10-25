//! The paused session and settings pages are presentation-only; transport,
//! MIDI routing and saved preferences stay owned by the existing scene/actions.
use super::{controls::Action, draw::Drawing, *};

pub(super) fn pause(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let w = (l.width - 28.).min(570.);
    let h = (l.height - 24.).min(370.);
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
                "{}  ·  {} / {}",
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

    // Resume is the only action that can restart the transport.
    d.menu_card(
        state,
        inner,
        y + 76.,
        usable,
        45.,
        "Resume  [Space]",
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

    let half = (usable - gap) * 0.5;
    let view_label = format!("View: {}", state.view.name());
    let sound_label = format!("Song audio: {}", if state.hear { "On" } else { "Off" });
    d.button(state, inner, y + 178., half, view_label, Action::View, t);
    d.button(
        state,
        inner + half + gap,
        y + 178.,
        half,
        sound_label,
        Action::Hear,
        t,
    );
    d.button(
        state,
        inner,
        y + 220.,
        half,
        "Settings",
        Action::Settings,
        t,
    );
    d.button(
        state,
        inner + half + gap,
        y + 220.,
        half,
        "Open library",
        Action::Library,
        t,
    );
    d.button(state, inner, y + 262., half, "Main menu", Action::Main, t);
    d.button(
        state,
        inner + half + gap,
        y + 262.,
        half,
        "Close  [Esc]",
        Action::Help,
        t,
    );

    if h > 330. {
        d.text(
            inner,
            y + 320.,
            "Space resumes  ·  Esc closes while staying paused",
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

    d.text(content_x, content_y, state.settings_tab.title(), 21., t.ink);
    let mut entries: Vec<(String, Action)> = vec![];
    let explanation = match state.settings_tab {
        SettingsTab::AudioMidi => {
            let output = state
                .midi_outputs
                .iter()
                .find(|p| p.id == state.midi_output)
                .map_or("Computer audio", |p| p.name.as_str());
            entries.extend([
                ("Rescan / connect MIDI".into(), Action::Midi),
                (format!("Output: {output}"), Action::MidiOutput),
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
                ("Toggle fullscreen".into(), Action::Fullscreen),
            ]);
            "Changes are saved locally. Sheet notation is most accurate with MusicXML."
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
            "Hints reveal answers; note and chord exercises stay untimed."
        }
        SettingsTab::Effects => {
            entries.push((
                format!("Sparkles: {}", if state.effects { "On" } else { "Off" }),
                Action::Effects,
            ));
            "Effects are cosmetic. Expected-key glow works with sparkles disabled."
        }
        SettingsTab::Accessibility => {
            "Reduced motion, UI scaling and high-contrast controls are planned, not yet available."
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

    let cols = if content_w >= 500. { 2 } else { 1 };
    let gap = 10.;
    let btn_w = (content_w - (cols - 1) as f32 * gap) / cols as f32;
    let start = content_y + 72.;
    let step = if l.height < 430. { 36. } else { 44. };
    for (i, (label, action)) in entries.into_iter().enumerate() {
        let x = content_x + (i % cols) as f32 * (btn_w + gap);
        let y = start + (i / cols) as f32 * step;
        if y + 30. < l.height - 16. {
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
