# User journeys, user stories and acceptance criteria

- Status: Proposed backlog for review, 2026-10-26
- Personas: [P1–P4](personas-and-jobs.md)
- Product contract: [product strategy](product-north-star.md)
- IDs are stable references for planning; no story is claimed delivered just because the underlying subsystem exists.

## Journey A — first success (P2, beginner)

Launch → choose Try sample or Import → optional MIDI connect (with fallback) → choose Guided Play → explain glow vs solid → first four-note success → simple results with next step → retry or try staff.

**Failure branches:** device permission denied or no output; show 'Use computer keys / on-screen piano' and how to retry MIDI. Audio locked until gesture; offer explicit Enable sound. If no saved piece exists, no fake Continue item.

**Moment of delight:** a correct phrase gets a proportional musical/visual response, not an intrusive modal. **Moment of trust:** incorrect pitch is shown in its actual octave and staff position.

## Journey B — fix a difficult passage (P1, returning pianist)

Main/Library → Resume piece → select bars 9–12 or A–B timeline range → choose right hand, 70%, loop and count-in → attempt → simple hit/miss annotations → retry and optionally increase speed → finish → compare passage to prior attempt at the same configuration.

**Failure branches:** no reliable written measure information for MIDI: use time range, not fabricated bar numbers; unknown hands: let user assign tracks or practice all parts; missing MIDI: keep settings and fall back.

**Critical metric:** time from recognizing difficult passage to second focused attempt.

## Journey C — fair score chase (P3, future)

Select piece → opt into Perform → calibrate or acknowledge output/input latency status → show song/rules and eligible modifiers → play without hints → show real-time streak if evaluator supports it → result separates pitch, timing, misses, extras and assists → compare only compatible sessions → replay weakest section in Practice.

**Failure branches:** unsupported judge/calibration: Performance unavailable, not guessed; interrupted/restarted/seeked run: mark partial/ineligible; speed changed: new comparison bucket, not a hidden universal best.

## Journey D — exercise and feedback (P4, teacher/parent)

Choose Train (clef and difficulty) → optionally hear each target (marked assistance) → short session → inspect clean/assisted/corrected/skipped results → replay suggested skill next time. Local history, no online sharing presumed.

**Failure branch:** current shared-machine history is not per-user. Do not infer a child profile or assign names before a profile feature exists.

## Prioritized user stories

| ID | Priority | Story | Acceptance criteria and edge cases |
| --- | --- | --- | --- |
| UX-01 | P0 | As P1/P2 I see a truthful first-run route | Main offers sample/import/continue only for resolvable local data; no dead ends; MIDI denial offers fallback |
| UX-02 | P0 | As P1 I can loop any passage | Set/drag A and B on timeline, show boundaries, one-button repeat, A < B, cancel on source change, no duplicate stuck MIDI notes; fallback to seconds without score measures |
| UX-03 | P0 | As P1 I can isolate a hand/part | Select explicit MusicXML staff/voice or mapped MIDI part; background audio remains optional; unmapped/ambiguous files never auto-label wrong hand |
| UX-04 | P0 | As P1 I can use saved tempo for a loop | Existing 25–200% rate applies across repeats and MIDI/audio/visuals without changing pitch; changing rate never changes source score; tempo represented in session metadata |
| UX-05 | P0 | As P1 I can retry weak material directly | Results reference recorded source and timestamp/measure range; open practice focused on valid source; if unavailable, explain and offer Library |
| UX-06 | P0 | As P2 I understand two key states | Actual presses solid, expected notes glow; simultaneous states legible with effects off, in three themes, high DPI and reduced-motion |
| UX-07 | P0 | As P1/P3 I can interpret a result | Existing song metric is explicitly pitch coverage; practice first-try and help rules are separate; interrupted/unsaved runs labelled; no timing accuracy claims |
| UX-08 | P0 | As P1 I can recover from a MIDI failure | Input/output states separate; device loss pauses or changes output safely, all notes off, retries or computer fallback shown, no accidental echo |
| UX-09 | P1 | As P2 I can enable wait-for-note | Guided Practice stops musical advancement until intended input, with safe chord-release behavior; reference output never earns score; off by default in Perform |
| UX-10 | P1 | As P2 I get a useful 5–10 minute challenge | Appropriate achievable skill goal; review offers one next exercise based on measured recent attempts, not fabricated mastery |
| UX-11 | P1 | As P1 I see session-vs-session passage progress | Same file identity + passage + scoring mode + tempo/assistance recorded; show trend only if comparable, else side-by-side descriptors |
| UX-12 | P1 | As P3 I can choose readable or arcade HUD | Focus/Standard/Arcade density; no overlays over score/strike line; intensity independent from information; settings persistent only with migration |
| UX-13 | P1 | As P1/P2 I can set a practical target | 'Practice 10 minutes' or 'repeat four bars three times' locally; no obligation to maintain a daily streak or network access |
| UX-14 | P1 | As P1 I can see off-screen pitches | Auto-fit/zoom control and out-of-range hint; never silently drop played/expected notes |
| UX-15 | P1 | As P1 I can return to my work | Last playable file + practice context restored when safe; no auto-play or stale held notes; source deletion handled gracefully |
| UX-16 | P1 | As P4 I can compare skills honestly | Drill-down displays assisted/first-try/corrected/skipped; reference audio marked assist; optional filtering by clef/level |
| UX-17 | P2 | As P2 I advance through readable notation | Optional progressive block→staff hints; at no point is incorrect/inferred musical spelling presented as authoritative MusicXML |
| UX-18 | P2 | As P3 I get satisfying but controlled impact | Effects vary naturally, normalized for viewport size; audio cue optional; rate-limited and reduced-motion safe; no occlusion of key/staff |
| UX-19 | P2 | As P4 I can back up practice history | Database integrity checked before restore; user confirms destructive action; clear origin-local scope and error recovery |
| UX-20 | P2 | As P1 I can identify a song from playing | Candidate labels and uncertainty; distinct from exact source identity; never jump to a result without consent |
| UX-21 | P3 | As P3 I can earn a trustworthy timing grade | Calibration documented and reproducible; onset windows tested, chord overlaps/sustain/duplicate inputs specified; no grade from mere pitch coverage |
| UX-22 | P3 | As P3 I can earn a compatible personal best | Eligibility requires unassisted validated run, exact chart identity, judge version, device class and speed/modifier policy; partial runs excluded |
| UX-23 | P3 | As P3 I can opt into a public leaderboard | Server-authoritative validation, permission, privacy, abuse controls, licensing and account deletion plan before any launch |
| UX-24 | P3 | As P4 I can use separate local profiles | Explicit identity isolation, migration and export, shared-device UX; child-data/privacy review before implementation |

