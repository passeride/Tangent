//! Real desktop entry point: native Bevy window, audio, MIDI and SQLite.
fn main() {
    if std::env::args().any(|a| a == "--self-test") {
        let player = tangent_player::Player::new();
        assert!(player.song_json().contains("First light"));
        println!("Tangent native player: core and embedded music OK");
        return;
    }
    tangent_player::run_native_player();
}
