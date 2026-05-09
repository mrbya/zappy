use std::path::{Path, PathBuf};

use clap::Parser;
use zappy_core::{VariableValue, parse_variable_overrides};

use crate::cli::{Cli, Command, CreateArgs, InfoArgs, InitArgs, ListArgs, NewArgs, ValidateArgs};

#[test]
fn parses_verbosity_count() {
    let cli = Cli::try_parse_from(["zappy", "-vvv", "list"]).expect("verbosity flags should parse");

    assert_eq!(cli.verbose, 3);
    assert!(matches!(cli.command, Command::List(_)));
}

#[test]
fn parses_list_command() {
    let cli = Cli::try_parse_from([
        "zappy",
        "list",
        "--templates-dir",
        "templates",
        "--language",
        "rust",
    ])
    .expect("list command should parse");

    let Command::List(args) = cli.command else {
        panic!("expected list command");
    };

    assert_eq!(args.templates_dir.as_deref(), Some(Path::new("templates")));
    assert_eq!(args.language.as_deref(), Some("rust"));
}

#[test]
fn parses_list_alias() {
    let cli = Cli::try_parse_from(["zappy", "ls"]).expect("list alias should parse");

    assert!(matches!(cli.command, Command::List(_)));
}

#[test]
fn parses_info_command() {
    let cli = Cli::try_parse_from([
        "zappy",
        "info",
        "--template",
        "rust-cli",
        "--templates-dir",
        "templates",
    ])
    .expect("info command should parse");

    let Command::Info(args) = cli.command else {
        panic!("expected info command");
    };

    assert_eq!(args.template, "rust-cli");
    assert_eq!(args.templates_dir.as_deref(), Some(Path::new("templates")));
}

#[test]
fn parses_info_alias() {
    let cli =
        Cli::try_parse_from(["zappy", "i", "-t", "rust-cli"]).expect("info alias should parse");

    let Command::Info(args) = cli.command else {
        panic!("expected info command");
    };

    assert_eq!(args.template, "rust-cli");
}

#[test]
fn parses_new_command_with_all_flags() {
    let cli = Cli::try_parse_from([
        "zappy",
        "new",
        "--template",
        "rust-cli",
        "--name",
        "my-tool",
        "--output",
        "out",
        "--var",
        "description=My CLI",
        "--var",
        "license=MIT",
        "--templates-dir",
        "templates",
        "--dry-run",
        "--non-interactive",
        "--force",
    ])
    .expect("new command should parse");

    let Command::New(args) = cli.command else {
        panic!("expected new command");
    };

    assert_eq!(args.template, "rust-cli");
    assert_eq!(args.project_name, "my-tool");
    assert_eq!(args.output.as_deref(), Some(Path::new("out")));
    assert_eq!(
        args.vars,
        ["description=My CLI".to_owned(), "license=MIT".to_owned(),],
    );
    assert_eq!(args.templates_dir.as_deref(), Some(Path::new("templates")));
    assert!(args.dry_run);
    assert!(args.non_interactive);
    assert!(args.force);
}

#[test]
fn parses_new_short_flags() {
    let cli = Cli::try_parse_from([
        "zappy",
        "new",
        "-t",
        "rust-cli",
        "-n",
        "my-tool",
        "-o",
        "out",
        "-a",
        "license=MIT",
        "-i",
        "templates",
        "-d",
        "-x",
        "-f",
    ])
    .expect("new command short flags should parse");

    let Command::New(args) = cli.command else {
        panic!("expected new command");
    };

    assert_eq!(args.template, "rust-cli");
    assert_eq!(args.project_name, "my-tool");
    assert_eq!(args.output.as_deref(), Some(Path::new("out")));
    assert_eq!(args.vars, ["license=MIT".to_owned()]);
    assert_eq!(args.templates_dir.as_deref(), Some(Path::new("templates")));
    assert!(args.dry_run);
    assert!(args.non_interactive);
    assert!(args.force);
}

#[test]
fn parses_new_alias() {
    let cli = Cli::try_parse_from(["zappy", "n", "-t", "rust-cli", "-n", "my-tool"])
        .expect("new alias should parse");

    assert!(matches!(cli.command, Command::New(_)));
}

#[test]
fn rejects_new_without_template() {
    let result = Cli::try_parse_from(["zappy", "new", "--name", "my-tool"]);

    assert!(result.is_err());
}

#[test]
fn rejects_new_without_project_name() {
    let result = Cli::try_parse_from(["zappy", "new", "--template", "rust-cli"]);

    assert!(result.is_err());
}

#[test]
fn parses_validate_command() {
    let cli = Cli::try_parse_from([
        "zappy",
        "validate",
        "--template",
        "rust-cli",
        "--templates-dir",
        "templates",
        "--keep-temp",
    ])
    .expect("validate command should parse");

    let Command::Validate(args) = cli.command else {
        panic!("expected validate command");
    };

    assert_eq!(args.template, "rust-cli");
    assert_eq!(args.templates_dir.as_deref(), Some(Path::new("templates")));
    assert!(args.keep_temp);
}

