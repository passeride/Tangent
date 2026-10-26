# ADR-0025: Personal themes and demonstrated hand assignments

- Status: Accepted; initial implementation
- Date: 2026-10-26
- Related: ADR-0005, ADR-0009, ADR-0021, ADR-0024

## Decision

Add a pure player personalization model and a separate Bevy editor/controller.
The model has no Bevy/platform dependencies; adapters handle only file bytes and
existing transactional settings persistence. JSON is the portable theme format:
versioned, bounded to 64 KiB, named RGBA byte-array tokens. Struct deserialization
rejects duplicate/unknown/missing fields and out-of-range channels. There is no
CSS interpreter, YAML dependency, script evaluation or path evaluation.

All existing Theme colours and four semantic hand colours are editable. Settings
→ Visuals opens the theme editor, with per-token/per-channel changes, fine/coarse
increments, undo, reset, import preview, explicit Apply and Cancel. Safe preset
chrome keeps editing controls readable even if the draft has invisible text.
Contrast diagnostics and hand swatches accompany the live palette preview.
Presets and the retained custom document have an independent selection flag.
Web exports a download; native writes a timestamped JSON file beside the SQLite
file and reports its path. Native import uses the platform picker and size checks.

Persist an additive defaulted personalization object in the existing version-1
preferences JSON, already saved transactionally in SQLite on both platforms.
A separate relational hand-profile migration was considered but would duplicate
the existing acknowledged settings write path for this bounded initial feature.
No schema bump is necessary. Profiles are bounded to 128 sources and 250,000
entries each, keyed by exact source-byte BLAKE3, with mapping version and revision.
Immutable accepted profiles use Arc so per-frame preference snapshots do not
copy their note maps; serde's rc feature adds no new package. Drafts are separate
and are copied once when editing. Larger libraries should move profiles behind
a dedicated storage query interface rather than raise these bounds indefinitely.

## Demonstration contract

Hands opens the current exact source's draft. Teach Left/Right closes a scoring
attempt, starts at loop A (or song start), and records positive input matches
without generating a scored attempt. Outgoing playback and MIDI command notes
cannot teach. Stop/review pauses and opens the draft, which supports per-note
Left/Right/Either/Unassigned corrections. Save explicitly applies it; Cancel
retains accepted data. Storage acknowledgement is shown separately; failures are
not described as saved. Reapplying forces a write retry.

The first implementation deliberately uses conservative alignment: exact pitch
and a unique non-percussion onset within ±200 source milliseconds. Search uses
sorted onset bounds, not a full-song traversal per attack. Ambiguous repeated
pitches and layered unisons abstain. Omissions never assign the opposite hand;
contradictions increment unresolved and preserve the existing assignment until
manual review. This is not the later rubato/sequence alignment proposal. Slowing
the transport and using short loops helps demonstrations remain within tolerance.
Pause, focus loss, transport edits and loop wrap stop teaching. No raw performance
recording is persisted.

Mapping version 1 identifies deterministic imported performance-note IDs under an
exact source hash. It is scoped to the existing import mapping; an importer change
that changes those IDs must bump/migrate this version. Repeated visits may be
annotated separately in falling views; written notation currently follows its
existing WrittenNote.performance_note reference. An authored all-visits editing
model and improved repeat links remain separate importer work. No fingerprint
match silently transfers annotations to another arrangement.

Hand colours affect falling notes, reading views and expected key edges; paired
edge bands represent simultaneous different hand colours on the same key. Played
solid fill retains its existing input semantics. Theme changes and hand annotations
do not change the target denominator or scoring. One-hand filtering, automatic
separation, robust sequence alignment and proof of physical hand use are not
introduced by this change.

## Library double-click

The text/background area of a library row opens through the same action as Open
when the same source is clicked twice within 400 ms and 8 logical pixels. Button
hits take precedence, so Edit/Remove do not accidentally open a piece. Refresh,
query changes and other actions clear pending clicks; matching uses source IDs,
not only row indexes. Keyboard Open and touch buttons remain available.

## Validation

Model tests cover strict theme parsing, source-key validation, profile roundtrip,
unique/ambiguous/missing demonstration matches and legacy preference defaults.
Real-app acceptance covers demonstration/no score, review/cancel/save/reload,
all theme tokens, JSON export, malformed import preservation, compact editor
bounds and library-row double-click. Existing regressions cover audio/MIDI,
practice, persistence and passage behavior. Physical MIDI timing remains a device
check; this is intended-hand annotation, not observed physical hand recognition.

Validation record: 105 workspace Rust tests and native warnings-denied Clippy
pass. Release WASM, the existing browser regressions, the new personalization
acceptance, packaged /Tangent/ smoke and native executable/import self-test pass.
The last binding/personalization tests were rerun after build-cache exhaustion;
the library test explicitly waits for asynchronous source loading after Open.
The theme editor screenshot was inspected and compact control bounds checked at
640×360 and 390×844. Hardware timing and native file-picker usability remain
manual checks. Later automatic hand inference is not represented as shipped.
