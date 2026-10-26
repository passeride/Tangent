import { instruments } from "./instruments.js";
// Web Audio boundary. Scheduling uses the audio clock, independently of rendering.
export class PianoAudio {
  context = null;
  voices = new Set();
  channels = new Map();
  live = new Map();
  instrument = "felt";
  volume = 0.45;
  async activate() {
    if (!this.context) {
      this.context = new AudioContext({ latencyHint: "interactive" });
      this.master = this.context.createGain();
      this.master.gain.value = this.volume;
      const compressor = this.context.createDynamicsCompressor();
      this.master.connect(compressor);
      compressor.connect(this.context.destination);
      this.analyser = this.context.createAnalyser();
      this.master.connect(this.analyser);
    }
    await this.context.resume();
  }
  get now() {
    return this.context?.currentTime ?? 0;
  }
  setVolume(v) {
    this.volume = v;
    if (this.master) this.master.gain.setTargetAtTime(v, this.now, 0.02);
  }
  voice(key, velocity, when = this.now, channel = null) {
    if (!this.context) return null;
    // Bound dense/untrusted imports, including voices already scheduled for release.
    if (this.voices.size >= 128)
      this.kill(this.voices.values().next().value, this.now, true);
    const ctx = this.context,
      preset = instruments[this.instrument],
      gain = ctx.createGain(),
      filter = ctx.createBiquadFilter();
    filter.type = "lowpass";
    filter.frequency.value = preset.brightness;
    gain.connect(filter);
    filter.connect(channel?.gain ?? this.master);
    const level = 0.15 * (velocity / 127) ** 1.4;
    gain.gain.setValueAtTime(0, when);
    gain.gain.linearRampToValueAtTime(level, when + 0.005);
    gain.gain.exponentialRampToValueAtTime(
      Math.max(0.00001, level * 0.06),
      when + preset.decay,
    );
    const frequency = 440 * 2 ** ((key - 69) / 12),
      oscillators = preset.harmonics.map((amount, i) => {
        const osc = ctx.createOscillator(),
          partial = ctx.createGain();
        osc.type = "sine";
        osc.frequency.value = frequency * (i + 1);
        partial.gain.value = amount;
        osc.connect(partial);
        partial.connect(gain);
        osc.start(when);
        return osc;
      });
    const voice = {
      channel,
      key,
      gain,
      filter,
      oscillators,
      release: preset.release,
      ended: false,
      level,
      when,
    };
    this.voices.add(voice);
    // Finite safety tail even for missing note releases.
    voice.timer = setTimeout(() => this.kill(voice, this.now), 30000);
    return voice;
  }
  kill(v, when = this.now, force = false) {
    if (!v || (v.ended && !force)) return;
    v.ended = true;
    clearTimeout(v.timer);
    if (force) this.voices.delete(v);
    if (v.gain.gain.cancelAndHoldAtTime) v.gain.gain.cancelAndHoldAtTime(when);
    else v.gain.gain.cancelScheduledValues(when);
    v.gain.gain.setTargetAtTime(0.00001, when, v.release / 4);
    for (const osc of v.oscillators) osc.stop(when + v.release);
    v.oscillators[0].onended = () => {
      v.gain.disconnect();
      v.filter.disconnect();
      this.voices.delete(v);
    };
  }
  liveOn(id, key, velocity = 95) {
    this.liveOff(id);
    const v = this.voice(key, velocity);
    if (v) this.live.set(id, v);
  }
  liveOff(id) {
    this.kill(this.live.get(id));
    this.live.delete(id);
  }
  state(port, channel) {
    const id = `${port}:${channel}`;
    if (!this.channels.has(id)) {
      const gain = this.context.createGain();
      gain.connect(this.master);
      this.channels.set(id, {
        keys: new Map(),
        held: [],
        pedal: false,
        volume: 1,
        expression: 1,
        bend: 8192,
        gain,
      });
    }
    return this.channels.get(id);
  }
  event(e, when) {
    const s = this.state(e.port, e.channel),
      m = e.message;
    // Channel 10 is percussion; this piano voice deliberately does not reinterpret it.
    if (e.channel === 9) return;
    if (m.On) {
      const { key, velocity } = m.On,
        v = this.voice(key, velocity, when, s);
      if (!v) return;

      for (const o of v.oscillators)
        o.detune.setValueAtTime(((s.bend - 8192) / 8192) * 200, when);
      if (!s.keys.has(key)) s.keys.set(key, []);
      s.keys.get(key).push(v);
    } else if (m.Off) {
      const queue = s.keys.get(m.Off.key);
      const v = queue?.shift();
      if (!queue?.length) s.keys.delete(m.Off.key);
      if (s.pedal && v) s.held.push(v);
      else this.kill(v, when);
    } else if (m.Bend) {
      s.bend = m.Bend.value;
      for (const v of this.voices)
        if (v.channel === s && !v.ended)
          for (const o of v.oscillators)
            o.detune.setValueAtTime(((s.bend - 8192) / 8192) * 200, when);
    } else if (m.Control) {
      const { controller: c, value: v } = m.Control;
      if (c === 64) {
        s.pedal = v >= 64;
        if (!s.pedal) {
          s.held.forEach((n) => this.kill(n, when));
          s.held = [];
        }
      }
      if (c === 7 || c === 11) {
        s[c === 7 ? "volume" : "expression"] = v / 127;
        s.gain.gain.setValueAtTime(s.volume * s.expression, when);
      }
      if (c === 120 || c === 123) {
        for (const queue of s.keys.values())
          for (const n of queue) {
            if (c === 123 && s.pedal) s.held.push(n);
            else this.kill(n, when);
          }
        s.keys.clear();
        if (c === 120) {
          s.held.forEach((n) => this.kill(n, when));
          s.held = [];
        }
      }
      if (c === 121) {
        s.pedal = false;
        s.held.forEach((n) => this.kill(n, when));
        s.held = [];
        s.volume = 1;
        s.expression = 1;
        s.bend = 8192;
        s.gain.gain.setValueAtTime(1, when);
        for (const n of this.voices)
          if (n.channel === s && !n.ended)
            for (const o of n.oscillators) o.detune.setValueAtTime(0, when);
      }
    }
  }
  restore(events, position, boundary = false) {
    // Reconstruct silent state first; do not replay historical attacks at seek.
    const channels = new Map();
    for (const e of events) {
      if (e.time_us > position || e.time_us === position && !boundary) break;
      if (boundary && e.time_us === position && e.message.On) continue;
      if (e.channel === 9) continue;
      const id = `${e.port}:${e.channel}`;
      if (!channels.has(id))
        channels.set(id, {
          port: e.port,
          channel: e.channel,
          keys: new Map(),
          held: [],
          pedal: false,
          controls: new Map(),
          bend: 8192,
        });
      const s = channels.get(id),
        m = e.message;
      if (m.On) {
        if (!s.keys.has(m.On.key)) s.keys.set(m.On.key, []);
        s.keys.get(m.On.key).push(m.On);
      }
      if (m.Off) {
        const queue = s.keys.get(m.Off.key),
          note = queue?.shift();
        if (!queue?.length) s.keys.delete(m.Off.key);
        if (s.pedal && note) s.held.push(note);
      }
      if (m.Bend) s.bend = m.Bend.value;
      if (m.Control) {
        const { controller: c, value: v } = m.Control;
        s.controls.set(c, v);
        if (c === 64) {
          s.pedal = v >= 64;
          if (!s.pedal) s.held = [];
        }
        if (c === 120 || c === 123) {
          if (c === 123 && s.pedal)
            for (const queue of s.keys.values()) s.held.push(...queue);
          s.keys.clear();
          if (c === 120) s.held = [];
        }
        if (c === 121) {
          s.pedal = false;
          s.held = [];
          s.controls.clear();
          s.bend = 8192;
        }
      }
    }
    for (const s of channels.values()) {
      const send = (message) =>
        this.event({ port: s.port, channel: s.channel, message }, this.now);
      for (const [controller, value] of s.controls)
        if ([7, 11, 64].includes(controller))
          send({ Control: { controller, value } });
      send({ Bend: { value: s.bend } });
      // Restore pedal-held voices first so FIFO offs cannot release a newer
      // physically held unison of the same pitch.
      for (const note of s.held) {
        send({ On: note });
        send({ Off: { key: note.key } });
      }
      for (const queue of s.keys.values())
        for (const note of queue) send({ On: note });
    }
  }
  stopSong() {
    for (const v of this.voices) if (v.channel) this.kill(v, this.now, true);
    for (const s of this.channels.values())
      setTimeout(() => s.gain.disconnect(), 600);
    this.channels.clear();
  }
  panic() {
    for (const v of this.voices) this.kill(v, this.now, true);
    for (const s of this.channels.values())
      setTimeout(() => s.gain.disconnect(), 600);
    this.channels.clear();
    this.live.clear();
  }
  energy() {
    if (!this.analyser) return 0;
    const data = new Float32Array(this.analyser.fftSize);
    this.analyser.getFloatTimeDomainData(data);
    return Math.sqrt(data.reduce((s, v) => s + v * v, 0) / data.length);
  }
}
