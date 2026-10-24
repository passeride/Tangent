# Third-party dependencies

Recorded 2026-10-24 from this repository's generated Cargo.lock and package manifests. The application code has no project-wide license selection yet; the dependencies below retain their own licenses. No package or lockfile was copied from the Neothesia fork.

The importer uses midly directly; serde derives the owned inspection model. serde_json implements the inspection boundary. wasm-bindgen supplies browser bindings. Other entries are resolved supporting/build packages. No Bevy, audio engine, font or SoundFont is included in this slice.

| Package | Version | Declared license | Original repository |
| --- | --- | --- | --- |
| bumpalo | 3.20.3 | MIT OR Apache-2.0 | https://github.com/fitzgen/bumpalo |
| cfg-if | 1.0.5 | MIT OR Apache-2.0 | https://github.com/rust-lang/cfg-if |
| itoa | 1.0.18 | MIT OR Apache-2.0 | https://github.com/dtolnay/itoa |
| memchr | 2.8.3 | Unlicense OR MIT | https://github.com/BurntSushi/memchr |
| midly | 0.5.3 | Unlicense | https://github.com/negamartin/midly |
| once_cell | 1.21.4 | MIT OR Apache-2.0 | https://github.com/matklad/once_cell |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 | https://github.com/dtolnay/proc-macro2 |
| quote | 1.0.47 | MIT OR Apache-2.0 | https://github.com/dtolnay/quote |
| rustversion | 1.0.23 | MIT OR Apache-2.0 | https://github.com/dtolnay/rustversion |
| serde | 1.0.229 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| serde_core | 1.0.229 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| serde_derive | 1.0.229 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| serde_json | 1.0.154 | MIT OR Apache-2.0 | https://github.com/serde-rs/json |
| syn | 3.0.9 | MIT OR Apache-2.0 | https://github.com/dtolnay/syn |
| unicode-ident | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 | https://github.com/dtolnay/unicode-ident |
| wasm-bindgen | 0.2.129 | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen |
| wasm-bindgen-macro | 0.2.129 | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/macro |
| wasm-bindgen-macro-support | 0.2.129 | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen/tree/main/crates/macro-support |
| wasm-bindgen-shared | 0.2.129 | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/shared |
| zmij | 1.0.23 | MIT | https://github.com/dtolnay/zmij |

Exact Cargo package checksums are in Cargo.lock. Corresponding license texts are in [THIRD_PARTY_NOTICES.txt](THIRD_PARTY_NOTICES.txt). This inventory is not a legal opinion or a clean room certification.

## Browser-test tooling

The optional Node smoke-test tooling is locked separately in web/midi-import/package-lock.json: Playwright and playwright-core 1.56.1 (Apache-2.0), and optional fsevents 2.3.2 (MIT). These are test tools, not dependencies shipped with the import page. Sources: https://github.com/microsoft/playwright and https://github.com/fsevents/fsevents. Chromium is downloaded by Playwright for tests and is not bundled into the app.

## Updating

When dependencies change, regenerate/check the license inventory and notices, including compound licenses such as Unicode-3.0. Verify features and native/wasm behavior. The wasm-bindgen CLI version must match the crate in Cargo.lock.
