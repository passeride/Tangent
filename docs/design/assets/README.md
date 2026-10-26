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

## Bravura music font — 2026-10-25

- File: `crates/tangent-player/assets/Bravura.otf` (unmodified).
- Source: Steinberg Media Technologies GmbH, SMuFL reference font.
- Pinned commit: `37b194378b710cc40e406ab6c4b07608bb9548ae`.
- URL: https://github.com/steinbergmedia/bravura/blob/37b194378b710cc40e406ab6c4b07608bb9548ae/redist/otf/Bravura.otf
- Copyright and SIL OFL 1.1: `licenses/Bravura-OFL.txt`; reserved name Bravura.
- Embedded directly in Bevy, no system-font fallback or remote font request.
- Usage: true G/F clefs, transparent staff watermarks, accidentals/rests/whole heads.

## First light MusicXML — 2026-10-25

Original complete piano study authored here with AI assistance, CC0 1.0 for the
music sample only. `scripts/generate_study.py` reproduces it. Source score and
provenance/full dedication: `web/piano/samples`. No MIDI transcription is involved.
The prior Mutopia MIDI was retired at the owner's request; its record is retained
in `docs/retired-midi-sample.md` and git history.

Bravura SHA-256: `cdf0f893ee1fdb64b7f6713d71ee0dcfc349c0ac01429a8e451b01a9e79f5f3b`.

## Noto Sans UI font — 2026-10-26

- File: `crates/tangent-player/assets/NotoSans-Regular.ttf` (unmodified; 569,208 bytes).
- Source: The Noto Project Authors, https://github.com/notofonts/noto-fonts.
- Pinned source commit: `ffebf8c1ee449e544955a7e813c54f9b73848eac`.
- Git blob SHA-1: `d55220958aa51eeeb85048d746eabe43d2cd9f14`.
- Upstream file: https://github.com/notofonts/noto-fonts/blob/ffebf8c1ee449e544955a7e813c54f9b73848eac/hinted/ttf/NotoSans/NotoSans-Regular.ttf
- SIL Open Font License 1.1 (unmodified font): `licenses/NotoSans-OFL.txt`.
- Embedded directly in Bevy for native and WASM. Interface text uses Noto Sans while notation glyphs continue to use Bravura.
- Covers Latin accented text (e.g. `Für Elise`, `Lettre à Élise`, Norwegian `æøå`), Greek and Cyrillic. Not intended as complete Unicode/CJK/emoji coverage.
