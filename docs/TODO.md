# Follow-up work

Keep implemented behavior separate from planned work. Decisions belong in ADRs;
this file tracks the remaining work, not an authorization gate.

## Piano/playback foundation

- Profile the delivered Bevy UI/wgpu surface on real desktop/mobile GPUs.
  WebGL2 is the current backend; evaluate WebGPU separately when justified.
- [x] Move native audio lookahead off the render thread;
  add output/display latency calibration and stress cases with dense piano files.
- Extract controller/voice-state reconstruction into a shared, testable reducer
  when native audio is added; currently owned by the browser sound adapter.
- Improve voice stealing and profile/sample loading behind the audio boundary;
  evaluate a piano synthesizer/SoundFont with independently documented licenses.
- Verify actual MIDI hardware, device disconnects, sustain input and MIDI latency;
  add port selection and browser capability coverage. No SysEx requirement.
- Index long sustained notes and visible windows; frame JSON and prefix scanning
  are prototype boundaries and need dense/long-song profiling.
- [x] Save user settings in local SQLite; export a portable database (ADR-0009).
- [x] Add a local SQLite symbolic fingerprint index and FreeForm MIDI identification (ADR-0011); original score files still are not persisted.
- Add music-library persistence, database backup restore and meaningful score/session recording.
- [x] Add a true native renderer, audio mixer, MIDI inputs and SQLite adapter
  without browser or localhost server (ADR-0012). Validate release binaries on
  physical Windows/macOS/Linux machines and profile voice/audio latency.
- Add OS-integrated drag/drop and robust cross-platform native file picker
  (currently PowerShell, AppleScript and optional Zenity).
- Extend fingerprint search with polyphonic voices, multiple arrangements per work, calibrated match confidence and large-library performance.
- Add managed desktop lifecycle/quit UI, signing and macOS notarization.

## Reading views

- Expand the initial MusicXML subset (see below). MIDI must continue to work
  without invented spelling.
- Add a notation renderer/layout boundary for rests, key/time signatures, beams,
  tuplets, ties across pages, stem direction, collisions and accidental context.
- Improve narrow-screen score density and auto-follow paging. The current sheet
  view uses time spacing and eight-second pages, not engraved measures.
- Evaluate the rotated-staff learning projection with users; keyboard spacing
  differs from conventional equal staff steps, and dense chords need layout work.
- Add key-range auto-fit/zoom and a clear indicator for off-screen pitches.

## Product layers

- Main/Train menus, FreeForm and basic note/chord matching are delivered in
  ADR-0008. Extend the fixed C-major exercises with bass-clef targets, curriculum
  options and feedback. Timed scoring, progress persistence and adaptive exercise
  selection remain future work in the domain layer.
- Add full reduced-motion preferences, contrast checks and accessible Bevy UI semantics for
  note/event information. Effects can already be disabled manually.

## MusicXML follow-up (ADR-0006)

- [x] Uncompressed partwise adapter, rational rhythm, voices/staves, sound ties/tempo.
- [x] Bounded compressed MXL support through META-INF/container.xml (ADR-0010).
- [x] Retain inherited key/time, written rhythm values/dots and tempo; initial clefs.
- [ ] Full clef/attribute changes, tuplets/beams and engraving detail.
- [x] Authored measure layout, clefs/rests, barlines/houses and repeat-visit mapping.
- [ ] Refined engraving collisions, ties/beam layout and full score pagination.
- [x] Bounded sequential repeats and numbered endings.
- [ ] Nested repeats/D.C./D.S./coda and dynamics/pedal/articulation playback.
- [ ] Broader exporter fixtures/interoperability.
