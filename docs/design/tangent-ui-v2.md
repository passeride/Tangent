# Tangent UI/UX v2 — screen, flow and interaction specification

- **Status:** Design direction approved 2026-10-25; **main/pause/settings and initial play/practice HUD implemented**. Remaining HUD and features are not implemented.
- **Scope:** Information architecture, menus, play HUD, practice, settings, library and responsive interaction.
- **Applies to:** One shared Bevy/wgpu UI in native desktop and web builds.
- **Visual reference:** Interactive HTML concept reviewed in the design conversation on 2026-10-25. It is a **mockup**, not a running Tangent build, test fixture or implementation asset.
- **Existing contracts:** [Piano player brief](piano-player.md), [ADR-0005](../adr/0005-one-bevy-gpu-surface.md), [ADR-0008](../adr/0008-navigation-and-practice.md), [ADR-0013](../adr/0013-local-music-library.md), [ADR-0015](../adr/0015-midi-output-and-reading-practice.md).
- **Language:** English labels in the application for now. This document is the implementation-facing product brief.

## Objective

Create a calm piano workspace in which the music and keyboard dominate the session. Navigation and settings should be discoverable without turning the playing surface into a configuration panel. Visual feedback should be immediately legible and never claim a performance result that has not been measured.

The existing implementation offers Main, Library, Train, FreeForm, Settings and Session. Its settings/pause view currently mixes transport, audio, display, files and help into a dense grid. UI v2 replaces that *presentation* with a small pause overlay and structured settings, while preserving musical behavior and the existing cross-platform architecture.

### Visual hierarchy

1. **Primary:** notation/falling notes, strike line, keyboard, visible exercise target.
2. **Secondary:** song title, progress, play/pause and device state.
3. **Contextual:** tempo, measure, view controls, volume and active exercise parameters.
4. **Optional:** score, accuracy, streaks, effects and diagnostics **only once their underlying features exist**.

Do not place text or effect overlays over important noteheads, key contacts or the strike line. A focus mode may hide secondary chrome but never the safety-critical device/error state or the music itself.

## Navigation map

~~~mermaid
flowchart TD
    Start[Launch] --> Home[Main menu]
    Home --> Play[Play session]
    Home --> Library[Library]
    Home --> Train[Train selection]
    Home --> Free[FreeForm session]
    Home --> Settings[Settings]
    Library --> Search[Search / filter / select]
    Search --> Play
    Train --> Notes[Note reading session]
    Train --> Chords[Chord training session]
    Play --> Pause[Pause overlay]
    Free --> Pause
    Notes --> Pause
    Chords --> Pause
    Pause --> Resume[Resume current session]
    Pause --> Settings
    Pause --> Home
    Settings --> Return[Return to originating screen]
    Return --> Home
    Return --> Pause
    Resume --> Play
    Resume --> Free
    Resume --> Notes
    Resume --> Chords
~~~

The diagram shows permitted transitions rather than a second, independent playback state machine. In particular:

- **Launch** leads to Main. Offer a prominent **Continue** card only when a real recent piece/session can be reopened; otherwise replace it with **Choose music** or **Try a sample**.
- **Play** uses the currently loaded piece when available, or directs the user to the library/open flow. Do not display a fake recent song.
- **Library** supports search, title/composer metadata, sorting/filtering, multi-file import, open and removal using the existing persistence layer.
- **Train** offers **Notes** and **Chords**. The exercise selection may display a concise summary of the current clef/difficulty; in-session changes remain available.
- **FreeForm** opens a silent-song, live-input session; music fingerprint matches may appear in a small non-blocking recognition panel.
- **Settings opened from Main** returns to Main. **Settings opened during a session** returns to the **paused session/overlay**, not directly to playing. Explicit Resume resumes playback.
- **Pause** does not destroy the session. Restart and Back to Main are separate, clearly labelled actions. Leaving a session releases live notes and stops sound per the existing contracts.
- **Exit** is native-desktop-only when lifecycle support is actually implemented; omit it on web.

## Screens

### 1. Main menu

Use the established Tangent brand/logo at the top-left or as an intentional hero element. The desktop layout has a restrained welcome/continue region and prominent mode tiles; a narrow viewport stacks these elements.

