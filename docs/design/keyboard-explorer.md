# Keyboard Explorer (N02)

Status: implemented initial octave-transfer variant, 2026-10-26.

## Learning goal and flow

Recognize repeating piano-key patterns and move the same note name to another
octave. Train → Keyboard Explorer presents a source name and a labelled REF pointer
on the 3D keyboard. Find the same note exactly one octave higher/lower using MIDI,
a computer-key binding, pointer or touch. Keep the destination hidden until an
explicit hint or a correct answer. Release every held key before advancing.

| Level | Source | Transfer |
| --- | --- | --- |
| C/D/E | C3, D3, E3 | Up one octave |
| White keys | C3–B3, seven white notes | Up one octave |
| All keys | C4–B4, twelve pitch classes | Alternating up/down one octave |

A short explanation states that an octave is 12 semitones. English/international
note names match the rest of the UI; no enharmonic naming quiz is claimed.
The REF marker is the task stimulus, not a pressed key or an expected destination.

## Feedback and persistence

Wrong source, wrong octave and extra held notes are rejected. Correct answers
remain visible until full release. Hint/Hear mark a trial assisted even if the
help is later hidden. Reference playback never counts as player input. Finish,
Skip, pause and existing 12-task results retain their established meanings.
Progress saves the mode, difficulty and exact source/destination evidence; retry
from history restores this exercise. Ordered levels are used here; the existing
adaptive note/chord preference is preserved but does not select Explorer tasks.

## Layout and verification

The source and unknown destination appear in a quiet card, with controls above
and feedback above the piano. Short windows use an inline card. All controls live
inside the Bevy surface and use the existing theme/reduced-motion behavior.
Capture actual UI via `TANGENT_CAPTURE_EXPLORER=1 node web/piano/capture.mjs` after
building WASM. Test mobile/short layouts, up/down directions, source versus
answer, assistance, release, persisted history/retry and old note/chord behavior.

Further N02 variants: note-name choice answers and theory-aware spelling. These
are not delivered by the octave-transfer implementation.
