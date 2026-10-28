# ADR-0037: Evaluate established notation and UI libraries

- Status: Proposed
- Created: 2026-10-28
- Decision date: Not decided
- Delivery: Research and future-work documentation only
- Related: [assessment](../design/refactoring-and-library-review.md),
  [display/library brief](../design/display-scaling-and-library-layout.md),
  [ADR-0005](0005-one-bevy-gpu-surface.md),
  [ADR-0032](0032-scalable-sheet-overlay.md),
  [ADR-0033](0033-written-beam-groups.md),
  [ADR-0036](0036-rhythmic-reading-and-note-glyphs.md)
- Supersedes: None

## Context

The owner reports engraving/colour inconsistencies, small UI on 4K and a library
list using only a fraction of the screen. Tangent owns substantial custom score
layout and absolute UI/widget behaviour. Existing fonts and engine primitives do
not replace an engraver or automatically create responsive forms. Source review
confirms a six-row presentation cap; actual hardware DPI behaviour is unmeasured.

## Options

1. Continue local geometry/widgets, first fixing row extent and shared UI metrics.
2. Delegate authored notation layout to Staveloom or Verovio behind an adapter;
   compare OSMD as a reference/web alternative.
3. Reuse Bevy's existing layout/scale capabilities for menus/forms, or evaluate
   bevy_egui/egui tables in a single Library/editing experiment.
4. Replace the complete UI/runtime. Disproportionate to present evidence and at
   odds with the one-Bevy-surface direction; not proposed as the next step.

The linked assessment covers capabilities, platform/license costs, SVG composition
and smaller rfd/RustySynth/notify opportunities. These are candidates, not adopted
dependencies or verified performance claims.

## Decision

No backend selected. First address global display metrics and viewport-filling
Library design independently of the engraving experiment. Compare Staveloom and
Verovio with identical scores, source-ID/time mapping and real Bevy native/WASM
composition; record results before accepting a replacement. A separate widget
comparison may establish whether native Bevy widgets or egui best serve forms.

The proposal retains Tangent's source identity, transport, scoring and persistence
as authoritative. No generated engine MIDI replaces playback during visual work.
Visible UI remains inside the Bevy window/canvas. Any browser engine capability
amendment must be explicit; no second DOM UI or browser-backed desktop runtime.

## Consequences

The plan avoids a coupled renderer/UI/audio rewrite, but requires adapter/mapping,
cache and input-coordinate work. Dynamic colours and cross-staff ownership need
explicit rules regardless of engine. Library fullness and readable UI should not
wait for notation replacement. Existing ADRs remain in force until implemented
changes explicitly amend them.

## Validation and follow-up

See the assessment's staged experiments and the display brief's DPI/viewport
matrix. Require actual captures, native and web builds, bounded resource use,
license inventory and unchanged audio/scoring/source identity. No library was
built or integrated for this documentation change; no runtime fix is claimed.

## References and provenance

Primary sources and review date are listed in the linked assessment. All new
text is project-authored from the owner's reports, this repository and upstream
public documentation. No excluded implementation, assets, test music or new
runtime dependency was imported.