Primary actions:

| Action | Destination | Supporting copy |
| --- | --- | --- |
| Play | Current piece or Library | Play a selected or recently opened piece |
| Library | Music collection | Search, open, import and manage pieces |
| Train | Exercise choice | Notes or chords |
| FreeForm | Live piano | Play freely; recognition feedback when available |
| Settings | Settings root | Audio, MIDI, visuals, practice and data |

Use one strong primary action, not five equally prominent calls to action. Show the live MIDI connection in a compact status chip, but do not claim a device is connected without actual platform status. Continue must only appear with persisted/valid recent data. Preserve the same destinations when there is no MIDI hardware.

### 2. Play session

**Default HUD, unobtrusive and legible:**

- Top-left: piece title and composer when known; otherwise a neutral fallback.
- Top-right: actual MIDI-input/device connection status; compact pause/settings entry.
- Musical area: currently selected view (falling blocks, staff lanes, falling notes or sheet/playhead), remaining uncluttered.
- Bottom: the full on-screen piano, accurate key states, strike line and a compact play/pause + seek/progress bar. Display elapsed and total time.
- Optional, collapsed context strip: current tempo, speed multiplier (only if supported), written measure/beat (only if sourced from MusicXML) and a view selector.
- A compact explanation of solid key vs outlined/glowing key can appear contextually on first run or on demand; never remain as a large banner.

For the initial delivery, keep unsupported features absent or explicitly disabled rather than populating them with invented data. Respect the existing difference between MusicXML written measures and approximate MIDI timing/notation.

**HUD visibility presets (proposed):**

| Preset | Display |
| --- | --- |
| Focus | Music, piano, very small progress/pause and critical state |
| Standard (default) | Focus + title, time and device status |
| Detailed | Standard + supported tempo, measure, view/volume controls |
| Performance (future) | Detailed + verified accuracy, score and streak |

Transport controls need a minimum reachable hit area and keyboard support; don't force users to open Settings for common actions. The playing area remains the primary focus.

### 3. Pause overlay

Opening the gear, Esc or F1 pauses the current session and releases live notes before showing a **compact, centered overlay** above a dimmed/blurred snapshot of the musical scene.

Visible controls, in priority order:

1. **Resume** (dominant).
2. **Restart** and **Back/Forward** (only for a song; not FreeForm).
3. Quick controls: volume when computer audio is active, notation view and optional toggle for song audio.
4. **Settings**, **Library/open music** and **Main menu**.
5. Shortcuts/help through a secondary disclosure.

**Do not put the full settings form in Pause.** Returning from Settings preserves the paused state, position and active mode. Switching MIDI outputs must not silently start playback. If no song is loaded, hide irrelevant seeking controls. Distinguish the low-risk Resume control from destructive/exit-like navigation.

### 4. Library

A search field and modest filters for title/composer, format and sorting sit above an easy-to-scan list/grid. Each item exposes title, composer (or unknown), file type, duration when reliable, and Open. Import is prominent. Metadata edit and Remove are secondary actions; removal follows the current confirmation/data semantics.

Use meaningful empty states:
- Empty library: **Import your first piece**.
- No search results: **No matches; clear filters**.
- Unsupported/broken source: preserve other library entries and show a recoverable error.
- Storage unavailable: explain the local-storage limitation without presenting a fake saved state.

Folders/tags, cloud sync, backup restore, recommendations and score-derived sorting are **future work**; do not render them as working options.

### 5. Train — note reading and chords

The notation should be large and centered with real clefs. Below or beside it, show compact progress and feedback, e.g. completed answers and **Correct — release keys** or **Try again**. Preserve exact octave rules for Notes and existing inversion/doubling behavior for Chords.

- Clef selector: treble, bass or both; difficulty: existing three levels.
- **Hide hints by default**. Showing a hint intentionally reveals information; don't put note names into the challenge header before the answer.
- Display the answered note until all keys are released; only then advance.
- Do not cover the stave with a scoreboard or effects.
- Optional audio feedback and timed/adaptive scoring are proposals, not delivered features.
- In a training session, use a pause/menu button but retain simple contextual clef/level/hint controls.

### 6. FreeForm

