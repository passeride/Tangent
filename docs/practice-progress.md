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

## Passage practice

In Play, pause at the desired start and choose **Set A**. Seek forward with the
existing timeline or +5 seconds, pause and choose **Set B**. Resume repeats [A,B)
at the selected 25–200% speed. Drag the A/B markers on the bottom timeline to
adjust a range; editing pauses playback. **Clear loop** returns to ordinary play.
Restart returns to A while a loop is active. Source replacement clears the loop.

**Targets** cycles All parts and explicit non-percussion imported parts. This
filters the scored targets and expected-key glow. The full score and optional
song audio remain backing context. This is part selection, not inferred hand
or per-staff separation. MIDI ranges use time; authored measure snapping and
repeat-visit selection are still follow-up work.

Each observed loop iteration is an independent attempt. Changing range, part,
rate or backing-audio policy and seeking closes the old attempt as partial;
pause/resume retains it. No targets in a selection means no scored run.
Results record source identity, range, part, tempo, backing audio, evaluator and
actual input categories. Only complete attempts with identical known contexts,
input categories and denominators contribute to compatible recent pitch-score
comparisons. These are not timing grades or competitive personal bests.

A completed attempt with recorded target evidence can offer **Practice this
section** for the exact currently loaded source. It picks the ten-source-second
range with most missed targets, earliest on ties, clipped to the attempt range.
The retry restores tempo, part, audio and range, paused for inspection. No missing
target evidence means no invented heatmap/recommendation. A different loaded
file requires Choose music; automatic cross-source resolution is not yet added.

SQLite remains schema v5: optional validated context/evidence fields live inside
its existing versioned JSON payload. Older records stay unchanged and unknown
context is visibly disclosed. No historical tempo is assumed. See
[ADR-0021](adr/0021-passage-practice-and-comparable-context.md).

## Readability and keyboard range

Settings → Visuals offers **Focus / Standard / Arcade** HUD density, saved locally.
Focus keeps playback/output/rate context; Arcade adds real pitch points/streaks.
Effects intensity and reduced motion remain independent settings. This does not
add a calibrated timing grade or change the score rules.

If selected music lies outside the visible keyboard, the player shows counts of
lower/higher notes and **Fit keys**. Fit chooses the smallest supported 49/61/88
range covering the selected pitches and current held keys. Pitches outside the
physical 88-key range stay flagged; no music is dropped. Short windows move
passage setup into Settings → Practice while retaining normal pause controls.

## Measures and repetition goals

For MusicXML, **Snap bars** expands a loop to its actual playback measure visits.
With no loop it selects the measure at the playhead. Repeated written bars remain
separate visits, shown in the range label. MIDI keeps time ranges and offers no
invented bar numbers. These controls also live in Settings → Practice.

After selecting a loop, **Goal** optionally asks for three complete attempts at
**80% pitch coverage**. Autoplay, partial attempts, empty selections and duplicate
checkpoints cannot advance it. Backing audio is allowed; wrong/extra attacks are
still reported separately. This is a coverage goal, not a clean-rhythm or mastery
award. Goal reached suggests a small speed increase without changing tempo.

Changing range, part, tempo or backing audio resets the goal's count. Clear loop,
source replacement or app restart removes it. Goals are session-local; individual
attempts remain in saved history. There is no daily streak penalty or XP bonus.
Results with an identical known context can now show the actual change in matched
notes versus the previous compatible attempt. Missing context produces no invented
comparison. Ctrl+O import works from paused sessions as well as live playback.

New attempts retain detailed onset/pitch hits up to 20,000 matches. Longer runs
keep accurate aggregate points/coverage but omit detailed weak-section analysis;
Results explicitly says when detail is unavailable. Older valid detailed records
remain readable. Repetition goals use actual evaluator totals and input even when
this detail cap has been exceeded. No historical data is deleted or rewritten.
