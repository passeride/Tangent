#![allow(clippy::too_many_arguments)]
use super::{controls::Action, layout::Rect, *};
use bevy::text::{FontSize, LineHeight};
#[derive(Resource)]
pub struct MusicFont(pub Handle<Font>);
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
pub(super) struct Label {
    x: f32,
    y: f32,
    text: String,
    size: f32,
    color: Color,
    music: bool,
}
#[derive(Default)]
pub(super) struct Drawing {
    shapes: Vec<Shape>,
    labels: Vec<Label>,
    background: Color,
}
impl Drawing {
    pub(super) fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color, radius: f32) {
        if w > 0. && h > 0. {
            self.shapes.push(Shape {
                r: Rect { x, y, w, h },
                color,
                radius,
            });
        }
    }
    pub(super) fn text(
        &mut self,
        x: f32,
        y: f32,
        text: impl Into<String>,
        size: f32,
        color: Color,
    ) {
        self.labels.push(Label {
            x,
            y,
            text: text.into(),
            size,
            color,
            music: false,
        });
    }
    pub(super) fn line(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.rect(x, y, w.max(1.), h.max(1.), color, 0.);
    }
    /// Bravura has symmetric ascent/descent; an explicit line height anchors its
    /// SMuFL baseline at y, independent of the font's extreme glyph repertoire.
    pub(super) fn glyph(&mut self, x: f32, baseline: f32, glyph: char, size: f32, color: Color) {
        self.labels.push(Label {
            x,
            y: baseline - size * 0.5,
            text: glyph.to_string(),
            size,
            color,
            music: true,
        });
    }
    pub(super) fn button(
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
    pub(super) fn note(
        &mut self,
        x: f32,
        y: f32,
        size: f32,
        color: Color,
        hollow: bool,
        flag: bool,
    ) {
        self.rect(x - size, y - size * 0.6, size * 2., size * 1.2, color, size);
        if hollow {
            self.rect(
                x - size + 2.,
                y - size * 0.6 + 2.,
                size * 2. - 4.,
                size * 1.2 - 4.,
                self.background,
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
    music_font: Res<MusicFont>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let l = Layout::new(window.width(), window.height(), &state.piano);
    let t = theme::palette(state.theme);
    let mut d = Drawing {
        background: t.bg,
        ..Drawing::default()
    };
    state.buttons.clear();
    d.rect(0., 0., l.width, l.height, t.bg, 0.);
    if state.screen != Screen::Session && !state.help {
        super::menu::render(&mut d, &mut state, &l, t);
    } else if !state.help {
        if state.mode == Mode::Play && state.view == View::Score {
            super::score::render(&mut d, &state, &l, t);
        } else if state.mode == Mode::Play {
            falling(&mut d, &state, &l, t);
        }
        let mut ids = BTreeSet::new();
        let mut bursts = vec![];
        for n in state.song.notes.iter().filter(|_| state.mode == Mode::Play) {
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
        if matches!(state.mode, Mode::Notes | Mode::Chords) {
            super::menu::exercise(&mut d, &state, &l, t);
        }
        if state.mode == Mode::Play {
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
                    * (state.transport.position_us as f32
                        / state.transport.duration_us.max(1) as f32),
                l.progress.h,
                t.colors[0],
                5.,
            );
        }
        // Symbolic recognition is visible during FreeForm, not only in Settings.
        if state.mode == Mode::FreeForm && !state.recognition_status.is_empty() {
            let width = (l.width - l.margin * 2.).min(690.);
            d.rect(l.margin, 48., width, 32., t.panel, 7.);
            let max_chars = ((width - 20.) / 7.).max(12.) as usize;
            let label: String = state.recognition_status.chars().take(max_chars).collect();
            d.text(l.margin + 10., 56., label, 12., t.ink);
        }
        // HUD goes above every musical layer, including moving ending brackets.
        gear(&mut d, &mut state, &l, t);
    } else {
        settings(&mut d, &mut state, &l, t);
    }
    #[cfg(target_arch = "wasm32")]
    {
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
    platform::report(&serde_json::json!({"screen":format!("{:?}",state.screen),"mode":format!("{:?}",state.mode),"completed":state.practice.completed,"preferences":state.preferences(),"storage_status":state.storage_status,"target":state.practice.target(),"position":state.transport.position_us,"playing":state.transport.playing,"volume":state.volume,"view":format!("{:?}",state.view),"staff_lines":if matches!(state.view,View::Staff|View::Notes|View::Score){10}else{0},"notes":state.song.notes.len(),"buttons":button_report,"width":l.width,"height":l.height,"live":state.held.values().copied().collect::<Vec<_>>(),"key_rects":key_rects,"renderer":"bevy-wgpu","help":state.help,"ending_labels":d.labels.iter().filter(|s|matches!(s.text.as_str(),"1."|"2.")).count(),"music_clefs":d.labels.iter().filter(|s|s.music&&matches!(s.text.as_str(),"\u{e050}"|"\u{e062}")).count(),"written_measures":state.song.notation.as_ref().map_or(0,|s|s.measures.len()),"measure_visits":state.song.notation.as_ref().map_or(0,|s|s.playback.len()),"ending_measures":state.song.notation.as_ref().map_or(0,|s|s.measures.iter().filter(|m|m.ending.is_some()).count())}).to_string());
    }
    apply(&mut commands, &mut pool, d, &music_font.0);
}
fn apply(commands: &mut Commands, pool: &mut Pool, d: Drawing, music_font: &Handle<Font>) {
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
                if s.music {
                    TextFont::from_font_size(s.size).with_font(music_font.clone())
                } else {
                    TextFont {
                        font_size: FontSize::Px(s.size),
                        ..default()
                    }
                },
                if s.music {
                    LineHeight::Px(s.size)
                } else {
                    LineHeight::default()
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
    d.text(
        l.margin,
        18.,
        if state.screen == Screen::Session {
            "Paused / Tangent"
        } else {
            "Settings / Tangent"
        },
        22.,
        t.ink,
    );
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
        ("XML demo [Ctrl+B]".into(), Action::Sample),
        ("Connect MIDI [Ctrl+M]".into(), Action::Midi),
        ("Music credits".into(), Action::Credits),
        ("Full screen [F11]".into(), Action::Fullscreen),
        ("Return [Esc / F1]".into(), Action::Help),
        ("Main menu".into(), Action::Main),
        ("Export database".into(), Action::ExportDatabase),
    ];
    let cols = if l.width > 700. { 3 } else { 2 };
    let row = if l.height < 600. { 35. } else { 43. };
    let start = if l.height < 600. { 50. } else { 65. };
    let w = (l.width - 2. * l.margin - 12. * (cols - 1) as f32) / cols as f32;
    let session = state.screen == Screen::Session;
    let song_mode = state.mode == Mode::Play;
    for (i, (label, action)) in items
        .into_iter()
        .filter(|(_, action)| {
            (song_mode || !matches!(action, Action::Restart | Action::Back | Action::Forward))
                && (session
                    || !matches!(
                        action,
                        Action::Play
                            | Action::Restart
                            | Action::Back
                            | Action::Forward
                            | Action::Main
                    ))
        })
        .enumerate()
    {
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
    d.text(
        l.margin,
        l.height - 14.,
        &state.storage_status,
        10.,
        t.muted,
    );
    let y = start + (20_usize.div_ceil(cols)) as f32 * row + 15.;
    let line = ((l.height - y - 16.) / 8.).clamp(10., 23.);
    for (i, text) in [
        "Space: play / pause",
        "Left / Right: seek 5 seconds (Shift: 15)",
        "Up / Down: volume (5% per step)",
        "Home: beginning    End: end",
        "A W S E D F T G Y H U J K: C4 to C5",
        "MIDI reading views use approximate notation.",
        "First light: original MusicXML piano study",
        "CC0 1.0. Music credits opens the provenance.",
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
        if state.mode == Mode::Play
            && n.end_us > state.transport.position_us
            && !state.song.parts[n.part].percussion
        {
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
    // Real clefs as subtle, upright watermarks inside each rotated staff band.
    for (key, glyph) in [(50, '\u{e062}'), (71, '\u{e050}')] {
        if let Some(x) = l.center(&state.piano, key) {
            let size = (l.unit * 6.).clamp(48., 100.);
            d.glyph(
                x - size * 0.35,
                (l.top + l.strike) * 0.5,
                glyph,
                size,
                t.ink.with_alpha(0.085),
            );
        }
    }
}
fn falling(d: &mut Drawing, state: &State, l: &Layout, t: theme::Theme) {
    if matches!(state.view, View::Staff | View::Notes) {
        staff(d, state, l, t);
        super::score::falling_structure(d, state, l, t);
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
