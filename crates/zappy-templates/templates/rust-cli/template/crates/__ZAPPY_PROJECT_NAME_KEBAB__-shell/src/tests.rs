use clap::Parser;

use crate::cli::{Cli, Command};

#[test]
fn parse_greet_command() {
    let cli = Cli::try_parse_from(["__ZAPPY_PROJECT_NAME_KEBAB__", "greet", "--name", "BruceLee"])
        .expect("greet command should parse");

    assert!(matches!(cli.command, Command::Greet(_)));

    let Command::Greet(args) = cli.command else {
        panic!("expected greet command");
    };

    assert!(
        args.name
            .expect("greet name should parse")
            .contains("BruceLee")
    );
}
