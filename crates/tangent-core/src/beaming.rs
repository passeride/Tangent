//! Written rhythm grouping only. MIDI timing is not an authored meter.
use crate::notation::{Beam, BeamValue, NotatedMeasure, NoteValue, StemDirection, WrittenNote};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize)]
pub struct BeamGroup {
    /// Each column is one onset/chord; indices refer to NotatedMeasure.notes.
    pub chords: Vec<Vec<usize>>,
    pub spans: Vec<BeamSpan>,
    pub stem: Option<StemDirection>,
}
#[derive(Debug, Clone, Serialize)]
pub struct BeamSpan {
    pub level: u8,
    pub from: usize,
    pub to: usize,
    /// A partial beam reaches towards the neighbour, not all the way to it.
    pub hook: bool,
}
struct Column {
    notes: Vec<usize>,
    beams: Vec<Beam>,
    levels: u8,
    stem: Option<StemDirection>,
}
pub fn levels(note: &WrittenNote) -> u8 {
    match note.value {
        Some(NoteValue::Eighth) => 1,
        Some(NoteValue::Sixteenth) => 2,
        Some(NoteValue::ThirtySecond) => 3,
        Some(NoteValue::SixtyFourth) => 4,
        None if note.dots <= 3 => {
            // Type is optional in MusicXML. Infer only exact binary/dotted values,
            // never approximate a performed duration or guess an unknown tuplet.
            (1..=4)
                .find(|level| {
                    u128::from(note.duration.numerator) * (1u128 << (note.dots + level))
                        == u128::from(note.duration.denominator.get())
                            * ((1u128 << (note.dots + 1)) - 1)
                })
                .unwrap_or(0)
        }
        _ => 0,
    }
}
fn same_fraction(a: crate::notation::QuarterFraction, b: crate::notation::QuarterFraction) -> bool {
    u64::from(a.numerator) * u64::from(b.denominator.get())
        == u64::from(b.numerator) * u64::from(a.denominator.get())
}
fn contiguous(a: &WrittenNote, b: &WrittenNote) -> bool {
    let an = u128::from(a.onset.numerator);
    let ad = u128::from(a.onset.denominator.get());
    let dn = u128::from(a.duration.numerator);
    let dd = u128::from(a.duration.denominator.get());
    (an * dd + dn * ad) * u128::from(b.onset.denominator.get())
        == u128::from(b.onset.numerator) * ad * dd
}
fn beat(note: &WrittenNote, m: &NotatedMeasure) -> Option<u128> {
    let compound = if m.meter.beat_type >= 8 && m.meter.beats.is_multiple_of(3) {
        3
    } else {
        1
    };
    let unit = 4 * compound;
    let an = u128::from(note.onset.numerator);
    let ad = u128::from(note.onset.denominator.get());
    let dn = u128::from(note.duration.numerator);
    let dd = u128::from(note.duration.denominator.get());
    let meter = u128::from(m.meter.beat_type);
    let bucket = an * meter / (ad * unit);
    // Never silently beam a syncopation through the next beat boundary.
    ((an * dd + dn * ad) * meter <= (bucket + 1) * unit * ad * dd).then_some(bucket)
}
impl Column {
    fn primary(&self) -> Option<BeamValue> {
        self.beams.iter().find(|b| b.level == 1).map(|b| b.value)
    }
}
/// Resolve once at import. Explicit complete primary groups win over meter rules.
/// Unsupported/malformed groups safely keep individual flags.
pub fn groups(m: &NotatedMeasure) -> Vec<BeamGroup> {
    let mut lanes: BTreeMap<(u16, &str), Vec<usize>> = BTreeMap::new();
    for (i, n) in m.notes.iter().enumerate() {
        lanes.entry((n.staff, &n.voice)).or_default().push(i);
    }
    let mut result = vec![];
    for indices in lanes.values_mut() {
        indices.sort_by(|&a, &b| {
            let a = m.notes[a].onset;
            let b = m.notes[b].onset;
            (u64::from(a.numerator) * u64::from(b.denominator.get()))
                .cmp(&(u64::from(b.numerator) * u64::from(a.denominator.get())))
        });
        let mut columns: Vec<Column> = vec![];
        for &i in indices.iter() {
            let n = &m.notes[i];
            if let Some(c) = columns.last_mut()
                && same_fraction(m.notes[c.notes[0]].onset, n.onset)
            {
                let first = &m.notes[c.notes[0]];
                if levels(n) != c.levels
                    || !same_fraction(first.duration, n.duration)
                    || n.pitch.is_none()
                    || c.stem.zip(n.stem).is_some_and(|(a, b)| a != b)
                    || n.beams.iter().any(|b| {
                        c.beams
                            .iter()
                            .any(|a| a.level == b.level && a.value != b.value)
                    })
                {
                    c.levels = 0;
                }
                for b in &n.beams {
                    if !c.beams.iter().any(|a| a.level == b.level) {
                        c.beams.push(*b);
                    }
                }
                c.stem = c.stem.or(n.stem);
                c.notes.push(i);
            } else {
                columns.push(Column {
                    notes: vec![i],
                    beams: n.beams.clone(),
                    levels: if n.pitch.is_some() { levels(n) } else { 0 },
                    stem: n.stem,
                });
            }
        }
        let mut i = 0;
        while i < columns.len() {
            let start = i;
            let first = &columns[i];
            if first.levels == 0 {
                i += 1;
                continue;
            }
            let explicit = !first.beams.is_empty();
            let mut end = i + 1;
            let mut direction = first.stem;
            let mut complete = false;
            if !explicit || first.primary() == Some(BeamValue::Begin) {
                while end < columns.len() {
                    let prev = &columns[end - 1];
                    let next = &columns[end];
                    if next.levels == 0
                        || !contiguous(&m.notes[prev.notes[0]], &m.notes[next.notes[0]])
                        || direction.zip(next.stem).is_some_and(|(a, b)| a != b)
                    {
                        break;
                    }
                    if explicit {
                        if !matches!(next.primary(), Some(BeamValue::Continue | BeamValue::End)) {
                            break;
                        }
                    } else if !next.beams.is_empty()
                        || beat(&m.notes[first.notes[0]], m).is_none()
                        || beat(&m.notes[first.notes[0]], m) != beat(&m.notes[next.notes[0]], m)
                    {
                        break;
                    }
                    direction = direction.or(next.stem);
                    end += 1;
                    if explicit && next.primary() == Some(BeamValue::End) {
                        complete = true;
                        break;
                    }
                }
            }
            if end - start >= 2 && (!explicit || complete) {
                result.push(make_group(&columns[start..end], direction, explicit));
                i = end;
            } else {
                i += 1;
            }
        }
    }
    result
}
fn make_group(cols: &[Column], stem: Option<StemDirection>, explicit: bool) -> BeamGroup {
    let mut spans = vec![];
    let max_level = cols.iter().map(|c| c.levels).max().unwrap_or(1);
    for level in 1..=max_level {
        let authored = explicit
            && cols
                .iter()
                .any(|c| c.beams.iter().any(|b| b.level == level));
        let mut connected = vec![false; cols.len()];
        for i in 0..cols.len() - 1 {
            let joins = if authored {
                let a = cols[i]
                    .beams
                    .iter()
                    .find(|b| b.level == level)
                    .map(|b| b.value);
                let b = cols[i + 1]
                    .beams
                    .iter()
                    .find(|b| b.level == level)
                    .map(|b| b.value);
                matches!(a, Some(BeamValue::Begin | BeamValue::Continue))
                    && matches!(b, Some(BeamValue::Continue | BeamValue::End))
            } else {
                cols[i].levels >= level && cols[i + 1].levels >= level
            };
            if joins {
                spans.push(BeamSpan {
                    level,
                    from: i,
                    to: i + 1,
                    hook: false,
                });
                connected[i] = true;
                connected[i + 1] = true;
            }
        }
        for (i, col) in cols.iter().enumerate() {
            if level == 1 || col.levels < level || connected[i] {
                continue;
            }
            let hook = col.beams.iter().find(|b| b.level == level).map(|b| b.value);
            let forward = match hook {
                Some(BeamValue::ForwardHook) => true,
                Some(BeamValue::BackwardHook) => false,
                _ if authored => continue,
                _ => i == 0,
            };
            if forward && i + 1 == cols.len() || !forward && i == 0 {
                continue;
            }
            spans.push(BeamSpan {
                level,
                from: i,
                to: if forward { i + 1 } else { i - 1 },
                hook: true,
            });
        }
    }
    BeamGroup {
        chords: cols.iter().map(|c| c.notes.clone()).collect(),
        spans,
        stem,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        music::{PitchLetter, WrittenPitch},
        notation::*,
    };
    fn q(n: u32, d: u32) -> QuarterFraction {
        QuarterFraction {
            numerator: n,
            denominator: d.try_into().unwrap(),
        }
    }
    fn note(at: u32, denominator: u32, value: NoteValue) -> WrittenNote {
        WrittenNote {
            performance_note: None,
            staff: 0,
            voice: "1".into(),
            onset: q(at, denominator),
            duration: q(1, denominator),
            pitch: Some(WrittenPitch {
                letter: PitchLetter::C,
                alteration: 0,
                octave: 4,
            }),
            value: Some(value),
            dots: 0,
            tie_from_previous: false,
            tie_to_next: false,
            beams: vec![],
            stem: None,
        }
    }
    fn measure(notes: Vec<WrittenNote>, beats: u8, beat_type: u16) -> NotatedMeasure {
        NotatedMeasure {
            label: "1".into(),
            duration: q(u32::from(beats) * 4, u32::from(beat_type)),
            notes,
            meter: TimeSignature { beats, beat_type },
            key_fifths: 0,
            initial_quarter_bpm: 120.,
            tempos: vec![],
            repeat_start: false,
            repeat_end: None,
            ending: None,
            beam_groups: vec![],
        }
    }
    #[test]
    fn simple_and_compound_beats_have_different_groups() {
        let notes = (0..8).map(|i| note(i, 2, NoteValue::Eighth)).collect();
        let m = measure(notes, 4, 4);
        let g = groups(&m);
        assert_eq!(
            g.iter().map(|g| g.chords.len()).collect::<Vec<_>>(),
            [2, 2, 2, 2]
        );
        let m = measure(m.notes[..6].to_vec(), 6, 8);
        assert_eq!(
            groups(&m)
                .iter()
                .map(|g| g.chords.len())
                .collect::<Vec<_>>(),
            [3, 3]
        );
        let m = measure(m.notes[..3].to_vec(), 3, 8);
        assert_eq!(groups(&m)[0].chords.len(), 3);
    }
    #[test]
    fn rests_gaps_syncopations_and_lanes_do_not_get_joined() {
        let mut m = measure(
            (0..8).map(|i| note(i, 2, NoteValue::Eighth)).collect(),
            4,
            4,
        );
        m.notes[1].pitch = None;
        m.notes[3].staff = 1;
        m.notes[5].voice = "2".into();
        m.notes[7].onset = q(15, 4);
        assert!(groups(&m).is_empty());
        // Two syncopated eighths cross the beat, so retain individual flags.
        let mut a = note(1, 4, NoteValue::Eighth);
        a.duration = q(1, 2);
        let mut b = a.clone();
        b.onset = q(3, 4);
        assert!(groups(&measure(vec![a, b], 4, 4)).is_empty());
    }
    #[test]
    fn mixed_values_get_secondary_hooks_and_exact_tuplets_group() {
        let mut m = measure(
            vec![
                note(0, 2, NoteValue::Eighth),
                note(2, 4, NoteValue::Sixteenth),
                note(3, 4, NoteValue::Sixteenth),
            ],
            4,
            4,
        );
        let g = groups(&m);
        assert_eq!(
            g[0].spans
                .iter()
                .filter(|s| s.level == 2 && !s.hook)
                .count(),
            1
        );
        m.notes[1].duration = q(1, 2);
        m.notes.pop();
        let g = groups(&m);
        assert!(
            g[0].spans
                .iter()
                .any(|s| s.level == 2 && s.hook && s.from == 1 && s.to == 0)
        );
        let m = measure(
            (0..3).map(|i| note(i, 3, NoteValue::Eighth)).collect(),
            4,
            4,
        );
        assert_eq!(groups(&m)[0].chords.len(), 3);
    }
    #[test]
    fn authored_primary_and_hooks_override_auto_groups_without_inventing_connections() {
        let mut m = measure(
            (0..4).map(|i| note(i, 2, NoteValue::Sixteenth)).collect(),
            4,
            4,
        );
        for (i, n) in m.notes.iter_mut().enumerate() {
            n.beams = vec![Beam {
                level: 1,
                value: match i {
                    0 => BeamValue::Begin,
                    3 => BeamValue::End,
                    _ => BeamValue::Continue,
                },
            }];
            n.stem = Some(StemDirection::Down);
        }
        m.notes[0].beams.push(Beam {
            level: 2,
            value: BeamValue::ForwardHook,
        });
        m.notes[3].beams.push(Beam {
            level: 2,
            value: BeamValue::BackwardHook,
        });
        let g = groups(&m);
        assert_eq!(g.len(), 1);
        assert_eq!(g[0].chords.len(), 4);
        assert_eq!(g[0].stem, Some(StemDirection::Down));
        assert_eq!(
            g[0].spans.iter().filter(|s| s.level == 2 && s.hook).count(),
            2
        );
        m.notes[3].beams[0].value = BeamValue::Continue;
        assert!(
            groups(&m).is_empty(),
            "An unclosed authored group is not fabricated"
        );
    }
    #[test]
    fn missing_types_infer_only_exact_binary_or_dotted_values() {
        let mut a = note(0, 2, NoteValue::Eighth);
        a.value = None;
        let mut b = a.clone();
        b.onset = q(1, 2);
        assert_eq!(groups(&measure(vec![a.clone(), b], 4, 4)).len(), 1);
        a.duration = q(3, 4);
        a.dots = 1;
        assert_eq!(levels(&a), 1);
        a.duration = q(1, 3);
        a.dots = 0;
        assert_eq!(
            levels(&a),
            0,
            "No guessed rhythmic type for unknown tuplets"
        );
    }
    #[test]
    fn chords_share_stems_and_conflicting_stems_stay_separate() {
        let mut a = note(0, 2, NoteValue::Eighth);
        a.stem = Some(StemDirection::Up);
        let mut b = a.clone();
        b.onset = q(1, 2);
        let m = measure(vec![a.clone(), a, b.clone(), b], 4, 4);
        assert_eq!(groups(&m)[0].chords, vec![vec![0, 1], vec![2, 3]]);
        let mut m = m;
        m.notes[2].stem = Some(StemDirection::Down);
        assert!(groups(&m).is_empty());
    }
}