Let the instrument occupy the stage. Show the live input, detected chord only when backed by a recognizer, and an unobtrusive **possible music match** row when the fingerprinting engine has a result. Use calibrated confidence before wording a match as definite; otherwise label it as a candidate. Avoid interruptive modals, false accuracy claims or song playback that contradicts FreeForm's current silence contract.

## Settings information architecture

Desktop: persistent left navigation and a content pane; compact viewport: accessible category tabs/stacked subpages. The Settings header identifies the active category and has a clear Back/Close action.

| Category | Controls / information | Availability |
| --- | --- | --- |
| **Audio & MIDI** | Input device and state, rescan, output choice (Computer audio / discovered MIDI outputs), internal synth volume/preset, no-echo explanation | Existing underlying features; new presentation |
| **Visuals & notation** | Four modes, Aurora/Ember/Paper, 49/61/88 keys, sparkles, optional key labels, HUD visibility | Existing modes/themes/ranges/effects; labels/HUD presets proposed |
| **Practice** | Note clef, level, explicit hint, chord training entry, exercise feedback explanation | Core controls existing; global defaults and optional audio feedback proposed |
| **Effects** | Effects enable, intensity, subdued correct/error feedback and future streak presets | Effects toggle existing; intensity, granular presets/streak effects proposed |
| **Accessibility** | Reduce motion, high contrast, UI scale, text descriptions in addition to colour, keyboard focus | Proposed; requires validation |
| **Data & About** | Local SQLite status/export, shortcuts, storage limitations, app/build info, license/credits, backup/restore notice | Export/shortcuts/credits existing; restore proposed |

#### MIDI behavior is not merely cosmetic

- MIDI **input** and **output** are separate choices; the Yamaha P-45 and other physical pianos already sound through their own speakers with Local Control On.
- Song playback to a selected MIDI output replaces internal computer synthesis; **never echo physical input back to the same instrument**.
- App synth volume and piano presets must clearly show when they do not control the external piano's own speaker volume/timbre.
- Output selection is **session-only** today. No automatic reconnection to a stale port index. Show connection/disconnection/error and the **Rescan** action.
- Selection/rescan is a deliberate operation that respects playback pause and cancellation behavior defined in ADR-0015.

#### Save semantics

Existing persistent preferences: view, theme, effects, volume, internal instrument, song-audio setting and key range. Continue using the existing SQLite persistence. New persistent options require a versioned schema/migration and tests; do not pretend the current app already stores them.

Use immediate feedback for safe toggles, but ensure MIDI device changes are explicit, accessible and reversible. Show errors next to the affected control, with a concise recovery path.

## Visual design system

The prototype explores a quiet, dark, piano-oriented interface, not an arcade-style dashboard as a default.

| Token | Proposed value / behavior |
| --- | --- |
| Background | Deep ink/navy, approx. #090F18 |
| Panels | Slightly lighter navy, approx. #111B29 / #152334 |
| Primary text | Soft white, approx. #EDF3F6 |
| Secondary text | Muted blue-grey, approx. #8FA3B1 |
| Primary accent | Muted mint, approx. #8EE6C8 |
| Secondary accent | Cool sky blue, approx. #7DBDE8 |
| Caution/error | Soft amber/red with explicit icons/text |
| Shape | Modest corner radii and restrained dividers; avoid heavy card nesting |
| Typography | Readable UI sans/monospace where appropriate; musical notation uses the existing licensed Bravura assets |

These are **proposal tokens**, not replacements for existing theme palettes. Map semantic tokens into Aurora, Ember and Paper, and check contrast in every theme. Preserve Tangent's approved brand direction instead of copying assets or visuals from Neothesia.

### Input feedback invariants

1. **Solid-filled key = note physically/virtually played.**
2. **Edge glow = note expected by the song, or by an explicitly revealed training hint.**
3. Both states may coexist on one key without becoming indistinguishable.
4. Expected-key glow must remain visible even when cosmetic particle effects are off.
5. Correct/wrong feedback must be supported by text/shape, not colour alone.
6. Effects may celebrate a streak only if a real evaluator has established that streak.

Prefer subtle, short-lived particles away from notation and keys. Reduced-motion mode removes strong secondary animation while preserving the timing and key-state cues.

## Responsive and accessibility behavior

