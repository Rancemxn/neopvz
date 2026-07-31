# neopvz Project Instructions

- The repository is `neopvz`, licensed under AGPL-3.0-or-later.
- Reproduce the target's observable behavior, not awkward original implementation
  details. Prefer idiomatic Rust with named structs, enums, and fields when they
  make version-specific data easier to audit; preserve the original ordering and
  values where those are player-observable or affect deterministic replay.
- Treat compatibility as an observable-behavior claim: a constant or definition
  table alone is not evidence that a plant, zombie, projectile, mode, or UI rule
  is implemented. Record an item only after a focused behavioral test, replay,
  or captured runtime observation supports it.
- Evidence priority is strict: establish facts from same-version local evidence
  first—the 1.0.0.1051 executable/runtime, cleaned IDB, function table, local
  headers/resources, or independently authored local observation scripts. Web
  search and Tavily results are hypothesis leads only; they must not set target
  constants, retire ledger entries, or serve as acceptance evidence until
  cross-checked against identifiable same-version local evidence. If provenance
  or version identity is uncertain, leave the behavior unresolved.
- Keep reverse-engineered data readable and auditable. Prefer explicit named
  definitions and small helpers over opaque positional tables or copied quirks;
  retain an original detail only when it changes player-visible behavior,
  deterministic state, resource compatibility, or replay results.
- Do not add copyrighted game assets, binaries, IDA databases, function tables, or reverse-engineering reference repositories to this repository.
- Resources are external inputs. Auto-detect a resource directory, `main.pak`/PAK archive, or use an explicit `--data-dir`/`--pak` path.
- Code, dependency, workflow, and test changes must pass the local stable Cargo
  gate: `cargo +stable fmt --all -- --check`, locked workspace Clippy, locked
  workspace tests, and a locked debug workspace build. A passing local gate is
  sufficient for acceptance. Documentation-only and instruction-only changes
  use only their relevant lightweight checks.
- GitHub Actions may record Ubuntu and Windows results as supplementary
  evidence, but are never an acceptance blocker. Build a local debug binary
  with external resources for changes requiring Windows, GUI, screenshot, or
  audio validation; do not block text-only changes on it. Keep macOS disabled
  until the Ubuntu and Windows paths are stable.
- Dependencies default to the latest stable versions. They may be changed when implementation evidence requires it.
- When a direct dependency is confirmed, use the version actually selected by `neopvz`'s manifest and `Cargo.lock` as the source of truth. Clone the upstream repository into the single ignored `third_party-src/` directory at the repository root, then check out that exact tag or commit before consulting it. Do not leave the clone on a different default-branch version or commit `third_party-src/`.
- Use issues, pull requests, milestones, and Actions to drive development. Do not create tags or releases until the project is stable.
- Search local files with `fd`, `sg`, and `rg`; list directories with `eza`.
  Use FastCtx (`mcp__fastctx__read`, `mcp__fastctx__grep`, and
  `mcp__fastctx__glob`) with absolute paths when reading files, batching reads,
  or handling encoding-sensitive text. Do not rely on the shell's default
  decoder for reference files with ambiguous encoding.
- Treat a FastCtx encoding ambiguity as unresolved: use its reported candidates
  and pass a verified explicit `encoding`; never guess or use PowerShell's
  default decoder. Do not rewrite or transcode external reference material
  merely to make it readable. Use `mcp__fastctx__replace` for mechanical text
  replacements so the source encoding and line endings are preserved.
- Use `jq` for structured JSON inspection and transformation, including `gh`
  and API output; do not parse JSON with ad hoc text manipulation.
- Run commands synchronously by default. A blocking command's returned result
  is its completion result; do not add a separate wait or poll afterward.
  Start work asynchronously only when it enables useful parallel work. Poll
  only genuinely background work, including GitHub Actions, and no more often
  than every 20 seconds.
- Do not manually convert numeric representations. Use the configured IDA `int_convert` MCP tool for reverse-engineering values and Math MCP for calculations.
- Use the current Tavily MCP namespace (`mcp__tavily_hikari__*`) as the default
  web search and research path, not Context7. Reserve Context7 for an explicit
  current library/API documentation request; never use it as compatibility
  evidence. Every Tavily search must set `search_depth="advanced"`;
  crawl/extract calls must set `extract_depth="advanced"` whenever supported.
  Do not use `basic`, `fast`, or `ultra-fast` modes.
- Treat the local `pvztools-master` project as read-only, non-authoritative
  reference material. Do not run, build, link, import, embed, call, or copy it.
  Independently authored local process-memory scripts are allowed, but every
  adopted clue must be cross-checked against runtime behavior, IDA, the function
  table, or another independent source.
- Keep original and neopvz screenshots, diffs, recordings, and observation logs
  in ignored local storage. Visual verification is capture, comparison, then
  independent review; pixel metrics and SSIM are diagnostic only. Original-game
  client capture must be DPI-aware and checked for blank or incorrect crops.
- Every non-trivial compatibility slice must leave a focused check behind and
  pass the local stable Cargo gate before merge. Record any GitHub Actions run
  as supplementary evidence without waiting for it. Documentation, `AGENTS.md`,
  issue/ledger, and other text-only changes apply only the lightweight checks
  relevant to the diff.
- For repeated GUI tuning, use a hidden checkpoint/debug entry point such as
  `--checkpoint <scene>` to start at the target screen instead of replaying
  earlier scenes. A local preview against external resources may tune static
  positions and scales, but it is diagnostic only. Batch related code changes
  and use one local debug build for the GUI/resource check. After stabilization, capture
  original and `neopvz` checkpoints and complete the comparison and independent
  review described in `docs/development.md`.
- Follow `loop.md` for the compatibility goal, completion evidence, termination
  bounds, approval gates, residual routing, and bounded subagent contracts.
