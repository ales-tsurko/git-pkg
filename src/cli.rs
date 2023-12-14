use clap::{Args, Parser, Subcommand};

/// The command line interface parser.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    // /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a package.
    Add(Add),
}

#[derive(Args)]
struct Add {
    /// Name for the package.
    name: Option<String>,
    #[arg(short, long)]
    url: String,
    #[command(flatten)]
    version: Ver,
    /// Destination path.
    #[arg(short, long, default_value = "./gpkgs/")]
    path: String,
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
