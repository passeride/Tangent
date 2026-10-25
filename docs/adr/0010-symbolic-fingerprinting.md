# ADR-0010: Symbolic fingerprint search and local recognition

- Status: Accepted
- Decision date: 2026-10-25
- Delivery: Initial melodic fingerprint extraction, SQLite catalog, fuzzy matcher and FreeForm MIDI integration
- Related: [ADR-0003](0003-performance-model-and-piano-web-slice.md), [ADR-0009](0009-local-sqlite-storage.md)

## Context

The owner wants to identify any short MIDI performance excerpt against locally
imported music, including one-octave shifts, key transpositions, missing or
incorrect notes, and positions other than the opening measure. MusicXML (and MIDI)
already becomes the shared `Song` model; no microphone transcription is needed.
SQLite is already local in the browser.

## Decision

Place symbolic extraction and fuzzy matching in `tangent-core::fingerprint`.
A part is reduced to a monophonic lane: simultaneous notes choose the top pitch;
percussion is ignored. Four signed semitone intervals (five notes) become
deterministic hexadecimal fingerprints. The browser adapter only hashes the
indexed payload for a stable ID, persists the Rust-produced lanes and fingerprints
in SQLite schema v2, and returns candidates selected through an indexed join.

A transposition-independent, weighted edit-distance alignment over interval
sequences reranks candidates. The first score position is returned in performance
note order with a microsecond onset. Similarity is **not a calibrated
probability**. No composition/arrangement identity is inferred from the title:
each imported score is a content-addressed local entry. With no exact fingerprint
hit, scan a bounded 30-lane fallback to admit noisy query segments.

The Bevy FreeForm session maintains the most recent 24 live key onsets, reduces
near-simultaneous (40ms) notes to the highest key, queries SQLite at most every
250ms, and displays a best-match title, note position, and interval similarity
inside the existing Bevy canvas. Query tickets discard stale asynchronous
responses. The bundled study is indexed on launch; imported scores are indexed
after parsing succeeds. Browser storage is local to the origin and profile, with
no audio capture, cloud service or backend.

## Trade-offs / follow-up

- Piano polyphony, independent voice separation and cross-arrangement linking
  are not implemented. A high-note heuristic is poor on accompaniment-only
  passages or chords.
- SQLite stores extracted lanes, not the original MusicXML files; successful
  identification does not imply that the original score can be reopened.
- The v2 SQLite catalog is a small-library MVP. Full-image IndexedDB writes,
  query transactions and the fallback scan require profiling/limits for a
  larger library. A configurable minimum evidence margin and false-positive
  calibration must precede confidence percentages in the UI.
- A note index (not a notation measure) is reported. Written measure mapping
  through repeat visits is future work.
- The existing separate MXL branch is not included in this change; compressed
  file support depends on that branch being merged separately.

## Verification

- `cargo test -p tangent-core` covers transposition, octave shifts,
  mid-score excerpts, a wrong or omitted note, percussion exclusion,
  top-voice extraction, minimum length and unrelated patterns.
- `node web/piano/fingerprint-schema.test.mjs` (run from `web/piano`) covers
  v1-to-v2 migration, inverted lookups, idempotent reindex, fallback and export.
- CI also runs WASM Clippy, all-platform Cargo checks, browser smoke and packaging.
  Physical MIDI-device acceptance and large-library benchmarks remain pending.

## Provenance

Original symbolic algorithms and persistence adapter authored for Tangent.
No excluded legacy implementation or assets were copied.
