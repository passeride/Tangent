//! Bevy-only lesson presentation. Content, scale roles and input evaluation live in tangent-core.
use super::{controls::Action, draw::Drawing, *};
use tangent_core::lesson::{self, Illustration, Task, ToneRole};

pub(super) fn catalog(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let x = l.margin + 8.;
    let compact = l.width < 680.;
    let short = l.height < 550.;
    d.text(x, 16., "LÆR / INTERAKTIV MUSIKKTEORI", if compact { 18. } else { 25. }, t.ink);
    d.text(x, 55., "Les, lytt og prøv selv på ditt piano.", 13., t.muted);
    let top = if short { 94. } else { 117. };
    let available = (l.height - top - 75.).max(160.);
    let gap = if short { 7. } else { 13. };
    let height = ((available - gap * 3.) / 4.).clamp(34., 81.);
    for (i, module) in lesson::MODULES.iter().enumerate() {
        d.menu_card(
            state, x, top + i as f32 * (height + gap),
            (l.width - x - l.margin - 8.).min(690.), height,
            module.title, module.summary, Action::LessonOpen(i as u8), i == 0, t,
        );
    }
    d.button(state, x, l.height - 43., 118., "Tilbake", Action::Train, t);
}

/// Text wrapping is explicit: the renderer's Drawing::text draws one line.
fn wrapped(d: &mut Drawing, x: f32, y: f32, width: f32, text: &str, size: f32,
    color: Color, max_lines: usize) -> f32 {
    let limit = ((width / (size * 0.58)).floor() as usize).max(11);
    let mut line = String::new();
    let mut lines = Vec::new();
    for word in text.split_whitespace() {
        if !line.is_empty() && line.chars().count() + 1 + word.chars().count() > limit {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() { line.push(' '); }
        line.push_str(word);
    }
    if !line.is_empty() { lines.push(line); }
    let rendered = lines.len().min(max_lines);
    for (i, line) in lines.into_iter().take(max_lines).enumerate() {
        d.text(x, y + i as f32 * (size + 5.), line, size, color);
    }
    rendered as f32 * (size + 5.)
}

pub(super) fn render(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let Some(module_index) = state.lesson.module else { return };
    let Some(step) = state.lesson.current().copied() else { return };
    let module = &lesson::MODULES[module_index];
    let compact = l.width < 650.;
    let short = l.height < 570.;
    let pad = l.margin + 8.;
    let width = l.width - 2. * pad;
    let panel_top = if short { 76. } else { 100. };
    let button_y = l.strike - 42.;
    let panel_bottom = button_y - 8.;
    let panel_h = (panel_bottom - panel_top).max(30.);
    d.text(pad, 12., format!("LÆR / {}", module.title), if compact { 17. } else { 23. }, t.ink);
    d.text(pad, 47., format!("Steg {} av {}  •  {}", state.lesson.step + 1, module.steps.len(),
        if state.lesson.complete() { "Klar for neste" } else { "Spill på pianoet" }), 12., t.muted);
    d.button(state, l.width - pad - 72., 10., 72., "Avslutt", Action::LessonExit, t);

    d.rect(pad, panel_top, width, panel_h, t.panel, 12.);
    d.rect(pad, panel_top, width * (state.lesson.step + 1) as f32 / module.steps.len() as f32,
        3., t.colors[0], 1.);
    d.text(pad + 14., panel_top + 11., step.title, if compact { 16. } else { 22. }, t.ink);
    let text_size = if compact || short { 11. } else { 14. };
    let mut text_y = panel_top + if compact { 37. } else { 45. };
    let text_bottom = if short { panel_top + 82. } else { panel_top + 128. };
    for paragraph in step.paragraphs {
        if text_y >= text_bottom { break; }
        text_y += wrapped(d, pad + 14., text_y, width - 28., paragraph, text_size, t.muted,
            if short { 1 } else { 2 }) + if short { 2. } else { 5. };
    }

    let visual_top = if short { panel_top + 90. } else { panel_top + 140. };
    let visual_h = (panel_bottom - visual_top - 27.).max(12.);
    match step.illustration {
        Illustration::MajorIntervals | Illustration::MinorIntervals =>
            intervals(d, state, pad + 12., visual_top, width - 24., visual_h, t,
                step.illustration == Illustration::MinorIntervals),
        Illustration::Fifths => fifths(d, state, pad + 12., visual_top, width - 24., visual_h, t),
        Illustration::Harmony => harmony(d, state, pad + 12., visual_top, width - 24., visual_h, t),
        Illustration::Keyboard => example(d, state, pad + 12., visual_top, width - 24., t),
    };
    if !short && visual_h >= 35. {
        d.text(pad + 14., panel_bottom - 22.,
            step.instruction, if compact { 10. } else { 12. }, t.ink);
    }
    let button_gap = 6.;
    let can_advance = state.lesson.complete();
    let mut actions = vec![
        ("Forrige", Action::LessonPrevious),
        ("Lytt", Action::LessonListen),
        (if state.lesson.reveal { "Skjul hjelp" } else { "Vis hjelp" }, Action::LessonReveal),
        (if state.lesson.step + 1 == module.steps.len() { "Fullfør" }
            else if can_advance { "Neste" } else { "Prøv først" },
            if state.lesson.step + 1 == module.steps.len() { Action::LessonExit } else { Action::LessonNext }),
    ];
    match step.illustration {
        Illustration::Harmony => actions.insert(2, ("Bytt akkord", Action::LessonContext)),
        Illustration::Fifths => actions.insert(2, ("Toneart +", Action::LessonContext)),
        _ => {}
    }
    let button_width = (width - (actions.len() - 1) as f32 * button_gap) / actions.len() as f32;
    let last = actions.len() - 1;
    for (i, (label, action)) in actions.into_iter().enumerate() {
        if i == last && !can_advance { continue; }
        d.button(state, pad + i as f32 * (button_width + button_gap),
            button_y, button_width, label, action, t);
    }
    if state.lesson.step == 0 && !short {
        d.text(pad, l.strike - 7., "MIDI, PC-tastatur eller skjermtangenter", 10., t.muted);
    }
    if state.lesson.complete() {
        d.text(pad, l.strike - 9., "Oppgaven er løst. Du kan fortsette.", 10., t.colors[0]);
    }
}

fn example(d: &mut Drawing, state: &State, x: f32, y: f32, w: f32, t: theme::Theme) {
    let step = state.lesson.current().unwrap();
    let notes: Vec<u8> = match step.task {
        Task::Chord(notes) | Task::Sequence(notes) => notes.to_vec(),
        Task::Note(n) | Task::Explore(n) => vec![n],
        Task::Read => step.example.to_vec(),
    };
    let mut label = String::from("Eksempel: ");
    for (i, note) in notes.iter().enumerate() {
        if i > 0 { label.push_str(if matches!(step.task, Task::Chord(_)) { " + " } else { " → " }); }
        label.push_str(lesson::note_name(*note));
    }
    wrapped(d, x, y + 2., w, &label, 16., t.colors[0], 2);
    d.text(x, y + 36.,
        if state.lesson.complete() { "Du fant mønsteret." } else { state.lesson.message },
        11., t.muted);
}

fn harmony(d: &mut Drawing, state: &mut State, x: f32, y: f32, w: f32, h: f32, t: theme::Theme) {
    let chord = state.lesson.chord();
    let label = state.lesson.chord_label();
    d.text(x, y + 1., format!("Underliggende akkord: {label}"), 15., t.colors[0]);
    let names: Vec<_> = chord.iter().map(|&p| lesson::note_name(p)).collect();
    d.text(x, y + 24., format!("Akkordtoner: {}", names.join(" – ")), 12., t.ink);
    if h > 95. {
        let description = state.lesson.last_note.map(|pitch| match state.lesson.role(pitch) {
            ToneRole::Chord => format!("{} er akkordtone og kan oppleves stabil.", lesson::note_name(pitch)),
            ToneRole::Scale => format!("{} er i skalaen, men ikke i akkorden.", lesson::note_name(pitch)),
            ToneRole::Outside => format!("{} er en kromatisk tone utenfor skalaen.", lesson::note_name(pitch)),
        }).unwrap_or_else(|| "Spill B og hør hva som endrer seg.".into());
        wrapped(d, x, y + 46., w, &description, 11., t.muted, 2);
    }
}


fn intervals(d: &mut Drawing, state: &State, x: f32, y: f32, w: f32, h: f32,
    t: theme::Theme, minor: bool) {
    let semitones = if minor { &lesson::NATURAL_MINOR } else { &lesson::MAJOR };
    let span = (w - 30.).max(80.);
    let labels = if minor {
        ["C", "D", "E♭", "F", "G", "A♭", "B♭", "C"]
    } else { ["C", "D", "E", "F", "G", "A", "B", "C"] };
    let mut previous = None;
    for i in 0..8 {
        let pos = if i == 7 { 12 } else { semitones[i] } as f32;
        let cx = x + 14. + pos / 12. * span;
        if let Some((from, difference)) = previous {
            d.rect(from + 4., y + 14., (cx - from - 8.).max(1.), 2.,
                if difference == 1 { t.colors[2] } else { t.colors[1] }, 1.);
        }
        d.rect(cx - 8., y + 7., 16., 16.,
            if i == 0 || i == 7 { t.colors[0] } else { t.colors[1] }, 8.);
        d.text(cx - 9., y + 32., labels[i], if w < 440. { 10. } else { 14. }, t.ink);
        if i > 0 && w > 600. {
            let before = if i == 1 { 0 } else { semitones[i - 1] };
            let difference = (pos as u8) - before;
            d.text(cx - (pos - before as f32) / 24. * span - 11., y - 9.,
                if difference == 1 { "Halv" } else { "Hel" }, 11., t.muted);
        }
        previous = Some((cx, if i == 7 { 1 } else { semitones.get(i + 1).copied().unwrap_or(12) - semitones[i] }));
    }
    if h > 75. {
        d.text(x + 2., y + 66., if minor {
            "Moll: hel – halv – hel – hel – halv – hel – hel"
        } else {
            "Dur: hel – hel – halv – hel – hel – hel – halv"
        }, if w < 430. { 10. } else { 13. }, t.muted);
    }
    if state.lesson.reveal && h > 107. {
        d.text(x + 2., y + 91., "Hjelp: markeringene på pianoet viser skala og akkord.", 11., t.colors[0]);
    }
}

fn fifths(d: &mut Drawing, state: &mut State, x: f32, y: f32, w: f32, h: f32, t: theme::Theme) {
    let selected = state.lesson.fifth;
    let (root, relative, accidentals) = lesson::FIFTHS[selected];
    let signature = if accidentals == 0 { "ingen fortegn".into() }
        else if accidentals > 0 { format!("{accidentals} kryss") }
        else { format!("{} b-er", -accidentals) };
    if h >= 160. && w >= 470. {
        let cx = x + w * 0.48;
        let cy = y + h * 0.49;
        let radius = (h * 0.37).min(105.);
        for (i, (name, _, _)) in lesson::FIFTHS.iter().enumerate() {
            let theta = i as f32 * std::f32::consts::TAU / 12. - std::f32::consts::FRAC_PI_2;
            let px = cx + radius * theta.cos();
            let py = cy + radius * theta.sin();
            d.rect(px - 17., py - 15., 34., 30.,
                if i == selected { t.colors[0] } else { t.bg }, 15.);
            d.text(px - 12., py - 9., *name, 13.,
                if i == selected { t.bg } else { t.ink });
        }
        d.text(cx - 43., cy - 13., format!("{root} / {relative}"), 17., t.ink);
    } else {
        // A small 6x2 circular-order grid remains legible on phones/short windows.
        let cell = (w / 6.).max(32.);
        let space = if h < 75. { 27. } else { 35. };
        for (i, (name, _, _)) in lesson::FIFTHS.iter().enumerate() {
            let px = x + (i % 6) as f32 * cell;
            let py = y + (i / 6) as f32 * space;
            d.rect(px + 1., py, (cell - 4.).max(27.), 25.,
                if i == selected { t.colors[0] } else { t.bg }, 7.);
            d.text(px + 7., py + 5., *name, 12.,
                if i == selected { t.bg } else { t.ink });
        }
    }
    d.text(x, y + h - 18., format!("{root}-dur  •  {relative}  •  {signature}"), 11., t.muted);
    // Selection is available in the fixed navigation row at every viewport.
}

pub(super) fn piano_overlay(d: &mut Drawing, state: &State, l: &Layout, t: theme::Theme) {
    if state.mode != Mode::Lesson || state.help { return; }
    let reveal = state.lesson.reveal || state.lesson.complete();
    let expected = if reveal { state.lesson.next_target() } else { vec![] };
    for key in &state.piano.keys {
        if !reveal && !state.held.values().any(|&played| played == key.key) { continue; }
        let Some(r) = l.key(&state.piano, key.key) else { continue };
        let role = state.lesson.role(key.key);
        let color = match role {
            ToneRole::Chord => t.colors[0],
            ToneRole::Scale => t.colors[2],
            ToneRole::Outside => t.muted,
        };
        if reveal {
            d.rect(r.x + r.w * 0.12, r.y + 3., r.w * 0.76,
                if expected.contains(&key.key) { 11. } else { 5. },
                color.with_alpha(if expected.contains(&key.key) { 0.9 } else { 0.55 }), 2.);
        }
    }
    if reveal {
        d.text(l.margin + 8., l.strike - 16.,
            "Grønn = akkord   Gull = skala   Grå = utenfor (ikke «feil»)",
            if l.width < 650. { 9. } else { 11. }, t.muted);
    }
}

pub(super) fn audition(state: &mut State) {
    state.stop_reference();
    state.lesson_demo = state.lesson.demonstration().into_iter().rev().collect();
    state.reference_remaining = 0.;
}
