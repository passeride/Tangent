# Note feedback VFX (Hanabi + WebGL2)

Tangent uses [bevy_hanabi](https://github.com/djeedai/bevy_hanabi) on native
desktop. The dependency is pinned to an upstream Bevy 0.20-compatible commit.
GPU particles need compute shaders, so the existing WASM/WebGL2 build uses
the same event grades and colors rendered as bounded Bevy UI particles instead.
Neither backend requires external art assets or an extra canvas.

## Presets

| Input event | Preset | Visual |
| --- | --- | --- |
| Correct melody note / completed practice chord | `correct` | Mint burst, 12 particles |
| Incorrect played note / irrelevant practice-chord note | `wrong` | Brief coral burst, 8 particles; resets streak |
| FreeForm / paused play / automatic score playback | `free-play` | Soft blue sparkle, 5 particles; no judgement |
| 5 correct hits in a row | `streak-5` | Gold 24-particle celebration |
| 10 correct hits | `streak-10` | Violet 40-particle celebration |
| 25, 50, 75… correct hits | `streak-25` | Large gold 60-particle celebration |

All other correct hits use `correct`. Partially played chords emit neither
a success nor a failure until the chord is complete or an unrelated note is
pressed. In reading practice an incorrect octave counts as a mistake.
The visible streak is session-local and deliberately **not** persisted as a
score; scoring should be integrated separately.

## Rendering and limits

- Native: Hanabi GPU particles in the shared Camera2d scene; emitted just above
  the strike line. The UI accent remains for clear key feedback.
- Web: WebGL2-safe UI particles; no compute shader or WebGPU requirement.
- Shared CPU particles are capped at 600; at most 32 bursts are sent per frame.
  Native GPU instances are automatically despawned after 1.3 seconds.
- `P` / Settings → Sparkles disables feedback effects. Importing a song,
  returning to the menu, and changing exercise reset streaks.
- The colors and particle counts live in `scene/effects.rs` and can be tuned
  independently of music evaluation.

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
that do and do not match the current score. Confirm the same colors and HUD in
native desktop and in the default browser build.

**Caveat:** Hanabi currently requires WebGPU on WASM; enabling it directly in
the shipped WebGL2 application would break browsers without WebGPU. The default
web build therefore deliberately does not instantiate Hanabi.
