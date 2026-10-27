# ADR-0035: Saved library folders and bulk import

- Status: Accepted
- Date: 2026-10-27
- Extends: [ADR-0014](0014-library-search-and-metadata.md), [ADR-0034](0034-metadata-providers-and-local-artwork.md)

## Context

The owner wants a folder containing all music, recursive scans and larger manual
imports. Multi-selection was already enabled in the platform pickers, but capped
at 20 files. Folder ingestion must preserve source identity, manual metadata,
artwork and scores, and must not replace the piece currently being played.

## Decision

Library → Folder provides Choose folder, Scan now and Remove folder setting.
Schema v7 adds a singleton library_folder path/name; existing sources and metadata
migrate unchanged. Native persists the actual path and scans it at startup. A
separate bounded worker performs directory discovery, parsing, fingerprinting
and SQLite writes, outside the render/audio loop. It opens its own connection
with a bounded busy timeout. Progress and a final catalog refresh cross the
existing message boundary. Single explicit file imports keep the existing open
behavior; native multi-file selections use background catalog ingestion.

Discovery includes .mid/.midi/.xml/.musicxml/.mxl, case insensitive. Skip child
symlinks, cap traversal at 10,000 entries/32 levels and read one bounded source
at a time. Hash source bytes before parsing; already imported bytes are unchanged
and retain all annotations. Continue past invalid/oversized/unreadable files with
counts and a final error summary. A changed file has a new content identity;
external removal never deletes a saved source. Removing the folder setting only
stops automatic scans. This is startup/manual scanning, not a live filesystem
watcher or a destructive mirror.

Web persists a FileSystemDirectoryHandle separately in the existing IndexedDB
files store, and a display name in SQLite. Startup scans only when queryPermission
already grants read access; it never triggers a permission prompt itself. Scan now
can request permission from user interaction. Unsupported browsers use a hidden
webkitdirectory/multiple input and must reselect the folder for another scan.
No arbitrary absolute path is exposed or promised in browsers. Handles are not
included in SQLite exports; capability permissions must be established on a new
device. Directory iteration/filtering is JS capability code; validated imports
and source metadata still come from the Rust importer.

Web batch imports carry catalog_only through the existing serialized import/save
receipt pipeline. They do not mutate the active song/transport. Folder scans skip
known source hashes; regular explicit reimport retains the established update
semantics. Final refresh follows durable saves, not merely file reads. Native and
web pickers accept up to 500 selected files; the pre-existing 500-piece/128 MiB
catalog and 16 MiB/source bounds remain. The Windows dialog explicitly explains
Ctrl/Shift multi-selection. All controls/progress remain in the Bevy canvas.

## Alternatives and consequences

Loading each scanned file into the player would interrupt practice and repeatedly
reset scoring/transport. Blocking directory I/O on the audio worker would harm
responsiveness. A native watcher dependency and a cross-browser permission bypass
are unnecessary for the requested startup/manual flow. Saved snapshots remain
independent of folder availability; automatic removal and path-based identity
would contradict the library's established source ownership model.

References consulted:
- https://developer.mozilla.org/en-US/docs/Web/API/Window/showDirectoryPicker
- https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/webkitdirectory
- https://developer.chrome.com/docs/capabilities/web-apis/file-system-access

## Validation

Native tests cover nested discovery, invalid files, content deduplication and
manual metadata preservation. JS tests cover directory filtering, traversal limits,
persisted-handle startup, permission prompts only on interactive scans and clear.
The actual Bevy WASM test selects 25 files at once, scans a real persisted OPFS
handle with nested files/duplicates/bad MIDI, adds files and rescans/restarts,
checks active-piece preservation, removes the setting without deleting music,
then uses the actual directory-upload chooser fallback. Narrow/short controls
are bounded. OS folder dialogs and permissions on physical end-user machines
remain manual platform checks.

See [the UI brief and actual screenshot](../design/library-folders.md).

Delivery checks: all 140 workspace Rust tests, native/workspace and WASM
warnings-denied Clippy, formatting, release WASM, native executable startup/import
self-test, folder API/schema checks, actual folder/library/metadata/storage/GPU
audio browser scenarios and packaged `/Tangent/` smoke pass. The actual folder
screenshot was inspected. OS dialogs on physical Windows/macOS remain manual.
