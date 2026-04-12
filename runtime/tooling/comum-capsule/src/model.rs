use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

pub const ALLOWED_ABI: &str = "comum-wasm-v1";

#[derive(Debug, Deserialize, Serialize)]
pub struct CapsuleManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub artifact: CapsuleArtifact,
    pub verbs: Vec<CapsuleVerb>,
}

#[derive(Debug, Serialize)]
pub struct CapsuleArtifact {
    pub crate_: String,
    pub wasm: String,
    pub runtime_abi: String,
    pub entrypoint: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CapsuleVerb {
    pub name: String,
    pub payload: BTreeMap<String, String>,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub choices: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppCapsulesManifest {
    pub capsules: Vec<AppCapsuleDependency>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AppCapsuleDependency {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CapsuleBuildInfo {
    pub schema: String,
    pub name: String,
    pub version: String,
    pub runtime_abi: String,
    pub entrypoint: String,
    pub crate_name: String,
    pub target: String,
    pub wasm_path: String,
    pub capsule_id: String,
    pub wasm_sha3_256: String,
    pub exports: Vec<String>,
    pub imports: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CapsulesLock {
    pub capsules: Vec<ResolvedCapsule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolvedCapsule {
    pub name: String,
    pub version: String,
    pub capsule_id: String,
    pub wasm: String,
    pub runtime_abi: String,
    pub path: String,
}

#[derive(Debug)]
pub struct CargoInfo {
    pub package_name: String,
    pub version: String,
    pub crate_types: Vec<String>,
}

impl<'de> Deserialize<'de> for CapsuleArtifact {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            crate_name: Option<String>,
            crate_: Option<String>,
            #[serde(rename = "crate")]
            crate_rename: Option<String>,
            wasm: String,
            runtime_abi: String,
            entrypoint: String,
        }

        let helper = Helper::deserialize(deserializer)?;
        let crate_ = helper
            .crate_rename
            .or(helper.crate_)
            .or(helper.crate_name)
            .ok_or_else(|| serde::de::Error::missing_field("artifact.crate"))?;
        Ok(Self {
            crate_,
            wasm: helper.wasm,
            runtime_abi: helper.runtime_abi,
            entrypoint: helper.entrypoint,
        })
    }
}

impl CapsuleManifest {
    pub fn validate(&self) -> Result<()> {
        validate_slug(&self.name, "name")?;
        Version::parse(&self.version).context("invalid semver in version")?;
        if self.description.trim().is_empty() {
            bail!("description must not be empty");
        }
        validate_slug(&self.artifact.crate_, "artifact.crate")?;
        if !self.artifact.wasm.ends_with(".wasm") {
            bail!("artifact.wasm must end with .wasm");
        }
        if self.artifact.runtime_abi != ALLOWED_ABI {
            bail!("unsupported runtime_abi: {}", self.artifact.runtime_abi);
        }
        if self.artifact.entrypoint != "invoke" {
            bail!("unsupported entrypoint: {}", self.artifact.entrypoint);
        }
        if self.verbs.is_empty() {
            bail!("verbs must not be empty");
        }
        let mut seen = std::collections::BTreeSet::new();
        for verb in &self.verbs {
            if !seen.insert(verb.name.clone()) {
                bail!("duplicate verb: {}", verb.name);
            }
            if !verb.name.starts_with(&(self.name.clone() + "/")) {
                bail!("verb {} must start with {}/", verb.name, self.name);
            }
            if verb.payload.is_empty() {
                bail!("verb {} must declare a payload", verb.name);
            }
            for ty in verb.payload.values() {
                validate_payload_type(ty)?;
            }
        }
        Ok(())
    }

    pub fn wasm_path(&self, capsule_dir: &Path) -> PathBuf {
        capsule_dir.join(&self.artifact.wasm)
    }

    pub fn build_info_path(capsule_dir: &Path) -> PathBuf {
        capsule_dir.join("capsule.build.json")
    }
}

impl AppCapsulesManifest {
    pub fn validate(&self) -> Result<()> {
        if self.capsules.is_empty() {
            bail!("capsules list must not be empty");
        }
        for dep in &self.capsules {
            validate_slug(&dep.name, "capsules[].name")?;
            parse_version_req(&dep.version).context("invalid dependency version requirement")?;
        }
        Ok(())
    }
}

pub fn read_capsule_manifest(path: &Path) -> Result<CapsuleManifest> {
    let content = std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let manifest: CapsuleManifest = serde_yaml::from_str(&content)
        .with_context(|| format!("parse {}", path.display()))?;
    manifest.validate()?;
    Ok(manifest)
}

pub fn read_app_manifest(path: &Path) -> Result<AppCapsulesManifest> {
    let content = std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let manifest: AppCapsulesManifest = serde_yaml::from_str(&content)
        .with_context(|| format!("parse {}", path.display()))?;
    manifest.validate()?;
    Ok(manifest)
}

pub fn parse_version_req(input: &str) -> Result<VersionReq> {
    let normalized = input.replace('x', "*").replace('X', "*");
    Ok(VersionReq::parse(&normalized)?)
}

fn validate_slug(value: &str, field: &str) -> Result<()> {
    if value.is_empty() {
        bail!("{} must not be empty", field);
    }
    let mut chars = value.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_lowercase() {
        bail!("{} must start with a lowercase ASCII letter", field);
    }
    if !value.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '/') {
        bail!("{} must use lowercase ASCII, digits, '-' or '/'", field);
    }
    Ok(())
}

fn validate_payload_type(value: &str) -> Result<()> {
    match value {
        "text" | "integer" | "did" | "bytes32" => Ok(()),
        other => bail!("unsupported payload type: {}", other),
    }
}

impl fmt::Display for CapsuleBuildInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "name: {}", self.name)?;
        writeln!(f, "version: {}", self.version)?;
        writeln!(f, "crate: {}", self.crate_name)?;
        writeln!(f, "wasm: {}", self.wasm_path)?;
        writeln!(f, "capsule_id: {}", self.capsule_id)?;
        writeln!(f, "runtime_abi: {}", self.runtime_abi)?;
        writeln!(f, "entrypoint: {}", self.entrypoint)?;
        writeln!(f, "exports: {}", self.exports.join(", "))?;
        write!(f, "imports: {}", self.imports.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_version_req, read_capsule_manifest};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn version_req_accepts_x() {
        assert!(parse_version_req("0.1.x").is_ok());
    }

    #[test]
    fn manifest_requires_namespaced_verbs() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("capsule.yaml");
        fs::write(
            &file,
            "name: feira\nversion: 0.1.0\ndescription: ok\nartifact:\n  crate: capsula-feira\n  wasm: feira.wasm\n  runtime_abi: comum-wasm-v1\n  entrypoint: invoke\nverbs:\n  - name: offer\n    payload:\n      item: text\n",
        )
        .unwrap();
        assert!(read_capsule_manifest(&file).is_err());
    }
}
