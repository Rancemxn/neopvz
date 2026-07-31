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

Current baseline: **1351 accepted / 1810 total; 459 unresolved**.

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
| ENT-PICKUP | Sun, coins, prizes, and pickup behavior | 15 | 26 | partial | SunPickupState, CoinPickupState, SunProduced/SunCollected/CoinProduced/CoinCollected events, CollectSun/CollectCoin input actions, source-like COIN_MOTION_COIN award arcs (launch, drift, gravity, item-award offsets, sunflower award elevation, landing), Raining Seeds usable-seed payload drops and free collection/planting, and money-bag fan-out into five from-present gold coins with 80-tick auto-collection are implemented; the source pickup catalog, money/sun variants, mode-unlock presents, garden prizes, chocolate, and remaining award-specific progression are covered by ignored local evidence and core tests; visual pickup particles unresolved |
| ENT-GRID | Graves, craters, portals, vases, and other grid items | 6 | 13 | partial | PRs `#50`, `#92`; runs `29979356134`/`29979356138`/`30148593526`/`30148593521`; graves (PR #50), craters (PR #50 DoomShroom crater with replant blocking), Vasebreaker vases (seeded layout, break/reveal, plant/zombie contents, no-vase rejection, and win condition), the per-row Zomboni ice trail (lay/melt/planting block, Jalapeno melt, spike-vehicle pop, Bobsled spawn dependency and end-of-ice crash), the single-use garden rake (first-zombie kill and consumption), placed ladders (Ladder-zombie placement, barrier bypass for later zombies, and Magnet-shroom removal, with focused core tests), and I Zombie lawn brains (placement, zombie brain-eating, and loss condition, evidenced by artifacts/izombie/verification.md) accepted; ignored `artifacts/vasebreaker/verification.md`, `artifacts/zomboni-ice-trail/verification.md`, and local source/runtime records; adventure level 4-5 Scary Potter (three-stage pot layouts from Challenge.cpp ScaryPotterPopulate, wave-clock suppression, stage advance on board clear, and the three-stage win) in ignored `artifacts/scary-potter-adventure/verification.md`; first-run adventure sod rows (1-1 row 2 only, 1-2/1-3 rows 1-3: planting, spawning, and mower gating) in ignored `artifacts/sod-rows/verification.md` |
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

## Player-Accessible Modes

| Obligation | Mode domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| MODE-ADVENTURE | Adventure levels | 50 | 50 | partial | Ignored `artifacts/adventure-levels/verification.md` and `artifacts/adventure-evidence/`; all 50 level identities (scene, wave totals, flag rules, conveyor membership, source wave composition via pick_adventure_waves, and runtime spawning with allow-list identity) have deterministic source checks; source pacing, scene-routed final-wave rises (graves/pool/sky), award/unlock identities, and setup/tutorial-gating identities implemented with deterministic checks; profile progression wiring, visual, and audio obligations remain; Issue `#18` |
| MODE-SURVIVAL | Visible normal, hard, and endless survival variants | 11 | 11 | partial | Ignored `artifacts/survival/verification.md`; all 11 stage scene, wave-count, endless-profile, and core repick identities have deterministic source checks; progression, visual, and audio remain; Issue `#18` |
| MODE-MINIGAME | Mini-game levels | 20 | 20 | partial | Ignored `artifacts/minigames/verification.md`; all catalog levels have deterministic source scene, wave, interaction, fixed seed-bank, or profile checks; conveyor checks are covered by ignored `artifacts/conveyor/verification.md`; progression, visual, and audio obligations remain; Issue `#18` |
| MODE-VASE | Vasebreaker levels, including endless | 10 | 10 | verified | Ignored `artifacts/vasebreaker/verification.md`; all 10 level identities have deterministic source-layout checks, with reveal, rejection, spawning, and win behavior covered by focused core evidence |
| MODE-IZOMBIE | I, Zombie levels, including endless | 10 | 10 | verified | Ignored `artifacts/izombie/verification.md`; source-defined layouts, fixed zombie seed banks, bank membership, deployment, brain eating, replay identity, and all 10 visible level identities have focused core evidence |
| MODE-GARDEN | Zen, mushroom, aquarium, and tree services | 4 | 4 | verified | Ignored `artifacts/garden-services/verification.md`; `garden_services_have_dedicated_state_and_inputs` covers all four service identities, service-specific input paths, and return to Adventure Select |

## Screens, Input, and Persistence

| Obligation | Behavior domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| UI-SCREEN | Loading, title, menu, selector, seed chooser, HUD, pause, options, help, almanac, shop, and result flows | 3 | 12 | partial | PR `#64`, runs `30030645194`/`30030645257` and `30048802223`/`30048802288`, ignored `artifacts/windows-7de9f1d/verification.md`/`artifacts/windows-beb00d3/verification.md`; selector/adventure tutorial route accepted; seed chooser remains partial; Issues `#2`, `#17` |
| INPUT-ACTION | Mouse, keyboard, hover, click, drag, placement, pause, restart, and command-line resource selection | 8 | 8 | verified | PR `#64`, commits `547f99b`/`514906c`, title mouse start, seed-chooser start/card selection and keyboard selection, left-click placement, Space pause/resume, and local terminal restart in ignored `artifacts/windows-a6c3f53/verification.md`/`artifacts/windows-aa443d7/verification.md`/`artifacts/windows-23f3f67/verification.md`/`artifacts/windows-514906c/verification.md`/`artifacts/windows-514906c/verification-keyboard.md`/`artifacts/local-restart/verification.md`; runs `30054043147`/`30054043130`; remaining pause-menu and visual semantics are tracked by Issues `#2`/`#17` |
| SAVE-PROGRESSION | Profile, settings, unlocks, awards, inventory, garden, mode completion, and load compatibility | 8 | 8 | verified | PR `#61`, runs `30024232209`/`30024232459`, core profile round-trip test, ignored `artifacts/profile-progression/verification.md` |
| PLATFORM-CONTRACT | Logical viewport, window/fullscreen behavior, DPI, audio device, and external-path behavior | 6 | 6 | verified | PR `#64`, 800x600 logical viewport, window startup, DPI-aware 1000x750 client capture, external `--data-dir` launch, and local debug `--fullscreen`/F11 round trip in ignored `artifacts/local-platform/verification.md`; the same artifact records the local app's graceful startup fallback when the audio backend reports an unavailable-device error. Remaining pause-menu, visual, and full audio obligations remain under Issues `#2`, `#14`, `#17`, and `#19` |

## Visual and Audio Evidence

| Obligation | Evidence domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| VIS-SCREEN | Declared visual checkpoints for player-accessible screens | 0 | 12 | missing | Issue `#14` |
| VIS-MODE | Declared visual checkpoints for every player-accessible mode unit | 0 | 105 | missing | Issue `#14` |
| VIS-PLANT | Plant animation, layering, clipping, and feedback review units | 0 | 49 | missing | Issue `#14` |
| VIS-ZOMBIE | Zombie animation, layering, clipping, and feedback review units | 0 | 33 | missing | Issue `#14` |
| VIS-PROJECTILE | Projectile animation and impact review units | 0 | 14 | missing | Issue `#14` |
| VIS-EFFECT | Effect and particle review units | 0 | 105 | missing | Issue `#14` |
| AUD-SFX | Simulation-tick and decoded-output sound-event units | 87 | 167 | partial | `SeedSelected` → `sounds/tap.ogg`, `InputRejected` → `sounds/buzzer.ogg`, `Paused` → `sounds/pause.ogg`, `ZombieDeployed` → `sounds/plant.ogg`/`sounds/plant2.ogg`, `PlantShoveled` → `sounds/plant2.ogg`, plant `SunProduced` -> `sounds/throw.ogg`, special-prize `CoinProduced` -> `sounds/chime.ogg`, Gold `CoinLanded` -> `sounds/moneyfalls.ogg`, `SunCollected` → `sounds/points.ogg`, Diamond `CoinCollected` -> `sounds/diamond.au`, usable-seed `PickupCollected` -> `sounds/seedlift.ogg`, sun `PickupCollected` -> `sounds/points.ogg`, prize `PickupCollected` -> `sounds/prize.ogg`, money `CoinCollected` → `sounds/coin.ogg`, `GardenWatered` → `sounds/watering.ogg` + companion `sounds/throw.ogg`, `GardenFertilized` → `sounds/fertilizer.ogg` + companion `sounds/throw.ogg`, `GardenBecameHappy` → `sounds/prize.ogg` + companion `sounds/throw.ogg`, IceShroom `PlantSpecialTriggered` → `sounds/frozen.ogg`, `ZombieChilled` → `sounds/frozen.ogg`, `CobCannonFired` → `sounds/coblaunch.ogg`, Catapult `ProjectileFired { Other(1) }` → `sounds/basketball.ogg`, Torchwood `ProjectileIgnited` -> `sounds/firepea.ogg`, `PortalOpened` → `sounds/portal.ogg`, GraveBuster `PlantSpecialTriggered` → `sounds/gravebusterchomp.ogg`, Coffee `PlantSpecialTriggered` → `sounds/coffee.ogg`, TangleKelp `TangleKelpGrabStarted` → `sounds/floop.ogg`, TangleKelp `TangleKelpWaterEntry` → `sounds/zombiesplash.ogg`, PotatoMine `PotatoMineArmed` → `sounds/dirt_rise.ogg`, Digger `DiggerSurfaced` → `sounds/dirt_rise.ogg` + companion `sounds/wakeup.ogg`, Magnet-shroom `MetalStolen` → `sounds/magnetshroom.ogg`, Zamboni `VehicleDisabled` → `sounds/balloon_pop.ogg`, PotatoMine `PlantSpecialTriggered` → `sounds/potato_mine.ogg`, Spikeweed `PlantSpecialTriggered` → `sounds/throw.ogg`, CherryBomb `PlantSpecialTriggered` → `sounds/cherrybomb.ogg`, ExplodeONut `PlantSpecialTriggered` → `sounds/cherrybomb.ogg` + companion `sounds/bowlingimpact2.ogg`, Jalapeno `PlantSpecialTriggered` → `sounds/jalapeno.ogg`, CherryBomb/Jalapeno companion → `sounds/juicy.ogg`, `ProjectileImpact` Butter → `sounds/butter.ogg`, `VaseBroken` → `sounds/vase_breaking.ogg`, `RakeTriggered` → `sounds/swing.ogg`, DoomShroom `PlantSpecialTriggered` → `sounds/doomshroom.ogg`, `BloverTriggered` → `sounds/blover.ogg`, Chomper `PlantSpecialTriggered` → `sounds/bigchomp.ogg`, Squash `PlantSpecialTriggered` -> `sounds/gargantuar_thump.ogg`, `SquashHumStarted` -> `sounds/squash_hmm.ogg`/`sounds/squash_hmm2.ogg`, `ZombieShieldHit` -> `sounds/shieldhit.ogg`/`sounds/shieldhit2.ogg`, `ZombieHypnotized` → `sounds/mindcontrolled.ogg`, `JackboxExploded` → `sounds/explosion.ogg`, day/night/roof `MowerTriggered { pool: false }` → `sounds/lawnmower.ogg`, pool `MowerTriggered { pool: true }` → `sounds/pool_cleaner.ogg`, `GameLost` → `sounds/losemusic.ogg`, `GameWon` → `sounds/winmusic.ogg`, `ZombieNewspaperRipped` → `sounds/newspaper_rip.ogg`, `ImpThrown` → `sounds/swing.ogg` + variant companion `sounds/imp.ogg`/`sounds/imp2.ogg`, `DolphinRider` appearance → `sounds/dolphin_appears.ogg`, `DolphinJumpStarted` → `sounds/dolphin_before_jumping.ogg` + companion `sounds/plant_water.ogg`, `ZombieEnteredPool` → `sounds/plant_water.ogg`/`sounds/zombie_entering_water.ogg`, `Zamboni` appearance → `sounds/zamboni.ogg`, `PogoBounceSound` → `sounds/pogo_zombie.ogg`, `PoleVaultGrassStep` → `sounds/grassstep.ogg`, `PoleVaultSound` → `sounds/polevault.ogg`, `Balloon` appearance → `sounds/ballooninflate.ogg`; source cross-checks, decoded PCM hashes, and local app tick/`playback started` traces in ignored `artifacts/local-audio/verification.md`; `PlantPlaced`/`ImitaterMorphed` terrain-aware planting Foley remains unresolved under Issue `#193`; `Resumed` and `ZombieDied` deliberately have no direct sound mapping; the accepted `ZombieDied` silent boundary and remaining SFX/device timing remain under Issue `#19` |
| AUD-MUSIC | Music playback, loop, and stem units | 1 | 2 | partial | Main/hihats MO3 loop duration and source track mapping verified in ignored `artifacts/music-loop/verification.md`; runtime playback/stem synchronization remains |
| AUD-SYNC | Event-to-device timing and music synchronization contract | 0 | 1 | missing | Issue `#19` |

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

The accepted Backup Dancer summon SFX unit maps the source
`Zombie::SummonBackupDancer` `FOLEY_GRAVESTONE_RUMBLE` call to
`ZombieSpawned { BackupDancer }` and `sounds/gravestone_rumble.ogg`; source,
event timing, mapping, external decode metadata, and checkpoint playback
evidence are recorded in ignored `artifacts/dancer-rumble/verification.md`.

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

Current baseline: **1369 accepted / 1810 total; 441 unresolved**.

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
| ENT-PICKUP | Sun, coins, prizes, and pickup behavior | 15 | 26 | partial | SunPickupState, CoinPickupState, SunProduced/SunCollected/CoinProduced/CoinCollected events, CollectSun/CollectCoin input actions, source-like COIN_MOTION_COIN award arcs (launch, drift, gravity, item-award offsets, sunflower award elevation, landing), Raining Seeds usable-seed payload drops and free collection/planting, and money-bag fan-out into five from-present gold coins with 80-tick auto-collection are implemented; the source pickup catalog, money/sun variants, mode-unlock presents, garden prizes, chocolate, and remaining award-specific progression are covered by ignored local evidence and core tests; visual pickup particles unresolved |
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

The Ice-shroom effect now follows the source `HitIceTrap` three-class contract:
`CanBeChilled` exclusions (Zamboni, sledded Bobsled team, hidden/rising Digger,
rising Backup Dancer, mind-controlled, and the Boss without an exposed-head
model) receive no chill, freeze, or 20 damage; `CanBeFrozen` exclusions
(vaulting Pole Vaulter, Dolphin entry/jump, Snorkel entry, flying Balloon,
thrown/landing Imp, SquashHead rise/fall, bouncing Pogo, and Bungee) receive
chill only; ordinary eligible zombies receive chill plus a source-range freeze
(300 in pool, 300..=400 when already cold, 400..=600 when fresh) plus 20
damage, with deterministic RNG consumption. Focused table-driven coverage is
`ice_shroom_applies_source_chill_freeze_and_damage_classes`; source evidence is
local `Zombie.cpp` `CanBeChilled` (`7983-8008`), `CanBeFrozen` (`8010-8032`),
and `HitIceTrap` (`8346-8382`); Issue `#186`.

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
two-of-a-kind and jackpot diamond/sun/usable-seed fan-outs, and reaches the
2,000-sun completion path. Focused coverage is
`slot_machine_rolls_three_reels_and_rejects_locked_or_empty_spins`,
`slot_machine_resolves_each_source_payout_class`,
`slot_machine_sun_jackpot_can_reach_the_source_completion_path`, and the legacy
state default check; source evidence is local `Challenge.cpp:1287-1298,1332-1335,2005-2048`
and `SeedPacket.cpp:33-82,162-180`; Issue `#245`.

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
| UI-SCREEN | Loading, title, menu, selector, seed chooser, HUD, pause, options, help, almanac, shop, and result flows | 3 | 12 | partial | PR `#64`, runs `30030645194`/`30030645257` and `30048802223`/`30048802288`, ignored `artifacts/windows-7de9f1d/verification.md`/`artifacts/windows-beb00d3/verification.md`; selector/adventure tutorial route accepted; seed chooser remains partial; Issues `#2`, `#17` |
| INPUT-ACTION | Mouse, keyboard, hover, click, drag, placement, pause, restart, and command-line resource selection | 8 | 8 | verified | PR `#64`, commits `547f99b`/`514906c`, title mouse start, seed-chooser start/card selection and keyboard selection, left-click placement, Space pause/resume, and local terminal restart in ignored `artifacts/windows-a6c3f53/verification.md`/`artifacts/windows-aa443d7/verification.md`/`artifacts/windows-23f3f67/verification.md`/`artifacts/windows-514906c/verification.md`/`artifacts/windows-514906c/verification-keyboard.md`/`artifacts/local-restart/verification.md`; runs `30054043147`/`30054043130`; remaining pause-menu and visual semantics are tracked by Issues `#2`/`#17` |
| SAVE-PROGRESSION | Profile, settings, unlocks, awards, inventory, garden, mode completion, and load compatibility | 8 | 8 | verified | PR `#61`, runs `30024232209`/`30024232459`, core profile round-trip test, ignored `artifacts/profile-progression/verification.md` |
| PLATFORM-CONTRACT | Logical viewport, window/fullscreen behavior, DPI, audio device, and external-path behavior | 6 | 6 | verified | PR `#64`, 800x600 logical viewport, window startup, DPI-aware 1000x750 client capture, external `--data-dir` launch, and local debug `--fullscreen`/F11 round trip in ignored `artifacts/local-platform/verification.md`; the same artifact records the local app's graceful startup fallback when the audio backend reports an unavailable-device error. Remaining pause-menu, visual, and full audio obligations remain under Issues `#2`, `#14`, `#17`, and `#19` |

## Visual and Audio Evidence

| Obligation | Evidence domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| VIS-SCREEN | Declared visual checkpoints for player-accessible screens | 0 | 12 | missing | Issue `#14` |
| VIS-MODE | Declared visual checkpoints for every player-accessible mode unit | 0 | 105 | missing | Issue `#14` |
| VIS-PLANT | Plant animation, layering, clipping, and feedback review units | 0 | 49 | missing | Issue `#14` |
| VIS-ZOMBIE | Zombie animation, layering, clipping, and feedback review units | 0 | 33 | missing | Issue `#14` |
| VIS-PROJECTILE | Projectile animation and impact review units | 0 | 14 | missing | Issue `#14` |
| VIS-EFFECT | Effect and particle review units | 0 | 105 | missing | Issue `#14` |
| AUD-SFX | Simulation-tick and decoded-output sound-event units | 98 | 167 | partial | `SeedSelected` → `sounds/tap.ogg`, `InputRejected` → `sounds/buzzer.ogg`, `Paused` → `sounds/pause.ogg`, `ZombieDeployed` → `sounds/plant.ogg`/`sounds/plant2.ogg`, `PlantShoveled` → `sounds/plant2.ogg`, plant `SunProduced` -> `sounds/throw.ogg`, special-prize `CoinProduced` -> `sounds/chime.ogg`, Gold `CoinLanded` -> `sounds/moneyfalls.ogg`, `SunCollected` → `sounds/points.ogg`, Diamond `CoinCollected` -> `sounds/diamond.au`, usable-seed `PickupCollected` -> `sounds/seedlift.ogg`, sun `PickupCollected` -> `sounds/points.ogg`, prize `PickupCollected` -> `sounds/prize.ogg`, money `CoinCollected` → `sounds/coin.ogg`, `GardenWatered` → `sounds/watering.ogg` + companion `sounds/throw.ogg`, `GardenFertilized` → `sounds/fertilizer.ogg` + companion `sounds/throw.ogg`, `GardenBecameHappy` → `sounds/prize.ogg` + companion `sounds/throw.ogg`, IceShroom `PlantSpecialTriggered` → `sounds/frozen.ogg`, `ZombieChilled` → `sounds/frozen.ogg`, `CobCannonFired` → `sounds/coblaunch.ogg`, Catapult `ProjectileFired { Other(1) }` → `sounds/basketball.ogg`, Torchwood `ProjectileIgnited` -> `sounds/firepea.ogg`, `PortalOpened` → `sounds/portal.ogg`, GraveBuster `PlantSpecialTriggered` → `sounds/gravebusterchomp.ogg`, Coffee `PlantSpecialTriggered` → `sounds/coffee.ogg`, TangleKelp `TangleKelpGrabStarted` → `sounds/floop.ogg`, TangleKelp `TangleKelpWaterEntry` → `sounds/zombiesplash.ogg`, PotatoMine `PotatoMineArmed` → `sounds/dirt_rise.ogg`, Digger `DiggerSurfaced` → `sounds/dirt_rise.ogg` + companion `sounds/wakeup.ogg`, Magnet-shroom `MetalStolen` → `sounds/magnetshroom.ogg`, Zamboni `VehicleDisabled` → `sounds/balloon_pop.ogg`, PotatoMine `PlantSpecialTriggered` → `sounds/potato_mine.ogg`, Spikeweed `PlantSpecialTriggered` → `sounds/throw.ogg`, CherryBomb `PlantSpecialTriggered` → `sounds/cherrybomb.ogg`, ExplodeONut `PlantSpecialTriggered` → `sounds/cherrybomb.ogg` + companion `sounds/bowlingimpact2.ogg`, Jalapeno `PlantSpecialTriggered` → `sounds/jalapeno.ogg`, CherryBomb/Jalapeno companion → `sounds/juicy.ogg`, `ProjectileImpact` Butter → `sounds/butter.ogg`, `VaseBroken` → `sounds/vase_breaking.ogg`, `RakeTriggered` → `sounds/swing.ogg`, DoomShroom `PlantSpecialTriggered` → `sounds/doomshroom.ogg`, `BloverTriggered` → `sounds/blover.ogg`, Chomper `PlantSpecialTriggered` → `sounds/bigchomp.ogg`, Squash `PlantSpecialTriggered` -> `sounds/gargantuar_thump.ogg`, `SquashHumStarted` -> `sounds/squash_hmm.ogg`/`sounds/squash_hmm2.ogg`, `ZombieShieldHit` -> `sounds/shieldhit.ogg`/`sounds/shieldhit2.ogg`, `ZombieHypnotized` → `sounds/mindcontrolled.ogg`, `JackboxExploded` → `sounds/explosion.ogg`, day/night/roof `MowerTriggered { pool: false }` → `sounds/lawnmower.ogg`, pool `MowerTriggered { pool: true }` → `sounds/pool_cleaner.ogg`, `GameLost` → `sounds/losemusic.ogg`, `GameWon` → `sounds/winmusic.ogg`, `ZombieNewspaperRipped` → `sounds/newspaper_rip.ogg`, `ImpThrown` → `sounds/swing.ogg` + variant companion `sounds/imp.ogg`/`sounds/imp2.ogg`, `DolphinRider` appearance → `sounds/dolphin_appears.ogg`, `DolphinJumpStarted` → `sounds/dolphin_before_jumping.ogg` + companion `sounds/plant_water.ogg`, `ZombieEnteredPool` → `sounds/plant_water.ogg`/`sounds/zombie_entering_water.ogg`, `Zamboni` appearance → `sounds/zamboni.ogg`, `PogoBounceSound` → `sounds/pogo_zombie.ogg`, `PoleVaultGrassStep` → `sounds/grassstep.ogg`, `PoleVaultSound` → `sounds/polevault.ogg`, `Balloon` appearance → `sounds/ballooninflate.ogg`, Jackbox/Digger `ZombieSongStarted` -> `sounds/jackinthebox.ogg`/`sounds/digger_zombie.ogg`, `ZombieGroaned` -> `sounds/lowgroan.ogg`/`sounds/lowgroan2.ogg`/`sounds/groan.ogg`/`sounds/groan2.ogg`, `ZombieChew` -> `sounds/chomp.ogg`/`sounds/chomp2.ogg`/`sounds/chompsoft.ogg`, Gargantuar `ZombieDeathSound` -> `sounds/gargantudeath.ogg`, Boss `ZombieDeathSound` -> `sounds/bossexplosion.ogg` + companion `sounds/gargantudeath.ogg`; source cross-checks, decoded PCM hashes, and local app tick/`playback started` traces in ignored `artifacts/local-audio/verification.md`; `PlantPlaced`/`ImitaterMorphed` terrain-aware planting Foley remains unresolved under Issue `#193`; `Resumed` and `ZombieDied` deliberately have no direct sound mapping; the accepted `ZombieDied` silent boundary and remaining SFX/device timing remain under Issue `#19` |
| AUD-MUSIC | Music playback, loop, and stem units | 1 | 2 | partial | Main/hihats MO3 loop duration and source track mapping verified in ignored `artifacts/music-loop/verification.md`; runtime playback/stem synchronization remains |
| AUD-SYNC | Event-to-device timing and music synchronization contract | 0 | 1 | missing | Issue `#19` |

The zombie song/groan/chew/death SFX slice maps `ZombieSongStarted`,
`ZombieGroaned`, `ZombieChew`, and `ZombieDeathSound` to the target
`jackinthebox`, `digger_zombie`, `lowgroan`/`lowgroan2`, `groan`/`groan2`,
`chomp`/`chomp2`/`chompsoft`, `gargantudeath`, and `bossexplosion` resource
families. Source boundaries are local `Zombie.cpp:4724-4777,7404-7412,8953-8957,10205-10214`;
the source-to-resource mapping, decoded PCM hashes, and focused core/App tests
are recorded in ignored `artifacts/audio-slice/verification.md`.

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

The accepted Backup Dancer summon SFX unit maps the source
`Zombie::SummonBackupDancer` `FOLEY_GRAVESTONE_RUMBLE` call to
`ZombieSpawned { BackupDancer }` and `sounds/gravestone_rumble.ogg`; source,
event timing, mapping, external decode metadata, and checkpoint playback
evidence are recorded in ignored `artifacts/dancer-rumble/verification.md`.

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

Current baseline: **1369 accepted / 1810 total; 441 unresolved**.

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
| ENT-PICKUP | Sun, coins, prizes, and pickup behavior | 15 | 26 | partial | SunPickupState, CoinPickupState, SunProduced/SunCollected/CoinProduced/CoinCollected events, CollectSun/CollectCoin input actions, source-like COIN_MOTION_COIN award arcs (launch, drift, gravity, item-award offsets, sunflower award elevation, landing), Raining Seeds usable-seed payload drops and free collection/planting, and money-bag fan-out into five from-present gold coins with 80-tick auto-collection are implemented; the source pickup catalog, money/sun variants, mode-unlock presents, garden prizes, chocolate, and remaining award-specific progression are covered by ignored local evidence and core tests; visual pickup particles unresolved |
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

CobCannon now defers its Cob projectile to the source launch boundary: the fire
input stores the target and the 206-tick firing counter, and the projectile is
created and emitted only when the counter reaches 1, matching `CobCannonFire`
and `UpdateShooting` (`Plant.cpp:4502-4516`, `3234-3340`). Reload arming stays
separate from the firing animation. Focused coverage is
`cob_cannon_defers_launch_until_the_firing_counter_elapses`, and the existing
launch-and-impact test now keeps targets stationary through the firing
animation; Issue `#199`.

The Ice-shroom effect now follows the source `HitIceTrap` three-class contract:
`CanBeChilled` exclusions (Zamboni, sledded Bobsled team, hidden/rising Digger,
rising Backup Dancer, mind-controlled, and the Boss without an exposed-head
model) receive no chill, freeze, or 20 damage; `CanBeFrozen` exclusions
(vaulting Pole Vaulter, Dolphin entry/jump, Snorkel entry, flying Balloon,
thrown/landing Imp, SquashHead rise/fall, bouncing Pogo, and Bungee) receive
chill only; ordinary eligible zombies receive chill plus a source-range freeze
(300 in pool, 300..=400 when already cold, 400..=600 when fresh) plus 20
damage, with deterministic RNG consumption. Focused table-driven coverage is
`ice_shroom_applies_source_chill_freeze_and_damage_classes`; source evidence is
local `Zombie.cpp` `CanBeChilled` (`7983-8008`), `CanBeFrozen` (`8010-8032`),
and `HitIceTrap` (`8346-8382`); Issue `#186`.

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
two-of-a-kind and jackpot diamond/sun/usable-seed fan-outs, and reaches the
2,000-sun completion path. Focused coverage is
`slot_machine_rolls_three_reels_and_rejects_locked_or_empty_spins`,
`slot_machine_resolves_each_source_payout_class`,
`slot_machine_sun_jackpot_can_reach_the_source_completion_path`, and the legacy
state default check; source evidence is local `Challenge.cpp:1287-1298,1332-1335,2005-2048`
and `SeedPacket.cpp:33-82,162-180`; Issue `#245`.

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
| UI-SCREEN | Loading, title, menu, selector, seed chooser, HUD, pause, options, help, almanac, shop, and result flows | 3 | 12 | partial | PR `#64`, runs `30030645194`/`30030645257` and `30048802223`/`30048802288`, ignored `artifacts/windows-7de9f1d/verification.md`/`artifacts/windows-beb00d3/verification.md`; selector/adventure tutorial route accepted; seed chooser remains partial; Issues `#2`, `#17` |
| INPUT-ACTION | Mouse, keyboard, hover, click, drag, placement, pause, restart, and command-line resource selection | 8 | 8 | verified | PR `#64`, commits `547f99b`/`514906c`, title mouse start, seed-chooser start/card selection and keyboard selection, left-click placement, Space pause/resume, and local terminal restart in ignored `artifacts/windows-a6c3f53/verification.md`/`artifacts/windows-aa443d7/verification.md`/`artifacts/windows-23f3f67/verification.md`/`artifacts/windows-514906c/verification.md`/`artifacts/windows-514906c/verification-keyboard.md`/`artifacts/local-restart/verification.md`; runs `30054043147`/`30054043130`; remaining pause-menu and visual semantics are tracked by Issues `#2`/`#17` |
| SAVE-PROGRESSION | Profile, settings, unlocks, awards, inventory, garden, mode completion, and load compatibility | 8 | 8 | verified | PR `#61`, runs `30024232209`/`30024232459`, core profile round-trip test, ignored `artifacts/profile-progression/verification.md` |
| PLATFORM-CONTRACT | Logical viewport, window/fullscreen behavior, DPI, audio device, and external-path behavior | 6 | 6 | verified | PR `#64`, 800x600 logical viewport, window startup, DPI-aware 1000x750 client capture, external `--data-dir` launch, and local debug `--fullscreen`/F11 round trip in ignored `artifacts/local-platform/verification.md`; the same artifact records the local app's graceful startup fallback when the audio backend reports an unavailable-device error. Remaining pause-menu, visual, and full audio obligations remain under Issues `#2`, `#14`, `#17`, and `#19` |

## Visual and Audio Evidence

| Obligation | Evidence domain | Accepted | Total | Status | Evidence / owner |
|---|---|---:|---:|---|---|
| VIS-SCREEN | Declared visual checkpoints for player-accessible screens | 0 | 12 | missing | Issue `#14` |
| VIS-MODE | Declared visual checkpoints for every player-accessible mode unit | 0 | 105 | missing | Issue `#14` |
| VIS-PLANT | Plant animation, layering, clipping, and feedback review units | 0 | 49 | missing | Issue `#14` |
| VIS-ZOMBIE | Zombie animation, layering, clipping, and feedback review units | 0 | 33 | missing | Issue `#14` |
| VIS-PROJECTILE | Projectile animation and impact review units | 0 | 14 | missing | Issue `#14` |
| VIS-EFFECT | Effect and particle review units | 0 | 105 | missing | Issue `#14` |
| AUD-SFX | Simulation-tick and decoded-output sound-event units | 98 | 167 | partial | `SeedSelected` → `sounds/tap.ogg`, `InputRejected` → `sounds/buzzer.ogg`, `Paused` → `sounds/pause.ogg`, `ZombieDeployed` → `sounds/plant.ogg`/`sounds/plant2.ogg`, `PlantShoveled` → `sounds/plant2.ogg`, plant `SunProduced` -> `sounds/throw.ogg`, special-prize `CoinProduced` -> `sounds/chime.ogg`, Gold `CoinLanded` -> `sounds/moneyfalls.ogg`, `SunCollected` → `sounds/points.ogg`, Diamond `CoinCollected` -> `sounds/diamond.au`, usable-seed `PickupCollected` -> `sounds/seedlift.ogg`, sun `PickupCollected` -> `sounds/points.ogg`, prize `PickupCollected` -> `sounds/prize.ogg`, money `CoinCollected` → `sounds/coin.ogg`, `GardenWatered` → `sounds/watering.ogg` + companion `sounds/throw.ogg`, `GardenFertilized` → `sounds/fertilizer.ogg` + companion `sounds/throw.ogg`, `GardenBecameHappy` → `sounds/prize.ogg` + companion `sounds/throw.ogg`, IceShroom `PlantSpecialTriggered` → `sounds/frozen.ogg`, `ZombieChilled` → `sounds/frozen.ogg`, `CobCannonFired` → `sounds/coblaunch.ogg`, Catapult `ProjectileFired { Other(1) }` → `sounds/basketball.ogg`, Torchwood `ProjectileIgnited` -> `sounds/firepea.ogg`, `PortalOpened` → `sounds/portal.ogg`, GraveBuster `PlantSpecialTriggered` → `sounds/gravebusterchomp.ogg`, Coffee `PlantSpecialTriggered` → `sounds/coffee.ogg`, TangleKelp `TangleKelpGrabStarted` → `sounds/floop.ogg`, TangleKelp `TangleKelpWaterEntry` → `sounds/zombiesplash.ogg`, PotatoMine `PotatoMineArmed` → `sounds/dirt_rise.ogg`, Digger `DiggerSurfaced` → `sounds/dirt_rise.ogg` + companion `sounds/wakeup.ogg`, Magnet-shroom `MetalStolen` → `sounds/magnetshroom.ogg`, Zamboni `VehicleDisabled` → `sounds/balloon_pop.ogg`, PotatoMine `PlantSpecialTriggered` → `sounds/potato_mine.ogg`, Spikeweed `PlantSpecialTriggered` → `sounds/throw.ogg`, CherryBomb `PlantSpecialTriggered` → `sounds/cherrybomb.ogg`, ExplodeONut `PlantSpecialTriggered` → `sounds/cherrybomb.ogg` + companion `sounds/bowlingimpact2.ogg`, Jalapeno `PlantSpecialTriggered` → `sounds/jalapeno.ogg`, CherryBomb/Jalapeno companion → `sounds/juicy.ogg`, `ProjectileImpact` Butter → `sounds/butter.ogg`, `VaseBroken` → `sounds/vase_breaking.ogg`, `RakeTriggered` → `sounds/swing.ogg`, DoomShroom `PlantSpecialTriggered` → `sounds/doomshroom.ogg`, `BloverTriggered` → `sounds/blover.ogg`, Chomper `PlantSpecialTriggered` → `sounds/bigchomp.ogg`, Squash `PlantSpecialTriggered` -> `sounds/gargantuar_thump.ogg`, `SquashHumStarted` -> `sounds/squash_hmm.ogg`/`sounds/squash_hmm2.ogg`, `ZombieShieldHit` -> `sounds/shieldhit.ogg`/`sounds/shieldhit2.ogg`, `ZombieHypnotized` → `sounds/mindcontrolled.ogg`, `JackboxExploded` → `sounds/explosion.ogg`, day/night/roof `MowerTriggered { pool: false }` → `sounds/lawnmower.ogg`, pool `MowerTriggered { pool: true }` → `sounds/pool_cleaner.ogg`, `GameLost` → `sounds/losemusic.ogg`, `GameWon` → `sounds/winmusic.ogg`, `ZombieNewspaperRipped` → `sounds/newspaper_rip.ogg`, `ImpThrown` → `sounds/swing.ogg` + variant companion `sounds/imp.ogg`/`sounds/imp2.ogg`, `DolphinRider` appearance → `sounds/dolphin_appears.ogg`, `DolphinJumpStarted` → `sounds/dolphin_before_jumping.ogg` + companion `sounds/plant_water.ogg`, `ZombieEnteredPool` → `sounds/plant_water.ogg`/`sounds/zombie_entering_water.ogg`, `Zamboni` appearance → `sounds/zamboni.ogg`, `PogoBounceSound` → `sounds/pogo_zombie.ogg`, `PoleVaultGrassStep` → `sounds/grassstep.ogg`, `PoleVaultSound` → `sounds/polevault.ogg`, `Balloon` appearance → `sounds/ballooninflate.ogg`, Jackbox/Digger `ZombieSongStarted` -> `sounds/jackinthebox.ogg`/`sounds/digger_zombie.ogg`, `ZombieGroaned` -> `sounds/lowgroan.ogg`/`sounds/lowgroan2.ogg`/`sounds/groan.ogg`/`sounds/groan2.ogg`, `ZombieChew` -> `sounds/chomp.ogg`/`sounds/chomp2.ogg`/`sounds/chompsoft.ogg`, Gargantuar `ZombieDeathSound` -> `sounds/gargantudeath.ogg`, Boss `ZombieDeathSound` -> `sounds/bossexplosion.ogg` + companion `sounds/gargantudeath.ogg`; source cross-checks, decoded PCM hashes, and local app tick/`playback started` traces in ignored `artifacts/local-audio/verification.md`; `PlantPlaced`/`ImitaterMorphed` terrain-aware planting Foley remains unresolved under Issue `#193`; `Resumed` and `ZombieDied` deliberately have no direct sound mapping; the accepted `ZombieDied` silent boundary and remaining SFX/device timing remain under Issue `#19` |
| AUD-MUSIC | Music playback, loop, and stem units | 1 | 2 | partial | Main/hihats MO3 loop duration and source track mapping verified in ignored `artifacts/music-loop/verification.md`; runtime playback/stem synchronization remains |
| AUD-SYNC | Event-to-device timing and music synchronization contract | 0 | 1 | missing | Issue `#19` |

The zombie song/groan/chew/death SFX slice maps `ZombieSongStarted`,
`ZombieGroaned`, `ZombieChew`, and `ZombieDeathSound` to the target
`jackinthebox`, `digger_zombie`, `lowgroan`/`lowgroan2`, `groan`/`groan2`,
`chomp`/`chomp2`/`chompsoft`, `gargantudeath`, and `bossexplosion` resource
families. Source boundaries are local `Zombie.cpp:4724-4777,7404-7412,8953-8957,10205-10214`;
the source-to-resource mapping, decoded PCM hashes, and focused core/App tests
are recorded in ignored `artifacts/audio-slice/verification.md`.

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

The accepted Backup Dancer summon SFX unit maps the source
`Zombie::SummonBackupDancer` `FOLEY_GRAVESTONE_RUMBLE` call to
`ZombieSpawned { BackupDancer }` and `sounds/gravestone_rumble.ogg`; source,
event timing, mapping, external decode metadata, and checkpoint playback
evidence are recorded in ignored `artifacts/dancer-rumble/verification.md`.

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
