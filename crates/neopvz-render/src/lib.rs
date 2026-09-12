use std::{collections::HashMap, sync::Arc};

use bytemuck::{Pod, Zeroable};
use neopvz_core::{LOGICAL_HEIGHT, LOGICAL_WIDTH};
use thiserror::Error;
use wgpu::util::DeviceExt;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    window::Window,
};

pub mod particles;

pub use particles::{
    ParticleCatalog, ParticleHolder, ParticleRenderParams, ParticleSystem, RenderedParticle,
};

pub const TITLE_IMAGE_ID: u32 = 1;
pub const SEED_CHOOSER_IMAGE_ID: u32 = 2;
pub const DAY_BACKGROUND_IMAGE_ID: u32 = 3;
pub const UI_PIXEL_IMAGE_ID: u32 = 4;
pub const SCREEN_PIXEL_IMAGE_ID: u32 = 5;
pub const TITLE_LOGO_IMAGE_ID: u32 = 6;
pub const SELECTOR_BASE_IMAGE_ID: u32 = 7;
pub const SELECTOR_LEFT_IMAGE_ID: u32 = 8;
pub const SELECTOR_CENTER_IMAGE_ID: u32 = 9;
pub const SELECTOR_RIGHT_IMAGE_ID: u32 = 10;
pub const SELECTOR_ADVENTURE_IMAGE_ID: u32 = 11;
pub const SELECTOR_CHALLENGES_IMAGE_ID: u32 = 12;
pub const SELECTOR_SURVIVAL_IMAGE_ID: u32 = 13;
pub const SELECTOR_VASEBREAKER_IMAGE_ID: u32 = 14;
pub const SELECTOR_WOODSIGN1_IMAGE_ID: u32 = 15;
pub const SELECTOR_WOODSIGN2_IMAGE_ID: u32 = 16;
pub const SELECTOR_WOODSIGN3_IMAGE_ID: u32 = 17;
pub const SELECTOR_LEAVES_IMAGE_ID: u32 = 18;
pub const SELECTOR_ZEN_GARDEN_IMAGE_ID: u32 = 19;
pub const SELECTOR_ALMANAC_IMAGE_ID: u32 = 20;
pub const SELECTOR_STORE_IMAGE_ID: u32 = 21;
pub const SELECTOR_OPTIONS_IMAGE_ID: u32 = 22;
pub const SELECTOR_HELP_IMAGE_ID: u32 = 23;
pub const SELECTOR_QUIT_IMAGE_ID: u32 = 24;
pub const SELECTOR_TROPHY_IMAGE_ID: u32 = 25;
pub const TUTORIAL_BUBBLE_IMAGE_ID: u32 = 26;
pub const CRAZY_DAVE_BODY_IMAGE_ID: u32 = 27;
pub const CRAZY_DAVE_HEAD_IMAGE_ID: u32 = 28;
pub const CRAZY_DAVE_BEARD_IMAGE_ID: u32 = 29;
pub const CRAZY_DAVE_POT_IMAGE_ID: u32 = 30;
pub const CRAZY_DAVE_EYE_IMAGE_ID: u32 = 31;
pub const CRAZY_DAVE_EYEBROW_IMAGE_ID: u32 = 32;
pub const CRAZY_DAVE_MOUTH_IMAGE_ID: u32 = 33;
pub const CRAZY_DAVE_OUTER_ARM_IMAGE_ID: u32 = 34;
pub const CRAZY_DAVE_OUTER_HAND_IMAGE_ID: u32 = 35;
pub const CRAZY_DAVE_INNER_ARM_IMAGE_ID: u32 = 36;
pub const CRAZY_DAVE_INNER_HAND_IMAGE_ID: u32 = 37;
pub const CRAZY_DAVE_INNER_FINGER1_IMAGE_ID: u32 = 38;
pub const CRAZY_DAVE_INNER_FINGER2_IMAGE_ID: u32 = 39;
pub const CRAZY_DAVE_INNER_FINGER3_IMAGE_ID: u32 = 40;
pub const CRAZY_DAVE_INNER_FINGER4_IMAGE_ID: u32 = 41;
pub const CRAZY_DAVE_OUTER_FINGER1_IMAGE_ID: u32 = 42;
pub const CRAZY_DAVE_OUTER_FINGER2_IMAGE_ID: u32 = 43;
pub const CRAZY_DAVE_OUTER_FINGER3_IMAGE_ID: u32 = 44;
pub const CRAZY_DAVE_OUTER_FINGER4_IMAGE_ID: u32 = 45;
pub const TUTORIAL_TEXT1_IMAGE_ID: u32 = 46;
pub const TUTORIAL_TEXT2_IMAGE_ID: u32 = 47;
pub const TUTORIAL_CONTINUE_IMAGE_ID: u32 = 48;
pub const SEED_PACKET_NORMAL_IMAGE_ID: u32 = 49;
pub const SEED_PEASHOOTER_IMAGE_ID: u32 = 50;
pub const SEED_SUNFLOWER_IMAGE_ID: u32 = 51;
pub const SEED_CHOOSER_BUTTON_IMAGE_ID: u32 = 52;
pub const SEED_CHOOSER_TITLE_IMAGE_ID: u32 = 53;
pub const SEED_PACKET_SILHOUETTE_IMAGE_ID: u32 = 54;
pub const SEED_PACKET_PLANT_BASE_IMAGE_ID: u32 = 70;
pub const MODE_SELECT_BACKGROUND_IMAGE_ID: u32 = 55;
pub const MODE_SELECT_WINDOW_IMAGE_ID: u32 = 56;
pub const MODE_SELECT_BLANK_IMAGE_ID: u32 = 57;
pub const TITLE_LOAD_BAR_DIRT_IMAGE_ID: u32 = 58;
pub const TITLE_LOAD_BAR_GRASS_IMAGE_ID: u32 = 59;
pub const TITLE_START_PROMPT_SHADOW_IMAGE_ID: u32 = 60;
pub const TITLE_START_PROMPT_IMAGE_ID: u32 = 61;
pub const TITLE_START_PROMPT_HOVER_IMAGE_ID: u32 = 62;
pub const TITLE_LOAD_BAR_ROCK1_IMAGE_ID: u32 = 63;
pub const TITLE_LOAD_BAR_ROCK3_IMAGE_ID: u32 = 64;
pub const TITLE_LOAD_BAR_SPROUT_BODY_IMAGE_ID: u32 = 65;
pub const TITLE_LOAD_BAR_SPROUT_PETAL_IMAGE_ID: u32 = 66;
pub const TITLE_LOAD_BAR_ZOMBIE_HEAD_IMAGE_ID: u32 = 67;
pub const TITLE_LOAD_BAR_ZOMBIE_HAIR_IMAGE_ID: u32 = 68;
pub const TITLE_LOAD_BAR_ZOMBIE_JAW_IMAGE_ID: u32 = 69;
pub const TITLE_SOD_ROLL_CAP_IMAGE_ID: u32 = 83;
pub const TITLE_LOADING_PROMPT_IMAGE_ID: u32 = 84;
pub const TITLE_LOADING_PROMPT_SHADOW_IMAGE_ID: u32 = 85;
pub const TITLE_POPCAP_LOGO_IMAGE_ID: u32 = 86;
pub const TITLE_LOADING_PROMPT_HOVER_IMAGE_ID: u32 = 87;
pub const SELECTOR_LEVEL_NUMBER_BASE_IMAGE_ID: u32 = 88;
pub const CHALLENGE_THUMBNAIL_BASE_IMAGE_ID: u32 = 100;
pub const SURVIVAL_THUMBNAIL_BASE_IMAGE_ID: u32 = 130;
pub const NIGHT_BACKGROUND_IMAGE_ID: u32 = 160;
pub const POOL_BACKGROUND_IMAGE_ID: u32 = 161;
pub const FOG_BACKGROUND_IMAGE_ID: u32 = 162;
pub const ROOF_BACKGROUND_IMAGE_ID: u32 = 163;
pub const BOSS_BACKGROUND_IMAGE_ID: u32 = 164;
pub const BOARD_ZOMBIE_BODY_IMAGE_ID: u32 = 165;
pub const BOARD_ZOMBIE_CONE_IMAGE_ID: u32 = 1040;
pub const BOARD_ZOMBIE_BUCKET_IMAGE_ID: u32 = 1041;
pub const BOARD_ZOMBIE_FLAG_POLE_IMAGE_ID: u32 = 1042;
pub const BOARD_ZOMBIE_FLAG_IMAGE_ID: u32 = 1043;
pub const BOARD_ZOMBIE_FLAG_HAND_IMAGE_ID: u32 = 1044;
pub const BOARD_ZOMBIE_SCREEN_DOOR_IMAGE_ID: u32 = 1045;
pub const BOARD_ZOMBIE_FOOTBALL_HELMET_IMAGE_ID: u32 = 1046;
pub const BOARD_ZOMBIE_FOOTBALL_UPPERBODY_IMAGE_ID: u32 = 1047;
pub const BOARD_ZOMBIE_NEWSPAPER_IMAGE_ID: u32 = 1048;
pub const BOARD_ZOMBIE_FOOTBALL_HEAD_IMAGE_ID: u32 = 1049;
pub const BOARD_ZOMBIE_NEWSPAPER_HEAD_IMAGE_ID: u32 = 1050;
pub const BOARD_PROJECTILE_PEA_IMAGE_ID: u32 = 166;
pub const BOARD_PROJECTILE_SNOW_PEA_IMAGE_ID: u32 = 167;
pub const BOARD_PROJECTILE_CABBAGE_IMAGE_ID: u32 = 1051;
pub const BOARD_PROJECTILE_MELON_IMAGE_ID: u32 = 1052;
pub const BOARD_PROJECTILE_WINTER_MELON_IMAGE_ID: u32 = 1053;
pub const BOARD_PROJECTILE_KERNEL_IMAGE_ID: u32 = 1054;
pub const BOARD_PROJECTILE_BUTTER_IMAGE_ID: u32 = 1055;
pub const BOARD_PROJECTILE_SPIKE_IMAGE_ID: u32 = 1056;
pub const BOARD_PROJECTILE_STAR_IMAGE_ID: u32 = 1057;
pub const BOARD_PROJECTILE_FIREBALL_IMAGE_ID: u32 = 1058;
pub const BOARD_PROJECTILE_COB_IMAGE_ID: u32 = 1059;
pub const BOARD_PROJECTILE_BASKETBALL_IMAGE_ID: u32 = 1060;
pub const BOARD_PROJECTILE_PUFF_IMAGE_ID: u32 = 1061;
pub const BOARD_PROJECTILE_SHADOW_DAY_IMAGE_ID: u32 = 1062;
pub const BOARD_PROJECTILE_SHADOW_NIGHT_IMAGE_ID: u32 = 1063;
pub const BOARD_SUN_IMAGE_ID: u32 = 168;
pub const BOARD_COIN_SILVER_IMAGE_ID: u32 = 169;
pub const BOARD_COIN_GOLD_IMAGE_ID: u32 = 170;
pub const BOARD_DIAMOND_IMAGE_ID: u32 = 171;
pub const BOARD_SNOWPEA_IMAGE_ID: u32 = 172;
pub const BOARD_PUFFSHROOM_IMAGE_ID: u32 = 173;
pub const BOARD_FUMESHROOM_IMAGE_ID: u32 = 174;
pub const BOARD_STARFRUIT_IMAGE_ID: u32 = 175;
pub const BOARD_WALLNUT_IMAGE_ID: u32 = 181;
pub const BOARD_MAGNETSHROOM_IMAGE_ID: u32 = 182;
pub const BOARD_BEGHOULED_TWIST_OVERLAY_IMAGE_ID: u32 = 183;
pub const BOARD_GRAVE_IMAGE_ID: u32 = 176;
pub const BOARD_CRATER_IMAGE_ID: u32 = 177;
pub const BOARD_BRAIN_IMAGE_ID: u32 = 178;
pub const BOARD_VASE_TOP_IMAGE_ID: u32 = 179;
pub const BOARD_VASE_BOTTOM_IMAGE_ID: u32 = 180;
pub const BOARD_PRESENT_IMAGE_ID: u32 = 1000;
pub const BOARD_MONEYBAG_IMAGE_ID: u32 = 1001;
pub const BOARD_CHOCOLATE_IMAGE_ID: u32 = 1002;
pub const BOARD_VASE_IMAGE_ID: u32 = 1003;
pub const BOARD_NOTE_IMAGE_ID: u32 = 1004;
pub const BOARD_SILVER_SUNFLOWER_IMAGE_ID: u32 = 1005;
pub const BOARD_GOLD_SUNFLOWER_IMAGE_ID: u32 = 1006;
pub const BOARD_SEED_BANK_IMAGE_ID: u32 = 1007;
pub const BOARD_CONVEYOR_BELT_BACKDROP_IMAGE_ID: u32 = 1008;
pub const BOARD_CONVEYOR_BELT_BASE_IMAGE_ID: u32 = 1009;
pub const BOARD_SHOVEL_BANK_IMAGE_ID: u32 = 1015;
pub const BOARD_SUN_BANK_IMAGE_ID: u32 = 1016;
pub const BOARD_SUN_COUNT_IMAGE_ID: u32 = 1017;
pub const BOARD_SEED_COST_BASE_IMAGE_ID: u32 = 1020;
pub const BOARD_SEED_BANK_EXTENSION_BASE_IMAGE_ID: u32 = 1030;
pub const BOARD_PROGRESS_METER_IMAGE_ID: u32 = 1034;
pub const BOARD_PROGRESS_FILL_IMAGE_ID: u32 = 1035;
pub const BOARD_PROGRESS_HEAD_IMAGE_ID: u32 = 1036;
pub const BOARD_PROGRESS_POLE_IMAGE_ID: u32 = 1037;
pub const BOARD_PROGRESS_FLAG_IMAGE_ID: u32 = 1038;
pub const BOARD_PROGRESS_LEVEL_IMAGE_ID: u32 = 1039;
pub const GARDEN_BACKGROUND_IMAGE_ID: u32 = 184;
pub const GARDEN_MUSHROOM_BACKGROUND_IMAGE_ID: u32 = 185;
pub const GARDEN_WATERING_CAN_IMAGE_ID: u32 = 186;
pub const GARDEN_FERTILIZER_IMAGE_ID: u32 = 187;
pub const GARDEN_BUG_SPRAY_IMAGE_ID: u32 = 188;
pub const GARDEN_PHONOGRAPH_IMAGE_ID: u32 = 189;
pub const GARDEN_NEED_BUBBLE_IMAGE_ID: u32 = 190;
pub const GARDEN_WATERDROP_IMAGE_ID: u32 = 191;
pub const GARDEN_NEED_FERTILIZER_IMAGE_ID: u32 = 192;
pub const GARDEN_NEED_BUG_SPRAY_IMAGE_ID: u32 = 193;
pub const GARDEN_NEED_PHONOGRAPH_IMAGE_ID: u32 = 194;
pub const STORE_BACKGROUND_IMAGE_ID: u32 = 195;
pub const STORE_SIGN_IMAGE_ID: u32 = 196;
pub const STORE_CAR_IMAGE_ID: u32 = 197;
pub const STORE_PRICE_TAG_IMAGE_ID: u32 = 198;
pub const STORE_MAIN_MENU_BUTTON_IMAGE_ID: u32 = 199;
pub const STORE_PACKET_UPGRADE_IMAGE_ID: u32 = 200;
pub const STORE_STINKY_IMAGE_ID: u32 = 201;
pub const STORE_ITEM_NAME_BASE_IMAGE_ID: u32 = 210;
pub const STORE_ITEM_PRICE_BASE_IMAGE_ID: u32 = 220;
pub const STORE_BACK_TEXT_IMAGE_ID: u32 = 230;
pub const PAUSE_DIALOG_TOP_LEFT_IMAGE_ID: u32 = 240;
pub const PAUSE_DIALOG_TOP_MIDDLE_IMAGE_ID: u32 = 241;
pub const PAUSE_DIALOG_TOP_RIGHT_IMAGE_ID: u32 = 242;
pub const PAUSE_DIALOG_HEADER_IMAGE_ID: u32 = 243;
pub const PAUSE_DIALOG_CENTER_LEFT_IMAGE_ID: u32 = 244;
pub const PAUSE_DIALOG_CENTER_MIDDLE_IMAGE_ID: u32 = 245;
pub const PAUSE_DIALOG_CENTER_RIGHT_IMAGE_ID: u32 = 246;
pub const PAUSE_DIALOG_BOTTOM_LEFT_IMAGE_ID: u32 = 247;
pub const PAUSE_DIALOG_BOTTOM_MIDDLE_IMAGE_ID: u32 = 248;
pub const PAUSE_DIALOG_BOTTOM_RIGHT_IMAGE_ID: u32 = 249;
pub const PAUSE_RESUME_BUTTON_IMAGE_ID: u32 = 250;
pub const PAUSE_HEADER_TEXT_IMAGE_ID: u32 = 260;
pub const PAUSE_BODY_TEXT_IMAGE_ID: u32 = 261;
pub const PAUSE_RESUME_TEXT_IMAGE_ID: u32 = 262;
pub const OPTIONS_BACKGROUND_IMAGE_ID: u32 = 270;
pub const OPTIONS_CHECKBOX_OFF_IMAGE_ID: u32 = 271;
pub const OPTIONS_CHECKBOX_ON_IMAGE_ID: u32 = 272;
pub const OPTIONS_SLIDER_SLOT_IMAGE_ID: u32 = 273;
pub const OPTIONS_SLIDER_KNOB_IMAGE_ID: u32 = 274;
pub const OPTIONS_MUSIC_LABEL_IMAGE_ID: u32 = 280;
pub const OPTIONS_SFX_LABEL_IMAGE_ID: u32 = 281;
pub const OPTIONS_ACCELERATION_LABEL_IMAGE_ID: u32 = 282;
pub const OPTIONS_FULLSCREEN_LABEL_IMAGE_ID: u32 = 283;
pub const OPTIONS_BACK_TEXT_IMAGE_ID: u32 = 284;
pub const HELP_ZOMBIE_NOTE_IMAGE_ID: u32 = 350;
pub const HELP_CONTENT_IMAGE_ID: u32 = 351;
pub const HELP_MENU_BUTTON_IMAGE_ID: u32 = 352;
pub const HELP_MAIN_MENU_TEXT_IMAGE_ID: u32 = 353;
pub const ALMANAC_INDEX_BACKGROUND_IMAGE_ID: u32 = 360;
pub const ALMANAC_PLANT_BACKGROUND_IMAGE_ID: u32 = 361;
pub const ALMANAC_ZOMBIE_BACKGROUND_IMAGE_ID: u32 = 362;
pub const ALMANAC_CLOSE_BUTTON_IMAGE_ID: u32 = 363;
pub const ALMANAC_INDEX_BUTTON_IMAGE_ID: u32 = 364;
pub const ALMANAC_NAV_BUTTON_IMAGE_ID: u32 = 365;
pub const ALMANAC_TITLE_TEXT_IMAGE_ID: u32 = 370;
pub const ALMANAC_PLANTS_TEXT_IMAGE_ID: u32 = 371;
pub const ALMANAC_ZOMBIES_TEXT_IMAGE_ID: u32 = 372;
pub const ALMANAC_CLOSE_TEXT_IMAGE_ID: u32 = 373;
pub const ALMANAC_INDEX_TEXT_IMAGE_ID: u32 = 374;
pub const ALMANAC_PLANT_CARD_IMAGE_ID: u32 = 415;
pub const ALMANAC_ZOMBIE_CARD_IMAGE_ID: u32 = 416;
pub const ALMANAC_ZOMBIE_WINDOW_IMAGE_ID: u32 = 417;
pub const ALMANAC_ZOMBIE_WINDOW2_IMAGE_ID: u32 = 418;
pub const ALMANAC_ZOMBIE_BLANK_IMAGE_ID: u32 = 419;
pub const ALMANAC_IMITATER_IMAGE_ID: u32 = 440;
pub const ALMANAC_PLANT_NAME_BASE_IMAGE_ID: u32 = 450;
pub const ALMANAC_PLANT_COST_BASE_IMAGE_ID: u32 = 500;
pub const ALMANAC_PLANT_RECHARGE_BASE_IMAGE_ID: u32 = 550;
pub const ALMANAC_ZOMBIE_NAME_BASE_IMAGE_ID: u32 = 600;
pub const ALMANAC_PLANT_DESCRIPTION_BASE_IMAGE_ID: u32 = 650;
pub const ALMANAC_ZOMBIE_DESCRIPTION_BASE_IMAGE_ID: u32 = 700;
pub const ALMANAC_ZOMBIE_SILHOUETTE_NAME_BASE_IMAGE_ID: u32 = 750;
pub const GAME_OVER_HEADER_TEXT_IMAGE_ID: u32 = 390;
pub const GAME_OVER_BODY_TEXT_IMAGE_ID: u32 = 391;
pub const GAME_OVER_TRY_AGAIN_TEXT_IMAGE_ID: u32 = 392;
pub const GAME_OVER_MAIN_MENU_TEXT_IMAGE_ID: u32 = 393;
pub const ZOMBIES_WON_IMAGE_ID: u32 = 394;
pub const AWARD_SCREEN_BACKGROUND_IMAGE_ID: u32 = 400;
pub const AWARD_TROPHY_IMAGE_ID: u32 = 401;
pub const AWARD_TITLE_TEXT_IMAGE_ID: u32 = 402;
pub const AWARD_BODY_TEXT_IMAGE_ID: u32 = 403;
pub const AWARD_CONTINUE_TEXT_IMAGE_ID: u32 = 404;
pub const AWARD_MAIN_MENU_TEXT_IMAGE_ID: u32 = 405;
pub const AWARD_SHOVEL_IMAGE_ID: u32 = 406;
pub const AWARD_ALMANAC_IMAGE_ID: u32 = 407;
pub const AWARD_CAR_KEYS_IMAGE_ID: u32 = 408;
pub const AWARD_TACO_IMAGE_ID: u32 = 409;
pub const AWARD_WATERING_CAN_IMAGE_ID: u32 = 410;
pub const AWARD_NOTE1_IMAGE_ID: u32 = 411;
pub const AWARD_NOTE2_IMAGE_ID: u32 = 412;
pub const AWARD_NOTE3_IMAGE_ID: u32 = 413;
pub const AWARD_NOTE4_IMAGE_ID: u32 = 414;
pub const AWARD_TITLE_TEXT_BASE_IMAGE_ID: u32 = 420;
pub const AWARD_BODY_TEXT_BASE_IMAGE_ID: u32 = 430;
pub const CREDITS_BIG_BRAIN_IMAGE_ID: u32 = 800;
pub const CREDITS_ZOMBIE_NOTE_IMAGE_ID: u32 = 801;
pub const CREDITS_PLAY_BUTTON_IMAGE_ID: u32 = 802;
pub const CREDITS_STAGE_IMAGE_ID: u32 = 803;
pub const CREDITS_WE_ARE_UNDEAD_IMAGE_ID: u32 = 804;
pub const CREDITS_MTV_IMAGE_ID: u32 = 805;
pub const CREDITS_TITLE_TEXT_IMAGE_ID: u32 = 810;
pub const CREDITS_LINE1_TEXT_IMAGE_ID: u32 = 811;
pub const CREDITS_LINE2_TEXT_IMAGE_ID: u32 = 812;
pub const CREDITS_LINE3_TEXT_IMAGE_ID: u32 = 813;
pub const CREDITS_LINE4_TEXT_IMAGE_ID: u32 = 814;
pub const CREDITS_LINE5_TEXT_IMAGE_ID: u32 = 815;
pub const CREDITS_LINE6_TEXT_IMAGE_ID: u32 = 816;
pub const CREDITS_REPLAY_TEXT_IMAGE_ID: u32 = 817;
pub const CREDITS_MAIN_MENU_TEXT_IMAGE_ID: u32 = 818;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LogicalViewport {
    pub width: u32,
    pub height: u32,
}

