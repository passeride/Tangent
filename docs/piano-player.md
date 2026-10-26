# Browser piano and song playback

This is the first interactive application slice, built on the existing importer.
The complete Bevy UI/wgpu application runs in WASM. Web Audio, file access,
MIDI and fullscreen are browser capabilities; no HTML menus are used.
See [ADR-0005](adr/0005-one-bevy-gpu-surface.md) for the current boundaries.

## Run locally

Use the pinned Rust toolchain and wasm-bindgen CLI 0.2.129:

```sh
cargo build -p tangent-player --target wasm32-unknown-unknown --release --locked
wasm-bindgen target/wasm32-unknown-unknown/release/tangent_player.wasm --target web --out-dir web/piano/pkg
npm ci --prefix web/piano
python3 -m http.server 8080 --directory web/piano
```

Open http://localhost:8080. Press Play, tap a piano key or use A–K to activate
sound. Import a MIDI file with the Import MIDI control. Files are processed locally and saved in the SQLite music library. They are not uploaded. The distribution workflow now includes GitHub Pages deployment; see
[distribution status/setup](distribution.md). GitHub Actions produces a `tangent-piano-web` artifact containing the
static app and generated WASM; serve it over HTTP rather than opening file URLs.

## Scope

- 49/61/88 keys; pointer, touch, keyboard and optional Web MIDI note input.
- Original MusicXML piano study and local MIDI type 0/1 or MusicXML import.
- Audio-clock transport, play/pause/restart/seek and optional song sound.
- Falling blocks, rotated staff blocks with ledger lines, held falling noteheads,
  stationary grand-staff reading view with paged playhead. Staff lines are also
  present behind the falling noteheads.
- Three visual themes, two synthesized piano profiles, volume and sparkles.
- Velocity, file sustain/controller/bend handling and bounded voices. Hardware
  note input is supported; hardware pedals/controllers are not yet interpreted.
- Percussion channels are silent and omitted from the piano render window.

Imported MIDI is a **performance**, not authored notation. Sharp spelling and
symbol duration categories are approximate. MusicXML now has an explicit initial
import subset; a full score engraver is not implemented. Program changes are retained in the performance stream
but all melodic channels use the selected piano. Pressure, SysEx, bank/program
instruments, RPN tuning/bend-range changes and pedal-aware scoring are not rendered.
Seeking re-attacks currently sounding voices with fresh envelopes. The 100 ms
lookahead can underrun under heavy main-thread stalls. It is not calibrated to
speaker/display latency. Ordinary browser permissions and Web MIDI support apply.

## Verification

```sh
cargo fmt --all -- --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo build -p tangent-player --target wasm32-unknown-unknown --release --locked
wasm-bindgen target/wasm32-unknown-unknown/release/tangent_player.wasm --target web --out-dir web/piano/pkg
npm ci --prefix web/piano
cd web/piano && npx playwright install chromium
npm test
```

The browser test starts its own HTTP server and Chromium, loads the real WASM
and checks audio energy rather than only counting calls to mocked audio methods.
Physical sound quality/latency and a hardware MIDI device require manual checks;
an analyser signal alone does not prove a particular speaker produced sound.
Native Cargo tests do not verify physical GPU/audio/MIDI behavior.

## Verification record — 2026-10-24

27 workspace Rust tests pass, together with formatting and warnings-denied Clippy.
The release WASM build and real Chromium piano/import smoke tests pass. Audio
checks include a nonzero analyser signal, FIFO repeated keys, sustain release,
silent state reconstruction at seek and cancellation of future voices. Desktop,
notation and narrow-layout screenshots were inspected. Windows CI is configured;
no Windows UI, physical MIDI piano or real-speaker latency result is claimed.

## Single-surface UI update — 2026-10-24

All menus, piano keys, notes, staff lines, progress, settings and help are Bevy UI
inside one full-window GPU canvas. The former Canvas 2D modules are removed.
The shell has only that canvas, a hidden OS file picker and a startup-error fallback.
The renderer uses wgpu with WebGL2; unsupported GPUs/browsers show a startup error.

Space plays/pauses; Left/Right seek 5s (Shift 15s); Up/Down adjust playback speed by 5 percentage points;
Home/End; F11 fullscreen; F1 settings; Esc closes settings; V view; B theme;
M song sound; P sparkles; X piano; R range; Ctrl+O import; Ctrl+B XML demo; Ctrl+M MIDI.
A W S E D F T G Y H U J K remain the piano keys.

