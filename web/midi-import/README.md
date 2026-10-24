# Browser MIDI import harness

This page calls the same Rust importer as native consumers. It is not the Bevy
app and does not play sound, request MIDI device access or upload files. Files
are kept in memory until reload. The included three-note byte sequence is new,
original test data authored in this repository.

From the repository root, with a Rust toolchain installed:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.129 --locked
cargo build -p tangent-midi-web --target wasm32-unknown-unknown --release --locked
wasm-bindgen target/wasm32-unknown-unknown/release/tangent_midi_web.wasm --target web --out-dir web/midi-import/pkg
python3 -m http.server 8080 --directory web/midi-import
```

Open `http://localhost:8080`. The wasm-bindgen CLI version must match the exact
wasm-bindgen crate in Cargo.lock; update the command if that dependency changes.
The listed CLI version is checked during this implementation's validation.

Try the study, a type-0 MIDI, a multitrack MIDI and an invalid file. Errors stay
visible and the file picker remains usable. The JSON panel includes metadata,
controllers, source indices, note spans and diagnostics. Rendering dense JSON is
for inspection, not the future production UI.

Generated `pkg/` files are not committed. CI builds the harness as a downloadable
artifact; it does not deploy a website. These instructions need no Node packages.

## Browser smoke test

After generating `pkg/` as above:

```sh
npm ci --prefix web/midi-import
cd web/midi-import
npx playwright install chromium
npm test
```

The test starts and closes its own local server. It runs actual WASM in Chromium,
checks the three-note study, imports a malformed file and retries successfully.
On Linux the browser may need system packages (`npx playwright install --with-deps chromium`).
