use std::{fmt::Display, path::PathBuf};

use clap::Parser;
use neopvz_core::{Game, SceneKind};
use neopvz_data::{AssetLayout, ResourceKind, ResourceManifest, ResourceProvider};

#[derive(Debug, Parser)]
#[command(name = "neopvz", version, about = "Rust PvZ reimplementation")]
struct Cli {
    #[arg(long, value_name = "PATH", conflicts_with = "pak")]
    data_dir: Option<PathBuf>,
    #[arg(long, value_name = "PATH", conflicts_with = "data_dir")]
    pak: Option<PathBuf>,
}

fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let explicit = cli.data_dir.as_deref().or(cli.pak.as_deref());
    match AssetLayout::discover(explicit) {
        Ok(layout) => {
            tracing::info!(source = ?layout.source, "resource source selected");
            match ResourceProvider::open(&layout.source) {
                Ok(resources) => match resources.read("properties/resources.xml") {
                    Ok(xml) => log_manifest(ResourceManifest::parse(&xml[..])),
                    Err(error) => tracing::error!(%error, "resource manifest lookup failed"),
                },
                Err(error) => tracing::error!(%error, "resource source opening failed"),
            }
        }
        Err(error) => tracing::error!(%error, "resource discovery failed"),
    }

    let mut game = Game::new(0, SceneKind::Title);
    game.advance(Default::default());
    tracing::info!(tick = game.state().tick, "simulation advanced");
}

fn log_manifest(error_or_manifest: Result<ResourceManifest, impl Display>) {
    match error_or_manifest {
        Ok(manifest) => tracing::info!(
            groups = manifest.groups.len(),
            entries = manifest.entry_count(),
            images = manifest.count(ResourceKind::Image),
            fonts = manifest.count(ResourceKind::Font),
            sounds = manifest.count(ResourceKind::Sound),
            "resource manifest parsed"
        ),
        Err(error) => tracing::error!(%error, "resource manifest parsing failed"),
    }
}
