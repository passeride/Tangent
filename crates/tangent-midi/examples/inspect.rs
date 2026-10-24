use std::io::Read;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os()
        .nth(1)
        .ok_or("Usage: inspect path/to/piece.mid")?;
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(tangent_midi::Limits::default().bytes as u64 + 1)
        .read_to_end(&mut bytes)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&tangent_midi::import(&bytes)?)?
    );
    Ok(())
}
