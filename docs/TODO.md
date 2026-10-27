# Follow-up work

Keep implemented behavior separate from planned work. Decisions belong in ADRs;
this file tracks the remaining work, not an authorization gate.

For user-facing sequencing, see the **proposed** [UX roadmap](design/ux-roadmap.md), [user stories](design/user-stories-and-journeys.md) and [product north star](design/product-north-star.md). The prioritized practice-loop bet is passage looping, honest hand/part isolation and action-oriented results. Initial implementation now includes source-time loops, explicit parts, authored measure snapping, contextual retry, HUD presets, keyboard fitting and session-local goals (ADR-0021–0023). The remaining milestones are proposals; see the execution-plan delivery log.

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
- [x] Add a local SQLite symbolic fingerprint index and FreeForm MIDI identification (ADR-0011).
- [x] Persist original music sources and add a paged Bevy Library view (ADR-0013).
- [x] Library title/composer search, metadata editing, sorting/filtering, batch import and source/fingerprint cleanup (ADR-0014).
- [x] Meaningful score/session recording, results, recent history and adaptive review (ADR-0018).
- Add library folders/tags and backup restore.
- Move source parsing and slow storage work off rendering/audio scheduling paths after profiling.
- [x] Add a true native renderer, audio mixer, MIDI inputs and SQLite adapter
  without browser or localhost server (ADR-0012). Validate release binaries on
  physical Windows/macOS/Linux machines and profile voice/audio latency.
- Add OS-integrated drag/drop and robust cross-platform native file picker
  (currently PowerShell, AppleScript and optional Zenity).
- Extend fingerprint search with polyphonic voices, multiple arrangements per work, calibrated match confidence and large-library performance.
- [x] Native Exit UI through managed Bevy shutdown (ADR-0012).
- Add signing and macOS notarization.

- [x] Explicit MIDI output selection, no hardware echo, cancellation and seek-state reconstruction (ADR-0015). Verify output with physical P-45.
- [x] Independent played solid fill and expected edge glow, including when particles are off.

## Reading views

- Expand the initial MusicXML subset (see below). MIDI must continue to work
  without invented spelling.
- Add a notation renderer/layout boundary for rests, key/time signatures, beams,
  tuplets, ties across pages, stem direction, collisions and accidental context.
- Improve narrow-screen score density and auto-follow paging. MusicXML uses authored measures; refine its layout and the approximate MIDI
  fallback independently.
- Evaluate the rotated-staff learning projection with users; keyboard spacing
  differs from conventional equal staff steps, and dense chords need layout work.
- [x] Explicit Fit keys for 49/61/88-key ranges and off-screen pitch counts (ADR-0022). Arbitrary zoom and physical display profiling remain open.

## Product layers

- Main/Train menus, FreeForm and basic note/chord matching are delivered in
  ADR-0008. [x] Extend the fixed C-major exercises with bass-clef targets, curriculum
  options and feedback (delivered in ADR-0015). Scored sessions, progress persistence and adaptive exercise
  selection are delivered in ADR-0018. Calibrated rhythmic scoring and broader
  curriculum goals remain future work.
- [x] Persistent reduced-motion preference and effect intensity (ADR-0017).
- [x] Optional practice reference audio and exact live pitch pointers (ADR-0017).
- Add contrast checks and accessible Bevy UI semantics for
  note/event information. Effects can already be disabled manually.

## MusicXML follow-up (ADR-0006)

- [x] Uncompressed partwise adapter, rational rhythm, voices/staves, sound ties/tempo.
- [x] Bounded compressed MXL support through META-INF/container.xml (ADR-0010).
- [x] Retain inherited key/time, written rhythm values/dots and tempo; initial clefs.
- [x] Initial written/metric beam groups and shared chord stems ([ADR-0033](adr/0033-written-beam-groups.md)).
- [ ] Full clef/attribute changes, tuplet brackets, cross-staff/rest-spanning beams and engraving detail.
- [x] Authored measure layout, clefs/rests, barlines/houses and repeat-visit mapping.
- [ ] Refined engraving collisions, ties/beam layout and full score pagination.
- [x] Bounded sequential repeats and numbered endings.
- [ ] Nested repeats/D.C./D.S./coda and dynamics/pedal/articulation playback.
- [ ] Broader exporter fixtures/interoperability.

## Next practice slices after ADR-0021–0023

- Retain per-performance staff/voice identifiers from MusicXML across repeated
  visits, then expose explicit staff/voice target selection; never infer MIDI hands.
- [x] Add guided wait-for-note with shared clock/output gating and chord release rules ([ADR-0029](adr/0029-sequential-play.md)).
- Resolve exact saved sources and restore practice context safely paused after restart.
- Add persistent goals and richer compatible passage trends without inventing legacy tempo.
- Validate P-45 loop boundaries, output switching, sustained notes and native GPU layouts.
- Implement validated backup restore before promising portable recovery.

## Next new Train exercises

The catalogue is a specification, not a blanket completion list. Existing Read
notes / Play chords remain the foundations; new exercise deliveries are explicit.

- [x] N02 Keyboard Explorer: marked reference key, exact ±octave transfers,
  three levels, hints/Hear, scored evidence and persisted retry (ADR-0027).
  Simulated MIDI/browser validated; physical P-45 validation remains open.
- [x] T01 Five Finger Flow: reusable ordered sequence runner, three positions/
  register levels, advisory fingers, release-gated input, whole-pattern scores,
  hint/Hear assistance and durable evidence/retry (ADR-0028).
  Legato, both hands together and calibrated timing remain future work.
- [x] S03 Major Scale Run: one-octave major scales in all 12 keys, explicit
  spelling, advisory fingers, exact ordered input and whole-scale history/retry
  ([ADR-0030](adr/0030-major-scale-run.md)). Physical P-45 validation remains open.

See [the 83-exercise catalogue](piano-exercise-catalog.md) for later candidates.
