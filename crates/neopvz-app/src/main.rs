use std::{
    collections::{HashMap, HashSet},
    io::ErrorKind,
    path::{Path, PathBuf},
    process::ExitCode,
    sync::Arc,
    time::{Duration, Instant},
};

use clap::{Parser, ValueEnum};
use neopvz_audio::{AudioBackend, AudioKind, KiraAudioBackend};
use neopvz_core::{
    AwardCollectionSound, ChallengeKind, CoinType, EntityId, Game, GameEvent, GardenNeed,
    GardenServiceKind, GardenTool, HiddenCode, InputAction, InputFrame, LootDropSound, ModeKind,
    PlantType, ProjectileImpactSound, ProjectileType, SEEING_STARS_STARFRUIT_CELLS, SaveError,
    SaveProfile, SceneKind, StoreItem, SunSource, VaseContents, WeatherSound, WhackHitSound,
    ZombieGroanFamily, ZombieState, ZombieType, adventure_award_seed, adventure_completion_award,
    adventure_flag_wave_count, adventure_level_is_conveyor, adventure_seed_choices,
    adventure_seed_slots, adventure_unlocks, fixed_point_to_logical, last_stand_seed_choices,
    last_stand_seed_slots, mode_level_name, mode_level_names, store_item_cost, zombie_wave_stats,
};
use neopvz_data::{
    AssetLayout, ReanimatorDefinition, ReanimatorTrack, ReanimatorTransform, ResourceKind,
    ResourceProvider,
};
use neopvz_render::{
    ALMANAC_CLOSE_BUTTON_IMAGE_ID, ALMANAC_CLOSE_TEXT_IMAGE_ID, ALMANAC_IMITATER_IMAGE_ID,
    ALMANAC_INDEX_BACKGROUND_IMAGE_ID, ALMANAC_INDEX_BUTTON_IMAGE_ID, ALMANAC_INDEX_TEXT_IMAGE_ID,
    ALMANAC_NAV_BUTTON_IMAGE_ID, ALMANAC_PLANT_BACKGROUND_IMAGE_ID, ALMANAC_PLANT_CARD_IMAGE_ID,
    ALMANAC_PLANT_COST_BASE_IMAGE_ID, ALMANAC_PLANT_DESCRIPTION_BASE_IMAGE_ID,
    ALMANAC_PLANT_NAME_BASE_IMAGE_ID, ALMANAC_PLANT_RECHARGE_BASE_IMAGE_ID,
    ALMANAC_PLANTS_TEXT_IMAGE_ID, ALMANAC_TITLE_TEXT_IMAGE_ID, ALMANAC_ZOMBIE_BACKGROUND_IMAGE_ID,
    ALMANAC_ZOMBIE_BLANK_IMAGE_ID, ALMANAC_ZOMBIE_CARD_IMAGE_ID,
    ALMANAC_ZOMBIE_DESCRIPTION_BASE_IMAGE_ID, ALMANAC_ZOMBIE_NAME_BASE_IMAGE_ID,
    ALMANAC_ZOMBIE_SILHOUETTE_NAME_BASE_IMAGE_ID, ALMANAC_ZOMBIE_WINDOW_IMAGE_ID,
    ALMANAC_ZOMBIE_WINDOW2_IMAGE_ID, ALMANAC_ZOMBIES_TEXT_IMAGE_ID, AWARD_ALMANAC_IMAGE_ID,
    AWARD_BODY_TEXT_BASE_IMAGE_ID, AWARD_BODY_TEXT_IMAGE_ID, AWARD_CAR_KEYS_IMAGE_ID,
    AWARD_CONTINUE_TEXT_IMAGE_ID, AWARD_MAIN_MENU_TEXT_IMAGE_ID, AWARD_NOTE1_IMAGE_ID,
    AWARD_NOTE2_IMAGE_ID, AWARD_NOTE3_IMAGE_ID, AWARD_NOTE4_IMAGE_ID,
    AWARD_SCREEN_BACKGROUND_IMAGE_ID, AWARD_SHOVEL_IMAGE_ID, AWARD_TACO_IMAGE_ID,
    AWARD_TITLE_TEXT_BASE_IMAGE_ID, AWARD_TITLE_TEXT_IMAGE_ID, AWARD_TROPHY_IMAGE_ID,
    AWARD_WATERING_CAN_IMAGE_ID, AffineSpriteCommand, AffineSpriteSource,
    BOARD_BEGHOULED_TWIST_OVERLAY_IMAGE_ID, BOARD_BRAIN_IMAGE_ID, BOARD_CHOCOLATE_IMAGE_ID,
    BOARD_COIN_GOLD_IMAGE_ID, BOARD_COIN_SILVER_IMAGE_ID, BOARD_CONVEYOR_BELT_BACKDROP_IMAGE_ID,
    BOARD_CONVEYOR_BELT_BASE_IMAGE_ID, BOARD_CRATER_IMAGE_ID, BOARD_DIAMOND_IMAGE_ID,
    BOARD_FUMESHROOM_IMAGE_ID, BOARD_GOLD_SUNFLOWER_IMAGE_ID, BOARD_GRAVE_IMAGE_ID,
    BOARD_MAGNETSHROOM_IMAGE_ID, BOARD_MONEYBAG_IMAGE_ID, BOARD_NOTE_IMAGE_ID,
    BOARD_PRESENT_IMAGE_ID, BOARD_PROGRESS_FILL_IMAGE_ID, BOARD_PROGRESS_FLAG_IMAGE_ID,
    BOARD_PROGRESS_HEAD_IMAGE_ID, BOARD_PROGRESS_LEVEL_IMAGE_ID, BOARD_PROGRESS_METER_IMAGE_ID,
    BOARD_PROGRESS_POLE_IMAGE_ID, BOARD_PROJECTILE_BASKETBALL_IMAGE_ID,
    BOARD_PROJECTILE_BUTTER_IMAGE_ID, BOARD_PROJECTILE_CABBAGE_IMAGE_ID,
    BOARD_PROJECTILE_COB_IMAGE_ID, BOARD_PROJECTILE_FIREBALL_IMAGE_ID,
    BOARD_PROJECTILE_KERNEL_IMAGE_ID, BOARD_PROJECTILE_MELON_IMAGE_ID,
    BOARD_PROJECTILE_PEA_IMAGE_ID, BOARD_PROJECTILE_PUFF_IMAGE_ID,
    BOARD_PROJECTILE_SHADOW_DAY_IMAGE_ID, BOARD_PROJECTILE_SHADOW_NIGHT_IMAGE_ID,
    BOARD_PROJECTILE_SNOW_PEA_IMAGE_ID, BOARD_PROJECTILE_SPIKE_IMAGE_ID,
    BOARD_PROJECTILE_STAR_IMAGE_ID, BOARD_PROJECTILE_WINTER_MELON_IMAGE_ID,
    BOARD_PUFFSHROOM_IMAGE_ID, BOARD_SEED_BANK_EXTENSION_BASE_IMAGE_ID, BOARD_SEED_BANK_IMAGE_ID,
    BOARD_SEED_COST_BASE_IMAGE_ID, BOARD_SHOVEL_BANK_IMAGE_ID, BOARD_SILVER_SUNFLOWER_IMAGE_ID,
    BOARD_SNOWPEA_IMAGE_ID, BOARD_STARFRUIT_IMAGE_ID, BOARD_SUN_BANK_IMAGE_ID,
    BOARD_SUN_COUNT_IMAGE_ID, BOARD_SUN_IMAGE_ID, BOARD_VASE_BOTTOM_IMAGE_ID, BOARD_VASE_IMAGE_ID,
    BOARD_VASE_TOP_IMAGE_ID, BOARD_WALLNUT_IMAGE_ID, BOARD_ZOMBIE_BODY_IMAGE_ID,
    BOARD_ZOMBIE_BUCKET_IMAGE_ID, BOARD_ZOMBIE_CONE_IMAGE_ID, BOARD_ZOMBIE_FLAG_HAND_IMAGE_ID,
    BOARD_ZOMBIE_FLAG_IMAGE_ID, BOARD_ZOMBIE_FLAG_POLE_IMAGE_ID,
    BOARD_ZOMBIE_FOOTBALL_HEAD_IMAGE_ID, BOARD_ZOMBIE_FOOTBALL_HELMET_IMAGE_ID,
    BOARD_ZOMBIE_FOOTBALL_UPPERBODY_IMAGE_ID, BOARD_ZOMBIE_NEWSPAPER_HEAD_IMAGE_ID,
    BOARD_ZOMBIE_NEWSPAPER_IMAGE_ID, BOARD_ZOMBIE_SCREEN_DOOR_IMAGE_ID, BOSS_BACKGROUND_IMAGE_ID,
    BlendMode, CHALLENGE_THUMBNAIL_BASE_IMAGE_ID, CRAZY_DAVE_BEARD_IMAGE_ID,
    CRAZY_DAVE_BODY_IMAGE_ID, CRAZY_DAVE_EYE_IMAGE_ID, CRAZY_DAVE_EYEBROW_IMAGE_ID,
    CRAZY_DAVE_HEAD_IMAGE_ID, CRAZY_DAVE_INNER_ARM_IMAGE_ID, CRAZY_DAVE_INNER_FINGER1_IMAGE_ID,
    CRAZY_DAVE_INNER_FINGER2_IMAGE_ID, CRAZY_DAVE_INNER_FINGER3_IMAGE_ID,
    CRAZY_DAVE_INNER_FINGER4_IMAGE_ID, CRAZY_DAVE_INNER_HAND_IMAGE_ID, CRAZY_DAVE_MOUTH_IMAGE_ID,
    CRAZY_DAVE_OUTER_ARM_IMAGE_ID, CRAZY_DAVE_OUTER_FINGER1_IMAGE_ID,
    CRAZY_DAVE_OUTER_FINGER2_IMAGE_ID, CRAZY_DAVE_OUTER_FINGER3_IMAGE_ID,
    CRAZY_DAVE_OUTER_FINGER4_IMAGE_ID, CRAZY_DAVE_OUTER_HAND_IMAGE_ID, CRAZY_DAVE_POT_IMAGE_ID,
    CREDITS_BIG_BRAIN_IMAGE_ID, CREDITS_LINE1_TEXT_IMAGE_ID, CREDITS_LINE2_TEXT_IMAGE_ID,
    CREDITS_LINE3_TEXT_IMAGE_ID, CREDITS_LINE4_TEXT_IMAGE_ID, CREDITS_LINE5_TEXT_IMAGE_ID,
    CREDITS_LINE6_TEXT_IMAGE_ID, CREDITS_MAIN_MENU_TEXT_IMAGE_ID, CREDITS_MTV_IMAGE_ID,
    CREDITS_PLAY_BUTTON_IMAGE_ID, CREDITS_REPLAY_TEXT_IMAGE_ID, CREDITS_STAGE_IMAGE_ID,
    CREDITS_TITLE_TEXT_IMAGE_ID, CREDITS_WE_ARE_UNDEAD_IMAGE_ID, CREDITS_ZOMBIE_NOTE_IMAGE_ID,
    DAY_BACKGROUND_IMAGE_ID, FOG_BACKGROUND_IMAGE_ID, GAME_OVER_BODY_TEXT_IMAGE_ID,
    GAME_OVER_HEADER_TEXT_IMAGE_ID, GAME_OVER_MAIN_MENU_TEXT_IMAGE_ID,
    GAME_OVER_TRY_AGAIN_TEXT_IMAGE_ID, GARDEN_BACKGROUND_IMAGE_ID, GARDEN_BUG_SPRAY_IMAGE_ID,
    GARDEN_FERTILIZER_IMAGE_ID, GARDEN_MUSHROOM_BACKGROUND_IMAGE_ID, GARDEN_NEED_BUBBLE_IMAGE_ID,
    GARDEN_NEED_BUG_SPRAY_IMAGE_ID, GARDEN_NEED_FERTILIZER_IMAGE_ID,
    GARDEN_NEED_PHONOGRAPH_IMAGE_ID, GARDEN_PHONOGRAPH_IMAGE_ID, GARDEN_WATERDROP_IMAGE_ID,
    GARDEN_WATERING_CAN_IMAGE_ID, GpuRenderer, HELP_CONTENT_IMAGE_ID, HELP_MAIN_MENU_TEXT_IMAGE_ID,
    HELP_MENU_BUTTON_IMAGE_ID, HELP_ZOMBIE_NOTE_IMAGE_ID, ImageAsset, LogicalViewport,
    MODE_SELECT_BACKGROUND_IMAGE_ID, MODE_SELECT_BLANK_IMAGE_ID, MODE_SELECT_WINDOW_IMAGE_ID,
    NIGHT_BACKGROUND_IMAGE_ID, OPTIONS_ACCELERATION_LABEL_IMAGE_ID, OPTIONS_BACK_TEXT_IMAGE_ID,
    OPTIONS_BACKGROUND_IMAGE_ID, OPTIONS_CHECKBOX_OFF_IMAGE_ID, OPTIONS_CHECKBOX_ON_IMAGE_ID,
    OPTIONS_FULLSCREEN_LABEL_IMAGE_ID, OPTIONS_MUSIC_LABEL_IMAGE_ID, OPTIONS_SFX_LABEL_IMAGE_ID,
    OPTIONS_SLIDER_KNOB_IMAGE_ID, OPTIONS_SLIDER_SLOT_IMAGE_ID, PAUSE_BODY_TEXT_IMAGE_ID,
    PAUSE_DIALOG_BOTTOM_LEFT_IMAGE_ID, PAUSE_DIALOG_BOTTOM_MIDDLE_IMAGE_ID,
    PAUSE_DIALOG_BOTTOM_RIGHT_IMAGE_ID, PAUSE_DIALOG_CENTER_LEFT_IMAGE_ID,
    PAUSE_DIALOG_CENTER_MIDDLE_IMAGE_ID, PAUSE_DIALOG_CENTER_RIGHT_IMAGE_ID,
    PAUSE_DIALOG_HEADER_IMAGE_ID, PAUSE_DIALOG_TOP_LEFT_IMAGE_ID, PAUSE_DIALOG_TOP_MIDDLE_IMAGE_ID,
    PAUSE_DIALOG_TOP_RIGHT_IMAGE_ID, PAUSE_HEADER_TEXT_IMAGE_ID, PAUSE_RESUME_BUTTON_IMAGE_ID,
    PAUSE_RESUME_TEXT_IMAGE_ID, POOL_BACKGROUND_IMAGE_ID, ParticleCatalog, ParticleHolder,
    ROOF_BACKGROUND_IMAGE_ID, RenderFrame, SCREEN_PIXEL_IMAGE_ID, SEED_CHOOSER_BUTTON_IMAGE_ID,
    SEED_CHOOSER_IMAGE_ID, SEED_CHOOSER_TITLE_IMAGE_ID, SEED_PACKET_NORMAL_IMAGE_ID,
    SEED_PACKET_PLANT_BASE_IMAGE_ID, SEED_PACKET_SILHOUETTE_IMAGE_ID, SEED_PEASHOOTER_IMAGE_ID,
    SEED_SUNFLOWER_IMAGE_ID, SELECTOR_ADVENTURE_IMAGE_ID, SELECTOR_ALMANAC_IMAGE_ID,
    SELECTOR_BASE_IMAGE_ID, SELECTOR_CENTER_IMAGE_ID, SELECTOR_CHALLENGES_IMAGE_ID,
    SELECTOR_HELP_IMAGE_ID, SELECTOR_LEAVES_IMAGE_ID, SELECTOR_LEFT_IMAGE_ID,
    SELECTOR_LEVEL_NUMBER_BASE_IMAGE_ID, SELECTOR_OPTIONS_IMAGE_ID, SELECTOR_QUIT_IMAGE_ID,
    SELECTOR_RIGHT_IMAGE_ID, SELECTOR_STORE_IMAGE_ID, SELECTOR_SURVIVAL_IMAGE_ID,
    SELECTOR_TROPHY_IMAGE_ID, SELECTOR_VASEBREAKER_IMAGE_ID, SELECTOR_WOODSIGN1_IMAGE_ID,
    SELECTOR_WOODSIGN2_IMAGE_ID, SELECTOR_WOODSIGN3_IMAGE_ID, SELECTOR_ZEN_GARDEN_IMAGE_ID,
    STORE_BACK_TEXT_IMAGE_ID, STORE_BACKGROUND_IMAGE_ID, STORE_CAR_IMAGE_ID,
    STORE_ITEM_NAME_BASE_IMAGE_ID, STORE_ITEM_PRICE_BASE_IMAGE_ID, STORE_MAIN_MENU_BUTTON_IMAGE_ID,
    STORE_PACKET_UPGRADE_IMAGE_ID, STORE_PRICE_TAG_IMAGE_ID, STORE_SIGN_IMAGE_ID,
    STORE_STINKY_IMAGE_ID, SURVIVAL_THUMBNAIL_BASE_IMAGE_ID, SpriteCommand, TITLE_IMAGE_ID,
    TITLE_LOAD_BAR_DIRT_IMAGE_ID, TITLE_LOAD_BAR_GRASS_IMAGE_ID, TITLE_LOAD_BAR_ROCK1_IMAGE_ID,
    TITLE_LOAD_BAR_ROCK3_IMAGE_ID, TITLE_LOAD_BAR_SPROUT_BODY_IMAGE_ID,
    TITLE_LOAD_BAR_SPROUT_PETAL_IMAGE_ID, TITLE_LOAD_BAR_ZOMBIE_HAIR_IMAGE_ID,
    TITLE_LOAD_BAR_ZOMBIE_HEAD_IMAGE_ID, TITLE_LOAD_BAR_ZOMBIE_JAW_IMAGE_ID,
    TITLE_LOADING_PROMPT_HOVER_IMAGE_ID, TITLE_LOADING_PROMPT_IMAGE_ID,
    TITLE_LOADING_PROMPT_SHADOW_IMAGE_ID, TITLE_LOGO_IMAGE_ID, TITLE_POPCAP_LOGO_IMAGE_ID,
    TITLE_SOD_ROLL_CAP_IMAGE_ID, TITLE_START_PROMPT_HOVER_IMAGE_ID, TITLE_START_PROMPT_IMAGE_ID,
    TITLE_START_PROMPT_SHADOW_IMAGE_ID, TUTORIAL_BUBBLE_IMAGE_ID, TUTORIAL_CONTINUE_IMAGE_ID,
    TUTORIAL_TEXT1_IMAGE_ID, TUTORIAL_TEXT2_IMAGE_ID, UI_PIXEL_IMAGE_ID, ZOMBIES_WON_IMAGE_ID,
    logical_position,
};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalPosition},
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Fullscreen, Window, WindowId},
};

#[derive(Debug, Parser)]
#[command(name = "neopvz", version, about = "Rust PvZ reimplementation")]
struct Cli {
    #[arg(long, value_name = "PATH", conflicts_with = "pak")]
    data_dir: Option<PathBuf>,
    #[arg(long, value_name = "PATH", conflicts_with = "data_dir")]
    pak: Option<PathBuf>,
    #[arg(long, value_name = "PATH")]
    profile: Option<PathBuf>,
    #[arg(long, hide = true, value_name = "SCENE")]
    checkpoint: Option<Checkpoint>,
    #[arg(long, hide = true, value_name = "PATH")]
    capture: Option<PathBuf>,
    #[arg(long, help = "Start in borderless fullscreen")]
    fullscreen: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Checkpoint {
    Title,
    AdventureSelect,
    Store,
    PauseMenu,
    Options,
    Help,
    Almanac,
    Complete,
    CompletePaper,
    Credits,
    CreditsParticles,
    AdventureTutorial,
    SeedChooser,
    ReadySetPlantAudio,
    ModeSelect,
    Day,
    GameOver,
    GameLost,
    GameLostAudio,
    GameWon,
    FinalFanfare,
    Pickups,
    PrizeChime,
    PrizeCollection,
    AwardCollectionAudio,
    LootDropAudio,
    LootChallengeAudio,
    WeatherAudio,
    SunPickupCollection,
    GoldCoinLanding,
    PickupArrowParticles,
    DiamondCollection,
    UsableSeedCollection,
    SunProduction,
    PlantFiring,
    PlantingAudio,
    WallnutBowlingAudio,
    WallnutBowlingImpactAudio,
    PlanternAudio,
    Torchwood,
    GardenWater,
    GardenFertilize,
    GardenFulfill,
    GardenBugSprayAudio,
    GardenPhonographAudio,
    GardenLeaveAudio,
    AquariumTapGlass,
    GardenTreeGrow,
    HugeWaveSound,
    FinalWaveSound,
    ZombiquariumSnorkel,
    ZombiquariumBrain,
    ZombiquariumDeath,
    WhackAudio,
    WhackRiseParticle,
    BodyPartAudio,
    HiddenCodeEffects,
    BungeeAudio,
    BungeeLiftAudio,
    BungeeGrassstepAudio,
    VehicleExplosion,
    MowerVehicleExplosion,
    ZombieFallingAudio,
    DancerRumble,
    GarlicYuckAudio,
    MowerHitAudio,
    MowerSquishAudio,
    FirstWaveSound,
    FlagWaveSound,
    BossAttack,
    BossRVAudio,
    BossStompAudio,
    BossDamageAudio,
    IceShroom,
    IceShroomParticle,
    PotatoMine,
    PotatoMineRiseParticle,
    PotatoMineParticle,
    ScreenFlashParticle,
    ExplosionPlants,
    PlantExplosionParticles,
    ExplodeONut,
    Squash,
    SquashHum,
    ZombieDeploy,
    BrainEaten,
    ImpThrow,
    NewspaperRip,
    NewspaperRarrghAudio,
    Butter,
    ProjectileImpacts,
    ProjectileParticleTail,
    ProjectileParticleFade,
    VaseBreak,
    Rake,
    BloverChomper,
    HypnoJackbox,
    JackboxAudio,
    JackboxBoingAudio,
    CobCannon,
    Portal,
    GraveBuster,
    Coffee,
    TangleKelp,
    DolphinJump,
    PoolEntry,
    PoolRiseParticle,
    PoolMower,
    GraveRumbleAudio,
    LadderAudio,
    Spikeweed,
    Digger,
    Magnet,
    ShieldHit,
    Zamboni,
    Catapult,
    BalloonAppearance,
    BalloonPopAudio,
    PoleVault,
    PogoBlock,
    PogoBounce,
    UmbrellaDeflect,
}

impl From<Checkpoint> for SceneKind {
    fn from(checkpoint: Checkpoint) -> Self {
        match checkpoint {
            Checkpoint::Title => Self::Title,
            Checkpoint::AdventureSelect => Self::AdventureSelect,
            Checkpoint::Store => Self::AdventureSelect,
            Checkpoint::PauseMenu => Self::Day,
            Checkpoint::Options => Self::AdventureSelect,
            Checkpoint::Help => Self::AdventureSelect,
            Checkpoint::Almanac => Self::AdventureSelect,
            Checkpoint::Complete | Checkpoint::CompletePaper => Self::Complete,
            Checkpoint::Credits | Checkpoint::CreditsParticles => Self::Complete,
            Checkpoint::AdventureTutorial => Self::AdventureTutorial,
            Checkpoint::SeedChooser => Self::SeedChooser,
            Checkpoint::ReadySetPlantAudio => Self::Day,
            Checkpoint::ModeSelect => Self::ModeSelect,
            Checkpoint::Day => Self::Day,
            Checkpoint::GameOver => Self::GameOver,
            Checkpoint::GameLost | Checkpoint::GameLostAudio => Self::Day,
            Checkpoint::GameWon => Self::Day,
            Checkpoint::FinalFanfare => Self::Day,
            Checkpoint::Pickups => Self::Day,
            Checkpoint::PrizeChime => Self::Day,
            Checkpoint::PrizeCollection => Self::Day,
            Checkpoint::AwardCollectionAudio => Self::Day,
            Checkpoint::LootDropAudio => Self::Day,
            Checkpoint::LootChallengeAudio => Self::Night,
            Checkpoint::WeatherAudio => Self::Night,
            Checkpoint::SunPickupCollection => Self::Day,
            Checkpoint::GoldCoinLanding => Self::Day,
            Checkpoint::PickupArrowParticles => Self::Day,
            Checkpoint::DiamondCollection => Self::Day,
            Checkpoint::UsableSeedCollection => Self::Day,
            Checkpoint::SunProduction => Self::Day,
            Checkpoint::PlantFiring => Self::Night,
            Checkpoint::PlantingAudio => Self::Pool,
            Checkpoint::WallnutBowlingAudio | Checkpoint::WallnutBowlingImpactAudio => Self::Day,
            Checkpoint::PlanternAudio => Self::Night,
            Checkpoint::Torchwood => Self::Day,
            Checkpoint::GardenWater
            | Checkpoint::GardenFertilize
            | Checkpoint::GardenFulfill
            | Checkpoint::GardenBugSprayAudio
            | Checkpoint::GardenPhonographAudio
            | Checkpoint::GardenLeaveAudio
            | Checkpoint::AquariumTapGlass
            | Checkpoint::GardenTreeGrow => Self::Garden,
            Checkpoint::HugeWaveSound => Self::Day,
            Checkpoint::FinalWaveSound => Self::Day,
            Checkpoint::ZombiquariumSnorkel
            | Checkpoint::ZombiquariumBrain
            | Checkpoint::ZombiquariumDeath => Self::Pool,
            Checkpoint::WhackAudio
            | Checkpoint::WhackRiseParticle
            | Checkpoint::BodyPartAudio
            | Checkpoint::BungeeAudio
            | Checkpoint::BungeeLiftAudio
            | Checkpoint::BungeeGrassstepAudio
            | Checkpoint::VehicleExplosion
            | Checkpoint::MowerVehicleExplosion
            | Checkpoint::ZombieFallingAudio
            | Checkpoint::HiddenCodeEffects => Self::Day,
            Checkpoint::DancerRumble => Self::Day,
            Checkpoint::GarlicYuckAudio => Self::Day,
            Checkpoint::MowerHitAudio => Self::Day,
            Checkpoint::MowerSquishAudio => Self::Boss,
            Checkpoint::FirstWaveSound => Self::Day,
            Checkpoint::FlagWaveSound => Self::Day,
            Checkpoint::BossAttack => Self::Boss,
            Checkpoint::BossRVAudio => Self::Boss,
            Checkpoint::BossStompAudio => Self::Boss,
            Checkpoint::BossDamageAudio => Self::Boss,
            Checkpoint::IceShroom | Checkpoint::IceShroomParticle => Self::Night,
            Checkpoint::PotatoMine
            | Checkpoint::PotatoMineRiseParticle
            | Checkpoint::PotatoMineParticle
            | Checkpoint::ScreenFlashParticle => Self::Day,
            Checkpoint::ExplosionPlants | Checkpoint::PlantExplosionParticles => Self::Night,
            Checkpoint::ExplodeONut => Self::Day,
            Checkpoint::Squash => Self::Day,
            Checkpoint::SquashHum => Self::Day,
            Checkpoint::ZombieDeploy => Self::Night,
            Checkpoint::BrainEaten => Self::Night,
            Checkpoint::ImpThrow => Self::Day,
            Checkpoint::NewspaperRip | Checkpoint::NewspaperRarrghAudio => Self::Day,
            Checkpoint::Butter => Self::Day,
            Checkpoint::ProjectileImpacts
            | Checkpoint::ProjectileParticleTail
            | Checkpoint::ProjectileParticleFade => Self::Day,
            Checkpoint::VaseBreak => Self::Day,
            Checkpoint::Rake => Self::Day,
            Checkpoint::BloverChomper => Self::Day,
            Checkpoint::HypnoJackbox => Self::Night,
            Checkpoint::JackboxAudio | Checkpoint::JackboxBoingAudio => Self::Day,
            Checkpoint::CobCannon => Self::Day,
            Checkpoint::Portal => Self::Day,
            Checkpoint::GraveBuster => Self::Day,
            Checkpoint::Coffee => Self::Day,
            Checkpoint::TangleKelp => Self::Pool,
            Checkpoint::DolphinJump => Self::Pool,
            Checkpoint::PoolEntry | Checkpoint::PoolRiseParticle => Self::Pool,
            Checkpoint::PoolMower => Self::Pool,
            Checkpoint::GraveRumbleAudio => Self::Night,
            Checkpoint::LadderAudio => Self::Day,
            Checkpoint::Spikeweed => Self::Day,
            Checkpoint::Digger => Self::Day,
            Checkpoint::Magnet => Self::Night,
            Checkpoint::ShieldHit => Self::Day,
            Checkpoint::Zamboni => Self::Day,
            Checkpoint::Catapult => Self::Day,
            Checkpoint::BalloonAppearance | Checkpoint::BalloonPopAudio => Self::Day,
            Checkpoint::PoleVault => Self::Day,
            Checkpoint::PogoBlock => Self::Day,
            Checkpoint::PogoBounce => Self::Day,
            Checkpoint::UmbrellaDeflect => Self::Day,
        }
    }
}

const SIMULATION_STEP: Duration = Duration::from_millis(10);
// Pixel-measured against the accepted original capture and the 1.0.0.1051
// LoadingPage_Draw decompilation: grass/clip at x=240, dirt at x=244, the
// resting start-button Y is 534 (BOUNCE clamps to the curve start at t>=1).
const TITLE_LOAD_BAR_X: f32 = 240.0;
const TITLE_LOAD_BAR_DIRT_X: f32 = 244.0;
const TITLE_START_BUTTON_Y: f32 = 534.0;
const TITLE_START_BUTTON_WIDTH: f32 = 314.0;
const TITLE_START_BUTTON_HEIGHT: f32 = 50.0;
const TITLE_START_SOUND_PATH: &str = "sounds/buttonclick.ogg";
const GAME_LOST_GRAPHIC_START: u32 = 6_000;
const GAME_LOST_DIALOG_TIME: u32 = 11_000;
const CREDITS_ANIM_RATE: f32 = 0.3;
const CREDITS_MAIN1_END_FRAME: f32 = 400.0;
const CREDITS_MAIN2_END_FRAME: f32 = 785.0;
const CREDITS_MAIN3_END_FRAME: f32 = 1033.0;
const CREDITS_END_BUTTON_DELAY_UPDATES: u32 = 50;
const CREDITS_MAIN1_STROBE_FRAMES: &[f32] =
    &[128.0, 130.0, 132.0, 134.0, 136.0, 138.0, 140.0, 142.0];
const CREDITS_MAIN2_STROBE_FRAMES: &[f32] = &[
    111.5, 115.5, 119.5, 121.5, 123.5, 125.5, 127.5, 131.5, 135.5, 139.5, 143.5, 147.5, 151.5,
    155.5, 159.5, 163.5, 167.5, 171.5, 175.5, 179.5, 183.5, 187.5, 191.5, 195.5, 199.5, 203.5,
    207.5, 211.5, 215.5, 219.5, 223.5, 227.5, 231.5, 235.5, 239.5, 243.5, 342.0,
];
const CREDITS_MAIN3_STROBE_FRAMES: &[f32] = &[
    111.0, 115.0, 119.0, 121.0, 123.0, 219.0, 223.0, 227.0, 231.0, 235.0, 239.0, 243.0, 247.0,
];
const JALAPENO_FIRE_DURATION: u64 = 100;
const PROJECTILE_FIRE_DURATION: u64 = 24;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CreditsParticleCue {
    Spawn(&'static str),
    StartFog(&'static str),
    StopFog,
}

fn credits_end_update_count() -> u32 {
    (CREDITS_MAIN3_END_FRAME / CREDITS_ANIM_RATE).ceil() as u32
}

fn credits_phase_at(update_count: u32) -> (u8, f32) {
    let frame = update_count as f32 * CREDITS_ANIM_RATE;
    if frame < CREDITS_MAIN1_END_FRAME {
        (0, frame)
    } else if frame < CREDITS_MAIN2_END_FRAME {
        (1, frame - CREDITS_MAIN1_END_FRAME)
    } else if frame < CREDITS_MAIN3_END_FRAME {
        (2, frame - CREDITS_MAIN2_END_FRAME)
    } else {
        (3, frame - CREDITS_MAIN3_END_FRAME)
    }
}

fn credits_particle_cues(previous_update: u32, current_update: u32) -> Vec<CreditsParticleCue> {
    let (previous_phase, previous_frame) = credits_phase_at(previous_update);
    let (current_phase, current_frame) = credits_phase_at(current_update);
    if previous_phase != current_phase {
        return Vec::new();
    }
    let crossed = |target| previous_frame < target && current_frame >= target;
    let mut cues = Vec::new();
    match current_phase {
        0 => {
            cues.extend(
                CREDITS_MAIN1_STROBE_FRAMES
                    .iter()
                    .filter(|target| crossed(**target))
                    .map(|_| CreditsParticleCue::Spawn("Credits_Strobe")),
            );
            if crossed(136.5) {
                cues.push(CreditsParticleCue::Spawn("Credits_RaysWipe"));
            }
        }
        1 => {
            if crossed(120.0) {
                cues.push(CreditsParticleCue::Spawn("Credits_ZombieHeadWipe"));
            }
            cues.extend(
                CREDITS_MAIN2_STROBE_FRAMES
                    .iter()
                    .filter(|target| crossed(**target))
                    .map(|_| CreditsParticleCue::Spawn("Credits_Strobe")),
            );
            if crossed(188.0) {
                cues.push(CreditsParticleCue::StartFog("Credits_fog"));
            }
            if crossed(248.0) {
                cues.push(CreditsParticleCue::StopFog);
            }
        }
        2 => {
            if crossed(120.0) {
                cues.push(CreditsParticleCue::Spawn("Credits_ZombieHeadWipe"));
            }
            cues.extend(
                CREDITS_MAIN3_STROBE_FRAMES
                    .iter()
                    .filter(|target| crossed(**target))
                    .map(|_| CreditsParticleCue::Spawn("Credits_Strobe")),
            );
        }
        _ => {}
    }
    cues
}

fn credits_fog_x(definition: Option<&ReanimatorDefinition>, phase_frame: f32) -> Option<f32> {
    let track = definition?
        .tracks
        .iter()
        .find(|track| track.name.eq_ignore_ascii_case("Background2"))?;
    Some(reanim_transform_at_any(track, phase_frame)?.x + 856.0)
}

fn credits_scroll_offset(update_count: u32) -> f32 {
    update_count.saturating_sub(credits_end_update_count()) as f32 * CREDITS_ANIM_RATE
}

fn credits_controls_visible(update_count: u32) -> bool {
    update_count >= credits_end_update_count() + CREDITS_END_BUTTON_DELAY_UPDATES
}

fn game_lost_cutscene_active(cutscene_time: Option<u32>) -> bool {
    cutscene_time.is_some_and(|time| time < GAME_LOST_DIALOG_TIME)
}

fn credits_replay_contains(x: f32, y: f32) -> bool {
    (10.0..135.0).contains(&x) && (530.0..595.0).contains(&y)
}

fn credits_main_menu_contains(x: f32, y: f32) -> bool {
    (298.0..507.0).contains(&x) && (554.0..600.0).contains(&y)
}

fn title_start_contains(x: f32, y: f32, button_y: f32) -> bool {
    (TITLE_LOAD_BAR_X..TITLE_LOAD_BAR_X + TITLE_START_BUTTON_WIDTH).contains(&x)
        && (button_y..button_y + TITLE_START_BUTTON_HEIGHT).contains(&y)
}

// The target executable uses 0.906 for the final trigger.
const TITLE_TRIGGER_FRACTIONS: [f32; 5] = [0.11, 0.32, 0.54, 0.72, 0.906];
const TITLE_REANIM_RATE: f32 = 18.0;

fn tod_curve_bounce(t: f32) -> f32 {
    1.0 - (2.0 * t - 1.0).abs()
}

#[derive(Clone, Debug)]
struct TitleLoadState {
    logo_counter: u32,
    counter: u32,
    tick: u32,
    bar_width: f32,
    bar_vel: f32,
    trigger_ticks: [Option<u32>; 5],
    label_complete: bool,
    pressed_button: Option<MouseButton>,
}

impl TitleLoadState {
    fn new() -> Self {
        // Resources load before the first frame, so the source's estimated
        // load time clamps to 100 ticks and mBarVel starts at 314/100.
        Self {
            logo_counter: 200,
            counter: 100,
            tick: 0,
            bar_width: 0.0,
            bar_vel: TITLE_START_BUTTON_WIDTH / 100.0,
            trigger_ticks: [None; 5],
            label_complete: false,
            pressed_button: None,
        }
    }

    fn completed() -> Self {
        let mut state = Self::new();
        while !state.label_complete {
            state.advance();
        }
        // Let the one-second load-bar reanimations reach their hold frames.
        state.tick += 100;
        state
    }

    fn ready(&self) -> bool {
        // Resources are loaded before App is created. The original enables
        // input on the first fill update that observes loader completion.
        self.tick > 0
    }

    fn mouse_press(&mut self, button: MouseButton, over_start: bool) -> bool {
        if !self.ready() {
            return false;
        }
        self.pressed_button = over_start.then_some(button);
        true
    }

    fn mouse_release(&mut self, button: MouseButton, over_start: bool) -> bool {
        if self.pressed_button != Some(button) {
            return false;
        }
        self.pressed_button = None;
        over_start && self.ready()
    }

    fn logo_alpha(&self) -> f32 {
        ((200 - self.logo_counter).min(self.logo_counter) as f32 / 50.0).min(1.0)
    }

    /// Start-button Y: EASE_IN 650->534 over counter 60..10, then BOUNCE
    /// 534<->529 over counter 10..0 (rests at the curve start 534).
    fn button_y(&self) -> f32 {
        if self.counter > 10 {
            let t = ((60.0 - self.counter as f32) / 50.0).clamp(0.0, 1.0);
            (650.0 - (650.0 - TITLE_START_BUTTON_Y) * t * t).round()
        } else {
            let t = (10.0 - self.counter as f32) / 10.0;
            (TITLE_START_BUTTON_Y - 5.0 * tod_curve_bounce(t)).round()
        }
    }

    /// Logo Y: EASE_IN -150->10 over counter 100..60, then BOUNCE 10<->15
    /// over counter 60..50 (rests at 10).
    fn logo_y(&self) -> f32 {
        if self.counter > 60 {
            let t = ((100.0 - self.counter as f32) / 40.0).clamp(0.0, 1.0);
            (-150.0 + 160.0 * t * t).round()
        } else {
            let t = ((60.0 - self.counter as f32) / 10.0).clamp(0.0, 1.0);
            (10.0 + 5.0 * tod_curve_bounce(t)).round()
        }
    }

    /// Advances one 10ms tick and returns the trigger indices whose loading
    /// bar crossings fired this tick (each plays loadingbar_flower, index 4
    /// also plays loadingbar_zombie).
    fn advance(&mut self) -> Vec<usize> {
        if self.logo_counter > 0 {
            self.logo_counter -= 1;
            return Vec::new();
        }
        if self.counter > 0 {
            self.counter -= 1;
            if self.counter > 0 {
                return Vec::new();
            }
        }
        self.tick += 1;
        let previous_width = self.bar_width;
        self.bar_width += self.bar_vel;
        if self.bar_width > TITLE_START_BUTTON_WIDTH {
            self.label_complete = true;
            self.bar_width = TITLE_START_BUTTON_WIDTH;
        }
        let diff = TITLE_START_BUTTON_WIDTH - self.bar_width;
        self.bar_vel = (self.bar_vel + diff * diff.abs() * 0.0001).clamp(0.01, 2.0);
        let mut fired = Vec::new();
        for (index, &fraction) in TITLE_TRIGGER_FRACTIONS.iter().enumerate() {
            let trigger_point = TITLE_START_BUTTON_WIDTH * fraction;
            if previous_width < trigger_point && self.bar_width >= trigger_point {
                self.trigger_ticks[index] = Some(self.tick);
                fired.push(index);
            }
        }
        fired
    }
}

fn push_title_load_bar_reanimations(
    frame: &mut RenderFrame,
    state: &TitleLoadState,
    catalog: &ReanimCatalog,
) {
    for (index, &fraction) in TITLE_TRIGGER_FRACTIONS.iter().enumerate() {
        let Some(fire_tick) = state.trigger_ticks[index] else {
            continue;
        };
        let x = TITLE_START_BUTTON_WIDTH * fraction + 225.0 + if index == 4 { -20.0 } else { 0.0 };
        let y = 511.0 - if index == 2 { 5.0 } else { 0.0 };
        let (overlay_scale_x, overlay_scale_y) = match index {
            1 | 3 => (-1.0, 1.0),
            2 => (1.1, 1.3),
            _ => (1.0, 1.0),
        };
        let definition = if index == 4 {
            catalog.loadbar_zombiehead.as_ref()
        } else {
            catalog.loadbar_sprout.as_ref()
        };
        if let Some(definition) = definition {
            let frame_count = definition
                .tracks
                .first()
                .map_or(0, |track| track.transforms.len());
            if frame_count == 0 {
                continue;
            }
            let progress = (state.tick.saturating_sub(fire_tick) as f32 * 0.01 * TITLE_REANIM_RATE
                / frame_count as f32)
                .min(1.0);
            let position = progress * (frame_count - 1) as f32;
            for track in &definition.tracks {
                let Some(transform) = reanim_transform_at(track, position) else {
                    continue;
                };
                let Some(image) = transform.image.as_deref() else {
                    continue;
                };
                let Some(&resource_id) = catalog.image_ids.get(&image.to_ascii_uppercase()) else {
                    continue;
                };
                let skew_x = -transform.skew_x.to_radians();
                let skew_y = -transform.skew_y.to_radians();
                frame.affine_sprites.push(AffineSpriteCommand {
                    resource_id,
                    x: x + overlay_scale_x * transform.x - 0.5,
                    y: y + overlay_scale_y * transform.y - 0.5,
                    m00: overlay_scale_x * skew_x.cos() * transform.scale_x,
                    m01: overlay_scale_x * skew_y.sin() * transform.scale_y,
                    m10: overlay_scale_y * -skew_x.sin() * transform.scale_x,
                    m11: overlay_scale_y * skew_y.cos() * transform.scale_y,
                    z: 4,
                    alpha: transform.alpha,
                    tint: [1.0; 3],
                    blend_mode: BlendMode::Alpha,
                    source: Some(AffineSpriteSource {
                        uv_min: [0.0; 2],
                        uv_max: [1.0; 2],
                        pivot_uv: [0.0; 2],
                    }),
                });
            }
        }
    }
}

fn main() -> ExitCode {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let profile_path = cli.profile;
    let capture_path = cli.capture;
    let mut profile = match profile_path.as_deref() {
        Some(path) => match load_profile(path) {
            Ok(profile) => Some(profile),
            Err(error) => {
                tracing::error!(%error, "profile load failed");
                return ExitCode::FAILURE;
            }
        },
        None => None,
    };

    let explicit = cli.data_dir.as_deref().or(cli.pak.as_deref());
    let layout = match AssetLayout::discover(explicit) {
        Ok(layout) => layout,
        Err(error) => {
            tracing::error!(%error, "resource discovery failed");
            return ExitCode::FAILURE;
        }
    };
    tracing::info!(source = ?layout.source, "resource source selected");

    let resources = match ResourceProvider::open(&layout.source) {
        Ok(resources) => resources,
        Err(error) => {
            tracing::error!(%error, "resource source opening failed");
            return ExitCode::FAILURE;
        }
    };
    let inventory = match resources.inventory() {
        Ok(inventory) => inventory,
        Err(error) => {
            tracing::error!(%error, "resource inventory failed");
            return ExitCode::FAILURE;
        }
    };
    let Some(version) = inventory.version() else {
        tracing::error!(
            groups = inventory.groups,
            entries = inventory.entries,
            images = inventory.images,
            fonts = inventory.fonts,
            sounds = inventory.sounds,
            compiled_animations = inventory.compiled_animations,
            music = inventory.music,
            "unsupported resource inventory"
        );
        return ExitCode::FAILURE;
    };
    tracing::info!(
        version,
        groups = inventory.groups,
        entries = inventory.entries,
        images = inventory.images,
        fonts = inventory.fonts,
        sounds = inventory.sounds,
        compiled_animations = inventory.compiled_animations,
        music = inventory.music,
        "resource inventory verified"
    );

    let force_game_over = matches!(cli.checkpoint, Some(Checkpoint::GameOver));
    let force_game_lost = matches!(
        cli.checkpoint,
        Some(Checkpoint::GameLost | Checkpoint::GameLostAudio)
    );
    let force_game_won = matches!(cli.checkpoint, Some(Checkpoint::GameWon));
    let force_pickups = matches!(cli.checkpoint, Some(Checkpoint::Pickups));
    let fullscreen = cli.fullscreen
        || profile
            .as_ref()
            .is_some_and(|profile| profile.settings.fullscreen);
    let initial_scene = if force_game_over || force_game_lost || force_game_won || force_pickups {
        SceneKind::Day
    } else {
        cli.checkpoint
            .map(SceneKind::from)
            .unwrap_or(SceneKind::Title)
    };
    let loaded_assets = match load_assets(&resources) {
        Ok(assets) => assets,
        Err(error) => {
            tracing::error!(%error, "required display resources failed to load");
            return ExitCode::FAILURE;
        }
    };
    let mut audio = match KiraAudioBackend::new() {
        Ok(audio) => Some(audio),
        Err(error) => {
            tracing::warn!(%error, "audio backend unavailable; continuing without audio");
            None
        }
    };
    if let Some(audio) = &mut audio {
        for path in [
            TITLE_START_SOUND_PATH,
            "sounds/loadingbar_flower.ogg",
            "sounds/loadingbar_zombie.ogg",
        ] {
            let preload = resources
                .read(path)
                .map_err(|error| error.to_string())
                .and_then(|bytes| {
                    audio
                        .preload_bytes(path, bytes)
                        .map_err(|error| error.to_string())
                });
            if let Err(error) = preload {
                tracing::warn!(%error, path, "title sound preload failed");
            }
        }
    }

    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(error) => {
            tracing::error!(%error, "event loop creation failed");
            return ExitCode::FAILURE;
        }
    };
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(
        loaded_assets,
        resources,
        audio,
        initial_scene,
        fullscreen,
        cli.checkpoint,
        profile.take(),
    );
    app.capture_path = capture_path;
    let run_result = event_loop.run_app(&mut app);

    if let Err(error) = run_result {
        tracing::error!(%error, "event loop failed");
        return ExitCode::FAILURE;
    }

    if app.capture_path.is_some() {
        return ExitCode::FAILURE;
    }

    app.sync_profile_settings();
    if let (Some(path), Some(mut profile)) = (profile_path, app.profile.take()) {
        app.game.update_profile(&mut profile);
        if let Err(error) = profile.write_atomic(&path) {
            tracing::error!(%error, "profile save failed");
            return ExitCode::FAILURE;
        }
        tracing::info!(path = ?path, "profile saved");
    }

    ExitCode::SUCCESS
}

fn load_profile(path: &Path) -> Result<SaveProfile, SaveError> {
    match SaveProfile::read(path) {
        Ok(profile) => Ok(profile),
        Err(SaveError::Io(error)) if error.kind() == ErrorKind::NotFound => {
            Ok(SaveProfile::new("default"))
        }
        Err(error) => Err(error),
    }
}

fn apply_profile_to_game(game: &mut Game, profile: Option<&SaveProfile>) {
    if let Some(profile) = profile {
        game.apply_profile(profile);
    }
}

fn carry_profile(current: &Game, next: &mut Game, profile: Option<&mut SaveProfile>) {
    next.carry_session_state_from(current);
    if let Some(profile) = profile {
        current.update_profile(profile);
        next.apply_profile(profile);
    }
}

const REANIM_IMAGE_ID_BASE: u32 = 10_000;
const PARTICLE_IMAGE_ID_BASE: u32 = 20_000;

#[derive(Clone, Copy, Debug)]
struct ParticleImage {
    resource_id: u32,
    columns: u32,
    rows: u32,
}

const SPECIALIZED_REANIM_FILES: &[(ZombieType, &str)] = &[
    (ZombieType::Zamboni, "Zombie_zamboni.reanim.compiled"),
    // The target archive uses Zombie_dancer; the source loader calls it disco.
    (ZombieType::Dancer, "Zombie_dancer.reanim.compiled"),
    (
        ZombieType::PoleVaulter,
        "Zombie_polevaulter.reanim.compiled",
    ),
    (ZombieType::Balloon, "Zombie_balloon.reanim.compiled"),
    (ZombieType::Gargantuar, "Zombie_gargantuar.reanim.compiled"),
    (ZombieType::Imp, "Zombie_imp.reanim.compiled"),
    (ZombieType::Digger, "Zombie_digger.reanim.compiled"),
    (
        ZombieType::DolphinRider,
        "Zombie_dolphinrider.reanim.compiled",
    ),
    (ZombieType::Pogo, "Zombie_pogo.reanim.compiled"),
    (ZombieType::Bobsled, "Zombie_bobsled.reanim.compiled"),
    (ZombieType::Jackbox, "Zombie_jackbox.reanim.compiled"),
    (ZombieType::Snorkel, "Zombie_snorkle.reanim.compiled"),
    (ZombieType::Bungee, "Zombie_bungi.reanim.compiled"),
    (ZombieType::Catapult, "Zombie_catapult.reanim.compiled"),
    (ZombieType::Ladder, "Zombie_ladder.reanim.compiled"),
    (ZombieType::Yeti, "Zombie_yeti.reanim.compiled"),
];

const PLANT_REANIM_FILES: &[(PlantType, &str)] = &[
    (PlantType::Peashooter, "PeaShooterSingle.reanim.compiled"),
    (PlantType::Sunflower, "SunFlower.reanim.compiled"),
    (PlantType::Other(2), "CherryBomb.reanim.compiled"),
    (PlantType::Other(3), "Wallnut.reanim.compiled"),
    (PlantType::Other(4), "PotatoMine.reanim.compiled"),
    (PlantType::Other(5), "SnowPea.reanim.compiled"),
    (PlantType::Other(6), "Chomper.reanim.compiled"),
    (PlantType::Other(7), "PeaShooter.reanim.compiled"),
    (PlantType::Other(8), "Puffshroom.reanim.compiled"),
    (PlantType::Other(9), "SunShroom.reanim.compiled"),
    (PlantType::Other(10), "Fumeshroom.reanim.compiled"),
    (PlantType::Other(11), "Gravebuster.reanim.compiled"),
    (PlantType::Other(12), "Hypnoshroom.reanim.compiled"),
    (PlantType::Other(13), "ScaredyShroom.reanim.compiled"),
    (PlantType::Other(14), "Iceshroom.reanim.compiled"),
    (PlantType::Other(15), "DoomShroom.reanim.compiled"),
    (PlantType::Other(16), "Lilypad.reanim.compiled"),
    (PlantType::Other(17), "Squash.reanim.compiled"),
    (PlantType::Other(18), "ThreePeater.reanim.compiled"),
    (PlantType::Other(19), "Tanglekelp.reanim.compiled"),
    (PlantType::Other(20), "Jalapeno.reanim.compiled"),
    (PlantType::Other(21), "Caltrop.reanim.compiled"),
    (PlantType::Other(22), "Torchwood.reanim.compiled"),
    (PlantType::Other(23), "Tallnut.reanim.compiled"),
    (PlantType::Other(24), "SeaShroom.reanim.compiled"),
    (PlantType::Other(25), "Plantern.reanim.compiled"),
    (PlantType::Other(26), "Cactus.reanim.compiled"),
    (PlantType::Other(27), "Blover.reanim.compiled"),
    (PlantType::Other(28), "SplitPea.reanim.compiled"),
    (PlantType::Other(29), "Starfruit.reanim.compiled"),
    (PlantType::Other(30), "Pumpkin.reanim.compiled"),
    (PlantType::Other(31), "Magnetshroom.reanim.compiled"),
    (PlantType::Other(32), "Cabbagepult.reanim.compiled"),
    (PlantType::Other(33), "Pot.reanim.compiled"),
    (PlantType::Other(34), "Cornpult.reanim.compiled"),
    (PlantType::Other(35), "Coffeebean.reanim.compiled"),
    (PlantType::Other(36), "Garlic.reanim.compiled"),
    (PlantType::Other(37), "Umbrellaleaf.reanim.compiled"),
    (PlantType::Other(38), "Marigold.reanim.compiled"),
    (PlantType::Other(39), "Melonpult.reanim.compiled"),
    (PlantType::Other(40), "GatlingPea.reanim.compiled"),
    (PlantType::Other(41), "TwinSunFlower.reanim.compiled"),
    (PlantType::Other(42), "GloomShroom.reanim.compiled"),
    (PlantType::Other(43), "Cattail.reanim.compiled"),
    (PlantType::Other(44), "WinterMelon.reanim.compiled"),
    (PlantType::Other(45), "GoldMagnet.reanim.compiled"),
    (PlantType::Other(46), "SpikeRock.reanim.compiled"),
    (PlantType::Other(47), "CobCannon.reanim.compiled"),
    (PlantType::Other(48), "Imitater.reanim.compiled"),
];

#[derive(Clone, Debug, Default)]
struct ReanimCatalog {
    zombie: Option<ReanimatorDefinition>,
    football: Option<ReanimatorDefinition>,
    newspaper: Option<ReanimatorDefinition>,
    boss: Option<ReanimatorDefinition>,
    boss_driver: Option<ReanimatorDefinition>,
    boss_fireball: Option<ReanimatorDefinition>,
    boss_iceball: Option<ReanimatorDefinition>,
    fire: Option<ReanimatorDefinition>,
    sun: Option<ReanimatorDefinition>,
    coin_silver: Option<ReanimatorDefinition>,
    coin_gold: Option<ReanimatorDefinition>,
    diamond: Option<ReanimatorDefinition>,
    credits_main2: Option<ReanimatorDefinition>,
    loadbar_sprout: Option<ReanimatorDefinition>,
    loadbar_zombiehead: Option<ReanimatorDefinition>,
    specialized: Vec<(ZombieType, ReanimatorDefinition)>,
    plants: Vec<(PlantType, ReanimatorDefinition)>,
    image_ids: HashMap<String, u32>,
}

type LoadedAssets = (
    Vec<ImageAsset>,
    ReanimCatalog,
    ParticleCatalog,
    HashMap<String, ParticleImage>,
);

fn load_assets(resources: &ResourceProvider) -> Result<LoadedAssets, String> {
    let mut assets = vec![
        load_image(resources, TITLE_IMAGE_ID, "images/titlescreen.jpg")?,
        load_image(
            resources,
            TITLE_POPCAP_LOGO_IMAGE_ID,
            "images/PopCap_Logo.jpg",
        )?,
        load_title_logo(resources)?,
        load_image(
            resources,
            TITLE_LOAD_BAR_DIRT_IMAGE_ID,
            "images/LoadBar_dirt.png",
        )?,
        load_image(
            resources,
            TITLE_LOAD_BAR_GRASS_IMAGE_ID,
            "images/LoadBar_grass.png",
        )?,
        load_image(
            resources,
            TITLE_LOAD_BAR_ROCK1_IMAGE_ID,
            "reanim/PotatoMine_rock1.png",
        )?,
        load_image(
            resources,
            TITLE_LOAD_BAR_ROCK3_IMAGE_ID,
            "reanim/PotatoMine_rock3.png",
        )?,
        load_image(
            resources,
            TITLE_LOAD_BAR_SPROUT_BODY_IMAGE_ID,
            "reanim/sprout_body.png",
        )?,
        load_image(
            resources,
            TITLE_LOAD_BAR_SPROUT_PETAL_IMAGE_ID,
            "reanim/sprout_petal.png",
        )?,
        load_image(
            resources,
            TITLE_LOAD_BAR_ZOMBIE_HEAD_IMAGE_ID,
            "reanim/Zombie_head.png",
        )?,
        load_image(
            resources,
            TITLE_LOAD_BAR_ZOMBIE_HAIR_IMAGE_ID,
            "reanim/Zombie_hair.png",
        )?,
        load_image(
            resources,
            TITLE_LOAD_BAR_ZOMBIE_JAW_IMAGE_ID,
            "reanim/Zombie_jaw.png",
        )?,
        load_image(
            resources,
            TITLE_SOD_ROLL_CAP_IMAGE_ID,
            "reanim/SodRollCap.png",
        )?,
        render_colored_text_image(
            TITLE_LOADING_PROMPT_SHADOW_IMAGE_ID,
            "\u{8f7d}\u{5165}\u{4e2d}\u{2026}\u{2026}",
            120,
            24,
            19,
            [71, 45, 0],
        )?,
        render_colored_text_image(
            TITLE_LOADING_PROMPT_IMAGE_ID,
            "\u{8f7d}\u{5165}\u{4e2d}\u{2026}\u{2026}",
            120,
            24,
            19,
            [218, 184, 33],
        )?,
        render_colored_text_image(
            TITLE_LOADING_PROMPT_HOVER_IMAGE_ID,
            "\u{8f7d}\u{5165}\u{4e2d}\u{2026}\u{2026}",
            120,
            24,
            19,
            [250, 90, 15],
        )?,
        render_colored_text_image(
            TITLE_START_PROMPT_SHADOW_IMAGE_ID,
            "\u{70b9}\u{51fb}\u{5f00}\u{59cb}",
            120,
            24,
            19,
            [71, 45, 0],
        )?,
        render_colored_text_image(
            TITLE_START_PROMPT_IMAGE_ID,
            "\u{70b9}\u{51fb}\u{5f00}\u{59cb}",
            120,
            24,
            19,
            [218, 184, 33],
        )?,
        render_colored_text_image(
            TITLE_START_PROMPT_HOVER_IMAGE_ID,
            "\u{70b9}\u{51fb}\u{5f00}\u{59cb}",
            120,
            24,
            19,
            [250, 90, 15],
        )?,
        load_image(
            resources,
            SELECTOR_BASE_IMAGE_ID,
            "reanim/SelectorScreen_BG.jpg",
        )?,
        load_masked_image(
            resources,
            SELECTOR_LEFT_IMAGE_ID,
            "reanim/SelectorScreen_BG_Left.jpg",
            "reanim/SelectorScreen_BG_Left_.png",
        )?,
        load_masked_image(
            resources,
            SELECTOR_CENTER_IMAGE_ID,
            "reanim/SelectorScreen_BG_Center.jpg",
            "reanim/SelectorScreen_BG_Center_.png",
        )?,
        load_masked_image(
            resources,
            SELECTOR_RIGHT_IMAGE_ID,
            "reanim/SelectorScreen_BG_Right.jpg",
            "reanim/SelectorScreen_BG_Right_.png",
        )?,
        load_image(
            resources,
            SELECTOR_ADVENTURE_IMAGE_ID,
            "reanim/SelectorScreen_Adventure_button.png",
        )?,
        load_image(
            resources,
            SELECTOR_CHALLENGES_IMAGE_ID,
            "reanim/SelectorScreen_Challenges_button.png",
        )?,
        load_image(
            resources,
            SELECTOR_SURVIVAL_IMAGE_ID,
            "reanim/SelectorScreen_Survival_button.png",
        )?,
        load_image(
            resources,
            SELECTOR_VASEBREAKER_IMAGE_ID,
            "reanim/SelectorScreen_Vasebreaker_button.png",
        )?,
        load_image(
            resources,
            SELECTOR_WOODSIGN1_IMAGE_ID,
            "reanim/SelectorScreen_WoodSign1.png",
        )?,
        load_image(
            resources,
            SELECTOR_WOODSIGN2_IMAGE_ID,
            "reanim/SelectorScreen_WoodSign2.png",
        )?,
        load_image(
            resources,
            SELECTOR_WOODSIGN3_IMAGE_ID,
            "reanim/SelectorScreen_WoodSign3.png",
        )?,
        load_image(
            resources,
            SELECTOR_LEAVES_IMAGE_ID,
            "reanim/SelectorScreen_Leaves.png",
        )?,
        load_image(
            resources,
            SELECTOR_ZEN_GARDEN_IMAGE_ID,
            "images/SelectorScreen_ZenGarden.png",
        )?,
        load_image(
            resources,
            SELECTOR_ALMANAC_IMAGE_ID,
            "images/SelectorScreen_Almanac.png",
        )?,
        load_image(
            resources,
            SELECTOR_STORE_IMAGE_ID,
            "images/SelectorScreen_Store.png",
        )?,
        load_image(
            resources,
            SELECTOR_OPTIONS_IMAGE_ID,
            "images/SelectorScreen_Options1.png",
        )?,
        load_image(
            resources,
            SELECTOR_HELP_IMAGE_ID,
            "images/SelectorScreen_Help1.png",
        )?,
        load_image(
            resources,
            SELECTOR_QUIT_IMAGE_ID,
            "images/SelectorScreen_Quit1.png",
        )?,
        load_cropped_image(
            resources,
            SELECTOR_TROPHY_IMAGE_ID,
            "images/Sunflower_trophy.png",
            157,
            0,
            157,
            269,
        )?,
        load_image(
            resources,
            TUTORIAL_BUBBLE_IMAGE_ID,
            "images/Store_SpeechBubble2.png",
        )?,
        load_masked_image(
            resources,
            CRAZY_DAVE_BODY_IMAGE_ID,
            "reanim/CrazyDave_body1.jpg",
            "reanim/CrazyDave_body1_.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_HEAD_IMAGE_ID,
            "reanim/CrazyDave_head.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_BEARD_IMAGE_ID,
            "reanim/CrazyDave_beard.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_POT_IMAGE_ID,
            "reanim/CrazyDave_pot.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_EYE_IMAGE_ID,
            "reanim/CrazyDave_eye.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_EYEBROW_IMAGE_ID,
            "reanim/CrazyDave_eyebrow.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_MOUTH_IMAGE_ID,
            "reanim/CrazyDave_mouth5.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_OUTER_ARM_IMAGE_ID,
            "reanim/CrazyDave_outerarm.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_OUTER_HAND_IMAGE_ID,
            "reanim/CrazyDave_outerhand.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_INNER_ARM_IMAGE_ID,
            "reanim/CrazyDave_innerarm.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_INNER_HAND_IMAGE_ID,
            "reanim/CrazyDave_innerhand.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_INNER_FINGER1_IMAGE_ID,
            "reanim/CrazyDave_innerfinger1.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_INNER_FINGER2_IMAGE_ID,
            "reanim/CrazyDave_innerfinger2.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_INNER_FINGER3_IMAGE_ID,
            "reanim/CrazyDave_innerfinger3.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_INNER_FINGER4_IMAGE_ID,
            "reanim/CrazyDave_innerfinger4.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_OUTER_FINGER1_IMAGE_ID,
            "reanim/CrazyDave_outerfinger1.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_OUTER_FINGER2_IMAGE_ID,
            "reanim/CrazyDave_outerfinger2.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_OUTER_FINGER3_IMAGE_ID,
            "reanim/CrazyDave_outerfinger3.png",
        )?,
        load_image(
            resources,
            CRAZY_DAVE_OUTER_FINGER4_IMAGE_ID,
            "reanim/CrazyDave_outerfinger4.png",
        )?,
        load_dialogue_text(
            TUTORIAL_TEXT1_IMAGE_ID,
            "\u{4f19}\u{8ba1}\u{ff0c}\u{90a3}\u{4e9b}\u{50f5}\u{5c38}\u{8fd8}\u{5728}\u{6e90}\u{6e90}\u{4e0d}\u{65ad}\u{7684}\n\u{6765}\u{88ad}\u{554a}\u{ff01}",
        )?,
        load_dialogue_text(
            TUTORIAL_TEXT2_IMAGE_ID,
            "\u{8fd9}\u{6b21}\u{ff0c}\u{6211}\u{60f3}\u{66ff}\u{4f60}\u{6311}\u{4e9b}\u{690d}\u{7269}\u{ff01}",
        )?,
        load_continue_text(TUTORIAL_CONTINUE_IMAGE_ID)?,
        load_image(
            resources,
            SEED_CHOOSER_IMAGE_ID,
            "images/SeedChooser_Background.png",
        )?,
        load_cropped_image(
            resources,
            SEED_PACKET_NORMAL_IMAGE_ID,
            "images/seeds.png",
            100,
            0,
            50,
            70,
        )?,
        load_image(resources, BOARD_SEED_BANK_IMAGE_ID, "images/SeedBank.png")?,
        load_image(
            resources,
            BOARD_CONVEYOR_BELT_BACKDROP_IMAGE_ID,
            "images/ConveyorBelt_backdrop.png",
        )?,
        load_image(
            resources,
            BOARD_SHOVEL_BANK_IMAGE_ID,
            "images/ShovelBank.png",
        )?,
        load_image(resources, BOARD_SUN_BANK_IMAGE_ID, "images/SunBank.png")?,
        load_cropped_image(
            resources,
            BOARD_PROGRESS_METER_IMAGE_ID,
            "images/FlagMeter.png",
            0,
            0,
            158,
            27,
        )?,
        load_cropped_image(
            resources,
            BOARD_PROGRESS_HEAD_IMAGE_ID,
            "images/FlagMeterParts.png",
            0,
            0,
            25,
            25,
        )?,
        load_cropped_image(
            resources,
            BOARD_PROGRESS_POLE_IMAGE_ID,
            "images/FlagMeterParts.png",
            25,
            0,
            25,
            25,
        )?,
        load_cropped_image(
            resources,
            BOARD_PROGRESS_FLAG_IMAGE_ID,
            "images/FlagMeterParts.png",
            50,
            0,
            25,
            25,
        )?,
        load_image(
            resources,
            BOARD_PROGRESS_LEVEL_IMAGE_ID,
            "images/FlagMeterLevelProgress.png",
        )?,
        load_image(
            resources,
            SEED_PACKET_SILHOUETTE_IMAGE_ID,
            "images/SeedPacketSilhouette.png",
        )?,
        load_image(
            resources,
            SEED_PEASHOOTER_IMAGE_ID,
            "reanim/PeaShooter_Head.png",
        )?,
        load_image(
            resources,
            SEED_SUNFLOWER_IMAGE_ID,
            "reanim/SunFlower_head.png",
        )?,
        load_image(
            resources,
            BOARD_ZOMBIE_BODY_IMAGE_ID,
            "reanim/Zombie_body.png",
        )?,
        load_image(
            resources,
            BOARD_ZOMBIE_CONE_IMAGE_ID,
            "reanim/Zombie_cone1.png",
        )?,
        load_image(
            resources,
            BOARD_ZOMBIE_BUCKET_IMAGE_ID,
            "reanim/Zombie_bucket1.png",
        )?,
        load_image(
            resources,
            BOARD_ZOMBIE_FLAG_POLE_IMAGE_ID,
            "reanim/Zombie_flagpole.png",
        )?,
        load_image(
            resources,
            BOARD_ZOMBIE_FLAG_IMAGE_ID,
            "reanim/Zombie_flag1.png",
        )?,
        load_image(
            resources,
            BOARD_ZOMBIE_FLAG_HAND_IMAGE_ID,
            "reanim/Zombie_flaghand.png",
        )?,
        load_image(
            resources,
            BOARD_ZOMBIE_SCREEN_DOOR_IMAGE_ID,
            "reanim/Zombie_screendoor1.png",
        )?,
        load_image(
            resources,
            BOARD_ZOMBIE_FOOTBALL_HELMET_IMAGE_ID,
            "reanim/Zombie_football_helmet.png",
        )?,
        load_image(
            resources,
            BOARD_ZOMBIE_FOOTBALL_UPPERBODY_IMAGE_ID,
            "reanim/Zombie_football_upperbody.png",
        )?,
        load_image(
            resources,
            BOARD_ZOMBIE_NEWSPAPER_IMAGE_ID,
            "reanim/Zombie_paper_paper1.png",
        )?,
        load_image(
            resources,
            BOARD_ZOMBIE_FOOTBALL_HEAD_IMAGE_ID,
            "reanim/Zombie_football_head.png",
        )?,
        load_image(
            resources,
            BOARD_ZOMBIE_NEWSPAPER_HEAD_IMAGE_ID,
            "reanim/Zombie_paper_head_look.png",
        )?,
        load_image(
            resources,
            BOARD_PROJECTILE_PEA_IMAGE_ID,
            "images/ProjectilePea.png",
        )?,
        load_image(
            resources,
            BOARD_PROJECTILE_SNOW_PEA_IMAGE_ID,
            "images/ProjectileSnowPea.png",
        )?,
        load_image(
            resources,
            BOARD_PROJECTILE_CABBAGE_IMAGE_ID,
            "reanim/Cabbagepult_cabbage.png",
        )?,
        load_image(
            resources,
            BOARD_PROJECTILE_MELON_IMAGE_ID,
            "reanim/Melonpult_melon.png",
        )?,
        load_image(
            resources,
            BOARD_PROJECTILE_WINTER_MELON_IMAGE_ID,
            "reanim/WinterMelon_projectile.png",
        )?,
        load_image(
            resources,
            BOARD_PROJECTILE_KERNEL_IMAGE_ID,
            "reanim/Cornpult_kernal.png",
        )?,
        load_image(
            resources,
            BOARD_PROJECTILE_BUTTER_IMAGE_ID,
            "reanim/Cornpult_butter.png",
        )?,
        load_image(
            resources,
            BOARD_PROJECTILE_SPIKE_IMAGE_ID,
            "images/ProjectileCactus.png",
        )?,
        load_image(
            resources,
            BOARD_PROJECTILE_STAR_IMAGE_ID,
            "images/Projectile_star.png",
        )?,
        load_image(
            resources,
            BOARD_PROJECTILE_FIREBALL_IMAGE_ID,
            "reanim/FirePea.png",
        )?,
        load_image(
            resources,
            BOARD_PROJECTILE_COB_IMAGE_ID,
            "reanim/CobCannon_cob.png",
        )?,
        load_image(
            resources,
            BOARD_PROJECTILE_BASKETBALL_IMAGE_ID,
            "reanim/Zombie_catapult_basketball.png",
        )?,
        load_image(
            resources,
            BOARD_PROJECTILE_PUFF_IMAGE_ID,
            "reanim/puff_1.png",
        )?,
        load_cropped_image(
            resources,
            BOARD_PROJECTILE_SHADOW_DAY_IMAGE_ID,
            "images/pea_shadows.png",
            0,
            0,
            21,
            9,
        )?,
        load_cropped_image(
            resources,
            BOARD_PROJECTILE_SHADOW_NIGHT_IMAGE_ID,
            "images/pea_shadows.png",
            21,
            0,
            21,
            9,
        )?,
        load_image(resources, BOARD_SUN_IMAGE_ID, "reanim/Sun1.png")?,
        load_image(
            resources,
            BOARD_COIN_SILVER_IMAGE_ID,
            "reanim/Coin_silver_dollar.png",
        )?,
        load_image(
            resources,
            BOARD_COIN_GOLD_IMAGE_ID,
            "reanim/Coin_gold_dollar.png",
        )?,
        load_image(resources, BOARD_DIAMOND_IMAGE_ID, "reanim/Diamond.png")?,
        load_image(resources, BOARD_PRESENT_IMAGE_ID, "images/Present.png")?,
        load_image(
            resources,
            BOARD_MONEYBAG_IMAGE_ID,
            "images/moneybag_hi_res.png",
        )?,
        load_image(resources, BOARD_CHOCOLATE_IMAGE_ID, "images/chocolate.png")?,
        load_image(resources, BOARD_VASE_IMAGE_ID, "images/Scary_Pot.png")?,
        load_image(resources, BOARD_NOTE_IMAGE_ID, "images/ZombieNoteSmall.png")?,
        load_cropped_image(
            resources,
            BOARD_SILVER_SUNFLOWER_IMAGE_ID,
            "images/Sunflower_trophy.png",
            0,
            0,
            157,
            269,
        )?,
        load_cropped_image(
            resources,
            BOARD_GOLD_SUNFLOWER_IMAGE_ID,
            "images/Sunflower_trophy.png",
            157,
            0,
            157,
            269,
        )?,
        load_image(resources, BOARD_SNOWPEA_IMAGE_ID, "reanim/SnowPea_head.png")?,
        load_image(
            resources,
            BOARD_PUFFSHROOM_IMAGE_ID,
            "reanim/PuffShroom_head.png",
        )?,
        load_image(
            resources,
            BOARD_FUMESHROOM_IMAGE_ID,
            "reanim/FumeShroom_head.png",
        )?,
        load_image(
            resources,
            BOARD_STARFRUIT_IMAGE_ID,
            "reanim/Starfruit_body.png",
        )?,
        load_image(resources, BOARD_WALLNUT_IMAGE_ID, "reanim/Wallnut_body.png")?,
        load_image(
            resources,
            BOARD_MAGNETSHROOM_IMAGE_ID,
            "reanim/Magnetshroom_head1.png",
        )?,
        load_image(
            resources,
            BOARD_BEGHOULED_TWIST_OVERLAY_IMAGE_ID,
            "images/Beghouled_Twist_overlay.png",
        )?,
        load_image(
            resources,
            BOARD_GRAVE_IMAGE_ID,
            "images/Night_grave_graphic.png",
        )?,
        load_cropped_image(
            resources,
            BOARD_CRATER_IMAGE_ID,
            "images/crater.png",
            0,
            0,
            90,
            61,
        )?,
        load_image(resources, BOARD_BRAIN_IMAGE_ID, "images/brain.png")?,
        load_image(resources, BOARD_VASE_TOP_IMAGE_ID, "reanim/Pot_top.png")?,
        load_image(
            resources,
            BOARD_VASE_BOTTOM_IMAGE_ID,
            "reanim/Pot_bottom.png",
        )?,
        load_image(
            resources,
            SEED_CHOOSER_BUTTON_IMAGE_ID,
            "images/SeedChooser_Button.png",
        )?,
        render_text_image(
            SEED_CHOOSER_TITLE_IMAGE_ID,
            "\u{9009}\u{62e9}\u{4f60}\u{7684}\u{690d}\u{7269}",
            220,
            32,
            18,
        )?,
        load_image(resources, DAY_BACKGROUND_IMAGE_ID, "images/background1.jpg")?,
        load_image(
            resources,
            MODE_SELECT_BACKGROUND_IMAGE_ID,
            "images/Challenge_Background.jpg",
        )?,
        load_image(
            resources,
            MODE_SELECT_WINDOW_IMAGE_ID,
            "images/Challenge_Window.png",
        )?,
        load_image(
            resources,
            MODE_SELECT_BLANK_IMAGE_ID,
            "images/Challenge_Blank.png",
        )?,
        load_image(
            resources,
            NIGHT_BACKGROUND_IMAGE_ID,
            "images/background2.jpg",
        )?,
        load_image(
            resources,
            POOL_BACKGROUND_IMAGE_ID,
            "images/background3.jpg",
        )?,
        load_image(resources, FOG_BACKGROUND_IMAGE_ID, "images/background4.jpg")?,
        load_image(
            resources,
            ROOF_BACKGROUND_IMAGE_ID,
            "images/background5.jpg",
        )?,
        load_image(
            resources,
            BOSS_BACKGROUND_IMAGE_ID,
            "images/background6boss.jpg",
        )?,
        load_image(
            resources,
            CREDITS_BIG_BRAIN_IMAGE_ID,
            "images/Credits_BigBrain.jpg",
        )?,
        load_image(
            resources,
            CREDITS_ZOMBIE_NOTE_IMAGE_ID,
            "images/Credits_ZombieNote.png",
        )?,
        load_image(
            resources,
            CREDITS_PLAY_BUTTON_IMAGE_ID,
            "images/Credits_PlayButton.png",
        )?,
        load_image(
            resources,
            CREDITS_STAGE_IMAGE_ID,
            "reanim/Credits_stage.png",
        )?,
        load_image(
            resources,
            CREDITS_WE_ARE_UNDEAD_IMAGE_ID,
            "reanim/Credits_wearetheundead.jpg",
        )?,
        load_image(resources, CREDITS_MTV_IMAGE_ID, "reanim/Credits_MTV.png")?,
        render_colored_text_image(
            CREDITS_TITLE_TEXT_IMAGE_ID,
            "植物大战僵尸",
            520,
            52,
            28,
            [255, 255, 255],
        )?,
        render_colored_text_image(
            CREDITS_LINE1_TEXT_IMAGE_ID,
            "游戏设计与编程",
            720,
            34,
            20,
            [255, 255, 255],
        )?,
        render_colored_text_image(
            CREDITS_LINE2_TEXT_IMAGE_ID,
            "PopCap Games",
            720,
            34,
            20,
            [255, 255, 255],
        )?,
        render_colored_text_image(
            CREDITS_LINE3_TEXT_IMAGE_ID,
            "音乐、动画与美术",
            720,
            34,
            20,
            [255, 255, 255],
        )?,
        render_colored_text_image(
            CREDITS_LINE4_TEXT_IMAGE_ID,
            "感谢每一位帮助过我们的人",
            720,
            34,
            20,
            [255, 255, 255],
        )?,
        render_colored_text_image(
            CREDITS_LINE5_TEXT_IMAGE_ID,
            "特别感谢：我们的玩家",
            720,
            34,
            20,
            [255, 255, 255],
        )?,
        render_colored_text_image(
            CREDITS_LINE6_TEXT_IMAGE_ID,
            "谢谢游玩",
            720,
            34,
            20,
            [255, 255, 255],
        )?,
        render_colored_text_image(
            CREDITS_REPLAY_TEXT_IMAGE_ID,
            "重播",
            80,
            28,
            16,
            [255, 255, 255],
        )?,
        render_colored_text_image(
            CREDITS_MAIN_MENU_TEXT_IMAGE_ID,
            "返回主菜单",
            180,
            28,
            16,
            [255, 255, 255],
        )?,
        load_image(
            resources,
            GARDEN_BACKGROUND_IMAGE_ID,
            "images/Background_Greenhouse.jpg",
        )?,
        load_image(
            resources,
            GARDEN_MUSHROOM_BACKGROUND_IMAGE_ID,
            "images/Background_MushroomGarden.jpg",
        )?,
        load_image(
            resources,
            GARDEN_WATERING_CAN_IMAGE_ID,
            "images/WateringCan.png",
        )?,
        load_image(
            resources,
            GARDEN_FERTILIZER_IMAGE_ID,
            "images/Fertilizer.png",
        )?,
        load_image(resources, GARDEN_BUG_SPRAY_IMAGE_ID, "images/bug_spray.png")?,
        load_image(
            resources,
            GARDEN_PHONOGRAPH_IMAGE_ID,
            "images/Phonograph.png",
        )?,
        load_image(
            resources,
            GARDEN_NEED_BUBBLE_IMAGE_ID,
            "images/Plantspeechbubble.png",
        )?,
        load_image(resources, GARDEN_WATERDROP_IMAGE_ID, "images/waterdrop.png")?,
        load_cropped_image(
            resources,
            GARDEN_NEED_FERTILIZER_IMAGE_ID,
            "images/zen_need_icons.png",
            0,
            0,
            30,
            30,
        )?,
        load_cropped_image(
            resources,
            GARDEN_NEED_BUG_SPRAY_IMAGE_ID,
            "images/zen_need_icons.png",
            30,
            0,
            30,
            30,
        )?,
        load_cropped_image(
            resources,
            GARDEN_NEED_PHONOGRAPH_IMAGE_ID,
            "images/zen_need_icons.png",
            60,
            0,
            30,
            30,
        )?,
        load_image(
            resources,
            STORE_BACKGROUND_IMAGE_ID,
            "images/Store_Background.jpg",
        )?,
        load_image(resources, STORE_SIGN_IMAGE_ID, "images/Store_Sign.png")?,
        load_image(resources, STORE_CAR_IMAGE_ID, "images/Store_Car.jpg")?,
        load_image(
            resources,
            STORE_PRICE_TAG_IMAGE_ID,
            "images/Store_PriceTag.png",
        )?,
        load_image(
            resources,
            STORE_MAIN_MENU_BUTTON_IMAGE_ID,
            "images/Store_MainMenuButton.png",
        )?,
        load_image(
            resources,
            STORE_PACKET_UPGRADE_IMAGE_ID,
            "images/Store_PacketUpgrade.png",
        )?,
        load_image(resources, STORE_STINKY_IMAGE_ID, "reanim/Stinky_body.png")?,
        render_colored_text_image(
            STORE_ITEM_NAME_BASE_IMAGE_ID,
            "Packet slots",
            130,
            24,
            14,
            [255, 255, 220],
        )?,
        render_colored_text_image(
            STORE_ITEM_NAME_BASE_IMAGE_ID + 1,
            "Fertilizer",
            130,
            24,
            14,
            [255, 255, 220],
        )?,
        render_colored_text_image(
            STORE_ITEM_NAME_BASE_IMAGE_ID + 2,
            "Bug spray",
            130,
            24,
            14,
            [255, 255, 220],
        )?,
        render_colored_text_image(
            STORE_ITEM_NAME_BASE_IMAGE_ID + 3,
            "Phonograph",
            130,
            24,
            14,
            [255, 255, 220],
        )?,
        render_colored_text_image(
            STORE_ITEM_NAME_BASE_IMAGE_ID + 4,
            "Stinky",
            130,
            24,
            14,
            [255, 255, 220],
        )?,
        render_colored_text_image(
            STORE_ITEM_PRICE_BASE_IMAGE_ID,
            "75",
            48,
            22,
            14,
            [32, 24, 12],
        )?,
        render_colored_text_image(
            STORE_ITEM_PRICE_BASE_IMAGE_ID + 1,
            "500",
            48,
            22,
            14,
            [32, 24, 12],
        )?,
        render_colored_text_image(
            STORE_ITEM_PRICE_BASE_IMAGE_ID + 2,
            "2000",
            48,
            22,
            14,
            [32, 24, 12],
        )?,
        render_colored_text_image(
            STORE_ITEM_PRICE_BASE_IMAGE_ID + 3,
            "8000",
            48,
            22,
            14,
            [32, 24, 12],
        )?,
        render_colored_text_image(
            STORE_ITEM_PRICE_BASE_IMAGE_ID + 4,
            "75",
            48,
            22,
            14,
            [32, 24, 12],
        )?,
        render_colored_text_image(
            STORE_ITEM_PRICE_BASE_IMAGE_ID + 5,
            "100",
            48,
            22,
            14,
            [32, 24, 12],
        )?,
        render_colored_text_image(
            STORE_ITEM_PRICE_BASE_IMAGE_ID + 6,
            "1500",
            48,
            22,
            14,
            [32, 24, 12],
        )?,
        render_colored_text_image(
            STORE_ITEM_PRICE_BASE_IMAGE_ID + 7,
            "300",
            48,
            22,
            14,
            [32, 24, 12],
        )?,
        render_colored_text_image(
            STORE_BACK_TEXT_IMAGE_ID,
            "Back",
            100,
            24,
            14,
            [255, 255, 220],
        )?,
        load_image(
            resources,
            PAUSE_DIALOG_TOP_LEFT_IMAGE_ID,
            "images/dialog_topleft.png",
        )?,
        load_image(
            resources,
            PAUSE_DIALOG_TOP_MIDDLE_IMAGE_ID,
            "images/dialog_topmiddle.png",
        )?,
        load_image(
            resources,
            PAUSE_DIALOG_TOP_RIGHT_IMAGE_ID,
            "images/dialog_topright.png",
        )?,
        load_image(
            resources,
            PAUSE_DIALOG_HEADER_IMAGE_ID,
            "images/dialog_header.png",
        )?,
        load_image(
            resources,
            PAUSE_DIALOG_CENTER_LEFT_IMAGE_ID,
            "images/dialog_centerleft.png",
        )?,
        load_image(
            resources,
            PAUSE_DIALOG_CENTER_MIDDLE_IMAGE_ID,
            "images/dialog_centermiddle.png",
        )?,
        load_image(
            resources,
            PAUSE_DIALOG_CENTER_RIGHT_IMAGE_ID,
            "images/dialog_centerright.png",
        )?,
        load_image(
            resources,
            PAUSE_DIALOG_BOTTOM_LEFT_IMAGE_ID,
            "images/dialog_bottomleft.png",
        )?,
        load_image(
            resources,
            PAUSE_DIALOG_BOTTOM_MIDDLE_IMAGE_ID,
            "images/dialog_bottommiddle.png",
        )?,
        load_image(
            resources,
            PAUSE_DIALOG_BOTTOM_RIGHT_IMAGE_ID,
            "images/dialog_bottomright.png",
        )?,
        load_image(
            resources,
            PAUSE_RESUME_BUTTON_IMAGE_ID,
            "images/options_backtogamebutton0.png",
        )?,
        render_colored_text_image(
            PAUSE_HEADER_TEXT_IMAGE_ID,
            "GAME PAUSED",
            250,
            28,
            20,
            [255, 255, 220],
        )?,
        render_colored_text_image(
            PAUSE_BODY_TEXT_IMAGE_ID,
            "Click to resume game",
            240,
            24,
            16,
            [255, 255, 220],
        )?,
        render_colored_text_image(
            PAUSE_RESUME_TEXT_IMAGE_ID,
            "Resume Game",
            200,
            28,
            18,
            [32, 24, 12],
        )?,
        load_image(
            resources,
            OPTIONS_BACKGROUND_IMAGE_ID,
            "images/options_menuback.jpg",
        )?,
        load_image(
            resources,
            OPTIONS_CHECKBOX_OFF_IMAGE_ID,
            "images/options_checkbox0.png",
        )?,
        load_image(
            resources,
            OPTIONS_CHECKBOX_ON_IMAGE_ID,
            "images/options_checkbox1.png",
        )?,
        load_image(
            resources,
            OPTIONS_SLIDER_SLOT_IMAGE_ID,
            "images/options_sliderslot.png",
        )?,
        load_image(
            resources,
            OPTIONS_SLIDER_KNOB_IMAGE_ID,
            "images/options_sliderknob2.png",
        )?,
        render_colored_text_image(
            OPTIONS_MUSIC_LABEL_IMAGE_ID,
            "Music",
            125,
            24,
            16,
            [107, 109, 145],
        )?,
        render_colored_text_image(
            OPTIONS_SFX_LABEL_IMAGE_ID,
            "Sound FX",
            125,
            24,
            16,
            [107, 109, 145],
        )?,
        render_colored_text_image(
            OPTIONS_ACCELERATION_LABEL_IMAGE_ID,
            "3D Acceleration",
            165,
            24,
            16,
            [107, 109, 145],
        )?,
        render_colored_text_image(
            OPTIONS_FULLSCREEN_LABEL_IMAGE_ID,
            "Full Screen",
            125,
            24,
            16,
            [107, 109, 145],
        )?,
        render_colored_text_image(
            OPTIONS_BACK_TEXT_IMAGE_ID,
            "OK",
            220,
            28,
            18,
            [255, 255, 255],
        )?,
        load_image(
            resources,
            HELP_ZOMBIE_NOTE_IMAGE_ID,
            "images/ZombieNote.jpg",
        )?,
        load_image(
            resources,
            HELP_CONTENT_IMAGE_ID,
            "images/ZombieNoteHelp.png",
        )?,
        load_image(
            resources,
            HELP_MENU_BUTTON_IMAGE_ID,
            "images/SeedChooser_Button2.png",
        )?,
        render_colored_text_image(
            HELP_MAIN_MENU_TEXT_IMAGE_ID,
            "Main Menu",
            111,
            26,
            12,
            [42, 42, 90],
        )?,
        load_image(
            resources,
            ALMANAC_INDEX_BACKGROUND_IMAGE_ID,
            "images/Almanac_IndexBack.jpg",
        )?,
        load_image(
            resources,
            ALMANAC_PLANT_BACKGROUND_IMAGE_ID,
            "images/Almanac_PlantBack.jpg",
        )?,
        load_image(
            resources,
            ALMANAC_ZOMBIE_BACKGROUND_IMAGE_ID,
            "images/Almanac_ZombieBack.jpg",
        )?,
        load_image(
            resources,
            ALMANAC_CLOSE_BUTTON_IMAGE_ID,
            "images/Almanac_CloseButton.png",
        )?,
        load_image(
            resources,
            ALMANAC_INDEX_BUTTON_IMAGE_ID,
            "images/Almanac_IndexButton.png",
        )?,
        load_image(
            resources,
            ALMANAC_NAV_BUTTON_IMAGE_ID,
            "images/SeedChooser_Button.png",
        )?,
        load_image(
            resources,
            ALMANAC_PLANT_CARD_IMAGE_ID,
            "images/Almanac_PlantCard.png",
        )?,
        load_image(
            resources,
            ALMANAC_ZOMBIE_CARD_IMAGE_ID,
            "images/Almanac_ZombieCard.png",
        )?,
        load_image(
            resources,
            ALMANAC_ZOMBIE_WINDOW_IMAGE_ID,
            "images/Almanac_ZombieWindow.png",
        )?,
        load_image(
            resources,
            ALMANAC_ZOMBIE_WINDOW2_IMAGE_ID,
            "images/Almanac_ZombieWindow2.png",
        )?,
        load_image(
            resources,
            ALMANAC_ZOMBIE_BLANK_IMAGE_ID,
            "images/Almanac_ZombieBlank.png",
        )?,
        load_image(
            resources,
            ALMANAC_IMITATER_IMAGE_ID,
            "images/Almanac_Imitater.png",
        )?,
        render_colored_text_image(
            ALMANAC_TITLE_TEXT_IMAGE_ID,
            "Suburban Almanac Index",
            400,
            32,
            20,
            [220, 220, 220],
        )?,
        render_colored_text_image(
            ALMANAC_PLANTS_TEXT_IMAGE_ID,
            "View Plants",
            156,
            32,
            18,
            [255, 255, 255],
        )?,
        render_colored_text_image(
            ALMANAC_ZOMBIES_TEXT_IMAGE_ID,
            "View Zombies",
            210,
            32,
            18,
            [255, 255, 255],
        )?,
        render_colored_text_image(
            ALMANAC_CLOSE_TEXT_IMAGE_ID,
            "Close",
            89,
            26,
            12,
            [42, 42, 90],
        )?,
        render_colored_text_image(
            ALMANAC_INDEX_TEXT_IMAGE_ID,
            "Index",
            164,
            26,
            12,
            [42, 42, 90],
        )?,
        load_image(
            resources,
            AWARD_SCREEN_BACKGROUND_IMAGE_ID,
            "images/AwardScreen_Back.jpg",
        )?,
        load_image(resources, AWARD_TROPHY_IMAGE_ID, "images/trophy_hi_res.png")?,
        render_colored_text_image(
            AWARD_TITLE_TEXT_IMAGE_ID,
            "LEVEL COMPLETE",
            400,
            36,
            24,
            [213, 159, 43],
        )?,
        render_colored_text_image(
            AWARD_BODY_TEXT_IMAGE_ID,
            "The lawn is safe!",
            300,
            48,
            18,
            [40, 50, 90],
        )?,
        render_colored_text_image(
            AWARD_CONTINUE_TEXT_IMAGE_ID,
            "Continue",
            156,
            42,
            16,
            [213, 159, 43],
        )?,
        render_colored_text_image(
            AWARD_MAIN_MENU_TEXT_IMAGE_ID,
            "Main Menu",
            111,
            26,
            12,
            [42, 42, 90],
        )?,
        render_colored_text_image(
            GAME_OVER_HEADER_TEXT_IMAGE_ID,
            "GAME OVER",
            250,
            28,
            20,
            [255, 255, 220],
        )?,
        render_colored_text_image(
            GAME_OVER_BODY_TEXT_IMAGE_ID,
            "The zombies ate your brains!",
            280,
            24,
            16,
            [255, 255, 220],
        )?,
        render_colored_text_image(
            GAME_OVER_TRY_AGAIN_TEXT_IMAGE_ID,
            "Try Again",
            200,
            28,
            18,
            [32, 24, 12],
        )?,
        render_colored_text_image(
            GAME_OVER_MAIN_MENU_TEXT_IMAGE_ID,
            "Main Menu",
            164,
            26,
            12,
            [42, 42, 90],
        )?,
        load_masked_image(
            resources,
            ZOMBIES_WON_IMAGE_ID,
            "reanim/ZombiesWon.jpg",
            "reanim/ZombiesWon_.png",
        )?,
    ];
    for index in 0..13 {
        assets.push(load_cropped_image(
            resources,
            SEED_PACKET_PLANT_BASE_IMAGE_ID + index,
            "images/packet_plants.png",
            index * 50,
            0,
            50,
            70,
        )?);
    }
    for index in 0..10 {
        assets.push(load_cropped_image(
            resources,
            SELECTOR_LEVEL_NUMBER_BASE_IMAGE_ID + index,
            "images/SelectorScreen_LevelNumbers.png",
            index * 12,
            0,
            12,
            17,
        )?);
    }
    for (frame, y) in (0..6).zip((0..6).map(|frame| frame * 16)) {
        assets.push(load_cropped_image(
            resources,
            BOARD_CONVEYOR_BELT_BASE_IMAGE_ID + frame,
            "images/ConveyorBelt.png",
            0,
            y,
            502,
            16,
        )?);
    }
    for (index, (source_x, width)) in [(374, 72), (358, 88), (322, 124), (281, 165)]
        .into_iter()
        .enumerate()
    {
        assets.push(load_cropped_image(
            resources,
            BOARD_SEED_BANK_EXTENSION_BASE_IMAGE_ID + u32::try_from(index).unwrap(),
            "images/SeedBank.png",
            source_x,
            0,
            width,
            87,
        )?);
    }
    for slot in 0..ALMANAC_PLANT_COUNT {
        let slot_id = u32::from(slot);
        assets.push(render_colored_text_image(
            ALMANAC_PLANT_NAME_BASE_IMAGE_ID + slot_id,
            almanac_plant_name(slot),
            258,
            32,
            18,
            [255, 255, 255],
        )?);
        if let Some(plant_type) = almanac_plant_type(slot)
            && plant_type != PlantType::Other(ALMANAC_IMITATER_SLOT)
        {
            assets.push(render_colored_text_image(
                ALMANAC_PLANT_COST_BASE_IMAGE_ID + slot_id,
                &format!("Cost: {}", plant_type.cost()),
                134,
                50,
                12,
                [255, 255, 255],
            )?);
            let recharge = match plant_type.refresh_time() {
                750 => "Short",
                3_000 => "Long",
                _ => "Very Long",
            };
            assets.push(render_colored_text_image(
                ALMANAC_PLANT_RECHARGE_BASE_IMAGE_ID + slot_id,
                &format!("Recharge: {recharge}"),
                139,
                50,
                12,
                [40, 50, 90],
            )?);
        }
    }
    for index in 0..ALMANAC_ZOMBIE_COUNT {
        let index_id = u32::from(index);
        assets.extend([
            render_colored_text_image(
                ALMANAC_ZOMBIE_NAME_BASE_IMAGE_ID + index_id,
                almanac_zombie_name(index),
                258,
                32,
                18,
                [190, 255, 235],
            )?,
            render_colored_text_image(
                ALMANAC_ZOMBIE_SILHOUETTE_NAME_BASE_IMAGE_ID + index_id,
                "???",
                258,
                32,
                18,
                [190, 255, 235],
            )?,
        ]);
    }
    assets.extend([
        load_image(resources, AWARD_SHOVEL_IMAGE_ID, "images/Shovel_hi_res.png")?,
        load_image(resources, AWARD_ALMANAC_IMAGE_ID, "images/Almanac.png")?,
        load_image(resources, AWARD_CAR_KEYS_IMAGE_ID, "images/CarKeys.png")?,
        load_image(resources, AWARD_TACO_IMAGE_ID, "images/Taco.png")?,
        load_image(
            resources,
            AWARD_WATERING_CAN_IMAGE_ID,
            "images/WateringCan.png",
        )?,
        load_image(resources, AWARD_NOTE1_IMAGE_ID, "images/ZombieNote1.png")?,
        load_image(resources, AWARD_NOTE2_IMAGE_ID, "images/ZombieNote2.png")?,
        load_image(resources, AWARD_NOTE3_IMAGE_ID, "images/ZombieNote3.png")?,
        load_image(resources, AWARD_NOTE4_IMAGE_ID, "images/ZombieNote4.png")?,
    ]);
    for (id, text, color) in [
        (0, "NEW PLANT", [213, 159, 43]),
        (1, "GOT SHOVEL", [213, 159, 43]),
        (2, "FOUND ALMANAC", [213, 159, 43]),
        (3, "FOUND KEYS", [213, 159, 43]),
        (4, "FOUND TACO", [213, 159, 43]),
        (5, "FOUND WATERING CAN", [213, 159, 43]),
        (6, "FOUND NOTE", [255, 200, 0]),
        (7, "YOU WIN", [213, 159, 43]),
        (8, "LEVEL COMPLETE", [213, 159, 43]),
    ] {
        assets.push(render_colored_text_image(
            AWARD_TITLE_TEXT_BASE_IMAGE_ID + id,
            text,
            400,
            36,
            24,
            color,
        )?);
    }
    for (id, text) in [
        (0, "A new seed packet is ready."),
        (1, "The shovel is ready for your lawn."),
        (2, "The Suburban Almanac is now available."),
        (3, "The garage and new modes are unlocked."),
        (4, "Crazy Dave's taco is safe."),
        (5, "Zen Garden tools are now available."),
        (6, "A new note from the zombies awaits."),
        (7, "Your garden is safe!"),
        (8, "Reward collected."),
    ] {
        assets.push(render_colored_text_image(
            AWARD_BODY_TEXT_BASE_IMAGE_ID + id,
            text,
            300,
            48,
            18,
            [40, 50, 90],
        )?);
    }
    for index in 0..22 {
        assets.push(load_cropped_image(
            resources,
            CHALLENGE_THUMBNAIL_BASE_IMAGE_ID + index,
            "images/Challenge_Thumbnails.jpg",
            index * 80,
            0,
            80,
            65,
        )?);
    }
    for index in 0..11 {
        assets.push(load_cropped_image(
            resources,
            SURVIVAL_THUMBNAIL_BASE_IMAGE_ID + index,
            "images/Survival_Thumbnails.jpg",
            index * 80,
            0,
            80,
            65,
        )?);
    }
    assets.push(
        ImageAsset::new(UI_PIXEL_IMAGE_ID, 1, 1, vec![70, 180, 80, 255])
            .map_err(|error| error.to_string())?,
    );
    assets.push(
        ImageAsset::new(SCREEN_PIXEL_IMAGE_ID, 1, 1, vec![16, 24, 32, 255])
            .map_err(|error| error.to_string())?,
    );
    let reanim_catalog = load_reanim_catalog(resources, &mut assets)?;
    let (particle_catalog, particle_images) = load_particle_catalog(resources, &mut assets)?;
    let mut image_ids = std::collections::HashSet::new();
    for asset in &assets {
        if !image_ids.insert(asset.resource_id) {
            return Err(format!(
                "duplicate image resource ID: {}",
                asset.resource_id
            ));
        }
    }
    Ok((assets, reanim_catalog, particle_catalog, particle_images))
}

fn load_particle_catalog(
    resources: &ResourceProvider,
    assets: &mut Vec<ImageAsset>,
) -> Result<(ParticleCatalog, HashMap<String, ParticleImage>), String> {
    let definition_paths = resources
        .compiled_animation_paths()
        .map_err(|error| format!("compiled particle inventory: {error}"))?;
    let mut catalog = ParticleCatalog::default();
    let mut symbols = HashMap::<String, (u32, u32)>::new();
    for path in definition_paths {
        let normalized = path.replace('\\', "/");
        if !normalized
            .to_ascii_uppercase()
            .starts_with("COMPILED/PARTICLES/")
        {
            continue;
        }
        let file = normalized.rsplit('/').next().unwrap_or(&normalized);
        let suffix = ".xml.compiled";
        let Some(name) = file.get(..file.len().saturating_sub(suffix.len())) else {
            continue;
        };
        if !file[name.len()..].eq_ignore_ascii_case(suffix) {
            continue;
        }
        let definition = resources
            .read_compiled(&path)
            .map_err(|error| format!("{path}: {error}"))?
            .particles()
            .map_err(|error| format!("{path}: {error}"))?;
        for emitter in &definition.emitters {
            if let Some(image) = &emitter.image {
                let columns = u32::try_from(emitter.image_col.max(0) + emitter.image_frames.max(1))
                    .unwrap_or(1);
                let rows = u32::try_from(emitter.image_row.max(0) + 1).unwrap_or(1);
                symbols
                    .entry(image.to_ascii_uppercase())
                    .and_modify(|grid| {
                        grid.0 = grid.0.max(columns);
                        grid.1 = grid.1.max(rows);
                    })
                    .or_insert((columns, rows));
            }
        }
        catalog.insert(name, definition);
    }
    for symbol in [
        "IMAGE_REANIM_ZOMBIE_MUSTACHE1",
        "IMAGE_REANIM_ZOMBIE_MUSTACHE2",
        "IMAGE_REANIM_ZOMBIE_MUSTACHE3",
    ] {
        symbols.entry(symbol.to_owned()).or_insert((1, 1));
    }

    let manifest = resources
        .manifest()
        .map_err(|error| format!("particle image manifest: {error}"))?;
    let paths = resources
        .paths()
        .map_err(|error| format!("particle image inventory: {error}"))?;
    let mut symbols: Vec<_> = symbols.into_iter().collect();
    symbols.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    let mut images = HashMap::new();
    for (symbol, inferred_grid) in symbols {
        let entry = manifest
            .groups
            .iter()
            .flat_map(|group| &group.entries)
            .find(|entry| {
                entry.kind == ResourceKind::Image && entry.id.eq_ignore_ascii_case(&symbol)
            });
        let path = entry
            .and_then(|entry| {
                let wanted = resource_stem_key(&entry.path);
                paths
                    .iter()
                    .find(|path| resource_stem_key(path) == wanted)
                    .cloned()
            })
            .or_else(|| particle_image_path(&symbol, &paths));
        let Some(path) = path else {
            tracing::warn!(symbol, "particle image resource is unavailable");
            continue;
        };
        let resource_id = PARTICLE_IMAGE_ID_BASE
            .checked_add(u32::try_from(images.len()).map_err(|_| "too many particle images")?)
            .ok_or("particle image ID overflow")?;
        assets.push(load_image(resources, resource_id, &path)?);
        images.insert(
            symbol,
            ParticleImage {
                resource_id,
                columns: entry.map_or(inferred_grid.0, |entry| entry.cols),
                rows: entry.map_or(inferred_grid.1, |entry| entry.rows),
            },
        );
    }
    tracing::info!(
        definitions = catalog.len(),
        images = images.len(),
        "particle definitions loaded"
    );
    Ok((catalog, images))
}

fn load_reanim_catalog(
    resources: &ResourceProvider,
    assets: &mut Vec<ImageAsset>,
) -> Result<ReanimCatalog, String> {
    let definition_paths = resources
        .compiled_animation_paths()
        .map_err(|error| format!("compiled animation inventory: {error}"))?;
    let zombie = load_reanim_definition(resources, &definition_paths, "Zombie.reanim.compiled")?;
    let football = load_reanim_definition(
        resources,
        &definition_paths,
        "Zombie_football.reanim.compiled",
    )?;
    let newspaper =
        load_reanim_definition(resources, &definition_paths, "Zombie_paper.reanim.compiled")?;
    let boss = load_reanim_definition(resources, &definition_paths, "Zombie_boss.reanim.compiled")?;
    let boss_driver = load_reanim_definition(
        resources,
        &definition_paths,
        "Zombie_Boss_driver.reanim.compiled",
    )?;
    let boss_fireball = load_reanim_definition(
        resources,
        &definition_paths,
        "Zombie_boss_fireball.reanim.compiled",
    )?;
    let boss_iceball = load_reanim_definition(
        resources,
        &definition_paths,
        "Zombie_boss_iceball.reanim.compiled",
    )?;
    let fire = load_reanim_definition(resources, &definition_paths, "fire.reanim.compiled")?;
    let sun = load_reanim_definition(resources, &definition_paths, "Sun.reanim.compiled")?;
    let coin_silver =
        load_reanim_definition(resources, &definition_paths, "Coin_silver.reanim.compiled")?;
    let coin_gold =
        load_reanim_definition(resources, &definition_paths, "Coin_gold.reanim.compiled")?;
    let diamond = load_reanim_definition(resources, &definition_paths, "Diamond.reanim.compiled")?;
    let credits_main2 = load_reanim_definition(
        resources,
        &definition_paths,
        "Credits_Main2.reanim.compiled",
    )?;
    let loadbar_sprout = load_reanim_definition(
        resources,
        &definition_paths,
        "LoadBar_sprout.reanim.compiled",
    )?;
    let loadbar_zombiehead = load_reanim_definition(
        resources,
        &definition_paths,
        "LoadBar_Zombiehead.reanim.compiled",
    )?;
    let mut specialized = Vec::new();
    for &(zombie_type, file_name) in SPECIALIZED_REANIM_FILES {
        if let Some(definition) = load_reanim_definition(resources, &definition_paths, file_name)? {
            specialized.push((zombie_type, definition));
        }
    }
    let mut plants = Vec::new();
    for &(plant_type, file_name) in PLANT_REANIM_FILES {
        if let Some(definition) = load_reanim_definition(resources, &definition_paths, file_name)? {
            plants.push((plant_type, definition));
        }
    }
    if zombie.is_none()
        && football.is_none()
        && newspaper.is_none()
        && boss.is_none()
        && boss_driver.is_none()
        && boss_fireball.is_none()
        && boss_iceball.is_none()
        && fire.is_none()
        && sun.is_none()
        && coin_silver.is_none()
        && coin_gold.is_none()
        && diamond.is_none()
        && loadbar_sprout.is_none()
        && loadbar_zombiehead.is_none()
        && specialized.is_empty()
        && plants.is_empty()
    {
        tracing::warn!("reanimation definitions are unavailable; using static layers");
        return Ok(ReanimCatalog::default());
    }
    let paths = resources
        .paths()
        .map_err(|error| format!("reanimation image inventory: {error}"))?;
    let mut symbols = HashSet::new();
    for definition in [
        &zombie,
        &football,
        &newspaper,
        &boss,
        &boss_driver,
        &boss_fireball,
        &boss_iceball,
        &fire,
        &sun,
        &coin_silver,
        &coin_gold,
        &diamond,
    ]
    .into_iter()
    .flatten()
    {
        for track in &definition.tracks {
            for transform in &track.transforms {
                if let Some(image) = &transform.image {
                    symbols.insert(image.to_ascii_uppercase());
                }
            }
        }
    }
    for (_, definition) in &specialized {
        for track in &definition.tracks {
            for transform in &track.transforms {
                if let Some(image) = &transform.image {
                    symbols.insert(image.to_ascii_uppercase());
                }
            }
        }
    }
    for (_, definition) in &plants {
        for track in &definition.tracks {
            for transform in &track.transforms {
                if let Some(image) = &transform.image {
                    symbols.insert(image.to_ascii_uppercase());
                }
            }
        }
    }
    symbols.extend(
        [
            "IMAGE_REANIM_ZOMBIE_MUSTACHE1",
            "IMAGE_REANIM_ZOMBIE_MUSTACHE2",
            "IMAGE_REANIM_ZOMBIE_MUSTACHE3",
            "IMAGE_REANIM_ZOMBIE_HEAD_SUNGLASSES1",
            "IMAGE_REANIM_ZOMBIE_HEAD_SUNGLASSES2",
            "IMAGE_REANIM_ZOMBIE_HEAD_SUNGLASSES3",
            "IMAGE_REANIM_ZOMBIE_HEAD_SUNGLASSES4",
        ]
        .into_iter()
        .map(str::to_owned),
    );
    let mut symbols: Vec<_> = symbols.into_iter().collect();
    symbols.sort_unstable();

    let mut image_ids = HashMap::new();
    for symbol in symbols {
        let Some(path) = reanim_image_path(&symbol, &paths) else {
            tracing::warn!(symbol, "reanimation image symbol is unavailable");
            continue;
        };
        let id = REANIM_IMAGE_ID_BASE
            .checked_add(u32::try_from(image_ids.len()).map_err(|_| "too many reanimation images")?)
            .ok_or("reanimation image ID overflow")?;
        assets.push(load_image(resources, id, &path)?);
        image_ids.insert(symbol, id);
    }
    // The load-bar reanim tracks reuse the title images already loaded as
    // fixed IDs; map their symbols without loading duplicate textures.
    for (symbol, id) in [
        (
            "IMAGE_REANIM_POTATOMINE_ROCK1",
            TITLE_LOAD_BAR_ROCK1_IMAGE_ID,
        ),
        (
            "IMAGE_REANIM_POTATOMINE_ROCK3",
            TITLE_LOAD_BAR_ROCK3_IMAGE_ID,
        ),
        (
            "IMAGE_REANIM_SPROUT_BODY",
            TITLE_LOAD_BAR_SPROUT_BODY_IMAGE_ID,
        ),
        (
            "IMAGE_REANIM_SPROUT_PETAL",
            TITLE_LOAD_BAR_SPROUT_PETAL_IMAGE_ID,
        ),
        (
            "IMAGE_REANIM_ZOMBIE_HEAD",
            TITLE_LOAD_BAR_ZOMBIE_HEAD_IMAGE_ID,
        ),
        (
            "IMAGE_REANIM_ZOMBIE_HAIR",
            TITLE_LOAD_BAR_ZOMBIE_HAIR_IMAGE_ID,
        ),
        (
            "IMAGE_REANIM_ZOMBIE_JAW",
            TITLE_LOAD_BAR_ZOMBIE_JAW_IMAGE_ID,
        ),
    ] {
        image_ids.entry(symbol.to_owned()).or_insert(id);
    }
    tracing::info!(
        zombie_tracks = zombie
            .as_ref()
            .map_or(0, |definition| definition.tracks.len()),
        football_tracks = football
            .as_ref()
            .map_or(0, |definition| definition.tracks.len()),
        newspaper_tracks = newspaper
            .as_ref()
            .map_or(0, |definition| definition.tracks.len()),
        boss_tracks = boss
            .as_ref()
            .map_or(0, |definition| definition.tracks.len()),
        boss_driver_tracks = boss_driver
            .as_ref()
            .map_or(0, |definition| definition.tracks.len()),
        boss_fireball_tracks = boss_fireball
            .as_ref()
            .map_or(0, |definition| definition.tracks.len()),
        boss_iceball_tracks = boss_iceball
            .as_ref()
            .map_or(0, |definition| definition.tracks.len()),
        fire_tracks = fire
            .as_ref()
            .map_or(0, |definition| definition.tracks.len()),
        pickup_definitions = [&sun, &coin_silver, &coin_gold, &diamond,]
            .into_iter()
            .flatten()
            .count(),
        pickup_tracks = [&sun, &coin_silver, &coin_gold, &diamond,]
            .into_iter()
            .flatten()
            .map(|definition| definition.tracks.len())
            .sum::<usize>(),
        specialized_definitions = specialized.len(),
        specialized_tracks = specialized
            .iter()
            .map(|(_, definition)| definition.tracks.len())
            .sum::<usize>(),
        plant_definitions = plants.len(),
        plant_tracks = plants
            .iter()
            .map(|(_, definition)| definition.tracks.len())
            .sum::<usize>(),
        images = image_ids.len(),
        "reanimation definitions loaded"
    );
    Ok(ReanimCatalog {
        zombie,
        football,
        newspaper,
        boss,
        boss_driver,
        boss_fireball,
        boss_iceball,
        fire,
        sun,
        coin_silver,
        coin_gold,
        diamond,
        credits_main2,
        loadbar_sprout,
        loadbar_zombiehead,
        specialized,
        plants,
        image_ids,
    })
}

fn load_reanim_definition(
    resources: &ResourceProvider,
    paths: &[String],
    file_name: &str,
) -> Result<Option<ReanimatorDefinition>, String> {
    let wanted = format!("compiled/reanim/{file_name}");
    let Some(path) = paths.iter().find(|path| path.eq_ignore_ascii_case(&wanted)) else {
        tracing::warn!(file_name, "compiled zombie reanimation is unavailable");
        return Ok(None);
    };
    let compiled = resources
        .read_compiled(path)
        .map_err(|error| format!("{path}: {error}"))?;
    compiled
        .reanimation()
        .map(Some)
        .map_err(|error| format!("{path}: {error}"))
}

fn reanim_image_path(symbol: &str, paths: &[String]) -> Option<String> {
    let suffix = symbol.strip_prefix("IMAGE_REANIM_")?;
    for directory in ["REANIM", "IMAGES"] {
        let wanted = format!("{directory}/{suffix}");
        if let Some(path) = paths
            .iter()
            .find(|path| resource_stem_key(path).is_some_and(|key| key == wanted))
        {
            return Some(path.clone());
        }
    }
    None
}

fn particle_image_path(symbol: &str, paths: &[String]) -> Option<String> {
    if let Some(path) = reanim_image_path(symbol, paths) {
        return Some(path);
    }
    let suffix = symbol.strip_prefix("IMAGE_")?;
    for directory in ["PARTICLES", "IMAGES", "REANIM"] {
        let wanted = format!("{directory}/{suffix}");
        if let Some(path) = paths
            .iter()
            .find(|path| resource_stem_key(path).is_some_and(|key| key == wanted))
        {
            return Some(path.clone());
        }
    }
    None
}

fn resource_stem_key(path: &str) -> Option<String> {
    let normalized = path.replace('\\', "/");
    let (directory, file) = normalized.rsplit_once('/')?;
    let stem = file.rsplit_once('.').map_or(file, |(stem, _)| stem);
    Some(format!("{directory}/{stem}").to_ascii_uppercase())
}

fn load_title_logo(resources: &ResourceProvider) -> Result<ImageAsset, String> {
    load_masked_image(
        resources,
        TITLE_LOGO_IMAGE_ID,
        "images/PvZ_Logo.jpg",
        "images/PvZ_Logo_.png",
    )
}

fn load_masked_image(
    resources: &ResourceProvider,
    resource_id: u32,
    color_path: &str,
    mask_path: &str,
) -> Result<ImageAsset, String> {
    let color = load_image(resources, resource_id, color_path)?;
    let mask = load_image(resources, resource_id, mask_path)?;
    if color.width != mask.width || color.height != mask.height {
        return Err(format!("{color_path} and {mask_path} dimensions differ"));
    }
    let mut rgba8 = color.rgba8;
    for (color_pixel, mask_pixel) in rgba8
        .as_chunks_mut::<4>()
        .0
        .iter_mut()
        .zip(mask.rgba8.as_chunks::<4>().0)
    {
        color_pixel[3] = mask_pixel[0];
    }
    ImageAsset::new(resource_id, color.width, color.height, rgba8)
        .map_err(|error| format!("masked image {color_path}: {error}"))
}

fn load_cropped_image(
    resources: &ResourceProvider,
    resource_id: u32,
    path: &str,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<ImageAsset, String> {
    let image = load_image(resources, resource_id, path)?;
    let Some(x_end) = x.checked_add(width) else {
        return Err(format!("{path}: crop exceeds image dimensions"));
    };
    let Some(y_end) = y.checked_add(height) else {
        return Err(format!("{path}: crop exceeds image dimensions"));
    };
    if x_end > image.width || y_end > image.height {
        return Err(format!("{path}: crop exceeds image dimensions"));
    }
    let rgba = image::RgbaImage::from_raw(image.width, image.height, image.rgba8)
        .ok_or_else(|| format!("{path}: invalid decoded image data"))?;
    let cropped = image::imageops::crop_imm(&rgba, x, y, width, height).to_image();
    ImageAsset::new(
        resource_id,
        cropped.width(),
        cropped.height(),
        cropped.into_raw(),
    )
    .map_err(|error| format!("{path}: {error}"))
}

fn load_image(
    resources: &ResourceProvider,
    resource_id: u32,
    path: &str,
) -> Result<ImageAsset, String> {
    let bytes = resources
        .read(path)
        .map_err(|error| format!("{path}: {error}"))?;
    let image = image::load_from_memory(&bytes)
        .map_err(|error| format!("{path}: image decode failed: {error}"))?
        .to_rgba8();
    ImageAsset::new(resource_id, image.width(), image.height(), image.into_raw())
        .map_err(|error| format!("{path}: {error}"))
}

fn load_dialogue_text(resource_id: u32, text: &str) -> Result<ImageAsset, String> {
    render_text_image(resource_id, text, 233, 144, 16)
}

fn load_continue_text(resource_id: u32) -> Result<ImageAsset, String> {
    render_text_image(
        resource_id,
        "\u{70b9}\u{51fb}\u{4ee5}\u{7ee7}\u{7eed}",
        120,
        24,
        14,
    )
}

#[cfg(windows)]
fn render_text_image(
    resource_id: u32,
    text: &str,
    width: u32,
    height: u32,
    font_size: i32,
) -> Result<ImageAsset, String> {
    windows_text::render(resource_id, text, width, height, font_size)
}

#[cfg(not(windows))]
fn render_text_image(
    resource_id: u32,
    _text: &str,
    width: u32,
    height: u32,
    _font_size: i32,
) -> Result<ImageAsset, String> {
    ImageAsset::new(
        resource_id,
        width,
        height,
        vec![0; usize::try_from(width).unwrap() * usize::try_from(height).unwrap() * 4],
    )
    .map_err(|error| error.to_string())
}

fn render_colored_text_image(
    resource_id: u32,
    text: &str,
    width: u32,
    height: u32,
    font_size: i32,
    color: [u8; 3],
) -> Result<ImageAsset, String> {
    let mut asset = render_text_image(resource_id, text, width, height, font_size)?;
    for pixel in asset.rgba8.as_chunks_mut::<4>().0 {
        if pixel[3] != 0 {
            pixel[..3].copy_from_slice(&color);
        }
    }
    Ok(asset)
}

fn load_lawn_strings(resources: &ResourceProvider) -> HashMap<String, String> {
    #[cfg(windows)]
    {
        let bytes = match resources.read("properties/LawnStrings.txt") {
            Ok(bytes) => bytes,
            Err(error) => {
                tracing::warn!(%error, "almanac strings unavailable");
                return HashMap::new();
            }
        };
        let text = match windows_text::decode_gbk(&bytes) {
            Ok(text) => text,
            Err(error) => {
                tracing::warn!(%error, "almanac strings decode failed");
                return HashMap::new();
            }
        };
        let strings = parse_lawn_strings(&text);
        tracing::info!(entries = strings.len(), "almanac strings loaded");
        strings
    }
    #[cfg(not(windows))]
    {
        let _ = resources;
        HashMap::new()
    }
}

fn parse_lawn_strings(text: &str) -> HashMap<String, String> {
    let mut entries = HashMap::new();
    let mut key = None;
    let mut value = String::new();
    for line in text.lines() {
        if let Some(name) = line
            .strip_prefix('[')
            .and_then(|line| line.strip_suffix(']'))
        {
            if let Some(previous) = key.replace(name.to_owned()) {
                entries.insert(previous, value.trim().to_owned());
                value.clear();
            }
        } else if !line.is_empty() && key.is_some() {
            if !value.is_empty() {
                value.push('\n');
            }
            value.push_str(line);
        }
    }
    if let Some(name) = key {
        entries.insert(name, value.trim().to_owned());
    }
    entries
}

fn almanac_lawn_text(strings: &HashMap<String, String>, key: &str, fallback: &str) -> String {
    let text = strings.get(key).map(String::as_str).unwrap_or(fallback);
    text.replace("{SHORTLINE}\n", "")
        .replace("\n{SHORTLINE}", "")
        .replace("{KEYWORD}", "")
        .replace("{STAT}", " ")
        .replace("{FLAVOR}", "\n")
}

fn almanac_lawn_description(
    strings: &HashMap<String, String>,
    key: &str,
    fallback: &str,
) -> String {
    almanac_lawn_text(strings, &format!("{key}_DESCRIPTION"), fallback)
}

#[cfg(windows)]
mod windows_text {
    use std::{ffi::c_void, ptr, slice};

    use neopvz_render::ImageAsset;

    type Handle = *mut c_void;

    #[repr(C)]
    struct BitmapInfoHeader {
        size: u32,
        width: i32,
        height: i32,
        planes: u16,
        bit_count: u16,
        compression: u32,
        size_image: u32,
        x_pixels_per_meter: i32,
        y_pixels_per_meter: i32,
        colors_used: u32,
        important_colors: u32,
    }

    #[repr(C)]
    struct RgbQuad {
        blue: u8,
        green: u8,
        red: u8,
        reserved: u8,
    }

    #[repr(C)]
    struct BitmapInfo {
        header: BitmapInfoHeader,
        colors: [RgbQuad; 1],
    }

    #[repr(C)]
    struct Rect {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    }

    #[link(name = "gdi32")]
    unsafe extern "system" {
        fn CreateCompatibleDC(hdc: Handle) -> Handle;
        fn CreateDIBSection(
            hdc: Handle,
            bitmap_info: *const BitmapInfo,
            usage: u32,
            bits: *mut *mut c_void,
            section: Handle,
            offset: u32,
        ) -> Handle;
        fn CreateFontW(
            height: i32,
            width: i32,
            escapement: i32,
            orientation: i32,
            weight: i32,
            italic: u32,
            underline: u32,
            strike_out: u32,
            charset: u32,
            output_precision: u32,
            clip_precision: u32,
            quality: u32,
            pitch_and_family: u32,
            face: *const u16,
        ) -> Handle;
        fn SelectObject(device_context: Handle, object: Handle) -> Handle;
        fn SetBkMode(device_context: Handle, mode: i32) -> i32;
        fn SetTextColor(device_context: Handle, color: u32) -> u32;
        fn DrawTextW(
            device_context: Handle,
            text: *const u16,
            length: i32,
            rect: *mut Rect,
            format: u32,
        ) -> i32;
        fn GdiFlush() -> i32;
        fn DeleteObject(object: Handle) -> i32;
        fn DeleteDC(device_context: Handle) -> i32;
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn MultiByteToWideChar(
            code_page: u32,
            flags: u32,
            multi_byte_str: *const i8,
            multi_byte_len: i32,
            wide_str: *mut u16,
            wide_len: i32,
        ) -> i32;
    }

    const BI_RGB: u32 = 0;
    const DIB_RGB_COLORS: u32 = 0;
    const TRANSPARENT: i32 = 1;
    const FW_NORMAL: i32 = 400;
    const DEFAULT_CHARSET: u32 = 1;
    const OUT_DEFAULT_PRECIS: u32 = 0;
    const CLIP_DEFAULT_PRECIS: u32 = 0;
    const DEFAULT_QUALITY: u32 = 0;
    const DEFAULT_PITCH: u32 = 0;
    const DT_CENTER: u32 = 0x0001;
    const DT_VCENTER: u32 = 0x0004;
    const DT_WORDBREAK: u32 = 0x0010;
    const DT_NOPREFIX: u32 = 0x0800;

    pub(super) fn decode_gbk(bytes: &[u8]) -> Result<String, String> {
        let byte_len =
            i32::try_from(bytes.len()).map_err(|_| "LawnStrings is too large".to_owned())?;
        if byte_len == 0 {
            return Ok(String::new());
        }
        let char_len = unsafe {
            MultiByteToWideChar(936, 0, bytes.as_ptr().cast(), byte_len, ptr::null_mut(), 0)
        };
        if char_len <= 0 {
            return Err("MultiByteToWideChar failed".to_owned());
        }
        let mut wide = vec![0_u16; usize::try_from(char_len).unwrap()];
        let written = unsafe {
            MultiByteToWideChar(
                936,
                0,
                bytes.as_ptr().cast(),
                byte_len,
                wide.as_mut_ptr(),
                char_len,
            )
        };
        if written != char_len {
            return Err("MultiByteToWideChar returned a short string".to_owned());
        }
        String::from_utf16(&wide).map_err(|_| "LawnStrings contains invalid UTF-16".to_owned())
    }

    pub(super) fn render(
        resource_id: u32,
        text: &str,
        width: u32,
        height: u32,
        font_size: i32,
    ) -> Result<ImageAsset, String> {
        let width_i32 = i32::try_from(width).map_err(|_| "text image is too wide".to_owned())?;
        let height_i32 = i32::try_from(height).map_err(|_| "text image is too high".to_owned())?;
        let pixel_count = usize::try_from(width)
            .ok()
            .and_then(|width| {
                usize::try_from(height)
                    .ok()
                    .and_then(|height| width.checked_mul(height))
            })
            .ok_or_else(|| "text image dimensions overflow".to_owned())?;
        let mut bits = ptr::null_mut();
        let bitmap_info = BitmapInfo {
            header: BitmapInfoHeader {
                size: std::mem::size_of::<BitmapInfoHeader>() as u32,
                width: width_i32,
                height: -height_i32,
                planes: 1,
                bit_count: 32,
                compression: BI_RGB,
                size_image: 0,
                x_pixels_per_meter: 0,
                y_pixels_per_meter: 0,
                colors_used: 0,
                important_colors: 0,
            },
            colors: [RgbQuad {
                blue: 0,
                green: 0,
                red: 0,
                reserved: 0,
            }],
        };
        let mut face: Vec<u16> = "Microsoft YaHei".encode_utf16().collect();
        face.push(0);
        let mut wide_text: Vec<u16> = text.encode_utf16().collect();
        wide_text.push(0);

        // GDI gives us a system-font rasterization while keeping text out of the
        // renderer API; the returned image remains an ordinary sprite.
        let (device_context, bitmap, font) = unsafe {
            let device_context = CreateCompatibleDC(ptr::null_mut());
            if device_context.is_null() {
                return Err("CreateCompatibleDC failed".to_owned());
            }
            let bitmap = CreateDIBSection(
                device_context,
                &bitmap_info,
                DIB_RGB_COLORS,
                &mut bits,
                ptr::null_mut(),
                0,
            );
            if bitmap.is_null() {
                DeleteDC(device_context);
                return Err("CreateDIBSection failed".to_owned());
            }
            let font = CreateFontW(
                -font_size,
                0,
                0,
                0,
                FW_NORMAL,
                0,
                0,
                0,
                DEFAULT_CHARSET,
                OUT_DEFAULT_PRECIS,
                CLIP_DEFAULT_PRECIS,
                DEFAULT_QUALITY,
                DEFAULT_PITCH,
                face.as_ptr(),
            );
            if font.is_null() {
                DeleteObject(bitmap);
                DeleteDC(device_context);
                return Err("CreateFontW failed".to_owned());
            }
            (device_context, bitmap, font)
        };

        unsafe {
            SelectObject(device_context, bitmap);
            SelectObject(device_context, font);
            let buffer = slice::from_raw_parts_mut(bits.cast::<u8>(), pixel_count * 4);
            buffer.fill(255);
            SetBkMode(device_context, TRANSPARENT);
            SetTextColor(device_context, 0);
            let mut rect = Rect {
                left: 0,
                top: 0,
                right: width_i32,
                bottom: height_i32,
            };
            if DrawTextW(
                device_context,
                wide_text.as_ptr(),
                -1,
                &mut rect,
                DT_CENTER | DT_VCENTER | DT_WORDBREAK | DT_NOPREFIX,
            ) == 0
                || GdiFlush() == 0
            {
                DeleteDC(device_context);
                DeleteObject(font);
                DeleteObject(bitmap);
                return Err("GDI text drawing failed".to_owned());
            }

            // GDI batches drawing calls; flush before directly reading the DIB.
            let mut rgba = Vec::with_capacity(pixel_count * 4);
            for pixel in buffer.as_chunks::<4>().0 {
                let luminance = (u16::from(pixel[0]) * 29
                    + u16::from(pixel[1]) * 150
                    + u16::from(pixel[2]) * 77)
                    / 256;
                rgba.extend([0, 0, 0, 255_u16.saturating_sub(luminance) as u8]);
            }
            DeleteDC(device_context);
            DeleteObject(font);
            DeleteObject(bitmap);
            ImageAsset::new(resource_id, width, height, rgba).map_err(|error| error.to_string())
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct BoardVisualAnchor {
    x: f32,
    y: f32,
    previous_x: f32,
    previous_y: f32,
    row: u8,
    zombie_type: Option<ZombieType>,
}

#[derive(Clone, Debug, Default)]
struct BoardVisualAnchors {
    plants: HashMap<EntityId, BoardVisualAnchor>,
    plant_types: HashMap<EntityId, PlantType>,
    plant_health: HashMap<EntityId, (i32, i32)>,
    projectiles: HashMap<EntityId, BoardVisualAnchor>,
    zombies: HashMap<EntityId, BoardVisualAnchor>,
    zombie_in_pool: HashMap<EntityId, bool>,
    coins: HashMap<EntityId, (f32, f32)>,
}

impl BoardVisualAnchors {
    fn from_state(state: &neopvz_core::GameState) -> Self {
        Self {
            plants: state
                .board
                .plants
                .iter()
                .map(|plant| {
                    (
                        plant.id,
                        BoardVisualAnchor {
                            x: 80.0 + f32::from(plant.column) * 80.0,
                            y: board_row_y(plant.row),
                            previous_x: 80.0 + f32::from(plant.column) * 80.0,
                            previous_y: board_row_y(plant.row),
                            row: plant.row,
                            zombie_type: None,
                        },
                    )
                })
                .collect(),
            plant_types: state
                .board
                .plants
                .iter()
                .map(|plant| (plant.id, plant.plant_type))
                .collect(),
            plant_health: state
                .board
                .plants
                .iter()
                .map(|plant| (plant.id, (plant.health, plant.max_health)))
                .collect(),
            projectiles: state
                .board
                .projectiles
                .iter()
                .map(|projectile| {
                    let x = fixed_point_to_logical(projectile.position_x);
                    let y = board_projectile_y(
                        projectile.position_y,
                        projectile.row,
                        projectile.lob_height,
                    );
                    (
                        projectile.id,
                        BoardVisualAnchor {
                            x,
                            y,
                            previous_x: x - fixed_point_to_logical(projectile.velocity_x),
                            previous_y: y
                                - fixed_point_to_logical(projectile.velocity_y)
                                - projectile.lob_velocity as f32 / 1_000.0,
                            row: projectile.row,
                            zombie_type: None,
                        },
                    )
                })
                .collect(),
            zombies: state
                .board
                .zombies
                .iter()
                .map(|zombie| {
                    (
                        zombie.id,
                        BoardVisualAnchor {
                            x: fixed_point_to_logical(zombie.position_x),
                            y: board_row_y(zombie.row) + 18.0,
                            previous_x: fixed_point_to_logical(zombie.position_x),
                            previous_y: board_row_y(zombie.row) + 18.0,
                            row: zombie.row,
                            zombie_type: Some(zombie.zombie_type),
                        },
                    )
                })
                .collect(),
            zombie_in_pool: state
                .board
                .zombies
                .iter()
                .map(|zombie| (zombie.id, zombie.in_pool))
                .collect(),
            coins: state
                .board
                .coins
                .iter()
                .map(|coin| {
                    (
                        coin.id,
                        (
                            fixed_point_to_logical(coin.position_x),
                            fixed_point_to_logical(coin.position_y),
                        ),
                    )
                })
                .collect(),
        }
    }
}

fn particle_effects_for_event(
    event: &GameEvent,
    scene: SceneKind,
    anchors: &BoardVisualAnchors,
    current_anchors: &BoardVisualAnchors,
) -> Vec<(&'static str, f32, f32, i32)> {
    if matches!(
        event,
        GameEvent::PuzzleStageStarted { .. } | GameEvent::BeghouledShuffled
    ) {
        return vec![("ScreenFlash", 400.0, 300.0, 30)];
    }
    if let GameEvent::PortalOpened {
        row,
        column,
        square,
    } = event
    {
        let (name, x_offset, y_offset) = if *square {
            ("PortalSquare", 5.0, -24.0)
        } else {
            ("PortalCircle", 13.0, -39.0)
        };
        return vec![(
            name,
            80.0 + f32::from(*column) * 80.0 + x_offset,
            board_row_y(*row) + y_offset,
            11,
        )];
    }
    if let GameEvent::ImitaterMorphed { entity, .. } = event {
        let Some(anchor) = anchors
            .plants
            .get(entity)
            .or_else(|| current_anchors.plants.get(entity))
        else {
            return Vec::new();
        };
        return vec![("ImitaterMorph", anchor.x + 40.0, anchor.y + 40.0, 13)];
    }
    if let GameEvent::PlantPlaced {
        plant_type,
        row,
        column,
        ..
    } = event
    {
        if scene == SceneKind::Garden || *plant_type == PlantType::Other(35) {
            return Vec::new();
        }
        let name = if matches!(scene, SceneKind::Pool | SceneKind::Fog) && matches!(row, 2 | 3) {
            "PlantingPool"
        } else {
            "Planting"
        };
        let y_offset = match plant_type {
            PlantType::Other(16) => 89.0,
            PlantType::Other(33) => 104.0,
            _ => 74.0,
        };
        let mut effects = vec![(
            name,
            121.0 + f32::from(*column) * 80.0,
            board_row_y(*row) + y_offset,
            13,
        )];
        if *plant_type == PlantType::Other(25) {
            effects.push((
                "LanternShine",
                120.0 + f32::from(*column) * 80.0,
                board_row_y(*row) + 40.0,
                10,
            ));
        }
        return effects;
    }
    if let GameEvent::PotatoMineArmed { entity } = event {
        let Some(anchor) = anchors
            .plants
            .get(entity)
            .or_else(|| current_anchors.plants.get(entity))
        else {
            return Vec::new();
        };
        return vec![("PotatoMineRise", anchor.x + 40.0, anchor.y + 40.0, 10)];
    }
    if let GameEvent::PlantSpecialTriggered { entity, plant_type } = event {
        let (name, z) = match plant_type {
            PlantType::Other(2) => ("Powie", 13),
            PlantType::Other(4) => ("PotatoMine", 11),
            PlantType::Other(11) => ("GraveBuster", 13),
            PlantType::Other(14) => ("IceTrap", 13),
            PlantType::Other(15) => ("Doom", 13),
            PlantType::Other(17) => ("Dust_Squash", 13),
            PlantType::Other(49) => ("Powie", 13),
            _ => return Vec::new(),
        };
        let Some(anchor) = anchors
            .plants
            .get(entity)
            .or_else(|| current_anchors.plants.get(entity))
        else {
            return Vec::new();
        };
        let y = if *plant_type == PlantType::Other(17) {
            anchor.y + if scene == SceneKind::Roof { 69.0 } else { 80.0 }
        } else {
            anchor.y + 40.0
        };
        return vec![(name, anchor.x + 40.0, y, z)];
    }
    if let GameEvent::GraveCleared { entity, .. } = event {
        let Some(anchor) = anchors
            .plants
            .get(entity)
            .or_else(|| current_anchors.plants.get(entity))
        else {
            return Vec::new();
        };
        return vec![("GraveBusterDie", anchor.x + 40.0, anchor.y + 40.0, 13)];
    }
    if let GameEvent::PlantFired {
        entity, plant_type, ..
    } = event
    {
        let (name, offset_x) = match plant_type {
            PlantType::Other(5) => ("SnowPeaPuff", 18.0),
            PlantType::Other(8) => ("PuffShroomMuzzle", 28.0),
            PlantType::Other(13) => ("PuffShroomMuzzle", 37.0),
            _ => return Vec::new(),
        };
        let Some(anchor) = anchors
            .plants
            .get(entity)
            .or_else(|| current_anchors.plants.get(entity))
        else {
            return Vec::new();
        };
        return vec![(name, anchor.x + offset_x, anchor.y + 18.0, 11)];
    }
    if let GameEvent::PlantParticleTriggered {
        entity, plant_type, ..
    } = event
    {
        let (name, x_offset, y_offset) = match plant_type {
            PlantType::Other(10) => ("FumeCloud", 85.0, 31.0),
            PlantType::Other(42) => ("GloomCloud", 40.0, 40.0),
            _ => return Vec::new(),
        };
        let Some(anchor) = anchors
            .plants
            .get(entity)
            .or_else(|| current_anchors.plants.get(entity))
        else {
            return Vec::new();
        };
        return vec![(name, anchor.x + x_offset, anchor.y + y_offset, 11)];
    }
    if let GameEvent::ZombieChew {
        entity,
        target: Some(plant),
        ..
    } = event
    {
        let Some(plant_type) = anchors
            .plant_types
            .get(plant)
            .copied()
            .or_else(|| current_anchors.plant_types.get(plant).copied())
        else {
            return Vec::new();
        };
        if !matches!(plant_type, PlantType::Other(3 | 23)) {
            return Vec::new();
        }
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        let (x_offset, y_offset) = match anchor.zombie_type {
            Some(ZombieType::Snorkel | ZombieType::DolphinRider) => (-7.0, 92.0),
            Some(ZombieType::Balloon) => (37.0, 69.0),
            Some(ZombieType::Imp) => (61.0, 62.0),
            _ => (37.0, 22.0),
        };
        return vec![(
            "WallnutEatSmall",
            anchor.x + x_offset,
            anchor.y + y_offset,
            11,
        )];
    }
    if let GameEvent::PlantDamaged {
        entity,
        damage,
        health_remaining,
    } = event
    {
        let Some(plant_type) = anchors
            .plant_types
            .get(entity)
            .copied()
            .or_else(|| current_anchors.plant_types.get(entity).copied())
        else {
            return Vec::new();
        };
        if !matches!(plant_type, PlantType::Other(3 | 23)) {
            return Vec::new();
        }
        let Some((_, max_health)) = anchors
            .plant_health
            .get(entity)
            .copied()
            .or_else(|| current_anchors.plant_health.get(entity).copied())
        else {
            return Vec::new();
        };
        let previous_health = anchors
            .plant_health
            .get(entity)
            .map_or(health_remaining.saturating_add(*damage), |(health, _)| {
                *health
            });
        let large_threshold = max_health * 2 / 3;
        let small_threshold = max_health / 3;
        if !((previous_health >= large_threshold && *health_remaining < large_threshold)
            || (previous_health >= small_threshold && *health_remaining < small_threshold))
        {
            return Vec::new();
        }
        let Some(anchor) = anchors
            .plants
            .get(entity)
            .or_else(|| current_anchors.plants.get(entity))
        else {
            return Vec::new();
        };
        let y_offset = if plant_type == PlantType::Other(23) {
            -22.0
        } else {
            10.0
        };
        return vec![("WallnutEatLarge", anchor.x + 40.0, anchor.y + y_offset, 13)];
    }
    if let GameEvent::VaseRevealed {
        row,
        column,
        contents,
        leaf,
        ..
    } = event
    {
        let name = if *leaf {
            "VaseShatterLeaf"
        } else if matches!(contents, VaseContents::Zombie(_)) {
            "VaseShatterZombie"
        } else {
            "VaseShatter"
        };
        return vec![(
            name,
            80.0 + f32::from(*column) * 80.0 + 20.0,
            board_row_y(*row),
            13,
        )];
    }
    if let GameEvent::ZombieGraveRumble { entity } = event {
        if scene != SceneKind::Night {
            return Vec::new();
        }
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        return vec![("ZombieRise", anchor.x + 60.0, anchor.y + 92.0, 11)];
    }
    if let GameEvent::ZombieWhackRise { entity } = event {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        return vec![("WhackAZombieRise", anchor.x + 60.0, anchor.y + 92.0, 11)];
    }
    if let GameEvent::WhackHit {
        x,
        y,
        sound: WhackHitSound::Bonk,
        ..
    } = event
    {
        return vec![("Pow", f32::from(*x) - 3.0, f32::from(*y) + 9.0, 30)];
    }
    if let GameEvent::ZombieSpawned {
        entity,
        zombie_type: ZombieType::Digger,
        ..
    } = event
    {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        return vec![("DiggerTunnel", anchor.x + 60.0, anchor.y + 82.0, 11)];
    }
    if let GameEvent::ZombieSpawned {
        entity,
        zombie_type: ZombieType::BackupDancer,
        ..
    } = event
    {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        return vec![("DancerRise", anchor.x + 60.0, anchor.y + 92.0, 11)];
    }
    if let GameEvent::MowerZombieHit { entity, pool, .. } = event {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        if !*pool
            && !matches!(
                anchor.zombie_type,
                Some(
                    ZombieType::Balloon
                        | ZombieType::Bobsled
                        | ZombieType::Bungee
                        | ZombieType::Digger
                        | ZombieType::DolphinRider
                        | ZombieType::Gargantuar
                        | ZombieType::Imp
                        | ZombieType::Pogo
                        | ZombieType::Yeti
                )
            )
        {
            return Vec::new();
        }
        return vec![("MowerCloud", anchor.x + 110.0, anchor.y - 18.0, 11)];
    }
    if let GameEvent::ZombieDied { entity } = event {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        if anchor.zombie_type != Some(ZombieType::Boss) {
            return Vec::new();
        }
        return vec![("BossExplosion", 700.0, 150.0, 11)];
    }
    if let GameEvent::DiggerSurfaced { entity } = event {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        return vec![("DiggerRise", anchor.x + 60.0, anchor.y + 100.0, 11)];
    }
    if let GameEvent::ZombieThawed { entity } = event {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        return vec![("IceTrapRelease", anchor.x + 75.0, anchor.y + 106.0, 11)];
    }
    if let GameEvent::JumpBlocked { plant, .. } = event {
        let Some(anchor) = anchors
            .plants
            .get(plant)
            .or_else(|| current_anchors.plants.get(plant))
        else {
            return Vec::new();
        };
        return vec![("TallNutBlock", anchor.x + 60.0, anchor.y - 20.0, 11)];
    }
    if let GameEvent::ZombieHypnotized { entity } = event {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        return vec![("MindControl", anchor.x + 60.0, anchor.y + 40.0, 11)];
    }
    if let GameEvent::JackboxExploded { entity, .. } = event {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        return vec![("JackExplode", anchor.x + 40.0, anchor.y + 22.0, 13)];
    }
    if let GameEvent::ZombieEnteredPool { entity, .. } = event {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        return vec![("PlantingPool", anchor.x + 60.0, anchor.y + 102.0, 11)];
    }
    if let GameEvent::TangleKelpWaterEntry { entity } = event {
        let Some(anchor) = anchors
            .plants
            .get(entity)
            .or_else(|| current_anchors.plants.get(entity))
        else {
            return Vec::new();
        };
        return vec![("PlantingPool", anchor.x + 31.0, anchor.y + 64.0, 11)];
    }
    if let GameEvent::ZombieNewspaperRipped { entity } = event {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        return vec![("ZombieNewspaper", anchor.x + 60.0, anchor.y + 42.0, 11)];
    }
    if let GameEvent::ZombieDamageTierChanged { entity, tier } = event {
        if *tier != 2 {
            return Vec::new();
        }
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        let (x, y) = match anchor.zombie_type {
            Some(ZombieType::Zamboni) => (anchor.x + 27.0, anchor.y + 54.0),
            Some(ZombieType::Catapult) => (anchor.x + 47.0, anchor.y + 59.0),
            _ => return Vec::new(),
        };
        return vec![("ZamboniSmoke", x, y, 11)];
    }
    if let GameEvent::VehicleDisabled { entity } = event {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        if !matches!(
            anchor.zombie_type,
            Some(ZombieType::Zamboni | ZombieType::Catapult)
        ) {
            return Vec::new();
        }
        return vec![("ZamboniTire", anchor.x + 29.0, anchor.y + 96.0, 11)];
    }
    if let GameEvent::PogoStickLost { entity } = event {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        return vec![("ZombiePogo", anchor.x + 60.0, anchor.y + 80.0, 11)];
    }
    if let GameEvent::VehicleExploded { entity } = event {
        return vehicle_explosion_particle(*entity, false, anchors, current_anchors);
    }
    if let GameEvent::ZombieMustacheDropped { entity, .. }
    | GameEvent::ZombieFutureGlassesDropped { entity, .. }
    | GameEvent::ZombiePinataDropped { entity } = event
    {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        let name = match event {
            GameEvent::ZombieMustacheDropped { .. } => "ZombieMustache",
            GameEvent::ZombieFutureGlassesDropped { .. } => "ZombieFutureGlasses",
            _ => "Pinata",
        };
        return vec![(name, anchor.x + 60.0, anchor.y + 40.0, 11)];
    }
    if let GameEvent::ZombieDaisiesDropped { entity } = event {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        let (x, mut y) = match anchor.zombie_type {
            Some(ZombieType::Football | ZombieType::Dancer | ZombieType::BackupDancer) => {
                (anchor.x + 180.0, anchor.y + 100.0)
            }
            Some(ZombieType::Pogo) => (anchor.x + 20.0, anchor.y + 120.0),
            Some(ZombieType::Balloon) => (anchor.x + 130.0, anchor.y + 130.0),
            _ => (anchor.x + 20.0, anchor.y + 100.0),
        };
        if scene == SceneKind::Night {
            y += 15.0;
        }
        return vec![("Daisy", x, y, 11)];
    }
    if let GameEvent::ZombieBodyPartLost { entity, head } = event {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        let name = if *head {
            match anchor.zombie_type {
                Some(ZombieType::Newspaper) => "ZombieNewspaperHead",
                Some(ZombieType::Pogo) => "ZombiePogoHead",
                Some(ZombieType::Balloon) => "ZombieBalloonHead",
                _ if anchors
                    .zombie_in_pool
                    .get(entity)
                    .copied()
                    .or_else(|| current_anchors.zombie_in_pool.get(entity).copied())
                    .unwrap_or(false) =>
                {
                    "ZombieHeadPool"
                }
                _ => "ZombieHead",
            }
        } else {
            "ZombieArm"
        };
        return vec![(
            name,
            anchor.x + 60.0,
            anchor.y + if *head { 40.0 } else { 50.0 },
            11,
        )];
    }
    if let GameEvent::ZombieArmorLost { entity } = event {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        let name = match anchor.zombie_type {
            Some(ZombieType::Conehead) => "ZombieTrafficCone",
            Some(ZombieType::Buckethead) => "ZombiePail",
            Some(ZombieType::Football) => "ZombieHelmet",
            Some(ZombieType::Digger) => "ZombieHeadLight",
            _ => return Vec::new(),
        };
        return vec![(name, anchor.x + 60.0, anchor.y + 40.0, 11)];
    }
    if let GameEvent::ZombieShieldLost { entity } = event {
        let Some(anchor) = anchors
            .zombies
            .get(entity)
            .or_else(|| current_anchors.zombies.get(entity))
        else {
            return Vec::new();
        };
        let (name, x, y) = match anchor.zombie_type {
            Some(ZombieType::ScreenDoor) => ("ZombieDoor", anchor.x + 60.0, anchor.y + 50.0),
            Some(ZombieType::Ladder) => ("ZombieLadder", anchor.x + 31.0, anchor.y + 62.0),
            _ => return Vec::new(),
        };
        return vec![(name, x, y, 11)];
    }
    if let GameEvent::PickupCollected {
        entity, coin_type, ..
    } = event
    {
        let Some(&(x, y)) = anchors
            .coins
            .get(entity)
            .or_else(|| current_anchors.coins.get(entity))
        else {
            return Vec::new();
        };
        let mut effects = Vec::with_capacity(2);
        if is_completion_award_coin(*coin_type) {
            effects.push(("Starburst", x + 30.0, y + 30.0, 13));
        }
        if matches!(
            coin_type,
            CoinType::PresentPlant
                | CoinType::Chocolate
                | CoinType::AwardChocolate
                | CoinType::PresentMinigames
                | CoinType::PresentPuzzleMode
                | CoinType::PresentSurvivalMode
                | CoinType::Note
        ) {
            effects.push(("PresentPickup", x + 30.0, y + 30.0, 13));
        }
        return effects;
    }
    let GameEvent::ProjectileImpact {
        projectile,
        projectile_type,
        zombie,
        kind,
        variant,
        ..
    } = event
    else {
        return Vec::new();
    };
    let Some(anchor) = anchors
        .projectiles
        .get(projectile)
        .or_else(|| current_anchors.projectiles.get(projectile))
        .or_else(|| {
            zombie
                .as_ref()
                .and_then(|zombie| anchors.zombies.get(zombie))
        })
        .or_else(|| {
            zombie
                .as_ref()
                .and_then(|zombie| current_anchors.zombies.get(zombie))
        })
    else {
        return Vec::new();
    };
    if zombie.is_none() && *kind == ProjectileImpactSound::Splat && *variant == 3 {
        return vec![("UmbrellaReflect", anchor.x + 20.0, anchor.y + 20.0, 13)];
    }
    if *projectile_type == ProjectileType::Cob && *kind == ProjectileImpactSound::Splat {
        return vec![
            ("BlastMark", anchor.x + 80.0, anchor.y + 40.0, 2),
            ("PopcornSplash", anchor.x + 80.0, anchor.y + 40.0, 13),
        ];
    }
    let (name, x, y) = match (*projectile_type, *kind) {
        (ProjectileType::Pea, ProjectileImpactSound::Splat | ProjectileImpactSound::Shield) => {
            ("PeaSplat", anchor.x - 3.0, anchor.y + 12.0)
        }
        (ProjectileType::SnowPea, ProjectileImpactSound::Splat | ProjectileImpactSound::Shield) => {
            ("SnowPeaSplat", anchor.x - 3.0, anchor.y + 12.0)
        }
        (ProjectileType::Puff, ProjectileImpactSound::Splat | ProjectileImpactSound::Shield) => {
            ("PuffSplat", anchor.x - 8.0, anchor.y + 12.0)
        }
        (ProjectileType::Star, ProjectileImpactSound::Splat | ProjectileImpactSound::Shield) => {
            ("StarSplat", anchor.x + 12.0, anchor.y + 12.0)
        }
        (ProjectileType::Cabbage, ProjectileImpactSound::Splat | ProjectileImpactSound::Shield) => {
            (
                "CabbageSplat",
                anchor.previous_x - 38.0,
                anchor.previous_y + 23.0,
            )
        }
        (ProjectileType::Butter, ProjectileImpactSound::Butter) => (
            "ButterSplat",
            anchor.previous_x - 20.0,
            anchor.previous_y + 63.0,
        ),
        (ProjectileType::Melon, ProjectileImpactSound::Melon) => (
            "MelonImpact",
            anchor.previous_x + 30.0,
            anchor.previous_y + 30.0,
        ),
        (ProjectileType::WinterMelon, ProjectileImpactSound::Melon) => (
            "WinterMelonImpact",
            anchor.previous_x + 30.0,
            anchor.previous_y + 30.0,
        ),
        (ProjectileType::ZombiePea, ProjectileImpactSound::Splat) => {
            ("PeaSplat", anchor.x - 3.0, anchor.y + 17.0)
        }
        _ => return Vec::new(),
    };
    // ponytail: systems stay in world space; add zombie attachments when moving-impact fidelity needs them.
    vec![(name, x, y, 13)]
}

fn vehicle_explosion_particle(
    entity: EntityId,
    mowed: bool,
    anchors: &BoardVisualAnchors,
    current_anchors: &BoardVisualAnchors,
) -> Vec<(&'static str, f32, f32, i32)> {
    let Some(anchor) = anchors
        .zombies
        .get(&entity)
        .or_else(|| current_anchors.zombies.get(&entity))
    else {
        return Vec::new();
    };
    let name = match (anchor.zombie_type, mowed) {
        (Some(ZombieType::Zamboni), true) => "ZamboniExplosion2",
        (Some(ZombieType::Zamboni), false) => "ZamboniExplosion",
        (Some(ZombieType::Catapult), _) => "CatapultExplosion",
        _ => return Vec::new(),
    };
    vec![(name, anchor.x + 80.0, anchor.y + 42.0, 11)]
}

fn particle_effects_for_event_batch(
    event: &GameEvent,
    events: &[GameEvent],
    scene: SceneKind,
    anchors: &BoardVisualAnchors,
    current_anchors: &BoardVisualAnchors,
) -> Vec<(&'static str, f32, f32, i32)> {
    if let GameEvent::VehicleExploded { entity } = event
        && events.iter().any(
            |candidate| matches!(candidate, GameEvent::MowerZombieHit { entity: hit, .. } if hit == entity),
        )
    {
        return vehicle_explosion_particle(*entity, true, anchors, current_anchors);
    }
    particle_effects_for_event(event, scene, anchors, current_anchors)
}

fn projectile_particle_trail(projectile_type: ProjectileType) -> Option<(&'static str, f32, f32)> {
    match projectile_type {
        ProjectileType::SnowPea => Some(("SnowPeaTrail", 8.0, 13.0)),
        ProjectileType::Puff => Some(("PuffShroomTrail", 13.0, 13.0)),
        _ => None,
    }
}

fn coin_arrow_particle(
    coin_type: CoinType,
    needs_bouncy_arrow: bool,
) -> Option<(&'static str, f32, f32)> {
    if !needs_bouncy_arrow {
        return None;
    }
    Some(match coin_type {
        CoinType::FinalSeedPacket => ("SeedPacket", 25.0, -25.0),
        CoinType::Silver | CoinType::Gold => ("CoinPickupArrow", 32.0, -17.0),
        CoinType::Trophy => ("AwardPickupArrow", 43.0, -29.0),
        CoinType::AwardMoneyBag | CoinType::AwardBagDiamond => ("AwardPickupArrow", 47.0, -23.0),
        CoinType::AwardPresent
        | CoinType::PresentMinigames
        | CoinType::PresentPuzzleMode
        | CoinType::PresentSurvivalMode => ("AwardPickupArrow", 40.0, -40.0),
        CoinType::AwardSilverSunflower | CoinType::AwardGoldSunflower => {
            ("AwardPickupArrow", 72.0, 34.0)
        }
        CoinType::Shovel
        | CoinType::Almanac
        | CoinType::CarKeys
        | CoinType::WateringCan
        | CoinType::Taco => ("AwardPickupArrow", 40.0, -20.0),
        CoinType::Vase => ("AwardPickupArrow", 40.0, -10.0),
        CoinType::Note => ("AwardPickupArrow", 39.0, -34.0),
        CoinType::AwardChocolate => ("AwardPickupArrow", 28.0, -27.0),
        _ => return None,
    })
}

#[derive(Clone, Copy, Debug)]
struct BoardFireEffect {
    x: f32,
    y: f32,
    start_tick: u64,
    duration: u64,
    z: i32,
}

struct App {
    renderer: Option<GpuRenderer>,
    assets: Vec<ImageAsset>,
    resources: ResourceProvider,
    reanim_catalog: ReanimCatalog,
    particle_catalog: ParticleCatalog,
    particle_images: HashMap<String, ParticleImage>,
    particle_holder: ParticleHolder,
    projectile_particle_systems: HashMap<EntityId, (usize, ProjectileType)>,
    zombie_seaweed_particle_systems: HashMap<EntityId, Vec<(usize, usize)>>,
    coin_arrow_particle_systems: HashMap<EntityId, usize>,
    pool_sparkly_particle_system: Option<usize>,
    ice_sparkle_particle_systems: HashMap<u8, usize>,
    credits_fog_particle_system: Option<usize>,
    audio: Option<KiraAudioBackend>,
    game: Game,
    pending_input: Vec<InputAction>,
    last_update: Option<Instant>,
    simulation_accumulator: Duration,
    cursor_position: Option<PhysicalPosition<f64>>,
    beghouled_drag_start: Option<(u8, u8)>,
    tutorial_page: u8,
    seed_chooser_selection: Vec<bool>,
    seed_chooser_scroll: usize,
    garden_tool: GardenTool,
    store_open: bool,
    options_open: bool,
    help_open: bool,
    almanac_open: bool,
    credits_open: bool,
    credits_frame: u32,
    credits_paused: bool,
    almanac_page: u8,
    almanac_selected_plant: u8,
    almanac_selected_zombie: u8,
    almanac_strings: HashMap<String, String>,
    almanac_loaded_descriptions: HashSet<u32>,
    seed_bank_cost_plants: HashMap<u8, PlantType>,
    sun_count_asset_value: Option<u32>,
    progress_meter_asset_width: Option<u32>,
    options_music_volume: u8,
    options_effects_volume: u8,
    selected_mode: ModeKind,
    selected_level: u8,
    debug_mode_select: bool,
    fullscreen: bool,
    capture_path: Option<PathBuf>,
    startup_events: Vec<GameEvent>,
    startup_anchors: Option<BoardVisualAnchors>,
    startup_coin_collection: Option<EntityId>,
    startup_particle_warmup: usize,
    visual_effects: Vec<BoardFireEffect>,
    title_load_state: TitleLoadState,
    profile: Option<SaveProfile>,
}

impl App {
    fn new(
        loaded_assets: LoadedAssets,
        resources: ResourceProvider,
        audio: Option<KiraAudioBackend>,
        initial_scene: SceneKind,
        fullscreen: bool,
        checkpoint: Option<Checkpoint>,
        profile: Option<SaveProfile>,
    ) -> Self {
        let (assets, reanim_catalog, particle_catalog, particle_images) = loaded_assets;
        let debug_mode_select = matches!(checkpoint, Some(Checkpoint::ModeSelect));
        let mut game = match checkpoint {
            Some(
                Checkpoint::GardenWater
                | Checkpoint::GardenFertilize
                | Checkpoint::GardenFulfill
                | Checkpoint::GardenBugSprayAudio
                | Checkpoint::GardenPhonographAudio
                | Checkpoint::GardenLeaveAudio,
            ) => Game::new_mode(0, ModeKind::ZenGarden, 0),
            Some(Checkpoint::AquariumTapGlass) => Game::new_mode(0, ModeKind::ZenGarden, 2),
            Some(Checkpoint::GardenTreeGrow) => Game::new_mode(7, ModeKind::ZenGarden, 3),
            Some(Checkpoint::HugeWaveSound) => Game::new_mode(7, ModeKind::Adventure, 6),
            Some(Checkpoint::FinalWaveSound) => Game::new_mode(7, ModeKind::Adventure, 1),
            Some(Checkpoint::ReadySetPlantAudio) => Game::new_adventure(7, 8, true, 0, false),
            Some(Checkpoint::FinalFanfare) => Game::new_adventure(7, 50, false, 0, false),
            Some(
                Checkpoint::ZombiquariumSnorkel
                | Checkpoint::ZombiquariumBrain
                | Checkpoint::ZombiquariumDeath,
            ) => Game::new_mode(7, ModeKind::MiniGame, 7),
            Some(Checkpoint::WhackAudio | Checkpoint::WhackRiseParticle) => {
                Game::new_mode(7, ModeKind::MiniGame, 14)
            }
            Some(Checkpoint::BodyPartAudio) => Game::new(7, SceneKind::Day),
            Some(Checkpoint::HiddenCodeEffects) => Game::new(7, SceneKind::Day),
            Some(Checkpoint::BungeeAudio) => Game::new(7, SceneKind::Day),
            Some(Checkpoint::BungeeLiftAudio) => Game::new(7, SceneKind::Day),
            Some(Checkpoint::BungeeGrassstepAudio) => Game::new(7, SceneKind::Day),
            Some(Checkpoint::VehicleExplosion | Checkpoint::MowerVehicleExplosion) => {
                Game::new(7, SceneKind::Day)
            }
            Some(Checkpoint::ZombieFallingAudio) => Game::new(7, SceneKind::Day),
            Some(Checkpoint::DancerRumble) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::GarlicYuckAudio) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::MowerHitAudio) => Game::new(7, SceneKind::Day),
            Some(Checkpoint::MowerSquishAudio) => Game::new_mode(3, ModeKind::MiniGame, 19),
            Some(Checkpoint::FirstWaveSound) => Game::new_mode(7, ModeKind::Adventure, 1),
            Some(Checkpoint::FlagWaveSound) => Game::new_mode(7, ModeKind::Adventure, 6),
            Some(Checkpoint::BossAttack) => Game::new_mode(3, ModeKind::MiniGame, 19),
            Some(Checkpoint::BossRVAudio) => Game::new_mode(3, ModeKind::MiniGame, 19),
            Some(Checkpoint::BossStompAudio) => Game::new_mode(3, ModeKind::MiniGame, 19),
            Some(Checkpoint::BossDamageAudio) => Game::new_mode(3, ModeKind::MiniGame, 19),
            Some(Checkpoint::Butter) => Game::new(0, SceneKind::Day),
            Some(
                Checkpoint::ProjectileImpacts
                | Checkpoint::ProjectileParticleTail
                | Checkpoint::ProjectileParticleFade,
            ) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::PrizeChime) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::PrizeCollection) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::AwardCollectionAudio) => Game::new_adventure(7, 4, true, 0, false),
            Some(Checkpoint::LootDropAudio) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::LootChallengeAudio) => Game::new_adventure(7, 22, true, 0, false),
            Some(Checkpoint::WeatherAudio) => Game::new_adventure(7, 40, false, 0, false),
            Some(Checkpoint::SunPickupCollection) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::GoldCoinLanding) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::PickupArrowParticles) => Game::new_adventure(7, 11, true, 0, false),
            Some(Checkpoint::DiamondCollection) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::UsableSeedCollection) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::SunProduction) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::PlantFiring) => Game::new(0, SceneKind::Night),
            Some(Checkpoint::PlantingAudio) => Game::new(0, SceneKind::Pool),
            Some(Checkpoint::WallnutBowlingAudio | Checkpoint::WallnutBowlingImpactAudio) => {
                Game::new_mode(7, ModeKind::MiniGame, 1)
            }
            Some(Checkpoint::PlanternAudio) => Game::new(0, SceneKind::Night),
            Some(Checkpoint::Torchwood) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::VaseBreak) => Game::new_mode(0, ModeKind::Vasebreaker, 0),
            Some(Checkpoint::Rake) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::ExplosionPlants | Checkpoint::PlantExplosionParticles) => {
                Game::new(0, SceneKind::Night)
            }
            Some(Checkpoint::ExplodeONut) => Game::new_mode(0, ModeKind::MiniGame, 1),
            Some(Checkpoint::Squash) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::SquashHum) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::ZombieDeploy | Checkpoint::BrainEaten) => {
                Game::new_mode(0, ModeKind::IZombie, 0)
            }
            Some(Checkpoint::ImpThrow) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::NewspaperRip | Checkpoint::NewspaperRarrghAudio) => {
                Game::new(0, SceneKind::Day)
            }
            Some(Checkpoint::BloverChomper) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::HypnoJackbox) => Game::new(0, SceneKind::Night),
            Some(Checkpoint::JackboxAudio | Checkpoint::JackboxBoingAudio) => {
                Game::new(7, SceneKind::Day)
            }
            Some(Checkpoint::CobCannon) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::GraveBuster) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::Coffee) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::TangleKelp) => Game::new(0, SceneKind::Pool),
            Some(Checkpoint::DolphinJump) => Game::new(0, SceneKind::Pool),
            Some(Checkpoint::PoolEntry | Checkpoint::PoolRiseParticle) => {
                Game::new(0, SceneKind::Pool)
            }
            Some(Checkpoint::PoolMower) => Game::new(0, SceneKind::Pool),
            Some(Checkpoint::GraveRumbleAudio) => Game::new(0, SceneKind::Night),
            Some(Checkpoint::LadderAudio) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::Spikeweed) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::Digger) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::Magnet) => Game::new(0, SceneKind::Night),
            Some(Checkpoint::ShieldHit) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::Zamboni) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::Catapult) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::BalloonAppearance | Checkpoint::BalloonPopAudio) => {
                Game::new(0, SceneKind::Day)
            }
            Some(Checkpoint::PoleVault) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::PogoBlock) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::UmbrellaDeflect) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::Complete) => {
                let mut game = Game::new_adventure(7, 1, true, 0, false);
                game.debug_prepare_game_won();
                game.advance(InputFrame::default());
                game
            }
            Some(Checkpoint::CompletePaper) => {
                let mut game = Game::new_adventure(7, 10, true, 0, false);
                game.debug_prepare_game_won();
                game
            }
            Some(Checkpoint::Credits | Checkpoint::CreditsParticles) => {
                let mut game = Game::new_adventure(7, 50, true, 0, false);
                game.debug_prepare_game_won();
                game.advance(InputFrame::default());
                game
            }
            _ => new_scene_game(initial_scene),
        };
        apply_profile_to_game(&mut game, profile.as_ref());
        let mut pending_input = Vec::new();
        let mut startup_events = Vec::new();
        let mut startup_anchors = None;
        let mut startup_coin_collection = None;
        match checkpoint {
            Some(Checkpoint::GameOver) => game.debug_force_game_over(),
            Some(Checkpoint::GameLost) => game.debug_prepare_game_lost(),
            Some(Checkpoint::GameLostAudio) => {
                startup_events = game.debug_prepare_game_lost_audio()
            }
            Some(Checkpoint::GameWon) => game.debug_prepare_game_won(),
            Some(Checkpoint::FinalFanfare) => game.debug_prepare_game_won(),
            Some(Checkpoint::CompletePaper) => startup_events = game.advance(InputFrame::default()),
            Some(Checkpoint::Pickups) => {
                let (sun, coin) = game.debug_prepare_pickups();
                pending_input.push(InputAction::CollectSun { entity: sun });
                pending_input.push(InputAction::CollectCoin { entity: coin });
            }
            Some(Checkpoint::SunProduction) => game.debug_prepare_sun_production(),
            Some(Checkpoint::PlantFiring) => {
                startup_events = game.debug_prepare_plant_firing_audio()
            }
            Some(Checkpoint::PlantingAudio) => startup_events = game.debug_prepare_planting_audio(),
            Some(Checkpoint::WallnutBowlingAudio) => {
                pending_input.extend([
                    InputAction::SelectSeed { slot: 0 },
                    InputAction::Plant { row: 2, column: 0 },
                ]);
            }
            Some(Checkpoint::WallnutBowlingImpactAudio) => {
                startup_events = game.debug_prepare_wallnut_bowling_impact()
            }
            Some(Checkpoint::PlanternAudio) => startup_events = game.debug_prepare_plantern_audio(),
            Some(Checkpoint::ReadySetPlantAudio) => {
                startup_events = game.debug_prepare_ready_set_plant_audio()
            }
            Some(Checkpoint::Torchwood) => startup_events = game.debug_prepare_torchwood(),
            Some(Checkpoint::PrizeChime) => startup_events = game.debug_prepare_prize_chime(),
            Some(Checkpoint::PrizeCollection) => {
                startup_events = game.debug_prepare_prize_collection();
                let entity = startup_events.iter().find_map(|event| match event {
                    GameEvent::PickupCollected { entity, .. } => Some(*entity),
                    _ => None,
                });
                if let Some(entity) = entity {
                    startup_anchors = Some(BoardVisualAnchors {
                        coins: HashMap::from([(entity, (300.0, 200.0))]),
                        ..BoardVisualAnchors::default()
                    });
                }
            }
            Some(Checkpoint::AwardCollectionAudio) => {
                startup_events = game.debug_prepare_award_collection_audio();
                let entity = startup_events.iter().find_map(|event| match event {
                    GameEvent::PickupCollected { entity, .. } => Some(*entity),
                    _ => None,
                });
                if let Some(entity) = entity {
                    startup_anchors = Some(BoardVisualAnchors {
                        coins: HashMap::from([(entity, (300.0, 200.0))]),
                        ..BoardVisualAnchors::default()
                    });
                }
            }
            Some(Checkpoint::LootDropAudio) => {
                startup_events = game.debug_prepare_loot_drop_audio()
            }
            Some(Checkpoint::LootChallengeAudio) => {
                startup_events = game.debug_prepare_loot_challenge_audio()
            }
            Some(Checkpoint::WeatherAudio) => game.debug_prepare_weather_audio(),
            Some(Checkpoint::SunPickupCollection) => {
                startup_events = game.debug_prepare_sun_pickup_collection()
            }
            Some(Checkpoint::GoldCoinLanding) => {
                startup_events = game.debug_prepare_gold_coin_landing()
            }
            Some(Checkpoint::PickupArrowParticles) => {
                let (events, entity) = game.debug_prepare_pickup_arrow_particles();
                startup_events = events;
                startup_coin_collection = Some(entity);
            }
            Some(Checkpoint::DiamondCollection) => {
                startup_events = game.debug_prepare_diamond_collection()
            }
            Some(Checkpoint::UsableSeedCollection) => {
                startup_events = game.debug_prepare_usable_seed_collection()
            }
            Some(Checkpoint::GardenWater) => {
                pending_input.push(InputAction::GardenWater { plant: 0 });
            }
            Some(Checkpoint::GardenFertilize) => {
                pending_input.push(InputAction::GardenFertilize { plant: 0 });
            }
            Some(Checkpoint::GardenFulfill) => {
                startup_events = game.debug_prepare_garden_fulfill();
            }
            Some(Checkpoint::GardenBugSprayAudio) => {
                startup_events = game.debug_prepare_garden_tool_audio(GardenTool::BugSpray);
            }
            Some(Checkpoint::GardenPhonographAudio) => {
                startup_events = game.debug_prepare_garden_tool_audio(GardenTool::Phonograph);
            }
            Some(Checkpoint::GardenLeaveAudio) => {
                startup_events = game.debug_prepare_garden_leave_audio();
            }
            Some(Checkpoint::AquariumTapGlass) => {
                startup_events = game.debug_prepare_aquarium_tap_glass();
            }
            Some(Checkpoint::GardenTreeGrow) => {
                startup_events = game.debug_prepare_garden_tree_grow();
            }
            Some(Checkpoint::HugeWaveSound) => game.debug_prepare_huge_wave_sound(),
            Some(Checkpoint::FinalWaveSound) => game.debug_prepare_final_wave_sound(),
            Some(Checkpoint::ZombiquariumSnorkel) => {
                startup_events = game.debug_prepare_zombiquarium_snorkel()
            }
            Some(Checkpoint::ZombiquariumBrain) => {
                startup_events = game.debug_prepare_zombiquarium_brain()
            }
            Some(Checkpoint::ZombiquariumDeath) => {
                startup_events = game.debug_prepare_zombiquarium_death()
            }
            Some(Checkpoint::WhackAudio) => startup_events = game.debug_prepare_whack_audio(),
            Some(Checkpoint::WhackRiseParticle) => {
                startup_events = game.debug_prepare_whack_zombie_rise()
            }
            Some(Checkpoint::BodyPartAudio) => {
                startup_events = game.debug_prepare_body_part_audio()
            }
            Some(Checkpoint::HiddenCodeEffects) => {
                startup_events = game.debug_prepare_hidden_code_effects()
            }
            Some(Checkpoint::BungeeAudio) => startup_events = game.debug_prepare_bungee_audio(),
            Some(Checkpoint::BungeeLiftAudio) => {
                startup_events = game.debug_prepare_bungee_lift_audio()
            }
            Some(Checkpoint::BungeeGrassstepAudio) => {
                startup_events = game.debug_prepare_bungee_grassstep_audio()
            }
            Some(Checkpoint::VehicleExplosion) => {
                startup_events = game.debug_prepare_vehicle_explosion()
            }
            Some(Checkpoint::MowerVehicleExplosion) => {
                game.debug_prepare_mower_vehicle_explosion();
                startup_anchors = Some(BoardVisualAnchors::from_state(game.state()));
                startup_events = game.advance(InputFrame::default());
            }
            Some(Checkpoint::ZombieFallingAudio) => {
                startup_events = game.debug_prepare_zombie_falling_audio()
            }
            Some(Checkpoint::DancerRumble) => startup_events = game.debug_prepare_dancer_rumble(),
            Some(Checkpoint::GarlicYuckAudio) => startup_events = game.debug_prepare_garlic_yuck(),
            Some(Checkpoint::MowerHitAudio) => startup_events = game.debug_prepare_mower_hit(),
            Some(Checkpoint::MowerSquishAudio) => {
                startup_events = game.debug_prepare_mower_squish()
            }
            Some(Checkpoint::FirstWaveSound) => {
                startup_events = game.debug_prepare_first_wave_sound()
            }
            Some(Checkpoint::FlagWaveSound) => {
                startup_events = game.debug_prepare_flag_wave_sound()
            }
            Some(Checkpoint::BossAttack) => startup_events = game.debug_prepare_boss_attack(),
            Some(Checkpoint::BossRVAudio) => startup_events = game.debug_prepare_boss_rv_audio(),
            Some(Checkpoint::BossStompAudio) => {
                startup_events = game.debug_prepare_boss_stomp_audio()
            }
            Some(Checkpoint::BossDamageAudio) => {
                startup_events = game.debug_prepare_boss_damage_audio()
            }
            Some(Checkpoint::IceShroom) => game.debug_prepare_ice_shroom(),
            Some(Checkpoint::IceShroomParticle) => {
                game.debug_prepare_ice_shroom();
                startup_anchors = Some(BoardVisualAnchors::from_state(game.state()));
                startup_events = game.advance(InputFrame::default());
            }
            Some(Checkpoint::PotatoMine) => game.debug_prepare_potato_mine(),
            Some(Checkpoint::PotatoMineRiseParticle) => {
                game.debug_prepare_potato_mine();
                startup_anchors = Some(BoardVisualAnchors::from_state(game.state()));
                startup_events = game.advance(InputFrame::default());
            }
            Some(Checkpoint::PotatoMineParticle) => {
                game.debug_prepare_potato_mine();
                let _ = game.advance(InputFrame::default());
                startup_anchors = Some(BoardVisualAnchors::from_state(game.state()));
                startup_events = game.advance(InputFrame::default());
            }
            Some(Checkpoint::ExplosionPlants) => {
                startup_events = game.debug_prepare_explosion_plants()
            }
            Some(Checkpoint::PlantExplosionParticles) => {
                let _ = game.debug_prepare_explosion_plants();
                startup_anchors = Some(BoardVisualAnchors::from_state(game.state()));
                startup_events = game.advance(InputFrame::default());
            }
            Some(Checkpoint::ExplodeONut) => game.debug_prepare_explode_o_nut(),
            Some(Checkpoint::Squash) => startup_events = game.debug_prepare_squash(),
            Some(Checkpoint::SquashHum) => startup_events = game.debug_prepare_squash_hum(),
            Some(Checkpoint::ZombieDeploy) => {
                pending_input.push(InputAction::DeployZombie {
                    zombie_type: neopvz_core::ZombieType::Normal,
                    row: 0,
                    column: 0,
                });
            }
            Some(Checkpoint::BrainEaten) => game.debug_prepare_brain_finished(),
            Some(Checkpoint::ImpThrow) => game.debug_prepare_imp_throw(),
            Some(Checkpoint::NewspaperRip) => startup_events = game.debug_prepare_newspaper_rip(),
            Some(Checkpoint::NewspaperRarrghAudio) => {
                startup_events = game.debug_prepare_newspaper_rarrgh()
            }
            Some(Checkpoint::Butter) => startup_events = game.debug_prepare_butter(),
            Some(Checkpoint::ProjectileImpacts) => {
                startup_events = game.debug_prepare_projectile_impacts()
            }
            Some(Checkpoint::ProjectileParticleTail | Checkpoint::ProjectileParticleFade) => {
                game.debug_prepare_projectile_particle_tail();
                startup_anchors = Some(BoardVisualAnchors::from_state(game.state()));
                startup_events = game.advance(InputFrame::default());
            }
            Some(Checkpoint::VaseBreak) => startup_events = game.debug_prepare_vase_break(),
            Some(Checkpoint::Rake) => startup_events = game.debug_prepare_rake(),
            Some(Checkpoint::BloverChomper) => game.debug_prepare_blover_chomper(),
            Some(Checkpoint::HypnoJackbox) => game.debug_prepare_hypno_jackbox(),
            Some(Checkpoint::JackboxAudio) => startup_events = game.debug_prepare_jackbox_audio(),
            Some(Checkpoint::JackboxBoingAudio) => {
                startup_events = game.debug_prepare_jackbox_boing()
            }
            Some(Checkpoint::CobCannon) => game.debug_prepare_cob_cannon(),
            Some(Checkpoint::Portal) => startup_events = game.debug_prepare_portal(),
            Some(Checkpoint::GraveBuster) => game.debug_prepare_gravebuster(),
            Some(Checkpoint::Coffee) => game.debug_prepare_coffee(),
            Some(Checkpoint::TangleKelp) => game.debug_prepare_tangle_kelp(),
            Some(Checkpoint::DolphinJump) => startup_events = game.debug_prepare_dolphin_jump(),
            Some(Checkpoint::PoolEntry) => startup_events = game.debug_prepare_pool_entry(),
            Some(Checkpoint::PoolRiseParticle) => {
                startup_events = game.debug_prepare_pool_zombie_rise()
            }
            Some(Checkpoint::PoolMower) => startup_events = game.debug_prepare_pool_mower(),
            Some(Checkpoint::GraveRumbleAudio) => {
                startup_events = game.debug_prepare_gravestone_rumble()
            }
            Some(Checkpoint::LadderAudio) => startup_events = game.debug_prepare_ladder_audio(),
            Some(Checkpoint::Spikeweed) => game.debug_prepare_spikeweed(),
            Some(Checkpoint::Digger) => game.debug_prepare_digger(),
            Some(Checkpoint::Magnet) => game.debug_prepare_magnet(),
            Some(Checkpoint::ShieldHit) => game.debug_prepare_shield_hit(),
            Some(Checkpoint::Zamboni) => {
                let mut setup_events = game.debug_prepare_zamboni();
                startup_anchors = Some(BoardVisualAnchors::from_state(game.state()));
                setup_events.extend(game.advance(InputFrame::default()));
                startup_events = setup_events;
            }
            Some(Checkpoint::Catapult) => startup_events = game.debug_prepare_catapult(),
            Some(Checkpoint::BalloonAppearance) => {
                startup_events = game.debug_prepare_balloon_appearance()
            }
            Some(Checkpoint::BalloonPopAudio) => {
                startup_events = game.debug_prepare_balloon_pop_audio()
            }
            Some(Checkpoint::PoleVault) => game.debug_prepare_pole_vault(),
            Some(Checkpoint::PogoBlock) => game.debug_prepare_pogo_block(),
            Some(Checkpoint::PogoBounce) => startup_events = game.debug_prepare_pogo_bounce(),
            Some(Checkpoint::UmbrellaDeflect) => game.debug_prepare_umbrella_deflect(),
            Some(Checkpoint::Store) => game.debug_prepare_store(),
            Some(Checkpoint::PauseMenu) => game.debug_prepare_pause_menu(),
            _ => {}
        }
        let almanac_strings = load_lawn_strings(&resources);
        let mut app = Self {
            renderer: None,
            assets,
            resources,
            reanim_catalog,
            particle_catalog,
            particle_images,
            // ponytail: independent RNG; share core RNG when cross-system draw order is modelled.
            particle_holder: ParticleHolder::new(0),
            projectile_particle_systems: HashMap::new(),
            zombie_seaweed_particle_systems: HashMap::new(),
            coin_arrow_particle_systems: HashMap::new(),
            pool_sparkly_particle_system: None,
            ice_sparkle_particle_systems: HashMap::new(),
            credits_fog_particle_system: None,
            audio,
            game,
            pending_input,
            last_update: None,
            simulation_accumulator: Duration::ZERO,
            cursor_position: None,
            beghouled_drag_start: None,
            tutorial_page: 0,
            seed_chooser_selection: vec![false; 2],
            seed_chooser_scroll: 0,
            garden_tool: GardenTool::WateringCan,
            store_open: matches!(checkpoint, Some(Checkpoint::Store)),
            options_open: matches!(checkpoint, Some(Checkpoint::Options)),
            help_open: matches!(checkpoint, Some(Checkpoint::Help)),
            almanac_open: matches!(checkpoint, Some(Checkpoint::Almanac)),
            credits_open: matches!(
                checkpoint,
                Some(Checkpoint::Credits | Checkpoint::CreditsParticles)
            ),
            credits_frame: if matches!(checkpoint, Some(Checkpoint::CreditsParticles)) {
                1_959
            } else {
                0
            },
            credits_paused: false,
            almanac_page: 0,
            almanac_selected_plant: 0,
            almanac_selected_zombie: 0,
            almanac_strings,
            almanac_loaded_descriptions: HashSet::new(),
            seed_bank_cost_plants: HashMap::new(),
            sun_count_asset_value: None,
            progress_meter_asset_width: None,
            options_music_volume: profile
                .as_ref()
                .map_or(100, |profile| profile.settings.music_volume_percent),
            options_effects_volume: profile
                .as_ref()
                .map_or(100, |profile| profile.settings.effects_volume_percent),
            selected_mode: ModeKind::MiniGame,
            selected_level: 0,
            debug_mode_select,
            fullscreen,
            capture_path: None,
            startup_events,
            startup_anchors,
            startup_coin_collection,
            startup_particle_warmup: match checkpoint {
                Some(Checkpoint::PlantExplosionParticles) => 30,
                Some(Checkpoint::PickupArrowParticles) => 20,
                Some(Checkpoint::PoolRiseParticle) => 10,
                Some(Checkpoint::HiddenCodeEffects) => 12,
                Some(Checkpoint::Zamboni | Checkpoint::MowerVehicleExplosion) => 3,
                _ => 0,
            },
            visual_effects: Vec::new(),
            // Normal startup runs the slide-in and fill animation; a
            // Title checkpoint captures the first rendered frame, so it
            // starts in the completed CLICK_TO_START state.
            title_load_state: if initial_scene == SceneKind::Title && checkpoint.is_none() {
                TitleLoadState::new()
            } else {
                TitleLoadState::completed()
            },
            profile,
        };
        let particle_warmup = match checkpoint {
            Some(Checkpoint::ProjectileParticleTail) => 10,
            Some(Checkpoint::ProjectileParticleFade) => 80,
            _ => 0,
        };
        if particle_warmup > 0 {
            app.update_projectile_particle_trails();
            for _ in 0..particle_warmup {
                let _ = app.game.advance(InputFrame::default());
                app.update_projectile_particle_trails();
                app.particle_holder.update(&app.particle_catalog);
            }
        }
        if matches!(checkpoint, Some(Checkpoint::ScreenFlashParticle)) {
            let _ =
                app.particle_holder
                    .spawn(&app.particle_catalog, "ScreenFlash", 400.0, 300.0, 30);
        }
        if matches!(checkpoint, Some(Checkpoint::CreditsParticles)) {
            for _ in 0..106 {
                app.advance_credits();
            }
        }
        app.apply_audio_settings();
        app.reset_seed_chooser_selection();
        app
    }

    fn initialize(&mut self, event_loop: &ActiveEventLoop) {
        if self.renderer.is_some() {
            if let Some(renderer) = &self.renderer {
                renderer.window().request_redraw();
            }
            return;
        }

        let window = match event_loop.create_window(
            Window::default_attributes()
                .with_title("neopvz")
                .with_inner_size(LogicalSize::new(800.0, 600.0)),
        ) {
            Ok(window) => Arc::new(window),
            Err(error) => {
                tracing::error!(%error, "window creation failed");
                event_loop.exit();
                return;
            }
        };
        if self.fullscreen {
            window.set_fullscreen(Some(Fullscreen::Borderless(window.current_monitor())));
        }
        let mut renderer = match pollster::block_on(GpuRenderer::new(window)) {
            Ok(renderer) => renderer,
            Err(error) => {
                tracing::error!(%error, "GPU initialization failed");
                event_loop.exit();
                return;
            }
        };
        self.ensure_almanac_description_asset(true);
        self.ensure_almanac_description_asset(false);
        self.ensure_board_hud_assets();
        self.ensure_board_progress_asset();
        for asset in self.assets.drain(..) {
            if let Err(error) = renderer.add_image(asset) {
                tracing::error!(%error, "GPU image upload failed");
                event_loop.exit();
                return;
            }
        }
        renderer.window().request_redraw();
        self.renderer = Some(renderer);
        self.update_projectile_particle_trails();
        self.update_zombie_seaweed_particles();
        self.update_pool_sparkly_particle();
        self.update_ice_sparkle_particles();
        let startup_events = std::mem::take(&mut self.startup_events);
        let startup_tick = self.game.state().tick;
        let startup_anchors = self
            .startup_anchors
            .take()
            .unwrap_or_else(|| BoardVisualAnchors::from_state(self.game.state()));
        self.record_visual_events(startup_tick, &startup_events, &startup_anchors);
        if let Some(entity) = self.startup_coin_collection.take() {
            let tick = self.game.state().tick;
            let anchors = BoardVisualAnchors::from_state(self.game.state());
            let events = self.game.advance(InputFrame {
                actions: vec![InputAction::CollectCoin { entity }],
            });
            self.record_visual_events(tick, &events, &anchors);
            self.play_audio(tick, &events);
        }
        for _ in 0..self.startup_particle_warmup {
            self.particle_holder.update(&self.particle_catalog);
        }
        self.play_audio(0, &startup_events);
        if self.credits_open {
            self.play_audio_resource(AudioKind::Music, "sounds/ZombiesOnYourLawn.ogg");
        }
    }

    fn title_start_hovered(&self) -> bool {
        if !self.title_load_state.ready() {
            return false;
        }
        let Some(position) = self.cursor_position else {
            return false;
        };
        let Some(renderer) = &self.renderer else {
            return false;
        };
        let size = renderer.window().inner_size();
        logical_position(
            size.width,
            size.height,
            position,
            LogicalViewport::default(),
        )
        .is_some_and(|(x, y)| title_start_contains(x, y, self.title_load_state.button_y()))
    }

    fn can_open_store(&self) -> bool {
        self.game.state().adventure_finished
            || self
                .profile
                .as_ref()
                .is_some_and(|profile| profile.adventure_level >= 25)
    }

    fn can_open_almanac(&self) -> bool {
        self.game.state().adventure_finished
            || self
                .profile
                .as_ref()
                .is_some_and(|profile| profile.adventure_level >= 15)
    }

    fn can_open_mode(&self, mode: ModeKind) -> bool {
        if self.debug_mode_select {
            return true;
        }
        mode_is_unlocked(
            mode,
            self.game.state().adventure_finished,
            self.profile
                .as_ref()
                .map_or(0, |profile| profile.adventure_level),
            self.game.state().unlocked_modes,
        )
    }

    fn apply_audio_settings(&mut self) {
        if let Some(audio) = &mut self.audio {
            audio.set_volume(
                AudioKind::Music,
                volume_percent_to_decibels(self.options_music_volume),
            );
            audio.set_volume(
                AudioKind::Effect,
                volume_percent_to_decibels(self.options_effects_volume),
            );
        }
    }

    fn sync_profile_settings(&mut self) {
        if let Some(profile) = &mut self.profile {
            profile.settings.music_volume_percent = self.options_music_volume;
            profile.settings.effects_volume_percent = self.options_effects_volume;
            profile.settings.fullscreen = self.fullscreen;
        }
    }

    fn close_options(&mut self) {
        self.sync_profile_settings();
        self.options_open = false;
    }

    fn set_options_volume(&mut self, kind: AudioKind, x: f32) {
        let value = (((x - 387.0) / 113.0) * 100.0).round().clamp(0.0, 100.0) as u8;
        match kind {
            AudioKind::Music => self.options_music_volume = value,
            AudioKind::Effect => self.options_effects_volume = value,
        }
        self.apply_audio_settings();
        self.sync_profile_settings();
    }

    fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: PhysicalKey) {
        if self.game.state().scene == SceneKind::Title {
            self.start_from_title();
            return;
        }
        let PhysicalKey::Code(key) = key else {
            return;
        };

        if self.almanac_open {
            match key {
                KeyCode::Escape | KeyCode::Enter => self.almanac_open = false,
                KeyCode::F11 => self.toggle_fullscreen(),
                _ => {}
            }
            return;
        }
        if self.help_open {
            match key {
                KeyCode::Escape | KeyCode::Enter | KeyCode::Space => self.help_open = false,
                KeyCode::F11 => self.toggle_fullscreen(),
                _ => {}
            }
            return;
        }
        if self.options_open {
            match key {
                KeyCode::Escape | KeyCode::Enter => self.close_options(),
                KeyCode::F11 => self.toggle_fullscreen(),
                _ => {}
            }
            return;
        }
        if self.store_open {
            match key {
                KeyCode::Escape => self.store_open = false,
                KeyCode::F11 => self.toggle_fullscreen(),
                KeyCode::Digit1 => self.purchase_store_slot(0),
                KeyCode::Digit2 => self.purchase_store_slot(1),
                KeyCode::Digit3 => self.purchase_store_slot(2),
                KeyCode::Digit4 => self.purchase_store_slot(3),
                KeyCode::Digit5 => self.purchase_store_slot(4),
                _ => {}
            }
            return;
        }
        if is_board_scene(self.game.state().scene) && self.game.state().paused {
            match key {
                KeyCode::Escape | KeyCode::Enter | KeyCode::Space => {
                    self.pending_input.push(InputAction::Resume)
                }
                KeyCode::F11 => self.toggle_fullscreen(),
                _ => {}
            }
            return;
        }
        if self.game.state().scene == SceneKind::GameOver {
            if game_lost_cutscene_active(self.game.state().game_lost_cutscene_time) {
                if key == KeyCode::F11 {
                    self.toggle_fullscreen();
                }
                return;
            }
            match key {
                KeyCode::Enter | KeyCode::Space | KeyCode::KeyR => {
                    self.pending_input.push(InputAction::Restart)
                }
                KeyCode::Escape => self.start_scene(SceneKind::AdventureSelect),
                KeyCode::F11 => self.toggle_fullscreen(),
                _ => {}
            }
            return;
        }
        if self.credits_open {
            match key {
                KeyCode::Enter | KeyCode::Space => self.credits_paused = !self.credits_paused,
                KeyCode::KeyR => self.restart_credits(),
                KeyCode::Escape => self.start_scene(SceneKind::AdventureSelect),
                KeyCode::F11 => self.toggle_fullscreen(),
                _ => {}
            }
            return;
        }
        if self.game.state().scene == SceneKind::Complete {
            match key {
                KeyCode::Enter | KeyCode::Space => self.continue_after_completion(),
                KeyCode::KeyR => self.pending_input.push(InputAction::Restart),
                KeyCode::Escape => self.start_scene(SceneKind::AdventureSelect),
                KeyCode::F11 => self.toggle_fullscreen(),
                _ => {}
            }
            return;
        }

        if is_board_scene(self.game.state().scene) || self.game.state().scene == SceneKind::Garden {
            self.pending_input.push(InputAction::TypeCode {
                character: typing_code_character(key),
            });
        }

        match key {
            KeyCode::Escape if self.game.state().scene == SceneKind::ModeSelect => {
                self.start_scene(SceneKind::AdventureSelect)
            }
            KeyCode::Escape if self.game.state().scene == SceneKind::Garden => {
                self.pending_input.push(InputAction::GardenLeave);
            }
            KeyCode::Escape => event_loop.exit(),
            KeyCode::F11 => self.toggle_fullscreen(),
            KeyCode::Enter => match self.game.state().scene {
                SceneKind::AdventureSelect => self.start_scene(SceneKind::AdventureTutorial),
                SceneKind::AdventureTutorial => self.advance_tutorial(),
                SceneKind::ModeSelect => self.start_selected_mode(),
                SceneKind::SeedChooser
                    if self.game.state().challenge.kind == ChallengeKind::LastStand =>
                {
                    self.confirm_last_stand_selection()
                }
                SceneKind::SeedChooser if self.game.state().mode == ModeKind::Survival => {
                    self.pending_input.push(InputAction::ConfirmSurvivalRepick)
                }
                SceneKind::SeedChooser => self.confirm_adventure_selection(),
                SceneKind::Garden => self.pending_input.push(InputAction::GardenLeave),
                _ if self.game.state().challenge.kind == ChallengeKind::LastStand
                    && is_board_scene(self.game.state().scene)
                    && !self.game.state().challenge.last_stand_onslaught =>
                {
                    self.pending_input.push(InputAction::StartLastStand)
                }
                _ => {}
            },
            KeyCode::ArrowLeft if self.game.state().scene == SceneKind::ModeSelect => {
                self.select_mode_level(-1)
            }
            KeyCode::ArrowRight if self.game.state().scene == SceneKind::ModeSelect => {
                self.select_mode_level(1)
            }
            KeyCode::ArrowUp if self.game.state().scene == SceneKind::ModeSelect => {
                self.select_mode_level(-5)
            }
            KeyCode::ArrowDown if self.game.state().scene == SceneKind::ModeSelect => {
                self.select_mode_level(5)
            }
            KeyCode::KeyI if self.game.state().scene == SceneKind::ModeSelect => {
                self.selected_mode = ModeKind::IZombie;
                self.selected_level = 0;
            }
            KeyCode::KeyM if self.game.state().scene == SceneKind::ModeSelect => {
                self.selected_mode = ModeKind::MiniGame;
                self.selected_level = 0;
            }
            KeyCode::KeyS if self.game.state().scene == SceneKind::ModeSelect => {
                self.selected_mode = ModeKind::Survival;
                self.selected_level = 0;
            }
            KeyCode::KeyV if self.game.state().scene == SceneKind::ModeSelect => {
                self.selected_mode = ModeKind::Vasebreaker;
                self.selected_level = 0;
            }
            KeyCode::KeyG if self.game.state().scene == SceneKind::ModeSelect => {
                self.selected_mode = ModeKind::ZenGarden;
                self.selected_level = 0;
            }
            KeyCode::Digit1 if self.game.state().scene == SceneKind::SeedChooser => {
                self.toggle_seed_choice(0);
            }
            KeyCode::Digit2 if self.game.state().scene == SceneKind::SeedChooser => {
                self.toggle_seed_choice(1);
            }
            KeyCode::Digit3 if self.game.state().scene == SceneKind::SeedChooser => {
                self.toggle_seed_choice(2);
            }
            KeyCode::Digit4 if self.game.state().scene == SceneKind::SeedChooser => {
                self.toggle_seed_choice(3);
            }
            KeyCode::Digit5 if self.game.state().scene == SceneKind::SeedChooser => {
                self.toggle_seed_choice(4);
            }
            KeyCode::Digit6 if self.game.state().scene == SceneKind::SeedChooser => {
                self.toggle_seed_choice(5);
            }
            KeyCode::Digit7 if self.game.state().scene == SceneKind::SeedChooser => {
                self.toggle_seed_choice(6);
            }
            KeyCode::Digit8 if self.game.state().scene == SceneKind::SeedChooser => {
                self.toggle_seed_choice(7);
            }
            KeyCode::Digit9 if self.game.state().scene == SceneKind::SeedChooser => {
                self.toggle_seed_choice(8);
            }
            KeyCode::KeyW if self.game.state().scene == SceneKind::Garden => {
                self.garden_tool = GardenTool::WateringCan;
            }
            KeyCode::KeyF if self.game.state().scene == SceneKind::Garden => {
                self.garden_tool = GardenTool::Fertilizer;
            }
            KeyCode::KeyB if self.game.state().scene == SceneKind::Garden => {
                self.garden_tool = GardenTool::BugSpray;
            }
            KeyCode::KeyP if self.game.state().scene == SceneKind::Garden => {
                self.garden_tool = GardenTool::Phonograph;
            }
            KeyCode::KeyH if self.game.state().scene == SceneKind::Garden => {
                self.pending_input
                    .push(InputAction::GardenFulfillNeed { plant: 0 });
            }
            KeyCode::Digit1 if is_board_scene(self.game.state().scene) => {
                self.pending_input.push(InputAction::SelectSeed { slot: 0 });
            }
            KeyCode::Digit2 if is_board_scene(self.game.state().scene) => {
                self.pending_input.push(InputAction::SelectSeed { slot: 1 });
            }
            KeyCode::Space if is_board_scene(self.game.state().scene) => {
                let action = if self.game.state().paused {
                    InputAction::Resume
                } else {
                    InputAction::Pause
                };
                self.pending_input.push(action);
            }
            KeyCode::KeyR
                if matches!(
                    self.game.state().scene,
                    SceneKind::GameOver | SceneKind::Complete
                ) =>
            {
                self.pending_input.push(InputAction::Restart);
            }
            KeyCode::KeyZ
                if self.game.state().mode == ModeKind::IZombie
                    && is_board_scene(self.game.state().scene) =>
            {
                self.pending_input.push(InputAction::DeployZombie {
                    zombie_type: neopvz_core::ZombieType::Normal,
                    row: 2,
                    column: 0,
                });
            }
            KeyCode::KeyP if is_board_scene(self.game.state().scene) => {
                self.pending_input
                    .push(InputAction::Plant { row: 2, column: 2 });
            }
            KeyCode::KeyC
                if is_board_scene(self.game.state().scene)
                    && self.game.state().board.plants.iter().any(|plant| {
                        matches!(plant.plant_type, neopvz_core::PlantType::Other(47))
                            && plant.special_armed
                    }) =>
            {
                if let Some(cob) = self.game.state().board.plants.iter().find(|plant| {
                    matches!(plant.plant_type, neopvz_core::PlantType::Other(47))
                        && plant.special_armed
                }) {
                    self.pending_input.push(InputAction::FireCobCannon {
                        entity: cob.id,
                        row: 2,
                        column: 4,
                    });
                }
            }
            KeyCode::KeyB
                if self.game.state().mode == ModeKind::Vasebreaker
                    && is_board_scene(self.game.state().scene) =>
            {
                self.pending_input
                    .push(InputAction::BreakVase { row: 2, column: 2 });
            }
            KeyCode::KeyL
                if self.game.state().challenge.kind == neopvz_core::ChallengeKind::SlotMachine
                    && is_board_scene(self.game.state().scene) =>
            {
                self.pending_input.push(InputAction::ChallengeSpin);
            }
            KeyCode::KeyC
                if self.game.state().challenge.kind == neopvz_core::ChallengeKind::Beghouled
                    && is_board_scene(self.game.state().scene) =>
            {
                self.pending_input.push(InputAction::ChallengeSwap {
                    from_column: 2,
                    from_row: 1,
                    to_column: 2,
                    to_row: 2,
                });
            }
            KeyCode::KeyT
                if self.game.state().challenge.kind
                    == neopvz_core::ChallengeKind::BeghouledTwist
                    && is_board_scene(self.game.state().scene) =>
            {
                self.pending_input
                    .push(InputAction::ChallengeTwist { column: 3, row: 0 });
            }
            KeyCode::KeyF
                if self.game.state().challenge.kind == neopvz_core::ChallengeKind::Zombiquarium
                    && is_board_scene(self.game.state().scene) =>
            {
                self.pending_input
                    .push(InputAction::ChallengeFeed { x: 400, y: 250 });
            }
            KeyCode::KeyH
                if self.game.state().challenge.kind == neopvz_core::ChallengeKind::WhackAZombie
                    && is_board_scene(self.game.state().scene) =>
            {
                self.pending_input.push(InputAction::ChallengeWhack {
                    row: 2,
                    column: 2,
                    x: 200,
                    y: 300,
                });
            }
            _ => {}
        }
    }

    fn toggle_fullscreen(&mut self) {
        let Some(renderer) = &self.renderer else {
            return;
        };
        let window = renderer.window();
        self.fullscreen = !self.fullscreen;
        window.set_fullscreen(if self.fullscreen {
            Some(Fullscreen::Borderless(window.current_monitor()))
        } else {
            None
        });
    }

    fn purchase_store_slot(&mut self, slot: usize) {
        if let Some(item) = store_item_at_slot(slot) {
            self.pending_input.push(InputAction::StorePurchase { item });
        }
    }

    fn start_from_title(&mut self) {
        if self.game.state().scene != SceneKind::Title || !self.title_load_state.ready() {
            return;
        }
        self.play_audio_resource(AudioKind::Effect, TITLE_START_SOUND_PATH);
        self.start_scene(SceneKind::AdventureSelect);
    }

    fn start_scene(&mut self, scene: SceneKind) {
        let mut game = if self.game.state().mode == ModeKind::Adventure
            && matches!(scene, SceneKind::Day | SceneKind::SeedChooser)
        {
            new_adventure_game(&self.game, self.profile.as_ref())
        } else {
            new_scene_game(scene)
        };
        carry_profile(&self.game, &mut game, self.profile.as_mut());
        self.game = game;
        self.clear_transient_effects();
        self.reset_board_hud_assets();
        self.store_open = false;
        self.options_open = false;
        self.help_open = false;
        self.almanac_open = false;
        self.credits_open = false;
        self.credits_frame = 0;
        self.credits_paused = false;
        self.almanac_page = 0;
        self.almanac_selected_plant = 0;
        self.almanac_selected_zombie = 0;
        self.tutorial_page = 0;
        self.pending_input.clear();
        self.simulation_accumulator = Duration::ZERO;
        self.last_update = Some(Instant::now());
        self.reset_seed_chooser_selection();
        self.selected_mode = ModeKind::MiniGame;
        self.selected_level = 0;
    }

    fn continue_after_completion(&mut self) {
        if self.game.state().mode == ModeKind::Adventure
            && self.game.state().level == 50
            && self.game.state().adventure_first_time
        {
            self.clear_transient_effects();
            self.credits_open = true;
            self.credits_frame = 0;
            self.credits_paused = false;
            self.pending_input.clear();
            self.simulation_accumulator = Duration::ZERO;
            self.last_update = Some(Instant::now());
            self.play_audio_resource(AudioKind::Music, "sounds/ZombiesOnYourLawn.ogg");
            return;
        }
        if self.game.state().mode != ModeKind::Adventure || self.game.state().level >= 50 {
            self.start_scene(SceneKind::AdventureSelect);
            return;
        }

        let next_level = self.game.state().level.saturating_add(1);
        let (first_time, packet_upgrades, stinky_purchased) =
            if let Some(profile) = self.profile.as_mut() {
                self.game.update_profile(profile);
                (
                    profile.adventure_rounds == 0,
                    profile.packet_upgrades,
                    profile.stinky_purchased,
                )
            } else {
                (true, 0, false)
            };
        let mut game =
            Game::new_adventure(0, next_level, first_time, packet_upgrades, stinky_purchased);
        game.carry_session_state_from(&self.game);
        if let Some(profile) = self.profile.as_ref() {
            game.apply_profile(profile);
        }
        self.game = game;
        self.clear_transient_effects();
        self.reset_board_hud_assets();
        self.tutorial_page = 0;
        self.pending_input.clear();
        self.simulation_accumulator = Duration::ZERO;
        self.last_update = Some(Instant::now());
        self.reset_seed_chooser_selection();
        self.selected_mode = ModeKind::MiniGame;
        self.selected_level = 0;
    }

    fn restart_credits(&mut self) {
        self.clear_transient_effects();
        self.credits_open = true;
        self.credits_frame = 0;
        self.credits_paused = false;
        self.pending_input.clear();
        self.simulation_accumulator = Duration::ZERO;
        self.last_update = Some(Instant::now());
        self.play_audio_resource(AudioKind::Music, "sounds/ZombiesOnYourLawn.ogg");
    }

    fn start_mode_select(&mut self, mode: ModeKind) {
        if !self.can_open_mode(mode) {
            return;
        }
        let mut game = Game::new(0, SceneKind::ModeSelect);
        carry_profile(&self.game, &mut game, self.profile.as_mut());
        self.game = game;
        self.clear_transient_effects();
        self.pending_input.clear();
        self.simulation_accumulator = Duration::ZERO;
        self.last_update = Some(Instant::now());
        self.selected_mode = mode;
        self.selected_level = 0;
    }

    fn start_selected_mode(&mut self) {
        if !self.can_open_mode(self.selected_mode)
            || mode_level_name(self.selected_mode, self.selected_level).is_none()
        {
            return;
        }
        let mut game = Game::new_mode(0, self.selected_mode, self.selected_level);
        carry_profile(&self.game, &mut game, self.profile.as_mut());
        self.game = game;
        self.clear_transient_effects();
        self.reset_board_hud_assets();
        self.pending_input.clear();
        self.simulation_accumulator = Duration::ZERO;
        self.last_update = Some(Instant::now());
        self.reset_seed_chooser_selection();
    }

    fn select_mode_level(&mut self, delta: i32) {
        let count = mode_level_names(self.selected_mode).len();
        let max = i32::try_from(count.saturating_sub(1)).unwrap_or(i32::MAX);
        self.selected_level = (i32::from(self.selected_level) + delta).clamp(0, max) as u8;
    }

    fn advance_tutorial(&mut self) {
        if self.tutorial_page == 0 {
            self.tutorial_page = 1;
        } else {
            self.start_scene(SceneKind::Day);
        }
    }

    fn toggle_seed_choice(&mut self, index: usize) {
        if let Some(selected) = self.seed_chooser_selection.get_mut(index) {
            *selected = !*selected;
        }
    }

    fn seed_chooser_has_seven_rows(&self) -> bool {
        let state = self.game.state();
        seed_chooser_has_seven_rows_at(
            state.adventure_finished || state.challenge.kind == ChallengeKind::LastStand,
            &state.unlocked_plants,
        )
    }

    fn reset_seed_chooser_selection(&mut self) {
        let count = if self.game.state().scene == SceneKind::SeedChooser {
            if self.game.state().challenge.kind == ChallengeKind::LastStand {
                last_stand_seed_choices().len()
            } else {
                adventure_seed_choices(
                    self.game.state().level,
                    self.game.state().adventure_first_time,
                )
                .len()
            }
        } else {
            2
        };
        self.seed_chooser_selection = vec![false; count];
        self.seed_chooser_scroll = 0;
    }

    fn clear_transient_effects(&mut self) {
        self.visual_effects.clear();
        self.particle_holder.clear();
        self.projectile_particle_systems.clear();
        self.zombie_seaweed_particle_systems.clear();
        self.coin_arrow_particle_systems.clear();
        self.pool_sparkly_particle_system = None;
        self.ice_sparkle_particle_systems.clear();
        self.credits_fog_particle_system = None;
    }

    fn confirm_adventure_selection(&mut self) {
        if self.game.state().mode != ModeKind::Adventure {
            return;
        }
        let choices = adventure_seed_choices(
            self.game.state().level,
            self.game.state().adventure_first_time,
        );
        let selected = choices
            .iter()
            .zip(&self.seed_chooser_selection)
            .filter_map(|(plant, selected)| (*selected).then_some(*plant))
            .collect::<Vec<_>>();
        if selected.len()
            == usize::from(adventure_seed_slots(
                self.game.state().level,
                self.game.state().adventure_first_time,
                self.game.state().packet_upgrades,
            ))
        {
            self.pending_input
                .push(InputAction::ConfirmAdventureSeeds { seeds: selected });
        }
    }

    fn confirm_last_stand_selection(&mut self) {
        if self.game.state().challenge.kind != ChallengeKind::LastStand {
            return;
        }
        let choices = last_stand_seed_choices();
        let selected = choices
            .iter()
            .zip(&self.seed_chooser_selection)
            .filter_map(|(plant, selected)| (*selected).then_some(*plant))
            .collect::<Vec<_>>();
        if selected.len() == usize::from(last_stand_seed_slots(self.game.state().packet_upgrades)) {
            self.pending_input
                .push(InputAction::ConfirmLastStandSeeds { seeds: selected });
        }
    }

    fn handle_beghouled_mouse_release(&mut self, button: MouseButton) {
        let Some((from_column, from_row)) = self.beghouled_drag_start.take() else {
            return;
        };
        if button != MouseButton::Left
            || self.game.state().challenge.kind != ChallengeKind::Beghouled
        {
            return;
        }
        let Some(position) = self.cursor_position else {
            return;
        };
        let Some((x, y)) = self.renderer.as_ref().and_then(|renderer| {
            let size = renderer.window().inner_size();
            logical_position(
                size.width,
                size.height,
                position,
                LogicalViewport::default(),
            )
        }) else {
            return;
        };
        if !(80.0..800.0).contains(&x) || !(120.0..570.0).contains(&y) {
            return;
        }
        let to_column = ((x - 80.0) / 80.0) as u8;
        let to_row = ((y - 120.0) / 90.0) as u8;
        if to_column >= 8 || to_row >= 5 {
            return;
        }
        let delta_column = i16::from(to_column) - i16::from(from_column);
        let delta_row = i16::from(to_row) - i16::from(from_row);
        let target = if delta_column.abs() > delta_row.abs() {
            if delta_column > 0 {
                from_column.checked_add(1).map(|column| (column, from_row))
            } else {
                from_column.checked_sub(1).map(|column| (column, from_row))
            }
        } else if delta_row > 0 {
            from_row.checked_add(1).map(|row| (from_column, row))
        } else if delta_row < 0 {
            from_row.checked_sub(1).map(|row| (from_column, row))
        } else {
            None
        };
        let Some((to_column, to_row)) = target else {
            return;
        };
        self.pending_input.push(InputAction::ChallengeSwap {
            from_column,
            from_row,
            to_column,
            to_row,
        });
    }

    fn handle_mouse_click(&mut self, button: MouseButton) {
        let scene = self.game.state().scene;
        if scene != SceneKind::Title
            && scene != SceneKind::AdventureSelect
            && scene != SceneKind::AdventureTutorial
            && scene != SceneKind::SeedChooser
            && scene != SceneKind::ModeSelect
            && scene != SceneKind::Garden
            && scene != SceneKind::Complete
            && scene != SceneKind::GameOver
            && !is_board_scene(scene)
        {
            return;
        }
        let Some(position) = self.cursor_position else {
            return;
        };
        let Some((x, y)) = self.renderer.as_ref().and_then(|renderer| {
            let size = renderer.window().inner_size();
            logical_position(
                size.width,
                size.height,
                position,
                LogicalViewport::default(),
            )
        }) else {
            return;
        };
        if scene == SceneKind::Title {
            let over_start = title_start_contains(x, y, self.title_load_state.button_y());
            if self.title_load_state.mouse_press(button, over_start) {
                self.play_audio_resource(AudioKind::Effect, TITLE_START_SOUND_PATH);
                if !over_start {
                    self.start_scene(SceneKind::AdventureSelect);
                }
            }
            return;
        }
        if self.almanac_open {
            if button == MouseButton::Left {
                if almanac_close_contains(x, y) {
                    self.almanac_open = false;
                } else if almanac_index_contains(x, y) {
                    self.almanac_page = 0;
                } else if self.almanac_page == 0 && almanac_plant_contains(x, y) {
                    self.almanac_page = 1;
                    self.almanac_selected_plant = 0;
                } else if self.almanac_page == 0 && almanac_zombie_contains(x, y) {
                    self.almanac_page = 2;
                    self.almanac_selected_zombie = 0;
                } else if self.almanac_page == 1
                    && let Some(slot) = almanac_plant_slot_at(x, y)
                    && let Some(plant_type) = almanac_plant_type(slot)
                    && self.almanac_plant_available(plant_type)
                {
                    self.almanac_selected_plant = slot;
                    self.ensure_almanac_description_asset(true);
                } else if self.almanac_page == 2
                    && let Some(index) = almanac_zombie_index_at(x, y)
                    && let Some(zombie_type) = almanac_zombie_type(index)
                    && self.almanac_zombie_is_shown(zombie_type)
                {
                    self.almanac_selected_zombie = index;
                    self.ensure_almanac_description_asset(false);
                }
            }
            return;
        }
        if self.help_open {
            if button == MouseButton::Left && help_button_contains(x, y) {
                self.help_open = false;
            }
            return;
        }
        if self.options_open {
            if button == MouseButton::Left
                && (215.0..585.0).contains(&x)
                && (420.0..545.0).contains(&y)
            {
                self.close_options();
            } else if button == MouseButton::Left && (375.0..540.0).contains(&x) {
                if (155.0..215.0).contains(&y) {
                    self.set_options_volume(AudioKind::Music, x);
                } else if (205.0..270.0).contains(&y) {
                    self.set_options_volume(AudioKind::Effect, x);
                }
            } else if button == MouseButton::Left
                && (450.0..535.0).contains(&x)
                && (235.0..315.0).contains(&y)
            {
                self.toggle_fullscreen();
                self.sync_profile_settings();
            }
            return;
        }
        if self.store_open {
            if button == MouseButton::Left
                && (315.0..500.0).contains(&x)
                && (525.0..595.0).contains(&y)
            {
                self.store_open = false;
            } else if button == MouseButton::Left
                && let Some(item) = store_item_at(x, y)
            {
                self.pending_input.push(InputAction::StorePurchase { item });
            }
            return;
        }
        if is_board_scene(scene) && self.game.state().paused {
            if button == MouseButton::Left {
                self.pending_input.push(InputAction::Resume);
            }
            return;
        }
        if scene == SceneKind::GameOver {
            if game_lost_cutscene_active(self.game.state().game_lost_cutscene_time) {
                return;
            }
            if button == MouseButton::Left {
                if game_over_main_menu_contains(x, y) {
                    self.start_scene(SceneKind::AdventureSelect);
                } else if game_over_try_again_contains(x, y) {
                    self.pending_input.push(InputAction::Restart);
                }
            }
            return;
        }
        if self.credits_open {
            if button == MouseButton::Left && credits_controls_visible(self.credits_frame) {
                if credits_replay_contains(x, y) {
                    self.restart_credits();
                } else if credits_main_menu_contains(x, y) {
                    self.start_scene(SceneKind::AdventureSelect);
                }
            }
            return;
        }
        if scene == SceneKind::AdventureSelect {
            if (400.0..730.0).contains(&x) && (55.0..175.0).contains(&y) {
                self.start_scene(SceneKind::AdventureTutorial);
            } else if (400.0..730.0).contains(&x) && (173.0..257.0).contains(&y) {
                self.start_mode_select(ModeKind::Survival);
            } else if (400.0..730.0).contains(&x) && (257.0..328.0).contains(&y) {
                self.start_mode_select(ModeKind::MiniGame);
            } else if (400.0..730.0).contains(&x) && (328.0..410.0).contains(&y) {
                self.start_mode_select(ModeKind::Vasebreaker);
            } else if (150.0..330.0).contains(&x) && (385.0..485.0).contains(&y) {
                self.start_mode_select(ModeKind::ZenGarden);
            } else if button == MouseButton::Left
                && (390.0..540.0).contains(&x)
                && (450.0..570.0).contains(&y)
                && self.can_open_store()
            {
                self.store_open = true;
            } else if button == MouseButton::Left
                && (320.0..390.0).contains(&x)
                && (420.0..530.0).contains(&y)
                && self.can_open_almanac()
            {
                self.almanac_open = true;
                self.almanac_page = 0;
            } else if button == MouseButton::Left
                && (535.0..685.0).contains(&x)
                && (445.0..570.0).contains(&y)
            {
                self.options_open = true;
            } else if button == MouseButton::Left && help_selector_contains(x, y) {
                self.help_open = true;
            }
            return;
        }
        if scene == SceneKind::AdventureTutorial {
            if (285.0..565.0).contains(&x) && (20.0..190.0).contains(&y) {
                self.advance_tutorial();
            }
            return;
        }
        if scene == SceneKind::ModeSelect {
            if (0.0..200.0).contains(&x) && (0.0..50.0).contains(&y) {
                self.selected_mode = ModeKind::MiniGame;
                self.selected_level = 0;
                return;
            }
            if (200.0..400.0).contains(&x) && (0.0..50.0).contains(&y) {
                self.selected_mode = ModeKind::IZombie;
                self.selected_level = 0;
                return;
            }
            if (400.0..600.0).contains(&x) && (0.0..50.0).contains(&y) {
                self.selected_mode = ModeKind::Survival;
                self.selected_level = 0;
                return;
            }
            if (600.0..800.0).contains(&x) && (0.0..50.0).contains(&y) {
                self.selected_mode = ModeKind::Vasebreaker;
                self.selected_level = 0;
                return;
            }
            let Some(index) = mode_level_at(self.selected_mode, x, y) else {
                return;
            };
            self.selected_level = index;
            self.start_selected_mode();
            return;
        }
        if scene == SceneKind::SeedChooser {
            if let Some(index) = seed_chooser_slot_at(
                x,
                y,
                self.seed_chooser_scroll,
                self.seed_chooser_has_seven_rows(),
            ) {
                self.toggle_seed_choice(index);
                return;
            }
            if (322.0..478.0).contains(&x) && (535.0..577.0).contains(&y) {
                if self.game.state().challenge.kind == ChallengeKind::LastStand {
                    self.confirm_last_stand_selection();
                } else {
                    self.confirm_adventure_selection();
                }
            }
            return;
        }
        if scene == SceneKind::Complete {
            if button == MouseButton::Left {
                if complete_main_menu_contains(x, y) {
                    self.start_scene(SceneKind::AdventureSelect);
                } else if complete_continue_contains(x, y) {
                    self.continue_after_completion();
                }
            }
            return;
        }
        if scene == SceneKind::Garden {
            if button == MouseButton::Left && (510.0..590.0).contains(&y) {
                self.garden_tool = if (80.0..160.0).contains(&x) {
                    GardenTool::WateringCan
                } else if (240.0..320.0).contains(&x) {
                    GardenTool::Fertilizer
                } else if (400.0..480.0).contains(&x) {
                    GardenTool::BugSpray
                } else if (560.0..640.0).contains(&x) {
                    GardenTool::Phonograph
                } else {
                    self.garden_tool
                };
                return;
            }
            let plant_hit =
                self.game
                    .state()
                    .garden
                    .plants
                    .iter()
                    .enumerate()
                    .find_map(|(index, _)| {
                        let center_x = 250.0 + index as f32 * 120.0;
                        ((x - center_x).abs() <= 40.0 && (y - 300.0).abs() <= 40.0)
                            .then(|| u8::try_from(index).ok())
                            .flatten()
                    });
            if button == MouseButton::Left
                && self.game.state().garden_service == Some(GardenServiceKind::Aquarium)
                && plant_hit.is_none()
            {
                self.pending_input.push(InputAction::GardenTapGlass);
            } else if button == MouseButton::Left
                && self.game.state().garden_service == Some(GardenServiceKind::TreeOfWisdom)
            {
                self.pending_input.push(InputAction::GardenFeedTree);
            } else if let Some(plant) = plant_hit {
                self.pending_input.push(InputAction::GardenUseTool {
                    plant,
                    tool: if button == MouseButton::Right {
                        GardenTool::Fertilizer
                    } else {
                        self.garden_tool
                    },
                });
            }
            return;
        }
        if self.game.state().challenge.kind == ChallengeKind::LastStand
            && !self.game.state().challenge.last_stand_onslaught
            && button == MouseButton::Left
            && (270.0..530.0).contains(&x)
            && (550.0..620.0).contains(&y)
        {
            self.pending_input.push(InputAction::StartLastStand);
            return;
        }
        if self.game.state().challenge.kind == ChallengeKind::SlotMachine
            && button == MouseButton::Left
            && (473.0..528.0).contains(&x)
            && (0.0..80.0).contains(&y)
        {
            self.pending_input.push(InputAction::ChallengeSpin);
            return;
        }
        if button == MouseButton::Left {
            let hit = |position_x: i64, position_y: i64| {
                (fixed_point_to_logical(position_x) - x).abs() <= 35.0
                    && (fixed_point_to_logical(position_y) - y).abs() <= 35.0
            };
            if let Some(sun) = self
                .game
                .state()
                .board
                .suns
                .iter()
                .find(|sun| hit(sun.position_x, sun.position_y))
            {
                self.pending_input
                    .push(InputAction::CollectSun { entity: sun.id });
                return;
            }
            if let Some(coin) = self
                .game
                .state()
                .board
                .coins
                .iter()
                .find(|coin| hit(coin.position_x, coin.position_y))
            {
                self.pending_input
                    .push(InputAction::CollectCoin { entity: coin.id });
                return;
            }
        }
        if button == MouseButton::Left
            && is_board_scene(scene)
            && let Some(slot) = board_seed_packet_at(self.game.state(), x, y)
        {
            self.pending_input.push(InputAction::SelectSeed { slot });
            return;
        }
        if self.game.state().challenge.kind == ChallengeKind::Zombiquarium
            && button == MouseButton::Left
        {
            self.pending_input.push(InputAction::ChallengeFeed {
                x: x as u16,
                y: y as u16,
            });
            return;
        }
        if !(80.0..800.0).contains(&x) || !(120.0..570.0).contains(&y) {
            return;
        }
        let row = ((y - 120.0) / 90.0) as u8;
        let column = ((x - 80.0) / 80.0) as u8;
        if button == MouseButton::Left
            && self.game.state().board.selected_seed.is_none()
            && let Some(cob) = self.game.state().board.plants.iter().find(|plant| {
                matches!(plant.plant_type, neopvz_core::PlantType::Other(47)) && plant.special_armed
            })
        {
            self.pending_input.push(InputAction::FireCobCannon {
                entity: cob.id,
                row,
                column,
            });
            return;
        }
        if self.game.state().mode == ModeKind::IZombie {
            if button == MouseButton::Left {
                self.pending_input.push(InputAction::DeployZombie {
                    zombie_type: neopvz_core::ZombieType::Normal,
                    row,
                    column,
                });
            }
            return;
        }
        if self.game.state().mode == ModeKind::Vasebreaker {
            if button == MouseButton::Left {
                self.pending_input
                    .push(InputAction::BreakVase { row, column });
            }
            return;
        }
        if self.game.state().challenge.kind == ChallengeKind::Beghouled {
            if button == MouseButton::Left && column < 8 && row < 5 {
                self.beghouled_drag_start = Some((column, row));
            } else if button == MouseButton::Right {
                self.pending_input.push(InputAction::ChallengeClearCrater);
            }
            return;
        }
        if self.game.state().challenge.kind == ChallengeKind::BeghouledTwist {
            self.pending_input.push(match button {
                MouseButton::Left => InputAction::ChallengeTwist { column, row },
                MouseButton::Right => InputAction::ChallengeClearCrater,
                _ => return,
            });
            return;
        }
        match self.game.state().challenge.kind {
            neopvz_core::ChallengeKind::WhackAZombie if button == MouseButton::Left => {
                self.pending_input.push(InputAction::ChallengeWhack {
                    row,
                    column,
                    x: x as u16,
                    y: y as u16,
                });
                return;
            }
            _ => {}
        }
        let action = match button {
            MouseButton::Left => InputAction::Plant { row, column },
            MouseButton::Right => InputAction::Shovel { row, column },
            _ => return,
        };
        self.pending_input.push(action);
    }

    fn advance_simulation(&mut self) {
        let Some(last_update) = self.last_update else {
            self.last_update = Some(Instant::now());
            return;
        };
        let now = Instant::now();
        self.last_update = Some(now);
        self.simulation_accumulator += now
            .saturating_duration_since(last_update)
            .min(Duration::from_millis(250));

        while self.simulation_accumulator >= SIMULATION_STEP {
            if self.credits_open {
                if !self.credits_paused {
                    self.advance_credits();
                }
                self.pending_input.clear();
                self.simulation_accumulator -= SIMULATION_STEP;
                continue;
            }
            if self.game.state().scene == SceneKind::Title {
                let was_complete = self.title_load_state.label_complete;
                let fired = self.title_load_state.advance();
                if self.title_load_state.tick == 1
                    || (!was_complete && self.title_load_state.label_complete)
                {
                    tracing::info!(
                        title_tick = self.title_load_state.tick,
                        ready = self.title_load_state.ready(),
                        complete = self.title_load_state.label_complete,
                        "title loading state changed"
                    );
                }
                for index in fired {
                    tracing::info!(
                        title_tick = self.title_load_state.tick,
                        trigger = index,
                        "title loading trigger"
                    );
                    self.play_audio_resource(AudioKind::Effect, "sounds/loadingbar_flower.ogg");
                    if index == 4 {
                        self.play_audio_resource(AudioKind::Effect, "sounds/loadingbar_zombie.ogg");
                    }
                }
            }
            let input = InputFrame {
                actions: std::mem::take(&mut self.pending_input),
            };
            let tick = self.game.state().tick;
            let visual_anchors = BoardVisualAnchors::from_state(self.game.state());
            let events = self.game.advance(input);
            self.ensure_board_hud_assets();
            self.ensure_board_progress_asset();
            if self.game.state().tick != tick {
                self.update_projectile_particle_trails();
                self.update_zombie_seaweed_particles();
                self.update_pool_sparkly_particle();
                self.update_ice_sparkle_particles();
                self.particle_holder.update(&self.particle_catalog);
            }
            self.record_visual_events(tick, &events, &visual_anchors);
            self.play_audio(tick, &events);
            self.simulation_accumulator -= SIMULATION_STEP;
        }
    }

    fn advance_credits(&mut self) {
        let previous_frame = self.credits_frame;
        self.credits_frame = self.credits_frame.saturating_add(1);
        let (_, phase_frame) = credits_phase_at(self.credits_frame);
        for cue in credits_particle_cues(previous_frame, self.credits_frame) {
            match cue {
                CreditsParticleCue::Spawn(name) => {
                    if self
                        .particle_holder
                        .spawn(&self.particle_catalog, name, 400.0, 300.0, 30)
                        .is_some()
                    {
                        tracing::info!(name, "credits particle system queued");
                    }
                }
                CreditsParticleCue::StartFog(name) => {
                    let Some(x) =
                        credits_fog_x(self.reanim_catalog.credits_main2.as_ref(), phase_frame)
                    else {
                        continue;
                    };
                    self.credits_fog_particle_system =
                        self.particle_holder
                            .spawn(&self.particle_catalog, name, x, 230.0, 30);
                    if self.credits_fog_particle_system.is_some() {
                        tracing::info!(x, "credits fog particle system queued");
                    }
                }
                CreditsParticleCue::StopFog => {
                    if let Some(system) = self.credits_fog_particle_system.take() {
                        self.particle_holder.die_system(system);
                    }
                }
            }
        }
        if let Some(system) = self.credits_fog_particle_system
            && let Some(x) = credits_fog_x(self.reanim_catalog.credits_main2.as_ref(), phase_frame)
        {
            self.particle_holder
                .move_system(&self.particle_catalog, system, x, 230.0);
        }
        self.particle_holder.update(&self.particle_catalog);
    }

    fn update_projectile_particle_trails(&mut self) {
        let trails = self
            .game
            .state()
            .board
            .projectiles
            .iter()
            .filter_map(|projectile| {
                let (name, offset_x, offset_y) =
                    projectile_particle_trail(projectile.projectile_type)?;
                Some((
                    projectile.id,
                    projectile.projectile_type,
                    name,
                    fixed_point_to_logical(projectile.position_x) + offset_x,
                    board_projectile_y(
                        projectile.position_y,
                        projectile.row,
                        projectile.lob_height,
                    ) + offset_y,
                ))
            })
            .collect::<Vec<_>>();
        let live_types = trails
            .iter()
            .map(|(entity, projectile_type, ..)| (*entity, *projectile_type))
            .collect::<HashMap<_, _>>();
        let particle_holder = &mut self.particle_holder;
        let particle_catalog = &self.particle_catalog;
        self.projectile_particle_systems
            .retain(|entity, (system, projectile_type)| {
                let keep = live_types
                    .get(entity)
                    .is_some_and(|live_type| *live_type == *projectile_type);
                if !keep {
                    particle_holder.cross_fade_system(particle_catalog, *system, "FadeOut");
                }
                keep
            });

        for (entity, projectile_type, name, x, y) in trails {
            if let Some((system, _)) = self.projectile_particle_systems.get(&entity) {
                self.particle_holder
                    .move_system(&self.particle_catalog, *system, x, y);
            } else if let Some(system) =
                self.particle_holder
                    .spawn(&self.particle_catalog, name, x, y, 11)
            {
                self.projectile_particle_systems
                    .insert(entity, (system, projectile_type));
            }
        }
    }

    fn update_zombie_seaweed_particles(&mut self) {
        let tick = self.game.state().tick;
        let positions = self
            .game
            .state()
            .board
            .zombies
            .iter()
            .filter_map(|zombie| {
                zombie_seaweed_attachment_positions(&self.reanim_catalog, zombie, tick)
                    .map(|positions| (zombie.id, positions))
            })
            .collect::<HashMap<_, _>>();
        let particle_holder = &mut self.particle_holder;
        let particle_catalog = &self.particle_catalog;
        self.zombie_seaweed_particle_systems
            .retain(|entity, systems| {
                let Some(positions) = positions.get(entity) else {
                    for (_, system) in systems.iter() {
                        particle_holder.die_system(*system);
                    }
                    return false;
                };
                for (index, system) in systems.iter() {
                    let (x, y) = positions[*index];
                    particle_holder.move_system(particle_catalog, *system, x, y);
                }
                true
            });
    }

    fn update_pool_sparkly_particle(&mut self) {
        if self.game.state().scene == SceneKind::Pool {
            if self.pool_sparkly_particle_system.is_none() {
                let system = self.particle_holder.spawn(
                    &self.particle_catalog,
                    "PoolSparkly",
                    450.0,
                    295.0,
                    2,
                );
                if system.is_some() {
                    tracing::info!(x = 450.0, y = 295.0, "pool sparkly particle queued");
                }
                self.pool_sparkly_particle_system = system;
            }
        } else if let Some(system) = self.pool_sparkly_particle_system.take() {
            self.particle_holder
                .cross_fade_system(&self.particle_catalog, system, "FadeOut");
        }
    }

    fn update_ice_sparkle_particles(&mut self) {
        let board = &self.game.state().board;
        let rows = board
            .ice_timer
            .iter()
            .enumerate()
            .filter(|(_, timer)| **timer > 0)
            .map(|(index, _)| {
                let row = u8::try_from(index).expect("ice row fits in u8");
                let x = board
                    .ice_min_x
                    .get(index)
                    .copied()
                    .map(fixed_point_to_logical)
                    .unwrap_or(800.0);
                (row, x, board_row_y(row))
            })
            .collect::<Vec<_>>();
        let live_rows = rows.iter().map(|(row, ..)| *row).collect::<HashSet<_>>();
        let particle_holder = &mut self.particle_holder;
        self.ice_sparkle_particle_systems.retain(|row, system| {
            let keep = live_rows.contains(row);
            if !keep {
                particle_holder.die_system(*system);
            }
            keep
        });
        for (row, x, y) in rows {
            if let Some(system) = self.ice_sparkle_particle_systems.get(&row) {
                self.particle_holder
                    .move_system(&self.particle_catalog, *system, x, y);
            } else if let Some(system) =
                self.particle_holder
                    .spawn(&self.particle_catalog, "IceSparkle", x, y, 3)
            {
                self.ice_sparkle_particle_systems.insert(row, system);
                tracing::info!(row, x, y, "ice sparkle particle queued");
            }
        }
    }

    fn record_visual_events(
        &mut self,
        tick: u64,
        events: &[GameEvent],
        anchors: &BoardVisualAnchors,
    ) {
        let current_anchors = BoardVisualAnchors::from_state(self.game.state());
        let scene = self.game.state().scene;
        for event in events {
            for (name, x, y, z) in
                particle_effects_for_event_batch(event, events, scene, anchors, &current_anchors)
            {
                if let Some(system) =
                    self.particle_holder
                        .spawn(&self.particle_catalog, name, x, y, z)
                {
                    match event {
                        GameEvent::ZombieMustacheDropped { variant, .. } => self
                            .particle_holder
                            .override_system_image(system, mustache_image_symbol(*variant)),
                        GameEvent::ZombieFutureGlassesDropped { frame, .. } => self
                            .particle_holder
                            .override_system_frame(system, i32::from(*frame)),
                        _ => {}
                    }
                    tracing::info!(tick, name, x, y, "particle system queued");
                }
            }
            match event {
                GameEvent::ZombiePoolRise { entity } => {
                    let Some(zombie) = self
                        .game
                        .state()
                        .board
                        .zombies
                        .iter()
                        .find(|zombie| zombie.id == *entity)
                    else {
                        continue;
                    };
                    let Some(positions) = zombie_seaweed_attachment_positions(
                        &self.reanim_catalog,
                        zombie,
                        self.game.state().tick,
                    ) else {
                        continue;
                    };
                    let systems = positions
                        .into_iter()
                        .enumerate()
                        .filter_map(|(index, (x, y))| {
                            self.particle_holder
                                .spawn(&self.particle_catalog, "Zombie_seaweed", x, y, 7)
                                .map(|system| (index, system))
                        })
                        .collect::<Vec<_>>();
                    tracing::info!(
                        tick,
                        entity,
                        systems = systems.len(),
                        "pool-rise seaweed particle attachments queued"
                    );
                    self.zombie_seaweed_particle_systems
                        .insert(*entity, systems);
                }
                GameEvent::CoinLanded { entity, .. } => {
                    let Some(coin) = self
                        .game
                        .state()
                        .board
                        .coins
                        .iter()
                        .find(|coin| coin.id == *entity)
                    else {
                        continue;
                    };
                    let Some((name, offset_x, offset_y)) =
                        coin_arrow_particle(coin.coin_type, coin.needs_bouncy_arrow)
                    else {
                        continue;
                    };
                    let x = fixed_point_to_logical(coin.position_x) + offset_x;
                    let y = fixed_point_to_logical(coin.position_y) + offset_y;
                    if let Some(system) =
                        self.particle_holder
                            .spawn(&self.particle_catalog, name, x, y, 13)
                    {
                        self.coin_arrow_particle_systems.insert(*entity, system);
                        tracing::info!(tick, name, x, y, "pickup arrow particle queued");
                    }
                }
                GameEvent::PickupCollected { entity, .. } => {
                    if let Some(system) = self.coin_arrow_particle_systems.remove(entity) {
                        self.particle_holder.die_system(system);
                        tracing::info!(tick, entity, "pickup arrow particle removed");
                    }
                }
                GameEvent::PlantSpecialTriggered {
                    entity,
                    plant_type: PlantType::Other(20),
                } => {
                    let Some(anchor) = anchors
                        .plants
                        .get(entity)
                        .or_else(|| current_anchors.plants.get(entity))
                    else {
                        continue;
                    };
                    for (x, y) in jalapeno_fire_effect_positions(scene, anchor.row) {
                        self.visual_effects.push(BoardFireEffect {
                            x,
                            y,
                            start_tick: tick,
                            duration: JALAPENO_FIRE_DURATION,
                            z: 11,
                        });
                    }
                    tracing::info!(
                        tick,
                        row = anchor.row,
                        effects = 12,
                        "jalapeno fire reanimations queued"
                    );
                }
                GameEvent::ProjectileImpact {
                    projectile,
                    kind: ProjectileImpactSound::Ignite,
                    ..
                } => {
                    let Some(anchor) = anchors
                        .projectiles
                        .get(projectile)
                        .or_else(|| current_anchors.projectiles.get(projectile))
                    else {
                        continue;
                    };
                    self.visual_effects.push(BoardFireEffect {
                        x: anchor.x + 38.0,
                        y: anchor.y - 20.0,
                        start_tick: tick,
                        duration: PROJECTILE_FIRE_DURATION,
                        z: 13,
                    });
                    tracing::info!(tick, projectile, "projectile fire reanimation queued");
                }
                _ => {}
            }
        }
        let current_tick = self.game.state().tick;
        self.visual_effects
            .retain(|effect| current_tick.saturating_sub(effect.start_tick) < effect.duration);
    }

    fn play_audio(&mut self, tick: u64, events: &[GameEvent]) {
        let state = self.game.state();
        for event in events {
            for (kind, path) in audio_sequence_for_event(event, state).into_iter().flatten() {
                let Some(bytes) = self.resources.read(path).ok() else {
                    tracing::debug!(path, "audio resource is unavailable");
                    continue;
                };
                tracing::info!(tick, ?kind, ?event, path, "audio event queued");
                if let Some(audio) = &mut self.audio {
                    match audio.play_bytes(kind, path, bytes) {
                        Ok(()) => tracing::info!(tick, ?kind, path, "audio playback started"),
                        Err(error) => tracing::warn!(%error, path, "audio playback failed"),
                    }
                }
            }
        }
    }

    fn play_audio_resource(&mut self, kind: AudioKind, path: &str) {
        let Some(bytes) = self.resources.read(path).ok() else {
            tracing::debug!(?kind, path, "audio resource is unavailable");
            return;
        };
        if let Some(audio) = &mut self.audio {
            match audio.play_bytes(kind, path, bytes) {
                Ok(()) => tracing::info!(?kind, path, "audio playback started"),
                Err(error) => tracing::warn!(%error, ?kind, path, "audio playback failed"),
            }
        }
    }

    fn push_tutorial_sprite(frame: &mut RenderFrame, resource_id: u32, x: f32, y: f32, z: i32) {
        frame.sprites.push(SpriteCommand {
            resource_id,
            x,
            y,
            z,
            scale: 1.0,
            alpha: 1.0,
        });
    }

    fn render_tutorial(&self, frame: &mut RenderFrame) {
        Self::push_tutorial_sprite(frame, DAY_BACKGROUND_IMAGE_ID, 0.0, 0.0, 0);

        for (resource_id, x, y, z) in [
            (CRAZY_DAVE_BODY_IMAGE_ID, 0.0, 199.0, 1),
            (CRAZY_DAVE_OUTER_ARM_IMAGE_ID, 0.0, 441.0, 2),
            (CRAZY_DAVE_INNER_ARM_IMAGE_ID, 218.0, 422.0, 2),
            (CRAZY_DAVE_OUTER_HAND_IMAGE_ID, 68.0, 392.0, 3),
            (CRAZY_DAVE_INNER_HAND_IMAGE_ID, 5.0, 430.0, 3),
            (CRAZY_DAVE_OUTER_FINGER1_IMAGE_ID, 234.0, 378.0, 4),
            (CRAZY_DAVE_OUTER_FINGER2_IMAGE_ID, 97.0, 399.0, 4),
            (CRAZY_DAVE_OUTER_FINGER3_IMAGE_ID, 105.0, 424.0, 4),
            (CRAZY_DAVE_OUTER_FINGER4_IMAGE_ID, 115.0, 422.0, 4),
            (CRAZY_DAVE_INNER_FINGER1_IMAGE_ID, 94.0, 400.0, 4),
            (CRAZY_DAVE_INNER_FINGER2_IMAGE_ID, 223.0, 394.0, 4),
            (CRAZY_DAVE_INNER_FINGER3_IMAGE_ID, 171.0, 450.0, 4),
            (CRAZY_DAVE_INNER_FINGER4_IMAGE_ID, 226.0, 395.0, 4),
            (CRAZY_DAVE_HEAD_IMAGE_ID, -4.0, 112.0, 5),
            (CRAZY_DAVE_EYEBROW_IMAGE_ID, 139.0, 157.0, 6),
            (CRAZY_DAVE_EYE_IMAGE_ID, 138.0, 170.0, 6),
            (CRAZY_DAVE_MOUTH_IMAGE_ID, 107.0, 226.0, 6),
            (CRAZY_DAVE_BEARD_IMAGE_ID, 78.0, 212.0, 7),
            (CRAZY_DAVE_POT_IMAGE_ID, 2.0, 103.0, 8),
        ] {
            Self::push_tutorial_sprite(frame, resource_id, x, y, z);
        }

        Self::push_tutorial_sprite(frame, TUTORIAL_BUBBLE_IMAGE_ID, 285.0, 20.0, 20);
        Self::push_tutorial_sprite(
            frame,
            if self.tutorial_page == 0 {
                TUTORIAL_TEXT1_IMAGE_ID
            } else {
                TUTORIAL_TEXT2_IMAGE_ID
            },
            310.0,
            26.0,
            21,
        );
        Self::push_tutorial_sprite(frame, TUTORIAL_CONTINUE_IMAGE_ID, 365.0, 151.0, 21);
    }

    fn render_mode_select(&self, frame: &mut RenderFrame) {
        frame.sprites.push(SpriteCommand {
            resource_id: MODE_SELECT_BACKGROUND_IMAGE_ID,
            x: 0.0,
            y: 0.0,
            z: 0,
            scale: 1.0,
            alpha: 1.0,
        });
        let thumbnail_base = match self.selected_mode {
            ModeKind::Survival => SURVIVAL_THUMBNAIL_BASE_IMAGE_ID,
            _ => CHALLENGE_THUMBNAIL_BASE_IMAGE_ID,
        };
        for index in 0..mode_level_names(self.selected_mode).len() {
            let column = index % 5;
            let row = index / 5;
            let x = 30.0 + column as f32 * 150.0;
            let y = 55.0 + row as f32 * 135.0;
            frame.sprites.push(SpriteCommand {
                resource_id: MODE_SELECT_WINDOW_IMAGE_ID,
                x,
                y,
                z: 1,
                scale: 1.0,
                alpha: if index == usize::from(self.selected_level) {
                    1.0
                } else {
                    0.72
                },
            });
            frame.sprites.push(SpriteCommand {
                resource_id: if self.selected_mode != ModeKind::ZenGarden
                    && (self.selected_mode == ModeKind::Survival || index < 22)
                {
                    thumbnail_base + index as u32
                } else {
                    MODE_SELECT_BLANK_IMAGE_ID
                },
                x: x + 19.0,
                y: y + 22.0,
                z: 2,
                scale: 1.0,
                alpha: 1.0,
            });
        }
    }

    fn render_store(&self, frame: &mut RenderFrame) {
        frame.sprites.push(SpriteCommand {
            resource_id: STORE_BACKGROUND_IMAGE_ID,
            x: 0.0,
            y: 0.0,
            z: 30,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: STORE_CAR_IMAGE_ID,
            x: 98.0,
            y: 300.0,
            z: 31,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: STORE_SIGN_IMAGE_ID,
            x: 144.0,
            y: 8.0,
            z: 32,
            scale: 0.82,
            alpha: 1.0,
        });

        for &(item, x, y) in &STORE_ITEM_LAYOUT {
            let sold_out = store_item_sold_out(self.game.state(), item);
            let affordable =
                self.game.state().coins >= store_item_cost(item, self.game.state().packet_upgrades);
            let alpha = if sold_out {
                0.35
            } else if affordable {
                1.0
            } else {
                0.55
            };
            let (resource_id, scale) = store_item_image(item);
            frame.sprites.push(SpriteCommand {
                resource_id,
                x,
                y,
                z: 33,
                scale,
                alpha,
            });
            frame.sprites.push(SpriteCommand {
                resource_id: store_item_name_image(item),
                x: x - 14.0,
                y: y + 76.0,
                z: 34,
                scale: 0.8,
                alpha,
            });
            frame.sprites.push(SpriteCommand {
                resource_id: STORE_PRICE_TAG_IMAGE_ID,
                x: x + 4.0,
                y: y + 101.0,
                z: 34,
                scale: 0.55,
                alpha,
            });
            frame.sprites.push(SpriteCommand {
                resource_id: store_item_price_image(item, self.game.state().packet_upgrades),
                x: x + 22.0,
                y: y + 108.0,
                z: 35,
                scale: 0.65,
                alpha,
            });
        }

        frame.sprites.push(SpriteCommand {
            resource_id: STORE_MAIN_MENU_BUTTON_IMAGE_ID,
            x: 318.0,
            y: 540.0,
            z: 36,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: STORE_BACK_TEXT_IMAGE_ID,
            x: 350.0,
            y: 550.0,
            z: 37,
            scale: 0.8,
            alpha: 1.0,
        });
    }

    fn render_dialog_shell(frame: &mut RenderFrame) {
        for (resource_id, x, y) in [
            (PAUSE_DIALOG_TOP_LEFT_IMAGE_ID, 240.0, 145.0),
            (PAUSE_DIALOG_TOP_MIDDLE_IMAGE_ID, 347.0, 145.0),
            (PAUSE_DIALOG_TOP_RIGHT_IMAGE_ID, 440.0, 145.0),
            (PAUSE_DIALOG_HEADER_IMAGE_ID, 306.0, 145.0),
            (PAUSE_DIALOG_CENTER_LEFT_IMAGE_ID, 240.0, 209.0),
            (PAUSE_DIALOG_CENTER_MIDDLE_IMAGE_ID, 347.0, 209.0),
            (PAUSE_DIALOG_CENTER_RIGHT_IMAGE_ID, 440.0, 209.0),
            (PAUSE_DIALOG_BOTTOM_LEFT_IMAGE_ID, 240.0, 263.0),
            (PAUSE_DIALOG_BOTTOM_MIDDLE_IMAGE_ID, 347.0, 263.0),
            (PAUSE_DIALOG_BOTTOM_RIGHT_IMAGE_ID, 440.0, 263.0),
        ] {
            frame.sprites.push(SpriteCommand {
                resource_id,
                x,
                y,
                z: 30,
                scale: 1.0,
                alpha: 1.0,
            });
        }
    }

    fn render_pause(&self, frame: &mut RenderFrame) {
        Self::render_dialog_shell(frame);
        frame.sprites.push(SpriteCommand {
            resource_id: PAUSE_HEADER_TEXT_IMAGE_ID,
            x: 278.0,
            y: 164.0,
            z: 31,
            scale: 0.8,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: PAUSE_BODY_TEXT_IMAGE_ID,
            x: 278.0,
            y: 224.0,
            z: 31,
            scale: 0.75,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: PAUSE_RESUME_BUTTON_IMAGE_ID,
            x: 220.0,
            y: 388.0,
            z: 32,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: PAUSE_RESUME_TEXT_IMAGE_ID,
            x: 300.0,
            y: 424.0,
            z: 33,
            scale: 0.9,
            alpha: 1.0,
        });
    }

    fn render_game_over(&self, frame: &mut RenderFrame) {
        let lost_time = self.game.state().game_lost_cutscene_time;
        if lost_time.is_some_and(|time| time >= GAME_LOST_GRAPHIC_START) {
            frame.sprites.push(SpriteCommand {
                resource_id: SCREEN_PIXEL_IMAGE_ID,
                x: 0.0,
                y: 0.0,
                z: 20,
                scale: 800.0,
                alpha: 1.0,
            });
        }
        if game_lost_cutscene_active(lost_time) {
            if lost_time.is_some_and(|time| time >= GAME_LOST_GRAPHIC_START) {
                frame.sprites.push(SpriteCommand {
                    resource_id: ZOMBIES_WON_IMAGE_ID,
                    x: 118.0,
                    y: 66.0,
                    z: 21,
                    scale: 1.0,
                    alpha: 1.0,
                });
            }
            return;
        }
        frame.sprites.push(SpriteCommand {
            resource_id: SCREEN_PIXEL_IMAGE_ID,
            x: 0.0,
            y: 0.0,
            z: 20,
            scale: 800.0,
            alpha: 0.65,
        });
        Self::render_dialog_shell(frame);
        frame.sprites.push(SpriteCommand {
            resource_id: GAME_OVER_HEADER_TEXT_IMAGE_ID,
            x: 278.0,
            y: 164.0,
            z: 31,
            scale: 0.8,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: GAME_OVER_BODY_TEXT_IMAGE_ID,
            x: 278.0,
            y: 224.0,
            z: 31,
            scale: 0.75,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: PAUSE_RESUME_BUTTON_IMAGE_ID,
            x: 220.0,
            y: 388.0,
            z: 32,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: GAME_OVER_TRY_AGAIN_TEXT_IMAGE_ID,
            x: 300.0,
            y: 424.0,
            z: 33,
            scale: 0.9,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: ALMANAC_INDEX_BUTTON_IMAGE_ID,
            x: 595.0,
            y: 145.0,
            z: 32,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: GAME_OVER_MAIN_MENU_TEXT_IMAGE_ID,
            x: 595.0,
            y: 145.0,
            z: 33,
            scale: 1.0,
            alpha: 1.0,
        });
    }

    fn render_credits(&self, frame: &mut RenderFrame) {
        let (phase, phase_frame) = credits_phase_at(self.credits_frame);
        if phase == 3 {
            frame.sprites.push(SpriteCommand {
                resource_id: SCREEN_PIXEL_IMAGE_ID,
                x: 0.0,
                y: 0.0,
                z: 0,
                scale: 800.0,
                alpha: 1.0,
            });
            let scroll = credits_scroll_offset(self.credits_frame);
            for (index, resource_id) in [
                CREDITS_TITLE_TEXT_IMAGE_ID,
                CREDITS_LINE1_TEXT_IMAGE_ID,
                CREDITS_LINE2_TEXT_IMAGE_ID,
                CREDITS_LINE3_TEXT_IMAGE_ID,
                CREDITS_LINE4_TEXT_IMAGE_ID,
                CREDITS_LINE5_TEXT_IMAGE_ID,
                CREDITS_LINE6_TEXT_IMAGE_ID,
            ]
            .into_iter()
            .enumerate()
            {
                frame.sprites.push(SpriteCommand {
                    resource_id,
                    x: if index == 0 { 140.0 } else { 40.0 },
                    y: 560.0 + index as f32 * 62.0 - scroll,
                    z: 2,
                    scale: if index == 0 { 1.0 } else { 0.85 },
                    alpha: 1.0,
                });
            }
        } else {
            frame.sprites.push(SpriteCommand {
                resource_id: match phase {
                    0 => DAY_BACKGROUND_IMAGE_ID,
                    1 => POOL_BACKGROUND_IMAGE_ID,
                    _ => NIGHT_BACKGROUND_IMAGE_ID,
                },
                x: 0.0,
                y: 0.0,
                z: 0,
                scale: 1.0,
                alpha: 1.0,
            });
            frame.sprites.push(SpriteCommand {
                resource_id: CREDITS_STAGE_IMAGE_ID,
                x: 248.0,
                y: 335.0,
                z: 1,
                scale: 1.0,
                alpha: 1.0,
            });
            frame.sprites.push(SpriteCommand {
                resource_id: CREDITS_TITLE_TEXT_IMAGE_ID,
                x: 140.0,
                y: 24.0,
                z: 5,
                scale: 1.0,
                alpha: 1.0,
            });
            let brain_x = 310.0 + (phase_frame * 0.035).sin() * 190.0;
            let brain_y = 120.0 + (phase_frame * 0.05).sin() * 18.0;
            frame.sprites.push(SpriteCommand {
                resource_id: CREDITS_BIG_BRAIN_IMAGE_ID,
                x: brain_x,
                y: brain_y,
                z: 4,
                scale: 0.7,
                alpha: 1.0,
            });
            let phase_asset = match phase {
                0 => CREDITS_LINE1_TEXT_IMAGE_ID,
                1 => CREDITS_LINE3_TEXT_IMAGE_ID,
                _ => CREDITS_LINE5_TEXT_IMAGE_ID,
            };
            frame.sprites.push(SpriteCommand {
                resource_id: phase_asset,
                x: 40.0,
                y: 220.0,
                z: 5,
                scale: 0.85,
                alpha: 1.0,
            });
            frame.sprites.push(SpriteCommand {
                resource_id: match phase {
                    0 => CREDITS_LINE2_TEXT_IMAGE_ID,
                    1 => CREDITS_ZOMBIE_NOTE_IMAGE_ID,
                    _ => CREDITS_MTV_IMAGE_ID,
                },
                x: if phase == 1 { 130.0 } else { 270.0 },
                y: if phase == 1 { 70.0 } else { 90.0 },
                z: 3,
                scale: if phase == 1 { 0.65 } else { 1.0 },
                alpha: 1.0,
            });
            if phase == 2 {
                frame.sprites.push(SpriteCommand {
                    resource_id: CREDITS_WE_ARE_UNDEAD_IMAGE_ID,
                    x: 80.0,
                    y: 480.0,
                    z: 3,
                    scale: 0.8,
                    alpha: 1.0,
                });
            }
        }
        if credits_controls_visible(self.credits_frame) {
            frame.sprites.push(SpriteCommand {
                resource_id: CREDITS_PLAY_BUTTON_IMAGE_ID,
                x: 10.0,
                y: 530.0,
                z: 20,
                scale: 1.0,
                alpha: 1.0,
            });
            frame.sprites.push(SpriteCommand {
                resource_id: CREDITS_REPLAY_TEXT_IMAGE_ID,
                x: 42.0,
                y: 550.0,
                z: 21,
                scale: 1.0,
                alpha: 1.0,
            });
            frame.sprites.push(SpriteCommand {
                resource_id: PAUSE_RESUME_BUTTON_IMAGE_ID,
                x: 298.0,
                y: 554.0,
                z: 20,
                scale: 1.0,
                alpha: 1.0,
            });
            frame.sprites.push(SpriteCommand {
                resource_id: CREDITS_MAIN_MENU_TEXT_IMAGE_ID,
                x: 312.0,
                y: 562.0,
                z: 21,
                scale: 1.0,
                alpha: 1.0,
            });
        }
        self.render_particles(frame);
    }

    fn render_particles(&self, frame: &mut RenderFrame) {
        for particle in self.particle_holder.render(&self.particle_catalog) {
            let Some(image) = self.particle_images.get(particle.image) else {
                continue;
            };
            frame.affine_sprites.push(particle.params.sprite(
                image.resource_id,
                particle.z,
                image.columns,
                image.rows,
            ));
        }
    }

    fn render_complete(&self, frame: &mut RenderFrame) {
        if self.credits_open {
            self.render_credits(frame);
            return;
        }
        let reward = completion_award_coin(self.game.state());
        let note = reward == Some(CoinType::Note);
        frame.sprites.push(SpriteCommand {
            resource_id: if note {
                if matches!(self.game.state().level, 19 | 39) {
                    NIGHT_BACKGROUND_IMAGE_ID
                } else {
                    DAY_BACKGROUND_IMAGE_ID
                }
            } else {
                AWARD_SCREEN_BACKGROUND_IMAGE_ID
            },
            x: if note { -700.0 } else { 0.0 },
            y: if note { -300.0 } else { 0.0 },
            z: 0,
            scale: if note { 2.0 } else { 1.0 },
            alpha: 1.0,
        });
        if note {
            frame.sprites.push(SpriteCommand {
                resource_id: HELP_ZOMBIE_NOTE_IMAGE_ID,
                x: 80.0,
                y: 80.0,
                z: 1,
                scale: 1.0,
                alpha: 1.0,
            });
            frame.sprites.push(SpriteCommand {
                resource_id: completion_note_asset(self.game.state().level),
                x: 131.0,
                y: 132.0,
                z: 2,
                scale: 1.0,
                alpha: 1.0,
            });
        }
        frame.sprites.push(SpriteCommand {
            resource_id: AWARD_TITLE_TEXT_BASE_IMAGE_ID + completion_award_text_slot(reward),
            x: 200.0,
            y: 42.0,
            z: 3,
            scale: 1.0,
            alpha: 1.0,
        });
        if let Some((resource_id, x, y, scale)) = completion_award_asset(self.game.state(), reward)
        {
            frame.sprites.push(SpriteCommand {
                resource_id,
                x,
                y,
                z: 2,
                scale,
                alpha: 1.0,
            });
        }
        if reward == Some(CoinType::FinalSeedPacket) {
            frame.sprites.push(SpriteCommand {
                resource_id: SEED_PACKET_NORMAL_IMAGE_ID,
                x: 322.0,
                y: 135.0,
                z: 2,
                scale: 1.0,
                alpha: 1.0,
            });
            if let Some(plant_type) = adventure_seed_choices(self.game.state().level, false)
                .into_iter()
                .nth(usize::from(adventure_award_seed(self.game.state().level)))
                && let Some((resource_id, offset_x, offset_y, scale)) =
                    board_plant_image(plant_type)
            {
                frame.sprites.push(SpriteCommand {
                    resource_id,
                    x: 322.0 + offset_x,
                    y: 135.0 + offset_y,
                    z: 3,
                    scale: scale * 0.75,
                    alpha: 1.0,
                });
            }
        }
        frame.sprites.push(SpriteCommand {
            resource_id: AWARD_BODY_TEXT_BASE_IMAGE_ID + completion_award_text_slot(reward),
            x: 250.0,
            y: 365.0,
            z: 4,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: SEED_CHOOSER_BUTTON_IMAGE_ID,
            x: 324.0,
            y: 500.0,
            z: 2,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: AWARD_CONTINUE_TEXT_IMAGE_ID,
            x: 324.0,
            y: 500.0,
            z: 3,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: HELP_MENU_BUTTON_IMAGE_ID,
            x: 677.0,
            y: 16.0,
            z: 2,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: AWARD_MAIN_MENU_TEXT_IMAGE_ID,
            x: 677.0,
            y: 16.0,
            z: 3,
            scale: 1.0,
            alpha: 1.0,
        });
    }

    fn render_options(&self, frame: &mut RenderFrame) {
        frame.sprites.push(SpriteCommand {
            resource_id: OPTIONS_BACKGROUND_IMAGE_ID,
            x: 188.0,
            y: 50.0,
            z: 30,
            scale: 1.0,
            alpha: 1.0,
        });
        for (resource_id, x, y) in [
            (OPTIONS_MUSIC_LABEL_IMAGE_ID, 220.0, 180.0),
            (OPTIONS_SFX_LABEL_IMAGE_ID, 220.0, 225.0),
            (OPTIONS_ACCELERATION_LABEL_IMAGE_ID, 195.0, 275.0),
            (OPTIONS_FULLSCREEN_LABEL_IMAGE_ID, 225.0, 320.0),
        ] {
            frame.sprites.push(SpriteCommand {
                resource_id,
                x,
                y,
                z: 31,
                scale: 0.9,
                alpha: 1.0,
            });
        }
        for (volume, y) in [
            (self.options_music_volume, 166.0),
            (self.options_effects_volume, 211.0),
        ] {
            frame.sprites.push(SpriteCommand {
                resource_id: OPTIONS_SLIDER_SLOT_IMAGE_ID,
                x: 387.0,
                y,
                z: 31,
                scale: 1.0,
                alpha: 1.0,
            });
            frame.sprites.push(SpriteCommand {
                resource_id: OPTIONS_SLIDER_KNOB_IMAGE_ID,
                x: 387.0 + 113.0 * f32::from(volume) / 100.0,
                y: y - 9.0,
                z: 32,
                scale: 1.0,
                alpha: 1.0,
            });
        }
        frame.sprites.push(SpriteCommand {
            resource_id: OPTIONS_CHECKBOX_ON_IMAGE_ID,
            x: 472.0,
            y: 225.0,
            z: 32,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: if self.fullscreen {
                OPTIONS_CHECKBOX_ON_IMAGE_ID
            } else {
                OPTIONS_CHECKBOX_OFF_IMAGE_ID
            },
            x: 472.0,
            y: 256.0,
            z: 32,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: PAUSE_RESUME_BUTTON_IMAGE_ID,
            x: 218.0,
            y: 431.0,
            z: 32,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: OPTIONS_BACK_TEXT_IMAGE_ID,
            x: 290.0,
            y: 466.0,
            z: 33,
            scale: 0.9,
            alpha: 1.0,
        });
    }

    fn almanac_plant_available(&self, plant_type: PlantType) -> bool {
        let unlocked = &self.game.state().unlocked_plants;
        // ponytail: profile-less checkpoints keep the two starter cards; the
        // save-backed list supplies the real SeedTypeAvailable state.
        if unlocked.is_empty() {
            matches!(plant_type, PlantType::Peashooter | PlantType::Sunflower)
        } else {
            unlocked.contains(&plant_type)
        }
    }

    fn almanac_zombie_is_shown(&self, zombie_type: ZombieType) -> bool {
        almanac_zombie_is_shown_at(
            self.profile
                .as_ref()
                .map_or(self.game.state().level.max(1), |profile| {
                    profile.adventure_level
                }),
            self.profile
                .as_ref()
                .map_or(0, |profile| profile.adventure_rounds),
            self.game.state().adventure_finished,
            self.game.state().defeated_zombies.contains(&zombie_type),
            zombie_type,
        )
    }

    fn almanac_zombie_has_silhouette(&self, zombie_type: ZombieType) -> bool {
        almanac_zombie_has_silhouette_at(
            self.profile
                .as_ref()
                .map_or(self.game.state().level.max(1), |profile| {
                    profile.adventure_level
                }),
            self.profile
                .as_ref()
                .map_or(0, |profile| profile.adventure_rounds),
            self.game.state().adventure_finished,
            zombie_type,
        )
    }

    fn almanac_zombie_has_description(&self, zombie_type: ZombieType) -> bool {
        almanac_zombie_has_description_at(
            self.profile
                .as_ref()
                .map_or(self.game.state().level.max(1), |profile| {
                    profile.adventure_level
                }),
            self.profile
                .as_ref()
                .map_or(0, |profile| profile.adventure_rounds),
            self.game.state().adventure_finished,
            self.game.state().defeated_zombies.contains(&zombie_type),
            zombie_type,
        )
    }

    fn add_runtime_asset(&mut self, asset: ImageAsset) -> bool {
        if let Some(renderer) = &mut self.renderer {
            if let Err(error) = renderer.add_image(asset) {
                tracing::error!(%error, "runtime image upload failed");
                return false;
            }
        } else {
            self.assets.push(asset);
        }
        true
    }

    fn ensure_board_hud_assets(&mut self) {
        let sun = self.game.state().sun;
        if self.sun_count_asset_value != Some(sun)
            && let Ok(asset) = render_colored_text_image(
                BOARD_SUN_COUNT_IMAGE_ID,
                &sun.to_string(),
                50,
                18,
                14,
                [0, 0, 0],
            )
            && self.add_runtime_asset(asset)
        {
            self.sun_count_asset_value = Some(sun);
        }

        let packets: Vec<_> = self
            .game
            .state()
            .board
            .seed_packets
            .iter()
            .take(10)
            .map(|packet| (packet.slot, packet.plant_type))
            .collect();
        for (slot, plant_type) in packets {
            if self.seed_bank_cost_plants.get(&slot) == Some(&plant_type) {
                continue;
            }
            let Ok(asset) = render_colored_text_image(
                BOARD_SEED_COST_BASE_IMAGE_ID + u32::from(slot),
                &seed_packet_cost(plant_type).to_string(),
                50,
                15,
                12,
                [0, 0, 0],
            ) else {
                continue;
            };
            if self.add_runtime_asset(asset) {
                self.seed_bank_cost_plants.insert(slot, plant_type);
            }
        }
    }

    fn reset_board_hud_assets(&mut self) {
        self.seed_bank_cost_plants.clear();
        self.sun_count_asset_value = None;
        self.progress_meter_asset_width = None;
        self.ensure_board_hud_assets();
        self.ensure_board_progress_asset();
    }

    fn ensure_board_progress_asset(&mut self) {
        let width = progress_meter_width(self.game.state());
        if self.progress_meter_asset_width == width {
            return;
        }
        let Some(width) = width else {
            self.progress_meter_asset_width = None;
            return;
        };
        let source_x = 158 - width - 7;
        let Ok(asset) = load_cropped_image(
            &self.resources,
            BOARD_PROGRESS_FILL_IMAGE_ID,
            "images/FlagMeter.png",
            source_x,
            27,
            width,
            27,
        ) else {
            return;
        };
        if self.add_runtime_asset(asset) {
            self.progress_meter_asset_width = Some(width);
        }
    }

    fn ensure_almanac_description_asset(&mut self, plant: bool) {
        let slot = if plant {
            self.almanac_selected_plant
        } else {
            self.almanac_selected_zombie
        };
        let id = if plant {
            ALMANAC_PLANT_DESCRIPTION_BASE_IMAGE_ID + u32::from(slot)
        } else {
            ALMANAC_ZOMBIE_DESCRIPTION_BASE_IMAGE_ID + u32::from(slot)
        };
        if self.almanac_loaded_descriptions.contains(&id) {
            return;
        }
        let (description, height) = if plant {
            (
                almanac_lawn_description(
                    &self.almanac_strings,
                    almanac_plant_string_key(slot),
                    "Plant description unavailable.",
                ),
                230,
            )
        } else {
            let description = match almanac_zombie_type(slot) {
                Some(zombie_type) if self.almanac_zombie_has_description(zombie_type) => {
                    almanac_lawn_description(
                        &self.almanac_strings,
                        almanac_zombie_string_key(slot),
                        "Zombie description unavailable.",
                    )
                }
                _ => almanac_lawn_text(
                    &self.almanac_strings,
                    "NOT_ENCOUNTERED_YET",
                    "Not encountered yet.",
                ),
            };
            (description, 170)
        };
        let asset = match render_colored_text_image(id, &description, 258, height, 12, [40, 50, 90])
        {
            Ok(asset) => asset,
            Err(error) => {
                tracing::error!(%error, "almanac description rendering failed");
                return;
            }
        };
        if let Some(renderer) = &mut self.renderer {
            if let Err(error) = renderer.add_image(asset) {
                tracing::error!(%error, "almanac description upload failed");
                return;
            }
        } else {
            self.assets.push(asset);
        }
        self.almanac_loaded_descriptions.insert(id);
    }

    fn render_almanac_plants(&self, frame: &mut RenderFrame) {
        frame.sprites.push(SpriteCommand {
            resource_id: ALMANAC_PLANTS_TEXT_IMAGE_ID,
            x: 322.0,
            y: 30.0,
            z: 31,
            scale: 1.0,
            alpha: 1.0,
        });
        for slot in 0..ALMANAC_PLANT_COUNT {
            let Some(plant_type) = almanac_plant_type(slot) else {
                continue;
            };
            if !self.almanac_plant_available(plant_type) {
                continue;
            }
            let (x, y) = almanac_plant_position(slot);
            if plant_type == PlantType::Other(ALMANAC_IMITATER_SLOT) {
                frame.sprites.push(SpriteCommand {
                    resource_id: ALMANAC_IMITATER_IMAGE_ID,
                    x,
                    y,
                    z: 32,
                    scale: 1.0,
                    alpha: 1.0,
                });
            } else {
                frame.sprites.push(SpriteCommand {
                    resource_id: SEED_PACKET_NORMAL_IMAGE_ID,
                    x,
                    y,
                    z: 31,
                    scale: 1.0,
                    alpha: 1.0,
                });
                if let Some((resource_id, scale)) = almanac_plant_icon(plant_type) {
                    frame.sprites.push(SpriteCommand {
                        resource_id,
                        x: x + 4.0,
                        y: y + 9.0,
                        z: 32,
                        scale,
                        alpha: 1.0,
                    });
                }
            }
        }
        frame.sprites.push(SpriteCommand {
            resource_id: ALMANAC_PLANT_CARD_IMAGE_ID,
            x: 459.0,
            y: 86.0,
            z: 33,
            scale: 1.0,
            alpha: 1.0,
        });
        if let Some(plant_type) = almanac_plant_type(self.almanac_selected_plant)
            .filter(|plant_type| self.almanac_plant_available(*plant_type))
        {
            let slot_id = u32::from(self.almanac_selected_plant);
            frame.sprites.push(SpriteCommand {
                resource_id: ALMANAC_PLANT_NAME_BASE_IMAGE_ID + slot_id,
                x: 488.0,
                y: 274.0,
                z: 34,
                scale: 1.0,
                alpha: 1.0,
            });
            if plant_type != PlantType::Other(ALMANAC_IMITATER_SLOT) {
                frame.sprites.push(SpriteCommand {
                    resource_id: ALMANAC_PLANT_COST_BASE_IMAGE_ID + slot_id,
                    x: 485.0,
                    y: 520.0,
                    z: 34,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: ALMANAC_PLANT_RECHARGE_BASE_IMAGE_ID + slot_id,
                    x: 600.0,
                    y: 520.0,
                    z: 34,
                    scale: 1.0,
                    alpha: 1.0,
                });
            }
        }
        if let Some(plant_type) = almanac_plant_type(self.almanac_selected_plant)
            .filter(|plant_type| self.almanac_plant_available(*plant_type))
            && let Some((resource_id, offset_x, offset_y, scale)) = board_plant_image(plant_type)
        {
            frame.sprites.push(SpriteCommand {
                resource_id,
                x: 578.0 + offset_x,
                y: 140.0 + offset_y,
                z: 32,
                scale: scale * 1.2,
                alpha: 1.0,
            });
        }
        frame.sprites.push(SpriteCommand {
            resource_id: ALMANAC_PLANT_DESCRIPTION_BASE_IMAGE_ID
                + u32::from(self.almanac_selected_plant),
            x: 485.0,
            y: 309.0,
            z: 34,
            scale: 1.0,
            alpha: 1.0,
        });
    }

    fn render_almanac_zombies(&self, frame: &mut RenderFrame) {
        frame.sprites.push(SpriteCommand {
            resource_id: ALMANAC_ZOMBIES_TEXT_IMAGE_ID,
            x: 295.0,
            y: 34.0,
            z: 31,
            scale: 1.0,
            alpha: 1.0,
        });
        for index in 0..ALMANAC_ZOMBIE_COUNT {
            let Some(zombie_type) = almanac_zombie_type(index) else {
                continue;
            };
            let (x, y) = almanac_zombie_position(index);
            frame.sprites.push(SpriteCommand {
                resource_id: ALMANAC_ZOMBIE_WINDOW_IMAGE_ID,
                x,
                y,
                z: 31,
                scale: 1.0,
                alpha: 1.0,
            });
            if !self.almanac_zombie_is_shown(zombie_type) {
                frame.sprites.push(SpriteCommand {
                    resource_id: ALMANAC_ZOMBIE_BLANK_IMAGE_ID,
                    x,
                    y,
                    z: 32,
                    scale: 1.0,
                    alpha: 1.0,
                });
            } else if let Some(resource_id) = board_zombie_image(zombie_type) {
                frame.sprites.push(SpriteCommand {
                    resource_id,
                    x: x + 8.0,
                    y: y + 4.0,
                    z: 32,
                    scale: 0.5,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: ALMANAC_ZOMBIE_WINDOW2_IMAGE_ID,
                    x,
                    y,
                    z: 33,
                    scale: 1.0,
                    alpha: 1.0,
                });
            } else {
                // ponytail: blank unsupported entries until the reanimation
                // catalog is wired; the source list and hit regions are real.
                frame.sprites.push(SpriteCommand {
                    resource_id: ALMANAC_ZOMBIE_BLANK_IMAGE_ID,
                    x,
                    y,
                    z: 32,
                    scale: 1.0,
                    alpha: 1.0,
                });
            }
        }
        if let Some(zombie_type) = almanac_zombie_type(self.almanac_selected_zombie)
            && let Some(resource_id) = board_zombie_image(zombie_type)
        {
            frame.sprites.push(SpriteCommand {
                resource_id,
                x: 559.0,
                y: 175.0,
                z: 31,
                scale: 0.9,
                alpha: 1.0,
            });
        }
        frame.sprites.push(SpriteCommand {
            resource_id: ALMANAC_ZOMBIE_CARD_IMAGE_ID,
            x: 455.0,
            y: 78.0,
            z: 33,
            scale: 1.0,
            alpha: 1.0,
        });
        let selected_zombie_name_id = if almanac_zombie_type(self.almanac_selected_zombie)
            .is_some_and(|zombie_type| self.almanac_zombie_has_silhouette(zombie_type))
        {
            ALMANAC_ZOMBIE_SILHOUETTE_NAME_BASE_IMAGE_ID
        } else {
            ALMANAC_ZOMBIE_NAME_BASE_IMAGE_ID
        } + u32::from(self.almanac_selected_zombie);
        frame.sprites.push(SpriteCommand {
            resource_id: selected_zombie_name_id,
            x: 484.0,
            y: 348.0,
            z: 34,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: ALMANAC_ZOMBIE_DESCRIPTION_BASE_IMAGE_ID
                + u32::from(self.almanac_selected_zombie),
            x: 484.0,
            y: 377.0,
            z: 34,
            scale: 1.0,
            alpha: 1.0,
        });
    }

    fn render_almanac(&self, frame: &mut RenderFrame) {
        let background = match self.almanac_page {
            1 => ALMANAC_PLANT_BACKGROUND_IMAGE_ID,
            2 => ALMANAC_ZOMBIE_BACKGROUND_IMAGE_ID,
            _ => ALMANAC_INDEX_BACKGROUND_IMAGE_ID,
        };
        frame.sprites.push(SpriteCommand {
            resource_id: background,
            x: 0.0,
            y: 0.0,
            z: 30,
            scale: 1.0,
            alpha: 1.0,
        });
        match self.almanac_page {
            1 => {
                self.render_almanac_plants(frame);
            }
            2 => {
                self.render_almanac_zombies(frame);
            }
            _ => {
                frame.sprites.push(SpriteCommand {
                    resource_id: ALMANAC_TITLE_TEXT_IMAGE_ID,
                    x: 200.0,
                    y: 44.0,
                    z: 31,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: SEED_SUNFLOWER_IMAGE_ID,
                    x: 170.0,
                    y: 250.0,
                    z: 31,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: BOARD_ZOMBIE_BODY_IMAGE_ID,
                    x: 560.0,
                    y: 245.0,
                    z: 31,
                    scale: 0.8,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: SEED_CHOOSER_BUTTON_IMAGE_ID,
                    x: 130.0,
                    y: 345.0,
                    z: 32,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: ALMANAC_PLANTS_TEXT_IMAGE_ID,
                    x: 130.0,
                    y: 350.0,
                    z: 33,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: ALMANAC_NAV_BUTTON_IMAGE_ID,
                    x: 487.0,
                    y: 345.0,
                    z: 32,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: ALMANAC_ZOMBIES_TEXT_IMAGE_ID,
                    x: 460.0,
                    y: 350.0,
                    z: 33,
                    scale: 0.7,
                    alpha: 1.0,
                });
            }
        }
        if self.almanac_page != 0 {
            frame.sprites.push(SpriteCommand {
                resource_id: ALMANAC_INDEX_BUTTON_IMAGE_ID,
                x: 32.0,
                y: 567.0,
                z: 32,
                scale: 1.0,
                alpha: 1.0,
            });
            frame.sprites.push(SpriteCommand {
                resource_id: ALMANAC_INDEX_TEXT_IMAGE_ID,
                x: 32.0,
                y: 567.0,
                z: 33,
                scale: 1.0,
                alpha: 1.0,
            });
        }
        frame.sprites.push(SpriteCommand {
            resource_id: ALMANAC_CLOSE_BUTTON_IMAGE_ID,
            x: 676.0,
            y: 567.0,
            z: 32,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: ALMANAC_CLOSE_TEXT_IMAGE_ID,
            x: 676.0,
            y: 567.0,
            z: 33,
            scale: 1.0,
            alpha: 1.0,
        });
    }

    fn render_help(&self, frame: &mut RenderFrame) {
        frame.sprites.push(SpriteCommand {
            resource_id: DAY_BACKGROUND_IMAGE_ID,
            x: -700.0,
            y: -300.0,
            z: 30,
            scale: 2.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: HELP_ZOMBIE_NOTE_IMAGE_ID,
            x: 80.0,
            y: 80.0,
            z: 31,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: HELP_CONTENT_IMAGE_ID,
            x: 131.0,
            y: 132.0,
            z: 32,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: SEED_CHOOSER_BUTTON_IMAGE_ID,
            x: 324.0,
            y: 520.0,
            z: 33,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: HELP_MENU_BUTTON_IMAGE_ID,
            x: 677.0,
            y: 16.0,
            z: 33,
            scale: 1.0,
            alpha: 1.0,
        });
        frame.sprites.push(SpriteCommand {
            resource_id: HELP_MAIN_MENU_TEXT_IMAGE_ID,
            x: 677.0,
            y: 16.0,
            z: 34,
            scale: 1.0,
            alpha: 1.0,
        });
    }

    fn render_board_hud(&self, frame: &mut RenderFrame) {
        let state = self.game.state();
        let conveyor = has_conveyor_seed_bank(state);
        let slot_machine = state.challenge.kind == ChallengeKind::SlotMachine;
        let packet_count = state.board.seed_packets.len().min(10);
        let background = if slot_machine {
            BOARD_SUN_BANK_IMAGE_ID
        } else if conveyor {
            BOARD_CONVEYOR_BELT_BACKDROP_IMAGE_ID
        } else {
            BOARD_SEED_BANK_IMAGE_ID
        };
        let (background_x, background_z) = if conveyor { (83.0, 18) } else { (0.0, 18) };
        frame.sprites.push(SpriteCommand {
            resource_id: background,
            x: background_x,
            y: 0.0,
            z: background_z,
            scale: 1.0,
            alpha: 1.0,
        });

        if !conveyor
            && !slot_machine
            && let Some((resource_id, x)) = seed_bank_extension_image(packet_count)
        {
            frame.sprites.push(SpriteCommand {
                resource_id,
                x,
                y: 0.0,
                z: 18,
                scale: 1.0,
                alpha: 1.0,
            });
        }
        if conveyor {
            let belt_frame = (state.tick / 4) % 6;
            frame.sprites.push(SpriteCommand {
                resource_id: BOARD_CONVEYOR_BELT_BASE_IMAGE_ID + belt_frame as u32,
                x: 90.0,
                y: 63.0,
                z: 19,
                scale: 1.0,
                alpha: 1.0,
            });
        }
        if !conveyor && self.sun_count_asset_value.is_some() {
            frame.sprites.push(SpriteCommand {
                resource_id: BOARD_SUN_COUNT_IMAGE_ID,
                x: 9.0,
                y: 66.0,
                z: 20,
                scale: 1.0,
                alpha: 1.0,
            });
        }

        for (index, packet) in state.board.seed_packets.iter().take(10).enumerate() {
            let x = seed_packet_position_x(index, packet_count, conveyor, slot_machine);
            let alpha = if packet.refresh_remaining != 0 {
                0.55
            } else if !conveyor && seed_packet_cost(packet.plant_type) > state.sun {
                0.72
            } else {
                1.0
            };
            frame.sprites.push(SpriteCommand {
                resource_id: SEED_PACKET_NORMAL_IMAGE_ID,
                x,
                y: 0.0,
                z: 20,
                scale: 1.0,
                alpha,
            });
            if let Some((resource_id, icon_x, icon_y, scale)) =
                seed_bank_plant_image(packet.plant_type)
            {
                frame.sprites.push(SpriteCommand {
                    resource_id,
                    x: x + icon_x,
                    y: icon_y,
                    z: 21,
                    scale,
                    alpha,
                });
            }
            if !conveyor
                && !slot_machine
                && self.seed_bank_cost_plants.get(&packet.slot) == Some(&packet.plant_type)
            {
                frame.sprites.push(SpriteCommand {
                    resource_id: BOARD_SEED_COST_BASE_IMAGE_ID + u32::from(packet.slot),
                    x,
                    y: 53.0,
                    z: 22,
                    scale: 1.0,
                    alpha,
                });
            }
        }

        let shovel_x = if slot_machine {
            600.0
        } else {
            456.0 + seed_bank_extra_width(packet_count) as f32
        };
        frame.sprites.push(SpriteCommand {
            resource_id: BOARD_SHOVEL_BANK_IMAGE_ID,
            x: shovel_x,
            y: 0.0,
            z: 19,
            scale: 1.0,
            alpha: 1.0,
        });
        self.render_progress_meter(frame);
    }

    fn render_progress_meter(&self, frame: &mut RenderFrame) {
        let state = self.game.state();
        let Some(width) = progress_meter_width(state) else {
            return;
        };
        frame.sprites.push(SpriteCommand {
            resource_id: BOARD_PROGRESS_METER_IMAGE_ID,
            x: 600.0,
            y: 575.0,
            z: 23,
            scale: 1.0,
            alpha: 1.0,
        });
        if self.progress_meter_asset_width == Some(width) {
            frame.sprites.push(SpriteCommand {
                resource_id: BOARD_PROGRESS_FILL_IMAGE_ID,
                x: 751.0 - width as f32,
                y: 575.0,
                z: 24,
                scale: 1.0,
                alpha: 1.0,
            });
        }
        frame.sprites.push(SpriteCommand {
            resource_id: BOARD_PROGRESS_LEVEL_IMAGE_ID,
            x: 638.0,
            y: 589.0,
            z: 25,
            scale: 1.0,
            alpha: 1.0,
        });
        if let Some((flag_count, waves_per_flag)) = progress_meter_flag_layout(state) {
            for flag_wave in 1..=flag_count {
                let wave = flag_wave * waves_per_flag;
                let x = 748.0 - wave as f32 * 142.0 / state.board.wave.total as f32;
                let raised = wave < state.board.wave.current;
                frame.sprites.push(SpriteCommand {
                    resource_id: BOARD_PROGRESS_POLE_IMAGE_ID,
                    x,
                    y: 571.0,
                    z: 26,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: BOARD_PROGRESS_FLAG_IMAGE_ID,
                    x,
                    y: if raised { 558.0 } else { 572.0 },
                    z: 27,
                    scale: 1.0,
                    alpha: 1.0,
                });
            }
        }
        if matches!(state.mode, ModeKind::Adventure | ModeKind::Survival)
            && state.scene != SceneKind::Boss
        {
            let head_progress = 135 * width / 150;
            frame.sprites.push(SpriteCommand {
                resource_id: BOARD_PROGRESS_HEAD_IMAGE_ID,
                x: 738.0 - head_progress as f32,
                y: 572.0,
                z: 28,
                scale: 1.0,
                alpha: 1.0,
            });
        }
    }

    fn render_board_effects(&self, frame: &mut RenderFrame) {
        let Some(definition) = self.reanim_catalog.fire.as_ref() else {
            return;
        };
        let tick = self.game.state().tick;
        for effect in &self.visual_effects {
            let age = tick.saturating_sub(effect.start_tick);
            if age >= effect.duration {
                continue;
            }
            let action = if effect.duration > PROJECTILE_FIRE_DURATION && age >= 50 {
                "anim_done"
            } else {
                "anim_flame"
            };
            let Some(frame_position) = reanim_frame_position(definition, action, age)
                .or_else(|| reanim_frame_position(definition, "anim_flame", age))
            else {
                continue;
            };
            push_reanim_tracks(
                frame,
                definition,
                &self.reanim_catalog.image_ids,
                frame_position,
                (effect.x, effect.y, 1.0),
                |_| Some((effect.z, BlendMode::Alpha)),
            );
        }
    }

    fn render_board_pickup_reanim(
        &self,
        frame: &mut RenderFrame,
        definition: Option<&ReanimatorDefinition>,
        x: f32,
        y: f32,
        tick: u64,
    ) -> bool {
        let Some(definition) = definition else {
            return false;
        };
        let Some(frame_position) = reanim_frame_position(definition, "anim_idle", tick) else {
            return false;
        };
        // ponytail: share the board clock for pickup reanimations; exact source random phase/rate needs persisted visual state.
        push_reanim_tracks(
            frame,
            definition,
            &self.reanim_catalog.image_ids,
            frame_position,
            (x, y, 1.0),
            |_| Some((6, BlendMode::Alpha)),
        )
    }

    fn render_board_plant_reanim(
        &self,
        frame: &mut RenderFrame,
        plant: &neopvz_core::PlantState,
        x: f32,
        y: f32,
    ) -> bool {
        let Some((_, definition)) = self
            .reanim_catalog
            .plants
            .iter()
            .find(|(plant_type, _)| *plant_type == plant.plant_type)
        else {
            return false;
        };
        let tick = self.game.state().tick;
        let Some((action, frame_position)) = board_plant_reanim_actions(
            plant.plant_type,
            plant.asleep,
            plant.special_counter,
            plant.special_armed,
            plant.shooting_counter,
            plant.production_stage,
        )
        .iter()
        .find_map(|action| {
            reanim_frame_position(definition, action, tick).map(|position| (*action, position))
        }) else {
            return false;
        };
        let body_drawn = push_reanim_tracks(
            frame,
            definition,
            &self.reanim_catalog.image_ids,
            frame_position,
            (x, y, 1.0),
            |name| {
                board_plant_reanim_track_style(
                    plant.plant_type,
                    plant.special_armed,
                    plant.kernel_pult_projectile,
                    name,
                )
            },
        );
        body_drawn
            || push_board_plant_reanim_attachments(
                frame,
                definition,
                &self.reanim_catalog.image_ids,
                board_plant_reanim_attachment_specs(
                    plant.plant_type,
                    action,
                    plant.firing_directions,
                ),
                tick,
                (x, y, frame_position),
            )
    }

    fn render_board_zombie_reanim(
        &self,
        frame: &mut RenderFrame,
        zombie: &ZombieState,
        x: f32,
        y: f32,
    ) -> bool {
        if zombie.zombie_type == ZombieType::Boss {
            return self.render_board_boss_reanim(frame, zombie, x, y);
        }
        let Some((definition, specialized)) =
            board_zombie_reanim_definition(&self.reanim_catalog, zombie.zombie_type)
        else {
            return false;
        };
        let action = board_zombie_reanim_action(zombie);
        let Some(frame_position) =
            reanim_frame_position(definition, action, self.game.state().tick)
                .or_else(|| reanim_frame_position(definition, "anim_walk", self.game.state().tick))
                .or_else(|| reanim_frame_position(definition, "anim_idle", self.game.state().tick))
        else {
            return false;
        };
        let drawn = push_reanim_tracks_with_image_override(
            frame,
            definition,
            &self.reanim_catalog.image_ids,
            frame_position,
            (x, y, 1.0),
            |name| {
                if specialized {
                    board_specialized_zombie_reanim_track_visible(
                        name,
                        zombie.zombie_type,
                        zombie.armor_intact,
                        zombie.has_head,
                        zombie.has_arm,
                    )
                    .then_some((7, BlendMode::Alpha))
                } else {
                    board_zombie_reanim_track_visible(
                        name,
                        zombie.zombie_type,
                        zombie.armor_intact,
                        zombie.has_head,
                        zombie.has_arm,
                        self.game.state().mustache_mode,
                    )
                    .then_some((7, BlendMode::Alpha))
                }
            },
            |name| {
                if self.game.state().future_mode && name.eq_ignore_ascii_case("anim_head1") {
                    Some(future_head_image_symbol(zombie.id))
                } else if self.game.state().mustache_mode && name.starts_with("Zombie_mustache") {
                    Some(mustache_image_symbol(zombie.mustache_variant))
                } else {
                    None
                }
            },
        );
        if drawn && zombie.zombie_type == ZombieType::Flag {
            push_board_zombie_flag(&mut *frame, x, y);
        }
        drawn
    }

    fn render_board_boss_reanim(
        &self,
        frame: &mut RenderFrame,
        zombie: &ZombieState,
        x: f32,
        y: f32,
    ) -> bool {
        let Some(body) = self.reanim_catalog.boss.as_ref() else {
            return false;
        };
        let tick = self.game.state().tick;
        let action = board_boss_reanim_action(zombie);
        let Some(body_frame_position) = reanim_frame_position(body, action, tick)
            .or_else(|| reanim_frame_position(body, "anim_idle", tick))
        else {
            return false;
        };
        let body_drawn = push_reanim_tracks(
            frame,
            body,
            &self.reanim_catalog.image_ids,
            body_frame_position,
            (x, y, 1.0),
            |name| Some((board_boss_reanim_track_z(name), BlendMode::Alpha)),
        );
        let driver_drawn = self
            .reanim_catalog
            .boss_driver
            .as_ref()
            .and_then(|driver| {
                let anchor = body
                    .tracks
                    .iter()
                    .find(|track| track.name.eq_ignore_ascii_case("Boss_head2"))
                    .and_then(|track| reanim_transform_at(track, body_frame_position))?;
                let driver_frame_position = reanim_frame_position(driver, "anim_idle", tick)?;
                Some(push_reanim_tracks(
                    frame,
                    driver,
                    &self.reanim_catalog.image_ids,
                    driver_frame_position,
                    (x + anchor.x + 28.0, y + anchor.y - 84.0, 1.2),
                    |_| Some((7, BlendMode::Alpha)),
                ))
            })
            .unwrap_or(false);
        let ball_drawn = if zombie.boss_ball_active {
            let ball = if zombie.boss_ball_fire {
                self.reanim_catalog.boss_fireball.as_ref()
            } else {
                self.reanim_catalog.boss_iceball.as_ref()
            };
            ball.and_then(|definition| {
                let frame_position = reanim_frame_position(definition, "anim_form", tick)
                    .or_else(|| reanim_frame_position(definition, "anim_role", tick))?;
                Some(push_reanim_tracks(
                    frame,
                    definition,
                    &self.reanim_catalog.image_ids,
                    frame_position,
                    (
                        fixed_point_to_logical(zombie.boss_ball_x),
                        board_row_y(zombie.boss_ball_row) - 90.0,
                        1.0,
                    ),
                    |name| {
                        Some((
                            board_boss_ball_track_z(name),
                            board_boss_ball_blend_mode(name),
                        ))
                    },
                ))
            })
            .unwrap_or(false)
        } else {
            false
        };
        body_drawn || driver_drawn || ball_drawn
    }

    fn render_frame(&self) -> RenderFrame {
        let mut frame = RenderFrame::default();
        match self.game.state().scene {
            SceneKind::Title => {
                let state = &self.title_load_state;
                if state.logo_counter > 0 {
                    frame.sprites.push(SpriteCommand {
                        resource_id: TITLE_POPCAP_LOGO_IMAGE_ID,
                        x: 250.0,
                        y: 150.0,
                        z: 0,
                        scale: 1.0,
                        alpha: state.logo_alpha(),
                    });
                    return frame;
                }
                let grass_y = state.button_y() - 17.0;
                frame.sprites.push(SpriteCommand {
                    resource_id: TITLE_IMAGE_ID,
                    x: 0.0,
                    y: 0.0,
                    z: 0,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: TITLE_LOGO_IMAGE_ID,
                    x: 50.0,
                    y: state.logo_y(),
                    z: 1,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: TITLE_LOAD_BAR_DIRT_IMAGE_ID,
                    x: TITLE_LOAD_BAR_DIRT_X,
                    y: grass_y + 18.0,
                    z: 2,
                    scale: 1.0,
                    alpha: 1.0,
                });
                if state.bar_width >= TITLE_START_BUTTON_WIDTH {
                    frame.sprites.push(SpriteCommand {
                        resource_id: TITLE_LOAD_BAR_GRASS_IMAGE_ID,
                        x: TITLE_LOAD_BAR_X,
                        y: grass_y,
                        z: 3,
                        scale: 1.0,
                        alpha: 1.0,
                    });
                } else {
                    let width = state.bar_width.floor();
                    if width > 0.0 {
                        // Clipped grass: ClipRect(240, aGrassY, width, 33) in
                        // the source, expressed as a cropped uv rectangle.
                        frame.affine_sprites.push(AffineSpriteCommand {
                            resource_id: TITLE_LOAD_BAR_GRASS_IMAGE_ID,
                            x: TITLE_LOAD_BAR_X,
                            y: grass_y,
                            m00: 1.0,
                            m01: 0.0,
                            m10: 0.0,
                            m11: 1.0,
                            z: 3,
                            alpha: 1.0,
                            tint: [1.0; 3],
                            blend_mode: BlendMode::Alpha,
                            source: Some(AffineSpriteSource {
                                uv_min: [0.0, 0.0],
                                uv_max: [width / TITLE_START_BUTTON_WIDTH, 1.0],
                                pivot_uv: [0.0, 0.0],
                            }),
                        });
                    }
                    // SodRollCap riding the leading edge of the fill.
                    let roll_len = state.bar_width * 0.94;
                    let roll_scale = 1.0 - 0.5 * state.bar_width / TITLE_START_BUTTON_WIDTH;
                    let rotation = -roll_len / 180.0 * std::f32::consts::TAU;
                    let (sin, cos) = rotation.sin_cos();
                    frame.affine_sprites.push(AffineSpriteCommand {
                        resource_id: TITLE_SOD_ROLL_CAP_IMAGE_ID,
                        x: TITLE_LOAD_BAR_X + 11.0 + roll_len,
                        y: grass_y - 3.0 - 35.0 * roll_scale + 35.0,
                        m00: cos * roll_scale,
                        m01: sin * roll_scale,
                        m10: -sin * roll_scale,
                        m11: cos * roll_scale,
                        z: 3,
                        alpha: 1.0,
                        tint: [1.0; 3],
                        blend_mode: BlendMode::Alpha,
                        source: None,
                    });
                }
                push_title_load_bar_reanimations(&mut frame, state, &self.reanim_catalog);
                // The source enables hover as soon as resources are ready,
                // even while the bar still displays the loading label.
                let label_y = state.button_y() + 12.0;
                let (shadow_id, label_id) = if state.label_complete {
                    (
                        TITLE_START_PROMPT_SHADOW_IMAGE_ID,
                        if self.title_start_hovered() {
                            TITLE_START_PROMPT_HOVER_IMAGE_ID
                        } else {
                            TITLE_START_PROMPT_IMAGE_ID
                        },
                    )
                } else {
                    (
                        TITLE_LOADING_PROMPT_SHADOW_IMAGE_ID,
                        if self.title_start_hovered() {
                            TITLE_LOADING_PROMPT_HOVER_IMAGE_ID
                        } else {
                            TITLE_LOADING_PROMPT_IMAGE_ID
                        },
                    )
                };
                frame.sprites.push(SpriteCommand {
                    resource_id: shadow_id,
                    x: 341.0,
                    y: label_y + 1.0,
                    z: 5,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: label_id,
                    x: 340.0,
                    y: label_y,
                    z: 6,
                    scale: 1.0,
                    alpha: 1.0,
                });
            }
            SceneKind::AdventureSelect => {
                frame.sprites.push(SpriteCommand {
                    resource_id: SELECTOR_BASE_IMAGE_ID,
                    x: 0.0,
                    y: 0.0,
                    z: -3,
                    scale: 8.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: SELECTOR_CENTER_IMAGE_ID,
                    x: 80.0,
                    y: 250.0,
                    z: -2,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: SELECTOR_LEFT_IMAGE_ID,
                    x: 0.0,
                    y: 0.0,
                    z: -1,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: SELECTOR_RIGHT_IMAGE_ID,
                    x: 70.0,
                    y: 40.0,
                    z: 0,
                    scale: 1.0,
                    alpha: 1.0,
                });
                for (resource_id, x, y, z) in [
                    (SELECTOR_LEAVES_IMAGE_ID, 0.0, 538.0, 1),
                    (SELECTOR_TROPHY_IMAGE_ID, 10.0, 310.0, 2),
                    (SELECTOR_ZEN_GARDEN_IMAGE_ID, 171.0, 401.0, 2),
                    (SELECTOR_ALMANAC_IMAGE_ID, 327.0, 428.0, 2),
                    (SELECTOR_STORE_IMAGE_ID, 405.0, 482.0, 2),
                    (SELECTOR_OPTIONS_IMAGE_ID, 564.0, 474.0, 3),
                    (SELECTOR_HELP_IMAGE_ID, 646.0, 498.0, 3),
                    (SELECTOR_QUIT_IMAGE_ID, 714.0, 509.0, 3),
                    (SELECTOR_WOODSIGN1_IMAGE_ID, 20.0, 0.0, 4),
                    (SELECTOR_WOODSIGN2_IMAGE_ID, 35.0, 125.0, 4),
                    (SELECTOR_WOODSIGN3_IMAGE_ID, 35.0, 185.0, 4),
                ] {
                    frame.sprites.push(SpriteCommand {
                        resource_id,
                        x,
                        y,
                        z,
                        scale: 1.0,
                        alpha: 1.0,
                    });
                }
                for (resource_id, x, y, mode) in [
                    (
                        SELECTOR_ADVENTURE_IMAGE_ID,
                        405.0,
                        79.0,
                        ModeKind::Adventure,
                    ),
                    (SELECTOR_SURVIVAL_IMAGE_ID, 406.0, 173.0, ModeKind::Survival),
                    (
                        SELECTOR_CHALLENGES_IMAGE_ID,
                        410.0,
                        257.0,
                        ModeKind::MiniGame,
                    ),
                    (
                        SELECTOR_VASEBREAKER_IMAGE_ID,
                        413.0,
                        328.0,
                        ModeKind::Vasebreaker,
                    ),
                ] {
                    frame.sprites.push(SpriteCommand {
                        resource_id,
                        x,
                        y,
                        z: 5,
                        scale: 1.0,
                        // ponytail: alpha-only lock tint; the original greys the button art.
                        alpha: if self.can_open_mode(mode) { 1.0 } else { 0.5 },
                    });
                }
                if self.profile.is_some() {
                    for (digit, x, y) in selector_level_digits(
                        self.profile
                            .as_ref()
                            .map_or(1, |profile| profile.adventure_level),
                    ) {
                        frame.sprites.push(SpriteCommand {
                            resource_id: SELECTOR_LEVEL_NUMBER_BASE_IMAGE_ID + u32::from(digit),
                            x: 70.0 + x,
                            y: 40.0 + y,
                            z: 6,
                            scale: 1.0,
                            alpha: 1.0,
                        });
                    }
                }
            }
            SceneKind::AdventureTutorial => self.render_tutorial(&mut frame),
            SceneKind::ModeSelect => self.render_mode_select(&mut frame),
            SceneKind::SeedChooser => {
                let has_seven_rows = self.seed_chooser_has_seven_rows();
                frame.sprites.push(SpriteCommand {
                    resource_id: SCREEN_PIXEL_IMAGE_ID,
                    x: 0.0,
                    y: 0.0,
                    z: -1,
                    scale: 800.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: SEED_CHOOSER_IMAGE_ID,
                    x: 167.5,
                    y: 43.5,
                    z: 0,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: SEED_CHOOSER_TITLE_IMAGE_ID,
                    x: 290.0,
                    y: 94.0,
                    z: 2,
                    scale: 1.0,
                    alpha: 1.0,
                });
                for index in 0..seed_chooser_grid_slots(has_seven_rows) {
                    if let Some((x, y)) =
                        seed_chooser_slot_position(index, self.seed_chooser_scroll, has_seven_rows)
                    {
                        frame.sprites.push(SpriteCommand {
                            resource_id: SEED_PACKET_SILHOUETTE_IMAGE_ID,
                            x,
                            y,
                            z: 2,
                            scale: 1.0,
                            alpha: 1.0,
                        });
                    }
                }
                let choices = if self.game.state().challenge.kind == ChallengeKind::LastStand {
                    last_stand_seed_choices()
                } else {
                    adventure_seed_choices(
                        self.game.state().level,
                        self.game.state().adventure_first_time,
                    )
                };
                let selected_count = self
                    .seed_chooser_selection
                    .iter()
                    .filter(|selected| **selected)
                    .count();
                let mut selected_index = 0usize;
                for (slot, plant_type) in choices.iter().copied().enumerate() {
                    let (x, y) = if self
                        .seed_chooser_selection
                        .get(slot)
                        .copied()
                        .unwrap_or(false)
                    {
                        let position = selected_index;
                        selected_index += 1;
                        (288.0 + position as f32 * 53.0, 445.0)
                    } else {
                        let Some(position) = seed_chooser_slot_position(
                            slot,
                            self.seed_chooser_scroll,
                            has_seven_rows,
                        ) else {
                            continue;
                        };
                        position
                    };
                    frame.sprites.push(SpriteCommand {
                        resource_id: SEED_PACKET_NORMAL_IMAGE_ID,
                        x,
                        y,
                        z: 3,
                        scale: 1.0,
                        alpha: 1.0,
                    });
                    if let Some((resource_id, icon_x, icon_y, scale)) =
                        seed_chooser_plant_image(plant_type)
                    {
                        frame.sprites.push(SpriteCommand {
                            resource_id,
                            x: x + icon_x,
                            y: y + icon_y,
                            z: 4,
                            scale,
                            alpha: 1.0,
                        });
                    }
                }
                frame.sprites.push(SpriteCommand {
                    resource_id: SEED_CHOOSER_BUTTON_IMAGE_ID,
                    x: 322.0,
                    y: 535.0,
                    z: 3,
                    scale: 1.0,
                    alpha: if selected_count
                        == if self.game.state().challenge.kind == ChallengeKind::LastStand {
                            usize::from(last_stand_seed_slots(self.game.state().packet_upgrades))
                        } else {
                            usize::from(adventure_seed_slots(
                                self.game.state().level,
                                self.game.state().adventure_first_time,
                                self.game.state().packet_upgrades,
                            ))
                        } {
                        1.0
                    } else {
                        0.55
                    },
                });
            }
            SceneKind::Garden => {
                frame.sprites.push(SpriteCommand {
                    resource_id: if self.game.state().garden_service
                        == Some(GardenServiceKind::Mushroom)
                    {
                        GARDEN_MUSHROOM_BACKGROUND_IMAGE_ID
                    } else {
                        GARDEN_BACKGROUND_IMAGE_ID
                    },
                    x: 0.0,
                    y: 0.0,
                    z: 0,
                    scale: 1.0,
                    alpha: 1.0,
                });
                for (index, plant) in self.game.state().garden.plants.iter().enumerate() {
                    let x = 250.0 + index as f32 * 120.0;
                    let y = 300.0;
                    if let Some((resource_id, offset_x, offset_y, scale)) =
                        board_plant_image(plant.plant_type)
                    {
                        frame.sprites.push(SpriteCommand {
                            resource_id,
                            x: x + offset_x,
                            y: y + offset_y,
                            z: 2,
                            scale,
                            alpha: if plant.watered { 1.0 } else { 0.8 },
                        });
                    } else {
                        frame.sprites.push(SpriteCommand {
                            resource_id: UI_PIXEL_IMAGE_ID,
                            x: x - 32.0,
                            y: y - 32.0,
                            z: 2,
                            scale: 64.0 + (plant.age_ticks.min(100) as f32 * 0.2),
                            alpha: if plant.watered { 1.0 } else { 0.75 },
                        });
                    }
                    if let Some(resource_id) = garden_need_image(plant.need) {
                        frame.sprites.push(SpriteCommand {
                            resource_id: GARDEN_NEED_BUBBLE_IMAGE_ID,
                            x: x + 26.0,
                            y: y - 72.0,
                            z: 4,
                            scale: 0.8,
                            alpha: 1.0,
                        });
                        frame.sprites.push(SpriteCommand {
                            resource_id,
                            x: x + 36.0,
                            y: y - 62.0,
                            z: 5,
                            scale: 0.8,
                            alpha: 1.0,
                        });
                    }
                }
                for (tool, x) in [
                    (GardenTool::WateringCan, 80.0),
                    (GardenTool::Fertilizer, 240.0),
                    (GardenTool::BugSpray, 400.0),
                    (GardenTool::Phonograph, 560.0),
                ] {
                    let available = garden_tool_available(self.game.state(), tool);
                    frame.sprites.push(SpriteCommand {
                        resource_id: garden_tool_image(tool),
                        x,
                        y: 510.0,
                        z: 3,
                        scale: 0.8,
                        alpha: if !available {
                            0.25
                        } else if self.garden_tool == tool {
                            1.0
                        } else {
                            0.5
                        },
                    });
                }
                if self.game.state().garden_service == Some(GardenServiceKind::TreeOfWisdom) {
                    frame.sprites.push(SpriteCommand {
                        resource_id: UI_PIXEL_IMAGE_ID,
                        x: 360.0,
                        y: 120.0,
                        z: 2,
                        scale: 80.0 + f32::from(self.game.state().tree_height),
                        alpha: 1.0,
                    });
                }
            }
            scene @ (SceneKind::Day
            | SceneKind::Night
            | SceneKind::Pool
            | SceneKind::Fog
            | SceneKind::Roof
            | SceneKind::Boss
            | SceneKind::GameOver) => {
                frame.sprites.push(SpriteCommand {
                    resource_id: board_background_id(scene),
                    x: 0.0,
                    y: 0.0,
                    z: 0,
                    scale: 1.0,
                    alpha: 1.0,
                });
                self.render_board_hud(&mut frame);
                for vase in &self.game.state().board.vases {
                    frame.sprites.push(SpriteCommand {
                        resource_id: BOARD_VASE_BOTTOM_IMAGE_ID,
                        x: 80.0 + f32::from(vase.column) * 80.0 + 24.0,
                        y: board_row_y(vase.row) + 48.0,
                        z: 8,
                        scale: 0.7,
                        alpha: 1.0,
                    });
                    frame.sprites.push(SpriteCommand {
                        resource_id: BOARD_VASE_TOP_IMAGE_ID,
                        x: 80.0 + f32::from(vase.column) * 80.0 + 16.0,
                        y: board_row_y(vase.row) + 8.0,
                        z: 9,
                        scale: 0.7,
                        alpha: 1.0,
                    });
                }
                for crater in &self.game.state().board.craters {
                    frame.sprites.push(SpriteCommand {
                        resource_id: BOARD_CRATER_IMAGE_ID,
                        x: 80.0 + f32::from(crater.column) * 80.0 - 4.0,
                        y: board_row_y(crater.row) + 26.0,
                        z: 2,
                        scale: 0.72,
                        alpha: 1.0,
                    });
                }
                for grave in &self.game.state().board.graves {
                    frame.sprites.push(SpriteCommand {
                        resource_id: BOARD_GRAVE_IMAGE_ID,
                        x: 80.0 + f32::from(grave.column) * 80.0 - 12.0,
                        y: board_row_y(grave.row) - 6.0,
                        z: 4,
                        scale: 0.65,
                        alpha: 1.0,
                    });
                }
                for brain in &self.game.state().board.brains {
                    if !brain.squished {
                        let (x, y) = if self.game.state().challenge.kind
                            == ChallengeKind::Zombiquarium
                            && brain.position_x != 0
                        {
                            (
                                fixed_point_to_logical(brain.position_x) - 15.0,
                                fixed_point_to_logical(brain.position_y) - 15.0,
                            )
                        } else {
                            (20.0, board_row_y(brain.row) + 26.0)
                        };
                        frame.sprites.push(SpriteCommand {
                            resource_id: BOARD_BRAIN_IMAGE_ID,
                            x,
                            y,
                            z: 9,
                            scale: 1.0,
                            alpha: 1.0,
                        });
                    }
                }
                let tick = self.game.state().tick;
                for sun in &self.game.state().board.suns {
                    let position_x = fixed_point_to_logical(sun.position_x);
                    let position_y = fixed_point_to_logical(sun.position_y);
                    if !self.render_board_pickup_reanim(
                        &mut frame,
                        self.reanim_catalog.sun.as_ref(),
                        position_x + 30.0,
                        position_y + 30.0,
                        tick,
                    ) {
                        frame.sprites.push(SpriteCommand {
                            resource_id: BOARD_SUN_IMAGE_ID,
                            x: position_x - 22.0,
                            y: position_y - 22.0,
                            z: 6,
                            scale: 0.65,
                            alpha: 1.0,
                        });
                    }
                }
                for coin in &self.game.state().board.coins {
                    let reanim_drawn = if board_coin_reanim_visible(
                        coin.coin_type,
                        coin.target_y,
                        coin.from_present,
                    ) {
                        let definition = match coin.coin_type {
                            CoinType::Silver => self.reanim_catalog.coin_silver.as_ref(),
                            CoinType::Gold => self.reanim_catalog.coin_gold.as_ref(),
                            CoinType::Diamond => self.reanim_catalog.diamond.as_ref(),
                            _ => None,
                        };
                        let (offset_x, offset_y) = board_coin_reanim_offset(coin.coin_type)
                            .expect("visible pickup reanimation has a source offset");
                        self.render_board_pickup_reanim(
                            &mut frame,
                            definition,
                            fixed_point_to_logical(coin.position_x) + offset_x,
                            fixed_point_to_logical(coin.position_y) + offset_y,
                            tick,
                        )
                    } else {
                        false
                    };
                    if !reanim_drawn
                        && let Some((resource_id, scale)) = board_coin_image(coin.coin_type)
                    {
                        frame.sprites.push(SpriteCommand {
                            resource_id,
                            x: fixed_point_to_logical(coin.position_x) - 22.0,
                            y: fixed_point_to_logical(coin.position_y) - 22.0,
                            z: 6,
                            scale,
                            alpha: 1.0,
                        });
                    }
                    if coin.coin_type == CoinType::UsableSeedPacket
                        && let Some(plant_type) = coin.usable_seed_type
                        && let Some((resource_id, _, _, scale)) = board_plant_image(plant_type)
                    {
                        frame.sprites.push(SpriteCommand {
                            resource_id,
                            x: fixed_point_to_logical(coin.position_x) - 14.0,
                            y: fixed_point_to_logical(coin.position_y) - 14.0,
                            z: 7,
                            scale: scale * 0.55,
                            alpha: 1.0,
                        });
                    }
                }
                for zombie in &self.game.state().board.zombies {
                    let x = fixed_point_to_logical(zombie.position_x);
                    let y = if zombie.zombie_type == ZombieType::Boss {
                        0.0
                    } else if self.game.state().challenge.kind == ChallengeKind::Zombiquarium
                        && zombie.position_y != 0
                    {
                        fixed_point_to_logical(zombie.position_y)
                    } else {
                        board_row_y(zombie.row)
                    };
                    if !self.render_board_zombie_reanim(&mut frame, zombie, x, y)
                        && let Some(body_resource_id) = board_zombie_image(zombie.zombie_type)
                    {
                        frame.sprites.push(SpriteCommand {
                            resource_id: body_resource_id,
                            x: x - 34.0,
                            y: y + 18.0,
                            z: 7,
                            scale: 0.85,
                            alpha: 1.0,
                        });
                        frame.sprites.push(SpriteCommand {
                            resource_id: board_zombie_head_image(zombie.zombie_type),
                            x: x - 30.0,
                            y: y - 8.0,
                            z: 8,
                            scale: 0.75,
                            alpha: 1.0,
                        });
                        push_board_zombie_equipment(
                            &mut frame,
                            x,
                            y,
                            zombie.zombie_type,
                            zombie.armor_intact,
                        );
                    }
                }
                if self.game.state().challenge.kind == ChallengeKind::SeeingStars {
                    for &(row, column) in &SEEING_STARS_STARFRUIT_CELLS {
                        let has_starfruit = self.game.state().board.plants.iter().any(|plant| {
                            plant.row == row
                                && plant.column == column
                                && plant.plant_type == PlantType::Other(29)
                        });
                        if !has_starfruit {
                            frame.sprites.push(SpriteCommand {
                                resource_id: BOARD_STARFRUIT_IMAGE_ID,
                                x: 80.0 + f32::from(column) * 80.0 + 6.0,
                                y: board_row_y(row) + 16.0,
                                z: 9,
                                scale: 0.8,
                                alpha: 100.0 / 255.0,
                            });
                        }
                    }
                }
                if self.game.state().challenge.kind == ChallengeKind::BeghouledTwist
                    && self.game.state().challenge.twist_state
                        == neopvz_core::BeghouledTwistState::Normal
                    && let Some(position) = self.cursor_position
                    && let Some((cursor_x, cursor_y)) =
                        self.renderer.as_ref().and_then(|renderer| {
                            let size = renderer.window().inner_size();
                            logical_position(
                                size.width,
                                size.height,
                                position,
                                LogicalViewport::default(),
                            )
                        })
                {
                    let column = ((cursor_x - 80.0) / 80.0) as u8;
                    let row = ((cursor_y - 120.0) / 90.0) as u8;
                    let complete_square = column < 7
                        && row < 4
                        && (0..=1).all(|row_offset| {
                            (0..=1).all(|column_offset| {
                                self.game.state().board.plants.iter().any(|plant| {
                                    plant.row == row + row_offset
                                        && plant.column == column + column_offset
                                        && plant.health > 0
                                })
                            })
                        });
                    if complete_square {
                        let angle =
                            -(self.game.state().tick as f32) * 2.0 * std::f32::consts::PI * 0.01;
                        frame.affine_sprites.push(AffineSpriteCommand {
                            resource_id: BOARD_BEGHOULED_TWIST_OVERLAY_IMAGE_ID,
                            x: 80.0 + f32::from(column) * 80.0 + 80.0,
                            y: board_row_y(row) + 45.0,
                            m00: angle.cos(),
                            m01: -angle.sin(),
                            m10: angle.sin(),
                            m11: angle.cos(),
                            z: 11,
                            alpha: 0.5,
                            tint: [1.0; 3],
                            blend_mode: BlendMode::Alpha,
                            source: None,
                        });
                    }
                }
                for plant in &self.game.state().board.plants {
                    let (x, y) = if self.game.state().challenge.kind
                        == ChallengeKind::WallnutBowling
                        && matches!(plant.plant_type, PlantType::Other(3 | 49 | 50))
                    {
                        (
                            fixed_point_to_logical(plant.position_x) - 40.0,
                            board_row_y(plant.row) + fixed_point_to_logical(plant.position_y)
                                - (80.0 + f32::from(plant.row) * 100.0),
                        )
                    } else {
                        (
                            80.0 + f32::from(plant.column) * 80.0,
                            board_row_y(plant.row),
                        )
                    };
                    if !self.render_board_plant_reanim(&mut frame, plant, x, y) {
                        if let Some((resource_id, offset_x, offset_y, scale)) =
                            board_plant_image(plant.plant_type)
                        {
                            frame.sprites.push(SpriteCommand {
                                resource_id,
                                x: x + offset_x,
                                y: y + offset_y,
                                z: 10,
                                scale,
                                alpha: 1.0,
                            });
                        } else {
                            frame.sprites.push(SpriteCommand {
                                resource_id: UI_PIXEL_IMAGE_ID,
                                x,
                                y,
                                z: 10,
                                scale: 36.0,
                                alpha: 1.0,
                            });
                        }
                    }
                }
                for projectile in &self.game.state().board.projectiles {
                    if let Some((resource_id, x, y, scale_x, scale_y)) = board_projectile_shadow(
                        self.game.state().scene,
                        projectile.projectile_type,
                        projectile.row,
                        projectile.position_x,
                        projectile.shadow_y,
                        projectile.lob_height,
                    ) {
                        frame.affine_sprites.push(AffineSpriteCommand {
                            resource_id,
                            x,
                            y,
                            m00: scale_x,
                            m01: 0.0,
                            m10: 0.0,
                            m11: scale_y,
                            z: 11,
                            alpha: 1.0,
                            tint: [1.0; 3],
                            blend_mode: BlendMode::Alpha,
                            source: None,
                        });
                    }
                    if let Some((resource_id, scale)) =
                        board_projectile_image(projectile.projectile_type)
                    {
                        let y = board_projectile_y(
                            projectile.position_y,
                            projectile.row,
                            projectile.lob_height,
                        );
                        let scale =
                            board_projectile_scale(projectile.projectile_type, projectile.age)
                                .unwrap_or(scale);
                        let rotation =
                            board_projectile_rotation(projectile.projectile_type, projectile.age);
                        let (sin, cos) = rotation.sin_cos();
                        frame.affine_sprites.push(AffineSpriteCommand {
                            resource_id,
                            x: fixed_point_to_logical(projectile.position_x),
                            y,
                            m00: cos * scale,
                            m01: -sin * scale,
                            m10: sin * scale,
                            m11: cos * scale,
                            z: 12,
                            alpha: 1.0,
                            tint: [1.0; 3],
                            blend_mode: BlendMode::Alpha,
                            source: None,
                        });
                    }
                }
                self.render_board_effects(&mut frame);
                self.render_particles(&mut frame);
                if self.game.state().challenge.kind == ChallengeKind::LastStand
                    && !self.game.state().challenge.last_stand_onslaught
                {
                    frame.sprites.push(SpriteCommand {
                        resource_id: SEED_CHOOSER_BUTTON_IMAGE_ID,
                        x: 322.0,
                        y: 535.0,
                        z: 15,
                        scale: 1.0,
                        alpha: 1.0,
                    });
                }
                if self.game.state().paused {
                    frame.sprites.push(SpriteCommand {
                        resource_id: SCREEN_PIXEL_IMAGE_ID,
                        x: 0.0,
                        y: 0.0,
                        z: 20,
                        scale: 800.0,
                        alpha: 0.65,
                    });
                }
            }
            SceneKind::Complete => self.render_complete(&mut frame),
        }
        if self.store_open {
            self.render_store(&mut frame);
        }
        if self.options_open {
            self.render_options(&mut frame);
        }
        if self.almanac_open {
            self.render_almanac(&mut frame);
        }
        if self.help_open {
            self.render_help(&mut frame);
        }
        if self.game.state().scene == SceneKind::GameOver {
            self.render_game_over(&mut frame);
        }
        if is_board_scene(self.game.state().scene) && self.game.state().paused {
            self.render_pause(&mut frame);
        }
        frame
    }
}

fn is_board_scene(scene: SceneKind) -> bool {
    matches!(
        scene,
        SceneKind::Day
            | SceneKind::Night
            | SceneKind::Pool
            | SceneKind::Fog
            | SceneKind::Roof
            | SceneKind::Boss
    )
}

fn typing_code_character(key: KeyCode) -> char {
    match key {
        KeyCode::KeyA => 'a',
        KeyCode::KeyB => 'b',
        KeyCode::KeyC => 'c',
        KeyCode::KeyD => 'd',
        KeyCode::KeyE => 'e',
        KeyCode::KeyF => 'f',
        KeyCode::KeyG => 'g',
        KeyCode::KeyH => 'h',
        KeyCode::KeyI => 'i',
        KeyCode::KeyJ => 'j',
        KeyCode::KeyK => 'k',
        KeyCode::KeyL => 'l',
        KeyCode::KeyM => 'm',
        KeyCode::KeyN => 'n',
        KeyCode::KeyO => 'o',
        KeyCode::KeyP => 'p',
        KeyCode::KeyQ => 'q',
        KeyCode::KeyR => 'r',
        KeyCode::KeyS => 's',
        KeyCode::KeyT => 't',
        KeyCode::KeyU => 'u',
        KeyCode::KeyV => 'v',
        KeyCode::KeyW => 'w',
        KeyCode::KeyX => 'x',
        KeyCode::KeyY => 'y',
        KeyCode::KeyZ => 'z',
        _ => '\0',
    }
}

fn board_background_id(scene: SceneKind) -> u32 {
    match scene {
        SceneKind::Night => NIGHT_BACKGROUND_IMAGE_ID,
        SceneKind::Pool => POOL_BACKGROUND_IMAGE_ID,
        SceneKind::Fog => FOG_BACKGROUND_IMAGE_ID,
        SceneKind::Roof => ROOF_BACKGROUND_IMAGE_ID,
        SceneKind::Boss => BOSS_BACKGROUND_IMAGE_ID,
        _ => DAY_BACKGROUND_IMAGE_ID,
    }
}

fn progress_meter_width(state: &neopvz_core::GameState) -> Option<u32> {
    if state.scene == SceneKind::Boss {
        let Some(boss) = state
            .board
            .zombies
            .iter()
            .find(|zombie| zombie.zombie_type == ZombieType::Boss)
        else {
            return Some(150);
        };
        if boss.max_health <= 0 {
            return Some(150);
        }
        let damage = (boss.max_health - boss.health).max(0);
        return Some(
            (150 * u32::try_from(damage).unwrap_or(0)
                / u32::try_from(boss.max_health).unwrap_or(1))
            .clamp(1, 150),
        );
    }
    let wave = &state.board.wave;
    if wave.endless || wave.current == 0 || wave.total <= 1 || wave.countdown_start == 0 {
        return None;
    }
    let (total_width, waves_per_flag) =
        progress_meter_flag_layout(state).map_or((150, None), |(_, waves_per_flag)| {
            let flag_count = wave.total / waves_per_flag;
            (150 - 12 * flag_count, Some(waves_per_flag))
        });
    let denominator = wave.total - 1;
    let mut current_start = (wave.current.saturating_sub(1) * total_width) / denominator;
    let mut next_start = wave.current * total_width / denominator;
    if let Some(waves_per_flag) = waves_per_flag {
        let extra = wave.current / waves_per_flag * 12;
        current_start += extra;
        next_start += extra;
    }
    let elapsed = wave
        .countdown_start
        .saturating_sub(wave.countdown)
        .min(wave.countdown_start);
    let span = next_start.saturating_sub(current_start);
    Some((current_start + span * elapsed / wave.countdown_start).clamp(1, 150))
}

fn progress_meter_flag_layout(state: &neopvz_core::GameState) -> Option<(u32, u32)> {
    let waves = state.board.wave.total;
    if state.mode == ModeKind::Adventure {
        let flag_count = adventure_flag_wave_count(state.level, !state.adventure_first_time);
        (flag_count > 0).then_some((flag_count, waves / flag_count))
    } else if state.mode == ModeKind::Survival {
        let waves_per_flag = 10;
        let flag_count = waves / waves_per_flag;
        (flag_count > 0).then_some((flag_count, waves_per_flag))
    } else {
        None
    }
}

fn has_conveyor_seed_bank(state: &neopvz_core::GameState) -> bool {
    (state.mode == ModeKind::MiniGame
        && matches!(
            state.challenge.kind,
            ChallengeKind::WallnutBowling
                | ChallengeKind::Invisighoul
                | ChallengeKind::LittleTrouble
                | ChallengeKind::PortalCombat
                | ChallengeKind::Column
                | ChallengeKind::FinalBoss
        ))
        || (state.adventure_configured
            && state.mode == ModeKind::Adventure
            && adventure_level_is_conveyor(state.level))
}

fn seed_bank_extra_width(packet_count: usize) -> u32 {
    match packet_count {
        0..=6 => 0,
        7 => 60,
        8 => 76,
        9 => 112,
        _ => 153,
    }
}

fn seed_packet_cost(plant_type: PlantType) -> u32 {
    match plant_type {
        PlantType::ZombiquariumSnorkel => 100,
        PlantType::ZombiquariumTrophy => 1_000,
        _ => plant_type.cost(),
    }
}

fn seed_bank_plant_image(plant_type: PlantType) -> Option<(u32, f32, f32, f32)> {
    if let Some((resource_id, _, _, _)) = seed_chooser_plant_image(plant_type)
        && (SEED_PACKET_PLANT_BASE_IMAGE_ID..SEED_PACKET_PLANT_BASE_IMAGE_ID + 13)
            .contains(&resource_id)
    {
        return Some((resource_id, 0.0, 0.0, 1.0));
    }
    match plant_type {
        PlantType::Peashooter => Some((SEED_PEASHOOTER_IMAGE_ID, 5.0, 8.0, 0.5)),
        PlantType::Sunflower => Some((SEED_SUNFLOWER_IMAGE_ID, 5.0, 8.0, 0.5)),
        PlantType::Other(5) => Some((BOARD_SNOWPEA_IMAGE_ID, 5.0, 8.0, 0.5)),
        PlantType::Other(8) => Some((BOARD_PUFFSHROOM_IMAGE_ID, 8.0, 12.0, 0.4)),
        PlantType::Other(10) => Some((BOARD_FUMESHROOM_IMAGE_ID, 8.0, 12.0, 0.4)),
        PlantType::Other(29) => Some((BOARD_STARFRUIT_IMAGE_ID, 6.0, 8.0, 0.5)),
        PlantType::Other(21) => Some((BOARD_WALLNUT_IMAGE_ID, 5.0, 8.0, 0.5)),
        PlantType::Other(31) => Some((BOARD_MAGNETSHROOM_IMAGE_ID, 5.0, 12.0, 0.5)),
        _ => None,
    }
}

fn seed_packet_position_x(
    index: usize,
    packet_count: usize,
    conveyor: bool,
    slot_machine: bool,
) -> f32 {
    if slot_machine {
        247.0 + index as f32 * 59.0
    } else if conveyor {
        91.0 + index as f32 * 50.0
    } else {
        let step = match packet_count {
            0..=7 => 59.0,
            8 => 54.0,
            9 => 52.0,
            _ => 51.0,
        };
        let start = match packet_count {
            0..=7 => 85.0,
            8 => 81.0,
            9 => 80.0,
            _ => 79.0,
        };
        start + index as f32 * step
    }
}

fn seed_bank_extension_image(packet_count: usize) -> Option<(u32, f32)> {
    match packet_count {
        7 => Some((BOARD_SEED_BANK_EXTENSION_BASE_IMAGE_ID, 434.0)),
        8 => Some((BOARD_SEED_BANK_EXTENSION_BASE_IMAGE_ID + 1, 434.0)),
        9 => Some((BOARD_SEED_BANK_EXTENSION_BASE_IMAGE_ID + 2, 434.0)),
        10 => Some((BOARD_SEED_BANK_EXTENSION_BASE_IMAGE_ID + 3, 434.0)),
        _ => None,
    }
}

fn board_seed_packet_at(state: &neopvz_core::GameState, x: f32, y: f32) -> Option<u8> {
    let packet_count = state.board.seed_packets.len().min(10);
    let conveyor = has_conveyor_seed_bank(state);
    let slot_machine = state.challenge.kind == ChallengeKind::SlotMachine;
    state
        .board
        .seed_packets
        .iter()
        .take(10)
        .enumerate()
        .find(|(index, _)| {
            let packet_x = seed_packet_position_x(*index, packet_count, conveyor, slot_machine);
            (packet_x..packet_x + 50.0).contains(&x) && (0.0..70.0).contains(&y)
        })
        .map(|(_, packet)| packet.slot)
}

fn board_plant_image(plant_type: PlantType) -> Option<(u32, f32, f32, f32)> {
    match plant_type {
        PlantType::Peashooter => Some((SEED_PEASHOOTER_IMAGE_ID, 10.0, 22.0, 0.8)),
        PlantType::Sunflower => Some((SEED_SUNFLOWER_IMAGE_ID, 12.0, 28.0, 0.8)),
        PlantType::Other(5) => Some((BOARD_SNOWPEA_IMAGE_ID, 10.0, 22.0, 0.8)),
        PlantType::Other(8) => Some((BOARD_PUFFSHROOM_IMAGE_ID, 16.0, 34.0, 0.9)),
        PlantType::Other(10) => Some((BOARD_FUMESHROOM_IMAGE_ID, 6.0, 24.0, 0.7)),
        PlantType::Other(29) => Some((BOARD_STARFRUIT_IMAGE_ID, 6.0, 16.0, 0.8)),
        PlantType::Other(21) => Some((BOARD_WALLNUT_IMAGE_ID, 8.0, 12.0, 0.8)),
        PlantType::Other(31) => Some((BOARD_MAGNETSHROOM_IMAGE_ID, 11.0, 18.0, 0.8)),
        _ => None,
    }
}

fn seed_chooser_plant_image(plant_type: PlantType) -> Option<(u32, f32, f32, f32)> {
    let packet_plant = match plant_type {
        PlantType::Other(4) => Some(0),
        PlantType::Other(6) => Some(1),
        PlantType::Other(12) => Some(2),
        PlantType::Other(23) => Some(3),
        PlantType::Other(27) => Some(4),
        PlantType::Other(30) => Some(5),
        PlantType::Other(41) => Some(6),
        PlantType::Other(47) => Some(7),
        PlantType::Other(32) => Some(8),
        PlantType::Other(34) => Some(9),
        PlantType::Other(39) => Some(10),
        PlantType::Other(44) => Some(11),
        PlantType::Other(46) => Some(12),
        _ => None,
    };
    packet_plant
        .map(|index| (SEED_PACKET_PLANT_BASE_IMAGE_ID + index, 0.0, 0.0, 1.0))
        .or_else(|| board_plant_image(plant_type))
}

fn garden_tool_image(tool: GardenTool) -> u32 {
    match tool {
        GardenTool::WateringCan => GARDEN_WATERING_CAN_IMAGE_ID,
        GardenTool::Fertilizer => GARDEN_FERTILIZER_IMAGE_ID,
        GardenTool::BugSpray => GARDEN_BUG_SPRAY_IMAGE_ID,
        GardenTool::Phonograph => GARDEN_PHONOGRAPH_IMAGE_ID,
    }
}

fn garden_tool_available(state: &neopvz_core::GameState, tool: GardenTool) -> bool {
    match tool {
        GardenTool::WateringCan => true,
        GardenTool::Fertilizer => state.fertilizer_charges > 0,
        GardenTool::BugSpray => state.bug_spray_charges > 0,
        GardenTool::Phonograph => state.phonograph_purchased,
    }
}

fn garden_need_image(need: GardenNeed) -> Option<u32> {
    match need {
        GardenNeed::None => None,
        GardenNeed::Water => Some(GARDEN_WATERDROP_IMAGE_ID),
        GardenNeed::Fertilizer => Some(GARDEN_NEED_FERTILIZER_IMAGE_ID),
        GardenNeed::BugSpray => Some(GARDEN_NEED_BUG_SPRAY_IMAGE_ID),
        GardenNeed::Phonograph => Some(GARDEN_NEED_PHONOGRAPH_IMAGE_ID),
    }
}

const STORE_ITEM_LAYOUT: [(StoreItem, f32, f32); 5] = [
    (StoreItem::PacketUpgrade, 100.0, 165.0),
    (StoreItem::Fertilizer, 250.0, 165.0),
    (StoreItem::BugSpray, 400.0, 165.0),
    (StoreItem::Phonograph, 550.0, 165.0),
    (StoreItem::Stinky, 330.0, 330.0),
];

fn store_item_at_slot(slot: usize) -> Option<StoreItem> {
    STORE_ITEM_LAYOUT.get(slot).map(|(item, _, _)| *item)
}

fn store_item_at(x: f32, y: f32) -> Option<StoreItem> {
    STORE_ITEM_LAYOUT.iter().find_map(|(item, item_x, item_y)| {
        ((*item_x - 40.0..*item_x + 125.0).contains(&x)
            && (item_y - 20.0..*item_y + 145.0).contains(&y))
        .then_some(*item)
    })
}

fn store_item_image(item: StoreItem) -> (u32, f32) {
    match item {
        StoreItem::PacketUpgrade => (STORE_PACKET_UPGRADE_IMAGE_ID, 0.85),
        StoreItem::Fertilizer => (GARDEN_FERTILIZER_IMAGE_ID, 0.7),
        StoreItem::BugSpray => (GARDEN_BUG_SPRAY_IMAGE_ID, 0.7),
        StoreItem::Phonograph => (GARDEN_PHONOGRAPH_IMAGE_ID, 0.65),
        StoreItem::Stinky => (STORE_STINKY_IMAGE_ID, 0.8),
    }
}

fn store_item_name_image(item: StoreItem) -> u32 {
    STORE_ITEM_NAME_BASE_IMAGE_ID
        + match item {
            StoreItem::PacketUpgrade => 0,
            StoreItem::Fertilizer => 1,
            StoreItem::BugSpray => 2,
            StoreItem::Phonograph => 3,
            StoreItem::Stinky => 4,
        }
}

fn store_item_price_image(item: StoreItem, packet_upgrades: u8) -> u32 {
    STORE_ITEM_PRICE_BASE_IMAGE_ID
        + match item {
            StoreItem::PacketUpgrade => u32::from(packet_upgrades.min(3)),
            StoreItem::Fertilizer => 4,
            StoreItem::BugSpray => 5,
            StoreItem::Phonograph => 6,
            StoreItem::Stinky => 7,
        }
}

fn store_item_sold_out(state: &neopvz_core::GameState, item: StoreItem) -> bool {
    match item {
        StoreItem::PacketUpgrade => state.packet_upgrades >= 4,
        StoreItem::Fertilizer => state.fertilizer_charges > 15,
        StoreItem::BugSpray => state.bug_spray_charges > 15,
        StoreItem::Phonograph => state.phonograph_purchased,
        StoreItem::Stinky => state.stinky_purchased,
    }
}

fn help_selector_contains(x: f32, y: f32) -> bool {
    (685.0..790.0).contains(&x) && (445.0..570.0).contains(&y)
}

fn help_button_contains(x: f32, y: f32) -> bool {
    ((324.0..480.0).contains(&x) && (500.0..562.0).contains(&y))
        || ((677.0..788.0).contains(&x) && (16.0..42.0).contains(&y))
}

const SEED_CHOOSER_COLUMN_COUNT: usize = 8;
const SEED_CHOOSER_VISIBLE_ROW_COUNT: usize = 6;
const SEED_CHOOSER_COLUMN_SPACING: f32 = 53.0;
const SEED_CHOOSER_CARD_WIDTH: f32 = 50.0;
const SEED_CHOOSER_CARD_HEIGHT: f32 = 70.0;
const SEED_CHOOSER_SLOT_LEFT: f32 = 189.5;
const SEED_CHOOSER_DEFAULT_SLOT_TOP: f32 = 171.5;
const SEED_CHOOSER_SEVEN_ROWS_SLOT_TOP: f32 = 166.5;

fn seed_chooser_has_seven_rows_at(finished: bool, unlocked: &[PlantType]) -> bool {
    finished
        || unlocked.iter().any(|plant_type| match plant_type {
            PlantType::Other(slot) => (40..=45).contains(slot),
            _ => false,
        })
}

fn seed_chooser_grid_slots(has_seven_rows: bool) -> usize {
    if has_seven_rows { 48 } else { 40 }
}

fn seed_chooser_slot_position(
    index: usize,
    scroll: usize,
    has_seven_rows: bool,
) -> Option<(f32, f32)> {
    if index >= seed_chooser_grid_slots(has_seven_rows) {
        return None;
    }
    let row = index / SEED_CHOOSER_COLUMN_COUNT;
    if row < scroll || row >= scroll + SEED_CHOOSER_VISIBLE_ROW_COUNT {
        return None;
    }
    let column = index % SEED_CHOOSER_COLUMN_COUNT;
    let top = if has_seven_rows {
        SEED_CHOOSER_SEVEN_ROWS_SLOT_TOP
    } else {
        SEED_CHOOSER_DEFAULT_SLOT_TOP
    };
    let row_height = if has_seven_rows { 70.0 } else { 73.0 };
    Some((
        SEED_CHOOSER_SLOT_LEFT + column as f32 * SEED_CHOOSER_COLUMN_SPACING,
        top + (row - scroll) as f32 * row_height,
    ))
}

fn seed_chooser_slot_at(x: f32, y: f32, scroll: usize, has_seven_rows: bool) -> Option<usize> {
    let top = if has_seven_rows {
        SEED_CHOOSER_SEVEN_ROWS_SLOT_TOP
    } else {
        SEED_CHOOSER_DEFAULT_SLOT_TOP
    };
    let row_height = if has_seven_rows { 70.0 } else { 73.0 };
    let local_x = x - SEED_CHOOSER_SLOT_LEFT;
    let local_y = y - top;
    if local_x < 0.0
        || local_y < 0.0
        || local_x
            >= (SEED_CHOOSER_COLUMN_COUNT - 1) as f32 * SEED_CHOOSER_COLUMN_SPACING
                + SEED_CHOOSER_CARD_WIDTH
        || local_y
            >= (SEED_CHOOSER_VISIBLE_ROW_COUNT - 1) as f32 * row_height + SEED_CHOOSER_CARD_HEIGHT
    {
        return None;
    }
    let column = (local_x / SEED_CHOOSER_COLUMN_SPACING).floor() as usize;
    let row_in_view = (local_y / row_height).floor() as usize;
    if local_x - column as f32 * SEED_CHOOSER_COLUMN_SPACING >= SEED_CHOOSER_CARD_WIDTH
        || local_y - row_in_view as f32 * row_height >= SEED_CHOOSER_CARD_HEIGHT
    {
        return None;
    }
    let row = row_in_view + scroll;
    let index = row * SEED_CHOOSER_COLUMN_COUNT + column;
    (index < seed_chooser_grid_slots(has_seven_rows)).then_some(index)
}

const ALMANAC_PLANT_COUNT: u8 = 49;
const ALMANAC_ZOMBIE_COUNT: u8 = 26;
const ALMANAC_IMITATER_SLOT: u8 = 48;

fn almanac_zombie_spawned_only(zombie_type: ZombieType) -> bool {
    matches!(
        zombie_type,
        ZombieType::BackupDancer | ZombieType::Bobsled | ZombieType::Imp
    )
}

fn almanac_can_spawn_yetis_at(level: u8, adventure_rounds: u8, finished: bool) -> bool {
    let (_, starting_level, _, _) = zombie_wave_stats(ZombieType::Yeti);
    finished && (adventure_rounds >= 2 || level >= starting_level)
}

fn almanac_zombie_has_silhouette_at(
    level: u8,
    adventure_rounds: u8,
    finished: bool,
    zombie_type: ZombieType,
) -> bool {
    if zombie_type != ZombieType::Yeti
        || almanac_can_spawn_yetis_at(level, adventure_rounds, finished)
    {
        return false;
    }
    let (_, starting_level, _, _) = zombie_wave_stats(ZombieType::Yeti);
    finished || level > starting_level
}

fn almanac_zombie_is_shown_at(
    level: u8,
    adventure_rounds: u8,
    finished: bool,
    defeated: bool,
    zombie_type: ZombieType,
) -> bool {
    if zombie_type == ZombieType::Yeti {
        return almanac_can_spawn_yetis_at(level, adventure_rounds, finished)
            || almanac_zombie_has_silhouette_at(level, adventure_rounds, finished, zombie_type);
    }
    if finished {
        return true;
    }
    let (_, starting_level, _, _) = zombie_wave_stats(zombie_type);
    starting_level <= level
        && (starting_level != level || !almanac_zombie_spawned_only(zombie_type) || defeated)
}

fn almanac_zombie_has_description_at(
    level: u8,
    adventure_rounds: u8,
    finished: bool,
    defeated: bool,
    zombie_type: ZombieType,
) -> bool {
    if zombie_type == ZombieType::Yeti {
        if !almanac_can_spawn_yetis_at(level, adventure_rounds, finished) {
            return false;
        }
        if adventure_rounds >= 2 {
            return true;
        }
    } else if finished {
        return true;
    }

    let (_, starting_level, _, _) = zombie_wave_stats(zombie_type);
    starting_level <= level && (starting_level != level || defeated)
}

fn almanac_plant_string_key(slot: u8) -> &'static str {
    [
        "PEASHOOTER",
        "SUNFLOWER",
        "CHERRY_BOMB",
        "WALL_NUT",
        "POTATO_MINE",
        "SNOW_PEA",
        "CHOMPER",
        "REPEATER",
        "PUFF_SHROOM",
        "SUN_SHROOM",
        "FUME_SHROOM",
        "GRAVE_BUSTER",
        "HYPNO_SHROOM",
        "SCAREDY_SHROOM",
        "ICE_SHROOM",
        "DOOM_SHROOM",
        "LILY_PAD",
        "SQUASH",
        "THREEPEATER",
        "TANGLE_KELP",
        "JALAPENO",
        "SPIKEWEED",
        "TORCHWOOD",
        "TALL_NUT",
        "SEA_SHROOM",
        "PLANTERN",
        "CACTUS",
        "BLOVER",
        "SPLIT_PEA",
        "STARFRUIT",
        "PUMPKIN",
        "MAGNET_SHROOM",
        "CABBAGE_PULT",
        "FLOWER_POT",
        "KERNEL_PULT",
        "COFFEE_BEAN",
        "GARLIC",
        "UMBRELLA_LEAF",
        "MARIGOLD",
        "MELON_PULT",
        "GATLING_PEA",
        "TWIN_SUNFLOWER",
        "GLOOM_SHROOM",
        "CATTAIL",
        "WINTER_MELON",
        "GOLD_MAGNET",
        "SPIKEROCK",
        "COB_CANNON",
        "IMITATER",
    ]
    .get(usize::from(slot))
    .copied()
    .unwrap_or("PLANT")
}

fn almanac_zombie_string_key(index: u8) -> &'static str {
    [
        "ZOMBIE",
        "FLAG_ZOMBIE",
        "CONEHEAD_ZOMBIE",
        "POLE_VAULTING_ZOMBIE",
        "BUCKETHEAD_ZOMBIE",
        "NEWSPAPER_ZOMBIE",
        "SCREEN_DOOR_ZOMBIE",
        "FOOTBALL_ZOMBIE",
        "DANCING_ZOMBIE",
        "BACKUP_DANCER",
        "DUCKY_TUBE_ZOMBIE",
        "SNORKEL_ZOMBIE",
        "ZOMBONI",
        "ZOMBIE_BOBSLED_TEAM",
        "DOLPHIN_RIDER_ZOMBIE",
        "JACK_IN_THE_BOX_ZOMBIE",
        "BALLOON_ZOMBIE",
        "DIGGER_ZOMBIE",
        "POGO_ZOMBIE",
        "ZOMBIE_YETI",
        "BUNGEE_ZOMBIE",
        "LADDER_ZOMBIE",
        "CATAPULT_ZOMBIE",
        "GARGANTUAR",
        "IMP",
        "BOSS",
    ]
    .get(usize::from(index))
    .copied()
    .unwrap_or("ZOMBIE")
}

fn almanac_plant_name(slot: u8) -> &'static str {
    [
        "Peashooter",
        "Sunflower",
        "Cherry Bomb",
        "Wall-nut",
        "Potato Mine",
        "Snow Pea",
        "Chomper",
        "Repeater",
        "Puff-shroom",
        "Sun-shroom",
        "Fume-shroom",
        "Grave Buster",
        "Hypno-shroom",
        "Scaredy-shroom",
        "Ice-shroom",
        "Doom-shroom",
        "Lily Pad",
        "Squash",
        "Threepeater",
        "Tangle Kelp",
        "Jalapeno",
        "Spikeweed",
        "Torchwood",
        "Tall-nut",
        "Sea-shroom",
        "Plantern",
        "Cactus",
        "Blover",
        "Split Pea",
        "Starfruit",
        "Pumpkin",
        "Magnet-shroom",
        "Cabbage-pult",
        "Flower Pot",
        "Kernel-pult",
        "Coffee Bean",
        "Garlic",
        "Umbrella Leaf",
        "Marigold",
        "Melon-pult",
        "Gatling Pea",
        "Twin Sunflower",
        "Gloom-shroom",
        "Cattail",
        "Winter Melon",
        "Gold Magnet",
        "Spikerock",
        "Cob Cannon",
        "Imitater",
    ]
    .get(usize::from(slot))
    .copied()
    .unwrap_or("Unknown Plant")
}

fn almanac_zombie_name(index: u8) -> &'static str {
    [
        "Zombie",
        "Flag Zombie",
        "Conehead Zombie",
        "Pole Vaulting Zombie",
        "Buckethead Zombie",
        "Newspaper Zombie",
        "Screen Door Zombie",
        "Football Zombie",
        "Dancing Zombie",
        "Backup Dancer",
        "Ducky Tube Zombie",
        "Snorkel Zombie",
        "Zomboni",
        "Zombie Bobsled Team",
        "Dolphin Rider Zombie",
        "Jack-in-the-Box Zombie",
        "Balloon Zombie",
        "Digger Zombie",
        "Pogo Zombie",
        "Zombie Yeti",
        "Bungee Zombie",
        "Ladder Zombie",
        "Catapult Zombie",
        "Gargantuar",
        "Imp",
        "Dr. Zomboss",
    ]
    .get(usize::from(index))
    .copied()
    .unwrap_or("Unknown Zombie")
}

fn almanac_plant_type(slot: u8) -> Option<PlantType> {
    match slot {
        0 => Some(PlantType::Peashooter),
        1 => Some(PlantType::Sunflower),
        2..ALMANAC_PLANT_COUNT => Some(PlantType::Other(slot)),
        _ => None,
    }
}

fn almanac_plant_position(slot: u8) -> (f32, f32) {
    if slot == ALMANAC_IMITATER_SLOT {
        return (20.0, 23.0);
    }
    (
        26.0 + f32::from(slot % 8) * 52.0,
        92.0 + f32::from(slot / 8) * 78.0,
    )
}

fn almanac_plant_slot_at(x: f32, y: f32) -> Option<u8> {
    (0..ALMANAC_PLANT_COUNT).find(|slot| {
        let (slot_x, slot_y) = almanac_plant_position(*slot);
        let (width, height) = if *slot == ALMANAC_IMITATER_SLOT {
            (34.0, 46.0)
        } else {
            (50.0, 70.0)
        };
        (slot_x..slot_x + width).contains(&x) && (slot_y..slot_y + height).contains(&y)
    })
}

fn almanac_plant_icon(plant_type: PlantType) -> Option<(u32, f32)> {
    board_plant_image(plant_type).map(|(resource_id, _, _, scale)| (resource_id, scale * 0.6))
}

fn almanac_zombie_type(index: u8) -> Option<ZombieType> {
    Some(match index {
        0 => ZombieType::Normal,
        1 => ZombieType::Flag,
        2 => ZombieType::Conehead,
        3 => ZombieType::PoleVaulter,
        4 => ZombieType::Buckethead,
        5 => ZombieType::Newspaper,
        6 => ZombieType::ScreenDoor,
        7 => ZombieType::Football,
        8 => ZombieType::Dancer,
        9 => ZombieType::BackupDancer,
        10 => ZombieType::DuckyTube,
        11 => ZombieType::Snorkel,
        12 => ZombieType::Zamboni,
        13 => ZombieType::Bobsled,
        14 => ZombieType::DolphinRider,
        15 => ZombieType::Jackbox,
        16 => ZombieType::Balloon,
        17 => ZombieType::Digger,
        18 => ZombieType::Pogo,
        19 => ZombieType::Yeti,
        20 => ZombieType::Bungee,
        21 => ZombieType::Ladder,
        22 => ZombieType::Catapult,
        23 => ZombieType::Gargantuar,
        24 => ZombieType::Imp,
        25 => ZombieType::Boss,
        _ => return None,
    })
}

fn almanac_zombie_position(index: u8) -> (f32, f32) {
    if almanac_zombie_type(index) == Some(ZombieType::Boss) {
        return (192.0, 486.0);
    }
    (
        22.0 + f32::from(index % 5) * 85.0,
        86.0 + f32::from(index / 5) * 80.0,
    )
}

fn almanac_zombie_index_at(x: f32, y: f32) -> Option<u8> {
    (0..ALMANAC_ZOMBIE_COUNT).find(|index| {
        let (slot_x, slot_y) = almanac_zombie_position(*index);
        (slot_x..slot_x + 76.0).contains(&x) && (slot_y..slot_y + 76.0).contains(&y)
    })
}

fn almanac_close_contains(x: f32, y: f32) -> bool {
    (676.0..765.0).contains(&x) && (567.0..593.0).contains(&y)
}

fn almanac_index_contains(x: f32, y: f32) -> bool {
    (32.0..196.0).contains(&x) && (567.0..593.0).contains(&y)
}

fn almanac_plant_contains(x: f32, y: f32) -> bool {
    (130.0..286.0).contains(&x) && (345.0..387.0).contains(&y)
}

fn almanac_zombie_contains(x: f32, y: f32) -> bool {
    (487.0..697.0).contains(&x) && (345.0..393.0).contains(&y)
}

fn game_over_try_again_contains(x: f32, y: f32) -> bool {
    (220.0..480.0).contains(&x) && (388.0..450.0).contains(&y)
}

fn game_over_main_menu_contains(x: f32, y: f32) -> bool {
    (595.0..759.0).contains(&x) && (140.0..191.0).contains(&y)
}

fn complete_continue_contains(x: f32, y: f32) -> bool {
    (324.0..480.0).contains(&x) && (500.0..542.0).contains(&y)
}

fn complete_main_menu_contains(x: f32, y: f32) -> bool {
    (677.0..788.0).contains(&x) && (16.0..42.0).contains(&y)
}

fn is_completion_award_coin(coin: CoinType) -> bool {
    matches!(
        coin,
        CoinType::FinalSeedPacket
            | CoinType::Trophy
            | CoinType::Shovel
            | CoinType::Almanac
            | CoinType::CarKeys
            | CoinType::Vase
            | CoinType::WateringCan
            | CoinType::Taco
            | CoinType::Note
            | CoinType::AwardMoneyBag
            | CoinType::AwardBagDiamond
            | CoinType::AwardSilverSunflower
            | CoinType::AwardGoldSunflower
            | CoinType::AwardPresent
            | CoinType::AwardChocolate
    )
}

fn completion_award_coin(state: &neopvz_core::GameState) -> Option<CoinType> {
    state
        .board
        .coins
        .iter()
        .rev()
        .map(|coin| coin.coin_type)
        .find(|coin| is_completion_award_coin(*coin))
        .or_else(|| {
            state
                .pickup_inventory
                .iter()
                .rev()
                .copied()
                .find(|coin| is_completion_award_coin(*coin))
        })
        .or_else(|| {
            if state.mode == ModeKind::Adventure {
                adventure_completion_award(state.level, state.adventure_first_time)
            } else {
                None
            }
        })
}

fn completion_award_text_slot(reward: Option<CoinType>) -> u32 {
    match reward {
        Some(CoinType::FinalSeedPacket) => 0,
        Some(CoinType::Shovel) => 1,
        Some(CoinType::Almanac) => 2,
        Some(CoinType::CarKeys) => 3,
        Some(CoinType::Taco) => 4,
        Some(CoinType::WateringCan) => 5,
        Some(CoinType::Note) => 6,
        Some(CoinType::Trophy | CoinType::AwardSilverSunflower | CoinType::AwardGoldSunflower) => 7,
        _ => 8,
    }
}

fn completion_note_asset(level: u8) -> u32 {
    match level {
        9 => AWARD_NOTE1_IMAGE_ID,
        19 => AWARD_NOTE2_IMAGE_ID,
        29 => AWARD_NOTE3_IMAGE_ID,
        39 => AWARD_NOTE4_IMAGE_ID,
        _ => {
            // ponytail: final note art is absent from the supplied resources; use the last page.
            AWARD_NOTE4_IMAGE_ID
        }
    }
}

fn completion_award_asset(
    _state: &neopvz_core::GameState,
    reward: Option<CoinType>,
) -> Option<(u32, f32, f32, f32)> {
    let resource_id = match reward {
        Some(CoinType::Shovel) => AWARD_SHOVEL_IMAGE_ID,
        Some(CoinType::Almanac) => AWARD_ALMANAC_IMAGE_ID,
        Some(CoinType::CarKeys) => AWARD_CAR_KEYS_IMAGE_ID,
        Some(CoinType::Taco) => AWARD_TACO_IMAGE_ID,
        Some(CoinType::WateringCan) => AWARD_WATERING_CAN_IMAGE_ID,
        Some(CoinType::Note | CoinType::FinalSeedPacket) => return None,
        Some(
            CoinType::Trophy
            | CoinType::AwardMoneyBag
            | CoinType::AwardBagDiamond
            | CoinType::AwardSilverSunflower
            | CoinType::AwardGoldSunflower
            | CoinType::AwardPresent
            | CoinType::AwardChocolate,
        )
        | None => AWARD_TROPHY_IMAGE_ID,
        _ => AWARD_TROPHY_IMAGE_ID,
    };
    Some((resource_id, 317.0, 118.0, 0.55))
}

fn volume_percent_to_decibels(percent: u8) -> f32 {
    if percent == 0 {
        -60.0
    } else {
        20.0 * (f32::from(percent) / 100.0).log10()
    }
}

fn board_row_y(row: u8) -> f32 {
    120.0 + f32::from(row) * 90.0
}

fn jalapeno_fire_effect_positions(scene: SceneKind, row: u8) -> [(f32, f32); 12] {
    std::array::from_fn(|index| {
        let x = 750.0 * index as f32 / 11.0 + 10.0;
        let source_x = x + 10.0;
        let roof_offset = if scene == SceneKind::Roof && source_x < 440.0 {
            (440.0 - source_x) * 0.25
        } else {
            0.0
        };
        (x, board_row_y(row) + roof_offset - 10.0)
    })
}

fn board_zombie_image(zombie_type: ZombieType) -> Option<u32> {
    (!matches!(zombie_type, ZombieType::Boss)).then_some(BOARD_ZOMBIE_BODY_IMAGE_ID)
}

fn board_zombie_reanim_definition(
    catalog: &ReanimCatalog,
    zombie_type: ZombieType,
) -> Option<(&ReanimatorDefinition, bool)> {
    match zombie_type {
        ZombieType::Football => catalog
            .football
            .as_ref()
            .map(|definition| (definition, true)),
        ZombieType::Newspaper => catalog
            .newspaper
            .as_ref()
            .map(|definition| (definition, true)),
        ZombieType::Normal
        | ZombieType::Flag
        | ZombieType::Conehead
        | ZombieType::Buckethead
        | ZombieType::ScreenDoor
        | ZombieType::DuckyTube => catalog
            .zombie
            .as_ref()
            .map(|definition| (definition, false)),
        _ => catalog
            .specialized
            .iter()
            .find(|(known_type, _)| {
                *known_type == zombie_type
                    || (zombie_type == ZombieType::Gigagargantuar
                        && *known_type == ZombieType::Gargantuar)
            })
            .map(|(_, definition)| (definition, true)),
    }
}

fn board_zombie_reanim_action(zombie: &ZombieState) -> &'static str {
    match zombie.zombie_type {
        ZombieType::Newspaper if zombie.newspaper_mad_pending => "anim_gasp",
        ZombieType::Newspaper if !zombie.armor_intact => {
            if zombie.eating {
                "anim_eat_nopaper"
            } else {
                "anim_walk_nopaper"
            }
        }
        ZombieType::Digger if zombie.digger_underground => "anim_dig",
        ZombieType::PoleVaulter if !zombie.has_vaulted => "anim_run",
        ZombieType::Pogo => "anim_pogo",
        ZombieType::Bobsled if zombie.bobsled_sliding => "anim_push",
        ZombieType::Zamboni => "anim_drive",
        ZombieType::DolphinRider if zombie.in_pool => "anim_swim",
        ZombieType::DolphinRider => "anim_walkdolphin",
        ZombieType::Snorkel if zombie.in_pool && !zombie.eating => "anim_swim",
        ZombieType::Ladder if zombie.eating && !zombie.ladder_placed => "anim_laddereat",
        ZombieType::Ladder if !zombie.ladder_placed => "anim_ladderwalk",
        ZombieType::Bungee => "anim_drop",
        ZombieType::Catapult if zombie.catapult_armed => "anim_shoot",
        ZombieType::Balloon | ZombieType::Gargantuar | ZombieType::Gigagargantuar => "anim_idle",
        ZombieType::Imp if zombie.imp_flight_ticks > 0 => "anim_thrown",
        ZombieType::Digger if zombie.digger_counter > 0 => "anim_drill",
        _ if zombie.eating => "anim_eat",
        _ => "anim_walk",
    }
}

fn zombie_seaweed_attachment_specs(zombie_type: ZombieType) -> [(&'static str, f32, f32); 3] {
    let head = match zombie_type {
        ZombieType::Conehead => ("anim_cone", 37.0, 20.0),
        ZombieType::Buckethead => ("anim_bucket", 37.0, 20.0),
        _ => ("anim_head1", 30.0, 20.0),
    };
    [
        head,
        ("Zombie_outerarm_upper", 5.0, 5.0),
        ("Zombie_duckytube", 77.0, 20.0),
    ]
}

fn reanim_attachment_position(
    definition: &ReanimatorDefinition,
    action: &str,
    tick: u64,
    origin: (f32, f32),
    track_name: &str,
    offset: (f32, f32),
) -> Option<(f32, f32)> {
    let frame_position = reanim_frame_position(definition, action, tick)
        .or_else(|| reanim_frame_position(definition, "anim_walk", tick))
        .or_else(|| reanim_frame_position(definition, "anim_idle", tick))?;
    let transform = definition
        .tracks
        .iter()
        .find(|track| track.name.eq_ignore_ascii_case(track_name))
        .and_then(|track| reanim_transform_at_any(track, frame_position))?;
    let matrix = reanim_matrix_mul(
        reanim_matrix_translation(origin.0, origin.1),
        reanim_matrix_mul(
            reanim_matrix_from_transform(&transform),
            reanim_matrix_translation(offset.0, offset.1),
        ),
    );
    Some((matrix.m02, matrix.m12))
}

fn zombie_seaweed_attachment_positions(
    catalog: &ReanimCatalog,
    zombie: &ZombieState,
    tick: u64,
) -> Option<[(f32, f32); 3]> {
    let (definition, _) = board_zombie_reanim_definition(catalog, zombie.zombie_type)?;
    let action = board_zombie_reanim_action(zombie);
    let origin = (
        fixed_point_to_logical(zombie.position_x),
        board_row_y(zombie.row),
    );
    let mut positions = [(0.0, 0.0); 3];
    for (index, (track, x, y)) in zombie_seaweed_attachment_specs(zombie.zombie_type)
        .into_iter()
        .enumerate()
    {
        positions[index] =
            reanim_attachment_position(definition, action, tick, origin, track, (x, y))?;
    }
    Some(positions)
}

fn board_plant_reanim_actions(
    plant_type: PlantType,
    asleep: bool,
    special_counter: u32,
    special_armed: bool,
    shooting_counter: u32,
    production_stage: u8,
) -> &'static [&'static str] {
    if asleep {
        if plant_type == PlantType::Other(9) {
            return &["anim_bigsleep", "anim_sleep", "anim_idle"];
        }
        return &["anim_sleep", "anim_idle"];
    }
    if plant_type == PlantType::Other(4) && special_armed {
        return &["anim_armed", "anim_idle"];
    }
    if plant_type == PlantType::Other(27) && special_counter > 0 {
        return &["anim_blow", "anim_idle"];
    }
    if matches!(plant_type, PlantType::Other(2 | 14 | 15 | 20)) && special_counter > 0 {
        return &["anim_explode", "anim_idle"];
    }
    if plant_type == PlantType::Other(47) && special_counter > 0 && !special_armed {
        return &["anim_unarmed_idle", "anim_idle"];
    }
    if shooting_counter > 0 {
        return &["anim_shooting", "anim_shoot", "anim_idle"];
    }
    if plant_type == PlantType::Other(9) && production_stage > 0 {
        return &["anim_bigidle", "anim_idle"];
    }
    &["anim_idle", "anim_walk", "anim_shoot"]
}

fn board_plant_reanim_track_style(
    plant_type: PlantType,
    special_armed: bool,
    kernel_pult_projectile: Option<ProjectileType>,
    name: &str,
) -> Option<(i32, BlendMode)> {
    if board_plant_reanim_head_track(plant_type, name) {
        return None;
    }
    if plant_type == PlantType::Other(4) && name.eq_ignore_ascii_case("anim_glow") && !special_armed
    {
        return None;
    }
    if plant_type == PlantType::Other(30) && name.eq_ignore_ascii_case("Pumpkin_back") {
        return Some((9, BlendMode::Alpha));
    }
    if plant_type == PlantType::Other(34) {
        if name.starts_with("Cornpult_butter") {
            return (kernel_pult_projectile == Some(ProjectileType::Butter))
                .then_some((10, BlendMode::Alpha));
        }
        if name.starts_with("Cornpult_kernal") {
            return (kernel_pult_projectile != Some(ProjectileType::Butter))
                .then_some((10, BlendMode::Alpha));
        }
    }
    Some((10, BlendMode::Alpha))
}

fn board_plant_reanim_head_track(plant_type: PlantType, name: &str) -> bool {
    match plant_type {
        PlantType::Peashooter | PlantType::Other(5 | 7 | 40) => name.starts_with("anim_head"),
        PlantType::Other(28) => name.starts_with("anim_head") || name.starts_with("anim_splitpea"),
        PlantType::Other(18) => name.starts_with("anim_head"),
        _ => false,
    }
}

fn board_plant_reanim_attachment_specs(
    plant_type: PlantType,
    action: &str,
    firing_directions: u8,
) -> &'static [(&'static str, &'static str)] {
    let shooting =
        action.eq_ignore_ascii_case("anim_shooting") || action.eq_ignore_ascii_case("anim_shoot");
    match plant_type {
        PlantType::Peashooter | PlantType::Other(5 | 7 | 40) => {
            if shooting {
                &[("anim_stem", "anim_shooting")]
            } else {
                &[("anim_stem", "anim_head_idle")]
            }
        }
        PlantType::Other(28) if shooting => match firing_directions & 3 {
            1 => &[("anim_idle", "anim_shooting")],
            2 => &[("anim_idle", "anim_splitpea_shooting")],
            _ => &[
                ("anim_idle", "anim_shooting"),
                ("anim_idle", "anim_splitpea_shooting"),
            ],
        },
        PlantType::Other(28) => &[
            ("anim_idle", "anim_head_idle"),
            ("anim_idle", "anim_splitpea_idle"),
        ],
        PlantType::Other(18) if shooting => &[
            ("anim_head1", "anim_shooting1"),
            ("anim_head2", "anim_shooting2"),
            ("anim_head3", "anim_shooting3"),
        ],
        PlantType::Other(18) => &[
            ("anim_head1", "anim_head_idle1"),
            ("anim_head2", "anim_head_idle2"),
            ("anim_head3", "anim_head_idle3"),
        ],
        _ => &[],
    }
}

fn board_plant_reanim_attached_track_visible(action: &str, name: &str) -> bool {
    name.eq_ignore_ascii_case(action)
}

#[derive(Clone, Copy)]
struct ReanimMatrix {
    m00: f32,
    m01: f32,
    m10: f32,
    m11: f32,
    m02: f32,
    m12: f32,
}

fn reanim_matrix_from_transform(transform: &ReanimatorTransform) -> ReanimMatrix {
    let skew_x = -transform.skew_x.to_radians();
    let skew_y = -transform.skew_y.to_radians();
    ReanimMatrix {
        m00: skew_x.cos() * transform.scale_x,
        m01: skew_y.sin() * transform.scale_y,
        m10: -skew_x.sin() * transform.scale_x,
        m11: skew_y.cos() * transform.scale_y,
        m02: transform.x,
        m12: transform.y,
    }
}

fn reanim_matrix_translation(x: f32, y: f32) -> ReanimMatrix {
    ReanimMatrix {
        m00: 1.0,
        m01: 0.0,
        m10: 0.0,
        m11: 1.0,
        m02: x,
        m12: y,
    }
}

fn reanim_matrix_mul(left: ReanimMatrix, right: ReanimMatrix) -> ReanimMatrix {
    ReanimMatrix {
        m00: left.m00 * right.m00 + left.m01 * right.m10,
        m01: left.m00 * right.m01 + left.m01 * right.m11,
        m10: left.m10 * right.m00 + left.m11 * right.m10,
        m11: left.m10 * right.m01 + left.m11 * right.m11,
        m02: left.m00 * right.m02 + left.m01 * right.m12 + left.m02,
        m12: left.m10 * right.m02 + left.m11 * right.m12 + left.m12,
    }
}

fn reanim_matrix_inverse(matrix: ReanimMatrix) -> Option<ReanimMatrix> {
    let determinant = matrix.m00 * matrix.m11 - matrix.m01 * matrix.m10;
    if determinant.abs() < f32::EPSILON {
        return None;
    }
    Some(ReanimMatrix {
        m00: matrix.m11 / determinant,
        m01: -matrix.m01 / determinant,
        m10: -matrix.m10 / determinant,
        m11: matrix.m00 / determinant,
        m02: (matrix.m01 * matrix.m12 - matrix.m02 * matrix.m11) / determinant,
        m12: (matrix.m02 * matrix.m10 - matrix.m00 * matrix.m12) / determinant,
    })
}

fn push_board_plant_reanim_attachments(
    frame: &mut RenderFrame,
    definition: &ReanimatorDefinition,
    image_ids: &HashMap<String, u32>,
    specs: &[(&str, &str)],
    tick: u64,
    (x, y, frame_position): (f32, f32, f32),
) -> bool {
    if specs.is_empty() {
        return false;
    }
    let Some(anchor_name) = specs.first().map(|(anchor, _)| *anchor) else {
        return false;
    };
    let anchor_track = definition
        .tracks
        .iter()
        .find(|track| track.name.eq_ignore_ascii_case(anchor_name))
        .or_else(|| {
            if anchor_name == "anim_stem" {
                definition
                    .tracks
                    .iter()
                    .find(|track| track.name.eq_ignore_ascii_case("anim_idle"))
            } else {
                None
            }
        });
    let Some(anchor_track) = anchor_track else {
        return false;
    };
    let Some((base_frame, _)) = reanim_frames(definition, "anim_idle") else {
        return false;
    };
    let Some(anchor_current) = reanim_transform_at_any(anchor_track, frame_position) else {
        return false;
    };
    let Some(anchor_base) = reanim_transform_at_any(anchor_track, base_frame as f32) else {
        return false;
    };
    let Some(anchor_base_inverse) =
        reanim_matrix_inverse(reanim_matrix_from_transform(&anchor_base))
    else {
        return false;
    };
    let parent_overlay = reanim_matrix_mul(
        reanim_matrix_translation(x, y),
        reanim_matrix_mul(
            reanim_matrix_from_transform(&anchor_current),
            anchor_base_inverse,
        ),
    );
    let mut drawn = false;
    for &(anchor, child_action) in specs {
        if anchor != anchor_name {
            continue;
        }
        let Some(child_position) = reanim_frame_position(definition, child_action, tick) else {
            continue;
        };
        for track in &definition.tracks {
            if !board_plant_reanim_attached_track_visible(child_action, &track.name) {
                continue;
            }
            let Some(transform) = reanim_transform_at(track, child_position) else {
                continue;
            };
            let Some(image) = transform.image.as_deref() else {
                continue;
            };
            let Some(&resource_id) = image_ids.get(&image.to_ascii_uppercase()) else {
                continue;
            };
            let matrix =
                reanim_matrix_mul(parent_overlay, reanim_matrix_from_transform(&transform));
            frame.affine_sprites.push(AffineSpriteCommand {
                resource_id,
                x: matrix.m02,
                y: matrix.m12,
                m00: matrix.m00,
                m01: matrix.m01,
                m10: matrix.m10,
                m11: matrix.m11,
                z: 11,
                alpha: transform.alpha * anchor_current.alpha,
                tint: [1.0; 3],
                blend_mode: BlendMode::Alpha,
                source: None,
            });
            drawn = true;
        }
    }
    drawn
}

fn reanim_frames(definition: &ReanimatorDefinition, name: &str) -> Option<(usize, usize)> {
    let track = definition
        .tracks
        .iter()
        .find(|track| track.name.eq_ignore_ascii_case(name))?;
    let start = track
        .transforms
        .iter()
        .position(|transform| transform.frame >= 0.0)?;
    let end = track
        .transforms
        .iter()
        .rposition(|transform| transform.frame >= 0.0)?;
    Some((start, end - start + 1))
}

fn reanim_frame_position(
    definition: &ReanimatorDefinition,
    action: &str,
    tick: u64,
) -> Option<f32> {
    let (frame_start, frame_count) = reanim_frames(definition, action)?;
    let frame_count = frame_count.max(1);
    let progress = (tick as f32 * 0.01 * definition.fps / frame_count as f32).fract();
    Some(frame_start as f32 + progress * frame_count.saturating_sub(1) as f32)
}

fn reanim_transform_at(track: &ReanimatorTrack, position: f32) -> Option<ReanimatorTransform> {
    let transform = reanim_transform_at_any(track, position)?;
    (transform.frame >= 0.0).then_some(transform)
}

fn reanim_transform_at_any(track: &ReanimatorTrack, position: f32) -> Option<ReanimatorTransform> {
    let before_index = (position.floor() as usize).min(track.transforms.len().saturating_sub(1));
    let after_index = (before_index + 1).min(track.transforms.len().saturating_sub(1));
    let before = track.transforms.get(before_index)?;
    let after = track.transforms.get(after_index)?;
    let fraction = position.fract();
    let lerp = |first: f32, second: f32| first + (second - first) * fraction;
    Some(ReanimatorTransform {
        x: lerp(before.x, after.x),
        y: lerp(before.y, after.y),
        skew_x: lerp(before.skew_x, after.skew_x),
        skew_y: lerp(before.skew_y, after.skew_y),
        scale_x: lerp(before.scale_x, after.scale_x),
        scale_y: lerp(before.scale_y, after.scale_y),
        frame: before.frame,
        alpha: lerp(before.alpha, after.alpha),
        image: before.image.clone(),
    })
}

fn push_reanim_tracks(
    frame: &mut RenderFrame,
    definition: &ReanimatorDefinition,
    image_ids: &HashMap<String, u32>,
    frame_position: f32,
    (x, y, scale): (f32, f32, f32),
    track_style: impl Fn(&str) -> Option<(i32, BlendMode)>,
) -> bool {
    push_reanim_tracks_with_image_override(
        frame,
        definition,
        image_ids,
        frame_position,
        (x, y, scale),
        track_style,
        |_| None,
    )
}

fn push_reanim_tracks_with_image_override(
    frame: &mut RenderFrame,
    definition: &ReanimatorDefinition,
    image_ids: &HashMap<String, u32>,
    frame_position: f32,
    (x, y, scale): (f32, f32, f32),
    track_style: impl Fn(&str) -> Option<(i32, BlendMode)>,
    image_override: impl Fn(&str) -> Option<&'static str>,
) -> bool {
    let mut drawn = false;
    for track in &definition.tracks {
        let Some((z, blend_mode)) = track_style(&track.name) else {
            continue;
        };
        let Some(transform) = reanim_transform_at(track, frame_position) else {
            continue;
        };
        let Some(image) = image_override(&track.name).or(transform.image.as_deref()) else {
            continue;
        };
        let Some(&resource_id) = image_ids.get(&image.to_ascii_uppercase()) else {
            continue;
        };
        let skew_x = -transform.skew_x.to_radians();
        let skew_y = -transform.skew_y.to_radians();
        frame.affine_sprites.push(AffineSpriteCommand {
            resource_id,
            x: x + transform.x,
            y: y + transform.y,
            m00: skew_x.cos() * transform.scale_x * scale,
            m01: skew_y.sin() * transform.scale_y * scale,
            m10: -skew_x.sin() * transform.scale_x * scale,
            m11: skew_y.cos() * transform.scale_y * scale,
            z,
            alpha: transform.alpha,
            tint: [1.0; 3],
            blend_mode,
            source: None,
        });
        drawn = true;
    }
    drawn
}

fn board_boss_reanim_action(zombie: &ZombieState) -> &'static str {
    boss_reanim_action(
        zombie.health,
        zombie.special_phase != 0,
        zombie.boss_ball_active,
    )
}

fn boss_reanim_action(health: i32, rv_active: bool, ball_active: bool) -> &'static str {
    if health == 0 {
        "anim_death"
    } else if rv_active {
        "anim_RV_1"
    } else if ball_active {
        "anim_head_attack_1"
    } else {
        "anim_idle"
    }
}

fn board_boss_reanim_track_z(name: &str) -> i32 {
    if name.starts_with("Boss_innerleg") {
        5
    } else if name.starts_with("Boss_outerleg") || name.eq_ignore_ascii_case("Boss_body2") {
        6
    } else if name.starts_with("Boss_innerarm") || name.starts_with("Boss_RV") {
        8
    } else {
        7
    }
}

fn board_boss_ball_track_z(name: &str) -> i32 {
    if name.eq_ignore_ascii_case("Layer 47") || name.eq_ignore_ascii_case("Layer 64") {
        11
    } else if name.eq_ignore_ascii_case("multiply") {
        12
    } else if name.eq_ignore_ascii_case("additive")
        || name.eq_ignore_ascii_case("superglow")
        || name.eq_ignore_ascii_case("ice_highlight")
    {
        14
    } else {
        13
    }
}

fn board_boss_ball_blend_mode(name: &str) -> BlendMode {
    if name.eq_ignore_ascii_case("additive")
        || name.eq_ignore_ascii_case("superglow")
        || name.eq_ignore_ascii_case("ice_highlight")
    {
        BlendMode::Additive
    } else {
        BlendMode::Alpha
    }
}

fn board_zombie_reanim_track_visible(
    name: &str,
    zombie_type: ZombieType,
    armor_intact: bool,
    has_head: bool,
    has_arm: bool,
    mustache_mode: bool,
) -> bool {
    if !has_head
        && (name.starts_with("anim_head")
            || name.starts_with("anim_hair")
            || name.starts_with("Zombie_mustache"))
    {
        return false;
    }
    if !has_arm && (name.starts_with("anim_innerarm") || name.starts_with("Zombie_outerarm")) {
        return false;
    }
    if name.starts_with("anim_cone") {
        return zombie_type == ZombieType::Conehead && armor_intact;
    }
    if name.starts_with("anim_bucket") {
        return zombie_type == ZombieType::Buckethead && armor_intact;
    }
    if name.starts_with("anim_screendoor") {
        return zombie_type == ZombieType::ScreenDoor && armor_intact;
    }
    if name.starts_with("anim_hair") {
        return !matches!(zombie_type, ZombieType::Conehead | ZombieType::Buckethead)
            || !armor_intact;
    }
    if name == "Zombie_flaghand" {
        return zombie_type == ZombieType::Flag;
    }
    if name.starts_with("Zombie_flag") {
        return false;
    }
    if name.starts_with("Zombie_duckytube") {
        return zombie_type == ZombieType::DuckyTube;
    }
    if name.starts_with("Zombie_outerarm_screendoor") {
        return zombie_type == ZombieType::ScreenDoor && armor_intact;
    }
    if name.starts_with("Zombie_innerarm_screendoor") {
        return (zombie_type == ZombieType::ScreenDoor && armor_intact)
            || (zombie_type == ZombieType::Flag && name == "Zombie_innerarm_screendoor");
    }
    if name.starts_with("anim_innerarm") {
        return !matches!(zombie_type, ZombieType::ScreenDoor | ZombieType::Flag);
    }
    if name.starts_with("Zombie_outerarm") {
        return zombie_type != ZombieType::ScreenDoor;
    }
    if name.starts_with("Zombie_mustache") {
        return mustache_mode && has_head;
    }
    if name.starts_with("anim_tongue")
        || name.starts_with("Zombie_paper_paper")
        || name.starts_with("Zombie_paper")
    {
        return false;
    }
    true
}

fn mustache_image_symbol(variant: u8) -> &'static str {
    match variant {
        2 => "IMAGE_REANIM_ZOMBIE_MUSTACHE2",
        3 => "IMAGE_REANIM_ZOMBIE_MUSTACHE3",
        _ => "IMAGE_REANIM_ZOMBIE_MUSTACHE1",
    }
}

fn future_head_image_symbol(entity: EntityId) -> &'static str {
    match entity & 3 {
        0 => "IMAGE_REANIM_ZOMBIE_HEAD_SUNGLASSES1",
        1 => "IMAGE_REANIM_ZOMBIE_HEAD_SUNGLASSES2",
        2 => "IMAGE_REANIM_ZOMBIE_HEAD_SUNGLASSES3",
        _ => "IMAGE_REANIM_ZOMBIE_HEAD_SUNGLASSES4",
    }
}

fn board_specialized_zombie_reanim_track_visible(
    name: &str,
    zombie_type: ZombieType,
    armor_intact: bool,
    has_head: bool,
    has_arm: bool,
) -> bool {
    if !has_head
        && (name.starts_with("anim_head")
            || name.starts_with("anim_hair")
            || name.starts_with("anim_hairpiece"))
    {
        return false;
    }
    if !has_arm
        && (name.eq_ignore_ascii_case("zombie_football_leftarm_lower")
            || name.eq_ignore_ascii_case("zombie_football_leftarm_hand")
            || name.eq_ignore_ascii_case("Zombie_paper_hands")
            || name.eq_ignore_ascii_case("Zombie_paper_leftarm_lower"))
    {
        return false;
    }
    if name.eq_ignore_ascii_case("zombie_football_helmet") {
        return zombie_type == ZombieType::Football && armor_intact;
    }
    if name.eq_ignore_ascii_case("anim_hair") {
        return zombie_type == ZombieType::Football && !armor_intact;
    }
    if name.starts_with("Zombie_paper_paper") {
        return zombie_type == ZombieType::Newspaper && armor_intact;
    }
    true
}

fn push_board_zombie_flag(frame: &mut RenderFrame, x: f32, y: f32) {
    frame.sprites.extend([
        SpriteCommand {
            resource_id: BOARD_ZOMBIE_FLAG_POLE_IMAGE_ID,
            x: x - 13.0,
            y: y - 35.0,
            z: 7,
            scale: 0.55,
            alpha: 1.0,
        },
        SpriteCommand {
            resource_id: BOARD_ZOMBIE_FLAG_IMAGE_ID,
            x: x - 3.0,
            y: y - 31.0,
            z: 9,
            scale: 0.65,
            alpha: 1.0,
        },
    ]);
}

fn board_zombie_head_image(zombie_type: ZombieType) -> u32 {
    match zombie_type {
        ZombieType::Football => BOARD_ZOMBIE_FOOTBALL_HEAD_IMAGE_ID,
        ZombieType::Newspaper => BOARD_ZOMBIE_NEWSPAPER_HEAD_IMAGE_ID,
        _ => TITLE_LOAD_BAR_ZOMBIE_HEAD_IMAGE_ID,
    }
}

fn push_board_zombie_equipment(
    frame: &mut RenderFrame,
    x: f32,
    y: f32,
    zombie_type: ZombieType,
    armor_intact: bool,
) {
    match zombie_type {
        ZombieType::Flag => {
            frame.sprites.push(SpriteCommand {
                resource_id: BOARD_ZOMBIE_FLAG_POLE_IMAGE_ID,
                x: x - 13.0,
                y: y - 35.0,
                z: 7,
                scale: 0.55,
                alpha: 1.0,
            });
            frame.sprites.push(SpriteCommand {
                resource_id: BOARD_ZOMBIE_FLAG_HAND_IMAGE_ID,
                x: x - 9.0,
                y: y + 8.0,
                z: 9,
                scale: 0.7,
                alpha: 1.0,
            });
            frame.sprites.push(SpriteCommand {
                resource_id: BOARD_ZOMBIE_FLAG_IMAGE_ID,
                x: x - 3.0,
                y: y - 31.0,
                z: 9,
                scale: 0.65,
                alpha: 1.0,
            });
        }
        ZombieType::Conehead if armor_intact => frame.sprites.push(SpriteCommand {
            resource_id: BOARD_ZOMBIE_CONE_IMAGE_ID,
            x: x - 34.0,
            y: y - 30.0,
            z: 9,
            scale: 0.75,
            alpha: 1.0,
        }),
        ZombieType::Buckethead if armor_intact => frame.sprites.push(SpriteCommand {
            resource_id: BOARD_ZOMBIE_BUCKET_IMAGE_ID,
            x: x - 35.0,
            y: y - 32.0,
            z: 9,
            scale: 0.7,
            alpha: 1.0,
        }),
        ZombieType::ScreenDoor if armor_intact => frame.sprites.push(SpriteCommand {
            resource_id: BOARD_ZOMBIE_SCREEN_DOOR_IMAGE_ID,
            x: x - 35.0,
            y: y - 3.0,
            z: 9,
            scale: 0.65,
            alpha: 1.0,
        }),
        ZombieType::Football if armor_intact => {
            frame.sprites.push(SpriteCommand {
                resource_id: BOARD_ZOMBIE_FOOTBALL_UPPERBODY_IMAGE_ID,
                x: x - 55.0,
                y: y + 1.0,
                z: 7,
                scale: 0.4,
                alpha: 1.0,
            });
            frame.sprites.push(SpriteCommand {
                resource_id: BOARD_ZOMBIE_FOOTBALL_HELMET_IMAGE_ID,
                x: x - 36.0,
                y: y - 31.0,
                z: 9,
                scale: 0.68,
                alpha: 1.0,
            });
        }
        ZombieType::Newspaper if armor_intact => frame.sprites.push(SpriteCommand {
            resource_id: BOARD_ZOMBIE_NEWSPAPER_IMAGE_ID,
            x: x - 39.0,
            y: y + 3.0,
            z: 9,
            scale: 0.65,
            alpha: 1.0,
        }),
        _ => {}
    }
}

fn board_projectile_image(projectile_type: ProjectileType) -> Option<(u32, f32)> {
    match projectile_type {
        ProjectileType::Pea | ProjectileType::ZombiePea => {
            Some((BOARD_PROJECTILE_PEA_IMAGE_ID, 0.75))
        }
        ProjectileType::SnowPea => Some((BOARD_PROJECTILE_SNOW_PEA_IMAGE_ID, 0.75)),
        ProjectileType::Puff => Some((BOARD_PROJECTILE_PUFF_IMAGE_ID, 0.75)),
        ProjectileType::Cabbage => Some((BOARD_PROJECTILE_CABBAGE_IMAGE_ID, 1.0)),
        ProjectileType::Melon => Some((BOARD_PROJECTILE_MELON_IMAGE_ID, 1.0)),
        ProjectileType::WinterMelon => Some((BOARD_PROJECTILE_WINTER_MELON_IMAGE_ID, 1.0)),
        ProjectileType::Kernel => Some((BOARD_PROJECTILE_KERNEL_IMAGE_ID, 0.95)),
        ProjectileType::Butter => Some((BOARD_PROJECTILE_BUTTER_IMAGE_ID, 0.8)),
        ProjectileType::Spike => Some((BOARD_PROJECTILE_SPIKE_IMAGE_ID, 1.0)),
        ProjectileType::Star => Some((BOARD_PROJECTILE_STAR_IMAGE_ID, 1.0)),
        ProjectileType::Fireball => Some((BOARD_PROJECTILE_FIREBALL_IMAGE_ID, 1.4)),
        ProjectileType::Cob => Some((BOARD_PROJECTILE_COB_IMAGE_ID, 0.9)),
        ProjectileType::Other(1) => Some((BOARD_PROJECTILE_BASKETBALL_IMAGE_ID, 1.1)),
        ProjectileType::Other(_) => None,
    }
}

fn board_projectile_scale(projectile_type: ProjectileType, age: u32) -> Option<f32> {
    let (_, scale) = board_projectile_image(projectile_type)?;
    if projectile_type == ProjectileType::Puff {
        let progress = age.min(30) as f32 / 30.0;
        Some(scale * (0.3 + 0.7 * progress))
    } else {
        Some(scale)
    }
}

fn board_projectile_rotation(projectile_type: ProjectileType, age: u32) -> f32 {
    // ponytail: midpoint spin for source-randomized projectile angles; persist RNG-backed visual state if exact replay fidelity needs it.
    let age = age as f32;
    match projectile_type {
        ProjectileType::Cabbage | ProjectileType::Butter => {
            -7.0 * std::f32::consts::PI / 25.0 - 0.05 * age
        }
        ProjectileType::Melon | ProjectileType::WinterMelon => {
            -2.0 * std::f32::consts::PI / 5.0 - 0.05 * age
        }
        ProjectileType::Kernel => -0.14 * age,
        ProjectileType::Cob => std::f32::consts::PI / 2.0,
        ProjectileType::Other(1) | ProjectileType::Star => 0.075 * age,
        _ => 0.0,
    }
}

fn board_projectile_y(position_y: i64, row: u8, lob_height: i32) -> f32 {
    let core_row_y = 80.0 + f32::from(row) * 100.0;
    board_row_y(row) + fixed_point_to_logical(position_y) - core_row_y
        + 18.0
        + lob_height as f32 / 1_000.0
}

fn board_projectile_shadow(
    scene: SceneKind,
    projectile_type: ProjectileType,
    row: u8,
    position_x: i64,
    shadow_y: i64,
    lob_height: i32,
) -> Option<(u32, f32, f32, f32, f32)> {
    let resource_id = if matches!(scene, SceneKind::Night | SceneKind::Fog | SceneKind::Boss) {
        BOARD_PROJECTILE_SHADOW_NIGHT_IMAGE_ID
    } else {
        BOARD_PROJECTILE_SHADOW_DAY_IMAGE_ID
    };
    let (offset_x, offset_y, scale, stretch) = match projectile_type {
        ProjectileType::Puff => return None,
        ProjectileType::Pea | ProjectileType::ZombiePea => (3.0, 0.0, 1.0, 1.0),
        ProjectileType::SnowPea => (-1.0, 0.0, 1.3, 1.0),
        ProjectileType::Star => (7.0, 0.0, 1.0, 1.0),
        ProjectileType::Cabbage
        | ProjectileType::Kernel
        | ProjectileType::Butter
        | ProjectileType::Melon
        | ProjectileType::WinterMelon => (3.0, 10.0, 1.6, 1.0),
        ProjectileType::Cob => (57.0, 0.0, 1.0, 3.0),
        ProjectileType::Fireball => (0.0, 0.0, 1.4, 1.0),
        ProjectileType::Spike | ProjectileType::Other(1) => (0.0, 0.0, 1.0, 1.0),
        ProjectileType::Other(_) => return None,
    };
    let lobbed = matches!(
        projectile_type,
        ProjectileType::Cabbage
            | ProjectileType::Kernel
            | ProjectileType::Butter
            | ProjectileType::Melon
            | ProjectileType::WinterMelon
            | ProjectileType::Cob
            | ProjectileType::Other(1)
    );
    let height = if lobbed {
        (-(lob_height as f32) / 1_000.0).clamp(0.0, 200.0)
    } else {
        0.0
    };
    let height_scale = 200.0 / (height + 200.0);
    let core_row_y = 80.0 + f32::from(row) * 100.0;
    Some((
        resource_id,
        fixed_point_to_logical(position_x) + offset_x,
        board_row_y(row) + fixed_point_to_logical(shadow_y) - core_row_y + offset_y,
        scale * height_scale * stretch,
        scale * height_scale,
    ))
}

fn board_coin_reanim_offset(coin_type: CoinType) -> Option<(f32, f32)> {
    match coin_type {
        CoinType::Silver | CoinType::Gold => Some((-1.0, 1.0)),
        CoinType::Diamond => Some((-18.0, -11.0)),
        _ => None,
    }
}

fn board_coin_reanim_visible(
    coin_type: CoinType,
    target_y: Option<i64>,
    from_present: bool,
) -> bool {
    match coin_type {
        CoinType::Silver | CoinType::Gold => !from_present && target_y.is_none(),
        CoinType::Diamond => true,
        _ => false,
    }
}

fn board_coin_image(coin_type: CoinType) -> Option<(u32, f32)> {
    match coin_type {
        CoinType::Silver => Some((BOARD_COIN_SILVER_IMAGE_ID, 0.65)),
        CoinType::Gold => Some((BOARD_COIN_GOLD_IMAGE_ID, 0.65)),
        CoinType::Diamond => Some((BOARD_DIAMOND_IMAGE_ID, 0.65)),
        CoinType::PresentPlant
        | CoinType::AwardPresent
        | CoinType::PresentMinigames
        | CoinType::PresentPuzzleMode
        | CoinType::PresentSurvivalMode => Some((BOARD_PRESENT_IMAGE_ID, 0.8)),
        CoinType::AwardMoneyBag | CoinType::AwardBagDiamond => Some((BOARD_MONEYBAG_IMAGE_ID, 0.5)),
        CoinType::Chocolate | CoinType::AwardChocolate => Some((BOARD_CHOCOLATE_IMAGE_ID, 0.8)),
        CoinType::FinalSeedPacket => Some((SEED_PACKET_NORMAL_IMAGE_ID, 0.8)),
        CoinType::Vase => Some((BOARD_VASE_IMAGE_ID, 0.8)),
        CoinType::Note => Some((BOARD_NOTE_IMAGE_ID, 0.8)),
        CoinType::Trophy => Some((AWARD_TROPHY_IMAGE_ID, 0.5)),
        CoinType::Shovel => Some((AWARD_SHOVEL_IMAGE_ID, 0.5)),
        CoinType::Almanac => Some((AWARD_ALMANAC_IMAGE_ID, 0.8)),
        CoinType::CarKeys => Some((AWARD_CAR_KEYS_IMAGE_ID, 0.8)),
        CoinType::WateringCan => Some((AWARD_WATERING_CAN_IMAGE_ID, 0.8)),
        CoinType::Taco => Some((AWARD_TACO_IMAGE_ID, 0.8)),
        CoinType::AwardSilverSunflower => Some((BOARD_SILVER_SUNFLOWER_IMAGE_ID, 0.5)),
        CoinType::AwardGoldSunflower => Some((BOARD_GOLD_SUNFLOWER_IMAGE_ID, 0.5)),
        CoinType::UsableSeedPacket => Some((SEED_PACKET_NORMAL_IMAGE_ID, 0.8)),
        _ => None,
    }
}

fn new_scene_game(scene: SceneKind) -> Game {
    if scene == SceneKind::Day {
        Game::new_mode(0, ModeKind::Adventure, 1)
    } else {
        Game::new(0, scene)
    }
}

fn new_adventure_game(current: &Game, profile: Option<&SaveProfile>) -> Game {
    let (level, first_time, packet_upgrades, stinky_purchased) = profile
        .map(|profile| {
            (
                profile.adventure_level,
                profile.adventure_rounds == 0,
                profile.packet_upgrades,
                profile.stinky_purchased,
            )
        })
        .unwrap_or((current.state().level.max(1), true, 0, false));
    Game::new_adventure(0, level, first_time, packet_upgrades, stinky_purchased)
}

fn mode_level_at(mode: ModeKind, x: f32, y: f32) -> Option<u8> {
    if !(30.0..780.0).contains(&x) || !(55.0..595.0).contains(&y) {
        return None;
    }
    let column = ((x - 30.0) / 150.0) as usize;
    let row = ((y - 55.0) / 135.0) as usize;
    let index = row.checked_mul(5)?.checked_add(column)?;
    (index < mode_level_names(mode).len())
        .then(|| u8::try_from(index).ok())
        .flatten()
}

fn selector_level_digits(level: u8) -> Vec<(u8, f32, f32)> {
    let level = level.clamp(1, 50);
    let stage = (level - 1) / 10 + 1;
    let sub = level - (stage - 1) * 10;
    let stage_x = if stage == 4 { 485.0 } else { 486.0 };
    let stage_y = if stage == 1 { 126.0 } else { 125.0 };
    let sub_x = if sub == 3 { 503.0 } else { 504.0 };
    let mut digits = vec![(stage, stage_x, stage_y)];
    if sub == 10 {
        digits.extend([(1, sub_x, 128.0), (0, sub_x + 9.0, 129.0)]);
    } else {
        digits.push((sub, sub_x, 128.0));
    }
    digits
}

fn mode_is_unlocked(
    mode: ModeKind,
    adventure_finished: bool,
    adventure_level: u8,
    unlocked_modes: u8,
) -> bool {
    if adventure_finished {
        return true;
    }
    match mode {
        ModeKind::MiniGame => unlocked_modes & CoinType::PresentMinigames.unlock_mask() != 0,
        ModeKind::Vasebreaker => unlocked_modes & CoinType::PresentPuzzleMode.unlock_mask() != 0,
        ModeKind::Survival => unlocked_modes & CoinType::PresentSurvivalMode.unlock_mask() != 0,
        ModeKind::ZenGarden => adventure_unlocks(adventure_level).2,
        ModeKind::IZombie | ModeKind::Adventure => true,
    }
}

fn audio_for_event(event: &GameEvent) -> Option<(AudioKind, &'static str)> {
    match event {
        GameEvent::ReadySetPlant => Some((AudioKind::Effect, "sounds/readysetplant.ogg")),
        GameEvent::SeedSelected { .. } => Some((AudioKind::Effect, "sounds/tap.ogg")),
        GameEvent::InputRejected { .. } => Some((AudioKind::Effect, "sounds/buzzer.ogg")),
        GameEvent::HiddenCodeRejected { .. } => Some((AudioKind::Effect, "sounds/buzzer.ogg")),
        GameEvent::HiddenCodeToggled { code, .. } => Some((
            AudioKind::Effect,
            match code {
                HiddenCode::Mustache => "sounds/polevault.ogg",
                HiddenCode::Future => "sounds/boing.ogg",
                HiddenCode::Pinata => "sounds/juicy.ogg",
                HiddenCode::Daisies => "sounds/loadingbar_flower.ogg",
                HiddenCode::Sukhbir => "sounds/sukhbir.ogg",
            },
        )),
        GameEvent::ChallengeAction {
            kind: ChallengeKind::Beghouled | ChallengeKind::BeghouledTwist,
            value,
        } if *value == 0 => Some((AudioKind::Effect, "sounds/floop.ogg")),
        GameEvent::ChallengeAction {
            kind: ChallengeKind::Beghouled | ChallengeKind::BeghouledTwist,
            value,
        } if *value > 0 => Some((AudioKind::Effect, "sounds/diamond.au")),
        GameEvent::ChallengeAction {
            kind: ChallengeKind::SlotMachine,
            value,
        } if *value > 0 => Some((AudioKind::Effect, "sounds/slotmachine.ogg")),
        GameEvent::ZombiquariumSnorkelPurchased { variant } => Some((
            AudioKind::Effect,
            if *variant == 0 {
                "sounds/plant_water.ogg"
            } else {
                "sounds/zombie_entering_water.ogg"
            },
        )),
        GameEvent::ZombiquariumBrainSlurped { .. } => Some((AudioKind::Effect, "sounds/slurp.ogg")),
        GameEvent::ZombiquariumZombieDied { .. } => {
            Some((AudioKind::Effect, "sounds/zombaquarium_die.ogg"))
        }
        GameEvent::WhackHammerSwung => Some((AudioKind::Effect, "sounds/swing.ogg")),
        GameEvent::WhackHit { sound, variant, .. } => Some((
            AudioKind::Effect,
            match sound {
                WhackHitSound::Bonk => "sounds/bonk.ogg",
                WhackHitSound::Shield => {
                    if *variant == 0 {
                        "sounds/shieldhit.ogg"
                    } else {
                        "sounds/shieldhit2.ogg"
                    }
                }
                WhackHitSound::Plastic => {
                    if *variant == 0 {
                        "sounds/plastichit.ogg"
                    } else {
                        "sounds/plastichit2.ogg"
                    }
                }
            },
        )),
        GameEvent::PlantPlaced { variant, .. } | GameEvent::ZombieDeployed { variant, .. } => {
            Some((
                AudioKind::Effect,
                if *variant == 0 {
                    "sounds/plant.ogg"
                } else {
                    "sounds/plant2.ogg"
                },
            ))
        }
        GameEvent::PlantShoveled { .. } => Some((AudioKind::Effect, "sounds/plant2.ogg")),
        GameEvent::BowlingImpact { .. } => Some((AudioKind::Effect, "sounds/bowlingimpact.ogg")),
        GameEvent::SunProduced {
            source: SunSource::Plant(_),
            ..
        } => Some((AudioKind::Effect, "sounds/throw.ogg")),
        GameEvent::CoinProduced {
            coin_type:
                CoinType::Diamond
                | CoinType::Chocolate
                | CoinType::AwardChocolate
                | CoinType::PresentPlant
                | CoinType::AwardPresent
                | CoinType::PresentMinigames
                | CoinType::PresentPuzzleMode
                | CoinType::PresentSurvivalMode,
            ..
        } => Some((AudioKind::Effect, "sounds/chime.ogg")),
        GameEvent::LootDropSound {
            sound: LootDropSound::SpawnSun,
        } => Some((AudioKind::Effect, "sounds/throw.ogg")),
        GameEvent::LootDropSound {
            sound: LootDropSound::ArtChallenge,
        } => Some((AudioKind::Effect, "sounds/diamond.au")),
        GameEvent::AwardCollectionSound { sound } => Some((
            AudioKind::Effect,
            match sound {
                AwardCollectionSound::Coin => "sounds/coin.ogg",
                AwardCollectionSound::Diamond => "sounds/diamond.au",
                AwardCollectionSound::Seedlift => "sounds/seedlift.ogg",
                AwardCollectionSound::Tap2 => "sounds/tap2.ogg",
                AwardCollectionSound::Shovel => "sounds/shovel.ogg",
            },
        )),
        GameEvent::WeatherSound {
            sound: WeatherSound::Rain,
        } => Some((AudioKind::Effect, "sounds/rain.ogg")),
        GameEvent::WeatherSound {
            sound: WeatherSound::Thunder,
        } => Some((AudioKind::Effect, "sounds/thunder.ogg")),
        GameEvent::CoinLanded {
            coin_type: CoinType::Gold,
            ..
        } => Some((AudioKind::Effect, "sounds/moneyfalls.ogg")),
        GameEvent::SunCollected { .. } => Some((AudioKind::Effect, "sounds/points.ogg")),
        GameEvent::PickupCollected {
            coin_type: CoinType::UsableSeedPacket,
            ..
        } => Some((AudioKind::Effect, "sounds/seedlift.ogg")),
        GameEvent::PickupCollected {
            coin_type: CoinType::Sun | CoinType::SmallSun | CoinType::LargeSun,
            ..
        } => Some((AudioKind::Effect, "sounds/points.ogg")),
        GameEvent::PickupCollected {
            coin_type:
                CoinType::Chocolate
                | CoinType::AwardChocolate
                | CoinType::PresentPlant
                | CoinType::AwardPresent
                | CoinType::PresentMinigames
                | CoinType::PresentPuzzleMode
                | CoinType::PresentSurvivalMode,
            ..
        } => Some((AudioKind::Effect, "sounds/prize.ogg")),
        GameEvent::CoinCollected {
            coin_type: CoinType::Diamond,
            ..
        } => Some((AudioKind::Effect, "sounds/diamond.au")),
        GameEvent::CoinCollected { .. } => Some((AudioKind::Effect, "sounds/coin.ogg")),
        GameEvent::GardenWatered { .. } => Some((AudioKind::Effect, "sounds/watering.ogg")),
        GameEvent::GardenFertilized { .. } => Some((AudioKind::Effect, "sounds/fertilizer.ogg")),
        GameEvent::GardenToolUsed {
            tool: GardenTool::BugSpray,
            ..
        } => Some((AudioKind::Effect, "sounds/bugspray.ogg")),
        GameEvent::GardenToolUsed {
            tool: GardenTool::Phonograph,
            ..
        } => Some((AudioKind::Effect, "sounds/phonograph.ogg")),
        GameEvent::GardenToolUsed { .. } => None,
        GameEvent::GardenBecameHappy { .. } => Some((AudioKind::Effect, "sounds/prize.ogg")),
        GameEvent::GardenTreeGrew { .. } => Some((AudioKind::Effect, "sounds/plantgrow.ogg")),
        GameEvent::GardenTapGlass => Some((AudioKind::Effect, "sounds/tapglass.au")),
        GameEvent::GardenLeft => Some((AudioKind::Effect, "sounds/gravebutton.ogg")),
        GameEvent::WaveStarted { wave: 0 } => Some((AudioKind::Effect, "sounds/awooga.ogg")),
        GameEvent::FlagWaveSound { .. } => Some((AudioKind::Effect, "sounds/siren.ogg")),
        GameEvent::HugeWaveSound { .. } => Some((AudioKind::Effect, "sounds/hugewave.ogg")),
        GameEvent::FinalWaveSound { .. } => Some((AudioKind::Effect, "sounds/finalwave.ogg")),
        GameEvent::BossStomp { .. } => Some((AudioKind::Effect, "sounds/gargantuar_thump.ogg")),
        GameEvent::BossRVStarted { .. } => Some((AudioKind::Effect, "sounds/hydraulic_short.ogg")),
        GameEvent::BossRVLanded { .. } => Some((AudioKind::Effect, "sounds/RVthrow.ogg")),
        GameEvent::BossHeadHydraulic { .. } => Some((AudioKind::Effect, "sounds/hydraulic.ogg")),
        GameEvent::BossAttackWindup { .. } => {
            Some((AudioKind::Effect, "sounds/bossboulderattack.ogg"))
        }
        GameEvent::BossProjectileStarted { .. } => {
            Some((AudioKind::Effect, "sounds/hydraulic_short.ogg"))
        }
        GameEvent::BossDamageExplosion { .. } => Some((AudioKind::Effect, "sounds/explosion.ogg")),
        GameEvent::PlantSpecialTriggered {
            plant_type: neopvz_core::PlantType::Other(14),
            ..
        } => Some((AudioKind::Effect, "sounds/frozen.ogg")),
        GameEvent::PlantSpecialTriggered {
            plant_type: neopvz_core::PlantType::Other(11),
            ..
        } => Some((AudioKind::Effect, "sounds/gravebusterchomp.ogg")),
        GameEvent::PlantSpecialTriggered {
            plant_type: neopvz_core::PlantType::Other(35),
            ..
        } => Some((AudioKind::Effect, "sounds/coffee.ogg")),
        GameEvent::TangleKelpGrabStarted { .. } => Some((AudioKind::Effect, "sounds/floop.ogg")),
        GameEvent::TangleKelpWaterEntry { .. } => {
            Some((AudioKind::Effect, "sounds/zombiesplash.ogg"))
        }
        GameEvent::PotatoMineArmed { .. } => Some((AudioKind::Effect, "sounds/dirt_rise.ogg")),
        GameEvent::DiggerSurfaced { .. } => Some((AudioKind::Effect, "sounds/dirt_rise.ogg")),
        GameEvent::MetalStolen { .. } => Some((AudioKind::Effect, "sounds/magnetshroom.ogg")),
        GameEvent::VehicleDisabled { .. } => Some((AudioKind::Effect, "sounds/balloon_pop.ogg")),
        GameEvent::VehicleExploded { .. } => Some((AudioKind::Effect, "sounds/explosion.ogg")),
        GameEvent::PlantSpecialTriggered {
            plant_type: neopvz_core::PlantType::Other(21),
            ..
        } => Some((AudioKind::Effect, "sounds/throw.ogg")),
        GameEvent::PlantFired {
            plant_type: neopvz_core::PlantType::Other(10),
            ..
        } => Some((AudioKind::Effect, "sounds/fume.ogg")),
        GameEvent::PlantFired {
            plant_type: neopvz_core::PlantType::Other(42),
            ..
        } => None,
        GameEvent::PlantFired { variant, .. } => Some((
            AudioKind::Effect,
            if *variant < 3 {
                "sounds/throw.ogg"
            } else {
                "sounds/throw2.ogg"
            },
        )),
        GameEvent::PlantSpecialTriggered {
            plant_type: neopvz_core::PlantType::Other(4),
            ..
        } => Some((AudioKind::Effect, "sounds/potato_mine.ogg")),
        GameEvent::PlantSpecialTriggered {
            plant_type: neopvz_core::PlantType::Other(2),
            ..
        } => Some((AudioKind::Effect, "sounds/cherrybomb.ogg")),
        GameEvent::PlantSpecialTriggered {
            plant_type: neopvz_core::PlantType::Other(49),
            ..
        } => Some((AudioKind::Effect, "sounds/cherrybomb.ogg")),
        GameEvent::PlantSpecialTriggered {
            plant_type: neopvz_core::PlantType::Other(20),
            ..
        } => Some((AudioKind::Effect, "sounds/jalapeno.ogg")),
        GameEvent::PlantSpecialTriggered {
            plant_type: neopvz_core::PlantType::Other(15),
            ..
        } => Some((AudioKind::Effect, "sounds/doomshroom.ogg")),
        GameEvent::PlantSpecialTriggered {
            plant_type: neopvz_core::PlantType::Other(17),
            ..
        } => Some((AudioKind::Effect, "sounds/gargantuar_thump.ogg")),
        GameEvent::SquashHumStarted { variant, .. } => Some((
            AudioKind::Effect,
            if *variant < 2 {
                "sounds/squash_hmm.ogg"
            } else {
                "sounds/squash_hmm2.ogg"
            },
        )),
        GameEvent::PlantSpecialTriggered {
            plant_type: neopvz_core::PlantType::Other(6),
            ..
        } => Some((AudioKind::Effect, "sounds/bigchomp.ogg")),
        GameEvent::BloverTriggered { .. } => Some((AudioKind::Effect, "sounds/blover.ogg")),
        GameEvent::ZombieHypnotized { .. } => {
            Some((AudioKind::Effect, "sounds/mindcontrolled.ogg"))
        }
        GameEvent::JackboxExploded { .. } => Some((AudioKind::Effect, "sounds/explosion.ogg")),
        GameEvent::JackboxBoing { .. } => Some((AudioKind::Effect, "sounds/boing.ogg")),
        GameEvent::JackboxSurprise { variant, .. } => Some((
            AudioKind::Effect,
            if *variant < 2 {
                "sounds/jack_surprise.ogg"
            } else {
                "sounds/jack_surprise2.ogg"
            },
        )),
        GameEvent::DancerRumble { .. } => Some((AudioKind::Effect, "sounds/dancer.ogg")),
        GameEvent::ZombieYuckSound { variant, .. } => Some((
            AudioKind::Effect,
            if *variant < 2 {
                "sounds/yuck.ogg"
            } else {
                "sounds/yuck2.ogg"
            },
        )),
        GameEvent::BrainFinished { .. } => Some((AudioKind::Effect, "sounds/gulp.ogg")),
        GameEvent::ImpThrown { .. } => Some((AudioKind::Effect, "sounds/swing.ogg")),
        GameEvent::ZombieSpawned {
            zombie_type: neopvz_core::ZombieType::DolphinRider,
            ..
        } => Some((AudioKind::Effect, "sounds/dolphin_appears.ogg")),
        GameEvent::ZombieSpawned {
            zombie_type: neopvz_core::ZombieType::Zamboni,
            ..
        } => Some((AudioKind::Effect, "sounds/zamboni.ogg")),
        GameEvent::ZombieSpawned {
            zombie_type: neopvz_core::ZombieType::Balloon,
            ..
        } => Some((AudioKind::Effect, "sounds/ballooninflate.ogg")),
        GameEvent::ZombieSpawned {
            zombie_type: neopvz_core::ZombieType::BackupDancer,
            ..
        } => Some((AudioKind::Effect, "sounds/gravestone_rumble.ogg")),
        GameEvent::ZombieGraveRumble { .. } => {
            Some((AudioKind::Effect, "sounds/gravestone_rumble.ogg"))
        }
        GameEvent::ZombieWhackRise { .. } => Some((AudioKind::Effect, "sounds/dirt_rise.ogg")),
        GameEvent::LadderPlaced { .. } => Some((AudioKind::Effect, "sounds/ladder_zombie.ogg")),
        GameEvent::BungeeScream { variant, .. } => Some((
            AudioKind::Effect,
            match variant {
                0 => "sounds/bungee_scream.ogg",
                1 => "sounds/bungee_scream2.ogg",
                _ => "sounds/bungee_scream3.ogg",
            },
        )),
        GameEvent::BungeePlantLifted { .. } => Some((AudioKind::Effect, "sounds/floop.ogg")),
        GameEvent::BungeeGrassStep { .. } => Some((AudioKind::Effect, "sounds/grassstep.ogg")),
        GameEvent::ZombieFallingSound { variant, .. } => Some((
            AudioKind::Effect,
            if *variant == 0 {
                "sounds/zombie_falling_1.ogg"
            } else {
                "sounds/zombie_falling_2.ogg"
            },
        )),
        GameEvent::ZombieSongStarted {
            zombie_type: neopvz_core::ZombieType::Jackbox,
            ..
        } => Some((AudioKind::Effect, "sounds/jackinthebox.ogg")),
        GameEvent::ZombieSongStarted {
            zombie_type: neopvz_core::ZombieType::Digger,
            ..
        } => Some((AudioKind::Effect, "sounds/digger_zombie.ogg")),
        GameEvent::ZombieGroaned {
            family, variant, ..
        } => Some((
            AudioKind::Effect,
            match family {
                ZombieGroanFamily::Low => {
                    ["sounds/lowgroan.ogg", "sounds/lowgroan2.ogg"][usize::from(*variant)]
                }
                ZombieGroanFamily::Normal => [
                    "sounds/groan.ogg",
                    "sounds/groan2.ogg",
                    "sounds/groan3.ogg",
                    "sounds/groan4.ogg",
                    "sounds/groan5.ogg",
                    "sounds/groan6.ogg",
                ][usize::from(*variant)],
                ZombieGroanFamily::Brains => [
                    "sounds/groan.ogg",
                    "sounds/groan2.ogg",
                    "sounds/groan3.ogg",
                    "sounds/groan4.ogg",
                    "sounds/groan5.ogg",
                    "sounds/groan6.ogg",
                    "sounds/sukhbir4.ogg",
                    "sounds/sukhbir5.ogg",
                    "sounds/sukhbir6.ogg",
                ][usize::from(*variant)],
                ZombieGroanFamily::Sukhbir => [
                    "sounds/groan.ogg",
                    "sounds/groan2.ogg",
                    "sounds/groan3.ogg",
                    "sounds/groan4.ogg",
                    "sounds/groan5.ogg",
                    "sounds/groan6.ogg",
                    "sounds/sukhbir.ogg",
                    "sounds/sukhbir2.ogg",
                    "sounds/sukhbir3.ogg",
                ][usize::from(*variant)],
            },
        )),
        GameEvent::ZombieChew { soft: true, .. } => {
            Some((AudioKind::Effect, "sounds/chompsoft.ogg"))
        }
        GameEvent::ZombieChew { variant, .. } => Some((
            AudioKind::Effect,
            if *variant == 0 {
                "sounds/chomp.ogg"
            } else {
                "sounds/chomp2.ogg"
            },
        )),
        GameEvent::ZombieDeathSound {
            zombie_type: neopvz_core::ZombieType::Boss,
            ..
        } => Some((AudioKind::Effect, "sounds/bossexplosion.ogg")),
        GameEvent::ZombieDeathSound {
            zombie_type:
                neopvz_core::ZombieType::Gargantuar | neopvz_core::ZombieType::Gigagargantuar,
            ..
        } => Some((AudioKind::Effect, "sounds/gargantudeath.ogg")),
        GameEvent::ZombieShieldHit { variant, .. } => Some((
            AudioKind::Effect,
            if *variant == 0 {
                "sounds/shieldhit.ogg"
            } else {
                "sounds/shieldhit2.ogg"
            },
        )),
        GameEvent::ZombieBodyPartLost { .. } => Some((AudioKind::Effect, "sounds/limbs_pop.ogg")),
        GameEvent::BalloonPopped { .. } => Some((AudioKind::Effect, "sounds/balloon_pop.ogg")),
        GameEvent::ZombieNewspaperRipped { .. } => {
            Some((AudioKind::Effect, "sounds/newspaper_rip.ogg"))
        }
        GameEvent::ZombieNewspaperRarrgh { variant, .. } => Some((
            AudioKind::Effect,
            if *variant < 2 {
                "sounds/newspaper_rarrgh.ogg"
            } else {
                "sounds/newspaper_rarrgh2.ogg"
            },
        )),
        GameEvent::PoleVaultGrassStep { .. } => Some((AudioKind::Effect, "sounds/grassstep.ogg")),
        GameEvent::PoleVaultSound { .. } => Some((AudioKind::Effect, "sounds/polevault.ogg")),
        GameEvent::PogoBounceSound { .. } => Some((AudioKind::Effect, "sounds/pogo_zombie.ogg")),
        GameEvent::DolphinJumpStarted { .. } => {
            Some((AudioKind::Effect, "sounds/dolphin_before_jumping.ogg"))
        }
        GameEvent::ZombieEnteredPool { variant, .. } => Some((
            AudioKind::Effect,
            if *variant == 0 {
                "sounds/plant_water.ogg"
            } else {
                "sounds/zombie_entering_water.ogg"
            },
        )),
        GameEvent::ZombieChilled { .. } => Some((AudioKind::Effect, "sounds/frozen.ogg")),
        GameEvent::VaseBroken { .. } => Some((AudioKind::Effect, "sounds/vase_breaking.ogg")),
        GameEvent::RakeTriggered { .. } => Some((AudioKind::Effect, "sounds/swing.ogg")),
        GameEvent::JumpBlocked { .. } => Some((AudioKind::Effect, "sounds/bonk.ogg")),
        GameEvent::UmbrellaDeflected { .. } => Some((AudioKind::Effect, "sounds/boing.ogg")),
        GameEvent::UmbrellaTriggered { .. } => Some((AudioKind::Effect, "sounds/throw2.ogg")),
        GameEvent::CobCannonFired { .. } => Some((AudioKind::Effect, "sounds/coblaunch.ogg")),
        GameEvent::ProjectileFired {
            projectile_type: neopvz_core::ProjectileType::Other(1),
            ..
        } => Some((AudioKind::Effect, "sounds/basketball.ogg")),
        GameEvent::ProjectileIgnited { .. } => Some((AudioKind::Effect, "sounds/firepea.ogg")),
        GameEvent::ProjectileWarmed { .. } => Some((AudioKind::Effect, "sounds/throw.ogg")),
        GameEvent::PortalOpened { .. } => Some((AudioKind::Effect, "sounds/portal.ogg")),
        GameEvent::ProjectileImpact { kind, variant, .. } => Some((
            AudioKind::Effect,
            match kind {
                ProjectileImpactSound::Splat => match variant {
                    0 => "sounds/splat.ogg",
                    1 => "sounds/splat2.ogg",
                    _ => "sounds/splat3.ogg",
                },
                ProjectileImpactSound::Kernel => {
                    if *variant == 0 {
                        "sounds/kernelpult.ogg"
                    } else {
                        "sounds/kernelpult2.ogg"
                    }
                }
                ProjectileImpactSound::Butter => "sounds/butter.ogg",
                ProjectileImpactSound::Ignite => {
                    if *variant < 3 {
                        "sounds/ignite.ogg"
                    } else {
                        "sounds/ignite2.ogg"
                    }
                }
                ProjectileImpactSound::Melon => {
                    if *variant == 0 {
                        "sounds/melonimpact.ogg"
                    } else {
                        "sounds/melonimpact2.ogg"
                    }
                }
                ProjectileImpactSound::Shield => {
                    if *variant == 0 {
                        "sounds/shieldhit.ogg"
                    } else {
                        "sounds/shieldhit2.ogg"
                    }
                }
                ProjectileImpactSound::Plastic => {
                    if *variant == 0 {
                        "sounds/plastichit.ogg"
                    } else {
                        "sounds/plastichit2.ogg"
                    }
                }
            },
        )),
        GameEvent::MowerTriggered { pool, .. } => Some((
            AudioKind::Effect,
            if *pool {
                "sounds/pool_cleaner.ogg"
            } else {
                "sounds/lawnmower.ogg"
            },
        )),
        GameEvent::MowerZombieHit { pool: true, .. } => {
            Some((AudioKind::Effect, "sounds/shoop.ogg"))
        }
        GameEvent::MowerZombieHit { variant, .. } => Some((
            AudioKind::Effect,
            match variant {
                0 => "sounds/splat.ogg",
                1 => "sounds/splat2.ogg",
                _ => "sounds/splat3.ogg",
            },
        )),
        GameEvent::MowerSquished { variant, .. } => Some((
            AudioKind::Effect,
            if *variant == 0 {
                "sounds/chomp.ogg"
            } else {
                "sounds/chomp2.ogg"
            },
        )),
        GameEvent::MowerEnteredPool { variant, .. } => Some((
            AudioKind::Effect,
            if *variant == 0 {
                "sounds/plant_water.ogg"
            } else {
                "sounds/zombie_entering_water.ogg"
            },
        )),
        GameEvent::MowerExitedPool { .. } => Some((AudioKind::Effect, "sounds/plant_water.ogg")),
        GameEvent::Paused => Some((AudioKind::Effect, "sounds/pause.ogg")),
        GameEvent::GameLost { .. } => Some((AudioKind::Music, "sounds/losemusic.ogg")),
        GameEvent::GameLostChomp { variant } => Some((
            AudioKind::Effect,
            if *variant == 0 {
                "sounds/chomp.ogg"
            } else {
                "sounds/chomp2.ogg"
            },
        )),
        GameEvent::GameLostScream => Some((AudioKind::Effect, "sounds/scream.ogg")),
        GameEvent::GameWon => Some((AudioKind::Music, "sounds/winmusic.ogg")),
        _ => None,
    }
}

fn planting_audio_path(
    scene: SceneKind,
    challenge: neopvz_core::ChallengeKind,
    plant_type: PlantType,
    row: u8,
    variant: u8,
) -> &'static str {
    if scene == SceneKind::Garden {
        "sounds/ceramic.ogg"
    } else if matches!(plant_type, PlantType::Other(35)) {
        "sounds/plant.ogg"
    } else if challenge == neopvz_core::ChallengeKind::Zombiquarium
        || (matches!(scene, SceneKind::Pool | SceneKind::Fog) && matches!(row, 2 | 3))
    {
        "sounds/plant_water.ogg"
    } else if variant == 0 {
        "sounds/plant.ogg"
    } else {
        "sounds/plant2.ogg"
    }
}

fn planting_audio_for_event(
    event: &GameEvent,
    state: &neopvz_core::GameState,
) -> Option<(AudioKind, &'static str)> {
    let (plant_type, row, variant) = match event {
        GameEvent::PlantPlaced {
            plant_type,
            row,
            variant,
            ..
        } => (*plant_type, *row, *variant),
        GameEvent::ImitaterMorphed {
            entity, plant_type, ..
        } => (
            *plant_type,
            state
                .board
                .plants
                .iter()
                .find(|plant| plant.id == *entity)
                .map(|plant| plant.row)?,
            0,
        ),
        _ => return None,
    };
    Some((
        AudioKind::Effect,
        planting_audio_path(state.scene, state.challenge.kind, plant_type, row, variant),
    ))
}

fn terminal_audio_for_event(
    event: &GameEvent,
    state: &neopvz_core::GameState,
) -> Option<(AudioKind, &'static str)> {
    (matches!(event, GameEvent::GameWon) && state.mode == ModeKind::Adventure && state.level == 50)
        .then_some((AudioKind::Effect, "sounds/finalfanfare.ogg"))
}

fn audio_sequence_for_event(
    event: &GameEvent,
    state: &neopvz_core::GameState,
) -> [Option<(AudioKind, &'static str)>; 2] {
    let primary = planting_audio_for_event(event, state)
        .or_else(|| terminal_audio_for_event(event, state))
        .or_else(|| audio_for_event(event));
    let companion = if state.challenge.kind == ChallengeKind::WallnutBowling
        && matches!(event, GameEvent::PlantPlaced { .. })
    {
        Some((AudioKind::Effect, "sounds/bowling.ogg"))
    } else if matches!(event, GameEvent::GameWon)
        && state.mode == ModeKind::Adventure
        && matches!(state.level, 10 | 20 | 30 | 40 | 50)
    {
        Some((AudioKind::Effect, "sounds/paper.ogg"))
    } else {
        audio_companion_for_event(event)
    };
    let companion_before_planting = matches!(
        event,
        GameEvent::PlantPlaced {
            plant_type: PlantType::Other(2 | 20),
            ..
        } | GameEvent::ImitaterMorphed {
            plant_type: PlantType::Other(2 | 20),
            ..
        } | GameEvent::PlantPlaced {
            plant_type: PlantType::Other(25),
            ..
        }
    );
    if companion_before_planting {
        [companion, primary]
    } else {
        [primary, companion]
    }
}

fn audio_companion_for_event(event: &GameEvent) -> Option<(AudioKind, &'static str)> {
    match event {
        GameEvent::PlantPlaced {
            plant_type: neopvz_core::PlantType::Other(2 | 20),
            ..
        }
        | GameEvent::ImitaterMorphed {
            plant_type: neopvz_core::PlantType::Other(2 | 20),
            ..
        } => Some((AudioKind::Effect, "sounds/reverse_explosion.ogg")),
        GameEvent::PlantPlaced {
            plant_type: neopvz_core::PlantType::Other(25),
            ..
        } => Some((AudioKind::Effect, "sounds/plantern.ogg")),
        GameEvent::PlantSpecialTriggered {
            plant_type: neopvz_core::PlantType::Other(2),
            ..
        }
        | GameEvent::PlantSpecialTriggered {
            plant_type: neopvz_core::PlantType::Other(20),
            ..
        } => Some((AudioKind::Effect, "sounds/juicy.ogg")),
        GameEvent::PlantSpecialTriggered {
            plant_type: neopvz_core::PlantType::Other(49),
            ..
        } => Some((AudioKind::Effect, "sounds/bowlingimpact2.ogg")),
        GameEvent::UmbrellaDeflected { .. } => Some((AudioKind::Effect, "sounds/throw2.ogg")),
        GameEvent::DiggerSurfaced { .. } => Some((AudioKind::Effect, "sounds/wakeup.ogg")),
        GameEvent::GardenWatered { .. }
        | GameEvent::GardenFertilized { .. }
        | GameEvent::GardenBecameHappy { .. } => Some((AudioKind::Effect, "sounds/throw.ogg")),
        GameEvent::ImpThrown { imp_variant, .. } => Some((
            AudioKind::Effect,
            if *imp_variant == 0 {
                "sounds/imp.ogg"
            } else {
                "sounds/imp2.ogg"
            },
        )),
        GameEvent::DolphinJumpStarted { .. } => Some((AudioKind::Effect, "sounds/plant_water.ogg")),
        GameEvent::ZombieDeathSound {
            zombie_type: neopvz_core::ZombieType::Boss,
            ..
        } => Some((AudioKind::Effect, "sounds/gargantudeath.ogg")),
        GameEvent::ZombieFallingSound {
            zombie_type:
                neopvz_core::ZombieType::Gargantuar | neopvz_core::ZombieType::Gigagargantuar,
            ..
        } => Some((AudioKind::Effect, "sounds/gargantuar_thump.ogg")),
        GameEvent::PlantFired {
            plant_type: neopvz_core::PlantType::Other(5 | 44),
            ..
        } => Some((AudioKind::Effect, "sounds/snow_pea_sparkles.ogg")),
        GameEvent::PlantFired {
            plant_type: neopvz_core::PlantType::Other(8 | 13 | 24),
            ..
        } => Some((AudioKind::Effect, "sounds/puff.ogg")),
        _ => None,
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.initialize(event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = Some(position);
            }
            WindowEvent::CursorLeft { .. } => self.cursor_position = None,
            WindowEvent::MouseWheel { delta, .. }
                if self.game.state().scene == SceneKind::SeedChooser =>
            {
                let rows = seed_chooser_grid_slots(self.seed_chooser_has_seven_rows())
                    .div_ceil(8)
                    .saturating_sub(6);
                let direction = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y,
                    MouseScrollDelta::PixelDelta(position) => position.y as f32,
                };
                if direction < 0.0 {
                    self.seed_chooser_scroll = self.seed_chooser_scroll.saturating_add(1).min(rows);
                } else if direction > 0.0 {
                    self.seed_chooser_scroll = self.seed_chooser_scroll.saturating_sub(1);
                }
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => self.handle_mouse_click(button),
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button,
                ..
            } => {
                if self.game.state().scene == SceneKind::Title {
                    let over_start = self.title_start_hovered();
                    if self.title_load_state.mouse_release(button, over_start) {
                        self.start_scene(SceneKind::AdventureSelect);
                    }
                } else {
                    self.handle_beghouled_mouse_release(button);
                }
            }
            WindowEvent::Focused(false) => self.title_load_state.pressed_button = None,
            WindowEvent::KeyboardInput { event, .. }
                if event.state == ElementState::Pressed && !event.repeat =>
            {
                self.handle_key(event_loop, event.physical_key);
                if let Some(renderer) = &self.renderer {
                    renderer.window().request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                self.advance_simulation();
                let frame = self.render_frame();
                if let Some(path) = self.capture_path.as_deref() {
                    let capture_result = self
                        .renderer
                        .as_mut()
                        .ok_or_else(|| "renderer is not initialized".to_owned())
                        .and_then(|renderer| {
                            renderer
                                .capture_frame(&frame)
                                .map_err(|error| error.to_string())
                        })
                        .and_then(|captured| {
                            image::save_buffer_with_format(
                                path,
                                &captured.rgba8,
                                captured.width,
                                captured.height,
                                image::ColorType::Rgba8,
                                image::ImageFormat::Png,
                            )
                            .map_err(|error| error.to_string())
                        });
                    match capture_result {
                        Ok(()) => {
                            tracing::info!(path = ?path, "frame captured");
                            self.capture_path = None;
                        }
                        Err(error) => {
                            tracing::error!(%error, path = ?path, "frame capture failed");
                        }
                    }
                    event_loop.exit();
                    return;
                }
                let render_result = self
                    .renderer
                    .as_mut()
                    .map(|renderer| renderer.render(&frame));
                if let Some(Err(error)) = render_result {
                    tracing::error!(%error, "rendering failed");
                    event_loop.exit();
                    return;
                }
                if let Some(renderer) = &self.renderer {
                    renderer.window().request_redraw();
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(renderer) = &self.renderer {
            renderer.window().request_redraw();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn options_volume_uses_a_bounded_decibel_scale() {
        assert_eq!(volume_percent_to_decibels(0), -60.0);
        assert_eq!(volume_percent_to_decibels(100), 0.0);
        assert!((-7.0..-5.0).contains(&volume_percent_to_decibels(50)));
    }

    #[test]
    fn projectile_particle_trails_use_source_definitions_and_offsets() {
        assert_eq!(
            projectile_particle_trail(ProjectileType::SnowPea),
            Some(("SnowPeaTrail", 8.0, 13.0))
        );
        assert_eq!(
            projectile_particle_trail(ProjectileType::Puff),
            Some(("PuffShroomTrail", 13.0, 13.0))
        );
        assert_eq!(projectile_particle_trail(ProjectileType::Pea), None);
    }

    #[test]
    fn projectile_impacts_queue_source_particle_offsets() {
        let event = |projectile_type, kind| GameEvent::ProjectileImpact {
            projectile: 7,
            projectile_type,
            zombie: Some(8),
            kind,
            variant: 0,
        };
        let mut anchors = BoardVisualAnchors::default();
        anchors.projectiles.insert(
            7,
            BoardVisualAnchor {
                x: 100.0,
                y: 200.0,
                previous_x: 90.0,
                previous_y: 180.0,
                row: 2,
                zombie_type: None,
            },
        );
        for (projectile_type, kind, expected) in [
            (
                ProjectileType::Pea,
                ProjectileImpactSound::Splat,
                vec![("PeaSplat", 97.0, 212.0, 13)],
            ),
            (
                ProjectileType::Pea,
                ProjectileImpactSound::Shield,
                vec![("PeaSplat", 97.0, 212.0, 13)],
            ),
            (
                ProjectileType::SnowPea,
                ProjectileImpactSound::Splat,
                vec![("SnowPeaSplat", 97.0, 212.0, 13)],
            ),
            (
                ProjectileType::Puff,
                ProjectileImpactSound::Splat,
                vec![("PuffSplat", 92.0, 212.0, 13)],
            ),
            (
                ProjectileType::Star,
                ProjectileImpactSound::Splat,
                vec![("StarSplat", 112.0, 212.0, 13)],
            ),
            (
                ProjectileType::Cabbage,
                ProjectileImpactSound::Splat,
                vec![("CabbageSplat", 52.0, 203.0, 13)],
            ),
            (
                ProjectileType::Butter,
                ProjectileImpactSound::Butter,
                vec![("ButterSplat", 70.0, 243.0, 13)],
            ),
            (
                ProjectileType::Melon,
                ProjectileImpactSound::Melon,
                vec![("MelonImpact", 120.0, 210.0, 13)],
            ),
            (
                ProjectileType::WinterMelon,
                ProjectileImpactSound::Melon,
                vec![("WinterMelonImpact", 120.0, 210.0, 13)],
            ),
            (
                ProjectileType::ZombiePea,
                ProjectileImpactSound::Splat,
                vec![("PeaSplat", 97.0, 217.0, 13)],
            ),
            (
                ProjectileType::Cob,
                ProjectileImpactSound::Splat,
                vec![
                    ("BlastMark", 180.0, 240.0, 2),
                    ("PopcornSplash", 180.0, 240.0, 13),
                ],
            ),
            (
                ProjectileType::Pea,
                ProjectileImpactSound::Plastic,
                Vec::new(),
            ),
        ] {
            assert_eq!(
                particle_effects_for_event(
                    &event(projectile_type, kind),
                    SceneKind::Day,
                    &anchors,
                    &BoardVisualAnchors::default(),
                ),
                expected
            );
        }
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ProjectileImpact {
                    projectile: 7,
                    projectile_type: ProjectileType::Pea,
                    zombie: None,
                    kind: ProjectileImpactSound::Splat,
                    variant: 3,
                },
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("UmbrellaReflect", 120.0, 220.0, 13)]
        );

        anchors.projectiles.clear();
        anchors.zombies.insert(
            8,
            BoardVisualAnchor {
                x: 100.0,
                y: 200.0,
                previous_x: 100.0,
                previous_y: 200.0,
                row: 2,
                zombie_type: None,
            },
        );
        assert_eq!(
            particle_effects_for_event(
                &event(ProjectileType::SnowPea, ProjectileImpactSound::Splat),
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("SnowPeaSplat", 97.0, 212.0, 13)]
        );
    }

    #[test]
    fn planting_events_queue_source_particle_offsets() {
        let event = |plant_type, row, column| GameEvent::PlantPlaced {
            entity: 1,
            plant_type,
            row,
            column,
            sun_remaining: 0,
            variant: 0,
        };
        let empty = BoardVisualAnchors::default();

        assert_eq!(
            particle_effects_for_event(
                &event(PlantType::Peashooter, 0, 2),
                SceneKind::Day,
                &empty,
                &empty,
            ),
            vec![("Planting", 281.0, 194.0, 13)]
        );
        assert_eq!(
            particle_effects_for_event(
                &event(PlantType::Other(16), 2, 0),
                SceneKind::Pool,
                &empty,
                &empty,
            ),
            vec![("PlantingPool", 121.0, 389.0, 13)]
        );
        assert_eq!(
            particle_effects_for_event(
                &event(PlantType::Other(33), 1, 1),
                SceneKind::Roof,
                &empty,
                &empty,
            ),
            vec![("Planting", 201.0, 314.0, 13)]
        );
        assert_eq!(
            particle_effects_for_event(
                &event(PlantType::Other(25), 1, 1),
                SceneKind::Night,
                &empty,
                &empty,
            ),
            vec![
                ("Planting", 201.0, 284.0, 13),
                ("LanternShine", 200.0, 250.0, 10),
            ]
        );
        assert!(
            particle_effects_for_event(
                &event(PlantType::Other(35), 0, 0),
                SceneKind::Day,
                &empty,
                &empty,
            )
            .is_empty()
        );
    }

    #[test]
    fn plant_special_events_queue_source_particle_centers() {
        let mut anchors = BoardVisualAnchors::default();
        anchors.plants.insert(
            1,
            BoardVisualAnchor {
                x: 100.0,
                y: 200.0,
                previous_x: 100.0,
                previous_y: 200.0,
                row: 2,
                zombie_type: None,
            },
        );
        let event = |slot| GameEvent::PlantSpecialTriggered {
            entity: 1,
            plant_type: PlantType::Other(slot),
        };

        for (slot, name, z) in [
            (2, "Powie", 13),
            (4, "PotatoMine", 11),
            (14, "IceTrap", 13),
            (15, "Doom", 13),
        ] {
            assert_eq!(
                particle_effects_for_event(
                    &event(slot),
                    SceneKind::Night,
                    &anchors,
                    &BoardVisualAnchors::default(),
                ),
                vec![(name, 140.0, 240.0, z)]
            );
        }
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::PotatoMineArmed { entity: 1 },
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("PotatoMineRise", 140.0, 240.0, 10)]
        );
        assert_eq!(
            particle_effects_for_event(
                &event(17),
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("Dust_Squash", 140.0, 280.0, 13)]
        );
        assert!(
            particle_effects_for_event(
                &event(20),
                SceneKind::Night,
                &anchors,
                &BoardVisualAnchors::default(),
            )
            .is_empty()
        );
    }

    #[test]
    fn lifecycle_particle_events_use_source_names_offsets_and_anchor_fallbacks() {
        let mut anchors = BoardVisualAnchors::default();
        anchors.plants.insert(
            1,
            BoardVisualAnchor {
                x: 100.0,
                y: 200.0,
                previous_x: 100.0,
                previous_y: 200.0,
                row: 2,
                zombie_type: None,
            },
        );
        anchors.zombies.insert(
            2,
            BoardVisualAnchor {
                x: 300.0,
                y: 400.0,
                previous_x: 300.0,
                previous_y: 400.0,
                row: 2,
                zombie_type: Some(ZombieType::Zamboni),
            },
        );
        anchors.zombies.insert(
            3,
            BoardVisualAnchor {
                x: 500.0,
                y: 600.0,
                previous_x: 500.0,
                previous_y: 600.0,
                row: 2,
                zombie_type: Some(ZombieType::BackupDancer),
            },
        );

        let current = BoardVisualAnchors::default();
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ImitaterMorphed {
                    entity: 1,
                    plant_type: PlantType::Peashooter,
                },
                SceneKind::Day,
                &anchors,
                &current,
            ),
            vec![("ImitaterMorph", 140.0, 240.0, 13)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::PlantFired {
                    entity: 1,
                    plant_type: PlantType::Other(8),
                    variant: 0,
                },
                SceneKind::Day,
                &anchors,
                &current,
            ),
            vec![("PuffShroomMuzzle", 128.0, 218.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::GraveCleared {
                    entity: 1,
                    row: 2,
                    column: 0,
                },
                SceneKind::Night,
                &BoardVisualAnchors::default(),
                &anchors,
            ),
            vec![("GraveBusterDie", 140.0, 240.0, 13)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieSpawned {
                    entity: 3,
                    zombie_type: ZombieType::BackupDancer,
                    row: 2,
                    wave: 1,
                },
                SceneKind::Night,
                &BoardVisualAnchors::default(),
                &anchors,
            ),
            vec![("DancerRise", 560.0, 692.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::DiggerSurfaced { entity: 2 },
                SceneKind::Day,
                &anchors,
                &current,
            ),
            vec![("DiggerRise", 360.0, 500.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieEnteredPool {
                    entity: 2,
                    variant: 0,
                },
                SceneKind::Pool,
                &anchors,
                &current,
            ),
            vec![("PlantingPool", 360.0, 502.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::TangleKelpWaterEntry { entity: 1 },
                SceneKind::Pool,
                &anchors,
                &current,
            ),
            vec![("PlantingPool", 131.0, 264.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieGraveRumble { entity: 2 },
                SceneKind::Night,
                &anchors,
                &current,
            ),
            vec![("ZombieRise", 360.0, 492.0, 11)]
        );
        assert!(
            particle_effects_for_event(
                &GameEvent::ZombieGraveRumble { entity: 2 },
                SceneKind::Day,
                &anchors,
                &current,
            )
            .is_empty()
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieWhackRise { entity: 2 },
                SceneKind::Night,
                &anchors,
                &current,
            ),
            vec![("WhackAZombieRise", 360.0, 492.0, 11)]
        );
    }

    #[test]
    fn whack_bonk_starts_pow_at_the_source_click_offset() {
        let anchors = BoardVisualAnchors::default();
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::WhackHit {
                    x: 200,
                    y: 300,
                    sound: WhackHitSound::Bonk,
                    variant: 0,
                },
                SceneKind::Night,
                &anchors,
                &anchors,
            ),
            vec![("Pow", 197.0, 309.0, 30)]
        );
        assert!(
            particle_effects_for_event(
                &GameEvent::WhackHit {
                    x: 200,
                    y: 300,
                    sound: WhackHitSound::Shield,
                    variant: 0,
                },
                SceneKind::Night,
                &anchors,
                &anchors,
            )
            .is_empty()
        );
    }

    #[test]
    fn source_stage_and_shuffle_events_start_the_screen_flash() {
        let anchors = BoardVisualAnchors::default();
        for event in [
            GameEvent::PuzzleStageStarted { stage: 1 },
            GameEvent::BeghouledShuffled,
        ] {
            assert_eq!(
                particle_effects_for_event(&event, SceneKind::Day, &anchors, &anchors,),
                vec![("ScreenFlash", 400.0, 300.0, 30)]
            );
        }
    }

    #[test]
    fn portal_open_events_start_the_matching_source_particle() {
        let anchors = BoardVisualAnchors::default();
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::PortalOpened {
                    row: 2,
                    column: 5,
                    square: false,
                },
                SceneKind::Day,
                &anchors,
                &anchors,
            ),
            vec![("PortalCircle", 493.0, 261.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::PortalOpened {
                    row: 2,
                    column: 5,
                    square: true,
                },
                SceneKind::Day,
                &anchors,
                &anchors,
            ),
            vec![("PortalSquare", 485.0, 276.0, 11)]
        );
    }

    #[test]
    fn digger_spawn_and_mower_hit_start_source_particles() {
        let mut anchors = BoardVisualAnchors::default();
        anchors.zombies.insert(
            1,
            BoardVisualAnchor {
                x: 300.0,
                y: 400.0,
                previous_x: 300.0,
                previous_y: 400.0,
                row: 2,
                zombie_type: Some(ZombieType::Digger),
            },
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieSpawned {
                    entity: 1,
                    zombie_type: ZombieType::Digger,
                    row: 2,
                    wave: 1,
                },
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("DiggerTunnel", 360.0, 482.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::MowerZombieHit {
                    entity: 1,
                    pool: false,
                    variant: 0,
                },
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("MowerCloud", 410.0, 382.0, 11)]
        );
        anchors.zombies.insert(
            2,
            BoardVisualAnchor {
                zombie_type: Some(ZombieType::Normal),
                ..anchors.zombies[&1]
            },
        );
        assert!(
            particle_effects_for_event(
                &GameEvent::MowerZombieHit {
                    entity: 2,
                    pool: false,
                    variant: 0,
                },
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            )
            .is_empty()
        );
        anchors.zombies.insert(
            3,
            BoardVisualAnchor {
                zombie_type: Some(ZombieType::Boss),
                ..anchors.zombies[&1]
            },
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieDied { entity: 3 },
                SceneKind::Boss,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("BossExplosion", 700.0, 150.0, 11)]
        );
    }

    #[test]
    fn particle_variants_follow_vase_vehicle_armor_and_special_source_definitions() {
        let mut anchors = BoardVisualAnchors::default();
        anchors.plants.insert(
            1,
            BoardVisualAnchor {
                x: 100.0,
                y: 200.0,
                previous_x: 100.0,
                previous_y: 200.0,
                row: 2,
                zombie_type: None,
            },
        );
        let mut zombie = |id, zombie_type| {
            anchors.zombies.insert(
                id,
                BoardVisualAnchor {
                    x: 300.0,
                    y: 400.0,
                    previous_x: 300.0,
                    previous_y: 400.0,
                    row: 2,
                    zombie_type: Some(zombie_type),
                },
            );
        };
        zombie(2, ZombieType::Catapult);
        zombie(8, ZombieType::Zamboni);
        zombie(3, ZombieType::Conehead);
        zombie(4, ZombieType::Buckethead);
        zombie(5, ZombieType::ScreenDoor);
        zombie(6, ZombieType::Ladder);
        zombie(7, ZombieType::Newspaper);
        zombie(9, ZombieType::Pogo);

        let empty = BoardVisualAnchors::default();
        for (contents, leaf, expected) in [
            (
                VaseContents::Plant(PlantType::Peashooter),
                false,
                "VaseShatter",
            ),
            (
                VaseContents::Zombie(ZombieType::Normal),
                false,
                "VaseShatterZombie",
            ),
            (VaseContents::Sun(25), true, "VaseShatterLeaf"),
        ] {
            assert_eq!(
                particle_effects_for_event(
                    &GameEvent::VaseRevealed {
                        entity: 1,
                        row: 2,
                        column: 3,
                        contents,
                        leaf,
                    },
                    SceneKind::Day,
                    &empty,
                    &empty,
                ),
                vec![(expected, 340.0, 300.0, 13)]
            );
        }
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::VehicleExploded { entity: 2 },
                SceneKind::Day,
                &anchors,
                &empty,
            ),
            vec![("CatapultExplosion", 380.0, 442.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieDamageTierChanged { entity: 2, tier: 2 },
                SceneKind::Day,
                &anchors,
                &empty,
            ),
            vec![("ZamboniSmoke", 347.0, 459.0, 11)]
        );
        assert!(
            particle_effects_for_event(
                &GameEvent::ZombieDamageTierChanged { entity: 2, tier: 1 },
                SceneKind::Day,
                &anchors,
                &empty,
            )
            .is_empty()
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::VehicleDisabled { entity: 8 },
                SceneKind::Day,
                &anchors,
                &empty,
            ),
            vec![("ZamboniTire", 329.0, 496.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::PogoStickLost { entity: 9 },
                SceneKind::Day,
                &anchors,
                &empty,
            ),
            vec![("ZombiePogo", 360.0, 480.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieArmorLost { entity: 3 },
                SceneKind::Day,
                &anchors,
                &empty,
            ),
            vec![("ZombieTrafficCone", 360.0, 440.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieArmorLost { entity: 4 },
                SceneKind::Day,
                &anchors,
                &empty,
            ),
            vec![("ZombiePail", 360.0, 440.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieShieldLost { entity: 5 },
                SceneKind::Day,
                &anchors,
                &empty,
            ),
            vec![("ZombieDoor", 360.0, 450.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieShieldLost { entity: 6 },
                SceneKind::Day,
                &anchors,
                &empty,
            ),
            vec![("ZombieLadder", 331.0, 462.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieNewspaperRipped { entity: 7 },
                SceneKind::Day,
                &anchors,
                &empty,
            ),
            vec![("ZombieNewspaper", 360.0, 442.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieBodyPartLost {
                    entity: 7,
                    head: true,
                },
                SceneKind::Day,
                &anchors,
                &empty,
            ),
            vec![("ZombieNewspaperHead", 360.0, 440.0, 11)]
        );
    }

    #[test]
    fn mower_zamboni_uses_the_source_explosion_variant() {
        let mut anchors = BoardVisualAnchors::default();
        anchors.zombies.insert(
            1,
            BoardVisualAnchor {
                x: 300.0,
                y: 400.0,
                previous_x: 300.0,
                previous_y: 400.0,
                row: 2,
                zombie_type: Some(ZombieType::Zamboni),
            },
        );
        let events = [
            GameEvent::MowerZombieHit {
                entity: 1,
                pool: false,
                variant: 0,
            },
            GameEvent::VehicleExploded { entity: 1 },
        ];
        assert_eq!(
            particle_effects_for_event_batch(
                &events[1],
                &events,
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("ZamboniExplosion2", 380.0, 442.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &events[1],
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("ZamboniExplosion", 380.0, 442.0, 11)]
        );
    }

    #[test]
    fn zombie_head_particles_follow_source_special_and_pool_variants() {
        for (zombie_type, scene, expected) in [
            (ZombieType::Pogo, SceneKind::Day, "ZombiePogoHead"),
            (ZombieType::Balloon, SceneKind::Day, "ZombieBalloonHead"),
            (ZombieType::Normal, SceneKind::Pool, "ZombieHeadPool"),
        ] {
            let mut anchors = BoardVisualAnchors::default();
            anchors.zombies.insert(
                1,
                BoardVisualAnchor {
                    x: 300.0,
                    y: 400.0,
                    previous_x: 300.0,
                    previous_y: 400.0,
                    row: 2,
                    zombie_type: Some(zombie_type),
                },
            );
            if expected == "ZombieHeadPool" {
                anchors.zombie_in_pool.insert(1, true);
            }
            assert_eq!(
                particle_effects_for_event(
                    &GameEvent::ZombieBodyPartLost {
                        entity: 1,
                        head: true,
                    },
                    scene,
                    &anchors,
                    &BoardVisualAnchors::default(),
                ),
                vec![(expected, 360.0, 440.0, 11)]
            );
        }
    }

    #[test]
    fn fume_and_gloom_firing_start_source_cloud_particles() {
        let mut anchors = BoardVisualAnchors::default();
        anchors.plants.insert(
            1,
            BoardVisualAnchor {
                x: 100.0,
                y: 200.0,
                previous_x: 100.0,
                previous_y: 200.0,
                row: 2,
                zombie_type: None,
            },
        );
        for (plant_type, expected) in [
            (PlantType::Other(10), ("FumeCloud", 185.0, 231.0, 11)),
            (PlantType::Other(42), ("GloomCloud", 140.0, 240.0, 11)),
        ] {
            assert_eq!(
                particle_effects_for_event(
                    &GameEvent::PlantParticleTriggered {
                        entity: 1,
                        plant_type,
                    },
                    SceneKind::Night,
                    &anchors,
                    &BoardVisualAnchors::default(),
                ),
                vec![expected]
            );
        }
    }

    #[test]
    fn wallnut_chew_particles_follow_target_and_zombie_offsets() {
        let mut anchors = BoardVisualAnchors::default();
        anchors.plant_types.insert(7, PlantType::Other(3));
        anchors.plant_types.insert(8, PlantType::Other(23));
        anchors.plant_types.insert(9, PlantType::Other(21));
        for (entity, zombie_type) in [
            (1, ZombieType::Normal),
            (2, ZombieType::Balloon),
            (3, ZombieType::Imp),
            (4, ZombieType::Snorkel),
        ] {
            anchors.zombies.insert(
                entity,
                BoardVisualAnchor {
                    x: 300.0,
                    y: 400.0,
                    previous_x: 300.0,
                    previous_y: 400.0,
                    row: 2,
                    zombie_type: Some(zombie_type),
                },
            );
        }
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieChew {
                    entity: 1,
                    target: Some(7),
                    soft: false,
                    variant: 0,
                },
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("WallnutEatSmall", 337.0, 422.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieChew {
                    entity: 2,
                    target: Some(8),
                    soft: false,
                    variant: 0,
                },
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("WallnutEatSmall", 337.0, 469.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieChew {
                    entity: 3,
                    target: Some(8),
                    soft: false,
                    variant: 0,
                },
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("WallnutEatSmall", 361.0, 462.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieChew {
                    entity: 4,
                    target: Some(7),
                    soft: false,
                    variant: 0,
                },
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("WallnutEatSmall", 293.0, 492.0, 11)]
        );
        assert!(
            particle_effects_for_event(
                &GameEvent::ZombieChew {
                    entity: 1,
                    target: Some(9),
                    soft: false,
                    variant: 0,
                },
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            )
            .is_empty()
        );
        assert!(
            particle_effects_for_event(
                &GameEvent::ZombieChew {
                    entity: 1,
                    target: None,
                    soft: false,
                    variant: 0,
                },
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            )
            .is_empty()
        );
    }

    #[test]
    fn wallnut_crack_particles_start_only_when_health_crosses_source_tiers() {
        let mut anchors = BoardVisualAnchors::default();
        anchors.plants.insert(
            1,
            BoardVisualAnchor {
                x: 100.0,
                y: 200.0,
                previous_x: 100.0,
                previous_y: 200.0,
                row: 2,
                zombie_type: None,
            },
        );
        anchors.plant_types.insert(1, PlantType::Other(3));
        anchors.plant_health.insert(1, (4_000, 4_000));
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::PlantDamaged {
                    entity: 1,
                    damage: 1_400,
                    health_remaining: 2_600,
                },
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("WallnutEatLarge", 140.0, 210.0, 13)]
        );
        anchors.plant_health.insert(1, (2_600, 4_000));
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::PlantDamaged {
                    entity: 1,
                    damage: 1_300,
                    health_remaining: 1_300,
                },
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("WallnutEatLarge", 140.0, 210.0, 13)]
        );
        anchors.plant_types.insert(1, PlantType::Other(23));
        anchors.plant_health.insert(1, (8_000, 8_000));
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::PlantDamaged {
                    entity: 1,
                    damage: 2_700,
                    health_remaining: 5_300,
                },
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("WallnutEatLarge", 140.0, 178.0, 13)]
        );
        anchors.plant_health.insert(1, (5_300, 8_000));
        assert!(
            particle_effects_for_event(
                &GameEvent::PlantDamaged {
                    entity: 1,
                    damage: 100,
                    health_remaining: 5_200,
                },
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            )
            .is_empty()
        );
    }

    #[test]
    fn collected_pickups_start_source_present_and_award_particles() {
        let mut anchors = BoardVisualAnchors::default();
        anchors.coins.insert(7, (100.0, 200.0));
        let event = |coin_type| GameEvent::PickupCollected {
            entity: 7,
            coin_type,
            value: 1,
            coins_total: 0,
            sun_total: 0,
        };

        assert_eq!(
            particle_effects_for_event(
                &event(CoinType::PresentPlant),
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("PresentPickup", 130.0, 230.0, 13)]
        );
        assert_eq!(
            particle_effects_for_event(
                &event(CoinType::Note),
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![
                ("Starburst", 130.0, 230.0, 13),
                ("PresentPickup", 130.0, 230.0, 13),
            ]
        );
        assert_eq!(
            particle_effects_for_event(
                &event(CoinType::AwardPresent),
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            ),
            vec![("Starburst", 130.0, 230.0, 13)]
        );
        assert!(
            particle_effects_for_event(
                &event(CoinType::Gold),
                SceneKind::Day,
                &anchors,
                &BoardVisualAnchors::default(),
            )
            .is_empty()
        );
    }

    #[test]
    fn landing_arrow_particles_follow_source_names_and_image_offsets() {
        for (coin_type, expected) in [
            (CoinType::FinalSeedPacket, ("SeedPacket", 25.0, -25.0)),
            (CoinType::Silver, ("CoinPickupArrow", 32.0, -17.0)),
            (CoinType::Gold, ("CoinPickupArrow", 32.0, -17.0)),
            (CoinType::Trophy, ("AwardPickupArrow", 43.0, -29.0)),
            (CoinType::Shovel, ("AwardPickupArrow", 40.0, -20.0)),
            (CoinType::Almanac, ("AwardPickupArrow", 40.0, -20.0)),
            (CoinType::CarKeys, ("AwardPickupArrow", 40.0, -20.0)),
            (CoinType::Vase, ("AwardPickupArrow", 40.0, -10.0)),
            (CoinType::WateringCan, ("AwardPickupArrow", 40.0, -20.0)),
            (CoinType::Taco, ("AwardPickupArrow", 40.0, -20.0)),
            (CoinType::Note, ("AwardPickupArrow", 39.0, -34.0)),
            (CoinType::AwardMoneyBag, ("AwardPickupArrow", 47.0, -23.0)),
            (CoinType::AwardPresent, ("AwardPickupArrow", 40.0, -40.0)),
            (CoinType::AwardBagDiamond, ("AwardPickupArrow", 47.0, -23.0)),
            (
                CoinType::AwardSilverSunflower,
                ("AwardPickupArrow", 72.0, 34.0),
            ),
            (
                CoinType::AwardGoldSunflower,
                ("AwardPickupArrow", 72.0, 34.0),
            ),
            (CoinType::AwardChocolate, ("AwardPickupArrow", 28.0, -27.0)),
            (
                CoinType::PresentMinigames,
                ("AwardPickupArrow", 40.0, -40.0),
            ),
            (
                CoinType::PresentPuzzleMode,
                ("AwardPickupArrow", 40.0, -40.0),
            ),
            (
                CoinType::PresentSurvivalMode,
                ("AwardPickupArrow", 40.0, -40.0),
            ),
        ] {
            assert_eq!(coin_arrow_particle(coin_type, true), Some(expected));
        }
        assert_eq!(coin_arrow_particle(CoinType::Trophy, false), None);
        assert_eq!(coin_arrow_particle(CoinType::Diamond, true), None);
    }

    #[test]
    fn help_controls_use_the_source_note_button_regions() {
        assert!(help_selector_contains(700.0, 500.0));
        assert!(help_button_contains(350.0, 520.0));
        assert!(help_button_contains(700.0, 25.0));
        assert!(!help_button_contains(500.0, 520.0));
    }

    #[test]
    fn almanac_navigation_matches_source_button_regions() {
        assert!(almanac_plant_contains(150.0, 360.0));
        assert!(almanac_zombie_contains(500.0, 360.0));
        assert!(almanac_index_contains(50.0, 580.0));
        assert!(almanac_close_contains(700.0, 580.0));
        assert!(!almanac_close_contains(500.0, 580.0));
    }

    #[test]
    fn seed_chooser_layout_matches_source_unlock_threshold() {
        assert!(!seed_chooser_has_seven_rows_at(
            false,
            &[PlantType::Other(39)]
        ));
        assert!(seed_chooser_has_seven_rows_at(
            false,
            &[PlantType::Other(40)]
        ));
        assert!(seed_chooser_has_seven_rows_at(
            false,
            &[PlantType::Other(45)]
        ));
        assert!(!seed_chooser_has_seven_rows_at(
            false,
            &[PlantType::Other(46)]
        ));
        assert!(seed_chooser_has_seven_rows_at(true, &[]));

        assert_eq!(seed_chooser_grid_slots(false), 40);
        assert_eq!(seed_chooser_grid_slots(true), 48);
        assert_eq!(
            seed_chooser_slot_position(0, 0, false),
            Some((189.5, 171.5))
        );
        assert_eq!(seed_chooser_slot_position(0, 0, true), Some((189.5, 166.5)));
        assert_eq!(
            seed_chooser_slot_position(39, 0, false),
            Some((560.5, 463.5))
        );
        assert_eq!(
            seed_chooser_slot_position(47, 0, true),
            Some((560.5, 516.5))
        );
        assert_eq!(seed_chooser_slot_position(40, 0, false), None);
        assert_eq!(seed_chooser_slot_at(189.5, 166.5, 0, true), Some(0));
        assert_eq!(seed_chooser_slot_at(609.0, 516.5, 0, true), Some(47));
        assert_eq!(seed_chooser_slot_at(240.0, 166.5, 0, true), None);
        assert_eq!(seed_chooser_slot_at(189.5, 242.0, 0, false), None);
    }

    #[test]
    fn almanac_catalog_uses_source_counts_and_order() {
        assert_eq!(almanac_plant_type(0), Some(PlantType::Peashooter));
        assert_eq!(almanac_plant_type(1), Some(PlantType::Sunflower));
        assert_eq!(almanac_plant_type(48), Some(PlantType::Other(48)));
        assert_eq!(almanac_plant_type(ALMANAC_PLANT_COUNT), None);
        assert_eq!(almanac_zombie_type(0), Some(ZombieType::Normal));
        assert_eq!(almanac_zombie_type(1), Some(ZombieType::Flag));
        assert_eq!(almanac_zombie_type(2), Some(ZombieType::Conehead));
        assert_eq!(almanac_zombie_type(25), Some(ZombieType::Boss));
        assert_eq!(almanac_zombie_type(ALMANAC_ZOMBIE_COUNT), None);
    }

    #[test]
    fn almanac_cards_use_source_names_and_seed_definition_values() {
        assert_eq!(almanac_plant_name(0), "Peashooter");
        assert_eq!(almanac_plant_name(48), "Imitater");
        assert_eq!(almanac_zombie_name(25), "Dr. Zomboss");
        assert_eq!(PlantType::Peashooter.cost(), 100);
        assert_eq!(PlantType::Peashooter.refresh_time(), 750);
        assert_eq!(PlantType::Other(14).refresh_time(), 5_000);
        assert_eq!(PlantType::Other(ALMANAC_IMITATER_SLOT).cost(), 0);
    }

    #[test]
    fn almanac_zombie_visibility_follows_source_progression() {
        assert!(almanac_zombie_is_shown_at(
            1,
            0,
            false,
            false,
            ZombieType::Normal
        ));
        assert!(!almanac_zombie_is_shown_at(
            1,
            0,
            false,
            false,
            ZombieType::Conehead
        ));
        assert!(almanac_zombie_is_shown_at(
            18,
            0,
            false,
            false,
            ZombieType::Dancer
        ));
        assert!(!almanac_zombie_is_shown_at(
            18,
            0,
            false,
            false,
            ZombieType::BackupDancer
        ));
        assert!(almanac_zombie_is_shown_at(
            19,
            0,
            false,
            false,
            ZombieType::BackupDancer
        ));
        assert!(almanac_zombie_is_shown_at(
            18,
            0,
            false,
            true,
            ZombieType::BackupDancer
        ));
        assert!(!almanac_zombie_is_shown_at(
            40,
            0,
            false,
            false,
            ZombieType::Yeti
        ));
        assert!(almanac_zombie_is_shown_at(
            41,
            0,
            false,
            false,
            ZombieType::Yeti
        ));
        assert!(almanac_zombie_has_silhouette_at(
            41,
            0,
            false,
            ZombieType::Yeti
        ));
        assert!(!almanac_zombie_has_description_at(
            41,
            0,
            false,
            false,
            ZombieType::Yeti
        ));
        assert!(almanac_zombie_has_description_at(
            41,
            1,
            true,
            false,
            ZombieType::Yeti
        ));
        assert!(almanac_zombie_has_description_at(
            1,
            2,
            true,
            false,
            ZombieType::Yeti
        ));
        assert!(!almanac_zombie_has_description_at(
            18,
            0,
            false,
            false,
            ZombieType::BackupDancer
        ));
        assert!(almanac_zombie_has_description_at(
            18,
            0,
            false,
            true,
            ZombieType::BackupDancer
        ));
    }

    #[test]
    fn almanac_description_parser_resolves_source_format_tokens() {
        let strings = parse_lawn_strings(
            "[PEASHOOTER_DESCRIPTION]\r\nfirst line\r\n{SHORTLINE}\r\n{KEYWORD}Power{STAT} 20\r\n\r\n[OTHER]\r\nunused\r\n",
        );
        assert_eq!(
            almanac_lawn_description(&strings, "PEASHOOTER", "fallback"),
            "first line\nPower  20"
        );
        assert_eq!(almanac_plant_string_key(3), "WALL_NUT");
        assert_eq!(almanac_zombie_string_key(3), "POLE_VAULTING_ZOMBIE");
    }

    #[test]
    fn almanac_entry_hit_tests_follow_source_grids() {
        assert_eq!(almanac_plant_slot_at(27.0, 93.0), Some(0));
        assert_eq!(almanac_plant_slot_at(79.0, 171.0), Some(9));
        assert_eq!(almanac_plant_position(ALMANAC_IMITATER_SLOT), (20.0, 23.0));
        assert_eq!(
            almanac_plant_slot_at(21.0, 24.0),
            Some(ALMANAC_IMITATER_SLOT)
        );
        assert_eq!(almanac_plant_slot_at(440.0, 160.0), None);
        assert_eq!(almanac_zombie_index_at(23.0, 87.0), Some(0));
        assert_eq!(almanac_zombie_index_at(107.0, 167.0), Some(6));
        assert_eq!(almanac_zombie_position(25), (192.0, 486.0));
        assert_eq!(almanac_zombie_index_at(193.0, 487.0), Some(25));
        assert_eq!(almanac_zombie_index_at(700.0, 90.0), None);
    }

    #[test]
    fn game_over_buttons_use_source_result_regions() {
        assert!(game_over_try_again_contains(300.0, 420.0));
        assert!(game_over_main_menu_contains(650.0, 160.0));
        assert!(!game_over_try_again_contains(500.0, 420.0));
        assert!(!game_over_main_menu_contains(500.0, 160.0));
    }

    #[test]
    fn game_over_cutscene_holds_result_controls_until_source_end() {
        assert!(game_lost_cutscene_active(Some(0)));
        assert!(game_lost_cutscene_active(Some(GAME_LOST_DIALOG_TIME - 10)));
        assert!(!game_lost_cutscene_active(Some(GAME_LOST_DIALOG_TIME)));
        assert!(!game_lost_cutscene_active(None));
    }

    #[test]
    fn complete_buttons_use_source_award_screen_regions() {
        assert!(complete_continue_contains(400.0, 520.0));
        assert!(complete_main_menu_contains(700.0, 25.0));
        assert!(!complete_continue_contains(500.0, 520.0));
        assert!(!complete_main_menu_contains(600.0, 25.0));
    }

    #[test]
    fn credits_keep_source_phase_boundaries_and_button_regions() {
        assert_eq!(credits_phase_at(0).0, 0);
        assert_eq!(credits_phase_at(1333).0, 0);
        assert_eq!(credits_phase_at(1334).0, 1);
        let main2_end = (CREDITS_MAIN2_END_FRAME / CREDITS_ANIM_RATE).ceil() as u32;
        assert_eq!(credits_phase_at(main2_end).0, 2);
        assert_eq!(credits_phase_at(credits_end_update_count()).0, 3);
        assert!(!credits_controls_visible(credits_end_update_count()));
        assert!(credits_controls_visible(
            credits_end_update_count() + CREDITS_END_BUTTON_DELAY_UPDATES
        ));
        assert!(credits_replay_contains(40.0, 560.0));
        assert!(credits_main_menu_contains(400.0, 580.0));
        assert!(!credits_main_menu_contains(200.0, 580.0));
    }

    #[test]
    fn credits_particles_follow_source_timed_events() {
        let cues = (1..=credits_end_update_count())
            .flat_map(|current| credits_particle_cues(current - 1, current))
            .collect::<Vec<_>>();
        let names = cues
            .iter()
            .filter_map(|cue| match cue {
                CreditsParticleCue::Spawn(name) | CreditsParticleCue::StartFog(name) => Some(*name),
                CreditsParticleCue::StopFog => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            names
                .iter()
                .filter(|name| **name == "Credits_Strobe")
                .count(),
            58
        );
        assert_eq!(
            names
                .iter()
                .filter(|name| **name == "Credits_RaysWipe")
                .count(),
            1
        );
        assert_eq!(
            names
                .iter()
                .filter(|name| **name == "Credits_ZombieHeadWipe")
                .count(),
            2
        );
        assert_eq!(
            names.iter().filter(|name| **name == "Credits_fog").count(),
            1
        );
        assert_eq!(
            cues.iter()
                .filter(|cue| **cue == CreditsParticleCue::StopFog)
                .count(),
            1
        );
        assert!(credits_particle_cues(1333, 1334).is_empty());
        assert!(credits_particle_cues(2000, 2000).is_empty());

        let transform = |x| ReanimatorTransform {
            x,
            y: 0.0,
            skew_x: 0.0,
            skew_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            frame: 0.0,
            alpha: 1.0,
            image: None,
        };
        let definition = ReanimatorDefinition {
            fps: 7.0,
            tracks: vec![ReanimatorTrack {
                name: "Background2".to_owned(),
                transforms: vec![transform(0.0), transform(-10.0)],
            }],
        };
        assert_eq!(credits_fog_x(Some(&definition), 0.5), Some(851.0));
    }

    #[test]
    fn complete_award_text_and_note_assets_follow_source_categories() {
        assert_eq!(completion_award_text_slot(Some(CoinType::Shovel)), 1);
        assert_eq!(completion_award_text_slot(Some(CoinType::Note)), 6);
        assert_eq!(completion_award_text_slot(Some(CoinType::Trophy)), 7);
        assert_eq!(completion_award_text_slot(None), 8);
        assert_eq!(completion_note_asset(9), AWARD_NOTE1_IMAGE_ID);
        assert_eq!(completion_note_asset(19), AWARD_NOTE2_IMAGE_ID);
        assert_eq!(completion_note_asset(49), AWARD_NOTE4_IMAGE_ID);
    }

    #[test]
    fn contextual_planting_audio_covers_terrain_morph_and_explosion_order() {
        for (game, row, plant_type, variant, path) in [
            (
                Game::new(7, SceneKind::Day),
                0,
                PlantType::Peashooter,
                1,
                "sounds/plant2.ogg",
            ),
            (
                Game::new(7, SceneKind::Pool),
                2,
                PlantType::Peashooter,
                1,
                "sounds/plant_water.ogg",
            ),
            (
                Game::new(7, SceneKind::Pool),
                0,
                PlantType::Peashooter,
                1,
                "sounds/plant2.ogg",
            ),
            (
                Game::new(7, SceneKind::Pool),
                0,
                PlantType::Other(35),
                1,
                "sounds/plant.ogg",
            ),
            (
                Game::new(7, SceneKind::Fog),
                3,
                PlantType::Peashooter,
                0,
                "sounds/plant_water.ogg",
            ),
            (
                Game::new(7, SceneKind::Fog),
                5,
                PlantType::Peashooter,
                0,
                "sounds/plant.ogg",
            ),
            (
                Game::new_mode(7, ModeKind::ZenGarden, 0),
                0,
                PlantType::Peashooter,
                1,
                "sounds/ceramic.ogg",
            ),
            (
                Game::new_mode(7, ModeKind::MiniGame, 7),
                0,
                PlantType::Peashooter,
                1,
                "sounds/plant_water.ogg",
            ),
            (
                Game::new(7, SceneKind::Night),
                2,
                PlantType::Other(25),
                0,
                "sounds/plant.ogg",
            ),
        ] {
            let event = GameEvent::PlantPlaced {
                entity: 1,
                plant_type,
                row,
                column: 0,
                sun_remaining: 0,
                variant,
            };
            assert_eq!(
                planting_audio_for_event(&event, game.state()),
                Some((AudioKind::Effect, path))
            );
        }

        let mut game = Game::new(7, SceneKind::Day);
        let placement = game.advance(InputFrame {
            actions: vec![
                InputAction::SelectSeed { slot: 48 },
                InputAction::PlantImitater {
                    plant_slot: 1,
                    row: 0,
                    column: 0,
                },
            ],
        });
        let entity = placement
            .iter()
            .find_map(|event| match event {
                GameEvent::PlantPlaced {
                    entity,
                    plant_type: PlantType::Other(48),
                    ..
                } => Some(*entity),
                _ => None,
            })
            .expect("imitater placement event");
        let morph = (0..220).find_map(|_| {
            game.advance(InputFrame::default())
                .into_iter()
                .find(|event| {
                    matches!(
                        event,
                        GameEvent::ImitaterMorphed {
                            entity: actual,
                            plant_type: PlantType::Sunflower,
                        } if *actual == entity
                    )
                })
        });
        let morph = morph.expect("imitater morph event");
        assert_eq!(
            planting_audio_for_event(&morph, game.state()),
            Some((AudioKind::Effect, "sounds/plant.ogg"))
        );

        for (event, planting_path) in [
            (
                GameEvent::PlantPlaced {
                    entity,
                    plant_type: PlantType::Other(2),
                    row: 0,
                    column: 0,
                    sun_remaining: 0,
                    variant: 1,
                },
                "sounds/plant2.ogg",
            ),
            (
                GameEvent::ImitaterMorphed {
                    entity,
                    plant_type: PlantType::Other(20),
                },
                "sounds/plant.ogg",
            ),
        ] {
            let paths = audio_sequence_for_event(&event, game.state())
                .into_iter()
                .flatten()
                .map(|(_, path)| path)
                .collect::<Vec<_>>();
            assert_eq!(paths, vec!["sounds/reverse_explosion.ogg", planting_path]);
        }
        let plantern = GameEvent::PlantPlaced {
            entity: 1,
            plant_type: PlantType::Other(25),
            row: 2,
            column: 2,
            sun_remaining: 0,
            variant: 0,
        };
        assert_eq!(
            audio_sequence_for_event(&plantern, game.state())
                .into_iter()
                .flatten()
                .map(|(_, path)| path)
                .collect::<Vec<_>>(),
            vec!["sounds/plantern.ogg", "sounds/plant.ogg"]
        );
        let non_explosive_morph = GameEvent::ImitaterMorphed {
            entity,
            plant_type: PlantType::Sunflower,
        };
        assert_eq!(
            audio_sequence_for_event(&non_explosive_morph, game.state())
                .into_iter()
                .flatten()
                .map(|(_, path)| path)
                .collect::<Vec<_>>(),
            vec!["sounds/plant.ogg"]
        );

        let bowling = Game::new_mode(7, ModeKind::MiniGame, 1);
        let bowling_plant = GameEvent::PlantPlaced {
            entity: 1,
            plant_type: PlantType::Other(3),
            row: 2,
            column: 0,
            sun_remaining: 50,
            variant: 0,
        };
        assert_eq!(
            audio_sequence_for_event(&bowling_plant, bowling.state())
                .into_iter()
                .flatten()
                .map(|(_, path)| path)
                .collect::<Vec<_>>(),
            vec!["sounds/plant.ogg", "sounds/bowling.ogg"]
        );
    }

    #[test]
    fn maps_terminal_and_player_events_to_audio_resources() {
        assert_eq!(
            audio_for_event(&GameEvent::PlantShoveled { entity: 1 }),
            Some((AudioKind::Effect, "sounds/plant2.ogg"))
        );
        for (event, path) in [
            (
                GameEvent::PlantPlaced {
                    entity: 1,
                    plant_type: neopvz_core::PlantType::Peashooter,
                    row: 0,
                    column: 0,
                    sun_remaining: 50,
                    variant: 0,
                },
                "sounds/plant.ogg",
            ),
            (
                GameEvent::ZombieDeployed {
                    entity: 1,
                    zombie_type: neopvz_core::ZombieType::Normal,
                    row: 0,
                    column: 0,
                    sun_remaining: 100,
                    variant: 1,
                },
                "sounds/plant2.ogg",
            ),
            (
                GameEvent::ProjectileIgnited { projectile: 1 },
                "sounds/firepea.ogg",
            ),
            (
                GameEvent::ProjectileWarmed { projectile: 1 },
                "sounds/throw.ogg",
            ),
            (
                GameEvent::LootDropSound {
                    sound: LootDropSound::SpawnSun,
                },
                "sounds/throw.ogg",
            ),
            (
                GameEvent::LootDropSound {
                    sound: LootDropSound::ArtChallenge,
                },
                "sounds/diamond.au",
            ),
            (
                GameEvent::AwardCollectionSound {
                    sound: AwardCollectionSound::Coin,
                },
                "sounds/coin.ogg",
            ),
            (
                GameEvent::AwardCollectionSound {
                    sound: AwardCollectionSound::Diamond,
                },
                "sounds/diamond.au",
            ),
            (
                GameEvent::AwardCollectionSound {
                    sound: AwardCollectionSound::Seedlift,
                },
                "sounds/seedlift.ogg",
            ),
            (
                GameEvent::AwardCollectionSound {
                    sound: AwardCollectionSound::Tap2,
                },
                "sounds/tap2.ogg",
            ),
            (
                GameEvent::AwardCollectionSound {
                    sound: AwardCollectionSound::Shovel,
                },
                "sounds/shovel.ogg",
            ),
            (
                GameEvent::WeatherSound {
                    sound: WeatherSound::Rain,
                },
                "sounds/rain.ogg",
            ),
            (
                GameEvent::WeatherSound {
                    sound: WeatherSound::Thunder,
                },
                "sounds/thunder.ogg",
            ),
            (
                GameEvent::CoinLanded {
                    entity: 1,
                    coin_type: CoinType::Gold,
                },
                "sounds/moneyfalls.ogg",
            ),
        ] {
            assert_eq!(audio_for_event(&event), Some((AudioKind::Effect, path)));
        }
        assert_eq!(
            audio_for_event(&GameEvent::SeedSelected {
                slot: 1,
                plant_type: neopvz_core::PlantType::Peashooter,
            }),
            Some((AudioKind::Effect, "sounds/tap.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ReadySetPlant),
            Some((AudioKind::Effect, "sounds/readysetplant.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::InputRejected {
                action: InputAction::Pause,
                reason: neopvz_core::InputRejectReason::OutsideBoard,
            }),
            Some((AudioKind::Effect, "sounds/buzzer.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ChallengeAction {
                kind: ChallengeKind::SlotMachine,
                value: 1,
            }),
            Some((AudioKind::Effect, "sounds/slotmachine.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ChallengeAction {
                kind: ChallengeKind::SlotMachine,
                value: 0,
            }),
            None
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombiquariumSnorkelPurchased { variant: 0 }),
            Some((AudioKind::Effect, "sounds/plant_water.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombiquariumSnorkelPurchased { variant: 1 }),
            Some((AudioKind::Effect, "sounds/zombie_entering_water.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombiquariumBrainSlurped { zombie: 1, row: 2 }),
            Some((AudioKind::Effect, "sounds/slurp.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombiquariumZombieDied { entity: 1 }),
            Some((AudioKind::Effect, "sounds/zombaquarium_die.ogg"))
        );
        for (event, path) in [
            (GameEvent::WhackHammerSwung, "sounds/swing.ogg"),
            (
                GameEvent::WhackHit {
                    x: 0,
                    y: 0,
                    sound: WhackHitSound::Bonk,
                    variant: 0,
                },
                "sounds/bonk.ogg",
            ),
            (
                GameEvent::WhackHit {
                    x: 0,
                    y: 0,
                    sound: WhackHitSound::Shield,
                    variant: 0,
                },
                "sounds/shieldhit.ogg",
            ),
            (
                GameEvent::WhackHit {
                    x: 0,
                    y: 0,
                    sound: WhackHitSound::Shield,
                    variant: 1,
                },
                "sounds/shieldhit2.ogg",
            ),
            (
                GameEvent::WhackHit {
                    x: 0,
                    y: 0,
                    sound: WhackHitSound::Plastic,
                    variant: 0,
                },
                "sounds/plastichit.ogg",
            ),
            (
                GameEvent::WhackHit {
                    x: 0,
                    y: 0,
                    sound: WhackHitSound::Plastic,
                    variant: 1,
                },
                "sounds/plastichit2.ogg",
            ),
            (GameEvent::DancerRumble { entity: 1 }, "sounds/dancer.ogg"),
        ] {
            assert_eq!(audio_for_event(&event), Some((AudioKind::Effect, path)));
        }
        for (variant, path) in [
            (0, "sounds/yuck.ogg"),
            (1, "sounds/yuck.ogg"),
            (2, "sounds/yuck2.ogg"),
        ] {
            assert_eq!(
                audio_for_event(&GameEvent::ZombieYuckSound { entity: 1, variant }),
                Some((AudioKind::Effect, path))
            );
        }
        assert_eq!(
            audio_for_event(&GameEvent::GameWon),
            Some((AudioKind::Music, "sounds/winmusic.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::GameLost { zombie: 1 }),
            Some((AudioKind::Music, "sounds/losemusic.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::GameLostChomp { variant: 0 }),
            Some((AudioKind::Effect, "sounds/chomp.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::GameLostChomp { variant: 1 }),
            Some((AudioKind::Effect, "sounds/chomp2.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::GameLostScream),
            Some((AudioKind::Effect, "sounds/scream.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::Paused),
            Some((AudioKind::Effect, "sounds/pause.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::MowerTriggered {
                row: 2,
                pool: false,
            }),
            Some((AudioKind::Effect, "sounds/lawnmower.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::MowerTriggered { row: 2, pool: true }),
            Some((AudioKind::Effect, "sounds/pool_cleaner.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::MowerZombieHit {
                entity: 1,
                pool: true,
                variant: 0,
            }),
            Some((AudioKind::Effect, "sounds/shoop.ogg"))
        );
        for (variant, path) in [
            (0, "sounds/splat.ogg"),
            (1, "sounds/splat2.ogg"),
            (2, "sounds/splat3.ogg"),
        ] {
            assert_eq!(
                audio_for_event(&GameEvent::MowerZombieHit {
                    entity: 1,
                    pool: false,
                    variant,
                }),
                Some((AudioKind::Effect, path))
            );
        }
        for (variant, path) in [(0, "sounds/chomp.ogg"), (1, "sounds/chomp2.ogg")] {
            assert_eq!(
                audio_for_event(&GameEvent::MowerSquished { row: 2, variant }),
                Some((AudioKind::Effect, path))
            );
        }
        assert_eq!(
            audio_for_event(&GameEvent::MowerEnteredPool { row: 2, variant: 0 }),
            Some((AudioKind::Effect, "sounds/plant_water.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::MowerEnteredPool { row: 2, variant: 1 }),
            Some((AudioKind::Effect, "sounds/zombie_entering_water.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::MowerExitedPool { row: 2 }),
            Some((AudioKind::Effect, "sounds/plant_water.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::SunCollected {
                entity: 1,
                value: 25,
                sun_total: 25,
            }),
            Some((AudioKind::Effect, "sounds/points.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::SunProduced {
                entity: 1,
                source: SunSource::Plant(2),
                value: 25,
            }),
            Some((AudioKind::Effect, "sounds/throw.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::SunProduced {
                entity: 1,
                source: SunSource::Sky,
                value: 25,
            }),
            None
        );
        for coin_type in [
            CoinType::Diamond,
            CoinType::Chocolate,
            CoinType::AwardChocolate,
            CoinType::PresentPlant,
            CoinType::AwardPresent,
            CoinType::PresentMinigames,
            CoinType::PresentPuzzleMode,
            CoinType::PresentSurvivalMode,
        ] {
            assert_eq!(
                audio_for_event(&GameEvent::CoinProduced {
                    entity: 1,
                    coin_type,
                    value: 0,
                }),
                Some((AudioKind::Effect, "sounds/chime.ogg"))
            );
        }
        for coin_type in [
            CoinType::Silver,
            CoinType::Gold,
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
            CoinType::AwardMoneyBag,
            CoinType::AwardBagDiamond,
            CoinType::AwardSilverSunflower,
            CoinType::AwardGoldSunflower,
        ] {
            assert_eq!(
                audio_for_event(&GameEvent::CoinProduced {
                    entity: 1,
                    coin_type,
                    value: 1,
                }),
                None
            );
        }
        assert_eq!(
            audio_for_event(&GameEvent::CoinCollected {
                entity: 2,
                coin_type: neopvz_core::CoinType::Silver,
                value: 1,
                coin_total: 1,
            }),
            Some((AudioKind::Effect, "sounds/coin.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::CoinCollected {
                entity: 3,
                coin_type: neopvz_core::CoinType::Diamond,
                value: 100,
                coin_total: 101,
            }),
            Some((AudioKind::Effect, "sounds/diamond.au"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::PickupCollected {
                entity: 4,
                coin_type: neopvz_core::CoinType::UsableSeedPacket,
                value: 1,
                coins_total: 0,
                sun_total: 0,
            }),
            Some((AudioKind::Effect, "sounds/seedlift.ogg"))
        );
        for coin_type in [
            neopvz_core::CoinType::Chocolate,
            neopvz_core::CoinType::AwardChocolate,
            neopvz_core::CoinType::PresentPlant,
            neopvz_core::CoinType::AwardPresent,
            neopvz_core::CoinType::PresentMinigames,
            neopvz_core::CoinType::PresentPuzzleMode,
            neopvz_core::CoinType::PresentSurvivalMode,
        ] {
            assert_eq!(
                audio_for_event(&GameEvent::PickupCollected {
                    entity: 5,
                    coin_type,
                    value: 1,
                    coins_total: 0,
                    sun_total: 0,
                }),
                Some((AudioKind::Effect, "sounds/prize.ogg"))
            );
        }
        for coin_type in [
            neopvz_core::CoinType::Sun,
            neopvz_core::CoinType::SmallSun,
            neopvz_core::CoinType::LargeSun,
        ] {
            assert_eq!(
                audio_for_event(&GameEvent::PickupCollected {
                    entity: 6,
                    coin_type,
                    value: 25,
                    coins_total: 0,
                    sun_total: 25,
                }),
                Some((AudioKind::Effect, "sounds/points.ogg"))
            );
        }
        assert_eq!(
            audio_for_event(&GameEvent::GardenWatered {
                plant: 0,
                age_ticks: 1,
            }),
            Some((AudioKind::Effect, "sounds/watering.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::GardenFertilized {
                plant: 0,
                age_ticks: 100,
            }),
            Some((AudioKind::Effect, "sounds/fertilizer.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::GardenToolUsed {
                plant: 0,
                tool: GardenTool::BugSpray,
            }),
            Some((AudioKind::Effect, "sounds/bugspray.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::GardenToolUsed {
                plant: 0,
                tool: GardenTool::Phonograph,
            }),
            Some((AudioKind::Effect, "sounds/phonograph.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::GardenBecameHappy {
                plant: 0,
                aquatic: false,
            }),
            Some((AudioKind::Effect, "sounds/prize.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::GardenTreeGrew { height: 2 }),
            Some((AudioKind::Effect, "sounds/plantgrow.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::GardenLeft),
            Some((AudioKind::Effect, "sounds/gravebutton.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::GardenTapGlass),
            Some((AudioKind::Effect, "sounds/tapglass.au"))
        );
        for event in [
            GameEvent::GardenWatered {
                plant: 0,
                age_ticks: 1,
            },
            GameEvent::GardenFertilized {
                plant: 0,
                age_ticks: 100,
            },
            GameEvent::GardenBecameHappy {
                plant: 0,
                aquatic: false,
            },
        ] {
            assert_eq!(
                audio_companion_for_event(&event),
                Some((AudioKind::Effect, "sounds/throw.ogg"))
            );
        }
        assert_eq!(
            audio_for_event(&GameEvent::WaveStarted { wave: 0 }),
            Some((AudioKind::Effect, "sounds/awooga.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::FlagWaveSound { wave: 9 }),
            Some((AudioKind::Effect, "sounds/siren.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::HugeWaveSound { wave: 9 }),
            Some((AudioKind::Effect, "sounds/hugewave.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::FinalWaveSound { wave: 9 }),
            Some((AudioKind::Effect, "sounds/finalwave.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::PlantSpecialTriggered {
                entity: 1,
                plant_type: neopvz_core::PlantType::Other(14),
            }),
            Some((AudioKind::Effect, "sounds/frozen.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::PlantSpecialTriggered {
                entity: 1,
                plant_type: neopvz_core::PlantType::Other(4),
            }),
            Some((AudioKind::Effect, "sounds/potato_mine.ogg"))
        );
        for (plant_type, path) in [
            (neopvz_core::PlantType::Other(2), "sounds/cherrybomb.ogg"),
            (neopvz_core::PlantType::Other(49), "sounds/cherrybomb.ogg"),
            (neopvz_core::PlantType::Other(20), "sounds/jalapeno.ogg"),
            (neopvz_core::PlantType::Other(15), "sounds/doomshroom.ogg"),
            (neopvz_core::PlantType::Other(6), "sounds/bigchomp.ogg"),
            (
                neopvz_core::PlantType::Other(11),
                "sounds/gravebusterchomp.ogg",
            ),
            (neopvz_core::PlantType::Other(35), "sounds/coffee.ogg"),
            (neopvz_core::PlantType::Other(21), "sounds/throw.ogg"),
        ] {
            assert_eq!(
                audio_for_event(&GameEvent::PlantSpecialTriggered {
                    entity: 1,
                    plant_type,
                }),
                Some((AudioKind::Effect, path))
            );
        }
        for (variant, path) in [
            (0, "sounds/throw.ogg"),
            (2, "sounds/throw.ogg"),
            (3, "sounds/throw2.ogg"),
        ] {
            assert_eq!(
                audio_for_event(&GameEvent::PlantFired {
                    entity: 1,
                    plant_type: neopvz_core::PlantType::Peashooter,
                    variant,
                }),
                Some((AudioKind::Effect, path))
            );
        }
        for plant_type in [
            neopvz_core::PlantType::Other(5),
            neopvz_core::PlantType::Other(44),
        ] {
            let event = GameEvent::PlantFired {
                entity: 1,
                plant_type,
                variant: 0,
            };
            assert_eq!(
                audio_for_event(&event),
                Some((AudioKind::Effect, "sounds/throw.ogg"))
            );
            assert_eq!(
                audio_companion_for_event(&event),
                Some((AudioKind::Effect, "sounds/snow_pea_sparkles.ogg"))
            );
        }
        for plant_type in [
            neopvz_core::PlantType::Other(8),
            neopvz_core::PlantType::Other(13),
            neopvz_core::PlantType::Other(24),
        ] {
            let event = GameEvent::PlantFired {
                entity: 1,
                plant_type,
                variant: 0,
            };
            assert_eq!(
                audio_for_event(&event),
                Some((AudioKind::Effect, "sounds/throw.ogg"))
            );
            assert_eq!(
                audio_companion_for_event(&event),
                Some((AudioKind::Effect, "sounds/puff.ogg"))
            );
        }
        let fume = GameEvent::PlantFired {
            entity: 1,
            plant_type: neopvz_core::PlantType::Other(10),
            variant: 0,
        };
        assert_eq!(
            audio_for_event(&fume),
            Some((AudioKind::Effect, "sounds/fume.ogg"))
        );
        assert_eq!(audio_companion_for_event(&fume), None);
        let gloom = GameEvent::PlantFired {
            entity: 1,
            plant_type: neopvz_core::PlantType::Other(42),
            variant: 0,
        };
        assert_eq!(audio_for_event(&gloom), None);
        assert_eq!(audio_companion_for_event(&gloom), None);
        assert_eq!(
            audio_for_event(&GameEvent::TangleKelpGrabStarted { entity: 1 }),
            Some((AudioKind::Effect, "sounds/floop.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::TangleKelpWaterEntry { entity: 1 }),
            Some((AudioKind::Effect, "sounds/zombiesplash.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::PotatoMineArmed { entity: 1 }),
            Some((AudioKind::Effect, "sounds/dirt_rise.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::DiggerSurfaced { entity: 1 }),
            Some((AudioKind::Effect, "sounds/dirt_rise.ogg"))
        );
        assert_eq!(
            audio_companion_for_event(&GameEvent::DiggerSurfaced { entity: 1 }),
            Some((AudioKind::Effect, "sounds/wakeup.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::MetalStolen {
                plant: 1,
                zombie: Some(2),
            }),
            Some((AudioKind::Effect, "sounds/magnetshroom.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::VehicleDisabled { entity: 1 }),
            Some((AudioKind::Effect, "sounds/balloon_pop.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::BalloonPopped { entity: 1 }),
            Some((AudioKind::Effect, "sounds/balloon_pop.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::VehicleExploded { entity: 1 }),
            Some((AudioKind::Effect, "sounds/explosion.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::BloverTriggered { entity: 1, row: 2 }),
            Some((AudioKind::Effect, "sounds/blover.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieHypnotized { entity: 1 }),
            Some((AudioKind::Effect, "sounds/mindcontrolled.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::JackboxExploded {
                entity: 1,
                row: 2,
                column: 5,
            }),
            Some((AudioKind::Effect, "sounds/explosion.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::JackboxBoing { entity: 1 }),
            Some((AudioKind::Effect, "sounds/boing.ogg"))
        );
        for (variant, path) in [
            (0, "sounds/jack_surprise.ogg"),
            (1, "sounds/jack_surprise.ogg"),
            (2, "sounds/jack_surprise2.ogg"),
        ] {
            assert_eq!(
                audio_for_event(&GameEvent::JackboxSurprise { entity: 1, variant }),
                Some((AudioKind::Effect, path))
            );
        }
        assert_eq!(
            audio_for_event(&GameEvent::ZombieChilled {
                entity: 1,
                duration: 1_000,
            }),
            Some((AudioKind::Effect, "sounds/frozen.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::BrainFinished {
                zombie: 1,
                row: 0,
                brains_remaining: 4,
            }),
            Some((AudioKind::Effect, "sounds/gulp.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ImpThrown {
                gargantuar: 1,
                imp: 2,
                imp_variant: 0,
            }),
            Some((AudioKind::Effect, "sounds/swing.ogg"))
        );
        assert_eq!(
            audio_companion_for_event(&GameEvent::ImpThrown {
                gargantuar: 1,
                imp: 2,
                imp_variant: 0,
            }),
            Some((AudioKind::Effect, "sounds/imp.ogg"))
        );
        assert_eq!(
            audio_companion_for_event(&GameEvent::ImpThrown {
                gargantuar: 1,
                imp: 2,
                imp_variant: 1,
            }),
            Some((AudioKind::Effect, "sounds/imp2.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieNewspaperRipped { entity: 1 }),
            Some((AudioKind::Effect, "sounds/newspaper_rip.ogg"))
        );
        for (variant, path) in [
            (0, "sounds/newspaper_rarrgh.ogg"),
            (1, "sounds/newspaper_rarrgh.ogg"),
            (2, "sounds/newspaper_rarrgh2.ogg"),
        ] {
            assert_eq!(
                audio_for_event(&GameEvent::ZombieNewspaperRarrgh { entity: 1, variant }),
                Some((AudioKind::Effect, path))
            );
        }
        assert_eq!(
            audio_for_event(&GameEvent::PoleVaultGrassStep { entity: 1 }),
            Some((AudioKind::Effect, "sounds/grassstep.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::BungeeGrassStep { entity: 1 }),
            Some((AudioKind::Effect, "sounds/grassstep.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::PoleVaultSound { entity: 1 }),
            Some((AudioKind::Effect, "sounds/polevault.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::PogoBounceSound { entity: 1 }),
            Some((AudioKind::Effect, "sounds/pogo_zombie.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::DolphinJumpStarted { entity: 1 }),
            Some((AudioKind::Effect, "sounds/dolphin_before_jumping.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieEnteredPool {
                entity: 1,
                variant: 0,
            }),
            Some((AudioKind::Effect, "sounds/plant_water.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieEnteredPool {
                entity: 1,
                variant: 1,
            }),
            Some((AudioKind::Effect, "sounds/zombie_entering_water.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieSpawned {
                entity: 1,
                zombie_type: neopvz_core::ZombieType::DolphinRider,
                row: 2,
                wave: 0,
            }),
            Some((AudioKind::Effect, "sounds/dolphin_appears.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieSpawned {
                entity: 1,
                zombie_type: neopvz_core::ZombieType::Zamboni,
                row: 2,
                wave: 0,
            }),
            Some((AudioKind::Effect, "sounds/zamboni.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieSpawned {
                entity: 1,
                zombie_type: neopvz_core::ZombieType::Balloon,
                row: 2,
                wave: 0,
            }),
            Some((AudioKind::Effect, "sounds/ballooninflate.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieSpawned {
                entity: 1,
                zombie_type: neopvz_core::ZombieType::BackupDancer,
                row: 2,
                wave: 0,
            }),
            Some((AudioKind::Effect, "sounds/gravestone_rumble.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieGraveRumble { entity: 1 }),
            Some((AudioKind::Effect, "sounds/gravestone_rumble.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieWhackRise { entity: 1 }),
            Some((AudioKind::Effect, "sounds/dirt_rise.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::LadderPlaced {
                zombie: 1,
                row: 2,
                column: 0,
            }),
            Some((AudioKind::Effect, "sounds/ladder_zombie.ogg"))
        );
        for (variant, path) in [
            (0, "sounds/bungee_scream.ogg"),
            (1, "sounds/bungee_scream2.ogg"),
            (2, "sounds/bungee_scream3.ogg"),
        ] {
            assert_eq!(
                audio_for_event(&GameEvent::BungeeScream { entity: 1, variant }),
                Some((AudioKind::Effect, path))
            );
        }
        assert_eq!(
            audio_for_event(&GameEvent::BungeePlantLifted {
                zombie: 1,
                plant: 2,
            }),
            Some((AudioKind::Effect, "sounds/floop.ogg"))
        );
        for (variant, path) in [
            (0, "sounds/zombie_falling_1.ogg"),
            (1, "sounds/zombie_falling_2.ogg"),
        ] {
            assert_eq!(
                audio_for_event(&GameEvent::ZombieFallingSound {
                    entity: 1,
                    zombie_type: neopvz_core::ZombieType::Normal,
                    variant,
                }),
                Some((AudioKind::Effect, path))
            );
        }
        let gargantuar_fall = GameEvent::ZombieFallingSound {
            entity: 1,
            zombie_type: neopvz_core::ZombieType::Gargantuar,
            variant: 0,
        };
        assert_eq!(
            audio_sequence_for_event(&gargantuar_fall, Game::new(0, SceneKind::Day).state())
                .into_iter()
                .flatten()
                .map(|(_, path)| path)
                .collect::<Vec<_>>(),
            vec!["sounds/zombie_falling_1.ogg", "sounds/gargantuar_thump.ogg"]
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieShieldHit {
                entity: 1,
                variant: 0,
            }),
            Some((AudioKind::Effect, "sounds/shieldhit.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieShieldHit {
                entity: 1,
                variant: 1,
            }),
            Some((AudioKind::Effect, "sounds/shieldhit2.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieBodyPartLost {
                entity: 1,
                head: false,
            }),
            Some((AudioKind::Effect, "sounds/limbs_pop.ogg"))
        );
        assert_eq!(
            audio_companion_for_event(&GameEvent::DolphinJumpStarted { entity: 1 }),
            Some((AudioKind::Effect, "sounds/plant_water.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieButtered { entity: 1 }),
            None
        );
        assert_eq!(
            audio_for_event(&GameEvent::PlantSpecialTriggered {
                entity: 1,
                plant_type: neopvz_core::PlantType::Other(17),
            }),
            Some((AudioKind::Effect, "sounds/gargantuar_thump.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::SquashHumStarted {
                entity: 1,
                variant: 0,
            }),
            Some((AudioKind::Effect, "sounds/squash_hmm.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::SquashHumStarted {
                entity: 1,
                variant: 2,
            }),
            Some((AudioKind::Effect, "sounds/squash_hmm2.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::VaseBroken {
                entity: 1,
                row: 2,
                column: 2,
            }),
            Some((AudioKind::Effect, "sounds/vase_breaking.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::RakeTriggered { zombie: 1 }),
            Some((AudioKind::Effect, "sounds/swing.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::JumpBlocked {
                zombie: 1,
                plant: 2,
            }),
            Some((AudioKind::Effect, "sounds/bonk.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::UmbrellaDeflected {
                plant: 1,
                zombie: 2,
            }),
            Some((AudioKind::Effect, "sounds/boing.ogg"))
        );
        assert_eq!(
            audio_companion_for_event(&GameEvent::UmbrellaDeflected {
                plant: 1,
                zombie: 2,
            }),
            Some((AudioKind::Effect, "sounds/throw2.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::UmbrellaTriggered {
                plant: 1,
                projectile: 2,
            }),
            Some((AudioKind::Effect, "sounds/throw2.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::CobCannonFired {
                entity: 1,
                target_row: 2,
                target_column: 4,
            }),
            Some((AudioKind::Effect, "sounds/coblaunch.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ProjectileFired {
                entity: 1,
                source: 2,
                projectile_type: neopvz_core::ProjectileType::Other(1),
                row: 2,
            }),
            Some((AudioKind::Effect, "sounds/basketball.ogg"))
        );
        for (kind, path) in [
            (ProjectileImpactSound::Splat, "sounds/splat.ogg"),
            (ProjectileImpactSound::Kernel, "sounds/kernelpult.ogg"),
            (ProjectileImpactSound::Butter, "sounds/butter.ogg"),
            (ProjectileImpactSound::Ignite, "sounds/ignite.ogg"),
            (ProjectileImpactSound::Melon, "sounds/melonimpact.ogg"),
            (ProjectileImpactSound::Shield, "sounds/shieldhit.ogg"),
            (ProjectileImpactSound::Plastic, "sounds/plastichit.ogg"),
        ] {
            assert_eq!(
                audio_for_event(&GameEvent::ProjectileImpact {
                    projectile: 1,
                    projectile_type: ProjectileType::Pea,
                    zombie: Some(2),
                    kind,
                    variant: match kind {
                        ProjectileImpactSound::Splat => 2,
                        ProjectileImpactSound::Ignite => 3,
                        _ => 1,
                    },
                }),
                Some((
                    AudioKind::Effect,
                    match kind {
                        ProjectileImpactSound::Splat => "sounds/splat3.ogg",
                        ProjectileImpactSound::Kernel => "sounds/kernelpult2.ogg",
                        ProjectileImpactSound::Butter => path,
                        ProjectileImpactSound::Ignite => "sounds/ignite2.ogg",
                        ProjectileImpactSound::Melon => "sounds/melonimpact2.ogg",
                        ProjectileImpactSound::Shield => "sounds/shieldhit2.ogg",
                        ProjectileImpactSound::Plastic => "sounds/plastichit2.ogg",
                    },
                ))
            );
        }
        assert_eq!(
            audio_for_event(&GameEvent::ProjectileHit {
                projectile: 1,
                zombie: 2,
                damage: 20,
                health_remaining: 250,
            }),
            None
        );
        assert_eq!(
            audio_for_event(&GameEvent::ProjectileSplashHit {
                projectile: 1,
                zombie: 2,
                damage: 13,
                health_remaining: 250,
            }),
            None
        );
        assert_eq!(
            audio_for_event(&GameEvent::BossAttackWindup {
                entity: 1,
                row: 2,
                fire: true,
            }),
            Some((AudioKind::Effect, "sounds/bossboulderattack.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::BossHeadHydraulic { entity: 1 }),
            Some((AudioKind::Effect, "sounds/hydraulic.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::BossStomp { entity: 1, row: 2 }),
            Some((AudioKind::Effect, "sounds/gargantuar_thump.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::BossRVStarted {
                entity: 1,
                row: 2,
                column: 1,
            }),
            Some((AudioKind::Effect, "sounds/hydraulic_short.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::BossRVLanded {
                entity: 1,
                row: 2,
                column: 1,
            }),
            Some((AudioKind::Effect, "sounds/RVthrow.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::BossProjectileStarted {
                entity: 1,
                row: 2,
                fire: true,
            }),
            Some((AudioKind::Effect, "sounds/hydraulic_short.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::BossDamageExplosion { entity: 1 }),
            Some((AudioKind::Effect, "sounds/explosion.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::PortalOpened {
                row: 2,
                column: 5,
                square: true,
            }),
            Some((AudioKind::Effect, "sounds/portal.ogg"))
        );
        assert_eq!(audio_for_event(&GameEvent::Resumed), None);
        assert_eq!(audio_for_event(&GameEvent::StateChanged), None);
    }

    #[test]
    fn final_adventure_win_uses_the_source_fanfare_in_context() {
        let event = GameEvent::GameWon;
        let final_game = Game::new_adventure(7, 50, false, 0, false);
        assert_eq!(
            audio_sequence_for_event(&event, final_game.state())
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
            vec![
                (AudioKind::Effect, "sounds/finalfanfare.ogg"),
                (AudioKind::Effect, "sounds/paper.ogg"),
            ]
        );

        let paper_game = Game::new_adventure(7, 10, false, 0, false);
        assert_eq!(
            audio_sequence_for_event(&event, paper_game.state())
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
            vec![
                (AudioKind::Music, "sounds/winmusic.ogg"),
                (AudioKind::Effect, "sounds/paper.ogg"),
            ]
        );

        let ordinary_game = Game::new(7, SceneKind::Day);
        assert_eq!(
            audio_sequence_for_event(&event, ordinary_game.state())
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
            vec![(AudioKind::Music, "sounds/winmusic.ogg")]
        );
    }

    #[test]
    fn maps_zombie_song_groan_chew_and_death_resources() {
        assert_eq!(
            audio_for_event(&GameEvent::ZombieSongStarted {
                entity: 1,
                zombie_type: neopvz_core::ZombieType::Jackbox,
            }),
            Some((AudioKind::Effect, "sounds/jackinthebox.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieSongStarted {
                entity: 2,
                zombie_type: neopvz_core::ZombieType::Digger,
            }),
            Some((AudioKind::Effect, "sounds/digger_zombie.ogg"))
        );
        for (family, paths) in [
            (
                ZombieGroanFamily::Low,
                &["sounds/lowgroan.ogg", "sounds/lowgroan2.ogg"][..],
            ),
            (
                ZombieGroanFamily::Normal,
                &[
                    "sounds/groan.ogg",
                    "sounds/groan2.ogg",
                    "sounds/groan3.ogg",
                    "sounds/groan4.ogg",
                    "sounds/groan5.ogg",
                    "sounds/groan6.ogg",
                ][..],
            ),
            (
                ZombieGroanFamily::Brains,
                &[
                    "sounds/groan.ogg",
                    "sounds/groan2.ogg",
                    "sounds/groan3.ogg",
                    "sounds/groan4.ogg",
                    "sounds/groan5.ogg",
                    "sounds/groan6.ogg",
                    "sounds/sukhbir4.ogg",
                    "sounds/sukhbir5.ogg",
                    "sounds/sukhbir6.ogg",
                ][..],
            ),
            (
                ZombieGroanFamily::Sukhbir,
                &[
                    "sounds/groan.ogg",
                    "sounds/groan2.ogg",
                    "sounds/groan3.ogg",
                    "sounds/groan4.ogg",
                    "sounds/groan5.ogg",
                    "sounds/groan6.ogg",
                    "sounds/sukhbir.ogg",
                    "sounds/sukhbir2.ogg",
                    "sounds/sukhbir3.ogg",
                ][..],
            ),
        ] {
            for (variant, path) in paths.iter().enumerate() {
                assert_eq!(
                    audio_for_event(&GameEvent::ZombieGroaned {
                        entity: 3,
                        zombie_type: neopvz_core::ZombieType::Normal,
                        family,
                        variant: variant as u8,
                    }),
                    Some((AudioKind::Effect, *path))
                );
            }
        }
        assert_eq!(
            audio_for_event(&GameEvent::ZombieChew {
                entity: 5,
                target: None,
                soft: true,
                variant: 0,
            }),
            Some((AudioKind::Effect, "sounds/chompsoft.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieChew {
                entity: 6,
                target: Some(7),
                soft: false,
                variant: 1,
            }),
            Some((AudioKind::Effect, "sounds/chomp2.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieDeathSound {
                entity: 8,
                zombie_type: neopvz_core::ZombieType::Gargantuar,
            }),
            Some((AudioKind::Effect, "sounds/gargantudeath.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::ZombieDeathSound {
                entity: 9,
                zombie_type: neopvz_core::ZombieType::Boss,
            }),
            Some((AudioKind::Effect, "sounds/bossexplosion.ogg"))
        );
        assert_eq!(
            audio_companion_for_event(&GameEvent::ZombieDeathSound {
                entity: 9,
                zombie_type: neopvz_core::ZombieType::Boss,
            }),
            Some((AudioKind::Effect, "sounds/gargantudeath.ogg"))
        );
    }

    #[test]
    fn zombie_died_has_no_direct_audio_mapping() {
        assert_eq!(audio_for_event(&GameEvent::ZombieDied { entity: 2 }), None);
    }

    #[test]
    fn maps_explosive_plant_companions() {
        for plant_type in [
            neopvz_core::PlantType::Other(2),
            neopvz_core::PlantType::Other(20),
        ] {
            assert_eq!(
                audio_companion_for_event(&GameEvent::PlantPlaced {
                    entity: 1,
                    plant_type,
                    row: 2,
                    column: 3,
                    sun_remaining: 50,
                    variant: 0,
                }),
                Some((AudioKind::Effect, "sounds/reverse_explosion.ogg"))
            );
        }
        assert_eq!(
            audio_companion_for_event(&GameEvent::PlantPlaced {
                entity: 1,
                plant_type: neopvz_core::PlantType::Other(15),
                row: 2,
                column: 3,
                sun_remaining: 50,
                variant: 0,
            }),
            None
        );
        assert_eq!(
            audio_companion_for_event(&GameEvent::PlantPlaced {
                entity: 1,
                plant_type: neopvz_core::PlantType::Other(25),
                row: 2,
                column: 3,
                sun_remaining: 50,
                variant: 0,
            }),
            Some((AudioKind::Effect, "sounds/plantern.ogg"))
        );
        for plant_type in [
            neopvz_core::PlantType::Other(2),
            neopvz_core::PlantType::Other(20),
        ] {
            assert_eq!(
                audio_companion_for_event(&GameEvent::PlantSpecialTriggered {
                    entity: 1,
                    plant_type,
                }),
                Some((AudioKind::Effect, "sounds/juicy.ogg"))
            );
        }
        assert_eq!(
            audio_companion_for_event(&GameEvent::PlantSpecialTriggered {
                entity: 1,
                plant_type: neopvz_core::PlantType::Other(15),
            }),
            None
        );
        assert_eq!(
            audio_companion_for_event(&GameEvent::PlantSpecialTriggered {
                entity: 1,
                plant_type: neopvz_core::PlantType::Other(49),
            }),
            Some((AudioKind::Effect, "sounds/bowlingimpact2.ogg"))
        );
    }

    #[test]
    fn maps_wallnut_bowling_impact_to_source_sample() {
        assert_eq!(
            audio_for_event(&GameEvent::BowlingImpact {
                plant: 1,
                zombie: 2,
            }),
            Some((AudioKind::Effect, "sounds/bowlingimpact.ogg"))
        );
    }

    #[cfg(windows)]
    #[test]
    fn windows_text_images_are_complete_across_repeated_draws() {
        let text = "\u{70b9}\u{51fb}\u{5f00}\u{59cb}";
        let first = render_text_image(1, text, 120, 24, 19).unwrap();
        assert!(
            first
                .rgba8
                .as_chunks::<4>()
                .0
                .iter()
                .any(|pixel| pixel[3] != 0)
        );
        for _ in 0..64 {
            let next = render_text_image(1, text, 120, 24, 19).unwrap();
            assert!(next.rgba8 == first.rgba8, "GDI returned incomplete text");
        }
    }

    #[test]
    fn title_start_hitbox_matches_the_source_load_bar() {
        // The start button rests at (240, 534) with size 314x50.
        assert!(title_start_contains(400.0, 550.0, 534.0));
        assert!(!title_start_contains(239.0, 550.0, 534.0));
        assert!(!title_start_contains(400.0, 533.0, 534.0));
        assert!(!title_start_contains(400.0, 584.0, 534.0));
    }

    #[test]
    fn title_start_input_uses_source_readiness_and_button_release() {
        assert_eq!(TITLE_START_SOUND_PATH, "sounds/buttonclick.ogg");
        let mut state = TitleLoadState::new();
        assert!(!state.mouse_press(MouseButton::Left, true));
        assert!(!state.mouse_release(MouseButton::Left, true));
        for _ in 0..300 {
            state.advance();
        }
        assert!(state.ready());
        assert!(!state.label_complete);
        for button in [MouseButton::Left, MouseButton::Right] {
            // A background press starts immediately, without a captured button.
            assert!(state.mouse_press(button, false));
            assert_eq!(state.pressed_button, None);
            assert!(!state.mouse_release(button, true));
            assert!(state.mouse_press(button, true));
            assert_eq!(state.pressed_button, Some(button));
            assert!(state.mouse_release(button, true));
            assert!(!state.mouse_release(button, true));
            // Dragging off the button before release cancels the press.
            assert!(state.mouse_press(button, true));
            assert!(!state.mouse_release(button, false));
            assert_eq!(state.pressed_button, None);
        }
    }

    #[test]
    fn title_start_guard_and_opening_logo_follow_the_live_app_route() {
        let mut app = App::new(
            Default::default(),
            ResourceProvider::Directory(PathBuf::from("neopvz-test-no-resources")),
            None,
            SceneKind::Title,
            false,
            None,
            None,
        );
        let frame = app.render_frame();
        assert_eq!(frame.sprites.len(), 1);
        assert_eq!(frame.sprites[0].resource_id, TITLE_POPCAP_LOGO_IMAGE_ID);
        assert_eq!((frame.sprites[0].x, frame.sprites[0].y), (250.0, 150.0));
        assert_eq!(frame.sprites[0].alpha, 0.0);
        for _ in 0..299 {
            app.start_from_title();
            assert_eq!(app.game.state().scene, SceneKind::Title);
            app.title_load_state.advance();
        }
        app.start_from_title();
        assert_eq!(app.game.state().scene, SceneKind::Title);
        app.title_load_state.advance();
        assert!(!app.title_load_state.label_complete);
        app.start_from_title();
        assert_eq!(app.game.state().scene, SceneKind::AdventureSelect);
    }

    #[test]
    fn title_curves_match_the_source_tod_animate_curves() {
        let mut state = TitleLoadState::new();
        for (counter, alpha) in [(200, 0.0), (175, 0.5), (150, 1.0), (50, 1.0), (25, 0.5)] {
            state.logo_counter = counter;
            assert_eq!(state.logo_alpha(), alpha);
        }
        // Logo off-screen at y=-150 while the counter is above 60.
        assert_eq!(state.logo_y(), -150.0);
        // BOUNCE peak 15 at counter 55 (curve 60->50, midpoint).
        state.counter = 55;
        assert_eq!(state.logo_y(), 15.0);
        state.counter = 50;
        assert_eq!(state.logo_y(), 10.0);
        // Logo rests at y=10 after the BOUNCE settles (t>=1 clamps to start).
        state.counter = 0;
        assert_eq!(state.logo_y(), 10.0);
        // Button rests at y=534 (BOUNCE clamps to start at t>=1).
        assert_eq!(state.button_y(), 534.0);
        // BOUNCE peak 529 at counter 5 (curve 10->0, midpoint).
        state.counter = 5;
        assert_eq!(state.button_y(), 529.0);
        // Button off-screen at y=650 while counter is above 60.
        state.counter = 61;
        assert_eq!(state.button_y(), 650.0);
        // EASE_IN midpoint at counter 35: t=0.5, 650-116*0.25.
        state.counter = 35;
        assert!((state.button_y() - 621.0).abs() < 0.001);
    }

    #[test]
    fn title_fill_follows_the_source_velocity_model() {
        let mut state = TitleLoadState::new();
        // The logo lasts 200 ticks. The slide-in decrements before its gate,
        // so its 100th update is also the first fill update.
        for _ in 0..299 {
            assert!(state.advance().is_empty());
            assert!(!state.ready());
        }
        assert_eq!(state.counter, 1);
        // First fill tick: width = 314/100, velocity clamps to 2.
        assert!(state.advance().is_empty());
        assert_eq!(state.tick, 1);
        assert_eq!(state.counter, 0);
        assert!(state.ready());
        assert!(
            (state.bar_width * 100.0 - 314.0).abs() < 0.01,
            "{}",
            state.bar_width
        );
        assert!((state.bar_vel - 2.0).abs() < 0.001, "{}", state.bar_vel);
        state.advance();
        assert!(
            (state.bar_width - 5.14).abs() < 0.001,
            "{}",
            state.bar_width
        );
        // width(t) = 2t + 1.14.
        state.advance();
        assert!(
            (state.bar_width - 7.14).abs() < 0.001,
            "{}",
            state.bar_width
        );
    }

    #[test]
    fn title_fill_fires_triggers_and_completes_at_source_ticks() {
        let mut state = TitleLoadState::new();
        let mut fired = Vec::new();
        while !state.label_complete {
            fired.extend(state.advance());
        }
        assert_eq!(fired, vec![0, 1, 2, 3, 4]);
        assert_eq!(state.tick, 157);
        assert_eq!(state.bar_width, TITLE_START_BUTTON_WIDTH);
        assert_eq!(
            state.trigger_ticks,
            [Some(17), Some(50), Some(85), Some(113), Some(142)]
        );
        // Reanimations keep animating after the bar completes: the last
        // trigger (tick 142) reaches its hold frame at tick 242.
        while state.tick < 242 {
            assert!(state.advance().is_empty());
        }
        for &fire_tick in state.trigger_ticks.iter().flatten() {
            assert!(state.tick - fire_tick >= 100);
        }
        // Advancing past completion is idempotent.
        let width = state.bar_width;
        assert!(state.advance().is_empty());
        assert_eq!(state.bar_width, width);
        assert_eq!(
            state.trigger_ticks,
            [Some(17), Some(50), Some(85), Some(113), Some(142)]
        );
    }

    #[test]
    fn title_completed_state_matches_the_accepted_capture() {
        let state = TitleLoadState::completed();
        assert!(state.label_complete);
        assert_eq!(state.button_y(), 534.0);
        assert_eq!(state.logo_y(), 10.0);
        assert_eq!(state.bar_width, TITLE_START_BUTTON_WIDTH);
        // Every trigger is at the hold frame.
        for &fire_tick in state.trigger_ticks.iter().flatten() {
            assert!(state.tick - fire_tick >= 100);
        }
    }

    #[test]
    fn title_reanimation_holds_its_final_frame() {
        let transform = ReanimatorTransform {
            x: 6.0,
            y: 8.0,
            skew_x: 0.0,
            skew_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            frame: 0.0,
            alpha: 1.0,
            image: Some("SYNTHETIC_IMAGE".to_owned()),
        };
        let definition = ReanimatorDefinition {
            fps: 12.0,
            tracks: vec![ReanimatorTrack {
                name: "synthetic".to_owned(),
                transforms: vec![transform; 18],
            }],
        };
        let image_ids = HashMap::from([("SYNTHETIC_IMAGE".to_owned(), 1)]);
        let mut catalog = ReanimCatalog {
            loadbar_sprout: Some(definition.clone()),
            loadbar_zombiehead: Some(definition),
            image_ids,
            ..Default::default()
        };
        // Only the final frame is visible, so a held checkpoint must retain
        // it, while an animation at its trigger tick must draw nothing.
        for definition in [&mut catalog.loadbar_sprout, &mut catalog.loadbar_zombiehead] {
            for transform in &mut definition.as_mut().unwrap().tracks[0].transforms[..17] {
                transform.frame = -1.0;
            }
        }
        let mut state = TitleLoadState::completed();
        let mut held = RenderFrame::default();
        push_title_load_bar_reanimations(&mut held, &state, &catalog);
        assert_eq!(held.affine_sprites.len(), 5);
        state.trigger_ticks = [Some(state.tick); 5];
        let mut born = RenderFrame::default();
        push_title_load_bar_reanimations(&mut born, &state, &catalog);
        assert!(born.affine_sprites.is_empty());
    }

    #[test]
    fn mode_entries_follow_unlock_state() {
        assert!(!mode_is_unlocked(ModeKind::MiniGame, false, 1, 0));
        assert!(mode_is_unlocked(
            ModeKind::MiniGame,
            false,
            1,
            CoinType::PresentMinigames.unlock_mask()
        ));
        assert!(!mode_is_unlocked(ModeKind::Vasebreaker, false, 1, 0));
        assert!(!mode_is_unlocked(ModeKind::Survival, false, 1, 0));
        assert!(mode_is_unlocked(
            ModeKind::Survival,
            false,
            1,
            CoinType::PresentSurvivalMode.unlock_mask()
        ));
        assert!(!mode_is_unlocked(ModeKind::ZenGarden, false, 44, 0));
        assert!(mode_is_unlocked(ModeKind::ZenGarden, false, 45, 0));
        assert!(mode_is_unlocked(ModeKind::Vasebreaker, true, 1, 0));
    }

    #[test]
    fn selector_level_digits_follow_source_cels_and_offsets() {
        assert_eq!(
            selector_level_digits(1),
            vec![(1, 486.0, 126.0), (1, 504.0, 128.0)]
        );
        assert_eq!(
            selector_level_digits(3),
            vec![(1, 486.0, 126.0), (3, 503.0, 128.0)]
        );
        assert_eq!(
            selector_level_digits(40),
            vec![(4, 485.0, 125.0), (1, 504.0, 128.0), (0, 513.0, 129.0)]
        );
        assert_eq!(
            selector_level_digits(50),
            vec![(5, 486.0, 125.0), (1, 504.0, 128.0), (0, 513.0, 129.0)]
        );
    }

    #[test]
    fn day_scene_starts_the_first_adventure_level() {
        let game = new_scene_game(SceneKind::Day);
        assert_eq!(game.state().mode, ModeKind::Adventure);
        assert_eq!(game.state().level, 1);
        assert_eq!(game.state().scene, SceneKind::Day);
        assert_eq!(game.state().board.wave.total, 4);
    }

    #[test]
    fn seed_bank_packet_geometry_matches_source_layouts() {
        assert_eq!(seed_packet_position_x(0, 6, false, false), 85.0);
        assert_eq!(seed_packet_position_x(6, 7, false, false), 439.0);
        assert_eq!(seed_packet_position_x(7, 8, false, false), 459.0);
        assert_eq!(seed_packet_position_x(8, 9, false, false), 496.0);
        assert_eq!(seed_packet_position_x(9, 10, false, false), 538.0);
        assert_eq!(seed_packet_position_x(0, 10, true, false), 91.0);
        assert_eq!(seed_packet_position_x(0, 3, false, true), 247.0);
        assert_eq!(seed_bank_extra_width(6), 0);
        assert_eq!(seed_bank_extra_width(7), 60);
        assert_eq!(seed_bank_extra_width(10), 153);
    }

    #[test]
    fn board_seed_packet_hit_test_routes_to_core_slot() {
        let ordinary = Game::new(0, SceneKind::Day);
        assert_eq!(board_seed_packet_at(ordinary.state(), 80.0, 1.0), Some(0));
        assert_eq!(board_seed_packet_at(ordinary.state(), 128.0, 69.0), Some(0));
        assert_eq!(board_seed_packet_at(ordinary.state(), 130.0, 1.0), Some(1));
        assert_eq!(board_seed_packet_at(ordinary.state(), 128.0, 70.0), None);

        let conveyor = Game::new_mode(0, ModeKind::MiniGame, 1);
        assert_eq!(board_seed_packet_at(conveyor.state(), 92.0, 1.0), Some(0));

        let slot_machine = Game::new_mode(0, ModeKind::MiniGame, 2);
        assert_eq!(
            board_seed_packet_at(slot_machine.state(), 248.0, 1.0),
            Some(0)
        );
    }

    #[test]
    fn scene_transition_carries_profile_progress() {
        let mut profile = SaveProfile::new("transition");
        let mut current = Game::new_mode(0, ModeKind::Adventure, 2);
        current.apply_profile(&profile);
        current.debug_prepare_diamond_collection();
        current.debug_prepare_game_won();
        current.advance(InputFrame::default());
        let mut next = Game::new(0, SceneKind::ModeSelect);

        carry_profile(&current, &mut next, Some(&mut profile));

        assert_eq!(profile.inventory.coins, 100);
        assert_eq!(next.state().coins, 100);
        assert_eq!(profile.mode_completion[0].completed_levels, 3);
    }

    #[test]
    fn board_plant_image_uses_loaded_assets_for_supported_plants() {
        assert_eq!(
            board_plant_image(PlantType::Peashooter),
            Some((SEED_PEASHOOTER_IMAGE_ID, 10.0, 22.0, 0.8))
        );
        assert_eq!(
            board_plant_image(PlantType::Sunflower),
            Some((SEED_SUNFLOWER_IMAGE_ID, 12.0, 28.0, 0.8))
        );
        assert_eq!(
            board_plant_image(PlantType::Other(5)),
            Some((BOARD_SNOWPEA_IMAGE_ID, 10.0, 22.0, 0.8))
        );
        assert_eq!(
            board_plant_image(PlantType::Other(8)),
            Some((BOARD_PUFFSHROOM_IMAGE_ID, 16.0, 34.0, 0.9))
        );
        assert_eq!(
            board_plant_image(PlantType::Other(10)),
            Some((BOARD_FUMESHROOM_IMAGE_ID, 6.0, 24.0, 0.7))
        );
        assert_eq!(
            board_plant_image(PlantType::Other(29)),
            Some((BOARD_STARFRUIT_IMAGE_ID, 6.0, 16.0, 0.8))
        );
        assert_eq!(board_plant_image(PlantType::Other(2)), None);
    }

    #[test]
    fn seed_chooser_plant_images_follow_the_source_packet_atlas() {
        assert_eq!(
            seed_chooser_plant_image(PlantType::Other(4)),
            Some((SEED_PACKET_PLANT_BASE_IMAGE_ID, 0.0, 0.0, 1.0))
        );
        assert_eq!(
            seed_chooser_plant_image(PlantType::Other(47)),
            Some((SEED_PACKET_PLANT_BASE_IMAGE_ID + 7, 0.0, 0.0, 1.0))
        );
        assert_eq!(
            seed_chooser_plant_image(PlantType::Other(5)),
            Some((BOARD_SNOWPEA_IMAGE_ID, 10.0, 22.0, 0.8))
        );
        assert_eq!(seed_chooser_plant_image(PlantType::Other(2)), None);
    }

    #[test]
    fn seed_bank_plant_images_use_packet_scale() {
        assert_eq!(
            seed_bank_plant_image(PlantType::Peashooter),
            Some((SEED_PEASHOOTER_IMAGE_ID, 5.0, 8.0, 0.5))
        );
        assert_eq!(
            seed_bank_plant_image(PlantType::Other(4)),
            Some((SEED_PACKET_PLANT_BASE_IMAGE_ID, 0.0, 0.0, 1.0))
        );
    }

    #[test]
    fn progress_meter_follows_wave_countdown_and_flags() {
        let game = Game::new_adventure(7, 6, true, 0, false);
        let mut state = game.state().clone();
        state.board.wave.current = 1;
        state.board.wave.countdown_start = 100;
        state.board.wave.countdown = 100;
        assert_eq!(progress_meter_width(&state), Some(1));
        state.board.wave.countdown = 0;
        assert_eq!(progress_meter_width(&state), Some(15));
        assert_eq!(progress_meter_flag_layout(&state), Some((1, 10)));
    }

    #[test]
    fn progress_meter_starts_full_for_boss_without_spawned_boss() {
        let game = Game::new_adventure(7, 50, false, 0, false);
        let state = game.state().clone();
        assert_eq!(state.scene, SceneKind::Boss);
        assert_eq!(progress_meter_width(&state), Some(150));
    }

    #[test]
    fn board_entity_images_use_loaded_assets_for_board_entities() {
        assert_eq!(
            board_zombie_image(ZombieType::Normal),
            Some(BOARD_ZOMBIE_BODY_IMAGE_ID)
        );
        assert_eq!(
            board_zombie_image(ZombieType::Snorkel),
            Some(BOARD_ZOMBIE_BODY_IMAGE_ID)
        );
        assert_eq!(
            board_zombie_image(ZombieType::Conehead),
            Some(BOARD_ZOMBIE_BODY_IMAGE_ID)
        );
        assert_eq!(board_zombie_image(ZombieType::Boss), None);
        assert_eq!(
            board_zombie_head_image(ZombieType::Football),
            BOARD_ZOMBIE_FOOTBALL_HEAD_IMAGE_ID
        );
        assert_eq!(
            board_zombie_head_image(ZombieType::Newspaper),
            BOARD_ZOMBIE_NEWSPAPER_HEAD_IMAGE_ID
        );
        let mut equipment = RenderFrame::default();
        push_board_zombie_equipment(&mut equipment, 400.0, 300.0, ZombieType::Conehead, true);
        assert_eq!(
            equipment.sprites.first().map(|sprite| sprite.resource_id),
            Some(BOARD_ZOMBIE_CONE_IMAGE_ID)
        );
        equipment.sprites.clear();
        push_board_zombie_equipment(&mut equipment, 400.0, 300.0, ZombieType::Conehead, false);
        assert!(equipment.sprites.is_empty());
        push_board_zombie_equipment(&mut equipment, 400.0, 300.0, ZombieType::Flag, false);
        assert_eq!(equipment.sprites.len(), 3);
        assert_eq!(
            board_projectile_image(ProjectileType::Pea),
            Some((BOARD_PROJECTILE_PEA_IMAGE_ID, 0.75))
        );
        assert_eq!(
            board_projectile_image(ProjectileType::SnowPea),
            Some((BOARD_PROJECTILE_SNOW_PEA_IMAGE_ID, 0.75))
        );
        assert_eq!(
            board_projectile_image(ProjectileType::Melon),
            Some((BOARD_PROJECTILE_MELON_IMAGE_ID, 1.0))
        );
        assert_eq!(
            board_projectile_image(ProjectileType::Other(1)),
            Some((BOARD_PROJECTILE_BASKETBALL_IMAGE_ID, 1.1))
        );
        assert_eq!(board_projectile_image(ProjectileType::Other(2)), None);
        assert_eq!(
            board_coin_image(CoinType::Silver),
            Some((BOARD_COIN_SILVER_IMAGE_ID, 0.65))
        );
        assert_eq!(
            board_coin_image(CoinType::Gold),
            Some((BOARD_COIN_GOLD_IMAGE_ID, 0.65))
        );
        assert_eq!(
            board_coin_image(CoinType::Diamond),
            Some((BOARD_DIAMOND_IMAGE_ID, 0.65))
        );
        assert_eq!(
            board_coin_image(CoinType::UsableSeedPacket),
            Some((SEED_PACKET_NORMAL_IMAGE_ID, 0.8))
        );
        assert_eq!(
            board_coin_image(CoinType::PresentSurvivalMode),
            Some((BOARD_PRESENT_IMAGE_ID, 0.8))
        );
        assert_eq!(
            board_coin_image(CoinType::AwardBagDiamond),
            Some((BOARD_MONEYBAG_IMAGE_ID, 0.5))
        );
        assert_eq!(
            board_coin_image(CoinType::AwardChocolate),
            Some((BOARD_CHOCOLATE_IMAGE_ID, 0.8))
        );
        assert_eq!(
            board_coin_image(CoinType::AwardSilverSunflower),
            Some((BOARD_SILVER_SUNFLOWER_IMAGE_ID, 0.5))
        );
        assert_eq!(
            board_coin_image(CoinType::AwardGoldSunflower),
            Some((BOARD_GOLD_SUNFLOWER_IMAGE_ID, 0.5))
        );
        assert_eq!(
            board_coin_image(CoinType::FinalSeedPacket),
            Some((SEED_PACKET_NORMAL_IMAGE_ID, 0.8))
        );
    }

    #[test]
    fn pickup_reanimations_follow_source_attachment_offsets() {
        assert_eq!(
            board_coin_reanim_offset(CoinType::Silver),
            Some((-1.0, 1.0))
        );
        assert_eq!(board_coin_reanim_offset(CoinType::Gold), Some((-1.0, 1.0)));
        assert_eq!(
            board_coin_reanim_offset(CoinType::Diamond),
            Some((-18.0, -11.0))
        );
        assert!(board_coin_reanim_visible(CoinType::Silver, None, false));
        assert!(!board_coin_reanim_visible(
            CoinType::Silver,
            Some(100),
            false
        ));
        assert!(!board_coin_reanim_visible(CoinType::Gold, None, true));
        assert!(board_coin_reanim_visible(
            CoinType::Diamond,
            Some(100),
            true
        ));
        assert!(!board_coin_reanim_visible(
            CoinType::PresentPlant,
            None,
            false
        ));
    }

    #[test]
    fn jalapeno_fire_matches_source_row_fwoosh_layout() {
        let positions = jalapeno_fire_effect_positions(SceneKind::Day, 0);
        assert_eq!(positions.len(), 12);
        assert_eq!(positions[0], (10.0, 110.0));
        assert_eq!(positions[11], (760.0, 110.0));

        let roof_positions = jalapeno_fire_effect_positions(SceneKind::Roof, 0);
        assert!(roof_positions[0].1 > positions[0].1);
        assert_eq!(roof_positions[11].1, positions[11].1);
    }

    #[test]
    fn board_projectiles_follow_source_scale_and_rotation_rules() {
        assert!((board_projectile_scale(ProjectileType::Puff, 0).unwrap() - 0.225).abs() < 0.0001);
        assert!((board_projectile_scale(ProjectileType::Puff, 30).unwrap() - 0.75).abs() < 0.0001);
        assert_eq!(
            board_projectile_rotation(ProjectileType::Cob, 0),
            std::f32::consts::PI / 2.0
        );
        assert!(
            board_projectile_rotation(ProjectileType::Cabbage, 10)
                < board_projectile_rotation(ProjectileType::Cabbage, 0)
        );
        assert!(
            board_projectile_rotation(ProjectileType::Kernel, 10)
                < board_projectile_rotation(ProjectileType::Kernel, 0)
        );
    }

    #[test]
    fn board_projectile_shadows_follow_source_offsets_and_lob_height() {
        let day_pea =
            board_projectile_shadow(SceneKind::Day, ProjectileType::Pea, 0, 0, 80 * 1_000_000, 0)
                .unwrap();
        assert_eq!(day_pea.0, BOARD_PROJECTILE_SHADOW_DAY_IMAGE_ID);
        assert!((day_pea.1 - 3.0).abs() < 0.0001);
        assert!((day_pea.2 - 120.0).abs() < 0.0001);
        assert!((day_pea.3 - 1.0).abs() < 0.0001);
        assert!((day_pea.4 - 1.0).abs() < 0.0001);

        let night_snowpea = board_projectile_shadow(
            SceneKind::Fog,
            ProjectileType::SnowPea,
            0,
            0,
            80 * 1_000_000,
            0,
        )
        .unwrap();
        assert_eq!(night_snowpea.0, BOARD_PROJECTILE_SHADOW_NIGHT_IMAGE_ID);
        assert!((night_snowpea.1 + 1.0).abs() < 0.0001);
        assert!((night_snowpea.3 - 1.3).abs() < 0.0001);

        let lobbed = board_projectile_shadow(
            SceneKind::Day,
            ProjectileType::Melon,
            0,
            0,
            80 * 1_000_000,
            -100_000,
        )
        .unwrap();
        assert!(lobbed.3 < 1.6);
        assert!(
            board_projectile_shadow(
                SceneKind::Day,
                ProjectileType::Puff,
                0,
                0,
                80 * 1_000_000,
                0,
            )
            .is_none()
        );
    }

    #[test]
    fn compiled_zombie_layers_follow_source_equipment_visibility() {
        assert!(board_zombie_reanim_track_visible(
            "anim_cone",
            ZombieType::Conehead,
            true,
            true,
            true,
            false
        ));
        assert!(!board_zombie_reanim_track_visible(
            "anim_cone",
            ZombieType::Conehead,
            false,
            true,
            true,
            false
        ));
        assert!(board_zombie_reanim_track_visible(
            "Zombie_outerarm_screendoor",
            ZombieType::ScreenDoor,
            true,
            true,
            true,
            false
        ));
        assert!(!board_zombie_reanim_track_visible(
            "Zombie_outerarm_screendoor",
            ZombieType::Normal,
            true,
            true,
            true,
            false
        ));
        assert!(board_zombie_reanim_track_visible(
            "Zombie_flaghand",
            ZombieType::Flag,
            true,
            true,
            true,
            false
        ));
        assert!(!board_zombie_reanim_track_visible(
            "anim_head1",
            ZombieType::Normal,
            true,
            false,
            true,
            false
        ));
        assert!(board_zombie_reanim_track_visible(
            "Zombie_mustache",
            ZombieType::Normal,
            true,
            true,
            true,
            true
        ));
        assert!(!board_zombie_reanim_track_visible(
            "Zombie_mustache",
            ZombieType::Normal,
            true,
            false,
            true,
            true
        ));
        assert!(board_zombie_reanim_track_visible(
            "Zombie_duckytube",
            ZombieType::DuckyTube,
            true,
            true,
            true,
            false
        ));
        assert_eq!(mustache_image_symbol(3), "IMAGE_REANIM_ZOMBIE_MUSTACHE3");
        assert_eq!(
            future_head_image_symbol(2),
            "IMAGE_REANIM_ZOMBIE_HEAD_SUNGLASSES3"
        );
    }

    #[test]
    fn hidden_code_keys_audio_and_effect_anchors_use_source_mappings() {
        assert_eq!(typing_code_character(KeyCode::KeyA), 'a');
        assert_eq!(typing_code_character(KeyCode::KeyZ), 'z');
        assert_eq!(typing_code_character(KeyCode::Digit1), '\0');
        for (code, path) in [
            (HiddenCode::Mustache, "sounds/polevault.ogg"),
            (HiddenCode::Future, "sounds/boing.ogg"),
            (HiddenCode::Pinata, "sounds/juicy.ogg"),
            (HiddenCode::Daisies, "sounds/loadingbar_flower.ogg"),
            (HiddenCode::Sukhbir, "sounds/sukhbir.ogg"),
        ] {
            assert_eq!(
                audio_for_event(&GameEvent::HiddenCodeToggled {
                    code,
                    enabled: true,
                }),
                Some((AudioKind::Effect, path))
            );
        }
        assert_eq!(
            audio_for_event(&GameEvent::HiddenCodeRejected {
                code: HiddenCode::Pinata,
            }),
            Some((AudioKind::Effect, "sounds/buzzer.ogg"))
        );

        let mut anchors = BoardVisualAnchors::default();
        anchors.zombies.insert(
            1,
            BoardVisualAnchor {
                x: 300.0,
                y: 400.0,
                previous_x: 300.0,
                previous_y: 400.0,
                row: 2,
                zombie_type: Some(ZombieType::Pogo),
            },
        );
        let empty = BoardVisualAnchors::default();
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieMustacheDropped {
                    entity: 1,
                    variant: 3,
                },
                SceneKind::Day,
                &anchors,
                &empty,
            ),
            vec![("ZombieMustache", 360.0, 440.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieFutureGlassesDropped {
                    entity: 1,
                    frame: 1,
                },
                SceneKind::Day,
                &anchors,
                &empty,
            ),
            vec![("ZombieFutureGlasses", 360.0, 440.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombiePinataDropped { entity: 1 },
                SceneKind::Day,
                &anchors,
                &empty,
            ),
            vec![("Pinata", 360.0, 440.0, 11)]
        );
        assert_eq!(
            particle_effects_for_event(
                &GameEvent::ZombieDaisiesDropped { entity: 1 },
                SceneKind::Night,
                &anchors,
                &empty,
            ),
            vec![("Daisy", 320.0, 535.0, 11)]
        );
    }

    #[test]
    fn specialized_zombie_layers_follow_source_equipment_visibility() {
        assert!(board_specialized_zombie_reanim_track_visible(
            "zombie_football_helmet",
            ZombieType::Football,
            true,
            true,
            true
        ));
        assert!(!board_specialized_zombie_reanim_track_visible(
            "zombie_football_helmet",
            ZombieType::Football,
            false,
            true,
            true
        ));
        assert!(!board_specialized_zombie_reanim_track_visible(
            "anim_hair",
            ZombieType::Football,
            true,
            true,
            true
        ));
        assert!(board_specialized_zombie_reanim_track_visible(
            "anim_hair",
            ZombieType::Football,
            false,
            true,
            true
        ));
        assert!(board_specialized_zombie_reanim_track_visible(
            "Zombie_paper_paper",
            ZombieType::Newspaper,
            true,
            true,
            true
        ));
        assert!(!board_specialized_zombie_reanim_track_visible(
            "Zombie_paper_paper",
            ZombieType::Newspaper,
            false,
            true,
            true
        ));
        assert!(!board_specialized_zombie_reanim_track_visible(
            "zombie_football_leftarm_lower",
            ZombieType::Football,
            true,
            true,
            false
        ));
        assert!(board_specialized_zombie_reanim_track_visible(
            "zombie_football_leftarm_upper",
            ZombieType::Football,
            true,
            true,
            false
        ));
        assert!(!board_specialized_zombie_reanim_track_visible(
            "Zombie_paper_hands",
            ZombieType::Newspaper,
            true,
            true,
            false
        ));
        assert!(board_specialized_zombie_reanim_track_visible(
            "anim_hairpiece",
            ZombieType::Newspaper,
            true,
            true,
            true
        ));
    }

    #[test]
    fn specialized_reanim_files_match_the_target_archive() {
        assert_eq!(SPECIALIZED_REANIM_FILES.len(), 16);
        assert!(
            SPECIALIZED_REANIM_FILES
                .contains(&(ZombieType::Dancer, "Zombie_dancer.reanim.compiled"))
        );
        assert!(
            SPECIALIZED_REANIM_FILES
                .contains(&(ZombieType::Snorkel, "Zombie_snorkle.reanim.compiled"))
        );
        assert!(
            !SPECIALIZED_REANIM_FILES
                .iter()
                .any(|(zombie_type, _)| *zombie_type == ZombieType::BackupDancer)
        );
    }

    #[test]
    fn plant_reanim_files_match_loaded_board_assets() {
        assert_eq!(PLANT_REANIM_FILES.len(), 49);
        assert!(
            PLANT_REANIM_FILES
                .contains(&(PlantType::Peashooter, "PeaShooterSingle.reanim.compiled"))
        );
        assert!(PLANT_REANIM_FILES.contains(&(PlantType::Other(21), "Caltrop.reanim.compiled")));
        assert!(PLANT_REANIM_FILES.contains(&(PlantType::Other(34), "Cornpult.reanim.compiled")));
        assert!(PLANT_REANIM_FILES.contains(&(PlantType::Other(48), "Imitater.reanim.compiled")));
        for slot in 2..=48 {
            assert!(
                PLANT_REANIM_FILES
                    .iter()
                    .any(|(plant_type, _)| *plant_type == PlantType::Other(slot))
            );
        }
    }

    #[test]
    fn plant_reanim_actions_follow_source_state_tracks() {
        assert_eq!(
            board_plant_reanim_actions(PlantType::Other(8), true, 0, false, 0, 0),
            &["anim_sleep", "anim_idle"]
        );
        assert_eq!(
            board_plant_reanim_actions(PlantType::Other(4), false, 0, true, 0, 0),
            &["anim_armed", "anim_idle"]
        );
        assert_eq!(
            board_plant_reanim_actions(PlantType::Other(20), false, 1, false, 0, 0),
            &["anim_explode", "anim_idle"]
        );
        assert_eq!(
            board_plant_reanim_actions(PlantType::Other(7), false, 0, false, 1, 0),
            &["anim_shooting", "anim_shoot", "anim_idle"]
        );
        assert_eq!(
            board_plant_reanim_actions(PlantType::Other(9), false, 0, false, 0, 1),
            &["anim_bigidle", "anim_idle"]
        );
    }

    #[test]
    fn plant_reanim_track_visibility_follows_source_groups() {
        assert_eq!(
            board_plant_reanim_track_style(PlantType::Other(4), false, None, "anim_glow"),
            None
        );
        assert_eq!(
            board_plant_reanim_track_style(PlantType::Other(4), true, None, "anim_glow"),
            Some((10, BlendMode::Alpha))
        );
        assert_eq!(
            board_plant_reanim_track_style(PlantType::Other(30), false, None, "Pumpkin_back"),
            Some((9, BlendMode::Alpha))
        );
        assert_eq!(
            board_plant_reanim_track_style(PlantType::Other(34), false, None, "Cornpult_butter"),
            None
        );
        assert_eq!(
            board_plant_reanim_track_style(
                PlantType::Other(34),
                false,
                Some(ProjectileType::Butter),
                "Cornpult_butter"
            ),
            Some((10, BlendMode::Alpha))
        );
        assert_eq!(
            board_plant_reanim_track_style(
                PlantType::Other(34),
                false,
                Some(ProjectileType::Butter),
                "Cornpult_kernal"
            ),
            None
        );
    }

    #[test]
    fn plant_reanim_attachments_follow_source_layer_setup() {
        assert_eq!(
            board_plant_reanim_attachment_specs(PlantType::Peashooter, "anim_idle", 0),
            &[("anim_stem", "anim_head_idle")]
        );
        assert_eq!(
            board_plant_reanim_attachment_specs(PlantType::Other(28), "anim_shooting", 1),
            &[("anim_idle", "anim_shooting")]
        );
        assert_eq!(
            board_plant_reanim_attachment_specs(PlantType::Other(18), "anim_idle", 0),
            &[
                ("anim_head1", "anim_head_idle1"),
                ("anim_head2", "anim_head_idle2"),
                ("anim_head3", "anim_head_idle3"),
            ]
        );
        assert_eq!(
            board_plant_reanim_track_style(PlantType::Peashooter, false, None, "anim_head_idle"),
            None
        );
        assert!(board_plant_reanim_attached_track_visible(
            "anim_head_idle",
            "anim_head_idle"
        ));

        let transform = ReanimatorTransform {
            x: 10.0,
            y: 20.0,
            skew_x: 0.0,
            skew_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            frame: 0.0,
            alpha: 1.0,
            image: None,
        };
        let matrix = reanim_matrix_mul(
            reanim_matrix_translation(100.0, 200.0),
            reanim_matrix_from_transform(&transform),
        );
        assert!((matrix.m02 - 110.0).abs() < 0.001);
        assert!((matrix.m12 - 220.0).abs() < 0.001);
        let inverse = reanim_matrix_inverse(reanim_matrix_from_transform(&transform)).unwrap();
        let identity = reanim_matrix_mul(reanim_matrix_from_transform(&transform), inverse);
        assert!((identity.m00 - 1.0).abs() < 0.001);
        assert!((identity.m11 - 1.0).abs() < 0.001);
        assert!(identity.m02.abs() < 0.001);
        assert!(identity.m12.abs() < 0.001);
    }

    #[test]
    fn pool_rise_seaweed_uses_source_tracks_offsets_and_current_transforms() {
        assert_eq!(
            zombie_seaweed_attachment_specs(ZombieType::Normal),
            [
                ("anim_head1", 30.0, 20.0),
                ("Zombie_outerarm_upper", 5.0, 5.0),
                ("Zombie_duckytube", 77.0, 20.0),
            ]
        );
        assert_eq!(
            zombie_seaweed_attachment_specs(ZombieType::Conehead)[0],
            ("anim_cone", 37.0, 20.0)
        );
        assert_eq!(
            zombie_seaweed_attachment_specs(ZombieType::Buckethead)[0],
            ("anim_bucket", 37.0, 20.0)
        );

        let transform = |x, y| ReanimatorTransform {
            x,
            y,
            skew_x: 0.0,
            skew_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            frame: 0.0,
            alpha: 1.0,
            image: None,
        };
        let definition = ReanimatorDefinition {
            fps: 100.0,
            tracks: vec![
                ReanimatorTrack {
                    name: "anim_walk".to_owned(),
                    transforms: vec![transform(0.0, 0.0), transform(0.0, 0.0)],
                },
                ReanimatorTrack {
                    name: "anim_head1".to_owned(),
                    transforms: vec![transform(10.0, 20.0), transform(30.0, 20.0)],
                },
            ],
        };
        assert_eq!(
            reanim_attachment_position(
                &definition,
                "anim_walk",
                0,
                (100.0, 200.0),
                "anim_head1",
                (30.0, 20.0),
            ),
            Some((140.0, 240.0))
        );
        assert_eq!(
            reanim_attachment_position(
                &definition,
                "anim_walk",
                1,
                (100.0, 200.0),
                "anim_head1",
                (30.0, 20.0),
            ),
            Some((150.0, 240.0))
        );
    }

    #[test]
    fn boss_reanim_actions_and_layers_follow_source_groups() {
        assert_eq!(boss_reanim_action(10, false, false), "anim_idle");
        assert_eq!(boss_reanim_action(10, false, true), "anim_head_attack_1");
        assert_eq!(boss_reanim_action(10, true, true), "anim_RV_1");
        assert_eq!(boss_reanim_action(0, false, false), "anim_death");
        assert_eq!(board_boss_reanim_track_z("Boss_innerleg_upper"), 5);
        assert_eq!(board_boss_reanim_track_z("Boss_body2"), 6);
        assert_eq!(board_boss_reanim_track_z("boss_body1"), 7);
        assert_eq!(board_boss_reanim_track_z("Boss_innerarm_hand"), 8);
        assert_eq!(board_boss_ball_track_z("Layer 47"), 11);
        assert_eq!(board_boss_ball_track_z("multiply"), 12);
        assert_eq!(board_boss_ball_track_z("additive"), 14);
        assert_eq!(board_boss_ball_blend_mode("additive"), BlendMode::Additive);
        assert_eq!(board_boss_ball_blend_mode("ice_overlay"), BlendMode::Alpha);
    }
}
