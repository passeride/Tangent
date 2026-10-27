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

## Album covers and external metadata

Choose **Edit → Find cover**, preview an album release, then **Use cover &
metadata**. The first provider is MusicBrainz with Cover Art Archive. Search uses
the displayed title/composer as title/artist hints; **Title only** helps when a
classical composer differs from a recording artist. Matching is a suggestion,
not a claim about the original arrangement. No score/MIDI bytes are uploaded.

Saved artwork works offline and survives reimport/reopen. Local title/composer
remain yours; artist/album/date/country and source IDs are separate annotations.
Search also finds saved album/artist text. **Remove metadata** keeps your source
and local edits. Missing covers keep a themed placeholder and can save metadata
only. Database export includes covers (32 MiB total cover budget). See
[the provider guide](design/library-metadata-providers.md) for service conditions,
matching limitations and the explicit live-verification boundary.
