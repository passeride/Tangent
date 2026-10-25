#![allow(clippy::too_many_arguments)]
use super::{controls::Action, layout::Rect, *};
use bevy::text::FontSize;
use tangent_core::notation::{ledger_steps, reading_pitch};
#[derive(Resource, Default)]
pub struct Pool {
    shapes: Vec<Entity>,
    labels: Vec<Entity>,
}
struct Shape {
    r: Rect,
    color: Color,
    radius: f32,
}
struct Label {
    x: f32,
    y: f32,
    text: String,
    size: f32,
    color: Color,
}
#[derive(Default)]
struct Drawing {
    shapes: Vec<Shape>,
    labels: Vec<Label>,
}
impl Drawing {
    fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color, radius: f32) {
        if w > 0. && h > 0. {
            self.shapes.push(Shape {
                r: Rect { x, y, w, h },
                color,
                radius,
            });
        }
    }
    fn text(&mut self, x: f32, y: f32, text: impl Into<String>, size: f32, color: Color) {
        self.labels.push(Label {
            x,
            y,
            text: text.into(),
            size,
            color,
        });
    }
    fn line(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.rect(x, y, w.max(1.), h.max(1.), color, 0.);
    }
    fn button(
        &mut self,
        state: &mut State,
        x: f32,
        y: f32,
        w: f32,
        label: impl Into<String>,
        action: Action,
        t: theme::Theme,
    ) {
        self.rect(x, y, w, 30., t.panel, 6.);
        let label = label.into();
        let size = 12.0_f32
            .min((w - 18.) / (label.chars().count().max(1) as f32 * 0.62))
            .max(9.);
        self.text(x + 9., y + 7., label, size, t.ink);
        state.buttons.push((Rect { x, y, w, h: 30. }, action));
    }
    fn note(&mut self, x: f32, y: f32, size: f32, color: Color, hollow: bool, flag: bool) {
        self.rect(x - size, y - size * 0.6, size * 2., size * 1.2, color, size);
        if hollow {
            self.rect(
                x - size + 2.,
                y - size * 0.6 + 2.,
                size * 2. - 4.,
                size * 1.2 - 4.,
                Color::srgb(0.06, 0.12, 0.15),
                size,
            );
        }
        self.line(x + size - 1., y - size * 4.5, 1.5, size * 4.5, color);
        if flag {
            self.rect(x + size, y - size * 4.5, size * 0.9, size * 1.8, color, 5.);
        }
    }
}
fn time(us: u64) -> String {
    let s = us / 1_000_000;
    format!("{}:{:02}", s / 60, s % 60)
}
pub fn render(
    mut commands: Commands,
    mut state: ResMut<State>,
    mut pool: ResMut<Pool>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let l = Layout::new(window.width(), window.height(), &state.piano);
    let t = theme::palette(state.theme);
    let mut d = Drawing::default();
    state.buttons.clear();
    d.rect(0., 0., l.width, l.height, t.bg, 0.);
    if !state.help {
        // A single gear is the only persistent application control.
        gear(&mut d, &mut state, &l, t);
        if state.view == View::Score {
            score(&mut d, &state, &l, t);
        } else {
            falling(&mut d, &state, &l, t);
        }
        let mut ids = BTreeSet::new();
        let mut bursts = vec![];
        for n in &state.song.notes {
            if n.start_us <= state.transport.position_us
                && n.end_us > state.transport.position_us
                && !state.song.parts[n.part].percussion
            {
                ids.insert(n.id);
                if state.effects
                    && state.transport.playing
                    && !state.previous.contains(&n.id)
                    && let Some(x) = l.center(&state.piano, n.key)
                {
                    bursts.push((x, n.id, t.colors[n.part % 4]));
                }
            }
        }
        for key in std::mem::take(&mut state.live_bursts) {
            if state.effects
                && let Some(x) = l.center(&state.piano, key)
            {
                bursts.push((x, key as usize, t.colors[0]));
            }
        }
        for (x, id, color) in bursts {
            for i in 0..8 {
                let seed = ((id * 17 + i * 31) % 101) as f32;
                state.particles.push(Particle {
                    x,
                    y: l.strike,
                    vx: seed - 50.,
                    vy: -40. - seed,
                    life: 0.4 + seed / 200.,
                    color,
                });
            }
        }
        state.previous = ids;
        if state.particles.len() > 400 {
            let extra = state.particles.len() - 400;
            state.particles.drain(..extra);
        }
        if state.effects {
            for p in &state.particles {
                d.rect(
                    p.x,
                    p.y,
                    3.,
                    3.,
                    p.color.with_alpha(p.life.clamp(0., 1.)),
                    3.,
                );
            }
        }
        keyboard(&mut d, &state, &l, t);
        d.rect(
            l.progress.x,
            l.progress.y,
            l.progress.w,
            l.progress.h,
            t.panel,
            5.,
        );
        d.rect(
            l.progress.x,
            l.progress.y,
            l.progress.w
                * (state.transport.position_us as f32 / state.transport.duration_us.max(1) as f32),
            l.progress.h,
            t.colors[0],
            5.,
        );
    } else {
        settings(&mut d, &mut state, &l, t);
    }
    let button_report: Vec<_> = state
        .buttons
        .iter()
        .map(|(r, a)| serde_json::json!({"id":a.id(),"x":r.x,"y":r.y,"w":r.w,"h":r.h}))
        .collect();
    let key_rects: Vec<_> = state
        .piano
        .keys
        .iter()
        .filter_map(|k| {
            l.key(&state.piano, k.key)
                .map(|r| serde_json::json!({"key":k.key,"x":r.x,"y":r.y,"w":r.w,"h":r.h}))
        })
        .collect();
    platform::report(&serde_json::json!({"position":state.transport.position_us,"playing":state.transport.playing,"volume":state.volume,"view":format!("{:?}",state.view),"staff_lines":if matches!(state.view,View::Staff|View::Notes|View::Score){10}else{0},"notes":state.song.notes.len(),"buttons":button_report,"width":l.width,"height":l.height,"live":state.held.values().copied().collect::<Vec<_>>(),"key_rects":key_rects,"renderer":"bevy-wgpu","help":state.help}).to_string());
    apply(&mut commands, &mut pool, d);
}
fn apply(commands: &mut Commands, pool: &mut Pool, d: Drawing) {
    while pool.shapes.len() < d.shapes.len() {
        pool.shapes.push(commands.spawn_empty().id());
    }
    while pool.labels.len() < d.labels.len() {
        pool.labels.push(commands.spawn_empty().id());
    }
    for (i, &entity) in pool.shapes.iter().enumerate() {
        if let Some(s) = d.shapes.get(i) {
            commands.entity(entity).insert((
                Node {
                    position_type: PositionType::Absolute,
                    left: px(s.r.x),
                    top: px(s.r.y),
                    width: px(s.r.w),
                    height: px(s.r.h),
                    border_radius: BorderRadius::all(px(s.radius)),
                    ..default()
                },
                BackgroundColor(s.color),
                GlobalZIndex(i as i32),
                Visibility::Visible,
            ));
        } else {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
    for (i, &entity) in pool.labels.iter().enumerate() {
        if let Some(s) = d.labels.get(i) {
            commands.entity(entity).insert((
                Node {
                    position_type: PositionType::Absolute,
                    left: px(s.x),
                    top: px(s.y),
                    ..default()
                },
                Text::new(&s.text),
                TextFont {
                    font_size: FontSize::Px(s.size),
                    ..default()
                },
                TextColor(s.color),
                GlobalZIndex(100_000 + i as i32),
                Visibility::Visible,
            ));
        } else {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
}
fn gear(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let x = l.width - l.margin - 36.;
    let y = 12.;
    d.rect(x, y, 36., 36., t.panel.with_alpha(0.8), 10.);
    // Geometry instead of a font glyph, so the icon works with the bundled font.
    for i in 0..8 {
        let a = i as f32 * std::f32::consts::TAU / 8.;
        d.rect(
            x + 15. + a.cos() * 10.,
            y + 15. + a.sin() * 10.,
            6.,
            6.,
            t.ink,
            2.,
        );
    }
    d.rect(x + 8., y + 8., 20., 20., t.ink, 10.);
    d.rect(x + 13., y + 13., 10., 10., t.panel, 5.);
    state.buttons.push((
        Rect {
            x,
            y,
            w: 36.,
            h: 36.,
        },
        Action::Help,
    ));
}
fn settings(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    d.text(l.margin, 18., "Paused / Tangent", 22., t.ink);
    let items = [
        ("Resume [Space]".into(), Action::Play),
        ("Restart".into(), Action::Restart),
        ("Seek -5s".into(), Action::Back),
        ("Seek +5s".into(), Action::Forward),
        (
            format!("Volume - ({}%)", (state.volume * 100.).round()),
            Action::Quieter,
        ),
        ("Volume +".into(), Action::Louder),
        (format!("View: {} [V]", state.view.name()), Action::View),
        (
            format!("Theme: {} [B]", ["Aurora", "Ember", "Paper"][state.theme]),
            Action::Theme,
        ),
        (format!("Hear song: {} [M]", state.hear), Action::Hear),
        (format!("Sparkles: {} [P]", state.effects), Action::Effects),
        (
            format!(
                "Piano: {} [X]",
                ["Soft felt", "Bright keys"][state.instrument]
            ),
            Action::Instrument,
        ),
        (
            format!("Keys: {} [R]", state.piano.keys.len()),
            Action::Range,
        ),
        ("Import score [Ctrl+O]".into(), Action::Import),
        ("Try Bach [Ctrl+B]".into(), Action::Sample),
        ("Connect MIDI [Ctrl+M]".into(), Action::Midi),
        ("Music credits".into(), Action::Credits),
        ("Full screen [F11]".into(), Action::Fullscreen),
        ("Return [Esc / F1]".into(), Action::Help),
    ];
    let cols = if l.width > 700. { 3 } else { 2 };
    let row = if l.height < 600. { 35. } else { 43. };
    let start = if l.height < 600. { 50. } else { 65. };
    let w = (l.width - 2. * l.margin - 12. * (cols - 1) as f32) / cols as f32;
    for (i, (label, action)) in items.into_iter().enumerate() {
        let label = if w < 160. {
            label
                .replace("Falling blocks", "Blocks")
                .replace("Staff blocks", "Staff")
                .replace("Staff notes", "Notes")
                .replace("Sheet & playhead", "Sheet")
                .replace("Soft felt", "Felt")
                .replace("Bright keys", "Bright")
                .split(" [")
                .next()
                .unwrap_or("")
                .to_string()
        } else {
            label
        };
        d.button(
            state,
            l.margin + (i % cols) as f32 * (w + 12.),
            start + (i / cols) as f32 * row,
            w,
            label,
            action,
            t,
        );
    }
    d.text(
        l.margin,
        l.height - 50.,
        format!(
            "{}  {} / {}",
            state.song.title,
            time(state.transport.position_us),
            time(state.transport.duration_us)
        ),
        11.,
        t.ink,
    );
    d.text(l.margin, l.height - 30., &state.status, 11., t.muted);
    let y = start + (18_usize.div_ceil(cols)) as f32 * row + 15.;
    let line = ((l.height - y - 16.) / 8.).clamp(10., 23.);
    for (i, text) in [
        "Space: play / pause",
        "Left / Right: seek 5 seconds (Shift: 15)",
        "Up / Down: volume (5% per step)",
        "Home: beginning    End: end",
        "A W S E D F T G Y H U J K: C4 to C5",
        "MIDI reading views use approximate notation.",
        "Bach BWV 924a: jeff covey / Mutopia",
        "CC BY-SA 3.0. Music credits opens the license.",
    ]
    .iter()
    .enumerate()
    {
        if y + i as f32 * line < l.height - 70. {
            d.text(l.margin, y + i as f32 * line, *text, 12., t.muted);
        }
    }
}
fn keyboard(d: &mut Drawing, state: &State, l: &Layout, t: theme::Theme) {
    let mut active = BTreeMap::new();
    let end = state
        .song
        .notes
        .partition_point(|n| n.start_us <= state.transport.position_us);
    for n in &state.song.notes[..end] {
        if n.end_us > state.transport.position_us && !state.song.parts[n.part].percussion {
            active.insert(n.key, t.colors[n.part % 4]);
        }
    }
    for key in state.held.values() {
        active.insert(*key, t.colors[0]);
    }
    for black in [false, true] {
        for k in state.piano.keys.iter().filter(|k| k.black == black) {
            let r = l.key(&state.piano, k.key).unwrap();
            d.rect(
                r.x + 1.,
                r.y + 1.,
                (r.w - 2.).max(1.),
                r.h - 3.,
                active
                    .get(&k.key)
                    .copied()
                    .unwrap_or(if black { t.black } else { t.white }),
                3.,
            );
            if k.key % 12 == 0 && !black {
                d.text(
                    r.x + 3.,
                    r.y + r.h - 19.,
                    format!("C{}", k.key as i16 / 12 - 1),
                    9.,
                    t.muted,
                );
            }
        }
    }
    d.line(l.margin, l.strike, l.width - 2. * l.margin, 1., t.colors[0]);
}
fn staff(d: &mut Drawing, state: &State, l: &Layout, t: theme::Theme) {
    for key in [43, 47, 50, 53, 57, 64, 67, 71, 74, 77] {
        if let Some(x) = l.center(&state.piano, key) {
            d.line(x, l.top, 1., l.strike - l.top, t.staff);
        }
    }
    if let Some(x) = l.center(&state.piano, 50) {
        d.text(x - 20., l.top + 8., "F / bass", 11., t.muted);
    }
    if let Some(x) = l.center(&state.piano, 71) {
        d.text(x - 23., l.top + 8., "G / treble", 11., t.muted);
    }
}
fn falling(d: &mut Drawing, state: &State, l: &Layout, t: theme::Theme) {
    if matches!(state.view, View::Staff | View::Notes) {
        staff(d, state, l, t);
    }
    let pps = (l.strike - l.top) / 5.5;
    let now = state.transport.position_us;
    let end = state
        .song
        .notes
        .partition_point(|n| n.start_us <= now + 6_000_000);
    for n in &state.song.notes[..end] {
        if n.end_us <= now || state.song.parts[n.part].percussion {
            continue;
        }
        let Some(r) = l.key(&state.piano, n.key) else {
            continue;
        };
        let onset = l.strike - (n.start_us as f64 - now as f64) as f32 / 1e6 * pps;
        let tail = l.strike - (n.end_us as f64 - now as f64) as f32 / 1e6 * pps;
        let color = t.colors[n.part % 4];
        let y = onset.min(l.strike - 7.);
        if y < l.top + 25. {
            continue;
        }
        if matches!(state.view, View::Staff | View::Notes) {
            let p = reading_pitch(n.key);
            for step in ledger_steps(p.step, if p.bass { 18 } else { 30 }) {
                if let Some(key) = (0..=127).find(|&k| {
                    let rp = reading_pitch(k);
                    rp.step == step && !rp.sharp
                }) && let Some(x) = l.center(&state.piano, key)
                {
                    d.line(x, y - 9., 1.5, 18., t.ink);
                }
            }
            if p.sharp {
                d.text(r.x - 6., y - 14., "#", 12., t.ink);
            }
        }
        if state.view == View::Notes {
            d.note(
                r.x + r.w / 2.,
                y,
                (l.unit * 0.42).clamp(3., 7.),
                color,
                n.end_us - n.start_us >= 800_000,
                n.end_us - n.start_us < 300_000,
            );
            if now >= n.start_us {
                d.rect(
                    r.x,
                    y + 9.,
                    r.w * (1. - (now - n.start_us) as f32 / (n.end_us - n.start_us).max(1) as f32),
                    2.,
                    color,
                    0.,
                );
            }
        } else {
            let top = tail.max(l.top + 25.);
            d.rect(
                r.x + r.w * 0.15,
                top,
                r.w * 0.7,
                (onset.min(l.strike) - top).max(3.),
                color,
                4.,
            );
        }
    }
}
fn score_pitch(
    n: &tangent_core::music::PerformanceNote,
    bass: Option<bool>,
) -> tangent_core::notation::ReadingPitch {
    use tangent_core::music::PitchLetter::*;
    let mut p = reading_pitch(n.key);
    if let Some(w) = &n.written {
        p.step = i16::from(w.octave) * 7
            + match w.letter {
                C => 0,
                D => 1,
                E => 2,
                F => 3,
                G => 4,
                A => 5,
                B => 6,
            };
        p.bass = bass.unwrap_or(p.bass);
    }
    p
}
fn score(d: &mut Drawing, state: &State, l: &Layout, t: theme::Theme) {
    let gap = ((l.strike - l.top) / 22.).clamp(5., 12.);
    let treble = l.top + gap * 7.;
    let bass = treble + gap * 10.;
    let left = l.margin + 38.;
    let right = l.width - l.margin;
    let span = 8_000_000;
    let start = state.transport.position_us / span * span;
    for bottom in [treble, bass] {
        for i in 0..5 {
            d.line(
                l.margin,
                bottom - i as f32 * gap,
                l.width - 2. * l.margin,
                1.,
                t.staff,
            );
        }
    }
    d.text(l.margin, treble - 3. * gap, "G", 20., t.ink);
    d.text(l.margin, bass - 3. * gap, "F", 20., t.ink);
    let py = |p: tangent_core::notation::ReadingPitch| {
        if p.bass {
            bass - (p.step - 18) as f32 * gap / 2.
        } else {
            treble - (p.step - 30) as f32 * gap / 2.
        }
    };
    let mut clefs = BTreeMap::new();
    if let Some(score) = &state.song.notation {
        let staff_clefs: BTreeMap<_, _> = score
            .staves
            .iter()
            .map(|s| (s.id, matches!(s.clef, tangent_core::notation::Clef::Bass)))
            .collect();
        for note in score.measures.iter().flat_map(|m| &m.notes) {
            if let Some(id) = note.performance_note
                && let Some(bass) = staff_clefs.get(&note.staff)
            {
                clefs.entry(id).or_insert(*bass);
            }
        }
    }
    for n in &state.song.notes {
        if n.start_us < start || n.start_us >= start + span || state.song.parts[n.part].percussion {
            continue;
        }
        let p = score_pitch(n, clefs.get(&n.id).copied());
        let x = left + (right - left) * (n.start_us - start) as f32 / span as f32;
        let y = py(p);
        let color = t.colors[n.part % 4];
        if y < l.top + 10. || y > l.strike - 25. {
            continue;
        }
        for step in ledger_steps(p.step, if p.bass { 18 } else { 30 }) {
            d.line(
                x - 9.,
                py(tangent_core::notation::ReadingPitch { step, ..p }),
                18.,
                1.,
                color,
            );
        }
        let accidental = n
            .written
            .as_ref()
            .map(|p| match p.alteration {
                -2 => "bb",
                -1 => "b",
                1 => "#",
                2 => "x",
                _ => "",
            })
            .unwrap_or(if p.sharp { "#" } else { "" });
        if !accidental.is_empty() {
            d.text(x - 20., y - 11., accidental, 14., color);
        }
        d.note(
            x,
            y,
            5.,
            color,
            n.end_us - n.start_us >= 800_000,
            n.end_us - n.start_us < 300_000,
        );
        d.line(
            x,
            y + 9.,
            ((right - left) * (n.end_us - n.start_us) as f32 / span as f32).min(right - x),
            2.,
            color.with_alpha(0.3),
        );
    }
    let x = left + (right - left) * (state.transport.position_us - start) as f32 / span as f32;
    d.line(
        x,
        l.top + 8.,
        2.,
        (bass + gap * 2. - l.top).min(l.strike - l.top),
        t.colors[0],
    );
}
