mod cli;
mod commands;
mod model;
mod ops;
mod workspace;

use anyhow::Result;
use clap::Parser;

use crate::cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::List { path } => commands::list(path),
        Commands::Check { capsule_dir } => commands::check(capsule_dir),
        Commands::Build { capsule_dir } => commands::build(capsule_dir),
        Commands::Verify { capsule_dir } => commands::verify(capsule_dir),
        Commands::Id { capsule_dir } => commands::id(capsule_dir),
        Commands::Inspect { capsule_dir } => commands::inspect(capsule_dir),
        Commands::Resolve { app_dir } => commands::resolve(app_dir),
    }
}
