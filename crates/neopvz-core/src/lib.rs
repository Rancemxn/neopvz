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
        match self.slot() {
            7 => 2,
            40 => 4,
            _ => 1,
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
            _ => ProjectileMotion::Straight,
        }
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
    pub wave: WaveState,
    pub sun_countdown: u32,
    pub suns_fallen: u32,
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
            wave: WaveState {
                current: 0,
                total: 1,
                countdown: FIRST_WAVE_COUNTDOWN,
                countdown_start: FIRST_WAVE_COUNTDOWN,
            },
            sun_countdown,
            suns_fallen: 0,
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
                match plant_type.slot() {
                    18 | 29 => row_distance <= 2,
                    28 => {
                        row_distance == 0
                            && (zombie.position_x > plant_attack_start(column)
                                || zombie.position_x < grid_x(column))
                    }
                    52 => row_distance == 0 && zombie.position_x < grid_x(column),
                    _ => row_distance == 0 && zombie.position_x > plant_attack_start(column),
                }
            });

            let mut fire = false;
            let mut produce_suns = 0;
            {
                let plant = &mut self.state.board.plants[index];
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
                        produce_suns = if plant_type.slot() == 41 { 2 } else { 1 };
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
            for _ in 0..produce_suns {
                self.spawn_sun(SunSource::Plant(id), grid_x(column), grid_y(row), events);
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
    }

    fn update_zombies(&mut self, events: &mut Vec<GameEvent>) {
        let zombie_count = self.state.board.zombies.len() as u32;
        for zombie_index in 0..self.state.board.zombies.len() {
            let (entity, row, position_x, age, was_eating) = {
                let zombie = &mut self.state.board.zombies[zombie_index];
                zombie.age = zombie.age.saturating_add(1);
                zombie.groan_counter -= 1;
                zombie.chilled_counter = zombie.chilled_counter.saturating_sub(1);
                if zombie.groan_counter == 0 && self.rng.range(zombie_count) == 0 {
                    zombie.groan_counter = (self.rng.range(1_000) + 500) as i32;
                }
                if !zombie.eating {
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
                )
            };

            if age % 4 == 0 {
                let target = self.find_plant_for_zombie(row, position_x);
                self.state.board.zombies[zombie_index].eating = target.is_some();
                if let Some(plant_index) = target {
                    let plant_id = self.state.board.plants[plant_index].id;
                    self.state.board.plants[plant_index].health -= 4;
                    let health_remaining = self.state.board.plants[plant_index].health;
                    events.push(GameEvent::PlantDamaged {
                        entity: plant_id,
                        damage: 4,
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
                self.state.board.projectiles.remove(projectile_index);
                if health_remaining <= 0 {
                    // ponytail: remove terminal entities now; add death phases when rendering consumes them.
                    self.state.board.zombies.remove(zombie_index);
                    events.push(GameEvent::ZombieDied { entity: zombie_id });
                }
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

    fn update_seed_packets(&mut self) {
        for packet in &mut self.state.board.seed_packets {
            packet.refresh_remaining = packet.refresh_remaining.saturating_sub(1);
        }
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

    fn fire_projectiles(
        &mut self,
        source: EntityId,
        plant_type: PlantType,
        row: u8,
        column: u8,
        events: &mut Vec<GameEvent>,
    ) {
        let projectile_type = plant_type.projectile_type();
        let position_x = grid_x(column) + 60 * POSITION_SCALE;
        let position_y = grid_y(row);
        match plant_type.slot() {
            18 => {
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
                        ProjectileMotion::Straight,
                        target_row,
                        position_x,
                        grid_y(target_row),
                        3_330_000,
                        0,
                        events,
                    );
                }
            }
            28 => {
                self.fire_projectile(
                    source,
                    projectile_type,
                    ProjectileMotion::Straight,
                    row,
                    position_x,
                    position_y,
                    3_330_000,
                    0,
                    events,
                );
                self.fire_projectile(
                    source,
                    projectile_type,
                    ProjectileMotion::Backwards,
                    row,
                    grid_x(column) + 20 * POSITION_SCALE,
                    position_y,
                    -3_330_000,
                    0,
                    events,
                );
            }
            29 => {
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
                        ProjectileMotion::Star,
                        row,
                        position_x,
                        position_y,
                        velocity_x,
                        velocity_y,
                        events,
                    );
                }
            }
            52 => self.fire_projectile(
                source,
                projectile_type,
                ProjectileMotion::Backwards,
                row,
                grid_x(column) + 20 * POSITION_SCALE,
                position_y,
                -3_330_000,
                0,
                events,
            ),
            _ => self.fire_projectile(
                source,
                projectile_type,
                projectile_type.motion(),
                row,
                position_x,
                position_y,
                3_330_000,
                0,
                events,
            ),
        }
    }

    fn fire_projectile(
        &mut self,
        source: EntityId,
        projectile_type: ProjectileType,
        motion: ProjectileMotion,
        row: u8,
        position_x: i64,
        position_y: i64,
        velocity_x: i64,
        velocity_y: i64,
        events: &mut Vec<GameEvent>,
    ) {
        let id = self.state.board.allocate_entity();
        self.state.board.projectiles.push(ProjectileState {
            id,
            projectile_type,
            motion,
            row,
            position_x,
            position_y,
            velocity_x,
            velocity_y,
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
        let id = self.state.board.allocate_entity();
        self.state.board.suns.push(SunPickupState {
            id,
            source,
            value: 25,
            position_x,
            position_y,
        });
        events.push(GameEvent::SunProduced {
            entity: id,
            source,
            value: 25,
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
