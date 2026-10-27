# ADR-0032: Scalable score overlay and multiple systems

- Status: Accepted
- Date: 2026-10-27
- Supersedes the split-screen layout in [ADR-0031](0031-combinable-note-views.md).

## Decision

When both views are enabled, retain the full-height falling field and draw a
translucent score overlay above it. The overlay has one notation system and does
not change falling-note timing or keyboard alignment. When horizontal notation is
the sole view, fit as many whole systems as the available height permits. Choose
consecutive authored measures per row, with page turns following the active
playback visit; repeats return to their written measure. MIDI uses consecutive
eight-second time windows instead of inventing measures. The current row retains
the playhead and held-note feedback. When a page is partially filled, omit empty
rows. Space for ledger lines and notes remains reserved between systems.

Persist `score_scale` in the existing v1 preferences JSON, default 100%, bounded
50–200% in 25% increments. Scale sets staff spacing and consequently the number
of systems and, where necessary, measures per row. Clamp to available height on
very small screens; the displayed setting is preserved. No database migration,
new dependencies or input/scoring behavior change is needed.

A fixed split previously shortened the falling field and created a blank region
between the two views. Alternatives included overlapping unbacked notation, which
can obscure notes and reduce contrast, and a full score renderer rewrite. A
translucent background makes the overlay legible while retaining vertical context.
