//! Physical output and a clocked event cursor; no Bevy or audio device required.
use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::mpsc::Sender,
};

#[derive(Clone, Deserialize)]
pub(super) struct Event {
    pub time_us: u64,
    channel: u8,
    message: Message,
}
#[derive(Clone, Deserialize)]
enum Message {
    On { key: u8, velocity: u8 },
    Off { key: u8 },
    Control { controller: u8, value: u8 },
    Bend { value: u16 },
    Program { program: u8 },
}
impl Event {
    fn bytes(&self) -> Option<Vec<u8>> {
        if self.channel > 15 || self.channel == 9 {
            return None;
        }
        let ch = self.channel;
        Some(match self.message {
            Message::On { key, velocity } if key < 128 && velocity < 128 => {
                vec![0x90 | ch, key, velocity]
            }
            Message::Off { key } if key < 128 => vec![0x80 | ch, key, 0],
            Message::Control { controller, value } if controller < 128 && value < 128 => {
                vec![0xb0 | ch, controller, value]
            }
            Message::Bend { value } if value < 16384 => {
                vec![0xe0 | ch, (value & 127) as u8, (value >> 7) as u8]
            }
            Message::Program { program } if program < 128 => vec![0xc0 | ch, program],
            _ => return None,
        })
    }
}
/// Restore channel state and notes at a seek, without replaying historical attacks.
pub(super) fn restore(events: &[Event], position: u64) -> Vec<Vec<u8>> {
    restore_at(events, position, false)
}
fn restore_at(events: &[Event], position: u64, boundary: bool) -> Vec<Vec<u8>> {
    let mut controls = BTreeMap::new();
    let mut keys: BTreeMap<(u8, u8), Vec<u8>> = BTreeMap::new();
    let mut pedal = [false; 16];
    let mut sustained: BTreeMap<(u8, u8), Vec<u8>> = BTreeMap::new();
    for e in events
        .iter()
        .take_while(|e| e.time_us < position || boundary && e.time_us == position)
    {
        if boundary && e.time_us == position && matches!(e.message, Message::On { .. }) {
            continue;
        }
        let Some(bytes) = e.bytes() else {
            continue;
        };
        match e.message {
            Message::On { key, velocity } if velocity > 0 => {
                keys.entry((e.channel, key)).or_default().push(velocity)
            }
            Message::Off { key } | Message::On { key, velocity: 0 } => {
                if let Some(notes) = keys.get_mut(&(e.channel, key))
                    && !notes.is_empty()
                {
                    let v = notes.remove(0);
                    if pedal[e.channel as usize] {
                        sustained.entry((e.channel, key)).or_default().push(v);
                    }
                }
            }
            Message::Control { controller, value } => {
                if controller == 64 {
                    pedal[e.channel as usize] = value >= 64;
                    if value < 64 {
                        sustained.retain(|(ch, _), _| *ch != e.channel);
                    }
                }
                if matches!(controller, 120 | 123) {
                    if controller == 123 && pedal[e.channel as usize] {
                        for (&k, v) in keys.iter().filter(|((ch, _), _)| *ch == e.channel) {
                            sustained.entry(k).or_default().extend(v);
                        }
                    }
                    keys.retain(|(ch, _), _| *ch != e.channel);
                    if controller == 120 {
                        sustained.retain(|(ch, _), _| *ch != e.channel);
                    }
                }
                if controller == 121 {
                    controls.retain(|(ch, _), _| *ch != e.channel);
                    pedal[e.channel as usize] = false;
                    sustained.retain(|(ch, _), _| *ch != e.channel);
                } else if !matches!(controller, 120 | 123) {
                    controls.insert((e.channel, controller as u16), bytes);
                }
            }
            Message::Bend { .. } => {
                controls.insert((e.channel, 256), bytes);
            }
            Message::Program { .. } => {
                controls.insert((e.channel, 257), bytes);
            }
            _ => {}
        }
    }
    let mut result: Vec<_> = controls.into_values().collect();
    for ((ch, key), values) in sustained {
        for velocity in values {
            result.push(vec![0x90 | ch, key, velocity]);
            result.push(vec![0x80 | ch, key, 0]);
        }
    }
    for ((ch, key), values) in keys {
        for velocity in values {
            result.push(vec![0x90 | ch, key, velocity]);
        }
    }
    result
}
pub(super) struct Output {
    ports: Vec<(String, String, MidiOutputPort)>,
    connection: Option<MidiOutputConnection>,
    selected: String,
    out: Sender<Value>,
    events: Vec<Event>,
    next: usize,
    live: BTreeMap<String, u8>,
    epoch: u64,
    boundary: Option<u64>,
    song_keys: BTreeSet<u8>,
}
impl Output {
    pub fn new(out: Sender<Value>) -> Self {
        let mut s = Self {
            ports: vec![],
            connection: None,
            selected: String::new(),
            out,
            events: vec![],
            next: 0,
            live: BTreeMap::new(),
            epoch: 0,
            boundary: None,
            song_keys: BTreeSet::new(),
        };
        s.scan();
        s
    }
    pub fn active(&self) -> bool {
        self.connection.is_some()
    }
    fn report(&self) {
        let ports: Vec<_> = self
            .ports
            .iter()
            .map(|(id, name, _)| json!({"id":id,"name":name}))
            .collect();
        let _ = self
            .out
            .send(json!({"type":"midi-outputs","ports":ports,"selected":self.selected}));
    }
    pub fn scan(&mut self) {
        self.select("");
        self.epoch += 1;
        self.ports.clear();
        if let Ok(midi) = MidiOutput::new("Tangent output scan") {
            for (i, port) in midi.ports().into_iter().enumerate() {
                let name = midi
                    .port_name(&port)
                    .unwrap_or_else(|_| format!("MIDI output {}", i + 1));
                self.ports.push((format!("{}:{i}", self.epoch), name, port));
            }
        }
        self.report();
    }
    pub fn select(&mut self, id: &str) {
        self.panic();
        self.connection = None;
        self.selected.clear();
        if !id.is_empty() {
            let result = (|| {
                let (_, _, port) = self
                    .ports
                    .iter()
                    .find(|(pid, _, _)| pid == id)
                    .ok_or("Output changed; rescan MIDI".to_string())?;
                let midi = MidiOutput::new("Tangent").map_err(|e| e.to_string())?;
                midi.connect(port, "Tangent piano output")
                    .map_err(|e| e.to_string())
            })();
            match result {
                Ok(connection) => {
                    self.connection = Some(connection);
                    self.selected = id.into();
                }
                Err(e) => {
                    let _ = self.out.send(
                        json!({"type":"status","text":format!("MIDI output unavailable: {e}")}),
                    );
                }
            }
        }
        self.report();
    }
    fn send(&mut self, bytes: &[u8]) {
        if let Some(connection) = self.connection.as_mut()
            && let Err(e) = connection.send(bytes)
        {
            let _=self.out.send(json!({"type":"status","text":format!("MIDI output disconnected: {e}. Playback paused; rescan MIDI.")}));
            let _ = self.out.send(json!({"type":"pause"}));
            self.connection = None;
            self.selected.clear();
            self.live.clear();
            self.report();
        }
    }
    pub fn panic(&mut self) {
        self.silence(false);
    }
    fn silence(&mut self, preserve: bool) {
        for ch in 0..16 {
            if ch == 9 {
                continue;
            }
            self.send(&[0xb0 | ch, 64, 0]);
            if preserve && ch == 15 {
                for key in self.song_keys.iter().copied().collect::<Vec<_>>() {
                    if !self.live.values().any(|k| *k == key) {
                        self.send(&[0x8f, key, 0]);
                    }
                }
            } else {
                self.send(&[0xb0 | ch, 123, 0]);
                self.send(&[0xb0 | ch, 120, 0]);
            }
            self.send(&[0xe0 | ch, 0, 64]);
            self.send(&[0xb0 | ch, 121, 0]);
        }
        self.song_keys.clear();
        if !preserve {
            self.live.clear();
        }
    }
    pub fn song(&mut self, value: &str) {
        self.panic();
        self.events = serde_json::from_str::<Value>(value)
            .ok()
            .and_then(|s| serde_json::from_value(s["events"].clone()).ok())
            .unwrap_or_default();
    }
    pub fn reset(&mut self, position: u64, playing: bool, guided: bool) {
        self.silence(guided);
        self.boundary = guided.then_some(position);
        self.next = self.events.partition_point(|e| e.time_us < position);
        if playing && self.active() {
            let restored = if guided {
                restore_at(&self.events, position, true)
            } else {
                restore(&self.events, position)
            };
            for bytes in restored {
                if bytes.first() == Some(&0x9f) && bytes.get(2).is_some_and(|v| *v > 0) {
                    self.song_keys.insert(bytes[1]);
                }
                self.send(&bytes);
            }
        }
    }
    pub fn tick(&mut self, position: u64) {
        while let Some(e) = self.events.get(self.next) {
            if e.time_us > position {
                break;
            }
            let bytes =
                if self.boundary == Some(e.time_us) && !matches!(e.message, Message::On { .. }) {
                    None
                } else {
                    e.bytes()
                };
            self.next += 1;
            if let Some(bytes) = bytes {
                if bytes.first() == Some(&0x9f) && bytes.get(2).is_some_and(|v| *v > 0) {
                    self.song_keys.insert(bytes[1]);
                }
                self.send(&bytes);
            }
        }
    }
    pub fn on(&mut self, id: &str, key: u8, velocity: u8) {
        // Pianos already sound their own keys with Local Control On. Never echo
        // hardware input back into its output (or synthesise it a second time).
        if id.starts_with("midi:") || key > 127 {
            return;
        }
        self.off(id);
        // Channel 16 isolates computer/touch voices from ordinary piano scores.
        self.send(&[0x9f, key, velocity.min(127)]);
        self.live.insert(id.into(), key);
    }
    pub fn off(&mut self, id: &str) {
        if let Some(key) = self.live.remove(id)
            && !self.live.values().any(|k| *k == key)
        {
            self.send(&[0x8f, key, 0]);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn events(s: &str) -> Vec<Event> {
        serde_json::from_str(s).unwrap()
    }
    #[test]
    fn midi_encoding_bounds_channels_and_bend() {
        let e = events(
            r#"[{"time_us":0,"channel":2,"message":{"Bend":{"value":16383}}},{"time_us":0,"channel":9,"message":{"On":{"key":60,"velocity":90}}}]"#,
        );
        assert_eq!(e[0].bytes(), Some(vec![0xe2, 127, 127]));
        assert_eq!(e[1].bytes(), None);
    }
    #[test]
    fn seek_restores_pedal_held_note_without_replaying_old_attacks() {
        let e = events(
            r#"[
          {"time_us":0,"channel":0,"message":{"Control":{"controller":64,"value":127}}},
          {"time_us":10,"channel":0,"message":{"On":{"key":60,"velocity":80}}},
          {"time_us":20,"channel":0,"message":{"Off":{"key":60}}},
          {"time_us":30,"channel":0,"message":{"Control":{"controller":64,"value":0}}}]
        "#,
        );
        assert_eq!(
            restore(&e, 25),
            vec![vec![0xb0, 64, 127], vec![0x90, 60, 80], vec![0x80, 60, 0]]
        );
        assert_eq!(restore(&e, 31), vec![vec![0xb0, 64, 0]]);
    }
}

#[cfg(test)]
mod guided_tests {
    use super::*;
    #[test]
    fn gate_snapshot_consumes_releases_and_leaves_attacks_pending() {
        let events: Vec<Event> = serde_json::from_str(
            r#"[
          {"time_us":0,"channel":0,"message":{"On":{"key":60,"velocity":90}}},
          {"time_us":500,"channel":0,"message":{"Off":{"key":60}}},
          {"time_us":500,"channel":0,"message":{"On":{"key":62,"velocity":90}}}]
        "#,
        )
        .unwrap();
        assert!(restore_at(&events, 500, true).is_empty());
        assert_eq!(restore(&events, 500), vec![vec![0x90, 60, 90]]);
    }
    #[test]
    fn guided_resets_keep_live_voice_identity() {
        let (out, _) = std::sync::mpsc::channel();
        let mut output = Output {
            ports: vec![],
            connection: None,
            selected: String::new(),
            out,
            events: vec![],
            next: 0,
            live: BTreeMap::new(),
            epoch: 0,
            boundary: None,
            song_keys: BTreeSet::new(),
        };
        output.on("key:test", 64, 90);
        output.song_keys.insert(67);
        output.reset(500, false, true);
        assert_eq!(output.live.get("key:test"), Some(&64));
        assert!(output.song_keys.is_empty());
        output.off("key:test");
        assert!(output.live.is_empty());
    }
}
