use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

/// The manifest (`gpkg.toml`) file.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Manifest {
    dependencies: Vec<Dependency>,
}

impl Manifest {
    /// Parse the manifest file and initialize a `Manifest` struct.
    pub fn parse(contents: &str) -> Result<Self, Error> {
        let manifest: Self = toml::from_str(&contents).map_err(|e| Error::Parse(e.to_string()))?;

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

    /// Add a dependency to the manifest.
    pub fn add_dependency(&mut self, dep: Dependency) -> Result<(), Error> {
        Self::validate_dependency(&dep)?;
        // TODO the dependency should not be added if it already exists
        self.dependencies.push(dep);
        Ok(())
    }
}

/// The `gpkg.toml` dependency entry.
#[derive(Debug, Deserialize, Serialize)]
pub struct Dependency {
    pub(crate) name: String,
    pub(crate) url: Url,
    pub(crate) branch: Option<String>,
    pub(crate) tag: Option<String>,
    pub(crate) path: Option<PathBuf>,
    #[serde(default = "default_bool")]
    pub(crate) recursive: bool,
    #[serde(default = "default_bool")]
    pub(crate) shallow: bool,
}

fn default_bool() -> bool {
    true
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
