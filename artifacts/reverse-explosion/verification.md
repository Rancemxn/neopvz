# Reverse Explosion Verification

Target: Plants vs. Zombies 1.0.0.1051 local source/reference set.

## Source Cross-Check

- `PvZ-Decomp-main/Lawn/Plant.cpp:298-310` handles Cherry Bomb and Jalapeno
  together. When `IsInPlay()` is true it sets the special countdown and calls
  `PlayFoley(FOLEY_REVERSE_EXPLOSION)`.
- `PvZ-Decomp-main/Debug/properties/resources.xml:520` maps
  `REVERSE_EXPLOSION` to `reverse_explosion`.
- `PvZ-Decomp-main/Sexy.TodLib/TodFoley.cpp:67` maps that foley enum to the
  `SOUND_REVERSE_EXPLOSION` decoded output.

## Implementation Cross-Check

- `crates/neopvz-app/src/main.rs` maps the in-play `PlantPlaced` event for
  seed types 2 (Cherry Bomb) and 20 (Jalapeno) to
  `sounds/reverse_explosion.ogg` as an effect companion.
- `crates/neopvz-core/src/lib.rs` test
  `debug_explosion_checkpoint_preserves_placement_audio_and_triggers_specials`
  retains both placement events through the checkpoint before advancing the
  special sequence.

Runtime device playback, decoded PCM hashing, and capture comparison remain
separate unresolved audio-device obligations under Issue #19; this record does
not claim them.
