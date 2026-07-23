# neopvz Project Instructions

- The repository is `neopvz`, licensed under AGPL-3.0-or-later.
- Reproduce the target's observable behavior, not awkward original implementation
  details. Prefer idiomatic Rust with named structs, enums, and fields when they
  make version-specific data easier to audit; preserve the original ordering and
  values where those are player-observable or affect deterministic replay.
- Do not add copyrighted game assets, binaries, IDA databases, function tables, or reverse-engineering reference repositories to this repository.
- Resources are external inputs. Auto-detect a resource directory, `main.pak`/PAK archive, or use an explicit `--data-dir`/`--pak` path.
- Local compilation and tests are forbidden because the local machine lacks the required resources. Use GitHub Actions for formatting, linting, tests, and builds.
- Keep the complete gate on Ubuntu. Build a resource-free Windows release artifact in Actions, download it for local GUI, screenshot, and audio validation, and keep macOS disabled until the Ubuntu and Windows paths are stable.
- Dependencies default to the latest stable versions. They may be changed when implementation evidence requires it.
- When a direct dependency is confirmed, clone its upstream source at the exact version used by Cargo into the ignored `third_party-src/` directory for reference. Do not commit that directory.
- Use issues, pull requests, milestones, and Actions to drive development. Do not create tags or releases until the project is stable.
- Search files with `fd`, `sg`, or `rg`; list directories with `eza`.
- Do not manually convert numeric representations. Use the configured IDA `int_convert` MCP tool for reverse-engineering values and Math MCP for calculations.
- Use the current Tavily MCP namespace (`mcp__tavily_hikari__*`) for web
  research. Every search must set `search_depth="advanced"`; crawl/extract
  calls must set `extract_depth="advanced"` whenever supported. Do not use
  `basic`, `fast`, or `ultra-fast` modes.
- Treat the local `pvztools-master` project as read-only, non-authoritative
  reference material. Do not run, build, link, import, embed, call, or copy it.
  Independently authored local process-memory scripts are allowed, but every
  adopted clue must be cross-checked against runtime behavior, IDA, the function
  table, or another independent source.
- Keep original and neopvz screenshots, diffs, recordings, and observation logs
  in ignored local storage. Visual verification is capture, comparison, then
  independent review; pixel metrics and SSIM are diagnostic only. Original-game
  client capture must be DPI-aware and checked for blank or incorrect crops.
- Follow `loop.md` for the compatibility goal, completion evidence, termination
  bounds, approval gates, residual routing, and bounded subagent contracts.
