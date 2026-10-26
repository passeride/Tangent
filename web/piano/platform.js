// Browser capability adapter: audio, files, MIDI and the fullscreen API.
// No rendering, menus, note placement or transport rules live in this module.
import { MidiOutput } from "./midi-output.js";
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
  const midiOutput=new MidiOutput(error=>{
    queue.push({type:"pause"}); reportOutputs();fail(error);
  });
  function reportOutputs() {
    queue.push({type:"midi-outputs",ports:access?[...access.outputs.values()].filter(p=>p.state!=="disconnected").map(p=>({id:p.id,name:p.name??p.id})):[],selected:midiOutput.port?.id??""});
  }
  let outputGeneration=0;
  async function selectOutput(id) {
    const ticket=++outputGeneration;
    // Pause first. Device selection is explicit and never restored blindly.
    audio.panic(); midiOutput.select(null);
    if(id) {
      const port=access?.outputs.get(id);
      if(!port||port.state==="disconnected")throw Error("Output changed; rescan MIDI.");
      await port.open();
      if(ticket!==outputGeneration)return;
      midiOutput.select(port);
    }
    reportOutputs();
  }
  let storage = null;
  const storageStatus = text => queue.push({ type: "storage-status", text });
  try {
    storage = await openStorage();
    storageStatus("Settings saved on this device.");
    queue.push({ type: "preferences", data: storage.settings });
    // Browser MIDI authorization may still require a user gesture. Reconnect
    // automatically only if the origin already has permission; otherwise the
    // saved device remains visible until the user selects Rescan / connect.
    if (storage.settings?.midi_output_name && navigator.permissions?.query) {
      navigator.permissions.query({name:"midi",sysex:false})
        .then(permission => {
          if (permission.state === "granted") return midi();
        })
        .catch(() => {}); // Unsupported Permissions API: manual connect is safe.
    }
  } catch (error) {
    storageStatus("Local saving unavailable; existing data preserved.");
    console.warn("Tangent storage:", error);
  }
  async function sourceId(bytes) {
    const hash = await crypto.subtle.digest("SHA-256", bytes);
    return [...new Uint8Array(hash)].map(n => n.toString(16).padStart(2, "0")).join("");
  }
  function fingerprintId(song) {
    return sourceId(new TextEncoder().encode(JSON.stringify(song)));
  }
  function reset() {
    audio.stopSong();
    midiOutput.panic();
    if (!song) return;
    index = song.events.findIndex((e) => e.time_us >= transport.position);
    if (index < 0) index = song.events.length;
    anchor = audio.now - transport.position / 1e6;
    if (transport.playing && transport.hear) {
      if(midiOutput.port) midiOutput.restore(song.events,transport.position);
      else audio.restore(song.events, transport.position);
    }
  }
  function schedule() {
    if (song && transport.playing && transport.hear) {
      // Physical output is sent only when due, so pause cannot leave scheduled
      // attacks behind. Web Audio keeps its independent 100ms lookahead.
      const horizon = (audio.now - anchor) * 1e6 + (midiOutput.port?0:100000);
      while (
        index < song.events.length &&
        song.events[index].time_us <= horizon
      ) {
        const e = song.events[index++];
        if(midiOutput.port)midiOutput.event(e);
        else audio.event(e, Math.max(audio.now, anchor + e.time_us / 1e6));
      }
    }
  }
  let importSequence=0, lastImportSave=Promise.resolve(true);
  const importReceipts=new Map();
  let batchInfo={id:0,pending:false,total:0,saved:0};
  async function load(bytes, title, ticket, fromLibrary=false) {
    if (ticket !== generation) return false;
    if (bytes.length > 16 * 1024 * 1024) throw Error("Music files must be 16 MiB or smaller.");
    const importTicket=++importSequence;
    const complete=new Promise(resolve=>importReceipts.set(importTicket,resolve));
    queue.push({ type:"file",bytes:[...bytes],title,from_library:fromLibrary,import_ticket:importTicket });
    return complete;
  }
  file.onchange = async () => {
    const selected=Array.from(file.files);
    // Closing/resetting the native chooser may emit an empty change event.
    // It must not cancel a batch whose File objects have already been captured.
    if (!selected.length) return;
    const ticket=++generation, batch=batchInfo.id+1;
    batchInfo={id:batch,pending:true,total:selected.length,saved:0};
    try {
      if (selected.length>20) throw Error("Choose at most 20 files at once.");
      let saved=0;
      for (const item of selected) {
        if (ticket!==generation) break;
        try {
          if (item.size>16*1024*1024) throw Error(`${item.name}: exceeds 16 MiB`);
          const ok=await load(new Uint8Array(await item.arrayBuffer()),item.name.replace(/\.(midi?|musicxml|xml|mxl)$/i,""),ticket);
          if (ok) saved++;
          if(batchInfo.id===batch) batchInfo.saved=saved;
        } catch(error) { queue.push({type:"library-status",text:String(error.message??error)}); }
      }
      if (ticket===generation && selected.length>1) queue.push({type:"library-status",text:`Saved ${saved} of ${selected.length} files. Duplicate files update existing entries.`});
    } catch(error) { queue.push({type:"library-status",text:String(error.message??error)}); }
    finally { if(batchInfo.id===batch) batchInfo.pending=false; file.value=""; }
  };
  async function midi() {
    if (!navigator.requestMIDIAccess)
      throw Error("Web MIDI unavailable. Use touch or the computer keyboard.");
    access ??= await navigator.requestMIDIAccess({ sysex: false });
    const attach = (event) => {
      // Ignore a connection-state-only notification from port.open().
      if(event && event.port.state!=="disconnected" && event.port.connection!=="closed") {reportOutputs();return;}
      queue.push({ type: "pause" });
      audio.panic();midiOutput.panic();
      if(midiOutput.port?.state==="disconnected")midiOutput.select(null);
      reportOutputs();
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
        "Tab",
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
  window.addEventListener("pagehide",()=>midiOutput.panic());
  window.addEventListener("blur", () => {
    audio.panic();
    midiOutput.panic();
    queue.push({ type: "pause" });
  });
  document.addEventListener("visibilitychange", () => {
    if (document.hidden) {
      audio.panic();
      midiOutput.panic();
      queue.push({ type: "pause" });
    }
  });
  window.tangentPlatform = {
    store_import(bytes, metadata) {
      const source=new Uint8Array(bytes), details=JSON.parse(metadata);
      if (storage) lastImportSave=Promise.all([sourceId(source),details.fingerprint?fingerprintId(details.fingerprint):null])
        .then(([id,fingerprint_id])=>storage.saveLibrarySong(id,source,{...details,fingerprint_id}))
        .then(()=>{ queue.push({type:"library-updated",text:"Saved in library."});return true; })
        .catch(error=>{queue.push({type:"library-status",text:`Could not save piece: ${error.message??error}`});return false;});
      else {
        lastImportSave=Promise.resolve(false);
        queue.push({type:"library-status",text:"Piece loaded; local library is unavailable."});
      }
    },
    clock_us: () => audio.now * 1e6,
    wall_time_ms: () => Date.now(),
    session_nonce: () => {const a=crypto.getRandomValues(new Uint32Array(2));return (BigInt(a[0])<<32n)|BigInt(a[1]);},
    pull: () => {
      schedule();
      const messages = queue;
      queue = [];
      return JSON.stringify(messages);
    },
    action: (name, data) => {
      try {
        switch (name) {
          case "progress-list":
            if(storage)storage.readProgress().then(runs=>queue.push({type:"progress",runs})).catch(e=>queue.push({type:"progress-error",text:String(e.message??e)}));
            break;
          case "progress-save": {
            const run=JSON.parse(data);
            if(storage)storage.saveProgress(run).then(()=>queue.push({type:"progress-saved",id:run.id,revision:run.revision})).catch(e=>queue.push({type:"progress-error",text:String(e.message??e)}));
            else queue.push({type:"progress-error",text:"Practice is not saved: local storage unavailable."});
            break;
          }
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
          case "import-result": {
            const result=JSON.parse(data), resolve=importReceipts.get(result.ticket);
            importReceipts.delete(result.ticket);
            const saved=result.ok?lastImportSave:Promise.resolve(false);
            saved.then(ok=>resolve?.(ok));
            break;
          }
          case "library-edit": {
            const request=JSON.parse(data);
            if(storage) storage.editLibraryMetadata(request.id,request.title,request.composer)
              .then(()=>queue.push({type:"library-updated",text:"Metadata saved.",metadata:true}))
              .catch(error=>queue.push({type:"library-status",text:String(error.message??error)}));
            break;
          }
          case "library-list": {
            const request = JSON.parse(data);
            if (storage) storage.listLibrary(request.offset, request.limit, request.query, request.sort, request.format)
              .then(result => queue.push({ type: "library-list", ticket: request.ticket, ...result }))
              .catch(error => queue.push({ type: "library-status", text: String(error.message ?? error) }));
            else queue.push({ type: "library-status", text: "Local library is unavailable on this device." });
            break;
          }
          case "library-open":
            if (storage) {
              const ticket = ++generation;
              storage.readLibrarySource(data).then(entry => load(entry.source, entry.title, ticket, true)).catch(fail);
            }
            break;
          case "library-remove":
            if (storage) storage.removeLibrarySong(data)
              .then(() => queue.push({ type: "library-updated", text: "Piece removed from library." })).catch(fail);
            break;
          case "fingerprint-index":
            if (storage) {
              const indexed = JSON.parse(data);
              fingerprintId(indexed)
                .then(id => storage.saveFingerprintSong({ ...indexed, id }))
                .catch(error => { console.warn("Fingerprint indexing:", error); storageStatus("Could not index imported music."); });
            }
            break;
          case "fingerprint-query":
            if (storage) {
              const request = JSON.parse(data);
              storage.findFingerprintCandidates(request.signatures)
                .then(candidates => queue.push({
                  type: "fingerprint-candidates",
                  ticket: request.ticket,
                  keys: request.keys,
                  candidates,
                }))
                .catch(error => { console.warn("Fingerprint search:", error); });
            }
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
            if(midiOutput.port)midiOutput.on(n.id,n.key,n.velocity);
            else audio.liveOn(n.id, n.key, n.velocity);
            break;
          }
          case "off":
            midiOutput.off(data);
            audio.liveOff(data);
            break;
          case "panic":
            midiOutput.panic();
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
          case "midi-output":
            selectOutput(data).catch(error=>{reportOutputs();fail(error);});
            break;
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
      state = { ...JSON.parse(data), frame: (state?.frame ?? 0) + 1 };
      document.getElementById("startup").hidden = true;
    },
  };
  window.tangentDiagnostics = () =>
    state
      ? {
          ...state,
          midiOutput: midiOutput.port?.id??"",
          imports: {...batchInfo,waiting:importReceipts.size},
          storage: storage?.diagnostics ?? { ready: false },
          energy: audio.energy(),
          audioState: audio.context?.state,
          voices: audio.voices.size,
        }
      : null;
  canvas.focus();
}
