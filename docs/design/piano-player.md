# Piano player: light, space and a clear strike line

- Status: Initial implementation, browser-tested
- Date: 2026-10-24
- Requirements: Owner's attached hand sketch and four-view description
- Architecture: [ADR-0003](../adr/0003-performance-model-and-piano-web-slice.md)

## Experience

The piano anchors the bottom of a quiet, dark stage. Colored notes arrive at a
thin strike line, illuminate their keys and create brief upward sparks. An
original two-hand study makes the first screen useful before importing a file.
Controls stay outside the moving field. No score/combo is invented in this slice;
those are separate practice/performance rules.

A single view selector offers:

1. **Falling blocks:** pitch lanes align with the physical keyboard; block length
   is key-down duration, including overlapping notes.
2. **Staff lanes:** the same timing with rotated bass and treble five-line staves.
   Line pitch positions follow piano geometry, so their spacing is intentionally
   nonuniform. Short ledger strokes extend these staves locally for outlying
   pitches. Sharp signs identify black keys. This is a learning projection, not
   conventional horizontal engraving.
3. **Falling notes:** the same staff/ledger lines remain visible while
   noteheads/stems/flags descend to the strike line and stay
   there until release. A small progress ring indicates remaining key-down time.
   Approximate symbol categories are not authoritative written rhythms.
4. **Sheet & playhead:** stationary bass/treble pitch notation, ledger lines and
   a moving vertical playhead. Eight-second pages change at the edge. Thin
   duration traces communicate performance spans. MIDI note spacing is temporal,
   and vertical guides are time divisions, not inferred barlines.

## Appearance and inputs

Aurora, Ember and Paper provide independent stage/background/key/note palettes.
Soft felt and Bright keys are synthesized sound profiles, not sampled physical
pianos. Sparkles can be disabled. These presentation settings do not mutate the
song. 49-, 61- and 88-key layouts share Rust geometry; narrow first loads use 49.
Keys outside the visible range can still sound; use 88 keys to see the full piano.

Desktop: A W S E D F T G Y H U J K play C4–C5. Space controls transport. Pointer
and multiple touches use independent source tokens; glissando prioritizes black
keys using the same geometry used for drawing. Form controls retain normal keys.
Hardware MIDI is optional and requires an explicit connection action.

## Important states

- Loading/build failure: visible error, playback stays disabled.
- Sound activation: user gesture resumes AudioContext; no autoplay on page load.
- Paused/free play: keys remain usable, song clock is stationary.
- Import error: retain the current song and allow another file immediately.
- Unsupported/denied MIDI: explain the capability; pointer/keyboard remain usable.
- Hidden tab/window blur: pause playback and release live/scheduled voices.
- Song end: stop voices, retain the last position, Play starts again at zero.

## Evidence and limitations

Real Chromium/WASM smoke covers a produced audio signal, keyboard and black-key
pointer input, transport, all four renderers, themes, import failure/recovery and
a narrow viewport. Screenshots were inspected for desktop and narrow layout;
clefs use original vector paths because musical glyphs were missing in the test
browser's default fonts. Browser screenshots are verification artifacts, not
imported design assets. Physical MIDI, mobile Safari and native playback have not
been tested. See [follow-up tasks](../TODO.md).

The supplied sketch informed the hierarchy; its photo is not bundled. The small
circle-and-tangent mark, clefs and note shapes are newly drawn SVG/Canvas geometry.
No external fonts, music recordings or SoundFont are bundled.

## In-engine surface update — 2026-10-24

Per the owner clarification, the page is one Bevy instance/camera/canvas, sized to
its viewport. All application menus, controls, settings and shortcut help render
with Bevy UI through wgpu. Native file/permission dialogs are capability boundaries.
The former HTML/Canvas UI is removed. See ADR-0005 and the README shortcut table.
