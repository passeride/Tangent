# ADR-0014: Searchable library metadata and source ownership

- Status: Accepted
- Decision date: 2026-10-25
- Delivery: Schema v4, shared Bevy search/edit controls, batch import and tests
- Related: ADR-0011, ADR-0012, ADR-0013
- Supersedes: ADR-0013's deferred search/edit and independent-source-removal behavior

## Context and options

Saved pieces need searchable titles/composers, sorting and efficient multi-file
import. Returning all source BLOBs to the UI would waste memory. Editing the
original score would change content identity and lose provenance. An external
HTML/native form would duplicate the established Bevy UI.

## Decision

Schema v4 adds composer, a Unicode-lowercased search key and optional fingerprint
ownership to library_songs. Both adapters migrate v3 title data into search keys.
MusicXML composer creators populate the format-neutral Song metadata; MIDI leaves
composer absent. User edits live in SQLite and survive reimport/reopen without
rewriting original source bytes. Reopened playback uses the saved display title.

Search is a literal, case-insensitive substring of title/composer, bound as SQL
parameters with instr rather than SQL wildcard syntax. Sort uses a fixed allowlist
(recent, title, composer, duration), combined with an optional exact format filter.
NOCASE sorting uses SQLite's default collation, not locale-specific Norwegian
alphabetical ordering. Results are paged metadata; source bytes are read on Open.

Keyboard text events edit Bevy fields, with Unicode text, Backspace, Ctrl+A, Tab,
Enter and Escape. Search debounces for 200 ms and invalidates old query tickets
as soon as the text changes. Mobile soft-keyboard/IME/clipboard bridges remain
follow-up work; the current text fields support a hardware keyboard.

Imports associate the original source with the recognition representation in the
same storage transaction. Renaming also updates its displayed recognition title.
Deleting a source removes its owned fingerprint only when no other source refers
to it. Standalone indexes, the bundled demo and unlinked pre-v4 fingerprints are
not treated as owned sources. Reopening/reimporting a legacy source links it.

File dialogs accept up to 20 files per batch. Browser imports await each parse and
save receipt before reading the next source. Native file delivery uses a bounded
two-item byte channel and the scene parses at most one source per frame. Import
failures remain visible; original bytes and metadata from failed saves are not
committed. Existing 16 MiB/source and 128 MiB/500-piece limits remain.

## Consequences and validation

This completes the basic local library workflow. It does not add folders, tags,
backup restore, canonical arrangements, synchronization or score recording.
Rendering, native audio and OS device behavior still need physical verification.
Storage and source parsing are not yet a fully isolated real-time pipeline;
large-file profiling remains in the backlog.

Test migrations, literal/Unicode search, sort/filter and paging, metadata edits,
reimport preservation, source restart/reopen and last-owner fingerprint deletion.
Exercise actual Bevy text input, multi-file import, restart, metadata and removal
in Chromium. Windows packages include TEST-P45.md and a short report launcher;
that manual check verifies hardware rather than claiming CI measures MIDI latency.

## Provenance

Original Tangent implementation using existing SQLite/serde/Bevy dependencies.
KeyboardInput's text/state contract was checked against the official Bevy v0.20.0
source. No excluded legacy implementation, fixtures or assets were consulted.
