use super::*;

const MINIMAL_MANIFEST: &str = r#"
[template]
id = "rust-cli"
name = "Rust CLI Application"
description = "Small Rust CLI application."
language = "rust"
version = "0.1.0"

[variables.test_project_name]
prompt = "Project name"
default = "my-cli"
required = true

[variables.description]
prompt = "Project description"
default = "A small Rust CLI application."
"#;

#[test]
fn parses_minimal_manifest() {
    let manifest =
        Manifest::from_toml_str(MINIMAL_MANIFEST, "zappy.toml").expect("manifest should parse");

    assert_eq!(manifest.template.id.as_str(), "rust-cli");
    assert_eq!(manifest.template.name, "Rust CLI Application");
    assert_eq!(manifest.template.source.root.as_str(), "template");
    assert!(manifest.variables.contains_key("test_project_name"));
    assert!(manifest.variables.contains_key("description"));
}

#[test]
fn rejects_empty_template_id() {
    let source = r#"
[template]
id = ""
name = "Broken"
"#;

    let err =
        Manifest::from_toml_str(source, "zappy.toml").expect_err("empty template id should fail");

    assert!(err.to_string().contains("template.id"));
}

#[test]
fn rejects_empty_template_name() {
    let source = r#"
[template]
id = "broken"
name = ""
"#;

    let err =
        Manifest::from_toml_str(source, "zappy.toml").expect_err("empty template name should fail");

    assert!(err.to_string().contains("template.name"));
}

#[test]
fn defaults_source_root_to_template() {
    let manifest =
        Manifest::from_toml_str(MINIMAL_MANIFEST, "zappy.toml").expect("manifest should parse");

    assert_eq!(manifest.template.source.root.as_str(), "template");
}

#[test]
fn rejects_absolute_source_root() {
    let source = r#"
[template]
id = "broken"
name = "Broken"

[template.source]
root = "/tmp/template"
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("absolute source root should fail");

    assert!(err.to_string().contains("template.source.root"));
}

#[test]
fn parses_paths_config() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[paths]
exclude = [".git", "target"]
binary_extensions = ["png", "jpg"]
binary_files = ["Cargo.lock"]
"#;

    let manifest = Manifest::from_toml_str(source, "zappy.toml").expect("manifest should parse");

    assert_eq!(manifest.paths.exclude, [".git", "target"]);
    assert_eq!(manifest.paths.binary_extensions, ["png", "jpg"]);
    assert_eq!(manifest.paths.binary_files, ["Cargo.lock"]);
}

#[test]
fn parses_hooks_and_validation_steps() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[[hooks.post_generate]]
name = "Format"
command = "cargo"
args = ["fmt"]
optional = true

[variables.project_variable]

[validation.variables]
project_variable = "zappy-test-cli"

[[validation.steps]]
name = "Test"
command = "cargo"
args = ["test"]
"#;

    let manifest = Manifest::from_toml_str(source, "zappy.toml").expect("manifest should parse");

    assert_eq!(manifest.hooks.post_generate.len(), 1);

    let validation = manifest.validation.expect("validation should exist");
    assert_eq!(validation.steps.len(), 1);
    assert_eq!(
        validation
            .steps
            .first()
            .expect("element 0 should be populated")
            .command,
        "cargo"
    );
}

#[test]
fn rejects_empty_hook_command() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[[hooks.post_generate]]
command = ""
"#;

    let err =
        Manifest::from_toml_str(source, "zappy.toml").expect_err("empty hook command should fail");

    assert!(err.to_string().contains("hook command"));
}

#[test]
fn rejects_validation_variable_not_declared_in_template_variables() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[variables.test_project_name]
default = "my-cli"

[validation.variables]
project_nmae = "zappy-test-cli"
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("unknown validation variable should fail");

    assert!(
        err.to_string()
            .contains("validation.variables.project_nmae")
    );
}

#[test]
fn rejects_variable_name_with_hyphen() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[variables.project-name]
default = "my-cli"
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("variable name with hyphen should fail");

    assert!(err.to_string().contains("project-name"));
    assert!(err.to_string().contains("must not contain `-`"));
}

#[test]
fn rejects_empty_variable_placeholder() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[variables.test_project_name]
default = "my-cli"

[variables.test_project_name.placeholders]
raw = ""
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("empty variable placeholder should fail");

    assert!(err.to_string().contains("placeholder"));
    assert!(err.to_string().contains("must not be empty"));
}

#[test]
fn rejects_empty_variable_prompt() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[variables.test_project_name]
prompt = ""
default = "my-cli"
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("empty variable prompt should fail");

    assert!(err.to_string().contains("variable prompt"));
    assert!(err.to_string().contains("must not be empty"));
}

