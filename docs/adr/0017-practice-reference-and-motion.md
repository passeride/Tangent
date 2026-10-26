# ADR-0017: Practice reference audio, live pitch pointers and motion preferences

- Status: Accepted
- Decision date: 2026-10-25
- Delivery: Implemented; physical P-45 and high-resolution GPU verification pending
- Related: [ADR-0015](0015-midi-output-and-reading-practice.md), [ADR-0016](0016-persistent-midi-output-selection.md), [UI v2](../design/tangent-ui-v2.md)

Reference interaction superseded by [ADR-0019](0019-automatic-practice-reference.md).

## Context and alternatives

Reading practice needs an optional audible reference and the location of the
actual played pitch. Correct/incorrect text alone cannot show the size of a
pitch error. A reference must not answer its own exercise, count toward streaks,
fill played keys or echo hardware input. Stronger effects must preserve the calm
musical hierarchy and offer a reduced-motion alternative.

## Decision

A separate practice presentation module owns staff layout and live pitch
pointers. The existing renderer-independent Practice model retains all target,
clef and answer rules. Played notes use the pressed-key theme colour in a second
column on the same staff as the target. Ledger positions are exact, including
wrong octaves. Staff spacing fits the held pitches rather than clipping or
transposing them. Text identifies played notes and signed semitone distance for
a single-note answer; colour is not the only feedback channel.

Hear target explicitly auditions the target through the selected output using
reserved reference IDs and existing live-note routing. It never enters the held
input/evaluation path. A 700 ms presentation timer releases those IDs; new player
input, pause, navigation, configuration changes and focus loss cancel them.
Hardware input retains Local Control/no-echo behavior from ADR-0015. The reference
is optional and does not reveal the written answer as a hint or advance targets.

The core owns song-hit timing and target identity: reattacking the same target
is neutral, so repeated presses cannot inflate a streak. Seek/restart clears hit
identity. This remains cosmetic feedback, not calibrated timing scores.

Effects retain shared grades and bounded emitters. Directional UI sparks, bright
cores, fading trails and fast-expanding arcs strengthen attacks without camera
movement. Native Hanabi retains matching colours and a brief white attack.
Soft/Full/Punchy scales size and reach; reduced motion suppresses bursts and rings
on both targets while preserving static key/notation feedback. These additive
v1 JSON preferences have explicit legacy defaults and validation tests; no SQLite
schema migration is needed. Defaults preserve full effects and normal motion. Reading clef and difficulty
are also additive preferences: starting another session preserves them while
resetting answer/progress state. Previously those Settings controls were lost
when entering Train. Hint visibility remains per-answer and is not saved.

## Consequences and validation

Reference timing depends on UI frames; the existing platform panic/focus handling
remains essential, and a stalled renderer can extend an audition. This is not a
new real-time scheduler. No sound samples, third-party art or dependencies added.
Pure preference tests cover missing/invalid fields. Real WASM browser tests cover
reference MIDI on/off, no self-answer, actual MIDI pitch feedback, unchanged
no-echo behavior and saved motion/intensity settings. Physical output/speaker
behavior and native GPU appearance remain manual checks.
