use super::*;
use std::collections::BTreeMap;
pub(super) fn song(
    title: String,
    grid: u64,
    parts: Vec<Part>,
    staves: Vec<NotatedStaff>,
    measures: Vec<Measure>,
    mut warnings: Vec<String>,
) -> Result<Song> {
    let order = structure::order(&measures)?;
    // Establish written tempo context; repeat jumps restore the tempo at their target.
    let mut context = Vec::new();
    let mut bpm = 120.;
    for m in &measures {
        context.push(bpm);
        let mut marks = BTreeMap::new();
        for &(at, tempo) in &m.tempos {
            if let Some(previous) = marks.insert(at, tempo)
                && previous != tempo
            {
                return Err("Conflicting tempos at the same score position".into());
            }
        }
        for tempo in marks.values() {
            bpm = *tempo;
        }
    }
    let mut offsets = Vec::new();
    let mut total = 0u64;
    let mut tempos = BTreeMap::new();
    for &(index, _) in &order {
        let m = &measures[index];
        offsets.push(total);
        tempos.insert(total, context[index]);
        for &(at, tempo) in &m.tempos {
            if at > m.length {
                return Err("Tempo offset exceeds measure duration".into());
            }
            tempos.insert(total + at, tempo);
        }
        total = total
            .checked_add(m.length)
            .filter(|v| *v < MAX_TICKS)
            .ok_or("Score duration too large")?;
    }
    tempos.entry(0).or_insert(120.);
    let mut segments = Vec::new();
    let mut last_tick = 0;
    let mut elapsed = 0.;
    let mut bpm = 120.;
    for (at, next_bpm) in tempos {
        elapsed += (at - last_tick) as f64 / grid as f64 * 60_000_000. / bpm;
        segments.push((at, elapsed, next_bpm));
        last_tick = at;
        bpm = next_bpm;
    }
    let time = |tick: u64| -> Result<u64> {
        let i = segments
            .partition_point(|(at, _, _)| *at <= tick)
            .saturating_sub(1);
        let (at, elapsed, bpm) = segments[i];
        let us = elapsed + (tick - at) as f64 / grid as f64 * 60_000_000. / bpm;
        if us > 86_400_000_000. {
            return Err("MusicXML exceeds 24 hours".into());
        }
        Ok(us.round() as u64)
    };
    let mut written_measures = Vec::new();
    for (index, m) in measures.iter().enumerate() {
        let mut written = NotatedMeasure {
            beam_groups: vec![],
            label: m.label.clone(),
            duration: fraction(m.length, grid)?,
            notes: m.entries.iter().map(|e| e.note.clone()).collect(),
            meter: m.meter,
            key_fifths: m.key_fifths,
            initial_quarter_bpm: m
                .tempos
                .iter()
                .rfind(|(at, _)| *at == 0)
                .map_or(context[index], |(_, bpm)| *bpm),
            tempos: m
                .tempos
                .iter()
                .map(|&(at, bpm)| {
                    Ok(TempoMark {
                        onset: fraction(at, grid)?,
                        quarter_bpm: bpm,
                    })
                })
                .collect::<Result<_>>()?,
            repeat_start: m.repeat_start,
            repeat_end: m.repeat_end,
            ending: m.ending.clone(),
        };
        written.beam_groups = tangent_core::beaming::groups(&written);
        written_measures.push(written);
    }
    let mut notes: Vec<PerformanceNote> = Vec::new();
    let mut voice_routes = BTreeMap::new();
    let mut routes = Vec::new();
    let mut pending = BTreeMap::<(usize, u16, String, u8), (usize, u64)>::new();
    let mut visits = Vec::new();
    let mut previous_measure = None;
    for ((index, pass), offset) in order.into_iter().zip(offsets) {
        if previous_measure.is_some_and(|prev| index != prev + 1) {
            pending.clear();
        }
        previous_measure = Some(index);
        let m = &measures[index];
        let mut timing_ticks = std::collections::BTreeSet::from([0, m.length]);
        timing_ticks.extend(m.tempos.iter().map(|(at, _)| *at));
        let timing = timing_ticks
            .into_iter()
            .map(|at| {
                Ok(BeatTime {
                    quarter: fraction(at, grid)?,
                    offset_us: time(offset + at)? - time(offset)?,
                })
            })
            .collect::<Result<_>>()?;
        visits.push(MeasureVisit {
            measure: index,
            pass,
            start_us: time(offset)?,
            end_us: time(offset + m.length)?,
            timing,
        });
        for (written_index, e) in m.entries.iter().enumerate() {
            if let Some(key) = e.key {
                let lane = (e.part, e.note.staff, e.note.voice.clone(), key);
                let start = offset + e.start;
                let end = offset + e.end;
                let preceding = if e.note.tie_from_previous {
                    pending
                        .remove(&lane)
                        .filter(|(_, t)| *t == start)
                        .map(|(id, _)| id)
                } else {
                    None
                };
                let id = if let Some(id) = preceding {
                    notes[id].end_us = time(end)?;
                    id
                } else {
                    if e.note.tie_from_previous {
                        warnings.push(
                            "Tie stop without adjacent matching start; played as a new note."
                                .into(),
                        );
                    }
                    if notes.len() >= LIMIT {
                        return Err("Expanded MusicXML note limit exceeded".into());
                    }
                    let next_route = voice_routes.len();
                    let route = *voice_routes
                        .entry((e.part, e.note.staff, e.note.voice.clone()))
                        .or_insert(next_route);
                    if route >= 3840 {
                        return Err("Too many independent playback voices".into());
                    }
                    routes.push(route);
                    let id = notes.len();
                    notes.push(PerformanceNote {
                        id,
                        part: e.part,
                        key,
                        velocity: 85,
                        start_us: time(start)?,
                        end_us: time(end)?,
                        written: e.note.pitch.clone(),
                    });
                    id
                };
                if e.note.tie_to_next {
                    pending.insert(lane, (id, end));
                }
                written_measures[index].notes[written_index]
                    .performance_note
                    .get_or_insert(id);
            }
        }
    }
    if !pending.is_empty() {
        warnings.push("Unclosed ties end at their last written duration.".into());
    }
    let mut events = Vec::with_capacity(notes.len() * 2);
    for n in &notes {
        let port = (routes[n.id] / 15) as u8;
        let lane = (routes[n.id] % 15) as u8;
        let channel = if lane >= 9 { lane + 1 } else { lane };
        events.push(TimedEvent {
            time_us: n.start_us,
            port,
            channel,
            message: PerformanceEvent::On {
                key: n.key,
                velocity: n.velocity,
            },
        });
        events.push(TimedEvent {
            time_us: n.end_us,
            port,
            channel,
            message: PerformanceEvent::Off { key: n.key },
        });
    }
    events.sort_by_key(|e| {
        (
            e.time_us,
            !matches!(e.message, PerformanceEvent::Off { .. }),
        )
    });
    notes.sort_by_key(|n| (n.start_us, n.id));
    warnings.sort();
    warnings.dedup();
    Ok(Song {
        composer: None,
        title,
        duration_us: time(total)?.max(notes.iter().map(|n| n.end_us).max().unwrap_or(0)),
        parts,
        notes,
        events,
        notation: Some(NotatedScore {
            staves,
            measures: written_measures,
            playback: visits,
        }),
        warnings,
    })
}
