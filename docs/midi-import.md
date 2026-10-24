# MIDI import — first implemented element

The import boundary consists of two crates in the Rust workspace:

- `tangent-midi`: pure byte-based SMF import with owned musical-performance data.
- `tangent-midi-web`: a small WASM/JSON boundary for browser inspection.

The importer itself has no UI/audio/device dependencies. A separate
[Bevy-core browser piano](piano-player.md) now consumes it. This import-only slice
is the foundation those consumers can use without importing another project's
wrappers or models.

## Native use

Install Rust/rustup. The repository pins its tested toolchain in
`rust-toolchain.toml`; rustup will install it on first use.

```sh
cargo test --workspace --locked
cargo run -p tangent-midi --example inspect -- path/to/piece.mid
```

Library use:

```rust
let song = tangent_midi::import(bytes)?;
// song.tracks, song.tempos, song.diagnostics
```

The API returns `ImportError` with code, message and source location when known.
Neither the importer nor the model reads files. The example opens the requested
file and prints JSON; the library remains equally usable for browser bytes.

## Supported input and preservation

- Standard MIDI Files 0/1, positive ticks-per-quarter timing.
- Original track indices/order/names, channels and MIDI ports.
- NoteOn/NoteOff including zero-velocity NoteOn as release; attack/release velocity,
  source event, source tick and derived microsecond positions.
- Complete tempo map; default 500,000 microseconds per quarter before an authored
  change. Fractional timing carries across segments before rounding down once.
- Time/key signatures, raw text bytes plus lossy UTF-8 display text, instrument/
  program names, lyrics, markers and copyright fields when supplied.
- Controllers (including sustain), program changes, pitch bend and pressure.
- SysEx/escaped data and remaining metadata retained as data, never executed.
- Trailing silence through the latest track end; empty tracks are valid.

The model is not a written score or a promise of audible duration. Channel 10
is marked as percussion by the helper; notes are retained rather than removed.
No automatic hand, chord, score spelling or difficulty interpretation is performed.

## Explicit limits and policies

Format 2 and SMPTE timing are currently errors. Tracks must end with EndOfTrack;
events following it, missing tracks, malformed input, tempo zero and conflicting
simultaneous tempos are errors. Defaults: 16 MiB input, 256 tracks, 250,000 total
events and 24 hours duration. Custom limits can be passed with `import_with_limits`.

Track/port/channel/key pairing is deterministic. Same-key overlap uses FIFO and
produces `OverlappingKey`. Unmatched NoteOff produces `OrphanRelease`; an unclosed
NoteOn ends at that track's end and produces `UnreleasedNote`/`TrackEnd`. Zero-length
source notes are retained. These diagnostics matter before scoring can treat the
file as a reference. Pairing does not cross source tracks.

CC120/123 closes pending keys in the affected source track/port/channel with a
`ChannelMode` ending. Playback adapters must merge events and interpret
shared channel/pedal state; import does not synthesize a sustain tail or perform
all audible-voice processing. Do not use derived note spans as the only source for
MIDI playback.

## Web inspection

See [the browser harness](../web/midi-import/README.md) to build and serve the
WASM importer. It includes a new three-note study and shows file metadata, note
counts, duration, diagnostics and raw imported JSON. It has no upload endpoint,
no persistence and no MIDI/audio permission request. Generated bindings are build
artifacts, not committed sources.

CI runs native tests on Linux/Windows, builds wasm and packages the web harness.
The web job also runs a real Chromium/WASM smoke test for import, error reporting
and recovery. See [.github/workflows/check.yml](../.github/workflows/check.yml).
A queued CI job is not a completed platform verification.

## Verification commands

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo build -p tangent-midi-web --target wasm32-unknown-unknown --release --locked
```

Browser smoke instructions are in the harness README. Test fixtures are generated
from new event sequences in this repository using midly's writer, with separate
expected timings. Malformed input tests also mutate/truncate the encoded data.
No original/fork test files or music assets are used.

See [requirements and primary sources](spec/midi-import.md) and
[ADR-0002](adr/0002-byte-based-midi-import.md). The separate
[transport/virtual-piano slice](piano-player.md) now consumes this boundary.

## Current verification record (2026-10-24)

The import boundary has 21 passing Rust tests (20 importer tests and one
JSON-boundary test), along with
formatting, Clippy and a release WASM build. The real Chromium smoke test passes
for the included study, malformed input and recovery. The native CLI was checked
on the same newly authored study. Windows CI is configured but has not been
verified locally. This record concerns the import-only harness; piano/audio
verification is recorded separately in [the piano guide](piano-player.md).