impl Default for LogicalViewport {
    fn default() -> Self {
        Self {
            width: LOGICAL_WIDTH,
            height: LOGICAL_HEIGHT,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ViewportRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn letterbox_rect(
    window_width: u32,
    window_height: u32,
    logical_viewport: LogicalViewport,
) -> ViewportRect {
    if window_width == 0
        || window_height == 0
        || logical_viewport.width == 0
        || logical_viewport.height == 0
    {
        return ViewportRect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
    }

    let scale = (window_width as f64 / logical_viewport.width as f64)
        .min(window_height as f64 / logical_viewport.height as f64);
    let width = (logical_viewport.width as f64 * scale).round() as u32;
    let height = (logical_viewport.height as f64 * scale).round() as u32;

    ViewportRect {
        x: (window_width - width) / 2,
        y: (window_height - height) / 2,
        width,
        height,
    }
}

pub fn logical_position(
    window_width: u32,
    window_height: u32,
    position: PhysicalPosition<f64>,
    logical_viewport: LogicalViewport,
) -> Option<(f32, f32)> {
    let rect = letterbox_rect(window_width, window_height, logical_viewport);
    if rect.width == 0
        || position.x < f64::from(rect.x)
        || position.y < f64::from(rect.y)
        || position.x >= f64::from(rect.x + rect.width)
        || position.y >= f64::from(rect.y + rect.height)
    {
        return None;
    }

    let scale = f64::from(rect.width) / f64::from(logical_viewport.width);
    Some((
        ((position.x - f64::from(rect.x)) / scale) as f32,
        ((position.y - f64::from(rect.y)) / scale) as f32,
    ))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageAsset {
    pub resource_id: u32,
    pub width: u32,
    pub height: u32,
    pub rgba8: Vec<u8>,
}

impl ImageAsset {
    pub fn new(
        resource_id: u32,
        width: u32,
        height: u32,
        rgba8: Vec<u8>,
    ) -> Result<Self, ImageAssetError> {
        let expected = usize::try_from(width)
            .ok()
            .and_then(|width| {
                usize::try_from(height)
                    .ok()
                    .and_then(|height| width.checked_mul(height))
            })
            .and_then(|pixels| pixels.checked_mul(4))
            .ok_or(ImageAssetError::InvalidDimensions { width, height })?;
        if width == 0 || height == 0 || rgba8.len() != expected {
            return Err(ImageAssetError::InvalidDataLength {
                width,
                height,
                expected,
                actual: rgba8.len(),
            });
        }

        Ok(Self {
            resource_id,
            width,
            height,
            rgba8,
        })
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ImageAssetError {
    #[error("image dimensions are too large: {width}x{height}")]
    InvalidDimensions { width: u32, height: u32 },
    #[error("image data length mismatch for {width}x{height}: expected {expected}, got {actual}")]
    InvalidDataLength {
        width: u32,
        height: u32,
        expected: usize,
        actual: usize,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpriteCommand {
    pub resource_id: u32,
    pub x: f32,
    pub y: f32,
    pub z: i32,
    pub scale: f32,
    pub alpha: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlendMode {
    Alpha,
    Additive,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AffineSpriteSource {
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
    /// Texture-space pivot retained when the source rectangle is clipped.
    pub pivot_uv: [f32; 2],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AffineSpriteCommand {
    pub resource_id: u32,
    pub x: f32,
    pub y: f32,
    pub m00: f32,
    pub m01: f32,
    pub m10: f32,
    pub m11: f32,
    pub z: i32,
    pub alpha: f32,
    /// Linear RGB multiplier; `[1.0, 1.0, 1.0]` draws the texture untinted.
    pub tint: [f32; 3],
    pub blend_mode: BlendMode,
    pub source: Option<AffineSpriteSource>,
}

#[derive(Clone, Debug, Default)]
pub struct RenderFrame {
    pub sprites: Vec<SpriteCommand>,
    pub affine_sprites: Vec<AffineSpriteCommand>,
}

pub struct CapturedFrame {
    pub width: u32,
    pub height: u32,
    pub rgba8: Vec<u8>,
}

impl RenderFrame {
    pub fn sort_for_submission(&mut self) {
        self.sprites.sort_by_key(|sprite| sprite.z);
        self.affine_sprites.sort_by_key(|sprite| sprite.z);
    }
}

#[derive(Clone, Copy)]
enum BatchSprite<'a> {
    AxisAligned(&'a SpriteCommand),
    Affine(&'a AffineSpriteCommand),
}

impl BatchSprite<'_> {
    fn resource_id(self) -> u32 {
        match self {
            Self::AxisAligned(sprite) => sprite.resource_id,
            Self::Affine(sprite) => sprite.resource_id,
        }
    }

    fn z(self) -> i32 {
        match self {
            Self::AxisAligned(sprite) => sprite.z,
            Self::Affine(sprite) => sprite.z,
        }
    }

    fn alpha(self) -> f32 {
        match self {
            Self::AxisAligned(sprite) => sprite.alpha,
            Self::Affine(sprite) => sprite.alpha,
        }
    }

    fn blend_mode(self) -> BlendMode {
        match self {
            Self::AxisAligned(_) => BlendMode::Alpha,
            Self::Affine(sprite) => sprite.blend_mode,
        }
    }

    fn tint(self) -> [f32; 3] {
        match self {
            Self::AxisAligned(_) => [1.0; 3],
            Self::Affine(sprite) => sprite.tint,
        }
    }

    fn source(self) -> Option<AffineSpriteSource> {
        match self {
            Self::AxisAligned(_) => None,
            Self::Affine(sprite) => sprite.source,
        }
    }
}

#[derive(Debug, Error)]
pub enum RendererError {
    #[error("failed to create GPU surface: {0}")]
    SurfaceCreation(String),
    #[error("no compatible GPU adapter was found: {0}")]
    Adapter(String),
    #[error("failed to create GPU device: {0}")]
    Device(String),
    #[error("the GPU surface has no supported configuration")]
    NoSurfaceConfiguration,
    #[error("invalid image asset: {0}")]
    InvalidImage(#[from] ImageAssetError),
    #[error("sprite references unloaded image {0}")]
    MissingImage(u32),
    #[error("GPU surface validation failed while acquiring a frame")]
    SurfaceValidation,
    #[error("GPU frame capture failed: {0}")]
    Capture(String),
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct SpriteVertex {
    position: [f32; 2],
    uv: [f32; 2],
    color: [f32; 4],
}

struct GpuImage {
    _texture: wgpu::Texture,
    _view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
}

#[derive(Clone, Copy, Debug)]
struct DrawCall {
    resource_id: u32,
    start: u32,
    end: u32,
    blend_mode: BlendMode,
}

pub struct GpuRenderer {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    logical_viewport: LogicalViewport,
    pipeline: wgpu::RenderPipeline,
    additive_pipeline: wgpu::RenderPipeline,
    texture_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    images: HashMap<u32, GpuImage>,
}

impl GpuRenderer {
    pub async fn new(window: Arc<Window>) -> Result<Self, RendererError> {
        let instance_descriptor = wgpu::InstanceDescriptor::new_without_display_handle_from_env();
        #[cfg(target_os = "windows")]
        let instance_descriptor = if std::env::var_os("WGPU_BACKEND").is_none() {
            // Prefer Windows' native backend; WGPU_BACKEND remains an explicit override.
            wgpu::InstanceDescriptor {
                backends: wgpu::Backends::DX12 | wgpu::Backends::GL,
                ..instance_descriptor
            }
        } else {
            instance_descriptor
        };
        let instance = wgpu::Instance::new(instance_descriptor);
        let surface = instance
            .create_surface(window.clone())
            .map_err(|error| RendererError::SurfaceCreation(error.to_string()))?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .map_err(|error| RendererError::Adapter(error.to_string()))?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .map_err(|error| RendererError::Device(error.to_string()))?;

        let size = window.inner_size();
        let surface_width = size.width.max(1);
        let surface_height = size.height.max(1);
        let config = surface
            .get_default_config(&adapter, surface_width, surface_height)
            .ok_or(RendererError::NoSurfaceConfiguration)?;
        surface.configure(&device, &config);

        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sprite texture layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("sprite sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("sprite shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sprite pipeline layout"),
            bind_group_layouts: &[Some(&texture_layout)],
            immediate_size: 0,
        });
        let vertex_attributes = wgpu::vertex_attr_array![
            0 => Float32x2,
            1 => Float32x2,
            2 => Float32x4,
        ];
        let vertex_buffers = [Some(wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SpriteVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &vertex_attributes,
        })];
        let make_pipeline = |label, blend| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &vertex_buffers,
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    ..Default::default()
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(blend),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview_mask: None,
                cache: None,
            })
        };
        let pipeline = make_pipeline("sprite pipeline", wgpu::BlendState::ALPHA_BLENDING);
        let additive_pipeline = make_pipeline(
            "sprite additive pipeline",
            wgpu::BlendState {
                color: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::SrcAlpha,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
                alpha: wgpu::BlendComponent {
                    src_factor: wgpu::BlendFactor::One,
                    dst_factor: wgpu::BlendFactor::One,
                    operation: wgpu::BlendOperation::Add,
                },
            },
        );

        Ok(Self {
            instance,
            surface,
            window,
            device,
            queue,
            config,
            size,
            logical_viewport: LogicalViewport::default(),
            pipeline,
            additive_pipeline,
            texture_layout,
            sampler,
            images: HashMap::new(),
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn add_image(&mut self, asset: ImageAsset) -> Result<(), RendererError> {
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("sprite texture"),
            size: wgpu::Extent3d {
                width: asset.width,
                height: asset.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &asset.rgba8,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(asset.width.checked_mul(4).ok_or(
                    RendererError::InvalidImage(ImageAssetError::InvalidDimensions {
                        width: asset.width,
                        height: asset.height,
                    }),
                )?),
                rows_per_image: Some(asset.height),
            },
            wgpu::Extent3d {
                width: asset.width,
                height: asset.height,
                depth_or_array_layers: 1,
            },
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sprite bind group"),
            layout: &self.texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });
        self.images.insert(
            asset.resource_id,
            GpuImage {
                _texture: texture,
                _view: view,
                bind_group,
            },
        );
        Ok(())
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        if size.width == 0 || size.height == 0 {
            return;
        }
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self, frame: &RenderFrame) -> Result<(), RendererError> {
        if self.size.width == 0 || self.size.height == 0 {
            return Ok(());
        }

        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                drop(texture);
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                self.surface = self
                    .instance
                    .create_surface(self.window.clone())
                    .map_err(|error| RendererError::SurfaceCreation(error.to_string()))?;
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                return Err(RendererError::SurfaceValidation);
            }
        };
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("sprite command encoder"),
            });
        self.encode_frame(frame, self.size, &texture_view, &mut encoder)?;
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        self.queue.present(surface_texture);
        Ok(())
    }

    pub fn capture_frame(&mut self, frame: &RenderFrame) -> Result<CapturedFrame, RendererError> {
        let size = PhysicalSize::new(LOGICAL_WIDTH, LOGICAL_HEIGHT);
        let (bytes_per_row, padded_bytes_per_row) = capture_row_layout(size.width)?;
        let swizzle_bgra = match self.config.format {
            wgpu::TextureFormat::Rgba8Unorm | wgpu::TextureFormat::Rgba8UnormSrgb => false,
            wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Bgra8UnormSrgb => true,
            format => {
                return Err(RendererError::Capture(format!(
                    "unsupported surface format for readback: {format:?}"
                )));
            }
        };
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("frame capture texture"),
            size: wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let output_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("frame capture readback"),
            size: u64::from(padded_bytes_per_row) * u64::from(size.height),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("frame capture encoder"),
            });
        self.encode_frame(frame, size, &texture_view, &mut encoder)?;
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &output_buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bytes_per_row),
                    rows_per_image: Some(size.height),
                },
            },
            wgpu::Extent3d {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            },
        );
        self.queue.submit([encoder.finish()]);

