# Local music library

Choose **Library → Import files** and select one or more MIDI (.mid/.midi),
MusicXML (.xml/.musicxml) or compressed MusicXML (.mxl) files. Up to 20 files can
be selected together; successful imports save their original bytes on this device.
Identical source bytes reuse one entry. Unsupported/invalid files report an error.

Click the **Search** field and type part of a title or composer. Search ignores
case and supports Norwegian letters; `%` and `_` are ordinary characters. Ctrl+F
focuses search, Ctrl+A selects the current field, and Clear resets the query.
**Sort** cycles recent/title/composer/duration; the format button cycles
All/MIDI/MusicXML/MXL. Pagination applies after filtering. Search input currently
requires a hardware keyboard; mobile keyboard and clipboard support are follow-ups.

**Edit** changes title and composer, with Tab switching fields and Enter saving.
MusicXML composer information is read automatically when present. Edited metadata
survives reimport and reopening; original files are never rewritten. **Open** loads
the saved source through the shared importer, paused, with the edited display title.

**Remove → Confirm** deletes the saved source and, for linked entries, its
recognition index when the last source reference disappears. Standalone/demo
indexes and unlinked legacy fingerprints remain independent. **Export database**
in Settings includes original files and metadata. Restore and sync are not yet
implemented. Web and native have independent local databases.

Limits: 16 MiB per file, 128 MiB of source bytes, 500 pieces. Clearing browser site
data removes that browser's library. Save errors are shown; playback remains
available even when storage is unavailable. See [ADR-0014](adr/0014-library-search-and-metadata.md)
and the [short P-45 test](test-p45.md).
