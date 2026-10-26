# Provenance and implementation boundaries

Updated: 2026-10-26.

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

## Local SQLite storage — 2026-10-25

Original preferences/schema/browser persistence code implements the owner’s
SQLite request. Primary sql.js documentation was consulted; sql.js 1.15.1 is
npm-locked, MIT, with its unchanged complete upstream license recorded. SQLite
is public domain. No excluded legacy source/assets were consulted. ADR-0009
records actual browser backing, schema and boundaries; prior exposure remains.

## Compressed MusicXML import — 2026-10-25

An original bounded MXL container reader was authored for Tangent using the public
MusicXML 4.0 ZIP-container reference. No excluded fork implementation or assets
were consulted. It reuses already locked and inventoried miniz_oxide 0.9.1 and
crc32fast 1.5.3, with no new registry packages or externally sourced fixtures.
Synthetic ZIP test inputs are generated within repository tests. ADR-0010 records
the container boundary, security limits and remaining unsupported ZIP features.

## Local music library — 2026-10-25

Original Bevy library UI, source-byte capability boundary and SQLite v3 migration
authored for Tangent. Existing SQLite and hashing libraries are reused. Source
files are user imports; no external music collection or excluded fork assets
were copied. ADR-0013 records storage limits and remaining performance work.

## Library completion — 2026-10-25

Original Unicode search, metadata editing, filter/sort, bounded batch import,
fingerprint ownership and Windows/P-45 test/report workflow. The official Bevy
v0.20.0 KeyboardInput source was consulted for text event semantics. Existing
dependencies and original test music are reused; no excluded fork source/assets
were consulted. ADR-0014 records the schema/capability changes.

## MIDI output and reading practice — 2026-10-25

Original MIDI output adapters, seek-state reconstruction, practice curricula and
Bevy feedback/glow presentation implement the owner's follow-up after a successful
P-45 test of the previous build. The earlier requirements were retrieved; no
excluded legacy source/assets were opened. Existing midir/Web MIDI, Bravura and
original music are reused, with no new dependency. ADR-0015 records boundaries;
physical output is not established by simulated-device tests.

## Hanabi and UI/practice integration — 2026-10-25

At the owner's request, native particles use independently published bevy_hanabi
at d277555d4a2350da6787245d15c56c62c739594d (0.20.0-dev, MIT OR Apache-2.0).
The upstream Cargo manifest and original LICENSE-MIT/LICENSE-APACHE2 were checked
in Cargo's source checkout; these notices and resolved transitive dependencies
are included by scripts/update_licenses.py. Bevy 0.20 compatibility is verified
by native compilation; WebGL2 uses independently authored bounded UI particles.
No upstream example art, legacy Neothesia material or external music was copied.
Practice pointer/reference behavior and UI refinements are newly authored for
Tangent. Earlier source exposure remains as recorded above.

## Scored sessions and review — 2026-10-26

Newly authored core score/review logic, Bevy history/results, native/browser SQLite
adapters and synthetic MIDI tests implement the owner's requested scores,
progress and further practice (ADR-0018). No legacy/fork code or external music,
art or samples were imported. Existing pinned BLAKE3 is now shared with WASM for
exact source-file score identity; its original notices remain included. The
bundled original CC0 study and Bravura/Fira font notices are preserved. Earlier
source exposure remains disclosed above.

## Playback speed — 2026-10-26

Original bounded tempo model, audio-clock mapping, Bevy pause controls and
synthetic MIDI/browser tests implement variable-speed song practice (ADR-0020).
Existing audio/MIDI libraries and the original CC0 study are reused. No new
dependencies, external samples or excluded legacy source/assets were imported.

## Passage practice — 2026-10-26

Owner authorized integration of the UX strategy and direct implementation on main.
New original Rust/JS implements source-time loops, explicit imported-part targets,
contextual pitch-coverage evidence and retry. Requirements are the strategy's
UX-02/03/04/05/07/11 and ADR-0021. New synthetic MIDI sequences exercise boundaries;
no competitor/fork code or assets were used. No dependency versions were added.
Prior exposure and the limits of the clean room claim above remain unchanged.

## Readability iteration — 2026-10-26

Original UI/core changes implement UX-12/14 and ADR-0022: HUD density, explicit
keyboard fitting and short-window passage controls. New screenshots use the
original bundled study or synthetic generated MIDI, never competitor assets.
No dependency or asset source was added.

## Authored passages and voluntary goals — 2026-10-26

New original core/UI behavior implements authored measure-visit snapping,
session-local repetition goals and compatible-result deltas, using the existing
MusicXML model and synthetic/original-study tests. Requirements: UX-02/11/13 and
ADR-0023. No dependency, external asset, competitor code or telemetry was added.

## Configurable input — 2026-10-26

Original shared action-binding preferences, MIDI modifier/prefix state machine,
Bevy capture/matrix UI and synthetic input regression tests implement the owner's
control-key request. Existing domain actions remain authoritative. Public upstream
Leafwing/Enhanced Input compatibility/API documentation was evaluated; neither is
added as a dependency. No excluded source, runtime asset or new package is imported.
ADR-0024 records the architecture, event-ordering and persistence contract.

## Personal appearance and demonstrated hand annotations — 2026-10-26

Original typed JSON theme tokens, safe Bevy colour editor, conservative one-hand
demonstration matching, reviewed source-specific annotations and library row
hit-testing implement the owner's follow-up. The shared model has no Bevy or
platform dependency; adapters only handle file bytes and existing SQLite settings.
No excluded project source or asset was consulted or imported. No package was
added; serde's existing rc feature supports immutable shared accepted profiles.
ADR-0025 records scope, persistence and explicit initial alignment limitations.

## Internationalized UI text — 2026-10-26

The unmodified, independently licensed Noto Sans Regular font is now embedded
in the shared Bevy renderer for UI text, replacing use of the limited default
Fira Mono subset. Bravura remains the notation font. Noto Sans is distributed
under SIL OFL 1.1; pinned source, content hash, copyright and license are in
[the design asset register](docs/design/assets/README.md). No legacy project
assets or implementation were copied.

## Procedural 3D piano — 2026-10-26

The chamfered key mesh, case, lighting and light-spill geometry in `scene/piano3d.rs` and `scene/draw.rs` are original project code. Bevy 0.20.0 PBR/light features use the already pinned MIT/Apache-2.0 engine. Official Bevy render-to-texture and orthographic examples/API were consulted; no external piano model, texture or shader was imported. Prior source-exposure limitations documented above remain unchanged.

## Keyboard Explorer — 2026-10-26

N02 octave-transfer generation, evidence, UI and tests are original project code based on the owner-requested internal exercise catalogue. No external exercise files, music, graphical assets or implementation sources were imported.

## Five Finger Flow — 2026-10-26

The release-gated sequence runner, original major pentascale patterns, fingering guidance and T01 UI/tests are project-authored from the internal catalogue. No exercise files, recordings, external model assets or implementation sources were imported.
