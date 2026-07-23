# Goal & Success Signal

- **Goal:** Deliver a clean-room Rust reimplementation of the player-observable
  behavior of Plants vs. Zombies 1.0.0.1051. Internal algorithms, data
  structures, dependencies, and binary layout may differ from the original.
- **Scope:** Cover every player-accessible screen, mode, level, plant, zombie,
  projectile, resource rule, wave rule, input, animation, particle, sound,
  music stem, setting, save/progression path, and unlock path present in the
  target version. DRM, installers, updaters, online commerce, undefined
  behavior, crashes, and machine-specific defects are out of scope.
- **Reference-only boundary:** `pvztools-master` may be inspected as a
  non-authoritative source of version-specific memory, structure, and control
  clues. Do not run, build, link, import, embed, or call the tool or copy its
  implementation. Independently authored local scripts may use operating-system
  process-memory APIs to inspect or control the original executable for
  debugging and reproducible observations. Validate every adopted clue against
  original runtime behavior, IDA, the 1.0.0.1051 function table, or another
  independent source.
- **Observable success signal:** The finite compatibility ledger has no
  `missing`, `unverified`, or `failing` required entries; the Ubuntu and
  Windows gates are green; a Windows release artifact starts with an external
  1.0.0.1051 resource directory or PAK; all required GUI, visual, audio,
  progression, and deterministic replay evidence exists in the ignored local
  verification area; and all required GitHub issues are closed.
- **Deterministic success signal:** With the same build, resource version,
  initial save, random seed, and input sequence, damage, cooldowns, resources,
  waves, entity state, event order, and input results are identical, including
  the final state hash.
- **Visual success signal:** At each declared checkpoint, capture the original
  and neopvz screenshots, generate their comparison artifacts, and complete an
  independent visual review. The review finds no semantic difference in layout,
  visible state, resource selection, z-order, animation phase, clipping, or
  interaction feedback. Pixel-difference and SSIM values are diagnostic signals
  only, with no global pass threshold. Font rasterization, anti-aliasing, color
  handling, and DirectX-versus-neopvz rendering differences are allowed unless
  they change player-observable meaning, geometry, timing, or interaction.
- **Audio success signal:** Simulation-side audio events occur at the same
  simulation tick; device-output timing may vary by at most one tick. OGG
  effects, MO3 playback, music loop points, and hi-hat stem synchronization
  have evidence.
- **Observable failure signal:** Any required behavior lacks an implementation,
  deterministic check, GUI evidence, visual evidence, audio evidence, or a
  legally safe external-resource path; a gate is bypassed or weakened; or the
  loop reaches a termination bound without satisfying the success signal.

# Termination Conditions

- **Max iterations / budget:** 500 total iterations, including failed and
  no-progress attempts. A progress-producing iteration retires at least one
  ledger obligation, replaces a placeholder with an equal or smaller set of
  concrete obligations while reducing the placeholder count, or resolves a
  technical blocker with evidence. A new implementation, behavior difference,
  or reverse-engineering conclusion counts only when it does one of those
  things.
- **Goal-achievement check:** Evaluate every required ledger entry and every
  success signal above against code, Action results, deterministic records,
  GUI runs, screenshots, audio/event records, and local resource-boundary
  checks. Claims without domain-matched evidence do not pass.
- **No-progress condition:** Three consecutive iterations that neither retire
  an obligation, replace a placeholder with an equal or smaller set of concrete
  obligations while reducing the placeholder count, nor resolve a technical
  blocker; or the same root cause fails three consecutive recovery attempts.
  The current work path terminates and routes to PLANNER. If PLANNER cannot
  define a new finite path that preserves the goal, the loop terminates
  incomplete.
- **Blocked condition:** All remaining required entries depend on the same
  unavailable external resource, permission, credential, legal decision, or
  irreproducible original behavior after the permitted recovery attempts.
- **Budget exhaustion:** Terminate as incomplete with the remaining ledger,
  evidence, blockers, and latest green commit recorded for a successor loop.
  Do not reset the ledger or reduce the target.

# Progress Invariant

- **Bounded quantity that must advance:** The compatibility ledger is finite.
  Each unresolved required behavior, evidence gap, or critical blocker is one
  unresolved obligation. Progress is lexicographic: first reduce the number of
  unresolved obligations, then reduce the number of unresolved placeholder
  obligations when the total is unchanged. A placeholder is an obligation
  missing either a concrete target behavior or its domain-matched acceptance
  artifact; record both counts in each ledger snapshot. `docs/compatibility.md`
  is the public ledger; detailed target mappings and original-game evidence
  remain in ignored local storage.
- **How each path advances or exits:** Every valid iteration must retire at
  least one obligation, or replace a discovered placeholder with an equal or
  smaller set of concrete obligations while decreasing the placeholder count;
  otherwise it must terminate through the conditions above. A path that does
  neither is not valid progress.

# Approval Gates

