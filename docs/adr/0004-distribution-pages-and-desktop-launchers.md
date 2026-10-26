# ADR-0004: One tested web distribution for Pages and desktop launchers

- Status: Accepted; random-port decision superseded by [ADR-0009](0009-local-sqlite-storage.md)
- Created / decision date: 2026-10-24
- Delivery: Packaging implemented; CI results and live deployment tracked separately
- Related: [ADR-0003](0003-performance-model-and-piano-web-slice.md), [distribution guide](../distribution.md)

## Context

The owner requested README screenshots, downloadable platform builds, GitHub Pages
hosting and a Creative Commons MIDI test piece. The current sound/visual adapter
is browser based. A library build alone would not give desktop testers a playable
application; introducing a second UI framework would duplicate the current slice.

## Decision

Build the Rust/WASM piano once on Linux, run the real browser tests, and assemble
an explicit static distribution with `scripts/package_web.py`. Use relative
module/sample URLs, tested beneath `/Tangent/`, so project Pages and local hosting
share identical content. The distribution includes all MIDI attribution/license
files and dependency notices. It excludes Node tooling and unrelated source files.

Publish Pages only for main/manual main runs, after the web and native jobs pass,
using GitHub's upload-pages-artifact/deploy-pages workflow and the github-pages
environment. Give only that deployment job `pages:write` and `id-token:write`;
ordinary build jobs retain `contents:read`. PRs build/test but cannot deploy.
GitHub's Pages source setting must be enabled once by an authenticated maintainer;
workflow creation alone is not site enablement.

Add `tangent-desktop`, a standard-library-only native launcher. Its build script
embeds the prepared web distribution at compile time when TANGENT_WEB_DIST is
provided. Release packaging requires the player WASM, sample and credits. A build
without assets supports checking/tests but fails clearly rather than serving an
empty application. The launcher binds only loopback on a random port, serves
exact embedded paths with correct WASM MIME, restricts Host, bounds headers and
concurrent connections, and opens the default browser without a shell command.
It has no upload/write endpoint. `--no-open` supports automated checks.

Build x64/ARM64 Windows and Linux, plus Intel/Apple Silicon macOS, on matching
GitHub runners. Run workspace validation and exercise the actual compiled launcher
on each runner before packaging ZIP/tar.gz archives and checksum files. macOS
packages contain a small .app bundle. These are browser-backed desktop launchers,
not a claim of native Bevy rendering, mobile app packages, signing or notarization.

Bundle the unmodified Mutopia Prelude BWV 924a MIDI, attributed to jeff covey,
CC BY-SA 3.0, with source URL, hash, full license and visible credit/download links.
Use a user-clicked sample loader, not network-dependent startup or autoplay.
Screenshots use Tangent's original study rather than the third-party music.

## Consequences and alternatives

All supported packages test the same UI/audio/model boundary and can run offline
in an installed browser. No Electron/Tauri/WebView runtime, OS audio backend or
new registry dependency is introduced. The native launcher remains a separate
platform adapter; song/import/music code does not depend on HTTP or packaging.

The launcher must keep running while the browser is in use. The current macOS
.app has no menu/quit UI; process lifetime is documented. Adding a managed native
window/lifecycle or a real Bevy renderer is a later decision, not hidden here.
Signing/notarization requires owner-managed certificates. Browser MIDI/audio
capabilities still apply. A green Linux test cannot substitute for platform CI.

Alternatives: distribute only a raw WASM ZIP (requires users to supply a server),
ship native libraries without a UI (not useful for testing the piano), introduce
another desktop UI framework, or prematurely implement a second renderer/audio
backend. The small launcher addresses today's download requirement while retaining
the accepted architecture and showing its actual limits.

## Independent references

- https://docs.github.com/en/pages/getting-started-with-github-pages/using-custom-workflows-with-github-pages
- https://github.com/actions/runner-images
- https://www.mutopiaproject.org/cgibin/piece-info.cgi?id=1028
- https://creativecommons.org/licenses/by-sa/3.0/

No legacy code or asset is used. See [PROVENANCE](../../PROVENANCE.md).

## Packaging refinement — 2026-10-26

The earlier nested platform archive + GitHub artifact ZIP was removed for native builds. The GitHub artifact ZIP itself now contains just the runnable Windows executable or Linux binary; macOS retains the minimal `.app` bundle. A manual tester script and user documentation are no longer packaged. Native CI smoke tests are retained. Windows embeds an original Tangent icon in the executable using the Windows resource API (no new Rust dependency), and a `--licenses` entrypoint exposes embedded third-party notices even for single-file distribution. GitHub supplies the artifact checksum. This is a distribution-only refinement; it does not alter the native Bevy runtime or the Pages build.
