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
    /// A large, single-action navigation target for menu screens.
    pub(super) fn menu_card(
        &mut self,
        state: &mut State,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        title: &str,
        subtitle: &str,
        action: Action,
        primary: bool,
        t: theme::Theme,
    ) {
        self.rect(
            x,
            y,
            w,
            h,
            if primary {
                t.colors[0].with_alpha(0.18)
            } else {
                t.panel
            },
            10.,
        );
        if primary {
            self.rect(x, y + 4., 3., h - 8., t.colors[0], 2.);
        }
        self.text(
            x + 16.,
            y + if h < 52. { 12. } else { 10. },
            title,
            16.,
            t.ink,
        );
        if h >= 52. && !subtitle.is_empty() {
            self.text(x + 16., y + 34., subtitle, 11., t.muted);
        }
        state.buttons.push((Rect { x, y, w, h }, action));
    }
    /// A larger two-column mode tile for the desktop home screen.
    pub(super) fn menu_tile(
        &mut self,
        state: &mut State,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        title: &str,
        subtitle: &str,
        action: Action,
        t: theme::Theme,
    ) {
        self.rect(x, y, w, h, t.panel, 12.);
        // Simple original note marker: avoids icons/assets outside Bevy.
        self.rect(x + 20., y + 22., 13., 9., t.colors[0], 8.);
        self.rect(x + 30., y + 10., 3., 20., t.colors[0], 1.);
        self.text(x + 20., y + h - 60., title, 19., t.ink);
        self.text(x + 20., y + h - 33., subtitle, 11., t.muted);
        state.buttons.push((Rect { x, y, w, h }, action));
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
pub(super) fn time(us: u64) -> String {
    let s = us / 1_000_000;
    format!("{}:{:02}", s / 60, s % 60)
}
pub fn render(
    mut commands: Commands,
    mut state: ResMut<State>,
    mut pool: ResMut<Pool>,
    windows: Query<&Window>,
    music_font: Res<MusicFont>,
    mut clear_color: ResMut<ClearColor>,
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
    // Let the Camera2d background show through UI, so native Hanabi particles
    // can be composited underneath note widgets instead of a solid fullscreen node.
    clear_color.0 = t.bg;
    if state.screen != Screen::Session && !state.help {
        if state.screen == Screen::Library {
            super::library::render(&mut d, &mut state, &l, t);
        } else {
            super::menu::render(&mut d, &mut state, &l, t);
        }
    } else if !state.help {
        if state.mode == Mode::Play && state.view == View::Score {
            super::score::render(&mut d, &state, &l, t);
        } else if state.mode == Mode::Play {
            falling(&mut d, &state, &l, t);
        }
        let scale = effects::viewport_scale(l.width, l.height) * state.effect_scale();
        let burst_y = l.strike - 16. * scale;
        let mut ids = BTreeSet::new();
        let mut bursts = vec![];
        for n in state.song.notes.iter().filter(|_| state.mode == Mode::Play) {
            if n.start_us <= state.transport.position_us
                && n.end_us > state.transport.position_us
                && !state.song.parts[n.part].percussion
            {
                ids.insert(n.id);
                if state.animated_effects()
                    && state.transport.playing
                    && !state.previous.contains(&n.id)
                    && let Some(x) = l.center(&state.piano, n.key)
                {
                    // The backing track can sparkle, but cannot increase streak.
                    bursts.push((x, n.id, effects::Preset::FreePlay));
                }
            }
        }
        for event in std::mem::take(&mut state.live_bursts) {
            if state.animated_effects()
                && let Some(x) = l.center(&state.piano, event.key)
            {
                bursts.push((x, event.key as usize, event.preset));
            }
        }
        state.previous = ids;
        for (x, id, preset) in bursts.into_iter().take(32) {
            let [r, g, b] = preset.rgb();
            let color = Color::srgb(r, g, b);
            #[cfg(not(target_arch = "wasm32"))]
            state.gpu_bursts.push(effects::ScreenBurst {
                x,
                y: burst_y,
                preset,
            });
            // A soft, expanding shockwave remains visible across backends.
            state.ripples.push(Ripple {
                x,
                y: burst_y,
                age: 0.,
                scale,
                preset,
            });
            let count = preset.count().min(80);
            let lifetime = preset.lifetime();
            let speed = preset.burst_radius() * scale / lifetime;
            for i in 0..count {
                let jitter = ((id.wrapping_mul(17) + i * 31) % 101) as f32 / 101.;
                let angle = -std::f32::consts::PI
                    * (0.08 + 0.84 * (i as f32 + jitter * 0.45) / count as f32);
                let radial = 0.62 + ((i * 13) % 11) as f32 / 25.;
                let (sin, cos) = angle.sin_cos();
                let mut vx = cos * speed * radial;
                let mut vy = sin * speed * radial - speed * 0.30;
                if preset == effects::Preset::Wrong {
                    vx *= 0.78;
                    vy += speed * 0.5;
                }
                let size = scale * (5.5 + ((i * 7) % 5) as f32);
                state.particles.push(Particle {
                    x,
                    y: burst_y,
                    vx,
                    vy,
                    life: lifetime,
                    duration: lifetime,
                    size,
                    color,
                });
            }
        }
        // Keep the UI fallback cheap even during dense passages on WebGL2.
        if state.particles.len() > 520 {
            let extra = state.particles.len() - 520;
            state.particles.drain(..extra);
        }
        if state.ripples.len() > 24 {
            let extra = state.ripples.len() - 24;
            state.ripples.drain(..extra);
        }
        if state.animated_effects() {
            // Velocity-aligned trails give attacks a crisp direction without
            // moving the camera or the notation. Newest particles retain detail.
            for p in &state.particles {
                let opacity = (p.life / p.duration).clamp(0., 1.);
                let size = p.size * (0.40 + opacity * 0.60);
                let glow = size * 3.2;
                for j in 1..=3 {
                    let lag = j as f32 * 0.014;
                    let trail = size * (1. - j as f32 * 0.18);
                    d.rect(
                        p.x - p.vx * lag - trail * 0.5,
                        p.y - p.vy * lag - trail * 0.5,
                        trail,
                        trail,
                        p.color.with_alpha(opacity * (0.28 - j as f32 * 0.06)),
                        trail,
                    );
                }
                d.rect(
                    p.x - glow * 0.5,
                    p.y - glow * 0.5,
                    glow,
                    glow,
                    p.color.with_alpha(0.18 * opacity * opacity),
                    glow,
                );
                d.rect(
                    p.x - size * 0.5,
                    p.y - size * 0.5,
                    size,
                    size,
                    p.color.with_alpha(0.88 * opacity),
                    size,
                );
                d.rect(
                    p.x - size * 0.16,
                    p.y - size * 0.16,
                    size * 0.32,
                    size * 0.32,
                    Color::WHITE.with_alpha(0.85 * opacity * opacity),
                    size,
                );
            }
            // The ring communicates the hit immediately, even if Hanabi
            // takes a frame to spawn its GPU emitter.
            for ripple in &state.ripples {
                let fraction = (ripple.age / (ripple.preset.lifetime() * 0.8)).clamp(0., 1.);
                let expansion = 1. - (1. - fraction).powi(3);
                let fade = (1. - fraction) * (1. - fraction);
                let radius = (12. + ripple.preset.burst_radius() * expansion * 0.75) * ripple.scale;
                let count = if ripple.preset.count() >= 60 { 24 } else { 16 };
                let [r, g, b] = ripple.preset.rgb();
                let tint = Color::srgb(r, g, b);
                let dot = (7. * ripple.scale * (1. - fraction * 0.55)).max(1.);
                for i in 0..count {
                    let theta = -std::f32::consts::PI * i as f32 / (count - 1) as f32;
                    let x = ripple.x + theta.cos() * radius;
                    let y = ripple.y + theta.sin() * radius;
                    d.rect(
                        x - dot * 0.5,
                        y - dot * 0.5,
                        dot,
                        dot,
                        tint.with_alpha(fade * 0.75),
                        dot,
                    );
                }
                let flash = (16. + 40. * (1. - fraction)) * ripple.scale;
                d.rect(
                    ripple.x - flash * 0.5,
                    ripple.y - flash * 0.5,
                    flash,
                    flash,
                    tint.with_alpha(0.25 * fade),
                    flash,
                );
            }
        }
        keyboard(&mut d, &state, &l, t);
        if state.mode == Mode::Play && state.feedback.streak() >= 2 {
            let streak = state.feedback.streak();
            d.text(
                l.margin,
                14.,
                format!("{streak} note streak · best {}", state.feedback.best()),
                14.,
                t.colors[0],
            );
        }
        if matches!(state.mode, Mode::Notes | Mode::Chords) {
            super::practice_view::render(&mut d, &mut state, &l, t);
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
        if state.screen == Screen::Session {
            // Freeze a faint copy of the music beneath the modal. Only shape
            // geometry is retained so old notation labels cannot float above
            // the pause/settings panel (all Bevy text has a high text Z-index).
            if state.mode == Mode::Play {
                if state.view == View::Score {
                    super::score::render(&mut d, &state, &l, t);
                } else {
                    falling(&mut d, &state, &l, t);
                }
            }
            keyboard(&mut d, &state, &l, t);
            d.labels.clear();
            d.rect(0., 0., l.width, l.height, t.bg.with_alpha(0.82), 0.);
        }
        if state.settings_open {
            super::settings::render(&mut d, &mut state, &l, t);
        } else {
            super::settings::pause(&mut d, &mut state, &l, t);
        }
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
        let mut report = serde_json::json!({"status":state.status,"title":state.song.title,"library_total":state.library.total,"library_items":state.library.items,"library_editing":state.library.edit.is_some(),"library_focus":state.library.focus.map(|f|match f {library::Field::Search=>"search",library::Field::Title=>"title",library::Field::Composer=>"composer"}),"library_query":state.library.query,"library_sort":library::SORTS[state.library.sort],"library_format":library::FORMATS[state.library.format],"library_status":state.library.status,"screen":format!("{:?}",state.screen),"mode":format!("{:?}",state.mode),"completed":state.practice.completed,"preferences":state.preferences(),"storage_status":state.storage_status,"target":state.practice.target(),"position":state.transport.position_us,"playing":state.transport.playing,"volume":state.volume,"view":format!("{:?}",state.view),"staff_lines":if matches!(state.view,View::Staff|View::Notes|View::Score){10}else{0},"notes":state.song.notes.len(),"buttons":button_report,"width":l.width,"height":l.height,"live":state.held.values().copied().collect::<Vec<_>>(),"key_rects":key_rects,"renderer":"bevy-wgpu","help":state.help,"settings_open":state.settings_open,"settings_tab":state.settings_tab.title(),"ending_labels":d.labels.iter().filter(|s|matches!(s.text.as_str(),"1."|"2.")).count(),"music_clefs":d.labels.iter().filter(|s|s.music&&matches!(s.text.as_str(),"\u{e050}"|"\u{e062}")).count(),"written_measures":state.song.notation.as_ref().map_or(0,|s|s.measures.len()),"measure_visits":state.song.notation.as_ref().map_or(0,|s|s.playback.len()),"ending_measures":state.song.notation.as_ref().map_or(0,|s|s.measures.iter().filter(|m|m.ending.is_some()).count())});
        report.as_object_mut().unwrap().extend(serde_json::json!({"practice_played_label":d.labels.iter().find(|s|s.text.starts_with("Played ")).map(|s|s.text.as_str()),"reference_keys":state.reference_keys,"vfx_scale":effects::viewport_scale(l.width,l.height),"vfx_particles":state.particles.len(),"vfx_ripples":state.ripples.len(),"practice_waiting_release":state.practice.waiting_release,"streak":state.feedback.streak(),"best_streak":state.feedback.best(),"effects_backend":if cfg!(target_arch="wasm32") {"webgl2-fallback"} else {"hanabi+ui"},"practice_wrong":state.practice.wrong,"practice_hint":state.practice.hint,"practice_clef":state.practice.clef.name(),"practice_level":state.practice.level.name(),"expected_glow":expected_keys(&state),"solid_keys":state.held.values().copied().collect::<BTreeSet<_>>(),"midi_output":state.midi_output,"midi_output_preferred":state.preferred_midi_output,"midi_outputs":state.midi_outputs.iter().map(|p|serde_json::json!({"id":p.id,"name":p.name})).collect::<Vec<_>>(),"midi_dropdown_open":state.midi_dropdown_open,"midi_dropdown_page":state.midi_dropdown_page}).as_object().unwrap().clone());
        platform::report(&report.to_string());
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
fn expected_keys(state: &State) -> BTreeSet<u8> {
    if matches!(state.mode, Mode::Notes | Mode::Chords) {
        return if state.practice.hint {
            state.practice.target().into_iter().collect()
        } else {
            BTreeSet::new()
        };
    }
    if state.mode != Mode::Play {
        return BTreeSet::new();
    }
    let now = state.transport.position_us;
    state
        .song
        .notes
        .iter()
        .take_while(|n| n.start_us <= now)
        .filter(|n| n.end_us > now && !state.song.parts[n.part].percussion)
        .map(|n| n.key)
        .collect()
}
fn keyboard(d: &mut Drawing, state: &State, l: &Layout, t: theme::Theme) {
    let expected = expected_keys(state);
    let played: BTreeSet<_> = state.held.values().copied().collect();
    for black in [false, true] {
        for k in state.piano.keys.iter().filter(|k| k.black == black) {
            let r = l.key(&state.piano, k.key).unwrap();
            d.rect(
                r.x + 1.,
                r.y + 1.,
                (r.w - 2.).max(1.),
                r.h - 3.,
                if played.contains(&k.key) {
                    t.colors[0]
                } else if black {
                    t.black
                } else {
                    t.white
                },
                3.,
            );
            if expected.contains(&k.key) {
                // Inset feathered edges keep the key body unfilled and avoid
                // painting neighbouring keys. Black keys remain above whites.
                let max = (r.w * 0.22).clamp(1., 5.);
                for i in 0..4 {
                    let inset = 1. + i as f32 * max / 4.;
                    let w = (r.w - 2. * inset).max(1.);
                    let h = (r.h - 3. - 2. * inset).max(1.);
                    let color = t.colors[0].with_alpha(0.8 - i as f32 * 0.17);
                    let edge = (max / 4.).max(0.7);
                    d.rect(r.x + inset, r.y + inset, w, edge, color, 1.);
                    d.rect(r.x + inset, r.y + inset + h - edge, w, edge, color, 1.);
                    d.rect(r.x + inset, r.y + inset, edge, h, color, 1.);
                    d.rect(r.x + inset + w - edge, r.y + inset, edge, h, color, 1.);
                }
            }
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