#[test]
fn parses_validate_alias() {
    let cli = Cli::try_parse_from(["zappy", "val", "-t", "rust-cli"])
        .expect("validate alias should parse");

    assert!(matches!(cli.command, Command::Validate(_)));
}

#[test]
fn rejects_validate_without_template() {
    let result = Cli::try_parse_from(["zappy", "validate"]);

    assert!(result.is_err());
}

#[test]
fn parses_init_command() {
    let cli = Cli::try_parse_from(["zappy", "init", "--output", "templates/rust-cli"])
        .expect("init command should parse");

    let Command::Init(args) = cli.command else {
        panic!("expected init command");
    };

    assert_eq!(args.output, PathBuf::from("templates/rust-cli"));
}

#[test]
fn parses_init_alias() {
    let cli = Cli::try_parse_from(["zappy", "it", "-o", "templates/rust-cli"])
        .expect("init alias should parse");

    assert!(matches!(cli.command, Command::Init(_)));
}

#[test]
fn rejects_init_without_output_path() {
    let result = Cli::try_parse_from(["zappy", "init"]);

    assert!(result.is_err());
}

#[test]
fn parses_create_command_from_existing_project() {
    let cli = Cli::try_parse_from([
        "zappy",
        "create",
        "--from",
        "existing-project",
        "--template",
        "rust-cli",
        "--description",
        "Rust CLI template",
        "--var",
        "project_name=my-tool",
        "--var",
        "license=MIT",
    ])
    .expect("create command should parse");

    let Command::Create(args) = cli.command else {
        panic!("expected create command");
    };

    assert_eq!(args.from.as_deref(), Some(Path::new("existing-project")));
    assert_eq!(args.template.as_deref(), Some("rust-cli"));
    assert_eq!(args.description.as_deref(), Some("Rust CLI template"));
    assert_eq!(
        args.vars,
        ["project_name=my-tool".to_owned(), "license=MIT".to_owned(),],
    );
    assert!(!args.empty);
}

#[test]
fn parses_create_command_empty_template() {
    let cli = Cli::try_parse_from([
        "zappy",
        "create",
        "--empty",
        "--template",
        "rust-cli",
        "--description",
        "Rust CLI template",
    ])
    .expect("create empty command should parse");

    let Command::Create(args) = cli.command else {
        panic!("expected create command");
    };

    assert!(args.empty);
    assert_eq!(args.template.as_deref(), Some("rust-cli"));
    assert_eq!(args.description.as_deref(), Some("Rust CLI template"));
}

#[test]
fn parses_create_alias() {
    let cli = Cli::try_parse_from(["zappy", "c", "-e", "-t", "rust-cli"])
        .expect("create alias should parse");

    assert!(matches!(cli.command, Command::Create(_)));
}

#[test]
fn rejects_missing_subcommand() {
    let result = Cli::try_parse_from(["zappy"]);

    assert!(result.is_err());
}

#[test]
fn rejects_unknown_subcommand() {
    let result = Cli::try_parse_from(["zappy", "generate"]);

    assert!(result.is_err());
}

#[test]
fn command_args_are_cloneable() {
    fn assert_clone<T: Clone>() {}

    assert_clone::<ListArgs>();
    assert_clone::<InfoArgs>();
    assert_clone::<NewArgs>();
    assert_clone::<ValidateArgs>();
    assert_clone::<InitArgs>();
    assert_clone::<CreateArgs>();
}

#[test]
fn parses_cli_variable_overrides() {
    let overrides = parse_variable_overrides(["project_name=my-tool", "use_ci=true", "retries=3"])
        .expect("overrides should parse");

    assert_eq!(
        overrides.get("project_name"),
        Some(&VariableValue::String(String::from("my-tool"))),
    );
    assert_eq!(overrides.get("use_ci"), Some(&VariableValue::Bool(true)));
    assert_eq!(overrides.get("retries"), Some(&VariableValue::Integer(3)));
}

#[test]
fn rejects_cli_variable_override_without_equals() {
    let err = parse_variable_overrides(["project_name"])
        .expect_err("override without equals should fail");

    assert!(err.to_string().contains("key=value"));
}

#[test]
fn rejects_cli_variable_override_with_empty_key() {
    let err =
        parse_variable_overrides(["=my-tool"]).expect_err("override with empty key should fail");

    assert!(err.to_string().contains("key=value"));
}

#[test]
fn new_rejects_invalid_variable_override() {
    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "new",
            "--template",
            "rust-cli",
            "--name",
            "my-tool",
            "--var",
            "bad",
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("key=value"));
}
