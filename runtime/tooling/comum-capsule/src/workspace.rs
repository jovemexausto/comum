use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use walkdir::WalkDir;

pub fn find_workspace_root(start: &Path) -> Result<PathBuf> {
    let start = start
        .canonicalize()
        .with_context(|| format!("canonicalize {}", start.display()))?;
    for dir in start.ancestors() {
        let cargo = dir.join("Cargo.toml");
        if cargo.exists() {
            let content = std::fs::read_to_string(&cargo).with_context(|| format!("read {}", cargo.display()))?;
            if content.contains("[workspace]") {
                return Ok(dir.to_path_buf());
            }
        }
    }
    bail!("workspace root not found from {}", start.display())
}

pub fn discover_capsule_dirs(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() && entry.file_name() == "capsule.yaml" {
            if let Some(parent) = entry.path().parent() {
                out.push(parent.to_path_buf());
            }
        }
    }
    out.sort();
    out
}
