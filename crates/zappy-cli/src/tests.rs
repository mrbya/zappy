use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;
use zappy_core::{VariableValue, parse_variable_overrides};

use crate::cli::{Cli, Command, CreateArgs, InfoArgs, InitArgs, ListArgs, NewArgs, ValidateArgs};
use crate::diagnostics::DiagnosticReport;
use crate::tracing::default_filter;

fn write_cli_template(templates_root: &Path, dir_name: &str, manifest: &str) -> PathBuf {
    let template_dir = templates_root.join(dir_name);
    let source_dir = template_dir.join("template");

    fs::create_dir_all(&source_dir).expect("template source dir should be created");
    fs::write(template_dir.join("zappy.toml"), manifest).expect("manifest should be written");
    fs::write(source_dir.join("README.md"), "# __NAME__\n")
        .expect("template file should be written");

    template_dir
}

#[test]
fn parses_verbosity_count() {
    let cli = Cli::try_parse_from(["zappy", "-vvv", "list"]).expect("verbosity flags should parse");

    assert_eq!(cli.verbose, 3);
    assert!(matches!(cli.command, Command::List(_)));
}

#[test]
fn parses_clear_flag() {
    let cli = Cli::try_parse_from(["zappy", "-c", "list"]).expect("clear flag should parse");

    assert!(cli.clear);
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
    assert!(!args.no_hooks);
}

#[test]
fn parses_new_no_hooks_flag() {
    let cli = Cli::try_parse_from([
        "zappy",
        "new",
        "--template",
        "rust-cli",
        "--name",
        "my-tool",
        "--no-hooks",
    ])
    .expect("new no-hooks flag should parse");

    let Command::New(args) = cli.command else {
        panic!("expected new command");
    };

    assert!(args.no_hooks);
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
fn rejects_new_without_test_project_name() {
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
    assert!(!args.no_hooks);
}

#[test]
fn parses_validate_no_hooks_flag() {
    let cli = Cli::try_parse_from(["zappy", "validate", "-t", "rust-cli", "--no-hooks"])
        .expect("validate no-hooks flag should parse");

    let Command::Validate(args) = cli.command else {
        panic!("expected validate command");
    };

    assert!(args.no_hooks);
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
    let cli = Cli::try_parse_from([
        "zappy",
        "init",
        "--output",
        "templates/rust-cli",
        "--template",
        "rust-cli",
        "--name",
        "Rust CLI",
        "--description",
        "Rust CLI template",
        "--force",
    ])
    .expect("init command should parse");

    let Command::Init(args) = cli.command else {
        panic!("expected init command");
    };

    assert_eq!(args.output, PathBuf::from("templates/rust-cli"));
    assert_eq!(args.template.as_deref(), Some("rust-cli"));
    assert_eq!(args.name.as_deref(), Some("Rust CLI"));
    assert_eq!(args.description.as_deref(), Some("Rust CLI template"));
    assert!(args.force);
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
        "test_project_name=my-tool",
        "--var",
        "license=MIT",
        "-o",
        "output_dir",
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
        [
            "test_project_name=my-tool".to_owned(),
            "license=MIT".to_owned(),
        ],
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
        "--name",
        "Rust CLI",
        "--force",
        "-o",
        "output_dir",
    ])
    .expect("create empty command should parse");

    let Command::Create(args) = cli.command else {
        panic!("expected create command");
    };

    assert!(args.empty);
    assert_eq!(args.template.as_deref(), Some("rust-cli"));
    assert_eq!(args.name.as_deref(), Some("Rust CLI"));
    assert_eq!(args.description.as_deref(), Some("Rust CLI template"));
    assert!(args.force);
}

#[test]
fn parses_create_alias() {
    let cli = Cli::try_parse_from(["zappy", "c", "-e", "-t", "rust-cli", "-o", "output_dir"])
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
    let overrides =
        parse_variable_overrides(["test_project_name=my-tool", "use_ci=true", "retries=3"])
            .expect("overrides should parse");

    assert_eq!(
        overrides.get("test_project_name"),
        Some(&VariableValue::String(String::from("my-tool"))),
    );
    assert_eq!(overrides.get("use_ci"), Some(&VariableValue::Bool(true)));
    assert_eq!(overrides.get("retries"), Some(&VariableValue::Integer(3)));
}

#[test]
fn rejects_cli_variable_override_without_equals() {
    let err = parse_variable_overrides(["test_project_name"])
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
fn command_builtins_include_project_name_and_dates() {
    let builtins = crate::commands::helpers::command_builtins(String::from("my-tool"));

    assert_eq!(
        builtins.get(zappy_core::builtins::PROJECT_NAME),
        Some(&VariableValue::String(String::from("my-tool"))),
    );

    for name in [
        zappy_core::builtins::USER,
        zappy_core::builtins::EMAIL,
        zappy_core::builtins::DATE,
        zappy_core::builtins::DAY,
        zappy_core::builtins::MONTH,
        zappy_core::builtins::YEAR,
    ] {
        assert!(builtins.contains_key(name));
    }
}

#[test]
fn discovery_config_disables_bundled_templates_for_explicit_dir() {
    let config = crate::commands::helpers::discovery_config(Some(PathBuf::from("templates")));

    assert_eq!(config.templates_dir, Some(PathBuf::from("templates")));
    assert_eq!(config.bundled_templates_dir, None);
}

#[test]
fn list_command_succeeds_for_empty_explicit_directory() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

    let result = crate::commands::list(&ListArgs {
        templates_dir: Some(temp_dir.path().to_path_buf()),
        language: None,
    });

    assert_eq!(result, ExitCode::SUCCESS);
}

