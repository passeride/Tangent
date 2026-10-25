# ADR-0009: Local SQLite persistence for the shared browser application

- Status: Accepted
- Decision date: 2026-10-25
- Delivery: Settings persistence, SQLite export and future score schema implemented
- Related: [ADR-0005](0005-one-bevy-gpu-surface.md), [ADR-0008](0008-navigation-and-practice.md)
- Supersedes: ADR-0004's random launcher port only

## Context and options

The owner requests SQLite for settings and future scores. Pages is static hosting;
it cannot host a server-side database. A native-only SQLite service would leave
web users without storage and require a new write API. OPFS SQLite configurations
can impose browser/worker/header requirements beyond the existing Pages surface.
Plain JSON storage would not meet the requested database format.

## Decision

Use npm-locked sql.js 1.15.1, SQLite compiled to WASM, in a separate browser storage
adapter. Ship its JS/WASM/license locally; no runtime CDN or backend is required.
The actual SQLite file image is stored as `tangent.sqlite` in the `files` object
store of IndexedDB `tangent-local`, version 1. IndexedDB is the durable file backing;
SQLite owns the schema and queries. This does not imply account sync or a server DB.

Each change opens the latest image inside one IndexedDB readwrite transaction,
runs synchronous SQLite migration/parameterized updates and writes a complete
image atomically. Competing connections/tabs serialize on that transaction;
settings use last-writer-wins semantics. The adapter never writes stale cached
images. Opening a database from a newer version or a corrupt database fails without
replacing it. Saving failures leave the previous committed image intact and show
a visible status in Bevy settings. The piano remains usable when storage fails.

`tangent-player::preferences` defines and validates the versioned settings payload
without platform types. Scene state restores validated settings before input and
sends changes over the capability boundary. Store view, theme, effects, volume,
instrument, song sound and piano range. Do not restore playback, held notes, device
permissions or application navigation. No scoring rules live in the storage layer.

Schema v1 uses PRAGMA user_version and creates settings (singleton JSON payload
plus timestamp), practice_sessions (id, mode, start/end timestamps), and scores
(id, session FK, metric, numeric value, timestamp), with a session index. Scores
and sessions remain empty: defining score meaning and recording sessions is later
work. Future migrations must preserve data and run in order. Settings payload
version is independent of the database schema version.

Provide Export database inside the Bevy settings screen. It downloads a real
SQLite database after queued saves finish. Backup restore/import remains future
work. Imported music is not persisted by this change.

The browser-backed launcher now defaults to loopback port 38765, making its origin
stable across restarts. TANGENT_PORT can override this (0 requests a temporary port
for tests). Changing the port changes the browser storage origin; never silently
fall back to another port when the default is busy. HTTP remains read-only.

## Consequences and validation

Data is local to a browser profile/origin. Pages and the local launcher have
separate databases; clearing site data removes the browser copy. Private browsing,
quota limits and browser eviction still apply. Export is the portable backup.
Full-image writes suit today's small settings DB; move to paged/worker/native
storage before growing a large song library or dense score history.

Verify schema migration/reopen, scores FK integrity, newer-version preservation,
real browser restart/settings restore, valid SQLite export, quota failure with
old data intact, concurrent saves and unavailable-storage playback. Package and
launcher tests must verify the SQLite WASM is shipped with the correct MIME type.

## References and provenance

Original adapter/schema/preferences authored for Tangent; no excluded legacy
source/assets consulted. Prior exposure remains disclosed in PROVENANCE.md.
Primary source: https://github.com/sql-js/sql.js and its API documentation,
consulted 2026-10-25. sql.js is MIT; SQLite is public domain. The npm lock pins
integrity; upstream complete license is in licenses/sql.js-MIT.txt and distributions.