        let slice = output_buffer.slice(..);
        let (sender, receiver) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
        self.device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|error| RendererError::Capture(error.to_string()))?;
        receiver
            .recv()
            .map_err(|error| RendererError::Capture(error.to_string()))?
            .map_err(|error| RendererError::Capture(error.to_string()))?;
        let mapped = slice
            .get_mapped_range()
            .map_err(|error| RendererError::Capture(error.to_string()))?;
        let rgba8 = copy_capture_rows(
            &mapped,
            size.width,
            size.height,
            bytes_per_row,
            padded_bytes_per_row,
            swizzle_bgra,
        );
        drop(mapped);
        output_buffer.unmap();
        Ok(CapturedFrame {
            width: size.width,
            height: size.height,
            rgba8,
        })
    }

    fn encode_frame(
        &self,
        frame: &RenderFrame,
        size: PhysicalSize<u32>,
        texture_view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    ) -> Result<(), RendererError> {
        let (vertices, draw_calls) = self.build_batch(frame, size)?;
        let viewport = letterbox_rect(size.width, size.height, self.logical_viewport);
        if viewport.width == 0 || viewport.height == 0 {
            return Ok(());
        }
        let vertex_buffer = (!vertices.is_empty()).then(|| {
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("sprite vertex buffer"),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                })
        });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("sprite render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        render_pass.set_scissor_rect(viewport.x, viewport.y, viewport.width, viewport.height);
        if let Some(vertex_buffer) = &vertex_buffer {
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            for draw_call in draw_calls {
                render_pass.set_pipeline(match draw_call.blend_mode {
                    BlendMode::Alpha => &self.pipeline,
                    BlendMode::Additive => &self.additive_pipeline,
                });
                let image = self
                    .images
                    .get(&draw_call.resource_id)
                    .expect("batch image was checked before encoding");
                render_pass.set_bind_group(0, &image.bind_group, &[]);
                render_pass.draw(draw_call.start..draw_call.end, 0..1);
            }
        }
        Ok(())
    }

    fn build_batch(
        &self,
        frame: &RenderFrame,
        size: PhysicalSize<u32>,
    ) -> Result<(Vec<SpriteVertex>, Vec<DrawCall>), RendererError> {
        let viewport = letterbox_rect(size.width, size.height, self.logical_viewport);
        let logical_scale = viewport.width as f32 / self.logical_viewport.width as f32;
        let mut sprites = Vec::with_capacity(frame.sprites.len() + frame.affine_sprites.len());
        sprites.extend(frame.sprites.iter().map(BatchSprite::AxisAligned));
        sprites.extend(frame.affine_sprites.iter().map(BatchSprite::Affine));
        sprites.sort_by_key(|sprite| sprite.z());

        let mut vertices = Vec::with_capacity(sprites.len() * 6);
        let mut draw_calls = Vec::with_capacity(sprites.len());

        for sprite in sprites {
            let image = self
                .images
                .get(&sprite.resource_id())
                .ok_or(RendererError::MissingImage(sprite.resource_id()))?;
            let source = sprite.source();
            let corners = match sprite {
                BatchSprite::AxisAligned(sprite) => {
                    let x = viewport.x as f32 + sprite.x * logical_scale;
                    let y = viewport.y as f32 + sprite.y * logical_scale;
                    let width = image_width(image) * sprite.scale * logical_scale;
                    let height = image_height(image) * sprite.scale * logical_scale;
                    [
                        [x, y],
                        [x + width, y],
                        [x + width, y + height],
                        [x, y + height],
                    ]
                }
                BatchSprite::Affine(sprite) => {
                    affine_corners(sprite, image_width(image), image_height(image), source).map(
                        |[x, y]| {
                            [
                                viewport.x as f32 + x * logical_scale,
                                viewport.y as f32 + y * logical_scale,
                            ]
                        },
                    )
                }
            };
            let tint = sprite.tint();
            let color = [
                tint[0].clamp(0.0, 1.0),
                tint[1].clamp(0.0, 1.0),
                tint[2].clamp(0.0, 1.0),
                sprite.alpha().clamp(0.0, 1.0),
            ];
            let start = u32::try_from(vertices.len()).unwrap_or(u32::MAX);
            append_quad(&mut vertices, corners, color, source, size);
            let end = u32::try_from(vertices.len()).unwrap_or(u32::MAX);
            draw_calls.push(DrawCall {
                resource_id: sprite.resource_id(),
                start,
                end,
                blend_mode: sprite.blend_mode(),
            });
        }

        Ok((vertices, draw_calls))
    }
}

