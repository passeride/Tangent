# Tangent product strategy: an instrument-first music game

- Status: Proposed product strategy for review; not implementation approval
- Date: 2026-10-26
- Audience: product, UX, engineering and contributors
- Evidence: repository main at 5987914 (2026-10-26), product docs and implementation entry points; see [evidence audit](competitive-benchmark-2026-10.md)

## Product definition

**Tangent turns time at a real piano into visible, rewarding musical progress.**
It is a piano practice environment with an optional rhythm-game layer: authentic 88-key playing, readable sheet music, imported personal repertoire, self-paced learning, and measurable performances. It should feel satisfying within the first minute but remain useful when a player progresses beyond falling notes.

Tangent is **not** a five-button rhythm-game port, an automated piano teacher, a repertoire licensing marketplace, or a substitute for musical judgment. Its differentiator is the bridge from arcade immediacy to real keyboard and notation literacy, with local ownership of music and history.

The product loop is: **Choose a meaningful goal → Play a small attempt → Understand what happened → Retry the relevant passage → Notice improvement → Choose the next challenge.** Decorative effects are a reward channel, not the definition of progress.

## Existing product: verified/documented foundation

| Capability | State on inspected main | Evidence / caveat |
| --- | --- | --- |
| Bevy single-surface web and native app | Implemented; native/hardware verification pending | README, ADR-0005/0012 |
| Falling blocks, staff lanes, falling noteheads, sheet/playhead | Implemented, MusicXML detail varies; MIDI score view is approximate | docs/piano-player.md |
| MIDI, MusicXML and bounded MXL import; library search and local storage | Implemented subset | docs/music-library.md, docs/musicxml-import.md |
| Live keyboard/MIDI input and external MIDI output selection | Implemented adapters; physical P-45 needs verification | ADR-0015/0016 |
| Notes/chords training, hint, reference audio, adaptive review | Implemented | docs/practice-progress.md |
| Song pitch-coverage scores, local session history, practice points | Implemented; **not rhythmic accuracy** | ADR-0018; tangent-core/src/progress.rs |
| 25–200% playback speed, saved preference | Implemented | ADR-0020 |
| Reactive particles, streak feedback and reduced motion | Implemented initial system | ADR-0017, docs/hanabi-vfx.md |
| Calibrated rhythm judgment, A–B loops, hands selection, goals, mastery | Not established as delivered; proposed | docs/TODO.md and code/docs review |
| Cloud accounts, leaderboards, multiplayer, content marketplace | Not part of present product | docs/practice-progress.md |

**Audit limitation:** repository/documentation inspection is not hands-on usability testing. A feature being implemented does not mean its experience is validated; do not infer physical-MIDI reliability or accessibility compliance.

## Four experience modes; two scoring contracts

1. **Play (song exploration):** choose a file and enjoy notes/audio at any speed or view. Assists are welcome. No competitive judgment.
2. **Practice (guided mastery):** train notes/chords or work on song passages. Loop, slow down, isolate hands and repeat. Score learning effort and improvement without treating help as cheating.
3. **Perform (deliberate challenge, future):** clearly enter an unassisted, fixed-rules run. Show fair note/rhythm metrics *only after* calibration and validation. Never silently grade a Practice session as competitive.
4. **FreeForm (discovery):** improvise, see played notes/chords and tentative fingerprint matches; never interrupt flow with unsolicited ratings.

Existing Main labels Play, Train and FreeForm are not a mandate for an immediate rename. **Perform** is a proposed future entry after evaluator readiness; a song's Practice affordance can appear inside Play first.

## Product principles and trade-offs

- **Real instrument first.** Do not compress piano to five lanes. Keyboard geometry, octave and notation matter.
- **Make mastery tangible, not compulsory.** Personal bests, clear objectives and section improvements beat compulsory login, daily pressure or public rankings.
- **Fast feedback, quiet reading surface.** Juice lives at the strike line, keys and post-phrase moments; staff and upcoming notes stay unobstructed.
- **Truthful metrics.** Pitch coverage is labelled pitch coverage; a streak is not a perfect performance; practice tempo and assistance travel with any comparison.
- **Recoverable sessions.** Pause, missing output, storage failure and partial results tell the truth and preserve user agency.
- **Accessibility is a design constraint.** Not just colour; adjustable density, reduced motion, keyboard focus and readable staff.
- **Offline/local-first.** No account required to play, practice, import or inspect history. Preserve export paths.
- **Architecture boundaries.** Evaluation and scoring live in musical core; Bevy owns presentation; platform adapters own device/file/storage; one Bevy surface.

## Opportunity statement

**Current weakness:** Tangent can show what to play and store outcomes, but does not consistently answer 'what should I do next?', 'where did I improve?', or 'which four bars should I repeat?'. The jump from input feedback to an intrinsically motivating practice loop is our highest-leverage UX gap.

**First product bet:** passage selection + loop + hands/part selection + 'retry weak passage' from results, accompanied by a clearly differentiated learning/performance HUD. This is more defensible than introducing public leaderboards before timing calibration.

## Guardrails and non-goals

No dark-pattern streak loss, fictional AI grading, paid content promises, copied Clone Hero art, blanket claim that MusicXML is perfectly engraved, or global scores for runs at different speed/assistance/latency. Do not silently choose the wrong hand from arbitrary MIDI parts: demand metadata or user mapping and offer an 'all parts' fallback. User-provided music is not automatically distributable.

## Product health questions

Instrument connection success, time from launch to first useful note, fraction of users reaching a first complete attempt, attempts per weak passage, change in weak-passage hit rate, voluntary return after seven days, and complaint rate for misleading scoring. These are proposed measurements, not measured baselines. Prefer privacy-preserving local analytics; any cross-user telemetry requires an explicit separate decision.

## Related work

- [Benchmark and evidence](competitive-benchmark-2026-10.md)
- [Personas and jobs](personas-and-jobs.md)
- [Journeys and stories](user-stories-and-journeys.md)
- [Reward/feedback rules](gamification-and-feedback.md)
- [Prioritized roadmap](ux-roadmap.md)
- [UI v2 implementation contract](tangent-ui-v2.md)
