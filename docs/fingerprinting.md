# Music fingerprinting (MVP)

Tangent recognizes short **symbolic MIDI** excerpts from locally imported songs.
This is not microphone/audio fingerprinting. Recognition is available during a
**FreeForm** session; import a MusicXML or MIDI file first, then play a passage
on a connected keyboard or the on-screen keyboard.

## Implemented

- Rust core extracts the highest pitch at each onset within each
  non-percussion part. Every five-note window is encoded as four signed
  semitone intervals, making it invariant to octave changes and global
  transposition.
- Browser SQLite schema v2 saves only these extracted melodic lanes and
  fingerprint positions. An indexed lookup produces candidates; the Rust
  weighted subsequence-edit matcher accepts tempo differences, limited
  incorrect/omitted notes and excerpts from the middle of a piece.
- Live FreeForm input collects up to 24 notes, collapses simultaneous
  (within 40 ms) onsets to the highest key, searches at most every 250ms
  once five notes are available, and shows the match title, zero-based
  position converted to a one-based note number, and **interval similarity**.
  It is not a probability or calibrated confidence.
- The bundled First Light study is indexed when the app starts. Successful
  MusicXML/MIDI imports are added to the local catalog. Reimporting identical
  content replaces its index idempotently.

The browser origin/profile owns the catalog. Export Database includes
fingerprints, but **not the original MusicXML or MIDI file**; the app cannot
reopen a previous file from the fingerprint alone.

## Boundaries

This first release does not recognize microphone audio, identify chords as
polyphonic events, select independent voices in a complex piano reduction,
merge different arrangements of a work, or report an authored measure. Similar
melodic passages can yield ambiguous matches; a score under the heuristic
threshold is not shown. A large song library needs a more scalable database
backend and approximate fallback indexing.

Compressed MXL import is a separate branch, not a dependency of this patch.

## Development checks

- `cargo test -p tangent-core`
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cd web/piano && npm test` (requires WASM app and Playwright browser setup)
- `cd web/piano && node fingerprint-schema.test.mjs` (SQLite unit test)

See [ADR-0010](adr/0010-symbolic-fingerprinting.md) for boundaries.