fn image_width(image: &GpuImage) -> f32 {
    image._texture.width() as f32
}

fn image_height(image: &GpuImage) -> f32 {
    image._texture.height() as f32
}

fn affine_corners(
    sprite: &AffineSpriteCommand,
    width: f32,
    height: f32,
    source: Option<AffineSpriteSource>,
) -> [[f32; 2]; 4] {
    let (left, top, right, bottom) = source.map_or(
        (-width * 0.5, -height * 0.5, width * 0.5, height * 0.5),
        |source| {
            (
                (source.uv_min[0] - source.pivot_uv[0]) * width,
                (source.uv_min[1] - source.pivot_uv[1]) * height,
                (source.uv_max[0] - source.pivot_uv[0]) * width,
                (source.uv_max[1] - source.pivot_uv[1]) * height,
            )
        },
    );
    [
        affine_point(sprite, left, top),
        affine_point(sprite, right, top),
        affine_point(sprite, right, bottom),
        affine_point(sprite, left, bottom),
    ]
}

fn affine_point(sprite: &AffineSpriteCommand, x: f32, y: f32) -> [f32; 2] {
    [
        sprite.x + sprite.m00 * x + sprite.m01 * y,
        sprite.y + sprite.m10 * x + sprite.m11 * y,
    ]
}

