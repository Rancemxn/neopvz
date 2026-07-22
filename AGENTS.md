# neopvz Project Instructions

- The repository is `neopvz`, licensed under AGPL-3.0-or-later.
- Do not add copyrighted game assets, binaries, IDA databases, function tables, or reverse-engineering reference repositories to this repository.
- Resources are external inputs. Auto-detect a resource directory, `main.pak`/PAK archive, or use an explicit `--data-dir`/`--pak` path.
- Local compilation and tests are forbidden because the local machine lacks the required resources. Use the GitHub Actions `full-gate` workflow for formatting, linting, tests, and builds.
- Run the full gate on Ubuntu only until it is consistently green. Add Windows and macOS back after the Ubuntu baseline is stable.
- Dependencies default to the latest stable versions. They may be changed when implementation evidence requires it.
- When a direct dependency is confirmed, clone its upstream source at the exact version used by Cargo into the ignored `third_party-src/` directory for reference. Do not commit that directory.
- Use issues, pull requests, milestones, and Actions to drive development. Do not create tags or releases until the project is stable.
- Search files with `fd`, `sg`, or `rg`; list directories with `eza`.
- Do not manually convert numeric representations. Use the configured IDA `int_convert` MCP tool for reverse-engineering values and Math MCP for calculations.
- Use the current Tavily MCP namespace (`mcp__tavily_hikari__*`) for web
  research. Every search must set `search_depth="advanced"`; crawl/extract
  calls must set `extract_depth="advanced"` whenever supported. Do not use
  `basic`, `fast`, or `ultra-fast` modes.
