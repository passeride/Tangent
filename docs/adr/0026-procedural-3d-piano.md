# ADR-0026: Procedural 3D piano on the shared Bevy surface

- Status: Accepted
- Decision date: 2026-10-26
- Delivery: Implemented; validation record in the PR
- Amends: [ADR-0005](0005-one-bevy-gpu-surface.md), camera count only
- Related: [UI v2](../design/tangent-ui-v2.md), [3D piano brief](../design/3d-piano-stage.md)

## Context

The owner requested replacing the flat session keyboard with a full 3D piano
and glow effects. Note lanes, pointer input, themes and the solid-played versus
edge-expected distinction must remain useful for practice on desktop and WebGL2.

## Decision

`scene/piano3d.rs` owns procedural chamfered key meshes, case geometry, PBR
materials, two directional lights, key depression and an orthographic Camera3d.
The additional camera renders only layer 1 into a bounded, dynamically resized
sRGB texture. `scene/draw.rs` composites that texture as a pooled Bevy ImageNode
at the keyboard's existing position. Camera2d is explicitly the default UI camera.
There is still one Bevy App, one window/canvas and one input/layout owner.

World X is expressed in logical UI pixels. An angled orthographic projection
shows front faces and raised black keys without bending note lanes or changing
horizontal hit regions. Layout's projected black-key rectangles include their
raised backs; input checks black keys before whites. Only live held keys depress
and fill with the played colour. Expected notes use emissive edge geometry with
hand colours and soft UI light spill above the strike line. They never depress
keys or fill their bodies. Training hints remain explicitly controlled.

The 3D module consumes scene state; it owns no timing, audio, MIDI or scoring
rules. Existing theme tokens drive the materials. Reduced motion snaps key states
instead of interpolating; essential expected edges remain with cosmetic FX off.
No new persistent fields or migrations are needed.

## Alternatives and consequences

Shaded UI rectangles would preserve the old rendering boundary but would not
provide actual geometry, lighting or occlusion. A perspective camera would
misalign outer keys and straight falling lanes. A second canvas or JS/WebGL
renderer would break the single-surface contract and duplicate input ownership.
External model files add asset provenance and loading costs without improving
this fixed practice view.

Enable only Bevy's existing pinned `bevy_pbr` and `bevy_light` features. Meshes
are original code, shared across keys; materials/entities are reused and per-key
assets are released on range changes. Target density is capped at 2x and width
at 4096 pixels. The 3D camera is disabled outside sessions. Shadow maps, HDR,
bloom postprocessing and imported model dependencies are deliberately absent;
soft glow uses portable UI layers plus real emissive mesh edges. This increases
GPU work and WASM size; physical-device performance still needs measurement.

## Validation

Check WASM and native builds, warnings-denied Clippy and existing browser flows.
Exercise black/white pointer boundaries, 49/61/88 keys, resize, 4K/HiDPI, all themes,
expected-only/held/both/released, FX off, reduced motion, pause/settings and menu
return. Screenshots must come from the real application, never a mockup.

Primary API reference: Bevy 0.20.0 source and official render-to-texture and
orthographic examples. No implementation or assets are taken from Neothesia.
