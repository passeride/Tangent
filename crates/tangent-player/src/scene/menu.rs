//! Navigation and exercise presentation share the single Bevy drawing surface.
use super::{controls::Action, draw::Drawing, *};

/// The start screen only advertises playable modes and information actually
/// available in State. It never fabricates a previous session or an achievement.
pub(super) fn render(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let compact = l.width < 1000.;
    let short = l.height < 620.;
    let pad = if compact { 20. } else { 48. };
    let title_y = if short { 20. } else { 50. };

    // Original full-note-and-tangent mark, drawn inside the Bevy surface.
    let center_x = pad + 19.;
    let center_y = title_y + 18.;
    for i in 0..28 {
        let a = i as f32 * std::f32::consts::TAU / 28.;
        d.rect(
            center_x + a.cos() * 14.,
            center_y + a.sin() * 14.,
            3.,
            3.,
            t.colors[0],
            2.,
        );
    }
    for i in 0..11 {
        d.rect(
            pad + 3. + i as f32 * 3.,
            title_y + 32. - i as f32 * 3.,
            4.,
            3.,
            t.ink,
            1.,
        );
    }
    d.text(pad + 54., title_y + 3., "TANGENT", 27., t.ink);

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

    if !compact && !short {
        let hero_y = (l.height * 0.19).clamp(145., 195.);
        let tile_x = (l.width * 0.49).max(600.).min(l.width - 365.);
        let gap = 14.;
        let tile_w = (l.width - tile_x - pad - gap) / 2.;
        let tile_h = (l.height * 0.21).clamp(140., 190.);

        d.text(pad, hero_y, "MAKE TIME FOR MUSIC", 12., t.colors[0]);
        d.text(pad, hero_y + 39., "Practice with", 40., t.ink);
        d.text(pad, hero_y + 86., "purpose.", 48., t.colors[0]);
        d.text(
            pad,
            hero_y + 150.,
            "Play favourites. Read notes. Explore freely.",
            12.,
            t.muted,
        );

        let card_y = hero_y + 202.;
        let card_w = (tile_x - pad - 34.).min(490.);
        d.rect(pad, card_y, card_w, 152., t.panel, 12.);
        d.text(
            pad + 18.,
            card_y + 16.,
            "CURRENTLY LOADED",
            11.,
            t.colors[0],
        );
        let max_title = ((card_w - 36.) / 11.).max(8.) as usize;
        let title: String = state.song.title.chars().take(max_title).collect();
        d.text(pad + 18., card_y + 46., title, 19., t.ink);
        d.text(
            pad + 18.,
            card_y + 78.,
            if state.midi_output.is_empty() {
                "Computer audio"
            } else {
                "External MIDI output selected"
            },
            11.,
            t.muted,
        );
        d.button(
            state,
            pad + 18.,
            card_y + 108.,
            145.,
            "Play loaded piece",
            Action::StartPlay,
            t,
        );

        let tile_y = hero_y + 9.;
        for (i, (title, subtitle, action)) in [
            ("Play", "Learn songs", Action::StartPlay),
            ("Train", "Notes & chords", Action::Train),
            ("FreeForm", "Explore freely", Action::FreeForm),
            ("Library", "Find your music", Action::Library),
        ]
        .into_iter()
        .enumerate()
        {
            let x = tile_x + (i % 2) as f32 * (tile_w + gap);
            let y = tile_y + (i / 2) as f32 * (tile_h + gap);
            d.menu_tile(state, x, y, tile_w, tile_h, title, subtitle, action, t);
        }
        d.button(
            state,
            l.width - pad - 118.,
            title_y + 4.,
            118.,
            "Settings",
            Action::Settings,
            t,
        );
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
