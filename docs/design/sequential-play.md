# Sequential Play

- Status: Implemented, 2026-10-26
- Owner request: first walkthrough of a piece, waiting for correct input
- Architecture: [ADR-0029](../adr/0029-sequential-play.md)

Enable Sequential in Play's Pause header or Settings → Practice, then Resume.
Notation, timeline and automatic sound stop at each pending note/chord. A small
status strip names required pitches; essential key glow shows the current target.
Correct pitch/octave advances. A chord requires fresh attacks held together;
repeated pitches require release/replay. A preheld key is explicitly identified.
Wrong keys produce existing feedback while the target stays available.

Early input is 0/100/250/500 ms (default 250). This lets a slightly early correct
attack count; once stopped there is no late limit. It is independent of tempo.
Manual pause remains explicit, with no advancement until Resume. Disable
Sequential to return to continuous playback. Selected part, A–B loop and speed
remain available; retries restore all recorded settings.

Desktop, touch and MIDI use the same musical gates. Short windows use three
settings columns; narrow screens use one. No new external assets or imagery.
Screenshots show the real app with an original synthetic five-target fixture:

- [Repeated-note release instruction](../screenshots/sequential-play.png)
- [390×844 Practice settings](../screenshots/sequential-settings-mobile.png)
- [1280×420 Practice settings](../screenshots/sequential-settings-short.png)

Results describe pitch coverage after waiting. These attempts compare only with
the same sequential mode/window, source, selection and speed. This does not
measure timing, fingering, physical hand use or pedal technique.

Verification: `cd web/piano && node guided-smoke.mjs`, or run
`TANGENT_CAPTURE_GUIDED=1 node web/piano/capture.mjs` from the repository root.
Physical device evaluation remains pending.