- Desktop/wide: two-column Main, top strip in Session, left Settings navigation, keyboard always spanning the musical view.
- Narrow/touch: vertically stacked Main, horizontal/stacked Settings category navigation, fewer HUD chips, scalable piano region. Never make noteheads/controls too small to read or hit.
- Support keyboard-only navigation and visible focus, as well as pointer/touch. Esc/F1, Space and existing shortcuts should retain their musical roles; do not intercept piano typing in normal sessions.
- Keep labels and controls operable with high display scaling and HiDPI. Aim for at least a roughly 44 logical-pixel touch target where practicable.
- Errors and unavailable features need explanatory text and recovery, not only muted disabled buttons.
- Check disconnected MIDI, denied Web MIDI, no audio permission, empty library, long piece titles, narrow windows and reduced motion.
- All controls remain in **Bevy's single surface** per ADR-0005. Browser/native integrations stay behind platform capability boundaries; no HTML overlay is introduced into the shipped UI.

## Implementation slices / acceptance criteria

The owner has authorized initial menu implementation. Preserve working playback, training and persistence while introducing the UI incrementally.

### Slice A — Structure and navigation

- [ ] Main has a clear mode hierarchy and real-data-based Continue state.
- [x] Settings are grouped into named categories, accessible from Main and Pause.
- [x] Pause is a compact overlay with explicit Resume/Restart/Main actions.
- [x] From Settings, Close returns to the previous screen; if originating from a session, remain paused.
- [ ] Existing song, exercise and library flows remain reachable by keyboard/pointer/touch.

### Slice B — Play and practice HUD

- [ ] Song title/time/progress, pause and actual device status are legible without covering the music.
- [x] Common transport actions work without entering Settings.
- [ ] Solid played-key and expected-key glow remain distinct in every theme, with effects on/off.
- [x] Train hides hints by default, shows clear correct/wrong/release feedback and advances only on release.
- [ ] FreeForm match labels are non-blocking and never invent confidence/recognition.

### Slice C — Preferences and robustness

- [ ] MIDI input/output selectors, Rescan and output limitations are accurately represented.
- [ ] Persist only actual supported settings; add versioned migration/tests before new fields.
- [ ] Verify responsive layouts, narrow-screen touch input, high contrast and keyboard focus.
- [ ] Validate session pause/voice release, output switching/disconnect and settings return paths on web and native.
- [ ] Measure UI responsiveness and avoid expensive work on audio/render-critical paths.

### Slice D — Optional future product features (not part of UI-only refactor)

- [ ] Add evaluation-driven score/accuracy/streak data and results screens.
- [ ] Add adaptive practice, goals and progress history backed by actual saved data.
- [x] Effect intensity/presets and reduced-motion preference (ADR-0017).
- [ ] Richer accessibility semantics and contrast verification.
- [ ] Add reliable restore/import of SQLite backups, after integrity/migration handling.
- [ ] Add truly supported HUD speed/tempo controls and verified recognition confidence.

## Explicit non-goals and review notes

- No changes to MIDI scheduling, note evaluation, playback rules, score schema or MusicXML interpretation are specified in this document.
- No real-time device support or performance is certified by the interactive design prototype.
- No copied third-party layout, music or assets are requested.
- The design is approved as a **direction**, while exact spacing, typography and individual settings may still be adjusted during Bevy implementation after testing on a real piano and narrow screen.
- This branch now includes initial Bevy menu, pause and settings implementation; HUD presets, scoring, new effects and accessibility options remain proposals.

## Verification of documentation

Review the Main -> Library/Train/FreeForm/Play -> Pause -> Settings -> Pause -> Resume navigation described above, check the distinction between implemented and proposed options against the linked ADRs, and confirm that the draft stays within the single-Bevy-surface requirement. UI behavior tests and visual screenshots belong to the later implementation branch.

## Integration update — 2026-10-25

Play now has a compact transport/title/time HUD. Menus support Tab/Shift+Tab,
visible focus and Enter activation. Practice supports optional target audio, exact
coloured live staff pointers and persisted clef/difficulty. Effects have persistent
intensity and reduced motion. Full screen-reader semantics, calibrated scores and
the remaining unchecked items are still follow-up work.
