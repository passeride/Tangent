# ADR-0024: Persistent keyboard and piano action bindings

- Status: Accepted
- Date: 2026-10-26
- Related: ADR-0005, ADR-0009, ADR-0015, ADR-0020
- Supersedes: the fixed keyboard dispatch in ADR-0020 and the proposed two-extreme
  command overlay in the MIDI-input design brief.

## Context and alternatives

The owner requests one assignable piano control-key, usable held with another
note or as a prefix for the next note, and an action × keyboard/piano settings
matrix. Reimplementing each musical action in MIDI handlers would split timing,
scoring and persistence semantics. A Bevy-only input map would put the custom MIDI
grammar and persistent identities inside renderer types.

We examined upstream [Leafwing Input Manager](https://github.com/Leafwing-Studios/leafwing-input-manager)
and [Bevy Enhanced Input](https://github.com/simgine/bevy_enhanced_input), on
2026-10-26. Leafwing's published compatibility table tops out at Bevy 0.19 with
0.21; Enhanced Input 0.27 supports Bevy 0.20 and custom inputs. Enhanced Input is a
viable future adapter, but neither eliminates our need for a MIDI prefix parser,
editable persistence schema, capture/conflict UI and application context routing.
For this bounded keyboard/MIDI scope, use existing Bevy keyboard events and a
small independently testable input module. No new dependency/license is introduced.

## Decision and boundaries

`tangent-core::input_bindings` owns stable action IDs, default physical keyboard
chords, validated binding preferences, pitch labels and a device/channel-isolated
MIDI command state machine. It imports no Bevy/platform types. Presentation names
and defaults are catalogued with those stable IDs. Bevy KeyCode debug names are
translated to a deliberately bounded physical-code vocabulary at the adapter.
MIDI pitch uses C4 = 60 and also displays the numeric pitch to avoid octave ambiguity.

`scene::shortcuts` owns capture, the paginated matrix, Bevy input translation and
mapping stable commands to existing `controls::Action` handlers. UI buttons,
keyboard and piano commands share transport/loop/tempo actions. Actual command
eligibility follows the session and overlay context; typing in Library suppresses
shortcuts. MIDI input/output adapters continue to supply hardware events, not
application actions. No raw recording, input-device dropdown or automatic device
selection is added by this change.

Persist an additive `bindings` object in the existing version-1 preferences JSON,
in the existing SQLite settings row on web/native. It has its own version, enabled
flag, concrete control-note number and rows keyed by action ID. Missing legacy
objects use defaults. Duplicate/unknown actions, conflicting chords/notes, invalid
keys/pitches and a command equal to the control note are rejected. Explicit null
means unbound. Newly introduced missing action IDs are filled without overriding
nulls or stealing user chords. No relational schema migration is needed.

## Piano grammar

Shortcuts default Off; enabling reserves one concrete input pitch (initially A0).
Choose the lowest/highest *visible range* key explicitly, or learn any input note.
The choice is stored as a pitch and does not drift when Fit keys changes the view.
Pressing control is consumed, never scored, synthesized, echoed or fingerprinted.
While held, another note triggers its mapped command once per attack. Alternatively,
release control and press the next note within two seconds of the control attack.
That prefix is single-use, including an unmapped next note. Every consumed note-off
is also consumed; duplicate held note-ons never retrigger the command.

The modifier and command must come from the same adapter device/channel identity.
Within one polled batch, control attacks are routed before other notes so a
simultaneous pair works in either message order. An earlier ordinary note already
processed in a previous frame is not retroactively reclassified or delayed. Holding
control works without a timeout; the released prefix has a two-second timeout.
Reset transient state on pause/panic, focus/device loss and configuration changes.
After those context changes press control again. Outgoing song audio cannot command
itself. A small armed label reminds the player that the next note is a command.

No full-screen command mode or pause merely from touching control. Actions retain
their ordinary semantics: tempo/seek can run during playback; Play explicitly
starts/stops; opening settings pauses. Train/FreeForm do not accept song seeks or
tempo changes. Piano live notes are direct input; their matrix cells say Direct
input rather than offering modifier-triggered synthesis. PC live-note keys are
rebindable and released even when modifiers/context change or a tap is contained
within one rendered frame.

## Capture and recovery

Settings → Accessibility → Keybindings displays action rows and two columns.
Click a keyboard cell and press a chord, or click a piano cell and play one note
(without the modifier). Learn control captures one note. Captured MIDI attacks
and their releases never reach audio/scoring. Conflicts keep the old binding and
show a message; they do not silently steal or swap keys. Esc cancels capture,
Delete clears a selected binding. Control itself cannot be deleted: disable piano
shortcuts instead. Reset restores keyboard defaults and switches piano shortcuts Off.

Navigation actions are also rows, including Tab/Shift+Tab, Enter/NumpadEnter and
Back. Escape always remains a recovery alias, cannot be assigned to other actions
and cancels capture before normal routing. Library text-edit navigation remains
text-editor behavior rather than a musical shortcut. Physical-key labels are
layout-independent, with Ctrl/Command treated as one shortcut modifier.

## Validation and consequences

Pure tests cover default validation, migration without chord stealing, conflicts,
control collision, unknown IDs, held/prefix/unmapped commands, timeout, reset,
release consumption and device isolation. Preferences tests cover round trip and
legacy absence. A real Bevy browser test covers capture, rejection, PC remap, MIDI
commands with reversed simultaneous message order, ordinary notes, compact bounds
and reload persistence. Native and WASM compilation validate shared integration.
Physical P-45 feedback/latency remains a manual device check.

Tradeoffs: the reserved control pitch cannot be played musically while enabled;
the UI makes the Off switch explicit. The prefix is timed and may need later user
study/configuration. Bindings do not auto-repeat, preventing accidental repeated
transport toggles; repeated tempo/seek presses are deliberate. OS/browser-reserved
chords may not reach Bevy. A broader gamepad/axis scope should revisit Enhanced
Input at the adapter, while retaining stable persisted commands and the MIDI core.

Validation record: 102 workspace tests; native/WASM warnings-denied Clippy;
full browser regression plus final focused binding checks; packaged /Tangent/
site and native debug executable/import self-test pass. The event-ordering
regression preserves note-offs between rapidly repeated modifier gestures, and
Escape handling preserves a following mouse click in the same render frame.
