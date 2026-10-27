# Product and visual design

This directory holds Tangent's own design briefs and explorations. The first piano design study has an implemented browser slice. Product goals
beyond that slice remain proposals; see each brief for evidence and limitations.

## Start a design study

Copy [the brief template](template.md) to `short-topic.md`. State the problem,
user flow, important states and what feedback the design should answer. Link
wireframes in [sketches](sketches/README.md), asset records in
[assets](assets/README.md), and any relevant [ADR](../adr/README.md).

| Brief | Status | Sketches | Outcome |
| --- | --- | --- | --- |
| [MIDI input and piano shortcuts](midi-input-and-piano-shortcuts.md) | Bindings implemented; input dropdown proposed | Device dropdown, calibrated command mode and shortcut labels | Input selection feasibility and implementation acceptance |
| [Hand assignment and colour](hand-assignment-and-colour.md) | Demonstration/editor slice implemented; broader inference proposed | Editable source-based hand map, uncertainty and semantic colours | Foundation for hand colours and later one-hand practice |
| [Editable themes](editable-themes.md) | Initial implementation | Versioned JSON, RGBA tokens and semantic hand colours | Persistent editor and import/export |
| [Piano player](piano-player.md) | Initial implementation | Owner-supplied sketch; not bundled | Four views, piano, sound and themes |
| [Tangent UI/UX v2](tangent-ui-v2.md) | Menus/pause/settings implemented; HUD and practice iterating | Interactive concept reviewed 2026-10-25; not bundled | Menus, navigation flow, player HUD, settings categories and design principles |

## Current UX product research (2026-10-26)

The research/personas remain hypotheses. The execution plan now contains dated delivery logs; accepted implementation decisions are in ADR-0021–0023. Do not interpret the whole roadmap as shipped:

| Document | Purpose |
| --- | --- |
| [Product north star](product-north-star.md) | Positioning, existing capability audit, product principles and learning loop |
| [Competitive benchmark](competitive-benchmark-2026-10.md) | Clone Hero, Synthesia, Piano Marvel, Yousician, flowkey and Melodics, with primary sources |
| [Personas and jobs](personas-and-jobs.md) | Four provisional personas and their conflicting needs |
| [User journeys and stories](user-stories-and-journeys.md) | Four end-to-end journeys and 24 prioritized acceptance-testable stories |
| [Gamification and feedback](gamification-and-feedback.md) | Feedback hierarchy, truthful score vocabulary, effects and game mechanics |
| [Product development execution plan](product-development-plan.md) | Concrete delivery slices, estimates, module ownership, acceptance gates and complete UX-01–24 traceability |
| [UX roadmap](ux-roadmap.md) | Milestones R0–R6, dependencies, gates and suggested next slice |
| [UX research plan](ux-research-plan.md) | Small user study, usability tasks, evidence collection and validation matrix |

**Read order:** product strategy → benchmark → personas → journeys/stories → feedback → roadmap. Research plan validates the hypotheses. Existing approved [UI v2](tangent-ui-v2.md) remains the current presentation contract; this new strategy proposes later product capabilities rather than silently rewriting it.

## Initial design questions

- **First use:** sound activation, device permissions, unsupported capabilities
  and getting started with keyboard/on-screen input.
- **Piano practice:** readable notes, bass/treble selection, chord presentation,
  input feedback, optional reference audio and scalable notation.
- **Music library:** import, search, persistence, progress and empty/error states.
- **Performance:** clear timing feedback, restrained effects, pause/recovery and
  a useful result screen.
- **Settings:** audio/MIDI status, calibration, themes, reduced motion and scale.

## Design principles to evaluate

Aim for a calm, legible musical workspace with clear visual hierarchy. Arcade
can be more expressive, but decoration must not obscure notes or timing. Test
small windows, touch targets, HiDPI, keyboard navigation and contrast. Color must
not be the sole carrier of correctness or state; provide text or shape feedback.

Record decisions and open questions after a sketch is reviewed. Do not label a
mockup as an implemented screenshot or claim device/browser support from an image.
Create original layouts; reference material needs a source and rights record.

- [3D piano stage](3d-piano-stage.md) — procedural lit keyboard, key motion and essential expected-note glow; see ADR-0026.

- [Keyboard Explorer](keyboard-explorer.md) — N02 keyboard orientation through exact octave transfers, levels and saved progress.

- [Five Finger Flow](five-finger-flow.md) — T01 five-note positions, suggested fingers and whole-pattern progress.

- [Sequential Play](sequential-play.md) — first-pass note/chord waiting with saved early-input tolerance.

- [Major Scale Run](major-scale-run.md) — S03 one-octave scales in 12 keys, advisory fingers and whole-scale progress.

- [Grouped notes](grouped-notes.md) — joined rhythmic beams and chord stems in the scalable MusicXML score.
