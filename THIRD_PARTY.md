# Third-party dependencies

Recorded 2026-10-24 from generated Cargo metadata/lockfiles. This includes resolved optional/build/target dependencies, not a claim that each is linked into the browser bundle. No legacy lockfile or package source was copied. The project-wide license remains unselected.

Direct dependencies include midly (import), serde/serde_json (owned models/bridges), wasm-bindgen (browser bindings), and Bevy app/ECS 0.20.0 with default features disabled and std enabled. The latter are the application core; the full Bevy renderer is not linked. See ADR-0003 for browser audio/rendering boundaries.

| Package | Version | Declared license | Original repository |
| --- | --- | --- | --- |
| arrayvec | 0.7.8 | MIT OR Apache-2.0 | https://github.com/bluss/arrayvec |
| assert_type_match | 0.1.1 | MIT OR Apache-2.0 | https://github.com/MrGVSV/assert_type_match |
| async-task | 4.7.1 | Apache-2.0 OR MIT | https://github.com/smol-rs/async-task |
| atomic-waker | 1.1.2 | Apache-2.0 OR MIT | https://github.com/smol-rs/atomic-waker |
| bevy_app | 0.20.0 | MIT OR Apache-2.0 | https://github.com/bevyengine/bevy |
| bevy_derive | 0.20.0 | MIT OR Apache-2.0 | https://github.com/bevyengine/bevy |
| bevy_ecs | 0.20.0 | MIT OR Apache-2.0 | https://github.com/bevyengine/bevy |
| bevy_ecs_macro_logic | 0.20.0 | MIT OR Apache-2.0 | https://crates.io/crates/bevy_ecs_macro_logic |
| bevy_ecs_macros | 0.20.0 | MIT OR Apache-2.0 | https://crates.io/crates/bevy_ecs_macros |
| bevy_macro_utils | 0.20.0 | MIT OR Apache-2.0 | https://github.com/bevyengine/bevy |
| bevy_platform | 0.20.0 | MIT OR Apache-2.0 | https://github.com/bevyengine/bevy |
| bevy_ptr | 0.20.0 | MIT OR Apache-2.0 | https://github.com/bevyengine/bevy |
| bevy_reflect | 0.20.0 | MIT OR Apache-2.0 | https://github.com/bevyengine/bevy |
| bevy_reflect_derive | 0.20.0 | MIT OR Apache-2.0 | https://github.com/bevyengine/bevy |
| bevy_tasks | 0.20.0 | MIT OR Apache-2.0 | https://github.com/bevyengine/bevy |
| bevy_utils | 0.20.0 | MIT OR Apache-2.0 | https://github.com/bevyengine/bevy |
| bitflags | 2.13.2 | MIT OR Apache-2.0 | https://github.com/bitflags/bitflags |
| block2 | 0.6.2 | MIT | https://github.com/madsmtm/objc2 |
| bumpalo | 3.20.3 | MIT OR Apache-2.0 | https://github.com/fitzgen/bumpalo |
| bytemuck | 1.25.2 | Zlib OR Apache-2.0 OR MIT | https://github.com/Lokathor/bytemuck |
| bytemuck_derive | 1.12.1 | Zlib OR Apache-2.0 OR MIT | https://github.com/Lokathor/bytemuck |
| byteorder | 1.5.0 | Unlicense OR MIT | https://github.com/BurntSushi/byteorder |
| cfg-if | 1.0.5 | MIT OR Apache-2.0 | https://github.com/rust-lang/cfg-if |
| cfg_aliases | 0.2.2 | MIT | https://github.com/katharostech/cfg_aliases |
| concurrent-queue | 2.5.0 | Apache-2.0 OR MIT | https://github.com/smol-rs/concurrent-queue |
| convert_case | 0.10.0 | MIT | https://github.com/rutrum/convert-case |
| critical-section | 1.2.0 | MIT OR Apache-2.0 | https://github.com/rust-embedded/critical-section |
| crossbeam-queue | 0.3.14 | MIT OR Apache-2.0 | https://github.com/crossbeam-rs/crossbeam |
| crossbeam-utils | 0.8.23 | MIT OR Apache-2.0 | https://github.com/crossbeam-rs/crossbeam |
| ctrlc | 3.5.2 | MIT/Apache-2.0 | https://github.com/Detegr/rust-ctrlc.git |
| derive_more | 2.1.1 | MIT | https://github.com/JelteF/derive_more |
| derive_more-impl | 2.1.1 | MIT | https://github.com/JelteF/derive_more |
| dispatch2 | 0.3.1 | Zlib OR Apache-2.0 OR MIT | https://github.com/madsmtm/objc2 |
| disqualified | 1.1.0 | MIT OR Apache-2.0 | https://github.com/bevyengine/disqualified |
| downcast-rs | 2.0.2 | MIT OR Apache-2.0 | https://github.com/marcianx/downcast-rs |
| equivalent | 1.0.3 | Apache-2.0 OR MIT | https://github.com/indexmap-rs/equivalent |
| erased-serde | 0.4.10 | MIT OR Apache-2.0 | https://github.com/dtolnay/erased-serde |
| fixedbitset | 0.5.7 | MIT OR Apache-2.0 | https://github.com/petgraph/fixedbitset |
| foldhash | 0.2.0 | Zlib | https://github.com/orlp/foldhash |
| futures-channel | 0.3.34 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| futures-core | 0.3.34 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| futures-lite | 2.6.1 | Apache-2.0 OR MIT | https://github.com/smol-rs/futures-lite |
| futures-task | 0.3.34 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| futures-util | 0.3.34 | MIT OR Apache-2.0 | https://github.com/rust-lang/futures-rs |
| getrandom | 0.4.3 | MIT OR Apache-2.0 | https://github.com/rust-random/getrandom |
| glam | 0.33.12 | MIT OR Apache-2.0 | https://github.com/bitshifter/glam-rs |
| hash32 | 0.3.1 | MIT OR Apache-2.0 | https://github.com/japaric/hash32 |
| hashbrown | 0.16.1 | MIT OR Apache-2.0 | https://github.com/rust-lang/hashbrown |
| hashbrown | 0.17.1 | MIT OR Apache-2.0 | https://github.com/rust-lang/hashbrown |
| heapless | 0.9.3 | MIT OR Apache-2.0 | https://github.com/rust-embedded/heapless |
| indexmap | 2.14.2 | Apache-2.0 OR MIT | https://github.com/indexmap-rs/indexmap |
| itoa | 1.0.18 | MIT OR Apache-2.0 | https://github.com/dtolnay/itoa |
| js-sys | 0.3.106 | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/js-sys |
| libc | 0.2.191 | MIT OR Apache-2.0 | https://github.com/rust-lang/libc |
| log | 0.4.34 | MIT OR Apache-2.0 | https://github.com/rust-lang/log |
| memchr | 2.8.3 | Unlicense OR MIT | https://github.com/BurntSushi/memchr |
| midly | 0.5.3 | Unlicense | https://github.com/negamartin/midly |
| mutants | 0.0.3 | MIT | https://github.com/sourcefrog/cargo-mutants |
| naga-types | 30.0.1 | MIT OR Apache-2.0 | https://github.com/gfx-rs/wgpu |
| nix | 0.31.3 | MIT | https://github.com/nix-rust/nix |
| nonmax | 0.5.5 | MIT OR Apache-2.0 | https://github.com/LPGhatguy/nonmax |
| objc2 | 0.6.5 | MIT | https://github.com/madsmtm/objc2 |
| objc2-encode | 4.1.0 | MIT | https://github.com/madsmtm/objc2 |
| once_cell | 1.21.4 | MIT OR Apache-2.0 | https://github.com/matklad/once_cell |
| pin-project-lite | 0.2.17 | Apache-2.0 OR MIT | https://github.com/taiki-e/pin-project-lite |
| portable-atomic | 1.15.0 | Apache-2.0 OR MIT | https://github.com/taiki-e/portable-atomic |
| portable-atomic-util | 0.2.8 | Apache-2.0 OR MIT | https://github.com/taiki-e/portable-atomic-util |
| proc-macro2 | 1.0.107 | MIT OR Apache-2.0 | https://github.com/dtolnay/proc-macro2 |
| quote | 1.0.47 | MIT OR Apache-2.0 | https://github.com/dtolnay/quote |
| r-efi | 6.0.0 | MIT OR Apache-2.0 OR LGPL-2.1-or-later | https://github.com/r-efi/r-efi |
| raw-window-handle | 0.6.2 | MIT OR Apache-2.0 OR Zlib | https://github.com/rust-windowing/raw-window-handle |
| rustc-hash | 1.1.0 | Apache-2.0/MIT | https://github.com/rust-lang-nursery/rustc-hash |
| rustc-hash | 2.1.3 | Apache-2.0 OR MIT | https://github.com/rust-lang/rustc-hash |
| rustc_version | 0.4.1 | MIT OR Apache-2.0 | https://github.com/djc/rustc-version-rs |
| rustversion | 1.0.23 | MIT OR Apache-2.0 | https://github.com/dtolnay/rustversion |
| semver | 1.0.28 | MIT OR Apache-2.0 | https://github.com/dtolnay/semver |
| serde | 1.0.229 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| serde_core | 1.0.229 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| serde_derive | 1.0.229 | MIT OR Apache-2.0 | https://github.com/serde-rs/serde |
| serde_json | 1.0.154 | MIT OR Apache-2.0 | https://github.com/serde-rs/json |
| slab | 0.4.12 | MIT | https://github.com/tokio-rs/slab |
| slotmap | 1.1.1 | Zlib | https://github.com/orlp/slotmap |
| smallvec | 1.16.4 | MIT OR Apache-2.0 | https://github.com/servo/rust-smallvec |
| smol_str | 0.2.2 | MIT OR Apache-2.0 | https://github.com/rust-analyzer/smol_str |
| spin | 0.12.3 | MIT | https://codeberg.org/zesterer/spin |
| stable_deref_trait | 1.2.1 | MIT OR Apache-2.0 | https://github.com/storyyeller/stable_deref_trait |
| static_assertions | 1.1.0 | MIT OR Apache-2.0 | https://github.com/nvzqz/static-assertions-rs |
| syn | 2.0.119 | MIT OR Apache-2.0 | https://github.com/dtolnay/syn |
| syn | 3.0.9 | MIT OR Apache-2.0 | https://github.com/dtolnay/syn |
| thiserror | 2.0.21 | MIT OR Apache-2.0 | https://github.com/dtolnay/thiserror |
| thiserror-impl | 2.0.21 | MIT OR Apache-2.0 | https://github.com/dtolnay/thiserror |
| thread_local | 1.1.10 | MIT OR Apache-2.0 | https://github.com/Amanieu/thread_local-rs |
| tokio | 1.54.1 | MIT | https://github.com/tokio-rs/tokio |
| toml_datetime | 1.1.2+spec-1.1.0 | MIT OR Apache-2.0 | https://github.com/toml-rs/toml |
| toml_edit | 0.25.17+spec-1.1.0 | MIT OR Apache-2.0 | https://github.com/toml-rs/toml |
| toml_parser | 1.1.5+spec-1.1.0 | MIT OR Apache-2.0 | https://github.com/toml-rs/toml |
| typeid | 1.0.3 | MIT OR Apache-2.0 | https://github.com/dtolnay/typeid |
| unicode-ident | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 | https://github.com/dtolnay/unicode-ident |
| unicode-segmentation | 1.13.3 | MIT OR Apache-2.0 | https://github.com/unicode-rs/unicode-segmentation |
| unicode-xid | 0.2.6 | MIT OR Apache-2.0 | https://github.com/unicode-rs/unicode-xid |
| unsynn | 0.3.0 | MIT OR Apache-2.0 | https://seed.pipapo.org/nodes/seed.pipapo.org/rad:z39WbeupErKS8TwbDS5yU8eZSa3C |
| uuid | 1.29.0 | Apache-2.0 OR MIT | https://github.com/uuid-rs/uuid |
| variadics_please | 2.0.0 | MIT OR Apache-2.0 | https://github.com/bevyengine/variadics_please |
| version_check | 0.9.5 | MIT/Apache-2.0 | https://github.com/SergioBenitez/version_check |
| wasm-bindgen | 0.2.129 | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen |
| wasm-bindgen-futures | 0.4.79 | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/futures |
| wasm-bindgen-macro | 0.2.129 | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/macro |
| wasm-bindgen-macro-support | 0.2.129 | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen/tree/main/crates/macro-support |
| wasm-bindgen-shared | 0.2.129 | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/shared |
| web-sys | 0.3.106 | MIT OR Apache-2.0 | https://github.com/wasm-bindgen/wasm-bindgen/tree/master/crates/web-sys |
| web-task | 1.1.3 | MIT OR Apache-2.0 | https://github.com/NthTensor/web-task |
| wgpu-types | 30.0.1 | MIT OR Apache-2.0 | https://github.com/gfx-rs/wgpu |
| windows-link | 0.2.1 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| windows-sys | 0.61.2 | MIT OR Apache-2.0 | https://github.com/microsoft/windows-rs |
| winnow | 1.0.4 | MIT | https://github.com/winnow-rs/winnow |
| zmij | 1.0.23 | MIT | https://github.com/dtolnay/zmij |

