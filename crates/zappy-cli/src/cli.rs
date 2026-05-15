//! CLI

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};

use crate::commands;

/// Zappy CLI.
#[derive(Debug, Parser)]
#[command(name = "zappy")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Output verbosity. Use `-v`, `-vv`, `-vvv` for increasingly detailed output.
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Clear templates cache before executing a command.
    #[arg(short = 'c', long)]
    pub clear: bool,

    /// Execute a Zappy command.
    #[command(subcommand)]
    pub command: Command,
}

/// Zappy commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// List available templates.
    #[command(alias = "ls")]
    List(ListArgs),

    /// Display detailed info about a template.
    #[command(alias = "i")]
    Info(InfoArgs),

    /// Generate a new project from a template.
    #[command(alias = "n")]
    New(NewArgs),

    /// Validate that a template generates a working project.
    #[command(alias = "val")]
    Validate(ValidateArgs),

    /// Initialize an empty template skeleton.
    #[command(alias = "it")]
    Init(InitArgs),

    /// Create a template from an existing project, or create an empty
    /// template skeleton.
    #[command(alias = "c")]
    Create(CreateArgs),
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
    #[arg(short = 't', long)]
    pub template: String,

    /// Optional templates directory override.
    #[arg(short = 'i', long)]
    pub templates_dir: Option<PathBuf>,
}

/// Zappy new command args.
#[derive(Args, Debug, Clone)]
pub struct NewArgs {
    /// Template ID to generate from.
    #[arg(short = 't', long)]
    pub template: String,

    /// Name of the project to generate.
    #[arg(short = 'n', long = "name")]
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

    /// Do not run template generation hooks.
    #[arg(short = 's', long)]
    pub no_hooks: bool,
}

/// Zappy validate command args
#[derive(Args, Debug, Clone)]
pub struct ValidateArgs {
    /// Template ID to validate.
    #[arg(short = 't', long)]
    pub template: String,

    /// Optional templates directory override.
    #[arg(short = 'i', long)]
    pub templates_dir: Option<PathBuf>,

    /// Keep the temp validation directory.
    #[arg(short = 'k', long)]
    pub keep_temp: bool,

    /// Do not run template generation hooks. (has no effect on validation hooks)
    #[arg(short = 's', long)]
    pub no_hooks: bool,
}

/// Zappy init command args.
#[derive(Args, Debug, Clone)]
pub struct InitArgs {
    /// Output path where the template skeleton should be created.
    #[arg(short = 'o', long)]
    pub output: PathBuf,

    /// Optional template id.
    #[arg(short = 't', long)]
    pub template: Option<String>,

    /// Optional template name.
    #[arg(short = 'n', long)]
    pub name: Option<String>,

    /// Optional template description.
    #[arg(short = 'd', long)]
    pub description: Option<String>,

    /// Force conflicting file overwrites?
    #[arg(short = 'f', long)]
    pub force: bool,
}

/// Zappy create command args.
#[derive(Args, Debug, Clone)]
pub struct CreateArgs {
    /// Existing project to turn into a template.
    #[arg(short = 'i', long)]
    pub from: Option<PathBuf>,

    /// Putput path where the template/skeleton should be created.
    #[arg(short = 'o', long)]
    pub output: PathBuf,

    /// Generate an empty template skeleton.
    #[arg(short = 'e', long)]
    pub empty: bool,

    /// Optional template id.
    #[arg(short = 't', long)]
    pub template: Option<String>,

    /// Optional template name.
    #[arg(short = 'n', long)]
    pub name: Option<String>,

    /// Optional template description.
    #[arg(short = 's', long)]
    pub description: Option<String>,

    /// Template variables in `key=value` form.
    #[arg(short = 'a', long = "var")]
    pub vars: Vec<String>,

    /// Force conflicting file overwrites?
    #[arg(short = 'f', long)]
    pub force: bool,
}

/// Run zappy CLI.
#[must_use]
pub fn run() -> ExitCode {
    let cli = Cli::parse();

    if cli.clear {
        match zappy_templates::clear_cache_dir() {
            Ok(()) => {}
            Err(error) => {
                eprintln!("Error: {error}");
                return ExitCode::FAILURE;
            }
        }
    }

    match cli.command {
        Command::List(args) => commands::list(&args),
        Command::Info(args) => commands::info(&args),
        Command::New(args) => commands::new(&args),
        Command::Validate(args) => commands::validate(&args),
        Command::Init(args) => commands::init_template(&args),
        Command::Create(args) => commands::create(&args),
    }
}
