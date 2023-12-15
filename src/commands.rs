use std::path::PathBuf;

use clap::Args;
use url::Url;
use anyhow::Result;

use crate::manifest::{Dependency, Manifest};

/// Any command (i.e. `add`, `remove`, etc) is implemented by this trait.
pub trait Command {
    /// Evaluate the command.
    fn eval(&self, manifest: &mut Manifest, cwd: &PathBuf) -> Result<()>;
}

#[derive(Args)]
pub(crate) struct Add {
    /// Name for the package.
    name: Option<String>,
    #[arg(short, long)]
    url: Url,
    #[command(flatten)]
    version: Ver,
    /// Destination path.
    #[arg(short, long, default_value = "gpkgs/")]
    path: PathBuf,
    /// By default git submodule is fetched recursively. Use this flag to disable that.
    #[arg(long)]
    non_recursive: bool,
    /// By default a clone of the git submodule is shallow (with a history depth of 1). Use this
    /// flag to disable that.
    #[arg(long)]
    deep: bool,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Ver {
    /// Use tag to specify package version.
    #[arg(short, long)]
    tag: Option<String>,
    /// Use branch to specify package version.
    #[arg(short, long)]
    branch: Option<String>,
}

impl Add {
    fn make_dependency(&self, cwd: &PathBuf) -> Dependency {
        let name = self.name.clone().unwrap_or_else(|| {
            self.url
                .path_segments()
                .unwrap()
                .last()
                .unwrap()
                .to_string()
        });

        let url = self.url.clone();
        let branch = self.version.branch.clone();
        let tag = self.version.tag.clone();
        let path = Some(if self.path.is_absolute() {
            self.path.clone()
        } else {
            cwd.join(&self.path)
        });
        let recursive = !self.non_recursive;
        let shallow = !self.deep;

        Dependency {
            name,
            url,
            branch,
            tag,
            path,
            recursive,
            shallow,
        }
    }
}

impl Command for Add {
    fn eval(&self, manifest: &mut Manifest, cwd: &PathBuf) -> Result<()> {
        let dep = self.make_dependency(cwd);
        manifest.add_dependency(dep)?;
        Ok(())
    }
}
