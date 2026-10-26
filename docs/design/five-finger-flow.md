# Five Finger Flow (T01)

Status: initial pitch/order implementation, 2026-10-26.

## Goal and task

Build familiarity with a five-note position by playing up and back down:
C–D–E–F–G–F–E–D–C. Train → Five Finger Flow displays all nine steps and suggested
finger numbers. A marker follows the current step; completed steps remain visible.
Release every held key before the marker moves. This is deliberately a detached,
beginner input exercise; legato overlap and timing are later variants.

| Level | Position | Suggested fingers |
| --- | --- | --- |
| Right C | C4 major pentascale | 1 2 3 4 5 4 3 2 1 |
| Left C | C3 major pentascale | 5 4 3 2 1 2 3 4 5 |
| Moving | C4, G3, F4, C3, G4, F3, repeating | By suggested hand/register |

MIDI verifies exact notes/order/release; it cannot verify actual fingers or hands.
F major displays Bb. No rhythmic precision, tempo consistency or mastery is claimed.

## Scoring and help

One pattern is one trial: 100 first try, 40 after any corrected attack, zero for
hint/reference assistance or skip. An error/help on an early step still affects
the final pattern score. Twelve patterns complete a session. A partial sequence
can be skipped and the next always starts at its first note. Finish saves completed
or skipped patterns; partial correct prefixes do not masquerade as solved tasks.

Visible note/finger labels are the teaching stimulus. Hint reveals the current
keyboard destination; Hear auditions each new step without playing the sequence
for the user or advancing it. Reference sound never counts as input. Pause retains
the in-session step while releasing voices. History/retry retains mode/difficulty
and exact pattern evidence; restart/retry begins a new pattern.

## Presentation and checks

Use the same Bevy surface, 3D piano, theme colours and reduced-motion settings.
Nine compact tiles fit mobile; short windows reduce tile height and text. Finish,
level/hint/Hear and Skip remain reachable without covering the pattern. Capture
actual UI with `TANGENT_CAPTURE_FLOW=1 node web/piano/capture.mjs` after building
WASM. Check whole-pattern counting, partial skip, pause/release, durable retry,
all three levels, Bb spelling, old exercises and 390px/420px-high windows.

See [ADR-0028](../adr/0028-five-finger-sequences.md) for ownership and persistence.
