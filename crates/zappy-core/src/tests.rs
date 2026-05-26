use indexmap::IndexMap;

use super::*;
use crate::variables::VariableMap;

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

#[test]
fn required_when_unknown_variable_fails() {
    Manifest::from_toml_str(
        r#"
[template]
id = "unknown-required-when-test"
name = "Unknown required_when"

[variables.use_test]
default = true

[variables.test_project_name]
required_when = "unknown"
prompt = "Project name"
"#,
        "zappy.toml",
    )
    .expect_err("should not parse with unknown required_when variable");
}

#[test]
fn required_when_self_fails() {
    Manifest::from_toml_str(
        r#"
[template]
id = "required-when-self-test"
name = "required_when self"

[variables.use_test]
default = true

[variables.test_project_name]
required_when = "test_project_name"
prompt = "Project name"
"#,
        "zappy.toml",
    )
    .expect_err("should not parse with required_when=self");
}

#[test]
fn required_when_false_allows_missing() {
    let manifest = Manifest::from_toml_str(
        r#"
[template]
id = "required-when"
name = "Required when test"

[variables.use_test]
default = false

[variables.test_project_name]
required_when = "use_test"
prompt = "Project name"
"#,
        "zappy.toml",
    )
    .expect("manifest should parse");

    resolve_variables(&manifest.variables, &VariableResolutionInput::default())
        .expect("missing conditionally required variable with required_when = false should pass");
}

#[test]
fn required_when_true_rejects_missing() {
    let manifest = Manifest::from_toml_str(
        r#"
[template]
id = "required-when"
name = "Required when test"

[variables.use_test]
default = true

[variables.test_project_name]
required_when = "use_test"
prompt = "Project name"
"#,
        "zappy.toml",
    )
    .expect("manifest should parse");

    let err = resolve_variables(&manifest.variables, &VariableResolutionInput::default())
        .expect_err("missing conditionally required variable should fail");

    assert!(err.to_string().contains("test_project_name"));
}

#[test]
fn required_when_true_rejects_empty_string() {
    let manifest = Manifest::from_toml_str(
        r#"
[template]
id = "required-when"
name = "Required when test"

[variables.use_test]
default = true

[variables.test_project_name]
required_when = "use_test"
prompt = "Project name"
"#,
        "zappy.toml",
    )
    .expect("manifest should parse");

    let mut input = VariableResolutionInput::default();
    input.interactive.insert(
        String::from("test_project_name"),
        VariableValue::String(String::new()),
    );

    let err = resolve_variables(&manifest.variables, &input)
        .expect_err("empty conditionally required variable should fail");

    assert!(err.to_string().contains("test_project_name"));
}

#[test]
fn required_when_true_accepts_value() {
    let manifest = Manifest::from_toml_str(
        r#"
[template]
id = "required-when"
name = "Required when test"

[variables.use_test]
default = true

[variables.test_project_name]
required_when = "use_test"
prompt = "Project name"
"#,
        "zappy.toml",
    )
    .expect("manifest should parse");

    let mut input = VariableResolutionInput::default();
    input.interactive.insert(
        String::from("test_project_name"),
        VariableValue::String(String::from("test_project")),
    );

    let resolved = resolve_variables(&manifest.variables, &input)
        .expect("conditionally required variable should accept a value");

    assert!(resolved.values.contains_key("test_project_name"));
    assert_eq!(
        resolved
            .values
            .get("test_project_name")
            .expect("should contain test_project_name"),
        &VariableValue::String(String::from("test_project"))
    );
}

#[test]
fn conflicts_with_unknown_fails() {
    Manifest::from_toml_str(
        r#"
[template]
id = "conflicts-with-unknown"
name = "Conflcits with unknown"

[variables.use_test1]
default = false
conflicts_with = ["unknown"]

[variables.use_test2]
default = false
"#,
        "zappy.toml",
    )
    .expect_err("should not parse with unknown conflicts_with variable");
}

