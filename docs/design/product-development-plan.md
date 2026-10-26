# Product development execution plan

- Date: 2026-10-26
- Status: Proposed execution plan; no runtime features delivered by this document.
- Baseline: strategy branch tree `9dc058d3fada9d0808d5fbf9e0a10b52dbab6a2c`.
- Scope: turn [R0–R6](ux-roadmap.md) and [UX-01–24](user-stories-and-journeys.md) into sequenced, reviewable work.
- Product priority: P1 returning pianist and P2 beginner first; P3 arcade and P4 teacher/parent remain supported without forcing competition.
- Existing [UI v2](tangent-ui-v2.md), one-Bevy-surface architecture and provenance boundaries remain in force.
- Planning estimates below are engineering days, not elapsed delivery dates or commitments. Hardware access and participant recruitment are external dependencies.

## Outcome and first release

The next release should let a player choose a real piece, repeat a difficult passage at 70%, isolate an explicitly selected part, understand the result and retry that passage at 80%. Music, notation and actual input remain more prominent than scores or effects.

Ship R0 and R1 as the first passage-practice release. R2–R4 build lasting learning value. R5 and R6 remain conditional; missing timing calibration must never block useful noncompetitive practice.

This request authorizes planning. Implementation work below is ready to pick up separately; nothing is marked implemented merely because it appears here.

## Evidence and concrete gaps

Inspected strategy, benchmark, personas, journeys, gamification, research plan, UI v2, README, provenance, ADR index, practice documentation, core transport/progress and ADR-0020.

| Existing evidence | Planning consequence |
| --- | --- |
| Core transport stores source microseconds and a bounded 25–200% rate; advance currently clamps at song end | Extend existing transport for loops; do not build a second UI clock |
| Run v1 stores source identity, totals and drill trials, but no passage range, selected part, speed or song-note detail | Add versioned context/evidence before passage comparisons or weak-section recommendations |
| ADR-0020 explicitly adds no score schema migration | A saved playback preference is not proof that historical runs recorded speed |
| Song results currently offer replay for the loaded exact source; other sources route to Library | UX-05 needs source resolution plus real passage evidence |
| Library, reference audio, output persistence, adaptive review and effects already exist | Integrate and verify these systems rather than schedule replacements |
| Physical MIDI/native reliability is still pending in repository evidence | Automated simulation and actual P-45 checks must be reported separately |
| UI v2 contains older proposed/unchecked wording alongside later delivery updates | R0 reconciles individual claims against code/tests; do not mark all checkboxes complete |

No runtime, device or participant study was performed for this plan. Benchmark claims remain sourced in the existing benchmark; this plan adds no new competitor claims.

## Delivery sequence

| Order | Slice | Depends on | Estimate | Release gate |
| --- | --- | --- | --- | --- |
| 1 | S0 truthful entry, device and result audit | Existing app | 2–4 days | G0 |
| 2 | S1 passage/run context and migration | S0 semantics | 3–5 days | Migration/evidence tests |
| 3 | S2 time-range loop and output resync | S1 context | 4–7 days | Loop contract on both adapters |
| 4 | S3 explicit parts + repeat-aware selection | S2, importer mapping audit | 4–7 days | No guessed hands or fake bars |
| 5 | S4 passage results and direct retry | S1–S3 | 3–5 days | G1, first release |
| 6 | S5 guided waiting and explained next task | S4 | 5–8 days | G2 |
| 7 | S6 comparable progress, goals and resume | S1, S4; goal integration after S5 | 4–7 days | G2 |
| 8 | S7 HUD, range visibility and effects | S0; integrate after S4 | 4–7 days | G3 |
| 9 | S8 notation bridge, restore and recognition UX | S6, S7 | Three independent 3–5 day slices | G4 |
| 10 | S9 calibrated timing and local PB | Separate feasibility study, S1 | Estimate after study | G5 |
| 11 | S10 optional social and profiles | Research and architecture gates | Estimate after scope decision | G6 |

First release estimate: 16–28 engineering days plus device checks and review. Re-estimate after S1/importer audit. Dependencies are mandatory even where work can be prepared independently. Prefer one cohesive PR per slice; split adapter/schema work when review size warrants it.

## S0 — trustworthy start and recovery (R0)

Stories: UX-01, UX-06, UX-07, UX-08.

