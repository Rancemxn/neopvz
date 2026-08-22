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

Current baseline: **1401 accepted / 1810 total; 409 unresolved**.

## Foundation and Boundaries

| Obligation | Domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| FND-001 | AGPL implementation boundary and repository exclusion | 1 | 1 | verified | `7bfb9ed`, repository scan |
| FND-002 | Ubuntu format, lint, test, and release gate | 1 | 1 | verified | Action runs `29939544390`, `30030645194` |
| FND-003 | Windows resource-free release artifact and local launch | 1 | 1 | verified | PR `#11`, run `29939544535`, PR `#64`, run `30030645257`, ignored local record |
| FND-004 | Directory, explicit path, and directory-embedded PAK discovery | 3 | 3 | verified | Issue `#4`; synthetic discovery tests and Actions |
| FND-005 | Standalone PAK parsing and resource access | 1 | 1 | verified | PR `#23`, run `29944500864`, ignored local record |
| FND-006 | Version identity and external-resource safety checks | 1 | 1 | verified | PR `#25`, runs `29947497509`/`29947497544`, ignored local record |
| FND-007 | Deterministic replay and state-hash harness | 1 | 1 | verified | PR `#27`, runs `29969592800`/`29969592830` |
| FND-008 | Screenshot, semantic comparison, and independent review pipeline | 1 | 1 | verified | Ignored `artifacts/visual-issue14/independent-review.md`, Issue `#14`; title checkpoint crop/diff/review artifacts |
| FND-009 | Original-process instrumentation provenance and cross-checks | 1 | 1 | verified | Ignored `artifacts/original-observation/verification.md`; independent `ReadProcessMemory` observation cross-checked against the 1.0.0.1051 lawn capture; Issue `#15` |

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
| RES-MUSIC | Target music files and loop metadata | 2 | 2 | verified | Source `Music.cpp:172-188` track mapping; exact-version `mainmusic.mo3` and `mainmusic_hihats.mo3` libopenmpt probe (both 48 kHz stereo, identical 189.826 s duration); ignored `artifacts/music-loop/verification.md` |

## Simulation Entities and Effects

