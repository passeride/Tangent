# ADR-0015: Explicit MIDI output and readable practice targets

- Status: Accepted
- Created / decision date: 2026-10-25
- Delivery: Implemented; physical output verification remains separate
- Related: [ADR-0005](0005-one-bevy-gpu-surface.md), [ADR-0012](0012-native-desktop-runtime.md)
- Supersedes: Exercise sequencing/presentation in [ADR-0008](0008-navigation-and-practice.md)

## Context and options

The owner verified the previous desktop/library build with a Yamaha P-45 and
requests MIDI output, better reading exercises and a distinction between played
and expected keys. The previous exercise projected every note from C4, exposed
the answer in its title and displayed the next target while the correct key was
still held. Merely recolouring those targets would preserve these problems.

Automatic routing/echo could double a piano's locally generated attacks. Replacing
Bevy with HTML/device-specific controls would violate the established UI boundary.

## Decision

The shared Bevy Settings screen cycles Computer audio and discovered MIDI outputs.
Rescan refreshes ports. Native ports use existing midir on the I/O worker; web uses
Web MIDI with explicit permission. Selection is session-only, never automatically
reconnected by a potentially stale port index. A native rescan returns to computer
audio; web retains an existing connected output. Selection pauses playback.

When selected, MIDI replaces internal synthesis. Song On/Off, controllers, bend
and program events retain channels and order, excluding percussion channel 10.
Logical MIDI ports converge on this single destination; multiport scores can
collide. Computer/pointer/touch notes use channel 16; physical MIDI input is shown
and evaluated but never echoed to the output. P-45 Local Control must stay On.
The piano controls its own speaker volume/timbre; app volume/piano presets control
the internal synthesizer. Scores using channel 16 can share that live channel.

Physical events are dispatched only when due (native worker ~4ms; web pull frame),
with no future timestamped attacks to leak after pause. Audio retains its existing
lookahead. Pause, seek, navigation, focus loss, output switch and graceful shutdown
clear notes/pedal/controllers. Seek restores channel state plus held/sustained
notes without replaying obsolete attacks. Native send errors and observed web
disconnection pause playback. Native hotplug enumeration still requires Rescan.
This is not calibrated low-latency scheduling or live pedal forwarding.

The renderer independently derives expected keys from song time and played keys
from held input. Expected keys have feathered inset edges in the pressed colour;
only physical/computer/pointer/touch input fills the body. Both can coexist. This
cue remains enabled when sparkles are disabled and uses the normal theme palette.
Practice shows expected keys only after Show hint, avoiding accidental answer cues.

The renderer-independent Practice model owns clef, level, targets and answer state.
Notes offer treble C4–C5, bass C3–C4, or alternating clefs; levels are stepwise,
mixed natural notes and chromatic sharp spelling. Reproducible permutations cover
each range. Chords retain diatonic C-major rules. Exact octave matching, feedback
and all-key release are explicit; completion advances on release, keeping the
answered note visible until then. No timed scores or progress persistence added.
Presentation uses the same E4/G2 bottom-line anchors and ledger rules as the score,
real Bravura G/F clefs and sharp glyphs. This does not implement full engraving,
beams, tuplet notation, new key signatures or adaptive curriculum.

## Validation and provenance

Pure practice and MIDI encoding/seek-state tests cover octaves, release gating,
bass/mixed/chromatic ranges, sustain and malformed data. Browser integration uses
the real Bevy WASM with a fake MIDI device to verify routing, no echo, pause,
disconnect, solid/glow independence and narrow controls; screenshots are generated
by `web/piano/practice-midi-smoke.mjs`. Physical piano output remains a manual check
in [test-p45](../test-p45.md). No new dependency, external music or legacy source
was introduced. Prior source exposure remains disclosed in PROVENANCE.md.
