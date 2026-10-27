# ADR-0031: Independent notation layers and played-pitch feedback

- Status: Accepted; split-screen layout superseded by [ADR-0032](0032-scalable-sheet-overlay.md)
- Date: 2026-10-27
- Delivery: independent horizontal score and vertical view, block labels and live pitch markers
- Related: [ADR-0007](0007-authored-measures-repeats-and-music-glyphs.md), [ADR-0022](0022-readable-hud-and-keyboard-fit.md), [piano design](../design/piano-player.md)

## Context and decision

Players need to see their actual pitch on notation, and to combine the horizontal
sheet and falling view while learning. The prior four-way View selection makes
those presentations mutually exclusive. Keep the existing V cycle and numeric
view setting for compatibility, and introduce independent saved horizontal and
vertical switches plus an independent note-name switch. Settings → Visuals offers
these controls and a separate vertical style selector. The existing V shortcut
continues choosing a single legacy view. Both switches can be off for piano only.

The same Bevy surface owns all presentation. If both layers are on, divide the
available space vertically: score above, falling view aligned to the piano below.
Horizontal staff notes for physically held MIDI/keyboard/pointer pitches appear
in the played-input colour at the current playhead, with accidentals and ledger
lines. Pitches outside available score height retain their explicit Played label,
rather than being silently clamped to the wrong pitch. MIDI provides pitch only;
sharps are approximate and do not claim the original written spelling.
Vertical staff modes also show held pitch markers at the strike line. Blocks can
show MIDI note names including octave, without changing input or scoring rules.

Preference JSON retains version 1, with optional boolean `show_score` and
`show_vertical`; absent values derive from the previous `view`, preserving the
previous horizontal-only or vertical-only selection. `note_names` defaults off.
No SQLite migration or new dependency is required.

## Tradeoffs

A fixed split keeps the projected falling lanes aligned with piano geometry, but
reduces sheet and block space on small windows. Physical finger/hand detection,
engraving enharmonic spelling for live input, and tempo/rhythm assessment are
outside this change. Tests exercise legacy preferences, toggles, persistence,
actual played MIDI, responsive layouts and the four original view shortcuts.
