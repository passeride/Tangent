# ADR-0027: Keyboard Explorer octave-transfer exercise

- Status: Accepted
- Decision date: 2026-10-26
- Delivery: Implemented; real-device MIDI validation remains pending
- Related: [exercise catalogue N02](../piano-exercise-catalog.md#n02--keyboard-explorer), [brief](../design/keyboard-explorer.md), [scored sessions](0018-scored-sessions-and-adaptive-review.md)

## Context

The owner requested the next new exercise after the 3D piano. The catalogue's
N02 follows existing note-reading foundations and calls for keyboard orientation:
a marked source key, then identifying its name or finding it in another register.
Implement the latter variant without introducing a notation/spelling theory test.

## Decision

`core/explorer.rs` owns deterministic source/target tasks and canonical versioned
evidence. `Practice` uses an explicit Exercise enum instead of a chord/not-chord
boolean; existing note/chord matching behavior is preserved. Explorer reuses exact
pitch matching, full-release advancement and the existing 12-task scoring/session
lifecycle. `scene/explorer_view.rs` owns presentation only.

N02 uses three shared difficulty values: C/D/E upward transfers; all white keys
upward; all 12 pitch classes in alternating upward/downward transfers. Source and
destination are exactly 12 semitones apart and fit C2–C6. A labelled REF pointer
identifies the source; solid keys remain player input, and expected destination
glow appears only after an explicit hint. The destination name stays hidden until
hint/correct answer. Hear auditions the destination and marks that trial assisted.

Runs use `mode = keyboard-explorer` and trial evidence
`explorer:1:<source MIDI>:<target MIDI>`. Source/target fully identify this version's
generated instance. Rust/native and browser checkpoint validators accept the new
mode and reject malformed Explorer evidence. SQLite tables/schema version remain
unchanged; no legacy settings, library or scores are rewritten. New apps read old
note/chord/play runs. Older apps may omit the unknown mode when validating history;
this is not a promise of forward compatibility with older executables.

Existing 100 first-try / 40 corrected / 0 assisted-or-skipped scoring applies.
No timing, finger/hand identity or theoretical enharmonic spelling is judged.
Progress/results and Practice again retain the new mode and difficulty. Explorer
uses an ordered curriculum rather than the reading/chord adaptive target picker;
its skill namespace cannot contaminate that picker's existing candidate pool.
The global difficulty and Hear preferences remain shared with existing training.

## Alternatives and consequences

A name-choice quiz would need a separate spelled-pitch/theory answer contract;
that part of N02 remains future work. Implementing this as an imported song would
mix task state with transport and omit exact source/answer evidence. Adding another
boolean to Practice would allow invalid combinations; the enum makes the mutually
exclusive exercise selection explicit without introducing a generic framework.

## Validation

Core tests cover all three pools, deterministic evidence, exact octave, wrong
source/extra keys, repeated held notes, full release, assistance/skips and score
validation. SQLite export/reopen tests cover the new mode and malformed evidence.
A real Bevy browser smoke sends simulated MIDI, completes a scored session,
reloads persisted evidence, retries the exercise and checks 390px and short-window
layouts. Existing note/chord and input tests guard the enum/lifecycle refactor.
Physical P-45 validation remains distinct from simulated MIDI event tests.
