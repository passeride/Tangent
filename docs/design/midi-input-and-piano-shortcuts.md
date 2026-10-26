# MIDI input selection and piano shortcuts

- Date: 2026-10-26
- Status: Design proposal; dropdown and piano commands are not implemented.
- Scope: web and native, one Bevy canvas, optional control without leaving the piano.
- Related: [UI v2](tangent-ui-v2.md), [development plan](product-development-plan.md),
  [MIDI output](../adr/0015-midi-output-and-reading-practice.md).

## Current implementation and feasibility

Both platforms currently accept all MIDI inputs. Web `platform.js` attaches a
handler to each Web MIDI input; native `platform/native.rs::connect_midi` opens
each midir input. Events already carry device/channel/key identity. Output has
an independent dropdown and saved preferred device name. Input selection is
feasible with the same Bevy dropdown pattern, but needs its own state, messages
and persistence; changing output must never change input.

| Audio & MIDI control | Choices | Feedback |
| --- | --- | --- |
| MIDI input | All connected inputs (legacy default), one device, Off | Preferred device and actually active input, separately |
| MIDI output | Computer audio or an output device | Existing independent selection |
| Piano shortcuts | Off (default), On | Device, calibrated range, entry gesture; Configure button |

List only connected inputs with names and a disambiguating session label where
names repeat. Preserve the preferred name across reconnects, but restore it only
when exactly one device matches. Never persist a transient port index as identity.
If the selected input disappears, pause, release its held keys and display
“Input unavailable: P-45 — Reconnect / Choose input”. Do not silently accept all
other devices. All-input mode may accept newly connected devices explicitly.
Off disables MIDI notes/commands while computer and pointer input still work.
Selecting an input cancels held input and pending command gestures without
starting playback. Changing devices during a run segments the attempt.

Web implementation should enumerate/report inputs after permission and state
changes, detach obsolete handlers, and filter before queueing note/control events.
Native should enumerate ports and own selected connections in the platform worker;
re-enumerate at explicit Rescan and expose connection failures. Device names may
be absent or duplicated; do not imply unique identity. Neither MIDI 1.0 nor these
adapters reliably reports physical key count: calibration must be user-driven.
No new dependency is needed. Hardware hotplug behavior remains a device test.

## First-time setup

1. Choose the input device in Settings → Audio & MIDI.
2. Enable Piano shortcuts and choose Configure. Explain that commands pause music.
3. Press the lowest playable key, then the highest; show exact MIDI numbers and
   note names. Offer manual correction and 49/61/73/76/88-key presets.
4. Rehearse entry and one harmless command, then confirm. Save device/range/map
   locally only after this explicit confirmation; cancellation keeps prior setup.

An 88-key P-45 normally maps A0 (21) through C8 (108); show MIDI numbers because
manufacturers disagree on octave naming. Do not hard-code these extremes for
other keyboards. With All inputs selected, nominate one command device; gestures
must originate from that same device. Duplicate device names require re-selection.

## Entering and leaving command mode

Chosen initial proposal: hold both calibrated outermost keys together for 800 ms,
with no other keys held and pedal released. Both note-ons must occur within
150 ms. A visible hold indicator appears as the gesture forms. Successful entry
pauses transport, checkpoints the attempt, cancels scheduled audio/MIDI and
releases held notes. Display “Piano controls — release both keys”. No command is
accepted until every key and pedal is released. This is a latched overlay, so
players do not need to span the piano while playing a third key.

The shortcut detector runs before scoring/output. Only when explicitly enabled,
buffer candidate extreme-key attacks for up to 150 ms; if entry fails, deliver
them in order as ordinary input. If the pair forms, reserve it through hold/release.
Document this small edge-key delay and offer button-only entry for repertoire
that uses those notes. Consumed command notes never score, synthesize, echo to
MIDI output, advance reference exercises or feed fingerprint recognition.
Sustain must track incoming CC64 for this gate even though pedal-aware scoring
is currently unsupported. Outgoing playback cannot enter command mode.

