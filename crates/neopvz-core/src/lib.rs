use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

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
const SUNSHROOM_GROWTH_TICKS: u32 = 12_000;
const SMALL_SUN_VALUE: u32 = 15;
const INSTANT_PLANT_COUNTDOWN: u32 = 100;
const POTATO_ARM_TICKS: u32 = 1_500;
const PLANT_SPECIAL_DAMAGE: i32 = 1_800;
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

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum SceneKind {
    #[default]
    Title,
    AdventureSelect,
    SeedChooser,
    Day,
    Night,
    Pool,
    Roof,
    Complete,
    GameOver,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum PlantType {
    Peashooter,
    Sunflower,
    Other(u8),
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

    fn slot(self) -> u8 {
        match self {
            Self::Peashooter => 0,
            Self::Sunflower => 1,
            Self::Other(slot) => slot,
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
        matches!(self.slot(), 1 | 9 | 41)
    }

    fn is_sunshroom(self) -> bool {
        self.slot() == 9
    }

    fn is_twin_sunflower(self) -> bool {
        self.slot() == 41
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

    fn is_jalapeno(self) -> bool {
        self.slot() == 20
    }

    fn is_ice_shroom(self) -> bool {
        self.slot() == 14
    }

    fn is_doom_shroom(self) -> bool {
        self.slot() == 15
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
        max_health: 300,
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
            Self::ZombiePea | Self::Other(_) => 20,
        }
    }

    fn motion(self) -> ProjectileMotion {
        match self {
            Self::Star => ProjectileMotion::Star,
            Self::Cabbage | Self::Melon | Self::WinterMelon | Self::Kernel | Self::Butter => {
                ProjectileMotion::Lobbed
            }
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
    Occupied,
    Crater,
    NotEnoughSun,
    MissingEntity,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum InputAction {
    Pause,
    Resume,
    SelectSeed { slot: u8 },
    Plant { row: u8, column: u8 },
    Shovel { row: u8, column: u8 },
    CollectSun { entity: EntityId },
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
    pub from_wave: u32,
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
pub struct CraterState {
    pub row: u8,
    pub column: u8,
    pub remaining: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WaveState {
    pub current: u32,
    pub total: u32,
    pub countdown: u32,
    pub countdown_start: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BoardState {
    pub rows: u8,
    pub columns: u8,
    pub next_entity_id: EntityId,
    pub selected_seed: Option<u8>,
    pub seed_packets: Vec<SeedPacketState>,
    pub plants: Vec<PlantState>,
    pub zombies: Vec<ZombieState>,
    pub projectiles: Vec<ProjectileState>,
    pub suns: Vec<SunPickupState>,
    pub craters: Vec<CraterState>,
    pub wave: WaveState,
    pub sun_countdown: u32,
    pub suns_fallen: u32,
    pub ice_counter: u32,
}

impl BoardState {
    fn new(scene: SceneKind, rng: &mut Mt19937) -> Self {
        let rows = if scene == SceneKind::Pool {
            POOL_ROWS
        } else {
            DAY_ROWS
        };
        let sun_countdown = SUN_COUNTDOWN + rng.range(276);
        Self {
            rows,
            columns: GRID_COLUMNS,
            next_entity_id: 1,
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
            plants: Vec::new(),
            zombies: Vec::new(),
            projectiles: Vec::new(),
            suns: Vec::new(),
            craters: Vec::new(),
            wave: WaveState {
                current: 0,
                total: 1,
                countdown: FIRST_WAVE_COUNTDOWN,
                countdown_start: FIRST_WAVE_COUNTDOWN,
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
    pub sun: u32,
    pub wave: u32,
    pub paused: bool,
    pub board: BoardState,
    pub rng: RngState,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum GameEvent {
    Started,
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
    PlantShoveled {
        entity: EntityId,
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
    WaveStarted {
        wave: u32,
    },
    ZombieSpawned {
        entity: EntityId,
        zombie_type: ZombieType,
        row: u8,
        wave: u32,
    },
    ProjectileFired {
        entity: EntityId,
        source: EntityId,
        projectile_type: ProjectileType,
        row: u8,
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
    GameLost {
        zombie: EntityId,
    },
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

        let mut game = Game::new(self.header.seed, self.scene);
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
        let mut rng = Mt19937::new(seed);
        let board = BoardState::new(scene, &mut rng);
        let state = GameState {
            seed,
            tick: 0,
            scene,
            sun: 50,
            wave: 0,
            paused: false,
            board,
            rng: rng.snapshot(),
        };
        Self { state, rng }
    }

    pub fn state(&self) -> &GameState {
        &self.state
    }

    pub fn advance(&mut self, input: InputFrame) -> Vec<GameEvent> {
        let mut events = Vec::new();
        for action in input.actions {
            self.apply_input(action, &mut events);
        }

        if !self.state.paused && self.is_playing_scene() {
            self.update_plants(&mut events);
            self.update_zombies(&mut events);
            self.update_projectiles(&mut events);
            self.update_seed_packets();
            self.update_sun_spawning(&mut events);
            self.update_wave_spawning(&mut events);
            self.state.board.ice_counter = self.state.board.ice_counter.saturating_sub(1);
            self.update_craters();
            self.state.tick = self.state.tick.saturating_add(1);
            self.state.wave = self.state.board.wave.current;
            events.push(GameEvent::StateChanged);
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
            SceneKind::Day | SceneKind::Night | SceneKind::Pool | SceneKind::Roof
        )
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
            InputAction::SelectSeed { slot } => self.select_seed(slot, events),
            InputAction::Plant { row, column } => self.plant(row, column, events),
            InputAction::Shovel { row, column } => self.shovel(row, column, events),
            InputAction::CollectSun { entity } => self.collect_sun(entity, events),
        }
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
        if packet.refresh_remaining != 0 {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::SeedRefreshing,
            });
            return;
        }

        self.state.board.selected_seed = Some(slot);
        events.push(GameEvent::SeedSelected {
            slot,
            plant_type: packet.plant_type,
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
        if row >= self.state.board.rows || column >= self.state.board.columns {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::OutsideBoard,
            });
            return;
        }
        if self
            .state
            .board
            .plants
            .iter()
            .any(|plant| plant.row == row && plant.column == column)
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

        let packet_index = self
            .state
            .board
            .seed_packets
            .iter()
            .position(|packet| packet.slot == slot)
            .expect("selected seed packet must exist");
        let plant_type = self.state.board.seed_packets[packet_index].plant_type;
        if self.state.sun < plant_type.cost() {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::NotEnoughSun,
            });
            return;
        }

        self.state.sun -= plant_type.cost();
        self.state.board.selected_seed = None;
        self.state.board.seed_packets[packet_index].refresh_remaining =
            plant_type.refresh_time() + 1;
        let id = self.state.board.allocate_entity();

        // Preserve the original gameplay RNG stream even before render state consumes these values.
        let _frame_length = self.rng.range_inclusive(12, 18);
        let _body_animation_rate = self.rng.next();
        let blink_counter = 400 + self.rng.range(400);
        if plant_type == PlantType::Peashooter {
            let _head_animation_rate = self.rng.next();
        }
        let launch_rate = plant_type.launch_rate();
        let launch_counter = if launch_rate == 0 {
            0
        } else if plant_type.is_producer() {
            self.rng.range_inclusive(300, launch_rate / 2)
        } else {
            self.rng.range_inclusive(0, launch_rate)
        };
        let max_health = plant_type.max_health();
        let (special_counter, special_armed) = if plant_type.is_cherry_bomb()
            || plant_type.is_jalapeno()
            || plant_type.is_ice_shroom()
            || plant_type.is_doom_shroom()
        {
            (INSTANT_PLANT_COUNTDOWN, false)
        } else if plant_type.is_potato_mine() {
            (POTATO_ARM_TICKS, false)
        } else {
            (0, false)
        };
        self.state.board.plants.push(PlantState {
            id,
            plant_type,
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
        });
        events.push(GameEvent::PlantPlaced {
            entity: id,
            plant_type,
            row,
            column,
            sun_remaining: self.state.sun,
        });
    }

    fn shovel(&mut self, row: u8, column: u8, events: &mut Vec<GameEvent>) {
        let action = InputAction::Shovel { row, column };
        let Some(index) = self
            .state
            .board
            .plants
            .iter()
            .position(|plant| plant.row == row && plant.column == column)
        else {
            events.push(GameEvent::InputRejected {
                action,
                reason: InputRejectReason::MissingEntity,
            });
            return;
        };
        let entity = self.state.board.plants.remove(index).id;
        events.push(GameEvent::PlantShoveled { entity });
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

    fn update_plants(&mut self, events: &mut Vec<GameEvent>) {
        let plant_count = self.state.board.plants.len();
        for index in 0..plant_count {
            let (id, plant_type, row, column) = {
                let plant = &self.state.board.plants[index];
                (plant.id, plant.plant_type, plant.row, plant.column)
            };
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
                    _ => row_distance == 0 && zombie.position_x > plant_attack_start(column),
                }
            });
            let chomper_target = if plant_type.is_chomper() {
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

            let mut fire = false;
            let mut produce_suns = 0;
            let mut produce_value = 25;
            let mut special = false;
            let mut chomper_bite_target = None;
            {
                let plant = &mut self.state.board.plants[index];
                if plant_type.is_chomper() {
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
                        produce_suns = if plant_type.is_twin_sunflower() { 2 } else { 1 };
                        if plant_type.is_sunshroom() && plant.production_stage == 0 {
                            produce_value = SMALL_SUN_VALUE;
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
            if let Some(zombie_id) = chomper_bite_target {
                if let Some(zombie_index) = self
                    .state
                    .board
                    .zombies
                    .iter()
                    .position(|zombie| zombie.id == zombie_id && zombie.health > 0)
                {
                    self.state.board.zombies.remove(zombie_index);
                    events.push(GameEvent::PlantSpecialTriggered {
                        entity: id,
                        plant_type,
                    });
                    events.push(GameEvent::ZombieDied { entity: zombie_id });
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
                let _vertical_motion = self.rng.next();
                let _horizontal_motion = self.rng.next();
                let _ground_offset = self.rng.range(20);
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
                self.state.board.zombies[zombie_index].health -= PLANT_SPECIAL_DAMAGE;
                let health_remaining = self.state.board.zombies[zombie_index].health;
                events.push(GameEvent::PlantSpecialHit {
                    plant: plant_id,
                    zombie: zombie_id,
                    damage: PLANT_SPECIAL_DAMAGE,
                    health_remaining,
                });
                if health_remaining <= 0 {
                    self.state.board.zombies.remove(zombie_index);
                    events.push(GameEvent::ZombieDied { entity: zombie_id });
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

                let zombie = &mut self.state.board.zombies[zombie_index];
                let had_debuff = zombie.frozen_counter != 0 || zombie.chilled_counter != 0;
                zombie.chilled_counter = zombie.chilled_counter.max(ICE_SHROOM_CHILL_TICKS);
                events.push(GameEvent::ZombieChilled {
                    entity: zombie_id,
                    duration: ICE_SHROOM_CHILL_TICKS,
                });

                // The current implementation only exposes Normal zombies, which
                // are freezeable in the target. Keep the eligibility decision
                // explicit so future zombie types cannot inherit this blindly.
                let can_freeze = matches!(zombie.zombie_type, ZombieType::Normal);
                if can_freeze {
                    let duration = if had_debuff {
                        ICE_SHROOM_REFRESH_FREEZE_TICKS
                    } else {
                        ICE_SHROOM_INITIAL_FREEZE_TICKS
                    };
                    zombie.frozen_counter = zombie.frozen_counter.max(duration);
                    events.push(GameEvent::ZombieFrozen {
                        entity: zombie_id,
                        duration,
                    });

                    zombie.health -= ICE_SHROOM_DAMAGE;
                    let health_remaining = zombie.health;
                    events.push(GameEvent::PlantSpecialHit {
                        plant: plant_id,
                        zombie: zombie_id,
                        damage: ICE_SHROOM_DAMAGE,
                        health_remaining,
                    });
                    if health_remaining <= 0 {
                        self.state.board.zombies.remove(zombie_index);
                        events.push(GameEvent::ZombieDied { entity: zombie_id });
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
            self.state.board.zombies[zombie_index].health -= PLANT_SPECIAL_DAMAGE;
            let health_remaining = self.state.board.zombies[zombie_index].health;
            events.push(GameEvent::PlantSpecialHit {
                plant: plant_id,
                zombie: zombie_id,
                damage: PLANT_SPECIAL_DAMAGE,
                health_remaining,
            });
            if health_remaining <= 0 {
                self.state.board.zombies.remove(zombie_index);
                events.push(GameEvent::ZombieDied { entity: zombie_id });
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

    fn update_zombies(&mut self, events: &mut Vec<GameEvent>) {
        let zombie_count = self.state.board.zombies.len() as u32;
        for zombie_index in 0..self.state.board.zombies.len() {
            let (entity, row, position_x, age, was_eating, frozen) = {
                let zombie = &mut self.state.board.zombies[zombie_index];
                zombie.age = zombie.age.saturating_add(1);
                zombie.groan_counter -= 1;
                zombie.frozen_counter = zombie.frozen_counter.saturating_sub(1);
                zombie.chilled_counter = zombie.chilled_counter.saturating_sub(1);
                if zombie.groan_counter == 0 && self.rng.range(zombie_count) == 0 {
                    zombie.groan_counter = (self.rng.range(1_000) + 500) as i32;
                }
                let frozen = zombie.frozen_counter != 0;
                if !frozen && !zombie.eating {
                    let speed = if zombie.chilled_counter == 0 {
                        zombie.speed
                    } else {
                        zombie.speed * 2 / 5
                    };
                    zombie.position_x -= speed;
                }
                (
                    zombie.id,
                    zombie.row,
                    zombie.position_x,
                    zombie.age,
                    zombie.eating,
                    frozen,
                )
            };

            if frozen {
                self.state.board.zombies[zombie_index].eating = false;
            } else if age % 4 == 0 {
                let target = self.find_plant_for_zombie(row, position_x);
                self.state.board.zombies[zombie_index].eating = target.is_some();
                if let Some(plant_index) = target {
                    let plant_id = self.state.board.plants[plant_index].id;
                    self.state.board.plants[plant_index].health -= ZOMBIE_BITE_DAMAGE;
                    let health_remaining = self.state.board.plants[plant_index].health;
                    events.push(GameEvent::PlantDamaged {
                        entity: plant_id,
                        damage: ZOMBIE_BITE_DAMAGE,
                        health_remaining,
                    });
                    if health_remaining <= 0 {
                        self.state.board.plants.remove(plant_index);
                        self.state.board.zombies[zombie_index].eating = false;
                        events.push(GameEvent::PlantDied { entity: plant_id });
                    }
                } else if was_eating {
                    self.state.board.zombies[zombie_index].eating = false;
                }
            }

            if self.state.board.zombies[zombie_index].position_x <= -100 * POSITION_SCALE {
                self.state.scene = SceneKind::GameOver;
                events.push(GameEvent::GameLost { zombie: entity });
                break;
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
            let projectile = self.state.board.projectiles[projectile_index].clone();
            let projectile_row = projectile_row(projectile.position_y, self.state.board.rows);
            let target = self
                .state
                .board
                .zombies
                .iter()
                .enumerate()
                .filter(|(_, zombie)| Some(zombie.row) == projectile_row && zombie.health > 0)
                .filter(|(_, zombie)| projectile_hits(projectile.position_x, zombie.position_x))
                .min_by_key(|(_, zombie)| zombie.position_x)
                .map(|(index, _)| index);

            if let Some(zombie_index) = target {
                let zombie_id = self.state.board.zombies[zombie_index].id;
                self.state.board.zombies[zombie_index].health -= projectile.damage;
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
                    self.state.board.zombies.remove(zombie_index);
                    events.push(GameEvent::ZombieDied { entity: zombie_id });
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

    fn steer_homing_projectile(&mut self, projectile_index: usize) {
        let projectile = &self.state.board.projectiles[projectile_index];
        let current_row =
            projectile_row(projectile.position_y, self.state.board.rows).unwrap_or(projectile.row);
        let target_row = self
            .state
            .board
            .zombies
            .iter()
            .filter(|zombie| zombie.health > 0)
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
            self.state.board.zombies[zombie_index].health -= splash_damage;
            let health_remaining = self.state.board.zombies[zombie_index].health;
            events.push(GameEvent::ProjectileSplashHit {
                projectile: projectile.id,
                zombie: zombie_id,
                damage: splash_damage,
                health_remaining,
            });
            self.apply_projectile_chill(zombie_id, projectile.projectile_type, events);
            if health_remaining <= 0 {
                self.state.board.zombies.remove(zombie_index);
                events.push(GameEvent::ZombieDied { entity: zombie_id });
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
            return;
        }
        self.state.board.wave.countdown = self.state.board.wave.countdown.saturating_sub(1);
        if self.state.board.wave.countdown != 0 {
            return;
        }

        let wave = self.state.board.wave.current;
        events.push(GameEvent::WaveStarted { wave });
        let row = self.rng.range(u32::from(self.state.board.rows)) as u8;
        self.spawn_normal_zombie(row, wave, None, events);
        self.state.board.wave.current += 1;
        self.state.board.wave.countdown_start = 0;
    }

    fn find_plant_for_zombie(&self, row: u8, zombie_x: i64) -> Option<usize> {
        self.state
            .board
            .plants
            .iter()
            .enumerate()
            .filter(|(_, plant)| plant.row == row && plant.health > 0)
            .filter(|(_, plant)| {
                let plant_x = grid_x(plant.column);
                zombie_x + 70 * POSITION_SCALE > plant_x
                    && zombie_x + 50 * POSITION_SCALE < plant_x + 80 * POSITION_SCALE
            })
            .max_by_key(|(_, plant)| plant.column)
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
        let position_x = grid_x(column) + 60 * POSITION_SCALE;
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
                    velocity_x: 3_330_000,
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

    fn spawn_normal_zombie(
        &mut self,
        row: u8,
        wave: u32,
        position_override: Option<i64>,
        events: &mut Vec<GameEvent>,
    ) -> EntityId {
        let id = self.state.board.allocate_entity();
        let position_x = position_override
            .unwrap_or_else(|| i64::from(780 + self.rng.range(40)) * POSITION_SCALE);
        let groan_counter = self.rng.range_inclusive(300, 400) as i32;
        let speed = self.rng.fixed_range(230_000, 320_000);
        self.state.board.zombies.push(ZombieState {
            id,
            zombie_type: ZombieType::Normal,
            row,
            position_x,
            speed,
            health: 270,
            max_health: 270,
            age: 0,
            groan_counter,
            frozen_counter: 0,
            chilled_counter: 0,
            eating: false,
            from_wave: wave,
        });
        events.push(GameEvent::ZombieSpawned {
            entity: id,
            zombie_type: ZombieType::Normal,
            row,
            wave,
        });
        id
    }
}

fn grid_x(column: u8) -> i64 {
    i64::from(column) * 80 * POSITION_SCALE + 40 * POSITION_SCALE
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
        let mut game = Game::new(7, SceneKind::Day);
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
        let mut game = Game::new(7, SceneKind::Day);
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
    fn doom_shroom_removes_targets_and_leaves_a_replant_blocking_crater() {
        let mut game = Game::new(7, SceneKind::Day);
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
        let mut game = Game::new(7, SceneKind::Day);
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
        let mut game = Game::new(7, SceneKind::Day);
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
}
