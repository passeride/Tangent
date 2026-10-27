//! Bevy review flow for provider candidates; musical metadata rules live in core.
use super::{
    controls::Action,
    draw::Drawing,
    layout::{Layout, Rect},
    theme, *,
};
use tangent_core::metadata::{Candidate, MetadataProvider, Query, providers};
#[derive(Default)]
pub(super) struct Lookup {
    pub open: bool,
    pub provider_index: usize,
    pub busy: bool,
    pub ticket: u64,
    pub query: Option<Query>,
    pub candidates: Vec<Candidate>,
    pub selected: Option<usize>,
    pub bytes: Vec<u8>,
    pub image: Option<Handle<Image>>,
    pub ready: bool,
    pub status: String,
    pub offset: usize,
    pub page_size: usize,
}
impl Lookup {
    pub fn provider(&self) -> &'static dyn MetadataProvider {
        providers()[self.provider_index.min(providers().len() - 1)]
    }
    pub fn close(&mut self) {
        self.ticket += 1;
        self.open = false;
        self.busy = false;
        self.selected = None;
        self.image = None;
        self.bytes.clear();
        self.candidates.clear();
    }
}
pub(super) fn search(state: &mut State, title_only: bool) {
    let Some(edit) = state.library.edit.as_ref() else {
        return;
    };
    let q = Query {
        title: edit.title.clone(),
        creator: if title_only {
            String::new()
        } else {
            edit.composer.clone()
        },
    };
    state.library.focus = None;
    let lookup = &mut state.library.lookup;
    if lookup.busy {
        return;
    }
    lookup.open = true;
    lookup.ticket += 1;
    lookup.query = Some(q.clone());
    lookup.candidates.clear();
    lookup.selected = None;
    lookup.bytes.clear();
    lookup.image = None;
    lookup.ready = false;
    lookup.offset = 0;
    match lookup.provider().search_url(&q) {
        Ok(url) => {
            lookup.busy = true;
            lookup.status = "Searching MusicBrainz…".into();
            platform::action(
                "metadata-fetch",
                &serde_json::json!({"url":url,"kind":"search","ticket":lookup.ticket}).to_string(),
            );
        }
        Err(e) => lookup.status = e,
    }
}
pub(super) fn preview(state: &mut State, index: usize) {
    let l = &mut state.library.lookup;
    if l.busy {
        return;
    }
    if let Some(c) = l.candidates.get(index) {
        l.ticket += 1;
        l.selected = Some(index);
        l.bytes.clear();
        l.image = None;
        l.ready = false;
        l.busy = true;
        l.status = "Loading front cover…".into();
        platform::action(
            "metadata-fetch",
            &serde_json::json!({"url":c.cover_url(),"kind":"cover","ticket":l.ticket}).to_string(),
        );
    }
}
pub(super) fn image(bytes: &[u8], images: &mut Assets<Image>) -> Result<Handle<Image>, String> {
    use bevy::{
        asset::RenderAssetUsages,
        render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    };
    let (mut width, mut height, mut rgba) = crate::cover::decode(bytes)?;
    // Fit non-square sleeve scans without stretching them in square library cells.
    if width != height {
        let side = width.max(height);
        let mut square = vec![0; side as usize * side as usize * 4];
        for row in 0..height as usize {
            let to = ((row + (side - height) as usize / 2) * side as usize
                + (side - width) as usize / 2)
                * 4;
            let from = row * width as usize * 4;
            square[to..to + width as usize * 4]
                .copy_from_slice(&rgba[from..from + width as usize * 4]);
        }
        rgba = square;
        width = side;
        height = side;
    }
    Ok(images.add(Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        rgba,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )))
}
pub(super) fn receive(state: &mut State, m: &serde_json::Value, images: &mut Assets<Image>) {
    let l = &mut state.library.lookup;
    if !l.open || m["ticket"].as_u64() != Some(l.ticket) {
        return;
    }
    l.busy = false;
    if let Some(e) = m["error"].as_str() {
        l.status = e.into();
        l.ready = l.selected.is_some();
        return;
    }
    if m["kind"] == "search" {
        let response = serde_json::from_str::<serde_json::Value>(m["body"].as_str().unwrap_or(""));
        match response
            .map_err(|_| "Provider returned unreadable data. Try again later.".to_string())
            .and_then(|r| l.provider().candidates(l.query.as_ref().unwrap(), &r))
        {
            Ok(c) => {
                l.candidates = c;
                l.status = if l.candidates.is_empty() {
                    "No matches. Try title only, or edit the local title first.".into()
                } else {
                    "Choose an album release to preview its cover.".into()
                };
            }
            Err(e) => l.status = e,
        }
    } else if m["kind"] == "cover" {
        let bytes: Vec<u8> = serde_json::from_value(m["bytes"].clone()).unwrap_or_default();
        match image(&bytes, images) {
            Ok(handle) => {
                l.image = Some(handle);
                l.bytes = bytes;
                l.status = "Review this release, then save if it fits your piece.".into();
            }
            Err(e) => l.status = format!("{e} Metadata can still be saved without a cover."),
        };
        l.ready = true;
    }
}
pub(super) fn save(state: &mut State) {
    let l = &state.library.lookup;
    if !l.ready || l.busy {
        return;
    }
    if let (Some(edit), Some(c)) = (
        &state.library.edit,
        l.selected.and_then(|i| l.candidates.get(i)),
    ) {
        platform::action(
            "library-enrich",
            &serde_json::json!({"id":edit.id,"candidate":c,"bytes":l.bytes}).to_string(),
        );
        state.library.lookup.busy = true;
        state.library.lookup.status = "Saving cover and metadata…".into();
    }
}
pub(super) fn render(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let x = 24.;
    let width = l.width - 48.;
    d.text(x, 24., "Find cover & metadata", 26., t.ink);
    d.text(x, 65., state.library.lookup.provider().name(), 12., t.muted);
    let selected = state
        .library
        .lookup
        .selected
        .and_then(|i| state.library.lookup.candidates.get(i))
        .cloned();
    if let Some(c) = selected {
        let compact = l.height < 560.;
        let size = if compact {
            80.
        } else if l.width < 620. {
            100.
        } else {
            156.
        };
        let y = 98.;
        if let Some(handle) = &state.library.lookup.image {
            d.image(
                Rect {
                    x,
                    y,
                    w: size,
                    h: size,
                },
                handle.clone(),
            );
        } else {
            d.rect(x, y, size, size, t.panel, 8.);
            d.glyph(
                x + size * 0.3,
                y + size * 0.65,
                '\u{e1d5}',
                size * 0.8,
                t.muted,
            );
        }
        let (tx, ty, tw) = if l.width < 620. && !compact {
            (x, y + size + 14., width)
        } else {
            (x + size + 24., y, width - size - 24.)
        };
        for (i, (text, size, color)) in [
            (&c.album, 17., t.ink),
            (&c.artists.join(", "), 13., t.ink),
            (&c.title, 12., t.muted),
            (&format!("{} · {}", c.date, c.country), 11., t.muted),
        ]
        .into_iter()
        .enumerate()
        {
            d.text(
                tx,
                ty + i as f32 * 26.,
                super::library::clip(text, tw, size),
                size,
                color,
            );
        }
        let by = (l.height - 128.).max(200.);
        if state.library.lookup.ready && !state.library.lookup.busy {
            d.button(
                state,
                x,
                by,
                width.min(250.),
                if state.library.lookup.bytes.is_empty() {
                    "Use metadata only"
                } else {
                    "Use cover & metadata"
                },
                Action::MetadataSave,
                t,
            );
        }
        d.button(
            state,
            x,
            by + 40.,
            110.,
            "Other results",
            Action::MetadataBack,
            t,
        );
        d.button(
            state,
            x + 120.,
            by + 40.,
            110.,
            "Back to edit",
            Action::MetadataClose,
            t,
        );
    } else {
        let busy = state.library.lookup.busy;
        if !busy {
            d.button(
                state,
                x,
                92.,
                (width * 0.5 - 4.).min(210.),
                "Search again",
                Action::MetadataSearch(false),
                t,
            );
            d.button(
                state,
                x + (width * 0.5 + 4.).min(218.),
                92.,
                (width * 0.5 - 4.).min(210.),
                "Title only",
                Action::MetadataSearch(true),
                t,
            );
        }
        let rows = (((l.height - 270.) / 92.).floor() as usize).clamp(1, 4);
        state.library.lookup.page_size = rows;
        let offset = state.library.lookup.offset;
        let candidates = state.library.lookup.candidates.clone();
        for (row, (index, c)) in candidates
            .iter()
            .enumerate()
            .skip(offset)
            .take(rows)
            .enumerate()
        {
            let y = 140. + row as f32 * 92.;
            d.text(
                x,
                y,
                super::library::clip(&c.album, width - 88., 15.),
                15.,
                t.ink,
            );
            d.text(
                x,
                y + 22.,
                super::library::clip(
                    &format!("{} · {}", c.artists.join(", "), c.date),
                    width - 88.,
                    11.,
                ),
                11.,
                t.muted,
            );
            d.text(
                x,
                y + 42.,
                super::library::clip(&c.title, width, 11.),
                11.,
                t.muted,
            );
            d.button(
                state,
                l.width - 104.,
                y,
                80.,
                "Preview",
                Action::MetadataPreview(index),
                t,
            );
        }
        let by = l.height - 100.;
        d.button(state, x, by, 100., "Back", Action::MetadataClose, t);
        if offset > 0 {
            d.button(
                state,
                x + 110.,
                by,
                70.,
                "Prev",
                Action::MetadataPage(false),
                t,
            );
        }
        if offset + rows < candidates.len() {
            d.button(
                state,
                x + 190.,
                by,
                70.,
                "Next",
                Action::MetadataPage(true),
                t,
            );
        }
    }
    d.text(
        x,
        l.height - 45.,
        super::library::clip(&state.library.lookup.status, width, 11.),
        11.,
        t.muted,
    );
    d.text(
        x,
        l.height - 25.,
        "Search shares title/artist hints, never your MIDI/score file.",
        10.,
        t.muted,
    );
}