Exit on the mapped Cancel key, Esc, overlay Close, 10 seconds idle, focus loss or
device loss. Closing leaves playback paused; Play/resume is explicit. Mode exit
requires release before ordinary notes can re-enter scoring. An on-screen button
provides equivalent entry, including for players unable to reach both extremes.
Never use the gesture as a global emergency-stop replacement.

## Initial command map

Use the first eight white keys strictly inside the calibrated low extreme.
Call them W1–W8; the overlay and settings display their resolved pitches/numbers.
Do not assume C-major key positions on a shortened/custom range. At least eight
interior white keys are required. Commands trigger once per new press and require
release; a held key never auto-repeats or repeatedly toggles playback.

| Piano key | Command | Feedback / boundary |
| --- | --- | --- |
| W1 | Slower | −5 percentage points, minimum 25%; show new percentage |
| W2 | Faster | +5 percentage points, maximum 200%; show new percentage |
| W3 | Back 5 seconds | Source-song time, clamped to song/active loop |
| W4 | Forward 5 seconds | Same source-time/loop contract |
| W5 | Set A | Current position; explicitly clears old B as existing control does |
| W6 | Set B | Validate B > A; show error and retain previous valid selection |
| W7 | Resume and close | Wait for full release, then explicitly resume |
| W8 | Cancel and close | Remain paused; retain changes already applied |

Only offer tempo/seek/loop commands for Play. Train and FreeForm show unavailable
commands with an explanation. First version excludes import, deletion, database
export, device changes and automatic output switching from piano commands.
Changing tempo/range uses existing domain actions and their attempt segmentation.
Volume, view, part and effects can be later configurable mappings; do not crowd
this initial eight-key map or silently repurpose commands by screen.

## How settings advertise their piano shortcut

Each supported setting/action gets a quiet secondary label next to its normal
computer-key shortcut. Example after calibration:

| Setting / action | Computer shortcut | Piano shortcut label |
| --- | --- | --- |
| Speed − | ↓ | Piano controls → W1 · B0 · MIDI 23 (example) |
| Speed + | ↑ | Piano controls → W2 · C1 · MIDI 24 (example) |
| Back / forward | ← / → · 5s; Shift · 15s | Piano controls → W3 / W4 · resolved pitch |
| Set A / Set B | Focusable buttons | Piano controls → W5 / W6 · resolved pitch |
| MIDI input | Focusable dropdown | No piano shortcut |

W1/B0 is an example for A0 as low extreme, not a universal mapping. Render the
actual calibrated pitch. Clicking a label opens Configure and highlights that key
on a miniature keyboard. With shortcuts Off, display “Piano shortcuts: Off — Set
up” once per settings group. With a missing device, retain the label but mark
“Unavailable: device disconnected”. Unmapped settings say “Not assigned” only in
Configure, avoiding noise throughout Settings. Never suggest an ordinary note is
always a global shortcut: every label starts with “Piano controls →”.

The overlay shows entry gesture, device, eight key tiles, action names, current
speed/position/loop and a short last-action confirmation. Use text and shapes as
well as colour, respect reduced motion, and retain keyboard/pointer navigation.
A low-height layout pages the key legend; all mapped actions remain discoverable.
No extra HTML controls: presentation stays inside Bevy.

## Implementation sequence and acceptance

1. Add independent input enumeration/selection and validated additive preferences;
   preserve legacy All inputs. Test duplicate names, denied permission, Off,
   selected-device disconnect and release of held keys on both adapters.
2. Add a platform-independent command state machine using device/key/CC events
   and monotonic time; unit-test hold thresholds, release gate, failed gestures,
   simultaneous multi-device input, zero-velocity note-off, sustain and timeout.
3. Integrate state machine ahead of scoring/audio with Bevy overlay and shared
   actions. Add actual-app tests that commands produce no target hits or outgoing
   note attacks, cannot self-trigger from playback, and never resume on cancel.
4. Test with the P-45, a short keyboard and two devices, then evaluate the entry
   gesture with players. Confirm 4K and short-window readability and manual
   alternatives. Record measured gesture usability before calling it the default.

A follow-up implementation ADR must define event ordering, cancellation and
preference validation before these interfaces ship. This document is a concrete
proposal for review, not a delivered capability or hardware validation claim.
