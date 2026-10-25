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

The desktop executable runs a **native Bevy/wgpu window**, with native OS MIDI
inputs (midir), an audio device mixer (rodio), and a local bundled SQLite database.
It does not launch a browser or a local HTTP server, and does not need Node,
Python or Rust installed. Web and desktop share the same Bevy scene and piano core.
There are no Android/iOS desktop packages; use the web version there.

Close the native window to exit Tangent. Desktop playback uses a lightweight
harmonic instrument; web sample/audio synthesis and native synthesis are not
yet identical. On Linux, Ctrl+O uses the system `zenity` file picker if installed.
Windows uses a PowerShell file picker and macOS the system Open panel via AppleScript.
MIDI input ports are enumerated on launch; Ctrl+M rescans. Actual hardware
latency and device hot-plugging still need physical testing. Desktop binaries are
unsigned and macOS is not notarized; OS launch checks may require owner approval.
SHA-256 checksum files accompany every archive.

## Included music

The player starts with **First light**, a complete MusicXML piano study. Press
Space or choose **XML demo** in the pause screen to reload it. It has 16 written
measures, 23 visits with first/second endings, 130 performed notes and lasts about
52 seconds. The score includes two staves, meter and tempo changes. The original
study is CC0 1.0; provenance and the full dedication ship in `samples/`. Bravura's
SIL OFL font notice is embedded in the distribution. The prior MIDI sample has
been retired (historical record: `docs/retired-midi-sample.md`).

## Build the native app locally

```sh
cargo build -p tangent-desktop --release
cargo run -p tangent-desktop --release
python3 scripts/smoke_desktop.py target/release/tangent
```

On Windows use `target/release/tangent.exe`. Bevy initializes the platform
graphics backend directly (DX12/Vulkan/Metal), not WebGL or a browser canvas.

## Local SQLite and native performance

Desktop settings, practice sessions and future score tables use the schema from
ADR-0009 in a native `tangent.sqlite` file under the user application data
folder: `%LOCALAPPDATA%/Tangent` on Windows, `~/Library/Application Support/Tangent`
on macOS and `$XDG_DATA_HOME/tangent` (default `~/.local/share/tangent`)
on Linux. Settings → Export database creates a timestamped SQLite backup in
`~/Downloads` (or the Windows user's Downloads folder).

The native audio/MIDI/SQLite worker uses a separate thread; file bytes do not
travel through JSON. The render thread reuses the same Bevy entity pools as
the browser player. The native audio scheduler uses a 32ms lookahead and a
64-voice ceiling. This is a performance-focused first desktop implementation,
not a claim of measured latency. Tune buffers and voices after device benchmarks.

Web storage remains browser-local and separate from native storage.

### Review regression checks

- Hold notes, rescan MIDI with Ctrl+M, release and play again. Held notes must
  clear, and exclusive Windows MIDI ports must reopen without restarting.
- Adjust volume while a note sounds, then play another note at the same velocity.
  Existing and new voices must use the same volume scale.
- Native imports enforce the 16 MiB limit on bytes read as well as metadata;
  automated tests exercise an unbounded input reader without opening devices.
