# Tangent UX research and validation plan

- Status: Proposed, 2026-10-26
- Aim: test product hypotheses and prioritize with evidence; not a user-study report

## Small formative study

Recruit **6–8 people** across P1 and P2 plus at least one P3 and one P4; use a mix of MIDI keyboards (ideally a Yamaha P-45), no-MIDI users, beginner/advanced readers, 1080p/4K and narrow-window setups. Do not use a convenient single tester as evidence for everyone.

Provide a bundled legally distributable study plus one imported MIDI/MusicXML; note where score notation is authoritative versus approximate. Observe tasks without coaching first; then collect severity, confusion and qualitative feedback. Get explicit consent for notes or screen recordings; avoid recording personal repertoire or MIDI history by default.

## Five core tasks

1. Start Tangent, get sound and press the right note; explain filled key vs glowing key.
2. Import or retrieve music, select 70% speed and find the difficult section.
3. Loop the same short passage three times; if proposed features are only mocked, test prototype and label it as hypothetical.
4. Complete a note-reading task with Hear active and explain the score/assistance.
5. Inspect song results, distinguish pitch coverage from rhythm accuracy, and choose what to practice next.

For arcade-oriented participant add a HUD preference/effect-intensity task; for teacher add 'What does this summary let you infer about learning?'.

## Record for each task

Completion without intervention; elapsed task time; number of wrong navigations; first hesitation point; observable device/audio failures; whether the participant correctly interprets metrics and states; verbatim quotes tagged anonymous; visual scaling/occlusion issue. Distinguish **prototype predictions** from real running-app measurements.

## Specific probes

- Is the word 'Train' clearer than 'Practice'? Would 'Perform' falsely suggest audio recording?
- Is a 12-task session motivating, too long, or too disconnected from a chosen song?
- Which effects are delightful vs distracting at high DPI and dense chords?
- Does 'Hear target' feel like help or autoplay? Do users expect its score to count?
- When opening a 60% speed session, which score do users expect to compare?
- Is keyboard hand mapping understandable for arbitrary MIDI arrangements?
- Is a weekly habit goal motivating or intrusive?

## Synthesis and decision rule

Tag findings by severity (blocker, major, minor, preference) and persona. Prioritize repeated blockers and trust errors over aesthetic preferences. Publish a short evidence table with participant counts and direct quotes, update personas, and revise the story/roadmap only when evidence warrants it. A target not met is a finding, not an excuse to change metric naming.

## Technical validation matrix

| Dimension | Minimum cases |
| --- | --- |
| Presentation | 720p narrow, 1080p, 4K/HiDPI; Aurora/Ember/Paper; Focus/Arcade proposal |
| Inputs | Real MIDI, PC keyboard, mouse/touch; disconnect/reconnect |
| Audio | Browser permission denied, internal synth, external MIDI, pause/seek/loop |
| Content | Valid MIDI, MusicXML with repeats, MXL, sparse/dense chords, malformed source |
| Accessibility | Reduced motion, effects off, keyboard focus, colour-blind-safe cues |
| Persistence | Restart, interrupted session, storage unavailable, export and recovery |
| Metrics | Unassisted, corrected, reference-assisted, partial, altered speed |

## Source and privacy guardrails

All research artifacts should be original, with appropriate participant consent. Do not record identifying player data, publish copyrighted imported repertoire, or embed competitor screenshots/assets without rights. Browser-local SQLite history is not telemetry and must not become one silently.
