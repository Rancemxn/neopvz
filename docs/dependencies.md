# Dependency Decisions

Only dependencies used by current code are kept. Cargo resolves the latest
stable patch within each listed stable line, and `Cargo.lock` records the exact
graph used by the GitHub full gate. Matching direct-dependency sources live in
the ignored `third_party-src/` directory and are never committed.

| Crate | Stable line | Purpose | Upstream source | Reference tag |
| --- | --- | --- | --- | --- |
| clap | 4 | Command-line resource overrides | https://github.com/clap-rs/clap | v4.6.4 |
| kira (`cpal`, `ogg`, `vorbis`) | 0.12 | OGG decoding, game mixer, music/effect tracks | https://github.com/tesselode/kira | v0.12.2 |
| rand (`chacha`) | 0.10 | Deterministic simulation RNG | https://github.com/rust-random/rand | 0.10.2 |
| serde | 1.0 | State and resource metadata types | https://github.com/serde-rs/serde | v1.0.229 |
| serde_json | 1.0 | Stable state snapshots and initial save format | https://github.com/serde-rs/json | v1.0.151 |
| sha2 | 0.11 | Reproducible SHA-256 state hashes | https://github.com/RustCrypto/hashes | sha2-v0.11.0 |
| tempfile | 3.27 | Isolated filesystem tests | https://github.com/Stebalien/tempfile | v3.27.0 |
| thiserror | 2.0 | Library error types | https://github.com/dtolnay/thiserror | 2.0.19 |
| tracing | 0.1 | Structured application events | https://github.com/tokio-rs/tracing | tracing-0.1.44 |
| tracing-subscriber | 0.3 | Executable log output | https://github.com/tokio-rs/tracing | tracing-subscriber-0.3.23 |

## Deferred Until Used

- `wgpu`, `winit`, `bytemuck`, and `image`: add with the first real renderer
  and window implementation.
- `quick-xml` and `binrw`: add with the manifest and PAK/compiled parsers.
- `image_dds`: add after the loader decides between CPU DXT3 decoding and
  direct compressed texture upload.
- `atomic-write-file` and `directories`: add with the first save/config write.
- `proptest`: add when a binary parser exposes useful invariants.

Kira replaces direct `cpal` and `symphonia` dependencies so the workspace does
not compile two audio stacks. Only the OGG container and Vorbis codec are
enabled; unused realtime-priority, MP3, FLAC, and WAV features remain disabled.

MO3 remains deferred. The published `libopenmpt-sys 0.3.0` repository is no
longer available, so it is not reproducible enough to adopt. A later audio PR
must pin the official libopenmpt source, provide a thin C-API wrapper, and prove
the Ubuntu link and MO3 playback path in the full gate before adding it.
