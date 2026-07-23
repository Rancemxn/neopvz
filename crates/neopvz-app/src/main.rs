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
    DAY_BACKGROUND_IMAGE_ID, GpuRenderer, ImageAsset, LogicalViewport, RenderFrame,
    SCREEN_PIXEL_IMAGE_ID, SEED_CHOOSER_IMAGE_ID, SELECTOR_ADVENTURE_IMAGE_ID,
    SELECTOR_BASE_IMAGE_ID, SELECTOR_CENTER_IMAGE_ID, SELECTOR_CHALLENGES_IMAGE_ID,
    SELECTOR_LEFT_IMAGE_ID, SELECTOR_RIGHT_IMAGE_ID, SELECTOR_SURVIVAL_IMAGE_ID,
    SELECTOR_VASEBREAKER_IMAGE_ID, SpriteCommand, TITLE_IMAGE_ID, TITLE_LOGO_IMAGE_ID,
    UI_PIXEL_IMAGE_ID, logical_position,
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

struct App {
    renderer: Option<GpuRenderer>,
    assets: Vec<ImageAsset>,
    game: Game,
    pending_input: Vec<InputAction>,
    last_update: Option<Instant>,
    simulation_accumulator: Duration,
    cursor_position: Option<PhysicalPosition<f64>>,
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
                SceneKind::AdventureSelect => self.start_scene(SceneKind::SeedChooser),
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
        self.pending_input.clear();
        self.simulation_accumulator = Duration::ZERO;
        self.last_update = Some(Instant::now());
    }

    fn handle_mouse_click(&mut self) {
        let scene = self.game.state().scene;
        if scene != SceneKind::Day && scene != SceneKind::AdventureSelect {
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
                self.start_scene(SceneKind::SeedChooser);
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
                    x: 0.0,
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
                    y: 0.0,
                    z: 0,
                    scale: 1.0,
                    alpha: 1.0,
                });
                for (resource_id, x, y) in [
                    (SELECTOR_ADVENTURE_IMAGE_ID, 400.0, 55.0),
                    (SELECTOR_CHALLENGES_IMAGE_ID, 407.0, 180.0),
                    (SELECTOR_SURVIVAL_IMAGE_ID, 395.0, 300.0),
                    (SELECTOR_VASEBREAKER_IMAGE_ID, 420.0, 425.0),
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
