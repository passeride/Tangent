# ADR-0007: Authored measures, repeat visits and music glyphs

- Status: Accepted
- Decision date: 2026-10-25
- Delivery: Implemented; verification recorded in this change
- Extends: [ADR-0006](0006-musicxml-import-and-authored-notation.md)
- Presentation boundary: [ADR-0005](0005-one-bevy-gpu-surface.md)

## Context

The owner requests real G/F clefs, transparent clefs on the rotated staves and a
MusicXML demonstration with measures, endings and tempo displayed. A performance
list alone cannot relate a repeated passage to its written position. Letter G/F
placeholders are insufficient musical symbols.

## Decision

Keep written measures once in NotatedScore. Preserve inherited meter/key signature,
initial tempo, authored tempo marks, note values/dots, repeats and ending spans.
Add an explicit ordered list of MeasureVisit records, each linking a written
measure to its absolute performance interval and repeat pass. Visit-local BeatTime
points map exact quarter fractions to microseconds. Interpolation lives in the
pure core model; Bevy never independently reintegrates tempo. The importer remains
responsible for validation, bounded repeat expansion and sound events.

Expand sequential non-nested repeats (2–8 passes), including first/second and
comma-separated numbered endings. Support implied backward repeat from the prior
section start. Reject nested/unclosed repeats and malformed/unclosed endings.
Cap expansion at 80,000 visits, 250,000 notes and 24h. Jumps break pending sound ties;
contiguous written measures retain adjacent ties. Returning to a repeat target
restores its written tempo context. D.C./D.S./coda navigation remains unsupported
and produces a warning; do not pretend to support it.

The authored score renderer is a separate scene module. Show two measures on
narrow screens or four on desktop, with genuine G/F clefs, meter, key signature,
barlines, measure numbers, quarter-note tempo and volta brackets. The playhead
uses the current performance visit; repeated music returns to its original page.
Falling staff/notes use that same visit map for moving measure/beat guides and
ending brackets. Upright clef watermarks identify the two rotated staff bands.
MIDI retains its independent approximate time-spaced reading view.

Embed the unmodified Bravura OpenType music font from Steinberg (pinned source in
the asset register), SIL OFL 1.1. Load it directly into Bevy Assets<Font>; no OS font,
network font fetch, browser overlay or second renderer. Draw SMuFL clefs/accidentals/
rests with that font. Retain Fira Mono for UI text. Explicit font line height anchors
SMuFL baselines to staff lines. Preserve both font notices in every distribution.

Replace the externally sourced MIDI demo and synthetic runtime event generator
with one original MusicXML piano study. Both initial load and the demo button use
that same file/parser. It includes two staves, first/second endings, repeats,
4/4→3/4 and 96→108 BPM. Reproduction script and CC0 sample dedication are bundled;
this does not select a license for application source. Retain the retired MIDI's
historical provenance record without shipping the retired music file.

## Consequences and alternatives

Written notes may have multiple performance attacks. The old performance_note
field links the first attack for compatibility with existing consumers; authored
layout must use written notes and visits, not pretend one ID covers every pass.
Full engraving, nested navigation, ornaments, pedal/dynamics playback, clef changes
and MXL containers remain separate work. The renderer is readable measure layout,
not a complete engraving engine (collision avoidance, beaming and ties are basic).

Flattening repeats destructively into duplicated written measures would lose score
identity. Converting MIDI into XML cannot recover authoritative meter/endings.
Hand-sketching a treble clef or relying on system font fallback is less reliable
than a properly licensed music font in the existing Bevy surface.

References:
- https://www.w3.org/2021/06/musicxml40/tutorial/midi-compatible-part/
- https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/ending/
- https://www.w3.org/2021/06/musicxml40/musicxml-reference/elements/metronome/
- https://w3c.github.io/smufl/latest/tables/clefs.html
- https://github.com/steinbergmedia/bravura

## Verification

42 workspace Rust tests, native/WASM Clippy with warnings denied, the actual
release WASM in Chromium/SwiftShader, visible clef/measure/ending diagnostics,
real file import, audio, fullscreen, responsive pause UI, packaged /Tangent/ URLs
and the embedded desktop launcher smoke pass locally. Actual screenshots of clefs,
watermarks and first/second endings were reviewed and committed via capture.mjs.
Physical MIDI and native OS GPU/audio are not newly verified by this change.

[ADR-0033](0033-written-beam-groups.md) extends the basic rendering with authored
beam marks, conservative meter-based groups, shared chord stems and beams/hooks.
