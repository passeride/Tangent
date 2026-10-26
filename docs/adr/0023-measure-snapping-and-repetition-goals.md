# ADR-0023: Authored measure visits and voluntary repetition goals

- Status: Accepted
- Date: 2026-10-26
- Delivery: Implemented; automated evidence tracked in product-development-plan
- Related: ADR-0007, ADR-0021, ADR-0022; UX-02/13

## Decision

Snap an existing loop outward to its particular expanded MusicXML measure visits.
Without a loop, Snap bars selects the visit at the current source position. A
range ending exactly on a measure boundary does not include the next measure.
Repeated written bars remain distinct visits; the range label includes authored
bar labels plus visit indexes. Plain MIDI keeps seconds and has no fabricated
bar snapping. Core owns snapping; Bevy owns the action and labels.

Offer an optional session-local goal of three complete loop attempts with at
least 80% pitch coverage. Count only validated, evidenced completed runs with
actual input, and deduplicate by run ID. Backing sound is allowed; this is not a
claim of clean rhythm, unassisted mastery or a competitive result. Extra/wrong
attacks remain separate result metrics. No XP or score bonus is introduced.

A goal belongs to the exact source and comparison context. Range, part, rate or
backing-audio changes reset its count; clearing the loop or loading another source
removes it. Goals do not survive restart. Progress remains in ordinary saved
attempt history. Reaching the goal suggests a small speed increase; speed never
changes automatically. No mandatory streak, daily loss or account is added.

## Alternatives and consequences

Applying a range to every occurrence of a written measure would silently expand
practice scope. Inferring MIDI barlines would overstate notation knowledge.
Awarding repetitions from autoplay, partial runs or repeated checkpoints would
make the target meaningless. Persistent weekly goals and validated learning
mastery need their own product/data scope and remain future work.

Validate repeat-visit snapping, exact boundaries and absent notation, goal
eligibility/deduplication, autoplay rejection, real-input completion, context
reset and low-window controls. No new dependency or external asset is introduced.

## Integration fixes

Song setup retains a Note/chord exercise defaults subpage under Practice settings;
changing those defaults must preserve the paused song run. Ctrl+O also works while
paused. Rapid computer-key taps that press/release in one frame are judged once
and end released, rather than leaving a held key. Browser regression covers these
actual input/navigation boundaries.

## Bounded details and derived-result work

Register matched onset/pitch evidence incrementally from the evaluator's latest
new target, rather than copying the growing hit set on every input. New attempts
retain at most 20,000 detailed hits; beyond that, keep accurate aggregate counters
and mark detail unavailable. Older larger valid payloads remain readable. No
relational schema change is needed. Goals use validated evaluator counters and
actual input categories even when a long run has exceeded the detail cap.

Cache weak-range analysis by result ID/revision and exact loaded source so Bevy
does not rebuild the source target set every render frame. Compute suggestions
only from valid available details and verify hits belong to the selected source
target set. Missing detail exposes Replay/manual range selection, never a fake
heatmap. These are work/memory bounds, not measured hardware latency guarantees.
