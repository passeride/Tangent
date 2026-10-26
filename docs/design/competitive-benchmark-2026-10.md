# Competitive UX benchmark — October 2026

- Status: Research synthesis / heuristic analysis, **not** direct user testing
- Reviewed: 2026-10-26
- Method: public first-party feature descriptions and Tangent repository/documentation inspection
- Source discipline: observations are attributed; recommendations are Tangent design hypotheses. No screenshots, music, UI assets or source code are copied.

## Comparison matrix

| Product | First-party observation | UX mechanism worth studying | Tangent interpretation |
| --- | --- | --- | --- |
| **Clone Hero** | Custom songs, note streaks, live scoring, leaderboard-aware modifiers, on-screen input view; 2026 v1.1 added rankings and new result detail | One glance answers 'am I doing well?'; compelling anticipation, impact, PB chase; optional online competition | Borrow visible milestones, tactile hit feedback and exciting results, **not** five-lane abstraction or online-first ranking |
| **Synthesia** | Falling notes or notation; waits for right note; hands separately; instrument plays background; immediate/long-term progress and song organization | Beginner enters quickly, then reduces assistance; practice can target a hand | Add song-side guided practice with explicit hand/part assignment and wait mode, keeping classical notation credible |
| **Piano Marvel** | Adaptive sight-reading assessment, progressive level difficulty, practice and history graph | Real challenge ladder tied to a narrowly defined skill | Build transparent note/chord skill progression; never claim standardized sight-reading scores without validation |
| **Yousician** | Guided learning plans, goals, progress tracking, instant accuracy/timing feedback, rewards and high scores | Next action is always legible; reward effort and improving technique | Build a small daily session recommendation and a specific next step on results, not a giant course marketplace |
| **flowkey** | Wait Mode, slow motion, loop section, hands separately, video/sheet view | Quick route from 'I cannot play this' to repeatable, bite-sized practice | A–B loop and isolate-parts beat additional menu polish for near-term user value |
| **Melodics** | Weekly streak based on practice across three days; XP, intentional grace/freeze | Motivation can be flexible instead of a punishing mandatory daily streak | If adding streaks, use forgiving weekly goals, offline opt-out and no guilt messaging |

## Evidence and nuance

**Clone Hero.** The [official 2026 v1.1 release notes](https://clonehero.net/2026/04/30/ch-v1.1-release.html) describe local vs online scoring, in-menu/in-song rankings, modifier-aware leaderboards, input trails and cleanliness bonuses. [Official site](https://clonehero.net/) describes broad instrument/game-controller support; [wiki gameplay explanation](https://wiki.clonehero.net/books/clone-hero-manual/page/how-to-play) describes hit/combo logic. Its large-gesture visual feedback is relevant, but piano uses many simultaneous keys and notation; one-to-one copying would harm readability. Observe that ranking by modified speed requires an explicit scoring contract, not just a number.

**Synthesia.** [Product overview](https://synthesiagame.com/) documents wait-for-note, separate hands, hints and progress; [song selection guide](https://ws.synthesiagame.com/support/guide/choosingSongs) explains song-level progress, search/sort; [song setup](https://www.synthesiagame.com/support/Guide/songSetup) explains part/hand mapping. Transfer the *task structure*, not branding.

**Piano Marvel.** [SASR](https://pianomarvel.com/en/feature/sasr) uses levels and recorded assessment history; [feature description](https://pianomarvel.com/en/new-features) differentiates practice, play and dashboard. Tangent's adaptive review is a task selector, not equivalent to a validated SASR ability scale.

**Yousician.** [Official product](https://yousician.com/) describes goal-driven lessons, rewards and realtime accuracy/timing. Tangent's song engine presently measures **pitch coverage** in a generous window; calling it timing precision would be a product error.

**flowkey.** [Official product](https://www.flowkey.com/en) explicitly lists wait, slow, loops and separate hands. These are concrete low-level interactions we should test in Tangent.

**Melodics.** [Official weekly streak specification, September 2026](https://support.melodics.com/en/articles/16979766-weekly-streaks) uses three practice days/week and a streak-freeze mechanic. This is inspiration for a low-pressure habit model, not evidence that streaks improve retention for Tangent's audience.

## Heuristic comparison on important jobs

| User job | Tangent today | Relevant benchmark | Priority |
| --- | --- | --- | --- |
| Start with my keyboard and music | Basic path available; hardware onboarding not validated | Synthesia onboarding; Clone Hero device mapping | P0 |
| Fix the three bars I keep missing | Speed control exists, passage loop/hand isolate missing | flowkey, Synthesia | **P0** |
| Know exactly why my score changed | Pitch coverage and practice point rules explicit, timing uncalibrated | Clone Hero per-input/stat HUD | **P0/P1** |
| Make practice feel worth returning to | History + adaptive exercise selection exist, goals and next action weak | Piano Marvel, Yousician, Melodics | **P1** |
| Compete under fair conditions | No competitive timing-calibrated contract | Clone Hero modifier-aware rankings | P3 |
| Discover/match music I play | Symbolic fingerprint prototype exists | No directly comparable benchmark in this set | Research first |

## Competitor-inspired hypotheses (not facts)

- H1: 'Retry this passage' on results causes more immediately productive second attempts than an isolated total score.
- H2: A discreet 2–4 beat anticipation cue + scaled impact improves perceived responsiveness without masking sheet notes.
- H3: Separating **Practice** from **Performance** reduces misunderstanding of help, mistakes, speed and personal-best eligibility.
- H4: Showing one recommended next challenge increases repeat sessions more than adding badges to the main menu.
- H5: A gradual opt-in notation path helps falling-block players build reading skill; forced notation would increase drop-off.

Test these with the [research protocol](ux-research-plan.md), not assumptions.

## What we should consciously *not* import

Do not adopt multiplayer, global rankings or punishments ahead of calibration; do not copy another UI skin, note highways, game art or fonts; do not make effects a proxy for correct technique; do not import licensed song catalogs. Keep Tangent's single Bevy surface and original visual identity.
