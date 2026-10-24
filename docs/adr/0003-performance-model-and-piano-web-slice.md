# ADR-0003: Shared performance model and a browser piano adapter

- Status: Superseded in part by [ADR-0005](0005-one-bevy-gpu-surface.md) (Canvas/HTML presentation only)
- Created / decision date: 2026-10-24
- Delivery: Bevy application core and browser piano implemented; GPU/native rendering and authored-score import remain open
- Related: [0001](0001-bevy-and-web.md), [0002](0002-byte-based-midi-import.md), [piano design](../design/piano-player.md)

## Context

The next slice needs a playable on-screen piano, imported songs, synchronized
falling notes, sound and effects. Four visualizations must share timing, and
future songs may originate from MusicXML rather than MIDI. Themes must not change
musical meaning. Native-only audio/rendering assumptions would delay web validation.

## Decision

Use three layers, with dependencies pointing toward the musical domain:

- `tangent-core`: owned performance songs/parts/notes/events, piano geometry,
  exact transport time and optional authored notation types. No Bevy, MIDI parser,
  filesystem, browser, pixel units or sound devices.
- `tangent-player`: a Bevy 0.20 application (`bevy_app` + `bevy_ecs`, `std`, default
  features disabled), an update system and MIDI-to-performance adapter. It exposes
  bounded-window frame data and piano hit geometry through WASM. It can be tested
  natively without a window. Bevy owns the transport update schedule.
- `web/piano`: browser input, Canvas 2D rendering and Web Audio adapters. Separate
  modules own keyboard drawing, falling views, sheet projection, particles,
  themes and synthesis. The browser drives Bevy updates from requestAnimationFrame;
  the elapsed time comes from AudioContext.currentTime, never frame count.

**Canvas 2D is an explicit initial renderer adapter**, not a claim that the full
Bevy GPU renderer is integrated. This reduces the feasibility slice's device
requirements and lets sound, pointer hit testing and timing be tested together.
The accepted Bevy direction remains; moving visuals to Bevy's renderer requires
its own measured web/GPU decision. Native playback is not delivered here.

Performance notes use microsecond spans with stable within-song IDs and part
identity. MIDI's track/port/channel lanes become parts, including explicit
percussion lanes. MIDI ticks remain in the importer model, not in the shared song.
The merged event stream retains controllers, program and bend events; audio is
not generated solely from visual spans. Program events are preserved but this
piano-only synthesizer deliberately uses the chosen piano voice for all melodic
channels. Pressure, SysEx, RPN/NRPN and bank-dependent instruments are not rendered.

Authored notation is separate and optional: measures, staff/voice references,
spelled pitches, exact quarter-note fractions, ties and performance-note links.
No importer supplies it yet. MIDI rendering uses a clearly labelled sharp-based
reading projection, not invented MusicXML, quantization or a canonical score.

The Web Audio adapter schedules a 100 ms lookahead with oscillator partials,
velocity, release envelopes, sustain, channel volume/expression and fixed
±2-semitone pitch bend. Pause/restart/import/blur cancels scheduled voices. Seeking
reconstructs controller and sounding-note state silently, then re-attacks only
still-sounding notes. It does not reconstruct the historical acoustic envelope.
At most 128 voices are retained; voice stealing and a 30-second safety tail apply.

Instrument sound profiles, key geometry, palette/background and particle settings
are separate concerns. Changing theme/view does not restart or edit a song.
Keyboard/pointer source tokens prevent one input releasing another held unison.
Web MIDI is an optional browser capability requested explicitly, without SysEx.

## Consequences

The first app has real browser sound and deterministic shared timing without
loading a GPU stack or native audio dependencies. Frame JSON is an internal
prototype bridge, not a persistence format or a stable plugin API. Large-score
windowing/indexing, native adapters, sampled instruments and full engraving need
additional work and profiling; see [follow-up tasks](../TODO.md).

Audio scheduling is independent of screen refresh, but heavy main-thread stalls
can still exhaust the 100 ms lookahead. AudioWorklet/worker scheduling and output
latency calibration remain future decisions. Live MIDI has not been verified
against physical hardware; browser support and permissions vary.

## Alternatives

- Couple songs and key geometry to MIDI/Bevy entities: makes format and rendering
  changes leak into musical behavior.
- Build the entire GPU/audio/sample pipeline now: postpones an end-to-end web
  feasibility check and adds platform dependencies before basic input validation.
- Browser-only song/timing rules: duplicates the shared Rust model and does not
  establish Bevy application integration.

## Primary references and provenance

- https://docs.rs/bevy_app/0.20.0/bevy_app/struct.App.html
- https://docs.rs/bevy_ecs/0.20.0/bevy_ecs/
- https://developer.mozilla.org/en-US/docs/Web/API/AudioContext
- https://developer.mozilla.org/en-US/docs/Web/API/Navigator/requestMIDIAccess

Original requirements: owner's 2026-10-24 request and supplied hand-drawn piano
sketch. No legacy project source/assets were consulted for this implementation;
the prior context exposure remains recorded in [PROVENANCE](../../PROVENANCE.md).
