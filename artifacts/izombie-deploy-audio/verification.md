# I, Zombie Deployment Audio Verification

Target: Plants vs. Zombies 1.0.0.1051 local source and external resource set.

## Source Cross-Check

- `PvZ-Decomp-main/Lawn/Challenge.cpp:4402-4413` calls `FOLEY_PLANT` only
  after an I, Zombie placement succeeds, spends sun, deploys the zombie, and
  starts the seed-packet refresh.
- `PvZ-Decomp-main/Sexy.TodLib/TodFoley.cpp:17,254-270` defines
  `FOLEY_PLANT` as a random choice between `SOUND_PLANT` and `SOUND_PLANT2`.
- `PvZ-Decomp-main/Sexy.TodLib/TodCommon.cpp:112-116` and
  `SexyAppFramework/MTRand.cpp:129-175` show that the choice consumes the
  shared, 31-bit MT19937 stream using the same modulo range rule as neopvz.
- `PvZ-Decomp-main/Debug/properties/resources.xml:439-440` maps those sounds
  to `plant` and `plant2`.

## Implementation Cross-Check

- Successful `PlantPlaced` and `ZombieDeployed` events carry a deterministic
  0/1 audio variant selected after entity construction.
- `crates/neopvz-app/src/main.rs` maps variant 0 to `sounds/plant.ogg` and
  variant 1 to `sounds/plant2.ogg` for both event types.
- Core tests retain the real successful plant/deployment input paths and assert
  that their emitted variants remain in the source range. The app mapping test
  covers both resources for both events.

## External Resource Check

The local debug app started with `--checkpoint zombie-deploy` and the external
1.0.0.1051 directory. Inventory verification reported 29 groups, 626 entries,
439 images, 20 fonts, 167 sounds, 250 compiled animations, and 2 music files.
The audio backend initialized without an unavailable-device or decode warning.

Decoded resource metadata:

| Resource | Vorbis stream | Source SHA-256 | Decoded-frame SHA-256 |
|---|---|---|---|
| `plant.ogg` | 44.1 kHz, mono, 0.754649 s | `1a2067fb16914b6bded36be4c014b3f1e097aaddf2a48803d3cd68319316e5f9` | `49305b48435f03b94208a7b8cfff32caeeac3fbc765f98c8d82aaabc65ff8474` |
| `plant2.ogg` | 44.1 kHz, stereo, 0.743039 s | `49afbb3413b2452797cfd57c4c92737baaf72d2376f2416b372f4bec6d3689b8` | `b95176370fb9110b8a103e1b0193b242c92d1cf96c3463bb12415a4ed6cd1a8a` |

Device-output timing remains a separate unresolved `AUD-SYNC` obligation.
