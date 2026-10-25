use super::*;
use std::collections::{BTreeMap, BTreeSet};
struct Context {
    divisions: u64,
    nominal: u64,
    staff_ids: BTreeMap<u64, u16>,
    meter: TimeSignature,
    key_fifths: i8,
    ending: Option<Ending>,
}
impl Context {
    fn duration(&self, n: Node<'_, '_>, grid: u64) -> Result<u64> {
        if self.divisions == 0 {
            return Err("A duration requires divisions".into());
        }
        number(n, "duration")?
            .checked_mul(grid / self.divisions)
            .filter(|d| *d > 0 && *d < MAX_TICKS)
            .ok_or("Invalid or excessive duration".into())
    }
    fn staff(&mut self, number: u64, part: usize, staves: &mut Vec<NotatedStaff>) -> Result<u16> {
        if !(1..=16).contains(&number) {
            return Err("Staff number must be 1..16".into());
        }
        Ok(*self.staff_ids.entry(number).or_insert_with(|| {
            let id = staves.len() as u16;
            staves.push(NotatedStaff {
                id,
                part,
                clef: if number == 2 {
                    Clef::Bass
                } else {
                    Clef::Treble
                },
            });
            id
        }))
    }
}
pub(super) fn part(
    p: Node<'_, '_>,
    part: usize,
    grid: u64,
    measures: &mut Vec<Measure>,
    staves: &mut Vec<NotatedStaff>,
    warnings: &mut BTreeSet<String>,
    count: &mut usize,
) -> Result<()> {
    let mut ctx = Context {
        divisions: 0,
        nominal: grid * 4,
        staff_ids: BTreeMap::new(),
        meter: TimeSignature::default(),
        key_fifths: 0,
        ending: None,
    };
    for (index, m) in p
        .children()
        .filter(|n| n.has_tag_name("measure"))
        .enumerate()
    {
        if index >= 20_000 {
            return Err("MusicXML measure limit exceeded".into());
        }
        if index == measures.len() {
            measures.push(Measure {
                label: m.attribute("number").unwrap_or("").into(),
                ..Default::default()
            });
        }
        let target = &mut measures[index];
        if part == 0 {
            target.ending = ctx.ending.clone().map(|mut e| {
                e.start = false;
                e.end = false;
                e
            });
        }
        let mut cursor = 0u64;
        let mut extent = 0u64;
        let mut anchor = None;
        for n in m.children().filter(|n| n.is_element()) {
            match n.tag_name().name() {
                "attributes" => {
                    if child(n, "transpose").is_some() {
                        return Err(
                            "Transposing parts are not supported yet; export concert pitch".into(),
                        );
                    }
                    if let Some(d) = text(n, "divisions") {
                        ctx.divisions = d.parse().map_err(|_| "Invalid divisions")?;
                    }
                    if let Some(t) = child(n, "time") {
                        if cursor != 0 {
                            return Err("Mid-measure time signatures are not supported yet".into());
                        }
                        let beats = number(t, "beats")?;
                        let unit = number(t, "beat-type")?;
                        if unit == 0
                            || beats == 0
                            || beats > 64
                            || unit > 1024
                            || !(grid * beats * 4).is_multiple_of(unit)
                        {
                            return Err("Unsupported time signature".into());
                        }
                        ctx.nominal = grid * beats * 4 / unit;
                        ctx.meter = TimeSignature {
                            beats: beats as u8,
                            beat_type: unit as u16,
                        };
                    }
                    if let Some(key) = child(n, "key") {
                        if cursor != 0 {
                            return Err("Mid-measure key signatures are not supported yet".into());
                        }
                        let fifths: i8 = text(key, "fifths")
                            .ok_or("Only standard key signatures are supported")?
                            .parse()
                            .map_err(|_| "Invalid key signature")?;
                        if !(-7..=7).contains(&fifths) {
                            return Err("Key signature outside -7..7 fifths".into());
                        }
                        ctx.key_fifths = fifths;
                    }
                    for c in n.children().filter(|n| n.has_tag_name("clef")) {
                        let staff = c
                            .attribute("number")
                            .unwrap_or("1")
                            .parse()
                            .map_err(|_| "Invalid clef staff")?;
                        let id = ctx.staff(staff, part, staves)?;
                        if text(c, "clef-octave-change").is_some_and(|v| v != "0") {
                            return Err("Octave-transposing clefs are not supported yet".into());
                        }
                        let clef = match (text(c, "sign"), text(c, "line")) {
                            (Some("G"), Some("2") | None) => Clef::Treble,
                            (Some("F"), Some("4") | None) => Clef::Bass,
                            _ => {
                                return Err("Only treble G2 and bass F4 clefs are supported".into());
                            }
                        };
                        if index > 0 || cursor != 0 {
                            warnings.insert("Clef changes retained only as initial staff clef; display is a reading aid.".into());
                        } else {
                            staves[id as usize].clef = clef;
                        }
                    }
                }
                "backup" => {
                    cursor = cursor
                        .checked_sub(ctx.duration(n, grid)?)
                        .ok_or("Backup crosses the start of a measure")?;
                    anchor = None;
                }
                "forward" => {
                    cursor = cursor
                        .checked_add(ctx.duration(n, grid)?)
                        .filter(|v| *v < MAX_TICKS)
                        .ok_or("Excessive forward")?;
                    extent = extent.max(cursor);
                    anchor = None;
                }
                "note" => {
                    *count += 1;
                    if *count > LIMIT {
                        return Err("MusicXML note limit exceeded".into());
                    }
                    if child(n, "grace").is_some() {
                        warnings.insert("Grace notes are skipped in this import subset.".into());
                        continue;
                    }
                    if child(n, "unpitched").is_some() {
                        return Err(
                            "Unpitched percussion is not supported by the piano importer".into(),
                        );
                    }
                    let duration = ctx.duration(n, grid)?;
                    let chord = child(n, "chord").is_some();
                    let start = if chord {
                        anchor.ok_or("Chord note has no preceding anchor")?
                    } else {
                        cursor
                    };
                    let end = start
                        .checked_add(duration)
                        .filter(|v| *v < MAX_TICKS)
                        .ok_or("Excessive note position")?;
                    if !chord {
                        anchor = Some(start);
                        cursor = end;
                    }
                    extent = extent.max(end);
                    let (pitch, key) = pitch(n)?;
                    let staff = ctx.staff(
                        text(n, "staff")
                            .unwrap_or("1")
                            .parse()
                            .map_err(|_| "Invalid staff")?,
                        part,
                        staves,
                    )?;
                    let tie = |kind| {
                        n.children()
                            .any(|c| c.has_tag_name("tie") && c.attribute("type") == Some(kind))
                    };
                    target.entries.push(Entry {
                        part,
                        key,
                        start,
                        end,
                        note: WrittenNote {
                            performance_note: None,
                            staff,
                            voice: text(n, "voice").unwrap_or("1").into(),
                            onset: fraction(start, grid)?,
                            duration: fraction(duration, grid)?,
                            pitch,
                            value: match text(n, "type") {
                                Some("whole") => Some(NoteValue::Whole),
                                Some("half") => Some(NoteValue::Half),
                                Some("quarter") => Some(NoteValue::Quarter),
                                Some("eighth") => Some(NoteValue::Eighth),
                                Some("16th") => Some(NoteValue::Sixteenth),
                                Some("32nd") => Some(NoteValue::ThirtySecond),
                                Some("64th") => Some(NoteValue::SixtyFourth),
                                _ => None,
                            },
                            dots: n
                                .children()
                                .filter(|c| c.has_tag_name("dot"))
                                .count()
                                .min(3) as u8,
                            tie_from_previous: tie("stop"),
                            tie_to_next: tie("start"),
                        },
                    });
                }
                "direction" | "sound" => {
                    let offset = text(n, "offset")
                        .map(|s| s.parse::<i64>().map_err(|_| "Invalid direction offset"))
                        .transpose()?
                        .unwrap_or(0);
                    let at = i128::from(cursor)
                        + i128::from(offset) * i128::from(grid / ctx.divisions.max(1));
                    if at < 0 || at >= i128::from(MAX_TICKS) {
                        return Err("Direction offset outside measure".into());
                    }
                    for s in n.descendants().filter(|s| s.has_tag_name("sound")) {
                        if let Some(t) = s.attribute("tempo") {
                            let tempo: f64 = t.parse().map_err(|_| "Invalid tempo")?;
                            if !tempo.is_finite() || !(1.0..=1000.0).contains(&tempo) {
                                return Err("Tempo must be 1..1000 quarter notes/minute".into());
                            }
                            target.tempos.push((at as u64, tempo));
                        }
                        if s.attributes()
                            .any(|a| matches!(a.name(), "dacapo" | "dalsegno" | "tocoda" | "fine"))
                        {
                            warnings.insert(
                                "Navigation/repeats are played once in written order.".into(),
                            );
                        }
                    }
                    if !n
                        .descendants()
                        .any(|s| s.has_tag_name("sound") && s.attribute("tempo").is_some())
                    {
                        for met in n.descendants().filter(|s| s.has_tag_name("metronome")) {
                            let unit = match text(met, "beat-unit") {
                                Some("whole") => 4.,
                                Some("half") => 2.,
                                Some("quarter") => 1.,
                                Some("eighth") => 0.5,
                                Some("16th") => 0.25,
                                _ => 0.,
                            };
                            let per = text(met, "per-minute").and_then(|s| s.parse::<f64>().ok());
                            let dots = met
                                .children()
                                .filter(|s| s.has_tag_name("beat-unit-dot"))
                                .count();
                            let factor = if dots <= 3 {
                                2. - 2f64.powi(-(dots as i32))
                            } else {
                                0.
                            };
                            if let Some(per) = per
                                && unit > 0.
                                && factor > 0.
                                && (1.0..=1000.0).contains(&(per * unit * factor))
                            {
                                target.tempos.push((at as u64, per * unit * factor));
                            } else {
                                warnings.insert("Unsupported metronome relationship; current playback tempo retained.".into());
                            }
                        }
                    }
                }
                "barline" => {
                    structure::barline(n, target, &mut ctx.ending, part == 0)?;
                }
                _ => {}
            }
        }
        if part == 0 {
            target.meter = ctx.meter;
            target.key_fifths = ctx.key_fifths;
        } else if target.meter != ctx.meter {
            return Err("Polymeter between parts is not supported yet".into());
        }
        // Pickups/cadenzas marked implicit use their actual duration. Other parts
        // share the maximum measure length, so sparse voices stay synchronized.
        let length = if m.attribute("implicit") == Some("yes") {
            extent
        } else {
            extent.max(ctx.nominal)
        };
        target.length = target.length.max(length);
    }
    if ctx.ending.is_some() {
        return Err("Ending has no stop or discontinue".into());
    }
    Ok(())
}
fn pitch(n: Node<'_, '_>) -> Result<(Option<WrittenPitch>, Option<u8>)> {
    if child(n, "rest").is_some() {
        return Ok((None, None));
    }
    let p = child(n, "pitch").ok_or("Note requires pitch or rest")?;
    let (letter, semitone) = match text(p, "step") {
        Some("C") => (PitchLetter::C, 0),
        Some("D") => (PitchLetter::D, 2),
        Some("E") => (PitchLetter::E, 4),
        Some("F") => (PitchLetter::F, 5),
        Some("G") => (PitchLetter::G, 7),
        Some("A") => (PitchLetter::A, 9),
        Some("B") => (PitchLetter::B, 11),
        _ => return Err("Invalid pitch step".into()),
    };
    let alteration: i8 = text(p, "alter")
        .unwrap_or("0")
        .parse()
        .map_err(|_| "Microtonal alterations are not supported")?;
    if !(-2..=2).contains(&alteration) {
        return Err("Only double-flat through double-sharp are supported".into());
    }
    let octave: i8 = text(p, "octave")
        .ok_or("Missing octave")?
        .parse()
        .map_err(|_| "Invalid octave")?;
    let key = (i16::from(octave) + 1) * 12 + semitone + i16::from(alteration);
    let key = u8::try_from(key)
        .ok()
        .filter(|k| *k < 128)
        .ok_or("Pitch outside MIDI 0..127")?;
    Ok((
        Some(WrittenPitch {
            letter,
            alteration,
            octave,
        }),
        Some(key),
    ))
}