#[test]
fn conflicts_with_self_fails() {
    Manifest::from_toml_str(
        r#"
[template]
id = "conflicts-with-self"
name = "Conflcits with self"

[variables.use_test1]
default = false
conflicts_with = ["use_test1"]

[variables.use_test2]
default = false
"#,
        "zappy.toml",
    )
    .expect_err("should not parse with unknown conflicts_with variable");
}

fn assert_conflicts_with(resolved: &ResolvedVariables, exp1: bool, exp2: bool) {
    assert!(resolved.values.contains_key("use_test1"));
    assert!(resolved.values.contains_key("use_test2"));
    assert_eq!(
        resolved
            .values
            .get("use_test1")
            .expect("should contain use_test1"),
        &VariableValue::Bool(exp1)
    );
    assert_eq!(
        resolved
            .values
            .get("use_test2")
            .expect("should contain use_test2"),
        &VariableValue::Bool(exp2)
    );
}

#[test]
fn conflicts_with_false_false_succeeds() {
    let manifest = Manifest::from_toml_str(
        r#"
[template]
id = "missing-test"
name = "Missing Test"

[variables.use_test1]
default = false
conflicts_with = ["use_test2"]

[variables.use_test2]
default = false
"#,
        "zappy.toml",
    )
    .expect("manifest should parse");

    let resolved = resolve_variables(&manifest.variables, &VariableResolutionInput::default())
        .expect("false false exclusive variables should pass");
    assert_conflicts_with(&resolved, false, false);
}

#[test]
fn conflicts_with_false_true_succeeds() {
    let manifest = Manifest::from_toml_str(
        r#"
[template]
id = "missing-test"
name = "Missing Test"

[variables.use_test1]
default = false
conflicts_with = ["use_test2"]

[variables.use_test2]
default = true
"#,
        "zappy.toml",
    )
    .expect("manifest should parse");

    let resolved = resolve_variables(&manifest.variables, &VariableResolutionInput::default())
        .expect("false true exclusive variables should pass");
    assert_conflicts_with(&resolved, false, true);
}

#[test]
fn conflicts_with_true_false_succeeds() {
    let manifest = Manifest::from_toml_str(
        r#"
[template]
id = "missing-test"
name = "Missing Test"

[variables.use_test1]
default = true
conflicts_with = ["use_test2"]

[variables.use_test2]
default = false
"#,
        "zappy.toml",
    )
    .expect("manifest should parse");

    let resolved = resolve_variables(&manifest.variables, &VariableResolutionInput::default())
        .expect("true false exclusive variables should pass");
    assert_conflicts_with(&resolved, true, false);
}

#[test]
fn conflicts_with_true_true_fails() {
    let manifest = Manifest::from_toml_str(
        r#"
[template]
id = "missing-test"
name = "Missing Test"

[variables.use_test1]
default = true
conflicts_with = ["use_test2"]

[variables.use_test2]
default = true
"#,
        "zappy.toml",
    )
    .expect("manifest should parse");

    let err = resolve_variables(&manifest.variables, &VariableResolutionInput::default())
        .expect_err("exclusive enabled variables should fail");

    assert!(err.to_string().contains("use_test1"));
    assert!(err.to_string().contains("use_test2"));
}

#[test]
fn renders_text_placeholders() {
    let mut replacements = indexmap::IndexMap::new();
    replacements.insert(String::from("__NAME__"), String::from("my-tool"));

    let rendered = crate::render::render_text("project = \"__NAME__\"", &replacements);

    assert_eq!(rendered, "project = \"my-tool\"");
}

#[test]
fn renders_relative_path_placeholders() {
    let mut replacements = indexmap::IndexMap::new();
    replacements.insert(String::from("__NAME__"), String::from("my_tool"));

    let rendered = crate::render::render_relative_path("src/__NAME__.rs", &replacements)
        .expect("path should render");

    assert_eq!(rendered, "src/my_tool.rs");
}

#[test]
fn rejects_rendered_parent_component_path() {
    let mut replacements = indexmap::IndexMap::new();
    replacements.insert(String::from("__BAD__"), String::from(".."));

    let err = crate::render::render_relative_path("src/__BAD__/main.rs", &replacements)
        .expect_err("parent component should fail");

    assert!(err.to_string().contains(".."));
}

