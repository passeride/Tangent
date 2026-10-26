//! Editor draft state is separate from acknowledged persistent preferences.
use super::{controls::Action, draw::Drawing, *};
use crate::personalization::{Hand, HandProfile, Personalization, ThemeFile, Tokens};
#[derive(Default)]
pub(super) struct Editor {
    pub saved: Personalization,
    pub open: u8,
    pub token: usize,
    pub page: usize,
    pub channel: usize,
    pub draft: Option<ThemeFile>,
    pub undo: Vec<ThemeFile>,
    pub hand_draft: Option<HandProfile>,
    pub teaching: Option<Hand>,
    pub source: String,
    pub note: usize,
    pub status: String,
    pub matched: usize,
    pub unresolved: usize,
}
fn rgba(c: Color) -> [u8; 4] {
    let c = c.to_srgba();
    [c.red, c.green, c.blue, c.alpha].map(|v| (v * 255.).round() as u8)
}
fn color(v: [u8; 4]) -> Color {
    Color::srgba_u8(v[0], v[1], v[2], v[3])
}
fn default_file(state: &State) -> ThemeFile {
    let t = theme::palette(state.theme);
    ThemeFile {
        schema_version: 1,
        name: "Custom".into(),
        tokens: Tokens::from_values([
            rgba(t.bg),
            rgba(t.panel),
            rgba(t.ink),
            rgba(t.muted),
            rgba(t.staff),
            rgba(t.white),
            rgba(t.black),
            rgba(t.colors[0]),
            rgba(t.colors[1]),
            rgba(t.colors[2]),
            rgba(t.colors[3]),
            rgba(t.colors[1]),
            rgba(t.colors[0]),
            rgba(t.colors[2]),
            rgba(t.muted),
        ]),
    }
}
pub(super) fn palette(state: &State) -> theme::Theme {
    let mut t = theme::palette(state.theme);
    let file = if state.personal.open == 1 {
        state
            .personal
            .draft
            .as_ref()
            .or(state.personal.saved.theme.as_ref())
    } else {
        state
            .personal
            .saved
            .theme
            .as_ref()
            .filter(|_| state.personal.saved.use_custom)
    };
    if let Some(f) = file {
        let v = f.tokens.values();
        t.bg = color(v[0]);
        t.panel = color(v[1]);
        t.ink = color(v[2]);
        t.muted = color(v[3]);
        t.staff = color(v[4]);
        t.white = color(v[5]);
        t.black = color(v[6]);
        t.colors = [color(v[7]), color(v[8]), color(v[9]), color(v[10])];
    }
    t
}
pub(super) fn note_color(state: &State, id: usize, t: theme::Theme) -> Color {
    let profile = if state.personal.source == state.piece_id
        && (state.personal.open == 2 || state.personal.teaching.is_some())
    {
        state.personal.hand_draft.as_ref().or(state
            .personal
            .saved
            .hands
            .get(&state.piece_id)
            .map(std::sync::Arc::as_ref))
    } else {
        state
            .personal
            .saved
            .hands
            .get(&state.piece_id)
            .map(std::sync::Arc::as_ref)
    };
    let hand = profile
        .and_then(|p| p.notes.get(&id))
        .copied()
        .unwrap_or(Hand::Unassigned);
    let file = if state.personal.open == 1 {
        state
            .personal
            .draft
            .as_ref()
            .or(state.personal.saved.theme.as_ref())
    } else {
        state
            .personal
            .saved
            .theme
            .as_ref()
            .filter(|_| state.personal.saved.use_custom)
    };
    let index = match hand {
        Hand::Left => 11,
        Hand::Right => 12,
        Hand::Either => 13,
        Hand::Unassigned => 14,
    };
    file.map_or(
        match hand {
            Hand::Left => t.colors[1],
            Hand::Right => t.colors[0],
            Hand::Either => t.colors[2],
            Hand::Unassigned => t.muted,
        },
        |f| color(f.tokens.values()[index]),
    )
}
pub(super) fn import(state: &mut State, bytes: &[u8]) {
    match ThemeFile::parse(bytes) {
        Ok(file) => {
            if let Some(previous) = &state.personal.draft {
                state.personal.undo.push(previous.clone());
            }
            state.personal.draft = Some(file);
            state.personal.open = 1;
            state.settings_open = true;
            state.help = true;
            state.personal.status = "Imported preview. Apply to save.".into();
        }
        Err(e) => state.personal.status = e,
    }
}
pub(super) fn capture(state: &mut State, key: u8) {
    if let Some(hand) = state.personal.teaching {
        if state.personal.source != state.piece_id || !state.transport.playing {
            return;
        }
        if let Some(id) =
            crate::personalization::match_attack(&state.song, state.transport.position_us, key)
        {
            let p = state.personal.hand_draft.as_mut().unwrap();
            if p.notes
                .get(&id)
                .is_some_and(|old| *old != hand && *old != Hand::Unassigned)
            {
                state.personal.unresolved += 1;
            } else {
                p.notes.insert(id, hand);
                state.personal.matched += 1;
            }
        } else {
            state.personal.unresolved += 1;
        }
    }
}
pub(super) fn action(state: &mut State, command: u8) {
    match command {
        0 => {
            state.personal.page = 0;
            state.personal.draft = Some(
                state
                    .personal
                    .saved
                    .theme
                    .clone()
                    .unwrap_or_else(|| default_file(state)),
            );
            state.personal.open = 1;
            state.personal.undo.clear();
        }
        1 => {
            state.personal.page = 0;
            if state.personal.source != state.piece_id || state.personal.hand_draft.is_none() {
                state.personal.source = state.piece_id.clone();
                state.personal.hand_draft = Some(
                    state
                        .personal
                        .saved
                        .hands
                        .get(&state.piece_id)
                        .map(|p| p.as_ref().clone())
                        .unwrap_or(HandProfile {
                            mapping_version: 1,
                            revision: 0,
                            notes: BTreeMap::new(),
                        }),
                );
            }
            state.personal.open = 2;
            state.personal.note = 0;
        }
        2 | 3 => {
            if state.mode != Mode::Play {
                state.personal.status = "Open a piece in Play first.".into();
                return;
            }
            training::end(state, false);
            state.panic();
            state.personal.teaching = Some(if command == 2 {
                Hand::Left
            } else {
                Hand::Right
            });
            state.personal.matched = 0;
            state.personal.unresolved = 0;
            state.help = false;
            state.settings_open = false;
            let start = state.transport.passage().map_or(0, |p| p.start_us);
            state.transport.seek(start);
            state.transport.playing = true;
            state.clock = platform::clock_us();
            state.sync();
        }
        4 => {
            state.panic();
            state.personal.open = 2;
            state.help = true;
            state.settings_open = true;
            state.settings_tab = SettingsTab::Visuals;
        }
        5 => {
            if !state.personal.saved.hands.contains_key(&state.piece_id)
                && state.personal.saved.hands.len() >= 128
            {
                state.personal.status = "Hand profile limit reached (128 pieces).".into();
                return;
            }
            if state.personal.source != state.piece_id {
                return;
            }
            let mut p = state.personal.hand_draft.clone().unwrap();
            p.revision = state
                .personal
                .saved
                .hands
                .get(&state.piece_id)
                .map_or(1, |old| old.revision.saturating_add(1));
            state.personal.hand_draft = Some(p.clone());
            state.saved_preferences = None;
            state
                .personal
                .saved
                .hands
                .insert(state.piece_id.clone(), std::sync::Arc::new(p));
            state.personal.status = "Assignment applied. Check storage status below.".into();
        }
        6 => {
            state.personal.open = 0;
            state.personal.draft = None;
            state.personal.hand_draft = None;
        }
        7 => state.personal.note = state.personal.note.saturating_sub(1),
        8 => {
            state.personal.note =
                (state.personal.note + 1).min(state.song.notes.len().saturating_sub(1))
        }
        9 => {
            if let Some(n) = state.song.notes.get(state.personal.note) {
                let p = state.personal.hand_draft.as_mut().unwrap();
                let hand = p
                    .notes
                    .get(&n.id)
                    .copied()
                    .unwrap_or(Hand::Unassigned)
                    .next();
                p.notes.insert(n.id, hand);
            }
        }
        10 => state.personal.token = (state.personal.token + 14) % 15,
        11 => state.personal.token = (state.personal.token + 1) % 15,
        12 => state.personal.channel = (state.personal.channel + 1) % 4,
        13..=16 => {
            let f = state.personal.draft.as_mut().unwrap();
            state.personal.undo.push(f.clone());
            if state.personal.undo.len() > 64 {
                state.personal.undo.remove(0);
            }
            let mut v = f.tokens.values();
            let value = &mut v[state.personal.token][state.personal.channel];
            *value = match command {
                13 => value.saturating_sub(16),
                14 => value.saturating_sub(1),
                15 => value.saturating_add(1),
                _ => value.saturating_add(16),
            };
            f.tokens = Tokens::from_values(v);
        }
        17 => {
            state.personal.saved.theme = state.personal.draft.clone();
            state.personal.saved.use_custom = true;
            state.saved_preferences = None;
            state.personal.status = "Theme applied. Check storage status below.".into();
        }
        18 => {
            if let Some(f) = &state.personal.draft {
                platform::action("theme-export", &serde_json::to_string_pretty(f).unwrap());
            }
        }
        19 => platform::action("theme-import", ""),
        26 => {
            if let Some(p) = &mut state.personal.hand_draft {
                p.notes.clear();
            }
        }
        24 => {
            state.personal.page = state.personal.page.saturating_sub(1);
        }
        25 => {
            state.personal.page += 1;
        }
        23 => {
            if state.personal.saved.theme.is_none() {
                state.personal.status = "Apply a custom theme first.".into();
                return;
            }
            state.personal.saved.use_custom = !state.personal.saved.use_custom;
        }
        22 => {
            let base = default_file(state).tokens.values()[state.personal.token];
            if let Some(f) = &mut state.personal.draft {
                let mut v = f.tokens.values();
                v[state.personal.token] = base;
                f.tokens = Tokens::from_values(v);
            }
        }
        20 => {
            if let Some(f) = state.personal.undo.pop() {
                state.personal.draft = Some(f)
            }
        }
        21 => {
            if let Some(f) = &state.personal.draft {
                state.personal.undo.push(f.clone());
            }
            state.personal.draft = Some(default_file(state));
        }
        _ => {}
    }
}
pub(super) fn render(
    d: &mut Drawing,
    state: &mut State,
    l: &Layout,
    t: theme::Theme,
    x: f32,
    y: f32,
    w: f32,
) {
    let theme = state.personal.open == 1;
    let t = if theme {
        super::theme::palette(state.theme)
    } else {
        t
    };
    d.rect(x - 6., y - 6., w + 12., (l.height - y).max(0.), t.bg, 0.);
    d.text(
        x,
        y,
        if theme {
            "Theme editor: RGBA"
        } else {
            "Hands: demonstrate / review"
        },
        20.,
        t.ink,
    );
    let mut entries: Vec<(String, u8)> = vec![];
    if theme {
        let f = state.personal.draft.as_ref().unwrap();
        let v = f.tokens.values()[state.personal.token];
        d.text(
            x,
            y + 32.,
            format!(
                "{}: {:?} / {}",
                Tokens::LABELS[state.personal.token],
                v,
                ["Red", "Green", "Blue", "Alpha"][state.personal.channel]
            ),
            12.,
            t.ink,
        );
        let contrast = crate::personalization::contrast(f.tokens.ink, f.tokens.bg);
        let message = if f.tokens.bg[3] != 255 {
            "Transparent background: inspect text contrast".into()
        } else {
            format!(
                "Text contrast {contrast:.1}:1{}",
                if contrast < 4.5 { " — low" } else { "" }
            )
        };
        d.text(x, y + 54., message, 10., t.muted);
        for i in 0..4 {
            d.rect(
                x + w - 68. + i as f32 * 17.,
                y + 54.,
                14.,
                14.,
                color(f.tokens.values()[11 + i]),
                2.,
            );
        }
        entries.extend([
            ("Previous token".into(), 10),
            ("Next token".into(), 11),
            ("Next RGBA channel".into(), 12),
            ("-16".into(), 13),
            ("-1".into(), 14),
            ("+1".into(), 15),
            ("+16".into(), 16),
            ("Undo".into(), 20),
            ("Reset to preset".into(), 21),
            ("Reset token".into(), 22),
            ("Apply / save".into(), 17),
            (
                format!(
                    "Active: {}",
                    if state.personal.saved.use_custom {
                        "Custom"
                    } else {
                        "Preset"
                    }
                ),
                23,
            ),
            ("Import JSON".into(), 19),
            ("Export JSON".into(), 18),
        ]);
    } else {
        d.text(
            x,
            y + 32.,
            format!(
                "Matched {} / unresolved {}",
                state.personal.matched, state.personal.unresolved
            ),
            12.,
            t.ink,
        );
        entries.extend([
            ("Teach LEFT".into(), 2),
            ("Teach RIGHT".into(), 3),
            ("Save reviewed assignment".into(), 5),
            ("Previous note".into(), 7),
            ("Next note".into(), 8),
            ("Cycle note hand".into(), 9),
            ("Clear draft assignments".into(), 26),
        ]);
        if let Some(n) = state.song.notes.get(state.personal.note) {
            let h = state
                .personal
                .hand_draft
                .as_ref()
                .and_then(|p| p.notes.get(&n.id))
                .copied()
                .unwrap_or(Hand::Unassigned);
            d.text(
                x,
                y + 53.,
                format!(
                    "Note {} / {} / {} / {}",
                    state.personal.note + 1,
                    super::draw::time(n.start_us),
                    tangent_core::input_bindings::pitch(n.key),
                    h.label()
                ),
                12.,
                note_color(state, n.id, t),
            );
        }
    }
    entries.push(("Cancel / back".into(), 6));
    let cols = if w > 500. { 3 } else { 2 };
    let bw = (w - 12.) / cols as f32;
    let rows = ((l.height - y - 160.) / 36.).floor().max(1.) as usize;
    let count = rows * cols;
    let pages = entries.len().div_ceil(count);
    state.personal.page = state.personal.page.min(pages - 1);
    for (i, (label, id)) in entries
        .into_iter()
        .skip(state.personal.page * count)
        .take(count)
        .enumerate()
    {
        let py = y + 80. + (i / cols) as f32 * 36.;
        d.button(
            state,
            x + (i % cols) as f32 * (bw + 6.),
            py,
            bw,
            label,
            Action::Personal(id),
            t,
        );
    }
    let footer = y + 80. + rows as f32 * 36.;
    d.button(state, x, footer, 90., "Previous", Action::Personal(24), t);
    d.text(
        x + 100.,
        footer + 8.,
        format!("Page {}/{}", state.personal.page + 1, pages),
        11.,
        t.muted,
    );
    d.button(
        state,
        x + w - 90.,
        footer,
        90.,
        "Next",
        Action::Personal(25),
        t,
    );
    d.text(
        x,
        l.height - 54.,
        state
            .personal
            .status
            .chars()
            .take((w / 6.) as usize)
            .collect::<String>(),
        10.,
        t.ink,
    );
    d.text(
        x,
        l.height - 34.,
        state
            .storage_status
            .chars()
            .take((w / 6.) as usize)
            .collect::<String>(),
        10.,
        t.muted,
    );
}