| Obligation | Entity/effect domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| ENT-PLANT | Plant definitions and player-observable behavior | 49 | 49 | verified | PRs `#27`, `#30`, `#33`, `#36`, `#39`, `#42`, `#44`, `#48`, `#50`, `#53`, `#56`, `#58`, `#59`, `#60`, `#66`, `#67`, `#69`, `#70`, `#71`, `#72`, `#73`, `#74`, `#75`, `#79`, `#80`, `#81`, `#82`, `#83`; runs `29969908209`/`29973097050`/`29973716257`/`29974169417`/`29974744365`/`29975305952`/`29975305940`/`29975652655`/`29975652651`/`29978884187`/`29978884201`/`29979356134`/`29979356138`/`29980230929`/`29980230938`/`29980841578`/`29980841574`/`30002936972`/`30002936877`/`30004214119`/`30004214079`/`30005507287`/`30005507199`/`30068974241`/`30099543625`/`30099543626`/`30132257878`/`30132257848`/`30132674816`/`30132674807`/`30133226675`/`30133226646`/`30133360999`/`30133360983`/`30134001945`/`30134001990`/`30134299774`/`30134299749`/`30134731995`/`30134731993`/`30135796477`/`30135796466`/`30136245552`/`30136245570`/`30137046838`/`30137046837`/`30137384077`/`30141692579`/`30141692532`; ignored `artifacts/pult-plants/verification.md`, `artifacts/basic-shooters/verification.md`, `artifacts/fume-shroom/verification.md`, `artifacts/gloom-shroom/verification.md`, `artifacts/scaredy-shroom/verification.md`, `artifacts/pumpkin-shell/verification.md`, `artifacts/spikerock/verification.md`, `artifacts/tanglekelp/verification.md`, `artifacts/aquatic-placement/verification.md`, `artifacts/flowerpot-placement/verification.md`, `artifacts/garlic-row-diversion/verification.md`, and local source/runtime records; PR `#86`, runs `30142737088`/`30142737093`, ignored `artifacts/marigold-coins/verification.md`, `artifacts/gold-magnet-coins/verification.md`, `artifacts/blover-special/verification.md`, `artifacts/gravebuster/verification.md`; PRs `#88`/`#90`/`#92`, runs `30144756597`/`30144756614`/`30148257254`/`30148257251`/`30148593526`/`30148593521`; PR `#94`, runs `30150244061`/`30150244058`, ignored `artifacts/instant-coffee/verification.md`; PR `#96`, runs `30150641968`/`30150641997`, ignored `artifacts/explode-o-nut/verification.md`; PR `#98`, runs `30151399161`/`30151399179`, ignored `artifacts/hypno-shroom/verification.md`; GiantWallnut and UmbrellaLeaf stationary behavior is identical to Wallnut (slot 3, PR `#50`), already verified by existing high-HP defensive plant tests; UmbrellaLeaf bungee/projectile deflection blocked by missing zombie types |
| ENT-ZOMBIE | Zombie definitions and player-observable behavior | 40 | 40 | verified | PRs `#27`, `#83`, `#107`, `#109`, `#111`, `#114`, `#117`, `#119`, `#121`, `#123`, `#125`, `#127`; runs `29969908209`/`29969908244`/`30141692579`/`30141692532`/`30152116202`/`30152116249`/`30152333302`/`30152333296`/`30152776730`/`30152776724`/`30152983281`/`30152983283`/`30153184493`/`30153184475`/`30153465523`/`30153465528`/`30153750557`/`30153750564`/`30153860072`/`30153860073`/`30154448402`/`30154448405`/`30156386652`/`30156386655`; Normal, Flag, Conehead, Buckethead, ScreenDoor, DuckyTube, Football (1670 HP, 2.5x speed), Imp (270 HP regular play, 70 HP I Zombie, 0.9 I Zombie speed), Newspaper (420 total HP, 0.89-0.91 mad speed after paper destroyed), PoleVaulter, Jackbox (500 HP, 0.66-0.68 speed, 500鈥?500-tick random detonation, 1800 damage within 115 units across 卤1 row from the timer pop only 鈥?no death-triggered explosion, 90-unit plant radius, 120-tick vase pop); Balloon (270 HP, 20 flying HP, projectile pop, walking transition, and Blover blow-away); Bobsled (four-zombie team, 270 body HP, 300 leader sled shield HP, 600000-unit sliding phase, and 500-tick slide); Ladder (500 body HP, 500 ladder shield HP, 0.79-0.81 carry speed with a walk re-pick after placement, barrier placement, and ladder bypass); Yeti (1350 HP, 1500-2000-tick phase, 0.4 walk / 0.8 flee speed, four 100-value diamond drops on defeat); ignored artifacts/conehead-zombie/verification.md, artifacts/pole-vaulter/verification.md, artifacts/screen-door-zombie/verification.md, artifacts/ducky-tube/verification.md, artifacts/football-zombie/verification.md, artifacts/newspaper-zombie/verification.md, artifacts/imp-zombie/verification.md, artifacts/jackbox-zombie/verification.md, artifacts/yeti-zombie/verification.md, artifacts/catapult-zombie/verification.md, artifacts/pogo-zombie/verification.md, artifacts/gargantuar-zombie/verification.md, artifacts/dancer-zombie/verification.md, artifacts/digger-zombie/verification.md, artifacts/bungee-zombie/verification.md, artifacts/dolphin-rider/verification.md, artifacts/snorkel-zombie/verification.md, artifacts/zamboni-zombie/verification.md, artifacts/balloon-zombie/verification.md, artifacts/bobsled-ladder/verification.md; Catapult (850 HP, 20 shots, 150-tick launch, 300-tick reload, 75-damage basketball); Pogo (500 HP, 80-tick bounce over a same-row plant, landing one grid cell to its left without biting); Gargantuar (3,000 HP, contact squashes plants, and deals 20 damage to SpikeRock); Dancer (500 HP, 300-tick entrance, and four 270-HP Backup Dancers); Digger (370 HP, 0.66-0.68 tunneling, 130-tick rise, 0.12 surfaced walk / 0.23 I Zombie); Bungee (450 HP, 300-tick bottom timer, and plant steal); Dolphin Rider (500 HP, pool entry, 120-tick jump over ordinary plants, and Tallnut block); Snorkel (270 HP, 0.66-0.68 water speed, submerged until eating); Zomboni (1350 HP, source speed profile, and drive-over plant removal); ignored `artifacts/zombotany-boss/verification.md`; Gargantuar/Gigagargantuar imp throw (half-HP one-time throw, source flight integration, 270-HP landed imp, with the regular-play Imp profile corrected from 70 to 270 HP and the I,Zombie 70-HP override retained) and the balloon damage-range rule (only Cactus/Cattail spikes hit fliers; cob blasts hit everything) in ignored `artifacts/gargantuar-imp-throw/verification.md` and `artifacts/balloon-cactus/verification.md`; independent verification issues `#129`-`#136` with fix PRs `#137`-`#144` corrected the I Zombie deploy costs, Conehead/Dancer/Imp deploy profiles, Jackbox, Newspaper, Dolphin Rider, Flag/Backup Dancer, Ladder carry, and Digger speed claims against the decomp and 1.0.0.1051 function table; Magnet-shroom metal steal, pool-row spawn gating with the ducky-tube overlay, and bungee wave delivery (roof final-wave sky drop) in ignored artifacts/magnet-shroom/verification.md, artifacts/pool-row-spawns/verification.md, and artifacts/bungee-delivery/verification.md; local DEBUG full gate (format, Clippy, workspace tests, and DEBUG workspace build) passed |
| ENT-PROJECTILE | Projectile types and collision behavior | 15 | 15 | verified | PRs `#30`, `#33`, `#36`, `#58`, `#67`, `#69`, `#71`, `#72`, `#74`, `#260`, `#268`, `#270`; runs `29973097050`/`29973716257`/`29974169417`/`30003168668`/`30003168691`/`30132257878`/`30132257848`/`30133226675`/`30133226646`/`30133613372`/`30133613399`/`30134299774`/`30134299749`/`30480181965`/`30480182225`/`30496450729`/`30496450907`/`30498120123`/`30498120119`; ignored `artifacts/puffshroom-range/verification.md`, `artifacts/pult-plants/verification.md`, `artifacts/fume-shroom/verification.md`, `artifacts/gloom-shroom/verification.md`, `artifacts/cob-cannon/verification.md`, `artifacts/pea-head/verification.md`, `artifacts/pult-trajectories/verification.md`, `artifacts/threepeater-trajectory/verification.md`, `artifacts/starfruit-trajectory/verification.md`, and local Torchwood source/runtime records; ProjectileType::Other(u8) (Other(1) lobbed basketball at 75 damage; other values default to a straight 20-damage projectile), Cob, ZombiePea, source-specific regular pult trajectory/collision behavior, the Threepeater vertical trajectory, and the Starfruit origin/shadow/vertical collision timing are covered by generic logic plus focused checks; Issues `#216`, `#227`, `#236` |
| ENT-PICKUP | Sun, coins, prizes, and pickup behavior | 26 | 26 | verified | SunPickupState, CoinPickupState, SunProduced/SunCollected/CoinProduced/CoinCollected events, CollectSun/CollectCoin input actions, source-like COIN_MOTION_COIN award arcs (launch, drift, gravity, item-award offsets, sunflower award elevation, landing), Raining Seeds usable-seed payload drops and free collection/planting, and money-bag fan-out into five from-present gold coins with 80-tick auto-collection are implemented; the source pickup catalog, money/sun variants, mode-unlock presents, garden prizes, chocolate, and remaining award-specific progression are covered by ignored local evidence and core tests; pickup particles remain unresolved|
| ENT-GRID | Graves, craters, portals, vases, and other grid items | 13 | 13 | verified | PRs `#50`, `#92`; runs `29979356134`/`29979356138`/`30148593526`/`30148593521`; graves (PR #50), craters (PR #50 DoomShroom crater with replant blocking), Vasebreaker vases (seeded layout, break/reveal, plant/zombie contents, no-vase rejection, and win condition), the per-row Zomboni ice trail (lay/melt/planting block, Jalapeno melt, spike-vehicle pop, Bobsled spawn dependency and end-of-ice crash), the single-use garden rake (first-zombie kill and consumption), placed ladders (Ladder-zombie placement, barrier bypass for later zombies, and Magnet-shroom removal, with focused core tests), I Zombie lawn brains (placement, zombie brain-eating, and loss condition, evidenced by artifacts/izombie/verification.md), and Zombiquarium click-position brains (three-item cap, age gate, Snorkel targeting/healing, and packet-spawn lifecycle) accepted; ignored `artifacts/vasebreaker/verification.md`, `artifacts/zomboni-ice-trail/verification.md`, and local source/runtime records; adventure level 4-5 Scary Potter (three-stage pot layouts from Challenge.cpp ScaryPotterPopulate, wave-clock suppression, stage advance on board clear, and the three-stage win) in ignored `artifacts/scary-potter-adventure/verification.md`; first-run adventure sod rows (1-1 row 2 only, 1-2/1-3 rows 1-3: planting, spawning, and mower gating) in ignored `artifacts/sod-rows/verification.md`; Portal Combat initial pair layout, 200-tick conveyor/wave start, 9,000-tick challenge state, 6,000-tick relocation, row spawn weights, source `mLastPortalX` destination-column guard, zombie/straight-projectile/triggered-mower transport, relocation row/column exclusion, and Peashooter/Repeater/Cactus cross-row target search are covered by `portal_combat_uses_source_pairs_timers_and_row_weights`, `portal_combat_transfers_paired_entities`, and `portal_combat_shooters_target_zombies_across_portal_rows`, with local source evidence `Challenge.cpp:3140-3376` and `Plant.cpp:4814-4862` |
| ENT-LAWNMOWER | Lawn, pool, roof, and special mower behavior | 4 | 4 | verified | PR `#84`; runs `30142029320`/`30142029344`; ignored `artifacts/lawnmower-trigger/verification.md`; all playable scenes (Day, Night, Pool, Roof) use the same mower trigger/sweep/retain logic initialized in BoardState::new — no scene-specific mower behavior exists in the target version |
| ENT-EFFECT | Player-observable particle/effect events | 73 | 105 | partial | Ignored `artifacts/effect-evidence/catalog.md` (full 106-entry trigger-site catalog), `artifacts/effect-anchors/verification.md`, `artifacts/boss-fireball/verification.md`, `artifacts/vehicle-effects/verification.md`, `artifacts/dead-effect-slots/verification.md`, `artifacts/seed-packet-ready/verification.md`, `artifacts/jackbox-effect/verification.md`, and `artifacts/garden-glow/verification.md`; 64 simulation-class effects have deterministic anchor events and focused tests (splats, specials, planting, grave/vase lifecycle, armor/shield drops, thaw, potato arm, seed packet readiness, Jackbox explosion, tallnut jump block, pogo break, digger rise, mind control, vehicle deaths, vehicle tier smoke/tire-pop, ice-trail state, portal open/teleport, umbrella deflect, butter, boss fire/ice ball spit-roll-destroy family, and Zen/Aquarium happy glows), plus five catalog-preserved dead/superseded slots with no gameplay trigger sites (#26, #57, #67, #76, #85); remaining units ride the renderer campaign; Issues `#2`, `#5` |
| SIM-SYSTEM | Tick ordering, RNG, damage, cooldown, resources, waves, collisions, placement, special rules, pause, win/loss, and restart | 13 | 13 | verified | PRs `#78`, `#81`, `#82`, `#83`, `#84`, `#85`, `#94`, `#98`, `#105`; runs `30135391025`/`30135391020`/`30137046838`/`30137046837`/`30137384083`/`30137384077`/`30141692579`/`30141692532`/`30142029320`/`30142029344`/`30142326755`/`30142326732`/`30150244061`/`30150244058`/`30151399161`/`30151399179`/`30151918720`/`30151918717`; core pause-freeze, aquatic-placement, roof-placement, Garlic timing, mower-boundary, terminal-restart, nocturnal-sleep/coffee-wake, zombie-hypnotize, wave-spawning, normal seed-packet cooldown, conveyor refill/removal, sun-resource, projectile-zombie collisions, loss-boundary, and win-condition (`GameWon`) tests with ignored local records |

The completed Imitater, Pole Vaulter, CobCannon, PeaHead, ZombiePea, Vasebreaker, Pogo Zombie, Gargantuar, Dancer, Backup Dancer, Digger, Bungee, Bobsled, and Ladder slices are evidenced
by ignored `artifacts/imitater/verification.md`,
`artifacts/pole-vaulter/verification.md`, `artifacts/cob-cannon/verification.md`,
`artifacts/pea-head/verification.md`,
`artifacts/vasebreaker/verification.md`,
`artifacts/pogo-zombie/verification.md`,
`artifacts/gargantuar-zombie/verification.md`, and
`artifacts/dancer-zombie/verification.md`,
`artifacts/digger-zombie/verification.md`, and
`artifacts/bungee-zombie/verification.md`.

The completed Boss, WallnutHead, JalapenoHead, GatlingHead, SquashHead,
TallnutHead, and Gigagargantuar slice is evidenced by ignored
`artifacts/zombotany-boss/verification.md` and the focused core tests recorded
there.

The current entity acceptance is intentionally narrow: Peashooter, Sunflower,
SnowPea, Repeater, Threepeater, SplitPea, Starfruit, Cattail,
Torchwood projectile conversion, SunShroom, TwinSunflower, CherryBomb,
PotatoMine, Squash, Jalapeno, IceShroom, DoomShroom, Wallnut, Tallnut, Chomper,
Spikeweed, SpikeRock, LilyPad, FlowerPot, Garlic, PuffShroom, SeaShroom, TangleKelp, FumeShroom,
GloomShroom, ScaredyShroom, CabbagePult, KernelPult, WinterMelon, GatlingPea, Cactus, LeftPeater, Marigold, GoldMagnet, Blover, GraveBuster, InstantCoffee, ExplodeONut, HypnoShroom, GiantWallnut, UmbrellaLeaf, PumpkinShell, Imitater, and Yeti behavior; the normal, PoleVaulter, Balloon, Bobsled, Ladder, Catapult, PeaHead, Pogo, Gargantuar, Dancer, Backup Dancer, Digger, Bungee, Dolphin Rider, Snorkel, Boss, WallnutHead, JalapenoHead, GatlingHead, SquashHead, TallnutHead, Gigagargantuar, and Zamboni zombies; and Pea, SnowPea, Puff, Cabbage,
Kernel, Butter, Melon, WinterMelon, Fireball, Star, Spike, Cob, ZombiePea, and Generic(Other) projectile/collision behavior. Remaining entity, mode, visual, audio, and effect obligations remain listed in the ledger above.

GoldMagnet now follows the 1.0.0.1051 target path: only mature Silver, Gold,
and Diamond coins are eligible, from-present coins are excluded, up to five
targets are retained during one suck, and each target travels to the plant
before the 200-300 tick recharge starts. Source evidence is local
`Plant.cpp:2172-2319`; focused coverage is
`gold_magnet_requires_magnetshroom_and_collects_coins` and
`gold_magnet_filters_present_coins_and_caps_suction_at_five_items`. This
refines the already accepted GoldMagnet plant unit without changing the
ledger totals; pickup visuals and remaining pickup catalog behavior remain
under `ENT-PICKUP`.

Whack-a-Zombie now uses the source grave lifecycle: nine initial graves on
columns 3-8, weighted grave placement that clears plants, the 200-tick opening
clock, 2,000-tick wave clock, phase-based Normal/Conehead/Buckethead counts,
50-tick grave rises, and the final wave's twenty Conehead/Buckethead spawns.
Same-version evidence is local `Board.cpp:1093-1105` and
`Challenge.cpp:2747-2915`; focused coverage is
`whack_a_zombie_uses_graves_for_the_first_and_final_waves` and
`whack_a_zombie_grave_selection_prefers_and_clears_a_plant`. The implementation
does not add a visual grave particle or hammer animation event, so those remain
under the visual/effect ledger.

The aquatic-placement units in `ENT-PLANT` and `SIM-SYSTEM` were re-evaluated
for Issue `#192`: the earlier scene-wide acceptance is superseded by the
per-row Pool/Fog check in ignored `artifacts/aquatic-placement/verification.md`,
with PR `#195` exact-head gates `30453887198`/`30453886891`.

The KernelPult unit now preserves the source weapon choice made when an attack
is armed: `range(4)` selects Kernel versus Butter before the shooting delay,
and the selected projectile is emitted at the later boundary. Source, state,
RNG, and target-change evidence is recorded in ignored
`artifacts/kernelpult/verification.md`; Issue `#238`.

The regular Cabbage, Kernel, Melon, Winter Melon, and Butter projectile unit
now follows the source target-specific 120-update lob: `ZombieTargetLeadX(50)`
and Dolphin/Pogo/Snorkel/Boss offsets are retained, gravity is integrated in
deterministic millipixels, and rising, minimum-height, pool, and walking
Snorkel collision gates are applied. Source, focused checks, and local gate
evidence are recorded in ignored `artifacts/pult-trajectories/verification.md`;
Issue `#216`, PR `#260`, Actions `30480181965`/`30480182225`.

The Catapult basketball now uses the selected plant's source X/Y, integrates
the 120-update arc with source gravity, applies Day/Pool/Fog/Roof/Boss ground
and collision-height rules, and resolves impact from the current rectangle and
top-plant priority. Focused checks cover first eligible collision, scene
heights and roof slope, stack priority, and target replacement; evidence is in
ignored `artifacts/pult-trajectories/verification.md`. Issue `#217`, PR `#261`,
Actions `30490198714`/`30490198670`.

The board projectile renderer now resolves the source `Projectile::Draw` image
table from the target archive for Pea/Zombie Pea, Snow Pea, Puff, Cabbage,
Kernel, Butter, Melon, Winter Melon, Fireball, Spike, Star, Cob, and Catapult
basketball projectiles. Source image paths and draw scales are retained in the
asset loader and focused board mapping checks. The board path now uses centered
affine sprites for source-style projectile spin, Puff age-based growth, source
day/night shadow cells, and lobbed-projectile height scaling. Exact
source-randomized starting angles/speeds, impact/trail particles, and visual
acceptance remain under the visual and effect ledgers.

Catapult launch targeting now shares the source grounded-plant boundary:
Squash rising/falling/done-falling states and health-zero terminal plants are
excluded before the leftmost-column search, while the source Catapult
top-plant order remains intact. Focused launch checks and same-version source
evidence are recorded in ignored `artifacts/catapult-zombie/verification.md`;
Issue `#218`, PR `#263`, Actions `30491696677`/`30491697034`.

Threepeater launch targeting now matches the source center and adjacent-row
gate, including board-row and attack-rectangle eligibility, while Starfruit's
wider directional targeting remains independent. Focused positive and no-fire
checks plus same-version source evidence are recorded in ignored
`artifacts/threepeater/verification.md`; Issue `#228`, PR `#264`, Actions
`30493122168`/`30493122181`.

Projectile impact now preserves the source shield damage flags: lobbed and
backwards shots, plus negative-X Starfruit shots, bypass shields; forward and
rightward shots consume them; splash projectiles damage shield and body. The
focused motion matrix and same-version source evidence are recorded in ignored
`artifacts/projectile-shield-flags/verification.md`; Issue `#234`, PR `#266`,
Actions `30494404599`/`30494404244`.

Threepeater adjacent-row projectiles now use the source-row origin and
dedicated vertical motion with damped velocity, scene-specific source heights,
shadow offsets, fired-row retention, and rectangle collision gating. Focused
trajectory and first-impact checks plus same-version source evidence are
recorded in ignored `artifacts/threepeater-trajectory/verification.md`; Issue
`#227`, PR `#268`, Actions `30496450729`/`30496450907`.

Starfruit projectiles now use the source `mX + 25, mY + 25` origin and the
dedicated shadow initialization, advance shadow and vertical-row state through
scene slopes, clean up at source vertical bounds, defer collision above the
shadow threshold, and stop at the source high-ground threshold. Focused
origin, row-transition, gated-collision, cleanup, and first-impact checks plus
same-version source evidence are recorded in ignored
`artifacts/starfruit-trajectory/verification.md`; Issue `#236`, PR `#270`,
Actions `30498120123`/`30498120119` at exact implementation head `eb82512`.

Starfruit launch eligibility now follows the source five-ray search instead of
the shared two-row shortcut: same-row leftward bodies, the off-row center ray,
row-specific diagonal angle bands, moving-target lead, damage eligibility, the
Digger offset, the high-column Boss case, and the 50-tick recently-eaten
override are preserved. The source 40-tick Starfruit firing counter and focused
positive, negative, movement, eligibility, Boss, bite, and first-fire checks
are recorded in ignored `artifacts/starfruit-target-search/verification.md`;
Issue `#230`, PR `#271`, Actions `30501265802`/`30501265816` at exact
implementation head `dddb9b8`.

SplitPea now evaluates and retains its forward and backward weapon gates
independently through delayed emission, while LeftPeater retains only its
backward gate. Both use the source inclusive zombie-rectangle overlap instead
of raw base-position comparisons. Focused direction, event-count, damage,
no-fire, edge-overlap, and eligibility checks are recorded in ignored
`artifacts/splitpea-directional-targets/verification.md`; Issue `#231`, PR
`#272`, Actions `30502935720`/`30502935723` at exact implementation head
`82f54f8`. The separately tracked generic cadence is covered below.

Plant firing cadence now uses source-specific animation counters and emission
boundaries instead of a universal 33-tick delay and five-tick burst. This
includes the Repeater/LeftPeater/SplitPea launch-counter-25 path, Cattail's
launch-counter-50 arm and counter-19 target recheck, GatlingPea's four shot
boundaries, GloomShroom's four pulses, and the individual pult/shroom counters.
Exact tick vectors and same-version source evidence are recorded in ignored
`artifacts/plant-firing-cadence/verification.md`; Issue `#229`, PR `#273`,
Actions `30504974125`/`30504974120` at exact implementation head `644c7c5`.

UmbrellaLeaf now intercepts both Zombie Pea and Catapult basketball plant
collisions through one live, ground, adjacent-cell lookup. The first impact
starts the source five-tick triggered phase without plant damage; the remaining
55 ticks reflect matching projectiles with the source Splat impact before the
13-frame block animation returns to idle. Same-version source, compiled
animation, deterministic, and audio-mapping evidence is recorded in ignored
`artifacts/umbrella-projectile-interception/verification.md`; Issue `#200`.

The Dolphin Rider and Snorkel phase machines now follow the 1.0.0.1051 source
profiles. The Rider keeps its 0.89-0.91 walk speed through pool entry and
riding, applying 0.5 only at jump start; its entry and jump phases are
off-ground windows that reject ordinary Pea/Butter ground projectiles, with
only CobCannon (GetDamageRangeFlags 127) carrying DAMAGES_OFF_GROUND. The
Snorkel keeps its 0.2 entry velocity through the first underwater walk,
re-picks 0.66..0.68 after eating and on returning to land, keeps walking
while surfacing, and takes only pult-family (DAMAGES_SUBMERGED) damage while
submerged (the source Cattail 11-bit range has no submerged bit). Both pause
phase progress while ice/butter immobilized and resume on the exact thaw tick,
matching the source pre-decrement ordering. Focused coverage is
`dolphin_rider_restores_source_pool_and_jump_phases`,
`ground_projectiles_cannot_hit_a_dolphin_during_entry_or_jump`,
`dolphin_phases_pause_while_immobilized_and_resume_on_thaw`,
`snorkel_uses_source_spawn_speed_and_hides_in_pool_until_it_eats`,
`snorkel_keeps_walking_while_surfacing_from_the_pool`,
`submerged_snorkel_takes_pult_but_not_pea_damage`, and
`snorkel_phases_pause_while_immobilized_and_resume_on_thaw`; source evidence is
local `Zombie.cpp` `PickRandomSpeed`, `UpdateZombieDolphinRider`,
`UpdateZombieSnorkel`, `EffectedByDamage`, and `CanBeFrozen`, plus
`Plant.cpp` `GetDamageRangeFlags`; Issues `#183`, `#184`, `#185`, `#188`,
`#189`, `#190`, `#191`.

The Ice-shroom effect now follows the source `HitIceTrap` three-class contract:
`CanBeChilled` exclusions (Zamboni, sledded Bobsled team, hidden/rising Digger,
grave risers, rising Backup Dancer, mind-controlled, and the Boss without an exposed-head
model) receive no chill, freeze, or 20 damage; `CanBeFrozen` exclusions
(vaulting Pole Vaulter, Dolphin entry/jump, Snorkel entry, flying Balloon,
thrown/landing Imp, SquashHead rise/fall, bouncing Pogo, and Bungee) receive
chill only; ordinary eligible zombies receive chill plus a source-range freeze
(300 in pool, 300..=400 when already cold, 400..=600 when fresh) plus 20
damage, with deterministic RNG consumption. Focused table-driven coverage is
`ice_shroom_applies_source_chill_freeze_and_damage_classes`; source evidence is
local `Zombie.cpp` `CanBeChilled` (`7983-8008`), `CanBeFrozen` (`8010-8032`),
and `HitIceTrap` (`8346-8382`); Issues `#186` and `#283`.

Final-wave grave and pool risers now use the source `RiseFromGrave` phase: a
150-tick off-ground rise on land or a 50-tick submerged rise in the pool,
stationary and rejecting ordinary ground attacks until the counter boundary,
then joining as normal zombies (pool risers keep their pool flag). Focused
coverage is `rising_zombies_are_off_ground_until_the_source_timer_elapses`;
source evidence is local `Zombie.cpp` `RiseFromGrave` (`8130-8160`),
`UpdateZombieRiseFromGrave` (`3009-3038`), and `EffectedByDamage`
(`8082-8085`); Issue `#226`.

Instant Coffee now resolves its target through the source normal plant layer
instead of insertion order: a Pumpkin shell or another flying Coffee no longer
hides the sleeping nocturnal plant, on both the planting and trigger paths.
Focused coverage is
`coffee_wakes_a_sleeping_mushroom_beneath_a_pumpkin_shell`; source evidence is
local `Board.cpp` `GetPlantsOnLawn`/`GetTopPlantAt` (`2155-2277`),
`Plant.cpp` `IsFlying` (`5144-5157`), and `Board.cpp` Coffee placement guards
(`2789-2802`); Issue `#197`.

CobCannon now defers its Cob projectile to the source launch boundary: the fire
input stores the target and the 206-tick firing counter, and the projectile is
created and emitted only when the counter reaches 1, matching `CobCannonFire`
and `UpdateShooting` (`Plant.cpp:4502-4516`, `3234-3340`). Reload arming stays
separate from the firing animation. Focused coverage is
`cob_cannon_defers_launch_until_the_firing_counter_elapses`, and the existing
launch-and-impact test now keeps targets stationary through the firing
animation; Issue `#199`.

Fireball splash now follows the source `IsZombieHitBySplash` and
`DoSplashDamage` rules: same-row-only targets with a width-100 rect,
fire-resistant zombies (Catapult, Zamboni, door/ladder shields) excluded, and
the aggregate splash capped at the original 40 damage before per-target
application. The primary impact keeps the source shield-and-body flags. Focused
coverage is `fireball_splash_is_same_row_and_capped`, and the Torchwood test
now asserts same-row splash with no adjacent-row hit; source evidence is local
`Projectile.cpp` `GetDamageFlags` (`381-407`), `IsZombieHitBySplash`
(`430-475`), `DoSplashDamage` (`477-530`), and `Zombie::IsFireResistant`;
Issue `#235`.

Torchwood interception now uses the source plant-attack and projectile
rectangles with a >=10 overlap threshold instead of the plant-cell line
crossing, and records the converting Torchwood column so a projectile is not
reconverted by the same plant while still converting at later Torchwoods.
Focused coverage is `torchwood_warms_then_ignites_across_two_columns` (a
SnowPea warms to a Pea at the first Torchwood and ignites at a second), and the
existing Pea/SnowPea tests retain the type behavior; source evidence is local
`Plant.cpp` `UpdateTorchwood` (`1374-1400`) and `GetPlantAttackRect`
(`5200-5229`), plus `Projectile.cpp` `ConvertToFireball`/`ConvertToPea`
(`1194-1230`); Issue `#233`.

Potato Mine acquisition now applies the source `FindTargetZombie` PotatoMine
branch at the trigger: an armed mine ignores a bouncing Pogo carrying its
object and a Pole Vaulter in vault, alongside the existing same-row, damage,
and 60-unit eligibility, instead of accepting every nearby ground zombie. The
detonation keeps the source 60-unit damage-range filter. Focused coverage is
`potato_mine_ignores_a_bouncing_pogo_until_a_ground_target_arrives`; source
evidence is local `Plant.cpp` `FindTargetZombie` (`4811-4925`, PotatoMine
branch `4890-4912`) and `UpdatePotato` (`1138-1187`); the Bungee target-column
rule is not yet modeled because the steal Bungee stores no target cell;
Issue `#210`.

`BossBungeeLeave`/`BossAreBungeesDone` coordinate the source Boss Bungee leave
phase and buttered-follower cleanup (`Zombie.cpp:9913-9973`). Rust does not
claim that lifecycle because it has no Boss Bungee phase or follower-ID state.

Adventure mode presents now follow the source `Board::DropLootPiece` boundary:
level 22 emits the minigame present and level 36 emits the puzzle present only
after wave 5, when the matching unlock bit is clear, and when no uncollected
present of that type is on the board. The Rust handoff keeps loot-bearing and
event-only death paths distinct, persists collected mode bits, and covers head
loss and final-wave award suppression. The source upsell-cutscene gate and
`PresentSurvivalMode` trigger remain unresolved.

The normal Adventure Select route now applies the target selector's independent
Minigame, Puzzle, and Survival lock bits, while a completed Adventure run opens
all three entries. The selector's locked visual is represented by the existing
renderer alpha path; exact grey tint, locked-message modal, and the source
Survival-present drop trigger remain unresolved and are not counted as new
accepted UI or visual units.

The board pickup renderer now loads the target `Sun`, `Coin_silver`,
`Coin_gold`, and `Diamond` compiled reanimations, using the source attachment
coordinates for the 60x60 sun and the three coin variants; moving silver/gold
coins retain the source static-image fallback. The target Present, moneybag,
chocolate, Scary Pot, small note, and silver/gold sunflower trophy resources
remain mapped alongside the existing money and seed-packet images. Source
collection particles, payload-specific seed art, remaining pickup animations,
and visual checkpoint acceptance remain under `ENT-PICKUP`, `ENT-EFFECT`, and
the VIS rows.

Chocolate collection now handles `Chocolate` and `AwardChocolate` through one
normalized usable-charge counter persisted by `SaveInventory.chocolates`; legacy
missing fields default to zero. The source purchase-count sentinel and separate
garden chocolate visibility/consumption state are not yet modeled.

The ordinary `Board::DropLootPiece` selector is implemented as the shared
`drop_loot_piece` path. It preserves the source mode-present precedence,
first-coin forcing, Column/Whack branches, 70-wave limit, Zen Garden capacity
and Stinky gates, packet affordability suppression, coin-value selection, and
caller-specific factors from zombie loot, I Zombie brain scoring, and Grave
Buster. Source cross-checks, the focused deterministic tests, and the exact
caller boundaries are recorded in ignored
`artifacts/ordinary-loot/verification.md`.

The Whack-a-Zombie sun contract is resolved: source `mSunMoney` is the modeled
`GameState.sun` pool, with source-aligned zero/150/5000 starting values by mode.
Rust collects and spends sun immediately, so the source in-flight collection
term is zero; Whack-a-Zombie's sun-drop thresholds remain deferred with the
ordinary selector.

The award integration audit preserves all three source `DropLootPiece` callers,
the zombie-value factors, the 75/500/2000/8000 packet-upgrade schedule, and the
source Silver/Gold/Diamond values. I Zombie and Grave Buster now retain their
caller boundaries, while profile challenge records, adventure completion state,
and the ordinary selector remain the shared prerequisite for exact replay
awards.

Fixed Adventure completion awards remain separate from ordinary loot. The source
first-run notes/seed packets, level-50 awards, replay money bags, and level-35
intermediate Scary Potter suppression are covered by the shared completion-award
helper. The level-50 Boss prerequisite and randomized endless potted-plant
payloads remain outside this simulation slice.

Non-Adventure completion awards preserve the source distinction between finite
money-bag/trophy outcomes, gold-sunflower progression, and endless Scary
Potter/I Zombie `PuzzlePhaseComplete` awards. Endless award fan-out keeps the
source diamond and five-gold-coin behavior; full profile challenge-record
selection remains unresolved.

Endless I Zombie and endless Scary Potter/Vasebreaker now use the shared source
`PuzzlePhaseComplete` continuation: they advance `challenge.stage`, reset or
repopulate mode state, and emit event-only deaths instead of winning on the
first clear. The source 500-tick fade/advice pacing and randomized garden award
payloads remain documented limitations.

Last Stand follows the source five-stage, ten-wave onslaught lifecycle: the start
and continue action, stage-offset wave budget and pool-row gate, six-card seed
chooser with forbidden seeds, seed refresh, transient-entity clear, and next
stage rebuild are modeled. The source advice/fade pacing, localized button text,
and exact button art remain outside this simulation slice.

The exact shared challenge-award helper remains blocked by profile shape. Source
records are per concrete `GameMode`, while Rust currently aggregates completion
by broad `ModeKind`; no speculative save field or generic award subsystem is
added until those records are represented.


## Player-Accessible Modes

| Obligation | Mode domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| MODE-ADVENTURE | Adventure levels | 50 | 50 | partial | Ignored `artifacts/adventure-levels/verification.md` and `artifacts/adventure-evidence/`; all 50 level identities (scene, wave totals, flag rules, conveyor membership, source wave composition via pick_adventure_waves, and runtime spawning with allow-list identity) have deterministic source checks; source pacing, scene-routed final-wave rises (graves/pool/sky), award/unlock identities, and setup/tutorial-gating identities implemented with deterministic checks; profile progression wiring, visual, and audio obligations remain; Issue `#18` |
| MODE-SURVIVAL | Visible normal, hard, and endless survival variants | 11 | 11 | partial | Ignored `artifacts/survival/verification.md`; all 11 stage scene, wave-count, endless-profile, and core repick identities have deterministic source checks; progression, visual, and audio remain; Issue `#18` |
| MODE-MINIGAME | Mini-game levels | 20 | 20 | partial | Ignored `artifacts/minigames/verification.md`; all catalog levels have deterministic source scene, wave, interaction, fixed seed-bank, or profile checks; conveyor checks are covered by ignored `artifacts/conveyor/verification.md`; progression, visual, and audio obligations remain; Issue `#18` |
| MODE-VASE | Vasebreaker levels, including endless | 10 | 10 | verified | Ignored `artifacts/vasebreaker/verification.md`; all 10 level identities have deterministic source-layout checks, with reveal, rejection, spawning, and win behavior covered by focused core evidence |
| MODE-IZOMBIE | I, Zombie levels, including endless | 10 | 10 | verified | Ignored `artifacts/izombie/verification.md`; source-defined layouts, fixed zombie seed banks, bank membership, deployment, brain eating, replay identity, and all 10 visible level identities have focused core evidence |
| MODE-GARDEN | Zen, mushroom, aquarium, and tree services | 4 | 4 | verified | Ignored `artifacts/garden-services/verification.md`; `garden_services_have_dedicated_state_and_inputs` covers all four service identities, service-specific input paths, and return to Adventure Select |

Column As You See 'Em now retains its source six-card conveyor start, wave-9
2,400-tick opening, 30-wave composition, six-times point budget, fixed
Ladder/Jack-in-the-Box/Gargantuar injections, flag pauses, and 750-tick
subsequent waves. `column_challenge_uses_the_source_wave_offset_and_injections`
checks the start state, fixed wave counts, source pool, high-density final wave,
and runtime spawn event sequence against local 1.0.0.1051 source
`Challenge.cpp:367-375,511-515` and `Board.cpp:647-790,5498-5508`; Issue `#248`.

Seeing Stars now uses its 14-cell source Starfruit pattern, rejects other seeds
on marked cells without consuming input state while retaining Lily Pad/Pumpkin
overlays, completes immediately when every normal position holds a Starfruit,
and shows source-style translucent missing-cell sprites. The 40-wave fallback
profile remains available until that completion. Focused core coverage is
`seeing_stars_uses_the_source_pattern_and_completion`; source evidence is local
`Challenge.cpp:279-286,2235-2247,2285-2305,2308-2324,2434-2445,2478-2484`;
Issue `#246`.

Beghouled and Beghouled Twist now start deterministic six-type 8x5 boards with no
immediate matches and a legal move, keep the source 200-tick zombie cadence and
1,500-tick challenge clock, resolve valid swaps/rotations through the source
100-tick moving/falling window, score line matches with sun rewards, refill the
board, and delay no-move shuffles by 500 ticks. Standard Beghouled accepts only
coordinate-based adjacent swaps; both modes leave zombie-eaten cells as
persistent, 200-sun-clearable craters. Focused core coverage is
`beghouled_standard_rejects_bad_swaps_and_resolves_a_match`,
`beghouled_standard_eaten_cells_are_craters_until_cleared`,
`beghouled_twist_initializes_board_and_needs_a_match`,
`beghouled_twist_validates_rotation_and_refills_matches`, and
`beghouled_twist_score_can_complete_level`; the App converts standard drags to
adjacent swaps. Source evidence is local
`Challenge.cpp:484-500,541-576,594-724,633-637,787-887,1019-1112,1533-1600,2094-2130,3617-3643`; Issues `#244` and `#247`.

Adventure completion now advances from the result state to the next source
level, persists the profile before constructing that level, and keeps first-run
seed-slot rules until the first 50-level round is complete. Focused coverage is
`adventure_first_run_rules_survive_between_levels`; the App routes Enter and
left-click from `Complete` through the same profile path. Existing profile
round-trip coverage remains in `profile_progression_round_trips_game_state`.

Zombiquarium now initializes two source-style free-swimming Snorkels with
independent `50..650`/`100..400` coordinates and the source 200/300 health
profile. Feed clicks cost 5 sun, create click-position brains only after the
source 15-tick arm window, enforce the three-brain cap, and let injured Snorkels
seek, consume, and heal from the nearest brain. The Snorkel packet costs 100 sun
and adds another aquarium entity; the trophy packet uses the source 1,000-sun
cost. Focused coverage is `zombiquarium_uses_snorkels_and_brain_bait`; source
evidence is local `Challenge.cpp:524-529,3648-3721` and
`Zombie.cpp:780-789,3053-3200`; Issue `#243`.

Slot Machine now accepts the source seed-bank handle hitbox, charges 25 sun only
for an unlocked spin, holds all three reels for the source 300-tick roll, uses
the source weighted symbol selection and third-reel match bias, pays the source
two-of-a-kind and jackpot diamond/sun/usable-seed fan-outs, including the
source x=360 offset for two-diamond and two-seed payouts, and reaches the
2,000-sun completion path. Focused coverage is
`slot_machine_rolls_three_reels_and_rejects_locked_or_empty_spins`,
`slot_machine_resolves_each_source_payout_class`,
`slot_machine_sun_jackpot_can_reach_the_source_completion_path`, and the legacy
state default check; source evidence is local `Challenge.cpp:1287-1298,1332-1335,2005-2048`
and `SeedPacket.cpp:33-82,162-180`; Issues `#245` and `#284`.

Portal Combat now preserves the source four-portal pair layout, conveyor and
challenge clocks, portal-row spawn weights, target-column `mLastPortalX`
re-entry guard, zombie/projectile/triggered-mower transport, and relocation
constraint that a moved portal cannot share a row or column with its pair.
Peashooter, Repeater, and Cactus use the source rightward portal path for
cross-row target acquisition. Focused coverage is
`portal_combat_uses_source_pairs_timers_and_row_weights`,
`portal_combat_transfers_paired_entities`, and
`portal_combat_shooters_target_zombies_across_portal_rows`; source evidence is
local `Challenge.cpp:3140-3376` and `Plant.cpp:4814-4862`.

## Screens, Input, and Persistence

| Obligation | Behavior domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| UI-SCREEN | Loading, title, menu, selector, seed chooser, HUD, pause, options, help, almanac, shop, and result flows | 4 | 12 | partial | PR `#64`, runs `30030645194`/`30030645257` and `30048802223`/`30048802288`, ignored `artifacts/windows-7de9f1d/verification.md`/`artifacts/windows-beb00d3/verification.md` and `artifacts/pause-menu/verification.md`; selector/adventure tutorial and pause dialog routes accepted; Options, Help, Almanac, GameOver, Complete, seed chooser, and shop remain partial; Issues `#2`, `#17` |
| INPUT-ACTION | Mouse, keyboard, hover, click, drag, placement, pause, restart, and command-line resource selection | 8 | 8 | verified | PR `#64`, commits `547f99b`/`514906c`, title mouse start, seed-chooser start/card selection and keyboard selection, in-play SeedBank packet hit routing, left-click placement, Space pause/resume, pause-dialog resume, and local terminal restart in ignored `artifacts/windows-a6c3f53/verification.md`/`artifacts/windows-aa443d7/verification.md`/`artifacts/windows-23f3f67/verification.md`/`artifacts/windows-514906c/verification.md`/`artifacts/windows-514906c/verification-keyboard.md`/`artifacts/local-restart/verification.md`/`artifacts/pause-menu/verification.md`/`artifacts/board-hud/verification.md`; runs `30054043147`/`30054043130`; remaining visual semantics are tracked by Issues `#2`/`#17` |
| SAVE-PROGRESSION | Profile, settings, unlocks, awards, inventory, garden, mode completion, and load compatibility | 8 | 8 | verified | PR `#61`, runs `30024232209`/`30024232459`, core profile round-trip test, ignored `artifacts/profile-progression/verification.md` |
| PLATFORM-CONTRACT | Logical viewport, window/fullscreen behavior, DPI, audio device, and external-path behavior | 6 | 6 | verified | PR `#64`, 800x600 logical viewport, window startup, DPI-aware 1000x750 client capture, external `--data-dir` launch, and local debug `--fullscreen`/F11 round trip in ignored `artifacts/local-platform/verification.md`; the same artifact records the local app's graceful startup fallback when the audio backend reports an unavailable-device error. Remaining pause-menu, visual, and full audio obligations remain under Issues `#2`, `#14`, `#17`, and `#19` |

The shop slice now has a source-aligned purchase action for packet upgrades,
fertilizer, bug spray, phonograph, and Stinky. It is reachable from the
Adventure Select store icon after the adventure unlock, renders against the
external Store resources, and persists through the existing profile handoff.
Focused state and external-launch evidence is recorded in ignored
`artifacts/store/verification.md`; the remaining StoreScreen pages, catalog
items, visual review, and purchase-specific audio remain unresolved, so this
shop slice does not add another accepted UI-SCREEN unit.

The pause dialog now follows the source modal route for board scenes: the
existing core pause state is rendered with the target dialog panel pieces and
resume text, and Escape, Space, Enter, or a left click resumes the board.
Source, focused nextest, external launch, and a local checkpoint capture are
recorded in ignored `artifacts/pause-menu/verification.md`.

The Options slice now follows the selector-specific source dialog route for
music/effects sliders, fullscreen, and the `OK` button. Values update the
existing audio tracks and `SaveSettings`, and the source Options resources are
loaded from the external 1.0.0.1051 directory. The selector Almanac/Credits,
Restart, Main Menu, hardware-capability dialogs, visual review, and device
audio synchronization remain unresolved; source and runtime evidence are in
ignored `artifacts/options/verification.md`.

The Help route now follows the source `AwardScreen::AWARD_HELP_ZOMBIENOTE`
paper-note path from the Game Selector, including the external background,
Zombie Note, help content, and source Main Menu button regions. Its focused
nextest and external-resource launch evidence are recorded in ignored
`artifacts/help/verification.md`; visual review and the separate Almanac,
Credits, and result/award variants remain unresolved.

The seed chooser now uses the source `Has7Rows` threshold and shared card
coordinates: unfinished adventures expose the 40-slot, 73-pixel-row layout,
while completed adventures or the relevant higher-tier plant unlocks expose
the 48-slot, 70-pixel-row layout. Mouse hit testing follows the rendered card
rectangles, including the previously unreachable right-hand columns. The
13-cell `packet_plants.png` atlas is now used for the source special packet
types, with the existing static plant images reused where available. The
Imitater special button, the remaining per-seed reanimation art, source
warning states, and visual acceptance remain unresolved, so this does not add
another accepted UI-SCREEN unit.

The in-play board now has a source-shaped SeedBank route backed by the external
1.0.0.1051 `SeedBank`, `SunBank`, `ShovelBank`, and conveyor resources. Ordinary
packet positions, bank extension widths, slot-machine and conveyor layouts,
sun/cost labels, cooldown dimming, and left-click packet selection follow the
local `Board.cpp:1304-1312,8927-8943` and `SeedPacket.cpp:523-574,882-1003`
evidence. Focused geometry/hit tests, a locked debug build, and a direct
window capture against the verified resource inventory are recorded in ignored
`artifacts/board-hud/verification.md`; unsupported per-seed reanimations,
shovel cursor semantics, and original-vs-neopvz comparison remain
open, so the HUD does not add another accepted UI-SCREEN unit.

The board now also draws the source `FlagMeter` background, dynamic clipped fill,
progress label, Adventure/Survival flag markers, and moving zombie head. Its
countdown mapping, Boss-health path, and source asset dimensions are covered by
focused tests and the external `day` checkpoint; evidence is recorded in ignored
`artifacts/board-progress/verification.md`. Non-Boss health-threshold
interpolation, exact raise/easing timing, challenge-specific labels, and a
valid WGPU window capture remain unresolved, so this remains a partial visual
slice and does not add another accepted UI-SCREEN unit.

Board entities now use source compiled reanim definitions for Normal, Flag,
Conehead, Buckethead, Screen Door, Football, Newspaper, the 16 available
specialized body types, and the Boss body/driver/fireball/iceball set in the
1.0.0.1051 archive. The catalog also loads the target Peashooter, Sunflower,
Snow Pea, Puff-shroom, Fume-shroom, Wall-nut, Starfruit, Magnet-shroom, and
the remaining source seed-slot definitions for board plant rendering.
The Rust decoder follows the target `Definition.cpp` cache layout and its
missing-field inheritance, maps the zombie/Boss set's 687 tracks and adds 734
tracks across all 49 source plant slots, resolving 963 same-version image
symbols. It interpolates the source transform fields, applies the source
equipment visibility groups and additive effect groups, and attaches the Boss
driver to `Boss_head2` with the source offset and scale. Newspaper armor
transitions, Football helmet loss, the supported specialized movement/eating
actions, and Boss idle/attack/RV/death states select source action tracks; the
supported plants select source sleep/armed/explode/shoot/big-idle tracks where
state and definition provide them. Plant render groups now also hide PotatoMine
glow until armed, switch KernelPult's Cornpult butter/kernel prefixes from the
stored weapon state, place Pumpkin_back behind the main shell, and compose the
source Peashooter-family, SplitPea, and Threepeater head reanimations through
their body anchor tracks. Backup Dancer has no matching target resource. Source
format checks, focused layer visibility/attachment tests, plant action tests,
and the external checkpoints are recorded in ignored
`artifacts/reanim/verification.md`; Boss fireball/iceball trail particles,
other attachments, particles, clipping, damage image overrides, other
plant-specific head attachments, exact action timing/blink state, and visual
acceptance remain unresolved, so this does not add an accepted VIS-ZOMBIE or
VIS-PLANT unit.

The Almanac selector route now follows the source unlock threshold and exposes
Index, Plants, and Zombies page states with the target 1.0.0.1051 page
backgrounds, close/index/navigation controls, source-shaped 49-plant and
26-zombie grids, and entry selection. The source Imitater slot and Boss
position are preserved, and zombie display order is mapped independently from
the internal Rust enum order. Selected cards now display source-ordered plant
and zombie names, and plant cards display cost and recharge metadata from the
core seed definition table. The save-backed plant availability rule is used
when a profile is present; unsupported entity reanimations remain blank, and
source description strings are loaded from the external `LawnStrings.txt` table
when a card is selected. Zombie slots now follow the source first-appearance
level, spawned-only, Yeti silhouette, and not-encountered description rules;
the core tracks and carries the source's in-memory `gZombieDefeated` state for
the active player session, and entity
reanimations, animation review, and visual acceptance are unfinished. Evidence is in ignored
`artifacts/almanac/verification.md`.

The GameOver route now renders a source-shaped failure sequence over the
terminal board: natural losses keep the board visible, hold result input until
the source 11000-millisecond boundary, and show the external masked
`reanim/ZombiesWon` art from the 6000-millisecond boundary before opening the
result dialog. Try Again still maps to the existing restart path and Main Menu
to Adventure Select. The source board-pan curve, reanimation frame progression,
shake behavior, challenge-specific message/header, mode-specific menu
destinations, footer lifecycle, original lost-board background, and visual
acceptance remain unresolved; implementation evidence is in ignored
`artifacts/game-over-visual/verification.md`, so this slice does not add
another accepted UI-SCREEN unit.

The Complete route now selects the current state award and renders the external
AwardScreen background, seed-packet plant reward, shovel/Almanac/keys/taco/
watering-can tools, trophy outcomes, and the supplied paper-note pages. Continue
uses the existing profile-aware next-level transition, while Main Menu returns
to Adventure Select. The level-49 final-note asset is absent from the supplied
resources and falls back to the last note page; exact award text/art cropping,
store and Zen Garden handoffs, audio timing, and visual acceptance remain
unresolved. Evidence is in ignored `artifacts/complete/verification.md`, so
this slice does not add another accepted UI-SCREEN unit.

The final first-run completion now enters a Credits route after the level-50
award, and `--checkpoint credits` starts the same route directly. Its source
phase boundaries (400, 785, and 1033 movie frames), final scrolling section,
50-update button delay, replay/main-menu regions, source Credits image assets,
and `ZombiesOnYourLawn.ogg` music path are covered by focused nextest checks.
The renderer still lacks the source Reanimation attachment/particle system, so
the phase scenes use the supplied static backgrounds and Credits art; exact
singing-Sunflower, zombie choreography, fog/disco effects, and visual acceptance
remain unresolved. This reduces the Credits behavior gap but does not add an
accepted UI-SCREEN or VIS unit.

## Visual and Audio Evidence

| Obligation | Evidence domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| VIS-SCREEN | Declared visual checkpoints for player-accessible screens | 0 | 12 | missing | Issue `#14` |
| VIS-MODE | Declared visual checkpoints for every player-accessible mode unit | 0 | 105 | missing | Issue `#14` |
| VIS-PLANT | Plant animation, layering, clipping, and feedback review units | 0 | 49 | missing | Issue `#14` |
| VIS-ZOMBIE | Zombie animation, layering, clipping, and feedback review units | 0 | 33 | missing | Issue `#14` |
| VIS-PROJECTILE | Projectile animation and impact review units | 0 | 14 | missing | Issue `#14` |
| VIS-EFFECT | Effect and particle review units | 0 | 105 | missing | Issue `#14` |
| AUD-SFX | Simulation-tick and decoded-output sound-event units | 151 | 167 | partial | `SeedSelected` 閳?`sounds/tap.ogg`, `InputRejected` 閳?`sounds/buzzer.ogg`, `Paused` 閳?`sounds/pause.ogg`, `ZombieDeployed` 閳?`sounds/plant.ogg`/`sounds/plant2.ogg`, `PlantShoveled` 閳?`sounds/plant2.ogg`, plant `SunProduced` -> `sounds/throw.ogg`, special-prize `CoinProduced` -> `sounds/chime.ogg`, Gold `CoinLanded` -> `sounds/moneyfalls.ogg`, `SunCollected` 閳?`sounds/points.ogg`, Diamond `CoinCollected` -> `sounds/diamond.au`, usable-seed `PickupCollected` -> `sounds/seedlift.ogg`, sun `PickupCollected` -> `sounds/points.ogg`, prize `PickupCollected` -> `sounds/prize.ogg`, money `CoinCollected` 閳?`sounds/coin.ogg`, `GardenWatered` 閳?`sounds/watering.ogg` + companion `sounds/throw.ogg`, `GardenFertilized` 閳?`sounds/fertilizer.ogg` + companion `sounds/throw.ogg`, `GardenBecameHappy` 閳?`sounds/prize.ogg` + companion `sounds/throw.ogg`, IceShroom `PlantSpecialTriggered` 閳?`sounds/frozen.ogg`, `ZombieChilled` 閳?`sounds/frozen.ogg`, `CobCannonFired` 閳?`sounds/coblaunch.ogg`, Catapult `ProjectileFired { Other(1) }` 閳?`sounds/basketball.ogg`, Torchwood `ProjectileIgnited` -> `sounds/firepea.ogg`, `PortalOpened` 閳?`sounds/portal.ogg`, GraveBuster `PlantSpecialTriggered` 閳?`sounds/gravebusterchomp.ogg`, Coffee `PlantSpecialTriggered` 閳?`sounds/coffee.ogg`, TangleKelp `TangleKelpGrabStarted` 閳?`sounds/floop.ogg`, TangleKelp `TangleKelpWaterEntry` 閳?`sounds/zombiesplash.ogg`, PotatoMine `PotatoMineArmed` 閳?`sounds/dirt_rise.ogg`, Digger `DiggerSurfaced` 閳?`sounds/dirt_rise.ogg` + companion `sounds/wakeup.ogg`, Magnet-shroom `MetalStolen` 閳?`sounds/magnetshroom.ogg`, Zamboni `VehicleDisabled` 閳?`sounds/balloon_pop.ogg`, PotatoMine `PlantSpecialTriggered` 閳?`sounds/potato_mine.ogg`, Spikeweed `PlantSpecialTriggered` 閳?`sounds/throw.ogg`, CherryBomb `PlantSpecialTriggered` 閳?`sounds/cherrybomb.ogg`, ExplodeONut `PlantSpecialTriggered` 閳?`sounds/cherrybomb.ogg` + companion `sounds/bowlingimpact2.ogg`, Jalapeno `PlantSpecialTriggered` 閳?`sounds/jalapeno.ogg`, CherryBomb/Jalapeno companion 閳?`sounds/juicy.ogg`, `ProjectileImpact` Butter 閳?`sounds/butter.ogg`, `VaseBroken` 閳?`sounds/vase_breaking.ogg`, `RakeTriggered` 閳?`sounds/swing.ogg`, DoomShroom `PlantSpecialTriggered` 閳?`sounds/doomshroom.ogg`, `BloverTriggered` 閳?`sounds/blover.ogg`, Chomper `PlantSpecialTriggered` 閳?`sounds/bigchomp.ogg`, Squash `PlantSpecialTriggered` -> `sounds/gargantuar_thump.ogg`, `SquashHumStarted` -> `sounds/squash_hmm.ogg`/`sounds/squash_hmm2.ogg`, `ZombieShieldHit` -> `sounds/shieldhit.ogg`/`sounds/shieldhit2.ogg`, `ZombieHypnotized` 閳?`sounds/mindcontrolled.ogg`, `JackboxExploded` 閳?`sounds/explosion.ogg`, day/night/roof `MowerTriggered { pool: false }` 閳?`sounds/lawnmower.ogg`, pool `MowerTriggered { pool: true }` 閳?`sounds/pool_cleaner.ogg`, `GameLost` 閳?`sounds/losemusic.ogg`, `GameWon` 閳?`sounds/winmusic.ogg`, `ZombieNewspaperRipped` 閳?`sounds/newspaper_rip.ogg`, `ImpThrown` 閳?`sounds/swing.ogg` + variant companion `sounds/imp.ogg`/`sounds/imp2.ogg`, `DolphinRider` appearance 閳?`sounds/dolphin_appears.ogg`, `DolphinJumpStarted` 閳?`sounds/dolphin_before_jumping.ogg` + companion `sounds/plant_water.ogg`, `ZombieEnteredPool` 閳?`sounds/plant_water.ogg`/`sounds/zombie_entering_water.ogg`, `Zamboni` appearance 閳?`sounds/zamboni.ogg`, `PogoBounceSound` 閳?`sounds/pogo_zombie.ogg`, `PoleVaultGrassStep` 閳?`sounds/grassstep.ogg`, `PoleVaultSound` 閳?`sounds/polevault.ogg`, `Balloon` appearance 閳?`sounds/ballooninflate.ogg`, Jackbox/Digger `ZombieSongStarted` -> `sounds/jackinthebox.ogg`/`sounds/digger_zombie.ogg`, `ZombieGroaned` -> `sounds/lowgroan.ogg`/`sounds/lowgroan2.ogg`/`sounds/groan.ogg`/`sounds/groan2.ogg`, `ZombieChew` -> `sounds/chomp.ogg`/`sounds/chomp2.ogg`/`sounds/chompsoft.ogg`, Gargantuar `ZombieDeathSound` -> `sounds/gargantudeath.ogg`, Boss `ZombieDeathSound` -> `sounds/bossexplosion.ogg` + companion `sounds/gargantudeath.ogg`; source cross-checks, decoded PCM hashes, and local app tick/`playback started` traces in ignored `artifacts/local-audio/verification.md`; `PlantPlaced`/`ImitaterMorphed` contextual planting Foley -> `sounds/plant.ogg`/`sounds/plant2.ogg`/`sounds/plant_water.ogg`/`sounds/ceramic.ogg`; evidence in ignored `artifacts/planting-audio/verification.md`; `Resumed` and `ZombieDied` deliberately have no direct sound mapping; the accepted `ZombieDied` silent boundary and remaining SFX/device timing remain under Issue `#19` |
| AUD-MUSIC | Music playback, loop, and stem units | 1 | 2 | partial | Main/hihats MO3 loop duration and source track mapping verified in ignored `artifacts/music-loop/verification.md`; runtime playback/stem synchronization remains |
| AUD-SYNC | Event-to-device timing and music synchronization contract | 0 | 1 | missing | Issue `#19` |

The zombie song/groan/chew/death SFX slice maps `ZombieSongStarted`,
`ZombieGroaned`, `ZombieChew`, and `ZombieDeathSound` to the target
`jackinthebox`, `digger_zombie`, `lowgroan`/`lowgroan2`, `groan`/`groan2`,
`chomp`/`chomp2`/`chompsoft`, `gargantudeath`, and `bossexplosion` resource
families. Source boundaries are local `Zombie.cpp:4724-4777,7404-7412,8953-8957,10205-10214`;
the source-to-resource mapping, decoded PCM hashes, and focused core/App tests
are recorded in ignored `artifacts/audio-slice/verification.md`.

The accepted Adventure award-note SFX unit maps `GameWon` at levels 10, 20, 30,
40, and 50 to the source `AwardScreen::IsPaperNote` branch and
`sounds/paper.ogg`, after the normal win music or final fanfare. The focused
mapping test and the hidden `complete-paper` checkpoint both confirm tick-0
queue order and external playback; evidence is recorded in ignored
`artifacts/award-paper-audio/verification.md`.

The accepted GameOver failure-cutscene SFX units preserve the source
`CutScene::UpdateZombiesWon` boundaries: `GameLost` starts `losemusic.ogg`,
`GameLostChomp` emits `chomp.ogg` and `chomp2.ogg` at 5100 and 5600 cutscene
milliseconds, and `GameLostScream` emits `scream.ogg` at 6000. Source timing,
focused core/App checks, and external playback are recorded in ignored
`artifacts/game-over-audio/verification.md`; the result dialog and brain
animation lifecycle remain outside these accepted audio units.

The accepted I, Zombie deployment SFX unit maps successful `ZombieDeployed`
events to the source `plant.ogg`/`plant2.ogg` variation family. Source, event,
mapping, decode, and checkpoint evidence is recorded in ignored
`artifacts/izombie-deploy-audio/verification.md`.

The accepted Torchwood conversion SFX unit maps the source
`Projectile::ConvertToFireball` event to `sounds/firepea.ogg`; source, event,
mapping, external decode metadata, and checkpoint evidence is recorded in
ignored `artifacts/firepea-conversion/verification.md`.

The accepted Gold coin ground SFX unit maps the source
`Coin::PlayGroundSound` boundary to `sounds/moneyfalls.ogg`; source, event,
mapping, external decode metadata, and checkpoint evidence is recorded in
ignored `artifacts/moneyfalls/verification.md`.

The accepted Diamond collection SFX unit maps the source
`Coin::PlayCollectSound` branch to `sounds/diamond.au`; source, event,
mapping, external decode metadata, and checkpoint evidence is recorded in
ignored `artifacts/diamond-collection/verification.md`.

The accepted usable-seed collection SFX unit maps the source
`Coin::PlayCollectSound` branch to `sounds/seedlift.ogg`; source, event,
mapping, external decode metadata, and checkpoint evidence is recorded in
ignored `artifacts/usable-seed-collection/verification.md`.

The accepted prize collection SFX unit maps the source
`Coin::PlayCollectSound` prize branch to `sounds/prize.ogg`; source, event,
mapping, external decode metadata, and checkpoint evidence is recorded in
ignored `artifacts/prize-collection/verification.md`.

The accepted sun pickup collection SFX unit maps the source
`Coin::PlayCollectSound` `IsSun()` branch through `FOLEY_SUN` to
`sounds/points.ogg`; source, event, mapping, external decode metadata, and
checkpoint evidence is recorded in ignored
`artifacts/sun-pickup-collection/verification.md`.

The accepted reverse-explosion SFX unit maps in-play Cherry Bomb and Jalapeno
`PlantPlaced` events to `sounds/reverse_explosion.ogg`; same-tick source,
mapping, decode, and checkpoint evidence is recorded in ignored
`artifacts/reverse-explosion/verification.md`.

The accepted `JumpBlocked` SFX unit maps to `sounds/bonk.ogg`; its source and
local tick/playback evidence are recorded in ignored
`artifacts/local-audio/verification.md`.

The accepted `UmbrellaDeflected` SFX units map to `sounds/boing.ogg` and
`sounds/throw2.ogg`; both source calls and same-tick local playback evidence
are recorded in ignored `artifacts/local-audio/verification.md`.

The accepted BrainFinished SFX unit maps to sounds/gulp.ogg; source and
same-tick local playback evidence are recorded in ignored
artifacts/local-audio/verification.md.

The accepted Butter projectile-impact SFX unit maps `ProjectileImpact` with
`kind: Butter` to `sounds/butter.ogg`; the local trace uses the real projectile
collision path and does not treat `ZombieButtered` as a second audio boundary.

The accepted projectile-impact SFX units map `ProjectileImpact` variants to the
source `splat`, `kernelpult`, `ignite`, `melonimpact`, `shieldhit`, and
`plastichit` resource families. The local `projectile-impacts` trace covers all
six families on one simulation tick with external 1.0.0.1051 resources. The
accepted `ZombieDied` boundary is deliberately silent: same-version source
binds the three splat resources to `FOLEY_SPLAT`, but its call sites are the
projectile, plant, and lawn-mower impact paths rather than
`Zombie::DieWithLoot`/`DieNoLoot`/`MowDown`. The app keeps `ZombieDied` unmapped,
with the focused `zombie_died_has_no_direct_audio_mapping` test and ignored
`artifacts/zombie-died-sfx/verification.md` recording the source, function-table,
and external-resource checks.

The accepted ImpThrown SFX unit maps to sounds/swing.ogg and carries the
source FOLEY_IMP imp/imp2 variation as a same-tick companion; the local trace
and decoded PCM records are in artifacts/local-audio/verification.md.

The accepted Pole Vaulter SFX units map `PoleVaultGrassStep` to
`sounds/grassstep.ogg` on animation update 36 and `PoleVaultSound` to
`sounds/polevault.ogg` on update 72. The 43-frame `anim_jump` timing, decoded
PCM, and exact-tick local playback evidence are recorded in ignored
`artifacts/pole-vaulter/verification.md`.

The accepted DolphinJumpStarted SFX unit maps to
sounds/dolphin_before_jumping.ogg with the same-tick companion
sounds/plant_water.ogg; source, decoded PCM, and local playback evidence are
recorded in ignored artifacts/local-audio/verification.md.

The accepted DolphinRider appearance SFX unit maps to
sounds/dolphin_appears.ogg on the source-aligned spawn/appearance event; source,
decoded PCM, and local playback evidence are recorded in ignored
artifacts/local-audio/verification.md.

The accepted Zamboni appearance SFX unit maps to sounds/zamboni.ogg on the
source-aligned spawn/appearance event; source, decoded PCM, and local playback
evidence are recorded in ignored artifacts/local-audio/verification.md.

The accepted PogoBounceSound SFX unit maps to sounds/pogo_zombie.ogg at the
source-aligned bounce sound boundary; source, decoded PCM, and local playback
evidence are recorded in ignored artifacts/local-audio/verification.md.

The accepted Balloon appearance SFX unit maps to sounds/ballooninflate.ogg on
the source-aligned spawn/appearance event; source, decoded PCM, and local
playback evidence are recorded in ignored artifacts/local-audio/verification.md.

The accepted Squash landing SFX unit maps to sounds/gargantuar_thump.ogg on the
source-aligned PlantSpecialTriggered event; source, decoded PCM, and local
playback evidence are recorded in ignored artifacts/local-audio/verification.md.

The accepted ScreenDoor/Ladder shield-hit SFX unit maps to the source Foley
variation pair sounds/shieldhit.ogg or sounds/shieldhit2.ogg on
ZombieShieldHit; source, decoded PCM, and local playback evidence are recorded
in ignored artifacts/local-audio/verification.md.

The accepted SquashHumStarted SFX unit maps the source 2:1 variation table to
sounds/squash_hmm.ogg or sounds/squash_hmm2.ogg; source, decoded PCM, and
same-tick local playback evidence are recorded in ignored
artifacts/local-audio/verification.md.

The accepted ZombieEnteredPool SFX unit maps the source splash variation pair
to sounds/plant_water.ogg or sounds/zombie_entering_water.ogg; source, decoded
PCM, and same-tick local playback evidence are recorded in ignored
artifacts/local-audio/verification.md.

The accepted Catapult basketball SFX unit maps `ProjectileFired { Other(1) }`
to `sounds/basketball.ogg`; source, decoded PCM, and same-tick local playback
evidence are recorded in ignored `artifacts/local-audio/verification.md`.

The accepted plant-firing SFX units map source-aligned `PlantFired` events to
the 3:1 `sounds/throw.ogg`/`sounds/throw2.ogg` variation, add
`sounds/snow_pea_sparkles.ogg` for Snow Pea/Winter Melon and `sounds/puff.ogg`
for Puff/Scaredy/Sea-shroom, and use `sounds/fume.ogg` alone for Fume-shroom.
Gloom-shroom remains silent as in `Plant::Fire`. Same-version source, the
1.0.0.1051 function-table identity, decoded PCM, and same-tick local playback
evidence are recorded in ignored `artifacts/local-audio/verification.md`.

The accepted plant-sun production SFX unit maps `SunProduced` with
`source: SunSource::Plant(_)` to `sounds/throw.ogg`; sky-produced suns remain
silent. Same-version source evidence is `PvZ-Decomp-main/Lawn/Plant.cpp:1021-1044`
and `Sexy.TodLib/TodFoley.cpp:14`. The local `sun-production` checkpoint uses
the real Sunflower planting/update path with external 1.0.0.1051 resources;
`artifacts/local-audio/sun-production-trace.stdout.log` records both
`SunProduced` and `sounds/throw.ogg` queued/playback-started at tick 1, with
empty stderr and no playback-failed line.

The accepted special-prize launch SFX unit maps `CoinProduced` for Diamond,
Chocolate, AwardChocolate, PresentPlant, AwardPresent, and the three
advice-bearing mode presents to `sounds/chime.ogg`; all other pickup types stay
silent at launch. Same-version source evidence is
`PvZ-Decomp-main/Lawn/Coin.cpp:1298-1309,420-422` and
`Sexy.TodLib/TodFoley.cpp:104`. The local `prize-chime` checkpoint creates a
real Diamond through the normal pickup path; the preserved trace records its
`CoinProduced` event and `sounds/chime.ogg` queued/playback-started at tick 0
with external 1.0.0.1051 resources, empty stderr, and no playback failure.

The accepted Zen Garden need-fulfillment SFX unit maps the source
`ZenGarden::PlantFulfillNeed` `FOLEY_PRIZE` call to `GardenBecameHappy` and
`sounds/prize.ogg`; source, event, mapping, external decode metadata, and
checkpoint playback evidence are recorded in ignored
`artifacts/garden-fulfill/verification.md`.

The accepted Zen Garden bug-spray and phonograph tool SFX units map the source
`ZenGarden::MouseDownWithFeedingTool` `FOLEY_BUGSPRAY` and
`FOLEY_PHONOGRAPH` calls to explicit `GardenToolUsed` events and
`sounds/bugspray.ogg`/`sounds/phonograph.ogg`. Both tools then use the existing
need-fulfillment transition and preserve its prize and spawn-sun companions.
The core now persists per-plant need, growth stage, feeding count, and care
cooldown plus wall-clock care timestamps; explicit tools must match
`GetPlantsNeed`-equivalent state, and watering uses the source 7-15 second need
delay. Fertilizer and bug-spray purchase counters, phonograph ownership, and
their consumption rules are persisted with legacy-save defaults. Daily refresh
uses the persisted UNIX epoch day; exact source timezone/DST behavior remains a
known limitation.
Source, event, mapping, external decode metadata, and both checkpoint playback
traces are recorded in ignored `artifacts/garden-tools/verification.md`.

The accepted Zen Garden spawn-sun companion SFX unit maps the source
`ZenGarden::PlantWatered`, `PlantFertilized`, and `PlantFulfillNeed`
`FOLEY_SPAWN_SUN` calls to `sounds/throw.ogg` companions on their corresponding
events; source, event, mapping, external decode metadata, and three checkpoint
playback traces are recorded in ignored
`artifacts/garden-spawn-sun/verification.md`.

The accepted pool mower startup SFX unit maps the source
`LawnMower::StartMower` `FOLEY_POOL_CLEANER` branch to
`MowerTriggered { pool: true }` and `sounds/pool_cleaner.ogg`; ordinary mower
events retain `sounds/lawnmower.ogg`. Source, event discriminator, mapping,
external decode metadata, and the pool checkpoint playback trace are recorded
in ignored `artifacts/pool-mower/verification.md`.

The accepted mower hit SFX unit maps the source `LawnMower::MowZombie`
`FOLEY_SPLAT`/`FOLEY_SHOOP` calls to `MowerZombieHit`, preserving the source
pool-row mower distinction and the three-way land splat variation. The existing
`MowerTriggered` event remains the startup Foley boundary. Same-version source,
resource probe, focused core/App checks, and both land/pool hidden checkpoint
playback traces are recorded in ignored
`artifacts/mower-hit-audio/verification.md`.

The accepted Boss mower-squish SFX unit maps the source
`Zombie::UpdateBossFireball` `FOLEY_SQUISH` call to `MowerSquished`, preserving
the source distinction between a rolling Boss ball and an ordinary mower
trigger. The two `SOUND_CHOMP` variants are mapped to `sounds/chomp.ogg` and
`sounds/chomp2.ogg`. Source, resource probe, focused Boss/App checks, and the
hidden playback checkpoint are recorded in ignored
`artifacts/mower-squish-audio/verification.md`.

The accepted Tree of Wisdom growth SFX unit maps the source
`Challenge::TreeOfWisdomGrow` `FOLEY_PLANTGROW` call to `GardenTreeGrew` and
`sounds/plantgrow.ogg`; source, event timing, mapping, external decode metadata,
and checkpoint playback evidence are recorded in ignored
`artifacts/garden-tree-grow/verification.md`.

The accepted huge-wave SFX unit maps the source `Board::UpdateZombieSpawning`
`SOUND_HUGE_WAVE` boundary at countdown 725 to `HugeWaveSound` and
`sounds/hugewave.ogg`; source, event timing, mapping, external decode metadata,
and checkpoint playback evidence are recorded in ignored
`artifacts/huge-wave-sound/verification.md`.

The accepted final-wave SFX unit maps the source `Board::NextWaveComing` 60-tick
counter and `Board::UpdateZombieSpawning` playback boundary to
`FinalWaveSound` and `sounds/finalwave.ogg`. The core preserves the source
exclusions for Survival repick stages, Last Stand, and continuous challenges;
Whack-a-Zombie retains its separate spawning path but still calls the shared
final-wave boundary. Same-version source, event timing,
resource decode metadata, and focused core/App tests are recorded in ignored
`artifacts/final-wave-sound/verification.md`.

The accepted Adventure final-fanfare SFX unit maps the source
`Board::FadeOutLevel` Adventure level-50 branch to the contextual `GameWon`
sequence and `sounds/finalfanfare.ogg`; ordinary wins retain `sounds/winmusic.ogg`.
The separate source branch keyed by `TrophiesNeedForGoldSunflower() == 1`
remains unresolved because the current core does not expose an equivalent trophy
count. Source, resource probe, focused App check, and the hidden final-fanfare
checkpoint are recorded in ignored `artifacts/final-fanfare/verification.md`.

The accepted Slot Machine launch SFX unit maps the source
`Challenge::MouseDown` successful handle-pull branch, after the 25-sun charge,
to `ChallengeAction { kind: SlotMachine, value: roll_count }` and
`sounds/slotmachine.ogg`. The mapping intentionally stays silent for rejected
spins and for the later reel-settlement update. Same-version source, event
mapping, resource decode metadata, and focused App tests are recorded in ignored
`artifacts/slot-machine-sound/verification.md`.

The accepted Zombiquarium Snorkel purchase SFX unit maps the source successful
Snorkel packet branch to the `FOLEY_ZOMBIESPLASH` two-resource variation and
`ZombiquariumSnorkelPurchased`; the runtime paths are
`sounds/plant_water.ogg` and `sounds/zombie_entering_water.ogg`. Feeding brains,
buying the Trophy, and rejected purchases do not emit this event. Evidence is
recorded in ignored `artifacts/zombiquarium-snorkel/verification.md`.

The accepted Zombiquarium brain-consumption SFX unit maps the source near-distance
`FOLEY_SLURP` call to `ZombiquariumBrainSlurped` and `sounds/slurp.ogg`; it is
emitted only by the aquarium bite branch. Evidence is recorded in ignored
`artifacts/zombiquarium-brain-slurp/verification.md`.

The accepted Zombiquarium death SFX unit maps the periodic-damage death branch's
`SOUND_ZOMBAQUARIUM_DIE` call to `ZombiquariumZombieDied` and
`sounds/zombaquarium_die.ogg`, separate from ordinary zombie death events.
Evidence is recorded in ignored `artifacts/zombiquarium-death-sound/verification.md`.

The accepted vehicle explosion SFX unit maps ordinary Zamboni and Catapult death
`FOLEY_EXPLOSION` calls to `VehicleExploded` and `sounds/explosion.ogg`; the
spike-disabled `VehicleDisabled` path remains `sounds/balloon_pop.ogg`.
Evidence is recorded in ignored `artifacts/vehicle-explosion-sound/verification.md`.

The accepted Balloon pop SFX unit maps the source `Zombie::LandFlyer`
`SOUND_BALLOON_POP` boundary to `BalloonPopped` when flying-balloon health
reaches zero, including the pool death path, and maps it to
`sounds/balloon_pop.ogg`. Same-version source, resource probe, focused checks,
and the hidden startup checkpoint are recorded in ignored
`artifacts/balloon-pop-audio/verification.md`.

The accepted Bungee landing SFX unit maps the below-1500 altitude
`FOLEY_BUNGEE_SCREAM` call to `BungeeScream` and the three scream resources;
ordinary target stealing and the Boss Bungee lifecycle remain separate. Evidence
is recorded in ignored `artifacts/bungee-scream/verification.md`.

The accepted Bungee plant-lift SFX unit maps the source
`BungeeLiftTarget` `FOLEY_FLOOP` call to `BungeePlantLifted` and
`sounds/floop.ogg`; umbrella deflection and empty target cells stay silent at
this boundary, and the lifted plant is held out of normal updates until the
rise completes. Evidence is recorded in ignored
`artifacts/bungee-lift-audio/verification.md`.

The accepted Bungee landing grassstep SFX unit maps the source descending
`BUNGEE_ZOMBIE_HEIGHT - 404` `FOLEY_GRASSSTEP` boundary to
`BungeeGrassStep` and `sounds/grassstep.ogg`. A Bungee carrying a dropped
zombie stays silent at this boundary. Same-version source, resource probe,
focused core/App checks, and the hidden startup checkpoint are recorded in
ignored `artifacts/bungee-grassstep/verification.md`.
The accepted Jack-in-the-Box surprise SFX unit maps the source
`Zombie::UpdateZombieJackInTheBox` remaining-counter `80` Foley boundary to
`JackboxSurprise` and the 2:1 `jack_surprise` resource family. Frozen Jacks
pause before the boundary and the final explosion remains a separate event.
Same-version source, resource probe, focused core/App checks, and the hidden
startup checkpoint are recorded in ignored `artifacts/jackbox-audio/verification.md`.

The accepted Jack-in-the-Box boing SFX unit maps the source
`Zombie::UpdateZombieJackInTheBox` transition into the 110-tick popping phase
to `JackboxBoing` and `sounds/boing.ogg`; the later counter-80 surprise remains
a separate event. Same-version source, resource probe, focused core/App
checks, and the hidden startup checkpoint are recorded in ignored
`artifacts/jackbox-boing-audio/verification.md`.

The accepted Newspaper Rarrgh SFX unit maps the source
`Zombie::UpdateZombieNewspaper` post-`anim_gasp` transition to
`ZombieNewspaperRarrgh` and the 2:1 `newspaper_rarrgh` resource family. The
source `CountZombiesOnScreen() <= 10` and `mHasHead` gates are preserved, and
paper-rip playback remains a separate event. Same-version source, resource
probe, focused core/App checks, and the hidden startup checkpoint are recorded
in ignored `artifacts/newspaper-rarrgh-audio/verification.md`.
The accepted zombie falling SFX unit maps the source ground-death
`FOLEY_ZOMBIE_FALLING` boundary to `ZombieFallingSound` and the two
`zombie_falling` resources. Source-excluded special types, pool deaths, and
Zombiquarium deaths remain silent; Gargantuar deaths retain the source
`gargantuar_thump` companion. Same-version source, resource probe, focused
core/App checks, and the hidden startup checkpoint are recorded in ignored
`artifacts/zombie-falling-sound/verification.md`.

The accepted Backup Dancer summon SFX unit maps the source
`Zombie::SummonBackupDancer` `FOLEY_GRAVESTONE_RUMBLE` call to
`ZombieSpawned { BackupDancer }` and `sounds/gravestone_rumble.ogg`; source,
event timing, mapping, external decode metadata, and checkpoint playback
evidence are recorded in ignored `artifacts/dancer-rumble/verification.md`.

The accepted Dancer leader SFX unit maps the source
`Zombie::UpdateZombieDancer` post-snap `FOLEY_DANCER` call to
`DancerRumble` and `sounds/dancer.ogg`, preserving the `CountZombiesOnScreen() <= 15`
gate and the event's ordering before Backup Dancer summoning. Same-version
source, resource probe, focused core/App checks, and the hidden startup
checkpoint are recorded in ignored
`artifacts/dancer-rumble-audio/verification.md`.

The accepted Garlic yuck SFX unit maps the source
`Zombie::UpdateYuckyFace` 70-update boundary, plus its early non-yucky-face
branch, to `ZombieYuckSound` and the 2:1 `yuck` resource family, preserving the
source headed-type and `CountZombiesOnScreen()` gates. Same-version source,
resource probe, focused core/App checks, and the hidden startup checkpoint are
recorded in ignored `artifacts/garlic-yuck-audio/verification.md`.

The accepted first-wave SFX unit maps the source `Board::StartWave`
`SOUND_AWOOGA` call for wave 0 to `WaveStarted { wave: 0 }` and
`sounds/awooga.ogg`; source, event timing, mapping, external decode metadata,
and checkpoint playback evidence are recorded in ignored
`artifacts/first-wave-sound/verification.md`.

The accepted flag-wave SFX unit maps the source `Board::StartWave`
`SOUND_SIREN` call to `FlagWaveSound` and `sounds/siren.ogg`; source, event
timing, mapping, external decode metadata, and checkpoint playback evidence
are recorded in ignored `artifacts/flag-wave-sound/verification.md`.

The accepted boss attack SFX unit maps the source `Zombie::BossHeadSpit`
`FOLEY_BOSS_BOULDER_ATTACK` call to `BossAttackWindup` and
`sounds/bossboulderattack.ogg`; source, event timing, mapping, external decode
metadata, and checkpoint playback evidence are recorded in ignored
`artifacts/boss-attack-sound/verification.md`.

The accepted Boss head hydraulic SFX unit maps the source
`Zombie::UpdateBoss` `PHASE_BOSS_HEAD_ENTER` `FOLEY_HYDRAULIC` boundary to
`BossHeadHydraulic` and `sounds/hydraulic.ogg`. The current fixed-step head
cycle emits this boundary immediately before the existing boulder-attack
event; source, resource probe, focused checks, and the Boss checkpoint trace
are recorded in ignored `artifacts/boss-head-hydraulic/verification.md`.

The accepted Boss projectile-start SFX unit maps the source
`Zombie::BossHeadSpitContact` `FOLEY_HYDRAULIC_SHORT` boundary to
`BossProjectileStarted` and `sounds/hydraulic_short.ogg`, one simulation tick
after the existing boulder-attack windup in the current fixed-step model.
Source, resource probe, focused Boss/App checks, and the existing Boss hidden
checkpoint playback trace are recorded in ignored
`artifacts/boss-projectile-start-audio/verification.md`.

The accepted Boss damage-explosion SFX unit maps the source
`Zombie::TakeDamage` one-tenth-health crossing and
`FOLEY_BOSS_EXPLOSION_SMALL` call to `BossDamageExplosion` and the existing
`sounds/explosion.ogg` resource. Source threshold evidence, resource probe,
focused core/App checks, and the hidden Boss damage checkpoint are recorded in
ignored `artifacts/boss-damage-explosion-audio/verification.md`.

The accepted Boss stomp SFX unit maps the source `Zombie::BossStompContact`
`FOLEY_THUMP` boundary to `BossStomp` and the existing
`sounds/gargantuar_thump.ogg` resource. The event is emitted only when the
source stomp rectangle contains a living plant; source, resource probe,
focused checks, and the hidden Boss stomp checkpoint trace are recorded in
ignored `artifacts/boss-stomp-audio/verification.md`.

The accepted Boss RV attack/landing SFX unit maps the source
`Zombie::BossRVAttack`/`BossRVLanding` boundaries to `BossRVStarted` and
`BossRVLanded`. The second damage tier selects the source two-row by three-column
target rectangle, emits the hydraulic-short resource at attack start, and emits
`sounds/RVthrow.ogg` at the 0.65-second landing boundary after the plant-death
events. Source timing, resource metadata, focused nextest, and hidden checkpoint
playback evidence are recorded in ignored `artifacts/boss-rv-audio/verification.md`.

The accepted pool-mower water SFX units map the source `LawnMower::UpdatePool`
entry and exit boundaries to `MowerEnteredPool` and `MowerExitedPool`. Entry
uses the existing `sounds/plant_water.ogg` /
`sounds/zombie_entering_water.ogg` Foley variants; exit uses
`sounds/plant_water.ogg`. Source boundaries, focused checks, and the pool
checkpoint playback trace are recorded in ignored
`artifacts/mower-water-audio/verification.md`.

The accepted night gravestone-rise SFX unit maps the source
`Zombie::RiseFromGrave` `FOLEY_GRAVESTONE_RUMBLE` boundary to
`ZombieGraveRumble` and `sounds/gravestone_rumble.ogg`. Pool, Whack-a-Zombie,
and Backup Dancer rise paths retain their source-specific mappings. Source,
resource probe, focused checks, and the hidden night checkpoint trace are
recorded in ignored `artifacts/gravestone-rumble-audio/verification.md`.

The accepted ladder-placement SFX unit maps the source
`Zombie::UpdateLadder` `SOUND_LADDER_ZOMBIE` call at the completed placement
boundary to `LadderPlaced` and `sounds/ladder_zombie.ogg`. Source boundary,
focused checks, and hidden checkpoint playback trace are recorded in ignored
`artifacts/ladder-placement-audio/verification.md`.

The accepted Plantern placement SFX unit maps the source
`Plant::PlantInitialize` `SOUND_PLANTERN` call for plant slot 25 ahead of the
existing `Board::DoPlantingEffects` generic planting Foley. The hidden
checkpoint preserves the source order as `sounds/plantern.ogg` followed by the
contextual planting resource; source, resource probe, focused checks, and
runtime playback are recorded in ignored
`artifacts/plantern-audio/verification.md`.

The accepted Garden leave UI SFX unit maps the source board main-menu button
`SOUND_GRAVEBUTTON` branch to the existing `GardenLeft` transition and
`sounds/gravebutton.ogg`. Source, resource probe, focused checks, and the
hidden Garden checkpoint playback trace are recorded in ignored
`artifacts/garden-leave-audio/verification.md`.

The accepted Aquarium empty-click UI SFX unit maps the source
`ZenGarden::MouseDownZenGarden` `SOUND_TAPGLASS` branch to `GardenTapGlass`
only for an Aquarium service with no plant hit, using `sounds/tapglass.au`.
Source, AU decode, focused checks, and the hidden Aquarium checkpoint playback
trace are recorded in ignored `artifacts/tapglass-audio/verification.md`.

The accepted Wall-nut Bowling placement SFX unit maps the source
`Board::MouseDown` `SOUND_BOWLING` branch to the existing `PlantPlaced` event
only for `WallnutBowling`, appending `sounds/bowling.ogg` after the normal
planting resource. Source, resource probe, focused checks, and the hidden
checkpoint playback trace are recorded in ignored
`artifacts/wallnut-bowling-audio/verification.md`.

The accepted Wall-nut Bowling impact SFX unit maps the source
`Plant::UpdateBowling` `FOLEY_BOWLINGIMPACT` boundary to `BowlingImpact` and
`sounds/bowlingimpact.ogg`; Explode-o-Nut retains the separate
`PlantSpecialTriggered`/`sounds/bowlingimpact2.ogg` companion. Source, event,
resource, focused nextest, and hidden checkpoint playback evidence are recorded
in ignored `artifacts/wallnut-bowling-impact-audio/verification.md`.

The accepted Wall-nut Bowling reward-group SFX unit maps the source
`Plant::UpdateBowling` reward branches' single `FOLEY_SPAWN_SUN` call to
`LootDropSound { SpawnSun }` before the group's `CoinProduced` events and
`sounds/throw.ogg`. Source order, focused nextest, resource metadata, and the
reward-enabled hidden checkpoint are recorded in ignored
`artifacts/wallnut-bowling-reward-audio/verification.md`.

The accepted standard Adventure opening SFX unit maps the source
`CutScene::Update` `SOUND_READYSETPLANT` boundary to `ReadySetPlant` on the
successful seed-confirmation transition or first ordinary board tick, using
`sounds/readysetplant.ogg`. The existing gates keep first-time levels 1-2 and
Adventure levels 5, 15, and 35 out of this event; source, resource probe,
focused checks, and the hidden opening checkpoint playback trace are recorded
in ignored
`artifacts/ready-set-plant-audio/verification.md`.

The accepted loot-drop SFX units map the source `Zombie::DropLoot`/
`Board::DropLootPiece` `FOLEY_SPAWN_SUN` call to `LootDropSound { SpawnSun }`
and `sounds/throw.ogg`; the first-run level-22/36 mode-unlock branch maps
`FOLEY_ART_CHALLENGE` to `LootDropSound { ArtChallenge }` and `sounds/diamond.au`.
The source order is preserved before the resulting `CoinProduced` event,
including Yeti and terminal-award drops. The audio backend decodes the target
8-bit mu-law Sun AU resource without changing the existing OGG path. Source,
resource hashes, focused checks, and both hidden checkpoint playback traces are
recorded in ignored `artifacts/loot-drop-audio/verification.md`.

The accepted level-award collection SFX units map the source `Coin::Collect`
branches to `AwardCollectionSound` and the existing `coin.ogg`, `diamond.au`,
`seedlift.ogg`, `tap2.ogg`, and `shovel.ogg` resources. `AwardPresent` and
`AwardChocolate` retain the source pre-collection prize path. Source ordering,
resource metadata, focused checks, and the first-run level-4 checkpoint trace
are recorded in ignored `artifacts/award-collection-audio/verification.md`.

The accepted weather SFX units map the source `Challenge::InitLevel`/
`UpdateStormyNight` `FOLEY_RAIN` and `FOLEY_THUNDER` boundaries to
`WeatherSound` and `sounds/rain.ogg`/`sounds/thunder.ogg`. Adventure level 40
preserves the source 400-update opening counter and 300/150 thunder boundaries;
source timing, resource metadata, focused checks, and the external checkpoint
trace are recorded in ignored `artifacts/weather-audio/verification.md`.

The accepted award-bag fan-out unit preserves the source money-bag award path:
collecting `AwardMoneyBag` creates five gold coins with the
`COIN_MOTION_FROM_PRESENT` drift and auto-collects them after 80 simulation
ticks. Same-version source evidence is `PvZ-Decomp-main/Lawn/Coin.cpp:998-1010,
471-480`; the deterministic core check and result are recorded in ignored
`artifacts/award-bag-fanout/verification.md`.

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
