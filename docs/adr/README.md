# Architecture decision records

An ADR records **why** a consequential choice was made: the context, alternatives,
tradeoffs and evidence. It is not a task checklist or proof of implementation.

| ID | Decision | Status | Delivery |
| --- | --- | --- | --- |
| [0001](0001-bevy-and-web.md) | Use Bevy and validate web from the first application slice | Accepted | Initial Bevy core/browser piano |
| [0002](0002-byte-based-midi-import.md) | Byte-based MIDI import and explicit source timing | Accepted | Initial importer implemented |
| [0003](0003-performance-model-and-piano-web-slice.md) | Shared music model, Bevy core and browser piano/audio adapters | Accepted | Initial web piano implemented |
| [0004](0004-distribution-pages-and-desktop-launchers.md) | Tested web distribution, Pages and desktop launchers | Accepted | Packaging implemented; CI/deployment verified separately |
| [0005](0005-one-bevy-gpu-surface.md) | One Bevy/wgpu surface, all controls and notation in-engine | Accepted | Replaces the initial Canvas/HTML presentation |

| [0006](0006-musicxml-import-and-authored-notation.md) | MusicXML adapter and authored notation | Accepted | Initial uncompressed partwise subset |

| [0007](0007-authored-measures-repeats-and-music-glyphs.md) | Authored measures, repeat visits and music glyphs | Accepted | MusicXML demo, Bravura clefs and measure/volta display |

## Create an ADR

1. Copy [the template](template.md) to `NNNN-short-title.md`, using the next unused
   four-digit number.
2. Begin with `Proposed`. Describe the concrete problem and viable alternatives.
3. Link relevant design briefs, experiments and independent primary references.
4. Record an explicit decision and its date before marking it `Accepted`; a
   maintainer may make that decision within the authorized work.
5. Add the record to the index above. Track implementation separately.

Statuses: `Proposed`, `Accepted`, `Rejected`, `Superseded`. A new decision that
replaces an accepted one gets a new ADR and links in both directions. Preserve the
old rationale rather than making history look as if the original choice never
existed. Small local refactors do not require an ADR.

Remaining questions include GPU rendering, off-main-thread audio, authored-score
layout, native adapters and local storage. See [follow-up tasks](../TODO.md).

- [ADR-0008: Navigation and practice](0008-navigation-and-practice.md) — accepted; main menu, mode routing and independent exercise rules.

- [ADR-0009: Local SQLite persistence](0009-local-sqlite-storage.md) — accepted; settings, export and future score schema.

- [ADR-0010: Bounded MXL container import](0010-bounded-mxl-container-import.md) — accepted; compressed MusicXML ZIP reading and limits.
- [ADR-0011: Symbolic music fingerprinting](0011-symbolic-fingerprinting.md) — accepted; Rust matching, SQLite inverted index and FreeForm lookup MVP.
- [ADR-0012: Native desktop runtime](0012-native-desktop-runtime.md) — accepted; shared Bevy/wgpu scene, MIDI, audio worker and SQLite on desktop.

- [ADR-0013: Local music library](0013-local-music-library.md) — accepted; source-byte persistence, metadata paging and reopen on web/native.

- [ADR-0014: Library search and metadata](0014-library-search-and-metadata.md) — accepted; Unicode search, editing, sort/filter, batches and fingerprint ownership.

- [ADR-0015: MIDI output and reading practice](0015-midi-output-and-reading-practice.md) — accepted; no echo, played/glow cues and clef/level exercises (output persistence superseded by ADR-0016).
- [ADR-0016: Persistent MIDI output selection](0016-persistent-midi-output-selection.md) — accepted; Bevy dropdown, SQLite preference and safe name-based reconnection.
- [ADR-0017: Practice reference and motion](0017-practice-reference-and-motion.md) — accepted; exact live pitch pointers, optional reference audio and persistent effect/motion choices.
- [ADR-0018: Scores, history and adaptive review](0018-scored-sessions-and-adaptive-review.md) — scored training/song sessions, SQLite checkpoints and recent-history review.
- [ADR-0019: Automatic practice reference](0019-automatic-practice-reference.md) — persistent Hear toggle and one audition per new task.
- [ADR-0020: Variable playback speed](0020-variable-playback-speed.md) — saved tempo multiplier with synchronized notation, audio and MIDI.

- [ADR-0021: Passage practice and comparable context](0021-passage-practice-and-comparable-context.md) — source-time A–B ranges, explicit parts, per-repeat attempts and truthful comparison context.

- [ADR-0022: Readable HUD and keyboard fit](0022-readable-hud-and-keyboard-fit.md) — independent Focus/Standard/Arcade density, explicit keyboard fitting and short-window pause.

- [ADR-0023: Measure snapping and repetition goals](0023-measure-snapping-and-repetition-goals.md) — exact MusicXML playback visits and voluntary 3×80% coverage goals.

- [ADR-0024: Configurable keyboard and piano bindings](0024-configurable-keyboard-and-piano-bindings.md) — persistent two-column matrix, capture/conflicts, shared actions and MIDI control-key prefix/chords.

- [ADR-0025: Personal themes and demonstrated hands](0025-personal-themes-and-demonstrated-hands.md) — persistent JSON themes, reviewed hand demonstrations and library-row double-click.

- [ADR-0026: Procedural 3D piano](0026-procedural-3d-piano.md) — PBR keyboard, one-surface texture composition and projected input geometry.

- [ADR-0027: Keyboard Explorer](0027-keyboard-explorer.md) — N02 octave transfers, explicit exercise kind and saved source/target evidence.

- [ADR-0028: Five Finger Flow](0028-five-finger-sequences.md) — reusable release-gated sequences, whole-pattern scoring and versioned pattern evidence.

- [ADR-0029: Sequential Play](0029-sequential-play.md) — shared clock/output barriers, exact chords, early input and comparable guided history.

- [ADR-0030: Major Scale Run](0030-major-scale-run.md) — one-octave scale plans, shared sequence input and durable key/hand evidence.

- [ADR-0031: Independent notation layers](0031-combinable-note-views.md) — combined sheet/falling views, live input markers and block note names.

- [ADR-0032: Scalable score overlay](0032-scalable-sheet-overlay.md) — full-height falling field and multi-system score.

- [ADR-0033: Written beam groups](0033-written-beam-groups.md) — authored/metric rhythmic groups, shared chord stems and scaled beams/hooks.

- [ADR-0034: Metadata providers and local artwork](0034-metadata-providers-and-local-artwork.md) — shared provider interface, reviewed MusicBrainz/CAA candidates and SQLite v6 covers.

- [ADR-0035: Library folders and bulk import](0035-library-folders-and-bulk-import.md) — saved roots, permission-aware startup scans and 500-file selections.

- [ADR-0036: Rhythmic reading and note glyphs](0036-rhythmic-reading-and-note-glyphs.md) - source-beat MIDI estimates and Bravura notes/flags.
