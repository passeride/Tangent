# Left/right hand assignment and colour

- Date: 2026-10-26
- Status: Initial demonstration/edit/colour slice implemented under [ADR-0025](../adr/0025-personal-themes-and-demonstrated-hands.md); broader source mapping and inference remain proposed.
- Goal: consistent hand colours and future one-hand practice, without claiming
  that a MIDI keyboard observes the player's physical hands.
- Related: [product plan](product-development-plan.md), [feedback contract](gamification-and-feedback.md),
  [ADR-0021](../adr/0021-passage-practice-and-comparable-context.md).

## Recommendation

Build a source-specific, editable hand-assignment layer between import and
presentation. Preserve what the source actually says; infer a suggested assignment
where useful; let the player override it at part, voice, phrase or individual-note
level. Hand identity is independent of pitch, clef, screen position and colour.

Use four outcomes: Left, Right, Either and Unassigned. Either means an intentional
choice of either hand, not uncertainty. Unassigned means insufficient information.
Do not force every note into one hand merely to obtain two attractive colours.
The initial release should use source structure and explicit mappings; automatic
separation of an undifferentiated MIDI performance is a later, evaluated feature.

## What Tangent currently retains

| Boundary | Existing information | Gap for hand assignment |
| --- | --- | --- |
| PerformanceNote | Runtime ID, part, pitch, onset/end, velocity, optional spelling | No explicit hand or stable authored-note identity |
| MusicXML notation | Written staff, voice, links to performance notes; expanded measure visits | Need a verified many-to-many origin map through repeats, ties and cross-staff notation |
| MIDI adapter | Creates a part per source track/port/channel lane | Structured source track/port/channel identity is not retained on Part; labels are not reliable IDs |
| Live input | Device/channel, pitch, velocity and release | No physical hand observation |
| Rendering | Theme colour slots, expected edge glow and solid played keys | No semantic hand palette; slots must not become stored hand identities |

Before implementation audit MusicXML note-to-performance links across repeated
visits. Do not assume that one WrittenNote.performance_note scalar already models
all repeated occurrences. Keep staff/voice information from import instead of
reconstructing it from audio routing channels or screen geometry.

## Evidence and precedence

| Evidence | Treatment |
| --- | --- |
| Player's note/phrase override | Highest priority; never overwritten by automatic analysis |
| Player's explicit voice/staff/part mapping | Authoritative user choice within its stated scope |
| Explicit, structurally supported source hand instruction | Source evidence, with its origin retained |
| Consistent source voice/part names such as RH/LH | Suggested mapping with explanation; names are fallible |
| Conventional two-staff piano layout | Offer upper → Right / lower → Left as a preview, labelled an assumption |
| Musical separation of mixed MIDI | Optional inferred suggestion, never an authored fact |
| Fixed pitch split | Explicit user-selected fallback, labelled Split at C4 or another chosen pitch |

Within user mappings, note overrides outrank phrase overrides, then voice/staff
rules, then whole-part rules. Opposing rules of equal specificity produce a
visible conflict requiring a choice; last-write order must not silently decide.
When source evidence conflicts, retain it for inspection and mark the result
Unassigned until resolved or accepted by the player.

A voice can cross staves while remaining the same musical line. Conversely,
voices may share a staff or exchange material. Voice-continuity analysis can
support a proposal but cannot prove a hand. Treble clef does not mean right hand;
bass clef does not mean left hand. Do not parse free-text hand directions as
certain until their wording and temporal scope are explicitly supported.

## Import and first-use flow

Import remains immediately playable. In Pause → Hands, show a miniature score or
timeline preview, the proposed colours and the reason for each mapping. A small
status chip says Hands: confirmed, suggested, mixed or unassigned. Confirmation
means accepted by the user, not anatomically measured.

| Source situation | Initial experience |
| --- | --- |
| MusicXML with a conventional grand staff | Preview a staff-based suggestion; offer Accept, Swap and Edit |
| MIDI with plausible RH/LH tracks | Preview lane-name suggestions; allow Listen/solo per lane before acceptance |
| Multiple unlabeled MIDI lanes | Show actual lanes with Left / Right / Either / Unassigned controls |
| One mixed MIDI lane | Start neutral; offer Suggest separation, manual editing or explicit pitch split |
| No accepted assignment | Continue playing in neutral colours; no blocking setup wizard |

Basic editing maps a part/staff/voice to a hand. Advanced editing selects a phrase
or note group and assigns Left, Right, Either or Unassigned, with Undo and Reset
suggestions. Show stable written measure labels plus repeat visits when relevant.
For MIDI show source time. Never offer misleading hand names for arbitrary tracks.

