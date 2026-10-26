# ADR-0020: Variable song playback speed

- Status: Accepted
- Date: 2026-10-26
- Delivery: Implemented; physical MIDI and device latency remain hardware checks
- Related: [ADR-0012](0012-native-desktop-runtime.md), [ADR-0015](0015-midi-output-and-reading-practice.md), [ADR-0018](0018-scored-sessions-and-adaptive-review.md)

## Decision

PlaybackRate in tangent-core validates a 25–200% multiplier. Transport advances
in source-song microseconds using scaled elapsed clock time and an integer
fractional remainder, avoiding accumulated rounding error across frames. Imported
notes, tempo maps, pitch and durations in the source model remain unchanged.
Seeking still moves five source-song seconds. Practice targets and live keyboard
voices do not change speed. A serde-defaulted speed_percent preference defaults
to 100, persists through the existing settings row, and survives source imports.

Pause exposes Slower / current percentage and reset / Faster in the existing
Bevy surface. Steps are 5%; the centre button resets to 100%. The song HUD shows
the active percentage. Music remains paused while changing pause controls.

Browser audio maps each source timestamp to its wall-clock deadline by dividing
by the multiplier. Its lookahead stays 100 wall-clock milliseconds. Physical MIDI
sends only due events; note-offs, sustain and controller events use the same
scaled timeline. Native scheduling shares PlaybackRate's conversion functions
for song position, lookahead, note duration and delay. A transport sync cancels
old song scheduling, anchors at the same musical position and restores sounding
notes/controllers at that position. Native oscillators and MIDI key numbers
retain their original pitch; global audio playback-rate/resampling is avoided.

## Consequences and validation

Changing tempo may reattack sustained notes when the output state is restored.
This deliberately follows the existing pause/seek safety contract. Scores remain
pitch coverage rather than calibrated timing or tempo-normalized competition;
no source composition metadata or score database migration is introduced.

Core tests cover 60/70/120%, bounds, pause/end behavior, saturation and tiny-frame
rounding. Browser clock tests check deadlines, lookahead and reanchoring. Real
Bevy browser tests measure source-time progression, sound, persisted speed,
reset, import retention, MIDI pitches/note-on/off spacing and mobile bounds.
Native/WASM compilation, Clippy and packaged-site checks verify integration.

## Keyboard controls — 2026-10-26

In the Play session, Up/Down change speed by five percentage points using the
same bounded, persisted actions as Pause. Left/Right seek five source seconds;
Shift+Left/Right seek fifteen. Paused transport remains paused. Settings/pause
overlays retain their existing focus navigation; tempo shortcuts do not change
Train/FreeForm. Volume remains available in Audio & MIDI settings.
