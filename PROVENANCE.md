# Provenance and implementation boundaries

Updated: 2026-10-24.

## Current record

This repository was created as a new Tangent project. Its initial contents were
a heading-only README and a Rust-oriented `.gitignore`. This documentation
bootstrap was then prepared at the owner's request using AI assistance.

The planning conversation and assistant have prior exposure to the Neothesia
fork's source code. This fact is recorded explicitly: a new repository is not
proof that its contributors or their context are unexposed. The current work is
project documentation, not a completed source-independent implementation or a
certification of clean room status.

No implementation, third-party runtime dependency, music fixture, font,
SoundFont, shader or visual asset is introduced by this bootstrap. The documents
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
| Implementation code | Not started | Contributor/context, approved inputs and commit history |
| Requirements handoff | Not established | Version, sources, review and rights |
| Runtime dependencies | None | Package, version/hash, original source, license and notices |
| Visual/music/audio assets | None | Use the [design asset register](docs/design/assets/README.md) |
| Project license | Not selected | Explicit owner decision and applicable scope |

This is a factual project record and process direction, not a legal determination.
Do not remove or rewrite exposure history to make a stronger claim of independence.