`npm test --prefix web/piano` now loads the actual Bevy GPU application and tests
in-canvas controls, staff-note mode, transport/volume shortcuts, fullscreen entry/
exit, MIDI import and viewport containment. The packaged-site smoke tests
`/Tangent/` paths. Software GPU rendering in CI does not establish physical
hardware performance or sound latency.

Verification for this update: 29 native Rust tests, native and WASM warnings-denied
Clippy, release WASM build, real Bevy/Chromium tests, packaged `/Tangent/` site and
embedded desktop launcher smoke pass locally. Fullscreen entry and exit, hidden
HTML controls, OS file chooser and black-key pointer input are included.

The bootstrap probes WebGL2 on a detached canvas before initializing Bevy, and
reports asynchronous renderer failures. A browser test simulates unavailable
WebGL2 and verifies a visible explanation instead of an endless loading screen.

## Pause screen and MusicXML

The corner gear, Esc or F1 opens a pause screen and stops song/live sound.
The pause overlay now contains Resume, restart/seek where applicable,
quick view/audio, Settings, Library and Main. Settings is a separate categorized
screen with Audio & MIDI, Visuals, Practice, Effects, Accessibility (planned),
and Data & About. Back to pause never resumes playback. Space resumes only from
the pause overlay; Esc closes Pause while leaving transport stopped. Playing shows
only a gear and the timeline, with shortcuts available. See [MusicXML import](musicxml-import.md) for the new authored subset.


The MusicXML demo and note display are described in ADR-0007: real Bravura G/F
clefs, translucent clef watermarks on rotated staves, measure/beat guides,
barlines, meter, tempo, repeat signs and first/second houses. The score playhead
uses explicit performance visits when returning to a repeated written measure.

## Main menu and practice

Start at Main; choose Play for the loaded song, FreeForm for live piano, or
Train → Notes/Chords for untimed exercises. Notes require the displayed octave;
chords accept inversions and octave doublings. Release all keys between answers.
Settings opens from Main or the session gear; Main menu in pause returns home.
All menus use the same Bevy GPU canvas. Browser-backed builds omit Exit.
See [ADR-0008](adr/0008-navigation-and-practice.md) for scope and boundaries.

Verification for this update: 44 native tests, formatting, native/WASM
warnings-denied Clippy, release WASM, real Chromium navigation/practice/audio
smoke, packaged `/Tangent/` smoke and embedded Linux launcher smoke pass.
Actual main-menu, Train and note-exercise screenshots are reproducible through
`web/piano/capture.mjs`. Physical MIDI remains unverified.

## Local database

Settings are restored from SQLite on this browser/device. All setting changes
are saved automatically; the bottom of Settings shows saving failures. Export
database downloads `tangent.sqlite`, readable with normal SQLite tools. Stored
settings: volume, theme, view, piano sound, range, effects and song sound.
Music files, scores and exercise progress are not yet recorded. The schema has
empty practice_sessions and scores tables for later work. See ADR-0009.

The local launcher uses port 38765 so browser storage survives restarts. Pages
and localhost have separate data, as do different browser profiles. TANGENT_PORT
can override the port; changing it selects a different storage origin. Clearing
site data removes the local copy. Keep exported backups when needed.

SQLite verification: 45 Rust tests, native/WASM warnings-denied Clippy, real
browser restart and settings restore, SQLite export/integrity/FKs, failed-write
preservation, concurrent connections, storage-unavailable playback, packaged
Pages SQLite loading and embedded launcher asset/port smoke pass.

## MIDI output and reading practice

Settings → Audio & MIDI → Rescan / connect MIDI discovers outputs; Output cycles Computer audio and devices.
Select the P-45/Digital Piano port to send playback to its speakers. Local Control
must stay On: hardware input is displayed/evaluated but not echoed. Computer and
onscreen keys are sent on channel 16. The piano's volume/timbre controls apply to
external output; Tangent volume/profile controls affect computer synthesis.
Output selection is persisted by device name in SQLite (ADR-0016). On startup
or after rescan, a uniquely matching available output reconnects automatically;
if missing or ambiguous, Computer audio remains active while the saved name is
retained. Web MIDI requires already-granted permission to reconnect without a
gesture; otherwise use Settings → Audio & MIDI → Rescan / connect MIDI. The
Bevy dropdown includes Computer audio and available devices, with pagination.
Pause, seek, switching outputs and exiting silence the MIDI destination.

