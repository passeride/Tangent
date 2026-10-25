//! Bounded MusicXML partwise adapter. No renderer, I/O, XML entity expansion or MIDI dependency.
mod parse;
mod timeline;
use roxmltree::{Document, Node};
use tangent_core::{music::*, notation::*};
type Result<T> = std::result::Result<T, String>;
const LIMIT: usize = 250_000;
const MAX_TICKS: u64 = 1_000_000_000_000;
fn child<'a, 'i>(n: Node<'a, 'i>, name: &str) -> Option<Node<'a, 'i>> {
    n.children().find(|c| c.has_tag_name(name))
}
fn text<'a>(n: Node<'a, '_>, name: &str) -> Option<&'a str> {
    child(n, name).and_then(|c| c.text()).map(str::trim)
}
fn number(n: Node<'_, '_>, name: &str) -> Result<u64> {
    text(n, name)
        .ok_or_else(|| format!("Missing {name}"))?
        .parse()
        .map_err(|_| format!("Invalid {name}"))
}
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn fraction(ticks: u64, grid: u64) -> Result<QuarterFraction> {
    let divisor = gcd(ticks, grid);
    Ok(QuarterFraction {
        numerator: u32::try_from(ticks / divisor).map_err(|_| "Notation duration too large")?,
        denominator: std::num::NonZeroU32::new((grid / divisor) as u32).unwrap(),
    })
}
#[derive(Clone)]
struct Entry {
    note: WrittenNote,
    key: Option<u8>,
    start: u64,
    end: u64,
    part: usize,
}
#[derive(Default)]
struct Measure {
    label: String,
    length: u64,
    entries: Vec<Entry>,
    tempos: Vec<(u64, f64)>,
}
/// Supports UTF-8 score-partwise .musicxml/.xml. Compressed .mxl is explicitly rejected.
pub fn import(bytes: &[u8], fallback_title: String) -> Result<Song> {
    if bytes.len() > 16 * 1024 * 1024 {
        return Err("MusicXML exceeds 16 MiB".into());
    }
    if bytes.starts_with(b"PK") {
        return Err(
            "Compressed MusicXML (.mxl) is not supported yet. Export uncompressed .musicxml."
                .into(),
        );
    }
    let xml = std::str::from_utf8(bytes).map_err(|_| "MusicXML must be UTF-8")?;
    // roxmltree does not resolve external entities. Permit the common external
    // MusicXML DOCTYPE, but never enable DTD/internal entity parsing.
    let xml = without_external_doctype(xml)?;
    let doc = Document::parse_with_options(
        &xml,
        roxmltree::ParsingOptions {
            allow_dtd: false,
            nodes_limit: 1_000_000,
        },
    )
    .map_err(|e| format!("Invalid MusicXML: {e}"))?;
    if doc.descendants().count() > 1_000_000 {
        return Err("MusicXML node limit exceeded".into());
    }
    let root = doc.root_element();
    if !root.has_tag_name("score-partwise") {
        return Err("Expected MusicXML score-partwise (timewise/opus unsupported)".into());
    }
    let mut grid = 1u64;
    for n in root.descendants().filter(|n| n.has_tag_name("divisions")) {
        let d: u64 = n
            .text()
            .unwrap_or("")
            .trim()
            .parse()
            .map_err(|_| "Invalid divisions")?;
        if d == 0 {
            return Err("Divisions must be positive".into());
        }
        grid = (grid / gcd(grid, d))
            .checked_mul(d)
            .filter(|g| *g <= 1_000_000_000)
            .ok_or("MusicXML rhythmic resolution is too large")?;
    }
    let mut measures = Vec::<Measure>::new();
    let mut parts = Vec::new();
    let mut staves = Vec::new();
    let mut warnings = std::collections::BTreeSet::new();
    let listed = child(root, "part-list").ok_or("Missing part-list")?;
    let mut ids = std::collections::BTreeSet::new();
    let mut count = 0;
    for p in root.children().filter(|n| n.has_tag_name("part")) {
        let id = p.attribute("id").ok_or("Missing part id")?;
        if !ids.insert(id) {
            return Err("Duplicate part id".into());
        }
        let definition = listed
            .children()
            .find(|n| n.has_tag_name("score-part") && n.attribute("id") == Some(id))
            .ok_or("Part missing from part-list")?;
        if parts.len() >= 128 {
            return Err("MusicXML supports at most 128 parts".into());
        }
        let part = parts.len();
        parts.push(Part {
            name: text(definition, "part-name").unwrap_or(id).into(),
            percussion: false,
        });
        parse::part(
            p,
            part,
            grid,
            &mut measures,
            &mut staves,
            &mut warnings,
            &mut count,
        )?;
    }
    if parts.is_empty() {
        return Err("MusicXML has no parts".into());
    }
    let title = child(root, "work")
        .and_then(|w| text(w, "work-title"))
        .or_else(|| text(root, "movement-title"))
        .unwrap_or(&fallback_title)
        .to_string();
    timeline::song(
        title,
        grid,
        parts,
        staves,
        measures,
        warnings.into_iter().collect(),
    )
}

// A normal external MusicXML declaration is metadata, never a resource request.
// Remove only a single declaration with no internal subset; entity parsing stays off.
fn without_external_doctype(xml: &str) -> Result<std::borrow::Cow<'_, str>> {
    let Some(start) = xml.find("<!DOCTYPE") else {
        return Ok(xml.into());
    };
    let mut quote = None;
    for (i, byte) in xml.as_bytes().iter().copied().enumerate().skip(start + 9) {
        if let Some(q) = quote {
            if byte == q {
                quote = None;
            }
            continue;
        }
        match byte {
            b'\'' | b'"' => quote = Some(byte),
            b'[' => return Err("Internal XML DTD/entity declarations are not supported".into()),
            b'>' => {
                let mut clean = xml.to_string();
                clean.replace_range(start..=i, &" ".repeat(i + 1 - start));
                return Ok(clean.into());
            }
            _ => {}
        }
    }
    Err("Unterminated DOCTYPE".into())
}
