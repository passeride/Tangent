# MusicXML import

Import `.musicxml` or `.xml` from the in-engine pause/settings menu (corner gear,
Esc or F1), or Ctrl+O. The same player, audio scheduler and note views accept it.
The first subset supports UTF-8 `score-partwise`, written pitch/flats/sharps,
multiple parts, voices and bass/treble staves, chords, rests, backup/forward,
divisions changes, rational durations and adjacent sound ties, meter/key signatures, written note values/dots, repeats and numbered endings. `sound tempo`
changes are global quarter-note BPM; default is 120 BPM. Normal measures are padded
to their meter, implicit pickups use actual length, and parts share measure offsets.

The stationary view uses written measures with real Bravura clefs, inherited
meter/key, quarter-note tempo, barlines, measure numbers, repeat signs and ending
brackets. Two measures fit narrow screens, four fit desktop. Its playhead follows
performance visits back to written positions on repeat. Falling staff/notes show
transparent clefs, measure/beat guides and numbered-ending brackets. Authored rests,
note values and dots are retained/drawn; layout is still basic, without full
engraving collision avoidance, beaming or refined tie curves.

Sequential non-nested repeats support 2–8 passes with comma-separated ending
numbers. The demo exercises first/second endings, including multi-measure endings.
The visit map holds local quarter/time points so tempo changes and jumps stay
synchronized without musical timing logic in Bevy. Metronome beat-unit/per-minute
(including up to three dots) is accepted when no sound tempo is supplied.
Playback uses fixed velocity and the chosen synthesized piano; score pedal,
dynamics and articulation playback remain future work.

Compressed `.mxl`, score-timewise/opus, transposing instruments, non-G2/F4 clefs,
microtones and out-of-range pitches are explicit errors. Export uncompressed
concert-pitch piano MusicXML. Nested/unclosed repeats and malformed endings are errors. D.C./D.S./coda
navigation plays in written order with a warning; grace notes are skipped with a warning. Unsupported metronome relationships and clef changes produce warnings. Dynamics, lyrics, layout, slurs, ornaments,
beams and other engraving detail are not retained/interpreted yet. XML import is
a documented subset, not schema validation or complete MusicXML conformance.

Import errors preserve the current song. Open the pause menu to inspect status and
warnings. No XML file can fetch external entities or resources. See ADR-0006 for
bounds, timing decisions and the adapter boundary.

Run `cargo test -p tangent-musicxml`, the workspace tests/Clippy and the WASM piano
browser smoke. `tests/fixtures/piano.musicxml` is an original synthetic score,
created for this repository; it exercises flats/chords/two staves/ties/tempo.

Next: compressed MXL container validation and bounded decompression; richer
authored engraving and nested navigation; dynamics/pedal and broader interoperability.
See ADR-0007 for the current written-score/performance-visit contract.
