# ADR-0006: MusicXML adapter and authored notation

- Status: Accepted
- Decision date: 2026-10-25
- Delivery: Initial uncompressed partwise subset implemented
- Related: ADR-0003 and ADR-0005

## Context

Piano scores need written spelling, rests, staff and voice identity that MIDI
cannot reliably supply. The owner requests continued MusicXML implementation.
The new repository previously had only a MIDI adapter and optional notation models.

## Decision

Add `tangent-musicxml`, depending on tangent-core and roxmltree 0.20.0, without
Bevy, browser, MIDI or filesystem dependencies. Import bytes into the same Song,
with optional NotatedScore plus timed performance notes/events. The player detects
MIDI magic bytes; other inputs are validated as MusicXML. Failed imports leave the
loaded song unchanged. No XML-to-MIDI conversion or parallel playback engine.

Start with UTF-8 score-partwise .musicxml/.xml. Resolve divisions onto a bounded
common integer quarter grid, preserving rational notated durations/onsets. Share
measure offsets across parts; implicit measures use actual length, others at least
the current meter length. Integrate global quarter-note BPM sound tempo changes
into microseconds, rounding only at the absolute-time boundary. Default 120 BPM.
Matching adjacent sound ties share a performance note while retaining individual
written notes, including rests, voice, staff, spelling and tie flags.
Allocate separate audio port/channel routes per part/staff/voice so overlapping
unisons in separate voices release independently; skip the percussion channel.

Use roxmltree's node limit and disable DTD entity parsing. External DOCTYPE headers
are accepted without fetching resources; internal subsets/entities are rejected.
No network requests, filesystem extraction or external resource resolution.
Limits: 16 MiB, 1 million XML nodes, 128 parts, 16 staves/part, 20,000 measures/part,
250,000 note/rest elements, 24h. Rhythmic resolution is bounded and overflow checked.

The stationary reading view uses authored pitch spelling and initial clef/staff
where supplied. It remains a performance-time reading aid, not full engraving.
The rotated falling views remain aligned to physical piano keys.

## Alternatives and consequences

A schema-generated full MusicXML model would retain much more notation but adds
considerable scope before the playback boundary is exercised. A handwritten XML
parser would duplicate complex XML validation and entity handling. roxmltree is
an independent MIT/Apache-2.0 package; record its source/license in inventories.

Compressed MXL/ZIP, timewise/opus, repeat navigation, transposition, full engraving,
key/time signature graphics, written glyph durations/dots/tuplets, pedals, dynamics
and articulation playback remain follow-up work. Reject MXL/timewise/transposition,
unsupported clefs and microtones explicitly. Warn for skipped grace notes, repeats,
clef changes and unsupported visual-only tempo. Do not imply complete MusicXML
conformance or a full score renderer. Import written order once.

Reference: W3C MusicXML 4.0 reference and MIDI-compatible tutorial:
https://www.w3.org/2021/06/musicxml40/
https://www.w3.org/2021/06/musicxml40/tutorial/midi-compatible-part/
https://github.com/RazrFalcon/roxmltree/tree/v0.20.0