Deliver:
- Main offers sample/import and Continue only when source resolution succeeds. Deleted source returns a clear Library action.
- Explain solid played keys versus expected edge glow on demand; retain both states with effects off.
- Separate MIDI input, selected output and actually active output states. Permission denied, missing/duplicate device names and disconnected output have actionable recovery.
- Results say Pitch coverage or Practice first-try; partial and Saving/Saved locally/Not saved states reflect actual events.
- Reconcile UI v2 status text and checkboxes; retain unresolved accessibility work.
- Capture four real app views: first use, passage-capable player baseline, MIDI recovery, results. Label capture platform and viewport.

Acceptance: first-use fallback works with no MIDI and denied permission; Settings returns to Pause; output switching never starts playback or echoes physical input; zero false saved/connected/perfect claims.

Owner modules: player menu/results/settings and platform adapters. Reuse current scoring and transport.

## S1 — passage identity, run context and evidence

Stories enabled: UX-04, UX-05, UX-07, UX-11, UX-15, UX-21/22 later.

Before code, record an ADR for passage identity, run segmentation and versioned persistence. Check the then-current ADR index for the next unused number.

Proposed contract to evaluate in that ADR:
- Exact source-byte identity remains authoritative. Fingerprint matches never substitute for source identity.
- A passage is a half-open range [A,B) of source-song microseconds on the expanded playback timeline. Optional authored measure/visit references explain it in the UI.
- Run context records passage, explicit part selection/mapping version, rate, mode, aids, evaluator version and input category. Future judge/device metadata is nullable until supported.
- A loop iteration is its own attempt. Aggregate repetitions separately; do not merge repeated target hits into a whole-song record.
- Changing source/range/part/rate/aid policy closes or segments the current attempt visibly.
- Record bounded per-target song evidence: target identity, onset/visit, selection and hit/miss state. Keep extras separate. Raw MIDI recording is unnecessary.
- Legacy runs without context remain readable but display unknown fields and are excluded from new comparable trends/PBs. Never default unknown historical speed to 100%.

Implement in tangent-core and versioned web/native storage. Inspect the live schema before choosing its next version. Preserve settings, imports, fingerprints and existing history; maintain atomic checkpoints and revision ordering.

Acceptance: migration fixtures for old/current data, rejected invalid ranges, duplicate/out-of-order checkpoints, interrupted writes and unavailable storage. Reload retains exact context; failed save never displays Saved.

## S2 — time-loop MVP (R1, first implementation priority after foundations)

Stories: UX-02, UX-04.

Deliver in Bevy: Set A, Set B, draggable boundaries, loop toggle, clear action, elapsed range and existing speed control in the same practice flow. Provide keyboard focus and equivalent non-drag controls.

Core owns validation and boundary transitions. Platform adapters consume discontinuity/resync events and cancel scheduled audio/MIDI. Preserve source pitches and source data. Use existing scheduling contracts where possible.

Specify before implementation:
- Reject A >= B and out-of-song ranges; expose readable validation.
- At B, release/cancel old voices and controllers, restore state at A and reanchor deadlines. Define crossing notes/sustain under existing seek semantics.
- Handle long frames and multiple-boundary overshoot deterministically; avoid an unbounded loop.
- Pause, blur, seek, device change and source replacement cancel stale scheduling. Source replacement clears the range.
- Each iteration resets only that attempt's target matching state. Autoplay/reference never contributes input.

Acceptance: repeat 01:15–01:25 three times at 70%, then 80%, with consistent audio/visual/MIDI positions, no stuck notes and no duplicate scoring. Test boundaries, sustained notes, pedal, chord overlap, final-song boundary, empty selected range and 25/200% rates.

## S3 — parts and authored repeat navigation

Story: UX-03; strengthens UX-02/04.

Audit MIDI and MusicXML import models for retained track/channel/part/staff/voice identity. Preserve missing identifiers in the importer/domain if required; do not infer a hand using pitch alone.

Deliver:
- All parts default; explicit part/staff/voice choice when supported.
- For arbitrary MIDI, track/channel labels and user mapping; ambiguous files say Unassigned.
- Selected targets determine evaluation denominator; backing audio is independent and clearly labelled.
- MusicXML selection exposes written measure plus repeat visit where needed. MVP selects one expanded occurrence; “all occurrences” is a later explicit operation.
- MIDI uses seconds where authored measures do not exist.
- Display an empty-selection state rather than a 100% result.