#[test]
fn rejects_template_variable_with_builtin_name() {
    let source = r#"
[template]
id = "builtin-test"
name = "Builtin Test"

[variables.project_name]
default = "bad"
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("builtin variable names should be reserved");

    assert!(err.to_string().contains("reserved built-in variable"));
}

#[test]
fn rejects_cli_override_for_builtin_variable() {
    let err = parse_variable_overrides(["project_name=bad"])
        .expect_err("builtin variable override should fail");

    assert!(err.to_string().contains("reserved built-in variable"));
}

#[test]
fn resolves_builtin_project_name_without_manifest_variable() {
    let manifest = Manifest::from_toml_str(
        r#"
[template]
id = "builtin-test"
name = "Builtin Test"
"#,
        "zappy.toml",
    )
    .expect("manifest should parse");

    let mut builtins = VariableValueMap::new();
    builtins.insert(
        String::from(crate::builtins::PROJECT_NAME),
        VariableValue::String(String::from("my cool tool")),
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
        resolved.replacements.get("__ZAPPY_PROJECT_NAME__"),
        Some(&String::from("my cool tool")),
    );
    assert_eq!(
        resolved.replacements.get("__ZAPPY_PROJECT_NAME_KEBAB__"),
        Some(&String::from("my-cool-tool")),
    );
    assert_eq!(
        resolved.replacements.get("__ZAPPY_PROJECT_NAME_SNAKE__"),
        Some(&String::from("my_cool_tool")),
    );
    assert_eq!(
        resolved.replacements.get("__ZAPPY_PROJECT_NAME_PASCAL__"),
        Some(&String::from("MyCoolTool")),
    );
}

#[test]
fn rejects_template_id_starting_with_digit() {
    let source = r#"
[template]
id = "1rust-cli"
name = "Rust CLI"
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("template id starting with a digit should fail");

    assert!(err.to_string().contains("template.id"));
    assert!(err.to_string().contains("must start"));
}

#[test]
fn rejects_source_root_with_parent_component() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[template.source]
root = "../template"
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("source root with parent component should fail");

    assert!(err.to_string().contains("template.source.root"));
    assert!(err.to_string().contains("must not contain `..`"));
}

#[test]
fn rejects_empty_items_in_path_lists() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[paths]
exclude = ["target", " "]
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("empty path list item should fail");

    assert!(err.to_string().contains("paths.exclude"));
    assert!(err.to_string().contains("empty string entries"));
}

#[test]
fn load_manifest_from_missing_path_reports_read_error() {
    let err = Manifest::load_from_path("missing-zappy.toml")
        .expect_err("missing manifest path should fail");

    assert!(matches!(err, crate::CoreError::ReadManifest { .. }));
}

#[test]
fn rejects_empty_conditional_when_field() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[[conditionals]]
path = "optional.md"
when = " "
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("empty conditional when should fail");

    assert!(err.to_string().contains("when"));
    assert!(err.to_string().contains("must not be empty"));
}

#[test]
fn rejects_empty_source_root() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[template.source]
root = " "
"#;

    let err =
        Manifest::from_toml_str(source, "zappy.toml").expect_err("empty source root should fail");

    assert!(err.to_string().contains("template.source.root"));
    assert!(err.to_string().contains("must not be empty"));
}

#[test]
fn rejects_template_id_with_invalid_character() {
    let source = r#"
[template]
id = "rust.cli"
name = "Rust CLI"
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("template id with invalid character should fail");

    assert!(err.to_string().contains("template.id"));
    assert!(err.to_string().contains("may only contain"));
}

#[test]
fn rejects_empty_validation_setup_hook_command() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[validation]

[[validation.setup]]
command = " "
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("empty validation setup hook command should fail");

    assert!(err.to_string().contains("validation.setup[0]"));
    assert!(err.to_string().contains("hook command"));
}

#[test]
fn parses_validation_setup_and_teardown_hooks() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[variables.test_project_name]
default = "my-tool"

[validation]
output_dir_name = "validation-output"

