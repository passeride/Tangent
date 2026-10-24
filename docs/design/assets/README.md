# Design asset register

Store original reusable design files here: icons, palettes, editable artwork and
other source assets. The first app draws original SVG/Canvas geometry and uses
original generated study events; no external font, SoundFont or recording is
bundled. These are newly authored assets, not assets moved from the legacy fork.

| File / identifier | Purpose | Creator / source URL | License or permission | Changes / assistance | Ready for runtime? |
| --- | --- | --- | --- | --- | --- |
| `web/piano/index.html` mark | Circle with a tangent | Newly drawn to owner description | Original project work; no project license selected | AI-assisted SVG geometry | Yes |
| `web/piano/falling.js`, `score.js` | Noteheads, flags, clefs | Newly drawn geometry | Original project work; no project license selected | AI-assisted Canvas paths | Yes |
| `web/piano/themes.js` | Aurora/Ember/Paper palettes | Newly authored profiles | Original project work; no project license selected | AI assistance | Yes |
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
