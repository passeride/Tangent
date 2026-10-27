//! Library navigation, text editing and presentation share the Bevy surface.
use super::{controls::Action, draw::Drawing, *};
pub(super) const PAGE_SIZE: usize = 6;
pub(super) const SORTS: [&str; 4] = ["recent", "title", "composer", "duration"];
pub(super) const FORMATS: [&str; 4] = ["", "MIDI", "MusicXML", "MXL"];
#[derive(Clone, Copy, PartialEq)]
pub(super) enum Field {
    Search,
    Title,
    Composer,
}
pub(super) type Edit = crate::library::MetadataEdit;
pub(super) struct Library {
    pub folder_open: bool,
    pub folder_path: String,
    pub folder_auto: bool,
    pub folder_busy: bool,
    pub folder_status: String,
    pub row_hits: Vec<(layout::Rect, usize)>,
    pub last_click: Option<(String, f64, Vec2)>,
    pub page_size: usize,
    pub items: Vec<crate::library::Entry>,
    pub total: usize,
    pub offset: usize,
    pub ticket: u64,
    pub status: String,
    pub pending_delete: Option<String>,
    pub query: String,
    pub sort: usize,
    pub format: usize,
    pub focus: Option<Field>,
    pub edit: Option<Edit>,
    pub lookup: metadata_view::Lookup,
    pub cover_handles: BTreeMap<String, Handle<Image>>,
    pub cover_requested: BTreeSet<String>,
    pub cover_epoch: u64,
    selected: bool,
    modifiers: [bool; 4],
    search_delay: Option<f32>,
}
impl Default for Library {
    fn default() -> Self {
        Self {
            folder_open: false,
            folder_path: String::new(),
            folder_auto: false,
            folder_busy: false,
            folder_status: String::new(),
            row_hits: vec![],
            last_click: None,
            page_size: PAGE_SIZE,
            items: vec![],
            total: 0,
            offset: 0,
            ticket: 0,
            status: String::new(),
            pending_delete: None,
            query: String::new(),
            sort: 0,
            format: 0,
            focus: None,
            edit: None,
            lookup: metadata_view::Lookup::default(),
            cover_handles: BTreeMap::new(),
            cover_requested: BTreeSet::new(),
            cover_epoch: 0,
            selected: false,
            modifiers: [false; 4],
            search_delay: None,
        }
    }
}
impl Library {
    pub fn refresh(&mut self) {
        self.last_click = None;
        self.ticket += 1;
        self.search_delay = None;
        platform::action(
            "library-list",
            &serde_json::json!({
                "offset":self.offset,"limit":self.page_size,"ticket":self.ticket,
                "query":self.query,"sort":SORTS[self.sort],"format":FORMATS[self.format]
            })
            .to_string(),
        );
    }
    pub fn reset_page(&mut self) {
        self.offset = 0;
        self.pending_delete = None;
        self.refresh();
    }
    pub fn focus(&mut self, field: Field) {
        self.focus = Some(field);
        self.selected = false;
    }
    pub fn tick(&mut self, elapsed: f32) {
        if let Some(delay) = &mut self.search_delay {
            *delay -= elapsed;
            if *delay <= 0. {
                self.reset_page();
            }
        }
    }
    pub fn input_event(&mut self, key: KeyCode, text: Option<&str>, pressed: bool, enabled: bool) {
        let modifier = match key {
            KeyCode::ControlLeft => Some(0),
            KeyCode::ControlRight => Some(1),
            KeyCode::SuperLeft => Some(2),
            KeyCode::SuperRight => Some(3),
            _ => None,
        };
        if let Some(index) = modifier {
            self.modifiers[index] = pressed;
        } else if enabled && pressed {
            self.type_key(key, text, self.modifiers.iter().any(|held| *held));
        }
    }
    pub fn clear_modifiers(&mut self) {
        self.modifiers = [false; 4];
    }
    pub fn type_key(&mut self, key: KeyCode, text: Option<&str>, ctrl: bool) {
        let Some(field) = self.focus else {
            return;
        };
        if ctrl && key == KeyCode::KeyA {
            self.selected = true;
            return;
        }
        if ctrl {
            return;
        }
        let target = match field {
            Field::Search => &mut self.query,
            Field::Title => match self.edit.as_mut() {
                Some(edit) => &mut edit.title,
                None => return,
            },
            Field::Composer => match self.edit.as_mut() {
                Some(edit) => &mut edit.composer,
                None => return,
            },
        };
        let before = target.clone();
        if key == KeyCode::Backspace || key == KeyCode::Delete {
            if self.selected {
                target.clear();
            } else if key == KeyCode::Backspace {
                target.pop();
            }
            self.selected = false;
        } else if let Some(text) = text {
            let visible: String = text.chars().filter(|c| !c.is_control()).collect();
            if !visible.is_empty() {
                if self.selected {
                    target.clear();
                }
                let remaining = 160_usize.saturating_sub(target.chars().count());
                target.extend(visible.chars().take(remaining));
                self.selected = false;
            }
        }
        if before != *target && field == Field::Search {
            // Invalidate old results immediately, even during the debounce window.
            self.ticket += 1;
            self.search_delay = Some(0.2);
            self.pending_delete = None;
        }
    }
}
pub(super) fn clip(text: &str, width: f32, size: f32) -> String {
    let max = (width / (size * 0.62)).max(4.) as usize;
    let mut out: String = text.chars().take(max).collect();
    if text.chars().count() > max {
        out.pop();
        out.push('…');
    }
    out
}
pub(super) fn render(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    if state.library.folder_open {
        folder_view(d, state, l, t);
        return;
    }
    if state.library.lookup.open {
        metadata_view::render(d, state, l, t);
        return;
    }
    let x = 24.;
    let width = (l.width - 48.).max(120.);
    if state.library.edit.is_some() {
        render_edit(d, state, l, t);
        return;
    }
    let narrow = l.width < 620.;
    let row_height = if narrow { 88. } else { 60. };
    let page_size = (((l.height - 310.).max(row_height) / row_height) as usize).clamp(1, PAGE_SIZE);
    if state.library.page_size != page_size {
        state.library.page_size = page_size;
        state.library.offset = 0;
        state.library.refresh();
    }
    d.text(x, 24., "Music library", 28., t.ink);
    d.text(
        x,
        63.,
        format!("{} pieces · local library", state.library.total),
        13.,
        t.muted,
    );
    let button = ((width - 24.) / 4.).min(180.);
    d.button(state, x, 90., button, "Import", Action::Import, t);
    d.button(
        state,
        x + button + 8.,
        90.,
        button,
        "Refresh",
        Action::Library,
        t,
    );
    d.button(
        state,
        x + (button + 8.) * 2.,
        90.,
        button,
        "Back",
        Action::Main,
        t,
    );
    d.button(
        state,
        x + (button + 8.) * 3.,
        90.,
        button,
        "Folder",
        Action::LibraryFolder,
        t,
    );
    let label = if state.library.query.is_empty() && state.library.focus != Some(Field::Search) {
        "Search title or composer…".into()
    } else {
        format!(
            "Search: {}{}",
            state.library.query,
            if state.library.focus == Some(Field::Search) {
                "|"
            } else {
                ""
            }
        )
    };
    d.button(
        state,
        x,
        132.,
        width - 74.,
        clip(&label, width - 86., 12.),
        Action::LibrarySearch,
        t,
    );
    d.button(
        state,
        l.width - 90.,
        132.,
        66.,
        "Clear",
        Action::LibraryClearSearch,
        t,
    );
    let sort = ["Recent", "Title A–Z", "Composer A–Z", "Duration"][state.library.sort];
    let format = if state.library.format == 0 {
        "All formats"
    } else {
        FORMATS[state.library.format]
    };
    d.button(
        state,
        x,
        174.,
        width * 0.56 - 4.,
        format!("Sort: {sort}"),
        Action::LibrarySort,
        t,
    );
    d.button(
        state,
        x + width * 0.56 + 4.,
        174.,
        width * 0.44 - 4.,
        format,
        Action::LibraryFormat,
        t,
    );
    let items = state.library.items.clone();
    if items.is_empty() {
        let text = if state.library.query.is_empty() && state.library.format == 0 {
            "Import MIDI, MusicXML or MXL files to begin."
        } else {
            "No matches. Clear search or change format."
        };
        d.text(x, 232., clip(text, width, 14.), 14., t.muted);
    }
    for (index, entry) in items.iter().enumerate() {
        let y = 222. + index as f32 * row_height;
        state.library.row_hits.push((
            layout::Rect {
                x,
                y,
                w: width,
                h: row_height,
            },
            index,
        ));
        let cover_size = 44.;
        if let Some(handle) = state.library.cover_handles.get(&entry.id) {
            d.image(
                layout::Rect {
                    x,
                    y,
                    w: cover_size,
                    h: cover_size,
                },
                handle.clone(),
            );
        } else {
            d.rect(x, y, cover_size, cover_size, t.panel, 5.);
            d.glyph(x + 13., y + 30., '\u{e1d5}', 30., t.colors[0]);
        }
        let text_x = x + 56.;
        let text_width = if narrow { width - 56. } else { width - 302. };
        d.text(text_x, y, clip(&entry.title, text_width, 16.), 16., t.ink);
        let seconds = entry.duration_us / 1_000_000;
        let artist = entry
            .enrichment
            .as_ref()
            .map(|c| c.artists.join(", "))
            .unwrap_or_default();
        let album = entry
            .enrichment
            .as_ref()
            .map(|c| format!(" · {}", c.album))
            .unwrap_or_default();
        let detail = format!(
            "{}{} · {} · {}:{:02} · {} notes",
            if entry.composer.is_empty() {
                if artist.is_empty() {
                    "Unknown composer"
                } else {
                    &artist
                }
            } else {
                &entry.composer
            },
            album,
            entry.format,
            seconds / 60,
            seconds % 60,
            entry.note_count
        );
        d.text(
            text_x,
            y + 23.,
            clip(&detail, text_width, 11.),
            11.,
            t.muted,
        );
        let bx = if narrow { x } else { l.width - 262. };
        let by = if narrow { y + 45. } else { y };
        d.button(state, bx, by, 66., "Open", Action::LibraryOpen(index), t);
        d.button(
            state,
            bx + 74.,
            by,
            65.,
            "Edit",
            Action::LibraryEdit(index),
            t,
        );
        let label = if state.library.pending_delete.as_ref() == Some(&entry.id) {
            "Confirm"
        } else {
            "Remove"
        };
        d.button(
            state,
            bx + 147.,
            by,
            91.,
            label,
            Action::LibraryRemove(index),
            t,
        );
    }
    let footer = (l.height - 80.).max(180.);
    if state.library.offset > 0 {
        d.button(
            state,
            x,
            footer,
            110.,
            "Previous",
            Action::LibraryPrevious,
            t,
        );
    }
    if state.library.offset + state.library.page_size < state.library.total {
        d.button(
            state,
            x + 120.,
            footer,
            110.,
            "Next",
            Action::LibraryNext,
            t,
        );
    }
    d.text(
        x,
        l.height - 30.,
        clip(&state.library.status, width, 11.),
        11.,
        t.muted,
    );
}
fn render_edit(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let x = 24.;
    let width = (l.width - 48.).max(120.);
    let edit = state.library.edit.as_ref().unwrap();
    let title = edit.title.clone();
    let composer = edit.composer.clone();
    d.text(x, 24., "Edit piece", 28., t.ink);
    d.text(x, 77., "Title", 14., t.muted);
    let label = format!(
        "{}{}",
        title,
        if state.library.focus == Some(Field::Title) {
            "|"
        } else {
            ""
        }
    );
    d.button(
        state,
        x,
        102.,
        width,
        clip(&label, width - 16., 13.),
        Action::LibraryTitle,
        t,
    );
    d.text(x, 151., "Composer (optional)", 14., t.muted);
    let label = format!(
        "{}{}",
        composer,
        if state.library.focus == Some(Field::Composer) {
            "|"
        } else {
            ""
        }
    );
    d.button(
        state,
        x,
        176.,
        width,
        clip(&label, width - 16., 13.),
        Action::LibraryComposer,
        t,
    );
    d.button(state, x, 231., 110., "Save", Action::LibrarySaveMetadata, t);
    d.button(
        state,
        x + 120.,
        231.,
        110.,
        "Cancel",
        Action::LibraryCancelEdit,
        t,
    );
    d.button(
        state,
        x,
        275.,
        (width * 0.5 - 4.).min(240.),
        "Find cover",
        Action::MetadataOpen,
        t,
    );
    if state.library.items.iter().any(|e| {
        state
            .library
            .edit
            .as_ref()
            .is_some_and(|edit| edit.id == e.id)
            && e.enrichment.is_some()
    }) {
        d.button(
            state,
            x + (width * 0.5 + 4.).min(248.),
            275.,
            (width * 0.5 - 4.).min(240.),
            "Remove metadata",
            Action::MetadataClear,
            t,
        );
    }
    d.text(
        x,
        (l.height - 55.).min(335.),
        clip(
            "Type to edit · Ctrl+A selects all · Tab changes field",
            width,
            11.,
        ),
        11.,
        t.muted,
    );
    d.text(
        x,
        (l.height - 30.).min(360.),
        clip(&state.library.status, width, 11.),
        11.,
        t.muted,
    );
}

