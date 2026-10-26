//! Real desktop entry point: native Bevy window, audio, MIDI and SQLite.
fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.iter().any(|a| a == "--licenses") {
        println!("Tangent third-party dependency notices:\n{}", include_str!("../../../THIRD_PARTY_NOTICES.txt"));
        println!("\nBravura font license:\n{}", include_str!("../../../licenses/Bravura-OFL.txt"));
        println!("\nNoto Sans font license:\n{}", include_str!("../../../licenses/NotoSans-OFL.txt"));
        println!("\nFira Mono font license:\n{}", include_str!("../../../licenses/FiraMono-OFL.txt"));
        return;
    }
    if args.iter().any(|a| a == "--self-test") {
        let player = tangent_player::Player::new();
        assert!(player.song_json().contains("First light"));
        println!("Tangent native player: core and embedded music OK");
        return;
    }
    tangent_player::run_native_player();
}
