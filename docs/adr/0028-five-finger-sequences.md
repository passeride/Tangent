# ADR-0028: Five Finger Flow and release-gated sequences

- Status: Accepted
- Decision date: 2026-10-26
- Delivery: Implemented initial T01 pitch/order variant; physical MIDI review pending
- Related: [T01 catalogue](../piano-exercise-catalog.md#t01--five-finger-flow), [brief](../design/five-finger-flow.md), [ADR-0027](0027-keyboard-explorer.md), [ADR-0018](0018-scored-sessions-and-adaptive-review.md)

## Context

The owner requested merging N02, working directly in main, and implementing the
next TODO exercise: T01 Five Finger Flow. This introduces ordered patterns rather
than another single-note question. Intermediate notes must not award completed
trials or erase an earlier mistake/assistance flag.

## Decision

`core/sequence.rs` owns a reusable, clock-independent SequenceRunner: exact note,
ordered cursor, wrong input and full-release gating. It reports accepted/released
steps separately from accepted/released whole patterns, including consecutive
repeated pitches. Empty/invalid plans reset safely. `core/five_finger.rs` owns the
original major pentascale patterns, deterministic level selection and advisory
fingers. `Practice` delegates sequence state through waiting_release()/wrong()
accessors; the runner owns those flags for sequences, and existing note/chord/
Explorer semantics remain unchanged. Skip/configure reset the sequence cursor.

T01 uses nine notes: degrees 1–2–3–4–5–4–3–2–1. Levels are Right C4, Left C3,
and moving C/G/F positions alternating suggested hands/registers. Fingering is
1–2–3–4–5–4–3–2–1 for right and 5–4–3–2–1–2–3–4–5 for left. F-position Bb is
spelled correctly in the UI. These are instructions, never inferred physical
hand/finger evidence. Legato overlap, both hands together, metronome pacing and
rhythmic assessment are not included in this first variant.

A training trial is one complete or skipped nine-note pattern. Training.answer
rejects intermediate steps at the domain boundary. The presentation lifecycle
keeps mistakes, assistance and response duration across intermediate releases;
it resets only after a whole pattern or skip. Existing 100 first-try / 40 corrected /
0 assisted-or-skipped scoring applies once per pattern. Sessions contain 12
patterns, not 12 individual notes. Correct steps can produce input feedback and
streak effects but reference audio and repeated held inputs never produce trials.

Runs use `mode = five-finger-flow` and versioned trial evidence
`flow:1:<L|R>:<nine exact MIDI pitches>`. Rust/native and browser validation check
version, hand instruction and the complete major up/down pattern. SQLite tables
and schema version remain unchanged. Old records remain readable; old apps may
omit the newly unknown mode. Whole completed/skipped patterns are durable history;
a partial in-session cursor is retained through pause, not restored after restart.
Practice again restores the new exercise and difficulty, starting a fresh pattern.

## Alternatives and consequences

Treating each note as a normal note-reading trial would overcount results, clear
mistakes/hints mid-pattern and lose the sequence identity. A song/transport-based
implementation would mix musical input gating with audio scheduling and imply
rhythmic evaluation. A generic scripting framework is unnecessary; the small
sequence runner is renderer/storage independent and usable by later scale work.

The destination note names and suggested fingers are visible task instructions.
Hint adds keyboard destination glow; Hear auditions each new step. Both mark the
whole pattern assisted. Difficulty/Hear use the existing shared training defaults.
Ordered patterns do not enter the reading/chord adaptive target picker.

## Validation

Core tests cover order, wrong octave/extra keys, repeated pitches, full release,
whole-pattern scoring, sticky assistance, partial skips, reset, fingering, spelling
and exact bounded evidence. SQLite tests export/reopen and reject malformed patterns.
The real Bevy browser test sends simulated MIDI through a complete 12-pattern
session, reloads saved evidence, retries, switches levels/positions, skips midway,
pauses a partial sequence and checks mobile/short menus. Real P-45 use and physical
GPU/audio latency remain separate verification work.