#[test]
fn rejects_parent_component_in_conditional_path() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[[conditionals]]
path = "../Dockerfile"
when = "use_docker"
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("conditional path with parent component should fail");

    assert!(err.to_string().contains("conditional path"));
    assert!(err.to_string().contains("must not contain `..`"));
}

#[test]
fn rejects_empty_validation_output_dir_name() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[validation]
output_dir_name = ""
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("empty validation output dir name should fail");

    assert!(err.to_string().contains("validation.output_dir_name"));
    assert!(err.to_string().contains("must not be empty"));
}

#[test]
fn parses_transform_placeholders() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[variables.test_project_name]
prompt = "Project name"
default = "my-cli"
transforms = ["raw", "kebab", "snake", "pascal", "camel", "screaming_snake", "upper", "lower"]

[variables.test_project_name.placeholders]
raw = "__ZAPPY_test_project_name__"
kebab = "__ZAPPY_test_project_name_KEBAB__"
snake = "__ZAPPY_test_project_name_SNAKE__"
pascal = "__ZAPPY_test_project_name_PASCAL__"
camel = "__ZAPPY_test_project_name_CAMEL__"
screaming_snake = "__ZAPPY_test_project_name_SCREAMING_SNAKE__"
upper = "__ZAPPY_test_project_name_UPPER__"
lower = "__ZAPPY_test_project_name_LOWER__"
"#;

    let manifest = Manifest::from_toml_str(source, "zappy.toml")
        .expect("manifest with transform placeholders should parse");

    let test_project_name = manifest
        .variables
        .get("test_project_name")
        .expect("test_project_name variable should exist");

    assert_eq!(
        test_project_name.transforms.as_slice(),
        &[
            crate::variables::TransformKind::Raw,
            crate::variables::TransformKind::Kebab,
            crate::variables::TransformKind::Snake,
            crate::variables::TransformKind::Pascal,
            crate::variables::TransformKind::Camel,
            crate::variables::TransformKind::ScreamingSnake,
            crate::variables::TransformKind::Upper,
            crate::variables::TransformKind::Lower,
        ],
    );

    assert_eq!(
        test_project_name
            .placeholders
            .get(&crate::variables::TransformKind::Raw),
        Some(&"__ZAPPY_test_project_name__".to_owned()),
    );
    assert_eq!(
        test_project_name
            .placeholders
            .get(&crate::variables::TransformKind::Kebab),
        Some(&"__ZAPPY_test_project_name_KEBAB__".to_owned()),
    );
    assert_eq!(
        test_project_name
            .placeholders
            .get(&crate::variables::TransformKind::ScreamingSnake),
        Some(&"__ZAPPY_test_project_name_SCREAMING_SNAKE__".to_owned()),
    );
}

#[test]
fn rejects_invalid_transform_name() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[variables.test_project_name]
default = "my-cli"
transforms = ["raw", "wat_case"]
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("invalid transform name should fail");

    assert!(matches!(err, crate::error::CoreError::ParseManifest { .. }));
}

#[test]
fn applies_string_transforms() {
    let value = VariableValue::String(String::from("my cool tool"));

    assert_eq!(apply_transform(&value, TransformKind::Raw), "my cool tool");
    assert_eq!(
        apply_transform(&value, TransformKind::Kebab),
        "my-cool-tool"
    );
    assert_eq!(
        apply_transform(&value, TransformKind::Snake),
        "my_cool_tool"
    );
    assert_eq!(apply_transform(&value, TransformKind::Pascal), "MyCoolTool");
    assert_eq!(apply_transform(&value, TransformKind::Camel), "myCoolTool");
    assert_eq!(
        apply_transform(&value, TransformKind::ScreamingSnake),
        "MY_COOL_TOOL",
    );
    assert_eq!(
        apply_transform(&value, TransformKind::Upper),
        "MY COOL TOOL"
    );
    assert_eq!(
        apply_transform(&value, TransformKind::Lower),
        "my cool tool"
    );
}

const VARIABLE_MANIFEST: &str = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[variables.test_project_name]
required = true
default = "template-default"
transforms = ["raw", "kebab", "snake", "pascal"]

[variables.test_project_name.placeholders]
raw = "__ZAPPY_test_project_name__"
kebab = "__ZAPPY_test_project_name_KEBAB__"
snake = "__ZAPPY_test_project_name_SNAKE__"
pascal = "__ZAPPY_test_project_name_PASCAL__"

[variables.license]
default = "MIT"
choices = ["MIT", "Apache-2.0"]

[variables.use_ci]
default = true
"#;

#[test]
fn resolves_template_defaults() {
    let manifest =
        Manifest::from_toml_str(VARIABLE_MANIFEST, "zappy.toml").expect("manifest should parse");

    let resolved = resolve_variables(&manifest.variables, &VariableResolutionInput::default())
        .expect("variables should resolve");

    assert_eq!(
        resolved.values.get("test_project_name"),
        Some(&VariableValue::String(String::from("template-default"))),
    );
    assert_eq!(
        resolved.values.get("license"),
        Some(&VariableValue::String(String::from("MIT"))),
    );
    assert_eq!(
        resolved.values.get("use_ci"),
        Some(&VariableValue::Bool(true))
    );
}

