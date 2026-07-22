use std::{fmt::Display, path::PathBuf};

use clap::Parser;
use neopvz_core::{Game, SceneKind};
use neopvz_data::{AssetLayout, PakArchive, ResourceKind, ResourceManifest, ResourceSource};

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
            match &layout.source {
                ResourceSource::Directory(_) => {
                    if let Some(path) = layout.manifest.as_deref() {
                        log_manifest(ResourceManifest::load(path));
                    }
                },
                ResourceSource::Pak(path) => match PakArchive::load(path) {
                    Ok(pak) => {
                        tracing::info!(entries = pak.entry_count(), "PAK archive parsed");
                        match pak.read("properties/resources.xml") {
                            Ok(xml) => log_manifest(ResourceManifest::parse(&xml[..])),
                            Err(error) => {
                                tracing::error!(%error, "PAK resource manifest lookup failed")
                            }
                        }
                    }
                    Err(error) => tracing::error!(%error, "PAK archive parsing failed"),
                }
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
