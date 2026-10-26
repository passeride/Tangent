# Editable themes and portable theme files

Status: Initial implementation under [ADR-0025](../adr/0025-personal-themes-and-demonstrated-hands.md).

The initial editor lives in Settings → Visuals. Tokens are named RGBA byte arrays,
not hex strings. Apply/Cancel, fine/coarse channel edits, undo, per-token reset,
contrast diagnostics and bounded JSON import/export are implemented. Native
exports beside the database with a reported path. Named-theme collections and
a Save As picker remain future enhancements. The original proposal below records
the longer-term UX intent; ADR-0025 describes the exact delivered boundaries.

Decision: use UTF-8 JSON with a versioned schema and `.tangent-theme.json` extension.
The existing Rust/JS stack already supports JSON. CSS would introduce browser
presentation semantics into the native Bevy renderer; YAML adds a parser and
ambiguous scalar syntax without useful benefit for this bounded configuration.

Expose every field of the current Theme structure through typed controls: bg,
panel, ink, muted, staff, white, black and all four colour slots. Add semantic
hand-left, hand-right, hand-either and hand-unassigned colours. Preserve alpha
explicitly as RGBA hex; validate channel ranges. Do not expose arbitrary Rust
expressions, CSS, paths or executable values. Future added theme variables need
schema defaults/migration and a documented purpose, rather than silent omission.

Settings → Appearance offers named presets, a user copy, colour/alpha editors,
live preview, Reset field, Undo, Apply, Cancel, Import and Export. Preview includes
white/black keys, notation, expected glow, played fill, both hand colours and error
feedback. Contrast diagnostics identify affected text/surfaces; L/R markers and
non-colour error cues remain available. Editing a built-in creates a user theme.

A theme document contains schema_version, name and a tokens object. Export only
theme data, excluding music, scores, MIDI device names and private preferences.
Limit imports to 64 KiB UTF-8 JSON; reject unsupported versions, duplicate object
keys, malformed RGBA values, missing required tokens and unknown tokens with a
clear error. Never replace the current theme on parse failure. Import opens a
preview and requires Apply; filename extensions alone are not format validation.

Use a shared pure theme schema/validator and resolved semantic palette in core;
Bevy converts validated tokens to Color. Web/native file adapters only read/write
bytes through their existing picker/download/save boundaries. Theme selection,
custom theme documents and revision persist transactionally in SQLite. Apply
changes in-memory appearance, then save with explicit saving/error/retry state;
restart restores the last acknowledged revision. Colours never alter scoring,
hand assignment or playback. Hand profiles store semantics, never colour hexes.

Acceptance: every existing palette field is editable; import/export round-trips
all RGBA values; malformed/future/oversized files preserve the active theme;
Apply/Cancel and failed writes behave consistently on web/native; restart restores
custom colours; screenshots cover every view, reduced motion and all hand states.
