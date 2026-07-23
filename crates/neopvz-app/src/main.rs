use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
    process::ExitCode,
    sync::Arc,
    time::{Duration, Instant},
};

use clap::Parser;
use neopvz_core::{Game, InputAction, InputFrame, SaveError, SaveProfile, SceneKind};
use neopvz_data::{AssetLayout, ResourceProvider};
use neopvz_render::{
    CRAZY_DAVE_BEARD_IMAGE_ID, CRAZY_DAVE_BODY_IMAGE_ID, CRAZY_DAVE_EYE_IMAGE_ID,
    CRAZY_DAVE_EYEBROW_IMAGE_ID, CRAZY_DAVE_HEAD_IMAGE_ID, CRAZY_DAVE_INNER_ARM_IMAGE_ID,
    CRAZY_DAVE_INNER_FINGER1_IMAGE_ID, CRAZY_DAVE_INNER_FINGER2_IMAGE_ID,
    CRAZY_DAVE_INNER_FINGER3_IMAGE_ID, CRAZY_DAVE_INNER_FINGER4_IMAGE_ID,
    CRAZY_DAVE_INNER_HAND_IMAGE_ID, CRAZY_DAVE_MOUTH_IMAGE_ID, CRAZY_DAVE_OUTER_ARM_IMAGE_ID,
    CRAZY_DAVE_OUTER_FINGER1_IMAGE_ID, CRAZY_DAVE_OUTER_FINGER2_IMAGE_ID,
    CRAZY_DAVE_OUTER_FINGER3_IMAGE_ID, CRAZY_DAVE_OUTER_FINGER4_IMAGE_ID,
    CRAZY_DAVE_OUTER_HAND_IMAGE_ID, CRAZY_DAVE_POT_IMAGE_ID, DAY_BACKGROUND_IMAGE_ID, GpuRenderer,
    ImageAsset, LogicalViewport, RenderFrame, SCREEN_PIXEL_IMAGE_ID, SEED_CHOOSER_IMAGE_ID,
    SELECTOR_ADVENTURE_IMAGE_ID, SELECTOR_ALMANAC_IMAGE_ID, SELECTOR_BASE_IMAGE_ID,
    SELECTOR_CENTER_IMAGE_ID, SELECTOR_CHALLENGES_IMAGE_ID, SELECTOR_HELP_IMAGE_ID,
    SELECTOR_LEAVES_IMAGE_ID, SELECTOR_LEFT_IMAGE_ID, SELECTOR_OPTIONS_IMAGE_ID,
    SELECTOR_QUIT_IMAGE_ID, SELECTOR_RIGHT_IMAGE_ID, SELECTOR_STORE_IMAGE_ID,
    SELECTOR_SURVIVAL_IMAGE_ID, SELECTOR_TROPHY_IMAGE_ID, SELECTOR_VASEBREAKER_IMAGE_ID,
    SELECTOR_WOODSIGN1_IMAGE_ID, SELECTOR_WOODSIGN2_IMAGE_ID, SELECTOR_WOODSIGN3_IMAGE_ID,
    SELECTOR_ZEN_GARDEN_IMAGE_ID, SpriteCommand, TITLE_IMAGE_ID, TITLE_LOGO_IMAGE_ID,
    TUTORIAL_BUBBLE_IMAGE_ID, TUTORIAL_CONTINUE_IMAGE_ID, TUTORIAL_TEXT1_IMAGE_ID,
    TUTORIAL_TEXT2_IMAGE_ID, UI_PIXEL_IMAGE_ID, logical_position,
};
use winit::{
    application::ApplicationHandler,
    dpi::{LogicalSize, PhysicalPosition},
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
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
}

const SIMULATION_STEP: Duration = Duration::from_millis(10);

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

    let assets = match load_assets(&resources) {
        Ok(assets) => assets,
        Err(error) => {
            tracing::error!(%error, "required display resources failed to load");
            return ExitCode::FAILURE;
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
    let mut app = App::new(assets);
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
        load_image(resources, DAY_BACKGROUND_IMAGE_ID, "images/background1.jpg")?,
    ];
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
    game: Game,
    pending_input: Vec<InputAction>,
    last_update: Option<Instant>,
    simulation_accumulator: Duration,
    cursor_position: Option<PhysicalPosition<f64>>,
    tutorial_page: u8,
}

