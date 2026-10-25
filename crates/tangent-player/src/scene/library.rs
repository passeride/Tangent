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
    selected: bool,
    modifiers: [bool; 4],
    search_delay: Option<f32>,
}
impl Default for Library {
    fn default() -> Self {
        Self {
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
            selected: false,
            modifiers: [false; 4],
            search_delay: None,
        }
    }
}
impl Library {
    pub fn refresh(&mut self) {
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
fn clip(text: &str, width: f32, size: f32) -> String {
    let max = (width / (size * 0.62)).max(4.) as usize;
    let mut out: String = text.chars().take(max).collect();
    if text.chars().count() > max {
        out.pop();
        out.push('…');
    }
    out
}
pub(super) fn render(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
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
    let button = ((width - 16.) / 3.).min(180.);
    d.button(state, x, 90., button, "Import files", Action::Import, t);
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
        &clip(&label, width - 86., 12.),
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
        &format!("Sort: {sort}"),
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
        let text_width = if narrow { width } else { width - 246. };
        d.text(x, y, clip(&entry.title, text_width, 16.), 16., t.ink);
        let seconds = entry.duration_us / 1_000_000;
        let detail = format!(
            "{} · {} · {}:{:02} · {} notes",
            if entry.composer.is_empty() {
                "Unknown composer"
            } else {
                &entry.composer
            },
            entry.format,
            seconds / 60,
            seconds % 60,
            entry.note_count
        );
        d.text(x, y + 23., clip(&detail, text_width, 11.), 11., t.muted);
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
        &clip(&label, width - 16., 13.),
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
        &clip(&label, width - 16., 13.),
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
    d.text(
        x,
        280.,
        "Type to edit · Ctrl+A selects all · Tab changes field",
        11.,
        t.muted,
    );
    d.text(
        x,
        305.,
        clip(&state.library.status, width, 11.),
        11.,
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
