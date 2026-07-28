use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
    process::ExitCode,
    sync::Arc,
    time::{Duration, Instant},
};

use clap::{Parser, ValueEnum};
use neopvz_audio::{AudioBackend, AudioKind, KiraAudioBackend};
use neopvz_core::{
    Game, GameEvent, GardenServiceKind, InputAction, InputFrame, ModeKind, SaveError, SaveProfile,
    SceneKind, mode_level_name, mode_level_names,
};
use neopvz_data::{AssetLayout, ResourceProvider};
use neopvz_render::{
    AffineSpriteCommand, BOSS_BACKGROUND_IMAGE_ID, CHALLENGE_THUMBNAIL_BASE_IMAGE_ID,
    CRAZY_DAVE_BEARD_IMAGE_ID, CRAZY_DAVE_BODY_IMAGE_ID, CRAZY_DAVE_EYE_IMAGE_ID,
    CRAZY_DAVE_EYEBROW_IMAGE_ID, CRAZY_DAVE_HEAD_IMAGE_ID, CRAZY_DAVE_INNER_ARM_IMAGE_ID,
    CRAZY_DAVE_INNER_FINGER1_IMAGE_ID, CRAZY_DAVE_INNER_FINGER2_IMAGE_ID,
    CRAZY_DAVE_INNER_FINGER3_IMAGE_ID, CRAZY_DAVE_INNER_FINGER4_IMAGE_ID,
    CRAZY_DAVE_INNER_HAND_IMAGE_ID, CRAZY_DAVE_MOUTH_IMAGE_ID, CRAZY_DAVE_OUTER_ARM_IMAGE_ID,
    CRAZY_DAVE_OUTER_FINGER1_IMAGE_ID, CRAZY_DAVE_OUTER_FINGER2_IMAGE_ID,
    CRAZY_DAVE_OUTER_FINGER3_IMAGE_ID, CRAZY_DAVE_OUTER_FINGER4_IMAGE_ID,
    CRAZY_DAVE_OUTER_HAND_IMAGE_ID, CRAZY_DAVE_POT_IMAGE_ID, DAY_BACKGROUND_IMAGE_ID,
    FOG_BACKGROUND_IMAGE_ID, GpuRenderer, ImageAsset, LogicalViewport,
    MODE_SELECT_BACKGROUND_IMAGE_ID, MODE_SELECT_BLANK_IMAGE_ID, MODE_SELECT_WINDOW_IMAGE_ID,
    NIGHT_BACKGROUND_IMAGE_ID, POOL_BACKGROUND_IMAGE_ID, ROOF_BACKGROUND_IMAGE_ID, RenderFrame,
    SCREEN_PIXEL_IMAGE_ID, SEED_CHOOSER_BUTTON_IMAGE_ID, SEED_CHOOSER_IMAGE_ID,
    SEED_CHOOSER_TITLE_IMAGE_ID, SEED_PACKET_NORMAL_IMAGE_ID, SEED_PACKET_SILHOUETTE_IMAGE_ID,
    SEED_PEASHOOTER_IMAGE_ID, SEED_SUNFLOWER_IMAGE_ID, SELECTOR_ADVENTURE_IMAGE_ID,
    SELECTOR_ALMANAC_IMAGE_ID, SELECTOR_BASE_IMAGE_ID, SELECTOR_CENTER_IMAGE_ID,
    SELECTOR_CHALLENGES_IMAGE_ID, SELECTOR_HELP_IMAGE_ID, SELECTOR_LEAVES_IMAGE_ID,
    SELECTOR_LEFT_IMAGE_ID, SELECTOR_OPTIONS_IMAGE_ID, SELECTOR_QUIT_IMAGE_ID,
    SELECTOR_RIGHT_IMAGE_ID, SELECTOR_STORE_IMAGE_ID, SELECTOR_SURVIVAL_IMAGE_ID,
    SELECTOR_TROPHY_IMAGE_ID, SELECTOR_VASEBREAKER_IMAGE_ID, SELECTOR_WOODSIGN1_IMAGE_ID,
    SELECTOR_WOODSIGN2_IMAGE_ID, SELECTOR_WOODSIGN3_IMAGE_ID, SELECTOR_ZEN_GARDEN_IMAGE_ID,
    SURVIVAL_THUMBNAIL_BASE_IMAGE_ID, SpriteCommand, TITLE_IMAGE_ID, TITLE_LOAD_BAR_DIRT_IMAGE_ID,
    TITLE_LOAD_BAR_GRASS_IMAGE_ID, TITLE_LOAD_BAR_ROCK1_IMAGE_ID, TITLE_LOAD_BAR_ROCK3_IMAGE_ID,
    TITLE_LOAD_BAR_SPROUT_BODY_IMAGE_ID, TITLE_LOAD_BAR_SPROUT_PETAL_IMAGE_ID,
    TITLE_LOAD_BAR_ZOMBIE_HAIR_IMAGE_ID, TITLE_LOAD_BAR_ZOMBIE_HEAD_IMAGE_ID,
    TITLE_LOAD_BAR_ZOMBIE_JAW_IMAGE_ID, TITLE_LOGO_IMAGE_ID, TITLE_START_PROMPT_HOVER_IMAGE_ID,
    TITLE_START_PROMPT_IMAGE_ID, TITLE_START_PROMPT_SHADOW_IMAGE_ID, TUTORIAL_BUBBLE_IMAGE_ID,
    TUTORIAL_CONTINUE_IMAGE_ID, TUTORIAL_TEXT1_IMAGE_ID, TUTORIAL_TEXT2_IMAGE_ID,
    UI_PIXEL_IMAGE_ID, logical_position,
};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalPosition},
    event::{ElementState, MouseButton, WindowEvent},
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
    #[arg(long, help = "Start in borderless fullscreen")]
    fullscreen: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Checkpoint {
    Title,
    AdventureSelect,
    AdventureTutorial,
    SeedChooser,
    ModeSelect,
    Day,
    GameOver,
    GameLost,
    GameWon,
    Pickups,
    GardenWater,
    GardenFertilize,
    IceShroom,
    PotatoMine,
    ExplosionPlants,
    ExplodeONut,
    BrainEaten,
    Butter,
    VaseBreak,
    Rake,
    BloverChomper,
    HypnoJackbox,
    CobCannon,
    Portal,
    GraveBuster,
    Coffee,
    TangleKelp,
    Spikeweed,
    Digger,
    Magnet,
    Zamboni,
    PogoBlock,
    UmbrellaDeflect,
}

