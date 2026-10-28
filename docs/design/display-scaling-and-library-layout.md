# Display scaling and a library that fills the window

- Status: Proposed; not implemented
- Created: 2026-10-28
- Related: [reuse/refactoring assessment](refactoring-and-library-review.md),
  [ADR-0037](../adr/0037-notation-and-ui-refactoring.md),
  [current UI contract](tangent-ui-v2.md), [score zoom](../adr/0032-scalable-sheet-overlay.md)

## Problem and evidence

The owner reports small UI on 4K and only about five library songs occupying
roughly 20% of the screen. Source review at `8bdb8bc` confirms:

- `scene/library.rs` derives row count from height but clamps it to `PAGE_SIZE = 6`.
  Desktop rows are 60 logical units, narrow rows 88. Headers/controls have fixed
  offsets; the footer is near the window bottom. This leaves a large empty middle.
- Title/detail text uses 16/11 units and covers 44 units. `scene/draw.rs` applies
  fixed positions and font sizes to absolute Bevy nodes. `scene/layout.rs` has
  additional bounded keyboard sizes. There is no shared app UI zoom setting.
- Draw/input already use Bevy logical window/cursor coordinates. The piano's
  texture density separately uses window scale factor. This is not evidence that
  OS DPI handling is absent or broken; capture the actual display configuration.
- Score scale (50–200%) and HUD density are existing independent controls. Neither
  is a global application readability setting. Storage fetches support up to 50
  metadata entries; the six-row cap belongs to presentation.

## Proposed experience

Settings → Visuals gains **Interface scale**: Auto and manual 100/125/150/175/200%.
These choices are a proposal; validate useful bounds on small windows. Persist a
manual preference. Auto starts with platform DPI/browser logical sizing; it must
not guess physical monitor size from resolution alone. The user can always change
it. Observe OS scale changes when moving between monitors.

Keep these concepts separate: OS/browser device scale, application UI zoom,
notation staff zoom and library density (Comfortable/Compact). Bigger text must
not force the entire score/piano into an unrelated zoom. Keep readable primary
labels and sufficient control targets; density changes row padding/secondary
information deliberately rather than shrinking all text indiscriminately.

The Library uses a full-height central list under the search/sort/import toolbar,
with a pinned status/footer. Grow the body into remaining space. Derive the number
of visible rows from body height and scaled row height, with no arbitrary six-row
presentation limit. When there are enough songs, remaining blank space within the
list should be less than one row. A short or filtered library may naturally have
blank space; do not manufacture rows or stretch five songs across a large screen.

Use vertical scrolling, a visible scrollbar when needed, keyboard selection,
Page Up/Down, Home/End and double-click Open. Preserve existing explicit Open/Edit/
Remove and deletion confirmation. On narrow windows, collapse secondary columns
and stack actions without clipping. On wide windows, use title, composer/artist,
album, format and duration columns with bounded action widths; title gets flexible
space. Optional detail/cover preview may use surplus width later; a card/grid view
is a separate choice, not required for this fix.

## Implementation constraints

Prefer existing Bevy Flexbox/Grid layout and a shared type/spacing scale before
changing frameworks. Evaluate egui tables only in the scoped comparison from the
refactoring plan. Merely inserting `UiScale` into the present absolute drawing
code can desynchronize hit rectangles, breakpoints and projected piano input.

Define a single display-metrics/coordinate boundary for drawing, manual hit tests,
focus outlines, dragging, touch and projected piano input. If app zoom reduces
layout-space width/height, inverse-transform pointer coordinates exactly once.
Do not multiply OS device scale a second time: Bevy window/cursor values are
already logical. Update diagnostics with logical size, physical size, device
scale, app zoom, effective scale and list viewport/row metrics.

Separate **visible rows** from **metadata fetch batch**. Keep current 50-entry
request bounds; page/prefetch adjacent metadata when required and virtualize rows
outside the viewport. Do not load source BLOBs or every cover to fill the list.
Bound thumbnail cache/prefetch; preserve stable source-ID selection and scroll
anchor through resize, edits, search and stale async replies. Query ticket
invalidation remains authoritative. Coordinate any changed defaults/limits across
native and browser adapters. Resize must not reset the list to the first song or
send a refresh every frame.

Scale typography, icons, padding and hit targets together. Keep vector/text output
sharp, refresh rasterized score/cover caches at effective density as needed, and
leave audio clocks/source times untouched. Preserve one Bevy window/canvas and
existing themes, hand colours, reduced motion and input bindings.

## Acceptance scenarios for the future implementation

| Scenario | Required evidence |
| --- | --- |
| 3840×2160 physical, OS 100%, manual UI 150/200% | Actual native captures show readable controls and correct pointer/piano hits, with no clipped footer/dialogs. |
| 3840×2160 physical, OS 150/200% | UI remains readable without double scaling; log actual logical viewport and scale factors. |
| Browser 1920×1080 CSS viewport, DPR 2 | Covers a 3840×2160 backing store; distinguish this from a 3840×2160 CSS viewport. Test browser zoom separately. |
| 1920×1080 and 2560×1440, several UI scales | Library body fills available height; with 100 synthetic pieces the visible count follows the layout, not a fixed cap. |
| 390×700 and 1280×420 logical viewports | Controls remain reachable by scrolling/reflow; no overlap or offscreen confirmation actions at supported scales. |
| 500-entry catalogue | Smooth bounded metadata/thumbnail loading, responsive search, no source BLOB fetch on scroll and no render/audio stalls. |
| Resize / move monitors / edit / remove | Stable selection and scroll anchor, no refresh loop or stale query results; double-click and keyboard activation target the displayed song. |
| Score overlay / sheet only / Train / Settings | App zoom and staff zoom act independently; live markers, key lanes and pointer input stay aligned. |

Capture actual app images at named logical/physical sizes; screenshots alone do
not prove physical readability. Include the owner's 4K review and Windows/browser
DPI behaviour. This document records requirements, not completed visual fixes.
