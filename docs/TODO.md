# Follow-up work

Keep implemented behavior separate from planned work. Decisions belong in ADRs;
this file tracks the remaining work, not an authorization gate.

## Piano/playback foundation

- Profile the delivered Bevy UI/wgpu surface on real desktop/mobile GPUs.
  WebGL2 is the current backend; evaluate WebGPU separately when justified.
- Move audio lookahead off the rendering/main thread where profiling justifies it;
  add output/display latency calibration and stress cases with dense piano files.
- Extract controller/voice-state reconstruction into a shared, testable reducer
  when native audio is added; currently owned by the browser sound adapter.
- Improve voice stealing and profile/sample loading behind the audio boundary;
  evaluate a piano synthesizer/SoundFont with independently documented licenses.
- Verify actual MIDI hardware, device disconnects, sustain input and MIDI latency;
  add port selection and browser capability coverage. No SysEx requirement.
- Index long sustained notes and visible windows; frame JSON and prefix scanning
  are prototype boundaries and need dense/long-song profiling.
- Save user settings and imported songs through a browser storage adapter.
- Add true native input, audio and renderer adapters. Desktop downloads now use
  browser-backed launchers; they do not implement a native Bevy/audio view.
- Add managed desktop lifecycle/quit UI, signing and macOS notarization.

## Reading views

- Import MusicXML into authored score/measure/voice/spelling models, linking written
  notes to performance IDs. MIDI must continue to work without invented spelling.
- Add a notation renderer/layout boundary for rests, key/time signatures, beams,
  tuplets, ties across pages, stem direction, collisions and accidental context.
- Improve narrow-screen score density and auto-follow paging. The current sheet
  view uses time spacing and eight-second pages, not engraved measures.
- Evaluate the rotated-staff learning projection with users; keyboard spacing
  differs from conventional equal staff steps, and dense chords need layout work.
- Add key-range auto-fit/zoom and a clear indicator for off-screen pitches.

## Product layers

- Practice matching and scoring consume performed notes; they do not belong in
  renderers or the import parser. Add their own requirements/ADRs before rules.
- Add full reduced-motion preferences, contrast checks and accessible Bevy UI semantics for
  note/event information. Effects can already be disabled manually.