fn folder_view(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let x = 24.;
    let width = l.width - 48.;
    d.text(x, 24., "Library folder", 28., t.ink);
    let path = if state.library.folder_path.is_empty() {
        "No folder selected"
    } else {
        &state.library.folder_path
    };
    d.text(x, 82., clip(path, width, 14.), 14., t.ink);
    d.text(
        x,
        112.,
        clip(
            if state.library.folder_path.is_empty() {
                "Choose a folder containing MIDI, MusicXML or MXL files."
            } else if state.library.folder_auto {
                "Scans on startup when folder access is available."
            } else {
                "Browser access may require selecting the folder again."
            },
            width,
            11.,
        ),
        11.,
        t.muted,
    );
    if !state.library.folder_busy {
        d.button(
            state,
            x,
            150.,
            width.min(240.),
            "Choose folder",
            Action::FolderChoose,
            t,
        );
        d.button(
            state,
            x,
            192.,
            width.min(240.),
            "Scan now",
            Action::FolderScan,
            t,
        );
        if !state.library.folder_path.is_empty() {
            d.button(
                state,
                x,
                234.,
                width.min(240.),
                "Remove folder setting",
                Action::FolderClear,
                t,
            );
        }
    }
    d.button(
        state,
        x,
        (l.height - 90.).max(278.),
        110.,
        "Back",
        Action::FolderBack,
        t,
    );
    d.text(
        x,
        l.height - 40.,
        clip(&state.library.folder_status, width, 11.),
        11.,
        t.muted,
    );
    d.text(
        x,
        l.height - 22.,
        clip(
            "Includes subfolders. Saved pieces are never deleted by scans.",
            width,
            10.,
        ),
        10.,
        t.muted,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn modifier_release_in_same_frame_does_not_swallow_replacement_text() {
        let mut library = Library::default();
        library.focus(Field::Search);
        library.query = "Old".into();
        library.input_event(KeyCode::ControlLeft, None, true, true);
        library.input_event(KeyCode::KeyA, Some("a"), true, true);
        library.input_event(KeyCode::ControlLeft, None, false, true);
        library.input_event(KeyCode::KeyB, Some("Bach"), true, true);
        assert_eq!(library.query, "Bach");
    }
    #[test]
    fn unicode_search_replaces_selection_and_preserves_text_controls() {
        let mut library = Library::default();
        library.focus(Field::Search);
        library.type_key(KeyCode::KeyA, Some("Ærlig"), false);
        assert_eq!(library.query, "Ærlig");
        let ticket = library.ticket;
        library.type_key(KeyCode::KeyA, Some("a"), true);
        library.type_key(KeyCode::KeyB, Some("Bach"), false);
        assert_eq!(library.query, "Bach");
        assert!(library.ticket > ticket);
        library.type_key(KeyCode::Backspace, None, false);
        assert_eq!(library.query, "Bac");
    }
}