Acceptance: grand staff, multi-voice, unlabeled MIDI, same-onset unisons, percussion, repeats/first-second endings and missing metadata. Changing part invalidates stale targets and records a new context.

## S4 — results that lead to another useful attempt

Story: UX-05; completes UX-07 and first R1 release.

Use recorded evidence to suggest a bounded weak range. Publish deterministic selection rules in the core: eligible ranges, minimum evidence, tie-breaking and no-data fallback. Avoid calling a section weak based on totals alone.

Results show source/part/range/rate/aids, factual pitch coverage, misses/extras and save state. Primary action Practice this section opens the resolved exact source paused at A with range and context applied; Replay and Library remain available.

Acceptance: suggestion maps to actual recorded misses; deleted/replaced source offers Library; absent detail says No passage detail available yet; looping never produces a false whole-piece PB. Demonstrate three attempts without reopening settings or restarting the full song.

## S5–S8 — learning and presentation

| Slice | Stories | Deliverable and acceptance |
| --- | --- | --- |
| S5 Guided learning | UX-09, UX-10 | Core wait-for-note gate; simultaneous chord completion/release, repeated pitches and skip/escape specified. Output/reference cannot answer itself. Offer one explainable 5–10 minute drill from observed weaknesses, with an unseen-skill fallback |
| S6 Progress | UX-11, UX-16 | Compare only exact compatible contexts; show distinct first-try/corrected/assisted/skipped outcomes and clef/level filters. Unknown context yields descriptors, not improvement claims |
| S6 Goals/resume | UX-13, UX-15 | Optional active-practice or repetition goal; exclude idle/background/autoplay time. Restore source/range/part/rate safely paused; missing data is recoverable; no daily-streak penalty |
| S7 Presentation | UX-12, UX-14, UX-18 | Focus/Standard/Arcade proposal reconciled with UI v2 Detailed/Performance naming before implementation. HUD density independent of FX intensity; critical device state always reachable. Auto-fit/out-of-range hints never hide dropped pitches |
| S7 Effects | UX-18 | Consume evaluator events only; varied bounded seeded spawns, logical viewport scaling, capped dense-chord overdraw, readable staff, reduced-motion/effects-off parity |
| S8 Notation bridge | UX-17 | Optional progressive block/staff hints, exact octave and authored spelling preserved; MIDI inference identified. Validate with beginner and teacher before expanding curriculum |
| S8 Restore | UX-19 | Validate backup integrity/schema before replacement, retain recoverable original, confirm destructive restore, reject corrupt/newer unsupported databases with clear recovery |
| S8 Recognition | UX-20 | Show non-blocking uncertain candidates in FreeForm; distinguish matched work from exact arrangement/source; explicit action to open; never interrupt playing or auto-change song |

S5 waiting changes scheduling semantics: record an ADR and test browser/native transitions. New preferences and history fields need explicit defaults, migration policy and failure states; not every preference requires a new table/schema version if the existing versioned preference contract safely supports it.

## S9–S10 — conditional, not a prerequisite for passage practice

UX-21: first run a calibration feasibility study across browser/native, computer audio/external MIDI and representative hardware. Define measured onset windows, input provenance, polyphony, reattacks, sustain, holds/releases, rests and duplicate handling in an ADR. Publish repeatable measurements and limitations. If calibration is unreliable, keep timing grade unavailable for that capability.

UX-22: introduce local PB only after judge validation. Proposed comparison key: exact chart/source, passage/part, evaluator version, input/device class, rate and aids/modifiers. Exclude interrupted/seeked/assisted runs from unassisted records; keep other runs useful in practice history. Do not reuse legacy pitch coverage as a timing grade.

UX-23: a public leaderboard needs server validation, threat model, privacy/consent, accounts/deletion, anti-abuse, moderation and repertoire licensing decisions. Build no public submission path before these gates.

UX-24: local profiles are separately scoped and can precede public competition after identity isolation, migration/export and child/shared-device review. Do not force cloud accounts on offline users. Never attribute existing shared history to a child automatically.

## Acceptance gates and evidence

