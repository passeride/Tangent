# Browser piano and song playback

This is the first interactive application slice, built on the existing importer.
The complete Bevy UI/wgpu application runs in WASM. Web Audio, file access,
MIDI and fullscreen are browser capabilities; no HTML menus are used.
See [ADR-0005](adr/0005-one-bevy-gpu-surface.md) for the current boundaries.

## Run locally

Use the pinned Rust toolchain and wasm-bindgen CLI 0.2.129:

```sh
cargo build -p tangent-player --target wasm32-unknown-unknown --release --locked
wasm-bindgen target/wasm32-unknown-unknown/release/tangent_player.wasm --target web --out-dir web/piano/pkg
python3 -m http.server 8080 --directory web/piano
```

Open http://localhost:8080. Press Play, tap a piano key or use A–K to activate
sound. Import a MIDI file with the Import MIDI control. Files are processed in
memory and are not uploaded or persisted. The distribution workflow now includes GitHub Pages deployment; see
[distribution status/setup](distribution.md). GitHub Actions produces a `tangent-piano-web` artifact containing the
static app and generated WASM; serve it over HTTP rather than opening file URLs.

## Scope

- 49/61/88 keys; pointer, touch, keyboard and optional Web MIDI note input.
- Original MusicXML piano study and local MIDI type 0/1 or MusicXML import.
- Audio-clock transport, play/pause/restart/seek and optional song sound.
- Falling blocks, rotated staff blocks with ledger lines, held falling noteheads,
  stationary grand-staff reading view with paged playhead. Staff lines are also
  present behind the falling noteheads.
- Three visual themes, two synthesized piano profiles, volume and sparkles.
- Velocity, file sustain/controller/bend handling and bounded voices. Hardware
  note input is supported; hardware pedals/controllers are not yet interpreted.
- Percussion channels are silent and omitted from the piano render window.

Imported MIDI is a **performance**, not authored notation. Sharp spelling and
symbol duration categories are approximate. MusicXML now has an explicit initial
import subset; a full score engraver is not implemented. Program changes are retained in the performance stream
but all melodic channels use the selected piano. Pressure, SysEx, bank/program
instruments, RPN tuning/bend-range changes and pedal-aware scoring are not rendered.
Seeking re-attacks currently sounding voices with fresh envelopes. The 100 ms
lookahead can underrun under heavy main-thread stalls. It is not calibrated to
speaker/display latency. Ordinary browser permissions and Web MIDI support apply.

## Verification

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo build -p tangent-player --target wasm32-unknown-unknown --release --locked
wasm-bindgen target/wasm32-unknown-unknown/release/tangent_player.wasm --target web --out-dir web/piano/pkg
npm ci --prefix web/piano
cd web/piano && npx playwright install chromium
npm test
```

The browser test starts its own HTTP server and Chromium, loads the real WASM
and checks audio energy rather than only counting calls to mocked audio methods.
Physical sound quality/latency and a hardware MIDI device require manual checks;
an analyser signal alone does not prove a particular speaker produced sound.
Native Cargo tests do not establish native UI/audio support.

## Verification record — 2026-10-24

27 workspace Rust tests pass, together with formatting and warnings-denied Clippy.
The release WASM build and real Chromium piano/import smoke tests pass. Audio
checks include a nonzero analyser signal, FIFO repeated keys, sustain release,
silent state reconstruction at seek and cancellation of future voices. Desktop,
notation and narrow-layout screenshots were inspected. Windows CI is configured;
no Windows UI, physical MIDI piano or real-speaker latency result is claimed.

## Single-surface UI update — 2026-10-24

All menus, piano keys, notes, staff lines, progress, settings and help are Bevy UI
inside one full-window GPU canvas. The former Canvas 2D modules are removed.
The shell has only that canvas, a hidden OS file picker and a startup-error fallback.
The renderer uses wgpu with WebGL2; unsupported GPUs/browsers show a startup error.

Space plays/pauses; Left/Right seek 5s (Shift 15s); Up/Down adjust volume 5%;
Home/End; F11 fullscreen; F1 settings; Esc closes settings; V view; B theme;
M song sound; P sparkles; X piano; R range; Ctrl+O import; Ctrl+B XML demo; Ctrl+M MIDI.
A W S E D F T G Y H U J K remain the piano keys.

`npm test --prefix web/piano` now loads the actual Bevy GPU application and tests
in-canvas controls, staff-note mode, transport/volume shortcuts, fullscreen entry/
exit, MIDI import and viewport containment. The packaged-site smoke tests
`/Tangent/` paths. Software GPU rendering in CI does not establish physical
hardware performance or sound latency.

Verification for this update: 29 native Rust tests, native and WASM warnings-denied
Clippy, release WASM build, real Bevy/Chromium tests, packaged `/Tangent/` site and
embedded desktop launcher smoke pass locally. Fullscreen entry and exit, hidden
HTML controls, OS file chooser and black-key pointer input are included.

The bootstrap probes WebGL2 on a detached canvas before initializing Bevy, and
reports asynchronous renderer failures. A browser test simulates unavailable
WebGL2 and verifies a visible explanation instead of an endless loading screen.

## Pause screen and MusicXML

The corner gear, Esc or F1 opens a pause screen and stops song/live sound.
All settings, import/sample, transport, volume, fullscreen and MIDI controls are
inside that screen. Space resumes and closes it; Return/Esc/F1 closes it while
remaining paused. Playing shows only a gear and the timeline, with shortcuts
available. See [MusicXML import](musicxml-import.md) for the new authored subset.


The MusicXML demo and note display are described in ADR-0007: real Bravura G/F
clefs, translucent clef watermarks on rotated staves, measure/beat guides,
barlines, meter, tempo, repeat signs and first/second houses. The score playhead
uses explicit performance visits when returning to a repeated written measure.
