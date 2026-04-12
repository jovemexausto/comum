use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "comum-capsule")]
#[command(about = "Capsule toolchain for the Comum monorepo")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    List { path: Option<PathBuf> },
    Check { capsule_dir: PathBuf },
    Build { capsule_dir: PathBuf },
    Verify { capsule_dir: PathBuf },
    Id { capsule_dir: PathBuf },
    Inspect { capsule_dir: PathBuf },
    Resolve { app_dir: PathBuf },
}
