# ADR-0005: One Bevy GPU surface for the complete application

- Status: Accepted
- Decision date: 2026-10-24
- Delivery: Implemented; real Chromium GPU UI, fullscreen, input and packaged-site tests pass
- Supersedes: ADR-0003's Canvas/HTML presentation choice, not its domain/audio boundaries
- Related: [ADR-0004](0004-distribution-pages-and-desktop-launchers.md)

## Context

The owner clarified that the entire page must be one Bevy instance, with all
menus inside the GPU window, a fullscreen control and keyboard shortcuts. Falling
noteheads must retain the bass/treble staff lines. The earlier Canvas/HTML adapter
was a feasibility implementation and does not satisfy this product requirement.

## Decision

The web entry point instantiates exactly one Bevy App with DefaultPlugins,
one window bound to the full-page canvas, one camera and Bevy UI rendered through
wgpu's WebGL2 backend. Bevy owns drawing, note projections, controls, settings,
input hit testing, themes, particles and transport. Reuse entity pools for UI
primitives and text; do not respawn a scene graph on every frame. Keep musical
models and metrical/absolute timing in tangent-core and the import adapter.

WebGL2 is the selected wgpu backend for this slice, including browsers without
WebGPU. It is real Bevy GPU rendering, not a Canvas 2D emulation or a CSS overlay.
There is no second Bevy App driving the visible player. The existing headless
Player facade remains available for native domain tests but is not instantiated
by the web entry point.

The HTML shell contains only the canvas, a hidden native file-picker input and
a loading/fatal-startup fallback. No application buttons, selects, menus or
transport controls live in HTML. The operating system file dialog and browser
permission/fullscreen APIs remain platform capabilities. Audio activation runs
inside a trusted canvas input event; all UI decisions remain in Bevy.

The JS platform adapter is limited to audio-clock access, scheduling on Web Audio,
MIDI permissions/messages, file bytes, fullscreen, external credits and diagnostics.
It has no drawing code, layout or shortcut action mapping. Pass commands at this
boundary; Rust owns song time and actions. Keep the 100ms sound lookahead from
ADR-0003 and preserve pause/seek/controller behavior. Hidden-tab time must not
advance the transport before a queued pause is consumed.

Use the same rotated five-line bass/treble staff in both staff-block and falling
notehead views, with local ledger lines and accidental hints. Noteheads hold at
the strike line for key-down duration; the source remains performance data rather
than authored engraving. The separate stationary score view retains its playhead.

Global shortcuts: Space play/pause; arrows left/right ±5s, Shift ±15s; arrows
up/down ±5% volume; Home/End; F11 fullscreen; V view; B theme; M song sound;
P particles; X piano profile; R visible range; F1 settings/help; Esc close settings;
Ctrl+O import, Ctrl+B sample, Ctrl+M MIDI. A/W/S/E/.../K remain piano keys.

UI refinement 2026-10-25: all controls/settings are now in a pause screen opened
by the corner gear, Esc or F1. Opening it pauses transport and releases voices.
Space resumes and closes it; closing via Esc/F1/Return leaves transport paused.
The playing surface retains only the gear, notes, piano and seek timeline.

Bevy is a wasm-target dependency here, so the browser-backed desktop launchers
continue embedding this same tested application without linking native GPU/audio
libraries. Native Bevy windows/audio are a distinct future adapter.

## Consequences and alternatives

WASM size and compilation cost increase. GPU/browser tests must exercise the
actual render surface, all controls, fullscreen entry/exit and packaged project
URLs. A headless engine build alone cannot verify the UI. README screenshots must
be recaptured from this implementation. WebGL2 is now a runtime requirement.

Bevy's default Fira Mono subset is embedded through the default_font feature;
include the Mozilla/Telefonica attribution and SIL OFL text with distributions.
Dependency inventory and notices are regenerated for the added rendering stack.

Keeping HTML controls would violate the clarified requirement. Wrapping Canvas
inside a nominal Bevy page would retain two render/input owners. A separate native
UI framework would not address the shared GPU surface either.

Primary references: Bevy 0.20.0 crate source/API and official UI examples,
https://bevy.org/examples/ui-user-interface/button/,
https://github.com/bevyengine/bevy/tree/v0.20.0.

The pinned toolchain produced zero-length object/archive failures in multi-CGU
WASM release builds in this environment. Release codegen-units is set to 1 after
reproducing the failure with a clean target directory; keep this workaround visible
and re-evaluate on a future toolchain update rather than silently changing Rust.
