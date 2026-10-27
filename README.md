# Tangent

Piano practice, note reading and music playback, built around **Bevy** with
**web as a first-class deployment target**.

Tangent is a new project with its own repository and history. The goal is an
independent implementation, developed through a documented clean room process,
without importing Neothesia or code from its forks.


[Play in the browser](https://passeride.github.io/Tangent/) ·
[Download platform builds](https://github.com/passeride/Tangent/actions/workflows/check.yml) ·
[Run/build guide](docs/distribution.md)

The browser link becomes available after the first successful Pages deployment.
The bundled **First light** MusicXML piano study includes bass/treble staves,
16 written measures, first/second endings, repeats, meter and tempo changes.
Choose **Play** from the new main menu, then press **Space** to play, **V** for the note views, or choose **Load sample** in **Settings → Data & About**. The original demo is [CC0 1.0](web/piano/samples/ATTRIBUTION.md).

During play, only the corner gear and timeline remain. The pause overlay has
Resume, transport, playback speed (25–200%), quick view/audio, Settings and navigation.
Use Slower/Faster in Pause to change by 5%; the percentage button resets to 100%.
Speed is saved locally and leaves note pitches unchanged. Full settings are
organized into Audio & MIDI, Visuals, Practice, Effects, Accessibility (planned),
and Data & About. Opening Pause stops playback and releases live notes; returning
from Settings stays paused until Resume.

The UI v2 branch opens at a responsive, action-card main menu: **Play**, **Library**, **Train**, **FreeForm**, and
**Settings**. Train offers treble/bass note reading with three levels, optional hints and diatonic chord exercises; FreeForm
keeps song playback silent. Return home from the corner-gear pause screen.

Pause in Play now includes **Set A / Set B / Clear loop** and explicit **Targets**
part selection. Repeat a passage at the saved playback speed; drag A/B on the
bottom timeline while paused. Each repetition records its own pitch-coverage
attempt. Completed results can suggest **Practice this section** from actual
missed targets; retries open paused with the recorded range, part and speed.
Legacy history remains readable with unknown tempo/selection and is excluded
from new comparable-result summaries. See [passage practice](docs/practice-progress.md#passage-practice).

**Sequential Play** waits at each note/chord until you play the correct keys.
Enable **Sequential** in Pause or Settings → Practice. Set **Early input** to
0/100/250/500 ms (default 250); late input waits as long as needed. Repeated notes
require release and replay. The mode/window are saved, and guided results remain
separate from continuous playback. See [the guide](docs/design/sequential-play.md).

Settings → Visuals now offers saved **Focus / Standard / Arcade** HUD density,
independent of effects. **Fit keys** and out-of-range warnings help keep selected
music visible on the keyboard.

MusicXML loops can **Snap bars** to explicit measure visits. An optional
**Goal** tracks three complete attempts at 80% pitch coverage; autoplay never
advances it. Results show matched-note change against a compatible previous run.

Settings → Accessibility → **Keybindings** has persistent Keyboard/Piano columns
for application actions and computer piano keys. Choose a low/high/custom MIDI
control-key, then hold it with a command note or tap it before the next note
within two seconds. Piano commands default Off; binding capture rejects conflicts.
See [input binding architecture](docs/adr/0024-configurable-keyboard-and-piano-bindings.md).

Settings → Visuals now includes **Hands** (Teach Left/Right, Stop/review, per-note
correction and explicit save) and a **Theme editor** with persistent RGBA colours,
JSON import/export and hand colours. Demonstrations accept only unambiguous
pitch/onset matches; unknown notes stay unassigned. Library row backgrounds also
support double-click to Open. See [ADR-0025](docs/adr/0025-personal-themes-and-demonstrated-hands.md).

The horizontal MusicXML score now groups short notes with shared beams, including
mixed 16ths/32nds and chord stems. Written groups take priority; missing groups
follow the meter. See [grouped notes](docs/design/grouped-notes.md).

Library → Edit → **Find cover** now offers reviewed MusicBrainz/Cover Art Archive
artwork and metadata. Approved covers save locally for offline use; your local
title/composer and original MIDI/MusicXML stay intact. See [the guide](docs/music-library.md#album-covers-and-external-metadata).

Library → **Folder** now saves a music folder and scans subfolders at startup
or on demand. Browser rescans follow folder permissions. Manual Import accepts
up to 500 files; background catalog imports keep your active piece intact.

## Screenshots

Actual running app, captured with Chromium at 1440 px width using Tangent's
original demonstration study. These are app screenshots, not design mockups.

![Main menu](docs/screenshots/main-menu.png)

![Note practice](docs/screenshots/train-notes.png)

| Scored session | Recent progress |
| --- | --- |
| ![Session results](docs/screenshots/practice-results.png) | ![Progress history](docs/screenshots/practice-progress.png) |

![Piano with falling blocks](docs/screenshots/piano-blocks.png)

| Staff lanes with ledger lines | Falling noteheads |
| --- | --- |
| ![Staff lanes](docs/screenshots/piano-staff.png) | ![Falling notes](docs/screenshots/piano-notes.png) |

![Stationary sheet reading view with a moving playhead](docs/screenshots/piano-score.png)

![MusicXML first and second endings](docs/screenshots/piano-endings.png)

![Previous combined pause/settings UI, captured before the UI v2 branch](docs/screenshots/piano-pause.png)

The screenshots above are baseline captures from the previous UI; they have not
been regenerated for UI v2. See [UI/UX v2 design](docs/design/tangent-ui-v2.md).

Reproduce with `node web/piano/capture.mjs` after building the WASM package.
New passage/HUD/goal captures: `TANGENT_CAPTURE_PRODUCT=1 node web/piano/capture.mjs`.

## Platform builds

The web player remains available through Pages. A separate **native Bevy desktop
binary** now targets Windows x64/ARM64, Linux x64/ARM64 and macOS Intel/Apple
Silicon. It uses the OS GPU backend through wgpu and opens its own window,
not a browser wrapper. Desktop MIDI (midir), audio (rodio) and SQLite
(rusqlite with bundled SQLite) run natively. See
[desktop builds and limitations](docs/distribution.md). Android/iOS use the web version.

Choose **Library** to import several files, search title/composer, edit metadata,
sort/filter, reopen and remove saved pieces. Windows packages include a short
[P-45 test](docs/test-p45.md) and PowerShell report starter.
See [music library](docs/music-library.md) for limits and backup behavior.

To hear playback through your piano, open **Settings → Audio & MIDI**, select
**Rescan / connect MIDI**, then choose your piano from the **MIDI output dropdown**.
The selected device name is saved in local SQLite and restored on later launches
when it matches exactly one connected port. Browser reconnection requires prior MIDI
permission; if the device is unavailable, Computer audio is used until reconnection.
The selected output replaces computer audio; leave the piano's Local Control On. Physical piano input
is not echoed. **Solid keys mean played input; glowing edges mean expected song
notes.** Train → Notes offers treble, bass or both clefs, three levels and Show hint.
The answer stays visible until all keys are released. **Hear target** auditions the note;
your played pitch appears in the pressed-key colour on the staff, with semitone
distance feedback. Effects have Soft/Full/Punchy intensity and a saved reduced-motion setting. See [MIDI and practice details](docs/adr/0015-midi-output-and-reading-practice.md).

## Project status

**Playable web player and native desktop implementation (pending hardware validation).** The Rust workspace has a format-neutral
music/piano model, one Bevy application with in-engine UI/wgpu rendering and a Web Audio adapter.
Play with keyboard, pointer or touch; import MIDI, switch between four note
views, choose a theme/piano profile and enable or disable sparkles and song audio.

See [piano build/run/test instructions](docs/piano-player.md) and the
[MIDI support contract](docs/midi-import.md). The web version has Web MIDI/Web
Audio and browser-local SQLite. The desktop branch uses the same Bevy GPU UI and
musical core with native keyboard/pointer, multidevice MIDI input, a separate
audio scheduler thread, a local SQLite file and OS file dialogs.
Partwise MusicXML (.xml/.musicxml/.mxl) import is an [explicit subset](docs/musicxml-import.md).
Desktop build and MIDI/audio behavior require physical machine verification,
especially low-latency playing. The native instrument is currently an efficient
harmonic voice, not the same timbre as the browser synthesizer. The local
music library saves original imports locally. Scored practice, adaptive review,
recent history and song pitch-coverage results are delivered; see
[practice and progress](docs/practice-progress.md).

The implementation is not certified as a completed clean room process. Prior
source exposure remains documented in [PROVENANCE.md](PROVENANCE.md).

## Product direction

- Connect a MIDI piano and show the notes being played.
- Practice reading notes in treble and bass clefs, and learn common chords.
- Play MIDI pieces with a virtual keyboard, falling notes and optional notation.
- Offer focused practice and a separate scored performance mode.
- Keep a personal music library, preferences and progress.
- Add MusicXML support with an explicit supported subset and synchronized notation.

These are product goals; the current piano slice implements a subset. Browser MIDI, audio and file
access must be validated on target devices; keyboard and on-screen input should
remain useful when hardware MIDI is unavailable.

## Technical direction

Bevy owns the application, UI, keyboard/pointer input and GPU drawing in one
full-page canvas. The browser adapter supplies audio, file/device capabilities
and fullscreen, with no separate HTML application menus. Musical data, exercise rules and
performance evaluation should be independently testable. Browser audio, MIDI,
file access and persistence need explicit platform boundaries.

The sound/clock boundaries are recorded in ADR-0003; ADR-0005 replaces its
initial Canvas/HTML presentation with a single Bevy GPU surface. Full
notation layout, sampled sound and further formats require separate decisions.

## Documentation

| Area | Purpose |
| --- | --- |
| [Piano player](docs/piano-player.md) | Playable web slice and run/test commands |
| [Practice and progress](docs/practice-progress.md) | Scores, adaptive review and saved session history |
| [Follow-up work](docs/TODO.md) | Remaining implementation and architecture work |
| [MIDI import](docs/midi-import.md) | Implemented scope, limitations and build/test commands |
| [Documentation index](docs/README.md) | Entry point for architecture and design work |
| [Architecture decisions](docs/adr/README.md) | Decision history, statuses and ADR template |
| [Product and visual design](docs/design/README.md) | UI v2, UX product strategy, benchmark, provisional personas, gamification and roadmap |
| [Sketches](docs/design/sketches/README.md) | Wireframes, flows and visual explorations |
| [Design assets](docs/design/assets/README.md) | Source files and asset provenance |
| [Provenance](PROVENANCE.md) | Source boundaries, exposure and future dependency records |
| [Agent guidance](AGENTS.md) | Instructions for automated contributors |

## First milestones

1. Document the requirements and implementation provenance.
2. Prove browser rendering, piano input, sound activation and local persistence.
3. Build a small MIDI playback and virtual-piano slice.
4. Expand into practice, library and scored performance after those foundations work.

The importer, local library, settings persistence and initial piano/practice slices
are delivered. Full notation and advanced practice/performance modes remain open in [the backlog](docs/TODO.md).

## Contributing

Read the relevant design brief and ADR before changing a cross-cutting decision.
Use the templates to explain the problem, alternatives and evidence. Keep proposals
clearly separate from accepted decisions and delivered behavior.

Do not copy implementation code, wrappers, fixtures, shaders or assets from
Neothesia or its forks. Independently published dependencies and assets need their
own license and provenance record. Do not treat a fresh repository or AI context
as proof of source independence.

## Licensing

A project-wide license has not been selected. Do not assume that this repository
is MIT, Apache-2.0 or licensed under the terms of another Tangent project.
Dependencies retain their own licenses: see [THIRD_PARTY.md](THIRD_PARTY.md) and
[license notices](THIRD_PARTY_NOTICES.txt). Bundled fonts and music have individual license and provenance records in
[the asset register](docs/design/assets/README.md). Noto Sans is used for
international song titles and interface text; Bravura is for music notation.

## Player shortcuts

| Action | Shortcut |
| --- | --- |
| Play / pause | Space |
| Seek backward / forward | Left / Right (5s), Shift for 15s |
| Playback speed down / up | Down / Up (5 percentage points; 25–200%) |
| Beginning / end | Home / End |
| Fullscreen | F11 or Settings → Visuals |
| Pause / Settings | Corner gear or Esc/F1 (session); Settings from pause or Main; Space resumes from pause |
| View / theme / piano / range | V / B / X / R |
| Song audio / particles | M / P |
| Import / MusicXML demo / MIDI | Ctrl+O / Ctrl+B / Ctrl+M |

Staff notes and staff blocks both include five-line bass and treble staves,
with ledger strokes outside them. All menus are inside the GPU canvas.


MusicXML's sheet view now uses written measures, real Bravura G/F clefs, meter,
tempo, key signatures, repeat signs and first/second-ending brackets. The playhead
follows repeat visits. Falling staff/notes show faint clef watermarks plus moving
measure/beat guides and endings. MIDI remains an approximate reading view.

The session piano is a procedural, lit 3D keyboard with chamfered keys, key depression and expected-note edge glow, composited into the same Bevy surface. See [3D piano stage](docs/design/3d-piano-stage.md).

Train also includes **Keyboard Explorer**: learn octave/key patterns through exact register transfers, three levels, optional hints/reference audio and saved scored attempts. See [N02](docs/design/keyboard-explorer.md).

**Five Finger Flow** adds ordered five-note up/down patterns, suggested fingers, three position/register levels and saved whole-pattern scores. See [T01](docs/design/five-finger-flow.md).

**Major Scale Run** adds one-octave up/down scales in all 12 major keys, explicit
spellings, suggested fingers and saved whole-scale scores. Three levels offer
right C, left C and all keys. This evaluates pitch/order with release per note;
see [S03](docs/design/major-scale-run.md).

In **Settings → Visuals**, enable the horizontal sheet and the vertical falling
view independently, choose the vertical style, and optionally show pitch names
on blocks. Held notes appear in the played colour on the sheet and staff lanes;
live MIDI spelling is approximate. When both layers are on, a scalable sheet overlays the full-height falling view. Sheet-only mode fits multiple systems on screen.