## Acceptance examples (Given / When / Then)

**UX-02:** Given a 4-minute song and selected A=01:15, B=01:25, when playback reaches B, then the engine returns to A without stuck MIDI notes or score duplication; when MIDI is disconnected, the loop remains visible and device recovery is offered. Given A >= B, when accepted, then validation prevents starting an invalid loop.

**UX-03:** Given an imported MIDI with unlabelled tracks, when I press 'Right hand', then Tangent does not silently guess which notes form a hand; I can assign a track or choose All parts.

**UX-07:** Given a saved song score that checks only matching pitches, when Results opens, then the label says 'Pitch coverage', not 'Timing accuracy' or 'Perfect run'. A 70% speed run is visibly tagged.

**UX-09:** Given guided note input with reference sound enabled, when the target arrives, then the reference may play through selected output but does not answer itself; when the user plays it, advancement waits for full release.

**UX-12:** Given a 4K display with effects enabled, when a correct note lands, then relative impact and staff legibility are consistent with 1080p; given reduced motion, essential strike and correctness cues remain.

## Delivery checks for every story

- Behavior tests in core when musical meaning changes; device scheduling tests on both adapters when audio/MIDI changes.
- Bevy browser and native interaction tests; manual physical P-45 MIDI input/output and disconnect when relevant.
- Keyboard/pointer/touch and narrow/high-DPI review, plus effects-off and reduced-motion.
- Verify data migrations and graceful offline/storage failure.
- Never silently create performance claims; ensure text and displayed values use the same evaluator.

Read the staged [roadmap](ux-roadmap.md) for sequence and dependencies.

## Initial software coverage — 2026-10-26

| Stories | Delivered initial slice | Remaining qualification |
| --- | --- | --- |
| UX-02/04 | A–B loops, drag/non-drag controls, source-time speed, authored visit snap and saved attempt rate | Physical MIDI boundary verification |
| UX-03 | Explicit imported-part targets and honest All parts fallback | Per-staff/voice identity and user-defined MIDI hand mapping |
| UX-05/07/11 | Actual missed-target retry, honest contexts and compatible previous-attempt deltas | Cross-source reopening and richer trends |
| UX-12/14 | Independent persisted HUD density, fixed keyboard fitting and offscreen counts | Broader accessibility semantics and physical 4K profiling |
| UX-13 | Optional session-local 3×80% pitch-coverage goal; no autoplay/partial credit | Persistent/time-based goals |

These are software delivery statements, not participant-study findings or claims
that every acceptance gate is closed. Read ADR-0021–0023 and the execution log.
