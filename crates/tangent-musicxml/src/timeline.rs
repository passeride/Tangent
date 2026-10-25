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
    let mut offsets = Vec::new();
    let mut total = 0u64;
    let mut tempos = BTreeMap::new();
    for m in &measures {
        offsets.push(total);
        for &(at, bpm) in &m.tempos {
            let pos = total.checked_add(at).ok_or("Tempo position overflow")?;
            if let Some(previous) = tempos.insert(pos, bpm)
                && previous != bpm
            {
                return Err("Conflicting tempos at the same score position".into());
            }
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
    let mut notes: Vec<PerformanceNote> = Vec::new();
    let mut voice_routes = BTreeMap::new();
    let mut routes = Vec::new();
    let mut pending = BTreeMap::<(usize, u16, String, u8), (usize, u64)>::new();
    let mut written_measures = Vec::new();
    for (offset, m) in offsets.into_iter().zip(measures) {
        let mut written = Vec::new();
        for mut e in m.entries {
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
                e.note.performance_note = Some(id);
            }
            written.push(e.note);
        }
        written_measures.push(NotatedMeasure {
            label: m.label,
            duration: fraction(m.length, grid)?,
            notes: written,
        });
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
    // Releases precede new attacks at boundaries, preserving repeated keys.
    events.sort_by_key(|e| {
        (
            e.time_us,
            !matches!(e.message, PerformanceEvent::Off { .. }),
        )
    });
    notes.sort_by_key(|n| (n.start_us, n.id));
    warnings.sort();
    warnings.dedup();
    let duration_us = time(total)?.max(notes.iter().map(|n| n.end_us).max().unwrap_or(0));
    Ok(Song {
        title,
        duration_us,
        parts,
        notes,
        events,
        notation: Some(NotatedScore {
            staves,
            measures: written_measures,
        }),
        warnings,
    })
}
