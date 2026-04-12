use std::path::Path;
use std::process::Command;

use anyhow::{bail, Context, Result};
use sha3::{Digest, Sha3_256};
use wasmparser::{Parser, Payload};

use crate::model::{CapsuleBuildInfo, CapsuleManifest, CargoInfo};
use crate::workspace::find_workspace_root;

pub fn read_cargo_info(capsule_dir: &Path) -> Result<CargoInfo> {
    let cargo_toml = capsule_dir.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml)
        .with_context(|| format!("read {}", cargo_toml.display()))?;
    let value: toml::Value = toml::from_str(&content).with_context(|| format!("parse {}", cargo_toml.display()))?;
    let package = value
        .get("package")
        .and_then(|v| v.as_table())
        .context("Cargo.toml missing [package]")?;
    let lib = value
        .get("lib")
        .and_then(|v| v.as_table())
        .context("Cargo.toml missing [lib]")?;
    let package_name = package
        .get("name")
        .and_then(|v| v.as_str())
        .context("Cargo.toml package.name missing")?
        .to_string();
    let version = package
        .get("version")
        .and_then(|v| v.as_str())
        .context("Cargo.toml package.version missing")?
        .to_string();
    let crate_types = lib
        .get("crate-type")
        .and_then(|v| v.as_array())
        .context("Cargo.toml lib.crate-type missing")?
        .iter()
        .filter_map(|v| v.as_str().map(ToOwned::to_owned))
        .collect::<Vec<_>>();
    Ok(CargoInfo {
        package_name,
        version,
        crate_types,
    })
}

pub fn validate_manifest_against_cargo(manifest: &CapsuleManifest, cargo: &CargoInfo) -> Result<()> {
    if manifest.artifact.crate_ != cargo.package_name {
        bail!(
            "artifact.crate {} does not match Cargo package.name {}",
            manifest.artifact.crate_,
            cargo.package_name
        );
    }
    if manifest.version != cargo.version {
        bail!(
            "capsule.yaml version {} does not match Cargo version {}",
            manifest.version,
            cargo.version
        );
    }
    if !cargo.crate_types.iter().any(|t| t == "cdylib") {
        bail!("Cargo lib.crate-type must include cdylib");
    }
    Ok(())
}

pub fn build_capsule(capsule_dir: &Path, manifest: &CapsuleManifest) -> Result<CapsuleBuildInfo> {
    let workspace_root = find_workspace_root(capsule_dir)?;
    let manifest_path = capsule_dir.join("Cargo.toml");
    let status = Command::new(cargo_bin())
        .arg("build")
        .arg("--manifest-path")
        .arg(&manifest_path)
        .arg("--release")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("RUSTFLAGS", "-C link-arg=--export=invoke -C link-arg=--export-memory")
        .current_dir(&workspace_root)
        .status()
        .context("spawn cargo build")?;
    if !status.success() {
        bail!("cargo build failed for {}", capsule_dir.display());
    }
    let built_wasm = workspace_root
        .join("target/wasm32-unknown-unknown/release")
        .join(crate_to_wasm_filename(&manifest.artifact.crate_));
    if !built_wasm.exists() {
        bail!("expected built wasm at {}", built_wasm.display());
    }
    let wasm_target = manifest.wasm_path(capsule_dir);
    std::fs::copy(&built_wasm, &wasm_target)
        .with_context(|| format!("copy {} -> {}", built_wasm.display(), wasm_target.display()))?;
    let info = inspect_wasm(&wasm_target, manifest)?;
    write_build_info(capsule_dir, &info)?;
    Ok(info)
}

pub fn verify_capsule(capsule_dir: &Path, manifest: &CapsuleManifest) -> Result<CapsuleBuildInfo> {
    let wasm_path = manifest.wasm_path(capsule_dir);
    if !wasm_path.exists() {
        bail!("built wasm not found at {}", wasm_path.display());
    }
    let info = inspect_wasm(&wasm_path, manifest)?;
    let build_info_path = CapsuleManifest::build_info_path(capsule_dir);
    if build_info_path.exists() {
        let content = std::fs::read_to_string(&build_info_path)?;
        let existing: CapsuleBuildInfo = serde_json::from_str(&content)?;
        if existing.capsule_id != info.capsule_id {
            bail!("capsule.build.json is stale: capsule_id mismatch");
        }
    }
    Ok(info)
}

pub fn write_build_info(capsule_dir: &Path, info: &CapsuleBuildInfo) -> Result<()> {
    let build_info_path = CapsuleManifest::build_info_path(capsule_dir);
    let content = serde_json::to_string_pretty(info)?;
    std::fs::write(&build_info_path, content)
        .with_context(|| format!("write {}", build_info_path.display()))?;
    Ok(())
}

pub fn inspect_wasm(wasm_path: &Path, manifest: &CapsuleManifest) -> Result<CapsuleBuildInfo> {
    let wasm = std::fs::read(wasm_path).with_context(|| format!("read {}", wasm_path.display()))?;
    let mut exports = Vec::new();
    let mut imports = Vec::new();
    for payload in Parser::new(0).parse_all(&wasm) {
        match payload? {
            Payload::ExportSection(section) => {
                for export in section {
                    let export = export?;
                    exports.push(export.name.to_string());
                }
            }
            Payload::ImportSection(section) => {
                for import in section {
                    let import = import?;
                    imports.push(format!("{}.{}", import.module, import.name));
                }
            }
            _ => {}
        }
    }
    if !exports.iter().any(|e| e == &manifest.artifact.entrypoint) {
        bail!("wasm does not export required entrypoint {}", manifest.artifact.entrypoint);
    }
    if !exports.iter().any(|e| e == "memory") {
        bail!("wasm does not export memory");
    }
    for import in &imports {
        if !matches!(import.as_str(), "env.read_graph" | "env.verify_proof" | "env.emit_testimony") {
            bail!("unsupported import {}", import);
        }
    }
    let digest = Sha3_256::digest(&wasm);
    let capsule_id = hex_encode(&digest);
    Ok(CapsuleBuildInfo {
        schema: "comum-capsule-build-v1".to_string(),
        name: manifest.name.clone(),
        version: manifest.version.clone(),
        runtime_abi: manifest.artifact.runtime_abi.clone(),
        entrypoint: manifest.artifact.entrypoint.clone(),
        crate_name: manifest.artifact.crate_.clone(),
        target: "wasm32-unknown-unknown".to_string(),
        wasm_path: wasm_path.to_string_lossy().to_string(),
        capsule_id,
        wasm_sha3_256: hex_encode(&digest),
        exports,
        imports,
    })
}

pub fn crate_to_wasm_filename(crate_name: &str) -> String {
    format!("{}.wasm", crate_name.replace('-', "_"))
}

fn cargo_bin() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}

pub fn hex_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}
