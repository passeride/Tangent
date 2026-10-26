# ADR-0018: Scored sessions, local history and adaptive review

- Status: Accepted
- Decision date: 2026-10-26
- Delivery: Implementation and verification in progress
- Related: ADR-0009, ADR-0015, ADR-0017 and the UI v2 design brief

## Context and decision

The owner requests scores, progress and further practice. The shared Rust musical
core owns score meaning, session records and review selection. Platform adapters
only validate and persist records; Bevy presents real results within its existing
surface. There is no server, account, public leaderboard or invented performance.

Reading/chord sessions have 12 tasks, plus explicit Skip and Finish. A clean
unassisted answer earns 100 points, a corrected answer 40, and a skipped or
hint/reference-assisted answer 0. First-try percentage counts clean unassisted
answers over all attempted tasks (including skips). All-key release gates task
advancement. Wrong attacks are counted separately; partial correct chords are
not mistakes. A skipped task is not solved. Assistance is task-local and cannot
be undone by hiding a hint. Pause/configuration/reference playback never answer
an exercise. Active-screen response times are informational, not timed bonuses.

Sequential practice remains available. Optional adaptive review picks weaker or
unseen task skills from the chosen clef/level, avoids the two immediately recent
tasks, and uses recent saved history plus the current session without double
counting it. This is deterministic practice scheduling, not validated mastery or
an adaptive-learning certification. Natural/chromatic notes and diatonic,
major/minor and dominant/minor-seventh chord pools are supported. Clef, level and
review preference persist; answers and transport are never auto-restored.

Song playback earns 100 points per unique non-percussion target hit using the
existing generous pitch window. Result coverage is hits/total targets, not a
claim of calibrated rhythm accuracy. Wrong attacks and missed targets remain
visible. Pause preserves target identity. Seek/restart closes a partial run;
scoring a fresh piece starts from its beginning. Natural completion shows results.
BLAKE3 of the exact source bytes identifies a piece for recent personal-best
comparison across restarts and targets. Metadata/title edits do not change that
identity. Different source files/arrangements remain distinct; this does not
replace the separate melodic fingerprint matcher. The existing pinned BLAKE3
dependency is now shared with WASM, with no new dependency version.

## Persistence and lifecycle

SQLite v5 adds practice_details, keyed by the existing practice_sessions FK,
with versioned checkpoint payload, revision and a numeric update timestamp. The
existing scores table contains named points/solved/clean/errors/elapsed_ms metrics.
Both adapters use atomic parameterized transactions. Repeated writes replace the
same session; an older revision cannot overwrite newer progress. Acknowledged
revisions remove matching pending snapshots only; failed writes remain visibly
unsaved and retry while the app runs. Navigation saves partial sessions. Checkpoints
are sent after answers and periodically during songs. Empty abandoned sessions
are not history. Results never promise durability before acknowledgement.

The database retains history; the UI/review loads the latest 200 sessions to bound
payload size. This is labelled recent progress rather than all-time mastery. Rows open full result/task details;
practice replay restores its recorded clef/level, while song replay requires the
exact piece to be loaded to avoid playing a different file under an old result.
Unexpected tab/process termination can lose the latest unacknowledged checkpoint;
there is no browser-unload durability guarantee. Full-image browser SQLite writes
remain an existing scaling limitation. No new dependency or third-party asset.

## Validation

Pure tests cover score/assistance/skip semantics, adaptive selection, record bounds
and unique-target feedback. SQLite tests cover v4 migration, idempotency, stale
writes, invalid payloads, FKs and export/reopen on both adapters. Real Bevy browser
tests exercise a complete scored session, results, restart/history persistence,
assisted answers, corrected input, adaptive tasks, song outcomes and failed saves.
Physical MIDI and native GPU/latency validation remain separate manual checks.
