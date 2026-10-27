# ADR-0030: Major Scale Run

- Status: Accepted
- Decision date: 2026-10-27
- Delivery: S03 initial pitch/order variant; physical piano validation pending
- Related: [sequence ownership](0028-five-finger-sequences.md), [brief](../design/major-scale-run.md)

## Context and decision

The remaining concrete Train TODO is S03. Reuse the release-gated SequenceRunner
and whole-pattern scoring from ADR-0028. A separate core/major_scale module owns
12 explicitly spelled major keys, one-octave ascending/descending patterns and
advisory hand-specific fingers. Practice owns the selected exercise and cursor;
Bevy presents a shared sequence view. No new dependencies are required.

Levels are right C, left C and all keys in circle order, with a right/left/alternating-hand selector. A session contains 12 whole scales. Release every note before the
next; pitch/order is assessed, timing and physical fingers are not. Hint/Hear and
mistakes remain sticky across the entire 15-step scale. Skip resets the cursor.

Runs use mode `major-scale-run`; evidence `scale:1:<L|R>:<0..11>` identifies the
versioned key spelling, register, hand instruction and full pattern deterministically.
Rust/native and browser validators reject unknown keys, versions and hands.
Existing SQLite tables and schema remain unchanged. Retry restores exercise and
level, starting a new scale. The evidence version must change if these plans change.

## Alternatives and consequences

Copying the five-finger engine would duplicate score and input rules. Extending
FiveFingerTask itself would confuse distinct musical curricula. A general script
engine is unnecessary for this bounded exercise. The shared view pages 15 notes
into overlapping ascent/descent windows on narrow screens.

This delivers the explicit TODO boundary: one octave with exact ordered input.
The catalogue's multi-octave, legato, metronome, timing and simultaneous-hand
variants remain future work. They must not be inferred from the S03 delivery.
