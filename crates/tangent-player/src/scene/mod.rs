//! One shared Bevy/wgpu window and UI renderer for native desktop and web.
mod controls;
mod draw;
mod effects;
mod layout;
mod library;
mod menu;
mod passage;
mod personal;
mod piano3d;
mod platform;
mod practice_view;
mod score;
mod session_hud;
mod settings;
mod shortcuts;
mod theme;
mod training;
use super::{import, preferences::Preferences};
use bevy::{prelude::*, window::WindowResolution};
use layout::Layout;
use std::collections::{BTreeMap, BTreeSet};
use tangent_core::{fingerprint, music::Song, piano::PianoLayout, transport::Transport};
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum View {
    Blocks,
    Staff,
    Notes,
    Score,
}
impl View {
    fn name(self) -> &'static str {
        match self {
            Self::Blocks => "Falling blocks",
            Self::Staff => "Staff blocks",
            Self::Notes => "Staff notes",
            Self::Score => "Sheet & playhead",
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Screen {
    Main,
    Train,
    Library,
    Session,
    Progress,
    Results,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mode {
    Play,
    FreeForm,
    Notes,
    Chords,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettingsTab {
    AudioMidi,
    Visuals,
    Practice,
    Effects,
    Accessibility,
    Data,
}
impl SettingsTab {
    const ALL: [Self; 6] = [
        Self::AudioMidi,
        Self::Visuals,
        Self::Practice,
        Self::Effects,
        Self::Accessibility,
        Self::Data,
    ];
    fn title(self) -> &'static str {
        match self {
            Self::AudioMidi => "Audio & MIDI",
            Self::Visuals => "Visuals",
            Self::Practice => "Practice",
            Self::Effects => "Effects",
            Self::Accessibility => "Accessibility",
            Self::Data => "Data & About",
        }
    }
}

#[derive(serde::Deserialize)]
struct MidiPort {
    id: String,
    name: String,
}
#[derive(Resource)]
pub struct State {
    screen: Screen,
    library: library::Library,
    mode: Mode,
    practice: tangent_core::practice::Practice,
    training: training::Sessions,
    reference_remaining: f32,
    reference_keys: Vec<u8>,
    song: Song,
    suggested_hands: BTreeMap<usize, crate::personalization::Hand>,
    piece_id: String,
    transport: Transport,
    loop_a: Option<u64>,
    loop_drag: Option<(String, bool)>,
    goal: Option<passage::Goal>,
    selected_part: Option<usize>,
    piano: PianoLayout,
    view: View,
    hud: u8,
    range_summary: Option<session_hud::RangeSummary>,
    theme: usize,
    personal: personal::Editor,
    effects: bool,
    reduced_motion: bool,
    effect_strength: u8,
    volume: f32,
    instrument: usize,
    hear: bool,
    practice_hear: bool,
    help: bool,
    settings_open: bool,
    settings_tab: SettingsTab,
    shortcuts: shortcuts::Shortcuts,
    practice_defaults: bool,
    status: String,
    midi_outputs: Vec<MidiPort>,
    midi_output: String,
    preferred_midi_output: Option<String>,
    midi_restore_pending: bool,
    midi_dropdown_open: bool,
    midi_dropdown_page: usize,
    clock: f64,
    held: BTreeMap<String, u8>,
    previous: BTreeSet<usize>,
    live_bursts: Vec<effects::Burst>,
    #[cfg(not(target_arch = "wasm32"))]
    gpu_bursts: Vec<effects::ScreenBurst>,
    feedback: effects::Feedback,
    variation: effects::Variation,
    song_hits: tangent_core::practice::SongHits,
    recognition_notes: Vec<(u64, u8)>,
    recognition_status: String,
    recognition_ticket: u64,
    last_recognition_query: u64,
    particles: Vec<Particle>,
    ripples: Vec<Ripple>,
    buttons: Vec<(layout::Rect, controls::Action)>,
    focused_button: Option<usize>,
    viewport_set: bool,
    storage_ready: bool,
    saved_preferences: Option<Preferences>,
    storage_status: String,
}
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    duration: f32,
    size: f32,
    color: Color,
}
struct Ripple {
    x: f32,
    y: f32,
    age: f32,
    scale: f32,
    preset: effects::Preset,
}
impl State {
    fn new() -> Self {
        let song = import::study();
        let duration_us = song.duration_us;
        let suggested_hands = crate::personalization::suggested_hands(&song);
        Self {
            screen: Screen::Main,
            library: library::Library::default(),
            mode: Mode::Play,
            practice: tangent_core::practice::Practice::default(),
            training: training::Sessions::default(),
            reference_remaining: 0.,
            reference_keys: vec![],
            song,
            suggested_hands,
            piece_id: blake3::hash(include_bytes!(
                "../../../../web/piano/samples/first-light.musicxml"
            ))
            .to_hex()
            .to_string(),
            transport: Transport::new(duration_us),
            loop_a: None,
            loop_drag: None,
            goal: None,
            selected_part: None,
            piano: PianoLayout::new(21, 108).unwrap(),
            view: View::Blocks,
            hud: 1,
            range_summary: None,
            theme: 0,
            personal: Default::default(),
            effects: true,
            reduced_motion: false,
            effect_strength: 1,
            volume: 0.45,
            instrument: 0,
            hear: true,
            practice_hear: false,
            help: false,
            settings_open: false,
            settings_tab: SettingsTab::AudioMidi,
            shortcuts: Default::default(),
            practice_defaults: false,
            status: "Tap a key or use Play to enable sound. Bindings: Settings / Accessibility"
                .into(),
            midi_outputs: vec![],
            midi_output: String::new(),
            preferred_midi_output: None,
            midi_restore_pending: false,
            midi_dropdown_open: false,
            midi_dropdown_page: 0,
            clock: platform::clock_us(),
            held: BTreeMap::new(),
            previous: BTreeSet::new(),
            live_bursts: vec![],
            #[cfg(not(target_arch = "wasm32"))]
            gpu_bursts: vec![],
            feedback: effects::Feedback::default(),
            variation: effects::Variation::new(platform::session_nonce()),
            song_hits: tangent_core::practice::SongHits::default(),
            recognition_notes: vec![],
            recognition_status: String::new(),
            recognition_ticket: 0,
            last_recognition_query: 0,
            particles: vec![],
            ripples: vec![],
            buttons: vec![],
            focused_button: None,
            viewport_set: false,
            storage_ready: false,
            saved_preferences: None,
            storage_status: "Opening local storage…".into(),
        }
    }
    fn preferences(&self) -> Preferences {
        Preferences {
            version: 1,
            personalization: self.personal.saved.clone(),
            bindings: self.shortcuts.config.clone(),
            view: match self.view {
                View::Blocks => 0,
                View::Staff => 1,
                View::Notes => 2,
                View::Score => 3,
            },
            hud: self.hud,
            theme: self.theme,
            effects: self.effects,
            reduced_motion: self.reduced_motion,
            effect_strength: self.effect_strength,
            practice_clef: self.practice.clef as u8,
            practice_level: self.practice.level as u8,
            adaptive_practice: self.training.model.adaptive,
            volume: self.volume,
            instrument: self.instrument,
            hear: self.hear,
            speed_percent: self.transport.rate().percent(),
            practice_hear: self.practice_hear,
            first_key: self.piano.keys.first().unwrap().key,
            last_key: self.piano.keys.last().unwrap().key,
            midi_output_name: self.preferred_midi_output.clone(),
        }
    }
    fn restore_preferences(&mut self, p: Preferences) {
        self.personal.saved = p.personalization;
        self.shortcuts.config = p.bindings;
        self.shortcuts.config.fill_missing();
        self.shortcuts.piano.reset();
        self.view = [View::Blocks, View::Staff, View::Notes, View::Score][p.view as usize];
        self.hud = p.hud;
        self.theme = p.theme;
        self.effects = p.effects;
        self.reduced_motion = p.reduced_motion;
        self.effect_strength = p.effect_strength;
        self.training.model.adaptive = p.adaptive_practice;
        use tangent_core::practice::{Level, ReadingClef};
        self.practice.configure(
            [ReadingClef::Treble, ReadingClef::Bass, ReadingClef::Mixed][p.practice_clef as usize],
            [Level::Stepwise, Level::Mixed, Level::Chromatic][p.practice_level as usize],
        );
        self.volume = p.volume;
        self.instrument = p.instrument;
        self.hear = p.hear;
        self.transport.set_rate(
            tangent_core::transport::PlaybackRate::new(p.speed_percent).unwrap_or_default(),
        );
        self.practice_hear = p.practice_hear;
        self.preferred_midi_output = p.midi_output_name;
        self.midi_restore_pending = self.preferred_midi_output.is_some();
        self.piano = PianoLayout::new(p.first_key, p.last_key).unwrap();
        self.viewport_set = true;
        self.sync();
        self.restore_midi_output();
    }
    /// Only a unique, currently reported name can be restored. Native port IDs
    /// are transient scan indices, never suitable for persistent preferences.
    fn restore_midi_output(&mut self) {
        if !self.midi_restore_pending {
            return;
        }
        let Some(name) = self.preferred_midi_output.as_ref() else {
            self.midi_restore_pending = false;
            return;
        };
        let matches: Vec<_> = self
            .midi_outputs
            .iter()
            .filter(|p| &p.name == name)
            .collect();
        if matches.len() > 1 {
            self.midi_restore_pending = false;
            self.status = format!("Multiple MIDI outputs named {name}; select one manually.");
        } else if let Some(port) = matches.first() {
            let id = port.id.clone();
            self.midi_restore_pending = false;
            self.panic();
            platform::action("midi-output", &id);
        }
        // Zero matches: retain the preference and let a future Rescan retry.
    }
    fn sync(&self) {
        platform::action("transport",&serde_json::json!({"position":self.transport.position_us,"playing":self.transport.playing && self.mode==Mode::Play && self.screen==Screen::Session && !self.help,"volume":self.volume,"hear":self.hear,"speed_percent":self.transport.rate().percent(),"loop_end_us":self.transport.passage().map(|p| p.end_us),"instrument":self.instrument}).to_string());
    }
    fn source(&mut self, id: String, key: Option<u8>, velocity: u8) {
        if self.held.get(&id).copied() == key {
            return;
        }
        if self.held.remove(&id).is_some() {
            platform::action("off", &id);
            if self.held.is_empty() {
                let answered = self.practice.waiting_release;
                self.practice.answer(&[]);
                if answered {
                    training::released(self);
                }
            } else if key.is_none() && matches!(self.mode, Mode::Notes | Mode::Chords) {
                let held: Vec<_> = self.held.values().copied().collect();
                if self.practice.answer(&held) {
                    training::grade(self, effects::Grade::Correct);
                    let preset = self.feedback.register(effects::Grade::Correct);
                    self.live_bursts.push(effects::Burst {
                        key: held[0],
                        preset,
                    });
                }
            }
        }
        if let Some(key) = key {
            if self.screen != Screen::Session || self.help {
                return;
            }
            self.stop_reference();
            self.held.insert(id.clone(), key);
            personal::capture(self, key);
            self.capture_for_recognition(key);
            if self.mode == Mode::Play
                && self.transport.playing
                && let Some(run) = &mut self.training.model.run
            {
                let category = if id.starts_with("midi:") {
                    "midi"
                } else if id.starts_with("key:") || id.starts_with("binding:") {
                    "computer"
                } else if id.starts_with("touch:") {
                    "touch"
                } else {
                    "pointer"
                };
                if let Some(sources) = &mut run.input_sources
                    && !sources.iter().any(|s| s == category)
                {
                    sources.push(category.into());
                    sources.sort();
                    run.revision += 1;
                }
            }
            // Never judge backing-track playback as a player hit. A partial
            // chord is not wrong merely because the other fingers are pending.
            let selection = passage::context(self);
            let grade = match self.mode {
                Mode::Play if self.transport.playing => Some(self.song_hits.judge_selection(
                    &self.song,
                    self.transport.position_us,
                    key,
                    Some(&selection),
                )),
                Mode::Play | Mode::FreeForm => Some(effects::Grade::Neutral),
                Mode::Notes | Mode::Chords => {
                    let target_member = effects::belongs_to_target(&self.practice, key);
                    let held: Vec<_> = self.held.values().copied().collect();
                    if self.practice.answer(&held) {
                        Some(effects::Grade::Correct)
                    } else if !self.practice.waiting_release && !target_member {
                        Some(effects::Grade::Wrong)
                    } else {
                        None
                    }
                }
            };
            if let Some(grade) = grade {
                training::grade(self, grade);
                let preset = self.feedback.register(grade);
                self.live_bursts.push(effects::Burst { key, preset });
            }
            platform::action(
                "on",
                &serde_json::json!({"id":id,"key":key,"velocity":velocity}).to_string(),
            );
        }
    }
    /// Feed a rolling monophonic MIDI query. Simultaneous chord onsets within
    /// 40 ms contribute their highest key. The SQL shortlist is asynchronous.
    fn capture_for_recognition(&mut self, key: u8) {
        if self.mode != Mode::FreeForm {
            return;
        }
        let now = platform::clock_us().max(0.) as u64;
        if let Some((at, top)) = self.recognition_notes.last_mut() {
            if now.saturating_sub(*at) <= 40_000 {
                *top = (*top).max(key);
            } else {
                self.recognition_notes.push((now, key));
            }
        } else {
            self.recognition_notes.push((now, key));
        }
        if self.recognition_notes.len() > 24 {
            self.recognition_notes.remove(0);
        }
        if self.recognition_notes.len() < fingerprint::MIN_QUERY_NOTES
            || now.saturating_sub(self.last_recognition_query) < 250_000
        {
            return;
        }
        self.last_recognition_query = now;
        self.recognition_ticket += 1;
        let keys: Vec<u8> = self.recognition_notes.iter().map(|(_, key)| *key).collect();
        let request = serde_json::json!({
            "ticket": self.recognition_ticket,
            "keys": keys,
            "signatures": fingerprint::signatures(&keys),
        });
        platform::action("fingerprint-query", &request.to_string());
    }
    fn animated_effects(&self) -> bool {
        self.effects && !self.reduced_motion
    }
    fn effect_scale(&self) -> f32 {
        [0.65, 1.0, 1.35][self.effect_strength as usize]
    }
    fn stop_reference(&mut self) {
        for key in self.reference_keys.drain(..) {
            platform::action("off", &format!("reference:{key}"));
        }
        self.reference_remaining = 0.;
    }
    fn hear_target(&mut self) {
        self.stop_reference();
        if !matches!(self.mode, Mode::Notes | Mode::Chords) || self.help {
            return;
        }
        if !self.practice.waiting_release {
            self.training.model.assist();
        }
        self.reference_keys = self.practice.target();
        self.reference_remaining = 0.7;
        for &key in &self.reference_keys {
            platform::action(
                "on",
                &serde_json::json!({"id":format!("reference:{key}"),"key":key,"velocity":85})
                    .to_string(),
            );
        }
    }
    fn panic(&mut self) {
        self.personal.teaching = None;
        self.shortcuts.piano.reset();
        self.shortcuts.capture = None;
        self.stop_reference();
        self.held.clear();
        self.practice.answer(&[]);
        self.live_bursts.clear();
        self.ripples.clear();
        self.particles.clear();
        #[cfg(not(target_arch = "wasm32"))]
        self.gpu_bursts.clear();
        self.feedback.reset();

        self.recognition_notes.clear();
        self.recognition_status.clear();
        self.recognition_ticket += 1;
        self.transport.playing = false;
        platform::action("panic", "");
        self.sync();
    }
}
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    let state = State::new();
    #[cfg(not(target_arch = "wasm32"))]
    let state = {
        let mut state = State::new();
        state.status = "Native MIDI/audio ready; open Settings to configure input".into();
        state
    };
    platform::action("song", &serde_json::to_string(&state.song).unwrap());
    platform::action(
        "fingerprint-index",
        &serde_json::to_string(&fingerprint::index_song(&state.song)).unwrap(),
    );
    let mut app = App::new();
    app.insert_resource(state)
        .insert_resource(draw::Pool::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tangent".into(),
                #[cfg(target_arch = "wasm32")]
                canvas: Some("#tangent".into()),
                #[cfg(target_arch = "wasm32")]
                fit_canvas_to_parent: true,
                resolution: WindowResolution::new(1280, 800),
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.03, 0.06, 0.08)))
        .add_systems(
            Startup,
            |mut commands: Commands, mut fonts: ResMut<Assets<Font>>| {
                commands.spawn((Camera2d, IsDefaultUiCamera));
                // Ship full UI typography with native and WASM builds.
                commands.insert_resource(draw::UiFont(fonts.add(Font::from_bytes(
                    include_bytes!("../../assets/NotoSans-Regular.ttf").to_vec(),
                ))));
                commands.insert_resource(draw::MusicFont(fonts.add(Font::from_bytes(
                    include_bytes!("../../assets/Bravura.otf").to_vec(),
                ))));
            },
        )
        .add_systems(Update, (update, controls::input, draw::render).chain());
    piano3d::install(&mut app);
    #[cfg(not(target_arch = "wasm32"))]
    effects::gpu::install(&mut app);
    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, platform::native_window)
        .add_systems(Last, training::on_exit);
    app.run();
    #[cfg(not(target_arch = "wasm32"))]
    platform::shutdown();
}
fn update(mut state: ResMut<State>, time: Res<Time>, windows: Query<&Window>) {
    let now = platform::clock_us();
    let elapsed = (now - state.clock).max(0.) as u64;
    state.clock = now;
    let messages: Vec<serde_json::Value> =
        serde_json::from_str(&platform::pull()).unwrap_or_default();
    // Hidden-tab time must not advance the song before the queued pause arrives.
    if messages.iter().any(|m| m["type"] == "pause") {
        state.panic();
    }
    let was = state.transport.playing;
    let wraps = state.transport.advance(elapsed);
    if wraps > 0 {
        state.personal.teaching = None;
        let next = state.transport.position_us;
        if let Some(p) = state.transport.passage() {
            state.transport.position_us = p.end_us;
            training::end(&mut state, false);
            state.transport.position_us = next;
            training::start_song(&mut state);
            state.previous.clear();
            state.feedback.reset();
            state.sync();
        }
    }
    if was && !state.transport.playing {
        if state.mode == Mode::Play && state.training.model.run.is_some() {
            training::end(&mut state, true);
        }
        state.sync();
    }
    let edges: Vec<_> = messages
        .iter()
        .map(|m| {
            (
                m["key"].as_u64().filter(|k| *k < 128).unwrap_or(128) as u8,
                m["type"] == "note" && m["down"] == true,
            )
        })
        .collect();
    let order = if state.shortcuts.config.piano_enabled && state.shortcuts.capture.is_none() {
        tangent_core::input_bindings::batch_order(&edges, state.shortcuts.config.control)
    } else {
        (0..messages.len()).collect()
    };
    for index in order {
        let m = &messages[index];
        if training::message(&mut state, m) {
            continue;
        }
        match m["type"].as_str().unwrap_or("") {
            "theme-file" => {
                personal::import(&mut state, m["text"].as_str().unwrap_or("").as_bytes());
            }
            "midi-outputs" => {
                state.midi_outputs = serde_json::from_value(m["ports"].clone()).unwrap_or_default();
                state.midi_output = m["selected"].as_str().unwrap_or("").into();
                state.restore_midi_output();
            }
            "preferences" => {
                if !m["data"].is_null() {
                    match serde_json::from_value::<Preferences>(m["data"].clone()) {
                        Ok(p) if p.validate().is_ok() => state.restore_preferences(p),
                        _ => {
                            state.storage_status =
                                "Saved settings are incompatible; database preserved.".into();
                            continue;
                        }
                    }
                }
                state.saved_preferences = Some(state.preferences());
                state.storage_ready = true;
                platform::action("progress-list", "");
            }
            "storage-status" => state.storage_status = m["text"].as_str().unwrap_or("").into(),
            "file" => {
                let bytes: Vec<u8> = m["bytes"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_u64().and_then(|n| u8::try_from(n).ok()))
                            .collect()
                    })
                    .unwrap_or_default();
                let ok = load_song(
                    &mut state,
                    &bytes,
                    m["title"].as_str().unwrap_or("MIDI").into(),
                    m["from_library"].as_bool().unwrap_or(false),
                );
                platform::action(
                    "import-result",
                    &serde_json::json!({"ticket":m["import_ticket"],"ok":ok}).to_string(),
                );
            }
            "library-list" => {
                if m["ticket"].as_u64() == Some(state.library.ticket) {
                    state.library.items =
                        serde_json::from_value(m["items"].clone()).unwrap_or_default();
                    state.library.total = m["total"].as_u64().unwrap_or(0) as usize;
                    if state.library.offset > 0 && state.library.items.is_empty() {
                        state.library.offset =
                            state.library.offset.saturating_sub(state.library.page_size);
                        state.library.refresh();
                    }
                }
            }
            "library-status" => {
                state.library.status = m["text"].as_str().unwrap_or("").into();
                state.status = state.library.status.clone();
            }
            "library-updated" => {
                if m["metadata"].as_bool() == Some(true) {
                    state.library.edit = None;
                    state.library.focus = None;
                }
                state.library.status = m["text"].as_str().unwrap_or("").into();
                state.library.refresh();
            }
            "fingerprint-candidates" => {
                if state.mode != Mode::FreeForm
                    || state.screen != Screen::Session
                    || m["ticket"].as_u64() != Some(state.recognition_ticket)
                {
                    continue;
                }
                let keys: Vec<u8> = m["keys"]
                    .as_array()
                    .map(|values| {
                        values
                            .iter()
                            .filter_map(|v| v.as_u64().and_then(|k| u8::try_from(k).ok()))
                            .collect()
                    })
                    .unwrap_or_default();
                let candidates: Vec<fingerprint::Candidate> =
                    serde_json::from_value(m["candidates"].clone()).unwrap_or_default();
                let matches = fingerprint::rank_matches(&keys, &candidates, 2);
                state.recognition_status.clear();
                if let Some(best) = matches.first() {
                    let ambiguous = matches.get(1).is_some_and(|next| {
                        (best.similarity - next.similarity) < 0.08 && best.song_id != next.song_id
                    });
                    state.recognition_status = format!(
                        "{}: {} · {:.0}% interval similarity · note {}",
                        if ambiguous {
                            "Possible match"
                        } else {
                            "Recognized"
                        },
                        best.title,
                        best.similarity * 100.,
                        best.note_index + 1
                    );
                }
            }
            "note" => {
                let id = m["id"].as_str().unwrap_or("midi").to_string();
                if let Some(key) = m["key"].as_u64().filter(|k| *k < 128) {
                    let down = m["down"].as_bool() == Some(true);
                    if windows.single().is_ok_and(|w| w.focused)
                        && !shortcuts::midi(&mut state, &id, key as u8, down, now as u64)
                    {
                        state.source(
                            id,
                            down.then_some(key as u8),
                            m["velocity"].as_u64().unwrap_or(90) as u8,
                        );
                    }
                }
            }
            "pause" => state.panic(),
            "status" => {
                state.status = m["text"].as_str().unwrap_or("").into();
                if state.personal.open != 0 {
                    state.personal.status = state.status.clone();
                }
            }
            _ => {}
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(file) = platform::take_file() {
        load_song(&mut state, &file.bytes, file.title, file.from_library);
    }
    if state.storage_ready {
        let p = state.preferences();
        if state.saved_preferences.as_ref() != Some(&p) {
            platform::action("save-settings", &serde_json::to_string(&p).unwrap());
            state.saved_preferences = Some(p);
        }
    }
    training::tick(
        &mut state,
        time.delta_secs().min(0.25),
        windows.single().is_ok_and(|w| w.focused),
    );
    let dt = time.delta_secs().min(0.05);
    if state.reference_remaining > 0. {
        state.reference_remaining -= time.delta_secs();
        if state.reference_remaining <= 0. {
            state.stop_reference();
        }
    }
    state.particles.retain(|p| p.life > 0.);
    state.ripples.retain(|r| r.age < r.preset.lifetime() * 0.8);
    for ripple in &mut state.ripples {
        ripple.age += dt;
    }
    for p in &mut state.particles {
        p.life -= dt;
        p.x += p.vx * dt;
        p.y += p.vy * dt;
        p.vy += 65. * dt;
    }
}

