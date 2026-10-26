# Gamification and feedback design contract

- Status: Proposed design direction; musical scoring changes need their own ADR before implementation
- Date: 2026-10-26
- Prerequisites: [ADR-0018](../adr/0018-scored-sessions-and-adaptive-review.md), [ADR-0017](../adr/0017-practice-reference-and-motion.md), [UI v2](tangent-ui-v2.md)

## What 'fun' should mean

The core reward should be audible and visible competence on a real piano. Gamification amplifies effort, attention and improvement; it does not replace playing. Borrow Clone Hero's legible anticipation/impact, but use piano-specific polyphony, staff correctness and honest performance states.

## Three feedback timescales

| Time | Feedback | Design notes |
| --- | --- | --- |
| 0–150 ms perceived | Keyboard filled state; strike impact; correctness cue | No claim of measured app latency; test end-to-end device/audio timing on hardware |
| 1–10 s | Current phrase goal, optional clean-note streak, subtle multiplier-style momentum | Only show streak based on evaluator events; do not conflate learning aid with perfect playing |
| End of passage/session | Improvement, missed locations, retry action, personal best under valid rules | Show *why* the outcome happened and one next action |

## UI composition: musical hierarchy

1. **Music plane:** notation/blocks, playhead and strike line occupy largest stable area; never obscured by particles.
2. **Playing plane:** full keyboard with solid currently pressed keys and edge glow for expected target; both states must coexist.
3. **Context plane:** title, measure/time, speed, device state, active loop/hand and pause.
4. **Gamification plane:** optional score/streak/progress at periphery; allowed to pulse but not displace notation.
5. **Review plane:** result details, passage heatmap and next actionable target, only between attempts.

Proposed HUD presets: Focus, Standard (default), Arcade. **Information density and effect intensity must be separate settings.** Accessible correctness cues include symbols/labels and motion-safe variants, never colour alone.

## Effect vocabulary

| Event | Visual/audio principle | Guardrail |
| --- | --- | --- |
| Anticipated target | Soft edge glow at key and readable onset line | Never fill an unpressed key |
| Played note | Solid key + localized transient | No flash covering accidentals/ledger lines |
| Correct hit | Radially varied, short burst anchored to correct key/onset | Seeded variability, bounded particle count, viewport-relative size |
| Wrong note | Distinct short amber outline/ripple + optional text 'Extra note' | No negative audio punishing a beginner; user toggle |
| Phrase completion | One modest flourish + 'Phrase complete' or accurate percent | Only after real phrase closure |
| Personal best | Results-only celebration after a comparable record | No fabricated PB for interrupted or assisted run |
| Streak | Peripheral meter and intensity step at explicit thresholds | Streak definition visible; do not state 'perfect' without rhythm grading |

Effects should be deterministic enough for reproducible test fixtures yet vary across spawns. Distinguish visual randomization from *musical* evaluation. Cap overdraw at dense chords; normalize in logical viewport units for 4K; preserve 49/61/88-key visibility and scaling. Keep effects off the staff read zone. Maintain reduced-motion and off toggles.

## Separate score vocabularies

**Current implementation:**
- Reading/chord exercises: 100 clean unassisted, 40 corrected, 0 assisted or skipped; first-try percentage has explicit denominator.
- Song performance: 100 per unique matched non-percussion note and pitch coverage; generous matching window; not precise rhythm correctness.
- Streak can be shown as a feedback event; it is **not** equivalent to music mastery.

**Proposed future Performance:**
- Clearly shown timing tolerance after measured latency calibration, with per-note onset windows and handling for holds/sustain, polyphony, rests, re-attacks and device-class differences.
- Separate fields: pitch hit coverage, onset distribution, extra/wrong notes, releases/holds when reliable, assistance, selected part, source identity, playback speed and judge version.
- Practice score and Performance score **must never share** a global ranking scale without explicit eligibility rules.
- Speed-adjusted runs are either separate modifier classes or ineligible for unmodified PB; decide before launch.
- No external leaderboards until integrity/abuse/privacy architecture exists.

## Motivational systems (in priority order)

1. **Passage goals:** three clean repetitions of four bars; maintain a percentage at 70%, then raise to 80%; show a specific target.
2. **Personal improvement:** '2 more matched notes in bars 9–12 at 70%' with identical scoring context.
3. **Skill path:** clef/level-specific practice tasks; recommend next exercise based on observed history, never hidden unexplained level assignment.
4. **Milestones:** completing a first imported song, five practice sessions, first passage without aids, optional badges.
5. **Weekly practice goal:** opt-in, forgiving, can count 3 of 7 days; no punishing daily loss.
6. **Competitive systems:** later and opt-in, after fair scoring and consent.

No XP for accidental autoplay, repeated assisted reference, background device traffic or idle time. The player can disable streak/badge surfaces. Respect progress saved/not-saved feedback.

## Results screen sketch (content, not UI asset)

- Piece/part, mode, 70% speed, any aid and local-save status.
- Primary factual metric (currently Pitch coverage or Practice first-try).
- *What improved:* compare compatible runs only.
- *Where to practice:* highlight the worst measured range; do not infer from missing note timing when not recorded.
- Primary action: **Practice this section**. Secondary: Replay, Library, View details.
- If analysis is insufficient: say 'No passage detail available yet' and offer replay, not fake heatmaps.

## Implementation boundaries

The score/evaluation model belongs in **tangent-core**, independent of Bevy. Persist the context needed for honest comparison in versioned local storage. The Bevy scene renders state; backend platform adapters expose capabilities. Do not insert game rules in the particle system, UI widget handlers or raw MIDI device callbacks. A new evaluator or schema boundary requires an ADR and migrations. See [story IDs](user-stories-and-journeys.md).
