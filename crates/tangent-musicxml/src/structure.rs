//! Written repeat structure is distinct from expanded performance order.
use super::*;
pub(super) fn barline(
    n: Node<'_, '_>,
    target: &mut Measure,
    active: &mut Option<Ending>,
    primary: bool,
) -> Result<()> {
    if let Some(r) = child(n, "repeat") {
        match r.attribute("direction") {
            Some("forward") => target.repeat_start = true,
            Some("backward") => {
                let times = r
                    .attribute("times")
                    .unwrap_or("2")
                    .parse::<u8>()
                    .map_err(|_| "Invalid repeat count")?;
                if !(2..=8).contains(&times) {
                    return Err("Repeat count must be 2..8".into());
                }
                if target.repeat_end.is_some_and(|v| v != times) {
                    return Err("Conflicting repeat counts between parts".into());
                }
                target.repeat_end = Some(times);
            }
            _ => return Err("Invalid repeat direction".into()),
        }
    }
    if let Some(e) = child(n, "ending") {
        let numbers: Vec<u8> = e
            .attribute("number")
            .ok_or("Ending needs numbers")?
            .split(',')
            .map(|s| {
                s.trim()
                    .parse::<u8>()
                    .map_err(|_| "Unsupported ending number (use comma-separated passes)")
            })
            .collect::<std::result::Result<_, _>>()?;
        if numbers.is_empty() || numbers.iter().any(|n| !(1..=8).contains(n)) {
            return Err("Ending numbers must be 1..8".into());
        }
        match e.attribute("type") {
            Some("start") => {
                *active = Some(Ending {
                    numbers,
                    start: true,
                    end: false,
                    open_end: false,
                });
                if primary {
                    target.ending = active.clone();
                }
            }
            Some("stop" | "discontinue") => {
                if primary {
                    let mut end = target
                        .ending
                        .clone()
                        .or_else(|| active.clone())
                        .ok_or("Ending stop without start")?;
                    if end.numbers != numbers {
                        return Err("Ending numbers disagree at stop".into());
                    }
                    end.end = true;
                    end.open_end = e.attribute("type") == Some("discontinue");
                    target.ending = Some(end);
                }
                *active = None;
            }
            _ => return Err("Invalid ending type".into()),
        }
    }
    Ok(())
}
pub(super) fn order(measures: &[Measure]) -> Result<Vec<(usize, u8)>> {
    let mut regions = std::collections::BTreeMap::new();
    let mut open = None;
    let mut previous_end = None;
    for (i, m) in measures.iter().enumerate() {
        if m.repeat_start {
            if open.is_some() {
                return Err("Nested repeats are not supported yet".into());
            }
            open = Some(i);
        }
        if let Some(times) = m.repeat_end {
            let start = open
                .take()
                .unwrap_or_else(|| previous_end.map_or(0, |e| e + 1));
            regions.insert(start, (i, times));
            previous_end = Some(i);
        }
    }
    if open.is_some() {
        return Err("Forward repeat has no backward repeat".into());
    }
    let mut result = Vec::new();
    let mut index = 0;
    let mut last_pass = 1;
    while index < measures.len() {
        if let Some(&(end, times)) = regions.get(&index) {
            for pass in 1..=times {
                for (i, m) in measures.iter().enumerate().take(end + 1).skip(index) {
                    if m.ending.as_ref().is_none_or(|e| e.numbers.contains(&pass)) {
                        result.push((i, pass));
                    }
                }
            }
            index = end + 1;
            last_pass = times;
        } else {
            if measures[index]
                .ending
                .as_ref()
                .is_none_or(|e| e.numbers.contains(&last_pass))
            {
                result.push((index, last_pass));
            }
            index += 1;
        }
        if result.len() > 80_000 {
            return Err("Expanded repeat measure limit exceeded".into());
        }
    }
    Ok(result)
}
