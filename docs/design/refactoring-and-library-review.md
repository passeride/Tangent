# Refactoring and established-library review

- Status: Proposed future work; no replacement library selected or integrated
- Created: 2026-10-28
- Evidence baseline: main at `8bdb8bc`, documentation/source review only
- Related: [ADR-0037](../adr/0037-notation-and-ui-refactoring.md),
  [4K and library layout brief](display-scaling-and-library-layout.md)

## Why this work exists

The owner reports incorrect-looking notation, white beams beside hand-coloured
notes, small controls/text on a 4K display and a library showing about five songs
in roughly 20% of the screen. The goal is to reuse established layout/rendering
capabilities rather than accumulate musical engraving rules and widget behaviour
inside Tangent. These are reported problems, not new claims of measured hardware
performance. Existing music, progress, annotations and playback must survive.

This is an initial suitability assessment, not a benchmark or dependency approval.
Upstream feature descriptions are documented capabilities, not verified Tangent
compatibility. No third-party implementation/assets have been copied.

## Current ownership and evidence

| Area | Existing implementation | Consequence / opportunity |
| --- | --- | --- |
| Score | `tangent-core::notation` / `beaming`, MusicXML importer, `scene/score.rs` and Bravura in `scene/draw.rs` | Symbols exist, but Tangent still owns spacing, stems, beams and collision handling. A font is not a complete engraver. |
| Forms and library | Absolute-positioned Bevy UI primitives/text; custom hit rectangles and text editing | Repeated fixed sizes and hand-built widgets make scaling, selection, scrolling, IME and focus expensive to maintain. |
| Library extent | `scene/library.rs`: `PAGE_SIZE = 6`, viewport-derived size clamped to that maximum | Six 60-unit desktop rows occupy only 360 logical units even on a tall window. The reported five rows depend on viewport/content; six is the source-code cap. |
| Library storage | Native `platform/native/library.rs` and browser `library-schema.js` cap metadata requests at 50 | Display row count and storage fetch size are separate concerns. Keep bounded queries while filling the viewport. |
| Audio | Native harmonic instrument through rodio; browser sound adapter; independent MIDI output | A sampled instrument may improve timbre without replacing transport or scoring. |
| File capabilities | PowerShell / AppleScript / Zenity dialogs; bounded startup/manual folder worker | A dialog library could replace platform scripts. Live watching is a separate optional capability. |

The existing dependencies already solve significant problems: Bevy/wgpu rendering,
Taffy-backed Bevy layout, midly MIDI parsing, midir device access, rodio audio output,
SQLite storage, serde preferences, image decoding and native Hanabi effects. Reuse
these before adding overlapping frameworks. Inspect the lockfile when choosing
an exact version; being transitive does not automatically make an API suitable.

## Notation candidates

