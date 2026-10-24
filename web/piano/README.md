# Tangent browser piano

Build/run/test instructions and actual support:
[docs/piano-player.md](../../docs/piano-player.md).
Architecture: [ADR-0003](../../docs/adr/0003-performance-model-and-piano-web-slice.md).

Generated WASM bindings in `pkg/` are ignored. `node_modules/` is test tooling only;
ship the HTML/CSS/JS modules and generated `pkg/`, plus third-party notices.
