use std::path::PathBuf;

use clap::Parser;
use neopvz_core::{Game, SceneKind};
use neopvz_data::AssetLayout;

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
        Ok(layout) => tracing::info!(source = ?layout.source, "resource source selected"),
        Err(error) => tracing::error!(%error, "resource discovery failed"),
    }

    let mut game = Game::new(0, SceneKind::Title);
    game.advance(Default::default());
    tracing::info!(tick = game.state().tick, "simulation advanced");
}
