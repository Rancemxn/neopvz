# Development Workflow

## Repository boundaries

Only implementation code, tests, documentation, and CI configuration belong
in this repository. Keep original binaries, extracted resources, IDA files,
function tables, reference repositories, screenshots, recordings, diffs, and
observation logs outside version control.

Search local files with `fd`, `sg`, and `rg`; list directories with `eza`.
Use FastCtx with absolute paths for reading files and encoding-sensitive text.
If FastCtx reports ambiguous encoding, verify a candidate before reading or
editing. Use `mcp__fastctx__replace` for mechanical replacements so source
encoding and line endings are preserved.

Direct dependency source snapshots belong in `third_party-src/` for local
reference and are ignored by Git. Record the upstream URL and exact tag or
commit in the dependency PR that introduces the dependency.

## Verification

The local stable Cargo gate is authoritative for implementation changes. Run
`cargo +stable fmt --all -- --check`, `cargo +stable clippy --workspace
--all-targets --locked -- -D warnings`, `cargo +stable test --workspace
--locked`, and `cargo +stable build --workspace --locked`. A passing local
gate is sufficient for acceptance; do not run a release build.

`.github/workflows/full-gate.yml` and `.github/workflows/windows-artifact.yml`
may still record formatting, lint, test, and build results. Treat their links
as supplementary evidence only and do not wait for completion. Do not spend
Action capacity on macOS or mobile until the Ubuntu and Windows paths are
stable.

Use the local debug build for resource-bound GUI, input, screenshot, and audio
checks. Keep all original resources and resulting evidence under ignored paths
such as `artifacts/`; never upload them to GitHub or Actions.

### Fast GUI iteration

For repeated visual tuning, use a checkpoint/debug entry point (for example a
hidden `--checkpoint <scene>` option) that starts directly at the target screen.
Do not replay the preceding title and tutorial path for every coordinate change.

Preview static layouts against the external resource PNGs with a small native
PowerShell/System.Drawing compositor before rebuilding. Use the preview only to
tune positions and scales; it is not GPU or compatibility evidence.

Batch related UI changes. Use an interim local debug build for quick checks.
After the layout stabilizes, run one complete route,
capture the original and neopvz checkpoints, generate comparisons, and complete
independent review. Never accept a preview or smoke result as final evidence.

## Deterministic replay

Replay records include the format version, build identity, resource version,
initial-save identity, random seed, input frames, event sequence, final state,
and final SHA-256 state hash. Actions stamp the build identity with the commit
SHA. A replay is accepted only when a fresh run reproduces the stored events,
state, and hash; metadata alone is not gameplay-compatibility evidence.

## Visual verification

At a declared checkpoint, capture the original and neopvz client areas with the
same resource version, save, seed, input sequence, viewport, scale, and DPI.
Reject blank or incorrectly cropped frames, generate diagnostic pixel
comparisons and a semantic comparison, then require an independent review of
both screenshots and the comparison.

There is no global SSIM or pixel-difference pass threshold. Review layout,
visible state, resource selection, z-order, animation phase, clipping,
interaction feedback, geometry, and player-observable timing. Font
rasterization, anti-aliasing, color handling, and DirectX/backend differences
are acceptable unless they change those semantics.

Original-game desktop capture must be DPI-aware. Verify that the output contains
the complete client area without desktop pixels or window chrome; use windowed
mode or Desktop Duplication when exclusive fullscreen cannot be captured.
Do not assume the game's logical dimensions match desktop pixels: live testing
confirmed that a non-DPI-aware capture can return a plausible but shifted crop.

## Original process instrumentation

`pvztools-master` is read-only, non-authoritative reference material. Do not
run, build, link, import, embed, call, or copy it. Independently authored local
scripts may use operating-system process-memory APIs to control or observe the
original for reproducible debugging. Cross-check adopted offsets and behavior
against the original runtime, IDA, the 1.0.0.1051 function table, or another
independent source, and keep scripts and observations in ignored local storage.

## Delivery flow

1. Open or update an issue.
2. Put the issue in the current milestone.
3. Work on a branch and open a pull request.
4. Pass the local stable Cargo gate and the domain-specific evidence check.
5. Merge after review and the passing local gate; append Action links when available without waiting for them.

Tags and releases are intentionally deferred until the implementation is stable.
