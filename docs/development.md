# Development Workflow

## Repository boundaries

Only implementation code, tests, documentation, and CI configuration belong
in this repository. Keep original binaries, extracted resources, IDA files,
function tables, and reference repositories outside the checkout.

Direct dependency source snapshots belong in `third_party-src/` for local
reference and are ignored by Git. Record the upstream URL and exact tag or
commit in the dependency PR that introduces the dependency.

## Verification

The local machine is intentionally not a build environment. Do not run local
Cargo compilation or tests. Every implementation PR must pass `.github/workflows/full-gate.yml`.

The full gate runs formatting, Clippy, unit tests, and release builds on the
latest stable Windows, Linux, and macOS runners. Mobile cross compilation is
not mandatory for the first gate.

## Delivery flow

1. Open or update an issue.
2. Put the issue in the current milestone.
3. Work on a branch and open a pull request.
4. Let the full gate verify the change.
5. Merge only after review and a green gate.

Tags and releases are intentionally deferred until the mainline is stable.
