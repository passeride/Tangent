# ADR-0021: Source-time passage practice and comparable results

- Status: Accepted
- Date: 2026-10-26
- Delivery: Initial implementation; validation tracked in the development plan
- Related: ADR-0005, ADR-0015, ADR-0018, ADR-0020
- Supersedes: ADR-0018's whole-song-only attempts and source-only recent-score comparison

## Context and alternatives

Focused practice needs repeatable passages and honest comparison. A second UI clock
would duplicate musical timing; guessing hands from pitch would mislabel arbitrary
arrangements. Source identity alone cannot distinguish 70% and 120% attempts.
Rewriting old records with assumed defaults would invent historical context.

## Decision

Core Passage is a validated half-open source-microsecond range on the expanded
playback timeline. Transport owns looping and fractional rate accumulation; it
returns boundary crossings with bounded arithmetic even for delayed frames.
The Bevy controller cancels/resyncs adapters at each observed boundary. Adapters
also receive the end boundary so their lookahead cannot emit later attacks.
Seek is bounded to the active loop; setting a new A clears the previous range.
Source replacement clears range and explicit part choice.

Each observed iteration is a separate attempt. If rendering misses multiple loops,
only the observed attempt is closed; no synthetic successful attempts are created.
Part choice uses imported part indexes with All parts default. It filters target
judgment and glow, while optional backing audio and the full score remain available.
It does not claim hand/staff isolation where the importer has not retained it.

An optional SongContext and optional matched-target evidence extend version-1 JSON
checkpoint payloads: exact range, source duration, rate, explicit part, evaluator
version, backing-audio setting and a bounded sorted set of actual input categories. SQLite schema remains v5: these are additive
fields inside the existing payload column, not new relational columns. Both adapters
validate them. Legacy payloads deserialize with absent context/evidence and remain
readable; unknown tempo is never replaced with 100%. Source identity stays BLAKE3
of exact source bytes. Runtime target IDs are not used as persistent identities;
matched targets use unique source onset/pitch, consistent with layered unisons.

Recent pitch-score comparisons require complete attempts with identical source,
context, input categories and target denominator. They are not competitive PBs or timing grades.
Changing rate, backing-audio, part, range or seeking closes the current attempt.
Pause retains it. Pointer timeline seeks must follow the same rule as arrow seeks.

Weak-section suggestions require a completed evidenced attempt and the exact loaded
source. Count missed unique targets in 10-source-second buckets anchored at passage
start; select most misses, earliest on ties, clipped to the original range. No
misses or missing evidence produces no invented recommendation. Retry restores
recorded rate/part/range and opens paused. Authored bar/repeat snapping, per-staff
selection, cross-source reopening, and raw MIDI recording are not delivered here.

## Consequences and validation

Sustained notes may reattack at loop A, matching existing seek semantics. Loop
boundaries reset target identity and cosmetic streaks. Half-open membership excludes
target onsets at B; held notes originating before A can sound as backing without
becoming new selected targets. No external dependencies/assets are added.

Tests cover bounds, large frame crossings, fractional tempo, context/evidence
validation, legacy absence, compatible comparisons, passage suggestions, browser
SQLite round trips and actual Bevy loop/retry interaction. Physical P-45 and native
GPU/audio verification remain explicitly separate from automated simulation.
