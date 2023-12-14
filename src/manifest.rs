use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The manifest (`gpkg.toml`) file.
#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    #[serde(skip)]
    path: PathBuf,
    dependencies: Vec<Dependency>,
}

impl Manifest {
    /// Parse the manifest file and initialize a `Manifest` struct.
    pub fn parse(path: &str) -> Result<Self, Error> {
        let mut manifest = File::open(path)?;
        let mut buf = String::new();
        manifest.read_to_string(&mut buf)?;

        let mut manifest: Self = toml::from_str(&buf).map_err(|e| Error::Parse(e.to_string()))?;
        manifest.path = PathBuf::from(path);

        for dep in manifest.dependencies.iter() {
            Self::validate_dependency(&dep)?;
        }

        Ok(manifest)
    }

    fn validate_dependency(dep: &Dependency) -> Result<(), Error> {
        // if `branch` and `tag` are both specified, or neither are.
        if dep.branch.is_some() && dep.tag.is_some() || dep.branch.is_none() && dep.tag.is_none() {
            return Err(Error::BranchTag(dep.name.clone()));
        }
        Ok(())
    }
}

/// The `gpkg.toml` dependency entry.
#[derive(Debug, Deserialize, Serialize)]
pub struct Dependency {
    name: String,
    branch: Option<String>,
    tag: Option<String>,
    path: Option<String>,
    recursive: Option<bool>,
    shallow: Option<bool>,
}

/// Module's error type.
#[derive(Debug, Error)]
pub enum Error {
    /// Cannot open manifest file.
    #[error("Cannot open manifest file: {0}")]
    Open(#[from] std::io::Error),
    /// Error parsing manifest file.
    #[error("Error parsing manifest file: {0}")]
    Parse(String),
    /// `branch` or `tag` must be specified (but not both).
    #[error("Dependency `{0}`: `branch` or `tag` must be specified (but not both).")]
    BranchTag(String),
}
