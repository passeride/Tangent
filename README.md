# Tangent

Piano practice, note reading and MIDI performance, built around **Bevy** with
**web as a first-class deployment target**.

Tangent is a new project with its own repository and history. The goal is an
independent implementation, developed through a documented clean room process,
without importing Neothesia or code from its forks.

## Project status

**First implementation slice: MIDI import.** A Rust workspace imports MIDI bytes
on native and WASM targets. It preserves tracks, note spans, tempo and metadata,
and includes a native inspection example plus a small browser import harness.

There is no Bevy application, sound engine, live MIDI connection, playback or
public web deployment yet. See the [support contract and commands](docs/midi-import.md).

The implementation is not certified as a completed clean room process. Prior
source exposure remains documented in [PROVENANCE.md](PROVENANCE.md).

## Product direction

- Connect a MIDI piano and show the notes being played.
- Practice reading notes in treble and bass clefs, and learn common chords.
- Play MIDI pieces with a virtual keyboard, falling notes and optional notation.
- Offer focused practice and a separate scored performance mode.
- Keep a personal music library, preferences and progress.
- Add MusicXML support with an explicit supported subset and synchronized notation.

These are product goals beyond the initial importer. Browser MIDI, audio and file
access must be validated on target devices; keyboard and on-screen input should
remain useful when hardware MIDI is unavailable.

## Technical direction

Bevy provides the application and visual layer. Musical data, exercise rules and
performance evaluation should be independently testable. Browser audio, MIDI,
file access and persistence need explicit platform boundaries.

Exact package versions, audio architecture, notation renderer and content formats
will be selected through small prototypes and recorded decisions. Start with a
working browser slice before building the complete interface.

## Documentation

| Area | Purpose |
| --- | --- |
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

The detailed implementation backlog will follow the initial design and feasibility
work. The byte-based MIDI importer is delivered; the remaining application milestones are open.

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
