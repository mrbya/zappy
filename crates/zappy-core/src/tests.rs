use super::*;

const MINIMAL_MANIFEST: &str = r#"
[template]
id = "rust-cli"
name = "Rust CLI Application"
description = "Small Rust CLI application."
language = "rust"
version = "0.1.0"

[variables.project_name]
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
    assert!(manifest.variables.contains_key("project_name"));
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

[variables.project_name]
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