fn load_song(state: &mut State, bytes: &[u8], title: String, from_library: bool) -> bool {
    match import::song(bytes, title.clone()) {
        Ok(mut song) => {
            state.personal.open = 0;
            state.personal.hand_draft = None;
            state.personal.draft = None;
            let metadata = crate::library::ImportMetadata::from_song(&song, bytes);
            platform::store_import(bytes, &serde_json::to_string(&metadata).unwrap());
            if from_library {
                song.title = title;
            }
            if state.mode == Mode::Play {
                training::end(state, false);
            }
            state.panic();
            let rate = state.transport.rate();
            state.transport = Transport::new(song.duration_us);
            state.loop_a = None;
            state.loop_drag = None;
            state.goal = None;
            state.selected_part = None;
            state.transport.set_rate(rate);
            state.status = format!(
                "{} notes loaded. {}",
                song.notes.len(),
                song.warnings.join(" ")
            );
            state.piece_id = blake3::hash(bytes).to_hex().to_string();
            state.suggested_hands = crate::personalization::suggested_hands(&song);
            state.song = song;
            platform::action("song", &serde_json::to_string(&state.song).unwrap());
            state.previous.clear();
            state.particles.clear();
            state.ripples.clear();
            #[cfg(not(target_arch = "wasm32"))]
            state.gpu_bursts.clear();
            state.feedback.reset();
            state.sync();
            true
        }
        Err(e) => {
            state.library.status = e.clone();
            state.status = e;
            false
        }
    }
}
