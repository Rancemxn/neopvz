use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

mod save;

pub use save::{
    GardenPlant, GardenState, ModeCompletion, ModeKind, SAVE_FORMAT_VERSION, SaveError,
    SaveInventory, SaveProfile, SaveSettings,
};

pub const LOGICAL_WIDTH: u32 = 800;
pub const LOGICAL_HEIGHT: u32 = 600;
pub const SIMULATION_HZ: u32 = 100;
pub const GRID_COLUMNS: u8 = 9;
pub const DAY_ROWS: u8 = 5;
pub const POOL_ROWS: u8 = 6;
pub const REPLAY_FORMAT_VERSION: u32 = 1;

const POSITION_SCALE: i64 = 1_000_000;
const FIRST_WAVE_COUNTDOWN: u32 = 1_800;
const SUN_COUNTDOWN: u32 = 425;
const SUN_COUNTDOWN_RANGE: u32 = 275;
const SUN_COUNTDOWN_MAX: u32 = 950;
const MAX_SUN: u32 = 9_990;
const MAX_SEED_SLOTS: u8 = 53;
const CONVEYOR_SEED_SLOTS: usize = 10;
const SUNSHROOM_GROWTH_TICKS: u32 = 12_000;
const NORMAL_SUN_VALUE: u32 = 25;
const SMALL_SUN_VALUE: u32 = 15;
const LARGE_SUN_VALUE: u32 = 50;
const INSTANT_PLANT_COUNTDOWN: u32 = 100;
const BLOVER_SPECIAL_COUNTDOWN: u32 = 50;
const COFFEE_WAKE_TICKS: u32 = 100;
const GRAVEBUSTER_EAT_TICKS: u32 = 400;
const POTATO_ARM_TICKS: u32 = 1_500;
const IMITATER_MORPH_TICKS: u32 = 200;
const COB_ARM_TICKS: u32 = 500;
const COB_RELOAD_TICKS: u32 = 206;
const PLANT_SPECIAL_DAMAGE: i32 = 1_800;
const SQUASH_LOOK_TICKS: u32 = 80;
const SQUASH_JUMP_UP_TICKS: u32 = 45;
const SQUASH_AIR_TICKS: u32 = 50;
const SQUASH_LANDING_HIT_TICKS: u32 = 5;
const SQUASH_OFF_GROUND_TICKS: u32 = SQUASH_AIR_TICKS + SQUASH_LANDING_HIT_TICKS;
const SQUASH_DONE_FALLING_TICKS: u32 = 100;
const SQUASH_HIT_DELAY_TICKS: u32 =
    SQUASH_JUMP_UP_TICKS + SQUASH_AIR_TICKS + SQUASH_LANDING_HIT_TICKS;
const SQUASH_TARGET_GAP: i64 = 70;
const SQUASH_EATING_TARGET_GAP: i64 = 110;
const ICE_SHROOM_INITIAL_FREEZE_TICKS: u32 = 400;
const ICE_SHROOM_REFRESH_FREEZE_TICKS: u32 = 300;
const ICE_SHROOM_CHILL_TICKS: u32 = 2_000;
const ICE_SHROOM_DAMAGE: i32 = 20;
const BOARD_ICE_TICKS: u32 = 300;
const DOOM_SHROOM_RADIUS: i64 = 250;
const DOOM_SHROOM_ROW_RADIUS: u8 = 3;
const DOOM_CRATER_TICKS: u32 = 18_000;
// Zombie_EatPlant in the target build subtracts four health per ordinary bite.
const ZOMBIE_BITE_DAMAGE: i32 = 4;
const CHOMPER_BITE_WINDUP_TICKS: u32 = 70;
const CHOMPER_CHEW_TICKS: u32 = 4_000;
// Zombie_UpdateAteGarlic in 1.0.0.1051 consumes Garlic at 70, changes row at
// 170, and clears its eating state at 270 updates.
const GARLIC_EAT_TICKS: u32 = 70;
const GARLIC_ROW_CHANGE_TICKS: u32 = 170;
const GARLIC_RESET_TICKS: u32 = 270;
const MOWER_TRIGGER_X: i64 = 0;
const MOWER_SPEED: i64 = 8 * POSITION_SCALE;
// Plant_UpdateTanglekelp starts its grab state with a 100-tick countdown.
const TANGLE_KELP_GRAB_TICKS: u32 = 100;
// Plant_UpdateSpike / Plant_SpikesSetAnimAttack in 1.0.0.1051: attack
// state lasts 100 ticks and deals 20 damage when the countdown reaches 75.
const SPIKEWEED_ATTACK_TICKS: u32 = 100;
const SPIKEWEED_DAMAGE_COUNTDOWN: u32 = 75;
const SPIKEWEED_DAMAGE: i32 = 20;
// Projectile::CheckForCollision MOTION_PUFF: die when mProjectileAge >= 75.
const PUFF_PROJECTILE_MAX_AGE: u32 = 75;
// Plant::GetPlantAttackRect SEED_PUFFSHROOM/SEASHROOM width 230 from mX+60.
const PUFF_ATTACK_RANGE: i64 = 230;
// Plant::GetPlantAttackRect SEED_FUMESHROOM width 340 from mX+60.
const FUME_ATTACK_RANGE: i64 = 340;
// Plant::GetPlantAttackRect SEED_GLOOMSHROOM returns a 240x240 area.
const GLOOM_ATTACK_RANGE: i64 = 240;
const GLOOM_ROW_RADIUS: u8 = 1;
// ScaredyShroom's same-version threat check uses a 120-unit radius.
const SCAREDY_THREAT_RADIUS: i64 = 120;
// GoldMagnet recharges for a random 200-300 updates after a suck.
const GOLD_MAGNET_RECHARGE_MIN: u32 = 200;
const GOLD_MAGNET_RECHARGE_MAX: u32 = 300;
// Zombie::UpdateYeti in 1.0.0.1051 flees after a 1500-2000 tick phase.
const YETI_HEALTH: i32 = 1_350;
const YETI_FLEE_MIN_TICKS: u32 = 1_500;
const YETI_FLEE_MAX_TICKS: u32 = 2_000;
const YETI_WALK_SPEED: i64 = 400_000;
const YETI_RUNNING_SPEED: i64 = 800_000;
const YETI_FLEE_EDGE: i64 = 850 * POSITION_SCALE;
const YETI_DIAMOND_COUNT: usize = 4;
const I_ZOMBIE_BRAIN_TICKS: u32 = 70;
const ZOMBIE_PEA_HEAD_RELOAD_TICKS: u32 = 150;
const POGO_BOUNCE_TICKS: u32 = 80;
const GARGANTUAR_SPIKEROCK_DAMAGE: i32 = 20;
// Source throw geometry: distance = posX - 360 (roof -180, floor -140; floor 40
// elsewhere; > 140 loses a random 0-100), imp leaves at posX - 133 with
// velZ = 0.5 * (distance / 3) * 0.05 from altitude 88, integrating
// velZ -= 0.05 / altitude += velZ / posX -= 3 per tick.
const GARGANTUAR_THROW_BASE_X: i64 = 360 * POSITION_SCALE;
const GARGANTUAR_THROW_MIN_DISTANCE: i64 = 40 * POSITION_SCALE;
// anim_throw spans 34 frames at 24 fps. Count half-speed animation steps so
// normal/chilled updates advance by 2/1 and freeze can pause the track.
const GARGANTUAR_THROW_EVENT_STEPS: u32 = 210;
const GARGANTUAR_THROW_RECOVERY_STEPS: u32 = 74;
const IMP_THROW_SPAWN_OFFSET: i64 = 133 * POSITION_SCALE;
const IMP_THROW_SPEED_X: i64 = 3 * POSITION_SCALE;
const IMP_THROW_START_ALTITUDE: i64 = 88 * POSITION_SCALE;
const THROWN_ZOMBIE_GRAVITY: i64 = POSITION_SCALE / 20;
// Zomboni ice trail: per-row Board state in the source (mIceMinX/mIceTimer),
// laid at posX + 118 (clamped to 25, roof 500), refreshed to 3000 ticks while
// the front is left of 800; Jalapeno sets the row timer to 20. Bobsleds only
// spawn on iced rows, keep their row's timer at >= 500, and the leader takes
// 6 damage per tick past the ice end until the 300 HP sled breaks.
// Board::UpdateZombieSpawning: waves after the first arm at 2500 + Rand(600).
// Zombie::ApplyButter 0x5326D0 immobilizes for 400 ticks.
const BUTTER_TICKS: u32 = 400;
// Coin.cpp:293-294 COIN_MOTION_FROM_SKY: sky suns fall at 0.67 per tick;
// plant suns launch upward and fall under the 0.09 gravity (Coin.cpp:487).
const SUN_FALL_SPEED: i64 = 670_000;
const SUN_GRAVITY: i64 = 90_000;
// Coin.cpp:491: dropped coins fall under the heavier 0.15 gravity.
const COIN_GRAVITY: i64 = 150_000;
const ZOMBIE_NEXT_WAVE_COUNTDOWN: u32 = 2_500;
const ZOMBIE_NEXT_WAVE_RANGE: u32 = 600;
const ICE_START_X: i64 = 800 * POSITION_SCALE;
const ICE_LAY_OFFSET: i64 = 118 * POSITION_SCALE;
const ICE_LAY_MIN_X: i64 = 25 * POSITION_SCALE;
const ICE_LAY_MIN_X_ROOF: i64 = 500 * POSITION_SCALE;
const ICE_TIMER_TICKS: u32 = 3_000;
const JALAPENO_ICE_MELT_TICKS: u32 = 20;
const SPIKE_VEHICLE_DAMAGE: i32 = 1_800;
const SPIKEROCK_VEHICLE_SELF_DAMAGE: i32 = 50;
const BOBSLED_ICE_KEEPALIVE_TICKS: u32 = 500;
const BOBSLED_ICE_END_DAMAGE: i32 = 6;
// Plant_UpdateMagnetshoom: one item per 1500-tick cycle; 270-unit reach (320
// against an eating victim), two-row window with an 80-per-row tie-break.
// A robbed tunneling Digger pauses 200 ticks and rises for 130 more.
const MAGNET_RECHARGE_TICKS: u32 = 1_500;
const MAGNET_STEAL_RADIUS: i64 = 270 * POSITION_SCALE;
const MAGNET_STEAL_EATING_RADIUS: i64 = 320 * POSITION_SCALE;
const DIGGER_AXE_LOSS_SURFACE_TICKS: u32 = 330;
// Bungee delivery (Zombie_AirborneInit / Zombie_BungeeFallingUpdate): the
// carrier starts at altitude 3000 + Rand(150) and dives 8 per tick, releases
// its held zombie on landing, rises 8 per tick, and departs at altitude 600.
// The roof final-wave sky drop fires 210 ticks after the wave spawns with
// Normal/Conehead/Buckethead weighted 4000/4000/3000 over columns 4-8, rows 0-4.
const BUNGEE_DROP_DIVE_ALTITUDE: i64 = 3_000;
const BUNGEE_DROP_SPEED: i64 = 8;
const BUNGEE_RISE_DEPART_TICKS: u32 = 75;
const SKY_DROP_DELAY_TICKS: u32 = 210;
const DANCER_SUMMON_TICKS: u32 = 300;
const BACKUP_DANCER_COUNT: usize = 4;
const DIGGER_RISE_TICKS: u32 = 130;
const BUNGEE_STEAL_TICKS: u32 = 300;
const CATAPULT_LAUNCH_TICKS: u32 = 150;
const CATAPULT_RELOAD_TICKS: u32 = 300;
const CATAPULT_SHOTS: u8 = 20;
const BOBSLED_HEALTH: i32 = 270;
const BOBSLED_HELM_HEALTH: i32 = 300;
const BOBSLED_SPEED: i64 = 600_000;
const BOBSLED_SLIDE_TICKS: u32 = 500;
const LADDER_HEALTH: i32 = 500;
const LADDER_SHIELD_HEALTH: i32 = 500;
const BOSS_ADVENTURE_HEALTH: i32 = 40_000;
const BOSS_CHALLENGE_HEALTH: i32 = 60_000;
const BOSS_ATTACK_TICKS: u32 = 500;
/// Zombie.cpp:9765 boss init and :9980 BossHeadAttack re-arm, plus the
/// 500-tick head idle before the spit (Zombie.cpp:10401).
const BOSS_HEAD_COUNTER_INITIAL: u32 = 5_000;
const BOSS_HEAD_SPIT_DELAY: u32 = 500;
/// Zombie.cpp:10118 BossHeadSpitContact and :10150 UpdateBossFireball.
const BOSS_BALL_START_X: i64 = 455 * POSITION_SCALE;
const BOSS_BALL_END_X: i64 = -180 * POSITION_SCALE;
const BOSS_BALL_MOWER_REACH: i64 = 50 * POSITION_SCALE;
/// The source speed rides the BOSS_FIREBALL reanim ground track; modeled
/// at half a pixel per tick pending capture evidence.
const BOSS_BALL_SPEED: i64 = POSITION_SCALE / 2;
const ZOMBOTANY_HEAD_RELOAD_TICKS: u32 = 150;
const ZOMBOTANY_WALLNUT_HELM_HEALTH: i32 = 1_100;
const ZOMBOTANY_TALLNUT_HELM_HEALTH: i32 = 2_200;
const ZOMBOTANY_JALAPENO_HEALTH: i32 = 500;
const ZOMBOTANY_SQUASH_RISE_TICKS: u32 = 95;
const ZOMBOTANY_SQUASH_FALL_TICKS: u32 = 10;
const ZOMBOTANY_SQUASH_DONE_TICKS: u32 = 100;
const ZOMBOTANY_SQUASH_DAMAGE: i32 = 1_800;
const GIGAGARGANTUAR_HEALTH: i32 = 6_000;
// Zombie_Init in 1.0.0.1051 gives Jack-in-the-Box a 500-HP body, and
// Zombie_UpdateJack pops it 110 updates after its run phase ends; on Scary
// Potter levels the run phase counter is forced to 10.
const JACKBOX_HEALTH: i32 = 500;
const VASE_JACKBOX_POP_TICKS: u32 = 120;
// KillAllPlantsInRadius uses JackInTheBoxPlantRadius (90); zombies use 115.
const JACKBOX_PLANT_RADIUS: i64 = 90;
// Zombie_Init in 1.0.0.1051 gives Newspaper the 270-HP body plus a 150-HP
// paper shield (420 lumped); Zombie_ResetSpeed runs the mad phase at 0.89-0.91.
const NEWSPAPER_PAPER_HEALTH: i32 = 150;
const NEWSPAPER_MAD_SPEED: i64 = 900_000;
const SCREEN_DOOR_SHIELD_HEALTH: i32 = 1_100;
// Zombie_ResetSpeed in 1.0.0.1051: PHASE_DIGGER_TUNNELING moves at 0.66-0.68;
// the surfaced PHASE_DIGGER_WALKING re-pick is 0.12, or 0.23 on I, Zombie
// levels.
const DIGGER_WALK_SPEED: i64 = 120_000;
const DIGGER_IZOMBIE_WALK_SPEED: i64 = 230_000;
// Zombie_UpdateDolphin in the target build uses a 120-tick jump and the
// source's 0.9/0.5/0.3 walk-speed phases.
const DOLPHIN_JUMP_TIME: u32 = 120;
const DOLPHIN_WALK_SPEED: i64 = 900_000;
const DOLPHIN_RIDE_SPEED: i64 = 500_000;
const DOLPHIN_POOL_SPEED: i64 = 300_000;
const DOLPHIN_JUMP_TARGET_OFFSET: i64 = 94 * POSITION_SCALE;
const SNORKEL_SPEED: i64 = 670_000;
const ZAMBONI_HEALTH: i32 = 1_350;
const BALLOON_FLYING_HEALTH: i32 = 20;
const BALLOON_FLYING_PHASE: u8 = 1;
const BALLOON_POPPING_PHASE: u8 = 2;
const BALLOON_WALKING_PHASE: u8 = 3;
const BALLOON_POP_TICKS: u32 = 25;
const BLOWN_AWAY_SPEED: i64 = 10 * POSITION_SCALE;
const BLOWN_AWAY_EDGE: i64 = 850 * POSITION_SCALE;

pub type Tick = u64;
pub type EntityId = u32;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("state serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("unsupported replay format {found}; expected {expected}")]
    ReplayFormat { expected: u32, found: u32 },
    #[error("replay build identity {found:?} does not match {expected:?}")]
    BuildIdentity { expected: String, found: String },
    #[error("replay record outcome does not match a fresh run")]
    ReplayMismatch,
}

pub fn build_identity() -> &'static str {
    option_env!("NEOPVZ_BUILD_ID").unwrap_or(concat!(
        env!("CARGO_PKG_NAME"),
        "@",
        env!("CARGO_PKG_VERSION")
    ))
}

fn default_mode() -> ModeKind {
    ModeKind::Adventure
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum SceneKind {
    #[default]
    Title,
    AdventureSelect,
    AdventureTutorial,
    SeedChooser,
    ModeSelect,
    Day,
    Night,
    Pool,
    Fog,
    Roof,
    Boss,
    Garden,
    Complete,
    GameOver,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum GardenServiceKind {
    #[default]
    Zen,
    Mushroom,
    Aquarium,
    TreeOfWisdom,
}

impl GardenServiceKind {
    fn from_level(level: u8) -> Self {
        match level {
            1 => Self::Mushroom,
            2 => Self::Aquarium,
            3 => Self::TreeOfWisdom,
            _ => Self::Zen,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum ChallengeKind {
    #[default]
    Generic,
    SlotMachine,
    RainingSeeds,
    Beghouled,
    Zombiquarium,
    WhackAZombie,
    WallnutBowling,
    LastStand,
    BobsledBonanza,
    PogoParty,
    WarAndPeas,
    WarAndPeas2,
    Invisighoul,
    SeeingStars,
    BeghouledTwist,
    LittleTrouble,
    PortalCombat,
    Column,
    ZombiesOnSpeed,
    FinalBoss,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ChallengeState {
    pub kind: ChallengeKind,
    pub score: u32,
    pub target: u32,
    pub countdown: u32,
    pub stage: u8,
    #[serde(default)]
    pub conveyor_countdown: u32,
    #[serde(default)]
    pub last_conveyor_seed: Option<PlantType>,
}

fn challenge_kind(level: u8) -> ChallengeKind {
    match level {
        0 => ChallengeKind::WarAndPeas,
        1 | 17 => ChallengeKind::WallnutBowling,
        2 => ChallengeKind::SlotMachine,
        3 => ChallengeKind::RainingSeeds,
        4 => ChallengeKind::Beghouled,
        5 => ChallengeKind::Invisighoul,
        6 => ChallengeKind::SeeingStars,
        7 => ChallengeKind::Zombiquarium,
        8 => ChallengeKind::BeghouledTwist,
        9 => ChallengeKind::LittleTrouble,
        10 => ChallengeKind::PortalCombat,
        11 => ChallengeKind::Column,
        12 => ChallengeKind::BobsledBonanza,
        13 => ChallengeKind::ZombiesOnSpeed,
        14 => ChallengeKind::WhackAZombie,
        15 => ChallengeKind::LastStand,
        16 => ChallengeKind::WarAndPeas2,
        18 => ChallengeKind::PogoParty,
        19 => ChallengeKind::FinalBoss,
        _ => ChallengeKind::Generic,
    }
}

fn is_conveyor_challenge(kind: ChallengeKind) -> bool {
    matches!(
        kind,
        ChallengeKind::WallnutBowling
            | ChallengeKind::Invisighoul
            | ChallengeKind::LittleTrouble
            | ChallengeKind::PortalCombat
            | ChallengeKind::Column
            | ChallengeKind::FinalBoss
    )
}

fn conveyor_initial_seeds(level: u8) -> &'static [PlantType] {
    match level {
        1 | 17 => &[PlantType::Other(3)],
        5 => &[PlantType::Peashooter, PlantType::Other(14)],
        11 => &[
            PlantType::Other(4),
            PlantType::Other(23),
            PlantType::Other(39),
            PlantType::Other(31),
            PlantType::Other(35),
            PlantType::Other(39),
        ],
        19 => &[
            PlantType::Other(32),
            PlantType::Other(20),
            PlantType::Other(32),
            PlantType::Other(14),
        ],
        _ => &[],
    }
}

fn fixed_seed_bank(mode: ModeKind, level: u8) -> Option<&'static [PlantType]> {
    match mode {
        ModeKind::Vasebreaker => Some(&[PlantType::Other(2)]),
        ModeKind::IZombie => Some(&[]),
        ModeKind::MiniGame => match challenge_kind(level) {
            ChallengeKind::SlotMachine => Some(&[
                PlantType::Sunflower,
                PlantType::Peashooter,
                PlantType::Other(5),
            ]),
            ChallengeKind::RainingSeeds
            | ChallengeKind::Beghouled
            | ChallengeKind::BeghouledTwist => Some(&[]),
            ChallengeKind::Zombiquarium => Some(&[
                PlantType::ZombiquariumSnorkel,
                PlantType::ZombiquariumTrophy,
            ]),
            ChallengeKind::WhackAZombie => Some(&[
                PlantType::Other(4),
                PlantType::Other(11),
                PlantType::Other(14),
            ]),
            _ => None,
        },
        _ => None,
    }
}

fn izombie_seed_bank(level: u8) -> &'static [ZombieType] {
    match level.min(9) {
        0 => &[
            ZombieType::Normal,
            ZombieType::Buckethead,
            ZombieType::Football,
        ],
        1 => &[
            ZombieType::Normal,
            ZombieType::ScreenDoor,
            ZombieType::Buckethead,
        ],
        2 => &[
            ZombieType::Normal,
            ZombieType::Buckethead,
            ZombieType::Digger,
        ],
        3 => &[
            ZombieType::Normal,
            ZombieType::Buckethead,
            ZombieType::Ladder,
        ],
        4 => &[
            ZombieType::Normal,
            ZombieType::Buckethead,
            ZombieType::Bungee,
            ZombieType::Balloon,
        ],
        5 => &[
            ZombieType::Normal,
            ZombieType::PoleVaulter,
            ZombieType::Buckethead,
            ZombieType::Gargantuar,
        ],
        6 => &[
            ZombieType::Normal,
            ZombieType::PoleVaulter,
            ZombieType::Buckethead,
            ZombieType::Dancer,
        ],
        7 => &[
            ZombieType::Imp,
            ZombieType::Conehead,
            ZombieType::Buckethead,
            ZombieType::Bungee,
            ZombieType::Digger,
            ZombieType::Ladder,
        ],
        8 => &[
            ZombieType::Imp,
            ZombieType::Conehead,
            ZombieType::PoleVaulter,
            ZombieType::Buckethead,
            ZombieType::Bungee,
            ZombieType::Digger,
            ZombieType::Ladder,
            ZombieType::Football,
        ],
        _ => &[
            ZombieType::Imp,
            ZombieType::Conehead,
            ZombieType::PoleVaulter,
            ZombieType::Buckethead,
            ZombieType::Bungee,
            ZombieType::Digger,
            ZombieType::Ladder,
            ZombieType::Football,
            ZombieType::Dancer,
        ],
    }
}

fn conveyor_initial_countdown(level: u8) -> u32 {
    match level {
        1 | 17 => 400,
        5 | 11 | 19 => 1_000,
        9 => 200,
        _ => 0,
    }
}

fn conveyor_seed_pool(level: u8) -> &'static [(PlantType, u32)] {
    match level {
        1 => &[(PlantType::Other(3), 85), (PlantType::Other(49), 15)],
        17 => &[
            (PlantType::Other(3), 85),
            (PlantType::Other(49), 15),
            (PlantType::Other(50), 15),
        ],
        5 => &[
            (PlantType::Peashooter, 25),
            (PlantType::Other(3), 15),
            (PlantType::Other(34), 5),
            (PlantType::Other(17), 15),
            (PlantType::Other(16), 30),
            (PlantType::Other(14), 10),
        ],
        9 => &[
            (PlantType::Other(16), 25),
            (PlantType::Other(3), 15),
            (PlantType::Peashooter, 25),
            (PlantType::Other(2), 35),
        ],
        10 => &[
            (PlantType::Peashooter, 25),
            (PlantType::Other(7), 20),
            (PlantType::Other(22), 10),
            (PlantType::Other(26), 15),
            (PlantType::Other(3), 15),
            (PlantType::Other(2), 15),
        ],
        11 => &[
            (PlantType::Other(33), 155),
            (PlantType::Other(39), 5),
            (PlantType::Other(6), 5),
            (PlantType::Other(30), 15),
            (PlantType::Other(20), 10),
            (PlantType::Other(17), 10),
        ],
        19 => &[
            (PlantType::Other(33), 55),
            (PlantType::Other(39), 10),
            (PlantType::Other(20), 12),
            (PlantType::Other(32), 10),
            (PlantType::Other(34), 5),
            (PlantType::Other(14), 8),
        ],
        _ => &[],
    }
}

fn conveyor_interval(kind: ChallengeKind, seed_count: usize) -> u32 {
    let base = match seed_count {
        0..=4 => 400,
        5..=6 => 425,
        7..=8 => 500,
        _ => 1_000,
    };
    match kind {
        ChallengeKind::FinalBoss => base * 7 / 8,
        ChallengeKind::PortalCombat => base * 3 / 2,
        ChallengeKind::Invisighoul => base * 2,
        ChallengeKind::Column => base * 3,
        _ => base,
    }
}

fn conveyor_curve(weight: u32, count: u32, limit: u32) -> u32 {
    if count >= limit {
        1
    } else {
        weight.saturating_sub(
            weight
                .saturating_sub(1)
                .saturating_mul(count)
                .checked_div(limit)
                .unwrap_or(0),
        )
    }
}

fn initial_challenge_state(mode: ModeKind, level: u8) -> ChallengeState {
    if mode != ModeKind::MiniGame {
        return ChallengeState::default();
    }
    let kind = challenge_kind(level);
    let (target, countdown) = match kind {
        ChallengeKind::SlotMachine => (2_000, 0),
        ChallengeKind::RainingSeeds => (0, 200),
        ChallengeKind::Beghouled => (75, 1_500),
        ChallengeKind::Zombiquarium => (1_000, 0),
        ChallengeKind::WhackAZombie => (0, 200),
        ChallengeKind::LastStand => (5, 0),
        _ => (0, 0),
    };
    ChallengeState {
        kind,
        score: 0,
        target,
        countdown,
        stage: 0,
        conveyor_countdown: 0,
        last_conveyor_seed: None,
    }
}

pub const SURVIVAL_LEVEL_NAMES: [&str; 11] = [
    "SURVIVAL_DAY_NORMAL",
    "SURVIVAL_NIGHT_NORMAL",
    "SURVIVAL_POOL_NORMAL",
    "SURVIVAL_FOG_NORMAL",
    "SURVIVAL_ROOF_NORMAL",
    "SURVIVAL_DAY_HARD",
    "SURVIVAL_NIGHT_HARD",
    "SURVIVAL_POOL_HARD",
    "SURVIVAL_FOG_HARD",
    "SURVIVAL_ROOF_HARD",
    "SURVIVAL_POOL_ENDLESS",
];

pub const MINIGAME_LEVEL_NAMES: [&str; 20] = [
    "WAR_AND_PEAS",
    "WALL_NUT_BOWLING",
    "SLOT_MACHINE",
    "ITS_RAINING_SEEDS",
    "BEGHOULED",
    "INVISIGHOUL",
    "SEEING_STARS",
    "ZOMBIQUARIUM",
    "BEGHOULED_TWIST",
    "LITTLE_TROUBLE",
    "PORTAL_COMBAT",
    "COLUMN_AS_YOU_SEE_EM",
    "BOBSLED_BONANZA",
    "ZOMBIES_ON_SPEED",
    "WHACK_A_ZOMBIE",
    "LAST_STAND",
    "WAR_AND_PEAS_2",
    "WALL_NUT_BOWLING_EXTREME",
    "POGO_PARTY",
    "FINAL_BOSS",
];

pub const VASEBREAKER_LEVEL_NAMES: [&str; 10] = [
    "SCARY_POTTER_1",
    "SCARY_POTTER_2",
    "SCARY_POTTER_3",
    "SCARY_POTTER_4",
    "SCARY_POTTER_5",
    "SCARY_POTTER_6",
    "SCARY_POTTER_7",
    "SCARY_POTTER_8",
    "SCARY_POTTER_9",
    "SCARY_POTTER_ENDLESS",
];

pub const IZOMBIE_LEVEL_NAMES: [&str; 10] = [
    "I_ZOMBIE_1",
    "I_ZOMBIE_2",
    "I_ZOMBIE_3",
    "I_ZOMBIE_4",
    "I_ZOMBIE_5",
    "I_ZOMBIE_6",
    "I_ZOMBIE_7",
    "I_ZOMBIE_8",
    "I_ZOMBIE_9",
    "I_ZOMBIE_ENDLESS",
];

pub const GARDEN_SERVICE_NAMES: [&str; 4] = [
    "Zen_Garden",
    "Mushroom_Garden",
    "Aquarium",
    "Tree_of_Wisdom",
];

pub fn mode_level_names(mode: ModeKind) -> &'static [&'static str] {
    match mode {
        ModeKind::Survival => &SURVIVAL_LEVEL_NAMES,
        ModeKind::MiniGame => &MINIGAME_LEVEL_NAMES,
        ModeKind::Vasebreaker => &VASEBREAKER_LEVEL_NAMES,
        ModeKind::IZombie => &IZOMBIE_LEVEL_NAMES,
        ModeKind::ZenGarden => &GARDEN_SERVICE_NAMES,
        ModeKind::Adventure => &[],
    }
}

pub fn mode_level_name(mode: ModeKind, level: u8) -> Option<&'static str> {
    mode_level_names(mode).get(usize::from(level)).copied()
}

pub fn mode_level_scene(mode: ModeKind, level: u8) -> SceneKind {
    match mode {
        ModeKind::Survival => match level {
            1 | 6 => SceneKind::Night,
            2 | 7 | 10 => SceneKind::Pool,
            3 | 8 => SceneKind::Fog,
            4 | 9 => SceneKind::Roof,
            _ => SceneKind::Day,
        },
        ModeKind::MiniGame if mode_level_name(mode, level) == Some("ZOMBIQUARIUM") => {
            SceneKind::Pool
        }
        ModeKind::MiniGame if mode_level_name(mode, level) == Some("BOBSLED_BONANZA") => {
            SceneKind::Pool
        }
        ModeKind::MiniGame if mode_level_name(mode, level) == Some("WAR_AND_PEAS_2") => {
            SceneKind::Pool
        }
        ModeKind::MiniGame if mode_level_name(mode, level) == Some("LITTLE_TROUBLE") => {
            SceneKind::Pool
        }
        ModeKind::MiniGame if mode_level_name(mode, level) == Some("ZOMBIES_ON_SPEED") => {
            SceneKind::Pool
        }
        ModeKind::MiniGame
            if mode_level_name(mode, level) == Some("WALL_NUT_BOWLING_EXTREME")
                || mode_level_name(mode, level) == Some("LAST_STAND") =>
        {
            SceneKind::Pool
        }
        ModeKind::MiniGame if mode_level_name(mode, level) == Some("POGO_PARTY") => SceneKind::Roof,
        ModeKind::MiniGame if mode_level_name(mode, level) == Some("FINAL_BOSS") => SceneKind::Boss,
        ModeKind::MiniGame if mode_level_name(mode, level) == Some("COLUMN_AS_YOU_SEE_EM") => {
            SceneKind::Roof
        }
        ModeKind::MiniGame if mode_level_name(mode, level) == Some("ITS_RAINING_SEEDS") => {
            SceneKind::Fog
        }
        ModeKind::MiniGame if mode_level_name(mode, level) == Some("INVISIGHOUL") => SceneKind::Fog,
        ModeKind::MiniGame
            if mode_level_name(mode, level) == Some("BEGHOULED")
                || mode_level_name(mode, level) == Some("BEGHOULED_TWIST")
                || mode_level_name(mode, level) == Some("PORTAL_COMBAT")
                || mode_level_name(mode, level) == Some("WHACK_A_ZOMBIE") =>
        {
            SceneKind::Night
        }
        ModeKind::IZombie => SceneKind::Night,
        ModeKind::ZenGarden => SceneKind::Garden,
        ModeKind::Adventure => adventure_level_scene(level),
        _ => SceneKind::Day,
    }
}

// gZombieWaves in 1.0.0.1051 (Challenge.cpp:33-39); level 15's entry is dead
// because the Whack-a-Zombie branch forces 8 waves.
const ADVENTURE_WAVES: [u32; 50] = [
    4, 6, 8, 10, 8, 10, 20, 10, 20, 20, // 1-10
    10, 20, 10, 20, 10, 10, 20, 10, 20, 20, // 11-20
    10, 20, 20, 30, 20, 20, 30, 20, 30, 30, // 21-30
    10, 20, 10, 20, 20, 10, 20, 10, 20, 20, // 31-40
    10, 20, 20, 30, 20, 20, 30, 20, 30, 30, // 41-50
];

/// Board::PickBackground: Vasebreaker level 35 is checked before the fog
/// range and plays on a night board.
pub fn adventure_level_scene(level: u8) -> SceneKind {
    match level {
        1..=10 => SceneKind::Day,
        11..=20 | 35 => SceneKind::Night,
        21..=30 => SceneKind::Pool,
        31..=40 => SceneKind::Fog,
        41..=49 => SceneKind::Roof,
        50 => SceneKind::Boss,
        _ => SceneKind::Day,
    }
}

/// Board::PickZombieWaves: Whack-a-Zombie (15) is always 8; replays bump
/// every non-mini-boss level (<10 becomes 20, otherwise +10).
pub fn adventure_wave_count(level: u8, replay: bool) -> u32 {
    if level == 15 {
        return 8;
    }
    let base = ADVENTURE_WAVES[usize::from(level.clamp(1, 50)) - 1];
    if replay && !matches!(level, 10 | 20 | 30) {
        if base < 10 { 20 } else { base + 10 }
    } else {
        base
    }
}

/// gZombieDefs wave-composition stats: (value, starting level, first allowed
/// wave, pick weight). The wave check is 1-based against waveIndex + 1.
pub fn zombie_wave_stats(zombie_type: ZombieType) -> (u32, u8, u32, u32) {
    match zombie_type {
        ZombieType::Normal => (1, 1, 1, 4_000),
        ZombieType::Flag => (1, 1, 1, 0),
        ZombieType::Conehead => (2, 3, 1, 4_000),
        ZombieType::PoleVaulter => (2, 6, 5, 2_000),
        ZombieType::Buckethead => (4, 8, 1, 3_000),
        ZombieType::Newspaper => (2, 11, 1, 1_000),
        ZombieType::ScreenDoor => (4, 13, 5, 3_500),
        ZombieType::Football => (7, 16, 5, 2_000),
        ZombieType::Dancer => (5, 18, 5, 1_000),
        ZombieType::BackupDancer => (1, 18, 1, 0),
        ZombieType::DuckyTube => (1, 21, 5, 0),
        ZombieType::Snorkel => (3, 23, 10, 2_000),
        ZombieType::Zamboni => (7, 26, 10, 2_000),
        ZombieType::Bobsled => (3, 26, 10, 2_000),
        ZombieType::DolphinRider => (3, 28, 10, 1_500),
        ZombieType::Jackbox => (3, 31, 10, 1_000),
        ZombieType::Balloon => (2, 33, 10, 2_000),
        ZombieType::Digger => (4, 36, 10, 1_000),
        ZombieType::Pogo => (4, 38, 10, 1_000),
        ZombieType::Yeti => (4, 40, 1, 1),
        ZombieType::Bungee => (3, 41, 10, 1_000),
        ZombieType::Ladder => (4, 43, 10, 1_000),
        ZombieType::Catapult => (5, 46, 10, 1_500),
        ZombieType::Gargantuar => (10, 48, 15, 1_500),
        ZombieType::Imp => (10, 48, 1, 0),
        ZombieType::Boss => (10, 50, 1, 0),
        ZombieType::PeaHead => (1, 99, 1, 4_000),
        ZombieType::WallnutHead => (4, 99, 1, 3_000),
        ZombieType::JalapenoHead => (3, 99, 10, 1_000),
        ZombieType::GatlingHead => (3, 99, 10, 2_000),
        ZombieType::SquashHead => (3, 99, 10, 2_000),
        ZombieType::TallnutHead => (4, 99, 10, 2_000),
        ZombieType::Gigagargantuar => (10, 48, 15, 6_000),
    }
}

/// gZombieAllowedLevels bitmap for adventure (Challenge.cpp:41-259). Yetis
/// route through CanSpawnYetis instead and are never in the random pool.
pub fn adventure_zombie_allowed(zombie_type: ZombieType, level: u8) -> bool {
    let (_, starting_level, _, pick_weight) = zombie_wave_stats(zombie_type);
    if pick_weight == 0 || level < starting_level {
        return false;
    }
    match zombie_type {
        ZombieType::Normal => true,
        ZombieType::Conehead => level != 11,
        ZombieType::PoleVaulter => matches!(level, 6 | 7 | 9 | 10 | 14 | 15 | 24 | 29 | 42),
        ZombieType::Buckethead => matches!(
            level,
            8 | 9 | 10 | 12 | 15 | 22 | 24 | 27 | 29 | 30 | 37 | 39 | 40 | 42 | 45 | 49 | 50
        ),
        ZombieType::Newspaper => matches!(level, 11 | 12 | 15 | 22 | 24),
        ZombieType::ScreenDoor => matches!(level, 13 | 14 | 17 | 19 | 20),
        ZombieType::Football => matches!(level, 16 | 17 | 20 | 22 | 25 | 32 | 44),
        ZombieType::Dancer => matches!(level, 18..=20),
        ZombieType::Snorkel => matches!(level, 23 | 24 | 25 | 27 | 30),
        ZombieType::Zamboni | ZombieType::Bobsled => matches!(level, 26 | 27 | 29 | 30),
        ZombieType::DolphinRider => matches!(level, 28 | 29 | 30 | 34),
        ZombieType::Jackbox => matches!(level, 31 | 32 | 37 | 40 | 49 | 50),
        ZombieType::Balloon => matches!(level, 33 | 34 | 39 | 40),
        ZombieType::Digger => matches!(level, 36 | 37 | 40),
        ZombieType::Pogo => matches!(level, 38 | 39 | 40 | 44),
        ZombieType::Bungee => matches!(level, 41 | 42 | 47 | 49 | 50),
        ZombieType::Ladder => matches!(level, 43 | 44 | 45 | 47 | 49 | 50),
        ZombieType::Catapult => matches!(level, 46 | 47 | 49 | 50),
        ZombieType::Gargantuar => matches!(level, 48..=50),
        _ => false,
    }
}

/// Board::GetIntroducedZombieType: the first enum-order type whose starting
/// level matches; never level 1 and never the preview-only Ducky Tube.
pub fn adventure_introduced_zombie(level: u8) -> Option<ZombieType> {
    if level <= 1 {
        return None;
    }
    match level {
        3 => Some(ZombieType::Conehead),
        6 => Some(ZombieType::PoleVaulter),
        8 => Some(ZombieType::Buckethead),
        11 => Some(ZombieType::Newspaper),
        13 => Some(ZombieType::ScreenDoor),
        16 => Some(ZombieType::Football),
        18 => Some(ZombieType::Dancer),
        23 => Some(ZombieType::Snorkel),
        26 => Some(ZombieType::Zamboni),
        28 => Some(ZombieType::DolphinRider),
        31 => Some(ZombieType::Jackbox),
        33 => Some(ZombieType::Balloon),
        36 => Some(ZombieType::Digger),
        38 => Some(ZombieType::Pogo),
        41 => Some(ZombieType::Bungee),
        43 => Some(ZombieType::Ladder),
        46 => Some(ZombieType::Catapult),
        48 => Some(ZombieType::Gargantuar),
        50 => Some(ZombieType::Boss),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdventureAward {
    Plant(u8),
    Shovel,
    Almanac,
    CarKeys,
    Taco,
    WateringCan,
    Note,
    Trophy,
}

/// LawnApp::GetAwardSeedForLevel: the seed slot unlocked by a level.
pub fn adventure_award_seed(level: u8) -> u8 {
    let l = u32::from(level.clamp(1, 50));
    let area = (l - 1) / 10 + 1;
    let sub = (l - 1) % 10 + 1;
    let mut n = (area - 1) * 8 + sub;
    if sub >= 10 {
        n -= 2;
    } else if sub >= 5 {
        n -= 1;
    }
    n.min(40) as u8
}

/// Zombie::TrySpawnLevelAward: the first-run end-of-level award identity.
pub fn adventure_award(level: u8) -> AdventureAward {
    match level {
        4 => AdventureAward::Shovel,
        14 => AdventureAward::Almanac,
        24 => AdventureAward::CarKeys,
        34 => AdventureAward::Taco,
        44 => AdventureAward::WateringCan,
        9 | 19 | 29 | 39 | 49 => AdventureAward::Note,
        50 => AdventureAward::Trophy,
        _ => AdventureAward::Plant(adventure_award_seed(level)),
    }
}

/// LawnApp::CanShowAlmanac / CanShowStore / CanShowZenGarden thresholds.
pub fn adventure_unlocks(level: u8) -> (bool, bool, bool) {
    (level >= 15, level >= 25, level >= 45)
}

/// Board::InitLevel starting sun: item levels 15/35 play with none; the
/// first-run tutorial level starts at 150; everything else at 50.
pub fn adventure_starting_sun(level: u8, first_time: bool) -> u32 {
    match level {
        15 | 35 => 0,
        1 if first_time => 150,
        _ => 50,
    }
}

/// Board::GetNumSeedsInBank for adventure: fixed banks on 15/35, the
/// conveyor's ten, and the first-run tutorial slot progression.
pub fn adventure_seed_slots(level: u8, first_time: bool, packet_upgrades: u8) -> u8 {
    match level {
        35 => 1,
        15 => 3,
        _ if adventure_level_is_conveyor(level) => 10,
        1 if first_time => 1,
        2 if first_time => 2,
        3 if first_time => 3,
        4 if first_time => 4,
        6 if first_time => 5,
        _ => (6 + packet_upgrades).min(10),
    }
}

/// Board::ChooseSeedsOnCurrentLevel: the chooser opens after the level-7
/// tutorial stretch, except on conveyor and fixed-bank levels.
pub fn adventure_uses_seed_chooser(level: u8, first_time: bool) -> bool {
    if adventure_level_is_conveyor(level) || matches!(level, 15 | 35) {
        return false;
    }
    !first_time || level > 7
}

/// Board::AddGraveStones counts for adventure night levels.
pub fn adventure_grave_count(level: u8) -> u8 {
    match level {
        11..=13 => 4,
        14 | 16 => 7,
        15 => 9,
        17..=19 => 11,
        20 => 13,
        _ => 0,
    }
}

/// Board.cpp:1021-1035: first-time adventure 1-1 sods only row 2 and
/// 1-2/1-3 rows 1-3; dirt rows reject planting (Board.cpp:1078), never
/// host zombies (RowCanHaveZombies, Board.cpp:5951), and get no mower
/// (Board.cpp:1652-1655).
pub fn adventure_row_is_sodded(level: u8, row: u8) -> bool {
    match level {
        1 => row == 2,
        2 | 3 => (1..=3).contains(&row),
        _ => true,
    }
}

/// Board::HasConveyorBeltSeedBank for adventure levels.
pub fn adventure_level_is_conveyor(level: u8) -> bool {
    matches!(level, 5 | 10 | 20 | 25 | 30 | 40 | 45 | 50)
}

/// Board::IsFlagWave for adventure levels.
pub fn adventure_is_flag_wave(level: u8, replay: bool, wave_index: u32) -> bool {
    if !replay && level == 1 {
        return false;
    }
    let waves = adventure_wave_count(level, replay);
    let per_flag = if !replay && waves < 10 { waves } else { 10 };
    wave_index % per_flag == per_flag - 1
}

// Enum order used by PickZombieType and PutInMissingZombies for the types
// reachable in adventure waves.
const ADVENTURE_PICK_ORDER: [ZombieType; 20] = [
    ZombieType::Normal,
    ZombieType::Conehead,
    ZombieType::PoleVaulter,
    ZombieType::Buckethead,
    ZombieType::Newspaper,
    ZombieType::ScreenDoor,
    ZombieType::Football,
    ZombieType::Dancer,
    ZombieType::Snorkel,
    ZombieType::Zamboni,
    ZombieType::Bobsled,
    ZombieType::DolphinRider,
    ZombieType::Jackbox,
    ZombieType::Balloon,
    ZombieType::Digger,
    ZombieType::Pogo,
    ZombieType::Bungee,
    ZombieType::Ladder,
    ZombieType::Catapult,
    ZombieType::Gargantuar,
];

fn put_zombie_in_wave(wave: &mut Vec<ZombieType>, points: &mut i32, zombie_type: ZombieType) {
    wave.push(zombie_type);
    *points -= zombie_wave_stats(zombie_type).0 as i32;
}

/// Board::GetNumWavesPerFlag / IsFlagWave: first-run level 1 has no flag
/// waves at all; short first-run levels flag only on their final wave;
/// everything else flags every 10 waves.
pub fn adventure_flag_wave_count(level: u8, replay: bool) -> u32 {
    if !replay && level == 1 {
        return 0;
    }
    let waves = adventure_wave_count(level, replay);
    let per_flag = if !replay && waves < 10 { waves } else { 10 };
    waves / per_flag
}

fn mode_wave_config(mode: ModeKind, level: u8) -> (u32, bool) {
    match mode {
        ModeKind::Survival if level < 5 => (10, false),
        ModeKind::Survival if level < 10 => (20, false),
        ModeKind::Survival => (20, true),
        ModeKind::Vasebreaker | ModeKind::IZombie | ModeKind::ZenGarden => (0, false),
        ModeKind::MiniGame => match challenge_kind(level) {
            ChallengeKind::SlotMachine
            | ChallengeKind::Beghouled
            | ChallengeKind::BeghouledTwist
            | ChallengeKind::Zombiquarium => (0, false),
            ChallengeKind::RainingSeeds
            | ChallengeKind::SeeingStars
            | ChallengeKind::ZombiesOnSpeed => (40, false),
            ChallengeKind::Invisighoul | ChallengeKind::PortalCombat => (20, false),
            ChallengeKind::LittleTrouble | ChallengeKind::Column => (30, false),
            ChallengeKind::WhackAZombie => (12, false),
            ChallengeKind::WallnutBowling if level == 17 => (30, false),
            ChallengeKind::WallnutBowling => (20, false),
            ChallengeKind::LastStand => (10, false),
            ChallengeKind::WarAndPeas => (20, false),
            ChallengeKind::WarAndPeas2 => (30, false),
            ChallengeKind::BobsledBonanza | ChallengeKind::PogoParty => (30, false),
            ChallengeKind::FinalBoss => (40, false),
            _ => (1, false),
        },
        ModeKind::Adventure if (1..=50).contains(&level) => {
            (adventure_wave_count(level, false), false)
        }
        _ => (1, false),
    }
}

fn survival_stage_limit(level: u8) -> u8 {
    if level < 5 {
        5
    } else if level < 10 {
        10
    } else {
        u8::MAX
    }
}

fn initial_garden_state(service: GardenServiceKind) -> GardenState {
    let plant_type = match service {
        GardenServiceKind::Zen => Some(PlantType::Sunflower),
        GardenServiceKind::Mushroom => Some(PlantType::Other(8)),
        GardenServiceKind::Aquarium => Some(PlantType::Other(24)),
        GardenServiceKind::TreeOfWisdom => None,
    };
    GardenState {
        plants: plant_type
            .into_iter()
            .map(|plant_type| GardenPlant {
                plant_type,
                age_ticks: 0,
                watered: false,
                happy: false,
            })
            .collect(),
    }
}

fn izombie_columns(level: u8) -> u8 {
    match level {
        0..=4 => 4,
        8 => 6,
        _ => 5,
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlantType {
    Peashooter,
    Sunflower,
    Other(u8),
    ZombiquariumSnorkel,
    ZombiquariumTrophy,
}

impl PlantType {
    fn from_slot(slot: u8) -> Option<Self> {
        match slot {
            0 => Some(Self::Peashooter),
            1 => Some(Self::Sunflower),
            2..=52 => Some(Self::Other(slot)),
            _ => None,
        }
    }

    pub(crate) fn slot(self) -> u8 {
        match self {
            Self::Peashooter => 0,
            Self::Sunflower => 1,
            Self::Other(slot) => slot,
            Self::ZombiquariumSnorkel | Self::ZombiquariumTrophy => MAX_SEED_SLOTS,
        }
    }

    fn is_plant(self) -> bool {
        match self {
            Self::Peashooter | Self::Sunflower => true,
            Self::Other(slot) => slot < MAX_SEED_SLOTS,
            Self::ZombiquariumSnorkel | Self::ZombiquariumTrophy => false,
        }
    }

    fn definition(self) -> PlantDefinition {
        PLANT_DEFINITIONS[usize::from(self.slot())]
    }

    fn cost(self) -> u32 {
        self.definition().cost
    }

    fn launch_rate(self) -> u32 {
        self.definition().launch_rate
    }

    fn refresh_time(self) -> u32 {
        self.definition().refresh_time
    }

    fn max_health(self) -> i32 {
        self.definition().max_health
    }

    fn is_producer(self) -> bool {
        matches!(self.slot(), 1 | 9 | 38 | 41)
    }

    fn is_imitater(self) -> bool {
        self.slot() == 48
    }

    fn is_cob_cannon(self) -> bool {
        self.slot() == 47
    }

    fn is_sunshroom(self) -> bool {
        self.slot() == 9
    }

    fn is_twin_sunflower(self) -> bool {
        self.slot() == 41
    }

    fn is_marigold(self) -> bool {
        self.slot() == 38
    }

    fn is_gold_magnet(self) -> bool {
        self.slot() == 45
    }

    fn is_instant_coffee(self) -> bool {
        self.slot() == 35
    }

    fn is_nocturnal(self) -> bool {
        matches!(self.slot(), 8 | 9 | 10 | 12 | 13 | 14 | 15 | 24 | 31 | 42)
    }

    fn is_gravebuster(self) -> bool {
        self.slot() == 11
    }

    fn is_blover(self) -> bool {
        self.slot() == 27
    }

    fn is_cherry_bomb(self) -> bool {
        self.slot() == 2
    }

    fn is_potato_mine(self) -> bool {
        self.slot() == 4
    }

    fn is_chomper(self) -> bool {
        self.slot() == 6
    }

    fn is_tangle_kelp(self) -> bool {
        self.slot() == 19
    }

    fn is_garlic(self) -> bool {
        self.slot() == 36
    }

    fn is_jalapeno(self) -> bool {
        self.slot() == 20
    }

    fn is_squash(self) -> bool {
        self.slot() == 17
    }

    fn is_ice_shroom(self) -> bool {
        self.slot() == 14
    }

    fn is_doom_shroom(self) -> bool {
        self.slot() == 15
    }

    fn is_torchwood(self) -> bool {
        self.slot() == 22
    }

    fn is_spikeweed(self) -> bool {
        matches!(self.slot(), 21 | 46)
    }

    fn is_explode_o_nut(self) -> bool {
        self.slot() == 49
    }

    fn is_hypno_shroom(self) -> bool {
        self.slot() == 12
    }

    fn is_fume_shroom(self) -> bool {
        self.slot() == 10
    }

    fn is_gloom_shroom(self) -> bool {
        self.slot() == 42
    }

    fn is_scaredy_shroom(self) -> bool {
        self.slot() == 13
    }

    fn is_puff_range_shooter(self) -> bool {
        // PuffShroom and SeaShroom use MOTION_PUFF short-range shots.
        matches!(self.slot(), 8 | 24)
    }

    fn is_shooter(self) -> bool {
        matches!(
            self.slot(),
            0 | 5
                | 7
                | 8
                | 10
                | 13
                | 18
                | 24
                | 26
                | 28
                | 29
                | 32
                | 34
                | 39
                | 40
                | 42
                | 43
                | 44
                | 52
        )
    }

    fn burst_count(self) -> u8 {
        match self.firing_pattern() {
            FiringPattern::Burst(count) => count,
            _ => 1,
        }
    }

    fn firing_pattern(self) -> FiringPattern {
        match self.slot() {
            7 => FiringPattern::Burst(2),
            18 => FiringPattern::ThreeRow,
            28 => FiringPattern::Split,
            29 => FiringPattern::Star,
            40 => FiringPattern::Burst(4),
            43 => FiringPattern::Homing,
            52 => FiringPattern::Backward,
            _ => FiringPattern::Single,
        }
    }

    fn projectile_motion(self) -> ProjectileMotion {
        if self.firing_pattern() == FiringPattern::Homing {
            ProjectileMotion::Homing
        } else if self.is_gloom_shroom() {
            ProjectileMotion::Gloom
        } else if self.is_fume_shroom() {
            ProjectileMotion::Fume
        } else if self.is_puff_range_shooter() {
            ProjectileMotion::Puff
        } else {
            self.projectile_type().motion()
        }
    }

    fn projectile_type(self) -> ProjectileType {
        match self.slot() {
            5 => ProjectileType::SnowPea,
            8 | 10 | 13 | 24 => ProjectileType::Puff,
            26 | 43 => ProjectileType::Spike,
            29 => ProjectileType::Star,
            32 => ProjectileType::Cabbage,
            34 => ProjectileType::Kernel,
            39 => ProjectileType::Melon,
            42 => ProjectileType::Puff,
            44 => ProjectileType::WinterMelon,
            _ => ProjectileType::Pea,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PlantDefinition {
    cost: u32,
    refresh_time: u32,
    launch_rate: u32,
    max_health: i32,
}

// Values are the player-facing seed packet values from the target build.
// Keep this slot order aligned with SeedType; field names make each value auditable.
const PLANT_DEFINITIONS: [PlantDefinition; 53] = [
    // 0 Peashooter
    PlantDefinition {
        cost: 100,
        refresh_time: 750,
        launch_rate: 150,
        max_health: 300,
    },
    // 1 Sunflower
    PlantDefinition {
        cost: 50,
        refresh_time: 750,
        launch_rate: 2_500,
        max_health: 300,
    },
    // 2 CherryBomb
    PlantDefinition {
        cost: 150,
        refresh_time: 5_000,
        launch_rate: 0,
        max_health: 300,
    },
    // 3 Wallnut
    PlantDefinition {
        cost: 50,
        refresh_time: 3_000,
        launch_rate: 0,
        max_health: 4_000,
    },
    // 4 PotatoMine
    PlantDefinition {
        cost: 25,
        refresh_time: 3_000,
        launch_rate: 0,
        max_health: 300,
    },
    // 5 SnowPea
    PlantDefinition {
        cost: 175,
        refresh_time: 750,
        launch_rate: 150,
        max_health: 300,
    },
    // 6 Chomper
    PlantDefinition {
        cost: 150,
        refresh_time: 750,
        launch_rate: 0,
        max_health: 300,
    },
    // 7 Repeater
    PlantDefinition {
        cost: 200,
        refresh_time: 750,
        launch_rate: 150,
        max_health: 300,
    },
    // 8 PuffShroom
    PlantDefinition {
        cost: 0,
        refresh_time: 750,
        launch_rate: 150,
        max_health: 300,
    },
    // 9 SunShroom
    PlantDefinition {
        cost: 25,
        refresh_time: 750,
        launch_rate: 2_500,
        max_health: 300,
    },
    // 10 FumeShroom
    PlantDefinition {
        cost: 75,
        refresh_time: 750,
        launch_rate: 150,
        max_health: 300,
    },
    // 11 GraveBuster
    PlantDefinition {
        cost: 75,
        refresh_time: 750,
        launch_rate: 0,
        max_health: 300,
    },
    // 12 HypnoShroom
    PlantDefinition {
        cost: 75,
        refresh_time: 3_000,
        launch_rate: 0,
        max_health: 300,
    },
    // 13 ScaredyShroom
    PlantDefinition {
        cost: 25,
        refresh_time: 750,
        launch_rate: 150,
        max_health: 300,
    },
    // 14 IceShroom
    PlantDefinition {
        cost: 75,
        refresh_time: 5_000,
        launch_rate: 0,
        max_health: 300,
    },
    // 15 DoomShroom
    PlantDefinition {
        cost: 125,
        refresh_time: 5_000,
        launch_rate: 0,
        max_health: 300,
    },
    // 16 LilyPad
    PlantDefinition {
        cost: 25,
        refresh_time: 750,
        launch_rate: 0,
        max_health: 300,
    },
    // 17 Squash
    PlantDefinition {
        cost: 50,
        refresh_time: 3_000,
        launch_rate: 0,
        max_health: 300,
    },
    // 18 ThreePeater
    PlantDefinition {
        cost: 325,
        refresh_time: 750,
        launch_rate: 150,
        max_health: 300,
    },
    // 19 TangleKelp
    PlantDefinition {
        cost: 25,
        refresh_time: 3_000,
        launch_rate: 0,
        max_health: 300,
    },
    // 20 Jalapeno
    PlantDefinition {
        cost: 125,
        refresh_time: 5_000,
        launch_rate: 0,
        max_health: 300,
    },
    // 21 Spikeweed
    PlantDefinition {
        cost: 100,
        refresh_time: 750,
        launch_rate: 0,
        max_health: 300,
    },
    // 22 Torchwood
    PlantDefinition {
        cost: 175,
        refresh_time: 750,
        launch_rate: 0,
        max_health: 300,
    },
    // 23 Tallnut
    PlantDefinition {
        cost: 125,
        refresh_time: 3_000,
        launch_rate: 0,
        max_health: 8_000,
    },
    // 24 SeaShroom
    PlantDefinition {
        cost: 0,
        refresh_time: 3_000,
        launch_rate: 150,
        max_health: 300,
    },
    // 25 Plantern
    PlantDefinition {
        cost: 25,
        refresh_time: 3_000,
        launch_rate: 2_500,
        max_health: 300,
    },
    // 26 Cactus
    PlantDefinition {
        cost: 125,
        refresh_time: 750,
        launch_rate: 150,
        max_health: 300,
    },
    // 27 Blover
    PlantDefinition {
        cost: 100,
        refresh_time: 750,
        launch_rate: 0,
        max_health: 300,
    },
    // 28 SplitPea
    PlantDefinition {
        cost: 125,
        refresh_time: 750,
        launch_rate: 150,
        max_health: 300,
    },
    // 29 Starfruit
    PlantDefinition {
        cost: 125,
        refresh_time: 750,
        launch_rate: 150,
        max_health: 300,
    },
    // 30 PumpkinShell
    PlantDefinition {
        cost: 125,
        refresh_time: 3_000,
        launch_rate: 0,
        max_health: 4_000,
    },
    // 31 MagnetShroom
    PlantDefinition {
        cost: 100,
        refresh_time: 750,
        launch_rate: 0,
        max_health: 300,
    },
    // 32 CabbagePult
    PlantDefinition {
        cost: 100,
        refresh_time: 750,
        launch_rate: 300,
        max_health: 300,
    },
    // 33 FlowerPot
    PlantDefinition {
        cost: 25,
        refresh_time: 750,
        launch_rate: 0,
        max_health: 300,
    },
    // 34 KernelPult
    PlantDefinition {
        cost: 100,
        refresh_time: 750,
        launch_rate: 300,
        max_health: 300,
    },
    // 35 InstantCoffee
    PlantDefinition {
        cost: 75,
        refresh_time: 750,
        launch_rate: 0,
        max_health: 300,
    },
    // 36 Garlic
    PlantDefinition {
        cost: 50,
        refresh_time: 750,
        launch_rate: 0,
        max_health: 400,
    },
    // 37 Umbrella
    PlantDefinition {
        cost: 100,
        refresh_time: 750,
        launch_rate: 0,
        max_health: 300,
    },
    // 38 Marigold
    PlantDefinition {
        cost: 50,
        refresh_time: 3_000,
        launch_rate: 2_500,
        max_health: 300,
    },
    // 39 MelonPult
    PlantDefinition {
        cost: 300,
        refresh_time: 750,
        launch_rate: 300,
        max_health: 300,
    },
    // 40 GatlingPea
    PlantDefinition {
        cost: 250,
        refresh_time: 5_000,
        launch_rate: 150,
        max_health: 300,
    },
    // 41 TwinSunflower
    PlantDefinition {
        cost: 150,
        refresh_time: 5_000,
        launch_rate: 2_500,
        max_health: 300,
    },
    // 42 GloomShroom
    PlantDefinition {
        cost: 150,
        refresh_time: 5_000,
        launch_rate: 200,
        max_health: 300,
    },
    // 43 Cattail
    PlantDefinition {
        cost: 225,
        refresh_time: 5_000,
        launch_rate: 150,
        max_health: 300,
    },
    // 44 WinterMelon
    PlantDefinition {
        cost: 200,
        refresh_time: 5_000,
        launch_rate: 300,
        max_health: 300,
    },
    // 45 GoldMagnet
    PlantDefinition {
        cost: 50,
        refresh_time: 5_000,
        launch_rate: 0,
        max_health: 300,
    },
    // 46 SpikeRock
    PlantDefinition {
        cost: 125,
        refresh_time: 5_000,
        launch_rate: 0,
        max_health: 450,
    },
    // 47 CobCannon
    PlantDefinition {
        cost: 500,
        refresh_time: 5_000,
        launch_rate: 600,
        max_health: 300,
    },
    // 48 Imitater
    PlantDefinition {
        cost: 0,
        refresh_time: 750,
        launch_rate: 0,
        max_health: 300,
    },
    // 49 ExplodeONut
    PlantDefinition {
        cost: 0,
        refresh_time: 3_000,
        launch_rate: 0,
        max_health: 4_000,
    },
    // 50 GiantWallnut
    PlantDefinition {
        cost: 0,
        refresh_time: 3_000,
        launch_rate: 0,
        max_health: 4_000,
    },
    // 51 Sprout
    PlantDefinition {
        cost: 0,
        refresh_time: 3_000,
        launch_rate: 0,
        max_health: 300,
    },
    // 52 LeftPeater
    PlantDefinition {
        cost: 200,
        refresh_time: 750,
        launch_rate: 150,
        max_health: 300,
    },
];

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ZombieType {
    Normal,
    Conehead,
    Flag,
    Buckethead,
    ScreenDoor,
    DuckyTube,
    DolphinRider,
    Snorkel,
    Zamboni,
    Football,
    Digger,
    Bungee,
    Newspaper,
    Imp,
    Jackbox,
    Balloon,
    PoleVaulter,
    Yeti,
    PeaHead,
    Catapult,
    Dancer,
    BackupDancer,
    Pogo,
    Gargantuar,
    Bobsled,
    Ladder,
    Boss,
    WallnutHead,
    JalapenoHead,
    GatlingHead,
    SquashHead,
    TallnutHead,
    Gigagargantuar,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProjectileType {
    Pea,
    SnowPea,
    Puff,
    Cabbage,
    Melon,
    WinterMelon,
    Kernel,
    Butter,
    Spike,
    Star,
    Fireball,
    Cob,
    ZombiePea,
    Other(u8),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProjectileMotion {
    Straight,
    Backwards,
    Lobbed,
    Homing,
    Star,
    Puff,
    Fume,
    Gloom,
}

#[derive(Clone, Copy, Debug)]
struct ProjectileTrajectory {
    motion: ProjectileMotion,
    position_x: i64,
    position_y: i64,
    velocity_x: i64,
    velocity_y: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FiringPattern {
    Single,
    Burst(u8),
    ThreeRow,
    Split,
    Star,
    Homing,
    Backward,
}

impl ProjectileType {
    fn damage(self) -> i32 {
        match self {
            Self::Pea | Self::SnowPea | Self::Puff | Self::Kernel | Self::Spike | Self::Star => 20,
            Self::Cabbage | Self::Fireball | Self::Butter => 40,
            Self::Melon | Self::WinterMelon => 80,
            Self::Cob => 1_800,
            Self::ZombiePea => 20,
            Self::Other(1) => 75,
            Self::Other(_) => 20,
        }
    }

    fn motion(self) -> ProjectileMotion {
        match self {
            Self::Star => ProjectileMotion::Star,
            Self::Cabbage
            | Self::Melon
            | Self::WinterMelon
            | Self::Kernel
            | Self::Butter
            | Self::Cob
            | Self::Other(1) => ProjectileMotion::Lobbed,
            _ => ProjectileMotion::Straight,
        }
    }

    fn chill_duration(self) -> u32 {
        match self {
            Self::SnowPea | Self::WinterMelon => 1_000,
            _ => 0,
        }
    }

    fn is_splash(self) -> bool {
        matches!(self, Self::Melon | Self::WinterMelon | Self::Fireball)
    }

    fn splash_damage(self) -> i32 {
        (self.damage() / 3).max(1)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ProjectileImpactSound {
    Splat,
    Kernel,
    Butter,
    Ignite,
    Melon,
    Shield,
    Plastic,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CoinType {
    Silver,
    Gold,
    Diamond,
    Sun,
    SmallSun,
    LargeSun,
    FinalSeedPacket,
    Trophy,
    Shovel,
    Almanac,
    CarKeys,
    Vase,
    WateringCan,
    Taco,
    Note,
    UsableSeedPacket,
    PresentPlant,
    AwardMoneyBag,
    AwardPresent,
    AwardBagDiamond,
    AwardSilverSunflower,
    AwardGoldSunflower,
    Chocolate,
    AwardChocolate,
    PresentMinigames,
    PresentPuzzleMode,
    PresentSurvivalMode,
}

impl CoinType {
    fn value(self) -> u32 {
        match self {
            Self::Silver => 1,
            Self::Gold => 5,
            Self::Diamond => 100,
            _ => 0,
        }
    }

    fn is_money(self) -> bool {
        matches!(self, Self::Silver | Self::Gold | Self::Diamond)
    }

    fn sun_value(self) -> u32 {
        match self {
            Self::Sun => NORMAL_SUN_VALUE,
            Self::SmallSun => SMALL_SUN_VALUE,
            Self::LargeSun => LARGE_SUN_VALUE,
            _ => 0,
        }
    }

    fn is_sun(self) -> bool {
        self.sun_value() != 0
    }

    fn award_value(self) -> u32 {
        match self {
            Self::Trophy => Self::Diamond.value(),
            Self::AwardMoneyBag => Self::Gold.value() * 5,
            Self::AwardBagDiamond | Self::AwardGoldSunflower => Self::Diamond.value() * 5,
            _ => 0,
        }
    }

    fn unlock_mask(self) -> u8 {
        match self {
            Self::PresentMinigames => 1,
            Self::PresentPuzzleMode => 2,
            Self::PresentSurvivalMode => 4,
            _ => 0,
        }
    }

    fn is_level_award(self) -> bool {
        matches!(
            self,
            Self::FinalSeedPacket
                | Self::Trophy
                | Self::AwardSilverSunflower
                | Self::AwardGoldSunflower
                | Self::Shovel
                | Self::CarKeys
                | Self::Almanac
                | Self::Vase
                | Self::WateringCan
                | Self::Taco
                | Self::Note
                | Self::AwardMoneyBag
                | Self::AwardBagDiamond
                | Self::AwardPresent
                | Self::AwardChocolate
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SunSource {
    Sky,
    Plant(EntityId),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum InputRejectReason {
    InvalidSlot,
    SeedRefreshing,
    NoSeedSelected,
    OutsideBoard,
    InvalidTerrain,
    Occupied,
    Crater,
    Ice,
    NotEnoughSun,
    NotReady,
    MissingEntity,
    NoVase,
    InvalidGardenTarget,
    ChallengeUnavailable,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum InputAction {
    Pause,
    Resume,
    Restart,
    SelectSeed {
        slot: u8,
    },
    Plant {
        row: u8,
        column: u8,
    },
    Shovel {
        row: u8,
        column: u8,
    },
    BreakVase {
        row: u8,
        column: u8,
    },
    DeployZombie {
        zombie_type: ZombieType,
        row: u8,
        column: u8,
    },
    GardenWater {
        plant: u8,
    },
    GardenFertilize {
        plant: u8,
    },
    // ponytail: one action covers bug spray and phonograph until their tool UI lands.
    GardenFulfillNeed {
        plant: u8,
    },
    PlantImitater {
        plant_slot: u8,
        row: u8,
        column: u8,
    },
    FireCobCannon {
        entity: EntityId,
        row: u8,
        column: u8,
    },
    GardenFeedTree,
    GardenLeave,
    ChallengeSpin,
    ChallengeMatch {
        length: u8,
    },
    ChallengeFeed {
        x: u16,
        y: u16,
    },
    ChallengeWhack {
        row: u8,
        column: u8,
    },
    CollectSun {
        entity: EntityId,
    },
    CollectCoin {
        entity: EntityId,
    },
    ConfirmSurvivalRepick,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct InputFrame {
    pub actions: Vec<InputAction>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SeedPacketState {
    pub slot: u8,
    pub plant_type: PlantType,
    pub refresh_remaining: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PlantState {
    pub id: EntityId,
    pub plant_type: PlantType,
    #[serde(default)]
    pub imitater_type: Option<PlantType>,
    pub row: u8,
    pub column: u8,
    pub health: i32,
    pub max_health: i32,
    pub launch_counter: u32,
    pub launch_rate: u32,
    pub shooting_counter: u32,
    pub burst_remaining: u8,
    pub burst_delay: u32,
    pub production_age: u32,
    pub production_stage: u8,
    pub special_counter: u32,
    pub special_armed: bool,
    pub special_target: Option<EntityId>,
    pub blink_counter: u32,
    #[serde(default)]
    pub asleep: bool,
    #[serde(default)]
    pub wake_up_counter: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ZombieState {
    pub id: EntityId,
    pub zombie_type: ZombieType,
    pub row: u8,
    pub position_x: i64,
    pub speed: i64,
    pub health: i32,
    pub max_health: i32,
    pub age: u32,
    pub groan_counter: i32,
    pub frozen_counter: u32,
    pub chilled_counter: u32,
    pub eating: bool,
    pub garlic_counter: u32,
    pub garlic_target: Option<EntityId>,
    pub from_wave: u32,
    #[serde(default)]
    pub hypnotized: bool,
    #[serde(default)]
    pub has_vaulted: bool,
    #[serde(default)]
    pub newspaper_health: i32,
    #[serde(default)]
    pub jackbox_timer: u32,
    #[serde(default)]
    pub yeti_counter: u32,
    #[serde(default)]
    pub yeti_running: bool,
    #[serde(default)]
    pub yeti_loot_dropped: bool,
    #[serde(default)]
    pub pea_head_counter: u32,
    #[serde(default)]
    pub catapult_counter: u32,
    #[serde(default)]
    pub catapult_shots: u8,
    #[serde(default)]
    pub catapult_armed: bool,
    #[serde(default)]
    pub vehicle_disabled: bool,
    #[serde(default)]
    pub damage_tier: u8,
    #[serde(default)]
    pub pogo_counter: u32,
    #[serde(default)]
    pub pogo_target_x: Option<i64>,
    #[serde(default)]
    pub pogo_velocity_x: i64,
    #[serde(default)]
    pub dancer_counter: u32,
    #[serde(default)]
    pub dancer_summoned: bool,
    #[serde(default)]
    pub digger_counter: u32,
    #[serde(default)]
    pub digger_underground: bool,
    #[serde(default)]
    pub bungee_counter: u32,
    #[serde(default)]
    pub bungee_stolen: bool,
    #[serde(default)]
    pub dolphin_phase: u8,
    #[serde(default)]
    pub dolphin_counter: u32,
    #[serde(default)]
    pub dolphin_target_x: Option<i64>,
    #[serde(default)]
    pub snorkel_phase: u8,
    #[serde(default)]
    pub balloon_phase: u8,
    #[serde(default)]
    pub balloon_counter: u32,
    #[serde(default)]
    pub balloon_flying_health: i32,
    #[serde(default)]
    pub blowing_away: bool,
    #[serde(default)]
    pub departed: bool,
    #[serde(default)]
    pub in_pool: bool,
    #[serde(default)]
    pub armor_intact: bool,
    #[serde(default)]
    pub portal_cooldown: u32,
    #[serde(default)]
    pub bungee_held: bool,
    #[serde(default)]
    pub imp_thrown: bool,
    #[serde(default)]
    pub imp_flight_ticks: u32,
    #[serde(default)]
    pub boss_head_counter: u32,
    #[serde(default)]
    pub boss_ball_active: bool,
    #[serde(default)]
    pub boss_ball_fire: bool,
    #[serde(default)]
    pub boss_ball_row: u8,
    #[serde(default)]
    pub boss_ball_x: i64,
    #[serde(default)]
    pub shield_health: i32,
    #[serde(default)]
    pub shield_max_health: i32,
    #[serde(default)]
    pub ladder_placed: bool,
    #[serde(default)]
    pub bobsled_leader: bool,
    #[serde(default)]
    pub bobsled_counter: u32,
    #[serde(default)]
    pub bobsled_sliding: bool,
    #[serde(default)]
    pub special_counter: u32,
    #[serde(default)]
    pub special_phase: u8,
    #[serde(default)]
    pub special_target: Option<EntityId>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ProjectileState {
    pub id: EntityId,
    pub projectile_type: ProjectileType,
    pub motion: ProjectileMotion,
    pub row: u8,
    pub position_x: i64,
    pub position_y: i64,
    pub velocity_x: i64,
    pub velocity_y: i64,
    pub damage: i32,
    pub age: u32,
    #[serde(default)]
    pub target_x: Option<i64>,
    #[serde(default)]
    pub target_row: Option<u8>,
    #[serde(default)]
    pub lob_height: i32,
    #[serde(default)]
    pub lob_velocity: i32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SunPickupState {
    pub id: EntityId,
    pub source: SunSource,
    pub value: u32,
    pub position_x: i64,
    pub position_y: i64,
    #[serde(default)]
    pub target_y: Option<i64>,
    #[serde(default)]
    pub velocity_x: i64,
    #[serde(default)]
    pub velocity_y: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CoinPickupState {
    pub id: EntityId,
    pub coin_type: CoinType,
    pub value: u32,
    pub position_x: i64,
    pub position_y: i64,
    #[serde(default)]
    pub plant_type: Option<PlantType>,
    #[serde(default)]
    pub usable_seed_type: Option<PlantType>,
    #[serde(default)]
    pub target_y: Option<i64>,
    #[serde(default)]
    pub velocity_x: i64,
    #[serde(default)]
    pub velocity_y: i64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CraterState {
    pub row: u8,
    pub column: u8,
    pub remaining: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GraveState {
    pub row: u8,
    pub column: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LadderState {
    pub row: u8,
    pub column: u8,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum VaseContents {
    Plant(PlantType),
    Zombie(ZombieType),
    Sun(u8),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VaseState {
    pub id: EntityId,
    pub row: u8,
    pub column: u8,
    pub contents: VaseContents,
    pub leaf: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BrainState {
    pub row: u8,
    pub remaining: u32,
    pub squished: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MowerState {
    pub row: u8,
    pub position_x: i64,
    pub active: bool,
    pub spent: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WaveState {
    pub current: u32,
    pub total: u32,
    pub countdown: u32,
    pub countdown_start: u32,
    #[serde(default)]
    pub endless: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BoardState {
    pub rows: u8,
    pub columns: u8,
    pub next_entity_id: EntityId,
    pub selected_seed: Option<u8>,
    pub seed_packets: Vec<SeedPacketState>,
    #[serde(default)]
    pub zombie_packets: Vec<ZombieType>,
    pub plants: Vec<PlantState>,
    pub zombies: Vec<ZombieState>,
    pub projectiles: Vec<ProjectileState>,
    pub suns: Vec<SunPickupState>,
    pub coins: Vec<CoinPickupState>,
    pub craters: Vec<CraterState>,
    #[serde(default)]
    pub graves: Vec<GraveState>,
    #[serde(default)]
    pub ladders: Vec<LadderState>,
    #[serde(default)]
    pub rake: Option<(u8, u8)>,
    #[serde(default)]
    pub portals: Vec<(u8, u8, bool)>,
    #[serde(default)]
    pub wave_plan: Vec<Vec<ZombieType>>,
    #[serde(default)]
    pub scary_pot_stage: u8,
    #[serde(default)]
    pub huge_wave_countdown: u32,
    #[serde(default)]
    pub wave_health_threshold: i32,
    #[serde(default)]
    pub sky_drop_countdown: u32,
    #[serde(default)]
    pub ice_min_x: Vec<i64>,
    #[serde(default)]
    pub ice_timer: Vec<u32>,
    #[serde(default)]
    pub vases: Vec<VaseState>,
    #[serde(default)]
    pub brains: Vec<BrainState>,
    pub mowers: Vec<MowerState>,
    pub wave: WaveState,
    pub sun_countdown: u32,
    pub suns_fallen: u32,
    pub ice_counter: u32,
}

impl BoardState {
    fn new(scene: SceneKind, mode: ModeKind, level: u8, rng: &mut Mt19937) -> Self {
        let rows = if scene == SceneKind::Pool {
            POOL_ROWS
        } else {
            DAY_ROWS
        };
        let sun_countdown = SUN_COUNTDOWN + rng.range(276);
        let mowers = if !matches!(
            mode,
            ModeKind::Vasebreaker | ModeKind::IZombie | ModeKind::ZenGarden
        ) && matches!(
            scene,
            SceneKind::Day | SceneKind::Night | SceneKind::Pool | SceneKind::Fog | SceneKind::Roof
        ) {
            (0..rows)
                .filter(|&row| mode != ModeKind::Adventure || adventure_row_is_sodded(level, row))
                .map(|row| MowerState {
                    row,
                    position_x: -80 * POSITION_SCALE,
                    active: false,
                    spent: false,
                })
                .collect()
        } else {
            Vec::new()
        };
        let vases = if mode == ModeKind::Vasebreaker {
            initial_vases(level, rng)
        } else if mode == ModeKind::Adventure && level == 35 {
            scary_pot_stage_vases(0, 1, rng)
        } else {
            Vec::new()
        };
        let brains = if mode == ModeKind::IZombie {
            (0..DAY_ROWS)
                .map(|row| BrainState {
                    row,
                    remaining: I_ZOMBIE_BRAIN_TICKS,
                    squished: false,
                })
                .collect()
        } else {
            Vec::new()
        };
        let zombie_packets = if mode == ModeKind::IZombie {
            izombie_seed_bank(level).to_vec()
        } else {
            Vec::new()
        };
        let next_entity_id = u32::try_from(vases.len())
            .unwrap_or(u32::MAX.saturating_sub(1))
            .saturating_add(1);
        Self {
            rows,
            columns: GRID_COLUMNS,
            next_entity_id,
            selected_seed: None,
            seed_packets: (0..MAX_SEED_SLOTS)
                .filter_map(|slot| {
                    PlantType::from_slot(slot).map(|plant_type| SeedPacketState {
                        slot,
                        plant_type,
                        refresh_remaining: 0,
                    })
                })
                .collect(),
            zombie_packets,
            plants: Vec::new(),
            zombies: Vec::new(),
            projectiles: Vec::new(),
            suns: Vec::new(),
            coins: Vec::new(),
            craters: Vec::new(),
            graves: Vec::new(),
            ladders: Vec::new(),
            rake: None,
            portals: Vec::new(),
            wave_plan: Vec::new(),
            scary_pot_stage: 0,
            huge_wave_countdown: 0,
            wave_health_threshold: -1,
            sky_drop_countdown: 0,
            ice_min_x: vec![ICE_START_X; usize::from(rows)],
            ice_timer: vec![0; usize::from(rows)],
            vases,
            brains,
            mowers,
            wave: WaveState {
                current: 0,
                total: 1,
                countdown: FIRST_WAVE_COUNTDOWN,
                countdown_start: FIRST_WAVE_COUNTDOWN,
                endless: false,
            },
            sun_countdown,
            suns_fallen: 0,
            ice_counter: 0,
        }
    }

    fn allocate_entity(&mut self) -> EntityId {
        let id = self.next_entity_id;
        self.next_entity_id = self.next_entity_id.saturating_add(1);
        id
    }

    fn set_seed_packets(&mut self, plant_types: &[PlantType]) {
        self.seed_packets = plant_types
            .iter()
            .copied()
            .enumerate()
            .map(|(slot, plant_type)| SeedPacketState {
                slot: u8::try_from(slot).expect("seed packet slot must fit in u8"),
                plant_type,
                refresh_remaining: 0,
            })
            .collect();
    }
}

fn push_vase_contents(contents: &mut Vec<VaseContents>, content: VaseContents, count: u8) {
    for _ in 0..count {
        contents.push(content);
    }
}

fn vase_contents(level: u8) -> Vec<VaseContents> {
    let mut contents = Vec::new();
    match level.min(9) {
        0 => {
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Peashooter), 5);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(5)), 5);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(17)), 5);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Normal), 6);
            push_vase_contents(
                &mut contents,
                VaseContents::Zombie(ZombieType::Buckethead),
                3,
            );
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Jackbox), 1);
        }
        1 => {
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(7)), 7);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(5)), 3);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(3)), 3);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(4)), 2);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Normal), 6);
            push_vase_contents(
                &mut contents,
                VaseContents::Zombie(ZombieType::Buckethead),
                3,
            );
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Jackbox), 1);
        }
        2 => {
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Peashooter), 5);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(5)), 5);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(12)), 5);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Normal), 6);
            push_vase_contents(
                &mut contents,
                VaseContents::Zombie(ZombieType::Buckethead),
                2,
            );
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Dancer), 1);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Jackbox), 1);
        }
        3 => {
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(7)), 6);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(5)), 4);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(17)), 2);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(12)), 3);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(3)), 3);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Normal), 8);
            push_vase_contents(
                &mut contents,
                VaseContents::Zombie(ZombieType::Buckethead),
                2,
            );
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Dancer), 1);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Jackbox), 1);
        }
        4 => {
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(8)), 11);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(12)), 4);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(7)), 4);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Jackbox), 8);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Normal), 7);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Football), 1);
        }
        5 => {
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(7)), 6);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(30)), 3);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(17)), 4);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(12)), 2);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(5)), 2);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(31)), 3);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Normal), 6);
            push_vase_contents(
                &mut contents,
                VaseContents::Zombie(ZombieType::Buckethead),
                5,
            );
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Jackbox), 1);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Football), 3);
        }
        6 => {
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(7)), 7);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(17)), 2);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(23)), 5);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(18)), 2);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(22)), 4);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Normal), 7);
            push_vase_contents(
                &mut contents,
                VaseContents::Zombie(ZombieType::PoleVaulter),
                5,
            );
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Football), 6);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Jackbox), 6);
        }
        7 => {
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(21)), 13);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(3)), 3);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(17)), 3);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Normal), 10);
            push_vase_contents(
                &mut contents,
                VaseContents::Zombie(ZombieType::Buckethead),
                1,
            );
        }
        8 => {
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(8)), 7);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(3)), 3);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(17)), 5);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(7)), 4);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Jackbox), 8);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Normal), 4);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Pogo), 4);
        }
        _ => {
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(7)), 6);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(5)), 2);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Peashooter), 2);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(18)), 2);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(17)), 5);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(4)), 1);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(3)), 1);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(25)), 1);
            push_vase_contents(&mut contents, VaseContents::Sun(1), 1);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Normal), 8);
            push_vase_contents(
                &mut contents,
                VaseContents::Zombie(ZombieType::Buckethead),
                5,
            );
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Jackbox), 1);
            push_vase_contents(
                &mut contents,
                VaseContents::Zombie(ZombieType::Gargantuar),
                1,
            );
        }
    }
    contents
}

fn initial_vases(level: u8, rng: &mut Mt19937) -> Vec<VaseState> {
    let contents = vase_contents(level);
    let excluded_columns = match level.min(9) {
        0 => 4,
        1 => 5,
        2 => 4,
        3 => 3,
        7 => 3,
        _ => 2,
    };
    let leaf_count = match level.min(9) {
        0 => 0,
        2 => 3,
        _ => 2,
    };
    place_vases(contents, excluded_columns, leaf_count, 1, rng)
}

/// Challenge.cpp:3900-3944 ScaryPotterPopulate: adventure level 4-5 pot
/// layouts by mSurvivalStage (columns 6-8, then 5-8, then 4-8).
fn scary_pot_stage_vases(stage: u8, first_id: u32, rng: &mut Mt19937) -> Vec<VaseState> {
    let mut contents = Vec::new();
    let (excluded_columns, leaf_count) = match stage {
        0 => {
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Peashooter), 5);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(17)), 5);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Normal), 4);
            push_vase_contents(
                &mut contents,
                VaseContents::Zombie(ZombieType::Buckethead),
                1,
            );
            (6, 0)
        }
        1 => {
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Peashooter), 4);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(5)), 5);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(17)), 4);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Normal), 5);
            push_vase_contents(
                &mut contents,
                VaseContents::Zombie(ZombieType::Buckethead),
                1,
            );
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Football), 1);
            (5, 2)
        }
        _ => {
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Peashooter), 5);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(5)), 5);
            push_vase_contents(&mut contents, VaseContents::Plant(PlantType::Other(12)), 5);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Normal), 6);
            push_vase_contents(
                &mut contents,
                VaseContents::Zombie(ZombieType::Buckethead),
                2,
            );
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Dancer), 1);
            push_vase_contents(&mut contents, VaseContents::Zombie(ZombieType::Jackbox), 1);
            (4, 3)
        }
    };
    place_vases(contents, excluded_columns, leaf_count, first_id, rng)
}

fn place_vases(
    contents: Vec<VaseContents>,
    excluded_columns: u8,
    leaf_count: u8,
    first_id: u32,
    rng: &mut Mt19937,
) -> Vec<VaseState> {
    let capacity = usize::from(GRID_COLUMNS - excluded_columns) * usize::from(DAY_ROWS);
    let excluded_columns = if contents.len() > capacity {
        0
    } else {
        excluded_columns
    };
    let mut cells = (excluded_columns..GRID_COLUMNS)
        .flat_map(|column| (0..DAY_ROWS).map(move |row| (column, row)))
        .collect::<Vec<_>>();
    let mut remaining_leaves = leaf_count;
    let mut vases = Vec::with_capacity(contents.len().min(cells.len()));
    // ponytail: O(n^2) cell removal is bounded to one 9x5 board; use a swap pool only if setup grows.
    for (index, content) in contents.into_iter().enumerate() {
        if cells.is_empty() {
            break;
        }
        let cell_index = rng.range(u32::try_from(cells.len()).unwrap_or(1)) as usize;
        let (column, row) = cells.remove(cell_index);
        let content = match content {
            VaseContents::Sun(_) => {
                VaseContents::Sun(u8::try_from(rng.range(3).saturating_add(1)).unwrap_or(1))
            }
            content => content,
        };
        let leaf = remaining_leaves > 0 && matches!(content, VaseContents::Plant(_));
        if leaf {
            remaining_leaves -= 1;
        }
        vases.push(VaseState {
            id: first_id.saturating_add(u32::try_from(index).unwrap_or(u32::MAX)),
            row,
            column,
            contents: content,
            leaf,
        });
    }
    vases
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RngState {
    pub words: Vec<u32>,
    pub index: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GameState {
    pub seed: u64,
    pub tick: Tick,
    pub scene: SceneKind,
    pub level_scene: SceneKind,
    #[serde(default = "default_mode")]
    pub mode: ModeKind,
    #[serde(default)]
    pub level: u8,
    pub sun: u32,
    pub coins: u32,
    #[serde(default)]
    pub unlocked_modes: u8,
    #[serde(default)]
    pub chocolates: u32,
    #[serde(default)]
    pub pickup_inventory: Vec<CoinType>,
    pub wave: u32,
    pub paused: bool,
    #[serde(default)]
    pub garden_service: Option<GardenServiceKind>,
    #[serde(default)]
    pub garden: GardenState,
    #[serde(default)]
    pub tree_height: u16,
    #[serde(default)]
    pub challenge: ChallengeState,
    pub board: BoardState,
    pub rng: RngState,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum GameEvent {
    Started,
    Restarted,
    Paused,
    Resumed,
    SeedSelected {
        slot: u8,
        plant_type: PlantType,
    },
    InputRejected {
        action: InputAction,
        reason: InputRejectReason,
    },
    PlantPlaced {
        entity: EntityId,
        plant_type: PlantType,
        row: u8,
        column: u8,
        sun_remaining: u32,
    },
    PlantCombined {
        entity: EntityId,
        consumed: EntityId,
        plant_type: PlantType,
    },
    ImitaterMorphed {
        entity: EntityId,
        plant_type: PlantType,
    },
    PlantShoveled {
        entity: EntityId,
    },
    VaseBroken {
        entity: EntityId,
        row: u8,
        column: u8,
    },
    VaseRevealed {
        entity: EntityId,
        row: u8,
        column: u8,
        contents: VaseContents,
        leaf: bool,
    },
    ZombieDeployed {
        entity: EntityId,
        zombie_type: ZombieType,
        row: u8,
        column: u8,
        sun_remaining: u32,
    },
    GardenWatered {
        plant: u8,
        age_ticks: u32,
    },
    GardenFertilized {
        plant: u8,
        age_ticks: u32,
    },
    GardenBecameHappy {
        plant: u8,
        aquatic: bool,
    },
    GardenTreeGrew {
        height: u16,
    },
    GardenLeft,
    ChallengeAction {
        kind: ChallengeKind,
        value: u32,
    },
    BrainEaten {
        zombie: EntityId,
        row: u8,
        brains_remaining: u8,
    },
    BrainFinished {
        zombie: EntityId,
        row: u8,
        brains_remaining: u8,
    },
    PlantDamaged {
        entity: EntityId,
        damage: i32,
        health_remaining: i32,
    },
    PlantDied {
        entity: EntityId,
    },
    CraterCreated {
        row: u8,
        column: u8,
        duration: u32,
    },
    PlantSpecialTriggered {
        entity: EntityId,
        plant_type: PlantType,
    },
    SquashHumStarted {
        entity: EntityId,
        variant: u8,
    },
    TangleKelpGrabStarted {
        entity: EntityId,
    },
    TangleKelpWaterEntry {
        entity: EntityId,
    },
    BloverTriggered {
        entity: EntityId,
        row: u8,
    },
    GraveCleared {
        entity: EntityId,
        row: u8,
        column: u8,
    },
    PlantSpecialHit {
        plant: EntityId,
        zombie: EntityId,
        damage: i32,
        health_remaining: i32,
    },
    SunProduced {
        entity: EntityId,
        source: SunSource,
        value: u32,
    },
    SunCollected {
        entity: EntityId,
        value: u32,
        sun_total: u32,
    },
    CoinProduced {
        entity: EntityId,
        coin_type: CoinType,
        value: u32,
    },
    CoinCollected {
        entity: EntityId,
        coin_type: CoinType,
        value: u32,
        coin_total: u32,
    },
    SeedPacketReady {
        slot: u8,
        plant_type: PlantType,
    },
    PickupCollected {
        entity: EntityId,
        coin_type: CoinType,
        value: u32,
        coins_total: u32,
        sun_total: u32,
    },
    SurvivalRepickStarted {
        stage: u8,
    },
    SurvivalStageStarted {
        stage: u8,
    },
    WaveStarted {
        wave: u32,
    },
    ZombieSpawned {
        entity: EntityId,
        zombie_type: ZombieType,
        row: u8,
        wave: u32,
    },
    ZombieDamaged {
        entity: EntityId,
        damage: i32,
        health_remaining: i32,
        attacker: Option<EntityId>,
    },
    ZombieDamageTierChanged {
        entity: EntityId,
        tier: u8,
    },
    VehicleDisabled {
        entity: EntityId,
    },
    PlantFired {
        entity: EntityId,
        plant_type: PlantType,
        variant: u8,
    },
    ProjectileFired {
        entity: EntityId,
        source: EntityId,
        projectile_type: ProjectileType,
        row: u8,
    },
    CobCannonFired {
        entity: EntityId,
        target_row: u8,
        target_column: u8,
    },
    ProjectileHit {
        projectile: EntityId,
        zombie: EntityId,
        damage: i32,
        health_remaining: i32,
    },
    ProjectileImpact {
        projectile: EntityId,
        zombie: Option<EntityId>,
        kind: ProjectileImpactSound,
        variant: u8,
    },
    ProjectileSplashHit {
        projectile: EntityId,
        zombie: EntityId,
        damage: i32,
        health_remaining: i32,
    },
    ZombieChilled {
        entity: EntityId,
        duration: u32,
    },
    ZombieFrozen {
        entity: EntityId,
        duration: u32,
    },
    ZombieDied {
        entity: EntityId,
    },
    JackboxExploded {
        entity: EntityId,
        row: u8,
        column: u8,
    },
    ZombieFled {
        entity: EntityId,
    },
    MowerTriggered {
        row: u8,
    },
    ZombieHypnotized {
        entity: EntityId,
    },
    ImpThrown {
        gargantuar: EntityId,
        imp: EntityId,
        imp_variant: u8,
    },
    RakeTriggered {
        zombie: EntityId,
    },
    ZombieThawed {
        entity: EntityId,
    },
    PotatoMineArmed {
        entity: EntityId,
    },
    JumpBlocked {
        zombie: EntityId,
        plant: EntityId,
    },
    PogoStickLost {
        entity: EntityId,
    },
    PogoBounceSound {
        entity: EntityId,
    },
    DiggerSurfaced {
        entity: EntityId,
    },
    ZombieArmorLost {
        entity: EntityId,
    },
    ZombieShieldLost {
        entity: EntityId,
    },
    ZombieShieldHit {
        entity: EntityId,
        variant: u8,
    },
    ZombieNewspaperRipped {
        entity: EntityId,
    },
    BossAttackWindup {
        entity: EntityId,
        row: u8,
        fire: bool,
    },
    BossProjectileDestroyed {
        entity: EntityId,
        fire: bool,
    },
    PortalOpened {
        row: u8,
        column: u8,
        square: bool,
    },
    ZombieTeleported {
        entity: EntityId,
        row: u8,
        column: u8,
    },
    ZombieButtered {
        entity: EntityId,
    },
    UmbrellaDeflected {
        plant: EntityId,
        zombie: EntityId,
    },
    MetalStolen {
        plant: EntityId,
        zombie: Option<EntityId>,
    },
    ZombieVaulted {
        entity: EntityId,
    },
    DolphinJumpStarted {
        entity: EntityId,
    },
    ZombieEnteredPool {
        entity: EntityId,
        variant: u8,
    },
    ZombieRowChanged {
        entity: EntityId,
        from: u8,
        to: u8,
    },
    GameLost {
        zombie: EntityId,
    },
    GameWon,
    StateChanged,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StateHash(pub [u8; 32]);

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayHeader {
    pub format_version: u32,
    pub build_identity: String,
    pub resource_version: String,
    pub initial_save: StateHash,
    pub seed: u64,
}

impl ReplayHeader {
    pub fn new(seed: u64, resource_version: impl Into<String>, initial_save: StateHash) -> Self {
        Self {
            format_version: REPLAY_FORMAT_VERSION,
            build_identity: build_identity().to_owned(),
            resource_version: resource_version.into(),
            initial_save,
            seed,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Replay {
    pub header: ReplayHeader,
    pub scene: SceneKind,
    #[serde(default = "default_mode")]
    pub mode: ModeKind,
    #[serde(default)]
    pub level: u8,
    pub frames: Vec<InputFrame>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayOutcome {
    pub events: Vec<Vec<GameEvent>>,
    pub final_state: GameState,
    pub final_hash: StateHash,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReplayRecord {
    pub replay: Replay,
    pub outcome: ReplayOutcome,
}

impl Replay {
    pub fn new(seed: u64, scene: SceneKind) -> Self {
        Self {
            header: ReplayHeader::new(seed, "1.0.0.1051", StateHash([0; 32])),
            scene,
            mode: ModeKind::Adventure,
            level: 0,
            frames: Vec::new(),
        }
    }

    pub fn new_mode(seed: u64, mode: ModeKind, level: u8) -> Self {
        Self {
            header: ReplayHeader::new(seed, "1.0.0.1051", StateHash([0; 32])),
            scene: mode_level_scene(mode, level),
            mode,
            level,
            frames: Vec::new(),
        }
    }

    pub fn run(&self) -> Result<ReplayOutcome, CoreError> {
        if self.header.format_version != REPLAY_FORMAT_VERSION {
            return Err(CoreError::ReplayFormat {
                expected: REPLAY_FORMAT_VERSION,
                found: self.header.format_version,
            });
        }
        if self.header.build_identity != build_identity() {
            return Err(CoreError::BuildIdentity {
                expected: build_identity().to_owned(),
                found: self.header.build_identity.clone(),
            });
        }

        let mut game = if self.mode == ModeKind::Adventure {
            Game::new(self.header.seed, self.scene)
        } else {
            Game::new_mode(self.header.seed, self.mode, self.level)
        };
        let events = self
            .frames
            .iter()
            .cloned()
            .map(|frame| game.advance(frame))
            .collect();
        Ok(ReplayOutcome {
            events,
            final_state: game.state.clone(),
            final_hash: game.snapshot_hash()?,
        })
    }
}

impl ReplayRecord {
    pub fn capture(replay: Replay) -> Result<Self, CoreError> {
        let outcome = replay.run()?;
        Ok(Self { replay, outcome })
    }

    pub fn verify(&self) -> Result<(), CoreError> {
        if self.replay.run()? == self.outcome {
            Ok(())
        } else {
            Err(CoreError::ReplayMismatch)
        }
    }

    pub fn to_json_pretty(&self) -> Result<Vec<u8>, CoreError> {
        Ok(serde_json::to_vec_pretty(self)?)
    }

    pub fn from_json(bytes: &[u8]) -> Result<Self, CoreError> {
        Ok(serde_json::from_slice(bytes)?)
    }
}

#[derive(Clone)]
struct Mt19937 {
    words: Vec<u32>,
    index: usize,
}

impl Mt19937 {
    const N: usize = 624;
    const M: usize = 397;

    fn new(seed: u64) -> Self {
        let seed = match seed as u32 {
            0 => 4_357,
            value => value,
        };
        let mut words = vec![0; Self::N];
        words[0] = seed;
        for index in 1..Self::N {
            words[index] = 1_812_433_253u32
                .wrapping_mul(words[index - 1] ^ (words[index - 1] >> 30))
                .wrapping_add(index as u32);
        }
        Self {
            words,
            index: Self::N,
        }
    }

    fn next(&mut self) -> u32 {
        if self.index >= Self::N {
            for index in 0..Self::N {
                let value = (self.words[index] & 0x8000_0000)
                    | (self.words[(index + 1) % Self::N] & 0x7fff_ffff);
                self.words[index] = self.words[(index + Self::M) % Self::N]
                    ^ (value >> 1)
                    ^ if value & 1 == 0 { 0 } else { 0x9908_b0df };
            }
            self.index = 0;
        }

        let mut value = self.words[self.index];
        self.index += 1;
        value ^= value >> 11;
        value ^= (value << 7) & 0x9d2c_5680;
        value ^= (value << 15) & 0xefc6_0000;
        value ^= value >> 18;
        value & 0x7fff_ffff
    }

    fn range(&mut self, range: u32) -> u32 {
        if range == 0 { 0 } else { self.next() % range }
    }

    fn range_inclusive(&mut self, minimum: u32, maximum: u32) -> u32 {
        minimum + self.range(maximum - minimum + 1)
    }

    fn fixed_range(&mut self, minimum: i64, maximum: i64) -> i64 {
        let span = maximum - minimum;
        minimum + i64::from(self.next()) * span / i64::from(0x7fff_ffffu32)
    }

    fn snapshot(&self) -> RngState {
        RngState {
            words: self.words.clone(),
            index: self.index as u16,
        }
    }
}

pub struct Game {
    state: GameState,
    rng: Mt19937,
}

impl Game {
    pub fn new(seed: u64, scene: SceneKind) -> Self {
        Self::new_with_mode(seed, ModeKind::Adventure, 0, scene)
    }

    pub fn new_mode(seed: u64, mode: ModeKind, level: u8) -> Self {
        Self::new_with_mode(seed, mode, level, mode_level_scene(mode, level))
    }

    #[doc(hidden)]
    pub fn debug_force_game_over(&mut self) {
        self.state.level_scene = self.state.scene;
        self.state.scene = SceneKind::GameOver;
    }

    #[doc(hidden)]
    pub fn debug_prepare_game_lost(&mut self) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.board.mowers.clear();
        self.state.board.zombies.clear();
        let mut setup_events = Vec::new();
        self.spawn_normal_zombie(2, 0, Some(-100 * POSITION_SCALE), &mut setup_events);
    }

    #[doc(hidden)]
    pub fn debug_prepare_game_won(&mut self) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.board.zombies.clear();
        self.state.board.wave.current = self.state.board.wave.total;
    }

    #[doc(hidden)]
    pub fn debug_prepare_pickups(&mut self) -> (EntityId, EntityId) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.board.suns.clear();
        self.state.board.coins.clear();
        let mut setup_events = Vec::new();
        self.spawn_sun_value(
            SunSource::Sky,
            NORMAL_SUN_VALUE,
            200 * POSITION_SCALE,
            200 * POSITION_SCALE,
            &mut setup_events,
        );
        self.spawn_pickup(
            CoinType::Silver,
            300 * POSITION_SCALE,
            200 * POSITION_SCALE,
            &mut setup_events,
        );
        (self.state.board.suns[0].id, self.state.board.coins[0].id)
    }

    #[doc(hidden)]
    pub fn debug_prepare_prize_chime(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.board.coins.clear();
        let mut events = Vec::new();
        self.spawn_pickup(
            CoinType::Diamond,
            300 * POSITION_SCALE,
            200 * POSITION_SCALE,
            &mut events,
        );
        events
    }

    #[doc(hidden)]
    pub fn debug_prepare_sun_production(&mut self) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 50;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.state.board.suns.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        self.state
            .board
            .plants
            .iter_mut()
            .find(|plant| plant.plant_type == PlantType::Sunflower)
            .expect("sun production checkpoint sunflower")
            .launch_counter = 1;
    }

    #[doc(hidden)]
    pub fn debug_prepare_plant_firing_audio(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Night;
        self.state.scene = SceneKind::Night;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.state.board.projectiles.clear();

        for (row, plant_type) in [
            (0, PlantType::Peashooter),
            (1, PlantType::Other(5)),
            (2, PlantType::Other(8)),
            (3, PlantType::Other(10)),
            (4, PlantType::Other(29)),
        ] {
            self.place_izombie_plant(plant_type, row, 2);
        }
        for plant in &mut self.state.board.plants {
            plant.launch_counter = plant.launch_rate;
            plant.shooting_counter = 2;
        }

        let mut setup_events = Vec::new();
        for row in 0..self.state.board.rows {
            self.spawn_normal_zombie(row, 0, Some(grid_x(7)), &mut setup_events);
        }
        for zombie in &mut self.state.board.zombies {
            zombie.speed = 0;
        }
        self.advance(InputFrame::default())
    }

    #[doc(hidden)]
    pub fn debug_prepare_ice_shroom(&mut self) {
        self.state.level_scene = SceneKind::Night;
        self.state.scene = SceneKind::Night;
        self.state.sun = 200;
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 14 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        self.state.board.plants[0].special_counter = 1;
        let center = grid_x(2);
        let mut setup_events = Vec::new();
        self.spawn_normal_zombie(0, 0, Some(center), &mut setup_events);
        self.spawn_normal_zombie(4, 0, Some(center), &mut setup_events);
    }

    #[doc(hidden)]
    pub fn debug_prepare_potato_mine(&mut self) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 50;
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 4 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        self.state.board.plants[0].special_counter = 1;
        let mut setup_events = Vec::new();
        self.spawn_normal_zombie(2, 0, Some(grid_x(2)), &mut setup_events);
    }

    #[doc(hidden)]
    pub fn debug_prepare_explosion_plants(&mut self) {
        self.state.level_scene = SceneKind::Night;
        self.state.scene = SceneKind::Night;
        self.state.sun = 500;
        self.state.board.zombies.clear();
        self.state.board.craters.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 2 },
                InputAction::Plant { row: 0, column: 0 },
                InputAction::SelectSeed { slot: 20 },
                InputAction::Plant { row: 1, column: 2 },
                InputAction::SelectSeed { slot: 15 },
                InputAction::Plant { row: 2, column: 4 },
            ],
        });
        for plant in &mut self.state.board.plants {
            plant.special_counter = 1;
        }
    }

    #[doc(hidden)]
    pub fn debug_prepare_explode_o_nut(&mut self) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 0;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.state.board.seed_packets = vec![SeedPacketState {
            slot: 0,
            plant_type: PlantType::Other(49),
            refresh_remaining: 0,
        }];
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        self.state.board.plants[0].health = ZOMBIE_BITE_DAMAGE;
        let mut setup_events = Vec::new();
        self.spawn_normal_zombie(
            2,
            0,
            Some(grid_x(2) + 30 * POSITION_SCALE),
            &mut setup_events,
        );
    }

    #[doc(hidden)]
    pub fn debug_prepare_squash(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 100;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 17 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        self.state.board.zombies.clear();
        let squash = self.state.board.plants[0].id;
        let mut setup_events = Vec::new();
        let target = self.spawn_normal_zombie(
            2,
            0,
            Some(grid_x(2) + 50 * POSITION_SCALE),
            &mut setup_events,
        );
        if let Some(plant) = self
            .state
            .board
            .plants
            .iter_mut()
            .find(|plant| plant.id == squash)
        {
            plant.special_target = Some(target);
            plant.special_armed = true;
            plant.special_counter = 1;
        }
        self.advance(InputFrame::default())
    }

    #[doc(hidden)]
    pub fn debug_prepare_squash_hum(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 100;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 17 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        self.state.board.zombies.clear();
        let mut setup_events = Vec::new();
        self.spawn_normal_zombie(
            2,
            0,
            Some(grid_x(2) + 50 * POSITION_SCALE),
            &mut setup_events,
        );
        self.advance(InputFrame::default())
    }

    #[doc(hidden)]
    pub fn debug_prepare_brain_finished(&mut self) {
        self.state.level_scene = SceneKind::Night;
        self.state.scene = SceneKind::Night;
        self.state.board.zombies.clear();
        if let Some(brain) = self.state.board.brains.first_mut() {
            brain.remaining = 1;
            brain.squished = false;
        }
        let mut setup_events = Vec::new();
        self.spawn_normal_zombie(0, 0, Some(-101 * POSITION_SCALE), &mut setup_events);
    }

    #[doc(hidden)]
    pub fn debug_prepare_imp_throw(&mut self) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.board.zombies.clear();
        let mut setup_events = Vec::new();
        let gargantuar =
            self.spawn_gargantuar_zombie(0, 0, Some(490 * POSITION_SCALE), &mut setup_events);
        if let Some(zombie) = self
            .state
            .board
            .zombies
            .iter_mut()
            .find(|zombie| zombie.id == gargantuar)
        {
            zombie.speed = 0;
            zombie.health = 1_499;
        }
    }

    #[doc(hidden)]
    pub fn debug_prepare_newspaper_rip(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.board.zombies.clear();
        let mut setup_events = Vec::new();
        let newspaper =
            self.spawn_newspaper_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup_events);
        let position_x = self
            .state
            .board
            .zombies
            .iter_mut()
            .find(|zombie| zombie.id == newspaper)
            .map(|zombie| {
                zombie.speed = 0;
                zombie.shield_health = ProjectileType::Pea.damage();
                zombie.position_x
            })
            .expect("newspaper checkpoint zombie exists");
        self.fire_projectile(
            0,
            ProjectileType::Pea,
            2,
            ProjectileTrajectory {
                motion: ProjectileMotion::Straight,
                position_x,
                position_y: grid_y(2),
                velocity_x: 0,
                velocity_y: 0,
            },
            &mut setup_events,
        );
        self.advance(InputFrame::default())
    }

    #[doc(hidden)]
    pub fn debug_prepare_butter(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.board.zombies.clear();
        let mut setup_events = Vec::new();
        let zombie = self.spawn_normal_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup_events);
        let position_x = self
            .state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == zombie)
            .map(|candidate| {
                candidate.speed = 0;
                candidate.position_x
            })
            .expect("butter checkpoint zombie exists");
        self.fire_projectile(
            0,
            ProjectileType::Butter,
            2,
            ProjectileTrajectory {
                motion: ProjectileMotion::Straight,
                position_x,
                position_y: grid_y(2),
                velocity_x: 0,
                velocity_y: 0,
            },
            &mut setup_events,
        );
        self.advance(InputFrame::default())
    }

    #[doc(hidden)]
    pub fn debug_prepare_projectile_impacts(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.state.board.projectiles.clear();

        let mut setup_events = Vec::new();
        self.spawn_normal_zombie(0, 0, Some(120 * POSITION_SCALE), &mut setup_events);
        self.spawn_normal_zombie(1, 0, Some(220 * POSITION_SCALE), &mut setup_events);
        self.spawn_normal_zombie(2, 0, Some(320 * POSITION_SCALE), &mut setup_events);
        self.spawn_normal_zombie(3, 0, Some(420 * POSITION_SCALE), &mut setup_events);
        self.spawn_normal_zombie(4, 0, Some(520 * POSITION_SCALE), &mut setup_events);
        self.spawn_buckethead_zombie(0, 0, Some(620 * POSITION_SCALE), &mut setup_events);
        self.spawn_conehead_zombie(1, 0, Some(620 * POSITION_SCALE), &mut setup_events);
        for zombie in &mut self.state.board.zombies {
            zombie.speed = 0;
        }

        for (projectile_type, row, position_x) in [
            (ProjectileType::Pea, 0, 120),
            (ProjectileType::Kernel, 1, 220),
            (ProjectileType::Butter, 2, 320),
            (ProjectileType::Melon, 3, 420),
            (ProjectileType::Fireball, 4, 520),
            (ProjectileType::Pea, 0, 620),
            (ProjectileType::Pea, 1, 620),
        ] {
            self.fire_projectile(
                0,
                projectile_type,
                row,
                ProjectileTrajectory {
                    motion: ProjectileMotion::Straight,
                    position_x: position_x * POSITION_SCALE,
                    position_y: grid_y(row),
                    velocity_x: 0,
                    velocity_y: 0,
                },
                &mut setup_events,
            );
        }
        self.advance(InputFrame::default())
    }

    #[doc(hidden)]
    pub fn debug_prepare_vase_break(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.mode = ModeKind::Vasebreaker;
        self.state.board.vases = vec![VaseState {
            id: 1,
            row: 2,
            column: 2,
            contents: VaseContents::Plant(PlantType::Peashooter),
            leaf: false,
        }];
        let mut events = Vec::new();
        self.break_vase(2, 2, &mut events);
        events
    }

    #[doc(hidden)]
    pub fn debug_prepare_rake(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.board.rake = Some((2, 2));
        self.state.board.zombies.clear();
        let mut setup_events = Vec::new();
        self.spawn_normal_zombie(2, 0, Some(grid_x(2)), &mut setup_events);
        self.advance(InputFrame::default())
    }

    #[doc(hidden)]
    pub fn debug_prepare_blover_chomper(&mut self) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 300;
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 27 },
                InputAction::Plant { row: 0, column: 0 },
                InputAction::SelectSeed { slot: 6 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let mut setup_events = Vec::new();
        let target = self.spawn_normal_zombie(
            2,
            0,
            Some(grid_x(2) + 30 * POSITION_SCALE),
            &mut setup_events,
        );
        for plant in &mut self.state.board.plants {
            match plant.plant_type {
                PlantType::Other(27) => plant.special_counter = 1,
                PlantType::Other(6) => {
                    plant.special_armed = true;
                    plant.special_target = Some(target);
                    plant.special_counter = 1;
                }
                _ => {}
            }
        }
    }

    #[doc(hidden)]
    pub fn debug_prepare_hypno_jackbox(&mut self) {
        self.state.level_scene = SceneKind::Night;
        self.state.scene = SceneKind::Night;
        self.state.sun = 75;
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 12 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let mut setup_events = Vec::new();
        self.spawn_normal_zombie(
            2,
            0,
            Some(grid_x(2) + 30 * POSITION_SCALE),
            &mut setup_events,
        );
        let jackbox =
            self.spawn_jackbox_zombie(0, 0, Some(780 * POSITION_SCALE), &mut setup_events);
        if let Some(zombie) = self
            .state
            .board
            .zombies
            .iter_mut()
            .find(|zombie| zombie.id == jackbox)
        {
            zombie.jackbox_timer = 1;
            zombie.speed = 0;
        }
    }

    #[doc(hidden)]
    pub fn debug_prepare_cob_cannon(&mut self) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 1_000;
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 34 },
                InputAction::Plant { row: 2, column: 1 },
            ],
        });
        self.state.board.seed_packets[34].refresh_remaining = 0;
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 34 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 47 },
                InputAction::Plant { row: 2, column: 1 },
            ],
        });
        if let Some(cob) = self
            .state
            .board
            .plants
            .iter_mut()
            .find(|plant| plant.plant_type == PlantType::Other(47))
        {
            cob.special_counter = 1;
            cob.special_armed = true;
        }
    }

    #[doc(hidden)]
    pub fn debug_prepare_portal(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.board.portals.clear();
        let mut events = Vec::new();
        self.place_portal(2, 5, true, &mut events);
        events
    }

    #[doc(hidden)]
    pub fn debug_prepare_gravebuster(&mut self) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 75;
        self.state.board.graves.clear();
        self.state.board.plants.clear();
        self.state
            .board
            .graves
            .push(GraveState { row: 2, column: 2 });
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 11 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        self.state.board.plants[0].special_counter = 1;
    }

    #[doc(hidden)]
    pub fn debug_prepare_coffee(&mut self) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 150;
        self.state.board.plants.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 10 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 35 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        if let Some(coffee) = self
            .state
            .board
            .plants
            .iter_mut()
            .find(|plant| plant.plant_type == PlantType::Other(35))
        {
            coffee.special_counter = 1;
        }
    }

    #[doc(hidden)]
    pub fn debug_prepare_tangle_kelp(&mut self) {
        self.state.level_scene = SceneKind::Pool;
        self.state.scene = SceneKind::Pool;
        self.state.sun = 25;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 19 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let mut setup_events = Vec::new();
        self.spawn_normal_zombie(
            2,
            0,
            Some(grid_x(2) + 30 * POSITION_SCALE),
            &mut setup_events,
        );
    }

    #[doc(hidden)]
    pub fn debug_prepare_dolphin_jump(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Pool;
        self.state.scene = SceneKind::Pool;
        self.state.sun = 100;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 24 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let mut setup_events = Vec::new();
        let dolphin = self.spawn_dolphin_rider_zombie(
            2,
            0,
            Some(grid_x(2) + 20 * POSITION_SCALE),
            &mut setup_events,
        );
        if let Some(zombie) = self
            .state
            .board
            .zombies
            .iter_mut()
            .find(|zombie| zombie.id == dolphin)
        {
            zombie.dolphin_phase = 1;
            zombie.speed = 0;
            zombie.age = 3;
        }
        setup_events
    }

    #[doc(hidden)]
    pub fn debug_prepare_pool_entry(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Pool;
        self.state.scene = SceneKind::Pool;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        let mut setup_events = Vec::new();
        self.spawn_normal_zombie(2, 0, Some(679 * POSITION_SCALE), &mut setup_events);
        self.advance(InputFrame::default())
    }

    #[doc(hidden)]
    pub fn debug_prepare_spikeweed(&mut self) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 100;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 21 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let mut setup_events = Vec::new();
        self.spawn_normal_zombie(
            2,
            0,
            Some(grid_x(2) + 40 * POSITION_SCALE),
            &mut setup_events,
        );
    }

    #[doc(hidden)]
    pub fn debug_prepare_digger(&mut self) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.board.zombies.clear();
        let mut setup_events = Vec::new();
        let digger = self.spawn_digger_zombie(2, 0, Some(5 * POSITION_SCALE), &mut setup_events);
        if let Some(zombie) = self
            .state
            .board
            .zombies
            .iter_mut()
            .find(|zombie| zombie.id == digger)
        {
            zombie.speed = 0;
        }
    }

    #[doc(hidden)]
    pub fn debug_prepare_magnet(&mut self) {
        self.state.level_scene = SceneKind::Night;
        self.state.scene = SceneKind::Night;
        self.state.sun = 1_000;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 31 },
                InputAction::Plant { row: 2, column: 4 },
            ],
        });
        let mut setup_events = Vec::new();
        let bucket =
            self.spawn_buckethead_zombie(2, 0, Some(520 * POSITION_SCALE), &mut setup_events);
        for zombie in &mut self.state.board.zombies {
            zombie.speed = 0;
        }
        debug_assert!(
            self.state
                .board
                .zombies
                .iter()
                .any(|zombie| zombie.id == bucket)
        );
    }

    #[doc(hidden)]
    pub fn debug_prepare_shield_hit(&mut self) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 100;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        if let Some(plant) = self.state.board.plants.first_mut() {
            plant.launch_counter = 0;
            plant.shooting_counter = 1;
        }
        let mut setup_events = Vec::new();
        let shielded = self.spawn_screen_door_zombie(
            2,
            0,
            Some(grid_x(2) + 100 * POSITION_SCALE),
            &mut setup_events,
        );
        if let Some(zombie) = self
            .state
            .board
            .zombies
            .iter_mut()
            .find(|zombie| zombie.id == shielded)
        {
            zombie.speed = 0;
            zombie.shield_health = 20;
        }
    }

    #[doc(hidden)]
    pub fn debug_prepare_zamboni(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 1_000;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 21 },
                InputAction::Plant { row: 2, column: 5 },
            ],
        });
        let mut setup_events = Vec::new();
        self.spawn_zamboni_zombie(2, 0, Some(grid_x(5)), &mut setup_events);
        setup_events
    }

    #[doc(hidden)]
    pub fn debug_prepare_catapult(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 100;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let mut setup_events = Vec::new();
        let catapult = self.spawn_catapult_zombie(2, 0, Some(grid_x(4)), &mut setup_events);
        if let Some(zombie) = self
            .state
            .board
            .zombies
            .iter_mut()
            .find(|zombie| zombie.id == catapult)
        {
            zombie.speed = 0;
            zombie.catapult_armed = true;
            zombie.catapult_counter = 1;
        }
        self.advance(InputFrame::default())
    }

    #[doc(hidden)]
    pub fn debug_prepare_balloon_appearance(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        let mut setup_events = Vec::new();
        self.spawn_balloon_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup_events);
        setup_events
    }

    #[doc(hidden)]
    pub fn debug_prepare_pogo_block(&mut self) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 1_000;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 23 },
                InputAction::Plant { row: 2, column: 5 },
            ],
        });
        let mut setup_events = Vec::new();
        self.spawn_pogo_zombie(
            2,
            0,
            Some(grid_x(5) + 40 * POSITION_SCALE),
            &mut setup_events,
        );
    }

    #[doc(hidden)]
    pub fn debug_prepare_pogo_bounce(&mut self) -> Vec<GameEvent> {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 1_000;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 3 },
                InputAction::Plant { row: 2, column: 5 },
            ],
        });
        let mut setup_events = Vec::new();
        self.spawn_pogo_zombie(
            2,
            0,
            Some(grid_x(5) + 40 * POSITION_SCALE),
            &mut setup_events,
        );
        setup_events
    }

    #[doc(hidden)]
    pub fn debug_prepare_umbrella_deflect(&mut self) {
        self.state.level_scene = SceneKind::Day;
        self.state.scene = SceneKind::Day;
        self.state.sun = 1_000;
        self.state.board.plants.clear();
        self.state.board.zombies.clear();
        self.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 37 },
                InputAction::Plant { row: 2, column: 3 },
            ],
        });
        let mut setup_events = Vec::new();
        let bungee = self.spawn_bungee_zombie(2, 0, None, &mut setup_events);
        for zombie in &mut self.state.board.zombies {
            if zombie.id == bungee {
                zombie.bungee_counter = 1;
            }
        }
    }

    fn new_with_mode(seed: u64, mode: ModeKind, level: u8, scene: SceneKind) -> Self {
        let mut rng = Mt19937::new(seed);
        let mut board = BoardState::new(scene, mode, level, &mut rng);
        let (total_waves, endless) = mode_wave_config(mode, level);
        board.wave.total = total_waves;
        board.wave.endless = endless;
        if mode == ModeKind::MiniGame {
            let countdown = match challenge_kind(level) {
                ChallengeKind::BobsledBonanza => 4_500,
                ChallengeKind::PogoParty => 5_500,
                _ => board.wave.countdown,
            };
            board.wave.countdown = countdown;
            board.wave.countdown_start = countdown;
        }
        if mode == ModeKind::Adventure && adventure_level_scene(level) == SceneKind::Night {
            // Board::AddGraveStones: per-column grave counts on random rows
            // (the Whack-a-Zombie nine-grave spread is approximated by
            // columns).
            let columns: &[(u8, u8)] = match level {
                11..=13 => &[(6, 1), (7, 1), (8, 2)],
                14 | 16 => &[(5, 1), (6, 1), (7, 2), (8, 3)],
                15 => &[(4, 2), (5, 2), (6, 2), (7, 2), (8, 1)],
                17..=19 => &[(4, 1), (5, 2), (6, 2), (7, 3), (8, 3)],
                20 => &[(3, 1), (4, 2), (5, 2), (6, 2), (7, 3), (8, 3)],
                _ => &[],
            };
            for (column, count) in columns {
                let mut rows: Vec<u8> = (0..board.rows).collect();
                for _ in 0..*count {
                    if rows.is_empty() {
                        break;
                    }
                    let pick = rng.range(rows.len() as u32) as usize;
                    let row = rows.remove(pick);
                    board.graves.push(GraveState {
                        row,
                        column: *column,
                    });
                }
            }
        }
        let mut challenge = initial_challenge_state(mode, level);
        if mode == ModeKind::MiniGame && is_conveyor_challenge(challenge.kind) {
            board.set_seed_packets(conveyor_initial_seeds(level));
            challenge.conveyor_countdown = conveyor_initial_countdown(level);
        } else if let Some(seed_bank) = fixed_seed_bank(mode, level) {
            board.set_seed_packets(seed_bank);
        }
        let garden_service =
            (mode == ModeKind::ZenGarden).then(|| GardenServiceKind::from_level(level));
        let state = GameState {
            seed,
            tick: 0,
            scene,
            level_scene: scene,
            mode,
            level,
            sun: if mode == ModeKind::IZombie {
                150
            } else if mode == ModeKind::MiniGame
                && challenge_kind(level) == ChallengeKind::LastStand
            {
                5_000
            } else if mode == ModeKind::Adventure && (1..=50).contains(&level) {
                adventure_starting_sun(level, true)
            } else {
                50
            },
            coins: 0,
            unlocked_modes: 0,
            chocolates: 0,
            pickup_inventory: Vec::new(),
            wave: 0,
            paused: false,
            garden_service,
            garden: garden_service.map(initial_garden_state).unwrap_or_default(),
            tree_height: 1,
            challenge,
            board,
            rng: rng.snapshot(),
        };
        let mut game = Self { state, rng };
        if mode == ModeKind::IZombie {
            game.initialize_izombie();
            game.state.rng = game.rng.snapshot();
        } else if mode == ModeKind::MiniGame
            && game.state.challenge.kind == ChallengeKind::Zombiquarium
        {
            game.initialize_zombiquarium();
            game.state.rng = game.rng.snapshot();
        } else if mode == ModeKind::MiniGame
            && game.state.challenge.kind == ChallengeKind::FinalBoss
        {
            let mut events = Vec::new();
            game.spawn_boss_zombie(0, 0, None, &mut events);
            game.state.rng = game.rng.snapshot();
        }
        game
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn advance(&mut self, input: InputFrame) -> Vec<GameEvent> {
        let mut events = Vec::new();
        let mut restarted = false;
        for action in input.actions {
            restarted |= matches!(action, InputAction::Restart);
            self.apply_input(action, &mut events);
        }

        if !restarted && !self.state.paused {
            if self.state.scene == SceneKind::Garden {
                self.update_garden();
                self.state.tick = self.state.tick.saturating_add(1);
                events.push(GameEvent::StateChanged);
            } else if self.is_playing_scene() {
                self.update_plants(&mut events);
                self.update_zombies(&mut events);
                self.update_mowers(&mut events);
                self.update_projectiles(&mut events);
                self.update_seed_packets(&mut events);
                // Board.cpp:1416: the sky-sun clock is only armed when the
                // stage is not a night stage.
                if self.state.mode != ModeKind::IZombie && !scene_is_night(self.state.scene) {
                    self.update_sun_spawning(&mut events);
                }
                self.update_wave_spawning(&mut events);
                self.update_challenge(&mut events);
                self.state.board.ice_counter = self.state.board.ice_counter.saturating_sub(1);
                self.update_craters();
                self.update_ice();
                self.update_suns();
                self.update_sky_drop(&mut events);
                self.state.tick = self.state.tick.saturating_add(1);
                self.state.wave = self.state.board.wave.current;

                let scary_potter = self.state.mode == ModeKind::Adventure && self.state.level == 35;
                if scary_potter
                    && self.state.board.scary_pot_stage < 2
                    && self.state.board.vases.is_empty()
                    && self.state.board.zombies.is_empty()
                {
                    self.state.board.scary_pot_stage += 1;
                    let stage = self.state.board.scary_pot_stage;
                    let first_id = self.state.board.next_entity_id;
                    let vases = scary_pot_stage_vases(stage, first_id, &mut self.rng);
                    self.state.board.next_entity_id =
                        first_id.saturating_add(u32::try_from(vases.len()).unwrap_or(0));
                    self.state.board.vases = vases;
                }
                let won = if scary_potter {
                    self.state.board.scary_pot_stage >= 2
                        && self.state.board.vases.is_empty()
                        && self.state.board.zombies.is_empty()
                } else if self.state.mode == ModeKind::Vasebreaker {
                    self.state.board.vases.is_empty() && self.state.board.zombies.is_empty()
                } else if self.state.mode == ModeKind::IZombie {
                    self.state.board.brains.iter().all(|brain| brain.squished)
                } else if self.state.mode == ModeKind::MiniGame {
                    match self.state.challenge.kind {
                        ChallengeKind::SlotMachine => self.state.sun >= self.state.challenge.target,
                        ChallengeKind::Beghouled => {
                            self.state.challenge.score >= self.state.challenge.target
                        }
                        ChallengeKind::Zombiquarium => {
                            self.state.sun >= self.state.challenge.target
                                && self.state.board.zombies.is_empty()
                        }
                        _ => {
                            self.state.board.wave.current >= self.state.board.wave.total
                                && self.state.board.zombies.is_empty()
                                && !self.state.board.wave.endless
                        }
                    }
                } else if self.state.mode == ModeKind::Survival {
                    if self.state.board.wave.current >= self.state.board.wave.total
                        && self.state.board.zombies.is_empty()
                    {
                        if !self.state.board.wave.endless
                            && self.state.challenge.stage.saturating_add(1)
                                < survival_stage_limit(self.state.level)
                        {
                            self.begin_survival_repick(&mut events);
                            false
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                } else {
                    self.state.board.wave.current >= self.state.board.wave.total
                        && self.state.board.zombies.is_empty()
                        && !self.state.board.wave.endless
                };
                if won {
                    self.state.scene = SceneKind::Complete;
                    events.push(GameEvent::GameWon);
                    // Zombie::TrySpawnLevelAward: adventure completion drops
                    // the level award (the trophy on 5-10, a seed packet
                    // elsewhere; item levels keep their award identity in
                    // adventure_award).
                    if self.state.mode == ModeKind::Adventure
                        && (1..=50).contains(&self.state.level)
                    {
                        let coin_type = if self.state.level == 50 {
                            CoinType::AwardSilverSunflower
                        } else {
                            CoinType::FinalSeedPacket
                        };
                        self.spawn_pickup(coin_type, grid_x(4), grid_y(2), &mut events);
                    }
                }
                events.push(GameEvent::StateChanged);
            }
        }

        self.state.rng = self.rng.snapshot();
        events
    }

    pub fn snapshot_hash(&self) -> Result<StateHash, CoreError> {
        let bytes = serde_json::to_vec(&self.state)?;
        let mut digest = Sha256::new();
        digest.update(bytes);
        Ok(StateHash(digest.finalize().into()))
    }

    fn is_playing_scene(&self) -> bool {
        matches!(
            self.state.scene,
            SceneKind::Day
                | SceneKind::Night
                | SceneKind::Pool
                | SceneKind::Fog
                | SceneKind::Roof
                | SceneKind::Boss
        )
    }

    fn begin_survival_repick(&mut self, events: &mut Vec<GameEvent>) {
        self.state.challenge.stage = self.state.challenge.stage.saturating_add(1);
        self.state.scene = SceneKind::SeedChooser;
        self.state.board.selected_seed = None;
        self.state.board.zombies.clear();
        self.state.board.projectiles.clear();
        self.state.board.suns.clear();
        self.state.board.coins.clear();
        self.state.board.wave.current = 0;
        self.state.board.wave.countdown = FIRST_WAVE_COUNTDOWN;
        self.state.board.wave.countdown_start = FIRST_WAVE_COUNTDOWN;
        self.state.wave = 0;
        events.push(GameEvent::SurvivalRepickStarted {
            stage: self.state.challenge.stage,
        });
    }

    fn confirm_survival_repick(&mut self, events: &mut Vec<GameEvent>) {
        let action = InputAction::ConfirmSurvivalRepick;
        if self.state.mode != ModeKind::Survival || self.state.scene != SceneKind::SeedChooser {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::ChallengeUnavailable,
            });
            return;
        }
        self.state.scene = self.state.level_scene;
        events.push(GameEvent::SurvivalStageStarted {
            stage: self.state.challenge.stage,
        });
    }

    fn has_conveyor_seed_bank(&self) -> bool {
        self.state.mode == ModeKind::MiniGame && is_conveyor_challenge(self.state.challenge.kind)
    }

    fn add_conveyor_seed(&mut self, plant_type: PlantType) {
        if self.state.board.seed_packets.len() >= CONVEYOR_SEED_SLOTS {
            return;
        }
        let slot = u8::try_from(self.state.board.seed_packets.len())
            .expect("conveyor seed slot must fit in u8");
        self.state.board.seed_packets.push(SeedPacketState {
            slot,
            plant_type,
            refresh_remaining: 0,
        });
    }

    fn remove_conveyor_seed(&mut self, packet_index: usize) {
        self.state.board.seed_packets.remove(packet_index);
        for (slot, packet) in self.state.board.seed_packets.iter_mut().enumerate() {
            packet.slot = u8::try_from(slot).expect("conveyor seed slot must fit in u8");
        }
    }

    fn update_conveyor_belt(&mut self) {
        let kind = self.state.challenge.kind;
        self.state.challenge.conveyor_countdown =
            self.state.challenge.conveyor_countdown.saturating_sub(1);
        if self.state.challenge.conveyor_countdown != 0 {
            return;
        }

        let seed_count = self.state.board.seed_packets.len();
        self.state.challenge.conveyor_countdown = conveyor_interval(kind, seed_count);
        let pool = conveyor_seed_pool(self.state.level);
        if pool.is_empty() {
            return;
        }

        let mut weights = [0u32; 8];
        for (index, (plant_type, weight)) in pool.iter().copied().enumerate() {
            let on_conveyor = self
                .state
                .board
                .seed_packets
                .iter()
                .filter(|packet| packet.plant_type == plant_type)
                .count();
            let on_board = self
                .state
                .board
                .plants
                .iter()
                .filter(|plant| plant.plant_type == plant_type)
                .count();
            let total_count =
                u32::try_from(on_conveyor.saturating_add(on_board)).unwrap_or(u32::MAX);
            let mut adjusted = weight;
            if plant_type.is_gravebuster()
                && self.state.board.graves.len() <= on_conveyor + on_board
            {
                adjusted = 0;
            } else if plant_type.slot() == 16 {
                adjusted = conveyor_curve(adjusted, total_count, 18);
            } else if plant_type.slot() == 33 {
                let limit = if kind == ChallengeKind::Column {
                    45
                } else {
                    35
                };
                adjusted = conveyor_curve(adjusted, total_count, limit);
            }
            if pool.len() > 2 {
                if on_conveyor >= 4 {
                    adjusted = 1;
                } else if on_conveyor >= 3 {
                    adjusted = 5;
                } else if self.state.challenge.last_conveyor_seed == Some(plant_type) {
                    adjusted /= 2;
                }
            }
            weights[index] = adjusted;
        }

        let total_weight = weights[..pool.len()].iter().sum();
        let mut pick = self.rng.range(total_weight);
        let plant_type = pool
            .iter()
            .zip(weights)
            .find_map(|((plant_type, _), weight)| {
                if pick < weight {
                    Some(*plant_type)
                } else {
                    pick -= weight;
                    None
                }
            })
            .unwrap_or(pool[0].0);
        self.add_conveyor_seed(plant_type);
        self.state.challenge.last_conveyor_seed = Some(plant_type);
    }

    fn update_garden(&mut self) {
        for plant in &mut self.state.garden.plants {
            plant.age_ticks = plant.age_ticks.saturating_add(1);
        }
    }

    fn update_challenge(&mut self, events: &mut Vec<GameEvent>) {
        if self.state.mode != ModeKind::MiniGame {
            return;
        }
        let kind = self.state.challenge.kind;
        if is_conveyor_challenge(kind) {
            self.update_conveyor_belt();
            return;
        }
        if self.state.challenge.countdown == 0 {
            return;
        }
        self.state.challenge.countdown -= 1;
        if self.state.challenge.countdown == 0 {
            let next_countdown = match kind {
                ChallengeKind::RainingSeeds => 500 + self.rng.range(500),
                ChallengeKind::WhackAZombie => 200,
                ChallengeKind::Beghouled => 1_500,
                _ => 0,
            };
            self.state.challenge.countdown = next_countdown;
            if kind == ChallengeKind::RainingSeeds {
                events.push(GameEvent::ChallengeAction {
                    kind,
                    value: self.rng.range(53),
                });
            }
        }
    }

    fn initialize_zombiquarium(&mut self) {
        let mut events = Vec::new();
        for _ in 0..2 {
            let row = self.rng.range(u32::from(self.state.board.rows)) as u8;
            let position = self.rng.fixed_range(80, 650) * POSITION_SCALE;
            self.spawn_normal_zombie(row, 0, Some(position), &mut events);
        }
    }

    fn initialize_izombie(&mut self) {
        // Challenge::IZombieInitLevel in the 1.0.0.1051 source lays out enemy
        // plants separately for each numbered stage. Random picks use the same
        // game RNG stream as the rest of the board, so the layout remains a
        // deterministic function of the replay seed.
        match self.state.level {
            0 => {
                self.place_izombie_plant(PlantType::Sunflower, 2, 3);
                self.place_izombie_plant(PlantType::Sunflower, 3, 3);
                self.place_izombie_plants(PlantType::Sunflower, 7, None);
                self.place_izombie_plants(PlantType::Other(17), 3, None);
                self.place_izombie_plants(PlantType::Peashooter, 6, None);
                self.place_izombie_plants(PlantType::Other(5), 2, None);
            }
            1 => {
                self.place_izombie_plant(PlantType::Other(21), 0, 3);
                self.place_izombie_plant(PlantType::Sunflower, 0, 2);
                self.place_izombie_plant(PlantType::Sunflower, 3, 3);
                self.place_izombie_plants(PlantType::Other(21), 1, Some(0));
                self.place_izombie_plants(PlantType::Peashooter, 1, Some(0));
                self.place_izombie_plants(PlantType::Other(5), 2, Some(3));
                self.place_izombie_plants(PlantType::Sunflower, 1, Some(3));
                self.place_izombie_plants(PlantType::Sunflower, 4, None);
                self.place_izombie_plants(PlantType::Other(21), 2, None);
                self.place_izombie_plants(PlantType::Other(5), 2, None);
                self.place_izombie_plants(PlantType::Peashooter, 4, None);
            }
            2 => {
                self.place_izombie_plant(PlantType::Other(4), 0, 3);
                self.place_izombie_plant(PlantType::Sunflower, 0, 2);
                self.place_izombie_plant(PlantType::Other(4), 2, 2);
                self.place_izombie_plant(PlantType::Sunflower, 4, 2);
                self.place_izombie_plant(PlantType::Other(22), 3, 3);
                self.place_izombie_plants(PlantType::Other(22), 2, None);
                self.place_izombie_plants(PlantType::Sunflower, 5, None);
                self.place_izombie_plants(PlantType::Peashooter, 7, None);
                self.place_izombie_plants(PlantType::Other(28), 1, None);
            }
            3 => {
                for row in 0..5 {
                    self.place_izombie_plant(PlantType::Other(3), row, 3);
                }
                for (row, column) in [(0, 2), (2, 2), (4, 2)] {
                    self.place_izombie_plant(PlantType::Sunflower, row, column);
                }
                self.place_izombie_plants(PlantType::Peashooter, 1, Some(0));
                self.place_izombie_plants(PlantType::Other(5), 1, Some(1));
                self.place_izombie_plants(PlantType::Other(10), 2, Some(2));
                self.place_izombie_plants(PlantType::Other(5), 1, Some(3));
                self.place_izombie_plants(PlantType::Peashooter, 1, Some(4));
                self.place_izombie_plants(PlantType::Peashooter, 2, None);
                self.place_izombie_plants(PlantType::Sunflower, 4, None);
            }
            4 => {
                self.place_izombie_plant(PlantType::Sunflower, 2, 3);
                self.place_izombie_plant(PlantType::Sunflower, 3, 3);
                self.place_izombie_plants(PlantType::Other(26), 1, Some(1));
                self.place_izombie_plants(PlantType::Other(26), 1, Some(4));
                self.place_izombie_plants(PlantType::Other(31), 1, None);
                self.place_izombie_plants(PlantType::Sunflower, 5, None);
                self.place_izombie_plants(PlantType::Peashooter, 8, None);
                self.place_izombie_plants(PlantType::Other(5), 2, None);
            }
            5 => {
                self.place_izombie_plant(PlantType::Other(36), 1, 4);
                self.place_izombie_plant(PlantType::Other(36), 3, 4);
                self.place_izombie_plants(PlantType::Sunflower, 3, Some(1));
                self.place_izombie_plants(PlantType::Sunflower, 3, Some(3));
                self.place_izombie_plants(PlantType::Other(22), 2, None);
                self.place_izombie_plants(PlantType::Sunflower, 2, None);
                self.place_izombie_plants(PlantType::Other(21), 3, None);
                self.place_izombie_plants(PlantType::Other(5), 1, None);
                self.place_izombie_plants(PlantType::Peashooter, 5, None);
                self.place_izombie_plants(PlantType::Other(17), 2, None);
                self.place_izombie_plants(PlantType::Other(34), 2, None);
            }
            6 => {
                self.place_izombie_plant(PlantType::Sunflower, 2, 4);
                self.place_izombie_plant(PlantType::Sunflower, 4, 4);
                self.place_izombie_plants(PlantType::Sunflower, 6, None);
                self.place_izombie_plants(PlantType::Other(4), 9, None);
                self.place_izombie_plants(PlantType::Other(6), 8, None);
            }
            7 => {
                self.place_izombie_plants(PlantType::Other(3), 3, None);
                self.place_izombie_plants(PlantType::Other(31), 2, None);
                self.place_izombie_plants(PlantType::Peashooter, 8, None);
                self.place_izombie_plants(PlantType::Other(17), 2, None);
                self.place_izombie_plants(PlantType::Other(4), 2, None);
                self.place_izombie_plants(PlantType::Sunflower, 8, None);
            }
            8 => {
                self.place_izombie_plant(PlantType::Other(23), 1, 5);
                self.place_izombie_plant(PlantType::Other(22), 3, 5);
                self.place_izombie_plants(PlantType::Other(4), 4, Some(0));
                self.place_izombie_plants(PlantType::Sunflower, 2, Some(0));
                self.place_izombie_plants(PlantType::Sunflower, 2, Some(1));
                self.place_izombie_plants(PlantType::Other(18), 1, Some(1));
                self.place_izombie_plants(PlantType::Other(5), 1, Some(1));
                self.place_izombie_plants(PlantType::Other(28), 1, Some(1));
                self.place_izombie_plants(PlantType::Other(6), 3, Some(2));
                self.place_izombie_plants(PlantType::Sunflower, 2, Some(2));
                self.place_izombie_plants(PlantType::Other(17), 1, Some(2));
                self.place_izombie_plants(PlantType::Peashooter, 3, Some(3));
                self.place_izombie_plants(PlantType::Sunflower, 2, Some(3));
                self.place_izombie_plants(PlantType::Sunflower, 1, Some(4));
                self.place_izombie_plants(PlantType::Other(10), 1, Some(4));
                self.place_izombie_plants(PlantType::Other(13), 1, Some(4));
                self.place_izombie_plants(PlantType::Other(29), 1, Some(4));
                self.place_izombie_plants(PlantType::Other(28), 1, Some(4));
                self.place_izombie_plants(PlantType::Other(31), 1, Some(4));
            }
            _ => self.initialize_izombie_endless(),
        }

        for packet in &mut self.state.board.seed_packets {
            packet.refresh_remaining = 0;
        }
        self.state.sun = 150;
        self.state.board.selected_seed = None;
    }

    fn initialize_izombie_endless(&mut self) {
        self.place_izombie_plants(PlantType::Sunflower, 8, None);

        // The current endless stage starts at zero, so the source skips its
        // later-stage formation branch and chooses one of these three 17-cell
        // formations after the eight initial sunflowers.
        match self.rng.range(6) {
            0..=2 => {
                self.place_izombie_plants(PlantType::Other(3), 1, None);
                self.place_izombie_plants(PlantType::Other(22), 1, None);
                self.place_izombie_plants(PlantType::Other(4), 1, None);
                self.place_izombie_plants(PlantType::Other(6), 2, None);
                self.place_izombie_plants(PlantType::Peashooter, 1, None);
                self.place_izombie_plants(PlantType::Other(28), 1, None);
                self.place_izombie_plants(PlantType::Other(34), 1, None);
                self.place_izombie_plants(PlantType::Other(18), 1, None);
                self.place_izombie_plants(PlantType::Other(5), 1, None);
                self.place_izombie_plants(PlantType::Other(17), 1, None);
                self.place_izombie_plants(PlantType::Other(10), 1, None);
                self.place_izombie_plants(PlantType::Other(37), 1, None);
                self.place_izombie_plants(PlantType::Other(29), 1, None);
                self.place_izombie_plants(PlantType::Other(31), 1, None);
                self.place_izombie_plants(PlantType::Other(21), 2, None);
            }
            3..=4 => {
                self.place_izombie_plants(PlantType::Other(22), 1, None);
                self.place_izombie_plants(PlantType::Other(28), 3, None);
                self.place_izombie_plants(PlantType::Other(7), 1, None);
                self.place_izombie_plants(PlantType::Other(34), 3, None);
                self.place_izombie_plants(PlantType::Other(18), 1, None);
                self.place_izombie_plants(PlantType::Other(5), 3, None);
                self.place_izombie_plants(PlantType::Other(37), 1, None);
                self.place_izombie_plants(PlantType::Other(31), 1, None);
                self.place_izombie_plants(PlantType::Other(21), 3, None);
            }
            _ => {
                self.place_izombie_plants(PlantType::Other(4), 4, None);
                self.place_izombie_plants(PlantType::Other(6), 3, None);
                self.place_izombie_plants(PlantType::Other(17), 3, None);
                self.place_izombie_plants(PlantType::Other(10), 4, None);
                self.place_izombie_plants(PlantType::Other(21), 3, None);
            }
        }
    }

    fn place_izombie_plant(&mut self, plant_type: PlantType, row: u8, column: u8) {
        if row >= self.state.board.rows
            || column >= self.state.board.columns
            || self
                .state
                .board
                .plants
                .iter()
                .any(|plant| plant.row == row && plant.column == column)
        {
            return;
        }
        let id = self.state.board.allocate_entity();
        let launch_rate = plant_type.launch_rate();
        self.state.board.plants.push(PlantState {
            id,
            plant_type,
            imitater_type: None,
            row,
            column,
            health: plant_type.max_health(),
            max_health: plant_type.max_health(),
            launch_counter: 0,
            launch_rate,
            shooting_counter: 0,
            burst_remaining: 0,
            burst_delay: 0,
            production_age: 0,
            production_stage: 0,
            special_counter: 0,
            special_armed: plant_type.is_potato_mine(),
            special_target: None,
            blink_counter: 0,
            asleep: false,
            wake_up_counter: 0,
        });
    }

    fn place_izombie_plants(&mut self, plant_type: PlantType, count: u8, row: Option<u8>) {
        let columns = izombie_columns(self.state.level);
        let occupied = self
            .state
            .board
            .plants
            .iter()
            .map(|plant| (plant.row, plant.column))
            .collect::<Vec<_>>();
        let mut cells = Vec::new();
        for column in 0..columns {
            for cell_row in 0..self.state.board.rows {
                if row.is_some_and(|required| required != cell_row)
                    || occupied.contains(&(cell_row, column))
                    || (matches!(plant_type.slot(), 3 | 22) && columns.saturating_sub(column) > 3)
                {
                    continue;
                }
                cells.push((cell_row, column));
            }
        }
        // ponytail: bounded 6x5 setup; remove one chosen cell per placement until
        // a larger board justifies a swap-pool helper.
        for _ in 0..count {
            if cells.is_empty() {
                break;
            }
            let cell = self.rng.range(u32::try_from(cells.len()).unwrap_or(1)) as usize;
            let (cell_row, column) = cells.remove(cell);
            self.place_izombie_plant(plant_type, cell_row, column);
        }
    }

    fn apply_input(&mut self, action: InputAction, events: &mut Vec<GameEvent>) {
        match action {
            InputAction::Pause => {
                if !self.state.paused {
                    self.state.paused = true;
                    events.push(GameEvent::Paused);
                }
            }
            InputAction::Resume => {
                if self.state.paused {
                    self.state.paused = false;
                    events.push(GameEvent::Resumed);
                }
            }
            InputAction::Restart => {
                if matches!(self.state.scene, SceneKind::GameOver | SceneKind::Complete) {
                    let seed = self.state.seed;
                    let scene = self.state.level_scene;
                    *self = if self.state.mode == ModeKind::Adventure {
                        Self::new(seed, scene)
                    } else {
                        Self::new_mode(seed, self.state.mode, self.state.level)
                    };
                    events.push(GameEvent::Restarted);
                }
            }
            InputAction::SelectSeed { slot } => self.select_seed(slot, events),
            InputAction::Plant { row, column } => self.plant(row, column, events),
            InputAction::PlantImitater {
                plant_slot,
                row,
                column,
            } => self.plant_imitater(plant_slot, row, column, events),
            InputAction::FireCobCannon {
                entity,
                row,
                column,
            } => self.fire_cob_cannon(entity, row, column, events),
            InputAction::Shovel { row, column } => self.shovel(row, column, events),
            InputAction::BreakVase { row, column } => self.break_vase(row, column, events),
            InputAction::DeployZombie {
                zombie_type,
                row,
                column,
            } => self.deploy_zombie(zombie_type, row, column, events),
            InputAction::GardenWater { plant } => self.garden_water(plant, events),
            InputAction::GardenFertilize { plant } => self.garden_fertilize(plant, events),
            InputAction::GardenFulfillNeed { plant } => self.garden_fulfill_need(plant, events),
            InputAction::GardenFeedTree => self.garden_feed_tree(events),
            InputAction::GardenLeave => self.garden_leave(events),
            InputAction::ChallengeSpin => self.challenge_spin(events),
            InputAction::ChallengeMatch { length } => self.challenge_match(length, events),
            InputAction::ChallengeFeed { x, y } => self.challenge_feed(x, y, events),
            InputAction::ChallengeWhack { row, column } => {
                self.challenge_whack(row, column, events)
            }
            InputAction::CollectSun { entity } => self.collect_sun(entity, events),
            InputAction::CollectCoin { entity } => self.collect_coin(entity, events),
            InputAction::ConfirmSurvivalRepick => self.confirm_survival_repick(events),
        }
    }

    fn garden_water(&mut self, plant: u8, events: &mut Vec<GameEvent>) {
        let action = InputAction::GardenWater { plant };
        if self.state.scene != SceneKind::Garden
            || self.state.garden_service.is_none()
            || self.state.garden.plants.get(usize::from(plant)).is_none()
        {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::InvalidGardenTarget,
            });
            return;
        }
        let target = &mut self.state.garden.plants[usize::from(plant)];
        target.watered = true;
        events.push(GameEvent::GardenWatered {
            plant,
            age_ticks: target.age_ticks,
        });
    }

    fn garden_fertilize(&mut self, plant: u8, events: &mut Vec<GameEvent>) {
        let action = InputAction::GardenFertilize { plant };
        if self.state.scene != SceneKind::Garden
            || self.state.garden_service.is_none()
            || self.state.garden.plants.get(usize::from(plant)).is_none()
        {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::InvalidGardenTarget,
            });
            return;
        }
        let target = &mut self.state.garden.plants[usize::from(plant)];
        target.age_ticks = target.age_ticks.saturating_add(100);
        target.watered = true;
        events.push(GameEvent::GardenFertilized {
            plant,
            age_ticks: target.age_ticks,
        });
    }

    fn garden_fulfill_need(&mut self, plant: u8, events: &mut Vec<GameEvent>) {
        let action = InputAction::GardenFulfillNeed { plant };
        if self.state.scene != SceneKind::Garden
            || self.state.garden_service.is_none()
            || self.state.garden.plants.get(usize::from(plant)).is_none()
        {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::InvalidGardenTarget,
            });
            return;
        }

        let target = &mut self.state.garden.plants[usize::from(plant)];
        if target.happy {
            return;
        }
        target.happy = true;
        events.push(GameEvent::GardenBecameHappy {
            plant,
            aquatic: matches!(target.plant_type.slot(), 16 | 19 | 24 | 43),
        });
    }

    fn garden_feed_tree(&mut self, events: &mut Vec<GameEvent>) {
        if self.state.scene != SceneKind::Garden
            || self.state.garden_service != Some(GardenServiceKind::TreeOfWisdom)
        {
            events.push(GameEvent::InputRejected {
                action: InputAction::GardenFeedTree,
                reason: InputRejectReason::InvalidGardenTarget,
            });
            return;
        }
        self.state.tree_height = self.state.tree_height.saturating_add(1).min(50);
        events.push(GameEvent::GardenTreeGrew {
            height: self.state.tree_height,
        });
    }

    fn garden_leave(&mut self, events: &mut Vec<GameEvent>) {
        if self.state.scene != SceneKind::Garden {
            events.push(GameEvent::InputRejected {
                action: InputAction::GardenLeave,
                reason: InputRejectReason::InvalidGardenTarget,
            });
            return;
        }
        self.state.scene = SceneKind::AdventureSelect;
        events.push(GameEvent::GardenLeft);
    }

    fn challenge_spin(&mut self, events: &mut Vec<GameEvent>) {
        if self.state.challenge.kind != ChallengeKind::SlotMachine || self.state.sun < 25 {
            events.push(GameEvent::InputRejected {
                action: InputAction::ChallengeSpin,
                reason: InputRejectReason::ChallengeUnavailable,
            });
            return;
        }
        self.state.sun -= 25;
        let symbol = self.rng.range(3);
        events.push(GameEvent::ChallengeAction {
            kind: ChallengeKind::SlotMachine,
            value: symbol,
        });
    }

    fn challenge_match(&mut self, length: u8, events: &mut Vec<GameEvent>) {
        if self.state.challenge.kind != ChallengeKind::Beghouled || length < 3 {
            events.push(GameEvent::InputRejected {
                action: InputAction::ChallengeMatch { length },
                reason: InputRejectReason::ChallengeUnavailable,
            });
            return;
        }
        let points = u32::from(length.saturating_sub(2));
        self.state.challenge.score = self
            .state
            .challenge
            .score
            .saturating_add(points)
            .min(self.state.challenge.target);
        events.push(GameEvent::ChallengeAction {
            kind: ChallengeKind::Beghouled,
            value: points,
        });
    }

    fn challenge_feed(&mut self, x: u16, y: u16, events: &mut Vec<GameEvent>) {
        if self.state.challenge.kind != ChallengeKind::Zombiquarium
            || !(80..=720).contains(&x)
            || !(90..=430).contains(&y)
            || self.state.sun < 5
        {
            events.push(GameEvent::InputRejected {
                action: InputAction::ChallengeFeed { x, y },
                reason: InputRejectReason::ChallengeUnavailable,
            });
            return;
        }
        self.state.sun -= 5;
        let row = self.rng.range(u32::from(self.state.board.rows)) as u8;
        let position = self.rng.fixed_range(80, 650) * POSITION_SCALE;
        self.spawn_normal_zombie(row, 0, Some(position), events);
        events.push(GameEvent::ChallengeAction {
            kind: ChallengeKind::Zombiquarium,
            value: 5,
        });
    }

    fn challenge_whack(&mut self, row: u8, column: u8, events: &mut Vec<GameEvent>) {
        let action = InputAction::ChallengeWhack { row, column };
        if self.state.challenge.kind != ChallengeKind::WhackAZombie
            || row >= self.state.board.rows
            || column >= self.state.board.columns
        {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::ChallengeUnavailable,
            });
            return;
        }
        let center = grid_x(column);
        let Some(index) = self.state.board.zombies.iter().position(|zombie| {
            zombie.health > 0
                && zombie.row == row
                && (zombie.position_x - center).abs() <= 80 * POSITION_SCALE
        }) else {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::MissingEntity,
            });
            return;
        };
        let entity = self.state.board.zombies[index].id;
        self.state.board.zombies[index].health = 0;
        self.emit_zombie_died(entity, events);
        self.state.board.zombies.remove(index);
        self.state.challenge.score = self.state.challenge.score.saturating_add(1);
        events.push(GameEvent::ChallengeAction {
            kind: ChallengeKind::WhackAZombie,
            value: 1,
        });
    }

    fn select_seed(&mut self, slot: u8, events: &mut Vec<GameEvent>) {
        let action = InputAction::SelectSeed { slot };
        let Some(packet) = self
            .state
            .board
            .seed_packets
            .iter()
            .find(|packet| packet.slot == slot)
        else {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::InvalidSlot,
            });
            return;
        };
        if self.state.challenge.kind == ChallengeKind::SlotMachine {
            self.challenge_spin(events);
            return;
        }
        if packet.refresh_remaining != 0 {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::SeedRefreshing,
            });
            return;
        }
        if !packet.plant_type.is_plant() {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::InvalidSlot,
            });
            return;
        }

        self.state.board.selected_seed = Some(slot);
        events.push(GameEvent::SeedSelected {
            slot,
            plant_type: packet.plant_type,
        });
    }

    fn deploy_zombie(
        &mut self,
        zombie_type: ZombieType,
        row: u8,
        column: u8,
        events: &mut Vec<GameEvent>,
    ) {
        let action = InputAction::DeployZombie {
            zombie_type,
            row,
            column,
        };
        if self.state.mode != ModeKind::IZombie
            || row >= self.state.board.rows
            || column >= izombie_columns(self.state.level)
        {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::InvalidTerrain,
            });
            return;
        }
        if !self.state.board.zombie_packets.contains(&zombie_type) {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::InvalidSlot,
            });
            return;
        }
        // Plant::GetCost in 1.0.0.1051 prices the I, Zombie seed cards;
        // types without a source card keep their previous fallback values.
        let cost = match zombie_type {
            ZombieType::Normal | ZombieType::Flag | ZombieType::Imp => 50,
            ZombieType::Conehead | ZombieType::PoleVaulter => 75,
            ZombieType::ScreenDoor => 100,
            ZombieType::Buckethead
            | ZombieType::Newspaper
            | ZombieType::Digger
            | ZombieType::Bungee => 125,
            ZombieType::Ladder | ZombieType::Balloon | ZombieType::Jackbox => 150,
            ZombieType::Bobsled => 150,
            ZombieType::Football | ZombieType::Zamboni => 175,
            ZombieType::Pogo => 200,
            ZombieType::Dancer => 350,
            ZombieType::Gargantuar | ZombieType::Gigagargantuar => 300,
            _ => 100,
        };
        if self.state.sun < cost {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::NotEnoughSun,
            });
            return;
        }
        self.state.sun -= cost;
        let health = match zombie_type {
            // I, Zombie imps use the source 70-HP override; everywhere else imps are 270.
            ZombieType::Imp => 70,
            ZombieType::Conehead => 640,
            ZombieType::Buckethead => 1_370,
            ZombieType::ScreenDoor => 270,
            ZombieType::Football => 1_670,
            ZombieType::Digger => 370,
            ZombieType::Bungee => 450,
            ZombieType::PoleVaulter | ZombieType::Pogo | ZombieType::Dancer => 500,
            ZombieType::Ladder => LADDER_HEALTH,
            ZombieType::Bobsled => BOBSLED_HEALTH,
            ZombieType::Gargantuar => 3_000,
            ZombieType::Gigagargantuar => GIGAGARGANTUAR_HEALTH,
            ZombieType::JalapenoHead => ZOMBOTANY_JALAPENO_HEALTH,
            ZombieType::Boss => {
                if self.state.mode == ModeKind::Adventure {
                    BOSS_ADVENTURE_HEALTH
                } else {
                    BOSS_CHALLENGE_HEALTH
                }
            }
            _ => 270,
        };
        let position = Some(grid_x(column) - 30 * POSITION_SCALE);
        let entity = if zombie_type == ZombieType::Bobsled {
            self.spawn_bobsled_zombie(row, 0, position, events)
        } else {
            self._spawn_zombie_inner(zombie_type, health, row, 0, position, events)
        };
        events.push(GameEvent::ZombieDeployed {
            entity,
            zombie_type,
            row,
            column,
            sun_remaining: self.state.sun,
        });
    }

    fn plant(&mut self, row: u8, column: u8, events: &mut Vec<GameEvent>) {
        let action = InputAction::Plant { row, column };
        let Some(slot) = self.state.board.selected_seed else {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::NoSeedSelected,
            });
            return;
        };
        let Some(packet) = self
            .state
            .board
            .seed_packets
            .iter()
            .find(|packet| packet.slot == slot)
        else {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::InvalidSlot,
            });
            return;
        };
        let plant_type = packet.plant_type;
        if !plant_type.is_plant() {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::InvalidSlot,
            });
            return;
        }
        if plant_type.is_cob_cannon() {
            self.plant_cob_cannon(row, column, events);
            return;
        }
        if plant_type.is_imitater() {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::InvalidSlot,
            });
            return;
        }
        self.plant_selected(action, row, column, plant_type, None, events);
    }

    fn plant_cob_cannon(&mut self, row: u8, column: u8, events: &mut Vec<GameEvent>) {
        let action = InputAction::Plant { row, column };
        let Some(right_column) = column.checked_add(1) else {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::OutsideBoard,
            });
            return;
        };
        if row >= self.state.board.rows || right_column >= self.state.board.columns {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::OutsideBoard,
            });
            return;
        }

        let kernel_slot = 34;
        let left_index = self.state.board.plants.iter().position(|plant| {
            plant.row == row && plant.column == column && plant.plant_type.slot() == kernel_slot
        });
        let right_index = self.state.board.plants.iter().position(|plant| {
            plant.row == row
                && plant.column == right_column
                && plant.plant_type.slot() == kernel_slot
        });
        let (Some(left_index), Some(right_index)) = (left_index, right_index) else {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::Occupied,
            });
            return;
        };
        let has_support = |column: u8, slot: u8| {
            self.state.board.plants.iter().any(|plant| {
                plant.row == row && plant.column == column && plant.plant_type.slot() == slot
            })
        };
        let left_lilypad = has_support(column, 16);
        let right_lilypad = has_support(right_column, 16);
        let left_flowerpot = has_support(column, 33);
        let right_flowerpot = has_support(right_column, 33);
        let valid_support = match self.state.scene {
            SceneKind::Pool => left_lilypad && right_lilypad,
            SceneKind::Roof => left_flowerpot && right_flowerpot,
            _ => left_lilypad == right_lilypad && left_flowerpot == right_flowerpot,
        };
        let has_pumpkin = self.state.board.plants.iter().any(|plant| {
            plant.row == row
                && (plant.column == column || plant.column == right_column)
                && plant.plant_type.slot() == 30
        });
        if !valid_support || has_pumpkin {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::InvalidTerrain,
            });
            return;
        }

        let cannon_type = PlantType::Other(47);
        if self.state.sun < cannon_type.cost() {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::NotEnoughSun,
            });
            return;
        }
        let packet_index = self
            .state
            .board
            .seed_packets
            .iter()
            .position(|packet| packet.slot == cannon_type.slot())
            .expect("selected CobCannon packet must exist");
        self.state.sun -= cannon_type.cost();
        self.state.board.selected_seed = None;
        self.state.board.seed_packets[packet_index].refresh_remaining =
            cannon_type.refresh_time() + 1;

        let consumed = self.state.board.plants[right_index].id;
        self.state.board.plants.remove(right_index);
        let left_index = if left_index > right_index {
            left_index - 1
        } else {
            left_index
        };
        let cannon_id = self.state.board.plants[left_index].id;
        let cannon_health = cannon_type.max_health();
        let blink_counter = 400 + self.rng.range(400);
        let cannon = &mut self.state.board.plants[left_index];
        cannon.plant_type = cannon_type;
        cannon.imitater_type = None;
        cannon.health = cannon_health;
        cannon.max_health = cannon_health;
        cannon.launch_counter = 0;
        cannon.launch_rate = 0;
        cannon.shooting_counter = 0;
        cannon.burst_remaining = 0;
        cannon.burst_delay = 0;
        cannon.production_age = 0;
        cannon.production_stage = 0;
        cannon.special_counter = COB_ARM_TICKS;
        cannon.special_armed = false;
        cannon.special_target = None;
        cannon.blink_counter = blink_counter;
        cannon.asleep = false;
        cannon.wake_up_counter = 0;
        events.push(GameEvent::PlantCombined {
            entity: cannon_id,
            consumed,
            plant_type: cannon_type,
        });
        events.push(GameEvent::PlantPlaced {
            entity: cannon_id,
            plant_type: cannon_type,
            row,
            column,
            sun_remaining: self.state.sun,
        });
    }

    fn plant_imitater(&mut self, plant_slot: u8, row: u8, column: u8, events: &mut Vec<GameEvent>) {
        let action = InputAction::PlantImitater {
            plant_slot,
            row,
            column,
        };
        if self.state.board.selected_seed != Some(48) {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::NoSeedSelected,
            });
            return;
        }
        let Some(target) = PlantType::from_slot(plant_slot).filter(|target| !target.is_imitater())
        else {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::InvalidSlot,
            });
            return;
        };
        self.plant_selected(
            action,
            row,
            column,
            PlantType::Other(48),
            Some(target),
            events,
        );
    }

    fn fire_cob_cannon(
        &mut self,
        entity: EntityId,
        target_row: u8,
        target_column: u8,
        events: &mut Vec<GameEvent>,
    ) {
        let action = InputAction::FireCobCannon {
            entity,
            row: target_row,
            column: target_column,
        };
        if target_row >= self.state.board.rows || target_column >= self.state.board.columns {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::OutsideBoard,
            });
            return;
        }
        let Some(index) = self
            .state
            .board
            .plants
            .iter()
            .position(|plant| plant.id == entity && plant.plant_type.is_cob_cannon())
        else {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::MissingEntity,
            });
            return;
        };
        if !self.state.board.plants[index].special_armed {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::NotReady,
            });
            return;
        }
        let source_row = self.state.board.plants[index].row;
        let source_column = self.state.board.plants[index].column;
        self.state.board.plants[index].special_armed = false;
        self.state.board.plants[index].special_counter = COB_RELOAD_TICKS;
        self.fire_cob_projectile(
            entity,
            source_row,
            source_column,
            target_row,
            target_column,
            events,
        );
        events.push(GameEvent::CobCannonFired {
            entity,
            target_row,
            target_column,
        });
    }

    fn plant_selected(
        &mut self,
        action: InputAction,
        row: u8,
        column: u8,
        plant_type: PlantType,
        imitater_type: Option<PlantType>,
        events: &mut Vec<GameEvent>,
    ) {
        let effective_type = imitater_type.unwrap_or(plant_type);
        if row >= self.state.board.rows || column >= self.state.board.columns {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::OutsideBoard,
            });
            return;
        }
        if self.state.mode == ModeKind::Adventure && !adventure_row_is_sodded(self.state.level, row)
        {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::InvalidTerrain,
            });
            return;
        }
        let has_lilypad = self.state.board.plants.iter().any(|plant| {
            plant.row == row && plant.column == column && plant.plant_type.slot() == 16
        });
        let has_flowerpot = self.state.board.plants.iter().any(|plant| {
            plant.row == row && plant.column == column && plant.plant_type.slot() == 33
        });
        let aquatic = matches!(effective_type.slot(), 16 | 19 | 24);
        let valid_terrain = match self.state.scene {
            SceneKind::Pool => {
                if aquatic {
                    true
                } else {
                    effective_type.slot() == 43 || has_lilypad
                }
            }
            SceneKind::Roof => effective_type.slot() == 33 || has_flowerpot,
            _ => !aquatic && effective_type.slot() != 33,
        };
        if !valid_terrain {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::InvalidTerrain,
            });
            return;
        }
        let has_grave = self
            .state
            .board
            .graves
            .iter()
            .any(|grave| grave.row == row && grave.column == column);
        if effective_type.slot() == 11 && !has_grave {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::InvalidTerrain,
            });
            return;
        }
        let occupied = self
            .state
            .board
            .plants
            .iter()
            .any(|plant| plant.row == row && plant.column == column);
        let has_top_plant = self.state.board.plants.iter().any(|plant| {
            plant.row == row
                && plant.column == column
                && !matches!(plant.plant_type.slot(), 16 | 33)
        });
        let coffee_target = self
            .state
            .board
            .plants
            .iter()
            .rfind(|plant| {
                plant.row == row
                    && plant.column == column
                    && !matches!(plant.plant_type.slot(), 16 | 33)
            })
            .is_some_and(|plant| {
                plant.plant_type.is_nocturnal() && plant.asleep && plant.wake_up_counter == 0
            });
        let pumpkin_already_present = self.state.board.plants.iter().any(|plant| {
            plant.row == row && plant.column == column && plant.plant_type.slot() == 30
        });
        let has_magnet_shroom = self.state.board.plants.iter().any(|plant| {
            plant.row == row && plant.column == column && plant.plant_type.slot() == 31
        });
        let gold_magnet_already_present = self.state.board.plants.iter().any(|plant| {
            plant.row == row && plant.column == column && plant.plant_type.slot() == 45
        });
        if effective_type.slot() == 45 && !has_magnet_shroom {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::Occupied,
            });
            return;
        }
        if effective_type.slot() == 35 && !coffee_target {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::Occupied,
            });
            return;
        }
        let can_layer_on_support = ((has_lilypad || has_flowerpot)
            && !has_top_plant
            && !matches!(effective_type.slot(), 16 | 19 | 24 | 33))
            || (effective_type.slot() == 45 && has_magnet_shroom && !gold_magnet_already_present)
            || (effective_type.slot() == 35 && coffee_target);
        if occupied
            && ((effective_type.slot() == 30 && pumpkin_already_present)
                || (effective_type.slot() != 30 && !can_layer_on_support))
        {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::Occupied,
            });
            return;
        }
        if self
            .state
            .board
            .craters
            .iter()
            .any(|crater| crater.row == row && crater.column == column)
        {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::Crater,
            });
            return;
        }
        if self.is_ice_at(row, column) {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::Ice,
            });
            return;
        }

        let packet_slot = self
            .state
            .board
            .selected_seed
            .expect("planting requires a selected seed packet");
        let packet_index = self
            .state
            .board
            .seed_packets
            .iter()
            .position(|packet| packet.slot == packet_slot)
            .expect("selected seed packet must exist");
        let conveyor = self.has_conveyor_seed_bank();
        if !conveyor && self.state.sun < effective_type.cost() {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::NotEnoughSun,
            });
            return;
        }

        if !conveyor {
            self.state.sun -= effective_type.cost();
        }
        self.state.board.selected_seed = None;
        if conveyor {
            self.remove_conveyor_seed(packet_index);
        } else {
            self.state.board.seed_packets[packet_index].refresh_remaining =
                effective_type.refresh_time() + 1;
        }
        let id = self.state.board.allocate_entity();

        // Preserve the original gameplay RNG stream even before render state consumes these values.
        let _frame_length = self.rng.range_inclusive(12, 18);
        let _body_animation_rate = self.rng.next();
        let blink_counter = 400 + self.rng.range(400);
        if effective_type == PlantType::Peashooter {
            let _head_animation_rate = self.rng.next();
        }
        let launch_rate = effective_type.launch_rate();
        let launch_counter = if launch_rate == 0 {
            0
        } else if effective_type.is_producer() {
            self.rng.range_inclusive(300, launch_rate / 2)
        } else {
            self.rng.range_inclusive(0, launch_rate)
        };
        let max_health = effective_type.max_health();
        let asleep = effective_type.is_nocturnal() && !scene_is_night(self.state.scene);
        let (special_counter, special_armed) = if imitater_type.is_some() {
            (IMITATER_MORPH_TICKS, false)
        } else if effective_type.is_instant_coffee() {
            (COFFEE_WAKE_TICKS, false)
        } else if effective_type.is_gravebuster() {
            (GRAVEBUSTER_EAT_TICKS, false)
        } else if effective_type.is_blover() {
            (BLOVER_SPECIAL_COUNTDOWN, false)
        } else if effective_type.is_cherry_bomb()
            || effective_type.is_jalapeno()
            || effective_type.is_ice_shroom()
            || effective_type.is_doom_shroom()
        {
            (INSTANT_PLANT_COUNTDOWN, false)
        } else if effective_type.is_potato_mine() {
            (POTATO_ARM_TICKS, false)
        } else {
            (0, false)
        };
        self.state.board.plants.push(PlantState {
            id,
            plant_type,
            imitater_type,
            row,
            column,
            health: max_health,
            max_health,
            launch_counter,
            launch_rate,
            shooting_counter: 0,
            burst_remaining: 0,
            burst_delay: 0,
            production_age: 0,
            production_stage: 0,
            special_counter,
            special_armed,
            special_target: None,
            blink_counter,
            asleep,
            wake_up_counter: 0,
        });
        events.push(GameEvent::PlantPlaced {
            entity: id,
            plant_type,
            row,
            column,
            sun_remaining: self.state.sun,
        });
    }

    fn morph_imitater(&mut self, index: usize, target: PlantType, events: &mut Vec<GameEvent>) {
        let entity = self.state.board.plants[index].id;
        let _frame_length = self.rng.range_inclusive(12, 18);
        let _body_animation_rate = self.rng.next();
        let blink_counter = 400 + self.rng.range(400);
        if target == PlantType::Peashooter {
            let _head_animation_rate = self.rng.next();
        }
        let launch_rate = target.launch_rate();
        let launch_counter = if launch_rate == 0 {
            0
        } else if target.is_producer() {
            self.rng.range_inclusive(300, launch_rate / 2)
        } else {
            self.rng.range_inclusive(0, launch_rate)
        };
        let asleep = target.is_nocturnal() && !scene_is_night(self.state.scene);
        let (special_counter, special_armed) = if target.is_cob_cannon() {
            (COB_ARM_TICKS, false)
        } else if target.is_instant_coffee() {
            (COFFEE_WAKE_TICKS, false)
        } else if target.is_gravebuster() {
            (GRAVEBUSTER_EAT_TICKS, false)
        } else if target.is_blover() {
            (BLOVER_SPECIAL_COUNTDOWN, false)
        } else if target.is_cherry_bomb()
            || target.is_jalapeno()
            || target.is_ice_shroom()
            || target.is_doom_shroom()
        {
            (INSTANT_PLANT_COUNTDOWN, false)
        } else if target.is_potato_mine() {
            (POTATO_ARM_TICKS, false)
        } else {
            (0, false)
        };
        let plant = &mut self.state.board.plants[index];
        plant.plant_type = target;
        plant.health = target.max_health();
        plant.max_health = target.max_health();
        plant.launch_counter = launch_counter;
        plant.launch_rate = launch_rate;
        plant.shooting_counter = 0;
        plant.burst_remaining = 0;
        plant.burst_delay = 0;
        plant.production_age = 0;
        plant.production_stage = 0;
        plant.special_counter = special_counter;
        plant.special_armed = special_armed;
        plant.special_target = None;
        plant.blink_counter = blink_counter;
        plant.asleep = asleep;
        plant.wake_up_counter = 0;
        events.push(GameEvent::ImitaterMorphed {
            entity,
            plant_type: target,
        });
    }

    fn shovel(&mut self, row: u8, column: u8, events: &mut Vec<GameEvent>) {
        let action = InputAction::Shovel { row, column };
        let Some(index) = self
            .state
            .board
            .plants
            .iter()
            .rposition(|plant| plant.row == row && plant.column == column)
        else {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::MissingEntity,
            });
            return;
        };
        let entity = self.state.board.plants.remove(index).id;
        self.state
            .board
            .ladders
            .retain(|ladder| ladder.row != row || ladder.column != column);
        events.push(GameEvent::PlantShoveled { entity });
    }

    fn break_vase(&mut self, row: u8, column: u8, events: &mut Vec<GameEvent>) {
        let action = InputAction::BreakVase { row, column };
        let Some(index) = self
            .state
            .board
            .vases
            .iter()
            .position(|vase| vase.row == row && vase.column == column)
        else {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::NoVase,
            });
            return;
        };
        let vase = self.state.board.vases.remove(index);
        events.push(GameEvent::VaseBroken {
            entity: vase.id,
            row: vase.row,
            column: vase.column,
        });
        events.push(GameEvent::VaseRevealed {
            entity: vase.id,
            row: vase.row,
            column: vase.column,
            contents: vase.contents,
            leaf: vase.leaf,
        });
        match vase.contents {
            VaseContents::Plant(_) => {}
            VaseContents::Zombie(zombie_type) => {
                self.spawn_vase_zombie(zombie_type, vase.row, vase.column, events);
            }
            VaseContents::Sun(value) => {
                self.spawn_sun_value(
                    SunSource::Plant(vase.id),
                    u32::from(value) * SMALL_SUN_VALUE,
                    grid_x(vase.column),
                    grid_y(vase.row),
                    events,
                );
            }
        }
    }

    fn collect_sun(&mut self, entity: EntityId, events: &mut Vec<GameEvent>) {
        let action = InputAction::CollectSun { entity };
        let Some(index) = self
            .state
            .board
            .suns
            .iter()
            .position(|sun| sun.id == entity)
        else {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::MissingEntity,
            });
            return;
        };
        let sun = self.state.board.suns.remove(index);
        self.state.sun = self.state.sun.saturating_add(sun.value).min(MAX_SUN);
        events.push(GameEvent::SunCollected {
            entity,
            value: sun.value,
            sun_total: self.state.sun,
        });
    }

    fn collect_coin(&mut self, entity: EntityId, events: &mut Vec<GameEvent>) {
        let action = InputAction::CollectCoin { entity };
        let Some(index) = self
            .state
            .board
            .coins
            .iter()
            .position(|coin| coin.id == entity)
        else {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::MissingEntity,
            });
            return;
        };
        let coin = self.state.board.coins.remove(index);
        let mut value = coin.value;
        if coin.coin_type.is_sun() {
            value = coin.coin_type.sun_value();
            self.state.sun = self.state.sun.saturating_add(value).min(MAX_SUN);
        } else if coin.coin_type.is_money() {
            self.state.coins = self.state.coins.saturating_add(value);
        } else if coin.coin_type.award_value() != 0 {
            value = coin.coin_type.award_value();
            self.state.coins = self.state.coins.saturating_add(value);
        } else if coin.coin_type.unlock_mask() != 0 {
            self.state.unlocked_modes |= coin.coin_type.unlock_mask();
        } else if matches!(
            coin.coin_type,
            CoinType::Chocolate | CoinType::AwardChocolate
        ) {
            self.state.chocolates = self.state.chocolates.saturating_add(1);
            value = 1;
        } else if matches!(
            coin.coin_type,
            CoinType::PresentPlant | CoinType::AwardPresent
        ) {
            self.state.garden.plants.push(GardenPlant {
                plant_type: coin.plant_type.unwrap_or(PlantType::Peashooter),
                age_ticks: 0,
                watered: false,
                happy: false,
            });
            value = 1;
        } else if coin.coin_type == CoinType::UsableSeedPacket {
            if let Some(plant_type) = coin.usable_seed_type {
                self.state.board.selected_seed = Some(plant_type.slot());
            } else {
                self.state.pickup_inventory.push(coin.coin_type);
            }
            value = 1;
        } else {
            self.state.pickup_inventory.push(coin.coin_type);
            value = if coin.coin_type.is_level_award() {
                1
            } else {
                coin.value.max(1)
            };
        }
        events.push(GameEvent::PickupCollected {
            entity,
            coin_type: coin.coin_type,
            value,
            coins_total: self.state.coins,
            sun_total: self.state.sun,
        });
        if coin.coin_type.is_money() {
            events.push(GameEvent::CoinCollected {
                entity,
                coin_type: coin.coin_type,
                value: coin.value,
                coin_total: self.state.coins,
            });
        }
    }

    fn update_plants(&mut self, events: &mut Vec<GameEvent>) {
        let plant_count = self.state.board.plants.len();
        for index in 0..plant_count {
            if index >= self.state.board.plants.len() {
                break;
            }
            let asleep = {
                let plant = &mut self.state.board.plants[index];
                if plant.wake_up_counter > 0 {
                    plant.wake_up_counter -= 1;
                    if plant.wake_up_counter == 0 {
                        plant.asleep = false;
                    }
                }
                plant.asleep
            };
            if asleep {
                continue;
            }
            let (id, plant_type, row, column, imitater_type) = {
                let plant = &self.state.board.plants[index];
                (
                    plant.id,
                    plant.plant_type,
                    plant.row,
                    plant.column,
                    plant.imitater_type,
                )
            };
            if plant_type.slot() == 31 {
                self.update_magnet_shroom(index, id, row, column, events);
                continue;
            }
            if plant_type.is_imitater() {
                let Some(target) = imitater_type else {
                    continue;
                };
                let morph = {
                    let plant = &mut self.state.board.plants[index];
                    plant.special_counter = plant.special_counter.saturating_sub(1);
                    plant.special_counter == 0
                };
                if morph {
                    self.morph_imitater(index, target, events);
                }
                continue;
            }
            let has_target = self.state.board.zombies.iter().any(|zombie| {
                if zombie.health <= 0 {
                    return false;
                }
                // Only Cactus (slot 26) and homing Cattail spikes target fliers.
                let targets_fliers =
                    plant_type.slot() == 26 || plant_type.firing_pattern() == FiringPattern::Homing;
                if (balloon_is_airborne(zombie) || zombie.imp_flight_ticks > 0) && !targets_fliers {
                    return false;
                }
                let row_distance = zombie.row.abs_diff(row);
                match plant_type.firing_pattern() {
                    FiringPattern::ThreeRow | FiringPattern::Star => row_distance <= 2,
                    FiringPattern::Split => {
                        row_distance == 0
                            && (zombie.position_x > plant_attack_start(column)
                                || zombie.position_x < grid_x(column))
                    }
                    FiringPattern::Homing => true,
                    FiringPattern::Backward => {
                        row_distance == 0 && zombie.position_x < grid_x(column)
                    }
                    _ if plant_type.is_scaredy_shroom() => {
                        row_distance == 0
                            && zombie.position_x > plant_attack_start(column)
                            && (zombie.position_x - grid_x(column)).abs()
                                > SCAREDY_THREAT_RADIUS * POSITION_SCALE
                    }
                    _ if plant_type.is_gloom_shroom() => {
                        row_distance <= GLOOM_ROW_RADIUS
                            && zombie.position_x > grid_x(column) - 80 * POSITION_SCALE
                            && zombie.position_x
                                < grid_x(column) + (GLOOM_ATTACK_RANGE - 80) * POSITION_SCALE
                    }
                    _ if plant_type.is_fume_shroom() => {
                        row_distance == 0
                            && zombie.position_x > plant_attack_start(column)
                            && zombie.position_x
                                < plant_attack_start(column) + FUME_ATTACK_RANGE * POSITION_SCALE
                    }
                    _ if plant_type.is_puff_range_shooter() => {
                        row_distance == 0
                            && zombie.position_x > plant_attack_start(column)
                            && zombie.position_x
                                < plant_attack_start(column) + PUFF_ATTACK_RANGE * POSITION_SCALE
                    }
                    _ => row_distance == 0 && zombie.position_x > plant_attack_start(column),
                }
            });
            let chomper_target = if plant_type.is_chomper() {
                self.find_chomper_target(row, column)
            } else {
                None
            };
            let squash_target = if plant_type.is_squash() {
                self.find_squash_target(row, column)
            } else {
                None
            };
            let tangle_target = if plant_type.is_tangle_kelp() {
                self.find_chomper_target(row, column)
            } else {
                None
            };
            let potato_trigger = plant_type.is_potato_mine()
                && self.state.board.zombies.iter().any(|zombie| {
                    zombie.health > 0
                        && zombie.row == row
                        && (zombie.position_x - grid_x(column)).abs() <= 60 * POSITION_SCALE
                });
            let spikeweed_target = plant_type.is_spikeweed()
                && self.state.board.zombies.iter().any(|zombie| {
                    zombie.health > 0
                        && zombie.row == row
                        && spikeweed_hits(zombie.position_x, column)
                });
            let gold_magnet_target = if plant_type.is_gold_magnet() {
                self.state
                    .board
                    .coins
                    .iter()
                    .min_by_key(|coin| {
                        (coin.position_x - grid_x(column)).abs()
                            + (coin.position_y - grid_y(row)).abs()
                    })
                    .map(|coin| coin.id)
            } else {
                None
            };

            let mut fire = false;
            let mut produce_suns = 0;
            let mut produce_value = 25;
            let mut produce_coin = false;
            let mut special = false;
            let mut spikeweed_hit = false;
            let mut spikeweed_started = false;
            let mut chomper_bite_target = None;
            let mut squash_hit_target = None;
            let mut squash_landed = false;
            let mut squash_finished = false;
            let mut tangle_grab_target = None;
            let mut tangle_started = false;
            let mut tangle_water_entry = false;
            let mut gold_magnet_coin = None;
            let mut potato_armed_now = false;
            let mut squash_hum_started = false;
            {
                let plant = &mut self.state.board.plants[index];
                if plant_type.is_cob_cannon() {
                    plant.special_counter = plant.special_counter.saturating_sub(1);
                    if plant.special_counter == 0 {
                        plant.special_armed = true;
                    }
                } else if plant_type.is_gold_magnet() {
                    if plant.special_counter > 0 {
                        plant.special_counter -= 1;
                    } else if let Some(target) = gold_magnet_target
                        && self.rng.range(50) == 0
                    {
                        gold_magnet_coin = Some(target);
                        plant.special_counter = self
                            .rng
                            .range_inclusive(GOLD_MAGNET_RECHARGE_MIN, GOLD_MAGNET_RECHARGE_MAX);
                    }
                } else if plant_type.is_instant_coffee() || plant_type.is_gravebuster() {
                    plant.special_counter = plant.special_counter.saturating_sub(1);
                    special = plant.special_counter == 0;
                } else if plant_type.is_blover() {
                    if !plant.special_armed {
                        plant.special_counter = plant.special_counter.saturating_sub(1);
                        if plant.special_counter == 0 {
                            plant.special_armed = true;
                            special = true;
                        }
                    }
                } else if plant_type.is_chomper() {
                    if plant.special_armed {
                        plant.special_counter = plant.special_counter.saturating_sub(1);
                        if plant.special_counter == 0 {
                            chomper_bite_target = plant.special_target.take();
                            plant.special_armed = false;
                            plant.special_counter = CHOMPER_CHEW_TICKS;
                        }
                    } else if plant.special_counter > 0 {
                        plant.special_counter -= 1;
                    } else if let Some(target) = chomper_target {
                        plant.special_armed = true;
                        plant.special_target = Some(target);
                        plant.special_counter = CHOMPER_BITE_WINDUP_TICKS;
                    }
                } else if plant_type.is_cherry_bomb()
                    || plant_type.is_jalapeno()
                    || plant_type.is_ice_shroom()
                    || plant_type.is_doom_shroom()
                {
                    plant.special_counter = plant.special_counter.saturating_sub(1);
                    special = plant.special_counter == 0;
                } else if plant_type.is_potato_mine() {
                    if plant.special_armed {
                        special = potato_trigger;
                    } else {
                        plant.special_counter = plant.special_counter.saturating_sub(1);
                        if plant.special_counter == 0 {
                            plant.special_armed = true;
                            // PARTICLE_POTATO_MINE_RISE anchor.
                            potato_armed_now = true;
                        }
                    }
                } else if plant_type.is_squash() {
                    if plant.special_armed {
                        plant.special_counter = plant.special_counter.saturating_sub(1);
                        if plant.special_counter == 0 {
                            if let Some(target) = plant.special_target.take() {
                                squash_hit_target = Some(target);
                                plant.special_counter = SQUASH_LANDING_HIT_TICKS;
                            } else if matches!(self.state.scene, SceneKind::Pool | SceneKind::Fog)
                                && matches!(row, 2 | 3)
                            {
                                plant.health = 0;
                                squash_finished = true;
                            } else {
                                plant.special_armed = false;
                                plant.special_counter = SQUASH_DONE_FALLING_TICKS;
                                squash_landed = true;
                            }
                        }
                    } else if plant.special_target.is_some() {
                        plant.special_counter = plant.special_counter.saturating_sub(1);
                        if plant.special_counter == 0 {
                            plant.special_armed = true;
                            plant.special_counter = SQUASH_HIT_DELAY_TICKS;
                        }
                    } else if plant.special_counter > 0 {
                        // A target-less, unarmed Squash is STATE_SQUASH_DONE_FALLING.
                        plant.special_counter -= 1;
                        if plant.special_counter == 0 {
                            plant.health = 0;
                            squash_finished = true;
                        }
                    } else if let Some(target) = squash_target {
                        plant.special_target = Some(target);
                        plant.special_counter = SQUASH_LOOK_TICKS;
                        squash_hum_started = true;
                    }
                } else if plant_type.is_tangle_kelp() {
                    if plant.special_armed {
                        tangle_water_entry = plant.special_counter == 20;
                        plant.special_counter = plant.special_counter.saturating_sub(1);
                        if plant.special_counter == 0 {
                            tangle_grab_target = plant.special_target.take();
                            plant.health = 0;
                        }
                    } else if let Some(target) = tangle_target {
                        plant.special_armed = true;
                        plant.special_counter = TANGLE_KELP_GRAB_TICKS;
                        plant.special_target = Some(target);
                        tangle_started = true;
                    }
                } else if plant_type.is_spikeweed() {
                    if plant.special_armed {
                        plant.special_counter = plant.special_counter.saturating_sub(1);
                        let damage_countdown = if plant_type.slot() == 46 {
                            matches!(plant.special_counter, 69 | 33)
                        } else {
                            plant.special_counter == SPIKEWEED_DAMAGE_COUNTDOWN
                        };
                        if damage_countdown {
                            spikeweed_hit = true;
                        } else if plant.special_counter == 0 {
                            plant.special_armed = false;
                        }
                    } else if spikeweed_target {
                        plant.special_armed = true;
                        plant.special_counter = SPIKEWEED_ATTACK_TICKS;
                        spikeweed_started = true;
                    }
                }
                if plant_type.is_sunshroom() {
                    plant.production_age = plant.production_age.saturating_add(1);
                    if plant.production_age >= SUNSHROOM_GROWTH_TICKS {
                        plant.production_stage = 1;
                    }
                }
                if plant.burst_remaining > 0 {
                    if plant.burst_delay > 0 {
                        plant.burst_delay -= 1;
                    }
                    if plant.burst_delay == 0 {
                        fire = true;
                        plant.burst_remaining -= 1;
                        if plant.burst_remaining > 0 {
                            plant.burst_delay = 5;
                        }
                    }
                } else if plant.shooting_counter > 0 {
                    plant.shooting_counter -= 1;
                    fire = plant.shooting_counter == 1;
                    if fire {
                        plant.shooting_counter = 0;
                        plant.burst_remaining = plant_type.burst_count().saturating_sub(1);
                        plant.burst_delay = if plant.burst_remaining == 0 { 0 } else { 5 };
                    }
                }

                if plant.launch_rate != 0
                    && plant.launch_counter <= 1
                    && plant.shooting_counter == 0
                    && plant.burst_remaining == 0
                {
                    if plant_type.is_producer() {
                        plant.launch_counter = self
                            .rng
                            .range_inclusive(plant.launch_rate - 150, plant.launch_rate);
                        if plant_type.is_marigold() {
                            produce_coin = true;
                        } else {
                            produce_suns = if plant_type.is_twin_sunflower() { 2 } else { 1 };
                            if plant_type.is_sunshroom() && plant.production_stage == 0 {
                                produce_value = SMALL_SUN_VALUE;
                            }
                        }
                    } else if plant_type.is_shooter() {
                        plant.launch_counter = plant.launch_rate - self.rng.range(15);
                        if has_target {
                            plant.shooting_counter = 33;
                        }
                    }
                } else if plant.launch_counter > 0 {
                    plant.launch_counter -= 1;
                }
            }

            if squash_hum_started {
                events.push(GameEvent::SquashHumStarted {
                    entity: id,
                    variant: self.rng.range(3) as u8,
                });
            }
            if fire {
                self.fire_projectiles(id, plant_type, row, column, events);
            }
            if let Some(coin_id) = gold_magnet_coin {
                self.collect_coin(coin_id, events);
            }
            if let Some(zombie_id) = chomper_bite_target
                && let Some(zombie_index) = self
                    .state
                    .board
                    .zombies
                    .iter()
                    .position(|zombie| zombie.id == zombie_id && zombie.health > 0)
            {
                self.emit_zombie_died(zombie_id, events);
                self.state.board.zombies.remove(zombie_index);
                events.push(GameEvent::PlantSpecialTriggered {
                    entity: id,
                    plant_type,
                });
            }
            if let Some(zombie_id) = squash_hit_target {
                if let Some(zombie_index) = self
                    .state
                    .board
                    .zombies
                    .iter()
                    .position(|zombie| zombie.id == zombie_id && zombie.health > 0)
                {
                    self.damage_zombie(zombie_index, PLANT_SPECIAL_DAMAGE, events);
                    let health_remaining = self.state.board.zombies[zombie_index].health;
                    events.push(GameEvent::PlantSpecialHit {
                        plant: id,
                        zombie: zombie_id,
                        damage: PLANT_SPECIAL_DAMAGE,
                        health_remaining,
                    });
                    if health_remaining <= 0 {
                        self.emit_zombie_died(zombie_id, events);
                        self.state.board.zombies.remove(zombie_index);
                    }
                }
                continue;
            }
            if squash_landed {
                events.push(GameEvent::PlantSpecialTriggered {
                    entity: id,
                    plant_type,
                });
            }
            if squash_finished {
                events.push(GameEvent::PlantDied { entity: id });
                continue;
            }
            if let Some(zombie_id) = tangle_grab_target {
                events.push(GameEvent::PlantSpecialTriggered {
                    entity: id,
                    plant_type,
                });
                if let Some(zombie_index) = self
                    .state
                    .board
                    .zombies
                    .iter()
                    .position(|zombie| zombie.id == zombie_id && zombie.health > 0)
                {
                    self.emit_zombie_died(zombie_id, events);
                    self.state.board.zombies.remove(zombie_index);
                }
                events.push(GameEvent::PlantDied { entity: id });
                continue;
            }
            if tangle_started {
                events.push(GameEvent::TangleKelpGrabStarted { entity: id });
            }
            if tangle_water_entry {
                events.push(GameEvent::TangleKelpWaterEntry { entity: id });
            }
            if spikeweed_started {
                events.push(GameEvent::PlantSpecialTriggered {
                    entity: id,
                    plant_type,
                });
            }
            if potato_armed_now {
                events.push(GameEvent::PotatoMineArmed { entity: id });
            }
            if spikeweed_hit {
                let vehicle_hit = self.apply_spikeweed_damage(id, row, column, events);
                if vehicle_hit && self.pop_spiky_plant(id, events) {
                    continue;
                }
            }
            if special {
                self.trigger_plant_special(id, plant_type, row, column, events);
                continue;
            }
            for _ in 0..produce_suns {
                self.spawn_sun_value(
                    SunSource::Plant(id),
                    produce_value,
                    grid_x(column),
                    grid_y(row),
                    events,
                );
                let vertical_motion = self.rng.next();
                let horizontal_motion = self.rng.next();
                let ground_offset = self.rng.range(20);
                if let Some(sun) = self.state.board.suns.last_mut() {
                    // COIN_MOTION_COIN: plant suns pop up in the -1.7..-3.4
                    // launch band with a small lateral drift and fall back to
                    // a ground stop near the plant under 0.09 gravity.
                    sun.velocity_y = -1_700_000 - (i64::from(vertical_motion) % 1_700_001);
                    sun.velocity_x = (i64::from(horizontal_motion) % 2_000_001) - 1_000_000;
                    sun.target_y = Some(sun.position_y + i64::from(ground_offset) * POSITION_SCALE);
                }
            }
            if produce_coin {
                let coin_type = if self.rng.range(100) < 10 {
                    CoinType::Gold
                } else {
                    CoinType::Silver
                };
                self.spawn_coin(coin_type, grid_x(column), grid_y(row), events);
            }

            let plant = &mut self.state.board.plants[index];
            if plant.blink_counter <= 1 {
                plant.blink_counter = 400 + self.rng.range(400);
            } else {
                plant.blink_counter -= 1;
            }
        }
        self.state.board.plants.retain(|plant| plant.health > 0);
    }

    fn trigger_plant_special(
        &mut self,
        plant_id: EntityId,
        plant_type: PlantType,
        row: u8,
        column: u8,
        events: &mut Vec<GameEvent>,
    ) {
        events.push(GameEvent::PlantSpecialTriggered {
            entity: plant_id,
            plant_type,
        });

        if plant_type.is_instant_coffee() {
            let target_id = self
                .state
                .board
                .plants
                .iter()
                .rfind(|plant| {
                    plant.id != plant_id
                        && plant.row == row
                        && plant.column == column
                        && !matches!(plant.plant_type.slot(), 16 | 33)
                })
                .and_then(|plant| {
                    (plant.plant_type.is_nocturnal() && plant.asleep && plant.wake_up_counter == 0)
                        .then_some(plant.id)
                });
            if let Some(target_id) = target_id
                && let Some(target) = self
                    .state
                    .board
                    .plants
                    .iter_mut()
                    .find(|plant| plant.id == target_id)
            {
                target.wake_up_counter = COFFEE_WAKE_TICKS;
            }
            if let Some(coffee) = self
                .state
                .board
                .plants
                .iter_mut()
                .find(|plant| plant.id == plant_id)
            {
                coffee.health = 0;
            }
            events.push(GameEvent::PlantDied { entity: plant_id });
            return;
        }

        if plant_type.is_blover() {
            self.blow_away_fliers();
            events.push(GameEvent::BloverTriggered {
                entity: plant_id,
                row,
            });
            return;
        }

        if plant_type.is_gravebuster() {
            if let Some(grave_index) = self
                .state
                .board
                .graves
                .iter()
                .position(|grave| grave.row == row && grave.column == column)
            {
                self.state.board.graves.remove(grave_index);
                events.push(GameEvent::GraveCleared {
                    entity: plant_id,
                    row,
                    column,
                });
            }
            if let Some(plant) = self
                .state
                .board
                .plants
                .iter_mut()
                .find(|plant| plant.id == plant_id)
            {
                plant.health = 0;
            }
            events.push(GameEvent::PlantDied { entity: plant_id });
            return;
        }

        if plant_type.is_doom_shroom() {
            let center_x = grid_x(column);
            let radius = DOOM_SHROOM_RADIUS * POSITION_SCALE;
            let target_ids = self
                .state
                .board
                .zombies
                .iter()
                .filter(|zombie| {
                    zombie.health > 0
                        && zombie.row.abs_diff(row) <= DOOM_SHROOM_ROW_RADIUS
                        && (zombie.position_x - center_x).abs() <= radius
                })
                .map(|zombie| zombie.id)
                .collect::<Vec<_>>();

            for zombie_id in target_ids {
                let Some(zombie_index) = self
                    .state
                    .board
                    .zombies
                    .iter()
                    .position(|zombie| zombie.id == zombie_id)
                else {
                    continue;
                };
                self.damage_zombie(zombie_index, PLANT_SPECIAL_DAMAGE, events);
                let health_remaining = self.state.board.zombies[zombie_index].health;
                events.push(GameEvent::PlantSpecialHit {
                    plant: plant_id,
                    zombie: zombie_id,
                    damage: PLANT_SPECIAL_DAMAGE,
                    health_remaining,
                });
                if health_remaining <= 0 {
                    self.emit_zombie_died(zombie_id, events);
                    self.state.board.zombies.remove(zombie_index);
                }
            }

            self.state.board.craters.push(CraterState {
                row,
                column,
                remaining: DOOM_CRATER_TICKS,
            });
            events.push(GameEvent::CraterCreated {
                row,
                column,
                duration: DOOM_CRATER_TICKS,
            });
            if let Some(plant) = self
                .state
                .board
                .plants
                .iter_mut()
                .find(|plant| plant.id == plant_id)
            {
                plant.health = 0;
            }
            events.push(GameEvent::PlantDied { entity: plant_id });
            return;
        }

        if plant_type.is_ice_shroom() {
            self.state.board.ice_counter = BOARD_ICE_TICKS;
            self.destroy_boss_ball(true, None, events);
            let target_ids = self
                .state
                .board
                .zombies
                .iter()
                .filter(|zombie| zombie.health > 0)
                .map(|zombie| zombie.id)
                .collect::<Vec<_>>();

            for zombie_id in target_ids {
                let Some(zombie_index) = self
                    .state
                    .board
                    .zombies
                    .iter()
                    .position(|zombie| zombie.id == zombie_id)
                else {
                    continue;
                };

                let (had_debuff, can_freeze) = {
                    let zombie = &mut self.state.board.zombies[zombie_index];
                    let had_debuff = zombie.frozen_counter != 0 || zombie.chilled_counter != 0;
                    zombie.chilled_counter = zombie.chilled_counter.max(ICE_SHROOM_CHILL_TICKS);
                    events.push(GameEvent::ZombieChilled {
                        entity: zombie_id,
                        duration: ICE_SHROOM_CHILL_TICKS,
                    });

                    // Keep the current freeze eligibility explicit.
                    (had_debuff, matches!(zombie.zombie_type, ZombieType::Normal))
                };
                if can_freeze {
                    let duration = if had_debuff {
                        ICE_SHROOM_REFRESH_FREEZE_TICKS
                    } else {
                        ICE_SHROOM_INITIAL_FREEZE_TICKS
                    };
                    let frozen_counter = self.state.board.zombies[zombie_index].frozen_counter;
                    self.state.board.zombies[zombie_index].frozen_counter =
                        frozen_counter.max(duration);
                    events.push(GameEvent::ZombieFrozen {
                        entity: zombie_id,
                        duration,
                    });

                    self.damage_zombie(zombie_index, ICE_SHROOM_DAMAGE, events);
                    let health_remaining = self.state.board.zombies[zombie_index].health;
                    events.push(GameEvent::PlantSpecialHit {
                        plant: plant_id,
                        zombie: zombie_id,
                        damage: ICE_SHROOM_DAMAGE,
                        health_remaining,
                    });
                    if health_remaining <= 0 {
                        self.emit_zombie_died(zombie_id, events);
                        self.state.board.zombies.remove(zombie_index);
                    }
                }
            }

            if let Some(plant) = self
                .state
                .board
                .plants
                .iter_mut()
                .find(|plant| plant.id == plant_id)
            {
                plant.health = 0;
            }
            events.push(GameEvent::PlantDied { entity: plant_id });
            return;
        }

        let (radius, row_radius, row_wide) = if plant_type.is_potato_mine() {
            (60 * POSITION_SCALE, 0, false)
        } else if plant_type.is_jalapeno() {
            (0, 0, true)
        } else {
            (115 * POSITION_SCALE, 1, false)
        };
        if plant_type.is_jalapeno() {
            let row_index = usize::from(row);
            if row_index < self.state.board.ice_timer.len()
                && self.state.board.ice_timer[row_index] > 0
            {
                self.state.board.ice_timer[row_index] = JALAPENO_ICE_MELT_TICKS;
            }
            self.destroy_boss_ball(false, Some(row), events);
        }
        let center_x = grid_x(column);
        let target_ids = self
            .state
            .board
            .zombies
            .iter()
            .filter(|zombie| {
                zombie.health > 0
                    && !zombie.bungee_held
                    && zombie.row.abs_diff(row) <= row_radius
                    && (row_wide || (zombie.position_x - center_x).abs() <= radius)
            })
            .map(|zombie| zombie.id)
            .collect::<Vec<_>>();
        for zombie_id in target_ids {
            let Some(zombie_index) = self
                .state
                .board
                .zombies
                .iter()
                .position(|zombie| zombie.id == zombie_id)
            else {
                continue;
            };
            self.damage_zombie(zombie_index, PLANT_SPECIAL_DAMAGE, events);
            let health_remaining = self.state.board.zombies[zombie_index].health;
            events.push(GameEvent::PlantSpecialHit {
                plant: plant_id,
                zombie: zombie_id,
                damage: PLANT_SPECIAL_DAMAGE,
                health_remaining,
            });
            if health_remaining <= 0 {
                self.emit_zombie_died(zombie_id, events);
                self.state.board.zombies.remove(zombie_index);
            }
        }
        if let Some(plant) = self
            .state
            .board
            .plants
            .iter_mut()
            .find(|plant| plant.id == plant_id)
        {
            plant.health = 0;
        }
        events.push(GameEvent::PlantDied { entity: plant_id });
    }

    fn advance_garlic_state(&mut self, zombie_index: usize, events: &mut Vec<GameEvent>) -> bool {
        let counter = self.state.board.zombies[zombie_index].garlic_counter;
        if counter == 0 {
            return false;
        }

        let next_counter = counter.saturating_add(1);
        let zombie_id = self.state.board.zombies[zombie_index].id;
        if next_counter == GARLIC_EAT_TICKS {
            let garlic_id = self.state.board.zombies[zombie_index].garlic_target;
            if let Some(plant_index) = self
                .state
                .board
                .plants
                .iter()
                .position(|plant| Some(plant.id) == garlic_id)
            {
                let plant_id = self.state.board.plants.remove(plant_index).id;
                events.push(GameEvent::PlantDied { entity: plant_id });
            }
        }
        if next_counter == GARLIC_ROW_CHANGE_TICKS && self.state.board.rows > 1 {
            let from = self.state.board.zombies[zombie_index].row;
            let choice = self.rng.range(u32::from(self.state.board.rows - 1)) as u8;
            let to = if choice >= from { choice + 1 } else { choice };
            self.state.board.zombies[zombie_index].row = to;
            events.push(GameEvent::ZombieRowChanged {
                entity: zombie_id,
                from,
                to,
            });
        }
        if next_counter >= GARLIC_RESET_TICKS {
            let zombie = &mut self.state.board.zombies[zombie_index];
            zombie.garlic_counter = 0;
            zombie.garlic_target = None;
            zombie.eating = false;
            return false;
        }

        let zombie = &mut self.state.board.zombies[zombie_index];
        zombie.garlic_counter = next_counter;
        zombie.eating = true;
        true
    }

    fn trigger_mower(&mut self, row: u8, events: &mut Vec<GameEvent>) -> bool {
        let Some(mower) = self
            .state
            .board
            .mowers
            .iter_mut()
            .find(|mower| mower.row == row && !mower.active && !mower.spent)
        else {
            return false;
        };
        mower.active = true;
        events.push(GameEvent::MowerTriggered { row });
        let mut dead_ids = Vec::new();
        for zombie in &mut self.state.board.zombies {
            if zombie.row == row && zombie.health > 0 {
                zombie.health = 0;
                zombie.eating = false;
                dead_ids.push(zombie.id);
            }
        }
        for entity in dead_ids {
            self.emit_zombie_died(entity, events);
        }
        true
    }

    fn update_mowers(&mut self, events: &mut Vec<GameEvent>) {
        let mut dead_ids = Vec::new();
        for mower in &mut self.state.board.mowers {
            if !mower.active {
                continue;
            }
            let previous_x = mower.position_x;
            mower.position_x = mower.position_x.saturating_add(MOWER_SPEED);
            if mower.position_x > i64::from(LOGICAL_WIDTH) * POSITION_SCALE {
                mower.active = false;
                mower.spent = true;
                continue;
            }
            let row = mower.row;
            let mower_x = mower.position_x;
            for zombie in &mut self.state.board.zombies {
                if zombie.row == row
                    && zombie.health > 0
                    && zombie.position_x + 70 * POSITION_SCALE > previous_x
                    && zombie.position_x < mower_x + 80 * POSITION_SCALE
                {
                    zombie.health = 0;
                    zombie.eating = false;
                    dead_ids.push(zombie.id);
                }
            }
        }
        for entity in dead_ids {
            self.emit_zombie_died(entity, events);
        }
        self.state.board.zombies.retain(|zombie| zombie.health > 0);
    }

    fn apply_jackbox_explosion(&mut self, zombie_index: usize, events: &mut Vec<GameEvent>) {
        let zx;
        let zrow;
        let zombie_id;
        {
            let zombie = &self.state.board.zombies[zombie_index];
            zx = zombie.position_x;
            zrow = zombie.row;
            zombie_id = zombie.id;
        }

        let explosion_radius = 115 * POSITION_SCALE;

        // KillAllPlantsInRadius uses the smaller 90-unit plant radius, ±1 row.
        let plant_radius = JACKBOX_PLANT_RADIUS * POSITION_SCALE;
        let mut plant_targets = Vec::new();
        for (i, plant) in self.state.board.plants.iter().enumerate() {
            if plant.health <= 0 {
                continue;
            }
            let row_diff = (i16::from(plant.row) - i16::from(zrow)).unsigned_abs();
            if row_diff > 1 {
                continue;
            }
            let px = grid_x(plant.column);
            let dx = (px - zx).unsigned_abs();
            if dx > plant_radius as u64 {
                continue;
            }
            plant_targets.push(i);
        }
        for i in plant_targets {
            self.state.board.plants[i].health = 0;
            events.push(GameEvent::PlantDied {
                entity: self.state.board.plants[i].id,
            });
        }

        // Also damage nearby zombies (friendly fire).
        let mut zombie_targets = Vec::new();
        for (other_idx, other) in self.state.board.zombies.iter().enumerate() {
            if other_idx == zombie_index {
                continue;
            }
            if other.health <= 0 {
                continue;
            }
            let row_diff = (i16::from(other.row) - i16::from(zrow)).unsigned_abs();
            if row_diff > 1 {
                continue;
            }
            let dx = (other.position_x - zx).unsigned_abs();
            if dx > explosion_radius as u64 {
                continue;
            }
            zombie_targets.push(other.id);
        }
        for target in zombie_targets {
            if let Some(target_index) = self
                .state
                .board
                .zombies
                .iter()
                .position(|zombie| zombie.id == target)
            {
                self.damage_zombie(target_index, PLANT_SPECIAL_DAMAGE, events);
                if self.state.board.zombies[target_index].health <= 0 {
                    self.emit_zombie_died(target, events);
                }
            }
        }

        let zombie_column = ((zx - 40 * POSITION_SCALE) / (80 * POSITION_SCALE))
            .clamp(0, i64::from(self.state.board.columns.saturating_sub(1)))
            as u8;
        events.push(GameEvent::JackboxExploded {
            entity: zombie_id,
            row: zrow,
            column: zombie_column,
        });
        self.emit_zombie_died(zombie_id, events);
    }

    fn eat_brain(&mut self, zombie_index: usize, row: u8, events: &mut Vec<GameEvent>) -> bool {
        let Some(brain_index) = self
            .state
            .board
            .brains
            .iter()
            .position(|brain| brain.row == row && !brain.squished)
        else {
            return false;
        };
        let remaining = self.state.board.brains[brain_index]
            .remaining
            .saturating_sub(1);
        self.state.board.brains[brain_index].remaining = remaining;
        let finished = remaining == 0;
        if finished {
            self.state.board.brains[brain_index].squished = true;
        }
        let brains_remaining = self
            .state
            .board
            .brains
            .iter()
            .filter(|brain| !brain.squished)
            .count()
            .try_into()
            .unwrap_or(u8::MAX);
        events.push(GameEvent::BrainEaten {
            zombie: self.state.board.zombies[zombie_index].id,
            row,
            brains_remaining,
        });
        if finished {
            events.push(GameEvent::BrainFinished {
                zombie: self.state.board.zombies[zombie_index].id,
                row,
                brains_remaining,
            });
        }
        if finished {
            self.state.board.zombies[zombie_index].health = 0;
            self.state.board.zombies[zombie_index].eating = false;
        } else {
            self.state.board.zombies[zombie_index].eating = true;
        }
        true
    }

    fn emit_zombie_died(&mut self, entity: EntityId, events: &mut Vec<GameEvent>) {
        let loot_position = self
            .state
            .board
            .zombies
            .iter_mut()
            .find(|zombie| zombie.id == entity)
            .and_then(|zombie| {
                if zombie.zombie_type == ZombieType::Yeti && !zombie.yeti_loot_dropped {
                    zombie.yeti_loot_dropped = true;
                    Some((zombie.position_x, zombie.row))
                } else {
                    None
                }
            });
        if let Some((position_x, row)) = loot_position {
            for offset in [20, 30, 40, 50].into_iter().take(YETI_DIAMOND_COUNT) {
                self.spawn_coin(
                    CoinType::Diamond,
                    position_x - offset * POSITION_SCALE,
                    grid_y(row),
                    events,
                );
            }
        }
        events.push(GameEvent::ZombieDied { entity });
    }

    fn update_dolphin_state(&mut self, zombie_index: usize) {
        let zombie = &mut self.state.board.zombies[zombie_index];
        if zombie.zombie_type != ZombieType::DolphinRider {
            return;
        }

        let in_pool = self.state.scene == SceneKind::Pool && matches!(zombie.row, 2 | 3);
        match zombie.dolphin_phase {
            0 if in_pool
                && zombie.position_x > 700 * POSITION_SCALE
                && zombie.position_x <= 720 * POSITION_SCALE =>
            {
                zombie.position_x -= 70 * POSITION_SCALE;
                zombie.dolphin_phase = 1;
                zombie.speed = DOLPHIN_RIDE_SPEED;
            }
            1 if zombie.position_x <= 10 * POSITION_SCALE => {
                zombie.dolphin_phase = 0;
                zombie.speed = DOLPHIN_WALK_SPEED;
            }
            2 => {
                zombie.dolphin_counter = zombie.dolphin_counter.saturating_sub(1);
                if zombie.dolphin_counter == 0 {
                    zombie.position_x = zombie.dolphin_target_x.unwrap_or(zombie.position_x);
                    zombie.dolphin_target_x = None;
                    zombie.dolphin_phase = 3;
                    zombie.speed = DOLPHIN_POOL_SPEED;
                }
            }
            3 if in_pool && zombie.position_x <= 10 * POSITION_SCALE => {
                zombie.dolphin_phase = 4;
                zombie.speed = DOLPHIN_WALK_SPEED;
            }
            _ => {}
        }
    }

    fn update_snorkel_state(&mut self, zombie_index: usize) {
        let zombie = &mut self.state.board.zombies[zombie_index];
        if zombie.zombie_type != ZombieType::Snorkel {
            return;
        }

        let in_pool = self.state.scene == SceneKind::Pool && matches!(zombie.row, 2 | 3);
        if !in_pool {
            return;
        }
        if zombie.snorkel_phase == 0
            && zombie.position_x > 700 * POSITION_SCALE
            && zombie.position_x <= 720 * POSITION_SCALE
        {
            zombie.snorkel_phase = 1;
            zombie.speed = SNORKEL_SPEED;
        } else if zombie.snorkel_phase == 1 && zombie.position_x <= 25 * POSITION_SCALE {
            zombie.snorkel_phase = 0;
            zombie.eating = false;
            zombie.speed = SNORKEL_SPEED;
        }
    }

    fn update_zamboni_state(&mut self, zombie_index: usize) {
        let (row, position_x) = {
            let zombie = &mut self.state.board.zombies[zombie_index];
            if zombie.zombie_type != ZombieType::Zamboni {
                return;
            }
            zombie.speed = zamboni_speed(zombie.position_x);
            (zombie.row, zombie.position_x)
        };
        let lay_min = if self.state.scene == SceneKind::Roof {
            ICE_LAY_MIN_X_ROOF
        } else {
            ICE_LAY_MIN_X
        };
        let ice_x = (position_x + ICE_LAY_OFFSET).max(lay_min);
        let row_index = usize::from(row);
        if row_index >= self.state.board.ice_timer.len() {
            return;
        }
        if ice_x < self.state.board.ice_min_x[row_index] {
            self.state.board.ice_min_x[row_index] = ice_x;
        }
        if ice_x < ICE_START_X {
            self.state.board.ice_timer[row_index] =
                if self.state.challenge.kind == ChallengeKind::BobsledBonanza {
                    u32::MAX
                } else {
                    ICE_TIMER_TICKS
                };
        }
    }

    fn update_ice(&mut self) {
        for row in 0..self.state.board.ice_timer.len() {
            if self.state.board.ice_timer[row] > 0 {
                self.state.board.ice_timer[row] -= 1;
                if self.state.board.ice_timer[row] == 0 {
                    self.state.board.ice_min_x[row] = ICE_START_X;
                }
            }
        }
    }

    fn is_ice_at(&self, row: u8, column: u8) -> bool {
        let row_index = usize::from(row);
        let timer = self
            .state
            .board
            .ice_timer
            .get(row_index)
            .copied()
            .unwrap_or(0);
        let min_x = self
            .state
            .board
            .ice_min_x
            .get(row_index)
            .copied()
            .unwrap_or(ICE_START_X);
        if timer == 0 || min_x > 750 * POSITION_SCALE {
            return false;
        }
        let ice_column =
            ((min_x + 12 * POSITION_SCALE - 40 * POSITION_SCALE) / (80 * POSITION_SCALE))
                .clamp(0, i64::from(self.state.board.columns.saturating_sub(1))) as u8;
        column >= ice_column
    }

    fn update_balloon_state(&mut self, zombie_index: usize) {
        let zombie = &mut self.state.board.zombies[zombie_index];
        if zombie.zombie_type == ZombieType::Balloon
            && zombie.balloon_phase == BALLOON_POPPING_PHASE
        {
            zombie.balloon_counter = zombie.balloon_counter.saturating_sub(1);
            if zombie.balloon_counter == 0 {
                zombie.balloon_phase = BALLOON_WALKING_PHASE;
            }
        }
    }

    fn update_special_zombie_state(
        &mut self,
        zombie_index: usize,
        events: &mut Vec<GameEvent>,
    ) -> bool {
        match self.state.board.zombies[zombie_index].zombie_type {
            ZombieType::Boss => {
                self.update_boss_state(zombie_index, events);
                true
            }
            ZombieType::JalapenoHead => self.update_jalapeno_head(zombie_index, events),
            ZombieType::GatlingHead => {
                self.update_gatling_head(zombie_index, events);
                false
            }
            ZombieType::SquashHead => self.update_squash_head(zombie_index, events),
            _ => false,
        }
    }

    fn update_boss_state(&mut self, zombie_index: usize, events: &mut Vec<GameEvent>) {
        self.update_boss_head(zombie_index, events);
        let should_stomp = {
            let zombie = &mut self.state.board.zombies[zombie_index];
            zombie.special_counter = zombie.special_counter.saturating_sub(1);
            zombie.special_counter == 0
        };
        if !should_stomp {
            return;
        }

        let max_row = self.state.board.rows.saturating_sub(2);
        let target_row = self.rng.range(u32::from(max_row) + 1) as u8;
        let target_ids = self
            .state
            .board
            .plants
            .iter()
            .filter(|plant| {
                plant.health > 0
                    && plant.row >= target_row
                    && plant.row <= target_row.saturating_add(1)
                    && plant.column >= 5
            })
            .map(|plant| plant.id)
            .collect::<Vec<_>>();
        for plant_id in target_ids {
            if let Some(plant) = self
                .state
                .board
                .plants
                .iter_mut()
                .find(|plant| plant.id == plant_id)
            {
                plant.health = 0;
                events.push(GameEvent::PlantDied { entity: plant_id });
            }
        }
        self.state.board.zombies[zombie_index].special_counter = BOSS_ATTACK_TICKS;
    }

    /// Zombie.cpp:9977-10202: the boss head cycle spits a fire or ice ball
    /// down a random row; the ball drives over plants in its cell, squishes
    /// mowers it passes, and leaves the lawn at x < -180.
    fn update_boss_head(&mut self, zombie_index: usize, events: &mut Vec<GameEvent>) {
        let (ball_active, ball_row, ball_x) = {
            let zombie = &self.state.board.zombies[zombie_index];
            (
                zombie.boss_ball_active,
                zombie.boss_ball_row,
                zombie.boss_ball_x,
            )
        };
        if ball_active {
            let ball_x = ball_x - BOSS_BALL_SPEED;
            if ball_x < BOSS_BALL_END_X {
                self.state.board.zombies[zombie_index].boss_ball_active = false;
            } else {
                self.state.board.zombies[zombie_index].boss_ball_x = ball_x;
                // Zombie.cpp:10156: drive-over squish at the ball center +75.
                let column = ((ball_x + 75 * POSITION_SCALE - 40 * POSITION_SCALE)
                    / (80 * POSITION_SCALE))
                    .clamp(0, i64::from(self.state.board.columns.saturating_sub(1)))
                    as u8;
                let plant_ids = self
                    .state
                    .board
                    .plants
                    .iter()
                    .filter(|plant| {
                        plant.health > 0 && plant.row == ball_row && plant.column == column
                    })
                    .map(|plant| plant.id)
                    .collect::<Vec<_>>();
                for plant_id in plant_ids {
                    if let Some(plant) = self
                        .state
                        .board
                        .plants
                        .iter_mut()
                        .find(|plant| plant.id == plant_id)
                    {
                        plant.health = 0;
                        events.push(GameEvent::PlantDied { entity: plant_id });
                    }
                }
                // Zombie.cpp:10158-10166: the ball squishes mowers it passes
                // instead of triggering them.
                self.state.board.mowers.retain(|mower| {
                    !(mower.row == ball_row
                        && mower.position_x > ball_x
                        && mower.position_x < ball_x + BOSS_BALL_MOWER_REACH)
                });
            }
        }
        let counter = self.state.board.zombies[zombie_index].boss_head_counter;
        if counter > 1 {
            self.state.board.zombies[zombie_index].boss_head_counter = counter - 1;
        } else if counter == 1 {
            // BossHeadSpit (Zombie.cpp:9987-10002): row 0-4, 50/50 fire.
            let row = self.rng.range(u32::from(DAY_ROWS)) as u8;
            let fire = self.rng.range(2) == 0;
            let next = BOSS_HEAD_SPIT_DELAY + self.rng.range_inclusive(4_000, 5_000);
            let entity = {
                let zombie = &mut self.state.board.zombies[zombie_index];
                zombie.boss_head_counter = next;
                zombie.boss_ball_active = true;
                zombie.boss_ball_fire = fire;
                zombie.boss_ball_row = row;
                zombie.boss_ball_x = BOSS_BALL_START_X;
                zombie.id
            };
            events.push(GameEvent::BossAttackWindup { entity, row, fire });
        }
    }

    /// Plant.cpp:4261-4265 (Ice-shroom kills the fire ball) and
    /// Zombie.cpp:2345-2349 (a row burn kills the ice ball in its row).
    fn destroy_boss_ball(&mut self, fire: bool, row: Option<u8>, events: &mut Vec<GameEvent>) {
        for zombie in &mut self.state.board.zombies {
            if zombie.zombie_type == ZombieType::Boss
                && zombie.boss_ball_active
                && zombie.boss_ball_fire == fire
                && (row.is_none() || row == Some(zombie.boss_ball_row))
            {
                zombie.boss_ball_active = false;
                events.push(GameEvent::BossProjectileDestroyed {
                    entity: zombie.id,
                    fire,
                });
            }
        }
    }

    fn update_jalapeno_head(&mut self, zombie_index: usize, events: &mut Vec<GameEvent>) -> bool {
        let should_burn = {
            let zombie = &mut self.state.board.zombies[zombie_index];
            if zombie.special_counter == 0 {
                true
            } else {
                zombie.special_counter -= 1;
                zombie.special_counter == 0
            }
        };
        if !should_burn {
            return false;
        }

        let row = self.state.board.zombies[zombie_index].row;
        let plant_ids = self
            .state
            .board
            .plants
            .iter()
            .filter(|plant| plant.health > 0 && plant.row == row)
            .map(|plant| plant.id)
            .collect::<Vec<_>>();
        for plant_id in plant_ids {
            if let Some(plant) = self
                .state
                .board
                .plants
                .iter_mut()
                .find(|plant| plant.id == plant_id)
            {
                plant.health = 0;
                events.push(GameEvent::PlantDied { entity: plant_id });
            }
        }
        let zombie_id = self.state.board.zombies[zombie_index].id;
        self.state.board.zombies[zombie_index].health = 0;
        self.state.board.zombies[zombie_index].eating = false;
        self.emit_zombie_died(zombie_id, events);
        true
    }

    fn update_gatling_head(&mut self, zombie_index: usize, events: &mut Vec<GameEvent>) {
        let should_fire = {
            let zombie = &mut self.state.board.zombies[zombie_index];
            zombie.special_counter = zombie.special_counter.saturating_sub(1);
            if zombie.special_counter == 0 {
                zombie.special_counter = ZOMBOTANY_HEAD_RELOAD_TICKS;
                true
            } else {
                false
            }
        };
        if !should_fire {
            return;
        }

        let (source, row, position_x) = {
            let zombie = &self.state.board.zombies[zombie_index];
            (zombie.id, zombie.row, zombie.position_x)
        };
        for _ in 0..4 {
            self.fire_projectile(
                source,
                ProjectileType::ZombiePea,
                row,
                ProjectileTrajectory {
                    motion: ProjectileMotion::Backwards,
                    position_x: position_x + 20 * POSITION_SCALE,
                    position_y: grid_y(row),
                    velocity_x: -3_330_000,
                    velocity_y: 0,
                },
                events,
            );
        }
    }

    fn update_squash_head(&mut self, zombie_index: usize, events: &mut Vec<GameEvent>) -> bool {
        let phase = self.state.board.zombies[zombie_index].special_phase;
        if phase == 0 {
            return false;
        }

        match phase {
            1 => {
                let zombie = &mut self.state.board.zombies[zombie_index];
                zombie.special_counter = zombie.special_counter.saturating_sub(1);
                if zombie.special_counter == 0 {
                    zombie.special_phase = 2;
                    zombie.special_counter = ZOMBOTANY_SQUASH_FALL_TICKS;
                }
            }
            2 => {
                let (counter, target_id) = {
                    let zombie = &mut self.state.board.zombies[zombie_index];
                    let counter = zombie.special_counter;
                    zombie.special_counter = counter.saturating_sub(1);
                    (counter, zombie.special_target.take())
                };
                if counter <= 3
                    && let Some(target_id) = target_id
                    && let Some(plant) = self
                        .state
                        .board
                        .plants
                        .iter_mut()
                        .find(|plant| plant.id == target_id && plant.health > 0)
                {
                    plant.health = 0;
                    events.push(GameEvent::PlantDied { entity: target_id });
                }
                if self.state.board.zombies[zombie_index].special_counter == 0 {
                    self.state.board.zombies[zombie_index].special_phase = 3;
                    self.state.board.zombies[zombie_index].special_counter =
                        ZOMBOTANY_SQUASH_DONE_TICKS;
                } else if target_id.is_some() {
                    self.state.board.zombies[zombie_index].special_target = target_id;
                }
            }
            3 => {
                let zombie_id = {
                    let zombie = &mut self.state.board.zombies[zombie_index];
                    zombie.special_counter = zombie.special_counter.saturating_sub(1);
                    if zombie.special_counter == 0 {
                        Some(zombie.id)
                    } else {
                        None
                    }
                };
                if let Some(zombie_id) = zombie_id {
                    self.damage_zombie(zombie_index, ZOMBOTANY_SQUASH_DAMAGE, events);
                    self.emit_zombie_died(zombie_id, events);
                }
            }
            _ => {}
        }
        true
    }

    fn blow_away_fliers(&mut self) {
        for zombie in &mut self.state.board.zombies {
            if balloon_is_airborne(zombie) {
                zombie.blowing_away = true;
            }
        }
    }

    fn damage_zombie(&mut self, zombie_index: usize, damage: i32, events: &mut Vec<GameEvent>) {
        let pool_lane = self.state.scene == SceneKind::Pool
            && matches!(self.state.board.zombies[zombie_index].row, 2 | 3);
        let (entity, shield_damage, shield_type, newspaper_ripped) = {
            let mut remaining = damage.max(0);
            let zombie = &mut self.state.board.zombies[zombie_index];
            if zombie.zombie_type == ZombieType::Balloon
                && zombie.balloon_phase == BALLOON_FLYING_PHASE
            {
                let absorbed = remaining.min(zombie.balloon_flying_health);
                zombie.balloon_flying_health -= absorbed;
                remaining -= absorbed;
                if zombie.balloon_flying_health == 0 {
                    if pool_lane {
                        zombie.health = 0;
                        zombie.balloon_phase = 0;
                    } else {
                        zombie.balloon_phase = BALLOON_POPPING_PHASE;
                        zombie.balloon_counter = BALLOON_POP_TICKS;
                    }
                }
            }
            let shield_damage = remaining.min(zombie.shield_health);
            zombie.shield_health -= shield_damage;
            remaining -= shield_damage;
            zombie.health -= remaining;
            (
                zombie.id,
                shield_damage,
                zombie.zombie_type,
                zombie.zombie_type == ZombieType::Newspaper
                    && shield_damage > 0
                    && zombie.shield_health == 0
                    && zombie.health > 0,
            )
        };
        if newspaper_ripped {
            events.push(GameEvent::ZombieNewspaperRipped { entity });
        }
        if shield_damage > 0 && matches!(shield_type, ZombieType::ScreenDoor | ZombieType::Ladder) {
            events.push(GameEvent::ZombieShieldHit {
                entity,
                variant: self.rng.range(2) as u8,
            });
        }
        self.update_damage_tier(zombie_index, false, events);
    }

    /// Fume, gloom, and spike damage carry DAMAGE_BYPASSES_SHIELD: doors,
    /// paper, and ladders are skipped, but the Bobsled sled is a helm in the
    /// source and still absorbs first.
    fn damage_zombie_bypassing_shield(
        &mut self,
        zombie_index: usize,
        damage: i32,
        spike: bool,
        events: &mut Vec<GameEvent>,
    ) {
        if self.state.board.zombies[zombie_index].zombie_type == ZombieType::Bobsled {
            self.damage_zombie(zombie_index, damage, events);
            return;
        }
        let zombie = &mut self.state.board.zombies[zombie_index];
        zombie.health -= damage.max(0);
        self.update_damage_tier(zombie_index, spike, events);
    }

    fn update_damage_tier(
        &mut self,
        zombie_index: usize,
        spike: bool,
        events: &mut Vec<GameEvent>,
    ) {
        let zombie = &mut self.state.board.zombies[zombie_index];
        let tier = damage_tier(zombie.health, zombie.max_health);
        if tier > zombie.damage_tier {
            zombie.damage_tier = tier;
            events.push(GameEvent::ZombieDamageTierChanged {
                entity: zombie.id,
                tier,
            });
        }
        if spike
            && matches!(
                zombie.zombie_type,
                ZombieType::Zamboni | ZombieType::Catapult
            )
            && !zombie.vehicle_disabled
        {
            zombie.vehicle_disabled = true;
            zombie.speed = 0;
            zombie.catapult_armed = false;
            events.push(GameEvent::VehicleDisabled { entity: zombie.id });
        }
    }

    fn update_gargantuar_throw(
        &mut self,
        zombie_index: usize,
        events: &mut Vec<GameEvent>,
    ) -> bool {
        let mut throw = None;
        {
            let zombie = &mut self.state.board.zombies[zombie_index];
            if !is_gargantuar(zombie.zombie_type) {
                return false;
            }

            if zombie.special_phase != 0 {
                if zombie.frozen_counter == 0 {
                    let step = if zombie.chilled_counter == 0 { 2 } else { 1 };
                    zombie.special_counter = zombie.special_counter.saturating_sub(step);
                    if zombie.special_counter == 0 {
                        if zombie.special_phase == 1 {
                            zombie.special_phase = 2;
                            zombie.special_counter = GARGANTUAR_THROW_RECOVERY_STEPS;
                            zombie.imp_thrown = true;
                            throw =
                                Some((zombie.id, zombie.row, zombie.position_x, zombie.from_wave));
                        } else {
                            zombie.special_phase = 0;
                        }
                    }
                }
            } else if !zombie.imp_thrown
                && zombie.frozen_counter == 0
                && zombie.health < zombie.max_health / 2
                && zombie.position_x - GARGANTUAR_THROW_BASE_X > GARGANTUAR_THROW_MIN_DISTANCE
            {
                zombie.special_phase = 1;
                zombie.special_counter = GARGANTUAR_THROW_EVENT_STEPS;
                zombie.eating = false;
            }
        }

        if let Some((gargantuar, row, position_x, wave)) = throw {
            self.throw_imp(gargantuar, row, position_x, wave, events);
        }
        self.state.board.zombies[zombie_index].special_phase != 0
    }

    fn update_zombies(&mut self, events: &mut Vec<GameEvent>) {
        let zombie_count = self.state.board.zombies.len() as u32;
        for zombie_index in 0..self.state.board.zombies.len() {
            if self.state.board.zombies[zombie_index].departed {
                continue;
            }
            if self.state.board.zombies[zombie_index].health <= 0 {
                // Zombie_UpdateJack only detonates when the pop phase finishes;
                // a Jack killed by damage never explodes.
                continue;
            }
            self.update_balloon_state(zombie_index);
            let bobsled_finished = {
                let zombie = &mut self.state.board.zombies[zombie_index];
                if zombie.zombie_type == ZombieType::Bobsled && zombie.bobsled_sliding {
                    zombie.bobsled_counter = zombie.bobsled_counter.saturating_sub(1);
                    if zombie.bobsled_counter == 0 {
                        zombie.bobsled_sliding = false;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };
            if bobsled_finished {
                self.state.board.zombies[zombie_index].speed =
                    self.rng.fixed_range(230_000, 320_000);
            }
            {
                let (sliding, sled_row, is_leader, leader_x) = {
                    let zombie = &self.state.board.zombies[zombie_index];
                    (
                        zombie.zombie_type == ZombieType::Bobsled && zombie.bobsled_sliding,
                        zombie.row,
                        zombie.bobsled_leader,
                        zombie.position_x,
                    )
                };
                if sliding {
                    let row_index = usize::from(sled_row);
                    if row_index < self.state.board.ice_timer.len() {
                        let timer = self.state.board.ice_timer[row_index];
                        self.state.board.ice_timer[row_index] =
                            timer.max(BOBSLED_ICE_KEEPALIVE_TICKS);
                    }
                    let ice_min = self
                        .state
                        .board
                        .ice_min_x
                        .get(row_index)
                        .copied()
                        .unwrap_or(ICE_START_X);
                    if is_leader && leader_x + 10 * POSITION_SCALE < ice_min {
                        self.damage_zombie(zombie_index, BOBSLED_ICE_END_DAMAGE, events);
                        if self.state.board.zombies[zombie_index].shield_health == 0 {
                            let team: Vec<usize> = self
                                .state
                                .board
                                .zombies
                                .iter()
                                .enumerate()
                                .filter(|(_, z)| {
                                    z.zombie_type == ZombieType::Bobsled
                                        && z.bobsled_sliding
                                        && z.row == sled_row
                                })
                                .map(|(index, _)| index)
                                .collect();
                            for teammate in team {
                                let speed = self.rng.fixed_range(230_000, 320_000);
                                let zombie = &mut self.state.board.zombies[teammate];
                                zombie.bobsled_sliding = false;
                                zombie.bobsled_counter = 0;
                                zombie.speed = speed;
                            }
                        }
                    }
                }
            }
            if self.state.board.zombies[zombie_index].blowing_away {
                let entity = self.state.board.zombies[zombie_index].id;
                self.state.board.zombies[zombie_index].position_x += BLOWN_AWAY_SPEED;
                if self.state.board.zombies[zombie_index].position_x > BLOWN_AWAY_EDGE {
                    self.emit_zombie_died(entity, events);
                    self.state.board.zombies[zombie_index].departed = true;
                }
                continue;
            }
            {
                let zombie = &mut self.state.board.zombies[zombie_index];
                if zombie.imp_flight_ticks > 0 {
                    zombie.imp_flight_ticks -= 1;
                    zombie.position_x -= IMP_THROW_SPEED_X;
                    continue;
                }
                if zombie.bungee_held {
                    continue;
                }
                // PARTICLE_ICE_TRAP_RELEASE fires the tick the freeze expires.
                if zombie.frozen_counter == 1 {
                    events.push(GameEvent::ZombieThawed { entity: zombie.id });
                }
                // DropHelm / DropShield anchors: armor loss is observed at the
                // tick boundary so every damage source routes through one site.
                if zombie.armor_intact {
                    let helm = matches!(
                        zombie.zombie_type,
                        ZombieType::Buckethead | ZombieType::Football | ZombieType::Conehead
                    );
                    let lost = if helm {
                        zombie.health <= 270
                    } else {
                        zombie.shield_health == 0
                    };
                    if lost {
                        zombie.armor_intact = false;
                        let entity = zombie.id;
                        if helm {
                            events.push(GameEvent::ZombieArmorLost { entity });
                        } else {
                            events.push(GameEvent::ZombieShieldLost { entity });
                        }
                    }
                }
            }
            {
                let (delivering, phase, counter, held) = {
                    let zombie = &self.state.board.zombies[zombie_index];
                    (
                        zombie.zombie_type == ZombieType::Bungee && zombie.special_phase > 0,
                        zombie.special_phase,
                        zombie.special_counter,
                        zombie.special_target,
                    )
                };
                if delivering {
                    let counter = counter.saturating_sub(1);
                    self.state.board.zombies[zombie_index].special_counter = counter;
                    if counter == 0 {
                        if phase == 1 {
                            if let Some(zombie) = held.and_then(|held_id| {
                                self.state
                                    .board
                                    .zombies
                                    .iter_mut()
                                    .find(|z| z.id == held_id)
                            }) {
                                zombie.bungee_held = false;
                            }
                            let zombie = &mut self.state.board.zombies[zombie_index];
                            zombie.special_phase = 2;
                            zombie.special_counter = BUNGEE_RISE_DEPART_TICKS;
                            zombie.special_target = None;
                        } else {
                            let entity = self.state.board.zombies[zombie_index].id;
                            self.emit_zombie_died(entity, events);
                            self.state.board.zombies[zombie_index].departed = true;
                        }
                    }
                    continue;
                }
            }
            let gargantuar_throwing = self.update_gargantuar_throw(zombie_index, events);
            let mut entered_pool = None;
            {
                let scene = self.state.scene;
                let zombie = &mut self.state.board.zombies[zombie_index];
                if scene == SceneKind::Pool
                    && matches!(zombie.row, 2 | 3)
                    && zombie.position_x < 680 * POSITION_SCALE
                    && !zombie.in_pool
                    && (zombie_type_can_go_in_pool(zombie.zombie_type)
                        || zombie.zombie_type == ZombieType::DuckyTube)
                {
                    zombie.in_pool = true;
                    entered_pool = Some(zombie.id);
                }
            }
            if let Some(entity) = entered_pool {
                events.push(GameEvent::ZombieEnteredPool {
                    entity,
                    variant: self.rng.range(2) as u8,
                });
            }
            self.update_dolphin_state(zombie_index);
            self.update_snorkel_state(zombie_index);
            self.update_zamboni_state(zombie_index);
            let garlic_active = self.advance_garlic_state(zombie_index, events);
            if self.update_special_zombie_state(zombie_index, events) {
                continue;
            }
            {
                let zombie = &mut self.state.board.zombies[zombie_index];
                if zombie.zombie_type == ZombieType::Yeti && !zombie.yeti_running {
                    // Phase countdown precedes thaw; UpdateYeti checks after thaw.
                    if zombie.frozen_counter == 0 {
                        zombie.yeti_counter = zombie.yeti_counter.saturating_sub(1);
                    }
                    if zombie.yeti_counter == 0 && !zombie.hypnotized && zombie.frozen_counter <= 1
                    {
                        zombie.yeti_running = true;
                    }
                }
            }
            let summon_dancer = {
                let zombie = &mut self.state.board.zombies[zombie_index];
                if zombie.zombie_type == ZombieType::Dancer && !zombie.dancer_summoned {
                    zombie.dancer_counter = zombie.dancer_counter.saturating_sub(1);
                    if zombie.dancer_counter == 0 {
                        zombie.dancer_summoned = true;
                        // Zombie_ResetSpeed re-picks the fixed 0.45 dance walk
                        // once the entrance finishes.
                        zombie.speed = 450_000;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };
            if summon_dancer {
                let (row, position_x, wave) = {
                    let zombie = &self.state.board.zombies[zombie_index];
                    (zombie.row, zombie.position_x, zombie.from_wave)
                };
                let positions = [
                    (row.checked_sub(1), position_x),
                    (row.checked_add(1), position_x),
                    (Some(row), position_x - 100 * POSITION_SCALE),
                    (Some(row), position_x + 100 * POSITION_SCALE),
                ];
                for (target_row, target_x) in positions
                    .into_iter()
                    .filter_map(|(target_row, target_x)| {
                        target_row.map(|target_row| (target_row, target_x))
                    })
                    .take(BACKUP_DANCER_COUNT)
                {
                    if target_row < self.state.board.rows {
                        self.spawn_backup_dancer(target_row, wave, Some(target_x), events);
                    }
                }
            }
            {
                let mode = self.state.mode;
                let zombie = &mut self.state.board.zombies[zombie_index];
                if zombie.zombie_type == ZombieType::Digger {
                    if zombie.digger_underground && zombie.position_x <= 10 * POSITION_SCALE {
                        zombie.digger_underground = false;
                        zombie.digger_counter = DIGGER_RISE_TICKS;
                        // PARTICLE_DIGGER_RISE anchor.
                        events.push(GameEvent::DiggerSurfaced { entity: zombie.id });
                        // Zombie_ResetSpeed: the surfaced walk is 0.12, or
                        // 0.23 on I, Zombie levels.
                        zombie.speed = if mode == ModeKind::IZombie {
                            DIGGER_IZOMBIE_WALK_SPEED
                        } else {
                            DIGGER_WALK_SPEED
                        };
                    } else if zombie.digger_counter > 0 {
                        zombie.digger_counter = zombie.digger_counter.saturating_sub(1);
                    }
                }
            }
            let bungee_steals = {
                let zombie = &mut self.state.board.zombies[zombie_index];
                if zombie.zombie_type == ZombieType::Bungee && !zombie.bungee_stolen {
                    zombie.bungee_counter = zombie.bungee_counter.saturating_sub(1);
                    if zombie.bungee_counter == 0 {
                        zombie.bungee_stolen = true;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };
            if bungee_steals {
                let zombie_id = self.state.board.zombies[zombie_index].id;
                if let Some(plant_index) = self
                    .state
                    .board
                    .plants
                    .iter()
                    .enumerate()
                    .filter(|(_, plant)| plant.health > 0 && plant.plant_type.slot() != 47)
                    .max_by_key(|(_, plant)| (plant.row, plant.column))
                    .map(|(index, _)| index)
                {
                    let (target_row, target_column) = {
                        let plant = &self.state.board.plants[plant_index];
                        (plant.row, plant.column)
                    };
                    // Plant::FindUmbrellaPlant: an Umbrella Leaf within one
                    // cell bounces the bungee before the grab.
                    let umbrella = self
                        .state
                        .board
                        .plants
                        .iter()
                        .find(|plant| {
                            plant.plant_type.slot() == 37
                                && plant.row.abs_diff(target_row) <= 1
                                && plant.column.abs_diff(target_column) <= 1
                        })
                        .map(|plant| plant.id);
                    if let Some(umbrella_id) = umbrella {
                        events.push(GameEvent::UmbrellaDeflected {
                            plant: umbrella_id,
                            zombie: zombie_id,
                        });
                    } else {
                        let plant_id = self.state.board.plants.remove(plant_index).id;
                        events.push(GameEvent::PlantDied { entity: plant_id });
                    }
                }
                self.state.board.zombies[zombie_index].health = 0;
                self.emit_zombie_died(zombie_id, events);
                continue;
            }
            {
                let zombie = &mut self.state.board.zombies[zombie_index];
                if zombie.zombie_type == ZombieType::Pogo
                    && zombie.pogo_counter > 0
                    && zombie.frozen_counter == 0
                {
                    zombie.pogo_counter -= 1;
                    if zombie.pogo_counter == 5 {
                        events.push(GameEvent::PogoBounceSound { entity: zombie.id });
                    }
                    if let Some(target_x) = zombie.pogo_target_x {
                        zombie.position_x += zombie.pogo_velocity_x;
                        if zombie.pogo_counter == 0 {
                            zombie.position_x = target_x;
                            zombie.pogo_target_x = None;
                            zombie.pogo_velocity_x = 0;
                        }
                    }
                }
            }
            // Jack-in-the-Box: decrement timer and explode when zero.
            {
                let zombie = &mut self.state.board.zombies[zombie_index];
                if zombie.zombie_type == ZombieType::Jackbox && zombie.jackbox_timer > 0 {
                    zombie.jackbox_timer = zombie.jackbox_timer.saturating_sub(1);
                    if zombie.jackbox_timer == 0 {
                        zombie.health = 0;
                        self.apply_jackbox_explosion(zombie_index, events);
                    }
                }
            }
            let (
                entity,
                row,
                position_x,
                age,
                was_eating,
                frozen,
                pogo_bouncing,
                dancer_dancing,
                digger_hidden,
                dolphin_jumping,
                balloon_airborne,
                bobsled_sliding,
            ) = {
                let zombie = &mut self.state.board.zombies[zombie_index];
                zombie.age = zombie.age.saturating_add(1);
                zombie.groan_counter -= 1;
                zombie.frozen_counter = zombie.frozen_counter.saturating_sub(1);
                zombie.chilled_counter = zombie.chilled_counter.saturating_sub(1);
                if zombie.zombie_type == ZombieType::PeaHead {
                    zombie.pea_head_counter = zombie.pea_head_counter.saturating_sub(1);
                }
                if zombie.groan_counter == 0 && self.rng.range(zombie_count) == 0 {
                    zombie.groan_counter = (self.rng.range(1_000) + 500) as i32;
                }
                let frozen = zombie.frozen_counter != 0;
                if !(frozen
                    || zombie.eating
                    || gargantuar_throwing
                    || garlic_active
                    || (zombie.zombie_type == ZombieType::Catapult && zombie.catapult_armed)
                    || (zombie.zombie_type == ZombieType::Pogo && zombie.pogo_counter > 0)
                    || (zombie.zombie_type == ZombieType::Dancer && zombie.dancer_counter > 0)
                    || (zombie.zombie_type == ZombieType::Digger
                        && (zombie.digger_underground || zombie.digger_counter > 0))
                    || (zombie.zombie_type == ZombieType::DolphinRider
                        && zombie.dolphin_phase == 2)
                    || (zombie.zombie_type == ZombieType::Balloon
                        && zombie.balloon_phase == BALLOON_POPPING_PHASE)
                    || zombie.zombie_type == ZombieType::Bungee)
                {
                    let sledding =
                        zombie.zombie_type == ZombieType::Bobsled && zombie.bobsled_sliding;
                    let base_speed = if sledding {
                        BOBSLED_SPEED
                    } else if zombie.yeti_running {
                        YETI_RUNNING_SPEED
                    } else if zombie.zombie_type == ZombieType::Newspaper
                        && zombie.shield_health == 0
                    {
                        // Zombie_ResetSpeed gives PHASE_NEWSPAPER_MAD 0.89-0.91.
                        NEWSPAPER_MAD_SPEED
                    } else {
                        zombie.speed
                    };
                    let speed = if zombie.chilled_counter == 0 || sledding {
                        base_speed
                    } else {
                        base_speed * 2 / 5
                    };
                    if zombie.hypnotized || zombie.yeti_running {
                        zombie.position_x = zombie.position_x.saturating_add(speed);
                    } else {
                        zombie.position_x = zombie.position_x.saturating_sub(speed);
                    }
                }
                (
                    zombie.id,
                    zombie.row,
                    zombie.position_x,
                    zombie.age,
                    zombie.eating,
                    frozen,
                    zombie.zombie_type == ZombieType::Pogo && zombie.pogo_counter > 0,
                    zombie.zombie_type == ZombieType::Dancer && zombie.dancer_counter > 0,
                    zombie.zombie_type == ZombieType::Digger
                        && (zombie.digger_underground || zombie.digger_counter > 0),
                    zombie.zombie_type == ZombieType::DolphinRider && zombie.dolphin_phase == 2,
                    balloon_is_airborne(zombie),
                    zombie.zombie_type == ZombieType::Bobsled && zombie.bobsled_sliding,
                )
            };

            if frozen {
                self.state.board.zombies[zombie_index].eating = false;
            } else if garlic_active {
                self.state.board.zombies[zombie_index].eating = true;
            } else if age % 4 == 0
                && !gargantuar_throwing
                && !dancer_dancing
                && !digger_hidden
                && !dolphin_jumping
                && !balloon_airborne
                && !bobsled_sliding
                && self.state.board.zombies[zombie_index].zombie_type != ZombieType::Bungee
            {
                let is_hypnotized = self.state.board.zombies[zombie_index].hypnotized;
                if is_hypnotized {
                    self.attack_zombie_target(entity, row, events);
                } else {
                    let ztype = self.state.board.zombies[zombie_index].zombie_type;
                    let target = self.find_plant_for_zombie(row, position_x, ztype);
                    let pogo_has_stick = ztype == ZombieType::Pogo
                        && self.state.board.zombies[zombie_index].special_phase == 0;
                    self.state.board.zombies[zombie_index].eating =
                        target.is_some() && !pogo_has_stick;
                    if let Some(plant_index) = target {
                        let plant_id = self.state.board.plants[plant_index].id;
                        let has_vaulted = self.state.board.zombies[zombie_index].has_vaulted;
                        if ztype == ZombieType::SquashHead
                            && self.state.board.zombies[zombie_index].special_phase == 0
                        {
                            let zombie = &mut self.state.board.zombies[zombie_index];
                            zombie.eating = false;
                            zombie.special_phase = 1;
                            zombie.special_counter = ZOMBOTANY_SQUASH_RISE_TICKS;
                            zombie.special_target = Some(plant_id);
                        } else if ztype == ZombieType::Ladder
                            && is_ladder_target(self.state.board.plants[plant_index].plant_type)
                            && !self.state.board.ladders.iter().any(|ladder| {
                                ladder.row == self.state.board.plants[plant_index].row
                                    && ladder.column == self.state.board.plants[plant_index].column
                            })
                        {
                            let (row, column) = {
                                let plant = &self.state.board.plants[plant_index];
                                (plant.row, plant.column)
                            };
                            self.state.board.ladders.push(LadderState { row, column });
                            // Leaving PHASE_LADDER_CARRYING re-picks the plain
                            // 0.23-0.32 walk (Zombie_ResetSpeed).
                            let walk_speed = self.rng.fixed_range(230_000, 320_000);
                            self.state.board.zombies[zombie_index].ladder_placed = true;
                            self.state.board.zombies[zombie_index].shield_health = 0;
                            self.state.board.zombies[zombie_index].eating = false;
                            self.state.board.zombies[zombie_index].speed = walk_speed;
                        } else if ztype == ZombieType::Pogo
                            && self.state.board.zombies[zombie_index].special_phase == 0
                        {
                            // PARTICLE_TALL_NUT_BLOCK + PogoBreak: a Tall-nut
                            // stops the bounce and costs the pogo its stick.
                            if self.state.board.plants[plant_index].plant_type.slot() == 23 {
                                let zombie = &mut self.state.board.zombies[zombie_index];
                                zombie.special_phase = 1;
                                zombie.pogo_counter = 0;
                                zombie.pogo_target_x = None;
                                zombie.pogo_velocity_x = 0;
                                zombie.eating = false;
                                events.push(GameEvent::JumpBlocked {
                                    zombie: entity,
                                    plant: plant_id,
                                });
                                events.push(GameEvent::PogoStickLost { entity });
                            } else if !pogo_bouncing {
                                let target_x = grid_x(self.state.board.plants[plant_index].column)
                                    - 80 * POSITION_SCALE;
                                let zombie = &mut self.state.board.zombies[zombie_index];
                                zombie.pogo_counter = POGO_BOUNCE_TICKS;
                                zombie.pogo_target_x = Some(target_x);
                                zombie.pogo_velocity_x =
                                    (target_x - position_x) / i64::from(POGO_BOUNCE_TICKS);
                                zombie.eating = false;
                            }
                        } else if ztype == ZombieType::DolphinRider
                            && self.state.board.zombies[zombie_index].dolphin_phase == 1
                            && self.state.board.plants[plant_index].plant_type.slot() != 23
                        {
                            let zombie = &mut self.state.board.zombies[zombie_index];
                            zombie.dolphin_phase = 2;
                            zombie.dolphin_counter = DOLPHIN_JUMP_TIME;
                            zombie.dolphin_target_x = Some(
                                grid_x(self.state.board.plants[plant_index].column)
                                    - DOLPHIN_JUMP_TARGET_OFFSET,
                            );
                            zombie.speed = 0;
                            zombie.eating = false;
                            events.push(GameEvent::DolphinJumpStarted { entity });
                        } else if ztype == ZombieType::Zamboni {
                            self.state.board.plants.remove(plant_index);
                            self.state.board.zombies[zombie_index].eating = false;
                            events.push(GameEvent::PlantDied { entity: plant_id });
                        } else if is_gargantuar(ztype) {
                            let plant_type = self.state.board.plants[plant_index].plant_type;
                            if plant_type.slot() == 46 {
                                self.state.board.plants[plant_index].health -=
                                    GARGANTUAR_SPIKEROCK_DAMAGE;
                                let health_remaining = self.state.board.plants[plant_index].health;
                                events.push(GameEvent::PlantDamaged {
                                    entity: plant_id,
                                    damage: GARGANTUAR_SPIKEROCK_DAMAGE,
                                    health_remaining,
                                });
                                if health_remaining <= 0 {
                                    self.state.board.plants.remove(plant_index);
                                    events.push(GameEvent::PlantDied { entity: plant_id });
                                }
                            } else {
                                self.state.board.plants.remove(plant_index);
                                events.push(GameEvent::PlantDied { entity: plant_id });
                            }
                            self.state.board.zombies[zombie_index].eating = false;
                        } else if ztype == ZombieType::PoleVaulter && !has_vaulted {
                            if self.state.board.plants[plant_index].plant_type.slot() == 23 {
                                // A Tall-nut blocks the vault; the pole is spent
                                // and the vaulter falls back to walking.
                                self.state.board.zombies[zombie_index].has_vaulted = true;
                                self.state.board.zombies[zombie_index].eating = false;
                                events.push(GameEvent::JumpBlocked {
                                    zombie: entity,
                                    plant: plant_id,
                                });
                            } else {
                                self.state.board.zombies[zombie_index].has_vaulted = true;
                                self.state.board.zombies[zombie_index].eating = false;
                                events.push(GameEvent::ZombieVaulted { entity });
                            }
                        } else if self.state.board.plants[plant_index]
                            .plant_type
                            .is_hypno_shroom()
                        {
                            self.state.board.plants.remove(plant_index);
                            self.state.board.zombies[zombie_index].hypnotized = true;
                            self.state.board.zombies[zombie_index].eating = false;
                            events.push(GameEvent::ZombieHypnotized { entity });
                            events.push(GameEvent::PlantDied { entity: plant_id });
                        } else if self.state.board.plants[plant_index].plant_type.is_garlic() {
                            self.state.board.zombies[zombie_index].eating = true;
                            self.state.board.zombies[zombie_index].garlic_counter = 1;
                            self.state.board.zombies[zombie_index].garlic_target = Some(plant_id);
                        } else {
                            let plant_type = self.state.board.plants[plant_index].plant_type;
                            self.state.board.plants[plant_index].health -= ZOMBIE_BITE_DAMAGE;
                            let health_remaining = self.state.board.plants[plant_index].health;
                            events.push(GameEvent::PlantDamaged {
                                entity: plant_id,
                                damage: ZOMBIE_BITE_DAMAGE,
                                health_remaining,
                            });
                            if health_remaining <= 0 {
                                if plant_type.is_explode_o_nut() {
                                    let column = self.state.board.plants[plant_index].column;
                                    let center_x = grid_x(column);
                                    let explode_targets = self
                                        .state
                                        .board
                                        .zombies
                                        .iter()
                                        .filter(|z| {
                                            z.health > 0
                                                && z.row.abs_diff(row) <= 1
                                                && (z.position_x - center_x).abs()
                                                    <= 115 * POSITION_SCALE
                                        })
                                        .map(|z| z.id)
                                        .collect::<Vec<_>>();
                                    if !explode_targets.is_empty() {
                                        events.push(GameEvent::PlantSpecialTriggered {
                                            entity: plant_id,
                                            plant_type,
                                        });
                                    }
                                    for zombie_id in &explode_targets {
                                        // Apply damage without removing — the outer
                                        // zombie loop uses a fixed range, and
                                        // update_mowers retains dead zombies.
                                        let Some(target_idx) = self
                                            .state
                                            .board
                                            .zombies
                                            .iter()
                                            .position(|z| z.id == *zombie_id)
                                        else {
                                            continue;
                                        };
                                        self.damage_zombie(
                                            target_idx,
                                            PLANT_SPECIAL_DAMAGE,
                                            events,
                                        );
                                        let health_remaining =
                                            self.state.board.zombies[target_idx].health;
                                        events.push(GameEvent::PlantSpecialHit {
                                            plant: plant_id,
                                            zombie: *zombie_id,
                                            damage: PLANT_SPECIAL_DAMAGE,
                                            health_remaining,
                                        });
                                        if health_remaining <= 0 {
                                            self.state.board.zombies[target_idx].health = 0;
                                            self.emit_zombie_died(*zombie_id, events);
                                        }
                                    }
                                }
                                self.state.board.plants.remove(plant_index);
                                // Find the eating zombie by ID — the explosion may have
                                // shifted indices or removed it entirely.
                                if let Some(pos) =
                                    self.state.board.zombies.iter().position(|z| z.id == entity)
                                {
                                    self.state.board.zombies[pos].eating = false;
                                }
                                events.push(GameEvent::PlantDied { entity: plant_id });
                            }
                        }
                    } else if was_eating {
                        self.state.board.zombies[zombie_index].eating = false;
                    }
                }
            }

            if self.state.board.zombies[zombie_index].zombie_type == ZombieType::PeaHead
                && self.state.board.zombies[zombie_index].pea_head_counter == 0
            {
                self.state.board.zombies[zombie_index].pea_head_counter =
                    ZOMBIE_PEA_HEAD_RELOAD_TICKS;
                self.fire_projectile(
                    entity,
                    ProjectileType::ZombiePea,
                    row,
                    ProjectileTrajectory {
                        motion: ProjectileMotion::Backwards,
                        position_x: position_x + 20 * POSITION_SCALE,
                        position_y: grid_y(row),
                        velocity_x: -3_330_000,
                        velocity_y: 0,
                    },
                    events,
                );
            }
            if self.state.board.zombies[zombie_index].zombie_type == ZombieType::Catapult {
                let target = self.find_catapult_target(row, position_x);
                let zombie = &self.state.board.zombies[zombie_index];
                if zombie.catapult_shots == 0 {
                    self.state.board.zombies[zombie_index].catapult_armed = false;
                } else {
                    if !zombie.catapult_armed
                        && position_x <= 650 * POSITION_SCALE
                        && target.is_some()
                    {
                        let zombie = &mut self.state.board.zombies[zombie_index];
                        zombie.catapult_armed = true;
                        zombie.catapult_counter = CATAPULT_LAUNCH_TICKS;
                        zombie.eating = false;
                    } else if zombie.catapult_armed && zombie.catapult_counter > 0 {
                        let counter = zombie.catapult_counter.saturating_sub(1);
                        self.state.board.zombies[zombie_index].catapult_counter = counter;
                        if counter == 0 {
                            if let Some(plant_index) = target {
                                let target_x = grid_x(self.state.board.plants[plant_index].column);
                                self.fire_catapult_projectile(
                                    entity, row, position_x, target_x, events,
                                );
                                let zombie = &mut self.state.board.zombies[zombie_index];
                                zombie.catapult_shots -= 1;
                                zombie.catapult_counter = CATAPULT_RELOAD_TICKS;
                                zombie.catapult_armed = zombie.catapult_shots > 0;
                            } else {
                                self.state.board.zombies[zombie_index].catapult_armed = false;
                            }
                        }
                    }
                }
            }

            // GridItem portals: a zombie reaching a portal cell teleports to
            // the next portal in the pair cycle.
            {
                let cooldown = self.state.board.zombies[zombie_index].portal_cooldown;
                if cooldown > 0 {
                    self.state.board.zombies[zombie_index].portal_cooldown = cooldown - 1;
                } else if self.state.board.portals.len() >= 2 {
                    let hit = self.state.board.portals.iter().position(
                        |(portal_row, portal_column, _)| {
                            *portal_row == row
                                && (position_x - grid_x(*portal_column)).abs()
                                    <= 10 * POSITION_SCALE
                        },
                    );
                    if let Some(index) = hit {
                        let (target_row, target_column, _) =
                            self.state.board.portals[(index + 1) % self.state.board.portals.len()];
                        let zombie = &mut self.state.board.zombies[zombie_index];
                        zombie.row = target_row;
                        zombie.position_x = grid_x(target_column);
                        zombie.portal_cooldown = 100;
                        events.push(GameEvent::ZombieTeleported {
                            entity,
                            row: target_row,
                            column: target_column,
                        });
                        continue;
                    }
                }
            }
            // GridItem rake: the first zombie that steps on it dies and
            // consumes it.
            if let Some((rake_row, rake_column)) = self.state.board.rake
                && rake_row == row
                && position_x <= grid_x(rake_column) + 30 * POSITION_SCALE
                && self.state.board.zombies[zombie_index].health > 0
            {
                self.state.board.rake = None;
                events.push(GameEvent::RakeTriggered { zombie: entity });
                self.emit_zombie_died(entity, events);
                self.state.board.zombies[zombie_index].departed = true;
                continue;
            }
            let mower_covering_row = self
                .state
                .board
                .mowers
                .iter()
                .any(|mower| mower.row == row && mower.active);
            let mower_triggered = position_x <= MOWER_TRIGGER_X
                && (self.trigger_mower(row, events) || mower_covering_row);
            let fled = self
                .state
                .board
                .zombies
                .get(zombie_index)
                .is_some_and(|zombie| zombie.yeti_running && zombie.position_x > YETI_FLEE_EDGE);
            if fled {
                let entity = self.state.board.zombies[zombie_index].id;
                self.state.board.zombies[zombie_index].departed = true;
                events.push(GameEvent::ZombieFled { entity });
                continue;
            }
            if !mower_triggered
                && self.state.board.zombies[zombie_index].health > 0
                && position_x <= -100 * POSITION_SCALE
            {
                if self.state.mode == ModeKind::IZombie {
                    if self.eat_brain(zombie_index, row, events) {
                        continue;
                    }
                    self.state.board.zombies[zombie_index].health = 0;
                    continue;
                }
                self.state.scene = SceneKind::GameOver;
                events.push(GameEvent::GameLost { zombie: entity });
                break;
            }
        }
        self.state.board.zombies.retain(|zombie| !zombie.departed);
    }

    fn apply_cob_explosion(
        &mut self,
        projectile: &ProjectileState,
        target_row: u8,
        target_x: i64,
        events: &mut Vec<GameEvent>,
    ) {
        self.push_projectile_impact(projectile.id, None, ProjectileImpactSound::Splat, 3, events);
        let mut targets = self
            .state
            .board
            .zombies
            .iter()
            .filter(|zombie| {
                // Cob blasts carry every damage-range flag: airborne and submerged
                // zombies are all valid targets, but not bungee-held deliveries.
                zombie.health > 0
                    && !zombie.bungee_held
                    && zombie.row.abs_diff(target_row) <= 1
                    && (zombie.position_x - target_x).abs() <= 115 * POSITION_SCALE
            })
            .map(|zombie| (zombie.id, (zombie.position_x - target_x).abs()))
            .collect::<Vec<_>>();
        targets.sort_by_key(|(_, distance)| *distance);
        for (target_index, (zombie_id, _)) in targets.into_iter().enumerate() {
            let Some(zombie_index) = self
                .state
                .board
                .zombies
                .iter()
                .position(|zombie| zombie.id == zombie_id)
            else {
                continue;
            };
            self.damage_zombie(zombie_index, projectile.damage, events);
            let health_remaining = self.state.board.zombies[zombie_index].health;
            if target_index == 0 {
                events.push(GameEvent::ProjectileHit {
                    projectile: projectile.id,
                    zombie: zombie_id,
                    damage: projectile.damage,
                    health_remaining,
                });
            } else {
                events.push(GameEvent::ProjectileSplashHit {
                    projectile: projectile.id,
                    zombie: zombie_id,
                    damage: projectile.damage,
                    health_remaining,
                });
            }
            if health_remaining <= 0 {
                self.emit_zombie_died(zombie_id, events);
                self.state.board.zombies.remove(zombie_index);
            }
        }
    }

    fn update_projectiles(&mut self, events: &mut Vec<GameEvent>) {
        let mut projectile_index = 0;
        while projectile_index < self.state.board.projectiles.len() {
            if self.state.board.projectiles[projectile_index].motion == ProjectileMotion::Homing {
                self.steer_homing_projectile(projectile_index);
            }
            {
                let projectile = &mut self.state.board.projectiles[projectile_index];
                projectile.age = projectile.age.saturating_add(1);
                projectile.position_x += projectile.velocity_x;
                projectile.position_y += projectile.velocity_y;
            }
            if self.state.board.projectiles[projectile_index].projectile_type == ProjectileType::Cob
            {
                {
                    let projectile = &mut self.state.board.projectiles[projectile_index];
                    if projectile.lob_height < -700 {
                        projectile.lob_velocity = 8;
                        projectile.row = projectile.target_row.unwrap_or(projectile.row);
                        projectile.position_x =
                            projectile.target_x.unwrap_or(projectile.position_x);
                        projectile.position_y = grid_y(projectile.row);
                    }
                    projectile.lob_height += projectile.lob_velocity;
                }
                let projectile = self.state.board.projectiles[projectile_index].clone();
                if projectile.lob_velocity > 0 && projectile.lob_height > -40 {
                    self.apply_cob_explosion(
                        &projectile,
                        projectile.target_row.unwrap_or(projectile.row),
                        projectile.target_x.unwrap_or(projectile.position_x),
                        events,
                    );
                    self.state.board.projectiles.remove(projectile_index);
                } else {
                    projectile_index += 1;
                }
                continue;
            }
            if self.state.board.projectiles[projectile_index].motion == ProjectileMotion::Puff
                && self.state.board.projectiles[projectile_index].age >= PUFF_PROJECTILE_MAX_AGE
            {
                self.state.board.projectiles.remove(projectile_index);
                continue;
            }
            if matches!(
                self.state.board.projectiles[projectile_index].motion,
                ProjectileMotion::Fume | ProjectileMotion::Gloom
            ) {
                let projectile = self.state.board.projectiles[projectile_index].clone();
                if projectile.motion == ProjectileMotion::Fume {
                    self.apply_fume_damage(&projectile, events);
                } else {
                    self.apply_gloom_damage(&projectile, events);
                }
                self.state.board.projectiles.remove(projectile_index);
                continue;
            }
            if self.state.board.projectiles[projectile_index].projectile_type
                == ProjectileType::ZombiePea
            {
                let projectile = self.state.board.projectiles[projectile_index].clone();
                let projectile_row = projectile_row(projectile.position_y, self.state.board.rows);
                let target = self
                    .state
                    .board
                    .plants
                    .iter()
                    .enumerate()
                    .filter(|(_, plant)| Some(plant.row) == projectile_row && plant.health > 0)
                    .filter(|(_, plant)| {
                        projectile_hits_plant(projectile.position_x, grid_x(plant.column))
                    })
                    .max_by_key(|(_, plant)| grid_x(plant.column))
                    .map(|(index, _)| index);

                if let Some(plant_index) = target {
                    let plant_id = self.state.board.plants[plant_index].id;
                    self.state.board.plants[plant_index].health -= projectile.damage;
                    let health_remaining = self.state.board.plants[plant_index].health;
                    events.push(GameEvent::PlantDamaged {
                        entity: plant_id,
                        damage: projectile.damage,
                        health_remaining,
                    });
                    if health_remaining <= 0 {
                        self.state.board.plants.remove(plant_index);
                        events.push(GameEvent::PlantDied { entity: plant_id });
                    }
                    self.state.board.projectiles.remove(projectile_index);
                } else if projectile.position_x > i64::from(LOGICAL_WIDTH) * POSITION_SCALE
                    || projectile.position_x < -100 * POSITION_SCALE
                    || projectile_row.is_none()
                {
                    self.state.board.projectiles.remove(projectile_index);
                } else {
                    projectile_index += 1;
                }
                continue;
            }
            if self.state.board.projectiles[projectile_index].projectile_type
                == ProjectileType::Other(1)
            {
                let projectile = self.state.board.projectiles[projectile_index].clone();
                let target_reached = projectile.age >= 120
                    || projectile
                        .target_x
                        .is_some_and(|target_x| projectile.position_x <= target_x);
                if target_reached {
                    let projectile_row =
                        projectile_row(projectile.position_y, self.state.board.rows);
                    let target = self
                        .state
                        .board
                        .plants
                        .iter()
                        .enumerate()
                        .filter(|(_, plant)| Some(plant.row) == projectile_row && plant.health > 0)
                        .filter(|(_, plant)| {
                            projectile
                                .target_x
                                .is_some_and(|target_x| grid_x(plant.column) == target_x)
                        })
                        .map(|(index, _)| index)
                        .next();
                    if let Some(plant_index) = target {
                        let plant_id = self.state.board.plants[plant_index].id;
                        self.state.board.plants[plant_index].health -= projectile.damage;
                        let health_remaining = self.state.board.plants[plant_index].health;
                        events.push(GameEvent::PlantDamaged {
                            entity: plant_id,
                            damage: projectile.damage,
                            health_remaining,
                        });
                        if health_remaining <= 0 {
                            self.state.board.plants.remove(plant_index);
                            events.push(GameEvent::PlantDied { entity: plant_id });
                        }
                    }
                    self.state.board.projectiles.remove(projectile_index);
                } else {
                    projectile_index += 1;
                }
                continue;
            }
            self.apply_torchwood(projectile_index);
            let projectile = self.state.board.projectiles[projectile_index].clone();
            let projectile_row = projectile_row(projectile.position_y, self.state.board.rows);
            let target = self
                .state
                .board
                .zombies
                .iter()
                .enumerate()
                .filter(|(_, zombie)| {
                    Some(zombie.row) == projectile_row
                        && zombie.health > 0
                        && projectile_can_hit_zombie(zombie, projectile.projectile_type)
                })
                .filter(|(_, zombie)| projectile_hits(projectile.position_x, zombie.position_x))
                .min_by_key(|(_, zombie)| zombie.position_x)
                .map(|(index, _)| index);

            if let Some(zombie_index) = target {
                let zombie_id = self.state.board.zombies[zombie_index].id;
                let target_zombie = self.state.board.zombies[zombie_index].clone();
                self.emit_projectile_impact(&projectile, Some(&target_zombie), events);
                self.damage_zombie(zombie_index, projectile.damage, events);
                let health_remaining = self.state.board.zombies[zombie_index].health;
                events.push(GameEvent::ProjectileHit {
                    projectile: projectile.id,
                    zombie: zombie_id,
                    damage: projectile.damage,
                    health_remaining,
                });
                self.apply_projectile_chill(zombie_id, projectile.projectile_type, events);
                if health_remaining <= 0 {
                    // ponytail: remove terminal entities now; add death phases when rendering consumes them.
                    self.emit_zombie_died(zombie_id, events);
                    self.state.board.zombies.remove(zombie_index);
                }
                if projectile.projectile_type.is_splash() {
                    self.apply_splash_damage(&projectile, zombie_id, events);
                }
                self.state.board.projectiles.remove(projectile_index);
            } else if projectile.position_x > i64::from(LOGICAL_WIDTH) * POSITION_SCALE
                || projectile.position_x < -100 * POSITION_SCALE
                || projectile_row.is_none()
            {
                self.state.board.projectiles.remove(projectile_index);
            } else {
                projectile_index += 1;
            }
        }
    }

    fn apply_torchwood(&mut self, projectile_index: usize) {
        let projectile_type = self.state.board.projectiles[projectile_index].projectile_type;
        if !matches!(
            projectile_type,
            ProjectileType::Pea | ProjectileType::SnowPea
        ) {
            return;
        }

        let projectile = &self.state.board.projectiles[projectile_index];
        let projectile_row = projectile.row;
        let previous_x = projectile.position_x - projectile.velocity_x;
        let current_x = projectile.position_x;
        let (left_x, right_x) = if previous_x <= current_x {
            (previous_x, current_x)
        } else {
            (current_x, previous_x)
        };
        if !self.state.board.plants.iter().any(|plant| {
            let torchwood_x = grid_x(plant.column);
            plant.row == projectile_row
                && plant.plant_type.is_torchwood()
                && left_x <= torchwood_x
                && torchwood_x <= right_x
        }) {
            return;
        }

        // ponytail: upgrade only the verified pea-family shots here; widen the bullet matrix once
        // the remaining torchwood cases are observed locally.
        let projectile = &mut self.state.board.projectiles[projectile_index];
        projectile.projectile_type = ProjectileType::Fireball;
        projectile.damage = ProjectileType::Fireball.damage();
    }

    fn emit_projectile_impact(
        &mut self,
        projectile: &ProjectileState,
        zombie: Option<&ZombieState>,
        events: &mut Vec<GameEvent>,
    ) {
        let zombie_id = zombie.map(|zombie| zombie.id);
        let mut emit = |kind: ProjectileImpactSound, variants| {
            self.push_projectile_impact(projectile.id, zombie_id, kind, variants, events);
        };
        let helm = zombie.and_then(|zombie| {
            if !zombie.armor_intact {
                return None;
            }
            Some(match zombie.zombie_type {
                ZombieType::Buckethead => ProjectileImpactSound::Shield,
                ZombieType::Conehead | ZombieType::Digger | ZombieType::Football => {
                    ProjectileImpactSound::Plastic
                }
                _ => return None,
            })
        });
        let fireball_splash = zombie
            .map(|zombie| {
                !matches!(
                    zombie.zombie_type,
                    ZombieType::Catapult
                        | ZombieType::Zamboni
                        | ZombieType::ScreenDoor
                        | ZombieType::Ladder
                )
            })
            .unwrap_or(true);
        match projectile.projectile_type {
            ProjectileType::Kernel => emit(ProjectileImpactSound::Kernel, 2),
            ProjectileType::Butter => {
                emit(ProjectileImpactSound::Butter, 1);
                if let Some(helm) = helm {
                    emit(helm, 2);
                }
            }
            ProjectileType::Fireball if fireball_splash => emit(ProjectileImpactSound::Ignite, 4),
            ProjectileType::Fireball => match helm {
                Some(ProjectileImpactSound::Shield) => emit(ProjectileImpactSound::Shield, 2),
                Some(ProjectileImpactSound::Plastic) => {
                    emit(ProjectileImpactSound::Plastic, 2);
                    emit(ProjectileImpactSound::Splat, 3);
                }
                None => emit(ProjectileImpactSound::Splat, 3),
                Some(kind) => emit(kind, 1),
            },
            ProjectileType::Melon | ProjectileType::WinterMelon => {
                emit(ProjectileImpactSound::Melon, 2);
                if let Some(helm) = helm {
                    emit(helm, 2);
                }
            }
            _ => match helm {
                Some(ProjectileImpactSound::Shield) => emit(ProjectileImpactSound::Shield, 2),
                Some(ProjectileImpactSound::Plastic) => {
                    emit(ProjectileImpactSound::Plastic, 2);
                    emit(ProjectileImpactSound::Splat, 3);
                }
                None => emit(ProjectileImpactSound::Splat, 3),
                Some(kind) => emit(kind, 1),
            },
        }
    }

    fn push_projectile_impact(
        &mut self,
        projectile: EntityId,
        zombie: Option<EntityId>,
        kind: ProjectileImpactSound,
        variants: u32,
        events: &mut Vec<GameEvent>,
    ) {
        events.push(GameEvent::ProjectileImpact {
            projectile,
            zombie,
            kind,
            variant: self.rng.range(variants) as u8,
        });
    }

    fn steer_homing_projectile(&mut self, projectile_index: usize) {
        let projectile = &self.state.board.projectiles[projectile_index];
        let current_row =
            projectile_row(projectile.position_y, self.state.board.rows).unwrap_or(projectile.row);
        let target_row = self
            .state
            .board
            .zombies
            .iter()
            .filter(|zombie| {
                zombie.health > 0 && projectile_can_hit_zombie(zombie, projectile.projectile_type)
            })
            .min_by_key(|zombie| {
                (
                    (zombie.position_x - projectile.position_x).abs(),
                    zombie.row.abs_diff(current_row),
                )
            })
            .map(|zombie| zombie.row);
        let Some(target_row) = target_row else {
            return;
        };
        let projectile = &mut self.state.board.projectiles[projectile_index];
        projectile.velocity_y = match target_row.cmp(&current_row) {
            std::cmp::Ordering::Less => -3_330_000,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 3_330_000,
        };
    }

    fn apply_projectile_chill(
        &mut self,
        zombie_id: EntityId,
        projectile_type: ProjectileType,
        events: &mut Vec<GameEvent>,
    ) {
        if projectile_type == ProjectileType::Butter {
            // Zombie::ApplyButter 0x5326D0: 400-tick immobilize, refused by
            // Zamboni, the Boss, sledded bobsleds, and airborne fliers.
            if let Some(zombie) = self
                .state
                .board
                .zombies
                .iter_mut()
                .find(|zombie| zombie.id == zombie_id)
                && !matches!(zombie.zombie_type, ZombieType::Zamboni | ZombieType::Boss)
                && !(zombie.zombie_type == ZombieType::Bobsled && zombie.bobsled_sliding)
                && !balloon_is_airborne(zombie)
            {
                zombie.frozen_counter = zombie.frozen_counter.max(BUTTER_TICKS);
                events.push(GameEvent::ZombieButtered { entity: zombie_id });
            }
            return;
        }
        let duration = projectile_type.chill_duration();
        if duration == 0 {
            return;
        }
        let Some(zombie) = self
            .state
            .board
            .zombies
            .iter_mut()
            .find(|zombie| zombie.id == zombie_id)
        else {
            return;
        };
        zombie.chilled_counter = zombie.chilled_counter.max(duration);
        events.push(GameEvent::ZombieChilled {
            entity: zombie_id,
            duration,
        });
    }

    fn apply_splash_damage(
        &mut self,
        projectile: &ProjectileState,
        primary_zombie: EntityId,
        events: &mut Vec<GameEvent>,
    ) {
        let Some(row) = projectile_row(projectile.position_y, self.state.board.rows) else {
            return;
        };
        let splash_damage = projectile.projectile_type.splash_damage();
        let target_ids = self
            .state
            .board
            .zombies
            .iter()
            .filter(|zombie| {
                zombie.id != primary_zombie
                    && zombie.health > 0
                    && projectile_can_hit_zombie(zombie, projectile.projectile_type)
                    && zombie.row.abs_diff(row) <= 1
                    && projectile_hits(projectile.position_x, zombie.position_x)
            })
            .map(|zombie| zombie.id)
            .collect::<Vec<_>>();

        for zombie_id in target_ids {
            let Some(zombie_index) = self
                .state
                .board
                .zombies
                .iter()
                .position(|zombie| zombie.id == zombie_id)
            else {
                continue;
            };
            self.damage_zombie(zombie_index, splash_damage, events);
            let health_remaining = self.state.board.zombies[zombie_index].health;
            events.push(GameEvent::ProjectileSplashHit {
                projectile: projectile.id,
                zombie: zombie_id,
                damage: splash_damage,
                health_remaining,
            });
            self.apply_projectile_chill(zombie_id, projectile.projectile_type, events);
            if health_remaining <= 0 {
                self.emit_zombie_died(zombie_id, events);
                self.state.board.zombies.remove(zombie_index);
            }
        }
    }

    fn apply_gloom_damage(&mut self, projectile: &ProjectileState, events: &mut Vec<GameEvent>) {
        let target_ids = self
            .state
            .board
            .zombies
            .iter()
            .filter(|zombie| {
                zombie.health > 0
                    && projectile_can_hit_zombie(zombie, projectile.projectile_type)
                    && zombie.row.abs_diff(projectile.row) <= GLOOM_ROW_RADIUS
                    && zombie.position_x > projectile.position_x
                    && zombie.position_x
                        < projectile.position_x + GLOOM_ATTACK_RANGE * POSITION_SCALE
            })
            .map(|zombie| zombie.id)
            .collect::<Vec<_>>();

        for zombie_id in target_ids {
            let Some(zombie_index) = self
                .state
                .board
                .zombies
                .iter()
                .position(|zombie| zombie.id == zombie_id)
            else {
                continue;
            };
            self.damage_zombie_bypassing_shield(zombie_index, projectile.damage, false, events);
            let health_remaining = self.state.board.zombies[zombie_index].health;
            events.push(GameEvent::ProjectileHit {
                projectile: projectile.id,
                zombie: zombie_id,
                damage: projectile.damage,
                health_remaining,
            });
            if health_remaining <= 0 {
                self.emit_zombie_died(zombie_id, events);
                self.state.board.zombies.remove(zombie_index);
            }
        }
    }

    fn apply_fume_damage(&mut self, projectile: &ProjectileState, events: &mut Vec<GameEvent>) {
        let target_ids = self
            .state
            .board
            .zombies
            .iter()
            .filter(|zombie| {
                zombie.health > 0
                    && projectile_can_hit_zombie(zombie, projectile.projectile_type)
                    && zombie.row == projectile.row
                    && zombie.position_x > projectile.position_x
                    && zombie.position_x
                        < projectile.position_x + FUME_ATTACK_RANGE * POSITION_SCALE
            })
            .map(|zombie| zombie.id)
            .collect::<Vec<_>>();

        for zombie_id in target_ids {
            let Some(zombie_index) = self
                .state
                .board
                .zombies
                .iter()
                .position(|zombie| zombie.id == zombie_id)
            else {
                continue;
            };
            self.damage_zombie_bypassing_shield(zombie_index, projectile.damage, false, events);
            let health_remaining = self.state.board.zombies[zombie_index].health;
            events.push(GameEvent::ProjectileHit {
                projectile: projectile.id,
                zombie: zombie_id,
                damage: projectile.damage,
                health_remaining,
            });
            if health_remaining <= 0 {
                self.emit_zombie_died(zombie_id, events);
                self.state.board.zombies.remove(zombie_index);
            }
        }
    }

    fn update_seed_packets(&mut self, events: &mut Vec<GameEvent>) {
        for packet in &mut self.state.board.seed_packets {
            let was_refreshing = packet.refresh_remaining > 0;
            packet.refresh_remaining = packet.refresh_remaining.saturating_sub(1);
            if was_refreshing && packet.refresh_remaining == 0 {
                events.push(GameEvent::SeedPacketReady {
                    slot: packet.slot,
                    plant_type: packet.plant_type,
                });
            }
        }
    }

    fn update_craters(&mut self) {
        for crater in &mut self.state.board.craters {
            crater.remaining = crater.remaining.saturating_sub(1);
        }
        self.state
            .board
            .craters
            .retain(|crater| crater.remaining != 0);
    }

    fn update_sun_spawning(&mut self, events: &mut Vec<GameEvent>) {
        self.state.board.sun_countdown = self.state.board.sun_countdown.saturating_sub(1);
        if self.state.board.sun_countdown != 0 {
            return;
        }

        self.state.board.suns_fallen = self.state.board.suns_fallen.saturating_add(1);
        self.state.board.sun_countdown = (SUN_COUNTDOWN
            + self.state.board.suns_fallen.saturating_mul(10))
        .min(SUN_COUNTDOWN_MAX)
            + self.rng.range(SUN_COUNTDOWN_RANGE);
        let position_x = i64::from(self.rng.range_inclusive(100, 649)) * POSITION_SCALE;
        self.spawn_sun(SunSource::Sky, position_x, 60 * POSITION_SCALE, events);
        let ground_y = self.rng.range(250);
        if let Some(sun) = self.state.board.suns.last_mut() {
            // COIN_MOTION_FROM_SKY: the sun drifts down at 0.67 per tick to a
            // ground stop in the randomized band.
            sun.target_y = Some((300 + i64::from(ground_y)) * POSITION_SCALE);
        }
    }

    fn update_suns(&mut self) {
        for sun in &mut self.state.board.suns {
            if let Some(target_y) = sun.target_y {
                if sun.velocity_y != 0 || sun.velocity_x != 0 {
                    sun.position_x += sun.velocity_x;
                    sun.position_y += sun.velocity_y;
                    sun.velocity_y += SUN_GRAVITY;
                    if sun.velocity_y > 0 && sun.position_y >= target_y {
                        sun.position_y = target_y;
                        sun.target_y = None;
                        sun.velocity_x = 0;
                        sun.velocity_y = 0;
                    }
                } else {
                    sun.position_y += SUN_FALL_SPEED;
                    if sun.position_y >= target_y {
                        sun.position_y = target_y;
                        sun.target_y = None;
                    }
                }
            }
        }
        for coin in &mut self.state.board.coins {
            if let Some(target_y) = coin.target_y {
                if coin.position_y + coin.velocity_y < target_y {
                    coin.position_x += coin.velocity_x;
                    coin.position_y += coin.velocity_y;
                    coin.velocity_y += COIN_GRAVITY;
                } else {
                    coin.position_y = target_y;
                    coin.target_y = None;
                    coin.velocity_x = 0;
                    coin.velocity_y = 0;
                }
            }
        }
    }

    fn update_wave_spawning(&mut self, events: &mut Vec<GameEvent>) {
        // Challenge::UpdateZombieSpawning claims spawning on Scary Potter
        // levels, so the board wave clock never runs on adventure 4-5.
        if self.state.mode == ModeKind::Adventure && self.state.level == 35 {
            return;
        }
        if self.state.board.wave.current >= self.state.board.wave.total {
            if self.state.board.wave.endless {
                self.state.board.wave.current = self.state.board.wave.total.saturating_sub(1);
                self.state.board.wave.countdown = FIRST_WAVE_COUNTDOWN;
                self.state.board.wave.countdown_start = FIRST_WAVE_COUNTDOWN;
            } else {
                return;
            }
        }
        if self.state.board.wave.current >= self.state.board.wave.total {
            return;
        }
        let adventure =
            self.state.mode == ModeKind::Adventure && (1..=50).contains(&self.state.level);
        // The huge-wave banner freezes the spawn clock for 750 ticks, then
        // the wave releases immediately.
        if adventure && self.state.board.huge_wave_countdown > 0 {
            self.state.board.huge_wave_countdown -= 1;
            if self.state.board.huge_wave_countdown > 0 {
                return;
            }
            self.state.board.wave.countdown = 1;
        }
        self.state.board.wave.countdown = self.state.board.wave.countdown.saturating_sub(1);
        if adventure
            && self.state.board.wave.countdown > 200
            && self.state.board.wave.countdown_start > self.state.board.wave.countdown + 400
            && self.state.board.wave_health_threshold >= 0
            && self.current_wave_health() <= self.state.board.wave_health_threshold
        {
            self.state.board.wave.countdown = 200;
        }
        if adventure
            && self.state.board.wave.countdown == 5
            && adventure_is_flag_wave(self.state.level, false, self.state.board.wave.current)
        {
            self.state.board.huge_wave_countdown = 750;
            return;
        }
        if self.state.board.wave.countdown != 0 {
            return;
        }

        let wave = self.state.board.wave.current;
        events.push(GameEvent::WaveStarted { wave });
        let row = self.rng.range(u32::from(self.state.board.rows)) as u8;
        match self.state.challenge.kind {
            ChallengeKind::BobsledBonanza => {
                // Bobsleds only spawn on iced rows; the source wave list leads
                // with Zombonis, whose Bonanza trails never expire.
                if let Some(ice_row) = self.pick_bobsled_row() {
                    self.spawn_bobsled_zombie(ice_row, wave, None, events);
                } else {
                    let zamboni_row = self.pick_spawn_row(ZombieType::Zamboni, wave);
                    self.spawn_zamboni_zombie(zamboni_row, wave, None, events);
                }
            }
            ChallengeKind::PogoParty => {
                self.spawn_pogo_zombie(row, wave, None, events);
            }
            ChallengeKind::WarAndPeas => {
                if wave.is_multiple_of(2) {
                    self.spawn_pea_head_zombie(row, wave, None, events);
                } else {
                    self.spawn_wallnut_head_zombie(row, wave, None, events);
                }
            }
            ChallengeKind::WarAndPeas2 => match wave % 6 {
                0 => {
                    self.spawn_pea_head_zombie(row, wave, None, events);
                }
                1 => {
                    self.spawn_wallnut_head_zombie(row, wave, None, events);
                }
                2 => {
                    self.spawn_jalapeno_head_zombie(row, wave, None, events);
                }
                3 => {
                    self.spawn_gatling_head_zombie(row, wave, None, events);
                }
                4 => {
                    self.spawn_squash_head_zombie(row, wave, None, events);
                }
                _ => {
                    self.spawn_tallnut_head_zombie(row, wave, None, events);
                }
            },
            _ => {
                if self.state.mode == ModeKind::Adventure && (1..=50).contains(&self.state.level) {
                    if self.state.board.wave_plan.is_empty() {
                        let level = self.state.level;
                        let plan = self.pick_adventure_waves(level, false);
                        self.state.board.wave_plan = plan;
                    }
                    let plan = self
                        .state
                        .board
                        .wave_plan
                        .get(wave as usize)
                        .cloned()
                        .unwrap_or_default();
                    for zombie_type in plan {
                        self.spawn_adventure_zombie(zombie_type, wave, events);
                    }
                } else {
                    let normal_row = if self.state.scene == SceneKind::Pool {
                        self.pick_spawn_row(ZombieType::Normal, wave)
                    } else {
                        row
                    };
                    self.spawn_normal_zombie(normal_row, wave, None, events);
                }
            }
        }
        self.state.board.wave.current += 1;
        self.state.board.wave.countdown_start = 0;
        if self.state.mode == ModeKind::Adventure
            && (1..=50).contains(&self.state.level)
            && self.state.board.wave.current < self.state.board.wave.total
        {
            let countdown = ZOMBIE_NEXT_WAVE_COUNTDOWN + self.rng.range(ZOMBIE_NEXT_WAVE_RANGE);
            self.state.board.wave.countdown = countdown;
            self.state.board.wave.countdown_start = countdown;
            // Board::UpdateZombieSpawning: the next wave releases early once
            // this wave's health falls to a random 50-65% of its start.
            let start_health = self.current_wave_health();
            let percent = 50 + self.rng.range(16) as i32;
            self.state.board.wave_health_threshold = start_health * percent / 100;
        }
        // The final wave schedules the scene-routed rise 210 ticks later:
        // gravestones on night boards, pool emerges, or the roof sky drop.
        if matches!(
            self.state.scene,
            SceneKind::Roof | SceneKind::Night | SceneKind::Pool
        ) && self.state.board.wave.current >= self.state.board.wave.total
            && !self.state.board.wave.endless
        {
            self.state.board.sky_drop_countdown = SKY_DROP_DELAY_TICKS;
        }
    }

    /// Board::PickGraveRisingZombieType: Normal/Conehead at 4000 each, with
    /// the 3000-weight Buckethead only off gravestone stages.
    fn pick_grave_rising_type(&mut self, include_pail: bool) -> ZombieType {
        let total = if include_pail { 11_000 } else { 8_000 };
        let roll = self.rng.range(total);
        if roll < 4_000 {
            ZombieType::Normal
        } else if roll < 8_000 {
            ZombieType::Conehead
        } else {
            ZombieType::Buckethead
        }
    }

    fn spawn_rising_zombie(
        &mut self,
        zombie_type: ZombieType,
        row: u8,
        column: u8,
        wave: u32,
        events: &mut Vec<GameEvent>,
    ) {
        let health = match zombie_type {
            ZombieType::Conehead => 640,
            ZombieType::Buckethead => 1_370,
            _ => 270,
        };
        let position = grid_x(column) - 25 * POSITION_SCALE;
        self._spawn_zombie_inner(zombie_type, health, row, wave, Some(position), events);
    }

    /// TotalZombiesHealthInWave for the most recently spawned wave: body plus
    /// a fifth of the shield, skipping bungees and the dead.
    fn current_wave_health(&self) -> i32 {
        let wave = self.state.board.wave.current.saturating_sub(1);
        self.state
            .board
            .zombies
            .iter()
            .filter(|zombie| {
                zombie.from_wave == wave
                    && zombie.health > 0
                    && !zombie.hypnotized
                    && zombie.zombie_type != ZombieType::Bungee
            })
            .map(|zombie| zombie.health + zombie.shield_health / 5)
            .sum()
    }

    fn update_sky_drop(&mut self, events: &mut Vec<GameEvent>) {
        if self.state.board.sky_drop_countdown == 0 {
            return;
        }
        self.state.board.sky_drop_countdown -= 1;
        if self.state.board.sky_drop_countdown != 0 {
            return;
        }
        let wave = self.state.board.wave.current.saturating_sub(1);
        match self.state.scene {
            SceneKind::Night => {
                // One riser per gravestone; night stages never roll pails.
                let graves: Vec<(u8, u8)> = self
                    .state
                    .board
                    .graves
                    .iter()
                    .map(|grave| (grave.row, grave.column))
                    .collect();
                for (row, column) in graves {
                    let zombie_type = self.pick_grave_rising_type(false);
                    self.spawn_rising_zombie(zombie_type, row, column, wave, events);
                }
                return;
            }
            SceneKind::Pool => {
                let count = if matches!(self.state.level, 21 | 22 | 31 | 32) {
                    2
                } else {
                    3
                };
                let mut cells: Vec<(u8, u8)> = (2u8..=3)
                    .flat_map(|row| (5u8..=8).map(move |column| (row, column)))
                    .collect();
                for _ in 0..count {
                    let pick = self.rng.range(cells.len() as u32) as usize;
                    let (row, column) = cells.remove(pick);
                    let zombie_type = self.pick_grave_rising_type(true);
                    self.spawn_rising_zombie(zombie_type, row, column, wave, events);
                }
                return;
            }
            _ => {}
        }
        for _ in 0..3 {
            let roll = self.rng.range(11_000);
            let zombie_type = if roll < 4_000 {
                ZombieType::Normal
            } else if roll < 8_000 {
                ZombieType::Conehead
            } else {
                ZombieType::Buckethead
            };
            let column = 4 + self.rng.range(5) as u8;
            let row_limit = u32::from(self.state.board.rows.min(5));
            let row = self.rng.range(row_limit) as u8;
            self.spawn_bungee_drop(zombie_type, row, column, wave, events);
        }
    }

    fn apply_spikeweed_damage(
        &mut self,
        plant_id: EntityId,
        row: u8,
        column: u8,
        events: &mut Vec<GameEvent>,
    ) -> bool {
        let mut vehicle_hit = false;
        let mut zombie_index = 0;
        while zombie_index < self.state.board.zombies.len() {
            let zombie = &self.state.board.zombies[zombie_index];
            if zombie.health <= 0 || zombie.row != row || !spikeweed_hits(zombie.position_x, column)
            {
                zombie_index += 1;
                continue;
            }
            let zombie_id = zombie.id;
            // Spike damage against the Zomboni and Catapult vehicles is raised to
            // 1800 and pops the tires; the spiky plant pays for it below.
            let damage = if matches!(
                zombie.zombie_type,
                ZombieType::Zamboni | ZombieType::Catapult
            ) {
                vehicle_hit = true;
                SPIKE_VEHICLE_DAMAGE
            } else {
                SPIKEWEED_DAMAGE
            };
            self.damage_zombie_bypassing_shield(zombie_index, damage, vehicle_hit, events);
            let health_remaining = self.state.board.zombies[zombie_index].health;
            events.push(GameEvent::PlantSpecialHit {
                plant: plant_id,
                zombie: zombie_id,
                damage,
                health_remaining,
            });
            if health_remaining <= 0 {
                self.emit_zombie_died(zombie_id, events);
                self.state.board.zombies.remove(zombie_index);
            } else {
                zombie_index += 1;
            }
        }
        vehicle_hit
    }

    /// Applies the vehicle-pop cost to a spiky plant. Returns true when the
    /// plant was removed so the caller can stop using its plant index.
    fn pop_spiky_plant(&mut self, plant_id: EntityId, events: &mut Vec<GameEvent>) -> bool {
        let Some(plant_index) = self
            .state
            .board
            .plants
            .iter()
            .position(|plant| plant.id == plant_id)
        else {
            return true;
        };
        let is_spikerock = self.state.board.plants[plant_index].plant_type.slot() == 46;
        if is_spikerock {
            self.state.board.plants[plant_index].health -= SPIKEROCK_VEHICLE_SELF_DAMAGE;
            let health_remaining = self.state.board.plants[plant_index].health;
            events.push(GameEvent::PlantDamaged {
                entity: plant_id,
                damage: SPIKEROCK_VEHICLE_SELF_DAMAGE,
                health_remaining,
            });
            if health_remaining <= 0 {
                self.state.board.plants.remove(plant_index);
                events.push(GameEvent::PlantDied { entity: plant_id });
                return true;
            }
            false
        } else {
            self.state.board.plants.remove(plant_index);
            events.push(GameEvent::PlantDied { entity: plant_id });
            true
        }
    }

    fn find_plant_for_zombie(
        &self,
        row: u8,
        zombie_x: i64,
        zombie_type: ZombieType,
    ) -> Option<usize> {
        if zombie_type == ZombieType::Boss {
            return None;
        }
        self.state
            .board
            .plants
            .iter()
            .enumerate()
            .filter(|(_, plant)| plant.row == row && plant.health > 0)
            .filter(|(_, plant)| {
                !plant.plant_type.is_squash()
                    || (!plant.special_armed
                        && (plant.special_target.is_some() || plant.special_counter == 0))
                    || (plant.special_armed
                        && plant.special_target.is_some()
                        && plant.special_counter > SQUASH_OFF_GROUND_TICKS)
            })
            // Spikeweed is walked over; zombies do not bite it.
            .filter(|(_, plant)| is_gargantuar(zombie_type) || !plant.plant_type.is_spikeweed())
            .filter(|(_, plant)| {
                zombie_type == ZombieType::Digger
                    || !self
                        .state
                        .board
                        .ladders
                        .iter()
                        .any(|ladder| ladder.row == plant.row && ladder.column == plant.column)
            })
            .filter(|(_, plant)| {
                let plant_x = grid_x(plant.column);
                zombie_x + 70 * POSITION_SCALE > plant_x
                    && zombie_x + 50 * POSITION_SCALE < plant_x + 80 * POSITION_SCALE
            })
            .max_by_key(|(_, plant)| plant.column)
            .map(|(index, _)| index)
    }

    fn find_catapult_target(&self, row: u8, zombie_x: i64) -> Option<usize> {
        self.state
            .board
            .plants
            .iter()
            .enumerate()
            .filter(|(_, plant)| {
                plant.health > 0
                    && plant.row == row
                    && !plant.plant_type.is_spikeweed()
                    && zombie_x >= grid_x(plant.column) + 100 * POSITION_SCALE
            })
            .min_by_key(|(_, plant)| plant.column)
            .map(|(index, _)| index)
    }

    fn find_chomper_target(&self, row: u8, column: u8) -> Option<EntityId> {
        let center_x = grid_x(column);
        self.state
            .board
            .zombies
            .iter()
            .filter(|zombie| {
                zombie.health > 0
                    && zombie.row == row
                    && matches!(zombie.zombie_type, ZombieType::Normal)
                    && zombie.position_x >= center_x - 20 * POSITION_SCALE
                    && zombie.position_x <= center_x + 80 * POSITION_SCALE
            })
            .min_by_key(|zombie| zombie.position_x.abs_diff(center_x))
            .map(|zombie| zombie.id)
    }

    fn find_squash_target(&self, row: u8, column: u8) -> Option<EntityId> {
        let center_x = grid_x(column);
        self.state
            .board
            .zombies
            .iter()
            .filter(|zombie| {
                let maximum_gap = if zombie.eating {
                    SQUASH_EATING_TARGET_GAP
                } else {
                    SQUASH_TARGET_GAP
                } * POSITION_SCALE;
                zombie.health > 0
                    && zombie.row == row
                    && matches!(zombie.zombie_type, ZombieType::Normal)
                    && (zombie.position_x - center_x).abs() <= maximum_gap
            })
            .min_by_key(|zombie| zombie.position_x.abs_diff(center_x))
            .map(|zombie| zombie.id)
    }

    /// A hypnotized zombie attacks a non-hypnotized zombie in the same row.
    /// Returns true when it found and damaged a target zombie.
    fn attack_zombie_target(
        &mut self,
        attacker_id: EntityId,
        row: u8,
        events: &mut Vec<GameEvent>,
    ) -> bool {
        let position_x = self
            .state
            .board
            .zombies
            .iter()
            .find(|z| z.id == attacker_id)
            .map(|z| z.position_x)
            .unwrap_or(i64::MAX);
        let target_id = self
            .state
            .board
            .zombies
            .iter()
            .filter(|z| {
                z.health > 0
                    && !z.hypnotized
                    && z.row == row
                    && (z.position_x - position_x).abs() <= 115 * POSITION_SCALE
            })
            .min_by_key(|z| z.position_x)
            .map(|z| z.id);
        let Some(target_id) = target_id else {
            return false;
        };
        if let Some(target_idx) = self
            .state
            .board
            .zombies
            .iter()
            .position(|z| z.id == target_id)
        {
            self.damage_zombie(target_idx, ZOMBIE_BITE_DAMAGE, events);
            let health_remaining = self.state.board.zombies[target_idx].health;
            events.push(GameEvent::ZombieDamaged {
                entity: target_id,
                damage: ZOMBIE_BITE_DAMAGE,
                health_remaining,
                attacker: Some(attacker_id),
            });
            if health_remaining <= 0 {
                self.state.board.zombies[target_idx].health = 0;
                self.emit_zombie_died(target_id, events);
            }
        }
        true
    }

    fn fire_cob_projectile(
        &mut self,
        source: EntityId,
        source_row: u8,
        source_column: u8,
        target_row: u8,
        target_column: u8,
        events: &mut Vec<GameEvent>,
    ) {
        let projectile_type = ProjectileType::Cob;
        let origin_x = grid_x(source_column) - 44 * POSITION_SCALE;
        let target_x = grid_x(target_column);
        let id = self.state.board.allocate_entity();
        self.state.board.projectiles.push(ProjectileState {
            id,
            projectile_type,
            motion: ProjectileMotion::Lobbed,
            row: target_row,
            position_x: origin_x,
            position_y: grid_y(source_row),
            velocity_x: 0,
            velocity_y: 0,
            damage: projectile_type.damage(),
            age: 0,
            target_x: Some(target_x),
            target_row: Some(target_row),
            lob_height: 0,
            lob_velocity: -8,
        });
        events.push(GameEvent::ProjectileFired {
            entity: id,
            source,
            projectile_type,
            row: target_row,
        });
    }

    fn fire_catapult_projectile(
        &mut self,
        source: EntityId,
        row: u8,
        source_x: i64,
        target_x: i64,
        events: &mut Vec<GameEvent>,
    ) {
        let projectile_type = ProjectileType::Other(1);
        let origin_x = source_x + 113 * POSITION_SCALE;
        let range_x = (origin_x - target_x - 20 * POSITION_SCALE).max(40 * POSITION_SCALE);
        let id = self.state.board.allocate_entity();
        self.state.board.projectiles.push(ProjectileState {
            id,
            projectile_type,
            motion: ProjectileMotion::Lobbed,
            row,
            position_x: origin_x,
            position_y: grid_y(row) - 44 * POSITION_SCALE,
            velocity_x: -range_x / 120,
            velocity_y: 0,
            damage: projectile_type.damage(),
            age: 0,
            target_x: Some(target_x),
            target_row: Some(row),
            lob_height: 0,
            lob_velocity: 0,
        });
        events.push(GameEvent::ProjectileFired {
            entity: id,
            source,
            projectile_type,
            row,
        });
    }

    fn fire_projectiles(
        &mut self,
        source: EntityId,
        plant_type: PlantType,
        row: u8,
        column: u8,
        events: &mut Vec<GameEvent>,
    ) {
        let projectile_type = match plant_type.projectile_type() {
            ProjectileType::Kernel if self.rng.range(4) == 0 => ProjectileType::Butter,
            projectile_type => projectile_type,
        };
        let position_x = if plant_type.is_gloom_shroom() {
            grid_x(column) - 80 * POSITION_SCALE
        } else {
            grid_x(column) + 60 * POSITION_SCALE
        };
        let position_y = grid_y(row);
        match plant_type.firing_pattern() {
            FiringPattern::ThreeRow => {
                for target_row in [
                    row.checked_sub(1),
                    Some(row),
                    row.checked_add(1)
                        .filter(|target_row| *target_row < self.state.board.rows),
                ]
                .into_iter()
                .flatten()
                {
                    self.emit_plant_fired(source, plant_type, events);
                    self.fire_projectile(
                        source,
                        projectile_type,
                        target_row,
                        ProjectileTrajectory {
                            motion: ProjectileMotion::Straight,
                            position_x,
                            position_y: grid_y(target_row),
                            velocity_x: 3_330_000,
                            velocity_y: 0,
                        },
                        events,
                    );
                }
            }
            FiringPattern::Split => {
                self.emit_plant_fired(source, plant_type, events);
                self.fire_projectile(
                    source,
                    projectile_type,
                    row,
                    ProjectileTrajectory {
                        motion: ProjectileMotion::Straight,
                        position_x,
                        position_y,
                        velocity_x: 3_330_000,
                        velocity_y: 0,
                    },
                    events,
                );
                self.emit_plant_fired(source, plant_type, events);
                self.fire_projectile(
                    source,
                    projectile_type,
                    row,
                    ProjectileTrajectory {
                        motion: ProjectileMotion::Backwards,
                        position_x: grid_x(column) + 20 * POSITION_SCALE,
                        position_y,
                        velocity_x: -3_330_000,
                        velocity_y: 0,
                    },
                    events,
                );
            }
            FiringPattern::Star => {
                self.emit_plant_fired(source, plant_type, events);
                for (velocity_x, velocity_y) in [
                    (-3_330_000, 0),
                    (0, 3_330_000),
                    (0, -3_330_000),
                    (2_883_865, 1_665_000),
                    (2_883_865, -1_665_000),
                ] {
                    self.fire_projectile(
                        source,
                        projectile_type,
                        row,
                        ProjectileTrajectory {
                            motion: ProjectileMotion::Star,
                            position_x,
                            position_y,
                            velocity_x,
                            velocity_y,
                        },
                        events,
                    );
                }
            }
            FiringPattern::Backward => {
                self.emit_plant_fired(source, plant_type, events);
                self.fire_projectile(
                    source,
                    projectile_type,
                    row,
                    ProjectileTrajectory {
                        motion: ProjectileMotion::Backwards,
                        position_x: grid_x(column) + 20 * POSITION_SCALE,
                        position_y,
                        velocity_x: -3_330_000,
                        velocity_y: 0,
                    },
                    events,
                );
            }
            _ => {
                self.emit_plant_fired(source, plant_type, events);
                self.fire_projectile(
                    source,
                    projectile_type,
                    row,
                    ProjectileTrajectory {
                        motion: plant_type.projectile_motion(),
                        position_x,
                        position_y,
                        velocity_x: if plant_type.is_fume_shroom() || plant_type.is_gloom_shroom() {
                            0
                        } else {
                            3_330_000
                        },
                        velocity_y: 0,
                    },
                    events,
                );
            }
        }
    }

    fn emit_plant_fired(
        &mut self,
        entity: EntityId,
        plant_type: PlantType,
        events: &mut Vec<GameEvent>,
    ) {
        let variant = if matches!(plant_type.slot(), 10 | 42) {
            0
        } else {
            self.rng.range(4) as u8
        };
        events.push(GameEvent::PlantFired {
            entity,
            plant_type,
            variant,
        });
    }

    fn fire_projectile(
        &mut self,
        source: EntityId,
        projectile_type: ProjectileType,
        row: u8,
        trajectory: ProjectileTrajectory,
        events: &mut Vec<GameEvent>,
    ) {
        let id = self.state.board.allocate_entity();
        self.state.board.projectiles.push(ProjectileState {
            id,
            projectile_type,
            motion: trajectory.motion,
            row,
            position_x: trajectory.position_x,
            position_y: trajectory.position_y,
            velocity_x: trajectory.velocity_x,
            velocity_y: trajectory.velocity_y,
            damage: projectile_type.damage(),
            age: 0,
            target_x: None,
            target_row: None,
            lob_height: 0,
            lob_velocity: 0,
        });
        events.push(GameEvent::ProjectileFired {
            entity: id,
            source,
            projectile_type,
            row,
        });
    }

    fn spawn_sun(
        &mut self,
        source: SunSource,
        position_x: i64,
        position_y: i64,
        events: &mut Vec<GameEvent>,
    ) {
        self.spawn_sun_value(source, 25, position_x, position_y, events);
    }

    fn spawn_sun_value(
        &mut self,
        source: SunSource,
        value: u32,
        position_x: i64,
        position_y: i64,
        events: &mut Vec<GameEvent>,
    ) {
        let id = self.state.board.allocate_entity();
        self.state.board.suns.push(SunPickupState {
            id,
            source,
            value,
            position_x,
            position_y,
            target_y: None,
            velocity_x: 0,
            velocity_y: 0,
        });
        events.push(GameEvent::SunProduced {
            entity: id,
            source,
            value,
        });
    }

    fn spawn_coin(
        &mut self,
        coin_type: CoinType,
        position_x: i64,
        position_y: i64,
        events: &mut Vec<GameEvent>,
    ) {
        self.spawn_pickup(coin_type, position_x, position_y, events);
        // COIN_MOTION_COIN (Coin.cpp:306): dropped coins pop up in the
        // -1.7..-3.4 band and fall back under the 0.15 gravity.
        let launch_y = self.rng.next();
        let launch_x = self.rng.next();
        let ground_offset = self.rng.range(20);
        if let Some(coin) = self.state.board.coins.last_mut() {
            coin.velocity_y = -1_700_000 - (i64::from(launch_y) % 1_700_001);
            coin.velocity_x = (i64::from(launch_x) % 1_000_001) - 500_000;
            coin.target_y = Some(coin.position_y + i64::from(ground_offset) * POSITION_SCALE);
        }
    }

    fn spawn_pickup(
        &mut self,
        coin_type: CoinType,
        position_x: i64,
        position_y: i64,
        events: &mut Vec<GameEvent>,
    ) {
        self.spawn_pickup_with_payload(coin_type, position_x, position_y, None, None, events);
    }

    fn spawn_pickup_with_payload(
        &mut self,
        coin_type: CoinType,
        position_x: i64,
        position_y: i64,
        plant_type: Option<PlantType>,
        usable_seed_type: Option<PlantType>,
        events: &mut Vec<GameEvent>,
    ) {
        let id = self.state.board.allocate_entity();
        let value = coin_type.value();
        let award_motion = coin_type.is_level_award() || coin_type.unlock_mask() != 0;
        let (position_y, target_y, velocity_x, velocity_y) = if award_motion {
            let launch_y = self.rng.next();
            let launch_x = self.rng.next();
            let ground_offset = 45 + self.rng.range(20);
            let (position_y, ground_y) = if matches!(
                coin_type,
                CoinType::AwardSilverSunflower | CoinType::AwardGoldSunflower
            ) {
                let position_y = position_y - 100 * POSITION_SCALE;
                let ground_y = (position_y + 45 * POSITION_SCALE).min(400 * POSITION_SCALE);
                (position_y, ground_y)
            } else {
                let mut ground_y = position_y + i64::from(ground_offset) * POSITION_SCALE;
                ground_y = ground_y.clamp(80 * POSITION_SCALE, 521 * POSITION_SCALE);
                if matches!(
                    coin_type,
                    CoinType::FinalSeedPacket
                        | CoinType::UsableSeedPacket
                        | CoinType::Trophy
                        | CoinType::Shovel
                        | CoinType::CarKeys
                        | CoinType::Almanac
                        | CoinType::Vase
                        | CoinType::WateringCan
                        | CoinType::Taco
                        | CoinType::Note
                ) {
                    ground_y -= 30 * POSITION_SCALE;
                }
                (position_y, ground_y)
            };
            (
                position_y,
                Some(ground_y),
                (i64::from(launch_x) % 1_000_001) - 500_000,
                -3_000_000 - (i64::from(launch_y) % 2_000_001),
            )
        } else {
            (position_y, None, 0, 0)
        };
        self.state.board.coins.push(CoinPickupState {
            id,
            coin_type,
            value,
            position_x,
            position_y,
            plant_type,
            usable_seed_type,
            target_y,
            velocity_x,
            velocity_y,
        });
        events.push(GameEvent::CoinProduced {
            entity: id,
            coin_type,
            value,
        });
    }

    fn spawn_normal_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::Normal,
            270,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_conehead_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::Conehead,
            640,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_pea_head_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::PeaHead,
            270,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_catapult_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::Catapult,
            850,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_bobsled_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        let leader = self._spawn_zombie_inner(
            ZombieType::Bobsled,
            BOBSLED_HEALTH,
            row,
            wave,
            position_override,
            events,
        );
        let leader_position = self
            .state
            .board
            .zombies
            .iter()
            .find(|zombie| zombie.id == leader)
            .map(|zombie| zombie.position_x)
            .expect("bobsled leader must be spawned");
        if let Some(zombie) = self
            .state
            .board
            .zombies
            .iter_mut()
            .find(|zombie| zombie.id == leader)
        {
            zombie.bobsled_leader = true;
            zombie.shield_health = BOBSLED_HELM_HEALTH;
            zombie.shield_max_health = BOBSLED_HELM_HEALTH;
        }
        for offset in 1..=3 {
            self._spawn_zombie_inner(
                ZombieType::Bobsled,
                BOBSLED_HEALTH,
                row,
                wave,
                Some(leader_position + i64::from(offset) * 50 * POSITION_SCALE),
                events,
            );
        }
        leader
    }

    #[allow(dead_code)]
    fn spawn_ladder_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::Ladder,
            LADDER_HEALTH,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_boss_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        let health = if self.state.mode == ModeKind::Adventure {
            BOSS_ADVENTURE_HEALTH
        } else {
            BOSS_CHALLENGE_HEALTH
        };
        self._spawn_zombie_inner(
            ZombieType::Boss,
            health,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_wallnut_head_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::WallnutHead,
            270,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_jalapeno_head_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::JalapenoHead,
            ZOMBOTANY_JALAPENO_HEALTH,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_gatling_head_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::GatlingHead,
            270,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_squash_head_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::SquashHead,
            270,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_tallnut_head_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::TallnutHead,
            270,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_gigagargantuar_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::Gigagargantuar,
            GIGAGARGANTUAR_HEALTH,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_pogo_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(ZombieType::Pogo, 500, row, wave, position_override, events)
    }

    #[allow(dead_code)]
    fn spawn_gargantuar_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::Gargantuar,
            3_000,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_dancer_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::Dancer,
            500,
            row,
            wave,
            position_override,
            events,
        )
    }

    fn spawn_backup_dancer(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::BackupDancer,
            270,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_digger_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::Digger,
            370,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_bungee_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::Bungee,
            450,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_flag_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(ZombieType::Flag, 270, row, wave, position_override, events)
    }

    #[allow(dead_code)]
    fn spawn_buckethead_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::Buckethead,
            1370,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    #[allow(dead_code)]
    fn spawn_screen_door_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::ScreenDoor,
            270,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_ducky_tube_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::DuckyTube,
            270,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_dolphin_rider_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::DolphinRider,
            // Zombie_Init in 1.0.0.1051 gives the Dolphin Rider a 500-HP body.
            500,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_snorkel_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::Snorkel,
            270,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_zamboni_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::Zamboni,
            ZAMBONI_HEALTH,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_balloon_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::Balloon,
            270,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_football_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        let id = self._spawn_zombie_inner(
            ZombieType::Football,
            1670,
            row,
            wave,
            position_override,
            events,
        );
        let football_speed = self.rng.fixed_range(660_000, 680_000);
        if let Some(zombie) = self.state.board.zombies.iter_mut().find(|z| z.id == id) {
            zombie.speed = football_speed;
        }
        id
    }

    #[allow(dead_code)]
    fn spawn_newspaper_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::Newspaper,
            270,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_pole_vaulter_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(
            ZombieType::PoleVaulter,
            500,
            row,
            wave,
            position_override,
            events,
        )
    }

    #[allow(dead_code)]
    fn spawn_imp_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        self._spawn_zombie_inner(ZombieType::Imp, 270, row, wave, position_override, events)
    }

    fn throw_imp(
        &mut self,
        gargantuar: EntityId,
        row: u8,
        gargantuar_x: i64,
        wave: u32,
        events: &mut Vec<GameEvent>,
    ) {
        let mut distance = gargantuar_x - GARGANTUAR_THROW_BASE_X;
        if self.state.scene == SceneKind::Roof {
            distance -= 180 * POSITION_SCALE;
            distance = distance.max(-140 * POSITION_SCALE);
        } else {
            distance = distance.max(GARGANTUAR_THROW_MIN_DISTANCE);
        }
        if distance > 140 * POSITION_SCALE {
            distance -= i64::from(self.rng.range(101)) * POSITION_SCALE;
        }
        let mut velocity_z = distance / 120;
        let mut altitude = IMP_THROW_START_ALTITUDE;
        let mut flight_ticks: u32 = 0;
        while altitude > 0 && flight_ticks < 10_000 {
            velocity_z -= THROWN_ZOMBIE_GRAVITY;
            altitude += velocity_z;
            flight_ticks += 1;
        }
        let spawn_x = gargantuar_x - IMP_THROW_SPAWN_OFFSET;
        let imp = self._spawn_zombie_inner(ZombieType::Imp, 270, row, wave, Some(spawn_x), events);
        if let Some(zombie) = self.state.board.zombies.iter_mut().find(|z| z.id == imp) {
            zombie.imp_flight_ticks = flight_ticks;
        }
        let imp_variant = self.rng.range(2) as u8;
        events.push(GameEvent::ImpThrown {
            gargantuar,
            imp,
            imp_variant,
        });
    }

    fn update_magnet_shroom(
        &mut self,
        plant_index: usize,
        plant_id: EntityId,
        row: u8,
        column: u8,
        events: &mut Vec<GameEvent>,
    ) {
        {
            let plant = &mut self.state.board.plants[plant_index];
            if plant.special_counter > 0 {
                plant.special_counter -= 1;
                return;
            }
        }
        let center_x = grid_x(column);
        let mut best: Option<(usize, i64)> = None;
        for (index, zombie) in self.state.board.zombies.iter().enumerate() {
            if zombie.health <= 0
                || zombie.hypnotized
                || zombie.departed
                || zombie.imp_flight_ticks > 0
                || zombie.position_x > 800 * POSITION_SCALE
                || row.abs_diff(zombie.row) > 2
            {
                continue;
            }
            if zombie.zombie_type == ZombieType::Digger
                && !zombie.digger_underground
                && zombie.digger_counter > 0
            {
                continue;
            }
            let stealable = match zombie.zombie_type {
                ZombieType::Buckethead | ZombieType::Football => zombie.health > 270,
                ZombieType::ScreenDoor | ZombieType::Ladder => zombie.shield_health > 0,
                ZombieType::Jackbox => zombie.jackbox_timer > 0,
                ZombieType::Pogo | ZombieType::Digger => zombie.special_phase == 0,
                _ => false,
            };
            if !stealable {
                continue;
            }
            let radius = if zombie.eating {
                MAGNET_STEAL_EATING_RADIUS
            } else {
                MAGNET_STEAL_RADIUS
            };
            let distance = (zombie.position_x - center_x).abs();
            if distance > radius {
                continue;
            }
            let score = distance + i64::from(row.abs_diff(zombie.row)) * 80 * POSITION_SCALE;
            if best.is_none_or(|(_, s)| score < s) {
                best = Some((index, score));
            }
        }
        if let Some((zombie_index, _)) = best {
            let walk_speed = self.rng.fixed_range(230_000, 320_000);
            let mode = self.state.mode;
            let zombie = &mut self.state.board.zombies[zombie_index];
            let entity = zombie.id;
            match zombie.zombie_type {
                ZombieType::Buckethead | ZombieType::Football => {
                    zombie.health = zombie.health.min(270);
                }
                ZombieType::ScreenDoor | ZombieType::Ladder => {
                    zombie.shield_health = 0;
                }
                ZombieType::Jackbox => {
                    zombie.jackbox_timer = 0;
                    zombie.speed = walk_speed;
                }
                ZombieType::Pogo => {
                    zombie.special_phase = 1;
                    zombie.pogo_counter = 0;
                    zombie.pogo_target_x = None;
                    zombie.pogo_velocity_x = 0;
                }
                ZombieType::Digger => {
                    zombie.special_phase = 1;
                    if zombie.digger_underground {
                        zombie.digger_underground = false;
                        zombie.digger_counter = DIGGER_AXE_LOSS_SURFACE_TICKS;
                        zombie.speed = if mode == ModeKind::IZombie {
                            DIGGER_IZOMBIE_WALK_SPEED
                        } else {
                            DIGGER_WALK_SPEED
                        };
                        events.push(GameEvent::DiggerSurfaced { entity });
                    }
                }
                _ => {}
            }
            self.state.board.plants[plant_index].special_counter = MAGNET_RECHARGE_TICKS;
            events.push(GameEvent::MetalStolen {
                plant: plant_id,
                zombie: Some(entity),
            });
            return;
        }
        if let Some(ladder_index) =
            self.state.board.ladders.iter().position(|ladder| {
                ladder.row.abs_diff(row) <= 2 && ladder.column.abs_diff(column) <= 2
            })
        {
            self.state.board.ladders.remove(ladder_index);
            self.state.board.plants[plant_index].special_counter = MAGNET_RECHARGE_TICKS;
            events.push(GameEvent::MetalStolen {
                plant: plant_id,
                zombie: None,
            });
        }
    }

    fn spawn_bungee_drop(
        &mut self,
        zombie_type: ZombieType,
        row: u8,
        column: u8,
        wave: u32,
        events: &mut Vec<GameEvent>,
    ) -> (EntityId, EntityId) {
        let health = match zombie_type {
            ZombieType::Conehead => 640,
            ZombieType::Buckethead => 1_370,
            _ => 270,
        };
        let carried = self._spawn_zombie_inner(
            zombie_type,
            health,
            row,
            wave,
            Some(grid_x(column) - 15 * POSITION_SCALE),
            events,
        );
        let carrier = self._spawn_zombie_inner(
            ZombieType::Bungee,
            450,
            row,
            wave,
            Some(grid_x(column)),
            events,
        );
        let altitude = BUNGEE_DROP_DIVE_ALTITUDE + i64::from(self.rng.range(151));
        let dive_ticks = ((altitude + BUNGEE_DROP_SPEED - 1) / BUNGEE_DROP_SPEED) as u32;
        if let Some(zombie) = self
            .state
            .board
            .zombies
            .iter_mut()
            .find(|z| z.id == carried)
        {
            zombie.bungee_held = true;
        }
        if let Some(zombie) = self
            .state
            .board
            .zombies
            .iter_mut()
            .find(|z| z.id == carrier)
        {
            zombie.bungee_stolen = true;
            zombie.special_phase = 1;
            zombie.special_counter = dive_ticks;
            zombie.special_target = Some(carried);
        }
        (carrier, carried)
    }

    fn row_can_have_zombie_type(&self, row: u8, zombie_type: ZombieType, wave: u32) -> bool {
        if self.state.mode == ModeKind::Adventure && !adventure_row_is_sodded(self.state.level, row)
        {
            return false;
        }
        if zombie_type == ZombieType::Bobsled
            && self
                .state
                .board
                .ice_timer
                .get(usize::from(row))
                .copied()
                .unwrap_or(0)
                == 0
        {
            return false;
        }
        let pool_row = self.state.scene == SceneKind::Pool && matches!(row, 2 | 3);
        if pool_row {
            if wave < 5 {
                return zombie_type_is_pool_only(zombie_type);
            }
            return zombie_type_can_go_in_pool(zombie_type);
        }
        !zombie_type_is_pool_only(zombie_type)
    }

    fn pick_spawn_row(&mut self, zombie_type: ZombieType, wave: u32) -> u8 {
        for _ in 0..16 {
            let row = self.rng.range(u32::from(self.state.board.rows)) as u8;
            if self.row_can_have_zombie_type(row, zombie_type, wave) {
                return row;
            }
        }
        (0..self.state.board.rows)
            .find(|&row| self.row_can_have_zombie_type(row, zombie_type, wave))
            .unwrap_or(0)
    }

    /// Board::PickZombieWaves for adventure: per-wave points budget, flag
    /// extras, level multipliers, intro spawns, the level-50 extra
    /// Gargantuar, PutInMissingZombies on the final wave, and the weighted
    /// fill. The Yeti intro requires a finished profile; `replay` stands in
    /// for CanSpawnYetis here.
    pub fn pick_adventure_waves(&mut self, level: u8, replay: bool) -> Vec<Vec<ZombieType>> {
        let total_waves = adventure_wave_count(level, replay);
        let intro = adventure_introduced_zombie(level);
        let mut saw_yeti = false;
        let mut waves = Vec::with_capacity(total_waves as usize);
        for wave_index in 0..total_waves {
            let mut wave: Vec<ZombieType> = Vec::new();
            let is_final = wave_index + 1 == total_waves;
            let is_flag = adventure_is_flag_wave(level, replay, wave_index);
            let mut points: i32 = if replay && level != 5 {
                (wave_index * 2 / 5 + 1) as i32
            } else {
                (wave_index / 3 + 1) as i32
            };
            if level == 45 && is_flag && !is_final {
                for _ in 0..5 {
                    put_zombie_in_wave(&mut wave, &mut points, ZombieType::Bungee);
                }
                waves.push(wave);
                continue;
            }
            if is_flag {
                let plain = points.min(8);
                points = (points as f32 * 2.5) as i32;
                for _ in 0..plain {
                    put_zombie_in_wave(&mut wave, &mut points, ZombieType::Normal);
                }
                put_zombie_in_wave(&mut wave, &mut points, ZombieType::Flag);
            }
            points = match level {
                5 | 25 => points * 4,
                10 | 20 | 30 | 40 => points * 3,
                45 => points * 2,
                _ => points,
            };
            if let Some(intro_type) = intro {
                let spawns = match intro_type {
                    ZombieType::Digger | ZombieType::Balloon => wave_index == 6 || is_final,
                    ZombieType::Yeti => {
                        let hit = replay && wave_index == total_waves / 2 && !saw_yeti;
                        if hit {
                            saw_yeti = true;
                        }
                        hit
                    }
                    ZombieType::Boss => false,
                    _ => wave_index == total_waves / 2 || is_final,
                };
                if spawns {
                    put_zombie_in_wave(&mut wave, &mut points, intro_type);
                }
            }
            if level == 50 && is_final {
                put_zombie_in_wave(&mut wave, &mut points, ZombieType::Gargantuar);
            }
            if is_final {
                for zombie_type in ADVENTURE_PICK_ORDER {
                    if !wave.contains(&zombie_type) && adventure_zombie_allowed(zombie_type, level)
                    {
                        put_zombie_in_wave(&mut wave, &mut points, zombie_type);
                    }
                }
            }
            while points > 0 && wave.len() < 50 {
                let zombie_type = self.pick_adventure_wave_type(level, wave_index, points);
                put_zombie_in_wave(&mut wave, &mut points, zombie_type);
            }
            waves.push(wave);
        }
        waves
    }

    /// SpawnZombieWave: composed entries spawn on smooth-picked legal rows;
    /// a Bobsled entry without a usable ice trail becomes four normals.
    fn spawn_adventure_zombie(
        &mut self,
        zombie_type: ZombieType,
        wave: u32,
        events: &mut Vec<GameEvent>,
    ) {
        match zombie_type {
            ZombieType::Bobsled => {
                if let Some(ice_row) = self.pick_bobsled_row() {
                    self.spawn_bobsled_zombie(ice_row, wave, None, events);
                } else {
                    for _ in 0..4 {
                        let row = self.pick_spawn_row(ZombieType::Normal, wave);
                        self.spawn_normal_zombie(row, wave, None, events);
                    }
                }
            }
            ZombieType::Jackbox => {
                let row = self.pick_spawn_row(zombie_type, wave);
                self.spawn_jackbox_zombie(row, wave, None, events);
            }
            ZombieType::Yeti => {
                let row = self.pick_spawn_row(zombie_type, wave);
                self.spawn_yeti_zombie(row, wave, None, events);
            }
            _ => {
                let health = match zombie_type {
                    ZombieType::Conehead => 640,
                    ZombieType::Buckethead => 1_370,
                    ZombieType::Football => 1_670,
                    ZombieType::Digger => 370,
                    ZombieType::Bungee => 450,
                    ZombieType::PoleVaulter
                    | ZombieType::Pogo
                    | ZombieType::Dancer
                    | ZombieType::DolphinRider => 500,
                    ZombieType::Ladder => LADDER_HEALTH,
                    ZombieType::Zamboni => ZAMBONI_HEALTH,
                    ZombieType::Catapult => 850,
                    ZombieType::Gargantuar => 3_000,
                    _ => 270,
                };
                let row = self.pick_spawn_row(zombie_type, wave);
                self._spawn_zombie_inner(zombie_type, health, row, wave, None, events);
            }
        }
    }

    fn pick_adventure_wave_type(&mut self, level: u8, wave_index: u32, points: i32) -> ZombieType {
        let mut total = 0u32;
        let mut candidates: Vec<(ZombieType, u32)> = Vec::new();
        for zombie_type in ADVENTURE_PICK_ORDER {
            if !adventure_zombie_allowed(zombie_type, level) {
                continue;
            }
            let (value, _, first_allowed_wave, weight) = zombie_wave_stats(zombie_type);
            if wave_index + 1 < first_allowed_wave || points < value as i32 {
                continue;
            }
            candidates.push((zombie_type, weight));
            total += weight;
        }
        if total == 0 {
            return ZombieType::Normal;
        }
        let mut roll = self.rng.range(total) as i64;
        for (zombie_type, weight) in candidates {
            roll -= i64::from(weight);
            if roll < 0 {
                return zombie_type;
            }
        }
        ZombieType::Normal
    }

    /// GridItem::OpenPortal: places a portal and emits its opening anchor.
    pub fn place_portal(&mut self, row: u8, column: u8, square: bool, events: &mut Vec<GameEvent>) {
        self.state.board.portals.push((row, column, square));
        events.push(GameEvent::PortalOpened {
            row,
            column,
            square,
        });
    }

    fn pick_bobsled_row(&mut self) -> Option<u8> {
        let can_add = self
            .state
            .board
            .ice_timer
            .iter()
            .zip(&self.state.board.ice_min_x)
            .any(|(timer, min_x)| *timer > 0 && *min_x < 700 * POSITION_SCALE);
        if !can_add {
            return None;
        }
        let iced_rows: Vec<u8> = (0..self.state.board.rows)
            .filter(|row| self.state.board.ice_timer[usize::from(*row)] > 0)
            .collect();
        let pick = self.rng.range(iced_rows.len() as u32) as usize;
        Some(iced_rows[pick])
    }

    #[allow(dead_code)]
    fn spawn_jackbox_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        let id = self._spawn_zombie_inner(
            ZombieType::Jackbox,
            JACKBOX_HEALTH,
            row,
            wave,
            position_override,
            events,
        );
        // Zombie_UpdateJack: the pop fires after (450 + Rand(300)) distance
        // units at the limp-doubled travel time, ~1323-2272 ticks at 0.66-0.68.
        let pop_distance = 450 + i64::from(self.rng.range(301));
        let jack_speed = self
            .state
            .board
            .zombies
            .iter()
            .find(|z| z.id == id)
            .map(|z| z.speed.max(1))
            .unwrap_or(660_000);
        if let Some(zombie) = self.state.board.zombies.iter_mut().find(|z| z.id == id) {
            zombie.jackbox_timer = (pop_distance * 2 * POSITION_SCALE / jack_speed) as u32;
        }
        id
    }

    #[allow(dead_code)]
    fn spawn_yeti_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        let id = self._spawn_zombie_inner(
            ZombieType::Yeti,
            YETI_HEALTH,
            row,
            wave,
            position_override,
            events,
        );
        if let Some(zombie) = self.state.board.zombies.iter_mut().find(|z| z.id == id) {
            zombie.speed = YETI_WALK_SPEED;
            zombie.yeti_counter = self
                .rng
                .range_inclusive(YETI_FLEE_MIN_TICKS, YETI_FLEE_MAX_TICKS);
        }
        id
    }

    fn spawn_vase_zombie(
        &mut self,
        zombie_type: ZombieType,
        row: u8,
        column: u8,
        events: &mut Vec<GameEvent>,
    ) {
        let health = match zombie_type {
            ZombieType::Conehead => 640,
            ZombieType::Buckethead => 1_370,
            ZombieType::ScreenDoor => 270,
            ZombieType::Football => 1_670,
            ZombieType::PoleVaulter | ZombieType::Pogo | ZombieType::Dancer => 500,
            ZombieType::Jackbox => JACKBOX_HEALTH,
            ZombieType::Gargantuar => 3_000,
            ZombieType::Gigagargantuar => GIGAGARGANTUAR_HEALTH,
            ZombieType::JalapenoHead => ZOMBOTANY_JALAPENO_HEALTH,
            ZombieType::Boss => BOSS_ADVENTURE_HEALTH,
            _ => 270,
        };
        let entity =
            self._spawn_zombie_inner(zombie_type, health, row, 0, Some(grid_x(column)), events);
        if zombie_type == ZombieType::Jackbox {
            // Zombie_Init forces the Scary Potter run counter to 10, so a
            // vase-released Jack pops after ~10 running + 110 popping updates.
            if let Some(zombie) = self
                .state
                .board
                .zombies
                .iter_mut()
                .find(|zombie| zombie.id == entity)
            {
                zombie.jackbox_timer = VASE_JACKBOX_POP_TICKS;
            }
        }
    }

    #[allow(dead_code)]
    fn _spawn_zombie_inner(
        &mut self,
        zombie_type: ZombieType,
        health: i32,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        let id = self.state.board.allocate_entity();
        let position_x = position_override
            .unwrap_or_else(|| i64::from(780 + self.rng.range(40)) * POSITION_SCALE);
        let groan_counter = self.rng.range_inclusive(300, 400) as i32;
        // Zombie_ResetSpeed in 1.0.0.1051 gives Dancer, Backup Dancer, Pogo,
        // and Flag a fixed 0.45 walk.
        let speed = if zombie_type == ZombieType::Pogo
            || zombie_type == ZombieType::Flag
            || zombie_type == ZombieType::BackupDancer
        {
            450_000
        } else if zombie_type == ZombieType::Imp && self.state.mode == ModeKind::IZombie {
            // Zombie_ResetSpeed in 1.0.0.1051 runs the I, Zombie Imp at 0.9.
            900_000
        } else if zombie_type == ZombieType::Digger {
            // Diggers spawn tunneling; Zombie_ResetSpeed gives 0.66-0.68.
            self.rng.fixed_range(660_000, 680_000)
        } else if zombie_type == ZombieType::Bungee {
            0
        } else if zombie_type == ZombieType::Bobsled {
            BOBSLED_SPEED
        } else if zombie_type == ZombieType::Dancer {
            500_000
        } else if zombie_type == ZombieType::DolphinRider {
            DOLPHIN_WALK_SPEED
        } else if zombie_type == ZombieType::Snorkel {
            SNORKEL_SPEED
        } else if zombie_type == ZombieType::Zamboni {
            zamboni_speed(position_x)
        } else if zombie_type == ZombieType::Boss {
            0
        } else if zombie_type == ZombieType::Jackbox {
            // Zombie_ResetSpeed in 1.0.0.1051 runs Jack-in-the-Box at 0.66-0.68.
            self.rng.fixed_range(660_000, 680_000)
        } else if zombie_type == ZombieType::Ladder {
            // Zombie_ResetSpeed gives PHASE_LADDER_CARRYING 0.79-0.81.
            self.rng.fixed_range(790_000, 810_000)
        } else {
            self.rng.fixed_range(230_000, 320_000)
        };
        let speed = if self.state.challenge.kind == ChallengeKind::ZombiesOnSpeed {
            speed.saturating_mul(2)
        } else {
            speed
        };
        self.state.board.zombies.push(ZombieState {
            id,
            zombie_type,
            row,
            position_x,
            speed,
            health,
            max_health: health,
            age: 0,
            groan_counter,
            frozen_counter: 0,
            chilled_counter: 0,
            eating: false,
            garlic_counter: 0,
            garlic_target: None,
            from_wave: wave,
            hypnotized: false,
            has_vaulted: false,
            newspaper_health: 0,
            jackbox_timer: 0,
            yeti_counter: 0,
            yeti_running: false,
            yeti_loot_dropped: false,
            pea_head_counter: if zombie_type == ZombieType::PeaHead {
                ZOMBIE_PEA_HEAD_RELOAD_TICKS
            } else {
                0
            },
            catapult_counter: 0,
            catapult_shots: if zombie_type == ZombieType::Catapult {
                CATAPULT_SHOTS
            } else {
                0
            },
            catapult_armed: false,
            vehicle_disabled: false,
            damage_tier: 0,
            pogo_counter: if zombie_type == ZombieType::Pogo {
                POGO_BOUNCE_TICKS
            } else {
                0
            },
            pogo_target_x: None,
            pogo_velocity_x: 0,
            dancer_counter: if zombie_type == ZombieType::Dancer {
                DANCER_SUMMON_TICKS
            } else {
                0
            },
            dancer_summoned: false,
            digger_counter: 0,
            digger_underground: zombie_type == ZombieType::Digger,
            bungee_counter: if zombie_type == ZombieType::Bungee {
                BUNGEE_STEAL_TICKS
            } else {
                0
            },
            bungee_stolen: false,
            dolphin_phase: 0,
            dolphin_counter: 0,
            dolphin_target_x: None,
            snorkel_phase: 0,
            balloon_phase: if zombie_type == ZombieType::Balloon {
                BALLOON_FLYING_PHASE
            } else {
                0
            },
            balloon_counter: 0,
            balloon_flying_health: if zombie_type == ZombieType::Balloon {
                BALLOON_FLYING_HEALTH
            } else {
                0
            },
            blowing_away: false,
            departed: false,
            in_pool: false,
            armor_intact: matches!(
                zombie_type,
                ZombieType::Buckethead
                    | ZombieType::Football
                    | ZombieType::Conehead
                    | ZombieType::ScreenDoor
                    | ZombieType::Newspaper
                    | ZombieType::Ladder
            ),
            portal_cooldown: 0,
            bungee_held: false,
            imp_thrown: false,
            imp_flight_ticks: 0,
            boss_head_counter: if zombie_type == ZombieType::Boss {
                BOSS_HEAD_COUNTER_INITIAL + BOSS_HEAD_SPIT_DELAY
            } else {
                0
            },
            boss_ball_active: false,
            boss_ball_fire: false,
            boss_ball_row: 0,
            boss_ball_x: 0,
            shield_health: match zombie_type {
                ZombieType::Ladder => LADDER_SHIELD_HEALTH,
                ZombieType::ScreenDoor => SCREEN_DOOR_SHIELD_HEALTH,
                ZombieType::Newspaper => NEWSPAPER_PAPER_HEALTH,
                ZombieType::WallnutHead => ZOMBOTANY_WALLNUT_HELM_HEALTH,
                ZombieType::TallnutHead => ZOMBOTANY_TALLNUT_HELM_HEALTH,
                _ => 0,
            },
            shield_max_health: match zombie_type {
                ZombieType::Ladder => LADDER_SHIELD_HEALTH,
                ZombieType::ScreenDoor => SCREEN_DOOR_SHIELD_HEALTH,
                ZombieType::Newspaper => NEWSPAPER_PAPER_HEALTH,
                ZombieType::WallnutHead => ZOMBOTANY_WALLNUT_HELM_HEALTH,
                ZombieType::TallnutHead => ZOMBOTANY_TALLNUT_HELM_HEALTH,
                _ => 0,
            },
            ladder_placed: false,
            bobsled_leader: false,
            bobsled_counter: if zombie_type == ZombieType::Bobsled {
                BOBSLED_SLIDE_TICKS
            } else {
                0
            },
            bobsled_sliding: zombie_type == ZombieType::Bobsled,
            special_counter: match zombie_type {
                ZombieType::JalapenoHead | ZombieType::GatlingHead => ZOMBOTANY_HEAD_RELOAD_TICKS,
                ZombieType::Boss => BOSS_ATTACK_TICKS,
                _ => 0,
            },
            special_phase: 0,
            special_target: None,
        });
        events.push(GameEvent::ZombieSpawned {
            entity: id,
            zombie_type,
            row,
            wave,
        });
        id
    }
}

fn spikeweed_hits(zombie_x: i64, column: u8) -> bool {
    // Attack rect from Plant::GetPlantAttackRect for SEED_SPIKEWEED:
    // [mX + 20, mX + mWidth - 30]. The board uses an 80-unit cell width.
    let attack_left = grid_x(column) + 20 * POSITION_SCALE;
    let attack_right = grid_x(column) + 50 * POSITION_SCALE;
    let zombie_left = zombie_x;
    let zombie_right = zombie_x + 70 * POSITION_SCALE;
    zombie_right > attack_left && zombie_left < attack_right
}

fn grid_x(column: u8) -> i64 {
    i64::from(column) * 80 * POSITION_SCALE + 40 * POSITION_SCALE
}

fn damage_tier(health: i32, max_health: i32) -> u8 {
    if health <= max_health / 3 {
        2
    } else if health <= max_health * 2 / 3 {
        1
    } else {
        0
    }
}

/// Board::StageIsNight (Board.cpp:8849-8857): the boss roof plays at night
/// alongside the night and fog lawns.
fn scene_is_night(scene: SceneKind) -> bool {
    matches!(scene, SceneKind::Night | SceneKind::Fog | SceneKind::Boss)
}

fn is_ladder_target(plant_type: PlantType) -> bool {
    matches!(plant_type.slot(), 3 | 23 | 30)
}

// Zombie_If_CanWater in 1.0.0.1051; ZOMBIE_DUCKY_TUBE itself is absent from
// the source list (it is preview-only and never wave-picked, weight 0).
fn zombie_type_can_go_in_pool(zombie_type: ZombieType) -> bool {
    matches!(
        zombie_type,
        ZombieType::Normal
            | ZombieType::Conehead
            | ZombieType::Buckethead
            | ZombieType::Flag
            | ZombieType::Snorkel
            | ZombieType::DolphinRider
            | ZombieType::PeaHead
            | ZombieType::WallnutHead
            | ZombieType::JalapenoHead
            | ZombieType::GatlingHead
            | ZombieType::TallnutHead
    )
}

fn zombie_type_is_pool_only(zombie_type: ZombieType) -> bool {
    matches!(zombie_type, ZombieType::Snorkel | ZombieType::DolphinRider)
}

fn is_gargantuar(zombie_type: ZombieType) -> bool {
    matches!(
        zombie_type,
        ZombieType::Gargantuar | ZombieType::Gigagargantuar
    )
}

fn grid_y(row: u8) -> i64 {
    i64::from(row) * 100 * POSITION_SCALE + 80 * POSITION_SCALE
}

fn projectile_row(position_y: i64, rows: u8) -> Option<u8> {
    let first_row_edge = 30 * POSITION_SCALE;
    let row_height = 100 * POSITION_SCALE;
    if position_y < first_row_edge {
        return None;
    }
    let row = (position_y - first_row_edge) / row_height;
    if row >= i64::from(rows) {
        None
    } else {
        Some(row as u8)
    }
}

fn plant_attack_start(column: u8) -> i64 {
    grid_x(column) + 60 * POSITION_SCALE
}

fn projectile_hits(projectile_x: i64, zombie_x: i64) -> bool {
    projectile_x + 45 * POSITION_SCALE > zombie_x + 36 * POSITION_SCALE
        && projectile_x - 15 * POSITION_SCALE < zombie_x + 78 * POSITION_SCALE
}

fn projectile_hits_plant(projectile_x: i64, plant_x: i64) -> bool {
    projectile_x + 45 * POSITION_SCALE > plant_x - 40 * POSITION_SCALE
        && projectile_x - 15 * POSITION_SCALE < plant_x + 40 * POSITION_SCALE
}

fn projectile_can_hit_zombie(zombie: &ZombieState, projectile_type: ProjectileType) -> bool {
    if zombie.bungee_held {
        return false;
    }
    if zombie.zombie_type == ZombieType::Snorkel && zombie.snorkel_phase == 1 && !zombie.eating {
        return false;
    }
    // Only Cactus/Cattail spikes carry the flying damage-range flag; airborne
    // balloons and mid-flight thrown imps reject every other projectile.
    if (balloon_is_airborne(zombie) || zombie.imp_flight_ticks > 0)
        && projectile_type != ProjectileType::Spike
    {
        return false;
    }
    true
}

fn balloon_is_airborne(zombie: &ZombieState) -> bool {
    zombie.zombie_type == ZombieType::Balloon
        && matches!(
            zombie.balloon_phase,
            BALLOON_FLYING_PHASE | BALLOON_POPPING_PHASE
        )
}

fn zamboni_speed(position_x: i64) -> i64 {
    // Zombie_UpdateZomboni only recomputes mVelX while mPosX > 400, so the
    // curve value at x=400 (0.10) holds for the rest of the drive.
    let position_x = position_x.max(400 * POSITION_SCALE);
    let min_x = 300 * POSITION_SCALE;
    let max_x = 700 * POSITION_SCALE;
    if position_x >= max_x {
        250_000
    } else {
        50_000 + (position_x - min_x) * 200_000 / (max_x - min_x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mt19937_matches_the_target_generator_sequence() {
        let mut rng = Mt19937::new(0);
        assert_eq!(
            (0..10).map(|_| rng.next()).collect::<Vec<_>>(),
            vec![
                2_146_374_468,
                699_692_587,
                1_213_834_231,
                1_920_714_022,
                994_957_275,
                2_082_945_813,
                1_964_848_567,
                1_049_283_459,
                171_986_203,
                1_030_590_208,
            ]
        );
    }

    #[test]
    fn seed_packet_ready_anchor_fires_when_cooldown_reaches_zero() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.board.seed_packets[1].refresh_remaining = 1;
        let events = game.advance(InputFrame::default());
        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::SeedPacketReady { slot: 1, .. }))
        );
        assert_eq!(game.state.board.seed_packets[1].refresh_remaining, 0);
    }

    #[test]
    fn planting_spends_sun_and_starts_the_original_refresh_length() {
        let mut game = Game::new(7, SceneKind::Day);
        let events = game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });

        assert_eq!(game.state.sun, 0);
        assert_eq!(game.state.board.plants.len(), 1);
        assert_eq!(game.state.board.seed_packets[1].refresh_remaining, 750);
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantPlaced {
                plant_type: PlantType::Sunflower,
                sun_remaining: 0,
                ..
            }
        )));
    }

    #[test]
    fn pumpkin_shell_covers_a_plant_and_absorbs_bites_first() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 500;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 2 },
                InputAction::SelectSeed { slot: 30 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let sunflower = game
            .state
            .board
            .plants
            .iter()
            .find(|plant| plant.plant_type == PlantType::Sunflower)
            .unwrap()
            .id;
        let pumpkin = game
            .state
            .board
            .plants
            .iter()
            .find(|plant| plant.plant_type == PlantType::Other(30))
            .unwrap()
            .id;
        assert_eq!(game.state.board.plants.len(), 2);
        assert_eq!(game.state.board.plants[1].id, pumpkin);
        assert_eq!(game.state.board.plants[1].max_health, 4_000);

        let mut setup_events = Vec::new();
        game.spawn_normal_zombie(
            2,
            0,
            Some(grid_x(2) + 30 * POSITION_SCALE),
            &mut setup_events,
        );
        for _ in 0..4 {
            game.advance(InputFrame::default());
        }

        assert_eq!(
            game.state
                .board
                .plants
                .iter()
                .find(|plant| plant.id == pumpkin)
                .unwrap()
                .health,
            3_996
        );
        assert_eq!(
            game.state
                .board
                .plants
                .iter()
                .find(|plant| plant.id == sunflower)
                .unwrap()
                .health,
            300
        );

        let events = game.advance(InputFrame {
            actions: vec![InputAction::Shovel { row: 2, column: 2 }],
        });
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantShoveled { entity } if *entity == pumpkin
        )));
        assert_eq!(game.state.board.plants.len(), 1);
        assert_eq!(game.state.board.plants[0].id, sunflower);
    }

    #[test]
    fn puffshroom_hits_nearby_and_expires_before_far_targets() {
        let mut game = Game::new(7, SceneKind::Night);
        game.state.sun = 50;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 8 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        assert_eq!(game.state.board.plants[0].plant_type.slot(), 8);
        game.state.board.plants[0].launch_counter = 1;
        let mut setup_events = Vec::new();
        // Near target inside the 230-unit puff attack rect.
        let near = game.spawn_normal_zombie(
            2,
            0,
            Some(plant_attack_start(0) + 100 * POSITION_SCALE),
            &mut setup_events,
        );
        let near_health = game
            .state
            .board
            .zombies
            .iter()
            .find(|zombie| zombie.id == near)
            .unwrap()
            .health;

        let events = (0..80)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileFired {
                projectile_type: ProjectileType::Puff,
                ..
            }
        )));
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileHit {
                zombie,
                damage: 20,
                ..
            } if *zombie == near
        )));
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|zombie| zombie.id == near)
                .unwrap()
                .health,
            near_health - 20
        );

        // Far target beyond puff range: plant should not arm a shot for it alone.
        let mut far_game = Game::new(7, SceneKind::Night);
        far_game.state.sun = 50;
        far_game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 8 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        far_game.state.board.plants[0].launch_counter = 1;
        far_game.state.board.plants[0].shooting_counter = 0;
        let mut far_setup = Vec::new();
        far_game.spawn_normal_zombie(
            2,
            0,
            Some(plant_attack_start(0) + 400 * POSITION_SCALE),
            &mut far_setup,
        );
        let far_events = (0..40)
            .flat_map(|_| far_game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert!(
            !far_events
                .iter()
                .any(|event| matches!(event, GameEvent::ProjectileFired { .. }))
        );

        // Forced puff projectile expires at age 75 without a hit.
        let mut expire_game = Game::new(7, SceneKind::Night);
        expire_game.state.sun = 50;
        expire_game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 8 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        let plant_id = expire_game.state.board.plants[0].id;
        let mut expire_events = Vec::new();
        expire_game.fire_projectile(
            plant_id,
            ProjectileType::Puff,
            2,
            ProjectileTrajectory {
                motion: ProjectileMotion::Puff,
                position_x: plant_attack_start(0),
                position_y: grid_y(2),
                velocity_x: 3_330_000,
                velocity_y: 0,
            },
            &mut expire_events,
        );
        assert_eq!(expire_game.state.board.projectiles.len(), 1);
        for _ in 0..PUFF_PROJECTILE_MAX_AGE {
            expire_game.advance(InputFrame::default());
        }
        assert!(expire_game.state.board.projectiles.is_empty());
    }

    #[test]
    fn seashroom_uses_the_same_short_range_puff_attack_in_pool() {
        let mut game = Game::new(7, SceneKind::Pool);
        game.state.sun = 50;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 24 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].asleep = false;
        game.state.board.plants[0].launch_counter = 1;
        let mut setup_events = Vec::new();
        let near = game.spawn_normal_zombie(
            2,
            0,
            Some(plant_attack_start(0) + 100 * POSITION_SCALE),
            &mut setup_events,
        );
        let events = (0..80)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileFired {
                projectile_type: ProjectileType::Puff,
                ..
            }
        )));
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileHit {
                zombie,
                damage: 20,
                ..
            } if *zombie == near
        )));

        let mut far_game = Game::new(7, SceneKind::Pool);
        far_game.state.sun = 50;
        far_game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 24 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        far_game.state.board.plants[0].asleep = false;
        far_game.state.board.plants[0].launch_counter = 1;
        let mut far_setup = Vec::new();
        far_game.spawn_normal_zombie(
            2,
            0,
            Some(plant_attack_start(0) + 300 * POSITION_SCALE),
            &mut far_setup,
        );
        let far_events = (0..60)
            .flat_map(|_| far_game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert!(
            !far_events
                .iter()
                .any(|event| matches!(event, GameEvent::ProjectileFired { .. }))
        );
    }

    #[test]
    fn scaredy_shroom_stops_shooting_at_a_nearby_zombie() {
        let mut close_game = Game::new(7, SceneKind::Night);
        close_game.state.sun = 100;
        close_game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 13 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        close_game.state.board.plants[0].launch_counter = 1;
        let mut close_setup = Vec::new();
        close_game.spawn_normal_zombie(
            2,
            0,
            Some(grid_x(0) + 80 * POSITION_SCALE),
            &mut close_setup,
        );
        let close_events = (0..60)
            .flat_map(|_| close_game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert!(
            !close_events
                .iter()
                .any(|event| matches!(event, GameEvent::ProjectileFired { .. }))
        );

        let mut far_game = Game::new(7, SceneKind::Night);
        far_game.state.sun = 100;
        far_game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 13 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        far_game.state.board.plants[0].launch_counter = 1;
        let mut far_setup = Vec::new();
        far_game.spawn_normal_zombie(
            2,
            0,
            Some(plant_attack_start(0) + 300 * POSITION_SCALE),
            &mut far_setup,
        );
        let far_events = (0..60)
            .flat_map(|_| far_game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert!(far_events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileFired {
                projectile_type: ProjectileType::Puff,
                ..
            }
        )));
    }

    #[test]
    fn fume_shroom_hits_every_zombie_inside_its_attack_rectangle() {
        let mut game = Game::new(7, SceneKind::Night);
        game.state.sun = 100;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 10 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;
        let mut setup_events = Vec::new();
        let attack_start = plant_attack_start(0);
        let near = game.spawn_normal_zombie(
            2,
            0,
            Some(attack_start + 100 * POSITION_SCALE),
            &mut setup_events,
        );
        let edge = game.spawn_normal_zombie(
            2,
            0,
            Some(attack_start + 300 * POSITION_SCALE),
            &mut setup_events,
        );
        let far = game.spawn_normal_zombie(
            2,
            0,
            Some(attack_start + 400 * POSITION_SCALE),
            &mut setup_events,
        );

        let events = (0..60)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();

        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileFired {
                projectile_type: ProjectileType::Puff,
                ..
            }
        )));
        for target in [near, edge] {
            assert!(events.iter().any(|event| matches!(
                event,
                GameEvent::ProjectileHit {
                    zombie,
                    damage: 20,
                    ..
                } if *zombie == target
            )));
            assert_eq!(
                game.state
                    .board
                    .zombies
                    .iter()
                    .find(|zombie| zombie.id == target)
                    .unwrap()
                    .health,
                250
            );
        }
        assert!(!events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileHit { zombie, .. } if *zombie == far
        )));
    }

    #[test]
    fn gloom_shroom_hits_three_rows_inside_its_area() {
        let mut game = Game::new(7, SceneKind::Night);
        game.state.sun = 200;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 42 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;
        let mut setup_events = Vec::new();
        let target_x = grid_x(2) + 100 * POSITION_SCALE;
        let row_one = game.spawn_normal_zombie(1, 0, Some(target_x), &mut setup_events);
        let row_two = game.spawn_normal_zombie(2, 0, Some(target_x), &mut setup_events);
        let row_three = game.spawn_normal_zombie(3, 0, Some(target_x), &mut setup_events);
        let far_row = game.spawn_normal_zombie(0, 0, Some(target_x), &mut setup_events);

        let events = (0..60)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();

        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileFired {
                projectile_type: ProjectileType::Puff,
                ..
            }
        )));
        for target in [row_one, row_two, row_three] {
            assert!(events.iter().any(|event| matches!(
                event,
                GameEvent::ProjectileHit {
                    zombie,
                    damage: 20,
                    ..
                } if *zombie == target
            )));
            assert_eq!(
                game.state
                    .board
                    .zombies
                    .iter()
                    .find(|zombie| zombie.id == target)
                    .unwrap()
                    .health,
                250
            );
        }
        assert!(!events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileHit { zombie, .. } if *zombie == far_row
        )));
    }

    #[test]
    fn peashooter_projectile_damages_a_normal_zombie() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 150;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;
        let mut setup_events = Vec::new();
        game.spawn_normal_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup_events);

        let mut hit = false;
        let mut impacts = 0;
        for _ in 0..200 {
            for event in game.advance(InputFrame::default()) {
                hit |= matches!(event, GameEvent::ProjectileHit { damage: 20, .. });
                if matches!(
                    event,
                    GameEvent::ProjectileImpact {
                        kind: ProjectileImpactSound::Splat,
                        variant: 0..=2,
                        ..
                    }
                ) {
                    impacts += 1;
                }
            }
        }

        assert!(hit);
        assert!(impacts > 0);
        assert!(
            game.advance(InputFrame::default())
                .iter()
                .all(|event| !matches!(event, GameEvent::ZombieDied { .. }))
        );
        assert!(game.state.board.zombies[0].health < 270);
    }

    #[test]
    fn splitpea_fires_forward_and_backward_projectiles() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 250;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 28 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        game.state.board.plants[0].shooting_counter = 2;
        let mut setup_events = Vec::new();
        let forward = game.spawn_normal_zombie(2, 0, Some(250 * POSITION_SCALE), &mut setup_events);
        let backward =
            game.spawn_normal_zombie(2, 0, Some(150 * POSITION_SCALE), &mut setup_events);

        let events = game.advance(InputFrame::default());

        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event, GameEvent::ProjectileFired { .. }))
                .count(),
            2
        );
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event, GameEvent::ProjectileHit { .. }))
                .count(),
            2
        );
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileHit {
                zombie,
                damage: 20,
                ..
            } if *zombie == forward
        )));
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileHit {
                zombie,
                damage: 20,
                ..
            } if *zombie == backward
        )));
        assert!(game.state.board.projectiles.is_empty());
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .map(|zombie| (zombie.id, zombie.health))
                .collect::<Vec<_>>(),
            vec![(forward, 250), (backward, 250)]
        );
    }

    #[test]
    fn torchwood_turns_pea_shots_into_fireballs() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 300;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 22 },
                InputAction::Plant { row: 2, column: 1 },
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state
            .board
            .plants
            .iter_mut()
            .find(|plant| plant.plant_type == PlantType::Peashooter)
            .expect("peashooter")
            .launch_counter = 1;
        let mut setup_events = Vec::new();
        game.spawn_normal_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup_events);
        game.spawn_normal_zombie(1, 0, Some(500 * POSITION_SCALE), &mut setup_events);

        let mut saw_fireball = false;
        let mut saw_fireball_hit = false;
        let mut saw_fireball_splash = false;
        for _ in 0..140 {
            let events = game.advance(InputFrame::default());
            saw_fireball |= game.state.board.projectiles.iter().any(|projectile| {
                projectile.projectile_type == ProjectileType::Fireball && projectile.damage == 40
            });
            saw_fireball_hit |= events
                .iter()
                .any(|event| matches!(event, GameEvent::ProjectileHit { damage: 40, .. }));
            saw_fireball_splash |= events
                .iter()
                .any(|event| matches!(event, GameEvent::ProjectileSplashHit { damage: 13, .. }));
            if saw_fireball && saw_fireball_hit && saw_fireball_splash {
                break;
            }
        }

        assert!(saw_fireball);
        assert!(saw_fireball_hit);
        assert!(saw_fireball_splash);
    }

    #[test]
    fn repeater_emits_a_two_shot_burst() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 250;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 7 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;
        let mut setup_events = Vec::new();
        game.spawn_normal_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup_events);

        let fired = (0..50)
            .flat_map(|_| game.advance(InputFrame::default()))
            .filter(|event| matches!(event, GameEvent::ProjectileFired { .. }))
            .count();

        assert_eq!(fired, 2);
    }

    #[test]
    fn threepeater_targets_the_three_adjacent_rows() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 400;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 18 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;
        let mut setup_events = Vec::new();
        for row in 1..=3 {
            game.spawn_normal_zombie(row, 0, Some(500 * POSITION_SCALE), &mut setup_events);
        }

        let fired_rows = (0..40)
            .flat_map(|_| game.advance(InputFrame::default()))
            .filter_map(|event| match event {
                GameEvent::ProjectileFired { row, .. } => Some(row),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(fired_rows, vec![1, 2, 3]);
    }

    #[test]
    fn gatlingpea_emits_a_four_shot_burst() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 300;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 40 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;
        let mut setup_events = Vec::new();
        game.spawn_normal_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup_events);

        let fired = (0..70)
            .flat_map(|_| game.advance(InputFrame::default()))
            .filter(|event| matches!(event, GameEvent::ProjectileFired { .. }))
            .count();

        assert_eq!(fired, 4);
    }

    #[test]
    fn cactus_fires_a_spike_projectile() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 200;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 26 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;
        let mut setup_events = Vec::new();
        let zombie = game.spawn_normal_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup_events);

        let mut fired = false;
        let mut hit = false;
        for _ in 0..200 {
            for event in game.advance(InputFrame::default()) {
                fired |= matches!(
                    event,
                    GameEvent::ProjectileFired {
                        projectile_type: ProjectileType::Spike,
                        ..
                    }
                );
                hit |= matches!(
                    event,
                    GameEvent::ProjectileHit {
                        zombie: hit_zombie,
                        damage: 20,
                        ..
                    } if hit_zombie == zombie
                );
            }
            if hit {
                break;
            }
        }

        assert!(fired);
        assert!(hit);
    }

    #[test]
    fn leftpeater_fires_only_backward() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut events = Vec::new();
        game.fire_projectiles(1, PlantType::Other(52), 2, 2, &mut events);

        assert!(matches!(
            events.as_slice(),
            [
                GameEvent::PlantFired {
                    entity: 1,
                    plant_type: PlantType::Other(52),
                    ..
                },
                GameEvent::ProjectileFired {
                    entity: 1,
                    source: 1,
                    projectile_type: ProjectileType::Pea,
                    row: 2,
                }
            ]
        ));
        assert_eq!(game.state.board.projectiles.len(), 1);
        assert_eq!(
            game.state.board.projectiles[0].motion,
            ProjectileMotion::Backwards
        );
        assert!(game.state.board.projectiles[0].velocity_x < 0);
    }

    #[test]
    fn starfruit_emits_five_directional_projectiles() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 250;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 29 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;
        let mut setup_events = Vec::new();
        game.spawn_normal_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup_events);

        let events = (0..40)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();

        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event, GameEvent::ProjectileFired { .. }))
                .count(),
            5
        );
        assert_eq!(game.state.board.projectiles.len(), 5);
        assert!(
            game.state
                .board
                .projectiles
                .iter()
                .any(|projectile| projectile.velocity_y != 0)
        );
    }

    #[test]
    fn snowpea_chills_a_zombie_after_impact() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 250;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 5 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;
        let mut setup_events = Vec::new();
        let zombie = game.spawn_normal_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup_events);

        let chilled = (0..200)
            .flat_map(|_| game.advance(InputFrame::default()))
            .any(|event| {
                matches!(
                    event,
                    GameEvent::ZombieChilled {
                        entity,
                        duration: 1_000
                    } if entity == zombie
                )
            });

        assert!(chilled);
        assert!(game.state.board.zombies[0].chilled_counter > 0);
    }

    #[test]
    fn ice_shroom_freezes_every_normal_zombie_and_applies_target_damage() {
        let mut game = Game::new(7, SceneKind::Night);
        game.state.sun = 200;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 14 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        game.state.board.plants[0].special_counter = 1;
        let center = grid_x(2);
        let mut setup_events = Vec::new();
        let first = game.spawn_normal_zombie(0, 0, Some(center), &mut setup_events);
        let second = game.spawn_normal_zombie(4, 0, Some(center), &mut setup_events);
        game.state.board.zombies[1].chilled_counter = 1;

        let events = game.advance(InputFrame::default());

        assert_eq!(game.state.board.ice_counter, BOARD_ICE_TICKS - 1);
        assert!(game.state.board.plants.is_empty());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantSpecialTriggered {
                plant_type: PlantType::Other(14),
                ..
            }
        )));
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(
                    event,
                    GameEvent::ZombieChilled {
                        duration: ICE_SHROOM_CHILL_TICKS,
                        ..
                    }
                ))
                .count(),
            2
        );
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieFrozen {
                entity,
                duration: ICE_SHROOM_INITIAL_FREEZE_TICKS,
            } if *entity == first
        )));
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieFrozen {
                entity,
                duration: ICE_SHROOM_REFRESH_FREEZE_TICKS,
            } if *entity == second
        )));
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(
                    event,
                    GameEvent::PlantSpecialHit {
                        damage: ICE_SHROOM_DAMAGE,
                        ..
                    }
                ))
                .count(),
            2
        );
        assert!(
            game.state
                .board
                .zombies
                .iter()
                .all(|zombie| zombie.health == 270 - ICE_SHROOM_DAMAGE)
        );
    }

    #[test]
    fn frozen_zombies_do_not_move_or_eat_until_the_counter_expires() {
        let mut game = Game::new(7, SceneKind::Night);
        game.state.sun = 200;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 14 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        game.state.board.plants[0].special_counter = 1;
        let mut setup_events = Vec::new();
        let zombie = game.spawn_normal_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup_events);
        let before = game
            .state
            .board
            .zombies
            .iter()
            .find(|candidate| candidate.id == zombie)
            .expect("spawned zombie")
            .position_x;

        game.advance(InputFrame::default());

        let frozen = game
            .state
            .board
            .zombies
            .iter()
            .find(|candidate| candidate.id == zombie)
            .expect("frozen zombie")
            .clone();
        assert_eq!(frozen.position_x, before);
        assert!(!frozen.eating);
        assert!(frozen.frozen_counter > 0);
    }

    #[test]
    fn gravebuster_requires_and_clears_a_grave_after_four_hundred_ticks() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 75;

        let rejected = game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 11 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        assert!(rejected.iter().any(|event| matches!(
            event,
            GameEvent::InputRejected {
                reason: InputRejectReason::InvalidTerrain,
                ..
            }
        )));

        game.state
            .board
            .graves
            .push(GraveState { row: 2, column: 2 });
        let placed = game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 11 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let gravebuster = game.state.board.plants[0].id;
        assert!(placed.iter().any(|event| matches!(
            event,
            GameEvent::PlantPlaced {
                entity,
                plant_type: PlantType::Other(11),
                ..
            } if *entity == gravebuster
        )));
        assert_eq!(
            game.state.board.plants[0].special_counter,
            GRAVEBUSTER_EAT_TICKS - 1
        );

        let mut cleared = false;
        for _ in 0..(GRAVEBUSTER_EAT_TICKS - 1) {
            cleared |= game.advance(InputFrame::default()).iter().any(|event| {
                matches!(
                    event,
                    GameEvent::GraveCleared { entity, row: 2, column: 2 }
                        if *entity == gravebuster
                )
            });
        }

        assert!(cleared);
        assert!(game.state.board.graves.is_empty());
        assert!(game.state.board.plants.is_empty());
    }

    #[test]
    fn doom_shroom_removes_targets_and_leaves_a_replant_blocking_crater() {
        let mut game = Game::new(7, SceneKind::Night);
        game.state.sun = 300;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 15 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        game.state.board.plants[0].special_counter = 1;
        let center = grid_x(2);
        let mut setup_events = Vec::new();
        game.spawn_normal_zombie(2, 0, Some(center + 100 * POSITION_SCALE), &mut setup_events);
        game.spawn_normal_zombie(0, 0, Some(center + 100 * POSITION_SCALE), &mut setup_events);
        let survivor =
            game.spawn_normal_zombie(2, 0, Some(center + 300 * POSITION_SCALE), &mut setup_events);

        let events = game.advance(InputFrame::default());

        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::CraterCreated {
                row: 2,
                column: 2,
                duration: DOOM_CRATER_TICKS,
            }
        )));
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieDied { entity } if *entity != survivor
        )));
        assert_eq!(game.state.board.zombies.len(), 1);
        assert_eq!(game.state.board.zombies[0].id, survivor);
        assert_eq!(game.state.board.craters.len(), 1);
        assert_eq!(game.state.board.craters[0].remaining, DOOM_CRATER_TICKS - 1);
        assert!(game.state.board.plants.is_empty());

        let rejected = game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        assert!(rejected.iter().any(|event| matches!(
            event,
            GameEvent::InputRejected {
                reason: InputRejectReason::Crater,
                ..
            }
        )));
    }

    #[test]
    fn melon_splash_damages_an_adjacent_row() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 500;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 39 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;
        let mut setup_events = Vec::new();
        game.spawn_normal_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup_events);
        game.spawn_normal_zombie(3, 0, Some(500 * POSITION_SCALE), &mut setup_events);

        let splash = (0..200)
            .flat_map(|_| game.advance(InputFrame::default()))
            .any(|event| matches!(event, GameEvent::ProjectileSplashHit { damage: 26, .. }));

        assert!(splash);
        assert_eq!(game.state.board.zombies[1].health, 244);
    }

    #[test]
    fn cabbagepult_fires_a_lobbed_cabbage_for_target_damage() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 200;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 32 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;
        let mut setup_events = Vec::new();
        let zombie = game.spawn_normal_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup_events);

        let mut fired = false;
        let mut hit = false;
        for _ in 0..200 {
            for event in game.advance(InputFrame::default()) {
                fired |= matches!(
                    event,
                    GameEvent::ProjectileFired {
                        projectile_type: ProjectileType::Cabbage,
                        row: 2,
                        ..
                    }
                );
                hit |= matches!(
                    event,
                    GameEvent::ProjectileHit {
                        zombie: hit_zombie,
                        damage: 40,
                        ..
                    } if hit_zombie == zombie
                );
            }
            if hit {
                break;
            }
        }

        assert!(fired);
        assert!(hit);
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == zombie)
                .unwrap()
                .health,
            230
        );
    }

    #[test]
    fn kernelpult_fires_lobbed_kernel_or_butter_shots() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut events = Vec::new();
        for _ in 0..128 {
            game.fire_projectiles(1, PlantType::Other(34), 2, 0, &mut events);
        }

        let fired = events
            .iter()
            .filter_map(|event| match event {
                GameEvent::ProjectileFired {
                    projectile_type, ..
                } => Some(*projectile_type),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(fired.len(), 128);
        assert!(fired.contains(&ProjectileType::Kernel));
        assert!(fired.contains(&ProjectileType::Butter));
        assert!(
            game.state
                .board
                .projectiles
                .iter()
                .all(|projectile| projectile.motion == ProjectileMotion::Lobbed)
        );
    }

    #[test]
    fn wintermelon_splash_chills_the_primary_and_adjacent_targets() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 300;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 44 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;
        let mut setup_events = Vec::new();
        let primary = game.spawn_normal_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup_events);
        let adjacent =
            game.spawn_normal_zombie(3, 0, Some(500 * POSITION_SCALE), &mut setup_events);

        let mut primary_hit = false;
        let mut adjacent_hit = false;
        let mut adjacent_chilled = false;
        for _ in 0..200 {
            for event in game.advance(InputFrame::default()) {
                primary_hit |= matches!(
                    event,
                    GameEvent::ProjectileHit {
                        zombie,
                        damage: 80,
                        ..
                    } if zombie == primary
                );
                adjacent_hit |= matches!(
                    event,
                    GameEvent::ProjectileSplashHit {
                        zombie,
                        damage: 26,
                        ..
                    } if zombie == adjacent
                );
                adjacent_chilled |= matches!(
                    event,
                    GameEvent::ZombieChilled {
                        entity,
                        duration: 1_000,
                    } if entity == adjacent
                );
            }
            if primary_hit && adjacent_hit && adjacent_chilled {
                break;
            }
        }

        assert!(primary_hit);
        assert!(adjacent_hit);
        assert!(adjacent_chilled);
        assert!(
            game.state
                .board
                .zombies
                .iter()
                .any(|zombie| zombie.id == adjacent && zombie.chilled_counter > 0)
        );
    }

    #[test]
    fn cattail_homes_across_rows() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 500;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 43 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;
        let mut setup_events = Vec::new();
        let zombie = game.spawn_normal_zombie(4, 0, Some(500 * POSITION_SCALE), &mut setup_events);

        let hit = (0..200)
            .flat_map(|_| game.advance(InputFrame::default()))
            .any(|event| matches!(event, GameEvent::ProjectileHit { zombie: hit_zombie, .. } if hit_zombie == zombie));

        assert!(hit);
    }

    #[test]
    fn sunshroom_starts_with_small_sun() {
        let mut game = Game::new(7, SceneKind::Night);
        game.state.sun = 100;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 9 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;

        let events = game.advance(InputFrame::default());

        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::SunProduced {
                value: SMALL_SUN_VALUE,
                ..
            }
        )));
        assert_eq!(game.state.board.suns[0].value, SMALL_SUN_VALUE);
    }

    #[test]
    fn sunshroom_grows_to_normal_sun() {
        let mut game = Game::new(7, SceneKind::Night);
        game.state.sun = 100;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 9 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].production_age = SUNSHROOM_GROWTH_TICKS - 1;
        game.state.board.plants[0].launch_counter = 1;

        let events = game.advance(InputFrame::default());

        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::SunProduced { value: 25, .. }))
        );
        assert_eq!(game.state.board.plants[0].production_stage, 1);
    }

    #[test]
    fn twin_sunflower_produces_two_suns() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 250;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 41 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;

        let events = game.advance(InputFrame::default());

        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event, GameEvent::SunProduced { .. }))
                .count(),
            2
        );
        assert_eq!(game.state.board.suns.len(), 2);
    }

    #[test]
    fn cherry_bomb_explodes_in_its_radius() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 250;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 2 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        game.state.board.plants[0].special_counter = 1;
        let center = grid_x(2);
        let mut setup_events = Vec::new();
        game.spawn_normal_zombie(2, 0, Some(center), &mut setup_events);
        game.spawn_normal_zombie(3, 0, Some(center), &mut setup_events);

        let events = game.advance(InputFrame::default());

        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantSpecialTriggered {
                plant_type: PlantType::Other(2),
                ..
            }
        )));
        assert!(game.state.board.plants.is_empty());
        assert!(game.state.board.zombies.is_empty());
    }

    #[test]
    fn potato_mine_arms_before_triggering() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 100;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 4 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        game.state.board.plants[0].special_counter = 1;
        let center = grid_x(2);
        let mut setup_events = Vec::new();
        game.spawn_normal_zombie(2, 0, Some(center + 50 * POSITION_SCALE), &mut setup_events);

        game.advance(InputFrame::default());
        assert!(game.state.board.plants[0].special_armed);
        assert_eq!(game.state.board.zombies.len(), 1);

        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantSpecialTriggered {
                plant_type: PlantType::Other(4),
                ..
            }
        )));
        assert!(game.state.board.plants.is_empty());
        assert!(game.state.board.zombies.is_empty());
    }

    #[test]
    fn squash_tracks_nearby_normal_zombie_then_smashes_it() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 100;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 17 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let squash = game.state.board.plants[0].id;
        let squash_thumps = |events: &[GameEvent]| {
            events
                .iter()
                .filter(|event| {
                    matches!(
                        event,
                        GameEvent::PlantSpecialTriggered {
                            entity,
                            plant_type: PlantType::Other(17),
                        } if *entity == squash
                    )
                })
                .count()
        };
        let center = grid_x(2);
        let mut setup_events = Vec::new();
        let target =
            game.spawn_normal_zombie(2, 0, Some(center + 50 * POSITION_SCALE), &mut setup_events);
        let far =
            game.spawn_normal_zombie(2, 0, Some(center + 200 * POSITION_SCALE), &mut setup_events);
        let other_row =
            game.spawn_normal_zombie(1, 0, Some(center + 50 * POSITION_SCALE), &mut setup_events);

        let acquired = game.advance(InputFrame::default());
        assert!(
            !acquired
                .iter()
                .any(|event| matches!(event, GameEvent::PlantSpecialHit { .. }))
        );
        assert!(acquired.iter().any(|event| matches!(
            event,
            GameEvent::SquashHumStarted { entity, variant }
                if *entity == squash && *variant < 3
        )));
        assert_eq!(game.state.board.plants[0].special_target, Some(target));
        assert_eq!(
            game.state.board.plants[0].special_counter,
            SQUASH_LOOK_TICKS
        );
        assert!(!game.state.board.plants[0].special_armed);
        assert_eq!(
            game.find_plant_for_zombie(2, center, ZombieType::Normal),
            Some(0),
            "STATE_SQUASH_LOOKING remains on the ground"
        );

        game.state.board.plants[0].special_counter = 1;
        let jump = game.advance(InputFrame::default());
        assert!(
            !jump
                .iter()
                .any(|event| matches!(event, GameEvent::PlantSpecialHit { .. }))
        );
        assert!(game.state.board.plants[0].special_armed);
        assert_eq!(
            game.state.board.plants[0].special_counter,
            SQUASH_HIT_DELAY_TICKS
        );
        assert_eq!(squash_thumps(&jump), 0);
        assert_eq!(
            game.find_plant_for_zombie(2, center, ZombieType::Normal),
            Some(0),
            "the 45-tick pre-launch remains on the ground"
        );

        game.state.board.plants[0].special_counter = SQUASH_OFF_GROUND_TICKS + 2;
        let pre_launch = game.advance(InputFrame::default());
        assert_eq!(squash_thumps(&pre_launch), 0);
        assert_eq!(
            game.state.board.plants[0].special_counter,
            SQUASH_OFF_GROUND_TICKS + 1
        );
        assert_eq!(
            game.find_plant_for_zombie(2, center, ZombieType::Normal),
            Some(0),
            "counter 56 is the final pre-launch tick"
        );

        let rising = game.advance(InputFrame::default());
        assert_eq!(squash_thumps(&rising), 0);
        assert_eq!(
            game.state.board.plants[0].special_counter,
            SQUASH_OFF_GROUND_TICKS
        );
        assert_eq!(
            game.find_plant_for_zombie(2, center, ZombieType::Normal),
            None,
            "counter 55 starts STATE_SQUASH_RISING"
        );

        game.state.board.plants[0].special_counter = 1;
        let events = game.advance(InputFrame::default());

        assert_eq!(squash_thumps(&events), 0, "damage precedes the thump");
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantSpecialHit {
                plant,
                zombie,
                damage: PLANT_SPECIAL_DAMAGE,
                ..
            } if *plant == squash && *zombie == target
        )));
        assert!(
            events.iter().any(
                |event| matches!(event, GameEvent::ZombieDied { entity } if *entity == target)
            )
        );
        assert!(
            !events
                .iter()
                .any(|event| matches!(event, GameEvent::PlantDied { entity } if *entity == squash))
        );
        assert_eq!(game.state.board.plants[0].id, squash);
        assert_eq!(
            game.state.board.plants[0].special_counter,
            SQUASH_LANDING_HIT_TICKS
        );
        assert!(game.state.board.plants[0].special_armed);
        assert_eq!(game.state.board.plants[0].special_target, None);
        assert_eq!(
            game.find_plant_for_zombie(2, center, ZombieType::Normal),
            None,
            "STATE_SQUASH_FALLING cannot be bitten"
        );
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .map(|zombie| zombie.id)
                .collect::<Vec<_>>(),
            vec![far, other_row]
        );

        for _ in 1..SQUASH_LANDING_HIT_TICKS {
            let events = game.advance(InputFrame::default());
            assert_eq!(squash_thumps(&events), 0);
            assert!(!events.iter().any(
                |event| matches!(event, GameEvent::PlantDied { entity } if *entity == squash)
            ));
            assert_eq!(game.state.board.plants[0].id, squash);
        }
        let landed = game.advance(InputFrame::default());
        assert_eq!(squash_thumps(&landed), 1);
        assert!(
            !landed
                .iter()
                .any(|event| matches!(event, GameEvent::PlantDied { entity } if *entity == squash))
        );
        assert_eq!(
            game.state.board.plants[0].special_counter,
            SQUASH_DONE_FALLING_TICKS
        );
        assert!(!game.state.board.plants[0].special_armed);
        assert_eq!(game.state.board.plants[0].special_target, None);
        assert_eq!(
            game.find_plant_for_zombie(2, center, ZombieType::Normal),
            None,
            "STATE_SQUASH_DONE_FALLING cannot be bitten"
        );

        for _ in 1..SQUASH_DONE_FALLING_TICKS {
            let events = game.advance(InputFrame::default());
            assert_eq!(squash_thumps(&events), 0);
            assert!(!events.iter().any(
                |event| matches!(event, GameEvent::PlantDied { entity } if *entity == squash)
            ));
            assert_eq!(game.state.board.plants[0].id, squash);
        }
        let finished = game.advance(InputFrame::default());
        assert_eq!(squash_thumps(&finished), 0);
        assert!(
            finished
                .iter()
                .any(|event| matches!(event, GameEvent::PlantDied { entity } if *entity == squash))
        );
        assert!(game.state.board.plants.is_empty());
    }

    #[test]
    fn squash_dies_when_it_lands_in_pool_or_fog_water() {
        for scene in [SceneKind::Pool, SceneKind::Fog] {
            let mut game = Game::new(7, scene);
            game.place_izombie_plant(PlantType::Other(17), 2, 2);
            let squash = game.state.board.plants[0].id;
            let mut setup_events = Vec::new();
            let target = game.spawn_normal_zombie(
                2,
                0,
                Some(grid_x(2) + 50 * POSITION_SCALE),
                &mut setup_events,
            );
            let plant = &mut game.state.board.plants[0];
            plant.special_armed = true;
            plant.special_target = Some(target);
            plant.special_counter = 1;

            let damage = game.advance(InputFrame::default());
            assert!(damage.iter().any(|event| matches!(
                event,
                GameEvent::PlantSpecialHit { plant, zombie, .. }
                    if *plant == squash && *zombie == target
            )));
            assert!(!damage.iter().any(|event| matches!(
                event,
                GameEvent::PlantSpecialTriggered { entity, .. } if *entity == squash
            )));
            assert_eq!(
                game.state.board.plants[0].special_counter,
                SQUASH_LANDING_HIT_TICKS
            );

            for _ in 1..SQUASH_LANDING_HIT_TICKS {
                let events = game.advance(InputFrame::default());
                assert!(!events.iter().any(|event| matches!(
                    event,
                    GameEvent::PlantDied { entity } if *entity == squash
                )));
            }
            let landed = game.advance(InputFrame::default());
            assert!(landed.iter().any(
                |event| matches!(event, GameEvent::PlantDied { entity } if *entity == squash)
            ));
            assert!(!landed.iter().any(|event| matches!(
                event,
                GameEvent::PlantSpecialTriggered { entity, .. } if *entity == squash
            )));
            assert!(game.state.board.plants.is_empty());
        }
    }

    #[test]
    fn jalapeno_burns_every_zombie_in_its_row() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 250;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 20 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        assert_eq!(
            game.state.board.plants[0].special_counter,
            INSTANT_PLANT_COUNTDOWN - 1
        );
        game.state.board.plants[0].special_counter = 1;

        let center = grid_x(2);
        let mut setup_events = Vec::new();
        let far_left =
            game.spawn_normal_zombie(2, 0, Some(-50 * POSITION_SCALE), &mut setup_events);
        let far_right =
            game.spawn_normal_zombie(2, 0, Some(10_000 * POSITION_SCALE), &mut setup_events);
        let other_row = game.spawn_normal_zombie(1, 0, Some(center), &mut setup_events);

        let events = game.advance(InputFrame::default());

        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantSpecialTriggered {
                plant_type: PlantType::Other(20),
                ..
            }
        )));
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(
                    event,
                    GameEvent::PlantSpecialHit {
                        damage: PLANT_SPECIAL_DAMAGE,
                        ..
                    }
                ))
                .count(),
            2
        );
        assert!(game.state.board.plants.is_empty());
        assert_eq!(game.state.board.zombies.len(), 1);
        assert_eq!(game.state.board.zombies[0].id, other_row);
        assert!(![far_left, far_right].contains(&other_row));
    }

    #[test]
    fn spikeweed_attacks_on_contact_and_is_not_eaten() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 200;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 21 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        let spikeweed = game.state.board.plants[0].id;
        let mut setup_events = Vec::new();
        let zombie = game.spawn_normal_zombie(
            2,
            0,
            Some(grid_x(0) + 10 * POSITION_SCALE),
            &mut setup_events,
        );
        let starting_health = game.state.board.zombies[0].health;

        let events = (0..=(SPIKEWEED_ATTACK_TICKS - SPIKEWEED_DAMAGE_COUNTDOWN))
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();

        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantSpecialTriggered {
                entity,
                plant_type: PlantType::Other(21),
            } if *entity == spikeweed
        )));
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantSpecialHit {
                plant,
                zombie: hit_zombie,
                damage: SPIKEWEED_DAMAGE,
                health_remaining,
            } if *plant == spikeweed
                && *hit_zombie == zombie
                && *health_remaining == starting_health - SPIKEWEED_DAMAGE
        )));
        assert_eq!(
            game.state.board.zombies[0].health,
            starting_health - SPIKEWEED_DAMAGE
        );
        // Zombies walk over spikeweed instead of chewing it.
        assert_eq!(
            game.state.board.plants[0].health,
            game.state.board.plants[0].max_health
        );
        assert!(!game.state.board.zombies[0].eating);
    }

    #[test]
    fn conehead_zombie_has_640_health_and_blocks_bites_like_normal() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_conehead_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);

        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .health,
            640
        );
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .zombie_type,
            ZombieType::Conehead
        );
    }

    #[test]
    fn flag_zombie_keeps_the_normal_body_but_walks_at_the_fixed_source_speed() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_flag_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);

        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .health,
            270
        );
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .zombie_type,
            ZombieType::Flag
        );
        // Zombie_ResetSpeed gives Flag and Backup Dancer a fixed 0.45 walk.
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .speed,
            450_000
        );
        let backup = game.spawn_backup_dancer(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == backup)
                .unwrap()
                .speed,
            450_000
        );
    }

    #[test]
    fn buckethead_zombie_has_1370_health() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_buckethead_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);

        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .health,
            1370
        );
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .zombie_type,
            ZombieType::Buckethead
        );
    }

    #[test]
    fn pole_vaulter_skips_the_first_plant_and_triggers_vault_event() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 50;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let sunflower_id = game.state.board.plants[0].id;

        let mut setup = Vec::new();
        let zombie =
            game.spawn_pole_vaulter_zombie(2, 0, Some(grid_x(2) + 20 * POSITION_SCALE), &mut setup);

        let mut vaulted = false;
        for _ in 0..100 {
            let events = game.advance(InputFrame::default());
            for event in &events {
                if matches!(event, GameEvent::ZombieVaulted { entity } if *entity == zombie) {
                    vaulted = true;
                }
            }
        }
        assert!(vaulted, "ZombieVaulted");
        assert!(
            game.state.board.plants.iter().any(|p| p.id == sunflower_id),
            "plant survived vault"
        );
    }

    #[test]
    fn screen_door_zombie_has_a_1100_shield_over_its_270_body() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_screen_door_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        let state = game
            .state
            .board
            .zombies
            .iter()
            .find(|z| z.id == zombie)
            .unwrap();
        assert_eq!(state.health, 270);
        assert_eq!(state.shield_health, SCREEN_DOOR_SHIELD_HEALTH);
        assert_eq!(state.zombie_type, ZombieType::ScreenDoor);
    }

    #[test]
    fn shield_damage_emits_the_source_hit_variation() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_screen_door_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        let zombie_index = game
            .state
            .board
            .zombies
            .iter()
            .position(|candidate| candidate.id == zombie)
            .unwrap();
        let mut events = Vec::new();

        game.damage_zombie(zombie_index, 20, &mut events);

        assert_eq!(game.state.board.zombies[zombie_index].shield_health, 1_080);
        let variant = events.iter().find_map(|event| match event {
            GameEvent::ZombieShieldHit { entity, variant } if *entity == zombie => Some(*variant),
            _ => None,
        });
        assert!(variant.is_some_and(|variant| variant < 2));
    }

    #[test]
    fn fume_damage_bypasses_the_screen_door_shield() {
        let mut game = Game::new(7, SceneKind::Night);
        game.state.sun = 1_000;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 10 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let mut setup = Vec::new();
        let zombie =
            game.spawn_screen_door_zombie(2, 0, Some(grid_x(2) + 100 * POSITION_SCALE), &mut setup);
        game.state
            .board
            .zombies
            .iter_mut()
            .for_each(|z| z.speed = 0);

        let mut hit = false;
        for _ in 0..200 {
            let events = game.advance(InputFrame::default());
            if events.iter().any(|event| {
                matches!(
                    event,
                    GameEvent::ProjectileHit { zombie: z, .. } if *z == zombie
                )
            }) {
                hit = true;
                break;
            }
        }
        assert!(hit, "the fume-shroom reaches the door zombie");
        let state = game
            .state
            .board
            .zombies
            .iter()
            .find(|z| z.id == zombie)
            .unwrap();
        assert_eq!(
            state.shield_health, SCREEN_DOOR_SHIELD_HEALTH,
            "fumes pass through the door"
        );
        assert!(state.health < 270, "the body takes the fume damage instead");
    }

    #[test]
    fn ducky_tube_zombie_has_270_health_and_ducky_type() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_ducky_tube_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .health,
            270
        );
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .zombie_type,
            ZombieType::DuckyTube
        );
    }

    #[test]
    fn dolphin_rider_enters_pool_and_jumps_over_a_plant_but_not_tallnut() {
        let mut game = Game::new(7, SceneKind::Pool);
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 24 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let plant_id = game.state.board.plants[0].id;
        let mut setup = Vec::new();
        let dolphin = game.spawn_dolphin_rider_zombie(2, 0, Some(720 * POSITION_SCALE), &mut setup);
        let dolphin_state = game
            .state
            .board
            .zombies
            .iter()
            .find(|candidate| candidate.id == dolphin)
            .unwrap();
        // Zombie_Init gives the Dolphin Rider a 500-HP body, not the plain 270.
        assert_eq!(dolphin_state.health, 500);
        assert_eq!(dolphin_state.speed, DOLPHIN_WALK_SPEED);

        game.advance(InputFrame::default());
        let dolphin_state = game
            .state
            .board
            .zombies
            .iter()
            .find(|candidate| candidate.id == dolphin)
            .unwrap();
        assert_eq!(dolphin_state.dolphin_phase, 1);
        assert_eq!(dolphin_state.speed, DOLPHIN_RIDE_SPEED);

        let dolphin_state = game
            .state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == dolphin)
            .unwrap();
        dolphin_state.position_x = grid_x(2) + 20 * POSITION_SCALE;
        dolphin_state.speed = 0;
        dolphin_state.age = 3;
        let jump_events = game.advance(InputFrame::default());
        assert!(jump_events.iter().any(|event| matches!(
            event,
            GameEvent::DolphinJumpStarted { entity } if *entity == dolphin
        )));
        assert!(!jump_events.iter().any(|event| matches!(
            event,
            GameEvent::PlantDamaged { entity, .. } | GameEvent::PlantDied { entity }
                if *entity == plant_id
        )));
        let dolphin_state = game
            .state
            .board
            .zombies
            .iter()
            .find(|candidate| candidate.id == dolphin)
            .unwrap();
        assert_eq!(dolphin_state.dolphin_phase, 2);
        assert_eq!(dolphin_state.dolphin_counter, DOLPHIN_JUMP_TIME);
        assert_eq!(game.state.board.plants[0].health, 300);

        for _ in 0..DOLPHIN_JUMP_TIME {
            game.advance(InputFrame::default());
        }
        let dolphin_state = game
            .state
            .board
            .zombies
            .iter()
            .find(|candidate| candidate.id == dolphin)
            .unwrap();
        assert_eq!(dolphin_state.dolphin_phase, 3);
        assert_eq!(
            dolphin_state.position_x,
            grid_x(2) - DOLPHIN_JUMP_TARGET_OFFSET - DOLPHIN_POOL_SPEED
        );
        assert_eq!(game.state.board.plants[0].health, 300);

        let mut blocked = Game::new(7, SceneKind::Day);
        blocked.state.sun = 125;
        blocked.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 23 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let tallnut_health = blocked.state.board.plants[0].health;
        let mut setup = Vec::new();
        let blocked_dolphin = blocked.spawn_dolphin_rider_zombie(
            2,
            0,
            Some(grid_x(2) + 20 * POSITION_SCALE),
            &mut setup,
        );
        let dolphin_state = blocked
            .state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == blocked_dolphin)
            .unwrap();
        dolphin_state.dolphin_phase = 1;
        dolphin_state.speed = 0;
        dolphin_state.age = 3;
        blocked.advance(InputFrame::default());
        let dolphin_state = blocked
            .state
            .board
            .zombies
            .iter()
            .find(|candidate| candidate.id == blocked_dolphin)
            .unwrap();
        assert_eq!(dolphin_state.dolphin_phase, 1);
        assert_eq!(
            blocked.state.board.plants[0].health,
            tallnut_health - ZOMBIE_BITE_DAMAGE
        );
    }

    #[test]
    fn snorkel_enters_pool_and_is_hidden_until_it_eats() {
        let mut game = Game::new(7, SceneKind::Pool);
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 24 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        game.state.board.plants[0].launch_counter = 10_000;
        let mut setup = Vec::new();
        let snorkel = game.spawn_snorkel_zombie(2, 0, Some(720 * POSITION_SCALE), &mut setup);
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == snorkel)
                .unwrap()
                .speed,
            SNORKEL_SPEED
        );

        game.advance(InputFrame::default());
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == snorkel)
                .unwrap()
                .snorkel_phase,
            1
        );

        let snorkel_position = {
            let snorkel_state = game
                .state
                .board
                .zombies
                .iter_mut()
                .find(|candidate| candidate.id == snorkel)
                .unwrap();
            snorkel_state.position_x = grid_x(2) + 20 * POSITION_SCALE;
            snorkel_state.speed = 0;
            snorkel_state.age = 0;
            snorkel_state.position_x
        };
        let mut projectile_events = Vec::new();
        game.fire_projectile(
            0,
            ProjectileType::Pea,
            2,
            ProjectileTrajectory {
                motion: ProjectileMotion::Straight,
                position_x: snorkel_position,
                position_y: grid_y(2),
                velocity_x: 0,
                velocity_y: 0,
            },
            &mut projectile_events,
        );
        let hidden_events = game.advance(InputFrame::default());
        assert!(!hidden_events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileHit { zombie, .. } if *zombie == snorkel
        )));
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == snorkel)
                .unwrap()
                .health,
            270
        );

        game.state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == snorkel)
            .unwrap()
            .eating = true;
        let mut projectile_events = Vec::new();
        game.fire_projectile(
            0,
            ProjectileType::Pea,
            2,
            ProjectileTrajectory {
                motion: ProjectileMotion::Straight,
                position_x: snorkel_position,
                position_y: grid_y(2),
                velocity_x: 0,
                velocity_y: 0,
            },
            &mut projectile_events,
        );
        let exposed_events = game.advance(InputFrame::default());
        assert!(exposed_events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileHit {
                zombie,
                damage: 20,
                health_remaining: 250,
                ..
            } if *zombie == snorkel
        )));
    }

    #[test]
    fn zamboni_drives_over_a_plant_with_source_speed_profile() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 50;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let plant = game.state.board.plants[0].id;
        let mut setup = Vec::new();
        let zamboni =
            game.spawn_zamboni_zombie(2, 0, Some(grid_x(2) + 20 * POSITION_SCALE), &mut setup);
        let zamboni_state = game
            .state
            .board
            .zombies
            .iter()
            .find(|candidate| candidate.id == zamboni)
            .unwrap();
        assert_eq!(zamboni_state.health, ZAMBONI_HEALTH);
        assert_eq!(zamboni_state.speed, zamboni_speed(zamboni_state.position_x));
        // The source stops recomputing mVelX below x=400: the 0.10 curve
        // value at x=400 holds instead of tapering to 0.05.
        assert_eq!(zamboni_state.speed, 100_000);

        game.state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == zamboni)
            .unwrap()
            .age = 3;
        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantDied { entity } if *entity == plant
        )));
        assert!(game.state.board.plants.is_empty());
        assert!(
            game.state
                .board
                .zombies
                .iter()
                .any(|candidate| candidate.id == zamboni)
        );
    }

    #[test]
    fn bobsled_team_slides_as_four_then_becomes_individual_walkers() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 50;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 3 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let plant = game.state.board.plants[0].id;
        let mut setup = Vec::new();
        let leader =
            game.spawn_bobsled_zombie(2, 0, Some(grid_x(2) + 20 * POSITION_SCALE), &mut setup);

        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .filter(|zombie| zombie.zombie_type == ZombieType::Bobsled)
                .count(),
            4
        );
        let leader_state = game
            .state
            .board
            .zombies
            .iter()
            .find(|zombie| zombie.id == leader)
            .unwrap();
        assert_eq!(leader_state.health, BOBSLED_HEALTH);
        assert_eq!(leader_state.shield_health, BOBSLED_HELM_HEALTH);
        assert_eq!(leader_state.speed, BOBSLED_SPEED);

        let leader_index = game
            .state
            .board
            .zombies
            .iter()
            .position(|zombie| zombie.id == leader)
            .unwrap();
        let mut damage_events = Vec::new();
        game.damage_zombie(leader_index, 20, &mut damage_events);
        assert_eq!(
            game.state.board.zombies[leader_index].health,
            BOBSLED_HEALTH
        );
        assert_eq!(
            game.state.board.zombies[leader_index].shield_health,
            BOBSLED_HELM_HEALTH - 20
        );

        for zombie in &mut game.state.board.zombies {
            zombie.speed = 0;
            zombie.age = 3;
            zombie.bobsled_counter = 2;
        }
        let events = game.advance(InputFrame::default());
        assert_eq!(game.state.board.plants[0].id, plant);
        assert_eq!(game.state.board.plants[0].health, 4_000);
        assert!(!events.iter().any(|event| matches!(
            event,
            GameEvent::PlantDamaged { entity, .. } if *entity == plant
        )));
        assert!(
            game.state
                .board
                .zombies
                .iter()
                .all(|zombie| zombie.bobsled_sliding)
        );

        for zombie in &mut game.state.board.zombies {
            zombie.age = 3;
            zombie.bobsled_counter = 1;
        }
        game.advance(InputFrame::default());
        assert!(
            game.state
                .board
                .zombies
                .iter()
                .all(|zombie| !zombie.bobsled_sliding)
        );
        assert!(game.state.board.plants[0].health < 4_000);
    }

    #[test]
    fn ladder_zombie_places_a_ladder_on_barriers_and_later_zombies_pass_it() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 50;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 3 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        let plant = game.state.board.plants[0].id;
        let mut setup = Vec::new();
        let ladder =
            game.spawn_ladder_zombie(2, 0, Some(grid_x(0) + 20 * POSITION_SCALE), &mut setup);
        let ladder_index = game
            .state
            .board
            .zombies
            .iter()
            .position(|zombie| zombie.id == ladder)
            .unwrap();
        let mut damage_events = Vec::new();
        game.damage_zombie(ladder_index, 20, &mut damage_events);
        assert_eq!(game.state.board.zombies[ladder_index].health, LADDER_HEALTH);
        assert_eq!(
            game.state.board.zombies[ladder_index].shield_health,
            LADDER_SHIELD_HEALTH - 20
        );

        game.state.board.zombies[ladder_index].speed = 0;
        game.state.board.zombies[ladder_index].age = 3;
        let events = game.advance(InputFrame::default());

        assert!(events.iter().all(|event| !matches!(
            event,
            GameEvent::PlantDamaged { entity, .. } if *entity == plant
        )));
        assert_eq!(game.state.board.plants[0].health, 4_000);
        assert_eq!(
            game.state.board.ladders,
            vec![LadderState { row: 2, column: 0 }]
        );
        let ladder_state = game
            .state
            .board
            .zombies
            .iter()
            .find(|zombie| zombie.id == ladder)
            .unwrap();
        assert!(ladder_state.ladder_placed);
        assert_eq!(ladder_state.shield_health, 0);

        let mut normal_setup = Vec::new();
        let normal = game.spawn_normal_zombie(
            2,
            0,
            Some(grid_x(0) + 20 * POSITION_SCALE),
            &mut normal_setup,
        );
        let normal_index = game
            .state
            .board
            .zombies
            .iter()
            .position(|zombie| zombie.id == normal)
            .unwrap();
        game.state.board.zombies[normal_index].speed = 0;
        game.state.board.zombies[normal_index].age = 3;
        game.advance(InputFrame::default());
        assert_eq!(game.state.board.plants[0].health, 4_000);
        assert!(!game.state.board.zombies[normal_index].eating);
    }

    #[test]
    fn ladder_zombie_carries_fast_and_slows_to_a_walk_after_placing() {
        // Zombie_ResetSpeed: PHASE_LADDER_CARRYING walks at 0.79-0.81 and the
        // plain 0.23-0.32 walk returns once the ladder is placed.
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 50;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 3 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        let mut setup = Vec::new();
        let ladder =
            game.spawn_ladder_zombie(2, 0, Some(grid_x(0) + 20 * POSITION_SCALE), &mut setup);
        let carrying_speed = game
            .state
            .board
            .zombies
            .iter()
            .find(|zombie| zombie.id == ladder)
            .unwrap()
            .speed;
        assert!(
            (790_000..=810_000).contains(&carrying_speed),
            "carrying speed {carrying_speed} should be in the 0.79-0.81 band"
        );

        for _ in 0..8 {
            game.advance(InputFrame::default());
            if game
                .state
                .board
                .zombies
                .iter()
                .find(|zombie| zombie.id == ladder)
                .unwrap()
                .ladder_placed
            {
                break;
            }
        }
        let placed = game
            .state
            .board
            .zombies
            .iter()
            .find(|zombie| zombie.id == ladder)
            .unwrap();
        assert!(placed.ladder_placed);
        assert!(
            (230_000..=320_000).contains(&placed.speed),
            "placed speed {} should re-pick the plain walk band",
            placed.speed
        );
    }

    #[test]
    fn balloon_starts_with_270_body_health_and_20_flying_health() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let balloon = game.spawn_balloon_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        let state = game
            .state
            .board
            .zombies
            .iter()
            .find(|candidate| candidate.id == balloon)
            .unwrap();

        assert_eq!(state.health, 270);
        assert_eq!(state.max_health, 270);
        assert_eq!(state.balloon_flying_health, BALLOON_FLYING_HEALTH);
        assert_eq!(state.balloon_phase, BALLOON_FLYING_PHASE);
    }

    #[test]
    fn flying_balloon_does_not_bite_a_plant() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 50;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        game.state.board.plants[0].launch_counter = 10_000;
        let plant = game.state.board.plants[0].id;
        let mut setup = Vec::new();
        let balloon =
            game.spawn_balloon_zombie(2, 0, Some(grid_x(2) + 20 * POSITION_SCALE), &mut setup);
        let balloon_state = game
            .state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == balloon)
            .unwrap();
        balloon_state.speed = 0;
        balloon_state.age = 3;

        let events = game.advance(InputFrame::default());

        assert_eq!(game.state.board.plants[0].health, 300);
        assert!(!events.iter().any(|event| matches!(
            event,
            GameEvent::PlantDamaged { entity, .. } | GameEvent::PlantDied { entity }
                if *entity == plant
        )));
        assert!(!game.state.board.zombies[0].eating);
    }

    #[test]
    fn first_spike_hit_pops_balloon_while_peas_pass_it_by() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let balloon =
            game.spawn_balloon_zombie(2, 0, Some(grid_x(2) + 20 * POSITION_SCALE), &mut setup);
        let balloon_position = {
            let balloon_state = game
                .state
                .board
                .zombies
                .iter_mut()
                .find(|candidate| candidate.id == balloon)
                .unwrap();
            balloon_state.speed = 0;
            balloon_state.age = 3;
            balloon_state.position_x
        };
        let mut setup_events = Vec::new();
        game.fire_projectile(
            0,
            ProjectileType::Pea,
            2,
            ProjectileTrajectory {
                motion: ProjectileMotion::Straight,
                position_x: balloon_position,
                position_y: grid_y(2),
                velocity_x: 0,
                velocity_y: 0,
            },
            &mut setup_events,
        );
        let pea_events = game.advance(InputFrame::default());
        assert!(
            !pea_events
                .iter()
                .any(|event| matches!(event, GameEvent::ProjectileHit { zombie, .. } if *zombie == balloon)),
            "a pea must not hit an airborne balloon"
        );

        let mut setup_events = Vec::new();
        game.fire_projectile(
            0,
            ProjectileType::Spike,
            2,
            ProjectileTrajectory {
                motion: ProjectileMotion::Straight,
                position_x: balloon_position,
                position_y: grid_y(2),
                velocity_x: 0,
                velocity_y: 0,
            },
            &mut setup_events,
        );
        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileHit {
                zombie,
                damage: 20,
                health_remaining: 270,
                ..
            } if *zombie == balloon
        )));
        let state = game
            .state
            .board
            .zombies
            .iter()
            .find(|candidate| candidate.id == balloon)
            .unwrap();
        assert_eq!(state.health, 270);
        assert_eq!(state.balloon_flying_health, 0);
        assert_eq!(state.balloon_phase, BALLOON_POPPING_PHASE);

        for _ in 0..BALLOON_POP_TICKS {
            game.advance(InputFrame::default());
        }
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == balloon)
                .unwrap()
                .balloon_phase,
            BALLOON_WALKING_PHASE
        );
    }

    #[test]
    fn pool_lane_balloon_dies_when_a_spike_pops_it() {
        let mut game = Game::new(7, SceneKind::Pool);
        let mut setup = Vec::new();
        let balloon =
            game.spawn_balloon_zombie(2, 0, Some(grid_x(2) + 20 * POSITION_SCALE), &mut setup);
        let balloon_position = {
            let balloon_state = game
                .state
                .board
                .zombies
                .iter_mut()
                .find(|candidate| candidate.id == balloon)
                .unwrap();
            balloon_state.speed = 0;
            balloon_state.position_x
        };
        let mut setup_events = Vec::new();
        game.fire_projectile(
            0,
            ProjectileType::Spike,
            2,
            ProjectileTrajectory {
                motion: ProjectileMotion::Straight,
                position_x: balloon_position,
                position_y: grid_y(2),
                velocity_x: 0,
                velocity_y: 0,
            },
            &mut setup_events,
        );

        let events = game.advance(InputFrame::default());

        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileHit {
                zombie,
                damage: 20,
                health_remaining: 0,
                ..
            } if *zombie == balloon
        )));
        assert!(events
            .iter()
            .any(|event| matches!(event, GameEvent::ZombieDied { entity } if *entity == balloon)));
        assert!(!game
            .state
            .board
            .zombies
            .iter()
            .any(|zombie| zombie.id == balloon));
    }

    #[test]
    fn blover_blows_a_flying_balloon_off_screen() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 100;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 27 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        game.state.board.plants[0].special_counter = 1;
        let mut setup = Vec::new();
        let balloon =
            game.spawn_balloon_zombie(2, 0, Some(BLOWN_AWAY_EDGE - POSITION_SCALE), &mut setup);

        let events = game.advance(InputFrame::default());

        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::BloverTriggered { .. }))
        );
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieDied { entity } if *entity == balloon
        )));
        assert!(
            !game
                .state
                .board
                .zombies
                .iter()
                .any(|zombie| zombie.id == balloon)
        );
    }

    #[test]
    fn balloon_departure_with_a_trailing_zombie_does_not_break_the_update_loop() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 100;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 27 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        game.state.board.plants[0].special_counter = 1;
        let mut setup = Vec::new();
        let balloon =
            game.spawn_balloon_zombie(2, 0, Some(BLOWN_AWAY_EDGE - POSITION_SCALE), &mut setup);
        let walker = game.spawn_normal_zombie(3, 0, Some(500 * POSITION_SCALE), &mut setup);

        let events = game.advance(InputFrame::default());

        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieDied { entity } if *entity == balloon
        )));
        assert!(
            !game
                .state
                .board
                .zombies
                .iter()
                .any(|zombie| zombie.id == balloon)
        );
        assert!(
            game.state
                .board
                .zombies
                .iter()
                .any(|zombie| zombie.id == walker)
        );
    }

    #[test]
    fn football_zombie_has_1670_health_and_increased_speed() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_football_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .health,
            1670
        );
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .zombie_type,
            ZombieType::Football
        );
        let football_speed = game
            .state
            .board
            .zombies
            .iter()
            .find(|z| z.id == zombie)
            .unwrap()
            .speed;
        assert!(
            (660_000..=680_000).contains(&football_speed),
            "Zombie_ResetSpeed gives Football the 0.66-0.68 band, got {football_speed}"
        );
    }

    #[test]
    fn newspaper_zombie_speeds_up_after_taking_enough_damage() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_newspaper_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);

        // Zombie_Init: 270 body plus the 150-HP paper shield.
        {
            let state = game
                .state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap();
            assert_eq!(state.health, 270);
            assert_eq!(state.shield_health, NEWSPAPER_PAPER_HEALTH);
            assert_eq!(state.zombie_type, ZombieType::Newspaper);
        }

        // Destroy the paper shield to trigger the mad phase.
        game.state.board.zombies.iter_mut().for_each(|z| {
            if z.id == zombie {
                z.shield_health = 0;
            }
        });
        let pos_before = game
            .state
            .board
            .zombies
            .iter()
            .find(|z| z.id == zombie)
            .unwrap()
            .position_x;
        game.advance(InputFrame::default());
        let pos_after = game
            .state
            .board
            .zombies
            .iter()
            .find(|z| z.id == zombie)
            .unwrap()
            .position_x;
        // Zombie_ResetSpeed gives the mad phase 0.89-0.91; the lumped model
        // uses the fixed NEWSPAPER_MAD_SPEED once the paper is gone.
        assert_eq!(
            pos_before - pos_after,
            NEWSPAPER_MAD_SPEED,
            "newspaper zombie at 260 HP should run at the source mad speed"
        );
    }

    #[test]
    fn imp_zombie_has_270_health_and_imp_type() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_imp_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .health,
            270
        );
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .zombie_type,
            ZombieType::Imp
        );
    }

    #[test]
    fn gargantuar_throws_one_imp_at_the_source_animation_boundary() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 100;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 6 },
            ],
        });
        let plant = game.state.board.plants[0].id;
        let mut setup = Vec::new();
        let garg = game.spawn_gargantuar_zombie(2, 0, Some(490 * POSITION_SCALE), &mut setup);
        {
            let zombie = game
                .state
                .board
                .zombies
                .iter_mut()
                .find(|z| z.id == garg)
                .unwrap();
            zombie.speed = POSITION_SCALE;
            zombie.health = 1_499;
        }

        let events = game.advance(InputFrame::default());
        assert!(
            !events
                .iter()
                .any(|event| matches!(event, GameEvent::ImpThrown { .. })),
            "the health threshold only starts anim_throw"
        );
        let gargantuar = game
            .state
            .board
            .zombies
            .iter()
            .find(|zombie| zombie.id == garg)
            .unwrap();
        assert_eq!(gargantuar.special_phase, 1);
        assert_eq!(gargantuar.special_counter, GARGANTUAR_THROW_EVENT_STEPS);
        assert_eq!(gargantuar.position_x, 490 * POSITION_SCALE);

        for _ in 0..104 {
            let events = game.advance(InputFrame::default());
            assert!(
                !events
                    .iter()
                    .any(|event| matches!(event, GameEvent::ImpThrown { .. })),
                "the 0.74 event must not fire before update 105"
            );
        }
        assert_eq!(game.state.board.plants[0].id, plant);
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|zombie| zombie.id == garg)
                .unwrap()
                .position_x,
            490 * POSITION_SCALE,
            "the Gargantuar stands still during anim_throw"
        );

        let events = game.advance(InputFrame::default());
        let (imp, imp_variant) = events
            .iter()
            .find_map(|event| match event {
                GameEvent::ImpThrown {
                    gargantuar,
                    imp,
                    imp_variant,
                } if *gargantuar == garg => Some((*imp, *imp_variant)),
                _ => None,
            })
            .expect("gargantuar below half health throws its imp");
        assert!(imp_variant < 2, "FOLEY_IMP uses one of two source variants");
        let (imp_health, imp_position, flight_ticks) = {
            let zombie = game
                .state
                .board
                .zombies
                .iter()
                .find(|z| z.id == imp)
                .unwrap();
            (zombie.health, zombie.position_x, zombie.imp_flight_ticks)
        };
        assert_eq!(imp_health, 270, "thrown imps keep the 270 HP profile");
        assert_eq!(imp_position, (490 - 133) * POSITION_SCALE);
        assert!(
            (80..=90).contains(&flight_ticks),
            "130px throw integrates to roughly 85 flight ticks, got {flight_ticks}"
        );

        for _ in 0..36 {
            let events = game.advance(InputFrame::default());
            assert!(
                !events
                    .iter()
                    .any(|event| matches!(event, GameEvent::ImpThrown { .. })),
                "the imp throw happens exactly once"
            );
        }
        assert_eq!(game.state.board.plants[0].id, plant);
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|zombie| zombie.id == garg)
                .unwrap()
                .position_x,
            490 * POSITION_SCALE,
            "the Gargantuar remains still through update 141"
        );

        let events = game.advance(InputFrame::default());
        assert!(
            !events
                .iter()
                .any(|event| matches!(event, GameEvent::ImpThrown { .. })),
            "the imp throw happens exactly once"
        );
        let gargantuar = game
            .state
            .board
            .zombies
            .iter()
            .find(|zombie| zombie.id == garg)
            .unwrap();
        assert_eq!(gargantuar.special_phase, 0);
        assert_eq!(
            gargantuar.position_x,
            489 * POSITION_SCALE,
            "normal movement resumes when anim_throw completes"
        );
        let imp_after = game
            .state
            .board
            .zombies
            .iter()
            .find(|z| z.id == imp)
            .unwrap();
        assert_eq!(
            imp_after.position_x,
            (490 - 133) * POSITION_SCALE - 37 * IMP_THROW_SPEED_X,
            "the airborne imp travels 3 px per recovery tick"
        );

        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantDied { entity } if *entity == plant
        )));
    }

    #[test]
    fn gargantuar_throw_animation_pauses_frozen_and_runs_half_speed_chilled() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let garg = game.spawn_gargantuar_zombie(2, 0, Some(490 * POSITION_SCALE), &mut setup);
        let gargantuar = game
            .state
            .board
            .zombies
            .iter_mut()
            .find(|zombie| zombie.id == garg)
            .unwrap();
        gargantuar.speed = 0;
        gargantuar.health = 1_499;

        game.advance(InputFrame::default());
        game.state.board.zombies[0].frozen_counter = 2;
        game.advance(InputFrame::default());
        game.advance(InputFrame::default());
        assert_eq!(
            game.state.board.zombies[0].special_counter, GARGANTUAR_THROW_EVENT_STEPS,
            "freeze pauses the animation"
        );

        game.state.board.zombies[0].chilled_counter = 2;
        game.advance(InputFrame::default());
        game.advance(InputFrame::default());
        assert_eq!(
            game.state.board.zombies[0].special_counter,
            GARGANTUAR_THROW_EVENT_STEPS - 2,
            "two chilled updates equal one normal update"
        );
        game.advance(InputFrame::default());
        assert_eq!(
            game.state.board.zombies[0].special_counter,
            GARGANTUAR_THROW_EVENT_STEPS - 4
        );
    }

    #[test]
    fn zomboni_lays_ice_that_blocks_planting_and_jalapeno_melts_it() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 1_000;
        let mut setup = Vec::new();
        game.spawn_zamboni_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);

        game.advance(InputFrame::default());
        assert!(
            game.state.board.ice_timer[2] > 0,
            "the trail timer refreshes"
        );
        assert!(
            game.state.board.ice_min_x[2] <= 618 * POSITION_SCALE,
            "ice starts 118 px ahead of the zomboni"
        );

        let events = game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 2, column: 8 },
            ],
        });
        assert!(
            events.iter().any(|event| matches!(
                event,
                GameEvent::InputRejected {
                    reason: InputRejectReason::Ice,
                    ..
                }
            )),
            "iced cells reject planting"
        );
        let events = game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 2, column: 3 },
            ],
        });
        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::PlantPlaced { .. })),
            "cells left of the trail stay plantable"
        );

        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 20 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        for _ in 0..300 {
            if game
                .state
                .board
                .zombies
                .iter()
                .all(|zombie| zombie.zombie_type != ZombieType::Zamboni)
            {
                break;
            }
            game.advance(InputFrame::default());
        }
        assert!(
            game.state.board.ice_timer[2] <= JALAPENO_ICE_MELT_TICKS,
            "jalapeno drops the row ice timer to the melt window"
        );
        for _ in 0..=JALAPENO_ICE_MELT_TICKS {
            game.advance(InputFrame::default());
        }
        assert_eq!(game.state.board.ice_timer[2], 0);
        assert_eq!(game.state.board.ice_min_x[2], ICE_START_X);
        let events = game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 8 },
            ],
        });
        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::PlantPlaced { .. })),
            "melted rows are plantable again"
        );
    }

    #[test]
    fn spikeweed_pops_the_zamboni_and_dies_doing_it() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 1_000;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 21 },
                InputAction::Plant { row: 2, column: 5 },
            ],
        });
        let mut setup = Vec::new();
        let zamboni = game.spawn_zamboni_zombie(2, 0, Some(grid_x(5)), &mut setup);

        let mut popped = false;
        let mut disabled = false;
        let mut spikeweed_died = false;
        for _ in 0..200 {
            let events = game.advance(InputFrame::default());
            for event in &events {
                if matches!(
                    event,
                    GameEvent::PlantSpecialHit {
                        zombie,
                        damage: SPIKE_VEHICLE_DAMAGE,
                        ..
                    } if *zombie == zamboni
                ) {
                    popped = true;
                }
                if matches!(event, GameEvent::VehicleDisabled { entity } if *entity == zamboni) {
                    disabled = true;
                }
                if matches!(event, GameEvent::PlantDied { .. }) {
                    spikeweed_died = true;
                }
            }
            if popped && disabled && spikeweed_died {
                break;
            }
        }
        assert!(popped, "spike contact deals the 1800 vehicle damage");
        assert!(disabled, "spike contact emits the vehicle tire-pop anchor");
        assert!(spikeweed_died, "the spikeweed is destroyed by the vehicle");
        assert!(
            game.state
                .board
                .zombies
                .iter()
                .all(|zombie| zombie.id != zamboni),
            "1800 spike damage kills the 1350 HP zomboni"
        );
    }

    #[test]
    fn vehicle_damage_tiers_emit_smoke_anchor_events() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zamboni = game.spawn_zamboni_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        let projectile = game.state.board.allocate_entity();
        game.state.board.projectiles.push(ProjectileState {
            id: projectile,
            projectile_type: ProjectileType::Pea,
            motion: ProjectileMotion::Straight,
            row: 2,
            position_x: 500 * POSITION_SCALE,
            position_y: grid_y(2),
            velocity_x: 0,
            velocity_y: 0,
            damage: ZAMBONI_HEALTH / 3 + 1,
            age: 0,
            target_x: None,
            target_row: None,
            lob_height: 0,
            lob_velocity: 0,
        });
        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieDamageTierChanged { entity, tier: 1 } if *entity == zamboni
        )));

        let mut game = Game::new(7, SceneKind::Day);
        let catapult = game.spawn_catapult_zombie(1, 0, Some(500 * POSITION_SCALE), &mut setup);
        let projectile = game.state.board.allocate_entity();
        game.state.board.projectiles.push(ProjectileState {
            id: projectile,
            projectile_type: ProjectileType::Puff,
            motion: ProjectileMotion::Fume,
            row: 1,
            position_x: 420 * POSITION_SCALE,
            position_y: grid_y(1),
            velocity_x: 0,
            velocity_y: 0,
            damage: 850 * 2 / 3 + 1,
            age: 0,
            target_x: None,
            target_row: None,
            lob_height: 0,
            lob_velocity: 0,
        });
        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieDamageTierChanged { entity, tier: 2 } if *entity == catapult
        )));
    }

    #[test]
    fn bobsled_team_crashes_into_walkers_past_the_ice_end() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.board.ice_timer[2] = 400;
        game.state.board.ice_min_x[2] = 600 * POSITION_SCALE;
        let mut setup = Vec::new();
        game.spawn_bobsled_zombie(2, 0, Some(700 * POSITION_SCALE), &mut setup);

        game.advance(InputFrame::default());
        assert!(
            (498..=500).contains(&game.state.board.ice_timer[2]),
            "a sliding team keeps its row ice alive at 500 ticks"
        );

        for _ in 0..320 {
            game.advance(InputFrame::default());
        }
        let team: Vec<&ZombieState> = game
            .state
            .board
            .zombies
            .iter()
            .filter(|zombie| zombie.zombie_type == ZombieType::Bobsled)
            .collect();
        assert_eq!(team.len(), 4, "the crash leaves all four zombies alive");
        assert!(
            team.iter().all(|zombie| !zombie.bobsled_sliding),
            "the team dismounts once the sled breaks past the ice end"
        );
        assert!(
            team.iter()
                .all(|zombie| !zombie.bobsled_leader || zombie.shield_health == 0),
            "the leader's 300 HP sled is consumed by the 6-per-tick overrun damage"
        );
    }

    #[test]
    fn magnetshroom_steals_the_nearest_bucket_and_recharges_1500_ticks() {
        let mut game = Game::new(7, SceneKind::Night);
        game.state.sun = 1_000;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 31 },
                InputAction::Plant { row: 2, column: 4 },
            ],
        });
        let mut setup = Vec::new();
        let bucket = game.spawn_buckethead_zombie(2, 0, Some(520 * POSITION_SCALE), &mut setup);
        let cone = game.spawn_conehead_zombie(2, 0, Some(440 * POSITION_SCALE), &mut setup);
        let second = game.spawn_buckethead_zombie(3, 0, Some(520 * POSITION_SCALE), &mut setup);
        let cone_health = game
            .state
            .board
            .zombies
            .iter()
            .find(|z| z.id == cone)
            .unwrap()
            .health;
        for zombie in &mut game.state.board.zombies {
            zombie.speed = 0;
        }

        let events = game.advance(InputFrame::default());
        assert!(
            events.iter().any(|event| matches!(
                event,
                GameEvent::MetalStolen { zombie: Some(z), .. } if *z == bucket
            )),
            "the nearest bucket loses its pail"
        );
        let health_of = |game: &Game, id: EntityId| {
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == id)
                .unwrap()
                .health
        };
        assert_eq!(health_of(&game, bucket), 270, "the 270 HP body survives");
        assert_eq!(
            health_of(&game, cone),
            cone_health,
            "traffic cones are not stealable"
        );
        assert!(health_of(&game, second) > 270, "one item per cycle");

        let mut extra_steals = 0;
        for _ in 0..1_502 {
            let events = game.advance(InputFrame::default());
            extra_steals += events
                .iter()
                .filter(|event| matches!(event, GameEvent::MetalStolen { .. }))
                .count();
        }
        assert_eq!(
            extra_steals, 1,
            "the next steal waits out the 1500-tick recharge"
        );
        assert_eq!(health_of(&game, second), 270);
    }

    #[test]
    fn magnetshroom_pickaxe_steal_surfaces_the_digger() {
        let mut game = Game::new(7, SceneKind::Night);
        game.state.sun = 1_000;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 31 },
                InputAction::Plant { row: 2, column: 4 },
            ],
        });
        let mut setup = Vec::new();
        let digger = game.spawn_digger_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);

        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::MetalStolen { zombie: Some(z), .. } if *z == digger
        )));
        assert!(
            events.iter().any(|event| matches!(
                event,
                GameEvent::DiggerSurfaced { entity } if *entity == digger
            )),
            "the robbed tunneler emits the surface anchor"
        );
        {
            let zombie = game
                .state
                .board
                .zombies
                .iter()
                .find(|z| z.id == digger)
                .unwrap();
            assert!(!zombie.digger_underground, "the robbed digger surfaces");
            assert!(
                (DIGGER_AXE_LOSS_SURFACE_TICKS - 1..=DIGGER_AXE_LOSS_SURFACE_TICKS)
                    .contains(&zombie.digger_counter),
                "the 200-tick pause plus 130-tick rise starts, got {}",
                zombie.digger_counter
            );
            assert_eq!(zombie.special_phase, 1);
            assert_eq!(zombie.speed, DIGGER_WALK_SPEED);
        }
        for _ in 0..=DIGGER_AXE_LOSS_SURFACE_TICKS {
            game.advance(InputFrame::default());
        }
        let zombie = game
            .state
            .board
            .zombies
            .iter()
            .find(|z| z.id == digger)
            .unwrap();
        assert_eq!(zombie.digger_counter, 0, "the digger finishes rising");
        assert_eq!(zombie.health, 370);
    }

    #[test]
    fn pool_rows_gate_spawns_and_tube_riders_keep_their_land_health() {
        let mut game = Game::new(7, SceneKind::Pool);
        assert!(!game.row_can_have_zombie_type(2, ZombieType::Football, 6));
        assert!(!game.row_can_have_zombie_type(2, ZombieType::DuckyTube, 6));
        assert!(game.row_can_have_zombie_type(2, ZombieType::Buckethead, 6));
        assert!(
            !game.row_can_have_zombie_type(2, ZombieType::Buckethead, 3),
            "the first five waves keep pool rows for swimmers"
        );
        assert!(game.row_can_have_zombie_type(2, ZombieType::Snorkel, 3));
        assert!(
            !game.row_can_have_zombie_type(0, ZombieType::Snorkel, 3),
            "pool-only swimmers never spawn on land rows"
        );

        let mut setup = Vec::new();
        let pool_bucket =
            game.spawn_buckethead_zombie(2, 6, Some(500 * POSITION_SCALE), &mut setup);
        let land_bucket =
            game.spawn_buckethead_zombie(0, 6, Some(500 * POSITION_SCALE), &mut setup);
        game.advance(InputFrame::default());
        let find = |game: &Game, id: EntityId| {
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == id)
                .cloned()
                .unwrap()
        };
        let pool_zombie = find(&game, pool_bucket);
        assert!(pool_zombie.in_pool, "pool-row walkers ride a ducky tube");
        assert_eq!(
            pool_zombie.health, 1_370,
            "the tube is a visual overlay; helm and body HP are unchanged"
        );
        assert!(!find(&game, land_bucket).in_pool);
    }

    #[test]
    fn pool_entry_emits_the_source_splash_variation_once() {
        let mut game = Game::new(7, SceneKind::Pool);
        let mut setup = Vec::new();
        let zombie = game.spawn_normal_zombie(2, 0, Some(679 * POSITION_SCALE), &mut setup);

        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieEnteredPool { entity, variant }
                if *entity == zombie && *variant < 2
        )));
        assert!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == zombie)
                .unwrap()
                .in_pool
        );

        let events = game.advance(InputFrame::default());
        assert!(
            !events
                .iter()
                .any(|event| matches!(event, GameEvent::ZombieEnteredPool { .. }))
        );
    }

    #[test]
    fn bungee_pair_delivers_a_zombie_and_departs_without_stealing() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let (carrier, carried) =
            game.spawn_bungee_drop(ZombieType::Buckethead, 2, 5, 0, &mut setup);
        let held_x = grid_x(5) - 15 * POSITION_SCALE;
        let find = |game: &Game, id: EntityId| {
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == id)
                .cloned()
        };

        game.advance(InputFrame::default());
        let held = find(&game, carried).unwrap();
        assert!(held.bungee_held, "the delivery rides down with the carrier");
        assert_eq!(held.position_x, held_x, "held zombies do not walk");

        let mut released_at = None;
        for tick in 0..450 {
            game.advance(InputFrame::default());
            if !find(&game, carried).unwrap().bungee_held {
                released_at = Some(tick);
                break;
            }
        }
        let released_at = released_at.expect("the carrier releases its zombie");
        assert!(
            (360..=400).contains(&released_at),
            "the 3000-3150 altitude dive at 8 per tick lands near tick 375-394, got {released_at}"
        );
        assert_eq!(
            find(&game, carried).unwrap().health,
            1_370,
            "the delivered zombie lands with its full profile"
        );

        let mut carrier_gone = false;
        for _ in 0..80 {
            let events = game.advance(InputFrame::default());
            if events.iter().any(
                |event| matches!(event, GameEvent::ZombieDied { entity } if *entity == carrier),
            ) {
                carrier_gone = true;
                break;
            }
        }
        assert!(carrier_gone, "the carrier rises and departs after 75 ticks");
        assert!(find(&game, carrier).is_none());
        let walk_start = find(&game, carried).unwrap().position_x;
        game.advance(InputFrame::default());
        game.advance(InputFrame::default());
        game.advance(InputFrame::default());
        game.advance(InputFrame::default());
        assert!(
            find(&game, carried).unwrap().position_x < walk_start,
            "the released zombie walks like a normal spawn"
        );
    }

    #[test]
    fn roof_final_wave_schedules_a_three_pair_sky_drop() {
        let mut game = Game::new(7, SceneKind::Roof);
        let total = game.state.board.wave.total;
        game.state.board.wave.current = total - 1;
        game.state.board.wave.countdown = 1;
        game.advance(InputFrame::default());
        assert!(
            (SKY_DROP_DELAY_TICKS - 1..=SKY_DROP_DELAY_TICKS)
                .contains(&game.state.board.sky_drop_countdown),
            "the final roof wave schedules the 210-tick sky drop"
        );

        for _ in 0..SKY_DROP_DELAY_TICKS {
            game.advance(InputFrame::default());
        }
        let carriers = game
            .state
            .board
            .zombies
            .iter()
            .filter(|z| z.zombie_type == ZombieType::Bungee)
            .count();
        assert_eq!(carriers, 3, "three carrier bungees dive in");
        let held: Vec<&ZombieState> = game
            .state
            .board
            .zombies
            .iter()
            .filter(|z| z.bungee_held)
            .collect();
        assert_eq!(held.len(), 3);
        for zombie in held {
            assert!(matches!(
                zombie.zombie_type,
                ZombieType::Normal | ZombieType::Conehead | ZombieType::Buckethead
            ));
            assert!(zombie.row <= 4, "drops land on rows 0-4");
            assert!(
                zombie.position_x >= grid_x(4) - 15 * POSITION_SCALE
                    && zombie.position_x <= grid_x(8) - 15 * POSITION_SCALE,
                "drops land on columns 4-8"
            );
        }
    }

    #[test]
    fn boss_head_spits_a_rolling_ball_on_schedule() {
        let mut game = Game::new_mode(3, ModeKind::MiniGame, 19);
        let boss_index = game
            .state
            .board
            .zombies
            .iter()
            .position(|zombie| zombie.zombie_type == ZombieType::Boss)
            .expect("final boss spawns with the level");
        assert_eq!(
            game.state.board.zombies[boss_index].boss_head_counter,
            BOSS_HEAD_COUNTER_INITIAL + BOSS_HEAD_SPIT_DELAY
        );
        game.state.board.zombies[boss_index].boss_head_counter = 1;
        let events = game.advance(InputFrame::default());
        let (row, fire) = events
            .iter()
            .find_map(|event| match event {
                GameEvent::BossAttackWindup { row, fire, .. } => Some((*row, *fire)),
                _ => None,
            })
            .expect("head spit emits a windup");
        let boss = &game.state.board.zombies[boss_index];
        assert!(boss.boss_ball_active);
        assert_eq!(boss.boss_ball_row, row);
        assert_eq!(boss.boss_ball_fire, fire);
        assert!(row < DAY_ROWS);
        assert_eq!(boss.boss_ball_x, BOSS_BALL_START_X);
        game.advance(InputFrame::default());
        assert!(game.state.board.zombies[boss_index].boss_ball_x < BOSS_BALL_START_X);

        // The ball squishes a mower it passes without firing it (the boss
        // level itself has none, so seed one).
        game.state.board.mowers.push(MowerState {
            row: 0,
            position_x: -80 * POSITION_SCALE,
            active: false,
            spent: false,
        });
        {
            let boss = &mut game.state.board.zombies[boss_index];
            boss.boss_ball_row = 0;
            boss.boss_ball_x = -85 * POSITION_SCALE;
        }
        game.advance(InputFrame::default());
        assert!(game.state.board.mowers.is_empty());

        // Off the lawn at x < -180 the ball ends.
        {
            let boss = &mut game.state.board.zombies[boss_index];
            boss.boss_ball_x = BOSS_BALL_END_X;
        }
        game.advance(InputFrame::default());
        assert!(!game.state.board.zombies[boss_index].boss_ball_active);
    }

    #[test]
    fn opposite_elements_destroy_the_boss_ball() {
        // FinalBoss conveyor: cabbage, jalapeno, cabbage, ice-shroom.
        let mut game = Game::new_mode(3, ModeKind::MiniGame, 19);
        {
            let boss = game
                .state
                .board
                .zombies
                .iter_mut()
                .find(|zombie| zombie.zombie_type == ZombieType::Boss)
                .expect("boss");
            boss.boss_ball_active = true;
            boss.boss_ball_fire = false;
            boss.boss_ball_row = 2;
            boss.boss_ball_x = BOSS_BALL_START_X;
        }
        game.state.sun = 500;
        let mut events = game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        for _ in 0..=INSTANT_PLANT_COUNTDOWN {
            events.extend(game.advance(InputFrame::default()));
        }
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::BossProjectileDestroyed { fire: false, .. }
        )));

        let mut game = Game::new_mode(3, ModeKind::MiniGame, 19);
        {
            let boss = game
                .state
                .board
                .zombies
                .iter_mut()
                .find(|zombie| zombie.zombie_type == ZombieType::Boss)
                .expect("boss");
            boss.boss_ball_active = true;
            boss.boss_ball_fire = true;
            boss.boss_ball_row = 4;
            boss.boss_ball_x = BOSS_BALL_START_X;
        }
        game.state.sun = 500;
        let mut events = game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 3 },
                InputAction::Plant { row: 0, column: 0 },
            ],
        });
        for _ in 0..=INSTANT_PLANT_COUNTDOWN {
            events.extend(game.advance(InputFrame::default()));
        }
        assert!(
            events.iter().any(|event| matches!(
                event,
                GameEvent::BossProjectileDestroyed { fire: true, .. }
            ))
        );
    }

    #[test]
    fn adventure_level_35_places_the_stage_zero_scary_pots() {
        let game = Game::new_mode(9, ModeKind::Adventure, 35);
        let vases = &game.state().board.vases;
        assert_eq!(vases.len(), 15);
        assert!(vases.iter().all(|vase| vase.column >= 6));
        assert!(vases.iter().all(|vase| !vase.leaf));
        let count = |contents: VaseContents| {
            vases
                .iter()
                .filter(|vase| vase.contents == contents)
                .count()
        };
        assert_eq!(count(VaseContents::Plant(PlantType::Peashooter)), 5);
        assert_eq!(count(VaseContents::Plant(PlantType::Other(17))), 5);
        assert_eq!(count(VaseContents::Zombie(ZombieType::Normal)), 4);
        assert_eq!(count(VaseContents::Zombie(ZombieType::Buckethead)), 1);
    }

    #[test]
    fn adventure_level_35_advances_through_three_scary_pot_stages() {
        let mut game = Game::new_mode(9, ModeKind::Adventure, 35);
        game.state.board.vases.clear();
        game.advance(InputFrame::default());
        assert_eq!(game.state.board.scary_pot_stage, 1);
        assert_eq!(game.state.board.vases.len(), 20);
        assert!(game.state.board.vases.iter().all(|vase| vase.column >= 5));
        assert_eq!(
            game.state
                .board
                .vases
                .iter()
                .filter(|vase| vase.leaf)
                .count(),
            2
        );
        game.state.board.vases.clear();
        game.advance(InputFrame::default());
        assert_eq!(game.state.board.scary_pot_stage, 2);
        assert_eq!(game.state.board.vases.len(), 25);
        assert!(game.state.board.vases.iter().all(|vase| vase.column >= 4));
        game.state.board.vases.clear();
        let events = game.advance(InputFrame::default());
        assert!(events.contains(&GameEvent::GameWon));
        // The wave clock stays parked the whole time.
        assert_eq!(game.state.board.wave.current, 0);
    }

    #[test]
    fn adventure_first_run_sod_rows_gate_the_lawn() {
        // 1-1: only row 2 is sodded (Board.cpp:1021-1029).
        let mut game = Game::new_mode(7, ModeKind::Adventure, 1);
        game.state.sun = 500;
        assert_eq!(
            game.state
                .board
                .mowers
                .iter()
                .map(|mower| mower.row)
                .collect::<Vec<_>>(),
            vec![2]
        );
        let events = game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 0, column: 1 },
            ],
        });
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::InputRejected {
                reason: InputRejectReason::InvalidTerrain,
                ..
            }
        )));
        assert!(game.state.board.plants.is_empty());
        let events = game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 2, column: 1 },
            ],
        });
        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::PlantPlaced { .. }))
        );
        assert!(!game.row_can_have_zombie_type(0, ZombieType::Normal, 6));
        assert!(game.row_can_have_zombie_type(2, ZombieType::Normal, 6));

        // 1-2/1-3: rows 1-3 (Board.cpp:1030-1034).
        let game = Game::new_mode(7, ModeKind::Adventure, 2);
        assert_eq!(
            game.state
                .board
                .mowers
                .iter()
                .map(|mower| mower.row)
                .collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
        assert!(!game.row_can_have_zombie_type(4, ZombieType::Normal, 6));
    }

    #[test]
    fn adventure_level_table_matches_the_source() {
        // Scene routing, including the level-35 night override.
        assert_eq!(adventure_level_scene(1), SceneKind::Day);
        assert_eq!(adventure_level_scene(11), SceneKind::Night);
        assert_eq!(adventure_level_scene(21), SceneKind::Pool);
        assert_eq!(adventure_level_scene(31), SceneKind::Fog);
        assert_eq!(adventure_level_scene(35), SceneKind::Night);
        assert_eq!(adventure_level_scene(40), SceneKind::Fog);
        assert_eq!(adventure_level_scene(41), SceneKind::Roof);
        assert_eq!(adventure_level_scene(50), SceneKind::Boss);

        // gZombieWaves identity plus the Whack-a-Zombie override.
        let first: Vec<u32> = (1..=50).map(|l| adventure_wave_count(l, false)).collect();
        assert_eq!(
            first,
            vec![
                4, 6, 8, 10, 8, 10, 20, 10, 20, 20, 10, 20, 10, 20, 8, 10, 20, 10, 20, 20, 10, 20,
                20, 30, 20, 20, 30, 20, 30, 30, 10, 20, 10, 20, 20, 10, 20, 10, 20, 20, 10, 20, 20,
                30, 20, 20, 30, 20, 30, 30,
            ]
        );

        // Replay bumps every non-mini-boss level; Whack-a-Zombie stays at 8.
        assert_eq!(adventure_wave_count(1, true), 20);
        assert_eq!(adventure_wave_count(7, true), 30);
        assert_eq!(adventure_wave_count(10, true), 20);
        assert_eq!(adventure_wave_count(15, true), 8);
        assert_eq!(adventure_wave_count(24, true), 40);
        assert_eq!(adventure_wave_count(30, true), 30);

        // Flag identity: first-run level 1 has none; short levels flag once;
        // replay always flags every ten waves (level 15 replay has zero).
        assert_eq!(adventure_flag_wave_count(1, false), 0);
        assert_eq!(adventure_flag_wave_count(2, false), 1);
        assert_eq!(adventure_flag_wave_count(7, false), 2);
        assert_eq!(adventure_flag_wave_count(15, true), 0);
        assert_eq!(adventure_flag_wave_count(24, false), 3);
        assert_eq!(adventure_flag_wave_count(50, false), 3);

        // Conveyor-belt levels.
        let conveyors: Vec<u8> = (1..=50)
            .filter(|l| adventure_level_is_conveyor(*l))
            .collect();
        assert_eq!(conveyors, vec![5, 10, 20, 25, 30, 40, 45, 50]);

        // Mode wiring uses the table.
        let game = Game::new_mode(7, ModeKind::Adventure, 35);
        assert_eq!(game.state().scene, SceneKind::Night);
        assert_eq!(game.state().board.wave.total, 20);
        let roof = Game::new_mode(7, ModeKind::Adventure, 41);
        assert_eq!(roof.state().scene, SceneKind::Roof);
        assert_eq!(roof.state().board.wave.total, 10);
    }

    #[test]
    fn adventure_wave_stats_and_allow_lists_match_the_source() {
        assert_eq!(zombie_wave_stats(ZombieType::Normal), (1, 1, 1, 4_000));
        assert_eq!(zombie_wave_stats(ZombieType::ScreenDoor), (4, 13, 5, 3_500));
        assert_eq!(
            zombie_wave_stats(ZombieType::Gargantuar),
            (10, 48, 15, 1_500)
        );
        assert_eq!(zombie_wave_stats(ZombieType::Yeti), (4, 40, 1, 1));
        assert_eq!(
            zombie_wave_stats(ZombieType::Flag).3,
            0,
            "flags are never picked"
        );
        assert_eq!(zombie_wave_stats(ZombieType::DuckyTube).3, 0);

        assert!(adventure_zombie_allowed(ZombieType::Normal, 1));
        assert!(adventure_zombie_allowed(ZombieType::Conehead, 12));
        assert!(
            !adventure_zombie_allowed(ZombieType::Conehead, 11),
            "cones sit out the newspaper introduction level"
        );
        assert!(adventure_zombie_allowed(ZombieType::PoleVaulter, 42));
        assert!(!adventure_zombie_allowed(ZombieType::PoleVaulter, 8));
        assert!(adventure_zombie_allowed(ZombieType::Buckethead, 50));
        assert!(!adventure_zombie_allowed(ZombieType::Dancer, 21));
        assert!(
            !adventure_zombie_allowed(ZombieType::Yeti, 40),
            "yetis spawn only through the intro path"
        );
        assert!(!adventure_zombie_allowed(ZombieType::Gigagargantuar, 50));
        assert!(!adventure_zombie_allowed(ZombieType::Boss, 50));

        let intros: Vec<(u8, ZombieType)> = (1..=50)
            .filter_map(|l| adventure_introduced_zombie(l).map(|z| (l, z)))
            .collect();
        assert_eq!(intros.len(), 19);
        assert_eq!(intros[0], (3, ZombieType::Conehead));
        assert_eq!(intros[6], (18, ZombieType::Dancer));
        assert_eq!(intros[18], (50, ZombieType::Boss));
        assert!(
            adventure_introduced_zombie(21).is_none(),
            "the ducky tube introduction is preview-only"
        );
    }

    #[test]
    fn adventure_wave_composition_follows_the_source_rules() {
        let mut game = Game::new(7, SceneKind::Day);

        // First-run level 1: four all-normal waves, no flags.
        let waves = game.pick_adventure_waves(1, false);
        assert_eq!(waves.len(), 4);
        assert_eq!(waves[0], vec![ZombieType::Normal]);
        assert!(
            waves.iter().flatten().all(|z| *z == ZombieType::Normal),
            "level 1 only knows normal zombies"
        );

        // First-run level 3: intro Conehead at the half-way wave and the
        // final wave; the final short-level wave is also the flag wave.
        let waves = game.pick_adventure_waves(3, false);
        assert_eq!(waves.len(), 8);
        assert!(waves[4].contains(&ZombieType::Conehead));
        assert!(waves[7].contains(&ZombieType::Flag));
        assert!(waves[7].contains(&ZombieType::Conehead));
        assert!(
            waves.iter().flatten().all(|z| matches!(
                z,
                ZombieType::Normal | ZombieType::Conehead | ZombieType::Flag
            )),
            "level 3 draws only from its allow-list"
        );

        // Bungee Blitz flag waves are exactly five bungees.
        let waves = game.pick_adventure_waves(45, false);
        assert_eq!(waves[9], vec![ZombieType::Bungee; 5]);

        // The level-50 final wave carries the extra Gargantuar and one of
        // every level-legal missing type.
        let waves = game.pick_adventure_waves(50, false);
        let last = waves.last().unwrap();
        for required in [
            ZombieType::Gargantuar,
            ZombieType::Buckethead,
            ZombieType::Jackbox,
            ZombieType::Bungee,
            ZombieType::Ladder,
            ZombieType::Catapult,
        ] {
            assert!(last.contains(&required), "final wave misses {required:?}");
        }

        // Replay waves are larger than first-run waves at the same level.
        let first_last = game.pick_adventure_waves(12, false).len();
        let replay_last = game.pick_adventure_waves(12, true).len();
        assert_eq!(first_last, 20);
        assert_eq!(replay_last, 30);
    }

    #[test]
    fn adventure_runtime_spawns_the_composed_waves() {
        let mut game = Game::new_mode(7, ModeKind::Adventure, 3);
        assert_eq!(game.state().board.wave.total, 8);
        for wave in 0..8u32 {
            game.state.board.wave.countdown = 1;
            let events = game.advance(InputFrame::default());
            let spawned: Vec<ZombieType> = events
                .iter()
                .filter_map(|event| match event {
                    GameEvent::ZombieSpawned {
                        zombie_type,
                        wave: w,
                        ..
                    } if *w == wave => Some(*zombie_type),
                    _ => None,
                })
                .collect();
            assert!(!spawned.is_empty(), "wave {wave} spawns its plan");
            assert!(
                spawned.iter().all(|z| matches!(
                    z,
                    ZombieType::Normal | ZombieType::Conehead | ZombieType::Flag
                )),
                "level 3 spawns only allow-listed types, got {spawned:?}"
            );
            assert_eq!(
                spawned, game.state.board.wave_plan[wave as usize],
                "wave {wave} matches the composed plan"
            );
            game.state.board.zombies.clear();
        }
    }

    #[test]
    fn adventure_waves_rearm_at_the_source_countdown() {
        let mut game = Game::new_mode(7, ModeKind::Adventure, 6);
        game.state.board.wave.countdown = 1;
        game.advance(InputFrame::default());
        assert_eq!(game.state.board.wave.current, 1);
        let countdown = game.state.board.wave.countdown;
        assert!(
            (ZOMBIE_NEXT_WAVE_COUNTDOWN..=ZOMBIE_NEXT_WAVE_COUNTDOWN + ZOMBIE_NEXT_WAVE_RANGE)
                .contains(&countdown),
            "next wave arms at 2500 + Rand(600), got {countdown}"
        );
        assert_eq!(game.state.board.wave.countdown_start, countdown);
    }

    #[test]
    fn night_graves_and_pool_cells_rise_on_the_final_wave() {
        let mut game = Game::new_mode(7, ModeKind::Adventure, 11);
        let grave_count = game.state.board.graves.len();
        assert_eq!(grave_count, 4, "level 11 boards own four graves");
        let total = game.state.board.wave.total;
        game.state.board.wave.current = total - 1;
        game.state.board.wave.countdown = 1;
        game.advance(InputFrame::default());
        assert!(game.state.board.sky_drop_countdown > 0);
        let before: Vec<EntityId> = game.state.board.zombies.iter().map(|z| z.id).collect();
        for _ in 0..SKY_DROP_DELAY_TICKS {
            game.advance(InputFrame::default());
        }
        let risers: Vec<&ZombieState> = game
            .state
            .board
            .zombies
            .iter()
            .filter(|z| !before.contains(&z.id))
            .collect();
        assert_eq!(risers.len(), grave_count, "one riser per gravestone");
        assert!(
            risers
                .iter()
                .all(|z| matches!(z.zombie_type, ZombieType::Normal | ZombieType::Conehead))
        );

        let mut pool = Game::new_mode(7, ModeKind::Adventure, 21);
        let total = pool.state.board.wave.total;
        pool.state.board.wave.current = total - 1;
        pool.state.board.wave.countdown = 1;
        pool.advance(InputFrame::default());
        let before: Vec<EntityId> = pool.state.board.zombies.iter().map(|z| z.id).collect();
        for _ in 0..SKY_DROP_DELAY_TICKS {
            pool.advance(InputFrame::default());
        }
        let emerged: Vec<&ZombieState> = pool
            .state
            .board
            .zombies
            .iter()
            .filter(|z| !before.contains(&z.id))
            .collect();
        assert_eq!(emerged.len(), 2, "levels 21-22 emerge two zombies");
        assert!(
            emerged
                .iter()
                .all(|z| matches!(z.row, 2 | 3) && z.position_x < grid_x(8)),
            "pool emerges use the source cell block"
        );
    }

    #[test]
    fn adventure_night_boards_place_source_graves_and_sun() {
        let game = Game::new_mode(7, ModeKind::Adventure, 11);
        assert_eq!(game.state().board.graves.len(), 4);
        assert!(
            game.state()
                .board
                .graves
                .iter()
                .all(|grave| (6..=8).contains(&grave.column)),
            "level 11 graves sit in columns 6-8"
        );
        assert_eq!(
            Game::new_mode(7, ModeKind::Adventure, 15)
                .state()
                .board
                .graves
                .len(),
            9
        );
        assert_eq!(
            Game::new_mode(7, ModeKind::Adventure, 20)
                .state()
                .board
                .graves
                .len(),
            13
        );
        assert!(
            Game::new_mode(7, ModeKind::Adventure, 21)
                .state()
                .board
                .graves
                .is_empty()
        );

        assert_eq!(Game::new_mode(7, ModeKind::Adventure, 1).state().sun, 150);
        assert_eq!(Game::new_mode(7, ModeKind::Adventure, 2).state().sun, 50);
        assert_eq!(Game::new_mode(7, ModeKind::Adventure, 15).state().sun, 0);
        assert_eq!(Game::new_mode(7, ModeKind::Adventure, 35).state().sun, 0);
    }

    #[test]
    fn dropped_coins_arc_and_settle() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut events = Vec::new();
        game.spawn_coin(
            CoinType::Silver,
            300 * POSITION_SCALE,
            280 * POSITION_SCALE,
            &mut events,
        );
        let coin = game.state.board.coins[0].clone();
        assert!(coin.velocity_y <= -1_700_000, "coins launch upward");
        let target = coin.target_y.expect("dropped coins carry a ground stop");

        for _ in 0..200 {
            game.advance(InputFrame::default());
            if game.state.board.coins[0].target_y.is_none() {
                break;
            }
        }
        let landed = &game.state.board.coins[0];
        assert_eq!(landed.position_y, target, "the coin settles at its stop");
        assert_eq!(landed.velocity_y, 0);
    }

    #[test]
    fn plant_suns_arc_up_and_land_near_the_plant() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 100;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let find_plant_sun = |game: &Game| {
            game.state
                .board
                .suns
                .iter()
                .find(|sun| matches!(sun.source, SunSource::Plant(_)))
                .cloned()
        };
        for _ in 0..3_000 {
            game.advance(InputFrame::default());
            if find_plant_sun(&game).is_some() {
                break;
            }
        }
        let sun = find_plant_sun(&game).expect("the sunflower produces a sun");
        // Gravity has already ticked between spawn and observation; the exact
        // launch band is enforced by the spawn formula itself.
        assert!(sun.velocity_y < 0, "the plant sun is still arcing upward");
        let target = sun.target_y.expect("plant suns carry a ground stop");

        let mut ticks = 0;
        while find_plant_sun(&game).is_some_and(|s| s.target_y.is_some()) && ticks < 400 {
            game.advance(InputFrame::default());
            ticks += 1;
        }
        let landed = find_plant_sun(&game).unwrap();
        assert_eq!(landed.position_y, target);
        assert_eq!(landed.velocity_y, 0, "landing clears the arc velocities");
    }

    #[test]
    fn sky_suns_fall_to_their_ground_stop() {
        let mut game = Game::new(7, SceneKind::Day);
        for _ in 0..1_000 {
            game.advance(InputFrame::default());
            if !game.state.board.suns.is_empty() {
                break;
            }
        }
        let sun = game.state.board.suns[0].clone();
        assert_eq!(sun.position_y, 60 * POSITION_SCALE + SUN_FALL_SPEED);
        let target = sun.target_y.expect("sky suns carry a ground stop");
        assert!((300..=549).contains(&(target / POSITION_SCALE)));

        let mut ticks = 0;
        while game.state.board.suns[0].target_y.is_some() && ticks < 1_200 {
            game.advance(InputFrame::default());
            ticks += 1;
        }
        assert_eq!(
            game.state.board.suns[0].position_y, target,
            "the sun stops exactly at its ground y"
        );
    }

    #[test]
    fn umbrella_leaf_bounces_the_bungee_steal() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 1_000;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 37 },
                InputAction::Plant { row: 2, column: 3 },
            ],
        });
        let mut setup = Vec::new();
        let bungee = game.spawn_bungee_zombie(2, 0, None, &mut setup);
        game.state
            .board
            .zombies
            .iter_mut()
            .for_each(|z| z.bungee_counter = 1);

        let events = game.advance(InputFrame::default());
        assert!(
            events.iter().any(
                |e| matches!(e, GameEvent::UmbrellaDeflected { zombie, .. } if *zombie == bungee)
            ),
            "the umbrella bounces the grab"
        );
        assert_eq!(game.state.board.plants.len(), 2, "no plant is stolen");
        assert!(
            !game.state.board.zombies.iter().any(|z| z.id == bungee),
            "the bounced bungee still departs"
        );
    }

    #[test]
    fn butter_immobilizes_for_400_ticks() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_normal_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        let mut events = Vec::new();
        game.apply_projectile_chill(zombie, ProjectileType::Butter, &mut events);
        assert!(events.iter().any(|e| matches!(
            e,
            GameEvent::ZombieButtered { entity } if *entity == zombie
        )));
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .frozen_counter,
            BUTTER_TICKS
        );

        let zamboni = game.spawn_zamboni_zombie(3, 0, Some(500 * POSITION_SCALE), &mut setup);
        let mut events = Vec::new();
        game.apply_projectile_chill(zamboni, ProjectileType::Butter, &mut events);
        assert!(
            events.is_empty(),
            "the Zamboni refuses butter per ApplyButter"
        );
    }

    #[test]
    fn portals_teleport_zombies_between_cells() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        game.place_portal(1, 4, false, &mut setup);
        game.place_portal(3, 7, true, &mut setup);
        assert_eq!(
            setup
                .iter()
                .filter(|e| matches!(e, GameEvent::PortalOpened { .. }))
                .count(),
            2
        );
        let zombie =
            game.spawn_normal_zombie(1, 0, Some(grid_x(4) + 15 * POSITION_SCALE), &mut setup);

        let mut teleported = false;
        for _ in 0..120 {
            let events = game.advance(InputFrame::default());
            if events.iter().any(|e| {
                matches!(
                    e,
                    GameEvent::ZombieTeleported { entity, row: 3, column: 7 } if *entity == zombie
                )
            }) {
                teleported = true;
                break;
            }
        }
        assert!(teleported, "the zombie rides the portal pair");
        let state = game
            .state
            .board
            .zombies
            .iter()
            .find(|z| z.id == zombie)
            .unwrap();
        assert_eq!(state.row, 3);
        assert!(
            state.portal_cooldown > 0,
            "teleports carry a re-entry cooldown"
        );
    }

    #[test]
    fn armor_and_shield_loss_emit_their_drop_anchors() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let bucket = game.spawn_buckethead_zombie(2, 0, Some(600 * POSITION_SCALE), &mut setup);
        let door = game.spawn_screen_door_zombie(3, 0, Some(600 * POSITION_SCALE), &mut setup);
        for zombie in &mut game.state.board.zombies {
            zombie.speed = 0;
        }
        game.state.board.zombies.iter_mut().for_each(|z| {
            if z.id == bucket {
                z.health = 270;
            }
            if z.id == door {
                z.shield_health = 0;
            }
        });

        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|e| matches!(
            e,
            GameEvent::ZombieArmorLost { entity } if *entity == bucket
        )));
        assert!(events.iter().any(|e| matches!(
            e,
            GameEvent::ZombieShieldLost { entity } if *entity == door
        )));
        let events = game.advance(InputFrame::default());
        assert!(
            !events.iter().any(|e| matches!(
                e,
                GameEvent::ZombieArmorLost { .. } | GameEvent::ZombieShieldLost { .. }
            )),
            "the drop anchors fire exactly once"
        );
    }

    #[test]
    fn newspaper_paper_loss_emits_rip_anchor_once() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let newspaper = game.spawn_newspaper_zombie(2, 0, Some(600 * POSITION_SCALE), &mut setup);
        let newspaper_index = game
            .state
            .board
            .zombies
            .iter()
            .position(|zombie| zombie.id == newspaper)
            .unwrap();
        game.state.board.zombies[newspaper_index].speed = 0;
        game.state.board.zombies[newspaper_index].shield_health = 20;

        let mut events = Vec::new();
        game.damage_zombie(newspaper_index, 20, &mut events);
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieNewspaperRipped { entity } if *entity == newspaper
        )));

        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieShieldLost { entity } if *entity == newspaper
        )));

        let events = game.advance(InputFrame::default());
        assert!(
            !events
                .iter()
                .any(|event| matches!(event, GameEvent::ZombieNewspaperRipped { .. }))
        );
    }

    #[test]
    fn debug_newspaper_checkpoint_emits_rip_audio_event() {
        let mut game = Game::new(7, SceneKind::Day);
        let events = game.debug_prepare_newspaper_rip();
        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::ZombieNewspaperRipped { .. }))
        );
    }

    #[test]
    fn tallnut_blocks_jumpers_and_breaks_the_pogo_stick() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 1_000;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 23 },
                InputAction::Plant { row: 2, column: 5 },
            ],
        });
        let mut setup = Vec::new();
        let pogo = game.spawn_pogo_zombie(2, 0, Some(grid_x(5) + 40 * POSITION_SCALE), &mut setup);

        let mut blocked = false;
        let mut stick_lost = false;
        for _ in 0..300 {
            let events = game.advance(InputFrame::default());
            for event in &events {
                if matches!(event, GameEvent::JumpBlocked { zombie, .. } if *zombie == pogo) {
                    blocked = true;
                }
                if matches!(event, GameEvent::PogoStickLost { entity } if *entity == pogo) {
                    stick_lost = true;
                }
            }
            if blocked && stick_lost {
                break;
            }
        }
        assert!(blocked, "the tallnut blocks the pogo bounce");
        assert!(stick_lost, "the blocked pogo loses its stick");
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == pogo)
                .unwrap()
                .special_phase,
            1
        );
        assert!(
            !game.state.board.plants.is_empty(),
            "the tallnut survives the block"
        );
    }

    #[test]
    fn potato_mines_emit_the_rise_event_when_armed() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 1_000;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 4 },
                InputAction::Plant { row: 2, column: 3 },
            ],
        });
        game.state.board.plants[0].special_counter = 2;
        let mut armed_ticks = 0;
        for _ in 0..4 {
            let events = game.advance(InputFrame::default());
            armed_ticks += events
                .iter()
                .filter(|e| matches!(e, GameEvent::PotatoMineArmed { .. }))
                .count();
        }
        assert_eq!(armed_ticks, 1, "the rise event fires exactly once");
        assert!(game.state.board.plants[0].special_armed);
    }

    #[test]
    fn thawing_zombies_emit_the_ice_trap_release_event() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_normal_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        game.state
            .board
            .zombies
            .iter_mut()
            .for_each(|z| z.frozen_counter = 3);

        let mut thaw_ticks = Vec::new();
        for tick in 0..5 {
            let events = game.advance(InputFrame::default());
            if events.iter().any(|e| {
                matches!(
                    e,
                    GameEvent::ZombieThawed { entity } if *entity == zombie
                )
            }) {
                thaw_ticks.push(tick);
            }
        }
        assert_eq!(thaw_ticks.len(), 1, "the thaw event fires exactly once");
    }

    #[test]
    fn rake_kills_the_first_zombie_and_is_consumed() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.board.rake = Some((2, 5));
        let mut setup = Vec::new();
        let first =
            game.spawn_normal_zombie(2, 0, Some(grid_x(5) + 20 * POSITION_SCALE), &mut setup);
        let mut raked = false;
        for _ in 0..200 {
            let events = game.advance(InputFrame::default());
            if events.iter().any(|e| {
                matches!(
                    e,
                    GameEvent::RakeTriggered { zombie } if *zombie == first
                )
            }) {
                raked = true;
                break;
            }
        }
        assert!(raked, "the first zombie steps on the rake and dies");
        assert!(game.state.board.rake.is_none(), "the rake is single-use");
        assert!(game.state.board.zombies.iter().all(|z| z.id != first));

        let second =
            game.spawn_normal_zombie(2, 0, Some(grid_x(5) + 20 * POSITION_SCALE), &mut setup);
        for _ in 0..120 {
            game.advance(InputFrame::default());
        }
        assert!(
            game.state.board.zombies.iter().any(|z| z.id == second),
            "later zombies walk past the spent rake"
        );
    }

    #[test]
    fn adventure_completion_drops_the_level_award() {
        let mut game = Game::new_mode(7, ModeKind::Adventure, 1);
        game.state.board.wave.current = game.state.board.wave.total;
        game.state.board.zombies.clear();
        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|e| matches!(e, GameEvent::GameWon)));
        assert!(
            game.state
                .board
                .coins
                .iter()
                .any(|coin| coin.coin_type == CoinType::FinalSeedPacket),
            "finishing an adventure level drops its award packet"
        );
    }

    #[test]
    fn adventure_setup_identities_match_the_source() {
        assert_eq!(adventure_starting_sun(1, true), 150);
        assert_eq!(adventure_starting_sun(1, false), 50);
        assert_eq!(adventure_starting_sun(15, true), 0);
        assert_eq!(adventure_starting_sun(35, false), 0);
        assert_eq!(adventure_starting_sun(41, true), 50);

        assert_eq!(adventure_seed_slots(1, true, 0), 1);
        assert_eq!(adventure_seed_slots(4, true, 0), 4);
        assert_eq!(
            adventure_seed_slots(5, true, 0),
            10,
            "bowling runs the conveyor"
        );
        assert_eq!(adventure_seed_slots(6, true, 0), 5);
        assert_eq!(adventure_seed_slots(7, true, 0), 6);
        assert_eq!(adventure_seed_slots(8, true, 0), 6);
        assert_eq!(adventure_seed_slots(15, true, 0), 3);
        assert_eq!(adventure_seed_slots(35, false, 4), 1);
        assert_eq!(adventure_seed_slots(22, false, 4), 10);
        assert_eq!(adventure_seed_slots(22, false, 2), 8);

        assert!(!adventure_uses_seed_chooser(7, true));
        assert!(adventure_uses_seed_chooser(8, true));
        assert!(adventure_uses_seed_chooser(3, false));
        assert!(
            !adventure_uses_seed_chooser(10, false),
            "conveyor levels skip the chooser"
        );
        assert!(!adventure_uses_seed_chooser(35, false));

        let counts: Vec<u8> = (11..=20).map(adventure_grave_count).collect();
        assert_eq!(counts, vec![4, 4, 4, 7, 9, 7, 11, 11, 11, 13]);
        assert_eq!(adventure_grave_count(21), 0);
    }

    #[test]
    fn adventure_awards_match_the_source_formula() {
        // GetAwardSeedForLevel identities from the decomp table.
        assert_eq!(adventure_award(1), AdventureAward::Plant(1)); // Sunflower
        assert_eq!(adventure_award(3), AdventureAward::Plant(3)); // Wall-nut
        assert_eq!(adventure_award(5), AdventureAward::Plant(4)); // Potato Mine
        assert_eq!(adventure_award(10), AdventureAward::Plant(8)); // Puff-shroom
        assert_eq!(adventure_award(20), AdventureAward::Plant(16)); // Lily Pad
        assert_eq!(adventure_award(38), AdventureAward::Plant(31)); // Magnet-shroom
        assert_eq!(adventure_award(48), AdventureAward::Plant(39)); // Melon-pult
        assert_eq!(adventure_award(4), AdventureAward::Shovel);
        assert_eq!(adventure_award(14), AdventureAward::Almanac);
        assert_eq!(adventure_award(24), AdventureAward::CarKeys);
        assert_eq!(adventure_award(34), AdventureAward::Taco);
        assert_eq!(adventure_award(44), AdventureAward::WateringCan);
        assert_eq!(adventure_award(49), AdventureAward::Note);
        assert_eq!(adventure_award(50), AdventureAward::Trophy);
        assert_eq!(adventure_unlocks(14), (false, false, false));
        assert_eq!(adventure_unlocks(15), (true, false, false));
        assert_eq!(adventure_unlocks(25), (true, true, false));
        assert_eq!(adventure_unlocks(45), (true, true, true));
    }

    #[test]
    fn adventure_flag_waves_pause_and_cleared_waves_release_early() {
        // Early advance: an old-enough countdown snaps to 200 once the last
        // wave's health falls below the 50-65% threshold.
        let mut game = Game::new_mode(7, ModeKind::Adventure, 6);
        game.state.board.wave.countdown = 1;
        game.advance(InputFrame::default());
        game.state.board.zombies.clear();
        game.state.board.wave.countdown_start = 3_000;
        game.state.board.wave.countdown = 2_599;
        game.advance(InputFrame::default());
        assert_eq!(
            game.state.board.wave.countdown, 200,
            "a cleared wave releases the next one early"
        );

        // Huge-wave pause: the flag wave freezes at countdown 5 for 750
        // ticks, then spawns immediately.
        let mut game = Game::new_mode(7, ModeKind::Adventure, 6);
        game.state.board.wave.current = 9;
        game.state.board.wave.countdown = 6;
        game.state.board.wave_plan = vec![vec![ZombieType::Normal]; 10];
        game.advance(InputFrame::default());
        assert_eq!(game.state.board.huge_wave_countdown, 750);
        assert_eq!(game.state.board.wave.current, 9, "the wave waits");
        for _ in 0..749 {
            game.advance(InputFrame::default());
        }
        assert_eq!(game.state.board.wave.current, 9);
        game.advance(InputFrame::default());
        assert_eq!(
            game.state.board.wave.current, 10,
            "the wave releases the tick the pause expires"
        );
    }

    #[test]
    fn jackbox_zombie_explodes_after_timer_and_damages_nearby_plants() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_jackbox_zombie(2, 0, Some(780 * POSITION_SCALE), &mut setup);
        let jack = game
            .state
            .board
            .zombies
            .iter_mut()
            .find(|jack| jack.id == zombie)
            .unwrap();
        jack.jackbox_timer = 1;
        jack.speed = 0;

        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .zombie_type,
            ZombieType::Jackbox
        );

        // Advance enough ticks to trigger the random timer (500-1500).
        let mut events = Vec::new();
        for _ in 0..2000 {
            events.extend(game.advance(InputFrame::default()));
            if !game.state.board.zombies.iter().any(|z| z.id == zombie) {
                break;
            }
        }

        // Jackbox zombie should be dead after detonation.
        assert!(
            !game.state.board.zombies.iter().any(|z| z.id == zombie),
            "Jackbox zombie should die in its own explosion"
        );
        // Verify ZombieDied event was emitted.
        let died: Vec<_> = events
            .iter()
            .filter(|e| matches!(e, GameEvent::ZombieDied { entity: eid } if *eid == zombie))
            .collect();
        assert!(!died.is_empty(), "ZombieDied should be emitted for Jackbox");
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::JackboxExploded { entity, row: 2, .. } if *entity == zombie
        )));
    }

    #[test]
    fn vase_jackbox_pops_quickly_and_a_damage_killed_jack_does_not_explode() {
        // Zombie_Init forces the Scary Potter run counter to 10, so vase Jacks
        // pop after ~120 updates; Zombie_UpdateJack never detonates on death.
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 100;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 2, column: 5 },
            ],
        });
        let plant_id = game.state.board.plants[0].id;
        let mut setup = Vec::new();
        game.spawn_vase_zombie(ZombieType::Jackbox, 2, 5, &mut setup);
        let jack = game
            .state
            .board
            .zombies
            .iter()
            .find(|zombie| zombie.zombie_type == ZombieType::Jackbox)
            .unwrap();
        assert_eq!(jack.health, JACKBOX_HEALTH);
        assert_eq!(jack.jackbox_timer, VASE_JACKBOX_POP_TICKS);
        let events = (0..VASE_JACKBOX_POP_TICKS)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert!(
            events.iter().any(|event| matches!(
                event,
                GameEvent::PlantDied { entity } if *entity == plant_id
            )),
            "the vase Jack should pop and destroy the adjacent plant"
        );

        let mut other = Game::new(7, SceneKind::Day);
        other.state.sun = 100;
        other.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 2, column: 5 },
            ],
        });
        let other_plant = other.state.board.plants[0].id;
        let mut setup = Vec::new();
        let zombie = other.spawn_jackbox_zombie(2, 0, Some(grid_x(5)), &mut setup);
        other.state.board.zombies.iter_mut().for_each(|candidate| {
            if candidate.id == zombie {
                candidate.health = 0;
            }
        });
        other.advance(InputFrame::default());
        assert!(
            other
                .state
                .board
                .plants
                .iter()
                .any(|plant| plant.id == other_plant && plant.health > 0),
            "a Jack killed by damage must not explode"
        );
    }

    #[test]
    fn yeti_flees_right_after_its_phase_and_emits_flee_event() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_yeti_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        let index = game
            .state
            .board
            .zombies
            .iter()
            .position(|candidate| candidate.id == zombie)
            .unwrap();

        assert_eq!(
            game.state.board.zombies[index].zombie_type,
            ZombieType::Yeti
        );
        assert_eq!(game.state.board.zombies[index].health, YETI_HEALTH);
        assert_eq!(game.state.board.zombies[index].max_health, YETI_HEALTH);
        assert_eq!(game.state.board.zombies[index].speed, YETI_WALK_SPEED);
        assert!(
            (YETI_FLEE_MIN_TICKS..=YETI_FLEE_MAX_TICKS)
                .contains(&game.state.board.zombies[index].yeti_counter)
        );

        game.state.board.zombies[index].yeti_counter = 1;
        let before = game.state.board.zombies[index].position_x;
        let events = game.advance(InputFrame::default());
        let running = game
            .state
            .board
            .zombies
            .iter()
            .find(|candidate| candidate.id == zombie)
            .unwrap();
        assert!(running.yeti_running);
        assert!(running.position_x > before);
        assert!(
            !events.iter().any(
                |event| matches!(event, GameEvent::ZombieFled { entity } if *entity == zombie)
            )
        );

        game.state.board.zombies[index].position_x = YETI_FLEE_EDGE;
        let events = game.advance(InputFrame::default());
        assert!(
            !game
                .state
                .board
                .zombies
                .iter()
                .any(|candidate| candidate.id == zombie)
        );
        assert!(
            events.iter().any(
                |event| matches!(event, GameEvent::ZombieFled { entity } if *entity == zombie)
            )
        );
    }

    #[test]
    fn yeti_flee_phase_respects_freeze_and_hypnosis() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        game.spawn_yeti_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        let yeti = &mut game.state.board.zombies[0];
        yeti.yeti_counter = 1;
        yeti.frozen_counter = 2;
        let state = |game: &Game| {
            let yeti = &game.state.board.zombies[0];
            (yeti.frozen_counter, yeti.yeti_counter, yeti.yeti_running)
        };

        game.advance(InputFrame::default());
        assert_eq!(state(&game), (1, 1, false));

        game.advance(InputFrame::default());
        assert_eq!(state(&game), (0, 1, false));

        game.advance(InputFrame::default());
        assert_eq!(state(&game), (0, 0, true));

        let yeti = &mut game.state.board.zombies[0];
        yeti.yeti_running = false;
        yeti.yeti_counter = 0;
        yeti.frozen_counter = 1;
        game.advance(InputFrame::default());
        assert_eq!(state(&game), (0, 0, true));

        let yeti = &mut game.state.board.zombies[0];
        yeti.yeti_running = false;
        yeti.yeti_counter = 1;
        yeti.hypnotized = true;
        game.advance(InputFrame::default());
        assert_eq!(state(&game), (0, 0, false));
    }

    #[test]
    fn yeti_drops_four_diamonds_when_killed() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_yeti_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        game.state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == zombie)
            .unwrap()
            .health = 0;

        let mut events = Vec::new();
        game.emit_zombie_died(zombie, &mut events);
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(
                    event,
                    GameEvent::CoinProduced {
                        coin_type: CoinType::Diamond,
                        value: 100,
                        ..
                    }
                ))
                .count(),
            YETI_DIAMOND_COUNT
        );
        assert_eq!(game.state.board.coins.len(), YETI_DIAMOND_COUNT);
        assert!(
            events.iter().any(
                |event| matches!(event, GameEvent::ZombieDied { entity } if *entity == zombie)
            )
        );

        let coin_ids = game
            .state
            .board
            .coins
            .iter()
            .map(|coin| coin.id)
            .collect::<Vec<_>>();
        for coin_id in coin_ids {
            game.collect_coin(coin_id, &mut events);
        }
        assert_eq!(game.state.coins, 400);
        assert!(game.state.board.coins.is_empty());

        game.emit_zombie_died(zombie, &mut events);
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event, GameEvent::CoinProduced { .. }))
                .count(),
            YETI_DIAMOND_COUNT
        );
    }

    #[test]
    fn wallnut_and_tallnut_keep_their_target_health_and_block_bites_until_zero() {
        for (slot, expected_health) in [(3, 4_000), (23, 8_000)] {
            let mut game = Game::new(7, SceneKind::Day);
            game.state.sun = expected_health as u32;
            game.advance(InputFrame {
                actions: vec![
                    InputAction::SelectSeed { slot },
                    InputAction::Plant { row: 2, column: 0 },
                ],
            });

            assert_eq!(game.state.board.plants.len(), 1);
            assert_eq!(game.state.board.plants[0].health, expected_health);
            assert_eq!(game.state.board.plants[0].max_health, expected_health);

            // Keep this focused check short while still exercising both the
            // non-terminal and terminal ordinary-bite paths.
            game.state.board.plants[0].health = ZOMBIE_BITE_DAMAGE * 2;
            let mut setup_events = Vec::new();
            game.spawn_normal_zombie(
                2,
                0,
                Some(grid_x(0) + 10 * POSITION_SCALE),
                &mut setup_events,
            );

            for _ in 0..4 {
                game.advance(InputFrame::default());
            }
            assert_eq!(game.state.board.plants[0].health, ZOMBIE_BITE_DAMAGE);
            assert!(game.state.board.plants[0].health > 0);

            let events = (0..4)
                .flat_map(|_| game.advance(InputFrame::default()))
                .collect::<Vec<_>>();
            assert!(events.iter().any(|event| matches!(
                event,
                GameEvent::PlantDamaged {
                    damage: ZOMBIE_BITE_DAMAGE,
                    health_remaining: 0,
                    ..
                }
            )));
            assert!(
                events
                    .iter()
                    .any(|event| matches!(event, GameEvent::PlantDied { .. }))
            );
            assert!(game.state.board.plants.is_empty());
        }
    }

    #[test]
    fn explode_o_nut_explodes_when_eaten_and_damages_nearby_zombies() {
        let mut game = Game::new(7, SceneKind::Night);
        game.state.sun = 200;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 49 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        assert_eq!(game.state.board.plants[0].health, 4_000);
        assert_eq!(game.state.board.plants[0].plant_type.slot(), 49);

        // Reduce health to save test time — 80 HP = 20 bites, ~80 ticks to eat through.
        game.state.board.plants[0].health = 80;

        let explode_id = game.state.board.plants[0].id;
        let mut setup = Vec::new();
        let zombie =
            game.spawn_normal_zombie(2, 0, Some(grid_x(2) + 30 * POSITION_SCALE), &mut setup);
        let other =
            game.spawn_normal_zombie(0, 0, Some(grid_x(2) + 30 * POSITION_SCALE), &mut setup);

        // Let zombie eat through the full 4000 HP (1000 bites at 4 damage each).
        let mut saw_special = false;
        let mut saw_hit = false;
        let mut saw_plant_died = false;
        for _ in 0..200 {
            let events = game.advance(InputFrame::default());
            for event in &events {
                match event {
                    GameEvent::PlantSpecialTriggered {
                        entity,
                        plant_type: PlantType::Other(49),
                    } if *entity == explode_id => saw_special = true,
                    GameEvent::PlantSpecialHit {
                        plant,
                        zombie: hit_zombie,
                        damage: 1_800,
                        ..
                    } if *plant == explode_id && *hit_zombie == zombie => saw_hit = true,
                    GameEvent::PlantDied { entity } if *entity == explode_id => {
                        saw_plant_died = true
                    }
                    _ => {}
                }
            }
        }
        assert!(saw_special, "PlantSpecialTriggered");
        assert!(saw_hit, "PlantSpecialHit");
        assert!(saw_plant_died, "PlantDied");
        assert!(game.state.board.plants.is_empty(), "explode plant removed");
        assert!(
            !game.state.board.zombies.iter().any(|z| z.id == zombie),
            "same-row zombie killed"
        );
        assert!(
            game.state.board.zombies.iter().any(|z| z.id == other),
            "other-row zombie survived"
        );
    }

    #[test]
    fn hypno_shroom_hypnotizes_a_biting_zombie() {
        let mut game = Game::new(7, SceneKind::Night);
        game.state.sun = 75;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 12 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        assert_eq!(game.state.board.plants[0].plant_type.slot(), 12);

        let hypno_id = game.state.board.plants[0].id;
        let mut setup = Vec::new();
        let zombie =
            game.spawn_normal_zombie(2, 0, Some(grid_x(2) + 30 * POSITION_SCALE), &mut setup);
        let other =
            game.spawn_normal_zombie(2, 0, Some(grid_x(2) + 120 * POSITION_SCALE), &mut setup);

        let mut hypnotized = false;
        let mut plant_died = false;
        let mut zombie_damaged = false;
        let mut other_died = false;
        for _ in 0..600 {
            let events = game.advance(InputFrame::default());
            for event in &events {
                match event {
                    GameEvent::ZombieHypnotized { entity } if *entity == zombie => {
                        hypnotized = true
                    }
                    GameEvent::PlantDied { entity } if *entity == hypno_id => plant_died = true,
                    GameEvent::ZombieDamaged {
                        entity: target,
                        attacker: Some(a),
                        ..
                    } if *a == zombie && *target == other => zombie_damaged = true,
                    GameEvent::ZombieDied { entity } if *entity == other => other_died = true,
                    _ => {}
                }
            }
        }
        assert!(hypnotized, "ZombieHypnotized");
        assert!(plant_died, "PlantDied");
        assert!(zombie_damaged, "hypnotized zombie attacked other zombie");
        assert!(other_died, "other zombie died from hypnotized attacks");
        assert!(game.state.board.plants.is_empty(), "hypno plant removed");
    }

    #[test]
    fn chomper_swallow_winds_up_then_removes_a_normal_zombie() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 200;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 6 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        let chomper = game.state.board.plants[0].id;
        let mut setup_events = Vec::new();
        let zombie = game.spawn_normal_zombie(
            2,
            0,
            Some(grid_x(0) + 10 * POSITION_SCALE),
            &mut setup_events,
        );

        let events = (0..=CHOMPER_BITE_WINDUP_TICKS)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();

        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantSpecialTriggered {
                entity,
                plant_type: PlantType::Other(6),
            } if *entity == chomper
        )));
        assert!(
            events.iter().any(
                |event| matches!(event, GameEvent::ZombieDied { entity } if *entity == zombie)
            )
        );
        assert!(game.state.board.zombies.is_empty());
        assert_eq!(
            game.state.board.plants[0].special_counter,
            CHOMPER_CHEW_TICKS
        );
        assert!(!game.state.board.plants[0].special_armed);
    }

    #[test]
    fn first_wave_spawns_on_tick_1800() {
        let mut game = Game::new(11, SceneKind::Day);
        let mut last_events = Vec::new();
        for _ in 0..FIRST_WAVE_COUNTDOWN {
            last_events = game.advance(InputFrame::default());
        }

        assert_eq!(game.state.tick, u64::from(FIRST_WAVE_COUNTDOWN));
        assert_eq!(game.state.wave, 1);
        assert_eq!(game.state.board.zombies.len(), 1);
        assert!(
            last_events
                .iter()
                .any(|event| matches!(event, GameEvent::WaveStarted { wave: 0 }))
        );
    }

    #[test]
    fn aquatic_plants_require_pool_and_lilypad_supports_a_land_plant() {
        let mut lawn = Game::new(7, SceneKind::Day);
        lawn.state.sun = 500;
        let rejected = lawn.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 16 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        assert!(rejected.iter().any(|event| matches!(
            event,
            GameEvent::InputRejected {
                reason: InputRejectReason::InvalidTerrain,
                ..
            }
        )));
        assert!(lawn.state.board.plants.is_empty());
        assert_eq!(lawn.state.sun, 500);

        let mut pool = Game::new(7, SceneKind::Pool);
        pool.state.sun = 500;
        pool.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 16 },
                InputAction::Plant { row: 2, column: 2 },
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        assert_eq!(pool.state.board.plants.len(), 2);
        assert_eq!(pool.state.board.plants[0].plant_type, PlantType::Other(16));
        assert_eq!(pool.state.board.plants[1].plant_type, PlantType::Peashooter);

        let top = pool.state.board.plants[1].id;
        let events = pool.advance(InputFrame {
            actions: vec![InputAction::Shovel { row: 2, column: 2 }],
        });
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantShoveled { entity } if *entity == top
        )));
        assert_eq!(pool.state.board.plants.len(), 1);
        assert_eq!(pool.state.board.plants[0].plant_type, PlantType::Other(16));

        let base = pool.state.board.plants[0].id;
        let events = pool.advance(InputFrame {
            actions: vec![InputAction::Shovel { row: 2, column: 2 }],
        });
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantShoveled { entity } if *entity == base
        )));
        assert!(pool.state.board.plants.is_empty());
    }

    #[test]
    fn flower_pot_is_required_for_roof_planting_and_shoveled_first() {
        let mut bare_roof = Game::new(7, SceneKind::Roof);
        bare_roof.state.sun = 500;
        let rejected = bare_roof.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        assert!(rejected.iter().any(|event| matches!(
            event,
            GameEvent::InputRejected {
                reason: InputRejectReason::InvalidTerrain,
                ..
            }
        )));
        assert!(bare_roof.state.board.plants.is_empty());

        let mut roof = Game::new(7, SceneKind::Roof);
        roof.state.sun = 500;
        roof.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 33 },
                InputAction::Plant { row: 2, column: 2 },
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        assert_eq!(roof.state.board.plants.len(), 2);
        assert_eq!(roof.state.board.plants[0].plant_type, PlantType::Other(33));
        assert_eq!(roof.state.board.plants[1].plant_type, PlantType::Peashooter);

        let top = roof.state.board.plants[1].id;
        let events = roof.advance(InputFrame {
            actions: vec![InputAction::Shovel { row: 2, column: 2 }],
        });
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantShoveled { entity } if *entity == top
        )));
        assert_eq!(roof.state.board.plants.len(), 1);
        assert_eq!(roof.state.board.plants[0].plant_type, PlantType::Other(33));
    }

    #[test]
    fn lawn_mower_triggers_before_a_zombie_can_end_the_row() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup_events = Vec::new();
        let zombie = game.spawn_normal_zombie(2, 0, Some(0), &mut setup_events);
        game.state.board.zombies[0].speed = 0;

        let events = game.advance(InputFrame::default());

        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::MowerTriggered { row: 2 }))
        );
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieDied { entity } if *entity == zombie
        )));
        assert!(matches!(game.state.scene, SceneKind::Day));
        assert!(game.state.board.zombies.is_empty());
        assert!(game.state.board.mowers[2].active);
    }

    #[test]
    fn a_row_without_a_lawn_mower_reaches_game_over() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.board.mowers.clear();
        let mut setup_events = Vec::new();
        let zombie = game.spawn_normal_zombie(2, 0, Some(-100 * POSITION_SCALE), &mut setup_events);
        game.state.board.zombies[0].speed = 0;

        let events = game.advance(InputFrame::default());

        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::GameLost { zombie: id } if *id == zombie
        )));
        assert!(matches!(game.state.scene, SceneKind::GameOver));
    }

    #[test]
    fn debug_game_lost_checkpoint_emits_terminal_audio_event() {
        let mut game = Game::new_mode(7, ModeKind::Adventure, 1);
        game.debug_prepare_game_lost();
        let events = game.advance(InputFrame::default());
        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::GameLost { .. }))
        );
        assert!(matches!(game.state.scene, SceneKind::GameOver));
    }

    #[test]
    fn debug_game_won_checkpoint_emits_terminal_audio_event() {
        let mut game = Game::new_mode(7, ModeKind::Adventure, 1);
        game.debug_prepare_game_won();
        let events = game.advance(InputFrame::default());
        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::GameWon))
        );
        assert!(matches!(game.state.scene, SceneKind::Complete));
    }

    #[test]
    fn debug_explosion_checkpoint_triggers_all_three_specials() {
        let mut game = Game::new(0, SceneKind::Night);
        game.debug_prepare_explosion_plants();
        assert_eq!(game.state.board.plants.len(), 3);
        let events = game.advance(InputFrame::default());
        for plant_type in [2, 20, 15] {
            assert!(events.iter().any(|event| matches!(
                event,
                GameEvent::PlantSpecialTriggered {
                    plant_type: PlantType::Other(actual),
                    ..
                } if *actual == plant_type
            )));
        }
    }

    #[test]
    fn debug_explode_o_nut_checkpoint_emits_special_event() {
        let mut game = Game::new_mode(0, ModeKind::MiniGame, 1);
        game.debug_prepare_explode_o_nut();
        let mut saw_special = false;
        for _ in 0..200 {
            saw_special |= game.advance(InputFrame::default()).iter().any(|event| {
                matches!(
                    event,
                    GameEvent::PlantSpecialTriggered {
                        plant_type: PlantType::Other(49),
                        ..
                    }
                )
            });
            if saw_special {
                break;
            }
        }
        assert!(
            saw_special,
            "Explode-O-Nut bite must emit its special event"
        );
    }

    #[test]
    fn debug_imp_throw_checkpoint_emits_throw_event() {
        let mut game = Game::new(0, SceneKind::Day);
        game.debug_prepare_imp_throw();
        let first = game.advance(InputFrame::default());
        assert!(
            !first
                .iter()
                .any(|event| matches!(event, GameEvent::ImpThrown { .. }))
        );
        let events = (0..105)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::ImpThrown { .. }))
        );
    }

    #[test]
    fn debug_blover_chomper_checkpoint_triggers_both_audio_events() {
        let mut game = Game::new(0, SceneKind::Day);
        game.debug_prepare_blover_chomper();
        let events = game.advance(InputFrame::default());
        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::BloverTriggered { .. }))
        );
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantSpecialTriggered {
                plant_type: PlantType::Other(6),
                ..
            }
        )));
    }

    #[test]
    fn debug_hypno_jackbox_checkpoint_triggers_both_audio_events() {
        let mut game = Game::new(0, SceneKind::Night);
        game.debug_prepare_hypno_jackbox();
        let mut saw_hypno = false;
        let mut saw_jackbox = false;
        for _ in 0..700 {
            for event in game.advance(InputFrame::default()) {
                saw_hypno |= matches!(event, GameEvent::ZombieHypnotized { .. });
                saw_jackbox |= matches!(event, GameEvent::JackboxExploded { .. });
            }
            if saw_hypno && saw_jackbox {
                break;
            }
        }
        assert!(saw_hypno);
        assert!(saw_jackbox);
    }

    #[test]
    fn debug_cob_cannon_checkpoint_emits_fire_audio_event() {
        let mut game = Game::new(0, SceneKind::Day);
        game.debug_prepare_cob_cannon();
        let cannon = game
            .state
            .board
            .plants
            .iter()
            .find(|plant| plant.plant_type == PlantType::Other(47))
            .expect("debug checkpoint must prepare a CobCannon");
        assert!(cannon.special_armed);
        let entity = cannon.id;
        let events = game.advance(InputFrame {
            actions: vec![InputAction::FireCobCannon {
                entity,
                row: 2,
                column: 4,
            }],
        });
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::CobCannonFired {
                entity: fired,
                target_row: 2,
                target_column: 4,
            } if *fired == entity
        )));
    }

    #[test]
    fn debug_portal_checkpoint_emits_open_audio_event() {
        let mut game = Game::new(0, SceneKind::Day);
        let events = game.debug_prepare_portal();
        assert_eq!(game.state.board.portals, vec![(2, 5, true)]);
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PortalOpened {
                row: 2,
                column: 5,
                square: true,
            }
        )));
    }

    #[test]
    fn debug_gravebuster_checkpoint_emits_special_audio_event() {
        let mut game = Game::new(0, SceneKind::Day);
        game.debug_prepare_gravebuster();
        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantSpecialTriggered {
                plant_type: PlantType::Other(11),
                ..
            }
        )));
        assert!(game.state.board.graves.is_empty());
    }

    #[test]
    fn debug_coffee_checkpoint_emits_special_audio_event() {
        let mut game = Game::new(0, SceneKind::Day);
        game.debug_prepare_coffee();
        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantSpecialTriggered {
                plant_type: PlantType::Other(35),
                ..
            }
        )));
    }

    #[test]
    fn debug_tangle_kelp_checkpoint_emits_special_audio_event() {
        let mut game = Game::new(0, SceneKind::Pool);
        game.debug_prepare_tangle_kelp();
        let events = game.advance(InputFrame::default());
        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::TangleKelpGrabStarted { .. }))
        );
    }

    #[test]
    fn garlic_consumes_itself_then_diverts_the_biting_zombie() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 500;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 36 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let garlic = game.state.board.plants[0].id;
        let mut setup_events = Vec::new();
        let zombie = game.spawn_normal_zombie(
            2,
            0,
            Some(grid_x(2) + 20 * POSITION_SCALE),
            &mut setup_events,
        );
        game.state.board.zombies[0].speed = 0;
        for _ in 0..8 {
            game.advance(InputFrame::default());
        }
        assert!(game.state.board.zombies[0].garlic_counter > 0);

        game.state.board.zombies[0].garlic_counter = GARLIC_EAT_TICKS - 1;
        let eaten = game.advance(InputFrame::default());
        assert!(eaten.iter().any(|event| matches!(
            event,
            GameEvent::PlantDied { entity } if *entity == garlic
        )));
        assert!(game.state.board.plants.is_empty());

        game.state.board.zombies[0].garlic_counter = GARLIC_ROW_CHANGE_TICKS - 1;
        let diverted = game.advance(InputFrame::default());
        assert!(diverted.iter().any(|event| matches!(
            event,
            GameEvent::ZombieRowChanged { entity, from, to }
                if *entity == zombie && from != to
        )));
        assert_ne!(game.state.board.zombies[0].row, 2);
    }

    #[test]
    fn tangle_kelp_grabs_a_nearby_zombie_then_dies() {
        let mut game = Game::new(7, SceneKind::Pool);
        game.state.sun = 500;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 19 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let tangle_kelp = game.state.board.plants[0].id;
        let mut setup_events = Vec::new();
        let zombie = game.spawn_normal_zombie(
            2,
            0,
            Some(grid_x(2) + 30 * POSITION_SCALE),
            &mut setup_events,
        );

        let events = (0..=TANGLE_KELP_GRAB_TICKS)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();

        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantSpecialTriggered {
                entity,
                plant_type: PlantType::Other(19),
            } if *entity == tangle_kelp
        )));
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::TangleKelpWaterEntry { entity } if *entity == tangle_kelp
        )));
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieDied { entity } if *entity == zombie
        )));
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantDied { entity } if *entity == tangle_kelp
        )));
        assert!(game.state.board.zombies.is_empty());
        assert!(game.state.board.plants.is_empty());
    }

    #[test]
    fn spikerock_attacks_twice_per_cycle_and_has_extra_health() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 500;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 46 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        let spikerock = game.state.board.plants[0].id;
        assert_eq!(game.state.board.plants[0].max_health, 450);

        let mut setup_events = Vec::new();
        let zombie = game.spawn_normal_zombie(
            2,
            0,
            Some(grid_x(0) + 40 * POSITION_SCALE),
            &mut setup_events,
        );
        let starting_health = game.state.board.zombies[0].health;
        let events = (0..70)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();

        let hits = events
            .iter()
            .filter(|event| {
                matches!(
                    event,
                    GameEvent::PlantSpecialHit {
                        plant,
                        zombie: hit_zombie,
                        damage: SPIKEWEED_DAMAGE,
                        ..
                    } if *plant == spikerock && *hit_zombie == zombie
                )
            })
            .count();
        assert_eq!(hits, 2);
        assert_eq!(
            game.state.board.zombies[0].health,
            starting_health - 2 * SPIKEWEED_DAMAGE
        );
    }

    #[test]
    fn source_coin_type_catalog_preserves_money_sun_and_award_groups() {
        let types = [
            CoinType::Silver,
            CoinType::Gold,
            CoinType::Diamond,
            CoinType::Sun,
            CoinType::SmallSun,
            CoinType::LargeSun,
            CoinType::FinalSeedPacket,
            CoinType::Trophy,
            CoinType::Shovel,
            CoinType::Almanac,
            CoinType::CarKeys,
            CoinType::Vase,
            CoinType::WateringCan,
            CoinType::Taco,
            CoinType::Note,
            CoinType::UsableSeedPacket,
            CoinType::PresentPlant,
            CoinType::AwardMoneyBag,
            CoinType::AwardPresent,
            CoinType::AwardBagDiamond,
            CoinType::AwardSilverSunflower,
            CoinType::AwardGoldSunflower,
            CoinType::Chocolate,
            CoinType::AwardChocolate,
            CoinType::PresentMinigames,
            CoinType::PresentPuzzleMode,
            CoinType::PresentSurvivalMode,
        ];
        assert_eq!(types.len(), 27);
        assert_eq!(
            types
                .iter()
                .copied()
                .filter(|coin_type| coin_type.is_money())
                .count(),
            3
        );
        assert_eq!(CoinType::Silver.value(), 1);
        assert_eq!(CoinType::Gold.value(), 5);
        assert_eq!(CoinType::Diamond.value(), 100);
        assert_eq!(CoinType::SmallSun.sun_value(), 15);
        assert_eq!(CoinType::Sun.sun_value(), 25);
        assert_eq!(CoinType::LargeSun.sun_value(), 50);
        assert!(CoinType::FinalSeedPacket.is_level_award());
        assert!(CoinType::AwardChocolate.is_level_award());
        assert_eq!(CoinType::PresentMinigames.unlock_mask(), 1);
        assert_eq!(CoinType::PresentPuzzleMode.unlock_mask(), 2);
        assert_eq!(CoinType::PresentSurvivalMode.unlock_mask(), 4);
    }

    #[test]
    fn prize_pickups_update_unlocks_garden_inventory_and_currency() {
        let mut game = Game::new(7, SceneKind::Day);
        let collect = |game: &mut Game, coin_type, plant_type, usable_seed_type| {
            let mut produced = Vec::new();
            game.spawn_pickup_with_payload(
                coin_type,
                grid_x(2),
                grid_y(2),
                plant_type,
                usable_seed_type,
                &mut produced,
            );
            let entity = game.state.board.coins.last().unwrap().id;
            let events = game.advance(InputFrame {
                actions: vec![InputAction::CollectCoin { entity }],
            });
            assert!(events.iter().any(|event| matches!(
                event,
                GameEvent::PickupCollected { entity: found, .. } if *found == entity
            )));
        };

        collect(&mut game, CoinType::PresentMinigames, None, None);
        collect(&mut game, CoinType::PresentPuzzleMode, None, None);
        collect(&mut game, CoinType::PresentSurvivalMode, None, None);
        assert_eq!(game.state.unlocked_modes, 7);

        collect(
            &mut game,
            CoinType::PresentPlant,
            Some(PlantType::Other(31)),
            None,
        );
        assert_eq!(game.state.garden.plants[0].plant_type, PlantType::Other(31));

        collect(
            &mut game,
            CoinType::UsableSeedPacket,
            None,
            Some(PlantType::Other(5)),
        );
        assert_eq!(game.state.board.selected_seed, Some(5));

        collect(&mut game, CoinType::Chocolate, None, None);
        collect(&mut game, CoinType::AwardMoneyBag, None, None);
        collect(&mut game, CoinType::AwardBagDiamond, None, None);
        assert_eq!(game.state.chocolates, 1);
        assert_eq!(game.state.coins, 525);
    }

    #[test]
    fn award_pickups_use_source_coin_arc_motion() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut events = Vec::new();
        game.spawn_pickup(CoinType::FinalSeedPacket, grid_x(4), grid_y(2), &mut events);
        let coin = game.state.board.coins.last().unwrap().clone();
        let coin_id = coin.id;
        assert!(coin.target_y.is_some());
        assert!(coin.velocity_y <= -3_000_000);
        assert!(coin.velocity_x.abs() <= 500_000);
        assert!(
            (coin.position_y + 15 * POSITION_SCALE..=coin.position_y + 34 * POSITION_SCALE)
                .contains(&coin.target_y.unwrap())
        );

        let mut sunflower_events = Vec::new();
        game.spawn_pickup(
            CoinType::AwardSilverSunflower,
            grid_x(4),
            380 * POSITION_SCALE,
            &mut sunflower_events,
        );
        let sunflower = game.state.board.coins.last().unwrap();
        assert_eq!(sunflower.position_y, 280 * POSITION_SCALE);
        assert_eq!(sunflower.target_y, Some(325 * POSITION_SCALE));

        let mut last_y = coin.position_y;
        for _ in 0..80 {
            game.advance(InputFrame::default());
            let coin = game
                .state
                .board
                .coins
                .iter()
                .find(|coin| coin.id == coin_id)
                .unwrap();
            if coin.target_y.is_none() {
                assert_eq!(coin.velocity_x, 0);
                assert_eq!(coin.velocity_y, 0);
                return;
            }
            last_y = coin.position_y;
        }
        panic!("award coin did not land, last y {last_y}");
    }

    #[test]
    fn marigold_produces_and_collects_a_coin() {
        let mut game = Game::new(7, SceneKind::Day);
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 38 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        game.state.board.plants[0].launch_counter = 1;

        let produced = game.advance(InputFrame::default());
        let coin = game.state.board.coins[0].clone();
        assert!(produced.iter().any(|event| matches!(
            event,
            GameEvent::CoinProduced {
                entity,
                coin_type: CoinType::Silver | CoinType::Gold,
                value: 1 | 5,
            } if *entity == coin.id
        )));

        let collected = game.advance(InputFrame {
            actions: vec![InputAction::CollectCoin { entity: coin.id }],
        });
        assert!(collected.iter().any(|event| matches!(
            event,
            GameEvent::CoinCollected {
                entity,
                coin_type,
                value,
                coin_total,
            } if *entity == coin.id
                && *coin_type == coin.coin_type
                && *value == coin.value
                && *coin_total == coin.value
        )));
        assert_eq!(game.state.coins, coin.value);
        assert!(game.state.board.coins.is_empty());
    }

    #[test]
    fn gold_magnet_requires_magnetshroom_and_collects_coins() {
        let mut bare = Game::new(7, SceneKind::Day);
        bare.state.sun = 500;
        let rejected = bare.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 45 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        assert!(rejected.iter().any(|event| matches!(
            event,
            GameEvent::InputRejected {
                action: InputAction::Plant { row: 2, column: 2 },
                reason: InputRejectReason::Occupied,
            }
        )));

        let mut game = Game::new(7, SceneKind::Night);
        game.state.sun = 500;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 31 },
                InputAction::Plant { row: 2, column: 2 },
                InputAction::SelectSeed { slot: 45 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        assert_eq!(game.state.board.plants.len(), 2);
        game.state.board.wave.countdown = 100_000;
        let mut spawn_events = Vec::new();
        game.spawn_coin(CoinType::Gold, grid_x(2), grid_y(2), &mut spawn_events);
        let coin_id = game.state.board.coins[0].id;

        let mut collected = false;
        for _ in 0..5_000 {
            let events = game.advance(InputFrame::default());
            if events.iter().any(|event| {
                matches!(
                    event,
                    GameEvent::CoinCollected { entity, .. } if *entity == coin_id
                )
            }) {
                collected = true;
                break;
            }
        }
        assert!(collected);
        assert_eq!(game.state.coins, 5);
        assert!(game.state.board.coins.is_empty());
    }

    #[test]
    fn blover_triggers_once_after_fifty_ticks_and_survives() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 100;
        let placed_events = game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 27 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let blover = game.state.board.plants[0].id;
        assert!(placed_events.iter().any(|event| matches!(
            event,
            GameEvent::PlantPlaced {
                entity,
                plant_type: PlantType::Other(27),
                ..
            } if *entity == blover
        )));
        assert_eq!(
            game.state.board.plants[0].special_counter,
            BLOVER_SPECIAL_COUNTDOWN - 1
        );

        let mut trigger_count = 0;
        for _ in 0..49 {
            let events = game.advance(InputFrame::default());
            trigger_count += events
                .iter()
                .filter(|event| matches!(event, GameEvent::BloverTriggered { entity, .. } if *entity == blover))
                .count();
        }
        assert_eq!(trigger_count, 1);
        assert!(
            game.state
                .board
                .plants
                .iter()
                .any(|plant| plant.id == blover)
        );

        let events = game.advance(InputFrame::default());
        assert!(!events.iter().any(
            |event| matches!(event, GameEvent::BloverTriggered { entity, .. } if *entity == blover)
        ));
    }

    #[test]
    fn instant_coffee_wakes_a_sleeping_mushroom_after_one_hundred_ticks() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 200;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 8 },
                InputAction::Plant { row: 2, column: 2 },
                InputAction::SelectSeed { slot: 35 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });

        let mushroom = game
            .state
            .board
            .plants
            .iter()
            .find(|plant| plant.plant_type.slot() == 8)
            .unwrap()
            .id;
        let coffee = game
            .state
            .board
            .plants
            .iter()
            .find(|plant| plant.plant_type.slot() == 35)
            .unwrap()
            .id;
        assert!(
            game.state
                .board
                .plants
                .iter()
                .find(|plant| plant.id == mushroom)
                .unwrap()
                .asleep
        );
        assert_eq!(
            game.state
                .board
                .plants
                .iter()
                .find(|plant| plant.id == coffee)
                .unwrap()
                .special_counter,
            COFFEE_WAKE_TICKS - 1
        );

        let mut triggered = false;
        for _ in 0..99 {
            triggered |= game.advance(InputFrame::default()).iter().any(|event| {
                matches!(
                    event,
                    GameEvent::PlantSpecialTriggered { entity, plant_type: PlantType::Other(35) }
                        if *entity == coffee
                )
            });
        }
        assert!(triggered);
        assert!(
            game.state
                .board
                .plants
                .iter()
                .all(|plant| plant.id != coffee)
        );
        let mushroom_state = game
            .state
            .board
            .plants
            .iter()
            .find(|plant| plant.id == mushroom)
            .unwrap();
        assert!(mushroom_state.asleep);
        assert_eq!(mushroom_state.wake_up_counter, COFFEE_WAKE_TICKS);

        for _ in 0..COFFEE_WAKE_TICKS - 1 {
            game.advance(InputFrame::default());
        }
        assert!(
            game.state
                .board
                .plants
                .iter()
                .find(|plant| plant.id == mushroom)
                .unwrap()
                .asleep
        );
        game.advance(InputFrame::default());
        let mushroom_state = game
            .state
            .board
            .plants
            .iter()
            .find(|plant| plant.id == mushroom)
            .unwrap();
        assert!(!mushroom_state.asleep);
        assert_eq!(mushroom_state.wake_up_counter, 0);
    }

    #[test]
    fn restart_restores_the_terminal_scene_and_clears_progress() {
        let mut game = Game::new(7, SceneKind::Night);
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        game.state.tick = 42;
        game.debug_force_game_over();
        game.state.sun = 25;

        let events = game.advance(InputFrame {
            actions: vec![InputAction::Restart],
        });

        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::Restarted))
        );
        assert_eq!(game.state.scene, SceneKind::Night);
        assert_eq!(game.state.level_scene, SceneKind::Night);
        assert_eq!(game.state.tick, 0);
        assert_eq!(game.state.sun, 50);
        assert!(game.state.board.plants.is_empty());
    }

    #[test]
    fn pause_freezes_simulation_until_resume() {
        let mut game = Game::new(7, SceneKind::Day);
        let initial = game.state.clone();

        let paused_events = game.advance(InputFrame {
            actions: vec![InputAction::Pause],
        });
        assert!(
            paused_events
                .iter()
                .any(|event| matches!(event, GameEvent::Paused))
        );
        for _ in 0..20 {
            game.advance(InputFrame::default());
        }
        assert!(game.state.paused);
        assert_eq!(game.state.tick, initial.tick);
        assert_eq!(game.state.board.sun_countdown, initial.board.sun_countdown);
        assert_eq!(game.state.board.wave, initial.board.wave);

        let resumed_events = game.advance(InputFrame {
            actions: vec![InputAction::Resume],
        });
        assert!(
            resumed_events
                .iter()
                .any(|event| matches!(event, GameEvent::Resumed))
        );
        assert!(!game.state.paused);
        assert_eq!(game.state.tick, initial.tick + 1);
    }

    #[test]
    fn all_waves_complete_and_no_zombies_triggers_game_won() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.board.wave.current = game.state.board.wave.total;
        game.state.board.wave.countdown = 0;

        let events = game.advance(InputFrame::default());

        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::GameWon))
        );
        assert!(matches!(game.state.scene, SceneKind::Complete));
    }

    #[test]
    fn replay_record_round_trips_and_verifies() {
        let mut replay = Replay::new(7, SceneKind::Day);
        replay.frames = vec![
            InputFrame {
                actions: vec![
                    InputAction::SelectSeed { slot: 1 },
                    InputAction::Plant { row: 2, column: 0 },
                ],
            },
            InputFrame {
                actions: vec![InputAction::Pause],
            },
            InputFrame {
                actions: vec![InputAction::Resume],
            },
        ];
        let record = ReplayRecord::capture(replay).unwrap();
        let encoded = record.to_json_pretty().unwrap();
        let decoded = ReplayRecord::from_json(&encoded).unwrap();

        decoded.verify().unwrap();
        assert_eq!(record, decoded);
        assert_eq!(decoded.outcome.final_state.tick, 2);
    }

    #[test]
    fn replay_record_detects_tampering() {
        let mut record = ReplayRecord::capture(Replay::new(7, SceneKind::Day)).unwrap();
        record.outcome.final_state.sun += 1;
        assert!(matches!(record.verify(), Err(CoreError::ReplayMismatch)));
    }

    #[test]
    fn seed_changes_the_complete_final_state_hash() {
        let first = Replay::new(7, SceneKind::Day).run().unwrap();
        let second = Replay::new(8, SceneKind::Day).run().unwrap();
        assert_ne!(first.final_hash, second.final_hash);
    }

    #[test]
    fn mode_catalog_matches_the_target_visible_unit_counts() {
        assert_eq!(mode_level_names(ModeKind::Survival).len(), 11);
        assert_eq!(mode_level_names(ModeKind::MiniGame).len(), 20);
        assert_eq!(mode_level_names(ModeKind::Vasebreaker).len(), 10);
        assert_eq!(mode_level_names(ModeKind::IZombie).len(), 10);
        assert_eq!(mode_level_names(ModeKind::ZenGarden).len(), 4);
        assert_eq!(
            mode_level_name(ModeKind::Survival, 10),
            Some("SURVIVAL_POOL_ENDLESS")
        );
        assert_eq!(
            mode_level_name(ModeKind::IZombie, 9),
            Some("I_ZOMBIE_ENDLESS")
        );
        assert_eq!(mode_level_name(ModeKind::Vasebreaker, 10), None);
        assert_eq!(mode_level_scene(ModeKind::MiniGame, 7), SceneKind::Pool);
        assert_eq!(mode_level_scene(ModeKind::ZenGarden, 3), SceneKind::Garden);
    }

    #[test]
    fn mode_identity_survives_game_and_replay_initialization() {
        let game = Game::new_mode(7, ModeKind::Survival, 2);
        assert_eq!(game.state().mode, ModeKind::Survival);
        assert_eq!(game.state().level, 2);
        assert_eq!(game.state().scene, SceneKind::Pool);

        let replay = Replay::new_mode(7, ModeKind::IZombie, 9);
        let outcome = replay.run().unwrap();
        assert_eq!(outcome.final_state.mode, ModeKind::IZombie);
        assert_eq!(outcome.final_state.level, 9);
        assert_eq!(outcome.final_state.scene, SceneKind::Night);
    }

    #[test]
    fn plant_firing_events_match_source_fire_calls() {
        let mut game = Game::new(0, SceneKind::Night);
        let events = game.debug_prepare_plant_firing_audio();
        let fired_types = events
            .iter()
            .filter_map(|event| match event {
                GameEvent::PlantFired { plant_type, .. } => Some(*plant_type),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            fired_types,
            [
                PlantType::Peashooter,
                PlantType::Other(5),
                PlantType::Other(8),
                PlantType::Other(10),
                PlantType::Other(29),
            ]
        );
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event, GameEvent::ProjectileFired { .. }))
                .count(),
            9
        );

        let mut threepeater_events = Vec::new();
        game.fire_projectiles(99, PlantType::Other(18), 2, 2, &mut threepeater_events);
        assert_eq!(
            threepeater_events
                .iter()
                .filter(|event| matches!(event, GameEvent::PlantFired { .. }))
                .count(),
            3
        );
        assert_eq!(
            threepeater_events
                .iter()
                .filter(|event| matches!(event, GameEvent::ProjectileFired { .. }))
                .count(),
            3
        );
    }

    #[test]
    fn imitater_morphs_after_target_delay_and_uses_target_behavior() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 100;
        let placement = game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 48 },
                InputAction::PlantImitater {
                    plant_slot: 0,
                    row: 2,
                    column: 2,
                },
            ],
        });
        let imitater = game.state.board.plants[0].id;
        assert!(placement.iter().any(|event| matches!(
            event,
            GameEvent::PlantPlaced {
                entity,
                plant_type: PlantType::Other(48),
                sun_remaining: 0,
                ..
            } if *entity == imitater
        )));
        assert_eq!(
            game.state.board.plants[0].imitater_type,
            Some(PlantType::Peashooter)
        );
        assert_eq!(
            game.state.board.plants[0].special_counter,
            IMITATER_MORPH_TICKS - 1
        );

        let mut morph_events = Vec::new();
        for _ in 0..IMITATER_MORPH_TICKS {
            morph_events.extend(game.advance(InputFrame::default()));
        }
        assert!(morph_events.iter().any(|event| matches!(
            event,
            GameEvent::ImitaterMorphed {
                entity,
                plant_type: PlantType::Peashooter,
            } if *entity == imitater
        )));
        assert_eq!(game.state.board.plants[0].plant_type, PlantType::Peashooter);

        let mut setup_events = Vec::new();
        game.spawn_normal_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup_events);
        game.state.board.plants[0].launch_counter = 1;
        let events = (0..40)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileFired {
                source,
                projectile_type: ProjectileType::Pea,
                ..
            } if *source == imitater
        )));
    }

    #[test]
    fn debug_spikeweed_checkpoint_emits_attack_audio_event() {
        let mut game = Game::new(0, SceneKind::Day);
        game.debug_prepare_spikeweed();
        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantSpecialTriggered {
                plant_type: PlantType::Other(21),
                ..
            }
        )));
    }

    #[test]
    fn debug_digger_checkpoint_emits_surface_audio_event() {
        let mut game = Game::new(0, SceneKind::Day);
        game.debug_prepare_digger();
        let events = game.advance(InputFrame::default());
        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::DiggerSurfaced { .. }))
        );
    }

    #[test]
    fn debug_magnet_checkpoint_emits_steal_audio_event() {
        let mut game = Game::new(0, SceneKind::Night);
        game.debug_prepare_magnet();
        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::MetalStolen {
                zombie: Some(_),
                ..
            }
        )));
    }

    #[test]
    fn debug_zamboni_checkpoint_emits_vehicle_audio_event() {
        let mut game = Game::new(0, SceneKind::Day);
        game.debug_prepare_zamboni();
        let events = (0..100)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::VehicleDisabled { .. }))
        );
    }

    #[test]
    fn debug_catapult_checkpoint_emits_basketball_audio_event() {
        let mut game = Game::new(0, SceneKind::Day);
        let events = game.debug_prepare_catapult();
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileFired {
                projectile_type: ProjectileType::Other(1),
                ..
            }
        )));
    }

    #[test]
    fn projectile_impact_audio_uses_live_armor_and_no_duplicate_cob_hit() {
        let mut game = Game::new(0, SceneKind::Day);
        let butter_events = game.debug_prepare_butter();
        assert!(butter_events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileImpact {
                kind: ProjectileImpactSound::Butter,
                zombie: Some(_),
                ..
            }
        )));

        let projectile = ProjectileState {
            id: 1,
            projectile_type: ProjectileType::Melon,
            motion: ProjectileMotion::Straight,
            row: 2,
            position_x: 0,
            position_y: 0,
            velocity_x: 0,
            velocity_y: 0,
            damage: ProjectileType::Melon.damage(),
            age: 0,
            target_x: None,
            target_row: None,
            lob_height: 0,
            lob_velocity: 0,
        };
        let mut setup = Vec::new();
        let cone = game.spawn_conehead_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        let cone = game
            .state
            .board
            .zombies
            .iter()
            .find(|zombie| zombie.id == cone)
            .unwrap()
            .clone();
        let mut events = Vec::new();
        game.emit_projectile_impact(&projectile, Some(&cone), &mut events);
        assert_eq!(
            events
                .iter()
                .map(|event| match event {
                    GameEvent::ProjectileImpact { kind, .. } => *kind,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>(),
            vec![ProjectileImpactSound::Melon, ProjectileImpactSound::Plastic]
        );

        let mut dropped = cone;
        dropped.armor_intact = false;
        events.clear();
        game.emit_projectile_impact(
            &ProjectileState {
                projectile_type: ProjectileType::Pea,
                ..projectile
            },
            Some(&dropped),
            &mut events,
        );
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileImpact {
                kind: ProjectileImpactSound::Splat,
                ..
            }
        )));
    }

    #[test]
    fn debug_projectile_impact_checkpoint_covers_source_audio_families() {
        let mut game = Game::new(0, SceneKind::Day);
        let events = game.debug_prepare_projectile_impacts();
        for kind in [
            ProjectileImpactSound::Splat,
            ProjectileImpactSound::Kernel,
            ProjectileImpactSound::Butter,
            ProjectileImpactSound::Melon,
            ProjectileImpactSound::Ignite,
            ProjectileImpactSound::Shield,
            ProjectileImpactSound::Plastic,
        ] {
            assert!(events.iter().any(|event| matches!(
                event,
                GameEvent::ProjectileImpact { kind: actual, .. } if *actual == kind
            )));
        }
    }

    #[test]
    fn cob_cannon_combines_kernel_pults_and_hits_a_selected_target() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 1_000;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 34 },
                InputAction::Plant { row: 2, column: 1 },
            ],
        });
        game.state.board.seed_packets[34].refresh_remaining = 0;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 34 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let combine_events = game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 47 },
                InputAction::Plant { row: 2, column: 1 },
            ],
        });
        let cannon = game
            .state
            .board
            .plants
            .iter()
            .find(|plant| plant.plant_type == PlantType::Other(47))
            .unwrap()
            .id;
        assert_eq!(game.state.sun, 300);
        assert_eq!(game.state.board.plants.len(), 1);
        assert!(combine_events.iter().any(|event| matches!(
            event,
            GameEvent::PlantCombined {
                entity,
                plant_type: PlantType::Other(47),
                ..
            } if *entity == cannon
        )));

        for _ in 0..COB_ARM_TICKS {
            game.advance(InputFrame::default());
        }
        assert!(game.state.board.plants[0].special_armed);

        let mut setup_events = Vec::new();
        let primary = game.spawn_normal_zombie(2, 0, Some(grid_x(4)), &mut setup_events);
        let adjacent = game.spawn_normal_zombie(
            3,
            0,
            Some(grid_x(4) - 20 * POSITION_SCALE),
            &mut setup_events,
        );
        let fire_events = game.advance(InputFrame {
            actions: vec![InputAction::FireCobCannon {
                entity: cannon,
                row: 2,
                column: 4,
            }],
        });
        assert!(fire_events.iter().any(|event| matches!(
            event,
            GameEvent::CobCannonFired {
                entity,
                target_row: 2,
                target_column: 4,
            } if *entity == cannon
        )));
        let mut impact_events = Vec::new();
        for _ in 0..250 {
            impact_events.extend(game.advance(InputFrame::default()));
            if game.state.board.projectiles.is_empty() {
                break;
            }
        }
        assert!(impact_events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileHit { zombie, damage: 1_800, .. } if *zombie == primary
        )));
        assert!(impact_events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileSplashHit { zombie, damage: 1_800, .. } if *zombie == adjacent
        )));
        assert_eq!(
            impact_events
                .iter()
                .filter(|event| matches!(
                    event,
                    GameEvent::ProjectileImpact {
                        zombie: None,
                        kind: ProjectileImpactSound::Splat,
                        ..
                    }
                ))
                .count(),
            1
        );
        assert!(game.state.board.zombies.is_empty());
    }

    #[test]
    fn garden_services_have_dedicated_state_and_inputs() {
        let services = [
            (0, GardenServiceKind::Zen, Some(PlantType::Sunflower)),
            (1, GardenServiceKind::Mushroom, Some(PlantType::Other(8))),
            (2, GardenServiceKind::Aquarium, Some(PlantType::Other(24))),
            (3, GardenServiceKind::TreeOfWisdom, None),
        ];

        for (level, service, plant_type) in services {
            let mut game = Game::new_mode(7, ModeKind::ZenGarden, level);
            assert_eq!(game.state().scene, SceneKind::Garden);
            assert_eq!(game.state().garden_service, Some(service));
            assert_eq!(game.state().board.wave.total, 0);
            assert!(game.state().board.mowers.is_empty());

            if let Some(plant_type) = plant_type {
                assert_eq!(game.state().garden.plants[0].plant_type, plant_type);
                let events = game.advance(InputFrame {
                    actions: vec![
                        InputAction::GardenWater { plant: 0 },
                        InputAction::GardenFertilize { plant: 0 },
                        InputAction::GardenFulfillNeed { plant: 0 },
                    ],
                });
                assert!(
                    events
                        .iter()
                        .any(|event| matches!(event, GameEvent::GardenWatered { plant: 0, .. }))
                );
                assert!(events.iter().any(|event| matches!(
                    event,
                    GameEvent::GardenFertilized {
                        plant: 0,
                        age_ticks: 100
                    }
                )));
                assert!(events.iter().any(|event| matches!(
                    event,
                    GameEvent::GardenBecameHappy { plant: 0, aquatic }
                        if *aquatic == (service == GardenServiceKind::Aquarium)
                )));
                assert!(game.state().garden.plants[0].watered);
                assert!(game.state().garden.plants[0].happy);
                assert!(game.state().garden.plants[0].age_ticks > 100);

                let repeated = game.advance(InputFrame {
                    actions: vec![InputAction::GardenFulfillNeed { plant: 0 }],
                });
                assert!(
                    !repeated
                        .iter()
                        .any(|event| matches!(event, GameEvent::GardenBecameHappy { .. }))
                );
            } else {
                assert!(game.state().garden.plants.is_empty());
                let events = game.advance(InputFrame {
                    actions: vec![InputAction::GardenFeedTree],
                });
                assert!(
                    events
                        .iter()
                        .any(|event| matches!(event, GameEvent::GardenTreeGrew { height: 2 }))
                );
                assert_eq!(game.state().tree_height, 2);
            }

            let events = game.advance(InputFrame {
                actions: vec![InputAction::GardenLeave],
            });
            assert_eq!(game.state().scene, SceneKind::AdventureSelect);
            assert!(
                events
                    .iter()
                    .any(|event| matches!(event, GameEvent::GardenLeft))
            );
        }
    }

    #[test]
    fn source_defined_minigame_inputs_use_deterministic_targets() {
        let mut slot_machine = Game::new_mode(7, ModeKind::MiniGame, 2);
        assert_eq!(
            slot_machine.state().challenge.kind,
            ChallengeKind::SlotMachine
        );
        let events = slot_machine.advance(InputFrame {
            actions: vec![InputAction::ChallengeSpin],
        });
        assert_eq!(slot_machine.state().sun, 25);
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ChallengeAction {
                kind: ChallengeKind::SlotMachine,
                value: 0..=2
            }
        )));

        let mut beghouled = Game::new_mode(7, ModeKind::MiniGame, 4);
        beghouled.state.challenge.score = 74;
        let events = beghouled.advance(InputFrame {
            actions: vec![InputAction::ChallengeMatch { length: 3 }],
        });
        assert_eq!(beghouled.state().challenge.score, 75);
        assert_eq!(beghouled.state().scene, SceneKind::Complete);
        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::GameWon))
        );

        let mut aquarium = Game::new_mode(7, ModeKind::MiniGame, 7);
        let before = aquarium.state().board.zombies.len();
        let events = aquarium.advance(InputFrame {
            actions: vec![InputAction::ChallengeFeed { x: 400, y: 250 }],
        });
        assert_eq!(aquarium.state().sun, 45);
        assert_eq!(aquarium.state().board.zombies.len(), before + 1);
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ChallengeAction {
                kind: ChallengeKind::Zombiquarium,
                value: 5
            }
        )));

        let mut whack = Game::new_mode(7, ModeKind::MiniGame, 14);
        let mut setup = Vec::new();
        whack.spawn_normal_zombie(2, 0, Some(grid_x(2)), &mut setup);
        let events = whack.advance(InputFrame {
            actions: vec![InputAction::ChallengeWhack { row: 2, column: 2 }],
        });
        assert!(whack.state().board.zombies.is_empty());
        assert!(
            events
                .iter()
                .any(|event| matches!(event, GameEvent::ZombieDied { .. }))
        );
    }

    #[test]
    fn fixed_seed_banks_match_source_modes() {
        let profiles = [
            (
                2,
                vec![
                    PlantType::Sunflower,
                    PlantType::Peashooter,
                    PlantType::Other(5),
                ],
            ),
            (3, Vec::new()),
            (4, Vec::new()),
            (
                7,
                vec![
                    PlantType::ZombiquariumSnorkel,
                    PlantType::ZombiquariumTrophy,
                ],
            ),
            (8, Vec::new()),
            (
                14,
                vec![
                    PlantType::Other(4),
                    PlantType::Other(11),
                    PlantType::Other(14),
                ],
            ),
        ];
        for (level, expected) in profiles {
            let game = Game::new_mode(7, ModeKind::MiniGame, level);
            assert_eq!(
                game.state()
                    .board
                    .seed_packets
                    .iter()
                    .map(|packet| packet.plant_type)
                    .collect::<Vec<_>>(),
                expected,
                "level {level}"
            );
            assert!(
                game.state()
                    .board
                    .seed_packets
                    .iter()
                    .enumerate()
                    .all(|(slot, packet)| packet.slot == slot as u8)
            );
        }

        let vasebreaker = Game::new_mode(7, ModeKind::Vasebreaker, 0);
        assert_eq!(
            vasebreaker.state().board.seed_packets[0].plant_type,
            PlantType::Other(2)
        );

        let mut slot_machine = Game::new_mode(7, ModeKind::MiniGame, 2);
        let events = slot_machine.advance(InputFrame {
            actions: vec![InputAction::SelectSeed { slot: 0 }],
        });
        assert_eq!(slot_machine.state().board.selected_seed, None);
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ChallengeAction {
                kind: ChallengeKind::SlotMachine,
                value: 0..=2
            }
        )));

        let mut aquarium = Game::new_mode(7, ModeKind::MiniGame, 7);
        let events = aquarium.advance(InputFrame {
            actions: vec![InputAction::SelectSeed { slot: 0 }],
        });
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::InputRejected {
                reason: InputRejectReason::InvalidSlot,
                ..
            }
        )));
    }

    #[test]
    fn bobsled_and_pogo_party_use_source_wave_profiles() {
        let mut bobsled = Game::new_mode(7, ModeKind::MiniGame, 12);
        assert_eq!(bobsled.state().scene, SceneKind::Pool);
        assert_eq!(
            bobsled.state().challenge.kind,
            ChallengeKind::BobsledBonanza
        );
        assert_eq!(bobsled.state().board.wave.total, 30);
        assert_eq!(bobsled.state().board.wave.countdown, 4_500);
        assert_eq!(bobsled.state().board.wave.countdown_start, 4_500);
        bobsled.state.board.wave.countdown = 1;

        let events = bobsled.advance(InputFrame::default());
        assert_eq!(bobsled.state().board.wave.current, 1);
        // No row has an ice trail yet, so the wave leads with a Zomboni.
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(
                    event,
                    GameEvent::ZombieSpawned {
                        zombie_type: ZombieType::Zamboni,
                        wave: 0,
                        ..
                    }
                ))
                .count(),
            1
        );
        assert!(
            bobsled
                .state()
                .board
                .zombies
                .iter()
                .all(|zombie| zombie.zombie_type != ZombieType::Bobsled)
        );

        // With a live trail the next wave spawns the four-zombie team on the
        // iced row.
        bobsled.state.board.ice_timer[1] = u32::MAX;
        bobsled.state.board.ice_min_x[1] = 400 * POSITION_SCALE;
        bobsled.state.board.wave.countdown = 1;
        let events = bobsled.advance(InputFrame::default());
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(
                    event,
                    GameEvent::ZombieSpawned {
                        zombie_type: ZombieType::Bobsled,
                        wave: 1,
                        ..
                    }
                ))
                .count(),
            4
        );
        assert!(
            bobsled
                .state()
                .board
                .zombies
                .iter()
                .filter(|zombie| zombie.zombie_type == ZombieType::Bobsled)
                .all(|zombie| zombie.row == 1)
        );

        let mut pogo = Game::new_mode(7, ModeKind::MiniGame, 18);
        assert_eq!(pogo.state().scene, SceneKind::Roof);
        assert_eq!(pogo.state().challenge.kind, ChallengeKind::PogoParty);
        assert_eq!(pogo.state().board.wave.total, 30);
        assert_eq!(pogo.state().board.wave.countdown, 5_500);
        assert_eq!(pogo.state().board.wave.countdown_start, 5_500);
        pogo.state.board.wave.countdown = 1;

        let events = pogo.advance(InputFrame::default());
        assert_eq!(pogo.state().board.wave.current, 1);
        assert_eq!(
            pogo.state()
                .board
                .zombies
                .iter()
                .filter(|zombie| zombie.zombie_type == ZombieType::Pogo)
                .count(),
            1
        );
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieSpawned {
                zombie_type: ZombieType::Pogo,
                wave: 0,
                ..
            }
        )));
    }

    #[test]
    fn war_and_peas_modes_use_source_wave_profiles_and_head_pools() {
        let mut first = Game::new_mode(7, ModeKind::MiniGame, 0);
        assert_eq!(first.state().scene, SceneKind::Day);
        assert_eq!(first.state().challenge.kind, ChallengeKind::WarAndPeas);
        assert_eq!(first.state().board.wave.total, 20);
        first.state.board.wave.countdown = 1;
        let events = first.advance(InputFrame::default());
        assert_eq!(first.state().board.wave.current, 1);
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieSpawned {
                zombie_type: ZombieType::PeaHead,
                wave: 0,
                ..
            }
        )));

        let mut second = Game::new_mode(7, ModeKind::MiniGame, 16);
        assert_eq!(second.state().scene, SceneKind::Pool);
        assert_eq!(second.state().challenge.kind, ChallengeKind::WarAndPeas2);
        assert_eq!(second.state().board.wave.total, 30);
        second.state.board.wave.countdown = 1;
        let events = second.advance(InputFrame::default());
        assert_eq!(second.state().board.wave.current, 1);
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieSpawned {
                zombie_type: ZombieType::PeaHead,
                wave: 0,
                ..
            }
        )));
        second.state.board.wave.countdown = 1;
        let events = second.advance(InputFrame::default());
        assert_eq!(second.state().board.wave.current, 2);
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieSpawned {
                zombie_type: ZombieType::WallnutHead,
                wave: 1,
                ..
            }
        )));
    }

    #[test]
    fn wallnut_bowling_and_last_stand_use_source_stage_profiles() {
        let first = Game::new_mode(7, ModeKind::MiniGame, 1);
        assert_eq!(first.state().scene, SceneKind::Day);
        assert_eq!(first.state().challenge.kind, ChallengeKind::WallnutBowling);
        assert_eq!(first.state().board.wave.total, 20);

        let extreme = Game::new_mode(7, ModeKind::MiniGame, 17);
        assert_eq!(extreme.state().scene, SceneKind::Pool);
        assert_eq!(
            extreme.state().challenge.kind,
            ChallengeKind::WallnutBowling
        );
        assert_eq!(extreme.state().board.wave.total, 30);

        let last_stand = Game::new_mode(7, ModeKind::MiniGame, 15);
        assert_eq!(last_stand.state().scene, SceneKind::Pool);
        assert_eq!(last_stand.state().challenge.kind, ChallengeKind::LastStand);
        assert_eq!(last_stand.state().board.wave.total, 10);
        assert_eq!(last_stand.state().sun, 5_000);
        assert_eq!(last_stand.state().challenge.target, 5);
    }

    #[test]
    fn remaining_minigame_profiles_match_source_scenes_and_wave_counts() {
        let profiles = [
            (2, ChallengeKind::SlotMachine, SceneKind::Day, 0),
            (3, ChallengeKind::RainingSeeds, SceneKind::Fog, 40),
            (4, ChallengeKind::Beghouled, SceneKind::Night, 0),
            (5, ChallengeKind::Invisighoul, SceneKind::Fog, 20),
            (6, ChallengeKind::SeeingStars, SceneKind::Day, 40),
            (7, ChallengeKind::Zombiquarium, SceneKind::Pool, 0),
            (8, ChallengeKind::BeghouledTwist, SceneKind::Night, 0),
            (9, ChallengeKind::LittleTrouble, SceneKind::Pool, 30),
            (10, ChallengeKind::PortalCombat, SceneKind::Night, 20),
            (11, ChallengeKind::Column, SceneKind::Roof, 30),
            (13, ChallengeKind::ZombiesOnSpeed, SceneKind::Pool, 40),
            (14, ChallengeKind::WhackAZombie, SceneKind::Night, 12),
            (19, ChallengeKind::FinalBoss, SceneKind::Boss, 40),
        ];
        for (level, kind, scene, total_waves) in profiles {
            let game = Game::new_mode(7, ModeKind::MiniGame, level);
            assert_eq!(game.state().challenge.kind, kind, "level {level}");
            assert_eq!(game.state().scene, scene, "level {level}");
            assert_eq!(game.state().board.wave.total, total_waves, "level {level}");
        }

        let mut regular = Game::new(7, SceneKind::Pool);
        let mut setup = Vec::new();
        regular.spawn_normal_zombie(2, 0, Some(grid_x(4)), &mut setup);
        let regular_speed = regular.state().board.zombies[0].speed;
        let mut fast = Game::new_mode(7, ModeKind::MiniGame, 13);
        fast.spawn_normal_zombie(2, 0, Some(grid_x(4)), &mut setup);
        assert_eq!(fast.state().board.zombies[0].speed, regular_speed * 2);

        let boss = Game::new_mode(7, ModeKind::MiniGame, 19);
        assert_eq!(boss.state().board.zombies.len(), 1);
        assert_eq!(boss.state().board.zombies[0].zombie_type, ZombieType::Boss);
        assert_eq!(boss.state().board.zombies[0].health, BOSS_CHALLENGE_HEALTH);
    }

    #[test]
    fn conveyor_minigames_use_source_seed_belts() {
        let profiles = [
            (1, vec![PlantType::Other(3)], 400),
            (5, vec![PlantType::Peashooter, PlantType::Other(14)], 1_000),
            (9, Vec::new(), 200),
            (10, Vec::new(), 0),
            (
                11,
                vec![
                    PlantType::Other(4),
                    PlantType::Other(23),
                    PlantType::Other(39),
                    PlantType::Other(31),
                    PlantType::Other(35),
                    PlantType::Other(39),
                ],
                1_000,
            ),
            (
                19,
                vec![
                    PlantType::Other(32),
                    PlantType::Other(20),
                    PlantType::Other(32),
                    PlantType::Other(14),
                ],
                1_000,
            ),
        ];
        for (level, expected, countdown) in profiles {
            let game = Game::new_mode(7, ModeKind::MiniGame, level);
            assert_eq!(
                game.state()
                    .board
                    .seed_packets
                    .iter()
                    .map(|packet| packet.plant_type)
                    .collect::<Vec<_>>(),
                expected,
                "level {level}"
            );
            assert_eq!(game.state().challenge.conveyor_countdown, countdown);
            assert!(
                game.state()
                    .board
                    .seed_packets
                    .iter()
                    .enumerate()
                    .all(|(slot, packet)| packet.slot == slot as u8)
            );
            assert!(
                !game
                    .state()
                    .board
                    .seed_packets
                    .iter()
                    .any(|packet| { packet.plant_type == PlantType::Sunflower })
            );
        }

        assert_eq!(conveyor_interval(ChallengeKind::WallnutBowling, 4), 400);
        assert_eq!(conveyor_interval(ChallengeKind::WallnutBowling, 5), 425);
        assert_eq!(conveyor_interval(ChallengeKind::WallnutBowling, 7), 500);
        assert_eq!(conveyor_interval(ChallengeKind::PortalCombat, 4), 600);
        assert_eq!(conveyor_interval(ChallengeKind::Invisighoul, 4), 800);
        assert_eq!(conveyor_interval(ChallengeKind::Column, 4), 1_200);
        assert_eq!(conveyor_interval(ChallengeKind::FinalBoss, 4), 350);

        let mut bowling = Game::new_mode(7, ModeKind::MiniGame, 1);
        let events = bowling.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        assert_eq!(bowling.state().sun, 50);
        assert!(bowling.state().board.seed_packets.is_empty());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantPlaced {
                plant_type: PlantType::Other(3),
                sun_remaining: 50,
                ..
            }
        )));

        bowling.state.challenge.conveyor_countdown = 1;
        bowling.advance(InputFrame::default());
        assert_eq!(bowling.state().board.seed_packets.len(), 1);
        assert!(matches!(
            bowling.state().board.seed_packets[0].plant_type,
            PlantType::Other(3) | PlantType::Other(49)
        ));
    }

    #[test]
    fn izombie_levels_use_deterministic_source_layouts() {
        let expected_counts = [20, 20, 20, 20, 20, 25, 25, 25, 30, 25];
        let expected_packets = [
            vec![
                ZombieType::Normal,
                ZombieType::Buckethead,
                ZombieType::Football,
            ],
            vec![
                ZombieType::Normal,
                ZombieType::ScreenDoor,
                ZombieType::Buckethead,
            ],
            vec![
                ZombieType::Normal,
                ZombieType::Buckethead,
                ZombieType::Digger,
            ],
            vec![
                ZombieType::Normal,
                ZombieType::Buckethead,
                ZombieType::Ladder,
            ],
            vec![
                ZombieType::Normal,
                ZombieType::Buckethead,
                ZombieType::Bungee,
                ZombieType::Balloon,
            ],
            vec![
                ZombieType::Normal,
                ZombieType::PoleVaulter,
                ZombieType::Buckethead,
                ZombieType::Gargantuar,
            ],
            vec![
                ZombieType::Normal,
                ZombieType::PoleVaulter,
                ZombieType::Buckethead,
                ZombieType::Dancer,
            ],
            vec![
                ZombieType::Imp,
                ZombieType::Conehead,
                ZombieType::Buckethead,
                ZombieType::Bungee,
                ZombieType::Digger,
                ZombieType::Ladder,
            ],
            vec![
                ZombieType::Imp,
                ZombieType::Conehead,
                ZombieType::PoleVaulter,
                ZombieType::Buckethead,
                ZombieType::Bungee,
                ZombieType::Digger,
                ZombieType::Ladder,
                ZombieType::Football,
            ],
            vec![
                ZombieType::Imp,
                ZombieType::Conehead,
                ZombieType::PoleVaulter,
                ZombieType::Buckethead,
                ZombieType::Bungee,
                ZombieType::Digger,
                ZombieType::Ladder,
                ZombieType::Football,
                ZombieType::Dancer,
            ],
        ];
        for (level, expected_count) in expected_counts.into_iter().enumerate() {
            let game = Game::new_mode(7, ModeKind::IZombie, level as u8);
            let plants = &game.state().board.plants;
            assert_eq!(plants.len(), expected_count, "level {level}");
            assert!(game.state().board.seed_packets.is_empty());
            assert_eq!(game.state().board.zombie_packets, expected_packets[level]);
            assert!(plants.iter().all(|plant| {
                plant.column < izombie_columns(level as u8) && plant.row < DAY_ROWS
            }));
            for (index, plant) in plants.iter().enumerate() {
                assert!(
                    !plants[index + 1..]
                        .iter()
                        .any(|other| { (plant.row, plant.column) == (other.row, other.column) })
                );
            }
        }

        let level_one = Game::new_mode(7, ModeKind::IZombie, 0);
        assert!(level_one.state().board.plants.iter().any(|plant| {
            plant.plant_type == PlantType::Sunflower && (plant.row, plant.column) == (2, 3)
        }));
        let level_nine = Game::new_mode(7, ModeKind::IZombie, 8);
        assert!(level_nine.state().board.plants.iter().any(|plant| {
            plant.plant_type == PlantType::Other(23) && (plant.row, plant.column) == (1, 5)
        }));

        let mut restricted = Game::new_mode(7, ModeKind::IZombie, 0);
        let events = restricted.advance(InputFrame {
            actions: vec![InputAction::DeployZombie {
                zombie_type: ZombieType::Dancer,
                row: 0,
                column: 0,
            }],
        });
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::InputRejected {
                reason: InputRejectReason::InvalidSlot,
                ..
            }
        )));
    }

    #[test]
    fn survival_stage_completion_requires_a_seed_repick_until_the_final_stage() {
        let mut game = Game::new_mode(7, ModeKind::Survival, 0);
        game.state.board.wave.current = game.state.board.wave.total;

        let repick = game.advance(InputFrame::default());
        assert_eq!(game.state.scene, SceneKind::SeedChooser);
        assert_eq!(game.state.challenge.stage, 1);
        assert!(
            repick
                .iter()
                .any(|event| matches!(event, GameEvent::SurvivalRepickStarted { stage: 1 }))
        );

        let started = game.advance(InputFrame {
            actions: vec![InputAction::ConfirmSurvivalRepick],
        });
        assert_eq!(game.state.scene, SceneKind::Day);
        assert_eq!(game.state.board.wave.current, 0);
        assert!(
            started
                .iter()
                .any(|event| matches!(event, GameEvent::SurvivalStageStarted { stage: 1 }))
        );

        game.state.challenge.stage = 4;
        game.state.board.wave.current = game.state.board.wave.total;
        let won = game.advance(InputFrame::default());
        assert_eq!(game.state.scene, SceneKind::Complete);
        assert!(won.iter().any(|event| matches!(event, GameEvent::GameWon)));
    }

    #[test]
    fn survival_modes_use_their_stage_wave_profiles() {
        let normal = Game::new_mode(1, ModeKind::Survival, 0);
        assert_eq!(normal.state().board.wave.total, 10);
        assert!(!normal.state().board.wave.endless);

        let hard = Game::new_mode(1, ModeKind::Survival, 5);
        assert_eq!(hard.state().board.wave.total, 20);
        assert!(!hard.state().board.wave.endless);

        let endless = Game::new_mode(1, ModeKind::Survival, 10);
        assert_eq!(endless.state().scene, SceneKind::Pool);
        assert_eq!(endless.state().board.wave.total, 20);
        assert!(endless.state().board.wave.endless);
    }

    #[test]
    fn survival_catalog_uses_source_stage_profiles() {
        let expected_scenes = [
            SceneKind::Day,
            SceneKind::Night,
            SceneKind::Pool,
            SceneKind::Fog,
            SceneKind::Roof,
            SceneKind::Day,
            SceneKind::Night,
            SceneKind::Pool,
            SceneKind::Fog,
            SceneKind::Roof,
            SceneKind::Pool,
        ];
        for (level, expected_scene) in expected_scenes.into_iter().enumerate() {
            let game = Game::new_mode(1, ModeKind::Survival, level as u8);
            let expected_waves = if level < 5 { 10 } else { 20 };
            assert_eq!(game.state().scene, expected_scene, "level {level}");
            assert_eq!(game.state().level_scene, expected_scene, "level {level}");
            assert_eq!(
                game.state().board.wave.total,
                expected_waves,
                "level {level}"
            );
            assert_eq!(
                game.state().board.wave.endless,
                level == 10,
                "level {level}"
            );
        }
    }

    #[test]
    fn endless_survival_restarts_the_stage_wave_cycle() {
        let mut game = Game::new_mode(1, ModeKind::Survival, 10);
        game.state.board.wave.current = game.state.board.wave.total;

        let events = game.advance(InputFrame::default());

        assert_eq!(game.state.scene, SceneKind::Pool);
        assert_eq!(game.state.board.wave.current, 19);
        assert!(
            !events
                .iter()
                .any(|event| matches!(event, GameEvent::GameWon))
        );
    }

    #[test]
    fn vasebreaker_levels_use_source_defined_layouts() {
        let expected_counts = [25, 25, 25, 30, 35, 35, 44, 30, 35, 36];
        for (level, expected_count) in expected_counts.into_iter().enumerate() {
            let first = Game::new_mode(7, ModeKind::Vasebreaker, level as u8);
            let second = Game::new_mode(7, ModeKind::Vasebreaker, level as u8);
            let vases = &first.state().board.vases;
            assert_eq!(vases.len(), expected_count, "level {level}");
            assert_eq!(vases, &second.state().board.vases);
            assert!(
                vases
                    .iter()
                    .all(|vase| { vase.row < DAY_ROWS && vase.column < GRID_COLUMNS })
            );
        }
    }

    #[test]
    fn vasebreaker_is_seeded_and_reveals_contents_before_winning() {
        let first = Game::new_mode(7, ModeKind::Vasebreaker, 0);
        let second = Game::new_mode(7, ModeKind::Vasebreaker, 0);
        assert_eq!(first.state().board.wave.total, 0);
        assert_eq!(first.state().board.vases.len(), 25);
        assert_eq!(first.state().board.vases, second.state().board.vases);
        assert!(
            first
                .state()
                .board
                .vases
                .iter()
                .all(|vase| vase.column >= 4)
        );

        let mut game = first;
        let plant_vase = game
            .state()
            .board
            .vases
            .iter()
            .find(|vase| matches!(vase.contents, VaseContents::Plant(_)))
            .map(|vase| (vase.row, vase.column))
            .unwrap();
        let plant_events = game.advance(InputFrame {
            actions: vec![InputAction::BreakVase {
                row: plant_vase.0,
                column: plant_vase.1,
            }],
        });
        assert!(plant_events.iter().any(|event| matches!(
            event,
            GameEvent::VaseRevealed {
                contents: VaseContents::Plant(_),
                ..
            }
        )));
        assert_ne!(game.state().scene, SceneKind::Complete);

        let duplicate_events = game.advance(InputFrame {
            actions: vec![InputAction::BreakVase {
                row: plant_vase.0,
                column: plant_vase.1,
            }],
        });
        assert!(duplicate_events.iter().any(|event| matches!(
            event,
            GameEvent::InputRejected {
                reason: InputRejectReason::NoVase,
                ..
            }
        )));

        let zombie_vase = game
            .state()
            .board
            .vases
            .iter()
            .find(|vase| matches!(vase.contents, VaseContents::Zombie(_)))
            .map(|vase| (vase.row, vase.column))
            .unwrap();
        let zombie_events = game.advance(InputFrame {
            actions: vec![InputAction::BreakVase {
                row: zombie_vase.0,
                column: zombie_vase.1,
            }],
        });
        assert!(
            zombie_events
                .iter()
                .any(|event| matches!(event, GameEvent::ZombieSpawned { .. }))
        );
        assert_eq!(game.state().board.zombies.len(), 1);

        game.state.board.vases.clear();
        game.state.board.zombies.clear();
        let win_events = game.advance(InputFrame::default());
        assert_eq!(game.state().scene, SceneKind::Complete);
        assert!(
            win_events
                .iter()
                .any(|event| matches!(event, GameEvent::GameWon))
        );
    }

    #[test]
    fn izombie_deploys_for_sun_and_eats_a_brain_in_seventy_ticks() {
        let mut game = Game::new_mode(7, ModeKind::IZombie, 0);
        assert_eq!(game.state().scene, SceneKind::Night);
        assert_eq!(game.state().sun, 150);
        assert_eq!(game.state().board.brains.len(), 5);
        assert!(game.state().board.mowers.is_empty());
        assert!(!game.state().board.plants.is_empty());

        let deploy_events = game.advance(InputFrame {
            actions: vec![InputAction::DeployZombie {
                zombie_type: ZombieType::Normal,
                row: 0,
                column: 0,
            }],
        });
        assert_eq!(game.state().sun, 100);
        assert!(deploy_events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieDeployed {
                zombie_type: ZombieType::Normal,
                sun_remaining: 100,
                ..
            }
        )));

        game.state.board.zombies[0].position_x = -101 * POSITION_SCALE;
        let events = (0..I_ZOMBIE_BRAIN_TICKS)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::BrainEaten {
                row: 0,
                brains_remaining: 4,
                ..
            }
        )));
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::BrainFinished {
                row: 0,
                brains_remaining: 4,
                ..
            }
        )));
        assert!(game.state().board.brains[0].squished);
        assert_ne!(game.state().scene, SceneKind::Complete);
    }

    #[test]
    fn debug_brain_finished_checkpoint_emits_terminal_bite_event() {
        let mut game = Game::new_mode(0, ModeKind::IZombie, 0);
        game.debug_prepare_brain_finished();
        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::BrainFinished {
                row: 0,
                brains_remaining: 4,
                ..
            }
        )));
    }

    #[test]
    fn izombie_deploy_costs_match_the_source_card_prices() {
        // Plant::GetCost in the target build prices the I, Zombie seeds.
        let cases: [(u8, ZombieType, u32); 5] = [
            (1, ZombieType::ScreenDoor, 100),
            (4, ZombieType::Bungee, 125),
            (4, ZombieType::Balloon, 150),
            (5, ZombieType::Gargantuar, 300),
            (6, ZombieType::Dancer, 350),
        ];
        for (level, zombie_type, cost) in cases {
            let mut game = Game::new_mode(7, ModeKind::IZombie, level);
            game.state.sun = 1_000;
            let events = game.advance(InputFrame {
                actions: vec![InputAction::DeployZombie {
                    zombie_type,
                    row: 0,
                    column: 0,
                }],
            });
            let expected = 1_000 - cost;
            assert!(
                events.iter().any(|event| matches!(
                    event,
                    GameEvent::ZombieDeployed {
                        zombie_type: deployed,
                        sun_remaining,
                        ..
                    } if *deployed == zombie_type && *sun_remaining == expected
                )),
                "{zombie_type:?} on level {level} should cost {cost} sun"
            );
        }
    }

    #[test]
    fn izombie_deploys_use_the_source_conehead_dancer_and_imp_profiles() {
        // Zombie_Init in 1.0.0.1051: Conehead carries a 370-HP cone over the
        // 270-HP body and the Dancer body is 500; Zombie_ResetSpeed runs the
        // I, Zombie Imp at 0.9.
        let cases: [(u8, ZombieType, i32); 3] = [
            (7, ZombieType::Conehead, 640),
            (6, ZombieType::Dancer, 500),
            (7, ZombieType::Imp, 70),
        ];
        for (level, zombie_type, health) in cases {
            let mut game = Game::new_mode(7, ModeKind::IZombie, level);
            game.state.sun = 1_000;
            game.advance(InputFrame {
                actions: vec![InputAction::DeployZombie {
                    zombie_type,
                    row: 0,
                    column: 0,
                }],
            });
            let zombie = game
                .state()
                .board
                .zombies
                .iter()
                .find(|zombie| zombie.zombie_type == zombie_type)
                .expect("deployed zombie must exist");
            assert_eq!(
                zombie.health, health,
                "{zombie_type:?} deployed on level {level} should have {health} HP"
            );
            if zombie_type == ZombieType::Imp {
                assert_eq!(zombie.speed, 900_000);
            }
        }
    }

    #[test]
    fn pea_head_reloads_and_damages_a_plant_with_a_backward_zombie_pea() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 100;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 0 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let plant_id = game.state.board.plants[0].id;
        let mut setup_events = Vec::new();
        let zombie = game.spawn_pea_head_zombie(2, 0, Some(grid_x(4)), &mut setup_events);
        game.state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == zombie)
            .unwrap()
            .speed = 0;

        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == zombie)
                .unwrap()
                .pea_head_counter,
            ZOMBIE_PEA_HEAD_RELOAD_TICKS
        );
        let before_reload = (0..ZOMBIE_PEA_HEAD_RELOAD_TICKS - 1)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert!(!before_reload.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileFired {
                source,
                projectile_type: ProjectileType::ZombiePea,
                ..
            } if *source == zombie
        )));

        let fire_events = game.advance(InputFrame::default());
        assert!(fire_events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileFired {
                source,
                projectile_type: ProjectileType::ZombiePea,
                row: 2,
                ..
            } if *source == zombie
        )));
        assert_eq!(
            game.state.board.projectiles[0].motion,
            ProjectileMotion::Backwards
        );
        let hit = (0..70)
            .flat_map(|_| game.advance(InputFrame::default()))
            .any(|event| {
                matches!(
                    event,
                    GameEvent::PlantDamaged {
                        entity,
                        damage: 20,
                        health_remaining: 280,
                    } if entity == plant_id
                )
            });
        assert!(hit);
        assert_eq!(game.state.board.plants[0].health, 280);
    }

    #[test]
    fn catapult_zombie_lobs_a_basketball_at_the_leftmost_plant() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 100;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        game.state
            .board
            .seed_packets
            .iter_mut()
            .find(|packet| packet.slot == 1)
            .unwrap()
            .refresh_remaining = 0;
        game.state.sun = 50;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 1 },
            ],
        });
        let left_plant_id = game
            .state
            .board
            .plants
            .iter()
            .find(|plant| plant.column == 1)
            .unwrap()
            .id;
        let right_plant_id = game
            .state
            .board
            .plants
            .iter()
            .find(|plant| plant.column == 2)
            .unwrap()
            .id;
        let mut setup_events = Vec::new();
        let zombie = game.spawn_catapult_zombie(2, 0, Some(grid_x(4)), &mut setup_events);
        game.state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == zombie)
            .unwrap()
            .speed = 0;

        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == zombie)
                .unwrap()
                .health,
            850
        );
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == zombie)
                .unwrap()
                .catapult_shots,
            CATAPULT_SHOTS
        );

        let before_fire = (0..CATAPULT_LAUNCH_TICKS)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert!(!before_fire.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileFired {
                source,
                projectile_type: ProjectileType::Other(1),
                ..
            } if *source == zombie
        )));
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == zombie)
                .unwrap()
                .catapult_counter,
            1
        );

        let fire_events = game.advance(InputFrame::default());
        assert!(fire_events.iter().any(|event| matches!(
            event,
            GameEvent::ProjectileFired {
                source,
                projectile_type: ProjectileType::Other(1),
                row: 2,
                ..
            } if *source == zombie
        )));
        let projectile = game
            .state
            .board
            .projectiles
            .iter()
            .find(|projectile| projectile.projectile_type == ProjectileType::Other(1))
            .unwrap();
        assert_eq!(projectile.motion, ProjectileMotion::Lobbed);
        assert_eq!(projectile.damage, 75);
        assert_eq!(projectile.target_x, Some(grid_x(1)));

        let hit = (0..120)
            .flat_map(|_| game.advance(InputFrame::default()))
            .any(|event| {
                matches!(
                    event,
                    GameEvent::PlantDamaged {
                        entity,
                        damage: 75,
                        health_remaining: 225,
                    } if entity == left_plant_id
                )
            });
        assert!(hit);
        assert_eq!(
            game.state
                .board
                .plants
                .iter()
                .find(|plant| plant.id == left_plant_id)
                .unwrap()
                .health,
            225
        );
        assert_eq!(
            game.state
                .board
                .plants
                .iter()
                .find(|plant| plant.id == right_plant_id)
                .unwrap()
                .health,
            300
        );
    }

    #[test]
    fn pogo_zombie_bounces_over_a_plant_without_biting_it() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 50;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let plant_id = game.state.board.plants[0].id;
        let mut setup_events = Vec::new();
        let zombie = game.spawn_pogo_zombie(
            2,
            0,
            Some(grid_x(2) + 20 * POSITION_SCALE),
            &mut setup_events,
        );
        let pogo = game
            .state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == zombie)
            .unwrap();
        pogo.speed = 0;
        pogo.pogo_counter = 1;

        for _ in 0..4 {
            game.advance(InputFrame::default());
        }
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == zombie)
                .unwrap()
                .pogo_counter,
            POGO_BOUNCE_TICKS
        );
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == zombie)
                .unwrap()
                .pogo_target_x,
            Some(grid_x(2) - 80 * POSITION_SCALE)
        );
        assert_eq!(game.state.board.plants[0].health, 300);

        let events = (0..POGO_BOUNCE_TICKS)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event, GameEvent::PogoBounceSound { entity } if *entity == zombie))
                .count(),
            1
        );
        assert!(!events.iter().any(|event| matches!(
            event,
            GameEvent::PlantDamaged { entity, .. } if *entity == plant_id
        )));
        assert_eq!(game.state.board.plants[0].health, 300);
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == zombie)
                .unwrap()
                .position_x,
            grid_x(2) - 80 * POSITION_SCALE
        );
    }

    #[test]
    fn gargantuar_squishes_a_plant_and_damages_spikerock() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 50;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let plant_id = game.state.board.plants[0].id;
        let mut setup_events = Vec::new();
        let zombie = game.spawn_gargantuar_zombie(
            2,
            0,
            Some(grid_x(2) + 20 * POSITION_SCALE),
            &mut setup_events,
        );
        game.state.board.zombies[0].speed = 0;
        let events = (0..4)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantDied { entity } if *entity == plant_id
        )));
        assert!(game.state.board.plants.is_empty());
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == zombie)
                .unwrap()
                .health,
            3_000
        );

        game.state.sun = 125;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 46 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let spikerock = game.state.board.plants[0].id;
        let gargantuar = game
            .state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == zombie)
            .unwrap();
        gargantuar.position_x = grid_x(2) + 20 * POSITION_SCALE;
        gargantuar.age = 0;
        let events = (0..4)
            .flat_map(|_| game.advance(InputFrame::default()))
            .collect::<Vec<_>>();
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantDamaged {
                entity,
                damage: GARGANTUAR_SPIKEROCK_DAMAGE,
                health_remaining: 430,
            } if *entity == spikerock
        )));
        assert_eq!(game.state.board.plants[0].health, 430);
    }

    #[test]
    fn dancer_summons_four_backup_dancers_after_entrance() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup_events = Vec::new();
        let dancer = game.spawn_dancer_zombie(2, 0, Some(grid_x(2)), &mut setup_events);
        let leader = game
            .state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == dancer)
            .unwrap();
        leader.speed = 0;
        leader.dancer_counter = 1;
        let events = game.advance(InputFrame::default());
        let backups = game
            .state
            .board
            .zombies
            .iter()
            .filter(|candidate| candidate.zombie_type == ZombieType::BackupDancer)
            .collect::<Vec<_>>();
        assert_eq!(backups.len(), BACKUP_DANCER_COUNT);
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(
                    event,
                    GameEvent::ZombieSpawned {
                        zombie_type: ZombieType::BackupDancer,
                        ..
                    }
                ))
                .count(),
            BACKUP_DANCER_COUNT
        );
        assert_eq!(backups[0].row, 1);
        assert_eq!(backups[1].row, 3);
        assert_eq!(backups[2].position_x, grid_x(2) - 100 * POSITION_SCALE);
        assert_eq!(backups[3].position_x, grid_x(2) + 100 * POSITION_SCALE);
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == dancer)
                .unwrap()
                .health,
            500
        );
    }

    #[test]
    fn digger_surfaces_at_the_left_edge_before_attacking() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 50;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        let plant_id = game.state.board.plants[0].id;
        let mut setup_events = Vec::new();
        let digger = game.spawn_digger_zombie(2, 0, Some(5 * POSITION_SCALE), &mut setup_events);
        game.state.board.zombies[0].speed = 0;
        let mut events = game.advance(InputFrame::default());
        let digger_state = game
            .state
            .board
            .zombies
            .iter()
            .find(|candidate| candidate.id == digger)
            .unwrap();
        assert!(!digger_state.digger_underground);
        assert_eq!(digger_state.digger_counter, DIGGER_RISE_TICKS);
        for _ in 0..DIGGER_RISE_TICKS {
            events.extend(game.advance(InputFrame::default()));
        }
        assert!(events.iter().all(|event| !matches!(
            event,
            GameEvent::PlantDamaged { entity, .. } | GameEvent::PlantDied { entity }
                if *entity == plant_id
        )));
        let digger_state = game
            .state
            .board
            .zombies
            .iter()
            .find(|candidate| candidate.id == digger)
            .unwrap();
        assert!(!digger_state.digger_underground);
        assert_eq!(digger_state.digger_counter, 0);
        assert_eq!(game.state.board.plants[0].health, 300);
    }

    #[test]
    fn digger_tunnels_fast_and_walks_slow_after_rising() {
        // Zombie_ResetSpeed: PHASE_DIGGER_TUNNELING moves at 0.66-0.68 and the
        // surfaced walk re-picks 0.12, or 0.23 on I, Zombie levels.
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let digger = game.spawn_digger_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        let tunneling = game
            .state
            .board
            .zombies
            .iter()
            .find(|candidate| candidate.id == digger)
            .unwrap()
            .speed;
        assert!(
            (660_000..=680_000).contains(&tunneling),
            "tunneling speed {tunneling} should be in the 0.66-0.68 band"
        );

        game.state.board.zombies[0].position_x = 5 * POSITION_SCALE;
        game.advance(InputFrame::default());
        let surfaced = game
            .state
            .board
            .zombies
            .iter()
            .find(|candidate| candidate.id == digger)
            .unwrap();
        assert!(!surfaced.digger_underground);
        assert_eq!(surfaced.speed, DIGGER_WALK_SPEED);

        let mut izombie = Game::new_mode(7, ModeKind::IZombie, 2);
        izombie.state.sun = 1_000;
        izombie.advance(InputFrame {
            actions: vec![InputAction::DeployZombie {
                zombie_type: ZombieType::Digger,
                row: 0,
                column: 0,
            }],
        });
        let index = izombie
            .state
            .board
            .zombies
            .iter()
            .position(|candidate| candidate.zombie_type == ZombieType::Digger)
            .unwrap();
        izombie.state.board.zombies[index].position_x = 5 * POSITION_SCALE;
        izombie.advance(InputFrame::default());
        assert_eq!(
            izombie.state.board.zombies[index].speed,
            DIGGER_IZOMBIE_WALK_SPEED
        );
    }

    #[test]
    fn bungee_steals_a_plant_after_its_bottom_timer() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 50;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        let plant_id = game.state.board.plants[0].id;
        let mut setup_events = Vec::new();
        let bungee = game.spawn_bungee_zombie(2, 0, None, &mut setup_events);
        game.state.board.zombies[0].bungee_counter = 1;
        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantDied { entity } if *entity == plant_id
        )));
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieDied { entity } if *entity == bungee
        )));
        assert!(game.state.board.plants.is_empty());
        assert!(
            !game
                .state
                .board
                .zombies
                .iter()
                .any(|candidate| candidate.id == bungee)
        );
    }

    #[test]
    fn boss_zombotany_heads_and_gigagargantuar_use_source_health_profiles() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let spawned = [
            (
                game.spawn_boss_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup),
                ZombieType::Boss,
                BOSS_ADVENTURE_HEALTH,
                0,
            ),
            (
                game.spawn_wallnut_head_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup),
                ZombieType::WallnutHead,
                270,
                ZOMBOTANY_WALLNUT_HELM_HEALTH,
            ),
            (
                game.spawn_jalapeno_head_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup),
                ZombieType::JalapenoHead,
                ZOMBOTANY_JALAPENO_HEALTH,
                0,
            ),
            (
                game.spawn_gatling_head_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup),
                ZombieType::GatlingHead,
                270,
                0,
            ),
            (
                game.spawn_squash_head_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup),
                ZombieType::SquashHead,
                270,
                0,
            ),
            (
                game.spawn_tallnut_head_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup),
                ZombieType::TallnutHead,
                270,
                ZOMBOTANY_TALLNUT_HELM_HEALTH,
            ),
            (
                game.spawn_gigagargantuar_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup),
                ZombieType::Gigagargantuar,
                GIGAGARGANTUAR_HEALTH,
                0,
            ),
        ];

        for (entity, zombie_type, health, shield) in spawned {
            let zombie = game
                .state
                .board
                .zombies
                .iter()
                .find(|zombie| zombie.id == entity)
                .unwrap();
            assert_eq!(zombie.zombie_type, zombie_type);
            assert_eq!(zombie.health, health);
            assert_eq!(zombie.max_health, health);
            assert_eq!(zombie.shield_health, shield);
        }

        let mut challenge = Game::new_mode(7, ModeKind::MiniGame, 0);
        let boss = challenge.spawn_boss_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        assert_eq!(
            challenge
                .state
                .board
                .zombies
                .iter()
                .find(|zombie| zombie.id == boss)
                .unwrap()
                .health,
            BOSS_CHALLENGE_HEALTH
        );
    }

    #[test]
    fn jalapeno_head_burns_its_row_and_dies() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 50;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 0 },
            ],
        });
        let plant = game.state.board.plants[0].id;
        let mut setup = Vec::new();
        let zombie = game.spawn_jalapeno_head_zombie(
            2,
            0,
            Some(grid_x(2) + 20 * POSITION_SCALE),
            &mut setup,
        );
        let state = game
            .state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == zombie)
            .unwrap();
        state.speed = 0;
        state.special_counter = 1;

        let events = game.advance(InputFrame::default());
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantDied { entity } if *entity == plant
        )));
        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::ZombieDied { entity } if *entity == zombie
        )));
        assert!(
            !game
                .state
                .board
                .zombies
                .iter()
                .any(|candidate| candidate.id == zombie)
        );
        game.advance(InputFrame::default());
        assert!(game.state.board.plants.is_empty());
    }

    #[test]
    fn gatling_head_fires_a_four_pea_burst() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_gatling_head_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
        let state = game
            .state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == zombie)
            .unwrap();
        state.speed = 0;
        state.age = 3;
        state.special_counter = 1;

        let events = game.advance(InputFrame::default());
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(
                    event,
                    GameEvent::ProjectileFired {
                        source,
                        projectile_type: ProjectileType::ZombiePea,
                        ..
                    } if *source == zombie
                ))
                .count(),
            4
        );
        assert_eq!(game.state.board.projectiles.len(), 4);
    }

    #[test]
    fn squash_head_lands_on_a_plant_then_dies() {
        let mut game = Game::new(7, SceneKind::Day);
        game.state.sun = 50;
        game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 1 },
                InputAction::Plant { row: 2, column: 2 },
            ],
        });
        let plant = game.state.board.plants[0].id;
        let mut setup = Vec::new();
        let zombie =
            game.spawn_squash_head_zombie(2, 0, Some(grid_x(2) + 20 * POSITION_SCALE), &mut setup);
        let state = game
            .state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == zombie)
            .unwrap();
        state.speed = 0;
        state.age = 3;
        game.advance(InputFrame::default());
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == zombie)
                .unwrap()
                .special_phase,
            1
        );

        let state = game
            .state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == zombie)
            .unwrap();
        state.special_counter = 1;
        game.advance(InputFrame::default());
        let state = game
            .state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == zombie)
            .unwrap();
        state.special_counter = 1;
        let landing = game.advance(InputFrame::default());
        assert!(landing.iter().any(|event| matches!(
            event,
            GameEvent::PlantDied { entity } if *entity == plant
        )));
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|candidate| candidate.id == zombie)
                .unwrap()
                .special_phase,
            3
        );

        game.state
            .board
            .zombies
            .iter_mut()
            .find(|candidate| candidate.id == zombie)
            .unwrap()
            .special_counter = 1;
        let death = game.advance(InputFrame::default());
        assert!(death.iter().any(|event| matches!(
            event,
            GameEvent::ZombieDied { entity } if *entity == zombie
        )));
        assert!(
            !game
                .state
                .board
                .zombies
                .iter()
                .any(|candidate| candidate.id == zombie)
        );
    }
}
