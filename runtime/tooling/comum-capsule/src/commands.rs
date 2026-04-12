use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use semver::Version;

use crate::model::{
    parse_version_req, read_app_manifest, read_capsule_manifest, CapsuleBuildInfo, CapsulesLock,
    ResolvedCapsule,
};
use crate::ops::{
    build_capsule, inspect_wasm, read_cargo_info, validate_manifest_against_cargo, verify_capsule,
};
use crate::workspace::{discover_capsule_dirs, find_workspace_root};

pub fn list(path: Option<PathBuf>) -> Result<()> {
    let root = match path {
        Some(path) => path,
        None => std::env::current_dir()?,
    };
    for dir in discover_capsule_dirs(&root) {
        let manifest = read_capsule_manifest(&dir.join("capsule.yaml"))?;
        println!("{} {} {}", manifest.name, manifest.version, dir.display());
    }
    Ok(())
}

pub fn check(capsule_dir: PathBuf) -> Result<()> {
    let capsule_dir = capsule_dir.canonicalize()?;
    let manifest = read_capsule_manifest(&capsule_dir.join("capsule.yaml"))?;
    let cargo = read_cargo_info(&capsule_dir)?;
    validate_manifest_against_cargo(&manifest, &cargo)?;
    let wasm = manifest.wasm_path(&capsule_dir);
    if wasm.exists() {
        let _ = inspect_wasm(&wasm, &manifest)?;
    }
    println!("ok {} {}", manifest.name, manifest.version);
    Ok(())
}

pub fn build(capsule_dir: PathBuf) -> Result<()> {
    let capsule_dir = capsule_dir.canonicalize()?;
    let manifest = read_capsule_manifest(&capsule_dir.join("capsule.yaml"))?;
    let cargo = read_cargo_info(&capsule_dir)?;
    validate_manifest_against_cargo(&manifest, &cargo)?;
    let info = build_capsule(&capsule_dir, &manifest)?;
    println!("built {} {}", info.name, info.capsule_id);
    Ok(())
}

pub fn verify(capsule_dir: PathBuf) -> Result<()> {
    let capsule_dir = capsule_dir.canonicalize()?;
    let manifest = read_capsule_manifest(&capsule_dir.join("capsule.yaml"))?;
    let cargo = read_cargo_info(&capsule_dir)?;
    validate_manifest_against_cargo(&manifest, &cargo)?;
    let info = verify_capsule(&capsule_dir, &manifest)?;
    println!("verified {} {}", info.name, info.capsule_id);
    Ok(())
}

pub fn id(capsule_dir: PathBuf) -> Result<()> {
    let capsule_dir = capsule_dir.canonicalize()?;
    let manifest = read_capsule_manifest(&capsule_dir.join("capsule.yaml"))?;
    let info = verify_capsule(&capsule_dir, &manifest)?;
    println!("{}", info.capsule_id);
    Ok(())
}

pub fn inspect(capsule_dir: PathBuf) -> Result<()> {
    let capsule_dir = capsule_dir.canonicalize()?;
    let manifest = read_capsule_manifest(&capsule_dir.join("capsule.yaml"))?;
    let cargo = read_cargo_info(&capsule_dir)?;
    validate_manifest_against_cargo(&manifest, &cargo)?;
    let info = if manifest.wasm_path(&capsule_dir).exists() {
        verify_capsule(&capsule_dir, &manifest)?
    } else {
        build_capsule(&capsule_dir, &manifest)?
    };
    println!("{}", info);
    Ok(())
}

pub fn resolve(app_dir: PathBuf) -> Result<()> {
    let app_dir = app_dir.canonicalize()?;
    let workspace_root = find_workspace_root(&app_dir)?;
    let manifest = read_app_manifest(&app_dir.join("capsules.yaml"))?;
    let mut resolved = Vec::new();
    for dep in manifest.capsules {
        let capsule_dir = if let Some(path) = dep.path.as_ref() {
            app_dir.join(path)
        } else {
            resolve_capsule_dir(&workspace_root, &dep.name, &dep.version)?
        };
        let capsule_manifest = read_capsule_manifest(&capsule_dir.join("capsule.yaml"))?;
        let req = parse_version_req(&dep.version)?;
        let version = Version::parse(&capsule_manifest.version)?;
        if !req.matches(&version) {
            bail!(
                "capsule {} version {} does not satisfy {}",
                capsule_manifest.name,
                capsule_manifest.version,
                dep.version
            );
        }
        let cargo = read_cargo_info(&capsule_dir)?;
        validate_manifest_against_cargo(&capsule_manifest, &cargo)?;
        let info: CapsuleBuildInfo = build_capsule(&capsule_dir, &capsule_manifest)?;
        resolved.push(ResolvedCapsule {
            name: capsule_manifest.name,
            version: capsule_manifest.version,
            capsule_id: format!("sha3:{}", info.capsule_id),
            wasm: relative_from(&app_dir, Path::new(&info.wasm_path))?,
            runtime_abi: info.runtime_abi,
            path: relative_from(&app_dir, &capsule_dir)?,
        });
    }
    let lock = CapsulesLock { capsules: resolved };
    let lock_path = app_dir.join("capsules.lock");
    std::fs::write(&lock_path, serde_yaml::to_string(&lock)?).with_context(|| format!("write {}", lock_path.display()))?;
    println!("resolved {}", lock_path.display());
    Ok(())
}

fn resolve_capsule_dir(workspace_root: &Path, name: &str, req: &str) -> Result<PathBuf> {
    let req = parse_version_req(req)?;
    for dir in discover_capsule_dirs(workspace_root) {
        let manifest = read_capsule_manifest(&dir.join("capsule.yaml"))?;
        if manifest.name == name && req.matches(&Version::parse(&manifest.version)?) {
            return Ok(dir);
        }
    }
    bail!("capsule {} matching {} not found", name, req)
}

fn relative_from(base: &Path, path: &Path) -> Result<String> {
    let rel = pathdiff::diff_paths(path, base).context("compute relative path")?;
    Ok(rel.to_string_lossy().to_string())
}
