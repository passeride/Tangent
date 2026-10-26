# Scores, progress and practice

Choose **Train → Read notes / Play chords** for a 12-task session. Finish ends a
session early; Skip records an unsolved task. Correct answers remain on the staff
until all keys are released. The coloured live pointer and reference audio remain
available. **Hear: On** plays each new target automatically through the selected
output. It is saved between sessions; **Hear: Off** stops reference audio.
Heard tasks count as assisted (zero reading-score points). Practice works with MIDI, computer keys and the on-screen piano.

## Score meaning

| Task outcome | Points | First-try credit |
| --- | ---: | --- |
| Correct without wrong input or help | 100 | Yes |
| Correct after a wrong attack | 40 | No |
| Correct after Show hint or Hear: On | 0 | No |
| Skipped | 0 | No |

Hiding a hint does not undo help already used. Partial correct chord shapes are
not errors; unrelated notes are. First-try percentage divides clean unassisted
answers by recorded tasks: completed answers and explicit skips. Unfinished
wrong attacks remain in the separate error count of a partial session. The results screen separates points,
solved tasks, wrong attacks, assistance and skips. Early finishes remain partial
sessions; no empty abandoned session is invented as practice history.

## Adaptive review and curricula

Review switches between Sequential and Adaptive; the choice is saved. Adaptive
review prefers weaker or unseen tasks from the selected clef/difficulty and avoids
the two immediately recent tasks. It uses recent history plus the current session
without counting a checkpoint twice. It does not silently promote difficulty.

- Notes: treble, bass or mixed clefs; stepwise, mixed natural or chromatic notes.
- Chords: diatonic triads, major/minor triads in all 12 roots, then dominant/minor
  seventh chords. Inversions and octave doublings are accepted.

**Progress** on Main/Train shows recent sessions, points, first-try rate and review
focus. Select a row to inspect its task outcomes; Practice again uses its recorded
clef and difficulty. A song result offers replay only when the exact file is loaded,
otherwise Choose music opens the library. Previous/Next pages through the latest 200 sessions. This is recent learning
feedback, not a validated mastery rating or an all-time leaderboard.

## Song scores

Start Play at the beginning to create a scored run. Each unique non-percussion
target hit earns 100 points. Same-pitch/same-onset unisons across parts count once. The generous existing matching window measures pitch
coverage, not calibrated rhythmic precision. Wrong attacks and targets not hit
remain visible. Automatic song audio cannot earn points. Pause/resume retains hit
identity; repeatedly pressing one held target does not increase the score.

Seeking closes a partial run. Restart starts a fresh run if already playing;
resuming after a seek in the middle does not invent a full-piece score. Natural
completion or Finish session in Pause opens results. Recent personal bests compare
the exact source-file identity, independent of title metadata. Different files or
arrangements are distinct; melodic recognition is a separate feature.

## Saving and limits

SQLite v5 migrates existing settings/library/fingerprints without deleting them.
Session checkpoints are atomic and keyed by session ID plus revision. Duplicate
or older requests do not double-count history or overwrite newer progress. Named
metrics are stored in the existing scores table; detailed tasks in practice_details.
No data leaves this device/profile.

Results show Saving / Saved locally / Not saved. Failed snapshots remain in memory
and retry while the app runs; the UI marks them unsaved. Native normal shutdown
flushes its final checkpoint. Abrupt termination/browser unload can lose changes
that have not been acknowledged. Export database remains the portable backup;
clearing browser storage removes the local database. Backup restore is still a
separate backlog item. The database keeps history while UI/review loads 200 runs.

The implementation is covered by pure domain tests, native/browser SQLite tests
and real Bevy browser integration, including simulated save failure/recovery.
Physical P-45 sound/latency and native GPU behavior still need hardware checks.
See [ADR-0018](adr/0018-scored-sessions-and-adaptive-review.md).

Reproduce scored-session screenshots with `TANGENT_CAPTURE_PROGRESS=1 node web/piano/capture.mjs` after building WASM.