impl App {
    fn new(assets: Vec<ImageAsset>) -> Self {
        Self {
            renderer: None,
            assets,
            game: Game::new(0, SceneKind::Title),
            pending_input: Vec::new(),
            last_update: None,
            simulation_accumulator: Duration::ZERO,
            cursor_position: None,
            tutorial_page: 0,
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
    }

    fn handle_key(&mut self, event_loop: &ActiveEventLoop, key: PhysicalKey) {
        let PhysicalKey::Code(key) = key else {
            return;
        };

        match key {
            KeyCode::Escape => event_loop.exit(),
            KeyCode::Enter => match self.game.state().scene {
                SceneKind::Title => self.start_scene(SceneKind::AdventureSelect),
                SceneKind::AdventureSelect => self.start_scene(SceneKind::AdventureTutorial),
                SceneKind::AdventureTutorial => self.advance_tutorial(),
                SceneKind::SeedChooser => self.start_scene(SceneKind::Day),
                _ => {}
            },
            KeyCode::Digit1 if self.game.state().scene == SceneKind::Day => {
                self.pending_input.push(InputAction::SelectSeed { slot: 0 });
            }
            KeyCode::Digit2 if self.game.state().scene == SceneKind::Day => {
                self.pending_input.push(InputAction::SelectSeed { slot: 1 });
            }
            KeyCode::Space if self.game.state().scene == SceneKind::Day => {
                let action = if self.game.state().paused {
                    InputAction::Resume
                } else {
                    InputAction::Pause
                };
                self.pending_input.push(action);
            }
            KeyCode::KeyP if self.game.state().scene == SceneKind::Day => {
                self.pending_input
                    .push(InputAction::Plant { row: 2, column: 2 });
            }
            _ => {}
        }
    }

    fn start_scene(&mut self, scene: SceneKind) {
        self.game = Game::new(0, scene);
        self.tutorial_page = 0;
        self.pending_input.clear();
        self.simulation_accumulator = Duration::ZERO;
        self.last_update = Some(Instant::now());
    }

    fn advance_tutorial(&mut self) {
        if self.tutorial_page == 0 {
            self.tutorial_page = 1;
        } else {
            self.start_scene(SceneKind::SeedChooser);
        }
    }

    fn handle_mouse_click(&mut self) {
        let scene = self.game.state().scene;
        if scene != SceneKind::Title
            && scene != SceneKind::Day
            && scene != SceneKind::AdventureSelect
            && scene != SceneKind::AdventureTutorial
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
            if (300.0..500.0).contains(&x) && (520.0..600.0).contains(&y) {
                self.start_scene(SceneKind::AdventureSelect);
            }
            return;
        }
        if scene == SceneKind::AdventureSelect {
            if (400.0..730.0).contains(&x) && (55.0..175.0).contains(&y) {
                self.start_scene(SceneKind::AdventureTutorial);
            }
            return;
        }
        if scene == SceneKind::AdventureTutorial {
            if (285.0..565.0).contains(&x) && (20.0..190.0).contains(&y) {
                self.advance_tutorial();
            }
            return;
        }
        if !(80.0..800.0).contains(&x) || !(120.0..570.0).contains(&y) {
            return;
        }
        self.pending_input.push(InputAction::Plant {
            row: ((y - 120.0) / 90.0) as u8,
            column: ((x - 80.0) / 80.0) as u8,
        });
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
            self.game.advance(input);
            self.simulation_accumulator -= SIMULATION_STEP;
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
                    y: 0.0,
                    z: 1,
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
            }
            SceneKind::Day => {
                frame.sprites.push(SpriteCommand {
                    resource_id: DAY_BACKGROUND_IMAGE_ID,
                    x: 0.0,
                    y: 0.0,
                    z: 0,
                    scale: 1.0,
                    alpha: 1.0,
                });
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
                button: MouseButton::Left,
                ..
            } => self.handle_mouse_click(),
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
