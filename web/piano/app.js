import init, { Player } from "./pkg/tangent_player.js";
import { PianoAudio } from "./audio.js";
import { themes } from "./themes.js";
import { geometry, center } from "./geometry.js";
import { drawKeyboard } from "./keyboard.js";
import { falling } from "./falling.js";
import { score } from "./score.js";
import { Particles } from "./particles.js";
import { bindInput } from "./input.js";
const $ = (id) => document.getElementById(id),
  canvas = $("piano"),
  ctx = canvas.getContext("2d");
const audio = new PianoAudio(),
  particles = new Particles();
let player,
  song,
  layout,
  reading,
  g,
  frame,
  live = new Set(),
  previous = new Set(),
  lastAudio = 0,
  lastRAF = 0,
  eventIndex = 0,
  anchor = 0,
  generation = 0,
  input;
function error(e) {
  $("error").hidden = false;
  $("error").textContent = String(e.message ?? e);
}
function status(s) {
  $("status").textContent = s;
  $("error").hidden = true;
}
function resize() {
  const r = canvas.getBoundingClientRect(),
    dpr = Math.min(2, devicePixelRatio || 1);
  canvas.width = r.width * dpr;
  canvas.height = r.height * dpr;
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  g = geometry(r.width, r.height, layout);
  particles.items = [];
}
function resetScheduler() {
  audio.stopSong();
  eventIndex = song.events.findIndex((e) => e.time_us >= frame.position_us);
  if (eventIndex < 0) eventIndex = song.events.length;
  anchor = audio.now - frame.position_us / 1e6;
  lastAudio = audio.now;
  // Rebuild controller state, then notes already held at the seek position. This
  // uses the merged event stream rather than synthesizing attacks from spans.
  if (frame.playing && $("autoplay").checked) {
    audio.restore(song.events, frame.position_us);
  }
}
function refreshSong() {
  song = JSON.parse(player.song_json());
  frame = JSON.parse(player.frame(0));
  $("title").textContent = song.title;
  $("source").textContent =
    `${song.parts.length} parts · ${song.notes.length} notes`;
  $("seek").max = String(song.duration_us / 1000);
  previous.clear();
  particles.items = [];
  resetScheduler();
}
async function toggle() {
  if (!player) return;
  try {
    await audio.activate();
    if (frame.position_us >= song.duration_us) player.seek(0);
    player.set_playing(!frame.playing);
    frame = JSON.parse(player.frame(0));
    resetScheduler();
    status(
      frame.playing
        ? "Playing · follow the light or play along."
        : "Paused · the piano is yours.",
    );
  } catch (e) {
    error(e);
  }
}
function pause() {
  if (!player) return;
  player.set_playing(false);
  frame = JSON.parse(player.frame(0));
  audio.panic();
  lastAudio = audio.now;
  previous.clear();
}
function format(us) {
  const s = Math.floor(us / 1e6);
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
}
function render(now) {
  const dt = Math.min(0.05, (now - lastRAF) / 1000 || 0);
  lastRAF = now;
  const clock = audio.now,
    elapsed = frame.playing ? Math.max(0, clock - lastAudio) * 1e6 : 0;
  lastAudio = clock;
  const wasPlaying = frame.playing;
  frame = JSON.parse(player.frame(elapsed));
  if (wasPlaying && !frame.playing) audio.stopSong();
  if (frame.playing && $("autoplay").checked) {
    const horizon = frame.position_us + 100_000;
    while (
      eventIndex < song.events.length &&
      song.events[eventIndex].time_us <= horizon
    ) {
      const e = song.events[eventIndex++];
      audio.event(e, Math.max(audio.now, anchor + e.time_us / 1e6));
    }
  }
  const theme = themes[$("theme").value],
    mode = $("view").value;
  const bg = ctx.createLinearGradient(0, 0, 0, g.strike);
  bg.addColorStop(0, theme.bg);
  bg.addColorStop(1, theme.horizon);
  ctx.fillStyle = bg;
  ctx.fillRect(0, 0, g.width, g.height);
  const active = new Map(),
    ids = new Set();
  for (const n of frame.notes)
    if (n.start_us <= frame.position_us && n.end_us > frame.position_us) {
      active.set(n.key, theme.colors[n.part % theme.colors.length]);
      ids.add(n.id);
      if (frame.playing && !previous.has(n.id) && $("effects").checked) {
        const x = center(g, n.key);
        if (x !== null) particles.emit(x, g.strike, active.get(n.key));
      }
    }
  for (const key of live) active.set(key, theme.glow);
  previous = ids;
  if (mode === "score") score(ctx, g, frame, theme, reading);
  else falling(ctx, g, frame, theme, mode, reading);
  particles.draw(ctx, dt, $("effects").checked);
  drawKeyboard(ctx, g, theme, active);
  $("play").textContent = frame.playing ? "Ⅱ Pause" : "▶ Play";
  $("seek").value = String(frame.position_us / 1000);
  $("time").textContent =
    `${format(frame.position_us)} / ${format(song.duration_us)}`;
  requestAnimationFrame(render);
}
try {
  await init();
  player = new Player();
  if (innerWidth < 650) {
    player.set_range(36, 84);
    $("range").value = "49";
  }
  layout = JSON.parse(player.layout_json());
  reading = JSON.parse(player.reading_json());
  refreshSong();
  resize();
  new ResizeObserver(resize).observe(canvas);
  input = bindInput({
    canvas,
    player,
    getGeometry: () => g,
    audio,
    onError: error,
    toggle,
    onChange: (next) => {
      const theme = themes[$("theme").value];
      if ($("effects").checked)
        for (const key of next)
          if (!live.has(key)) {
            const x = center(g, key);
            if (x !== null) particles.emit(x, g.strike, theme.glow);
          }
      live = next;
    },
  });
  $("sample").disabled = false;
  $("sample").onclick = async () => {
    const ticket = ++generation;
    $("sample").disabled = true;
    try {
      const response = await fetch(
        new URL("./samples/bach-prelude-bwv924a.mid", import.meta.url),
      );
      if (!response.ok)
        throw new Error(`Sample download failed (${response.status})`);
      const bytes = new Uint8Array(await response.arrayBuffer());
      if (ticket !== generation) return;
      player.load_midi(bytes, "Bach · Prelude BWV 924a");
      input.clear();
      refreshSong();
      status(
        "Bach loaded · press Play. MIDI: jeff covey / Mutopia, CC BY-SA 3.0.",
      );
    } catch (e) {
      error(e);
    } finally {
      $("sample").disabled = false;
    }
  };
  $("play").disabled = false;
  $("play").onclick = toggle;
  $("restart").onclick = () => {
    player.seek(0);
    frame = JSON.parse(player.frame(0));
    previous.clear();
    particles.items = [];
    resetScheduler();
  };
  $("seek").oninput = () => {
    player.seek(Number($("seek").value) * 1000);
    frame = JSON.parse(player.frame(0));
    previous.clear();
    particles.items = [];
    resetScheduler();
  };
  $("autoplay").onchange = resetScheduler;
  $("volume").oninput = () => audio.setVolume(Number($("volume").value));
  $("instrument").onchange = () => {
    audio.instrument = $("instrument").value;
    input.clear();
    resetScheduler();
  };
  $("midi").onclick = async () => {
    try {
      const count = await input.connect();
      status(
        count
          ? `${count} MIDI input(s) connected.`
          : "MIDI enabled. Connect a piano to begin.",
      );
    } catch (e) {
      error(e);
    }
  };
  $("range").onchange = () => {
    const range = $("range").value;
    player.set_range(
      range === "88" ? 21 : 36,
      range === "49" ? 84 : range === "61" ? 96 : 108,
    );
    layout = JSON.parse(player.layout_json());
    resize();
  };
  $("view").onchange = () => {
    const labels = {
      blocks: "FOLLOW THE FALLING LIGHT",
      staff: "FIND YOUR PLACE ON THE STAFF",
      notes: "A NOTE AT A TIME",
      score: "READ AT YOUR OWN PACE",
    };
    $("mode-name").textContent = labels[$("view").value];
    $("notation-hint").textContent =
      $("view").value === "blocks"
        ? "Performance view"
        : "MIDI reading aid · approximate notation";
  };
  $("file").onchange = async () => {
    const file = $("file").files[0],
      ticket = ++generation;
    if (!file) return;
    try {
      if (file.size > 16 * 1024 * 1024)
        throw new Error("MIDI files must be 16 MiB or smaller.");
      const bytes = new Uint8Array(await file.arrayBuffer());
      if (ticket !== generation) return;
      player.load_midi(bytes, file.name.replace(/\.midi?$/i, ""));
      input.clear();
      refreshSong();
      status(
        song.warnings.length
          ? `MIDI loaded with ${song.warnings.length} import warning(s): ${song.warnings.slice(0, 3).join("; ")}`
          : "MIDI loaded · press Play to begin.",
      );
    } catch (e) {
      error(e);
    } finally {
      $("file").value = "";
    }
  };
  document.addEventListener("visibilitychange", () => {
    if (document.hidden) {
      pause();
      input.clear();
    }
  });
  window.addEventListener("blur", pause);
  status("Ready · press Play, tap a key, or use A–K to enable sound.");
  requestAnimationFrame(render);
  // A narrow read-only diagnostics surface used by the browser smoke test.
  window.tangentDiagnostics = () => ({
    position: frame.position_us,
    playing: frame.playing,
    live: [...live],
    voices: audio.voices.size,
    energy: audio.energy(),
    particles: particles.items.length,
    notes: song.notes.length,
    audioState: audio.context?.state,
    geometry: g,
  });
} catch (e) {
  error(
    `The piano could not start. Build the WASM package first. ${e.message ?? e}`,
  );
}
