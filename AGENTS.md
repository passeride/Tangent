# Contributor guidance

Tangent is a new Bevy piano-practice project targeting web from the first
application slice. The first implemented element is byte-based native/WASM MIDI import; the Bevy
application and playback remain future work.

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

## Decisions and design

Create ADRs from the template, number them sequentially and add them to the index.
Record actual decisions; keep unresolved implementation choices proposed. A new
ADR supersedes an old decision with links in both directions.

Create design briefs before collecting unrelated mockups. Link sketches to their
briefs, identify viewport/input assumptions, preserve alternatives and record
asset provenance. Distinguish mockups from actual running-app screenshots.

## Validation and reporting

For documentation changes, check local relative links and the staged diff. Use the actual commands in `docs/midi-import.md`: Cargo formatting/tests/Clippy,
WASM build and browser smoke where applicable. There is no Bevy application yet.
Do not confuse building an import library with device, audio or app verification.

Keep README, indexes and provenance current when their facts change. Do not mark
a planned feature, prototype or accepted sketch as a finished implementation.
