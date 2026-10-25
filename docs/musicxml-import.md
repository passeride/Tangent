# MusicXML import

Import `.musicxml` or `.xml` from the in-engine pause/settings menu (corner gear,
Esc or F1), or Ctrl+O. The same player, audio scheduler and note views accept it.
The first subset supports UTF-8 `score-partwise`, written pitch/flats/sharps,
multiple parts, voices and bass/treble staves, chords, rests, backup/forward,
divisions changes, rational durations and adjacent sound ties. `sound tempo`
changes are global quarter-note BPM; default is 120 BPM. Normal measures are padded
to their meter, implicit pickups use actual length, and parts share measure offsets.

The stationary view uses authored spelling and initial staff clefs. It is still an
8-second performance-time reading aid, not a fully engraved sheet. Falling modes
remain aligned to piano keys. Exact rhythm fractions, voices, rests and tie flags
are preserved in Song.notation even where not drawn yet. Playback currently uses
a fixed velocity and the selected synthesized piano, with no score pedal or
articulation interpretation.

Compressed `.mxl`, score-timewise/opus, transposing instruments, non-G2/F4 clefs,
microtones and out-of-range pitches are explicit errors. Export uncompressed
concert-pitch piano MusicXML. Repeats/endings/jumps play once in written order and
produce warnings; grace notes are skipped with a warning. Visual-only metronome
marks and clef changes produce warnings. Dynamics, lyrics, layout, slurs, ornaments,
beams and other engraving detail are not retained/interpreted yet. XML import is
a documented subset, not schema validation or complete MusicXML conformance.

Import errors preserve the current song. Open the pause menu to inspect status and
warnings. No XML file can fetch external entities or resources. See ADR-0006 for
bounds, timing decisions and the adapter boundary.

Run `cargo test -p tangent-musicxml`, the workspace tests/Clippy and the WASM piano
browser smoke. `tests/fixtures/piano.musicxml` is an original synthetic score,
created for this repository; it exercises flats/chords/two staves/ties/tempo.

Next: compressed MXL container validation and bounded decompression; written
rhythm/attributes in the notation model; authored layout; repeat expansion with
an explicit playback-to-score mapping; dynamics/pedal and broader interoperability.
