# ADR-0034: Metadata providers and local reviewed artwork

- Status: Accepted
- Date: 2026-10-27
- Extends: [ADR-0014](0014-library-search-and-metadata.md), [ADR-0009](0009-local-sqlite-storage.md)
- Brief/provider evaluation: [library artwork](../design/library-metadata-providers.md)

## Context

The owner requested an interface for external metadata providers, primarily to
obtain album covers for MIDI/MusicXML/MXL library entries. A score or arrangement
is not a specific recording: name-only matches must not silently establish an
album identity or rewrite local metadata. Web/native must share provider matching
and review behavior while keeping HTTP/storage out of the render/audio thread.

## Decision

`tangent-core::metadata` owns the renderer-independent provider trait/registry,
query cleanup/escaping, candidate validation and parsing/ranking. The contract has
provider ID/name, search URL generation, identity validation, source/cover URL
mapping and candidate parsing. New adapters register here rather than adding
provider-specific UI. The first provider is MusicBrainz recording search plus
Cover Art Archive release-front thumbnails. The normalized candidate stores track,
artist credits, album, release date/country, recording length, disambiguation,
recording/release/release-group IDs, ISRCs when returned and provider relevance/
exact-text evidence. Identity validation and URL mapping belong to the provider.
The first adapter's release/recording IDs are MusicBrainz IDs; a later adapter
must extend transport/storage validation together, not bypass those boundaries.

Bevy owns explicit Search → Preview → Use actions and cancellation tickets.
Composer is an optional artist search hint, with a title-only alternative.
Query text comes from local/imported title/composer, not uploaded source bytes or
fingerprint/audio data. Imported songs do not trigger background requests.
Candidates never replace local title/composer or affect musical identity/scoring.
MusicBrainz credits are recording artists, never assumed composers. Complete
lookup chains, work relationships, genre taxonomies and automatic fingerprint-to-
provider matching remain separate improvements.

Web fetch and native ureq implement bounded HTTPS GET capabilities, with fixed
provider endpoint allowlists, 20-second timeouts, 1 MiB responses and no provider
credentials. Serialize/cache requests for ten minutes (32 entries); MusicBrainz
search starts at least 1100 ms apart. No public CORS proxy or JSONP script loading.
Native uses a dedicated eight-job HTTP worker, independent of the audio/device/
SQLite loop, and sends a named User-Agent with repository contact. Browsers manage
User-Agent themselves; requests include the named client parameter and origin
referrer. End-user browser CORS/service availability is a runtime constraint,
reported as an error rather than bypassed. Live service access was not verifiable
from this execution environment; see the brief's evidence boundary.

Shared bounded PNG/JPEG decoding lives in `tangent-player::cover`; Bevy consumes
RGBA pixels rather than asking JS to render images. Dimensions/allocations are
bounded before full decoding. UI caches handles only for requested local covers;
cover epochs prevent late local reads from restoring removed artwork. Missing or
invalid covers retain a themed music-note placeholder. Short/narrow screens use
the same Bevy review controls, with no HTML UI or second canvas.

SQLite schema v6 adds `library_enrichment`, keyed to the exact source entry with
ON DELETE CASCADE. Store validated candidate JSON, cover BLOB, Unicode-lowercased
annotation search key and saved-at timestamp. Maximum total covers: 32 MiB, separate
from existing source-byte limits. Both web/native migrations preserve v5 data,
settings and practice history, and reject future schemas without replacing them.
Reimport/local edits preserve annotations; removal can clear only annotations or
cascade with source deletion. Export includes saved artwork. Library reads return
annotation summaries without image BLOBs; images load only for displayed entries.
Opening a library never refetches stored cover URLs.

## Alternatives and consequences

Direct remote image URLs would break offline libraries, repeatedly contact third
parties and complicate WebGL/CORS. Auto-applying the first search result would
mislabel compositions and arrangements. Fetching from the audio thread would
introduce device latency. Duplicating the provider parser/ranker in JS/native would
create conflicting matches. Adding a server/proxy is unnecessary for the static
site model; unsupported CORS is surfaced honestly.

ureq 3.5.0 (MIT OR Apache-2.0, rustls) supplies native HTTP; image 0.25.10 was
already resolved by Bevy and now explicitly enables bounded PNG/JPEG decoding.
Resolved package sources/licenses are in THIRD_PARTY.md/notices. MusicBrainz core
metadata and cover-image rights are distinct; real album art is not shipped as a
repository asset. The tests/screenshots use original synthetic data and artwork.

## Verification

Core tests cover filename cleanup, escaped queries, provider registration,
identity/response validation, artist separation and matching evidence. Web storage
and native SQLite tests cover migration, original data preservation, reimport,
cover export/reopen, annotation search and deletion. HTTP capability tests cover
caching/rate limiting, endpoint/response bounds and errors. The actual WASM Bevy
scenario searches/previews/saves a synthetic release, checks no premature save,
manual composer preservation, no remote fetch on reopen, remove behavior and
narrow/short controls. Live catalogue/service availability remains a separate
manual check, not inferred from intercepted HTTP tests.

Delivery verification: all 139 workspace Rust tests pass, including native
SQLite annotations and bounded raster decoding. Native/workspace and WASM
warnings-denied Clippy, formatting, the release WASM build, HTTP/schema unit
checks, metadata/library/storage/baseline GPU/audio browser scenarios, packaged
`/Tangent/` smoke and native executable startup/import self-test pass. Both actual
Bevy screenshots were visually inspected. Live MusicBrainz/CAA catalogue access
remains unverified here as documented above; intercepted tests do not establish it.
