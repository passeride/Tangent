//! One Bevy window, camera and UI renderer. Browser code is a capability adapter.
mod controls;
mod draw;
mod layout;
mod platform;
mod theme;
use super::import;
use bevy::{prelude::*, window::WindowResolution};
use layout::Layout;
use std::collections::{BTreeMap, BTreeSet};
use tangent_core::{music::Song, piano::PianoLayout, transport::Transport};
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
#[derive(Resource)]
pub struct State {
    song: Song,
    transport: Transport,
    piano: PianoLayout,
    view: View,
    theme: usize,
    effects: bool,
    volume: f32,
    instrument: usize,
    hear: bool,
    help: bool,
    status: String,
    clock: f64,
    held: BTreeMap<String, u8>,
    previous: BTreeSet<usize>,
    live_bursts: Vec<u8>,
    particles: Vec<Particle>,
    buttons: Vec<(layout::Rect, controls::Action)>,
    arrow_repeat: f32,
    viewport_set: bool,
}
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    color: Color,
}
impl State {
    fn new() -> Self {
        let song = import::study();
        let duration_us = song.duration_us;
        Self {
            song,
            transport: Transport {
                duration_us,
                ..default()
            },
            piano: PianoLayout::new(21, 108).unwrap(),
            view: View::Blocks,
            theme: 0,
            effects: true,
            volume: 0.45,
            instrument: 0,
            hear: true,
            help: false,
            status: "Tap a key or press Space to enable sound. F1: shortcuts".into(),
            clock: platform::clock_us(),
            held: BTreeMap::new(),
            previous: BTreeSet::new(),
            live_bursts: vec![],
            particles: vec![],
            buttons: vec![],
            arrow_repeat: 0.,
            viewport_set: false,
        }
    }
    fn sync(&self) {
        platform::action("transport",&serde_json::json!({"position":self.transport.position_us,"playing":self.transport.playing,"volume":self.volume,"hear":self.hear,"instrument":self.instrument}).to_string());
    }
    fn source(&mut self, id: String, key: Option<u8>, velocity: u8) {
        if self.held.get(&id).copied() == key {
            return;
        }
        if self.held.remove(&id).is_some() {
            platform::action("off", &id);
        }
        if let Some(key) = key {
            self.held.insert(id.clone(), key);
            self.live_bursts.push(key);
            platform::action(
                "on",
                &serde_json::json!({"id":id,"key":key,"velocity":velocity}).to_string(),
            );
        }
    }
    fn panic(&mut self) {
        self.held.clear();
        self.transport.playing = false;
        platform::action("panic", "");
        self.sync();
    }
}
pub fn run() {
    let state = State::new();
    platform::action("song", &serde_json::to_string(&state.song).unwrap());
    App::new()
        .insert_resource(state)
        .insert_resource(draw::Pool::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tangent".into(),
                canvas: Some("#tangent".into()),
                fit_canvas_to_parent: true,
                resolution: WindowResolution::new(1280, 800),
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.03, 0.06, 0.08)))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2d);
        })
        .add_systems(Update, (update, controls::input, draw::render).chain())
        .run();
}
fn update(mut state: ResMut<State>, time: Res<Time>) {
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
    state.transport.advance(elapsed);
    if was && !state.transport.playing {
        state.sync();
    }
    for m in messages {
        match m["type"].as_str().unwrap_or("") {
            "file" => {
                let bytes: Vec<u8> = m["bytes"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .filter_map(|v| v.as_u64().and_then(|n| u8::try_from(n).ok()))
                            .collect()
                    })
                    .unwrap_or_default();
                match import::song(&bytes, m["title"].as_str().unwrap_or("MIDI").into()) {
                    Ok(song) => {
                        state.panic();
                        state.transport = Transport {
                            duration_us: song.duration_us,
                            ..default()
                        };
                        state.status = format!(
                            "{} notes loaded. {}",
                            song.notes.len(),
                            song.warnings.join(" ")
                        );
                        state.song = song;
                        platform::action("song", &serde_json::to_string(&state.song).unwrap());
                        state.previous.clear();
                        state.particles.clear();
                        state.sync();
                    }
                    Err(e) => state.status = e,
                }
            }
            "note" => {
                let id = m["id"].as_str().unwrap_or("midi").to_string();
                let key = if !state.help && m["down"].as_bool() == Some(true) {
                    m["key"].as_u64().filter(|k| *k < 128).map(|k| k as u8)
                } else {
                    None
                };
                state.source(id, key, m["velocity"].as_u64().unwrap_or(90) as u8);
            }
            "pause" => state.panic(),
            "status" => state.status = m["text"].as_str().unwrap_or("").into(),
            _ => {}
        }
    }
    let dt = time.delta_secs().min(0.05);
    state.particles.retain(|p| p.life > 0.);
    for p in &mut state.particles {
        p.life -= dt;
        p.x += p.vx * dt;
        p.y += p.vy * dt;
        p.vy += 65. * dt;
    }
}
