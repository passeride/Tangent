# Architecture decision records

An ADR records **why** a consequential choice was made: the context, alternatives,
tradeoffs and evidence. It is not a task checklist or proof of implementation.

| ID | Decision | Status | Delivery |
| --- | --- | --- | --- |
| [0001](0001-bevy-and-web.md) | Use Bevy and validate web from the first application slice | Accepted | Not implemented |

## Create an ADR

1. Copy [the template](template.md) to `NNNN-short-title.md`, using the next unused
   four-digit number.
2. Begin with `Proposed`. Describe the concrete problem and viable alternatives.
3. Link relevant design briefs, experiments and independent primary references.
4. Record an explicit decision and its date before marking it `Accepted`; a
   maintainer may make that decision within the authorized work.
5. Add the record to the index above. Track implementation separately.

Statuses: `Proposed`, `Accepted`, `Rejected`, `Superseded`. A new decision that
replaces an accepted one gets a new ADR and links in both directions. Preserve the
old rationale rather than making history look as if the original choice never
existed. Small local refactors do not require an ADR.

Likely next questions: audio scheduling and clock ownership, browser MIDI
integration, music/notation representation, local storage and the notation
renderer. These are open topics, not preselected solutions.
