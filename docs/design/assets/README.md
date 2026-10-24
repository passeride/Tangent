# Design asset register

Store original reusable design files here: icons, palettes, editable artwork and
other source assets. The app draws original Bevy UI geometry and uses
original generated study events; Bevy embeds its Fira Mono UI font under SIL OFL; no SoundFont or recording is
bundled. These are newly authored assets, not assets moved from the legacy fork.

| File / identifier | Purpose | Creator / source URL | License or permission | Changes / assistance | Ready for runtime? |
| --- | --- | --- | --- | --- | --- |
| `crates/tangent-player/src/scene/draw.rs` identity | In-engine Tangent identity | Newly drawn to owner description | Original project work; no project license selected | AI-assisted geometry/text | Yes |
| `crates/tangent-player/src/scene/draw.rs` | Noteheads, flags, clefs | Newly drawn geometry | Original project work; no project license selected | AI-assisted Bevy UI shapes | Yes |
| `crates/tangent-player/src/scene/theme.rs` | Aurora/Ember/Paper palettes | Newly authored profiles | Original project work; no project license selected | AI assistance | Yes |
| `crates/tangent-player/src/import.rs` study | First-use demonstration | Original synthetic 40-note sequence | Original project work; no external music used | AI assistance | Yes |
| Owner's 2026-10-24 sketch photo | Requirements reference | Owner attachment | Supplied as design reference, not redistributed | Hierarchy interpreted in design brief | Not bundled |

Record provenance before an asset is used. For third-party files, include the
exact release or content hash, applicable license file and attribution obligations.
For original or AI-assisted work, record how it was created and any reference
material. Unknown provenance means it is not ready for the runtime bundle.

Do not move assets from another repository merely because it also uses the name
Tangent. References and shipped assets are different categories; a reference URL
is not permission to redistribute its contents.

## Test music and screenshots — 2026-10-24

- `web/piano/samples/bach-prelude-bwv924a.mid`: original unmodified Mutopia MIDI,
  Bach / typesetting by jeff covey, CC BY-SA 3.0. Exact source, checksum and credit:
  [sample attribution](../../../web/piano/samples/ATTRIBUTION.md). Runtime ready.
- `docs/screenshots/piano-{blocks,staff,notes,score}.png`: real Chromium captures
  of this repository's app with its original study, newly created for README.
  Reproduce via `web/piano/capture.mjs`; no external music/photo is embedded.

## Bevy surface update — 2026-10-24

The former Canvas paths in keyboard/falling/score modules have been replaced by
original Bevy UI geometry in `crates/tangent-player/src/scene`. Palettes now live
in its theme module; the JS instrument profiles remain audio-only. Screenshots
are recaptured from the Bevy/wgpu version. Bevy embeds Fira Mono's subset font:
Mozilla Foundation and Telefonica S.A., SIL OFL 1.1; see
[full font license](../../../licenses/FiraMono-OFL.txt).
