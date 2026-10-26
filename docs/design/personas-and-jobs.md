# Personas and jobs-to-be-done

- Status: **Provisional hypothesis personas**, 2026-10-26
- Source: existing product goals and competitive patterns; **not** interviews or real user segmentation
- Use: guide flow review and testing recruitment; revise after observing users
- Priority: primary P1 and P2; secondary P3/P4

## P1 — The returning adult pianist (primary)

**Situation:** Has a digital piano, limited practice time, knows fragments of favorite pieces, may be rusty at notation.

**Job:** 'When I sit down for 15 minutes, help me make one hard passage noticeably better without learning a new app each time.'

**Goals:** Import own MIDI/MusicXML; resume; isolate troublesome bars; play at 60–80%; see objective progress; route output back to piano speakers.

**Pain:** Replaying the first minute to reach a difficult bar; dense settings; meaningless numbers; hardware setup interrupts playing.

**Success:** Connects hardware quickly; fixes a passage with repeat and gradual speed-up; has an understandable personal record.

**UX requirements:** Low-friction Continue, A–B loop, saved practice parameters, clear piano/output states, local first. Do not award false rhythmic mastery.

**Recruitment check:** Can this user start a relevant session in under two minutes and repeat a passage three times with no external tutorial?

## P2 — The curious beginner (primary)

**Situation:** New to reading notation, motivated by a recognizable tune, may use 49/61 keys, computer keyboard or a modest MIDI controller.

**Job:** 'Let me succeed at something musical today, then teach me what the falling notes mean on the staff.'

**Goals:** Easy first success; selectable assistance; targeted note-reading; a safe feeling when wrong; visible next step.

**Pain:** Simultaneous information overload; octave confusion; music vanishes before finding a key; shame from repeated misses.

**Success:** Learns a short pattern, understands filled vs glowing keys, recognizes a few notes in treble/bass, willingly starts a second session.

**UX requirements:** Gentle onboarding, wait-for-note in Practice, optional note names, short levels with clear goals and immediate feedback, adaptive difficulty that explains itself.

**Recruitment check:** Does the user correctly identify what to press and what their actual input was on first exposure?

## P3 — The arcade/score chaser (secondary)

**Situation:** Enjoys Clone Hero/Rock Band-style feedback and repeating sections to raise personal bests; uses a real MIDI keyboard.

**Job:** 'Give me a visible challenge, instant honest feedback and a reason to play it cleaner next time.'

**Goals:** Pre-song challenge rules, responsive juice, combos/streaks, interesting result breakdown, local best, eventually ranked competition.

**Pain:** Ambiguous timing, inconsistent scores, unresponsive input, small effects on high-DPI screens, equal rewards for assisted and unassisted runs.

**Success:** Trusts the score and knows the next improvement target; can turn on a richer HUD without affecting notation clarity.

**UX requirements:** Dedicated later Performance rules, input calibration, tempo/assistance metadata, optional intense effects, deterministic scoring and fairness boundaries.

**Recruitment check:** Does the user distinguish pitch coverage from rhythm accuracy, and can they reproduce a result under the same settings?

## P4 — The teacher or parent (secondary)

**Situation:** Helps a learner on a shared machine, favors musical growth over high-pressure scoring.

**Job:** 'Give us a short, repeatable exercise and let me see progress without encouraging shortcuts or confusion.'

**Goals:** Choose an appropriate note/chord level; inspect wrong attempts and help usage; protect the child's confidence; export local history.

**Pain:** Shallow badges, assisted answers scored as mastery, accidental deletion or mixing histories, unreadable reports.

**Success:** Can assign a ten-minute task, explain the results and find the next weak skill.

**UX requirements:** Explicit assistance accounting, focused results, optional goals and future local profiles. **Profiles, child controls and teacher dashboards are not implemented and require privacy/product design.**

## Cross-persona jobs and tensions

| Job | P1 | P2 | P3 | P4 |
| --- | --- | --- | --- | --- |
| Start in < 2 minutes | Essential | Essential | Essential | Essential |
| Isolate difficult bars | Essential | Useful | Essential | Essential |
| Immediate impact + rich HUD | Optional | Low density | Essential | Optional |
| Learn staff literacy | Useful | Essential | Optional | Essential |
| Fair comparable scoring | Useful | Optional | Essential | Useful |
| Offline, no account | Essential | Useful | Useful | Essential |
| Correct MIDI/device state | Essential | Useful | Essential | Useful |

**Design tension:** P3 wants a dramatic highway and dense stats; P2/P4 need uncluttered notation and low cognitive load. Resolve with **Focus / Standard / Arcade presentation presets**, not different semantics or a single compromise cluttered view. Effects and HUD density are independent choices.

## Exclusion and accessibility considerations

Test keyboard-only and mouse/touch fallback, colour-vision differences, reduced motion, narrow 720 px view, 4K/high DPI, left/right-hand use, Web MIDI denied, no MIDI device, screen magnification and auditory reference disabled. A user without MIDI must still be able to learn, but physical skill scoring must not pretend touchscreen performance is equivalent.

## Unknowns to validate

Actual device distribution, notation fluency, reasons for practice abandonment, whether weekly goals motivate or discourage, desirability of competitive play, and whether users interpret note colours correctly. See [research plan](ux-research-plan.md).