impl From<Checkpoint> for SceneKind {
    fn from(checkpoint: Checkpoint) -> Self {
        match checkpoint {
            Checkpoint::Title => Self::Title,
            Checkpoint::AdventureSelect => Self::AdventureSelect,
            Checkpoint::AdventureTutorial => Self::AdventureTutorial,
            Checkpoint::SeedChooser => Self::SeedChooser,
            Checkpoint::ModeSelect => Self::ModeSelect,
            Checkpoint::Day => Self::Day,
            Checkpoint::GameOver => Self::GameOver,
            Checkpoint::GameLost => Self::Day,
            Checkpoint::GameWon => Self::Day,
            Checkpoint::Pickups => Self::Day,
            Checkpoint::GardenWater => Self::Garden,
            Checkpoint::GardenFertilize => Self::Garden,
            Checkpoint::IceShroom => Self::Night,
            Checkpoint::PotatoMine => Self::Day,
            Checkpoint::ExplosionPlants => Self::Night,
            Checkpoint::ExplodeONut => Self::Day,
            Checkpoint::BrainEaten => Self::Night,
            Checkpoint::Butter => Self::Day,
            Checkpoint::VaseBreak => Self::Day,
            Checkpoint::Rake => Self::Day,
            Checkpoint::BloverChomper => Self::Day,
            Checkpoint::HypnoJackbox => Self::Night,
            Checkpoint::CobCannon => Self::Day,
            Checkpoint::Portal => Self::Day,
            Checkpoint::GraveBuster => Self::Day,
            Checkpoint::Coffee => Self::Day,
            Checkpoint::TangleKelp => Self::Pool,
            Checkpoint::Spikeweed => Self::Day,
            Checkpoint::Digger => Self::Day,
            Checkpoint::Magnet => Self::Night,
            Checkpoint::Zamboni => Self::Day,
            Checkpoint::PogoBlock => Self::Day,
            Checkpoint::UmbrellaDeflect => Self::Day,
        }
    }
}

const SIMULATION_STEP: Duration = Duration::from_millis(10);
const TITLE_LOAD_BAR_X: f32 = 243.0;
const TITLE_LOAD_BAR_Y: f32 = 530.0;
const TITLE_START_BUTTON_Y: f32 = 529.0;
const TITLE_START_BUTTON_WIDTH: f32 = 314.0;
const TITLE_START_BUTTON_HEIGHT: f32 = 50.0;

#[derive(Clone, Copy)]
struct TitleReanimPart {
    resource_id: u32,
    x: f32,
    y: f32,
    skew_x: f32,
    skew_y: f32,
    scale_x: f32,
    scale_y: f32,
}

const fn title_reanim_part(
    resource_id: u32,
    x: f32,
    y: f32,
    skew_x: f32,
    skew_y: f32,
    scale_x: f32,
    scale_y: f32,
) -> TitleReanimPart {
    TitleReanimPart {
        resource_id,
        x,
        y,
        skew_x,
        skew_y,
        scale_x,
        scale_y,
    }
}

#[rustfmt::skip]
const TITLE_SPROUT_PARTS: [TitleReanimPart; 10] = [
    title_reanim_part(TITLE_LOAD_BAR_ROCK3_IMAGE_ID,          5.2, 22.6,   15.0,   15.0, 0.200, 0.200),
    title_reanim_part(TITLE_LOAD_BAR_SPROUT_BODY_IMAGE_ID,   -1.5,  4.5,    0.0,    0.0, 0.800, 0.753),
    title_reanim_part(TITLE_LOAD_BAR_ROCK3_IMAGE_ID,         -0.2, 29.6, -119.9, -119.9, 0.424, 0.424),
    title_reanim_part(TITLE_LOAD_BAR_ROCK3_IMAGE_ID,          6.0, 22.5,    0.0,    0.0, 0.382, 0.359),
    title_reanim_part(TITLE_LOAD_BAR_ROCK3_IMAGE_ID,          1.2, 24.1,   15.0,   15.0, 0.200, 0.200),
    title_reanim_part(TITLE_LOAD_BAR_ROCK3_IMAGE_ID,          6.4, 24.9,   15.0,   15.0, 0.200, 0.200),
    title_reanim_part(TITLE_LOAD_BAR_ROCK1_IMAGE_ID,          2.1, 28.9, -104.9, -104.9, 0.555, 0.555),
    title_reanim_part(TITLE_LOAD_BAR_SPROUT_PETAL_IMAGE_ID,   5.0, -6.4,   84.5,   82.5, 0.800, 0.800),
    title_reanim_part(TITLE_LOAD_BAR_SPROUT_PETAL_IMAGE_ID,  11.1, -4.1,  135.0,  -44.9, 0.800, 0.800),
    title_reanim_part(TITLE_LOAD_BAR_SPROUT_PETAL_IMAGE_ID,  -5.7,  1.5,    7.3,    7.3, 0.800, 0.800),
];

