#![allow(missing_docs)]

use std::env;

use clap::Parser;
use git_pkg::cli::Cli;
use git_pkg::manifest;

use anyhow::Result;

fn main() -> Result<()> {
    let cwd = env::current_dir()?;
    dbg!(cwd);

    let cli = Cli::parse();

    Ok(())
}
