# UX roadmap: foundation → mastery loop → deliberate challenge

- Status: Proposed prioritization, 2026-10-26; relative sequence **not** a calendar commitment
- Planning unit: testable vertical slice, not a large unreviewed redesign
- Dependencies: UI v2, ADR-0018 score semantics, ADR-0020 playback speed and one-Bevy-surface contract
- Priority meaning: P0 highest user benefit / correctness, P1 next, P2 later, P3 conditional research

## Priority 0 — close the practice loop (next implementation branch)

**Milestone R0: reliable entry and feedback**
- UX-01/06/07/08: verify Connect/Continue/sample path; surface solid-vs-glow meaning, accurate device states and metric names; manually validate P-45.
- Fix contradictions between old UI v2 checkboxes and delivered score/tempo; treat code/tests as evidence before marking further work complete.
- Output: a first-run smoke script, device-state map, accessibility baseline and four real-app screenshots.
- Exit criteria: user starts music without a dead end and can interpret a result; no false score/audio state.

**Milestone R1: passage practice MVP (highest-value bet)**
- UX-02/03/04: A–B time loop with optional MusicXML measure snapping; hand/part filtering with an honest ambiguous-MIDI fallback; existing speed control in the same flow.
- UX-05: Results → Practice this section, initially only where actual note evidence supports identifying a section.
- Output: focused practice slice with playhead/loop boundary UI; core loop and MIDI-state tests; result CTA.
- Exit criteria: P1 repeats a selected 10-second passage three times at 70% and then 80% without resetting to the beginning or leaving stuck notes; keyboard and MIDI output both coherent.
- Dependencies: map loops in authored repeats consistently; distinguish source time from playback time; cancellation/resync for MIDI outputs; do not invent measure numbers for plain MIDI.

## Priority 1 — guided improvement and visible progress

**Milestone R2: guided learning**
- UX-09/10/11/13/15/16: optional wait-for-note, one next-task recommendation, comparable passage history, small voluntary goals and contextual resume.
- Preserve distinction between reference sound and actual answer; assistance must remain traceable.
- Exit criteria: user identifies the weak passage, starts the next appropriate attempt, and can explain how the two results compare; help does not count as an unassisted score.
- Data note: new progress granularity likely needs a versioned SQLite migration and pure core data model before UI polish.

**Milestone R3: scalable, legible presentation**
- UX-12/14/18: Focus/Standard/Arcade HUD, auto-fit/offscreen hints, accessible effect variation and 4K sizing.
- Exit criteria: same piece remains readable at 720px and 4K, 49/61/88 keys, three themes, reduced-motion/effects-off; noteheads and strike line never covered; no frame stutter from dense chords.
- Dependency: responsive/layout telemetry or repeated visual verification, platform profiling, retained existing Bevy rendering.

## Priority 2 — a differentiated long-term practice product

**Milestone R4: notation progression and safer data**
- UX-17/19/20: block-to-notation bridge, robust local backup restore, contextual non-blocking fingerprint candidates.
- Validation: compare with teachers/beginners; distinguish partial MusicXML coverage and approximate MIDI spelling. No automatic public content sharing.

## Priority 3 — performance and community, explicitly gated

**Milestone R5: score integrity**
- UX-21/22: design and implement timing evaluator after acoustic/MIDI calibration study. Test latency envelopes, device classes and reproducibility. Version score contract and normalize eligibility.
- Exit criteria: independent domain tests and repeatable real-hardware timing judgments; scores say exactly what they measure; source/tempo/mode/rules stored.

**Milestone R6: social play**
- UX-23/24 only after user research, privacy/licensing/security work, account strategy and moderation/cheat prevention. Offline mode remains first-class.
- A public leaderboard is **not** the current P0 feature.

## Prioritization rationale

| Opportunity | User pain | Learning value | Dependencies | Decision |
| --- | --- | --- | --- | --- |
| Passage loop + hand isolate | High, blocks focused practice | High | Medium | **Do first** |
| Results → next attempt | High, closes loop | High | Medium | **Do first** |
| Transparent onboarding/score | High, protects trust | High | Low/medium | **Do first** |
| Adaptive goals and progression | Medium/high | High | Medium | Next |
| More particle presets | Medium for arcade users | Low alone | Low | Pair with clear evaluation |
| Public leaderboard | Segment-specific | Unproven | Very high | Defer |
| Social feeds/marketplace | Unknown | Unproven | Very high | Out of current scope |

## Validation gates (no invented baselines)

- **First useful note:** measure median launch→first intentional input; target to test, not claim: under 2 minutes for a new MIDI user, under 45s returning user.
- **Focused retry:** percentage of test users who can loop and repeat a weak section without instruction; target hypothesis ≥ 4/5.
- **Result comprehension:** at least 4/5 test users correctly explain pitch coverage vs rhythm accuracy.
- **Readability:** zero notation-occlusion failures in scripted responsive themes/effects matrix.
- **Trust:** zero known cases of replay/reference counted as user input and no false 'Saved' state.
- **Technical:** deterministic core tests, web/native tests, P-45 manual check for scheduled MIDI cancellation and output switching.
- **Return/retention:** review voluntary return and beneficial repeat attempts only after a consented study; no dark-pattern daily-streak mandate.

## Implementation breakdown and ownership

| Workstream | Likely owner module | Notes |
| --- | --- | --- |
| Loop/range and transport state | tangent-core transport + player controller | Avoid UI-owned timing |
| Track/hand mapping | imported song/notation domain + per-song preference | Do not infer hands without metadata |
| Passage statistics | tangent-core progress + versioned storage | Score semantics, comparable context |
| Device/resync | platform audio/MIDI adapters | Browser/native separate, shared contract |
| HUD/results | tangent-player Bevy scenes | One in-engine UI, no HTML menus |
| Visual FX | scene/effects consuming evaluator events | Can be disabled without semantic loss |

## Required decisions before implementation

1. How is a 'section' identified across MusicXML repeats versus flattened MIDI time?
2. Should performance scores at 60/70/120% ever share PB categories?
3. Is a MIDI input latency calibration reliable across native/web/device changes?
4. How should arbitrary MIDI parts map to hands without deceptive inference?
5. Which passage-level mistakes are actually available from stored trials?

Record decisions that alter timing, scoring or persistence in new ADRs; update the [UI v2 brief](tangent-ui-v2.md) and [research plan](ux-research-plan.md). No code changes are authorized or claimed by this roadmap.

## Delivery update — 2026-10-26

ADR-0021–0023 deliver initial time loops and measure-visit snapping, explicit
imported parts, per-attempt context/evidence, contextual retry/comparison,
Focus/Standard/Arcade HUDs, keyboard fitting and voluntary session-local goals.
This does not close all R0–R3 gates: per-staff/voice targets, guided waiting,
persisted resume/goals, physical P-45/GPU checks and participant research remain.
See [execution-plan log](product-development-plan.md#implementation-log--2026-10-26).
