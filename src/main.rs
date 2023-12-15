#![allow(missing_docs)]

use std::env;
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use git_pkg::cli::Cli;
use git_pkg::manifest::Manifest;
use git_pkg::commands::Command;

use anyhow::Result;

fn main() -> Result<()> {
    let cwd = env::current_dir()?;
    let mut manifest = init_manifest(&cwd)?;

    let cli = Cli::parse();
    cli.eval(&mut manifest, &cwd)?;
    // TODO: manifest.write(&cwd)?;
    dbg!(manifest);
    // TODO: when add, we should validate and add deps to the manifest
    // TODO: when install, we should use the Manifest to fetch the packages, resolve versions and
    // update .gitmodules file
    // TODO: remove
    // TODO: update

    Ok(())
}

fn init_manifest(cwd: &PathBuf) -> Result<Manifest> {
    let mut file = cwd.clone();
    file.push("gpkg.toml");

    if file.exists() {
        let contents = fs::read_to_string(&cwd)?;
        Ok(Manifest::parse(&contents)?)
    } else {
        Ok(Manifest::default())
    }
}