[validation.variables]
test_project_name = "checked-tool"

[[validation.setup]]
name = "Setup"
command = "cargo"
args = ["fetch"]

[[validation.teardown]]
name = "Cleanup"
command = "cargo"
args = ["clean"]
optional = true
"#;

    let manifest = Manifest::from_toml_str(source, "zappy.toml")
        .expect("validation setup and teardown hooks should parse");

    let validation = manifest.validation.expect("validation should exist");

    assert_eq!(
        validation.output_dir_name.as_deref(),
        Some("validation-output")
    );
    assert_eq!(validation.setup.len(), 1);
    assert_eq!(validation.teardown.len(), 1);
    assert_eq!(
        validation
            .setup
            .first()
            .expect("should contain 1 element")
            .args,
        ["fetch"]
    );
    assert!(
        validation
            .teardown
            .first()
            .expect("should contain 1 element")
            .optional
    );
}

#[test]
fn rejects_invalid_hook_working_dir() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[[hooks.pre_generate]]
command = "cargo"
working_dir = "../outside"
"#;

    let err = Manifest::from_toml_str(source, "zappy.toml")
        .expect_err("hook working dir with parent component should fail");

    assert!(err.to_string().contains("hooks.pre_generate[0]"));
    assert!(err.to_string().contains("hook working dir"));
}

#[test]
fn parses_hook_env_when_and_shell_flags() {
    let source = r#"
[template]
id = "rust-cli"
name = "Rust CLI"

[[hooks.post_generate]]
name = "Install"
command = "cargo"
args = ["build"]
working_dir = "generated"
when = "run_build"
optional = true
shell = true

[hooks.post_generate.env]
RUST_LOG = "debug"
"#;

    let manifest =
        Manifest::from_toml_str(source, "zappy.toml").expect("hook env and flags should parse");
    let hook = manifest
        .hooks
        .post_generate
        .first()
        .expect("hook should exist");

    assert_eq!(hook.name.as_deref(), Some("Install"));
    assert_eq!(hook.when.as_deref(), Some("run_build"));
    assert_eq!(
        hook.working_dir.as_deref(),
        Some(camino::Utf8Path::new("generated"))
    );
    assert_eq!(hook.env.get("RUST_LOG"), Some(&String::from("debug")));
    assert!(hook.optional);
    assert!(hook.shell);
}

#[test]
fn builtin_helpers_filter_unknown_names_and_generate_all_placeholders() {
    let mut values = VariableValueMap::new();
    values.insert(
        String::from(crate::builtins::PROJECT_NAME),
        VariableValue::String(String::from("My Cool Tool")),
    );
    values.insert(
        String::from("not_builtin"),
        VariableValue::String(String::from("ignored")),
    );

    assert!(!crate::builtins::are_builtin_names(&values));

    let transformations = crate::builtins::builtin_transformations(&values);

    assert!(transformations.contains_key(crate::builtins::PROJECT_NAME));
    assert!(!transformations.contains_key("not_builtin"));
    assert_eq!(
        transformations
            .get(crate::builtins::PROJECT_NAME)
            .and_then(|transforms| transforms.get(&TransformKind::ScreamingSnake)),
        Some(&String::from("MY_COOL_TOOL")),
    );

    let replacements = crate::builtins::builtin_replacements(&values);

    assert_eq!(
        replacements.get("__ZAPPY_PROJECT_NAME_CAMEL__"),
        Some(&String::from("myCoolTool")),
    );
    assert_eq!(
        replacements.get("__ZAPPY_PROJECT_NAME_SCREAMING__"),
        Some(&String::from("MY_COOL_TOOL")),
    );
    assert_eq!(
        crate::builtins::builtin_placeholder(crate::builtins::EMAIL, TransformKind::Lower),
        "__ZAPPY_EMAIL_LOWER__",
    );
}

#[test]
fn builtin_name_helpers_accept_only_known_builtin_maps() {
    let mut builtins = VariableValueMap::new();

    for name in crate::builtins::BUILTIN_NAMES {
        builtins.insert(
            String::from(*name),
            VariableValue::String(String::from("value")),
        );
    }

    assert!(crate::builtins::are_builtin_names(&builtins));
}