fn append_quad(
    vertices: &mut Vec<SpriteVertex>,
    corners: [[f32; 2]; 4],
    color: [f32; 4],
    source: Option<AffineSpriteSource>,
    size: PhysicalSize<u32>,
) {
    let (uv_min, uv_max) = source.map_or(([0.0, 0.0], [1.0, 1.0]), |source| {
        (source.uv_min, source.uv_max)
    });
    vertices.extend([
        SpriteVertex::new(ndc(corners[0][0], corners[0][1], size), uv_min, color),
        SpriteVertex::new(
            ndc(corners[1][0], corners[1][1], size),
            [uv_max[0], uv_min[1]],
            color,
        ),
        SpriteVertex::new(ndc(corners[2][0], corners[2][1], size), uv_max, color),
        SpriteVertex::new(ndc(corners[0][0], corners[0][1], size), uv_min, color),
        SpriteVertex::new(ndc(corners[2][0], corners[2][1], size), uv_max, color),
        SpriteVertex::new(
            ndc(corners[3][0], corners[3][1], size),
            [uv_min[0], uv_max[1]],
            color,
        ),
    ]);
}

fn ndc(x: f32, y: f32, size: PhysicalSize<u32>) -> [f32; 2] {
    [
        x / size.width as f32 * 2.0 - 1.0,
        1.0 - y / size.height as f32 * 2.0,
    ]
}