| Gate | Required evidence |
| --- | --- |
| G0 Trust | No-MIDI/denied-MIDI first-run smoke; correct state/result labels; four reproducible actual-app captures; device state-transition table |
| G1 Passage release | Core boundary/evaluation tests, adapter cancellation tests, migration tests; real Bevy web/native flow; P-45 loop/output/disconnect checklist; no stuck voices or autoplay points |
| G2 Learning | Release-gated chords/waiting; reference assistance traceable; compatible history after restart; goals exclude idle/autoplay; safe missing-source resume |
| G3 Readability | 720px narrow, 1080p and 4K; 49/61/88 keys; three themes; keyboard/pointer/touch; reduced motion and effects off. No occluded notation/strike line; profile dense chords and compare baseline |
| G4 Durable learning | Consented beginner/teacher observations, original backup survives restore failure, recognition uncertainty visible |
| G5 Performance | Reproducible hardware calibration and independent domain tests; explicit eligibility; changed device/rate/judge invalidates inappropriate comparisons |
| G6 Social/profile | Documented user need and privacy/security/licensing design; isolation/deletion/export demonstrated; offline flow retained |

For relevant implementation PRs run repository-prescribed formatting, focused tests, Clippy, native/WASM builds, browser integration and packaging checks from the current build docs. Web asset changes additionally require package_web.py, packaged /Tangent/ site smoke and embedded launcher smoke. Record exact executed commands and results; compilation is not physical-device evidence.

Use the existing 6–8 participant research plan. Hypotheses: first useful note under 2 minutes new MIDI / 45 seconds returning; at least 4/5 observed users can retry a passage without coaching and explain pitch coverage. These are targets, never measured results. Separate qualitative small-sample findings from retention claims. No silent telemetry.

## Complete requirements trace

| Requirement | Delivery owner | Gate |
| --- | --- | --- |
| UX-01 truthful first run | S0 | G0 |
| UX-02 loop | S2/S3 | G1 |
| UX-03 parts/hands | S3 | G1 |
| UX-04 tempo context | S1/S2 | G1 |
| UX-05 focused retry | S4 | G1 |
| UX-06 solid/glow | S0/S7 | G0/G3 |
| UX-07 truthful results | S0/S1/S4 | G0/G1 |
| UX-08 device recovery | S0/S2 | G1 |
| UX-09 wait-for-note | S5 | G2 |
| UX-10 short challenge | S5 | G2 |
| UX-11 comparable passage history | S1/S6 | G2 |
| UX-12 HUD presets | S7 | G3 |
| UX-13 voluntary goals | S6 | G2 |
| UX-14 offscreen pitches | S7 | G3 |
| UX-15 safe resume | S6 | G2 |
| UX-16 honest skill outcomes | S6 | G2 |
| UX-17 notation progression | S8 | G4 |
| UX-18 controlled impact | S7 | G3 |
| UX-19 backup restore | S8 | G4 |
| UX-20 recognition candidates | S8 | G4 |
| UX-21 calibrated timing | S9 | G5 |
| UX-22 compatible PB | S9 | G5 |
| UX-23 leaderboard | S10, conditional | G6 |
| UX-24 profiles | S10, separately gated | G6 |

## Risks and scope control

- Repeats/parts may lack imported identifiers: inspect before promising hand/measure UI; retain seconds/all-parts fallback.
- Loop resync can duplicate scheduled voices: treat it as a transport discontinuity with explicit cancellation, test each output.
- Legacy history lacks comparison context: preserve it and disclose unknowns rather than inventing migration values.
- Scene controllers are already large: split cohesive transport/practice presentation responsibilities when touched; keep musical rules in core.
- Hardware and research access affect validation: finish software evidence, list pending physical checks and never label the release hardware-validated prematurely.
- Avoid feature accumulation: each PR updates delivery docs and its story evidence. New particles, profiles or rankings must not displace the passage release.

## Ready-to-start first implementation PR

Suggested branch: `feat/passage-practice-context`, based on the current integrated main after reconciling this strategy branch.

Scope: S0 result/state contradictions plus S1 proposed ADR, core passage/context model and backward-compatible storage. Keep loop UI for S2. Include migration fixtures and an updated story/gate checklist. This establishes the information necessary to deliver retries honestly.

Planning completion check: all 24 stories mapped; R0–R6 sequenced; first release bounded; dependencies and unresolved decisions explicit; module ownership, estimates, risks and test/research gates supplied. Runtime implementation remains separate work.

## Implementation log — 2026-10-26

