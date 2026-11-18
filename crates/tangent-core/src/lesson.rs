//! Declarative, renderer-independent interactive music lessons.
//! Exercises consume fresh player note attacks, never reference playback events.
use std::collections::BTreeSet;

pub const MAJOR: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];
pub const NATURAL_MINOR: [u8; 7] = [0, 2, 3, 5, 7, 8, 10];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Illustration {
    Keyboard,
    MajorIntervals,
    MinorIntervals,
    Harmony,
    Fifths,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Task {
    Read,
    Note(u8),
    Chord(&'static [u8]),
    Sequence(&'static [u8]),
    Explore(u8),
}
#[derive(Clone, Copy, Debug)]
pub struct Step {
    pub title: &'static str,
    pub paragraphs: &'static [&'static str],
    pub instruction: &'static str,
    pub illustration: Illustration,
    pub task: Task,
    /// A single example chord or, for sequences, consecutive notes.
    pub example: &'static [u8],
}
const fn step(
    title: &'static str,
    paragraphs: &'static [&'static str],
    instruction: &'static str,
    illustration: Illustration,
    task: Task,
    example: &'static [u8],
) -> Step {
    Step {
        title,
        paragraphs,
        instruction,
        illustration,
        task,
        example,
    }
}
pub struct Module {
    pub id: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
    pub steps: &'static [Step],
}

const HOME: &[Step] = &[
    step(
        "Hva betyr hjem?",
        &[
            "C er grunntonen: punktet melodien ofte vender tilbake til.",
            "Spill en C og lytt til følelsen av hvile.",
        ],
        "Spill C4 (midt-C).",
        Illustration::Keyboard,
        Task::Note(60),
        &[60],
    ),
    step(
        "Akkordens anker",
        &[
            "En C-durakkord består av C, E og G. Det er ikke det samme som alle tonene i C-durskalaen.",
            "Prøv å holde disse tre tonene samtidig.",
        ],
        "Hold C4, E4 og G4 samtidig.",
        Illustration::Harmony,
        Task::Chord(&[60, 64, 67]),
        &[60, 64, 67],
    ),
    step(
        "En tone vil videre",
        &[
            "B ligger i C-dur, men ikke i C-akkorden. B trekker ofte mot C.",
            "Prøv B etterfulgt av C. Lytt til oppløsningen.",
        ],
        "Spill B4 og deretter C5.",
        Illustration::Keyboard,
        Task::Sequence(&[71, 72]),
        &[71, 72],
    ),
    step(
        "Samme tone, ny rolle",
        &[
            "B er ikke akkordtone over C-dur, men er tersen i G-dur.",
            "Bytt bakgrunnsakkord og prøv B hver gang. Avslør fargene etter at du har lyttet.",
        ],
        "Spill B4. Bruk «Bytt akkord» og prøv igjen.",
        Illustration::Harmony,
        Task::Explore(71),
        &[71],
    ),
    step(
        "Finn hjem med øret",
        &[
            "Du kjenner nå forskjellen på skalatoner og akkordtoner.",
            "En tone utenfor akkorden kan være akkurat den bevegelsen musikken trenger.",
        ],
        "Spill B4 → C5 uten å se på fargene.",
        Illustration::Keyboard,
        Task::Sequence(&[71, 72]),
        &[71, 72],
    ),
];
const MAJOR_STEPS: &[Step] = &[
    step(
        "Dur er et avstandsmønster",
        &[
            "C-dur er C D E F G A B C.",
            "Mønsteret er hel–hel–halv–hel–hel–hel–halv. Hel = 2 tangenter i kromatisk avstand.",
        ],
        "Se på mønsteret. Trykk «Videre» når du er klar.",
        Illustration::MajorIntervals,
        Task::Read,
        &[60, 62, 64, 65, 67, 69, 71, 72],
    ),
    step(
        "Hør og spill skalaen",
        &[
            "Lytt først til de åtte tonene fra C til neste C.",
            "Spill så skalaen oppover i riktig rekkefølge.",
        ],
        "Spill C4 D4 E4 F4 G4 A4 B4 C5.",
        Illustration::MajorIntervals,
        Task::Sequence(&[60, 62, 64, 65, 67, 69, 71, 72]),
        &[60, 62, 64, 65, 67, 69, 71, 72],
    ),
    step(
        "Hvor er halvtrinnene?",
        &[
            "Mellom E og F og mellom B og C er det bare ett halvtonetrinn.",
            "Derfor går C-dur helt på de hvite tangentene.",
        ],
        "Spill E4 og så F4.",
        Illustration::MajorIntervals,
        Task::Sequence(&[64, 65]),
        &[64, 65],
    ),
    step(
        "Bygg en durakkord",
        &[
            "Bruk skalatrinn 1, 3 og 5 for en C-durtreklang: C E G.",
            "Tersen E ligger fire halvtoner over C.",
        ],
        "Hold C4 E4 G4 samtidig.",
        Illustration::Keyboard,
        Task::Chord(&[60, 64, 67]),
        &[60, 64, 67],
    ),
];
const MINOR_STEPS: &[Step] = &[
    step(
        "Moll bruker andre avstander",
        &[
            "Sammenlign C-dur med C naturlig moll.",
            "C-moll har E♭, A♭ og B♭ i stedet for E, A og B.",
        ],
        "Lytt til mollskalaen før du går videre.",
        Illustration::MinorIntervals,
        Task::Read,
        &[60, 62, 63, 65, 67, 68, 70, 72],
    ),
    step(
        "Spill naturlig moll",
        &[
            "Naturlig moll følger hel–halv–hel–hel–halv–hel–hel.",
            "Legg særlig merke til avstanden fra C til E♭.",
        ],
        "Spill C4 D4 E♭4 F4 G4 A♭4 B♭4 C5.",
        Illustration::MinorIntervals,
        Task::Sequence(&[60, 62, 63, 65, 67, 68, 70, 72]),
        &[60, 62, 63, 65, 67, 68, 70, 72],
    ),
    step(
        "Dur mot moll",
        &[
            "C-dur: C E G. C-moll: C E♭ G.",
            "Bare én tangent skiller treklangene: tersen.",
        ],
        "Hold C4 E♭4 G4 samtidig.",
        Illustration::MinorIntervals,
        Task::Chord(&[60, 63, 67]),
        &[60, 63, 67],
    ),
    step(
        "Moll er ikke alltid trist",
        &[
            "Dur og moll beskriver forhold mellom toner, ikke en bestemt følelse.",
            "Tempo, rytme, harmonikk og melodi påvirker også uttrykket.",
        ],
        "Spill E♭4 og E4 etter tur. Lytt til forskjellen.",
        Illustration::MinorIntervals,
        Task::Sequence(&[63, 64]),
        &[63, 64],
    ),
];
const FIFTHS_STEPS: &[Step] = &[
    step(
        "Kvintsirkelen",
        &[
            "En ren kvint er sju halvtoner. Fra C kommer du til G.",
            "Gjentar du spranget, kommer du til D og så A.",
        ],
        "Se på kvintsirkelen. Bytt toneart for å utforske.",
        Illustration::Fifths,
        Task::Read,
        &[60, 67, 74],
    ),
    step(
        "Lær spranget i fingrene",
        &[
            "C → G → D er to kvintsprang oppover.",
            "Etter D ligger neste kvint på A.",
        ],
        "Spill C4 → G4 → D5.",
        Illustration::Fifths,
        Task::Sequence(&[60, 67, 74]),
        &[60, 67, 74],
    ),
    step(
        "Hvorfor flere fortegn?",
        &[
            "Med klokken: C har ingen fortegn, G har én kryss, D har to.",
            "Mot klokken får vi i stedet flere b-er. Bytt toneart for å se mønsteret.",
        ],
        "Finn G4 på pianoet.",
        Illustration::Fifths,
        Task::Note(67),
        &[67],
    ),
    step(
        "Moll på innsiden",
        &[
            "Hver durtoneart deler tonene sine med en relativ molltoneart.",
            "C-dur hører sammen med A-moll, G-dur med E-moll.",
        ],
        "Spill A4, grunntonen til C-durs relative moll.",
        Illustration::Fifths,
        Task::Note(69),
        &[69],
    ),
];
pub const MODULES: &[Module] = &[
    Module {
        id: "find-home",
        title: "Finn hjem",
        summary: "Hør når toner hviler eller vil videre",
        steps: HOME,
    },
    Module {
        id: "major",
        title: "Dur og skalatrinn",
        summary: "Heltrinn, halvtrinn og durtreklanger",
        steps: MAJOR_STEPS,
    },
    Module {
        id: "minor",
        title: "Moll og kontraster",
        summary: "Hvordan tersen endrer lyden",
        steps: MINOR_STEPS,
    },
    Module {
        id: "fifths",
        title: "Kvintsirkelen",
        summary: "Kvintsprang, fortegn og relativ moll",
        steps: FIFTHS_STEPS,
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToneRole {
    Chord,
    Scale,
    Outside,
}
pub const FIFTHS: [(&str, &str, i8); 12] = [
    ("C", "Am", 0),
    ("G", "Em", 1),
    ("D", "Bm", 2),
    ("A", "F#m", 3),
    ("E", "C#m", 4),
    ("B", "G#m", 5),
    ("F#", "D#m", 6),
    ("Db", "Bbm", -5),
    ("Ab", "Fm", -4),
    ("Eb", "Cm", -3),
    ("Bb", "Gm", -2),
    ("F", "Dm", -1),
];
pub const FIFTH_ROOTS: [u8; 12] = [0, 7, 2, 9, 4, 11, 6, 1, 8, 3, 10, 5];
pub fn note_name(key: u8) -> &'static str {
    [
        "C", "C#", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
    ][key as usize % 12]
}
pub fn scale_contains(key: u8, root: u8, minor: bool) -> bool {
    let degree = (key as i16 - root as i16).rem_euclid(12) as u8;
    (if minor { &NATURAL_MINOR } else { &MAJOR }).contains(&degree)
}
pub fn chord_pitches(root: u8, minor: bool) -> Vec<u8> {
    [0, if minor { 3 } else { 4 }, 7]
        .into_iter()
        .map(|offset| root.saturating_add(offset))
        .collect()
}

#[derive(Default)]
pub struct Session {
    pub module: Option<usize>,
    pub step: usize,
    pub cursor: usize,
    pub reveal: bool,
    pub context: usize,
    pub fifth: usize,
    pub last_note: Option<u8>,
    pub message: &'static str,
    chord_held: BTreeSet<u8>,
}
impl Session {
    pub fn open(&mut self, module: usize) -> bool {
        if module >= MODULES.len() {
            return false;
        }
        *self = Self {
            module: Some(module),
            message: "Hør først. Prøv så på pianoet.",
            ..Self::default()
        };
        true
    }
    pub fn current(&self) -> Option<&'static Step> {
        MODULES.get(self.module?)?.steps.get(self.step)
    }
    pub fn complete(&self) -> bool {
        match self.current().map(|s| s.task) {
            Some(Task::Read) => true,
            Some(Task::Note(_) | Task::Explore(_)) => self.cursor >= 1,
            Some(Task::Chord(notes)) => self.chord_held.len() == notes.len(),
            Some(Task::Sequence(notes)) => self.cursor == notes.len(),
            None => false,
        }
    }
    pub fn next(&mut self) -> bool {
        if !self.complete() {
            return false;
        }
        let Some(module) = self.module else {
            return false;
        };
        if self.step + 1 >= MODULES[module].steps.len() {
            return false;
        }
        self.step += 1;
        self.reset_step();
        true
    }
    pub fn previous(&mut self) {
        if self.step > 0 {
            self.step -= 1;
            self.reset_step();
        }
    }
    fn reset_step(&mut self) {
        self.cursor = 0;
        self.reveal = false;
        self.context = 0;
        self.last_note = None;
        self.chord_held.clear();
        self.message = "Hør først. Prøv så på pianoet.";
    }
    pub fn press(&mut self, key: u8, held: &[u8], fresh: bool) {
        if !fresh {
            return;
        }
        let Some(task) = self.current().map(|s| s.task) else {
            return;
        };
        self.last_note = Some(key);
        if self.complete() {
            return;
        }
        match task {
            Task::Read => {}
            Task::Note(note) | Task::Explore(note) => {
                if key == note {
                    self.cursor = 1;
                    self.message = "Bra. Sammenlign med de andre tonene.";
                } else {
                    self.message = "Lytt og finn tonen som står i oppgaven.";
                }
            }
            Task::Sequence(notes) => {
                if notes.get(self.cursor) == Some(&key) {
                    self.cursor += 1;
                    self.message = if self.complete() {
                        "Mønsteret er fullført!"
                    } else {
                        "Fortsett med neste tone."
                    };
                } else {
                    self.message = "Prøv neste tone i mønsteret. Du kan begynne der du var.";
                }
            }
            Task::Chord(notes) => {
                if notes.iter().all(|note| held.contains(note)) {
                    self.chord_held.extend(notes.iter().copied());
                    self.message = "Du fant alle tre akkordtonene samtidig!";
                } else {
                    self.message = "Hold akkordtonene samtidig. Rekkefølgen spiller ingen rolle.";
                }
            }
        }
    }
    pub fn next_target(&self) -> Vec<u8> {
        if self.complete() {
            return vec![];
        }
        match self.current().map(|s| s.task) {
            Some(Task::Note(key) | Task::Explore(key)) => vec![key],
            Some(Task::Chord(keys)) => keys.to_vec(),
            Some(Task::Sequence(keys)) => keys.get(self.cursor).copied().into_iter().collect(),
            _ => vec![],
        }
    }
    pub fn chord(&self) -> Vec<u8> {
        match self.current().map(|s| s.illustration) {
            Some(Illustration::Harmony) => match self.context % 3 {
                0 => vec![60, 64, 67],
                1 => vec![65, 69, 72],
                _ => vec![67, 71, 74],
            },
            Some(Illustration::MinorIntervals) => vec![60, 63, 67],
            Some(Illustration::Fifths) => chord_pitches(60 + FIFTH_ROOTS[self.fifth], false),
            _ => vec![60, 64, 67],
        }
    }
    pub fn chord_label(&self) -> &'static str {
        if self
            .current()
            .is_some_and(|s| s.illustration == Illustration::Harmony)
        {
            ["C-dur", "F-dur", "G-dur"][self.context % 3]
        } else {
            "C-dur"
        }
    }
    pub fn role(&self, pitch: u8) -> ToneRole {
        let minor = self
            .current()
            .is_some_and(|s| s.illustration == Illustration::MinorIntervals);
        let root = if self
            .current()
            .is_some_and(|s| s.illustration == Illustration::Fifths)
        {
            FIFTH_ROOTS[self.fifth]
        } else {
            0
        };
        if self.chord().iter().any(|key| key % 12 == pitch % 12) {
            ToneRole::Chord
        } else if scale_contains(pitch, root, minor) {
            ToneRole::Scale
        } else {
            ToneRole::Outside
        }
    }
    pub fn demonstration(&self) -> Vec<Vec<u8>> {
        let Some(step) = self.current() else {
            return vec![];
        };
        match step.illustration {
            Illustration::Harmony | Illustration::Fifths
                if matches!(step.task, Task::Read | Task::Explore(_)) =>
            {
                vec![self.chord()]
            }
            _ => match step.task {
                Task::Chord(_) => vec![step.example.to_vec()],
                _ => step.example.iter().map(|&note| vec![note]).collect(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scales_transpose_without_special_casing_white_keys() {
        for root in 0..12 {
            assert_eq!(
                (0..12).filter(|&n| scale_contains(n, root, false)).count(),
                7
            );
            assert_eq!(
                (0..12).filter(|&n| scale_contains(n, root, true)).count(),
                7
            );
        }
        assert!(scale_contains(64, 0, false));
        assert!(!scale_contains(63, 0, false));
        assert!(scale_contains(63, 0, true));
        assert!(!scale_contains(64, 0, true));
    }
    #[test]
    fn same_b_has_different_chord_role() {
        let mut s = Session::default();
        assert!(s.open(0));
        s.step = 3;
        assert_eq!(s.role(71), ToneRole::Scale);
        s.context = 2;
        assert_eq!(s.role(71), ToneRole::Chord);
        assert_eq!(s.role(66), ToneRole::Outside);
    }
    #[test]
    fn sequence_is_ordered_and_ignores_duplicate_held_attack() {
        let mut s = Session::default();
        s.open(1);
        s.step = 1;
        s.press(62, &[62], true);
        assert_eq!(s.cursor, 0);
        s.press(60, &[60], true);
        s.press(62, &[60, 62], false);
        assert_eq!(s.cursor, 1);
        for note in [62, 64, 65, 67, 69, 71, 72] {
            s.press(note, &[note], true);
        }
        assert!(s.complete());
        assert!(s.next());
        assert!(!s.reveal);
    }
    #[test]
    fn chord_requires_simultaneous_notes_and_navigation_is_gated() {
        let mut s = Session::default();
        s.open(0);
        assert!(!s.next());
        s.press(60, &[60], true);
        assert!(s.next());
        s.press(60, &[60], true);
        s.press(64, &[64], true);
        assert!(!s.complete());
        s.press(67, &[60, 64, 67], true);
        assert!(s.complete());
        assert!(s.next());
        assert_eq!(s.step, 2);
    }
    #[test]
    fn fifths_are_true_semitone_fifths_and_have_relative_minors() {
        for i in 0..12 {
            let a = FIFTH_ROOTS[i] as i16;
            let b = FIFTH_ROOTS[(i + 1) % 12] as i16;
            assert_eq!((b - a).rem_euclid(12), 7);
            assert!(!FIFTHS[i].1.is_empty());
        }
        assert_eq!(FIFTHS[0], ("C", "Am", 0));
        assert_eq!(FIFTHS[1], ("G", "Em", 1));
    }
}
