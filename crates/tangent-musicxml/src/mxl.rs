//! Read only the declared MusicXML score from a bounded MXL/ZIP container.
//! No filesystem extraction, resource loading, ZIP64 or encrypted archives.
use roxmltree::{Document, ParsingOptions};
use std::collections::HashSet;

const MAX_XML: usize = 16 * 1024 * 1024;
const MAX_CONTAINER: usize = 64 * 1024;
const MAX_ENTRIES: usize = 4096;

struct Entry<'a> {
    name: &'a str,
    method: u16,
    flags: u16,
    crc: u32,
    size: usize,
    packed: usize,
    offset: usize,
}

fn u16_at(data: &[u8], at: usize) -> Result<u16, String> {
    let b = data.get(at..at.checked_add(2).ok_or("Invalid MXL offset")?)
        .ok_or("Truncated MXL ZIP header")?;
    Ok(u16::from_le_bytes([b[0], b[1]]))
}
fn u32_at(data: &[u8], at: usize) -> Result<u32, String> {
    let b = data.get(at..at.checked_add(4).ok_or("Invalid MXL offset")?)
        .ok_or("Truncated MXL ZIP header")?;
    Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}
fn field<'a>(data: &'a [u8], at: usize, len: usize) -> Result<&'a [u8], String> {
    data.get(at..at.checked_add(len).ok_or("Invalid MXL offset")?)
        .ok_or_else(|| "Truncated MXL ZIP entry".into())
}
fn valid_path(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains(['\\', '\0', ':'])
        && path.split('/').all(|part| !part.is_empty() && part != "." && part != "..")
}
fn entries(bytes: &[u8]) -> Result<Vec<Entry<'_>>, String> {
    // EOCD can have a comment of up to 65535 bytes. Ignore signatures embedded
    // inside the comment by requiring its declared length to reach EOF.
    let start = bytes.len().saturating_sub(22 + 65_535);
    let end = bytes.len().checked_sub(22).ok_or("Truncated MXL archive")?;
    let eocd = (start..=end).rev().find(|&p| {
        bytes.get(p..p + 4) == Some(b"PK\x05\x06".as_slice())
            && u16_at(bytes, p + 20).is_ok_and(|n| p + 22 + n as usize == bytes.len())
    }).ok_or("Missing MXL ZIP central directory")?;
    if u16_at(bytes, eocd + 4)? != 0 || u16_at(bytes, eocd + 6)? != 0
        || u16_at(bytes, eocd + 8)? != u16_at(bytes, eocd + 10)?
    {
        return Err("Multi-disk MXL archives are not supported".into());
    }
    let count = u16_at(bytes, eocd + 10)? as usize;
    let dir_size = u32_at(bytes, eocd + 12)? as usize;
    let dir_start = u32_at(bytes, eocd + 16)? as usize;
    if count == 0 || count > MAX_ENTRIES || count == u16::MAX
        || dir_size == u32::MAX as usize || dir_start == u32::MAX as usize
    {
        return Err("MXL ZIP entry count or ZIP64 format is unsupported".into());
    }
    let dir_end = dir_start.checked_add(dir_size).ok_or("Invalid MXL ZIP directory")?;
    if dir_end > eocd {
        return Err("Invalid MXL ZIP directory offset".into());
    }
    let mut entries = Vec::with_capacity(count);
    let mut seen = HashSet::new();
    let mut pos = dir_start;
    for _ in 0..count {
        if u32_at(bytes, pos)? != 0x02014b50 {
            return Err("Invalid MXL ZIP central directory entry".into());
        }
        let flags = u16_at(bytes, pos + 8)?;
        let method = u16_at(bytes, pos + 10)?;
        let crc = u32_at(bytes, pos + 16)?;
        let packed = u32_at(bytes, pos + 20)? as usize;
        let size = u32_at(bytes, pos + 24)? as usize;
        let name_len = u16_at(bytes, pos + 28)? as usize;
        let extra_len = u16_at(bytes, pos + 30)? as usize;
        let comment_len = u16_at(bytes, pos + 32)? as usize;
        if u16_at(bytes, pos + 34)? != 0 || u32_at(bytes, pos + 42)? == u32::MAX {
            return Err("Multi-disk or ZIP64 MXL entry unsupported".into());
        }
        let offset = u32_at(bytes, pos + 42)? as usize;
        let name_at = pos.checked_add(46).ok_or("Invalid MXL ZIP header")?;
        let name = std::str::from_utf8(field(bytes, name_at, name_len)?)
            .map_err(|_| "MXL ZIP filename must be UTF-8")?;
        let next = name_at.checked_add(name_len)
            .and_then(|p| p.checked_add(extra_len))
            .and_then(|p| p.checked_add(comment_len))
            .ok_or("Invalid MXL ZIP entry size")?;
        if next > dir_end || !valid_path(name.trim_end_matches('/')) {
            return Err("Invalid MXL ZIP entry name or directory length".into());
        }
        if !seen.insert(name) {
            return Err("Duplicate MXL ZIP entry name".into());
        }
        entries.push(Entry { name, flags, method, crc, size, packed, offset });
        pos = next;
    }
    if pos != dir_end {
        return Err("MXL ZIP directory size mismatch".into());
    }
    Ok(entries)
}
fn read_entry(bytes: &[u8], entry: &Entry<'_>, limit: usize) -> Result<Vec<u8>, String> {
    if entry.flags & (1 | (1 << 6)) != 0 {
        return Err("Encrypted MXL ZIP entries are not supported".into());
    }
    if entry.size > limit {
        return Err("MXL decompressed entry exceeds size limit".into());
    }
    if u32_at(bytes, entry.offset)? != 0x04034b50 {
        return Err("Invalid MXL ZIP local header".into());
    }
    if u16_at(bytes, entry.offset + 6)? != entry.flags
        || u16_at(bytes, entry.offset + 8)? != entry.method
    {
        return Err("MXL ZIP local/central header mismatch".into());
    }
    let name_len = u16_at(bytes, entry.offset + 26)? as usize;
    let extra_len = u16_at(bytes, entry.offset + 28)? as usize;
    let name_at = entry.offset.checked_add(30).ok_or("Invalid MXL ZIP offset")?;
    if field(bytes, name_at, name_len)? != entry.name.as_bytes() {
        return Err("MXL ZIP local filename mismatch".into());
    }
    let at = name_at.checked_add(name_len).and_then(|p| p.checked_add(extra_len))
        .ok_or("Invalid MXL ZIP data offset")?;
    let packed = field(bytes, at, entry.packed)?;
    let data = match entry.method {
        0 => packed.to_vec(),
        8 => miniz_oxide::inflate::decompress_to_vec_with_limit(packed, limit)
            .map_err(|_| "Invalid or oversized MXL deflate stream")?,
        _ => return Err("Unsupported MXL ZIP compression method".into()),
    };
    if data.len() != entry.size || crc32fast::hash(&data) != entry.crc {
        return Err("MXL ZIP size or CRC mismatch".into());
    }
    Ok(data)
}
pub(super) fn unpack(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let files = entries(bytes)?;
    let container = files.iter().find(|e| e.name == "META-INF/container.xml")
        .ok_or("MXL is missing META-INF/container.xml")?;
    let manifest = read_entry(bytes, container, MAX_CONTAINER)?;
    let xml = std::str::from_utf8(&manifest).map_err(|_| "MXL container.xml must be UTF-8")?;
    let doc = Document::parse_with_options(xml, ParsingOptions {
        allow_dtd: false,
        nodes_limit: 1024,
    }).map_err(|e| format!("Invalid MXL container.xml: {e}"))?;
    let root = doc.root_element();
    if !root.has_tag_name("container") {
        return Err("Invalid MXL container root".into());
    }
    let roots = root.children().find(|n| n.has_tag_name("rootfiles"))
        .ok_or("Missing MXL rootfiles")?;
    // The first rootfile must be the main MusicXML document, per MusicXML 4.0.
    let score = roots.children().find(|n| n.has_tag_name("rootfile"))
        .ok_or("Missing MXL rootfile")?;
    if score.attribute("media-type").is_some_and(|kind| {
        kind != "application/vnd.recordare.musicxml+xml"
    }) {
        return Err("First MXL rootfile is not MusicXML".into());
    }
    let path = score.attribute("full-path").ok_or("Missing MXL rootfile full-path")?;
    if !valid_path(path) {
        return Err("Unsafe MXL rootfile path".into());
    }
    let entry = files.iter().find(|e| e.name == path && !e.name.ends_with('/'))
        .ok_or("MXL rootfile is missing from ZIP archive")?;
    read_entry(bytes, entry, MAX_XML)
}
