# ADR-0033: Written beam groups and shared chord stems

- Status: Accepted
- Date: 2026-10-27
- Extends: [ADR-0007](0007-authored-measures-repeats-and-music-glyphs.md)
- Related: [ADR-0032](0032-scalable-sheet-overlay.md), [brief](../design/grouped-notes.md)

## Context

The owner requested joined note groups (🎵). The score renderer previously gave
every short note one isolated flag, including chord members, obscuring both beat
groups and the difference between eighths and shorter values. Playback timing is
already independent of notation; this change must preserve that boundary.

## Decision

Preserve optional authored up/down stems and numbered MusicXML beam marks on
WrittenNote. Retain begin/continue/end and forward/backward hooks through 64ths
(levels 1–4). Validate beam number/value and duplicate levels; higher standard
levels warn and are omitted. Feathered beams warn and display ordinary beams.

`tangent-core::beaming` owns written grouping with no renderer/XML dependencies.
Its input is one NotatedMeasure; columns combine compatible simultaneous notes
in the same staff/voice. Complete contiguous explicit primary chains override
metric grouping. Without marks, group within denominator beats, or dotted beats
for compound meters. Rational comparisons preserve exact tuplet/beat boundaries.
Rests, gaps, conflicting stems, incompatible chord durations and barlines prevent
unsafe joins. Incomplete explicit chains keep individual flags. Numbered secondary
marks are respected; where absent, derive secondary connections/partial hooks
from written values. MIDI never acquires invented meter or beams.

The importer computes the BeamGroup plan once per written measure, with indices
into that measure's notes. Repeat visits reuse it; playback/performance notes and
source-time timing remain untouched. The derived plan is serialized with the
existing score payload; saved sources continue to be reimported, with no progress
or preference schema migration or fingerprint algorithm change.

The Bevy score module owns geometry: one stem per beamed chord, authored stem
direction or staff-relative pitch average, bounded slope and minimum stem length.
Dense onsets reserve note-head room before distributing remaining rhythmic space;
notes, tempo marks and playhead share that geometric map.
Each numbered beam is one rotated UI rectangle. Pooled shapes reset UiTransform
on every reuse so a beam cannot rotate an unrelated menu/key later. Existing
note heads/hand colours remain; common beams use theme ink. Standalone short
notes display the corresponding flag count. Scale and multi-system/overlay rules
are inherited unchanged. No extra canvas, HTML controls, renderer or dependency.

## Alternatives and consequences

Purely proximity-based joining would bridge voices, rests or beats. Renderer-only
grouping would bury musical rules in Bevy and repeat work every frame. Importing
an engraving engine is disproportionate to this bounded request. Explicit groups
win because they encode the author's rhythm decisions; metric inference is a
conservative fallback, not a complete music engraving algorithm.

Cross-staff groups, rest-spanning beams, feathering, adjacent-second chord collision
layout, double/no-stem notation and beams beyond 64ths remain follow-ups. Tuplet
brackets/numbers and complete ties remain independent. Malformed grouping does
not alter sound or prevent otherwise valid music from being played.

Primary reference consulted: W3C MusicXML 4.0 beam element and beam-value type:
https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/beam/
https://www.w3.org/2021/06/musicxml40/musicxml-reference/data-types/beam-value/

## Verification

Core tests cover simple/compound meter, rests/gaps, distinct staves/voices,
syncopations, rational tuplets, mixed durations/hooks, explicit overrides,
malformed chains, chord stems and direction conflicts. Import tests preserve
marks and repeated timing, auto-group missing marks and reject invalid marks.
The real WASM browser scenario imports original test music and checks shared
chord stems, both directions, 6/8, authored overrides, secondary/tertiary beams,
hooks, scaled/narrow systems and switching back to falling views. Screenshots
show the actual single Bevy canvas; see the brief for reproduction.

Delivery checks pass: all 134 workspace Rust tests; native/workspace and WASM
Clippy with warnings denied; formatting; release WASM build; actual browser beam
and independent-layer scenarios; baseline GPU/audio/transport smoke; packaged
`/Tangent/` smoke; native debug executable startup/import self-test. Desktop and
390px screenshots were inspected, including dense-note spacing. Physical device
latency and native GPU output were not newly measured.