fn capture_row_layout(width: u32) -> Result<(u32, u32), RendererError> {
    let bytes_per_row = width
        .checked_mul(4)
        .ok_or_else(|| RendererError::Capture("capture row is too wide".to_owned()))?;
    let padded_bytes_per_row = bytes_per_row
        .div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT)
        .checked_mul(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT)
        .ok_or_else(|| RendererError::Capture("capture row padding overflowed".to_owned()))?;
    Ok((bytes_per_row, padded_bytes_per_row))
}

fn copy_capture_rows(
    mapped: &[u8],
    width: u32,
    height: u32,
    bytes_per_row: u32,
    padded_bytes_per_row: u32,
    swizzle_bgra: bool,
) -> Vec<u8> {
    let row_bytes = bytes_per_row as usize;
    let padded_row_bytes = padded_bytes_per_row as usize;
    let mut rgba8 = Vec::with_capacity(row_bytes * height as usize);
    for row in mapped.chunks_exact(padded_row_bytes).take(height as usize) {
        rgba8.extend_from_slice(&row[..row_bytes]);
    }
    if swizzle_bgra {
        for pixel in rgba8.chunks_exact_mut(4) {
            pixel.swap(0, 2);
        }
    }
    debug_assert_eq!(rgba8.len(), width as usize * height as usize * 4);
    rgba8
}

