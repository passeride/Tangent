# ADR-0010: Bounded compressed MusicXML container import

- Status: Accepted
- Decision date: 2026-10-25
- Delivery: Bounded in-memory MXL reader, importer integration, and tests
- Related: [ADR-0006](0006-musicxml-import-and-authored-notation.md), [ADR-0007](0007-authored-measures-repeats-and-music-glyphs.md)

## Context

MusicXML exports commonly use compressed .mxl files. Tangent already imports
uncompressed UTF-8 score-partwise through a format-neutral Song boundary, but
rejected ZIP input. Browser and desktop launchers share the same Rust/WASM
importer, so format handling belongs in tangent-musicxml rather than JS.

## Decision

Detect ZIP magic bytes in the MusicXML adapter. Parse its ZIP central directory,
read META-INF/container.xml and use the first rootfile's full-path, as specified
by MusicXML 4.0. Support stored and DEFLATE entries, including the older archives
without a mimetype file. Additional non-score assets are not extracted or loaded.
Pass decompressed score bytes to the existing secure XML parser unchanged.

Use existing lockfile versions of miniz_oxide 0.9.1 for raw DEFLATE and crc32fast
1.5.3 for integrity checks; they are already inventoried under their upstream
licenses. The bounded ZIP container module has no Bevy, MIDI, browser, or
filesystem dependencies. Compression does not alter the common Song model.

Keep the 16 MiB input size and 16 MiB decompressed score size limits. Bound
container.xml to 64 KiB, ZIP entries to 4,096, and its XML nodes to 1,024.
Verify ZIP offsets, local/central names, declared lengths and CRC32; reject
duplicate paths, traversal/absolute paths, encryption, unsupported compression,
multi-volume ZIP and ZIP64. The selected rootfile must be MusicXML when its media
type is present. Do not resolve external files/resources, expand XML entities,
or write archive content to disk.

## Alternatives and consequences

JavaScript ZIP processing would duplicate an import rule across targets and
complicate the wasm boundary. A full ZIP crate would add another dependency and
features not needed for the current bounded, in-memory subset. The limited ZIP
reader intentionally does not support ZIP64, encrypted archives, arbitrary ZIP
features or images linked from score XML. Malformed or unsupported containers
return errors without replacing the currently loaded Song.

The existing score-partwise limitation still applies: adding MXL does not make
score-timewise/opus, transpositions, microtones or full notation supported.
Synthetic archive fixtures cover stored/deflated entries, old-format manifests,
corruption, unsafe paths, duplicate names, and decompression-size limits.

## References

- [MusicXML 4.0 compressed MXL tutorial](https://www.w3.org/2021/06/musicxml40/tutorial/compressed-mxl-files/)
- [miniz_oxide 0.9.1](https://docs.rs/miniz_oxide/0.9.1/miniz_oxide/)
- [crc32fast 1.5.3](https://docs.rs/crc32fast/1.5.3/crc32fast/)
