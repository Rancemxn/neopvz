# Development Workflow

## Repository boundaries

Only implementation code, tests, documentation, and CI configuration belong
in this repository. Keep original binaries, extracted resources, IDA files,
function tables, reference repositories, screenshots, recordings, diffs, and
observation logs outside version control.

Direct dependency source snapshots belong in `third_party-src/` for local
reference and are ignored by Git. Record the upstream URL and exact tag or
commit in the dependency PR that introduces the dependency.

## Verification

The local machine is intentionally not a build environment. Do not run local
Cargo compilation or tests. Every implementation PR must pass `.github/workflows/full-gate.yml`.

The full gate runs formatting, Clippy, unit tests, and a release build on
Ubuntu. `.github/workflows/windows-artifact.yml` runs the required non-GUI
tests, builds a release binary without original resources, and uploads that
binary as an artifact. Do not spend Action capacity on macOS or mobile until
the Ubuntu and Windows paths are stable.

Download the Windows artifact for local resource-bound GUI, input, screenshot,
and audio checks. This local execution does not permit local Cargo compilation.
Keep all original resources and resulting evidence under ignored paths such as
`artifacts/`; never upload them to GitHub or Actions.

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
4. Let the required Ubuntu and Windows Actions verify the change.
5. Merge only after review and a green gate.

Tags and releases are intentionally deferred until the implementation is stable.
