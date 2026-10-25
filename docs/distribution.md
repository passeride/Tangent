# Run Tangent

## Web

GitHub Pages is published by `.github/workflows/check.yml` after web and native
checks pass. The intended URL is https://passeride.github.io/Tangent/.
First-time setup: Settings → Pages → Build and deployment → Source → GitHub Actions.
The deployment requires that repository setting; the GitHub connector cannot
change it. A workflow file alone does not prove the site is live.

## Desktop packages

Download the artifact for your platform from the latest successful **Build and
publish Tangent** run in https://github.com/passeride/Tangent/actions.
Extract the artifact ZIP, then extract the platform archive inside it.

| System | Artifact | Start |
| --- | --- | --- |
| Windows x64 | `Tangent-x86_64-pc-windows-msvc` | `Tangent.exe` |
| Windows ARM64 | `Tangent-aarch64-pc-windows-msvc` | `Tangent.exe` |
| Linux x64 | `Tangent-x86_64-unknown-linux-gnu` | `./tangent` |
| Linux ARM64 | `Tangent-aarch64-unknown-linux-gnu` | `./tangent` |
| macOS Intel | `Tangent-x86_64-apple-darwin` | `Tangent.app` |
| macOS Apple Silicon | `Tangent-aarch64-apple-darwin` | `Tangent.app` |

The launcher contains the web app and WASM, starts a read-only server on a random
**127.0.0.1** port, and opens your default browser. It works without a network
connection, Python, Node or Rust installed. A browser remains required. This is
packaging for the current browser adapter, not a native Bevy GPU/audio build.
There are no Android/iOS app packages; use the web version on those devices.

Keep the launcher running while using the app. On Windows/Linux, close its console
or press Ctrl+C to stop. On macOS the .app launches in the background; to stop it,
quit the Tangent process in Activity Monitor, or run
`Tangent.app/Contents/MacOS/Tangent` from Terminal and stop with Ctrl+C.
Builds are currently unsigned and macOS builds are not notarized. OS launch checks
may therefore require a local owner decision. No signing secrets are configured.
SHA-256 checksum files accompany every archive.

## Included music

The player starts with **First light**, a complete MusicXML piano study. Press
Space or choose **XML demo** in the pause screen to reload it. It has 16 written
measures, 23 visits with first/second endings, 130 performed notes and lasts about
52 seconds. The score includes two staves, meter and tempo changes. The original
study is CC0 1.0; provenance and the full dedication ship in `samples/`. Bravura's
SIL OFL font notice is embedded in the distribution. The prior MIDI sample has
been retired (historical record: `docs/retired-midi-sample.md`).

## Build the same package locally

```sh
cargo build -p tangent-player --target wasm32-unknown-unknown --release --locked
wasm-bindgen target/wasm32-unknown-unknown/release/tangent_player.wasm --target web --out-dir web/piano/pkg
python3 scripts/package_web.py
TANGENT_WEB_DIST="$PWD/dist/piano" cargo build -p tangent-desktop --release --locked
python3 scripts/smoke_desktop.py target/release/tangent
```

The environment assignment shown is for POSIX shells. In PowerShell set
`$env:TANGENT_WEB_DIST = "$PWD/dist/piano"` before Cargo, and test `tangent.exe`.
Builds without that environment variable support workspace checking/tests but
produce a launcher that explicitly reports missing packaged assets at startup.
Production packaging refuses a missing WASM or test-music asset.
