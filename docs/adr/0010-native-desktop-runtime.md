# ADR-0010: Native desktop without a browser runtime

- Status: Accepted
- Decision date: 2026-10-25
- Delivery: Initial implementation on feat/native-desktop-performance; native device verification pending
- Related: ADR-0003, ADR-0005, ADR-0009
- Supersedes: Desktop packaging choice in ADR-0004 only; Pages/web distribution remains

## Context

The former desktop "app" was a small HTTP server opening the existing
WebAssembly player in a browser. That does not provide native graphics/input
capabilities or predictable desktop audio latency. The user requests a genuine
desktop application and gives performance priority. We must not fork the
renderer, music model or practice rules to create two different products.

## Decision

Compile the same tangent-player Bevy scene for native targets with a real OS
window. Keep the web wasm-bindgen adapter on wasm32; add a native Rust
capability adapter that has no HTTP, browser or embedded web assets. The native
binary in tangent-desktop only calls the shared run_native_player entry point.

Keep native MIDI reception, sampled/algorithmic audio synthesis, SQLite and file
I/O off the Bevy rendering thread. A dedicated worker receives commands via
channels, polls inputs through midir and runs the rodio output mixer with
32ms song lookahead and a bounded 64-voice ceiling. Timestamps originate from
a monotonic Instant. The current simple harmonic voice is not yet equivalent to
the richer browser Web Audio tone, and MIDI sustain/controllers and accurate
latency calibration remain subsequent tasks.

The existing score/notation GPU entity pool and piano/practice domains remain
shared. Avoid serializing binary files into large JSON arrays for desktop
imports; transfer actual bytes on an in-process channel. Native preferences
and future scores use an independently opened SQLite file with the same schema
version 1 as ADR-0009. The native database is not the browser's IndexedDB
image; migrations must stay compatible at the SQLite schema level.

Native dependencies: Bevy 0.20, rodio 0.22.2, midir 0.11 and rusqlite 0.40.2
with bundled SQLite. Keep Bevy feature selection lean; avoid including browser
WebGL and Node-based runtime assets in the desktop binary. Do not enable
host-specific CPU instruction sets in published builds: binaries must work on
other machines of the same architecture.

## Alternatives and consequences

Embedding Chromium, Electron, a webview or a loopback server would keep the
browser scheduling, input and distribution constraints. Reimplementing UI in
a second toolkit would duplicate drawing and break the shared Bevy interface.
Putting SQLite reads or audio decoding in an ECS Update system risks jank.

A fully mature desktop release still requires actual hardware tests:
multiple MIDI devices, note-off and pedal sustain, unplug/replug, file
picker failure, audio device loss, long scores/tempo changes, measured MIDI
latency, render FPS and memory. CI can compile each native target and run
a headless CLI smoke check; that does not verify physical GPU/audio/MIDI.

Dependencies are independently published crates; the older Neothesia fork
and its implementation code/assets are excluded. Previously disclosed source
exposure remains documented in PROVENANCE.md.