#[test]
fn list_command_fails_for_missing_explicit_directory() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

    let result = crate::commands::list(&ListArgs {
        templates_dir: Some(temp_dir.path().join("missing")),
        language: None,
    });

    assert_eq!(result, ExitCode::FAILURE);
}

#[test]
fn info_command_fails_for_unknown_template() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

    let result = crate::commands::info(&InfoArgs {
        template: String::from("missing"),
        templates_dir: Some(temp_dir.path().to_path_buf()),
    });

    assert_eq!(result, ExitCode::FAILURE);
}

#[test]
fn init_command_creates_template_skeleton() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let output = temp_dir.path().join("rust-cli");

    let result = crate::commands::init_template(&InitArgs {
        output: output.clone(),
        template: Some(String::from("rust-cli")),
        name: Some(String::from("Rust CLI")),
        description: Some(String::from("Rust CLI template")),
        force: false,
    });

    assert_eq!(result, ExitCode::SUCCESS);
    assert!(output.join("zappy.toml").exists());
}

#[test]
fn init_command_fails_for_existing_template_without_force() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let output = temp_dir.path().join("rust-cli");
    let args = InitArgs {
        output,
        template: Some(String::from("rust-cli")),
        name: None,
        description: None,
        force: false,
    };

    assert_eq!(crate::commands::init_template(&args), ExitCode::SUCCESS);
    assert_eq!(crate::commands::init_template(&args), ExitCode::FAILURE);
}

#[test]
fn create_empty_command_creates_template_skeleton() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let output = temp_dir.path().join("rust-cli");

    let result = crate::commands::create(&CreateArgs {
        from: None,
        output: output.clone(),
        empty: true,
        template: Some(String::from("rust-cli")),
        name: Some(String::from("Rust CLI")),
        description: Some(String::from("Rust CLI template")),
        vars: Vec::new(),
        force: false,
    });

    assert_eq!(result, ExitCode::SUCCESS);
    assert!(output.join("zappy.toml").exists());
}

#[test]
fn create_non_empty_stub_returns_success() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

    let result = crate::commands::create(&CreateArgs {
        from: Some(temp_dir.path().join("project")),
        output: temp_dir.path().join("template"),
        empty: false,
        template: Some(String::from("rust-cli")),
        name: None,
        description: None,
        vars: vec![String::from("name=value")],
        force: false,
    });

    assert_eq!(result, ExitCode::SUCCESS);
}

#[test]
fn new_command_fails_for_invalid_variable_override() {
    let result = crate::commands::new(&NewArgs {
        template: String::from("rust-cli"),
        project_name: String::from("my-tool"),
        output: None,
        vars: vec![String::from("invalid")],
        templates_dir: None,
        dry_run: false,
        non_interactive: false,
        force: false,
        no_hooks: false,
    });

    assert_eq!(result, ExitCode::FAILURE);
}

#[test]
fn new_command_fails_for_unknown_template_in_explicit_dir() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

    let result = crate::commands::new(&NewArgs {
        template: String::from("missing"),
        project_name: String::from("my-tool"),
        output: None,
        vars: Vec::new(),
        templates_dir: Some(temp_dir.path().to_path_buf()),
        dry_run: true,
        non_interactive: false,
        force: false,
        no_hooks: false,
    });

    assert_eq!(result, ExitCode::FAILURE);
}

#[test]
fn new_command_dry_run_succeeds_with_explicit_template() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("out");

    write_cli_template(
        temp_dir.path(),
        "rust-cli",
        r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[variables.name]
default = "my-tool"

[variables.name.placeholders]
raw = "__NAME__"
"#,
    );

    let result = crate::commands::new(&NewArgs {
        template: String::from("rust-cli"),
        project_name: String::from("my-tool"),
        output: Some(output_dir.clone()),
        vars: Vec::new(),
        templates_dir: Some(temp_dir.path().to_path_buf()),
        dry_run: true,
        non_interactive: false,
        force: false,
        no_hooks: false,
    });

    assert_eq!(result, ExitCode::SUCCESS);
    assert!(!output_dir.exists());
}

#[test]
fn validate_command_fails_without_validation_config() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

    write_cli_template(
        temp_dir.path(),
        "rust-cli",
        r#"
[template]
id = "rust-cli"
name = "Rust CLI"
"#,
    );

    let result = crate::commands::validate(&ValidateArgs {
        template: String::from("rust-cli"),
        templates_dir: Some(temp_dir.path().to_path_buf()),
        keep_temp: false,
        no_hooks: false,
    });

    assert_eq!(result, ExitCode::FAILURE);
}

#[test]
fn formats_error_with_details_and_hints() {
    let report = DiagnosticReport::error("template `foo` was not found")
        .detail("Searched:")
        .detail("  bundled templates: /tmp/templates")
        .hint("run `zappy list` to see available templates");

    let rendered = report.to_string();

    assert!(rendered.contains("Error: template `foo` was not found"));
    assert!(rendered.contains("Searched:"));
    assert!(rendered.contains("bundled templates"));
    assert!(rendered.contains("Hint:"));
    assert!(rendered.contains("zappy list"));
}

#[test]
fn maps_zero_verbosity_to_warn() {
    let filter = default_filter(0);

    assert!(filter.contains("zappy_cli=warn"));
}

#[test]
fn maps_single_verbosity_to_info() {
    let filter = default_filter(1);

    assert!(filter.contains("zappy_cli=info"));
}

#[test]
fn maps_double_verbosity_to_debug() {
    let filter = default_filter(2);

    assert!(filter.contains("zappy_cli=debug"));
}

#[test]
fn maps_triple_verbosity_to_trace() {
    let filter = default_filter(3);

    assert!(filter.contains("zappy_cli=trace"));
}