#[test]
fn explicit_values_override_template_defaults() {
    let manifest =
        Manifest::from_toml_str(VARIABLE_MANIFEST, "zappy.toml").expect("manifest should parse");

    let mut explicit = VariableValueMap::new();
    explicit.insert(
        String::from("test_project_name"),
        VariableValue::String(String::from("my cool tool")),
    );

    let resolved = resolve_variables(
        &manifest.variables,
        &VariableResolutionInput {
            explicit,
            ..VariableResolutionInput::default()
        },
    )
    .expect("variables should resolve");

    assert_eq!(
        resolved.values.get("test_project_name"),
        Some(&VariableValue::String(String::from("my cool tool"))),
    );
    assert_eq!(
        resolved
            .replacements
            .get("__ZAPPY_test_project_name_KEBAB__"),
        Some(&String::from("my-cool-tool")),
    );
    assert_eq!(
        resolved
            .replacements
            .get("__ZAPPY_test_project_name_SNAKE__"),
        Some(&String::from("my_cool_tool")),
    );
    assert_eq!(
        resolved
            .replacements
            .get("__ZAPPY_test_project_name_PASCAL__"),
        Some(&String::from("MyCoolTool")),
    );
}

#[test]
fn user_defaults_are_lower_priority_than_template_defaults() {
    let manifest =
        Manifest::from_toml_str(VARIABLE_MANIFEST, "zappy.toml").expect("manifest should parse");

    let mut user_defaults = VariableValueMap::new();
    user_defaults.insert(
        String::from("license"),
        VariableValue::String(String::from("Apache-2.0")),
    );

    let resolved = resolve_variables(
        &manifest.variables,
        &VariableResolutionInput {
            user_defaults,
            ..VariableResolutionInput::default()
        },
    )
    .expect("variables should resolve");

    assert_eq!(
        resolved.values.get("license"),
        Some(&VariableValue::String(String::from("MIT"))),
    );
}

#[test]
fn builtins_can_fill_declared_variables() {
    let manifest = Manifest::from_toml_str(
        r#"
[template]
id = "builtin-test"
name = "Builtin Test"

[variables.test_project_name]
required = true
prompt = "Project name"
"#,
        "zappy.toml",
    )
    .expect("manifest should parse");

    let mut builtins = VariableValueMap::new();
    builtins.insert(
        String::from("test_project_name"),
        VariableValue::String(String::from("builtin-project")),
    );

    let resolved = resolve_variables(
        &manifest.variables,
        &VariableResolutionInput {
            builtins,
            ..VariableResolutionInput::default()
        },
    )
    .expect("variables should resolve");

    assert_eq!(
        resolved.values.get("test_project_name"),
        Some(&VariableValue::String(String::from("builtin-project"))),
    );
}

#[test]
fn missing_required_variable_fails() {
    let manifest = Manifest::from_toml_str(
        r#"
[template]
id = "missing-test"
name = "Missing Test"

[variables.test_project_name]
required = true
prompt = "Project name"
"#,
        "zappy.toml",
    )
    .expect("manifest should parse");

    let err = resolve_variables(&manifest.variables, &VariableResolutionInput::default())
        .expect_err("missing required variable should fail");

    assert!(err.to_string().contains("test_project_name"));
}

#[test]
fn unknown_explicit_variable_fails() {
    let manifest =
        Manifest::from_toml_str(VARIABLE_MANIFEST, "zappy.toml").expect("manifest should parse");

    let mut explicit = VariableValueMap::new();
    explicit.insert(
        String::from("unknown"),
        VariableValue::String(String::from("value")),
    );

    let err = resolve_variables(
        &manifest.variables,
        &VariableResolutionInput {
            explicit,
            ..VariableResolutionInput::default()
        },
    )
    .expect_err("unknown explicit variable should fail");

    assert!(err.to_string().contains("unknown variable"));
    assert!(err.to_string().contains("unknown"));
}

#[test]
fn invalid_choice_fails() {
    let manifest =
        Manifest::from_toml_str(VARIABLE_MANIFEST, "zappy.toml").expect("manifest should parse");

    let mut explicit = VariableValueMap::new();
    explicit.insert(
        String::from("license"),
        VariableValue::String(String::from("GPL-3.0")),
    );

    let err = resolve_variables(
        &manifest.variables,
        &VariableResolutionInput {
            explicit,
            ..VariableResolutionInput::default()
        },
    )
    .expect_err("invalid choice should fail");

    assert!(err.to_string().contains("license"));
    assert!(err.to_string().contains("GPL-3.0"));
}
