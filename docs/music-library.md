# Local music library

Choose **Library** in the main menu, then **Import** a MIDI (.mid/.midi),
MusicXML (.xml/.musicxml) or compressed MusicXML (.mxl) file. Successful imports
are saved on this device. Use **Open** to reopen a saved piece in Play mode,
paused. Reimporting the same source bytes updates its entry rather than adding
a duplicate. The list includes title, format, duration and number of notes.

**Remove** asks for a second click on **Confirm**. This removes the saved source;
the separate recognition index currently remains. The original file elsewhere
on your computer is unchanged. Export Database in Settings includes the library.
Backup restore and web/native synchronization are not implemented.

Limits: 16 MiB per file, 128 MiB of source bytes, 500 pieces. Web storage belongs
to this browser profile/origin; clearing site data removes it. Native SQLite lives
in the standard application data folder. A failed save leaves playback available
and shows an error; do not assume a file was saved when an error appears.

This first slice does not add composer tags, folders, search or recorded scores.
See [ADR-0013](adr/0013-local-music-library.md) for the schema and trade-offs.
