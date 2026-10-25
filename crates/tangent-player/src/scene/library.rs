//! Paged library presentation in the shared Bevy UI.
use super::{controls::Action, draw::Drawing, *};
pub(super) const PAGE_SIZE: usize = 6;

pub(super) struct Library {
    pub page_size: usize,
    pub items: Vec<crate::library::Entry>,
    pub total: usize,
    pub offset: usize,
    pub ticket: u64,
    pub status: String,
    pub pending_delete: Option<String>,
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
        }
    }
}
impl Library {
    pub fn refresh(&mut self) {
        self.ticket += 1;
        platform::action(
            "library-list",
            &serde_json::json!({
                "offset":self.offset,"limit":self.page_size,"ticket":self.ticket
            })
            .to_string(),
        );
    }
}
pub(super) fn render(d: &mut Drawing, state: &mut State, l: &Layout, t: theme::Theme) {
    let page_size = (((l.height - 260.).max(52.) / 52.) as usize).clamp(1, PAGE_SIZE);
    if state.library.page_size != page_size {
        state.library.page_size = page_size;
        state.library.offset = 0;
        state.library.refresh();
    }
    let x = 24.;
    let width = (l.width - 48.).max(120.);
    d.text(x, 25., "Music library", 28., t.ink);
    d.text(
        x,
        65.,
        format!("{} pieces saved on this device", state.library.total),
        13.,
        t.muted,
    );
    let button = (width / 3. - 8.).min(180.);
    d.button(state, x, 95., button, "Import", Action::Import, t);
    d.button(
        state,
        x + button + 8.,
        95.,
        button,
        "Refresh",
        Action::Library,
        t,
    );
    d.button(
        state,
        x + (button + 8.) * 2.,
        95.,
        button,
        "Back",
        Action::Main,
        t,
    );
    let row_height = ((l.height - 260.) / state.library.page_size as f32).clamp(40., 72.);
    let items = state.library.items.clone();
    if items.is_empty() {
        d.text(
            x,
            160.,
            "Import MIDI, MusicXML or MXL to save your first piece.",
            14.,
            t.muted,
        );
    }
    for (index, entry) in items.iter().enumerate() {
        let y = 150. + index as f32 * row_height;
        let title: String = entry
            .title
            .chars()
            .take(((width - 165.) / 8.).max(6.) as usize)
            .collect();
        d.text(x, y, title, 16., t.ink);
        let seconds = entry.duration_us / 1_000_000;
        d.text(
            x,
            y + 23.,
            format!(
                "{} · {}:{:02} · {} notes",
                entry.format,
                seconds / 60,
                seconds % 60,
                entry.note_count
            ),
            11.,
            t.muted,
        );
        d.button(
            state,
            l.width - 174.,
            y,
            65.,
            "Open",
            Action::LibraryOpen(index),
            t,
        );
        let label = if state.library.pending_delete.as_ref() == Some(&entry.id) {
            "Confirm"
        } else {
            "Remove"
        };
        d.button(
            state,
            l.width - 101.,
            y,
            77.,
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
        state.library.status.chars().take(110).collect::<String>(),
        11.,
        t.muted,
    );
}