Exact checksums: Cargo.lock. License texts: [THIRD_PARTY_NOTICES.txt](THIRD_PARTY_NOTICES.txt). This inventory is not a legal opinion.

## Packages without a standalone license text in the registry archive

- block2 0.6.2: manifest declares MIT; archive has no standalone matching license file. This is recorded rather than inventing a text.
- dispatch2 0.3.1: manifest declares Zlib OR Apache-2.0 OR MIT; archive has no standalone matching license file. This is recorded rather than inventing a text.
- midly 0.5.3: manifest declares Unlicense; archive has no standalone matching license file. This is recorded rather than inventing a text.
- mutants 0.0.3: manifest declares MIT; archive has no standalone matching license file. This is recorded rather than inventing a text.
- objc2 0.6.5: manifest declares MIT; archive has no standalone matching license file. This is recorded rather than inventing a text.
- objc2-encode 4.1.0: manifest declares MIT; archive has no standalone matching license file. This is recorded rather than inventing a text.
- r-efi 6.0.0: manifest declares MIT OR Apache-2.0 OR LGPL-2.1-or-later; archive has no standalone matching license file. This is recorded rather than inventing a text.

## Browser-test tooling

The two web harnesses use Playwright/playwright-core 1.56.1 (Apache-2.0), with optional fsevents 2.3.2 (MIT), locked in their package-lock.json files. Sources: https://github.com/microsoft/playwright and https://github.com/fsevents/fsevents. These are development tools, not bundled app dependencies. Chromium is downloaded for tests and not shipped. No external fonts or audio samples are bundled.

## Updating

Regenerate/check metadata and license texts when dependencies change. Check compound license requirements and both native/WASM builds. The wasm-bindgen CLI must match the locked crate version.

## Included MIDI asset

`web/piano/samples/bach-prelude-bwv924a.mid` is an unmodified Mutopia file:
Johann Sebastian Bach, Prelude BWV 924a, typeset by jeff covey © 2007,
CC BY-SA 3.0 Unported. Source, exact hash and attribution are in
[samples/ATTRIBUTION.md](web/piano/samples/ATTRIBUTION.md); the complete license is
bundled alongside it. It is not relicensed as application code. The native
launcher adds no registry dependencies and embeds these notices with the music.