#[rustfmt::skip]
const TITLE_ZOMBIE_PARTS: [TitleReanimPart; 12] = [
    title_reanim_part(TITLE_LOAD_BAR_ZOMBIE_HEAD_IMAGE_ID, -14.8, -10.1,    1.5,    1.6, 0.799, 0.776),
    title_reanim_part(TITLE_LOAD_BAR_ZOMBIE_HAIR_IMAGE_ID, -18.5, -12.6,    2.8,    2.8, 0.800, 0.774),
    title_reanim_part(TITLE_LOAD_BAR_ZOMBIE_JAW_IMAGE_ID,   -9.9,  18.0,    0.0,    0.0, 0.820, 0.792),
    title_reanim_part(TITLE_LOAD_BAR_ROCK3_IMAGE_ID,         26.3,  28.1,  150.0,  150.0, 0.437, 0.437),
    title_reanim_part(TITLE_LOAD_BAR_ROCK3_IMAGE_ID,         12.6,  23.4,    0.0,    0.0, 0.400, 0.400),
    title_reanim_part(TITLE_LOAD_BAR_ROCK1_IMAGE_ID,         19.6,  29.8,  165.0,  165.0, 0.700, 0.700),
    title_reanim_part(TITLE_LOAD_BAR_ROCK1_IMAGE_ID,         -0.3,  27.6, -179.9, -179.9, 0.555, 0.555),
    title_reanim_part(TITLE_LOAD_BAR_ROCK1_IMAGE_ID,         16.9,  27.3,  135.0,  135.0, 0.700, 0.700),
    title_reanim_part(TITLE_LOAD_BAR_ROCK3_IMAGE_ID,         -2.4,  29.2, -179.9, -179.9, 0.471, 0.445),
    title_reanim_part(TITLE_LOAD_BAR_ROCK3_IMAGE_ID,         -9.3,  28.7,  -59.9,  -59.9, 0.492, 0.555),
    title_reanim_part(TITLE_LOAD_BAR_ROCK3_IMAGE_ID,          5.1,  23.8,   15.0,   15.0, 0.400, 0.400),
    title_reanim_part(TITLE_LOAD_BAR_ROCK1_IMAGE_ID,          7.2,  27.5, -224.9, -224.9, 0.648, 0.648),
];

fn title_start_contains(x: f32, y: f32) -> bool {
    (TITLE_LOAD_BAR_X..TITLE_LOAD_BAR_X + TITLE_START_BUTTON_WIDTH).contains(&x)
        && (TITLE_START_BUTTON_Y..TITLE_START_BUTTON_Y + TITLE_START_BUTTON_HEIGHT).contains(&y)
}

fn push_title_reanimation(
    frame: &mut RenderFrame,
    x: f32,
    y: f32,
    overlay_scale_x: f32,
    overlay_scale_y: f32,
    parts: &[TitleReanimPart],
) {
    for part in parts {
        let skew_x = -part.skew_x.to_radians();
        let skew_y = -part.skew_y.to_radians();
        frame.affine_sprites.push(AffineSpriteCommand {
            resource_id: part.resource_id,
            x: x + overlay_scale_x * part.x,
            y: y + overlay_scale_y * part.y,
            m00: overlay_scale_x * skew_x.cos() * part.scale_x,
            m01: overlay_scale_x * skew_y.sin() * part.scale_y,
            m10: overlay_scale_y * -skew_x.sin() * part.scale_x,
            m11: overlay_scale_y * skew_y.cos() * part.scale_y,
            z: 4,
            alpha: 1.0,
        });
    }
}

fn push_title_load_bar_reanimations(frame: &mut RenderFrame) {
    for (index, fraction) in [0.11, 0.32, 0.54, 0.72, 0.91].into_iter().enumerate() {
        let x = TITLE_START_BUTTON_WIDTH * fraction + 225.0;
        let mut y = 511.0;
        let mut scale_x = 1.0;
        let mut scale_y = 1.0;
        let parts: &[TitleReanimPart] = if index == 4 {
            &TITLE_ZOMBIE_PARTS
        } else {
            if index == 1 || index == 3 {
                scale_x = -1.0;
            } else if index == 2 {
                y -= 5.0;
                scale_x = 1.1;
                scale_y = 1.3;
            }
            &TITLE_SPROUT_PARTS
        };
        push_title_reanimation(frame, x, y, scale_x, scale_y, parts);
    }
}

