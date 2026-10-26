# ADR-0022: HUD density and explicit keyboard fitting

- Status: Accepted
- Date: 2026-10-26
- Delivery: Implemented; automated/visual validation recorded separately
- Related: ADR-0005, ADR-0009, ADR-0017, ADR-0021; UI v2; UX-12/14

## Decision

Use Focus, Standard (default) and Arcade as presentation density presets. Focus
retains transport, output, rate and loop context; Standard adds title/time; Arcade
adds real evaluator points/streak where space permits. This reconciles the older
UI v2 Detailed/Performance proposals with the product strategy. Arcade is a HUD,
not a new calibrated Performance mode. No scoring, timing, input or effect rules
change when switching presets. FX intensity and reduced motion remain independent.

Persist an optional validated hud enum (0–2) in the existing version-1 preference
JSON. Missing fields migrate in memory to Standard; invalid choices are rejected
without rewriting the database. No relational schema or dependency changes.

Provide explicit Fit keys using the smallest existing 49/61/88-key range covering
selected target pitches and held input. Never shift geometry automatically while
playing: this could move the player's input target. Notes outside even an 88-key
piano remain flagged. Cache counts of selected notes outside the current keyboard
by source/context/range, avoiding an extra full-song traversal every render frame.
Held-input bounds are checked separately. Fitting changes presentation only.

For low windows the pause overlay condenses passage setup into Settings → Practice;
very low windows retain that route through Settings. Short settings use two columns.
All surfaces remain Bevy UI, with keyboard focus and non-drag loop controls.

## Alternatives and limits

A separate arcade renderer would fork UI semantics. Coupling density to particles
would make quiet information-rich or motion-safe layouts impossible. Continuous
keyboard auto-fit would move keys under held fingers. Dynamic zoom, screen-reader
semantics, per-staff filtering and validated timing grades remain separate work.

Validate legacy preferences/defaults and rejected values, persistent independent
HUD/FX choices, 49/61/88 fitting including extreme pitches, reduced motion,
720p/narrow/4K and short-window controls. No physical-device or user-study result
is implied by automated screenshots.
