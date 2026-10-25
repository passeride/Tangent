# ADR-0008: Navigation and independent practice rules

- Status: Accepted
- Decision date: 2026-10-25
- Delivery: Implemented; verification recorded in the change report
- Related: [ADR-0005](0005-one-bevy-gpu-surface.md)

## Context and options

The owner requests a logo-first main menu with Play, Train, FreeForm, Settings,
and Exit where relevant, plus note/chord exercises. Extending the pause toggle
alone would couple navigation to transport. HTML menus would violate ADR-0005.

## Decision

Bevy scene state owns Main, Train and Session navigation, independently from the
pause/settings flag. Entering a mode or returning home releases voices and pauses
transport. Settings returns to the originating screen. Browser-backed launchers
use the same browser UI, so Exit is omitted; native window exit remains future work.
Play retains the imported song. FreeForm and practice do not advance or sound it.
The original piano-key wordmark uses in-engine geometry, with no new asset.

Exercise targets and answer evaluation live in tangent-core::practice, independent
of rendering and platform input. Notes require the exact octave; chords compare
complete pitch-class sets, accepting inversions and doublings. C-major diatonic
notes/triads cycle in a fixed sequence; all keys must be released between answers.
The scene presents the target on a staff and counts completions. These simple
exercises do not implement timed scoring, random curricula, adaptive difficulty,
persistence or a general chord-recognition engine.

## Consequences and validation

All input sources share the existing held-note boundary. Menus block note input.
Exercise behavior has native domain tests; browser smoke covers initial routing,
settings return, note/chord answers, FreeForm silence and existing Play behavior.
No dependencies or excluded legacy sources/assets are introduced. Existing
provenance disclosure remains unchanged.
