# ADR-0038: Declarative MIDI-driven interactive lessons

- Status: Accepted
- Created: 2026-11-18
- Decision date: 2026-11-18
- Delivery: First lesson engine and four authored modules
- Related: [ADR-0005](0005-one-bevy-gpu-surface.md), [ADR-0008](0008-navigation-and-practice.md), [ADR-0017](0017-practice-reference-and-motion.md), [design brief](../design/interactive-lessons.md)
- Supersedes: None

## Context

Standalone reading and scale drills provide little understanding of why the
same note changes function under different harmony. Tangent needs theory
lessons working with MIDI, touch and computer keyboards on both runtimes.
All presentation remains in the existing Bevy surface.

## Decision

A static declarative curriculum in tangent-core::lesson contains modules
(stable ID, title, summary) with steps (paragraphs, instruction, illustration,
task and reference example). Music theory and advancement are evaluated in core,
independent of Bevy and platform adapters. Tasks: Read, exact Note, simultaneous
Chord, ordered Sequence, open-ended Explore. Fresh player input completes tasks;
reference audio does not. Explicit help reveals musical roles, avoiding binary
right/wrong feedback about deliberate nonchord tones. The lesson_view module
renders reusable text, keyboard, interval, harmony and circle-of-fifths widgets
inside the existing Bevy Drawing and Action systems, using the existing platform
note-output adapter for examples.

Alternatives rejected: an HTML overlay violates ADR-0005; extending the scored
drill model would grade nonchord tones as wrong; static sheet music cannot
express all interactive prompts.

## Consequences

Initially bundled: Finn hjem, Dur og skalatrinn, Moll og kontraster and
Kvintsirkelen. No new runtime dependency or external curriculum format.
Progression is in-memory. Persistence, localization, score/staff components,
audio calibration, physical MIDI checks and external curricula are follow-ups.
New task kinds require core tests and renderer implementation. Examples use
international B naming.

## Validation

Core tests cover tonal role, scale intervals, sequences, simultaneous chords
and the fifth-circle. A browser smoke checks navigation, MIDI input, reference
isolation and viewport button bounds. CI/device results are reported separately.
