# ADR-0036: Rhythmic reading and music-font notes

- Status: Accepted
- Decision date: 2026-10-27
- Delivery: Implemented; validation recorded in the pull request
- Extends: [ADR-0007](0007-authored-measures-repeats-and-music-glyphs.md)

## Context and options

Fixed 800/300 ms thresholds confused note values with playback duration. The
same quarter note changes its duration when BPM changes. Hand-built rounded
rectangles also made all short notes look like the same single flag. Existing
Bravura contains complete note glyphs; adding another rendering library or
external font is unnecessary for the current independent-note reading views.

## Decision

The pure core owns optional NoteRhythm (value and augmentation dots) and a
conservative quarter-duration classifier. The MIDI adapter derives quarters
from source tick span divided by PPQ, which remains exact across tempo changes
and independent of meter denominator and playback-speed preferences. Match
whole through 64th values with zero to three dots within 8% relative error.
Zero, irregular, excessively long and unrecognised tuplet spans stay unknown.
Do not fabricate MusicXML notation or measures from this estimate.

MusicXML explicit type/dots take precedence over duration inference, preserving
written tuplet values. Each performance attack retains the first written
segment's rhythm, including repeated attacks. A tied performance note keeps
that first segment; the sheet continues to display all written tied segments.

Both sheet and falling notes use the existing embedded Bravura font's SMuFL
precomposed whole/half/quarter/8th/16th/32nd/64th notes. The sheet selects stem
orientation from the middle staff line, with dots moved off staff lines.
Rests use matching glyphs through 64ths and augmentation dots. Unknown MIDI
rhythm uses a bare head, retaining its duration guide. Live and exercise pitch
markers reuse font notes without interpreting the player's held duration.

## Consequences and validation

No new dependency, asset or audio/scoring/timing behavior. MIDI remains an
approximate time-spaced reading aid: articulation, beaming, voice separation,
rests, ties, bar splitting and tuplets are not reconstructed. Authored beaming and shared chord stems remain implemented by
[ADR-0033](0033-written-beam-groups.md); beamed heads reuse the music font while
keeping the established beam geometry. Authored stem directions take precedence
over staff-based defaults. Collision avoidance, tuplet brackets and curved ties
remain engraving follow-up.

Regression tests cover rhythmic classes, dots, rejected spans, tempo changes
inside MIDI notes, compound meter and MusicXML written values. Native/WASM
compilation and visual browser checks are separate validation steps; report
actual results and environment limitations in the pull request.

## References and provenance

Existing Bravura/SIL OFL asset and notices retained. Original implementation;
no excluded fork source or external assets consulted.

- https://smufl.formats.music/latest/tables/individual-notes.html
- https://smufl.formats.music/latest/tables/rests.html

## Integration with main

Resolved against the existing beam-group renderer rather than replacing it.
Standalone notes use complete Bravura glyphs; grouped notes use Bravura heads
with the established shared stems, slope and numbered beams. Explicit authored
stem direction remains authoritative. ADR-0033 retains ownership of grouping.

The integrated tree passes all 145 workspace Rust tests, native/workspace and
WASM Clippy with warnings denied, formatting and a release WASM build. The
actual Bevy browser beam scenario passes chord stems, up/down directions, mixed
values/hooks, 6/8, scaled/narrow systems and view switching. Updated desktop
and mobile captures were visually inspected; reproduce with
`TANGENT_CAPTURE_BEAMS=1 node web/piano/capture.mjs`.
