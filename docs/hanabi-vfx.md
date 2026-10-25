# Note feedback VFX (Hanabi + WebGL2)

Tangent uses [bevy_hanabi](https://github.com/djeedai/bevy_hanabi) on native
desktop. The dependency is pinned to an upstream Bevy 0.20-compatible commit.
GPU particles need compute shaders, so the existing WASM/WebGL2 build uses
the same event grades and colors rendered as bounded Bevy UI particles instead.
Neither backend requires external art assets or an extra canvas.

## Presets

| Input event | Preset | Visual |
| --- | --- | --- |
| Correct melody note / completed practice chord | `correct` | Large mint burst, 32 Hanabi particles + bloom/ripple |
| Incorrect played note / irrelevant practice-chord note | `wrong` | Coral impact, 22 Hanabi particles + brief ripple; resets streak |
| FreeForm / paused play / automatic score playback | `free-play` | Blue sparkle, 12 Hanabi particles; no judgement |
| 5 correct hits in a row | `streak-5` | Gold 60-particle celebration |
| 10 correct hits | `streak-10` | Violet 96-particle celebration |
| 25, 50, 75… correct hits | `streak-25` | Wide gold 144-particle celebration |

All other correct hits use `correct`. A song target can award a hit only once
until seek/restart; repeated presses on it are neutral. Partially played chords emit neither
a success nor a failure until the chord is complete or an unrelated note is
pressed. In reading practice an incorrect octave counts as a mistake.
The visible streak is session-local and deliberately **not** persisted as a
score; scoring should be integrated separately.

## Rendering and limits

- Native: Hanabi GPU particles in the shared Camera2d scene; emitted just above
  the strike line. The UI bloom and expanding shockwave remain visible immediately.
- Web: WebGL2-safe glowing UI particles and expanding rings; no compute shader or WebGPU requirement.
- Particles and shockwave rings scale with the **logical viewport**, not fixed pixels:
  `scale = clamp(min(width/1280, height/800), 0.65, 3.5)`.
  At 1920×1080 scale is **1.35×**; at 3840×2160 it is **2.7×**.
  OS DPI settings determine how physical pixels map to Bevy logical pixels.
- Native Hanabi's particle size **and speed** use an effect-instance scale property;
  the browser fallback uses the same scale for size and travel distance.
- UI particles are capped at 520, rings at 24, and at most 32 bursts enter per frame.
  Hanabi GPU emitters despawn after their preset duration + 0.25 seconds.
- `P` / Settings → Sparkles disables feedback effects. Importing a song,
  returning to the menu, and changing exercise reset streaks.
- Color, duration, burst radius and particle counts live in `scene/effects.rs`,
  independent of music evaluation. WebGL2 caps particles per visual burst at 56
  to avoid creating excessive Bevy UI nodes on modest GPUs.
- CI deliberately builds WebGL2 and **Windows x64 / Linux x64** native targets;
  other desktop targets are excluded as in the latest `main` workflow.

## Verification

```sh
cargo test -p tangent-player
cargo clippy -p tangent-player --all-targets -- -D warnings
cargo build -p tangent-desktop --release

rustup target add wasm32-unknown-unknown
cargo check -p tangent-player --target wasm32-unknown-unknown
npm test --prefix web/piano
```

Manual test: connect a MIDI keyboard, start *Train → Notes*, play a right note
and a wrong note, then reach 5 and 10 correct consecutive exercises. Repeat in
*Train → Chords* (partial chords should not penalize). In *Play*, strike keys
that do and do not match the current score. Confirm that shockwaves, color and HUD are consistent in native and web.
Resize from 1280×800 to 1920×1080 and confirm bursts grow proportionally;
check an actual 4K viewport at normal and HiDPI scaling.

**Caveat:** Hanabi currently requires WebGPU on WASM; enabling it directly in
the shipped WebGL2 application would break browsers without WebGPU. The default
web build therefore deliberately does not instantiate Hanabi.

## Attack detail and comfort

UI sparks launch in an upward fan with fading velocity trails and a white core;
shockwave arcs expand quickly and decay gently. Hanabi starts bright and settles
to its grade colour. Settings → Effects offers Soft / Full / Punchy intensity.
Settings → Accessibility → Reduced motion removes bursts/rings on both backends;
played-key colour, expected glow and practice pitch pointers remain available.
Both preferences are saved, with backwards-compatible defaults for older data.
No camera shake or full-screen flash is used. Practice streak text no longer
occupies the exercise heading. See ADR-0017 and the physical checks below.
