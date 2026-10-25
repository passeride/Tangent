# ADR-0016: Persistent MIDI output selection and safe restoration

- Status: Accepted
- Date: 2026-10-25
- Delivery: Implemented on UI v2 branch; CI and physical desktop/MIDI validation tracked separately
- Related: [ADR-0009](0009-local-sqlite-storage.md), [ADR-0012](0012-native-desktop-runtime.md), [ADR-0015](0015-midi-output-and-reading-practice.md), [UI v2 brief](../design/tangent-ui-v2.md)
- Supersedes: ADR-0015's *session-only output selection* and automatic reset assumption. No-echo and transport-safety rules remain unchanged.

## Problem

The original MIDI output selector cycled through ports, forgot the choice after restart, and native scan IDs contained a transient epoch/index. Persisting such IDs could route playback to a different piano after a device reorder. Web MIDI also cannot reliably ask for permission without a user interaction.

## Decision

- Present a real Bevy dropdown listing **Computer audio** and the currently enumerated MIDI outputs; selection is explicit, not a cycling button. The dropdown is paged when more than five outputs are available. No HTML controls or changes to the musical domain layer.
- Extend the JSON preference payload with the optional `midi_output_name` field, stored in the existing SQLite settings row. A missing field is interpreted as Computer audio; schema version and existing preference version remain compatible. Selecting Computer audio saves `null`.
- Save the human-readable port name, **not** a native OS index, scan epoch or Web MIDI transient handle. On startup, match that name to the newly enumerated output ports. Automatically select it only when **exactly one** current output matches. Zero matches keeps the preference but routes to Computer audio and invites a rescan. Multiple equal names are ambiguous: do not select one implicitly; require explicit user selection.
- Native launches enumerate outputs on their I/O worker as before and restore once preferences and a port list are available. Native MIDI Rescan re-enumerates and retries the preference. Browser launches automatically reconnect only if the browser's MIDI permission was already granted. Otherwise `Rescan / connect MIDI` is an explicit user gesture and triggers reconnection afterwards.
- Device selection, rescan, and drop-outs continue to silence physical voices and pause transport. A disconnected/failed device does **not** overwrite the saved preference; users can reconnect on rescan or explicitly switch to Computer audio. No physical keyboard echo is added.
- Device names may legitimately collide or change across driver updates. This deliberately conservative behavior is better than silently sending performance MIDI to the wrong port; the user can reselect after a rename.

## Validation plan

- Pure Rust preference tests cover old JSON payloads, round-trip, and invalid names.
- Bevy/WASM fake-MIDI browser integration should cover opening the dropdown, selecting an output, SQLite persistence across reload, granted-permission restoration, explicitly saving Computer audio, and retaining preference after disconnect.
- Native checks still need a real MIDI keyboard/port and restart/rescan tests. CI cannot prove actual external MIDI sound or stable OS device discovery.

## Alternatives rejected

- **Persist native port index/scan ID:** unsafe if enumeration order changes.
- **Reconnect by substring matching:** can select a different device unexpectedly.
- **Automatically prompt for Web MIDI permission at every startup:** browsers may block prompts and repeated permission requests are intrusive.
- **Persist a selected port only if successfully opened:** discards the user's intended destination when a piano is temporarily unplugged.