| Gate | Trigger | If approved | If denied |
|---|---|---|---|
| Public release | Creating a public tag, release, installer, or store package | Perform the explicitly approved publication | Skip publication and continue technical work |
| Legal, license, or evidence publication | Changing AGPL terms, adding incompatible licensed material, or publishing local original-game screenshot/audio evidence outside the repository | Perform only the explicitly approved change without committing source assets | Keep the implementation and evidence local; continue safe work |
| Paid or privileged external action | New paid service, credential, or organization permission | Perform the explicitly approved action | Use an alternative or route the affected obligations to HUMAN |
| Scope expansion | Adding another game version, platform, or product target | Add the approved target and its finite ledger | Keep the current 1.0.0.1051 target |

Routine inspection, implementation, dependency selection, issue/PR work,
Action execution, artifact download, local GUI validation, and safe PR merges
require no HUMAN approval when their acceptance checks pass.

Force-pushing or rewriting `main`, deleting the repository, deleting
irrecoverable user reference data, weakening a gate, exposing credentials, or
placing original assets, binaries, IDBs, function tables, extracted data, or
local visual/audio evidence in the repository or Action artifacts is always
prohibited and cannot be approved by this loop.

# Measurement Domain

| Output domain | Verification method | Required artifact |
|---|---|---|
| Simulation and gameplay state | Same build, resource version, initial save, seed, and input sequence; compare state hashes, event order, damage, cooldowns, resources, waves, and input results | Deterministic replay record and matching final hash |
| Resource and format loading | Synthetic fixtures plus legally supplied external directory/PAK; verify manifest, PAK, compiled, font, image, animation, and audio boundaries | Parser tests, fixture results, and local resource manifest |
| Ubuntu build | GitHub full gate with formatting, Clippy, tests, and release build | Green Ubuntu Action run and release artifact |
| Windows build | GitHub Windows Action with the required non-GUI checks and release build | Green Windows Action run and downloadable artifact without original resources |
| GUI interaction | Run the Windows artifact locally with the ignored external resources; drive every player-accessible flow and record outcomes | Local interaction log, scenario result, and failure trace when applicable |
| Visual output | Capture the original and neopvz displayed client area at the same checkpoint, seed, save, input sequence, scale, DPI, and viewport; reject blank or incorrectly cropped captures; generate diagnostic pixel comparisons and a semantic comparison; then require an independent visual review of both screenshots and the comparison | Ignored local original screenshot, neopvz screenshot, diagnostic diff image/metrics, semantic comparison record, capture metadata, and independent review result |
| Original screenshot capture | Launch the original executable outside the repository, locate its top-level window, keep it visible and unobstructed, and use a DPI-aware capture process to capture the composed desktop pixels for its client area; verify that the result contains the complete client area without desktop pixels or window chrome; use windowed mode or Desktop Duplication if exclusive fullscreen is not capturable | Ignored local capture manifest containing executable/resource hashes, window bounds, DPI, checkpoint, seed, input record, and crop-validation result |
| Original process instrumentation | Inspect `pvztools-master` source only for hypotheses, then independently author local scripts that use operating-system process-memory APIs to observe or control seed, state, time, or capture checkpoints; cross-check adopted offsets and behavior against original runtime, IDA, or the function table | Ignored self-authored script, provenance note, observation log, and reproducible checkpoint record; no pvztools binary or copied implementation |
| Audio output | Compare simulation-side event ticks, decoded PCM or event traces, loop points, and stem synchronization; allow at most one tick for device-output timing | Local audio event log and decoder/stem verification record |
| Progress and persistence | Exercise unlocks, settings, save/load, and progression from clean and known saves | Local save fixtures and replay/progression evidence |
| Repository boundary | Scan commits, PR artifacts, and Action artifacts for original assets, binaries, IDBs, function tables, extracted data, and reference repositories | Clean repository scan with only implementation-owned files |

# Residual Routing

| Residual / failure | Route: LOCAL / PLANNER / HUMAN | Action |
|---|---|---|
| Reproducible implementation, parser, Action, GUI, screenshot, audio, or deterministic test failure | LOCAL | Make a changed recovery attempt with new evidence; after three same-root-cause failures, route to PLANNER |
| Tavily, IDA, Excel, GitHub, download, or capture service transient failure | LOCAL | Retry with a changed query or evidence path; continue independent obligations |
| Issue too large, stale, conflicting, or crossing ownership boundaries | PLANNER | Split or re-scope the issue without deleting required ledger obligations |
| Dependency, FFI, format, or architecture dead end | PLANNER | Preserve verified behavior and tests, then replace the internal approach |
| Measurement method does not match the output domain | PLANNER | Replace the measurement with a domain-matched artifact; do not mark the item passed |
| Missing, corrupt, version-ambiguous, or legally unavailable original input | HUMAN | Ask for the external input or authority; continue unrelated obligations |
| Missing credential, permission, paid service, or irreversible approval | HUMAN | Ask only for that authority; do not block separable safe work |
| Two mutually exclusive original behaviors remain after triangulating runtime, IDA, function table, resources, `pvztools-master` source clues, and trusted references | HUMAN | Ask for a ruling; keep the behavior unresolved until answered |
| All remaining obligations share one unresolved HUMAN residual | HUMAN | Terminate as blocked with evidence and the exact missing decision/input |

