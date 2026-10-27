# Saved library folders

- Status: Implemented, 2026-10-27
- Owner goal: put music in a folder, scan it, and import many files at once
- Decision: [ADR-0035](../adr/0035-library-folders-and-bulk-import.md)

Library retains Import, Refresh and Back, with Folder as a fourth short toolbar
action. The folder page displays the saved path/name, startup behavior, Choose
folder, Scan now, Remove folder setting and a persistent progress/result line.
Back/Escape return to the library. Busy scans hide actions that would overlap jobs.
Controls fit 390px-wide and 420px-high viewports in the actual Bevy test.

Native startup/manual scans use an independent ingest worker. Browser startup
requires a saved handle and already granted read permission; otherwise Scan now
requests access, or the directory-upload fallback requires reselecting the folder.
Do not promise browser access to an arbitrary path or automatic permission renewal.

Background imports leave the active piece alone. Duplicate source bytes, edited
metadata, artwork and history remain attached. Invalid files do not stop the
remaining imports. Removing the setting or an external file never deletes saved
music. Folder imports are snapshots, not a live filesystem mirror.

[Actual folder screen](../screenshots/library-folder.png), using original synthetic
MusicXML. Reproduce with `TANGENT_CAPTURE_FOLDER=1 node web/piano/capture.mjs`.
The scenario includes a 25-file selection, nested/duplicate/invalid files, rescan,
startup handle reuse, setting removal and the real directory-upload chooser.
