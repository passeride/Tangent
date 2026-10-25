# Provenance and implementation boundaries

Updated: 2026-10-24.

## Current record

This repository was created as a new Tangent project. Its initial contents were
a heading-only README and a Rust-oriented `.gitignore`. This documentation
bootstrap was then prepared at the owner's request using AI assistance.

The planning conversation and assistant have prior exposure to the Neothesia
fork's source code. This fact is recorded explicitly: a new repository is not
proof that its contributors or their context are unexposed. The owner subsequently authorized MIDI implementation directly on main. This
work proceeds with the same disclosed exposure; it is not represented as a
completed source-independent handoff or certification of clean room status.

The documentation bootstrap introduced no implementation or runtime assets.
The subsequent MIDI slice adds newly authored Rust/JS/HTML, synthetic test-event
sequences and independent dependencies recorded in THIRD_PARTY.md. No font,
SoundFont, shader or prior project asset is introduced. The documents
were written for this repository; the old fork's source files, tests, assets and
ADRs have not been transplanted here. Exposure still needs consideration when
preparing any implementation handoff.

## Before implementation

Establish a documented, implementation-neutral requirements package and identify
which material and context an implementation contributor can access. Record
requirements origins, actual prior exposure, independent primary references and
rights to any handed-over documents. Do not assume another AI session establishes
independence by itself.

Requirements can describe desired behavior without including excluded code,
internal structures or source-derived implementation recipes. Create new test
cases from those requirements and public standards. Maintain the record when
new material is introduced.

## Ongoing records

| Category | Current state | Record when introduced |
| --- | --- | --- |
| Implementation code | Initial MIDI import, authorized 2026-10-24; same disclosed context | Contributor/context, approved inputs and commit history |
| Requirements handoff | MIDI requirements recorded in docs/spec/midi-import.md; no independent clean room review claimed | Version, sources, review and rights |
| Runtime dependencies | See THIRD_PARTY.md and lockfiles | Package, version/hash, original source, license and notices |
| Visual/music/audio assets | No external assets; original synthetic MIDI test data | Use the [design asset register](docs/design/assets/README.md) |
| Project license | Not selected | Explicit owner decision and applicable scope |

This is a factual project record and process direction, not a legal determination.
Do not remove or rewrite exposure history to make a stronger claim of independence.

## MIDI implementation record — 2026-10-24

Authorized task: start implementing MIDI import in this new repository directly
on main. The implementation requirements are recorded in
[docs/spec/midi-import.md](docs/spec/midi-import.md). Their references are MIDI
Association documentation and midly's independent package/API documentation.
No Neothesia/fork source was opened or imported for this task; the assistant's
earlier source exposure remains disclosed above. This statement does not assert
that exposure has been eliminated.

The importer, its model, tests, CLI example, WASM boundary and inspection page
were authored for this repository with AI assistance. Synthetic fixtures are
generated from new event sequences. Third-party dependencies are obtained from
original registries; Cargo.lock was generated here, not copied from the fork.

## Piano implementation record — 2026-10-24

The owner requested a playable piano, MIDI playback, four note presentations,
themes and effects, and supplied a hand-drawn reference photo. That requirements
sketch informs the new [design brief](docs/design/piano-player.md); the photo is
not bundled. The implementation adds original domain/application Rust modules,
Canvas/SVG geometry, Web Audio synthesis, original study events and browser tests.
No legacy source/assets were consulted during this task; the disclosed prior
context exposure remains unchanged. New Bevy dependencies come from crates.io,
with their manifest licenses and license files recorded in THIRD_PARTY files.
No external music font, sampled piano, recording or SoundFont is introduced.

## Distribution and sample record — 2026-10-24

The owner authorized screenshots, platform builds, GitHub Pages hosting and a CC
MIDI sample. New standard-library Rust launcher, build/packaging scripts and CI
were authored in this repository without consulting legacy source. No new runtime
registry dependency is added. The external MIDI is independently obtained from
Mutopia: Bach Prelude BWV 924a, typeset by jeff covey, CC BY-SA 3.0. Original bytes,
hash, source, credit and full license are in `web/piano/samples`. This music license
does not select a project-wide source-code license. README screenshots are actual
app captures with the original study, not assets copied from another project.

## Single Bevy surface — 2026-10-24

The owner clarified that the whole application, controls and note views must be
one Bevy GPU surface. New Rust scene/layout/control/theme modules replace the
Canvas/HTML renderer; the JS adapter now only supplies browser capabilities.
Independent Bevy 0.20 registry source/API and official examples were consulted;
no excluded legacy source/assets were consulted. Dependencies/notices were
regenerated; Bevy's embedded Fira Mono subset adds a separately recorded SIL OFL
font attribution/license. Screenshots are actual new-app captures.

## MusicXML and pause menu — 2026-10-25

Original MusicXML adapter and synthetic XML fixtures were authored for Tangent
using W3C MusicXML 4.0 as the public reference, without consulting excluded legacy
source. roxmltree 0.20.0 comes from crates.io (MIT OR Apache-2.0). The pause gear
uses original Bevy geometry, no imported icon/font asset. Prior exposure above
remains disclosed. ADR-0006 records the delivered subset and remaining work.

## Authored measures and clefs — 2026-10-25

The owner requests actual G/F clefs, watermarks, a MusicXML demo and visible
measures/endings. New model/import/presentation code and an original piano study
were authored using W3C MusicXML 4.0/SMuFL references; no excluded legacy source was
consulted. The full unmodified Bravura font comes from Steinberg's independent
repository at the pinned commit in the asset register, SIL OFL 1.1. It is embedded
in Bevy. First light MusicXML is original synthetic demonstration music (CC0 1.0);
it replaces, rather than transcribes, the Mutopia MIDI. The retired MIDI credit
record remains in docs/retired-midi-sample.md. ADR-0007 records repeat/score mapping.

## Navigation and practice — 2026-10-25

Original Bevy navigation, piano-key wordmark geometry and independent core
exercise rules implement the owner’s main-menu/Train request. No legacy source
was consulted or imported, and no new dependency or external asset was added.
The disclosed prior exposure remains unchanged; see ADR-0008.
