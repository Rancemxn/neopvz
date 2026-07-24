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

Current baseline: **949 accepted / 1802 total; 853 unresolved**.

## Foundation and Boundaries

| Obligation | Domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| FND-001 | AGPL implementation boundary and repository exclusion | 1 | 1 | verified | `7bfb9ed`, repository scan |
| FND-002 | Ubuntu format, lint, test, and release gate | 1 | 1 | verified | Action run `29939544390` |
| FND-003 | Windows resource-free release artifact and local launch | 1 | 1 | verified | PR `#11`, run `29939544535`, ignored local record |
| FND-004 | Directory, explicit path, and directory-embedded PAK discovery | 3 | 3 | verified | Issue `#4`; synthetic discovery tests and Actions |
| FND-005 | Standalone PAK parsing and resource access | 1 | 1 | verified | PR `#23`, run `29944500864`, ignored local record |
| FND-006 | Version identity and external-resource safety checks | 1 | 1 | verified | PR `#25`, runs `29947497509`/`29947497544`, ignored local record |
| FND-007 | Deterministic replay and state-hash harness | 1 | 1 | verified | PR `#27`, runs `29969592800`/`29969592830` |
| FND-008 | Screenshot, semantic comparison, and independent review pipeline | 0 | 1 | missing | Issue `#14` |
| FND-009 | Original-process instrumentation provenance and cross-checks | 0 | 1 | missing | Issue `#15` |

## External Resource Inventory

The totals below are the target manifest inventory used to scope parser and
loader coverage. They do not authorize committing the manifest or its assets.

| Obligation | Resource domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| RES-GROUP | Manifest resource groups | 29 | 29 | verified | PR `#22`, run `29943384294`, ignored local record |
| RES-IMAGE | Manifest image entries | 439 | 439 | verified | PR `#22`, run `29943384294`, ignored local record |
| RES-FONT | Manifest font entries | 20 | 20 | verified | PR `#22`, run `29943384294`, ignored local record |
| RES-SOUND | Manifest sound entries | 167 | 167 | verified | PR `#22`, run `29943384294`, ignored local record |
| RES-ANIMATION | Compiled animation resources | 250 | 250 | verified | PR `#25`, run `29947497544`, ignored local record |
| RES-MUSIC | Target music files and loop metadata | 0 | 2 | missing | MO3 entries verified by PR `#25`; loop metadata remains in Issue `#3` |

## Simulation Entities and Effects

| Obligation | Entity/effect domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| ENT-PLANT | Plant definitions and player-observable behavior | 21 | 49 | partial | PRs `#27`, `#30`, `#33`, `#36`, `#39`, `#42`, `#44`, `#48`, `#50`, `#53`, `#56`, `#58`, `#59`, `#60`, `#66`; runs `29969908209`/`29973097050`/`29973716257`/`29974169417`/`29974744365`/`29975305952`/`29975305940`/`29975652655`/`29975652651`/`29978884187`/`29978884201`/`29979356134`/`29979356138`/`29980230929`/`29980230938`/`29980841578`/`29980841574`/`30002936972`/`30002936877`/`30004214119`/`30004214079`/`30005507287`/`30005507199`/`30068974241`; ignored local source/runtime records |
| ENT-ZOMBIE | Zombie definitions and player-observable behavior | 1 | 33 | partial | PR `#27`; runs `29969908209`/`29969908244`; ignored local source/runtime records |
| ENT-PROJECTILE | Projectile types and collision behavior | 5 | 14 | partial | PRs `#30`, `#33`, `#36`; runs `29973097050`/`29973716257`/`29974169417`; ignored local source/runtime records |
| ENT-PICKUP | Sun, coins, prizes, and pickup behavior | 0 | 26 | missing | Issue `#5` |
| ENT-GRID | Graves, craters, portals, vases, and other grid items | 1 | 13 | partial | PR `#50`; runs `29979356134`/`29979356138`; ignored local source/runtime records |
| ENT-LAWNMOWER | Lawn, pool, roof, and special mower behavior | 0 | 4 | missing | Issue `#5` |
| ENT-EFFECT | Player-observable particle/effect events | 0 | 105 | missing | Issues `#2`, `#5` |
| SIM-SYSTEM | Tick ordering, RNG, damage, cooldown, resources, waves, collisions, placement, special rules, pause, win/loss, and restart | 0 | 13 | missing | Issue `#5` |

The current partial entity acceptance is intentionally narrow: Peashooter,
Sunflower, SnowPea, Repeater, Threepeater, SplitPea, Starfruit, Cattail,
Torchwood projectile conversion, SunShroom, TwinSunflower, CherryBomb,
PotatoMine, Squash, Jalapeno, IceShroom, DoomShroom, Wallnut, Tallnut, Chomper, and Spikeweed
behavior; the normal zombie; and Pea, SnowPea, Melon, Star, and Spike
projectile/collision behavior. Remaining definitions are unresolved until their
player-observable rules and domain-matched evidence are implemented.

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
| SAVE-PROGRESSION | Profile, settings, unlocks, awards, inventory, garden, mode completion, and load compatibility | 8 | 8 | verified | PR `#61`, runs `30024232209`/`30024232459`, ignored local record |
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
