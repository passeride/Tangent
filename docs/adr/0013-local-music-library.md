# ADR-0013: Persistent source music library

- Status: Accepted
- Decision date: 2026-10-25
- Delivery: First local library slice; CI and device validation tracked separately
- Related: ADR-0009, ADR-0011, ADR-0012

## Context

A fingerprint cannot reopen a score. Users need imports to survive app/browser
restarts and appear in a shared Bevy library view. Original MIDI and MusicXML
bytes preserve information that today's normalized model may not yet understand.

## Options

Persisting renderer/domain JSON ties saved music to an unstable serialization
schema. Referencing external paths does not work reliably in browsers or after
files move. Storing original bounded source bytes in SQLite works on both platforms
and keeps the shared Rust importer authoritative when reopening a piece.

## Decision

Schema v3 adds library_songs: an opaque source-content ID, title, detected format,
duration, note count, original source BLOB, import date and last-opened date.
Migrate v1 through v2 to v3 without losing settings, scores or fingerprints.
Save only after the Rust importer succeeds. Reopening reparses the original
bytes through that same importer. No Song/notation Deserialize contract is added.

The adapter hashes source bytes (SHA-256 on web, existing BLAKE3 on native) and
upserts identical sources within its catalog. Different arrangements remain
separate entries. IDs are local opaque identifiers, not composition identities.
The source is passed through a dedicated typed byte boundary, not a native JSON
byte array. Storage modules own SQL; the scene owns navigation and presentation.

The Bevy Library screen imports, pages through metadata, opens pieces paused and
removes their saved sources after an in-app confirmation. Page size adapts to
viewport height, at most six entries. Queries never return source BLOBs until
Open is selected. Main menu Play continues to open the current piece.

## Consequences

The browser still writes a complete SQLite image in one IndexedDB transaction.
This is a deliberately bounded initial slice: 16 MiB per source, 128 MiB total
source bytes and 500 pieces. Large libraries, worker/paged browser storage,
search/filter, composer/tag editing, folders, batch import, backup restore,
canonical-work grouping and scores remain follow-up work. Hashing and SQLite
operations run in native I/O, but this worker also drives audio lookahead;
isolating potentially slow storage from real-time scheduling needs profiling.
Source parsing still uses the shared scene import path and may hitch on large
files; async import/generation cancellation is a follow-up.

Export Database includes original sources. Clearing browser site data removes
that browser's library. Web and native keep separate local databases. Removing
a source leaves the independent recognition catalog intact in this slice; there
is no source-to-fingerprint ownership/cascade relation yet.

## Validation

SQLite tests cover migration, source-byte export/reopen, metadata-only paging,
idempotent imports, removal and size validation. Native tests reparse a saved
source. Browser acceptance imports a MusicXML, reloads the app, reopens the saved
piece in Bevy and checks confirmed removal. Existing import, settings, web
packaging and platform CI remain required. Physical MIDI/audio/GPU tests remain
separate from database acceptance.

## Provenance

Original library implementation for Tangent, using its existing independent
SQLite, serde and hash dependencies. No excluded legacy code or assets consulted.