# Subagent Using Policy

Subagents are optional bounded collaborators. The primary role owns the ledger,
cross-module decisions, final acceptance, PR merge, and all approval gates.
The platform provides at most three additional equal-capability slots and a
shared workspace.

## Dispatch Point: Evidence analysis

- **Trigger:** A required behavior or format lacks a reproducible reference
  fact and can be investigated independently.
- **Role capability:** Read-only reverse-engineering, resource, dependency, or
  documentation analyst.
- **Tool boundary:** May use Tavily advanced, IDA, Excel, external references,
  ignored local resources, and read-only `pvztools-master` source inspection;
  may not run or call pvztools, modify project files, copy its implementation,
  or upload reference data.
- **Input contract:** One ledger obligation, exact target version, available
  evidence, and the unresolved question.
- **Output contract:** Evidence citations, conclusion, uncertainty, and a
  proposed deterministic or domain-matched acceptance check.
- **Acceptance check:** The primary role can reproduce the conclusion and turn
  it into an issue, test, or measurement entry.
- **Concurrency:** Up to three read-only analysts; no writer runs on the same
  files.
- **Failure routing:** LOCAL for changed retries, PLANNER for a scope or
  evidence-method problem, HUMAN for irreducible ambiguity or missing input.
- **Sub-task termination:** At most three subagent iterations, or two without
  new evidence; terminate earlier once the output contract is satisfied.

## Dispatch Point: Isolated implementation

- **Trigger:** One issue has explicit acceptance checks and a disjoint file
  ownership boundary.
- **Role capability:** Full-capability Rust implementer constrained to the
  assigned files and issue.
- **Tool boundary:** May edit and inspect assigned implementation/tests; may
  not alter shared Cargo/workflow/public API files without primary ownership,
  lower gates, commit copyrighted data, merge, or publish.
- **Input contract:** One issue, file ownership, compatibility entries, test
  vectors, and required evidence.
- **Output contract:** Minimal diff, tests or evidence, unresolved risks, and
  a clear acceptance result.
- **Acceptance check:** The primary role reviews the diff and the relevant
  Ubuntu/Windows/local domain evidence passes.
- **Concurrency:** At most one writer per overlapping ownership boundary;
  shared manifests and Cargo files remain primary-owned.
- **Failure routing:** LOCAL for implementation defects, PLANNER for boundary
  or architecture conflicts, HUMAN only for external authority/input.
- **Sub-task termination:** At most three iterations, or two without a smaller
  diff, new evidence, or a reduced ledger remainder.

## Dispatch Point: Independent verification

- **Trigger:** An implementation claims to retire a compatibility obligation,
  or a visual comparison is ready for review.
- **Role capability:** Read-only verifier independent of the implementation.
- **Tool boundary:** May inspect the diff, Action artifacts, local GUI results,
  screenshots, audio records, and deterministic logs; may not edit the claimed
  implementation or approve its own work.
- **Input contract:** Claimed obligation, expected behavior, build/resource
  identity, required measurement domain, and, for visual claims, the generated
  comparison artifacts.
- **Output contract:** Pass/fail result, evidence paths, mismatches, and exact
  residual classification.
- **Acceptance check:** Evidence is reproducible and matches the domain; a
  reviewer assertion alone never retires an obligation.
- **Concurrency:** One verifier per claimed result; may run alongside read-only
  evidence work but not the writer touching the same output.
- **Failure routing:** LOCAL for a fixable mismatch, PLANNER for a bad test or
  boundary, HUMAN for missing authority or irreducible reference ambiguity.
- **Sub-task termination:** One verification pass plus at most two changed
  rechecks; stop after two identical failures.

# Worst Case & Plan B

| Worst case | Plan B |
|---|---|
| Original behavior cannot be uniquely inferred | Triangulate same-version runtime, IDA, function table, resources, read-only `pvztools-master` source clues, and trusted references; independently script a minimal observation when useful; escalate only if mutually exclusive behavior remains |
| Dependency, FFI, parser, or architecture dead end | Preserve verified behavior and test vectors, return to the latest green implementation, and replace only the failing internal approach |
| Original or neopvz screenshot is blank, incorrectly cropped, or uncomparable | Keep the window visible and unobstructed, use a DPI-aware client-area capture, validate the crop, switch from exclusive fullscreen to windowed or Desktop Duplication, and rely on semantic checks while retaining pixel metrics only as diagnostics |
| Windows GUI or external resource validation is unavailable | Continue non-GUI obligations; leave affected entries unresolved and do not claim full success |
| Ubuntu or Windows Action is unavailable | Retry changed queries/runs and continue independent work; never replace Action compilation with local Cargo compilation |
| 500 iterations are exhausted | Terminate incomplete with the full residual ledger and latest green commit; a successor loop inherits the same target and evidence requirements |
