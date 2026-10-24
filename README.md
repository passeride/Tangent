# Tangent

Piano practice, note reading and MIDI performance, built around **Bevy** with
**web as a first-class deployment target**.

Tangent is a new project with its own repository and history. The goal is an
independent implementation, developed through a documented clean room process,
without importing Neothesia or code from its forks.


[Play in the browser](https://passeride.github.io/Tangent/) ·
[Download platform builds](https://github.com/passeride/Tangent/actions/workflows/check.yml) ·
[Run/build guide](docs/distribution.md)

The browser link becomes available after the first successful Pages deployment.
Click **Try Bach**, then **Play**, to test the included Prelude BWV 924a.
MIDI typesetting by **jeff covey / Mutopia**, **CC BY-SA 3.0**;
[source, attribution and license](web/piano/samples/ATTRIBUTION.md).

## Screenshots

Actual running app, captured with Chromium at 1440 px width using Tangent's
original demonstration study. These are app screenshots, not design mockups.

![Piano with falling blocks](docs/screenshots/piano-blocks.png)

| Staff lanes with ledger lines | Falling noteheads |
| --- | --- |
| ![Staff lanes](docs/screenshots/piano-staff.png) | ![Falling notes](docs/screenshots/piano-notes.png) |

![Stationary sheet reading view with a moving playhead](docs/screenshots/piano-score.png)

Reproduce with `node web/piano/capture.mjs` after building the WASM package.

## Platform builds

Every push to main builds the web player and downloadable Windows x64/ARM64,
Linux x64/ARM64 and macOS Intel/Apple Silicon packages. Desktop packages contain
a self-contained launcher that serves the same player on localhost and opens
your browser. They do not require Rust, Node or Python installed. See the
[distribution guide](docs/distribution.md) for downloads, starting/stopping,
checksums and unsigned-build limitations. Android/iOS use the web version.

## Project status

**Playable web piano and MIDI playback.** The Rust workspace has a format-neutral
music/piano model, one Bevy application with in-engine UI/wgpu rendering and a Web Audio adapter.
Play with keyboard, pointer or touch; import MIDI, switch between four note
views, choose a theme/piano profile and enable or disable sparkles and song audio.

See [piano build/run/test instructions](docs/piano-player.md) and the
[MIDI support contract](docs/midi-import.md). Hardware Web MIDI is optional and
has not been verified with a physical device. Desktop launchers package this
browser player. All player controls and visualizations render inside the Bevy GPU
canvas.
Native OS windows/audio, persistent library and MusicXML import remain future work.

The implementation is not certified as a completed clean room process. Prior
source exposure remains documented in [PROVENANCE.md](PROVENANCE.md).

## Product direction

- Connect a MIDI piano and show the notes being played.
- Practice reading notes in treble and bass clefs, and learn common chords.
- Play MIDI pieces with a virtual keyboard, falling notes and optional notation.
- Offer focused practice and a separate scored performance mode.
- Keep a personal music library, preferences and progress.
- Add MusicXML support with an explicit supported subset and synchronized notation.

These are product goals; the current piano slice implements a subset. Browser MIDI, audio and file
access must be validated on target devices; keyboard and on-screen input should
remain useful when hardware MIDI is unavailable.

## Technical direction

Bevy owns the application, UI, keyboard/pointer input and GPU drawing in one
full-page canvas. The browser adapter supplies audio, file/device capabilities
and fullscreen, with no separate HTML application menus. Musical data, exercise rules and
performance evaluation should be independently testable. Browser audio, MIDI,
file access and persistence need explicit platform boundaries.

The sound/clock boundaries are recorded in ADR-0003; ADR-0005 replaces its
initial Canvas/HTML presentation with a single Bevy GPU surface. Full
notation layout, sampled sound and further formats require separate decisions.

## Documentation

| Area | Purpose |
| --- | --- |
| [Piano player](docs/piano-player.md) | Playable web slice and run/test commands |
| [Follow-up work](docs/TODO.md) | Remaining implementation and architecture work |
| [MIDI import](docs/midi-import.md) | Implemented scope, limitations and build/test commands |
| [Documentation index](docs/README.md) | Entry point for architecture and design work |
| [Architecture decisions](docs/adr/README.md) | Decision history, statuses and ADR template |
| [Product and visual design](docs/design/README.md) | Design questions, briefs and sketch index |
| [Sketches](docs/design/sketches/README.md) | Wireframes, flows and visual explorations |
| [Design assets](docs/design/assets/README.md) | Source files and asset provenance |
| [Provenance](PROVENANCE.md) | Source boundaries, exposure and future dependency records |
| [Agent guidance](AGENTS.md) | Instructions for automated contributors |

## First milestones

1. Document the requirements and implementation provenance.
2. Prove browser rendering, piano input, sound activation and local persistence.
3. Build a small MIDI playback and virtual-piano slice.
4. Expand into practice, library and scored performance after those foundations work.

The importer and initial web piano/playback slice are delivered. Persistence,
full notation and product modes remain open in [the backlog](docs/TODO.md).

## Contributing

Read the relevant design brief and ADR before changing a cross-cutting decision.
Use the templates to explain the problem, alternatives and evidence. Keep proposals
clearly separate from accepted decisions and delivered behavior.

Do not copy implementation code, wrappers, fixtures, shaders or assets from
Neothesia or its forks. Independently published dependencies and assets need their
own license and provenance record. Do not treat a fresh repository or AI context
as proof of source independence.

## Licensing

A project-wide license has not been selected. Do not assume that this repository
is MIT, Apache-2.0 or licensed under the terms of another Tangent project.
Dependencies retain their own licenses: see [THIRD_PARTY.md](THIRD_PARTY.md) and
[license notices](THIRD_PARTY_NOTICES.txt). Fonts, music and audio assets will
require their own records when introduced.

## Player shortcuts

| Action | Shortcut |
| --- | --- |
| Play / pause | Space |
| Seek backward / forward | Left / Right (5s), Shift for 15s |
| Volume down / up | Down / Up (5%) |
| Beginning / end | Home / End |
| Fullscreen | F11 or the in-engine Full button |
| Settings and all shortcuts | F1; Esc closes |
| View / theme / piano / range | V / B / X / R |
| Song audio / particles | M / P |
| Import / Bach sample / MIDI | Ctrl+O / Ctrl+B / Ctrl+M |

Staff notes and staff blocks both include five-line bass and treble staves,
with ledger strokes outside them. All menus are inside the GPU canvas.
