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
| [Piano player](piano-player.md) | Initial implementation | Owner-supplied sketch; not bundled | Four views, piano, sound and themes |
| [Tangent UI/UX v2](tangent-ui-v2.md) | Design direction approved; not implemented | Interactive concept reviewed 2026-10-25; not bundled | Menus, navigation flow, player HUD, settings categories and design principles |

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
