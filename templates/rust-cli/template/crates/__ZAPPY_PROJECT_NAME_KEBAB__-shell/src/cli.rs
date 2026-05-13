use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};

use crate::commands;

/// `__ZAPPY_PROJECT_NAME__` CLI parser.
#[derive(Debug, Parser)]
#[command(name = "__ZAPPY_PROJECT_NAME_KEBAB__")]
#[command(version, about)]
pub struct Cli {
    /// Print a greeting.
    #[command(subcommand)]
    pub command: Command,
}

/// `__ZAPPY_PROJECT_NAME__` commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Print a greeting.
    #[command(alias = "g")]
    Greet(GreetArgs),

    /// New command stub,
    NewCommand,
}

/// `--__ZAPPY_PROJECT_NAME__` `greet` command args.
#[derive(Debug, Args, Clone)]
pub struct GreetArgs {
    /// Your name.
    #[arg(short = 'n', long)]
    pub name: Option<String>,
}

/// Runs `__ZAPPY_PROJECT_NAME__` CLI.
#[must_use]
pub fn run() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::Greet(args) => commands::greet(args),
        Command::NewCommand => {
            eprintln!("Command unknown or not implemented yet");
            ExitCode::FAILURE
        }
    }
}