Song notes glow around unfilled keys; your actual key presses fill them. These
independent cues also work with sparkles disabled. Train → Notes has treble, bass
and mixed clefs, stepwise/mixed/chromatic levels and an optional answer hint. Read
the staff and match its exact octave; correct notes remain until full release.
Advanced key signatures, score engraving and progress/scoring remain future work.
See ADR-0015 and `web/piano/practice-midi-smoke.mjs` for scope and verification.

## UI v2 and practice integration — 2026-10-25

The approved UI v2 main/pause/settings structure is implemented. Play has compact
Play/Pause, title, time and output context; Tab/Shift+Tab and Enter navigate menu
controls with a visible focus border. Native Main has Exit through Bevy shutdown.

Train → Hear: On automatically auditions each new target without answering;
Hear: Off stops it. The choice is saved (ADR-0019). Your actual held pitch appears
in the pressed-key colour beside the target, on a continuous grand staff; a
single-note response includes semitone distance. Releasing a wrong finger
rechecks any remaining held answer. Clef/difficulty persist between sessions.
Settings includes saved Soft/Full/Punchy effect intensity and Reduced motion;
the latter removes bursts/rings while preserving key and notation feedback.
Song target identity prevents repeated presses inflating cosmetic streaks.
See [ADR-0017](adr/0017-practice-reference-and-motion.md).

Validation: all 74 workspace tests, formatting, native/WASM warnings-denied
Clippy, native release executable/self-test, release WASM, complete real Bevy
browser tests (including actual reference audio energy, MIDI reference on/off,
no self-answer/echo, rapid held-note correction, saved practice/motion preferences),
MusicXML/MIDI browser import and packaged `/Tangent/` smoke pass locally. Actual
app screenshots are refreshed with capture.mjs. Upstream Hanabi/transitive
notices are included. Physical P-45 speakers/latency, native GPU appearance and
4K hardware performance remain manual verification, not claims of these tests.

## Scores and adaptive practice — 2026-10-26

[Practice and progress](practice-progress.md) describes 12-task scored sessions,
results/task details, saved recent history, adaptive review and expanded chord
pools. Song scores use unique playable pitch/onset targets, grouping layered
unisons; percentages are pitch coverage rather than calibrated rhythm accuracy.
SQLite v5 migrates existing data and retries failed revisioned checkpoints.

Validation: 82 workspace Rust tests, native/WASM warnings-denied Clippy, native
release/self-test, release WASM, all browser tests, focused history/profile replay,
save-failure recovery, song restart/pause/unison scoring, non-overlapping mobile
touch controls and packaged-site smoke pass locally. Native packages include the
practice guide. Screenshots are from the actual Bevy app; capture.mjs supports
TANGENT_CAPTURE_PROGRESS=1 for reproducible scored results and history.

The local executor reproduced the pinned toolchain's multi-CGU archive/link
failures. Local verification used CARGO_PROFILE_DEV_CODEGEN_UNITS=1 and
CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_CODEGEN_UNITS=1; these are build workarounds,
not runtime scoring options. The pinned toolchain was not changed. Physical P-45
sound, hardware latency and Windows/native GPU behavior still require target-device
checks; a Linux executable/import smoke is not that verification.

## Playback tempo

Open the pause gear in Play to select 25–200% speed in 5% steps. The percentage
button resets to 100%; Resume continues from the same song position. The HUD
shows the multiplier and the saved choice survives reload and music imports.
Notation, synthesized song voices and external MIDI events follow the same
source-song timeline while pitches stay unchanged. Existing MusicXML tempo
changes are scaled too. Seeking moves source-song seconds. Reading exercises
and live keyboard notes retain their normal behavior. See [ADR-0020](adr/0020-variable-playback-speed.md).

Verification: 84 workspace Rust tests, native/WASM warnings-denied Clippy,
release WASM and native executable self-test, source-time browser checks at
60/70/120%, scaled MIDI note-on/off spacing, persisted settings/import retention,
mobile pause bounds and packaged `/Tangent/` playback. The clock mapping tests
are in `web/piano/playback-clock.test.mjs`; real-app acceptance is in
`web/piano/speed-smoke.mjs`. Physical MIDI latency remains a device check.
