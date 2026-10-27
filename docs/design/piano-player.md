# Piano player: light, space and a clear strike line

- Status: Initial implementation, browser-tested
- Date: 2026-10-24
- Requirements: Owner's attached hand sketch and four-view description
- Architecture: [ADR-0003](../adr/0003-performance-model-and-piano-web-slice.md)

## Experience

The piano anchors the bottom of a quiet, dark stage. Colored notes arrive at a
thin strike line, illuminate their keys and create brief upward sparks. The app opens at a centered logo-first main menu. Play opens the original
two-hand study; Train offers note/chord exercises, FreeForm opens live piano,
and Settings returns to the menu when closed. See ADR-0008.
Only a small gear in the top-right corner remains during play. It opens a pause
screen containing every setting and transport/file/fullscreen control. No score/combo is invented in this slice;
those are separate practice/performance rules.

The pause screen view selector offers:

1. **Falling blocks:** pitch lanes align with the physical keyboard; block length
   is key-down duration, including overlapping notes.
2. **Staff lanes:** the same timing with rotated bass and treble five-line staves.
   Line pitch positions follow piano geometry, so their spacing is intentionally
   nonuniform. Short ledger strokes extend these staves locally for outlying
   pitches. Sharp signs identify black keys. This is a learning projection, not
   conventional horizontal engraving.
3. **Falling notes:** the same staff/ledger lines remain visible while
   noteheads/stems/flags descend to the strike line and stay
   there until release. A small duration trace indicates remaining key-down time.
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
keys using the same geometry used for drawing. All application controls are Bevy UI.
Hardware MIDI is optional and requires an explicit connection action.

## Important states

- Loading/build failure: visible error, playback stays disabled.
- Sound activation: user gesture resumes AudioContext; no autoplay on page load.
- Paused/free play: keys remain usable, song clock is stationary.
- Pause menu: song and live voices stop; piano input is disabled until leaving.
- Import error: retain the current song and allow another file immediately.
- Unsupported/denied MIDI: explain the capability; pointer/keyboard remain usable.
- Hidden tab/window blur: pause playback and release live/scheduled voices.
- Song end: stop voices, retain the last position, Play starts again at zero.

## Evidence and limitations

Real Chromium/WASM smoke covers a produced audio signal, keyboard and black-key
pointer input, transport, all four renderers, themes, import failure/recovery and
a narrow viewport. Screenshots were inspected for desktop and narrow layout;
the GPU renderer uses geometric noteheads and text clef labels. Browser screenshots are verification artifacts, not
imported design assets. Physical MIDI, mobile Safari and native playback have not
been tested. See [follow-up tasks](../TODO.md).

The supplied sketch informed the hierarchy; its photo is not bundled. The small
gear and note shapes are original Bevy geometry. Bevy's Fira Mono font and the
original CC0 MusicXML study have separate license records; no recording or SoundFont is bundled.

## In-engine surface update — 2026-10-24

Per the owner clarification, the page is one Bevy instance/camera/canvas, sized to
its viewport. All application menus, controls, settings and shortcut help render
with Bevy UI through wgpu. Native file/permission dialogs are capability boundaries.
The former HTML/Canvas UI is removed. See ADR-0005 and the README shortcut table.


ADR-0007 adds measure-based MusicXML layout with real Bravura clefs, meter/key,
tempo, repeats and first/second-ending brackets. The MIDI eight-second reading aid
remains a separate fallback. Rotated staff views show faint upright G/F watermarks
and moving measure/beat/ending guides. The clefs are licensed font outlines, not
newly hand-drawn approximations.

## Independent notation layers

Settings → Visuals offers Horizontal sheet and Vertical view on/off independently,
vertical style (blocks, staff blocks or staff notes) and Names on blocks. When both views are enabled, the score overlays the full-height falling view; both off leaves the keyboard. Sheet size is adjustable from 50–200%. In sheet-only mode, as many systems as fit are shown.
Held notes are drawn in the played-input colour on the horizontal score at the
playhead and on the rotated staff at the strike line, with sharp/ledger guidance.
The score also labels held pitches. Live MIDI uses approximate sharp spelling;
MusicXML's authored notation retains its written spelling. Out-of-height notes
remain named rather than appearing at a false staff position. These settings are
saved; older view preferences migrate without changing the previous display.
The V shortcut continues cycling the four legacy standalone views.
