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
const ZOMBOTANY_HEAD_RELOAD_TICKS: u32 = 150;
const ZOMBOTANY_WALLNUT_HELM_HEALTH: i32 = 1_100;
const ZOMBOTANY_TALLNUT_HELM_HEALTH: i32 = 2_200;
const ZOMBOTANY_JALAPENO_HEALTH: i32 = 500;
const ZOMBOTANY_SQUASH_RISE_TICKS: u32 = 95;
const ZOMBOTANY_SQUASH_FALL_TICKS: u32 = 10;
const ZOMBOTANY_SQUASH_DONE_TICKS: u32 = 100;
const ZOMBOTANY_SQUASH_DAMAGE: i32 = 1_800;
const GIGAGARGANTUAR_HEALTH: i32 = 6_000;
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
        _ => SceneKind::Day,
    }
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
    let capacity = usize::from(GRID_COLUMNS - excluded_columns) * usize::from(DAY_ROWS);
    let excluded_columns = if contents.len() > capacity {
        0
    } else {
        excluded_columns
    };
    let mut cells = (excluded_columns..GRID_COLUMNS)
        .flat_map(|column| (0..DAY_ROWS).map(move |row| (column, row)))
        .collect::<Vec<_>>();
    let leaf_count = match level.min(9) {
        0 => 0,
        2 => 3,
        _ => 2,
    };
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
            id: u32::try_from(index).unwrap_or(u32::MAX).saturating_add(1),
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
    ZombieFled {
        entity: EntityId,
    },
    MowerTriggered {
        row: u8,
    },
    ZombieHypnotized {
        entity: EntityId,
    },
    ZombieVaulted {
        entity: EntityId,
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
                self.update_seed_packets();
                if self.state.mode != ModeKind::IZombie {
                    self.update_sun_spawning(&mut events);
                }
                self.update_wave_spawning(&mut events);
                self.update_challenge(&mut events);
                self.state.board.ice_counter = self.state.board.ice_counter.saturating_sub(1);
                self.update_craters();
                self.state.tick = self.state.tick.saturating_add(1);
                self.state.wave = self.state.board.wave.current;

                let won = if self.state.mode == ModeKind::Vasebreaker {
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
        let cost = match zombie_type {
            ZombieType::Normal | ZombieType::Flag | ZombieType::Imp => 50,
            ZombieType::Conehead | ZombieType::PoleVaulter | ZombieType::Pogo => 75,
            ZombieType::Buckethead | ZombieType::ScreenDoor | ZombieType::Newspaper => 125,
            ZombieType::Football => 175,
            ZombieType::Jackbox | ZombieType::Dancer => 150,
            ZombieType::Bobsled | ZombieType::Ladder => 150,
            ZombieType::Gargantuar | ZombieType::Gigagargantuar => 250,
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
            ZombieType::Buckethead | ZombieType::ScreenDoor => 1_370,
            ZombieType::Football => 1_670,
            ZombieType::Digger => 370,
            ZombieType::Bungee => 450,
            ZombieType::PoleVaulter | ZombieType::Pogo => 500,
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
        let asleep = effective_type.is_nocturnal()
            && !matches!(self.state.scene, SceneKind::Night | SceneKind::Fog);
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
        let asleep =
            target.is_nocturnal() && !matches!(self.state.scene, SceneKind::Night | SceneKind::Fog);
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
            let mut tangle_grab_target = None;
            let mut tangle_started = false;
            let mut gold_magnet_coin = None;
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
                        }
                    }
                } else if plant_type.is_squash() {
                    if plant.special_armed {
                        plant.special_counter = plant.special_counter.saturating_sub(1);
                        if plant.special_counter == 0 {
                            squash_hit_target = plant.special_target.take().or(squash_target);
                            plant.health = 0;
                        }
                    } else if plant.special_target.is_some() {
                        plant.special_counter = plant.special_counter.saturating_sub(1);
                        if plant.special_counter == 0 {
                            plant.special_armed = true;
                            plant.special_counter = SQUASH_HIT_DELAY_TICKS;
                        }
                    } else if let Some(target) = squash_target {
                        plant.special_target = Some(target);
                        plant.special_counter = SQUASH_LOOK_TICKS;
                    }
                } else if plant_type.is_tangle_kelp() {
                    if plant.special_armed {
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
                    self.damage_zombie(zombie_index, PLANT_SPECIAL_DAMAGE);
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
                events.push(GameEvent::PlantSpecialTriggered {
                    entity: id,
                    plant_type,
                });
            }
            if spikeweed_started {
                events.push(GameEvent::PlantSpecialTriggered {
                    entity: id,
                    plant_type,
                });
            }
            if spikeweed_hit {
                self.apply_spikeweed_damage(id, row, column, events);
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
                let _vertical_motion = self.rng.next();
                let _horizontal_motion = self.rng.next();
                let _ground_offset = self.rng.range(20);
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
                self.damage_zombie(zombie_index, PLANT_SPECIAL_DAMAGE);
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

                    self.damage_zombie(zombie_index, ICE_SHROOM_DAMAGE);
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
        let center_x = grid_x(column);
        let target_ids = self
            .state
            .board
            .zombies
            .iter()
            .filter(|zombie| {
                zombie.health > 0
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
            self.damage_zombie(zombie_index, PLANT_SPECIAL_DAMAGE);
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

        // Damage plants within 115 unit radius, same row ±1 row.
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
            if dx > explosion_radius as u64 {
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
                self.damage_zombie(target_index, PLANT_SPECIAL_DAMAGE);
                if self.state.board.zombies[target_index].health <= 0 {
                    self.emit_zombie_died(target, events);
                }
            }
        }

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
        let zombie = &mut self.state.board.zombies[zombie_index];
        if zombie.zombie_type == ZombieType::Zamboni {
            zombie.speed = zamboni_speed(zombie.position_x);
        }
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
                    self.damage_zombie(zombie_index, ZOMBOTANY_SQUASH_DAMAGE);
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

    fn damage_zombie(&mut self, zombie_index: usize, damage: i32) {
        let mut remaining = damage.max(0);
        let zombie = &mut self.state.board.zombies[zombie_index];
        if zombie.zombie_type == ZombieType::Balloon && zombie.balloon_phase == BALLOON_FLYING_PHASE
        {
            let absorbed = remaining.min(zombie.balloon_flying_health);
            zombie.balloon_flying_health -= absorbed;
            remaining -= absorbed;
            if zombie.balloon_flying_health == 0 {
                zombie.balloon_phase = BALLOON_POPPING_PHASE;
                zombie.balloon_counter = BALLOON_POP_TICKS;
            }
        }
        let shield_damage = remaining.min(zombie.shield_health);
        zombie.shield_health -= shield_damage;
        remaining -= shield_damage;
        zombie.health -= remaining;
    }

    fn update_zombies(&mut self, events: &mut Vec<GameEvent>) {
        let zombie_count = self.state.board.zombies.len() as u32;
        for zombie_index in 0..self.state.board.zombies.len() {
            if self.state.board.zombies[zombie_index].health <= 0 {
                // Jack-in-the-Box: explode when killed by external damage.
                if self.state.board.zombies[zombie_index].zombie_type == ZombieType::Jackbox
                    && self.state.board.zombies[zombie_index].jackbox_timer > 0
                {
                    self.state.board.zombies[zombie_index].jackbox_timer = 0;
                    self.apply_jackbox_explosion(zombie_index, events);
                }
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
            if self.state.board.zombies[zombie_index].blowing_away {
                let entity = self.state.board.zombies[zombie_index].id;
                self.state.board.zombies[zombie_index].position_x += BLOWN_AWAY_SPEED;
                if self.state.board.zombies[zombie_index].position_x > BLOWN_AWAY_EDGE {
                    self.emit_zombie_died(entity, events);
                    self.state.board.zombies.remove(zombie_index);
                }
                continue;
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
                    zombie.yeti_counter = zombie.yeti_counter.saturating_sub(1);
                    if zombie.yeti_counter == 0 {
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
                let zombie = &mut self.state.board.zombies[zombie_index];
                if zombie.zombie_type == ZombieType::Digger {
                    if zombie.digger_underground && zombie.position_x <= 10 * POSITION_SCALE {
                        zombie.digger_underground = false;
                        zombie.digger_counter = DIGGER_RISE_TICKS;
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
                    let plant_id = self.state.board.plants.remove(plant_index).id;
                    events.push(GameEvent::PlantDied { entity: plant_id });
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
                    || (zombie.zombie_type == ZombieType::Bobsled && zombie.bobsled_sliding)
                    || zombie.zombie_type == ZombieType::Bungee)
                {
                    let base_speed = if zombie.yeti_running {
                        YETI_RUNNING_SPEED
                    } else if zombie.zombie_type == ZombieType::Newspaper
                        && zombie.health > 0
                        && zombie.health <= 270
                    {
                        660_000
                    } else {
                        zombie.speed
                    };
                    let speed = if zombie.chilled_counter == 0 {
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
                    self.state.board.zombies[zombie_index].eating =
                        target.is_some() && ztype != ZombieType::Pogo;
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
                            self.state.board.zombies[zombie_index].ladder_placed = true;
                            self.state.board.zombies[zombie_index].shield_health = 0;
                            self.state.board.zombies[zombie_index].eating = false;
                        } else if ztype == ZombieType::Pogo {
                            if !pogo_bouncing {
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
                            self.state.board.zombies[zombie_index].has_vaulted = true;
                            self.state.board.zombies[zombie_index].eating = false;
                            events.push(GameEvent::ZombieVaulted { entity });
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
                                        self.damage_zombie(target_idx, PLANT_SPECIAL_DAMAGE);
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
                let entity = self.state.board.zombies.remove(zombie_index).id;
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
    }

    fn apply_cob_explosion(
        &mut self,
        projectile: &ProjectileState,
        target_row: u8,
        target_x: i64,
        events: &mut Vec<GameEvent>,
    ) {
        let mut targets = self
            .state
            .board
            .zombies
            .iter()
            .filter(|zombie| {
                zombie.health > 0
                    && projectile_can_hit_zombie(zombie)
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
            self.damage_zombie(zombie_index, projectile.damage);
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
                        && projectile_can_hit_zombie(zombie)
                })
                .filter(|(_, zombie)| projectile_hits(projectile.position_x, zombie.position_x))
                .min_by_key(|(_, zombie)| zombie.position_x)
                .map(|(index, _)| index);

            if let Some(zombie_index) = target {
                let zombie_id = self.state.board.zombies[zombie_index].id;
                self.damage_zombie(zombie_index, projectile.damage);
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

    fn steer_homing_projectile(&mut self, projectile_index: usize) {
        let projectile = &self.state.board.projectiles[projectile_index];
        let current_row =
            projectile_row(projectile.position_y, self.state.board.rows).unwrap_or(projectile.row);
        let target_row = self
            .state
            .board
            .zombies
            .iter()
            .filter(|zombie| zombie.health > 0 && projectile_can_hit_zombie(zombie))
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
                    && projectile_can_hit_zombie(zombie)
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
            self.damage_zombie(zombie_index, splash_damage);
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
                    && projectile_can_hit_zombie(zombie)
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
            self.damage_zombie(zombie_index, projectile.damage);
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
                    && projectile_can_hit_zombie(zombie)
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
            self.damage_zombie(zombie_index, projectile.damage);
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

    fn update_seed_packets(&mut self) {
        for packet in &mut self.state.board.seed_packets {
            packet.refresh_remaining = packet.refresh_remaining.saturating_sub(1);
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
        let _ground_y = self.rng.range(250);
    }

    fn update_wave_spawning(&mut self, events: &mut Vec<GameEvent>) {
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
        self.state.board.wave.countdown = self.state.board.wave.countdown.saturating_sub(1);
        if self.state.board.wave.countdown != 0 {
            return;
        }

        let wave = self.state.board.wave.current;
        events.push(GameEvent::WaveStarted { wave });
        let row = self.rng.range(u32::from(self.state.board.rows)) as u8;
        match self.state.challenge.kind {
            ChallengeKind::BobsledBonanza => {
                self.spawn_bobsled_zombie(row, wave, None, events);
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
                self.spawn_normal_zombie(row, wave, None, events);
            }
        }
        self.state.board.wave.current += 1;
        self.state.board.wave.countdown_start = 0;
    }

    fn apply_spikeweed_damage(
        &mut self,
        plant_id: EntityId,
        row: u8,
        column: u8,
        events: &mut Vec<GameEvent>,
    ) {
        let mut zombie_index = 0;
        while zombie_index < self.state.board.zombies.len() {
            let zombie = &self.state.board.zombies[zombie_index];
            if zombie.health <= 0 || zombie.row != row || !spikeweed_hits(zombie.position_x, column)
            {
                zombie_index += 1;
                continue;
            }
            let zombie_id = zombie.id;
            self.damage_zombie(zombie_index, SPIKEWEED_DAMAGE);
            let health_remaining = self.state.board.zombies[zombie_index].health;
            events.push(GameEvent::PlantSpecialHit {
                plant: plant_id,
                zombie: zombie_id,
                damage: SPIKEWEED_DAMAGE,
                health_remaining,
            });
            if health_remaining <= 0 {
                self.emit_zombie_died(zombie_id, events);
                self.state.board.zombies.remove(zombie_index);
            } else {
                zombie_index += 1;
            }
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
            self.damage_zombie(target_idx, ZOMBIE_BITE_DAMAGE);
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
            FiringPattern::Backward => self.fire_projectile(
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
            ),
            _ => self.fire_projectile(
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
            ),
        }
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
        self.state.board.coins.push(CoinPickupState {
            id,
            coin_type,
            value,
            position_x,
            position_y,
            plant_type,
            usable_seed_type,
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
            1370,
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
            270,
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
        if let Some(zombie) = self.state.board.zombies.iter_mut().find(|z| z.id == id) {
            zombie.speed = 660_000;
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
            540,
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
        self._spawn_zombie_inner(ZombieType::Imp, 70, row, wave, position_override, events)
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
            270,
            row,
            wave,
            position_override,
            events,
        );
        if let Some(zombie) = self.state.board.zombies.iter_mut().find(|z| z.id == id) {
            zombie.jackbox_timer = self.rng.range_inclusive(500, 1500);
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
            ZombieType::Buckethead | ZombieType::ScreenDoor => 1_370,
            ZombieType::Football => 1_670,
            ZombieType::PoleVaulter | ZombieType::Pogo => 500,
            ZombieType::Gargantuar => 3_000,
            ZombieType::Gigagargantuar => GIGAGARGANTUAR_HEALTH,
            ZombieType::JalapenoHead => ZOMBOTANY_JALAPENO_HEALTH,
            ZombieType::Boss => BOSS_ADVENTURE_HEALTH,
            _ => 270,
        };
        self._spawn_zombie_inner(zombie_type, health, row, 0, Some(grid_x(column)), events);
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
        let speed = if zombie_type == ZombieType::Pogo {
            450_000
        } else if zombie_type == ZombieType::Digger {
            120_000
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
            shield_health: match zombie_type {
                ZombieType::Ladder => LADDER_SHIELD_HEALTH,
                ZombieType::WallnutHead => ZOMBOTANY_WALLNUT_HELM_HEALTH,
                ZombieType::TallnutHead => ZOMBOTANY_TALLNUT_HELM_HEALTH,
                _ => 0,
            },
            shield_max_health: match zombie_type {
                ZombieType::Ladder => LADDER_SHIELD_HEALTH,
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

fn is_ladder_target(plant_type: PlantType) -> bool {
    matches!(plant_type.slot(), 3 | 23 | 30)
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

fn projectile_can_hit_zombie(zombie: &ZombieState) -> bool {
    zombie.zombie_type != ZombieType::Snorkel || zombie.snorkel_phase != 1 || zombie.eating
}

fn balloon_is_airborne(zombie: &ZombieState) -> bool {
    zombie.zombie_type == ZombieType::Balloon
        && matches!(
            zombie.balloon_phase,
            BALLOON_FLYING_PHASE | BALLOON_POPPING_PHASE
        )
}

fn zamboni_speed(position_x: i64) -> i64 {
    let min_x = 300 * POSITION_SCALE;
    let max_x = 700 * POSITION_SCALE;
    if position_x <= min_x {
        50_000
    } else if position_x >= max_x {
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
        for _ in 0..200 {
            hit |= game
                .advance(InputFrame::default())
                .iter()
                .any(|event| matches!(event, GameEvent::ProjectileHit { damage: 20, .. }));
        }

        assert!(hit);
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

        assert_eq!(
            events,
            vec![GameEvent::ProjectileFired {
                entity: 1,
                source: 1,
                projectile_type: ProjectileType::Pea,
                row: 2,
            }]
        );
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
        assert_eq!(game.state.board.plants[0].special_target, Some(target));
        assert_eq!(
            game.state.board.plants[0].special_counter,
            SQUASH_LOOK_TICKS
        );
        assert!(!game.state.board.plants[0].special_armed);

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

        game.state.board.plants[0].special_counter = 1;
        let events = game.advance(InputFrame::default());

        assert!(events.iter().any(|event| matches!(
            event,
            GameEvent::PlantSpecialTriggered {
                entity,
                plant_type: PlantType::Other(17),
            } if *entity == squash
        )));
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
            events
                .iter()
                .any(|event| matches!(event, GameEvent::PlantDied { entity } if *entity == squash))
        );
        assert!(game.state.board.plants.is_empty());
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .map(|zombie| zombie.id)
                .collect::<Vec<_>>(),
            vec![far, other_row]
        );
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
    fn flag_zombie_is_identical_to_normal_except_type() {
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
    fn screen_door_zombie_has_1370_health_and_type() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_screen_door_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);
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
            ZombieType::ScreenDoor
        );
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
        assert_eq!(dolphin_state.health, 270);
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
        assert_eq!(zamboni_state.speed, 50_000);

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
        game.damage_zombie(leader_index, 20);
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
        game.damage_zombie(ladder_index, 20);
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
    fn first_projectile_hit_pops_balloon_before_body_damage() {
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
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .speed,
            660_000
        );
    }

    #[test]
    fn newspaper_zombie_speeds_up_after_taking_enough_damage() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_newspaper_zombie(2, 0, Some(500 * POSITION_SCALE), &mut setup);

        // Verify starting health is 540.
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .health,
            540
        );
        assert_eq!(
            game.state
                .board
                .zombies
                .iter()
                .find(|z| z.id == zombie)
                .unwrap()
                .zombie_type,
            ZombieType::Newspaper
        );

        // Damage the zombie to below 270 HP (breaking the newspaper).
        game.state.board.zombies.iter_mut().for_each(|z| {
            if z.id == zombie {
                z.health = 260;
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
        // With newspaper speed (660,000) the zombie should move at least 100,000 units.
        assert!(
            pos_before - pos_after >= 100_000,
            "newspaper zombie at 260 HP should move fast; moved {}",
            pos_before - pos_after
        );
    }

    #[test]
    fn imp_zombie_has_70_health_and_imp_type() {
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
            70
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
    fn jackbox_zombie_explodes_after_timer_and_damages_nearby_plants() {
        let mut game = Game::new(7, SceneKind::Day);
        let mut setup = Vec::new();
        let zombie = game.spawn_jackbox_zombie(2, 0, Some(780 * POSITION_SCALE), &mut setup);

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
        game.state.scene = SceneKind::GameOver;
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
                assert!(game.state().garden.plants[0].watered);
                assert!(game.state().garden.plants[0].age_ticks > 100);
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
        assert_eq!(
            bobsled
                .state()
                .board
                .zombies
                .iter()
                .filter(|zombie| zombie.zombie_type == ZombieType::Bobsled)
                .count(),
            4
        );
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(
                    event,
                    GameEvent::ZombieSpawned {
                        zombie_type: ZombieType::Bobsled,
                        wave: 0,
                        ..
                    }
                ))
                .count(),
            4
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
        assert!(game.state().board.brains[0].squished);
        assert_ne!(game.state().scene, SceneKind::Complete);
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
