use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::{Add, Command};
use crate::manifest::Manifest;

/// The command line interface parser.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a package.
    Add(Add),
}

impl Command for Cli {
    fn eval(&self, manifest: &mut Manifest, cwd: &PathBuf) -> Result<()> {
        match &self.command {
            Commands::Add(command) => command.eval(manifest, cwd),
        }
    }
}