Default edits apply to the written note in all repeated visits. An explicit
“This visit only” override can be a later feature; do not silently support two
inconsistent editing scopes in the first release. Tied segments keep one hand
through a sounding note. Flag a contradictory handover inside a tie instead of
splitting/re-attacking the sound. Fingering and hand redistribution are separate
annotations and do not modify imported pitches, timing or source bytes.

## Automatic separation: later proposal, not a pitch threshold

Analyze the complete piece outside the rendering/audio path, in onset groups and
phrases. Candidate assignments should balance voice continuity, plausible reach,
held-note overlap, movement over time and source evidence. Onset groups may be
split between hands: a simultaneous chord is not necessarily one hand. Sustain
pedal duration is not evidence that fingers stayed on the keys.

Use bounded sequence search, such as beam search, with adjustable soft reach and
movement costs. Allow hand crossing and both hands playing on the same side of
middle C. Avoid fixed octave reach limits that reject different bodies or valid
techniques. Hard user annotations constrain the proposal; impossible or ambiguous
regions remain Unassigned. Do not use a confidence percentage until calibrated
against labelled examples. Initially report source/user/suggested/ambiguous plus
an explanation, such as “voice continuity across staff change”.

A first heuristic may use voice-level majority staff plus phrase continuity, but
must be benchmarked on cross-staff chords, crossing hands, melody/accompaniment
exchanges and wide arpeggios. Do not promise a trained classifier or a large model
before a labelled corpus and measured benefit justify it. No cloud upload is
required for this analysis.

## Architecture and durable identity

| Proposed owner | Responsibility |
| --- | --- |
| Import adapters | Preserve structured source origins: source note, part, staff/voice or MIDI lane/event identity |
| tangent-core::hands (proposed) | Pure assignment resolver, precedence/conflicts and optional inference |
| Storage adapters | Versioned hand profiles and transactional user overrides |
| Bevy Hands UI | Preview, mapping, edit/undo and explanation |
| Renderer | Map resolved hand semantics to theme tokens; never decide hand from pitch |
| Practice target selector | Consume the same resolved assignments if one-hand practice is enabled later |

Proposed records:

| Record | Fields / contract |
| --- | --- |
| NoteOrigin | Stable import-local source identity; links written segments and expanded performance occurrences |
| Assignment | Left / Right / Either / Unassigned; evidence kind and reason; conflicting evidence preserved |
| HandProfile | Exact source-byte hash, profile ID, format/mapping version and revision |
| Override | Stable note/group/voice reference, scope and chosen hand; persisted before reporting saved |
| Derived proposal | Analyzer version, source/context identity, assignments; cache, not user authority |

Runtime note IDs are not durable keys. MIDI identities should distinguish source
track, port/channel and original event/note occurrence, including duplicate
same-pitch/onset notes. MusicXML identities should distinguish source part,
measure occurrence in the document and note element position. Derive these from
unchanged source bytes, not editable titles, current screen order or floating
point time. Repeat expansion maps to these origins rather than creating unrelated
annotations. An importer mapping-version change must migrate deterministically
or invalidate the reference visibly; it must never attach an override to a
different note by accident.

Persist profiles with the exact source, separately from global display colours.
Source replacement requires fresh resolution; fingerprint similarity cannot
silently transfer hand annotations between arrangements. A later matching feature
may propose transfer with review. Global settings contain only palette, display
mode and personal defaults, not assignments for every song. Choose the next
SQLite migration/version after inspecting the live schema during implementation.

Compute assignments once per source/profile/analyzer revision and provide an
indexed immutable lookup to rendering. Changing speed, loop or view does not
reclassify notes or repeatedly run analysis. Background results carry a generation
ID; discard stale results after source/profile changes. For large files bound
analysis work, allow cancellation and continue with the last accepted map.

## Colour and feedback contract

Proposed default: Left uses the theme's blue family, Right its mint family.
Provide dedicated semantic tokens hand_left, hand_right, hand_either and
hand_unassigned, with per-theme contrast verification and a user palette override.
Illustrative dark-theme candidates are blue #A8B9F4 and mint #8CE0C1; these are
candidates, not a claim of tested accessibility across backgrounds.

| Visual | Meaning |
| --- | --- |
| Falling note / notation accent | Intended hand of that target, stable across the whole piece |
| Expected keyboard edge glow | Intended hand; never solid fill before physical input |
| Played solid key | Existing played-input feedback; a matched target may supply its intended hand colour |
| Unmatched live note | Neutral played colour; do not guess which physical hand played it |
| Either | Explicit E marker and paired accent; no rapid alternating colours |
| Unassigned | Neutral outline and ? marker; “Not assigned” in details |
| Both hand targets on the same key | Two edge accents or a paired badge; never whichever renderer draws last |
| Wrong/extra note | Existing error shape/ripple/text, orthogonal to hand hue |

