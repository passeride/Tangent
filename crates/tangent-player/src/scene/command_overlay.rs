//! Piano command discoverability. Reads the existing grammar and saved bindings;
//! never owns input, command dispatch, audio or persistence.
use super::{draw::Drawing, theme, *};
use tangent_core::input_bindings::{CATALOG, pitch};

fn symbol(action: &str, playing: bool) -> &'static str {
    match action {
        "play" if playing => "II",
        "play" => ">",
        "pause" => "II",
        "back" | "back_long" | "nav_previous" => "<<",
        "forward" | "forward_long" | "nav_next" => ">>",
        "slower" | "quieter" => "-",
        "faster" | "louder" => "+",
        "restart" => "|<",
        "end" => ">|",
        "loop_a" => "A",
        "loop_b" => "B",
        "loop_clear" => "X",
        "speed_reset" => "1x",
        "hint" => "?",
        "fullscreen" | "fit" => "[]",
        "view" => "V",
        "theme" => "T",
        "hear" | "listen" => "S",
        "effects" => "FX",
        "instrument" => "I",
        "range" => "R",
        "import" => "IN",
        "sample" => "DE",
        "midi" => "M",
        "search" => "?",
        "loop_snap" => "#",
        "part" => "P",
        "skip" => ">|",
        "nav_back" => "<",
        "nav_activate" | "nav_activate_alt" => "OK",
        _ => "...",
    }
}

pub(super) fn render(d: &mut Drawing, state: &State, l: &Layout, t: theme::Theme, now: u64) {
    let shortcuts = &state.shortcuts;
    if !shortcuts.config.piano_enabled || shortcuts.capture.is_some() {
        return;
    }
    let armed = shortcuts.piano.armed(now);
    let accent = t.colors[0];
    // Always identify the reserved control note, even before the first gesture.
    if let Some(r) = l.key(&state.piano, shortcuts.config.control) {
        let size = (r.w * 0.75).clamp(7., 22.);
        let x = r.x + r.w * 0.1;
        let y = r.y + r.h - size - 5.;
        d.rect(x, y, r.w * 0.8, size + 3., t.panel, 3.);
        // A three-bar control/sliders icon uses geometry, not font coverage.
        for i in 0..3 {
            let iy = y + 3. + i as f32 * size * 0.25;
            d.rect(x + 2., iy, r.w * 0.8 - 4., 1.5, accent, 0.);
            d.rect(
                x + r.w * (0.2 + i as f32 * 0.15),
                iy - 1.,
                2.,
                4.,
                accent,
                0.,
            );
        }
    }
    if !armed {
        return;
    }
    // A wrapping legend keeps full names readable even with 88 narrow keys.
    // Include offscreen notes: fitting the keyboard must not hide their binding.
    let available = l.width - 2. * l.margin;
    let columns = ((available / 210.).floor() as usize).max(1);
    let cell = available / columns as f32;
    let rows: Vec<_> = shortcuts
        .config
        .rows
        .iter()
        .filter(|r| r.piano.is_some())
        .collect();
    let max_lines = ((l.strike - l.top - 80.) / 24.).floor().max(1.) as usize;
    let count = rows.len().min(max_lines * columns);
    let lines = count.div_ceil(columns);
    let top = (l.strike - 40. - lines as f32 * 24.).max(l.top + 30.);
    d.rect(
        l.margin,
        top - 8.,
        available,
        34. + lines.max(1) as f32 * 24.,
        t.panel,
        6.,
    );
    d.text(
        l.margin + 8.,
        top,
        format!(
            "Piano controls / Ctrl {} / next note = command",
            pitch(shortcuts.config.control)
        ),
        12.,
        accent,
    );
    for (i, binding) in rows.iter().enumerate() {
        let key = binding.piano.unwrap();
        let icon = symbol(&binding.action, state.transport.playing);
        let label = if binding.action == "play" {
            if state.transport.playing {
                "Pause"
            } else {
                "Play"
            }
        } else {
            CATALOG
                .iter()
                .find(|r| r.0 == binding.action)
                .map_or(binding.action.as_str(), |r| r.1)
        };
        let x = l.margin + (i % columns) as f32 * cell + 8.;
        let y = top + 24. + (i / columns) as f32 * 24.;
        let text = format!("{} {icon} {label}", pitch(key));
        let size = (cell / (text.chars().count() as f32 * 0.62)).clamp(8., 13.);
        if i < count {
            d.text(x, y, text, size, t.ink);
        }
        if let Some(r) = l.key(&state.piano, key) {
            // Black-key badges sit high, white-key badges below their tips.
            let y = if state.piano.keys.iter().any(|k| k.key == key && k.black) {
                r.y + r.h * 0.25
            } else {
                r.y + r.h * 0.76
            };
            let size = (r.w * 0.7 / (icon.len() as f32 * 0.62)).clamp(7., 22.);
            d.rect(r.x + 1., y - 2., r.w - 2., size + 7., accent, 3.);
            d.text(r.x + r.w * 0.15, y, icon, size, t.bg);
        }
    }
    if count < rows.len() {
        d.text(
            l.margin + 8.,
            l.strike - 14.,
            "More commands: Settings / Accessibility / Keybindings",
            10.,
            accent,
        );
    }
    if rows.is_empty() {
        d.text(
            l.margin + 8.,
            top + 24.,
            "Assign command notes in Settings / Accessibility / Keybindings",
            12.,
            t.ink,
        );
    }
}
