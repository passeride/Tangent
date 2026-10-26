# ADR-0019: Automatic practice reference audio

- Status: Accepted
- Date: 2026-10-26
- Delivery: Implemented; real-device audio remains a hardware check
- Supersedes: ADR-0017's one-shot Hear target interaction
- Related: [ADR-0017](0017-practice-reference-and-motion.md), [ADR-0018](0018-scored-sessions-and-adaptive-review.md)

## Decision

The practice Hear button is a persistent On/Off toggle. Turning it on auditions
that target and each subsequent note/chord automatically, including session
start, skip, replay and curriculum changes. A missing `practice_hear` preference
defaults to Off; song audio remains independent. SQLite schema is unchanged.

The shared session presentation tick tracks run ID, trial index and target keys,
so consecutive equal pitches are distinct tasks and frames cannot retrigger an
audition. Playback waits for an active, focused, unpaused exercise with no held
keys. Resuming an already heard task does not replay it. Turning Hear off releases
reference voices immediately. Existing reference IDs, 700 ms release and
input/pause/focus cancellation are retained, with no hardware MIDI echo.

References still mark their task assisted under ADR-0018: zero reading-score
points. They never enter input evaluation, advance a task or award a streak.
The alternative of only replaying on click does not meet the requested flow.

## Validation

Legacy and round-trip preference tests; native/WASM checks; real browser MIDI
checks for automatic next-target note-on/off, no frame retrigger, no self-answer,
Hear-off suppression, persistence and first-target playback after reload.
