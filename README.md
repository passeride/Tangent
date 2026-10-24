# Tangent

Piano practice, note reading and MIDI performance, built around **Bevy** with
**web as a first-class deployment target**.

Tangent is a new project with its own repository and history. The goal is an
independent implementation, developed through a documented clean room process,
without importing Neothesia or code from its forks.

## Project status

**Playable web piano and MIDI playback.** The Rust workspace has a format-neutral
music/piano model, a Bevy application core and browser Canvas/Web Audio adapters.
Play with keyboard, pointer or touch; import MIDI, switch between four note
views, choose a theme/piano profile and enable or disable sparkles and song audio.

See [piano build/run/test instructions](docs/piano-player.md) and the
[MIDI support contract](docs/midi-import.md). Hardware Web MIDI is optional and
has not been verified with a physical device. Full Bevy GPU rendering, native UI/
audio, persistent library and MusicXML import are not implemented yet.

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

Bevy provides the application core; the first web renderer is an explicit Canvas adapter. Musical data, exercise rules and
performance evaluation should be independently testable. Browser audio, MIDI,
file access and persistence need explicit platform boundaries.

The initial sound/clock/rendering boundaries are recorded in ADR-0003. Full
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
