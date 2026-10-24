// A deliberately small browser boundary. Musical interpretation stays in Rust.
const $ = id => document.getElementById(id);
let inspect;
let generation = 0;
function fail(error) {
  let value;
  try { value = JSON.parse(String(error)); } catch { value = { message: String(error) }; }
  $("error").textContent = [value.code, value.message].filter(Boolean).join(": ");
  $("error").hidden = false;
}
try {
  const module = await import("./pkg/tangent_midi_web.js");
  await module.default();
  inspect = module.inspect_midi;
  $("status").textContent = "Ready. Select a file or try the study.";
  $("demo").disabled = false;
} catch {
  $("status").textContent = "The WASM module is missing or could not load. Build it using the instructions in this folder, then serve over HTTP.";
}
function show(song, name) {
  $("result").hidden = false;
  $("result-title").textContent = name;
  const notes = song.tracks.flatMap(t => t.notes);
  const values = [[notes.length, "notes"], [song.tracks.length, "tracks"], [(song.duration_us / 1e6).toFixed(2) + " s", "duration incl. silence"], [song.ticks_per_quarter, "ticks / quarter"]];
  $("stats").replaceChildren(...values.map(([value, title]) => {
    const el = document.createElement("div");
    const strong = document.createElement("strong"); strong.textContent = value;
    const label = document.createElement("span"); label.textContent = title;
    el.append(strong, label); return el;
  }));
  $("tracks").replaceChildren(...song.tracks.map(track => {
    const el = document.createElement("div"); el.className = "track";
    const title = document.createElement("strong"); title.textContent = `${track.index + 1} · ${track.name || "Untitled track"}`;
    const info = document.createElement("small");
    info.textContent = `${track.notes.length} notes · ${track.events.length} events · ${new Set(track.notes.map(n => `${n.port}:${n.channel}`)).size} port/channel pairs`;
    el.append(title, info); return el;
  }));
  $("warnings").textContent = song.diagnostics.length ? `${song.diagnostics.length} import diagnostic(s). Check the data before treating these notes as a scoring reference.` : "No pairing diagnostics. Note spans describe key-down time; pedal events are retained separately.";
  $("json").textContent = JSON.stringify(song, null, 2);
}
async function load(file) {
  const request = ++generation;
  $("result").hidden = true; $("error").hidden = true;
  try {
    if (!inspect) throw new Error("The import module is not ready.");
    if (file.size > 16 * 1024 * 1024) throw new Error("File exceeds the 16 MiB import limit.");
    $("status").textContent = "Reading and importing…";
    const bytes = new Uint8Array(await file.arrayBuffer());
    if (request !== generation) return;
    show(JSON.parse(inspect(bytes)), file.name);
    $("status").textContent = "Import complete. No file was uploaded.";
  } catch (error) {
    if (request !== generation) return;
    fail(error); $("status").textContent = "Import failed. You can choose another file.";
  }
}
$("file").addEventListener("change", e => { if (e.target.files[0]) load(e.target.files[0]); });
$("demo").addEventListener("click", () => {
  // Authored for this repository: 480 PPQ, 120 BPM, three quarter notes, C/E/G.
  const track = [0,0x90,60,80,0x83,0x60,0x80,60,0,0,0x90,64,80,0x83,0x60,0x80,64,0,0,0x90,67,80,0x83,0x60,0x80,67,0,0,0xff,0x2f,0];
  const header = [0x4d,0x54,0x68,0x64,0,0,0,6,0,0,0,1,1,0xe0,0x4d,0x54,0x72,0x6b,0,0,0,track.length];
  load(new File([new Uint8Array([...header, ...track])], "Tangent import study.mid"));
});