The owner subsequently authorized merging this plan and implementation on main.
Initial S1/S2/S4 behavior is implemented in ADR-0021: source-time A–B loop,
explicit imported parts, separate repetition attempts, context/evidence storage,
compatible recent pitch-score summaries and direct weak-range retry. Loop markers
are draggable; Set A/B and ordinary seek controls offer a non-drag path.

This is an initial passage-practice slice, not all of G1/R1. Remaining gaps at that first iteration (measure snapping was added below):
per-staff/voice targets, exact-source
reopening from history, full R0 first-run/Continue audit, and physical P-45/native
GPU validation. S5–S10 remain planned. Automated evidence is recorded in
practice-progress and the committed passage-smoke test; no participant study or
hardware measurements are claimed.


Second iteration: S7 now has independent saved Focus/Standard/Arcade HUD density,
explicit Fit keys and out-of-range warnings, plus short-window pause/settings
layouts. ADR-0022 records the decisions. This is initial UX-12/14 delivery;
comprehensive accessibility semantics and physical 4K profiling remain open.

Automated baseline for the first iteration: 91 workspace Rust tests, native/WASM
warnings-denied Clippy, full real-Bevy browser suite including three 70% loops,
80% segmentation, MIDI boundaries, contextual retry and drag, packaged /Tangent/
site smoke, native release executable/self-test and native archive packaging.
Physical P-45/native GPU and participant-study gates remain pending.

Second-iteration verification: 92 Rust tests; native/WASM warnings-denied Clippy;
real Bevy HUD persistence/range/4K/short-window test; existing player and passage
browser regression; packaged site smoke; native release and executable self-test.
Screenshots were inspected at 1280×800, 3840×2160 and 640×360. Software rendering
checks establish layout, not physical 4K frame-time or display-latency guarantees.

Third iteration: authored MusicXML snapping now uses explicit expanded measure
visits and labels repeated occurrences; MIDI remains time-based. Optional
session-local 3×80% pitch-coverage goals advance only from completed evidenced
user-input attempts, reset on context changes and never reward autoplay. Results
show matched-note change versus the previous compatible attempt. ADR-0023 records
the semantics. Per-staff/voice selection, saved resume, persistent goals and
guided wait-for-note remain separate follow-up slices.

Integration/performance follow-up: restored access to exercise defaults from
paused-song Practice settings (CI caught this regression); preserved the paused
run-ID assertion. Fixed press+release within one PC-input frame. Registered match
detail incrementally, bounded new detailed evidence at 20,000 hits while preserving
aggregate totals, and cached weak-range analysis by result revision/source.

Third-iteration local validation: 95 workspace tests, native/WASM warnings-denied
Clippy, full Bevy browser suite followed by focused progress/passage/HUD/goal
regression after the performance refinements, packaged /Tangent/ smoke, native
release/self-test and archive checks. The 20,001-target fixture confirms bounded
new detail, retained totals and legacy larger-record readability. Product captures
reproduce through TANGENT_CAPTURE_PRODUCT=1 node web/piano/capture.mjs. Physical
P-45/GPU testing and participant research remain separate gates.

Historical-result comparison follow-up: the previous compatible attempt is selected
by the nearest strictly earlier nonzero start timestamp, independent of history
ordering. Future attempts, equal/unknown timestamps and the same run are excluded;
compatibility still requires identical source, passage, tempo, aids and input.
This prevents opening an older result from reporting a newer attempt as previous.
Regression coverage includes unordered history and changed playback speed.
Local validation for this follow-up: all 34 tangent-core tests, warnings-denied
core Clippy, workspace formatting, diff checks and web/native progress-schema
parity tests pass. Full application builds/browser regression run in CI.

Input-control iteration (ADR-0024): added persistent action × Keyboard/Piano
bindings under Settings → Accessibility, low/high/custom piano control-note,
held and single-use prefix commands, conflict rejection, capture/clear/reset and
shortcut labels derived from the current bindings. Ordinary MIDI notes remain
direct musical input. The MIDI input dropdown remains an independent proposal.
Validation for ADR-0024: all 102 workspace Rust tests, native/WASM warnings-denied
Clippy, formatting, complete real Bevy browser suite, then focused final binding
regression (including wrong-mode command isolation and reconnect), packaged
/Tangent/ smoke and native debug executable self-test pass. Matrix screenshot was
inspected at 1280×800; bounds/overlap checks cover 640×360 and 390×844. Physical
P-45 usability/latency remains a manual check. No new dependency was added.
