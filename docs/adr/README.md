# Architecture decision records

An ADR records **why** a consequential choice was made: the context, alternatives,
tradeoffs and evidence. It is not a task checklist or proof of implementation.

| ID | Decision | Status | Delivery |
| --- | --- | --- | --- |
| [0001](0001-bevy-and-web.md) | Use Bevy and validate web from the first application slice | Accepted | Initial Bevy core/browser piano |
| [0002](0002-byte-based-midi-import.md) | Byte-based MIDI import and explicit source timing | Accepted | Initial importer implemented |
| [0003](0003-performance-model-and-piano-web-slice.md) | Shared music model, Bevy core and browser piano/audio adapters | Accepted | Initial web piano implemented |
| [0004](0004-distribution-pages-and-desktop-launchers.md) | Tested web distribution, Pages and desktop launchers | Accepted | Packaging implemented; CI/deployment verified separately |
| [0005](0005-one-bevy-gpu-surface.md) | One Bevy/wgpu surface, all controls and notation in-engine | Accepted | Replaces the initial Canvas/HTML presentation |

| [0006](0006-musicxml-import-and-authored-notation.md) | MusicXML adapter and authored notation | Accepted | Initial uncompressed partwise subset |

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

Remaining questions include GPU rendering, off-main-thread audio, authored-score
layout, native adapters and local storage. See [follow-up tasks](../TODO.md).
