# ADR-0001: Bevy with web as an initial deployment target

- Status: Accepted
- Created: 2026-10-24
- Decision date: 2026-10-24
- Delivery: Initial Bevy core/browser piano implemented via ADR-0003; GPU/native adapters and persistence remain open
- Basis: The project's owner requested a new Bevy implementation with web deployment as a goal
- Supersedes / superseded by: None

## Context

Tangent is a new piano-practice project. It needs interactive visual feedback and
should be usable in a browser. Treating web as an afterthought would leave audio,
MIDI permissions, file access and persistence assumptions untested until late.

## Decision

Use Bevy as the application and visual framework, and include a real browser
build in the first application slice. Keep music and practice behavior testable
without a renderer or connected device. Put browser and native capabilities
behind explicit integration boundaries.

Begin with a small feasibility slice: activate sound, receive a piano or fallback
input, show visual feedback, import a music file and retain it across reload.
Package versions and implementations of these capabilities remain open choices.
Record them separately after checking compatibility and measuring behavior.

## Consequences

Web constraints shape the architecture early. A successful native build alone
will not close a milestone that promises browser behavior. Native support can
share the musical behavior while using different platform integrations.

Bevy does not remove browser permission, audio-latency or storage constraints.
The initial prototype must establish those capabilities rather than promise
support for every browser or device.

## Alternatives

- Native first, web later: delays the validation of a stated product goal.
- A framework migration of an existing fork: does not meet the requested new
  implementation and source-provenance boundary.

## Provenance and scope

This records the owner's framework/platform direction. It does not import another
project's source or architecture records and does not certify clean room status.
The exposure and implementation handoff still need to be recorded as described
in [PROVENANCE.md](../../PROVENANCE.md).
