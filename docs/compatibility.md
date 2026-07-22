# 1.0.0.1051 Compatibility Ledger

This is the finite public obligation index for the `loop.md` goal. Every unit
inside a range is an independent obligation. A range is not a permission to add
more work later: its total is fixed for this target version.

The ledger records behavior and evidence domains, not original asset IDs,
extracted tables, binary details, screenshots, or reference source. Detailed
same-version mappings, original-process observations, and capture files stay in
ignored local storage. An obligation is accepted only with domain-matched
evidence; a reviewer assertion alone is never sufficient.

Status is `verified`, `partial`, or `missing`. The progress quantity is the sum
of `total - accepted` across all rows.

Current baseline: **5 accepted / 1802 total; 1797 unresolved**.

## Foundation and Boundaries

| Obligation | Domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| FND-001 | AGPL implementation boundary and repository exclusion | 1 | 1 | verified | `7bfb9ed`, repository scan |
| FND-002 | Ubuntu format, lint, test, and release gate | 1 | 1 | verified | Action run `29939544390` |
| FND-003 | Windows resource-free release artifact and local launch | 1 | 1 | verified | PR `#11`, run `29939544535`, ignored local record |
| FND-004 | Directory, explicit path, and directory-embedded PAK discovery | 2 | 3 | partial | Issue `#4`; discovery tests cover explicit forms |
| FND-005 | Standalone PAK parsing and resource access | 0 | 1 | missing | Issue `#13` |
| FND-006 | Version identity and external-resource safety checks | 0 | 1 | missing | Issue `#13` |
| FND-007 | Deterministic replay and state-hash harness | 0 | 1 | missing | Issue `#5` |
| FND-008 | Screenshot, semantic comparison, and independent review pipeline | 0 | 1 | missing | Issue `#14` |
| FND-009 | Original-process instrumentation provenance and cross-checks | 0 | 1 | missing | Issue `#15` |

## External Resource Inventory

The totals below are the target manifest inventory used to scope parser and
loader coverage. They do not authorize committing the manifest or its assets.

| Obligation | Resource domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| RES-GROUP | Manifest resource groups | 0 | 29 | missing | Issue `#13` |
| RES-IMAGE | Manifest image entries | 0 | 439 | missing | Issue `#13` |
| RES-FONT | Manifest font entries | 0 | 20 | missing | Issue `#13` |
| RES-SOUND | Manifest sound entries | 0 | 167 | missing | Issues `#13`, `#19` |
| RES-ANIMATION | Compiled animation resources | 0 | 250 | missing | Issues `#13`, `#2` |
| RES-MUSIC | Target music files and loop metadata | 0 | 2 | missing | Issues `#13`, `#3` |

## Simulation Entities and Effects

| Obligation | Entity/effect domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| ENT-PLANT | Plant definitions and player-observable behavior | 0 | 49 | missing | Issue `#5` |
| ENT-ZOMBIE | Zombie definitions and player-observable behavior | 0 | 33 | missing | Issue `#5` |
| ENT-PROJECTILE | Projectile types and collision behavior | 0 | 14 | missing | Issue `#5` |
| ENT-PICKUP | Sun, coins, prizes, and pickup behavior | 0 | 26 | missing | Issue `#5` |
| ENT-GRID | Graves, craters, portals, vases, and other grid items | 0 | 13 | missing | Issue `#5` |
| ENT-LAWNMOWER | Lawn, pool, roof, and special mower behavior | 0 | 4 | missing | Issue `#5` |
| ENT-EFFECT | Player-observable particle/effect events | 0 | 105 | missing | Issues `#2`, `#5` |
| SIM-SYSTEM | Tick ordering, RNG, damage, cooldown, resources, waves, collisions, placement, special rules, pause, win/loss, and restart | 0 | 13 | missing | Issue `#5` |

## Player-Accessible Modes

| Obligation | Mode domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| MODE-ADVENTURE | Adventure levels | 0 | 50 | missing | Issue `#5`, renderer issue |
| MODE-SURVIVAL | Visible normal, hard, and endless survival variants | 0 | 11 | missing | Issue `#18` |
| MODE-MINIGAME | Mini-game levels | 0 | 20 | missing | Issue `#18` |
| MODE-VASE | Vasebreaker levels, including endless | 0 | 10 | missing | Issue `#18` |
| MODE-IZOMBIE | I, Zombie levels, including endless | 0 | 10 | missing | Issue `#18` |
| MODE-GARDEN | Zen, mushroom, aquarium, and tree services | 0 | 4 | missing | Issues `#18`, `#16` |

## Screens, Input, and Persistence

| Obligation | Behavior domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| UI-SCREEN | Loading, title, menu, selector, seed chooser, HUD, pause, options, help, almanac, shop, and result flows | 0 | 12 | missing | Issues `#2`, `#17` |
| INPUT-ACTION | Mouse, keyboard, hover, click, drag, placement, pause, restart, and command-line resource selection | 0 | 8 | missing | Issue `#17` |
| SAVE-PROGRESSION | Profile, settings, unlocks, awards, inventory, garden, mode completion, and load compatibility | 0 | 8 | missing | Issue `#16` |
| PLATFORM-CONTRACT | Logical viewport, window/fullscreen behavior, DPI, audio device, and external-path behavior | 0 | 6 | missing | Issue `#17` |

## Visual and Audio Evidence

| Obligation | Evidence domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| VIS-SCREEN | Declared visual checkpoints for player-accessible screens | 0 | 12 | missing | Issue `#14` |
| VIS-MODE | Declared visual checkpoints for every player-accessible mode unit | 0 | 105 | missing | Issue `#14` |
| VIS-PLANT | Plant animation, layering, clipping, and feedback review units | 0 | 49 | missing | Issue `#14` |
| VIS-ZOMBIE | Zombie animation, layering, clipping, and feedback review units | 0 | 33 | missing | Issue `#14` |
| VIS-PROJECTILE | Projectile animation and impact review units | 0 | 14 | missing | Issue `#14` |
| VIS-EFFECT | Effect and particle review units | 0 | 105 | missing | Issue `#14` |
| AUD-SFX | Simulation-tick and decoded-output sound-event units | 0 | 167 | missing | Issue `#19` |
| AUD-MUSIC | Music playback, loop, and stem units | 0 | 2 | missing | Issues `#3`, `#19` |
| AUD-SYNC | Event-to-device timing and music synchronization contract | 0 | 1 | missing | Issue `#19` |

## Acceptance Rules

- `verified` means every unit in the row has reproducible evidence in its
  declared domain and the relevant Ubuntu/Windows checks are green.
- `partial` records accepted units without hiding the remaining total; it is
  not a completion state.
- Pixel differences and SSIM remain diagnostics only. Visual acceptance is the
  semantic screenshot comparison followed by independent review defined in
  `loop.md` and `docs/development.md`.
- Original resources, IDA databases, function tables, reference repositories,
  screenshots, recordings, and local evidence remain outside version control.
- The final loop completion check requires every row to reach `accepted == total`.
