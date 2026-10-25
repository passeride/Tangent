//! Navigation and exercise presentation share the single Bevy drawing surface.
use super::{controls::Action, draw::Drawing, *};

/// The start screen only advertises playable modes and information actually
/// available in State. It never fabricates a previous session or an achievement.
pub(super) fn render(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let compact = l.width < 760.;
    let short = l.height < 620.;
    let pad = if compact { 20. } else { 48. };
    let title_y = if short { 20. } else { 50. };

    // Original Tangent piano-key mark, drawn by Bevy rather than HTML.
    let logo_x = pad;
    for i in 0..7 {
        d.rect(logo_x + i as f32 * 11., title_y, 10., 38., t.white, 2.);
    }
    for i in [0, 1, 3, 4, 5] {
        d.rect(logo_x + i as f32 * 11. + 7., title_y, 6., 23., t.black, 1.);
    }
    d.text(pad + 93., title_y - 2., "TANGENT", 30., t.ink);
    d.text(
        pad + 94.,
        title_y + 31.,
        "Make time for music.",
        13.,
        t.muted,
    );

    if state.screen == Screen::Train {
        let x = pad;
        let width = if compact {
            l.width - 2. * pad
        } else {
            (l.width * 0.54).min(560.)
        };
        d.text(x, title_y + 80., "Practice", 26., t.ink);
        d.text(x, title_y + 114., "Choose what to learn", 14., t.muted);
        let top = title_y + if short { 137. } else { 156. };
        let height = if short { 48. } else { 62. };
        for (i, (label, caption, action)) in [
            (
                "Read notes",
                "Treble, bass and mixed clefs",
                Action::TrainNotes,
            ),
            (
                "Play chords",
                "Recognize and play chord shapes",
                Action::TrainChords,
            ),
            ("Back to main menu", "", Action::Main),
        ]
        .into_iter()
        .enumerate()
        {
            d.menu_card(
                state,
                x,
                top + i as f32 * (height + 12.),
                width,
                height,
                label,
                caption,
                action,
                i == 0,
                t,
            );
        }
        return;
    }

    let x = pad;
    let w = if compact {
        l.width - pad * 2.
    } else {
        (l.width * 0.52).clamp(370., 620.)
    };
    let top = title_y + if short { 70. } else { 104. };
    d.text(
        x,
        top,
        "Your piano workspace",
        if compact { 21. } else { 27. },
        t.ink,
    );
    let cards_top = top + if short { 34. } else { 52. };
    let card_h = if short { 42. } else { 58. };
    let gap = if short { 6. } else { 12. };
    for (i, (label, description, action)) in [
        ("Play", "Open the currently loaded piece", Action::StartPlay),
        (
            "Library",
            "Search, import and organize music",
            Action::Library,
        ),
        ("Train", "Learn notes and chords", Action::Train),
        (
            "FreeForm",
            "Just play; recognize melodies",
            Action::FreeForm,
        ),
        ("Settings", "MIDI, audio and appearance", Action::Settings),
    ]
    .into_iter()
    .enumerate()
    {
        d.menu_card(
            state,
            x,
            cards_top + i as f32 * (card_h + gap),
            w,
            card_h,
            label,
            description,
            action,
            i == 0,
            t,
        );
    }

    // Context region uses existing state only. On narrow windows it becomes
    // compact text below the menu instead of an overlapping second column.
    let context_y = if compact {
        cards_top + 5. * (card_h + gap) + 12.
    } else {
        cards_top
    };
    let context_x = if compact { pad } else { x + w + 36. };
    let context_w = if compact {
        l.width - pad * 2.
    } else {
        (l.width - context_x - pad).max(0.)
    };
    if context_w > 150. && context_y + 100. < l.height {
        if !compact {
            d.rect(
                context_x - 16.,
                context_y - 12.,
                context_w + 16.,
                152.,
                t.panel,
                12.,
            );
        }
        d.text(context_x, context_y, "READY TO PLAY", 11., t.colors[0]);
        let max = (context_w / 8.).max(10.) as usize;
        let title: String = state.song.title.chars().take(max).collect();
        d.text(
            context_x,
            context_y + 28.,
            title,
            if compact { 16. } else { 20. },
            t.ink,
        );
        d.text(
            context_x,
            context_y + 62.,
            if state.midi_output.is_empty() {
                "Output: Computer audio".into()
            } else {
                let port = state
                    .midi_outputs
                    .iter()
                    .find(|p| p.id == state.midi_output);
                format!(
                    "Output: {}",
                    port.map_or("MIDI device", |p| p.name.as_str())
                )
            },
            12.,
            t.muted,
        );
        if !compact {
            d.text(
                context_x,
                context_y + 89.,
                "Connect a MIDI piano or use your keyboard",
                11.,
                t.muted,
            );
        }
    }
}
pub(super) fn exercise(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let x = l.margin + 8.;
    let chords = state.mode == Mode::Chords;
    d.text(
        x,
        18.,
        if chords {
            "Train / chords"
        } else {
            "Train / note reading"
        },
        20.,
        t.ink,
    );
    let message = if state.practice.waiting_release {
        "Correct! Release all keys to continue".to_string()
    } else if state.practice.wrong {
        "Try again - check the clef and octave".to_string()
    } else if chords || state.practice.hint {
        format!("Play {}", state.practice.label())
    } else {
        "Read the note and play it".into()
    };
    d.text(
        x,
        52.,
        message,
        if l.width < 600. { 14. } else { 20. },
        t.ink,
    );
    d.text(
        x,
        80.,
        format!("Completed: {}", state.practice.completed),
        12.,
        t.muted,
    );
    let button_w = ((l.width - l.margin * 2. - 12.) / 3.).min(190.);
    if !chords {
        for (i, (label, action)) in [
            (
                format!("Clef: {}", state.practice.clef.name()),
                Action::PracticeClef,
            ),
            (
                format!("Level: {}", state.practice.level.name()),
                Action::PracticeLevel,
            ),
            (
                if state.practice.hint {
                    "Hide hint"
                } else {
                    "Show hint"
                }
                .into(),
                Action::PracticeHint,
            ),
        ]
        .into_iter()
        .enumerate()
        {
            d.button(
                state,
                l.margin + i as f32 * (button_w + 6.),
                104.,
                button_w,
                label,
                action,
                t,
            );
        }
    }
    // The same bottom-line anchors as the score renderer: E4=30, G2=18.
    // Independent bass placement fixes the old treble-relative positioning.
    let gap = ((l.strike - 165.) / 18.).clamp(4., 14.);
    let top = if chords { 140. } else { 160. };
    let treble = top + 4. * gap;
    let bass = treble + 10. * gap;
    let center = l.width * 0.5;
    let half = (l.width * 0.34).clamp(85., 260.);
    for bottom in [treble, bass] {
        for i in 0..5 {
            d.line(
                center - half,
                bottom - i as f32 * gap,
                half * 2.,
                1.,
                t.staff,
            );
        }
    }
    d.glyph(
        center - half + 3.,
        treble - gap,
        '\u{e050}',
        gap * 4.,
        t.ink,
    );
    d.glyph(
        center - half + 3.,
        bass - gap * 3.,
        '\u{e062}',
        gap * 4.,
        t.ink,
    );
    let is_bass = !chords && state.practice.bass();
    let bottom = if is_bass { bass } else { treble };
    let bottom_step = if is_bass { 18 } else { 30 };
    for key in state.practice.target() {
        let pitch = tangent_core::notation::reading_pitch(key);
        let y = bottom - (pitch.step - bottom_step) as f32 * gap * 0.5;
        for step in tangent_core::notation::ledger_steps(pitch.step, bottom_step) {
            d.line(
                center - gap,
                y + (pitch.step - step) as f32 * gap * 0.5,
                gap * 2.,
                1.,
                t.ink,
            );
        }
        if pitch.sharp {
            d.glyph(center - gap * 2., y, '\u{e262}', gap * 3.5, t.ink);
        }
        d.note(center, y, gap * 0.55, t.ink, false, false);
    }
    d.text(
        x,
        l.strike - 25.,
        "Solid = played   Glow = song / hint",
        11.,
        t.muted,
    );
}