Provide L/R badges, a legend and optional hand patterns so colour is not the only
signal. Correctness must not recolour a left-hand target as a right-hand target.
Reduced motion and effects Off keep hand cues intact. Streak intensity changes
motion/intensity, not hand identity. Matching a played note to a target permits
“matched Left target”; it never permits “Left hand verified”. In FreeForm use
neutral notes unless the player explicitly chooses a split or device/channel map,
labelled as such.

## Practice and ambiguous unisons

A later Both / Left / Right selector must use this same profile. Hand assignment
is not a new score evaluator and does not change pitch coverage by itself.
For single-hand practice, explicitly list/exclude Unassigned targets, optionally
include Either, and show the selected denominator before starting. Never present
zero assigned notes as 100% success. Backing-track scope stays independently visible.

The existing evaluator groups same-onset/pitch unisons into one physical target.
If both hands are assigned to a unison, retain the visual dual membership while
requiring at most one key attack for coverage. Deduplicate after the selected-hand
filter and never invent two distinguishable hand hits from one MIDI event.
If hand-based selection affects evaluated targets, record profile identity/revision
or an immutable assignment snapshot in run context. Historical results must not
be reinterpreted after edits. A colour/palette-only change does not segment a run;
changing the selected target set does. Coordinate this with ADR-0021 rather than
adding a separate competing practice model.

## Delivery order and acceptance

1. Verify/preserve origin links through MusicXML repeats/ties and MIDI lanes.
2. Deliver editable source-based mappings, neutral unknown state and saved profile.
3. Add consistent hand colours to every view, preserving glow/solid semantics.
4. Add single-hand filtering and compatible historical context as a separate slice.
5. Evaluate automatic mixed-MIDI proposals on labelled examples before defaulting On.

Fixtures must include conventional grand staff, both hands in treble clef,
cross-staff voice, voice IDs reused across parts, crossing hands, one-track MIDI,
misleading track names, repeated bars/endings, ties, shared-key unisons and missing
metadata. Test override precedence, source/profile restore, mapping-version
mismatch, rejected writes and analyser cancellation. Inspect all themes, reduced
motion, 4K and narrow windows. A P-45 check must demonstrate that the same key
played with either physical hand produces the same ordinary MIDI note and is not
claimed as verified hand recognition.

Measure proposal quality separately for covered notes and errors among assigned
notes; include abstentions and manual correction effort. Do not hide errors by
reporting confidence only on easy passages. Set launch thresholds after building
the fixture corpus and observing correction time; no accuracy figure is claimed.

## Primary references

- [MusicXML notation basics](https://www.w3.org/2021/06/musicxml40/tutorial/notation-basics/): independent voice/staff concepts and a cross-staff chord example.
- [MIDI Association: MIDI messages](https://midi.org/about-midi-part-3midi-messages): Note On carries channel, key and velocity; those fields do not identify a physical hand.

Sources describe format semantics. The assignment rules, UI, architecture and
proposed search algorithm above are Tangent design recommendations, not claims
that either format already supplies reliable hand labels.

## One-hand demonstration and confirmation — requested extension

Offer Teach left / Teach right in Hands. The player selects a bounded passage,
listens if needed, then plays only the chosen hand against a slowed transport.
Record incoming note attacks locally with source-song timestamps, device identity
and the transport discontinuity generation. Pause, seek, speed changes and loop
wraps segment the demonstration; never align across a discontinuity accidentally.
Command-key notes and backing playback are excluded before capture.

Align the demonstration to score targets using a bounded sequence alignment with
pitch, relative onset and chord membership; permit missed/extra notes and local
rubato. Do not label every unplayed note as the other hand: omissions are normal.
Repeated pitches/phrases and same-key unisons may have several candidate matches;
show them unresolved rather than applying arbitrary nearest-time matches.

Preview matched notes in the selected hand colour, unmatched notes neutral, and
extras separately. Let the player correct the preview and press Save assignment.
Playing alone never silently overwrites a confirmed map. A second demonstration
of the other hand can add positive evidence; contradictions require review.
Store the accepted assignments, not an unsupported claim of observed physical
hand identity. Raw demonstration events are temporary by default; retain them
only through an explicit optional recording setting.

Persist with a transactional hand_profiles / hand_overrides model keyed by exact
source hash, profile ID, source-origin mapping version and profile revision.
Demonstration provenance records selected hand, passage, alignment algorithm
version and review status. Accepted matches reference stable source-note origins;
ambiguous matches are not saved as confirmed. Store an immutable assignment
snapshot/revision reference when a later practice run filters by hand. Report
Saved only after storage acknowledgement; failed writes retain the editable
preview with Retry. Reload restores the same map and edit provenance. Exact
SQLite migration numbering is reserved for implementation against the live schema.