#[test]
fn parses_cli_override_boolean_aliases_and_negative_integers() {
    let overrides = parse_variable_overrides([
        "yes_value=yes",
        "uppercase_yes=Y",
        "no_value=no",
        "uppercase_no=N",
        "one_value=1",
        "zero_value=0",
        "negative=-3",
    ])
    .expect("overrides should parse");

    assert_eq!(overrides.get("yes_value"), Some(&VariableValue::Bool(true)));
    assert_eq!(
        overrides.get("uppercase_yes"),
        Some(&VariableValue::Bool(true)),
    );
    assert_eq!(overrides.get("no_value"), Some(&VariableValue::Bool(false)));
    assert_eq!(
        overrides.get("uppercase_no"),
        Some(&VariableValue::Bool(false)),
    );
    assert_eq!(overrides.get("one_value"), Some(&VariableValue::Bool(true)));
    assert_eq!(
        overrides.get("zero_value"),
        Some(&VariableValue::Bool(false))
    );
    assert_eq!(overrides.get("negative"), Some(&VariableValue::Integer(-3)));
}

#[test]
fn rejects_cli_override_with_invalid_variable_name() {
    let err = parse_variable_overrides(["bad-name=value"])
        .expect_err("invalid variable override name should fail");

    assert!(err.to_string().contains("bad-name"));
}

#[test]
fn declared_builtin_value_is_replaced_by_injected_builtin() {
    let manifest = Manifest::from_toml_str(
        r#"
[template]
id = "builtin-precedence"
name = "Builtin Precedence"

[variables.test_project_name]
default = "declared"
"#,
        "zappy.toml",
    )
    .expect("manifest should parse");

    let mut explicit = VariableValueMap::new();
    explicit.insert(
        String::from("test_project_name"),
        VariableValue::String(String::from("explicit-value")),
    );

    let mut builtins = VariableValueMap::new();
    builtins.insert(
        String::from(crate::builtins::PROJECT_NAME),
        VariableValue::String(String::from("builtin-value")),
    );

    let resolved = resolve_variables(
        &manifest.variables,
        &VariableResolutionInput {
            builtins,
            explicit,
            ..VariableResolutionInput::default()
        },
    )
    .expect("variables should resolve");

    assert_eq!(
        resolved.values.get("test_project_name"),
        Some(&VariableValue::String(String::from("explicit-value"))),
    );
    assert_eq!(
        resolved.values.get(crate::builtins::PROJECT_NAME),
        Some(&VariableValue::String(String::from("builtin-value"))),
    );
}

#[test]
fn unknown_builtin_input_fails() {
    let manifest = Manifest::from_toml_str(
        r#"
[template]
id = "unknown-builtin"
name = "Unknown Builtin"
"#,
        "zappy.toml",
    )
    .expect("manifest should parse");

    let mut builtins = VariableValueMap::new();
    builtins.insert(
        String::from("not_a_builtin"),
        VariableValue::String(String::from("value")),
    );

    let err = resolve_variables(
        &manifest.variables,
        &VariableResolutionInput {
            builtins,
            ..VariableResolutionInput::default()
        },
    )
    .expect_err("unknown builtin value should fail");

    assert!(err.to_string().contains("not_a_builtin"));
}

#[test]
fn var_regex_passes_manifest_validation() {
    Manifest::from_toml_str(
        r#"
[template]
id = "with-regex"
name = "With Regex"
description = "Variable regex test"
language = "test"
version = "0.1.0"

[variables.package_name]
required = true
default = "demo"
validation_regex = "[0-9a-zA-Z-_]*"
"#,
        "zappy.toml",
    )
    .expect("variable with validation regex should parse");
}

#[test]
fn empty_regex_fails_manifest_validation() {
    let error = Manifest::from_toml_str(
        r#"
[template]
id = "empty-regex"
name = "Empty Regex"
description = "Empty regex test"
language = "test"
version = "0.1.0"

[variables.package_name]
required = true
default = "demo"
validation_regex = ""
"#,
        "zappy.toml",
    )
    .expect_err("empty regex should fail validation");

    assert!(matches!(error, CoreError::InvalidManifest { .. }));
}

