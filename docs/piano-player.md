# Browser piano and song playback

This is the first interactive application slice, built on the existing importer.
The Bevy core runs in WASM; Canvas and Web Audio are explicit browser adapters.
See [ADR-0003](adr/0003-performance-model-and-piano-web-slice.md) for boundaries.

## Run locally

Use the pinned Rust toolchain and wasm-bindgen CLI 0.2.129:

```sh
cargo build -p tangent-player --target wasm32-unknown-unknown --release --locked
wasm-bindgen target/wasm32-unknown-unknown/release/tangent_player.wasm --target web --out-dir web/piano/pkg
python3 -m http.server 8080 --directory web/piano
```

Open http://localhost:8080. Press Play, tap a piano key or use A–K to activate
sound. Import a MIDI file with the Import MIDI control. Files are processed in
memory and are not uploaded or persisted. Nothing is publicly deployed by this
change. GitHub Actions produces a `tangent-piano-web` artifact containing the
static app and generated WASM; serve it over HTTP rather than opening file URLs.

## Scope

- 49/61/88 keys; pointer, touch, keyboard and optional Web MIDI note input.
- Original demonstration study and local MIDI type 0/1 import.
- Audio-clock transport, play/pause/restart/seek and optional song sound.
- Falling blocks, rotated staff blocks with ledger lines, held falling noteheads,
  stationary grand-staff reading view with paged playhead.
- Three visual themes, two synthesized piano profiles, volume and sparkles.
- Velocity, file sustain/controller/bend handling and bounded voices. Hardware
  note input is supported; hardware pedals/controllers are not yet interpreted.
- Percussion channels are silent and omitted from the piano render window.

Imported MIDI is a **performance**, not authored notation. Sharp spelling and
symbol duration categories are approximate; no MusicXML parser or full score
engraver is implemented. Program changes are retained in the performance stream
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
