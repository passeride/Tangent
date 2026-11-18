# Interactive learning modules — design brief

The teaching sequence is **hear → experiment → reveal → understand**. A note
can be in the active chord, a scale tone outside it, or chromatic. This is a
description, not a musical-value judgment.

## Entry and flow

Train → Lær musikkteori opens the module catalog. A module contains short
steps with paragraphs, an instruction, a reusable visual, playable reference
and an action to perform at the physical piano. Previous, Next, Listen and
Help are common controls. Tasks gate Next on real user MIDI, touch or computer
keyboard input. Text is wrapped on the Bevy surface, not a DOM overlay.

## Authoring vocabulary

- Module: stable ID, title, summary, ordered steps
- Step: heading, paragraphs, instruction, illustration, task, example notes
- Read: theory without a required piano input
- Note: exact MIDI note, including octave
- Chord: notes held simultaneously, any order
- Sequence: ordered attacks with release between repeated notes
- Explore: invite a pitch without labeling its dissonance an error
- Illustration: keyboard, major/minor interval strip, chord context, fifths
- Listen: platform-generated notes, never counted as the player's answer
- Reveal: optional chord/scale/chromatic cues and next-note hints

The lesson domain belongs to crates/tangent-core/src/lesson.rs, the renderer
to crates/tangent-player/src/scene/lesson_view.rs; UI interacts through the
existing State, Action and platform interfaces.

## Bundled curricula

**Finn hjem:** C as tonal center; C E G anchor; B→C resolution; B over
C/F/G to experience changed harmonic role.

**Dur og skalatrinn:** the whole/half-step pattern, C major ascending,
E→F semitone and a C major triad.

**Moll og kontraster:** C natural minor, E♭ versus E, C minor triad
and the difference between harmonic pattern and emotional expression.

**Kvintsirkelen:** fifth = seven semitones; C→G→D; key signatures and
relative minor relationships across all twelve major roots.

## Boundaries, verification and follow-up

Reference note attacks never enter player evaluation and are stopped by
navigation, new player notes and loss of focus. No new HTML UI. Cues include
labels as well as colors. Check desktop, 390×700 and 1280×420 layouts.
Browser smoke uses simulated MIDI hardware input; physical MIDI/audio timing
must be checked on hardware.

Later work: persisted learning progress, validation for externally authored
modules, localization, written notation components, rhythm/ear challenges,
configurable key/octave, accessible cue alternatives and long-form lessons.
