# 3D piano stage

Date: 2026-10-26. Status: implementation; physical GPU/MIDI review pending.

## Goal and visual hierarchy

Make the session feel like a playable instrument while keeping musical reading
primary. The piano spans the same 49/61/88-key range below the music. A dark
case, narrow illuminated rear rail, chamfered ivory surfaces and raised black
keys create depth. Broad lighting exposes the front faces. No camera movement
or perspective convergence competes with the falling notes.

An orthographic view preserves alignment from every note lane to its key. The
piano includes its case, side cheeks and front bed; it is a practice keyboard
view rather than a distant model of a grand piano with legs and lid.

## Interaction states

| State | Appearance |
| --- | --- |
| Idle | Lit, neutral key surface with visible bevel and depth |
| Expected note / revealed hint | Hand-coloured luminous edges; neutral surface |
| Held by MIDI, typing or pointer | Solid played colour and short downward key tilt |
| Held and expected | Solid surface plus expected luminous outline |
| Released | Fast return; no stuck highlight |
| Effects off | Essential expected edges remain; cosmetic held spill stops |
| Reduced motion | Immediate key state; no interpolated key movement or particles |
| Paused/settings | Existing dimmed scene below the Bevy modal |

Falling blocks gain an illuminated ridge and shaded fill. Text labels appear only
where the key width can support them. Glows stay close to the strike rail, leaving
the reading area clear. Theme colours are consumed from the existing personal
palette, including Paper; colour is not the only key-state indicator.

## Ownership and verification

See [ADR-0026](../adr/0026-procedural-3d-piano.md). Test desktop, short windows,
390px touch layouts and 4K/HiDPI. Check black-key overlap and backs, range changes,
press/release, hints off, pause/resume and no stale piano when leaving a session.
The capped render target is a resource budget, not a measured FPS guarantee.

All meshes and glow geometry are authored in this project. There are no new
external visual assets. Capture actual screens through `web/piano/capture.mjs`.
