# MIDI input selection and piano shortcuts

- Date: 2026-10-26
- Status: Piano bindings implemented under ADR-0024; MIDI input dropdown proposed.
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

## Revised piano control design — 2026-10-26

The owner replaced the proposed two-extreme gesture with one assignable control
note. The authoritative implemented contract is now
[ADR-0024](../adr/0024-configurable-keyboard-and-piano-bindings.md).
The input dropdown section above remains a proposal; it is independent of bindings.

Open Settings → Accessibility → Keybindings. Each row names an action; the columns
are Keyboard and Piano. Click a cell to record a new keyboard chord or input note.
For example, Piano C4 (60) on Faster means hold the chosen control-key and press C4,
or tap/release control then press C4 within two seconds. Capture the C4 binding by
playing C4 alone in Settings. Conflicts are rejected with the existing assignment
retained. Esc cancels capture, Delete clears the selected cell; Reset restores
defaults and turns piano commands Off. All changes use the saved SQLite preferences.

Control-key choices: explicitly use the lowest/highest currently visible piano
key, or Learn any custom input pitch. Show its resolved pitch and MIDI number.
Changing view range later does not change that stored control note. Input shortcuts
default Off because an enabled control note is reserved for commands and silent.
The next note is consumed as a command even when it has no assignment; a held
modifier may invoke multiple commands, one new attack per command. The modifier
and command must originate from the same input device/channel.

| Action | Keyboard default | Piano column example |
| --- | --- | --- |
| Play / pause | Space | C5 (72) |
| Faster / slower | ↑ / ↓ | C4 (60) / D4 (62) |
| Jump forward / back | → / ←, five source seconds | E4 (64) / F4 (65) |
| Longer jump | Shift+→ / Shift+←, fifteen seconds | User assignment |
| Set A / Set B / clear loop / snap | Unbound | User assignment |
| Volume, view, theme, sound, effects, instrument, range | Existing keys where present | User assignment |
| Import, sample, rescan MIDI, fullscreen | Existing Ctrl/F keys | User assignment |
| Hint, hear, skip, target part and Fit keys | Unbound | User assignment |
| Menu focus, activate and back | Tab / Shift+Tab / Enter / Esc | User assignment |
| PC live notes | A W S E D F T G Y H U J K | Direct piano input |

Piano examples are suggestions; no command notes are preassigned. Column heading
“Piano (Ctrl first)” and the control-note summary explain the meaning of C4.
An armed in-session label appears when control is waiting for a command. Ordinary
notes need no modifier and retain sound/scoring. All matrix editing stays in Bevy,
with paging on low windows. Reset and Escape remain recoverable paths if the user
unbinds their usual controls. Reduced motion remains in Accessibility beside the
Keybindings entry.

Future improvements: per-device piano mappings, user-controlled prefix timeout,
searchable/filterable action rows and broader shortcut badges beyond the existing settings/resume labels. These
are not claimed as delivered. Physical keyboard
range detection is still explicit/user-driven; MIDI does not reliably report it.