impl SpriteVertex {
    fn new(position: [f32; 2], uv: [f32; 2], color: [f32; 4]) -> Self {
        Self {
            position,
            uv,
            color,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letterbox_rect_preserves_the_logical_aspect_ratio() {
        assert_eq!(
            letterbox_rect(1280, 720, LogicalViewport::default()),
            ViewportRect {
                x: 160,
                y: 0,
                width: 960,
                height: 720,
            }
        );
        assert_eq!(
            letterbox_rect(800, 600, LogicalViewport::default()),
            ViewportRect {
                x: 0,
                y: 0,
                width: 800,
                height: 600,
            }
        );
    }

    #[test]
    fn logical_position_ignores_letterbox_borders() {
        assert_eq!(
            logical_position(
                1280,
                720,
                PhysicalPosition::new(10.0, 10.0),
                LogicalViewport::default(),
            ),
            None
        );
        assert_eq!(
            logical_position(
                1280,
                720,
                PhysicalPosition::new(160.0, 0.0),
                LogicalViewport::default(),
            ),
            Some((0.0, 0.0))
        );
    }

    #[test]
    fn render_frame_sort_keeps_equal_depth_order() {
        let mut frame = RenderFrame {
            sprites: vec![
                SpriteCommand {
                    resource_id: 1,
                    x: 0.0,
                    y: 0.0,
                    z: 2,
                    scale: 1.0,
                    alpha: 1.0,
                },
                SpriteCommand {
                    resource_id: 2,
                    x: 0.0,
                    y: 0.0,
                    z: 1,
                    scale: 1.0,
                    alpha: 1.0,
                },
                SpriteCommand {
                    resource_id: 3,
                    x: 0.0,
                    y: 0.0,
                    z: 2,
                    scale: 1.0,
                    alpha: 1.0,
                },
            ],
            ..Default::default()
        };

        frame.sort_for_submission();

        assert_eq!(
            frame
                .sprites
                .iter()
                .map(|sprite| sprite.resource_id)
                .collect::<Vec<_>>(),
            vec![2, 1, 3]
        );
    }

    #[test]
    fn affine_sprite_uses_a_centered_transform() {
        let sprite = AffineSpriteCommand {
            resource_id: 1,
            x: 10.0,
            y: 20.0,
            m00: 1.0,
            m01: 0.0,
            m10: 0.0,
            m11: 1.0,
            z: 0,
            alpha: 1.0,
            tint: [1.0; 3],
            blend_mode: BlendMode::Alpha,
            source: None,
        };

        assert_eq!(
            affine_corners(&sprite, 4.0, 6.0, None),
            [[8.0, 17.0], [12.0, 17.0], [12.0, 23.0], [8.0, 23.0]]
        );
        let source = AffineSpriteSource {
            uv_min: [0.25, 0.25],
            uv_max: [0.5, 0.75],
            pivot_uv: [0.375, 0.5],
        };
        assert_eq!(
            affine_corners(&sprite, 400.0, 200.0, Some(source)),
            [[-40.0, -30.0], [60.0, -30.0], [60.0, 70.0], [-40.0, 70.0]]
        );
    }

    #[test]
    fn image_asset_rejects_mismatched_rgba_data() {
        assert!(matches!(
            ImageAsset::new(1, 2, 2, vec![0; 3]),
            Err(ImageAssetError::InvalidDataLength {
                width: 2,
                height: 2,
                expected: 16,
                actual: 3,
            })
        ));
    }

    #[test]
    fn capture_rows_drop_padding_and_convert_bgra() {
        let mapped = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 9, 9, 9, 9, 9, 9, 11, 12, 13, 14, 15, 16, 17, 18, 19, 19,
            19, 19, 19, 19, 19, 19,
        ];
        assert_eq!(
            copy_capture_rows(&mapped, 2, 2, 8, 16, true),
            vec![3, 2, 1, 4, 7, 6, 5, 8, 13, 12, 11, 14, 17, 16, 15, 18]
        );
    }
}