#[test]
fn evaluates_conditions_only_for_true_boolean_values() {
    let mut values = VariableValueMap::new();
    values.insert(String::from("enabled"), VariableValue::Bool(true));
    values.insert(String::from("disabled"), VariableValue::Bool(false));
    values.insert(
        String::from("stringy"),
        VariableValue::String(String::from("true")),
    );
    values.insert(String::from("integer"), VariableValue::Integer(1));

    assert!(crate::condition::evaluate_condition("enabled", &values));
    assert!(!crate::condition::evaluate_condition("disabled", &values));
    assert!(!crate::condition::evaluate_condition("stringy", &values));
    assert!(!crate::condition::evaluate_condition("integer", &values));
    assert!(!crate::condition::evaluate_condition("missing", &values));
}

#[test]
fn render_text_preserves_missing_placeholders_and_applies_in_order() {
    let mut replacements = indexmap::IndexMap::new();
    replacements.insert(String::from("__NAME__"), String::from("my-tool"));
    replacements.insert(String::from("my-tool"), String::from("renamed-tool"));

    let rendered = crate::render::render_text("__NAME__ uses __MISSING__", &replacements);

    assert_eq!(rendered, "renamed-tool uses __MISSING__");
}

#[test]
fn rejects_empty_and_absolute_rendered_paths() {
    let replacements = indexmap::IndexMap::new();

    let empty = crate::render::render_relative_path(" ", &replacements)
        .expect_err("empty rendered path should fail");
    assert!(empty.to_string().contains("must not be empty"));

    let absolute = crate::render::render_relative_path("/tmp/project", &replacements)
        .expect_err("absolute rendered path should fail");
    assert!(absolute.to_string().contains("must be relative"));
}

#[test]
fn regex_valid_string_passes() {
    let mut variables = VariableMap::new();
    variables.insert(
        String::from("package_name"),
        VariableSpec {
            prompt: None,
            default: Some(VariableValue::String(String::from("my-tool"))),
            required: true,
            required_when: None,
            conflicts_with: Vec::new(),
            choices: Vec::new(),
            validation_regex: Some(String::from("^[a-z][a-z0-9_-]*$")),
            transforms: Vec::new(),
            placeholders: IndexMap::new(),
        },
    );

    let resolved = resolve_variables(&variables, &VariableResolutionInput::default())
        .expect("value should pass as it matches regex");
    assert!(resolved.values.contains_key("package_name"));
}

#[test]
fn regex_invalid_string_fails() {
    let mut variables = VariableMap::new();
    variables.insert(
        String::from("package_name"),
        VariableSpec {
            prompt: None,
            default: Some(VariableValue::String(String::from("123 invalid"))),
            required: true,
            required_when: None,
            conflicts_with: Vec::new(),
            choices: Vec::new(),
            validation_regex: Some(String::from("^[a-z][a-z0-9_-]*$")),
            transforms: Vec::new(),
            placeholders: IndexMap::new(),
        },
    );

    let error = resolve_variables(&variables, &VariableResolutionInput::default())
        .expect_err("invalid value should fail regex validation");
    assert!(matches!(error, CoreError::InvalidRegexVariableValue { .. }));
}

#[test]
fn regex_non_string_fails() {
    let mut variables = VariableMap::new();
    variables.insert(
        String::from("package_name"),
        VariableSpec {
            prompt: None,
            default: Some(VariableValue::Bool(true)),
            required: true,
            required_when: None,
            conflicts_with: Vec::new(),
            choices: Vec::new(),
            validation_regex: Some(String::from("^[a-z][a-z0-9_-]*$")),
            transforms: Vec::new(),
            placeholders: IndexMap::new(),
        },
    );

    let error = resolve_variables(&variables, &VariableResolutionInput::default())
        .expect_err("invalid value should fail regex validation");
    assert!(matches!(error, CoreError::RegexVariableNotString { .. }));
}
