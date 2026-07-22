use std::{path::PathBuf, process::ExitCode};

use clap::Parser;
use neopvz_core::{Game, SceneKind};
use neopvz_data::{AssetLayout, ResourceProvider};

#[derive(Debug, Parser)]
#[command(name = "neopvz", version, about = "Rust PvZ reimplementation")]
struct Cli {
    #[arg(long, value_name = "PATH", conflicts_with = "pak")]
    data_dir: Option<PathBuf>,
    #[arg(long, value_name = "PATH", conflicts_with = "data_dir")]
    pak: Option<PathBuf>,
}

fn main() -> ExitCode {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let explicit = cli.data_dir.as_deref().or(cli.pak.as_deref());
    match AssetLayout::discover(explicit) {
        Ok(layout) => {
            tracing::info!(source = ?layout.source, "resource source selected");
            match ResourceProvider::open(&layout.source) {
                Ok(resources) => match resources.inventory() {
                    Ok(inventory) => {
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
                    }
                    Err(error) => {
                        tracing::error!(%error, "resource inventory failed");
                        return ExitCode::FAILURE;
                    }
                },
                Err(error) => {
                    tracing::error!(%error, "resource source opening failed");
                    return ExitCode::FAILURE;
                }
            }
        }
        Err(error) => {
            tracing::error!(%error, "resource discovery failed");
            return ExitCode::FAILURE;
        }
    }

    let mut game = Game::new(0, SceneKind::Title);
    game.advance(Default::default());
    tracing::info!(tick = game.state().tick, "simulation advanced");
    ExitCode::SUCCESS
}
