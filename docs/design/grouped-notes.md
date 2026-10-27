# Grouped notes

- Status: Implemented, 2026-10-27; browser visual review recorded below
- Owner request: notes should group like the joined-note symbol 🎵
- Decision: [ADR-0033](../adr/0033-written-beam-groups.md)

The horizontal MusicXML score should reveal rhythmic groups without adding
settings. Short notes share beams instead of carrying independent flags. An
onset containing a chord has one shared stem, so the beam joins rhythmic attacks
rather than every pitch. Retain note-head hand colours; neutral beams keep the
rhythm legible even when neighbouring notes have different assigned hands.

Use written MusicXML groups when complete and supported. Otherwise group short
notes conservatively within beats: two eighths in a quarter-note beat, three in
compound 6/8, 9/8 or 12/8. Rests, gaps, staff/voice changes and barlines separate
automatic groups. Mixed short durations use secondary beams and partial hooks.
Dense rhythms reserve space between attacks so short-note heads remain readable.
Groups share a stem direction with a restrained slope; standalone short notes
retain the correct number of flags. Written up/down stems take precedence.

Desktop assumptions: the existing scalable multi-system score and translucent
score overlay. Touch/narrow assumptions: the same measure layout at a smaller
viewport, respecting the saved scale. No new control or shortcut is required.
Falling-note timing, live input feedback, sequential gates and audio are unchanged.
MIDI has no authored meter and retains its approximate time-spaced reading aid.

This is a bounded engraving improvement, not complete engraving: cross-staff
beams, beams through rests, feathering, unusual stem modes, collision avoidance
for adjacent chord seconds and beams beyond 64ths remain outside this slice.
Tuplet note durations group exactly; tuplet brackets/numbers remain separate work.

The original six-measure synthetic fixture includes eighths, mixed 16ths/32nds,
partial hooks, chords, explicit down stems, a rest and 4/4↔6/8. It is test data,
not a new bundled piece. Actual app screenshots are reproducible with
`TANGENT_CAPTURE_BEAMS=1 node web/piano/capture.mjs`:

- [Desktop score](../screenshots/grouped-notes.png)
- [Narrow score](../screenshots/grouped-notes-mobile.png)

Visual review: actual 1280×960 and 390×844 captures inspected; dense 32nds remain
separated, up/down beams connect to their shared stems, and the score stays above
the keyboard. Short-window/view-switch checks also pass.