fn main() -> ExitCode {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let profile_path = cli.profile;
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
    let force_game_lost = matches!(cli.checkpoint, Some(Checkpoint::GameLost));
    let force_game_won = matches!(cli.checkpoint, Some(Checkpoint::GameWon));
    let force_pickups = matches!(cli.checkpoint, Some(Checkpoint::Pickups));
    let initial_scene = if force_game_over || force_game_lost || force_game_won || force_pickups {
        SceneKind::Day
    } else {
        cli.checkpoint
            .map(SceneKind::from)
            .unwrap_or(SceneKind::Title)
    };
    let assets = match load_assets(&resources) {
        Ok(assets) => assets,
        Err(error) => {
            tracing::error!(%error, "required display resources failed to load");
            return ExitCode::FAILURE;
        }
    };
    let audio = match KiraAudioBackend::new() {
        Ok(audio) => Some(audio),
        Err(error) => {
            tracing::warn!(%error, "audio backend unavailable; continuing without audio");
            None
        }
    };

    let event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop,
        Err(error) => {
            tracing::error!(%error, "event loop creation failed");
            return ExitCode::FAILURE;
        }
    };
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(
        assets,
        resources,
        audio,
        initial_scene,
        cli.fullscreen,
        cli.checkpoint,
    );
    let run_result = event_loop.run_app(&mut app);

    if let Err(error) = run_result {
        tracing::error!(%error, "event loop failed");
        return ExitCode::FAILURE;
    }

    if let (Some(path), Some(profile)) = (profile_path, profile.take()) {
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

fn load_assets(resources: &ResourceProvider) -> Result<Vec<ImageAsset>, String> {
    let mut assets = vec![
        load_image(resources, TITLE_IMAGE_ID, "images/titlescreen.jpg")?,
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
        render_colored_text_image(
            TITLE_START_PROMPT_SHADOW_IMAGE_ID,
            "\u{70b9}\u{51fb}\u{5f00}\u{59cb}",
            120,
            24,
            16,
            [71, 45, 0],
        )?,
        render_colored_text_image(
            TITLE_START_PROMPT_IMAGE_ID,
            "\u{70b9}\u{51fb}\u{5f00}\u{59cb}",
            120,
            24,
            16,
            [218, 184, 33],
        )?,
        render_colored_text_image(
            TITLE_START_PROMPT_HOVER_IMAGE_ID,
            "\u{70b9}\u{51fb}\u{5f00}\u{59cb}",
            120,
            24,
            16,
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
    ];
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
    Ok(assets)
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
    for (color_pixel, mask_pixel) in rgba8.chunks_exact_mut(4).zip(mask.rgba8.chunks_exact(4)) {
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
    for pixel in asset.rgba8.chunks_exact_mut(4) {
        if pixel[3] != 0 {
            pixel[..3].copy_from_slice(&color);
        }
    }
    Ok(asset)
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
        fn DeleteObject(object: Handle) -> i32;
        fn DeleteDC(device_context: Handle) -> i32;
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
            {
                DeleteObject(font);
                DeleteObject(bitmap);
                DeleteDC(device_context);
                return Err("DrawTextW failed".to_owned());
            }

            let mut rgba = Vec::with_capacity(pixel_count * 4);
            for pixel in buffer.chunks_exact(4) {
                let luminance = (u16::from(pixel[0]) * 29
                    + u16::from(pixel[1]) * 150
                    + u16::from(pixel[2]) * 77)
                    / 256;
                rgba.extend([0, 0, 0, 255_u16.saturating_sub(luminance) as u8]);
            }
            DeleteObject(font);
            DeleteObject(bitmap);
            DeleteDC(device_context);
            ImageAsset::new(resource_id, width, height, rgba).map_err(|error| error.to_string())
        }
    }
}

struct App {
    renderer: Option<GpuRenderer>,
    assets: Vec<ImageAsset>,
    resources: ResourceProvider,
    audio: Option<KiraAudioBackend>,
    game: Game,
    pending_input: Vec<InputAction>,
    last_update: Option<Instant>,
    simulation_accumulator: Duration,
    cursor_position: Option<PhysicalPosition<f64>>,
    tutorial_page: u8,
    seed_chooser_selection: [bool; 2],
    selected_mode: ModeKind,
    selected_level: u8,
    fullscreen: bool,
    startup_events: Vec<GameEvent>,
}

impl App {
    fn new(
        assets: Vec<ImageAsset>,
        resources: ResourceProvider,
        audio: Option<KiraAudioBackend>,
        initial_scene: SceneKind,
        fullscreen: bool,
        checkpoint: Option<Checkpoint>,
    ) -> Self {
        let mut game = match checkpoint {
            Some(Checkpoint::GardenWater | Checkpoint::GardenFertilize) => {
                Game::new_mode(0, ModeKind::ZenGarden, 0)
            }
            Some(Checkpoint::Butter) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::VaseBreak) => Game::new_mode(0, ModeKind::Vasebreaker, 0),
            Some(Checkpoint::Rake) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::ExplosionPlants) => Game::new(0, SceneKind::Night),
            Some(Checkpoint::ExplodeONut) => Game::new_mode(0, ModeKind::MiniGame, 1),
            Some(Checkpoint::BrainEaten) => Game::new_mode(0, ModeKind::IZombie, 0),
            Some(Checkpoint::BloverChomper) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::HypnoJackbox) => Game::new(0, SceneKind::Night),
            Some(Checkpoint::CobCannon) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::GraveBuster) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::Coffee) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::TangleKelp) => Game::new(0, SceneKind::Pool),
            Some(Checkpoint::Spikeweed) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::Digger) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::Magnet) => Game::new(0, SceneKind::Night),
            Some(Checkpoint::Zamboni) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::PogoBlock) => Game::new(0, SceneKind::Day),
            Some(Checkpoint::UmbrellaDeflect) => Game::new(0, SceneKind::Day),
            _ => new_scene_game(initial_scene),
        };
        let mut pending_input = Vec::new();
        let mut startup_events = Vec::new();
        match checkpoint {
            Some(Checkpoint::GameOver) => game.debug_force_game_over(),
            Some(Checkpoint::GameLost) => game.debug_prepare_game_lost(),
            Some(Checkpoint::GameWon) => game.debug_prepare_game_won(),
            Some(Checkpoint::Pickups) => {
                let (sun, coin) = game.debug_prepare_pickups();
                pending_input.push(InputAction::CollectSun { entity: sun });
                pending_input.push(InputAction::CollectCoin { entity: coin });
            }
            Some(Checkpoint::GardenWater) => {
                pending_input.push(InputAction::GardenWater { plant: 0 });
            }
            Some(Checkpoint::GardenFertilize) => {
                pending_input.push(InputAction::GardenFertilize { plant: 0 });
            }
            Some(Checkpoint::IceShroom) => game.debug_prepare_ice_shroom(),
            Some(Checkpoint::PotatoMine) => game.debug_prepare_potato_mine(),
            Some(Checkpoint::ExplosionPlants) => game.debug_prepare_explosion_plants(),
            Some(Checkpoint::ExplodeONut) => game.debug_prepare_explode_o_nut(),
            Some(Checkpoint::BrainEaten) => game.debug_prepare_brain_finished(),
            Some(Checkpoint::Butter) => startup_events = game.debug_prepare_butter(),
            Some(Checkpoint::VaseBreak) => startup_events = game.debug_prepare_vase_break(),
            Some(Checkpoint::Rake) => startup_events = game.debug_prepare_rake(),
            Some(Checkpoint::BloverChomper) => game.debug_prepare_blover_chomper(),
            Some(Checkpoint::HypnoJackbox) => game.debug_prepare_hypno_jackbox(),
            Some(Checkpoint::CobCannon) => game.debug_prepare_cob_cannon(),
            Some(Checkpoint::Portal) => startup_events = game.debug_prepare_portal(),
            Some(Checkpoint::GraveBuster) => game.debug_prepare_gravebuster(),
            Some(Checkpoint::Coffee) => game.debug_prepare_coffee(),
            Some(Checkpoint::TangleKelp) => game.debug_prepare_tangle_kelp(),
            Some(Checkpoint::Spikeweed) => game.debug_prepare_spikeweed(),
            Some(Checkpoint::Digger) => game.debug_prepare_digger(),
            Some(Checkpoint::Magnet) => game.debug_prepare_magnet(),
            Some(Checkpoint::Zamboni) => game.debug_prepare_zamboni(),
            Some(Checkpoint::PogoBlock) => game.debug_prepare_pogo_block(),
            Some(Checkpoint::UmbrellaDeflect) => game.debug_prepare_umbrella_deflect(),
            _ => {}
        }
        Self {
            renderer: None,
            assets,
            resources,
            audio,
            game,
            pending_input,
            last_update: None,
            simulation_accumulator: Duration::ZERO,
            cursor_position: None,
            tutorial_page: 0,
            seed_chooser_selection: [false; 2],
            selected_mode: ModeKind::MiniGame,
            selected_level: 0,
            fullscreen,
            startup_events,
        }
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
        for asset in self.assets.drain(..) {
            if let Err(error) = renderer.add_image(asset) {
                tracing::error!(%error, "GPU image upload failed");
                event_loop.exit();
                return;
            }
        }
        renderer.window().request_redraw();
        self.renderer = Some(renderer);
        self.last_update = Some(Instant::now());
        let startup_events = std::mem::take(&mut self.startup_events);
        self.play_audio(0, &startup_events);
    }

    fn title_start_hovered(&self) -> bool {
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
        .is_some_and(|(x, y)| title_start_contains(x, y))
    }

    fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: PhysicalKey) {
        let PhysicalKey::Code(key) = key else {
            return;
        };

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
                SceneKind::Title => self.start_scene(SceneKind::AdventureSelect),
                SceneKind::AdventureSelect => self.start_scene(SceneKind::AdventureTutorial),
                SceneKind::AdventureTutorial => self.advance_tutorial(),
                SceneKind::ModeSelect => self.start_selected_mode(),
                SceneKind::SeedChooser if self.game.state().mode == ModeKind::Survival => {
                    self.pending_input.push(InputAction::ConfirmSurvivalRepick)
                }
                SceneKind::SeedChooser
                    if self.seed_chooser_selection.iter().all(|selected| *selected) =>
                {
                    self.start_scene(SceneKind::Day)
                }
                SceneKind::Garden => self.pending_input.push(InputAction::GardenLeave),
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
                self.seed_chooser_selection[0] = !self.seed_chooser_selection[0];
            }
            KeyCode::Digit2 if self.game.state().scene == SceneKind::SeedChooser => {
                self.seed_chooser_selection[1] = !self.seed_chooser_selection[1];
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
                self.pending_input
                    .push(InputAction::ChallengeMatch { length: 3 });
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
                self.pending_input
                    .push(InputAction::ChallengeWhack { row: 2, column: 2 });
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

    fn start_scene(&mut self, scene: SceneKind) {
        self.game = new_scene_game(scene);
        self.tutorial_page = 0;
        self.pending_input.clear();
        self.simulation_accumulator = Duration::ZERO;
        self.last_update = Some(Instant::now());
        self.seed_chooser_selection = [false; 2];
        self.selected_mode = ModeKind::MiniGame;
        self.selected_level = 0;
    }

    fn start_mode_select(&mut self, mode: ModeKind) {
        self.game = Game::new(0, SceneKind::ModeSelect);
        self.pending_input.clear();
        self.simulation_accumulator = Duration::ZERO;
        self.last_update = Some(Instant::now());
        self.selected_mode = mode;
        self.selected_level = 0;
    }

    fn start_selected_mode(&mut self) {
        if mode_level_name(self.selected_mode, self.selected_level).is_none() {
            return;
        }
        self.game = Game::new_mode(0, self.selected_mode, self.selected_level);
        self.pending_input.clear();
        self.simulation_accumulator = Duration::ZERO;
        self.last_update = Some(Instant::now());
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
            self.start_scene(SceneKind::SeedChooser);
        }
    }

    fn handle_mouse_click(&mut self, button: MouseButton) {
        let scene = self.game.state().scene;
        if scene != SceneKind::Title
            && scene != SceneKind::AdventureSelect
            && scene != SceneKind::AdventureTutorial
            && scene != SceneKind::SeedChooser
            && scene != SceneKind::ModeSelect
            && scene != SceneKind::Garden
            && !is_board_scene(scene)
        {
            return;
        }
        if scene == SceneKind::Title {
            self.start_scene(SceneKind::AdventureSelect);
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
            if ((189.5..239.5).contains(&x) && (171.5..241.5).contains(&y))
                || ((288.0..338.0).contains(&x) && (445.0..515.0).contains(&y))
            {
                self.seed_chooser_selection[0] = !self.seed_chooser_selection[0];
                return;
            }
            if ((242.5..292.5).contains(&x) && (171.5..241.5).contains(&y))
                || ((341.0..391.0).contains(&x) && (445.0..515.0).contains(&y))
            {
                self.seed_chooser_selection[1] = !self.seed_chooser_selection[1];
                return;
            }
            if (322.0..478.0).contains(&x)
                && (535.0..577.0).contains(&y)
                && self.seed_chooser_selection.iter().all(|selected| *selected)
            {
                self.start_scene(SceneKind::Day);
            }
            return;
        }
        if scene == SceneKind::Garden {
            if button == MouseButton::Left {
                if self.game.state().garden_service == Some(GardenServiceKind::TreeOfWisdom) {
                    self.pending_input.push(InputAction::GardenFeedTree);
                } else {
                    self.pending_input
                        .push(InputAction::GardenWater { plant: 0 });
                }
            } else if button == MouseButton::Right {
                self.pending_input
                    .push(InputAction::GardenFertilize { plant: 0 });
            }
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
        match self.game.state().challenge.kind {
            neopvz_core::ChallengeKind::Zombiquarium if button == MouseButton::Left => {
                self.pending_input.push(InputAction::ChallengeFeed {
                    x: x as u16,
                    y: y as u16,
                });
                return;
            }
            neopvz_core::ChallengeKind::WhackAZombie if button == MouseButton::Left => {
                self.pending_input
                    .push(InputAction::ChallengeWhack { row, column });
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
            let input = InputFrame {
                actions: std::mem::take(&mut self.pending_input),
            };
            let tick = self.game.state().tick;
            let events = self.game.advance(input);
            self.play_audio(tick, &events);
            self.simulation_accumulator -= SIMULATION_STEP;
        }
    }

    fn play_audio(&mut self, tick: u64, events: &[GameEvent]) {
        for event in events {
            for (kind, path) in [audio_for_event(event), audio_companion_for_event(event)]
                .into_iter()
                .flatten()
            {
                let Some(bytes) = self.resources.read(path).ok() else {
                    tracing::debug!(path, "audio resource is unavailable");
                    continue;
                };
                tracing::debug!(tick, ?kind, ?event, path, "audio event queued");
                if let Some(audio) = &mut self.audio {
                    match audio.play_bytes(kind, path, bytes) {
                        Ok(()) => tracing::debug!(tick, ?kind, path, "audio playback started"),
                        Err(error) => tracing::warn!(%error, path, "audio playback failed"),
                    }
                }
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

    fn render_frame(&self) -> RenderFrame {
        let mut frame = RenderFrame::default();
        match self.game.state().scene {
            SceneKind::Title => {
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
                    y: 15.0,
                    z: 1,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: TITLE_LOAD_BAR_DIRT_IMAGE_ID,
                    x: TITLE_LOAD_BAR_X,
                    y: TITLE_LOAD_BAR_Y,
                    z: 2,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: TITLE_LOAD_BAR_GRASS_IMAGE_ID,
                    x: TITLE_LOAD_BAR_X,
                    y: TITLE_START_BUTTON_Y - 17.0,
                    z: 3,
                    scale: 1.0,
                    alpha: 1.0,
                });
                // The source holds these load-bar reanimations on their final frame once ready.
                push_title_load_bar_reanimations(&mut frame);
                frame.sprites.push(SpriteCommand {
                    resource_id: TITLE_START_PROMPT_SHADOW_IMAGE_ID,
                    x: 341.0,
                    y: 544.0,
                    z: 5,
                    scale: 1.0,
                    alpha: 1.0,
                });
                frame.sprites.push(SpriteCommand {
                    resource_id: if self.title_start_hovered() {
                        TITLE_START_PROMPT_HOVER_IMAGE_ID
                    } else {
                        TITLE_START_PROMPT_IMAGE_ID
                    },
                    x: 340.0,
                    y: 543.0,
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
                for (resource_id, x, y) in [
                    (SELECTOR_ADVENTURE_IMAGE_ID, 405.0, 79.0),
                    (SELECTOR_SURVIVAL_IMAGE_ID, 406.0, 173.0),
                    (SELECTOR_CHALLENGES_IMAGE_ID, 410.0, 257.0),
                    (SELECTOR_VASEBREAKER_IMAGE_ID, 413.0, 328.0),
                ] {
                    frame.sprites.push(SpriteCommand {
                        resource_id,
                        x,
                        y,
                        z: 5,
                        scale: 1.0,
                        alpha: 1.0,
                    });
                }
            }
            SceneKind::AdventureTutorial => self.render_tutorial(&mut frame),
            SceneKind::ModeSelect => self.render_mode_select(&mut frame),
            SceneKind::SeedChooser => {
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
                for column in 2..8 {
                    frame.sprites.push(SpriteCommand {
                        resource_id: SEED_PACKET_SILHOUETTE_IMAGE_ID,
                        x: 189.5 + column as f32 * 53.0,
                        y: 171.5,
                        z: 2,
                        scale: 1.0,
                        alpha: 1.0,
                    });
                }
                let packet_positions = [(189.5, 171.5), (242.5, 171.5)];
                let bank_positions = [(288.0, 445.0), (341.0, 445.0)];
                for (
                    slot,
                    (packet_x, packet_y),
                    (bank_x, bank_y),
                    (resource_id, icon_x, icon_y, scale),
                ) in [
                    (
                        0,
                        packet_positions[0],
                        bank_positions[0],
                        (SEED_PEASHOOTER_IMAGE_ID, 7.0, 9.5, 0.5),
                    ),
                    (
                        1,
                        packet_positions[1],
                        bank_positions[1],
                        (SEED_SUNFLOWER_IMAGE_ID, 8.0, 12.0, 0.6),
                    ),
                ] {
                    let (x, y) = if self.seed_chooser_selection[slot] {
                        (bank_x, bank_y)
                    } else {
                        (packet_x, packet_y)
                    };
                    frame.sprites.push(SpriteCommand {
                        resource_id: SEED_PACKET_NORMAL_IMAGE_ID,
                        x,
                        y,
                        z: 3,
                        scale: 1.0,
                        alpha: 1.0,
                    });
                    frame.sprites.push(SpriteCommand {
                        resource_id,
                        x: x + icon_x,
                        y: y + icon_y,
                        z: 4,
                        scale,
                        alpha: 1.0,
                    });
                }
                frame.sprites.push(SpriteCommand {
                    resource_id: SEED_CHOOSER_BUTTON_IMAGE_ID,
                    x: 322.0,
                    y: 535.0,
                    z: 3,
                    scale: 1.0,
                    alpha: if self.seed_chooser_selection.iter().all(|selected| *selected) {
                        1.0
                    } else {
                        0.55
                    },
                });
            }
            SceneKind::Garden => {
                frame.sprites.push(SpriteCommand {
                    resource_id: SCREEN_PIXEL_IMAGE_ID,
                    x: 0.0,
                    y: 0.0,
                    z: 0,
                    scale: 800.0,
                    alpha: 1.0,
                });
                for (index, plant) in self.game.state().garden.plants.iter().enumerate() {
                    frame.sprites.push(SpriteCommand {
                        resource_id: UI_PIXEL_IMAGE_ID,
                        x: 250.0 + index as f32 * 120.0,
                        y: 300.0,
                        z: 2,
                        scale: 64.0 + (plant.age_ticks.min(100) as f32 * 0.2),
                        alpha: if plant.watered { 1.0 } else { 0.75 },
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
            | SceneKind::Boss) => {
                frame.sprites.push(SpriteCommand {
                    resource_id: board_background_id(scene),
                    x: 0.0,
                    y: 0.0,
                    z: 0,
                    scale: 1.0,
                    alpha: 1.0,
                });
                for vase in &self.game.state().board.vases {
                    frame.sprites.push(SpriteCommand {
                        resource_id: UI_PIXEL_IMAGE_ID,
                        x: 80.0 + f32::from(vase.column) * 80.0 + 20.0,
                        y: 120.0 + f32::from(vase.row) * 90.0 + 20.0,
                        z: 9,
                        scale: 40.0,
                        alpha: 0.9,
                    });
                }
                for brain in &self.game.state().board.brains {
                    if !brain.squished {
                        frame.sprites.push(SpriteCommand {
                            resource_id: UI_PIXEL_IMAGE_ID,
                            x: 20.0,
                            y: 120.0 + f32::from(brain.row) * 90.0 + 20.0,
                            z: 9,
                            scale: 40.0,
                            alpha: 0.8,
                        });
                    }
                }
                for plant in &self.game.state().board.plants {
                    frame.sprites.push(SpriteCommand {
                        resource_id: UI_PIXEL_IMAGE_ID,
                        x: 80.0 + f32::from(plant.column) * 80.0,
                        y: 120.0 + f32::from(plant.row) * 90.0,
                        z: 10,
                        scale: 36.0,
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
            _ => {}
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

fn new_scene_game(scene: SceneKind) -> Game {
    if scene == SceneKind::Day {
        Game::new_mode(0, ModeKind::Adventure, 1)
    } else {
        Game::new(0, scene)
    }
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

fn audio_for_event(event: &GameEvent) -> Option<(AudioKind, &'static str)> {
    match event {
        GameEvent::SeedSelected { .. } => Some((AudioKind::Effect, "sounds/tap.ogg")),
        GameEvent::InputRejected { .. } => Some((AudioKind::Effect, "sounds/buzzer.ogg")),
        GameEvent::PlantPlaced { .. } => Some((AudioKind::Effect, "sounds/plant.ogg")),
        GameEvent::PlantShoveled { .. } => Some((AudioKind::Effect, "sounds/plant2.ogg")),
        GameEvent::SunCollected { .. } => Some((AudioKind::Effect, "sounds/points.ogg")),
        GameEvent::CoinCollected { .. } => Some((AudioKind::Effect, "sounds/coin.ogg")),
        GameEvent::GardenWatered { .. } => Some((AudioKind::Effect, "sounds/watering.ogg")),
        GameEvent::GardenFertilized { .. } => Some((AudioKind::Effect, "sounds/fertilizer.ogg")),
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
        GameEvent::PlantSpecialTriggered {
            plant_type: neopvz_core::PlantType::Other(21),
            ..
        } => Some((AudioKind::Effect, "sounds/throw.ogg")),
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
            plant_type: neopvz_core::PlantType::Other(6),
            ..
        } => Some((AudioKind::Effect, "sounds/bigchomp.ogg")),
        GameEvent::BloverTriggered { .. } => Some((AudioKind::Effect, "sounds/blover.ogg")),
        GameEvent::ZombieHypnotized { .. } => {
            Some((AudioKind::Effect, "sounds/mindcontrolled.ogg"))
        }
        GameEvent::JackboxExploded { .. } => Some((AudioKind::Effect, "sounds/explosion.ogg")),
        GameEvent::BrainFinished { .. } => Some((AudioKind::Effect, "sounds/gulp.ogg")),
        GameEvent::ZombieChilled { .. } => Some((AudioKind::Effect, "sounds/frozen.ogg")),
        GameEvent::ZombieButtered { .. } => Some((AudioKind::Effect, "sounds/butter.ogg")),
        GameEvent::VaseBroken { .. } => Some((AudioKind::Effect, "sounds/vase_breaking.ogg")),
        GameEvent::RakeTriggered { .. } => Some((AudioKind::Effect, "sounds/swing.ogg")),
        GameEvent::JumpBlocked { .. } => Some((AudioKind::Effect, "sounds/bonk.ogg")),
        GameEvent::UmbrellaDeflected { .. } => Some((AudioKind::Effect, "sounds/boing.ogg")),
        GameEvent::CobCannonFired { .. } => Some((AudioKind::Effect, "sounds/coblaunch.ogg")),
        GameEvent::PortalOpened { .. } => Some((AudioKind::Effect, "sounds/portal.ogg")),
        GameEvent::ProjectileHit { .. } | GameEvent::ProjectileSplashHit { .. } => {
            Some((AudioKind::Effect, "sounds/splat.ogg"))
        }
        GameEvent::ZombieDied { .. } => Some((AudioKind::Effect, "sounds/splat2.ogg")),
        GameEvent::MowerTriggered { .. } => Some((AudioKind::Effect, "sounds/lawnmower.ogg")),
        GameEvent::Paused => Some((AudioKind::Effect, "sounds/pause.ogg")),
        GameEvent::GameLost { .. } => Some((AudioKind::Music, "sounds/losemusic.ogg")),
        GameEvent::GameWon => Some((AudioKind::Music, "sounds/winmusic.ogg")),
        _ => None,
    }
}

fn audio_companion_for_event(event: &GameEvent) -> Option<(AudioKind, &'static str)> {
    match event {
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
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => self.handle_mouse_click(button),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_terminal_and_player_events_to_audio_resources() {
        assert_eq!(
            audio_for_event(&GameEvent::PlantShoveled { entity: 1 }),
            Some((AudioKind::Effect, "sounds/plant2.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::SeedSelected {
                slot: 1,
                plant_type: neopvz_core::PlantType::Peashooter,
            }),
            Some((AudioKind::Effect, "sounds/tap.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::InputRejected {
                action: InputAction::Pause,
                reason: neopvz_core::InputRejectReason::OutsideBoard,
            }),
            Some((AudioKind::Effect, "sounds/buzzer.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::GameWon),
            Some((AudioKind::Music, "sounds/winmusic.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::GameLost { zombie: 1 }),
            Some((AudioKind::Music, "sounds/losemusic.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::Paused),
            Some((AudioKind::Effect, "sounds/pause.ogg"))
        );
        assert_eq!(
            audio_for_event(&GameEvent::MowerTriggered { row: 2 }),
            Some((AudioKind::Effect, "sounds/lawnmower.ogg"))
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
            audio_for_event(&GameEvent::CoinCollected {
                entity: 2,
                coin_type: neopvz_core::CoinType::Silver,
                value: 1,
                coin_total: 1,
            }),
            Some((AudioKind::Effect, "sounds/coin.ogg"))
        );
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
            audio_for_event(&GameEvent::ZombieButtered { entity: 1 }),
            Some((AudioKind::Effect, "sounds/butter.ogg"))
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
            audio_for_event(&GameEvent::CobCannonFired {
                entity: 1,
                target_row: 2,
                target_column: 4,
            }),
            Some((AudioKind::Effect, "sounds/coblaunch.ogg"))
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
    fn maps_explosive_plant_companions_to_juicy() {
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
    fn title_start_hitbox_matches_the_source_load_bar() {
        assert!(title_start_contains(400.0, 550.0));
        assert!(!title_start_contains(242.0, 550.0));
        assert!(!title_start_contains(400.0, 579.0));
    }

    #[test]
    fn day_scene_starts_the_first_adventure_level() {
        let game = new_scene_game(SceneKind::Day);
        assert_eq!(game.state().mode, ModeKind::Adventure);
        assert_eq!(game.state().level, 1);
        assert_eq!(game.state().scene, SceneKind::Day);
        assert_eq!(game.state().board.wave.total, 4);
    }
}
