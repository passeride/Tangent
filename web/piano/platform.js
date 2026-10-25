// Browser capability adapter: audio, files, MIDI and the fullscreen API.
// No rendering, menus, note placement or transport rules live in this module.
import { PianoAudio } from "./audio.js";
import { openStorage } from "./storage.js";
export async function installPlatform() {
  const canvas = document.getElementById("tangent"),
    file = document.getElementById("midi-file"),
    audio = new PianoAudio();
  let queue = [],
    song = null,
    transport = {
      position: 0,
      playing: false,
      volume: 0.45,
      hear: true,
      instrument: 0,
    },
    anchor = 0,
    index = 0,
    generation = 0,
    access = null,
    state = null;
  const status = (text) => queue.push({ type: "status", text });
  const fail = (error) => status(String(error.message ?? error));
  let storage = null;
  const storageStatus = text => queue.push({ type: "storage-status", text });
  try {
    storage = await openStorage();
    storageStatus("Settings saved on this device.");
    queue.push({ type: "preferences", data: storage.settings });
  } catch (error) {
    storageStatus("Local saving unavailable; existing data preserved.");
    console.warn("Tangent storage:", error);
  }
  function reset() {
    audio.stopSong();
    if (!song) return;
    index = song.events.findIndex((e) => e.time_us >= transport.position);
    if (index < 0) index = song.events.length;
    anchor = audio.now - transport.position / 1e6;
    if (transport.playing && transport.hear)
      audio.restore(song.events, transport.position);
  }
  function schedule() {
    if (song && transport.playing && transport.hear) {
      const horizon = (audio.now - anchor) * 1e6 + 100000;
      while (
        index < song.events.length &&
        song.events[index].time_us <= horizon
      ) {
        const e = song.events[index++];
        audio.event(e, Math.max(audio.now, anchor + e.time_us / 1e6));
      }
    }
  }
  async function load(bytes, title, ticket) {
    if (ticket !== generation) return;
    if (bytes.length > 16 * 1024 * 1024)
      throw Error("Music files must be 16 MiB or smaller.");
    queue.push({ type: "file", bytes: [...bytes], title });
  }
  file.onchange = async () => {
    const selected = file.files[0],
      ticket = ++generation;
    if (!selected) return;
    try {
      if (selected.size > 16 * 1024 * 1024)
        throw Error("Music files must be 16 MiB or smaller.");
      await load(
        new Uint8Array(await selected.arrayBuffer()),
        selected.name.replace(/\.(midi?|musicxml|xml)$/i, ""),
        ticket,
      );
    } catch (e) {
      fail(e);
    } finally {
      file.value = "";
    }
  };
  async function midi() {
    if (!navigator.requestMIDIAccess)
      throw Error("Web MIDI unavailable. Use touch or the computer keyboard.");
    access ??= await navigator.requestMIDIAccess({ sysex: false });
    const attach = () => {
      queue.push({ type: "pause" });
      for (const port of access.inputs.values()) {
        port.onmidimessage = (e) => {
          const [s, key, value = 0] = e.data,
            ch = s & 15,
            type = s & 240,
            id = `midi:${port.id}:${ch}:${key}`;
          if (type === 144 || type === 128)
            queue.push({
              type: "note",
              id,
              key,
              velocity: value,
              down: type === 144 && value > 0,
            });
          if (type === 176 && (key === 120 || key === 123))
            queue.push({ type: "pause" });
        };
      }
      status(`${access.inputs.size} MIDI input(s).`);
    };
    attach();
    access.onstatechange = attach;
  }
  // These APIs need a user gesture. The capture listener unlocks sound, while
  // Bevy receives and interprets the same event for all application actions.
  const unlock = () => audio.activate().catch(fail);
  canvas.addEventListener("pointerdown", () => {
    canvas.focus();
    unlock();
  });
  canvas.addEventListener("keydown", (e) => {
    unlock();
    if (
      [
        "Space",
        "ArrowUp",
        "ArrowDown",
        "ArrowLeft",
        "ArrowRight",
        "F1",
        "F11",
      ].includes(e.code) ||
      (e.ctrlKey && ["KeyO", "KeyB", "KeyM"].includes(e.code))
    )
      e.preventDefault();
  });
  window.addEventListener("blur", () => {
    audio.panic();
    queue.push({ type: "pause" });
  });
  document.addEventListener("visibilitychange", () => {
    if (document.hidden) {
      audio.panic();
      queue.push({ type: "pause" });
    }
  });
  window.tangentPlatform = {
    clock_us: () => audio.now * 1e6,
    pull: () => {
      schedule();
      const messages = queue;
      queue = [];
      return JSON.stringify(messages);
    },
    action: (name, data) => {
      try {
        switch (name) {
          case "save-settings":
            if (storage) {
              storageStatus("Saving settings…");
              storage.saveSettings(JSON.parse(data))
                .then(() => storageStatus("Settings saved on this device."))
                .catch(() => storageStatus("Could not save settings. Export a backup."));
            }
            break;
          case "export-database":
            if (!storage) { storageStatus("Local database unavailable."); break; }
            storage.exportDatabase().then(bytes => {
              const url = URL.createObjectURL(new Blob([bytes], { type: "application/vnd.sqlite3" }));
              const link = document.createElement("a");
              link.href = url; link.download = "tangent.sqlite"; link.click();
              setTimeout(() => URL.revokeObjectURL(url), 30_000);
            }).catch(() => storageStatus("Could not export database."));
            break;
          case "song":
            song = JSON.parse(data);
            reset();
            break;
          case "transport":
            transport = JSON.parse(data);
            audio.instrument = transport.instrument ? "bright" : "felt";
            audio.setVolume(transport.volume);
            reset();
            break;
          case "volume":
            audio.setVolume(Number(data));
            break;
          case "on": {
            const n = JSON.parse(data);
            audio.liveOn(n.id, n.key, n.velocity);
            break;
          }
          case "off":
            audio.liveOff(data);
            break;
          case "panic":
            audio.panic();
            break;
          case "fullscreen":
            if (document.fullscreenElement)
              document.exitFullscreen().catch(fail);
            else canvas.requestFullscreen().catch(fail);
            break;
          case "import":
            file.click();
            break;
          case "sample": {
            const ticket = ++generation;
            fetch(
              new URL("./samples/first-light.musicxml", import.meta.url),
            )
              .then((r) => {
                if (!r.ok) throw Error("Could not load MusicXML study.");
                return r.arrayBuffer();
              })
              .then((b) =>
                load(new Uint8Array(b), "First light — Tangent piano study", ticket),
              )
              .catch(fail);
            break;
          }
          case "midi":
            midi().catch(fail);
            break;
          case "credits":
            window.open(
              new URL("./samples/ATTRIBUTION.md", import.meta.url),
              "_blank",
              "noopener",
            );
            break;
        }
      } catch (e) {
        fail(e);
      }
    },
    report: (data) => {
      state = JSON.parse(data);
      document.getElementById("startup").hidden = true;
    },
  };
  window.tangentDiagnostics = () =>
    state
      ? {
          ...state,
          storage: storage?.diagnostics ?? { ready: false },
          energy: audio.energy(),
          audioState: audio.context?.state,
          voices: audio.voices.size,
        }
      : null;
  canvas.focus();
}
