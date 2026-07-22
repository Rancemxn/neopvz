# Dependency Decisions

These are the direct dependencies selected for the initial scaffold. Cargo
resolves the latest stable patch within each listed stable line; the lockfile
created by the GitHub full gate records the exact transitive graph used by a
build. The matching upstream source is kept locally under ignored
`third_party-src/` for reference and is never committed.

| Crate | Stable line | Upstream source | Reference tag |
| --- | --- | --- | --- |
| wgpu | 30 | https://github.com/gfx-rs/wgpu | v30.0.0 |
| winit | 0.30 | https://github.com/rust-windowing/winit | v0.30.9 |
| image | 0.25 | https://github.com/image-rs/image | v0.25.10 |
| quick-xml | 0.41 | https://github.com/tafia/quick-xml | v0.41.0 |
| serde | 1.0 | https://github.com/serde-rs/serde | v1.0.229 |
| serde_json | 1.0 | https://github.com/serde-rs/json | v1.0.151 |
| rand/rand_chacha | 0.10 | https://github.com/rust-random/rand | 0.10.0 |
| thiserror | 2.0 | https://github.com/dtolnay/thiserror | 2.0.19 |
| tracing/tracing-subscriber | 0.1/0.3 | https://github.com/tokio-rs/tracing | tracing-0.1.44 |
| bytemuck | 1.25 | https://github.com/Lokathor/bytemuck | v1.25.2 |
| cpal | 0.18 | https://github.com/RustAudio/cpal | v0.18.1 |
| symphonia | 0.6 | https://github.com/pdeljanov/Symphonia | v0.6.0 |
| sha2 | 0.10 | https://github.com/RustCrypto/hashes | sha2-v0.10.9 |
| clap | 4.6 | https://github.com/clap-rs/clap | v4.6.3 |

The MO3 decoder is intentionally not a Cargo dependency yet. The audio PR
must first compare a libopenmpt FFI build against a pre-conversion pipeline on
the supported mobile targets, then record the selected upstream tag here.