| Candidate | Documented capability | Initial fit and unresolved costs |
| --- | --- | --- |
| [Staveloom core 0.1.3](https://docs.rs/crate/staveloom-core/0.1.3) and [WASM binding](https://docs.rs/crate/staveloom-wasm/0.1.3) | Rust MusicXML/MXL and MIDI parsing, SVG layout, auto-beaming and playback metadata; desktop library and browser host | First Rust feasibility candidate. Young 0.1.x API; verify dense piano, cross-staff notation, authored beams, element identity and importer limits. MIT OR Apache-2.0 for code; font/asset licenses remain separate. |
| [Verovio](https://github.com/rism-digital/verovio), [verovio-rs](https://github.com/ro-ag/verovio-rs), [Verovioxide](https://github.com/music-comp/verovioxide) | C++ engraving, MusicXML import, SVG; Rust bindings and official browser WASM toolkit | Main comparison candidate. Native C++ toolchain, binding maintenance, WASM adapter, distribution size and LGPL obligations need evaluation. Desktop Rust bindings are not evidence of direct `wasm32-unknown-unknown` compatibility. |
| [OpenSheetMusicDisplay](https://github.com/opensheetmusicdisplay/opensheetmusicdisplay) | TypeScript MusicXML renderer over VexFlow, SVG/PNG, configurable layout and colours | Useful visual reference and web candidate; native JS runtime/bridge and Bevy composition increase scope. Some MusicXML features remain unsupported. BSD-3-Clause code. |
| [VexFlow](https://github.com/vexflow/vexflow) | TypeScript music notation primitives/layout, SVG/Canvas | Lower-level option. OSMD already provides its MusicXML translation; using VexFlow alone leaves more integration/layout ownership with us. MIT. |
| [GUIDO](https://github.com/grame-cncm/guidolib) | Embeddable cross-platform score engine with its own GMN representation | Viable alternative if the first pair fails; needs an extra notation conversion boundary and mapping validation. MPL-2.0. |
| [LilyPond](https://lilypond.org/doc/v2.24/Documentation/usage/command_002dline-usage), [webmscore](https://github.com/LibreScore/webmscore), [abcjs](https://github.com/paulrosen/abcjs) | Respectively command-line engraving, MuseScore core in WASM, and ABC rendering | Useful comparison/export tools. Not the initial integration choice: different input/runtime/distribution tradeoffs. Inspect exact component/asset licensing before adoption. |

[Staff](https://docs.rs/staff/latest/staff/render/index.html) has a rendering module,
but its published API does not establish a complete replacement for our MusicXML
score layout. [rslily](https://docs.rs/crate/rslily/0.1.4) explicitly says engraving
is not implemented. Do not select projects on name/README adjectives alone.

### Proposed boundary

Tangent keeps source identity, original bytes, hand annotations, score/progress
rules and its authoritative transport. An engraving adapter consumes an immutable
notation source plus viewport/theme settings, returning per-system graphics and
an element/measure map. Keep that adapter outside `tangent-core`; engine-specific
models and SVG types must not leak into exercise/scoring rules.

Do not adopt an engine's regenerated MIDI as the playback truth during a visual
refactor. Align written element IDs to performance visits, repeats and tied
attacks explicitly; unknown/ambiguous matches must remain visible as such.
MusicXML and approximate MIDI-to-score inference need separate acceptance cases.
MIDI cannot recover the composer's intended spelling, voices or beams reliably.

SVG generation is not Bevy rendering. Compare bounded cached rasterization via
[resvg/usvg](https://github.com/linebender/resvg) against a vector/mesh adapter.
Raster caches need resolution/theme/layout keys, memory budgets and regeneration
at effective display density; vector adapters need correct paths, fonts, clips
and transforms. Neither is a reason to write an SVG renderer ourselves. Keep all
visible controls and notation in the existing Bevy window/canvas (ADR-0005).
Browser engines would require an explicit capability-boundary amendment, not an
unannounced DOM score overlay.

Use stable note IDs for coloured overlays and played feedback. Beams/stems must
follow hand/theme ownership, including a documented policy for groups spanning
hands or staves. SVG CSS support does not automatically implement that policy;
rasterized textures also need recolouring/cache invalidation or a separate overlay.

### Small comparison experiment

Use the same independently sourced scores for both engines: the bundled original
study, existing synthetic beam fixture, and owner-supplied files kept out of the
repository unless redistribution is permitted. Include dense piano chords,
seconds, multiple voices, changing clefs, tuplets, ties, cross-staff beams,
explicit beam overrides, endings/repeats and a tempo change within a note.

Record native and browser results, import warnings/loss, visual discrepancies,
source-to-element mapping, cold render/relayout time, memory, WASM/archive growth
and license inventory. Compare actual images against a reference score, not just
successful compilation. Test hand colours, live-input overlays, zoom and passage
repeats. Pin versions/commits for reproducibility. No winner until the results
exist; retain the current renderer as a comparison/fallback during the experiment.

## Other reuse opportunities: rough assessment

Benefit and effort are relative estimates, not measured delivery promises.

| Area | Candidate / existing capability | Expected benefit | Effort / recommendation |
| --- | --- | --- | --- |
| UI layout and 4K | Bevy `UiScale`, Flexbox/Grid through existing Taffy; shared spacing/type tokens | High: legibility and viewport use without a framework switch | Low–medium. First choice for scaling/layout fixes; rendering and manual hit tests must share one coordinate transform. |
| Library/forms | `bevy_egui` 0.43 family for Bevy 0.20, egui `ScrollArea::show_rows`, `egui_extras::TableBuilder` | High: scrolling tables, selection, editable fields, focus and clipboard | Medium–high. Prototype Library + one edit form beside native Bevy widgets; decide from theme fidelity, accessibility, touch/IME, WebGL2 and input capture tests. No wholesale UI rewrite yet. |
| SVG composition | resvg/usvg | High if an SVG engraver is selected | Medium. Benchmark cache/upload latency and 4K sharpness; this draws SVG, not music layout. |
| OS file dialogs | `rfd` async file/folder APIs | Medium: remove three platform scripts, improve parent-window integration and multi-select | Low–medium. Native-first experiment; check Linux portal/GTK dependencies and event-thread rules. Browser directory permissions/persistent handles remain separate. |
| Piano timbre | RustySynth with a separately licensed SoundFont | Medium–high: a sampled instrument instead of harmonic voices | Medium. Keep rodio/output scheduler; compare latency, sustain, voice limits, CPU and WASM cost. SoundFont quality/license/download size matter independently of library code. |
| Live folder updates | notify + debounce layer | Medium when live updates are wanted | Medium; later. Native adapter only, with bounded queue and reconciliation scans. Network drives can miss events; preserve manual scanning and browser permission behaviour. |
| Accessibility/text | Existing Bevy/AccessKit support; standard text widgets instead of custom key-string editing | High usability value | Cross-cutting. Semantics, focus, keyboard navigation and IME require explicit wiring/testing; merely adding egui or AccessKit does not make the app accessible. |

Do not replace SQLite, midly, midir, rodio or wgpu just to shorten our code. Do not
adopt a theory library for exercise rules without checking spelling/voicing and
scoring semantics. Keep domain rules independently tested; avoid moving them into
widget callbacks or a renderer's playback engine.

## Suggested sequence and exit evidence

1. **UI-1: Display metrics and library extent.** Implement the
   [4K/library brief](display-scaling-and-library-layout.md) using existing Bevy
   capabilities first. No engraving migration prerequisite.
2. **UI-2: One-screen widget comparison.** Compare native Bevy layout/widgets with
   egui Library/editing. Verify themes and keyboard/MIDI command capture; choose
   one owner for each event and form. Publish actual captures and costs.
3. **N-1: Engraver comparison.** Run Staveloom versus Verovio through the bounded
   adapter; select/reject with evidence, then amend ADR-0037 or create a decision.
4. **N-2: Gradual migration.** Move authored score view first, preserve source/time
   mapping and progress; evaluate falling/live notation separately. Record any
   MusicXML support changes rather than silently broadening the playback subset.
5. **C-1: Capabilities and sound.** Evaluate rfd, then optional sampled piano and
   live watching independently. No coupled dependency bundle.

## Primary sources reviewed 2026-10-28

Notation repositories/docs are linked in the comparison above. Additional sources:

- [Bevy UI Flexbox/Grid](https://docs.rs/bevy/latest/bevy/ui/index.html)
- [Bevy UiScale](https://docs.rs/bevy/latest/bevy/prelude/struct.UiScale.html)
- [Bevy window logical/physical sizes](https://docs.rs/bevy/latest/bevy/prelude/struct.Window.html)
- [Taffy](https://github.com/DioxusLabs/taffy)
- [bevy_egui compatibility/features](https://github.com/vladbat00/bevy_egui)
- [egui visible-row scrolling](https://docs.rs/egui/latest/egui/containers/scroll_area/struct.ScrollArea.html)
- [egui_extras table](https://docs.rs/egui_extras/latest/egui_extras/struct.TableBuilder.html)
- [rfd platform support](https://github.com/PolyMeilex/rfd) and [async API](https://docs.rs/rfd/latest/rfd/struct.AsyncFileDialog.html)
- [RustySynth](https://github.com/sinshu/rustysynth)
- [notify](https://github.com/notify-rs/notify)
- [Verovio web build](https://book.verovio.org/installing-or-building-from-sources/javascript-and-webassembly.html)
- [Verovio SVG styling/identity](https://book.verovio.org/interactive-notation/css-and-svg.html)

Version compatibility and all code/asset notices must be checked again at the
implementation date. Only documentation and this repo's implementation were
reviewed; no external engine was built, benchmarked or integrated for this study.
