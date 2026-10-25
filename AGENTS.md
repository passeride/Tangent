# Contributor guidance

Tangent is a new Bevy piano-practice project targeting web from the first
application slice. Implemented slices include native/WASM MIDI import, a format-neutral musical core,
one Bevy UI/wgpu application and a browser audio/file/MIDI capability adapter.
Desktop launcher packaging and Pages deployment are described in
`docs/distribution.md` and ADR-0004. Keep the distinction between a browser-backed
launcher and native rendering/audio explicit.
Read `docs/piano-player.md` and ADR-0005 for the actual delivered boundaries;
native windows/audio remain future work; see ADR-0006 for the MusicXML subset. Read ADR-0005:
all application controls and visuals must remain inside the single Bevy canvas.

## Read first

Read `README.md`, `PROVENANCE.md` and the relevant design brief/ADR before work.
Use `docs/adr/README.md` and `docs/design/README.md` as indexes. Accepted direction
and delivered behavior are separate statuses.

## Scope and provenance

Do not import Neothesia/fork implementation files, wrappers, tests, fixtures,
shaders or assets. Independently published libraries need their own provenance,
license and compatibility check. Do not copy another repository's lockfile or
relicense material by changing its header.

The initial planning context has prior fork-source exposure. Keep that record
honest. A clean room implementation handoff must define approved requirements and
references; a new agent/session does not itself prove source independence.
These boundaries do not prevent the documentation and design preparation the
owner has requested. Do not invent additional approval gates for ordinary work.

## Architecture comes first

Always prioritize a clean, understandable architecture over shortcuts that mix
unrelated responsibilities. Keep changes proportional to the task; this is not
a reason to rewrite working systems or introduce speculative frameworks.

- Segment code into cohesive modules where responsibilities naturally differ:
  musical models, import/validation, timing, practice/scoring, platform adapters,
  storage and presentation. Give each module a clear purpose and a small public
  interface. Do not create a crate or abstraction for every small helper.
- Keep dependency direction explicit. Musical behavior must be testable without
  Bevy, a browser, files or connected devices. UI and platform adapters consume
  domain behavior; they must not become the home of import or scoring rules.
- Keep browser/native differences at integration boundaries. Avoid duplicating
  musical rules across targets or allowing platform types into shared models.
- When adding a feature, identify its owner module first. Split existing code
  when it has accumulated distinct responsibilities, rather than extending a
  catch-all module. Explain non-obvious boundaries and dependencies in nearby
  documentation.
- Verify boundaries through focused behavior tests. Refactoring must preserve
  existing musical semantics unless a behavior change is explicitly intended.
  Keep unresolved architectural debt visible in a task or design document.

## Decisions and design

Create ADRs from the template, number them sequentially and add them to the index.
Record actual decisions; keep unresolved implementation choices proposed. A new
ADR supersedes an old decision with links in both directions.

Record meaningful architectural decisions when they are taken, in the same
change that introduces or implements them; do not leave the rationale only in
chat, commit messages or personal memory. This includes changes to module or
crate boundaries, shared data models, timing contracts, persistence formats,
platform integration and consequential dependency choices. Describe context,
alternatives, the chosen approach and its consequences, and update the ADR index.
Routine local implementation details do not need an ADR.

Before changing an established boundary, read the relevant ADR. Update delivery
status when its implementation changes, and use a superseding ADR when the
decision itself changes. Keep documentation consistent with the actual code so
future contributors can understand both the architecture and why it exists.

Create design briefs before collecting unrelated mockups. Link sketches to their
briefs, identify viewport/input assumptions, preserve alternatives and record
asset provenance. Distinguish mockups from actual running-app screenshots.

## Validation and reporting

For documentation changes, check local relative links and the staged diff. Use the actual commands in `docs/midi-import.md`: Cargo formatting/tests/Clippy,
WASM build and browser smoke where applicable. The piano app commands are in
`docs/piano-player.md`; test its real WASM/audio boundary when changing playback.
Do not confuse building an import library with device, audio or app verification.

Keep README, indexes and provenance current when their facts change. Do not mark
a planned feature, prototype or accepted sketch as a finished implementation.

When changing web files or samples, run `scripts/package_web.py`, the packaged
site smoke under `/Tangent/`, and a launcher smoke with embedded assets. Screenshot
updates must show the real app and remain reproducible via `web/piano/capture.mjs`.
Preserve the bundled MusicXML sample provenance/license and the Bravura font
license when packaging. The prior MIDI sample was replaced at the owner's request.
