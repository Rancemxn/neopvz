use std::{io::ErrorKind, path::PathBuf, process::ExitCode};

use clap::Parser;
use neopvz_core::{Game, SaveError, SaveProfile, SceneKind};
use neopvz_data::{AssetLayout, ResourceProvider};

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

    if let (Some(path), Some(profile)) = (profile_path, profile.take()) {
        if let Err(error) = profile.write_atomic(&path) {
            tracing::error!(%error, "profile save failed");
            return ExitCode::FAILURE;
        }
        tracing::info!(path = ?path, "profile saved");
    }

    ExitCode::SUCCESS
}

fn load_profile(path: &std::path::Path) -> Result<SaveProfile, SaveError> {
    match SaveProfile::read(path) {
        Ok(profile) => Ok(profile),
        Err(SaveError::Io(error)) if error.kind() == ErrorKind::NotFound => {
            Ok(SaveProfile::new("default"))
        }
        Err(error) => Err(error),
    }
}
