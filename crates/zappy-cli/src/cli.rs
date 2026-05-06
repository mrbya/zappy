//! CLI

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

/// Zappy CLI.
#[derive(Debug, Parser)]
#[command(name = "zappy")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Output verbosity..
    ///
    /// Use `-v`, `-vv`, `-vvv` for increasingly detailed output.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Execute a Zappy command.
    #[command(subcommand)]
    pub command: Command,
}

/// Zappy commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// List available tempaltes.
    #[command(alias = "ls")]
    List(ListArgs),

    /// Display detailed info about a template.
    #[command(alias = "i")]
    Info(InfoArgs),
}

/// Zappy list command args.
#[derive(Args, Debug, Clone)]
pub struct ListArgs {
    /// Optional template directory override.
    #[arg(short = 'i', long)]
    pub templates_dir: Option<PathBuf>,

    /// Optional language/ecosystem filter.
    #[arg(short = 'l', long)]
    pub language: Option<String>,
}

/// Zappy info command args.
#[derive(Args, Debug, Clone)]
pub struct InfoArgs {
    /// Template ID to inspect.
    #[arg(short = 'i', long)]
    pub template: String,

    /// Optional templates directory override.
    #[arg(short = 'd', long)]
    pub tempaltes_dir: Option<PathBuf>,
}

/// Zappy new command args.
#[derive(Args, Debug, Clone)]
pub struct NewArgs {
    /// Template ID to generate from.
    #[arg(short = 't', long)]
    pub template: String,

    /// Name of the project to generate.
    #[arg(short = 'n', long)]
    pub project_name: String,

    /// Output directory.
    #[arg(short = 'o', long)]
    pub output: Option<PathBuf>,

    /// Template variable override in `key=value` form.
    #[arg(short = 'a', long = "var")]
    pub vars: Vec<String>,

    /// Optional templates directory override.
    #[arg(short = 'i', long)]
    pub templates_dir: Option<PathBuf>,

    /// Preview generation without writing files.
    #[arg(short = 'd', long)]
    pub dry_run: bool,

    /// Non-interactive mode (does not prompt for missing variables).
    #[arg(short = 'x', long)]
    pub non_interactive: bool,

    /// Overwrite existing files.
    #[arg(short = 'f', long)]
    pub force: bool,
}
