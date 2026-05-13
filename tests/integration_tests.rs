use std::fs;
use std::path::Path;

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

fn write_template(templates_root: &Path, dir_name: &str, id: &str, name: &str, language: &str) {
    let template_dir = templates_root.join(dir_name);
    fs::create_dir_all(&template_dir).expect("template dir should be created");

    let manifest = format!(
        r#"
[template]
id = "{id}"
name = "{name}"
description = "Fixture template"
language = "{language}"
version = "0.1.0"

[template.source]
root = "template"
"#
    );

    fs::write(template_dir.join("zappy.toml"), manifest).expect("manifest should be written");
}

#[test]
fn list_shows_templates_from_explicit_directory() {
    let temp_dir = TempDir::new().expect("tempdir should be created");

    write_template(temp_dir.path(), "rust-cli", "rust-cli", "Rust CLI", "rust");

    Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "list",
            "--templates-dir",
            temp_dir.path().to_str().expect("path should be utf-8"),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("rust-cli"))
        .stdout(predicate::str::contains("Rust CLI"));
}

#[test]
fn list_filters_by_language() {
    let temp_dir = TempDir::new().expect("tempdir should be created");

    write_template(temp_dir.path(), "rust-cli", "rust-cli", "Rust CLI", "rust");
    write_template(
        temp_dir.path(),
        "python-cli",
        "python-cli",
        "Python CLI",
        "python",
    );

    Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "list",
            "--templates-dir",
            temp_dir.path().to_str().expect("path should be utf-8"),
            "--language",
            "rust",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("rust-cli"))
        .stdout(predicate::str::contains("python-cli").not());
}

#[test]
fn info_shows_selected_template_details() {
    let temp_dir = TempDir::new().expect("tempdir should be created");

    write_template(temp_dir.path(), "rust-cli", "rust-cli", "Rust CLI", "rust");

    Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "info",
            "--template",
            "rust-cli",
            "--templates-dir",
            temp_dir.path().to_str().expect("path should be utf-8"),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("ID:"))
        .stdout(predicate::str::contains("rust-cli"))
        .stdout(predicate::str::contains("Rust CLI"))
        .stdout(predicate::str::contains("Fixture template"));
}

#[test]
fn info_fails_for_missing_template() {
    let temp_dir = TempDir::new().expect("tempdir should be created");

    Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "info",
            "--template",
            "missing",
            "--templates-dir",
            temp_dir.path().to_str().expect("path should be utf-8"),
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("template `missing` was not found"));
}

#[test]
fn new_dry_run_prints_generation_plan() {
    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "new",
            "--template",
            "test-template",
            "--name",
            "my-tool",
            "--templates-dir",
            "tests/fixtures/templates",
        ])
        .arg("--var")
        .arg("test_var=my cool tool")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicates::str::contains("Dry-run generation plan"))
        .stdout(predicates::str::contains("RENDER"))
        .stdout(predicates::str::contains("WARN"))
        .stdout(predicates::str::contains("SKIP"))
        .stdout(predicates::str::contains("[Symlink]"))
        .stdout(predicates::str::contains("[ConditionalFalse"))
        .stdout(predicates::str::contains("[Excluded]"))
        .stdout(predicates::str::contains("myCoolTool.md"))
        .stdout(predicates::str::contains("my-cool-tool.md"))
        .stdout(predicates::str::contains("my cool tool.md"))
        .stdout(predicates::str::contains("MyCoolTool.md"))
        .stdout(predicates::str::contains("MY_COOL_TOOL.md"))
        .stdout(predicates::str::contains("my_cool_tool.md"))
        .stdout(predicates::str::contains("CREATE DIR"));

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "new",
            "--template",
            "test-template",
            "--name",
            "my-tool",
            "--templates-dir",
            "tests/fixtures/templates",
        ])
        .arg("--var")
        .arg("test_var=my cool tool")
        .arg("--var")
        .arg("include_optional=yes")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicates::str::contains("Dry-run generation plan"))
        .stdout(predicates::str::contains("RENDER"))
        .stdout(predicates::str::contains("WARN"))
        .stdout(predicates::str::contains("SKIP"))
        .stdout(predicates::str::contains("[Symlink]"))
        .stdout(predicates::str::contains("[Excluded]"))
        .stdout(predicates::str::contains("my-tool/optional.md"))
        .stdout(predicates::str::contains("myCoolTool.md"))
        .stdout(predicates::str::contains("my-cool-tool.md"))
        .stdout(predicates::str::contains("my cool tool.md"))
        .stdout(predicates::str::contains("MyCoolTool.md"))
        .stdout(predicates::str::contains("MY_COOL_TOOL.md"))
        .stdout(predicates::str::contains("my_cool_tool.md"))
        .stdout(predicates::str::contains("CREATE DIR"));
}

#[test]
fn new_generates_project_files() {
    let output = TempDir::new().expect("output tempdir should be created");
    let project_dir = output.path().join("my-tool");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "new",
            "--template",
            "test-template",
            "--templates-dir",
            "tests/fixtures/templates",
            "--name",
            "my-tool",
            "--var",
            "test_var=my cool tool",
            "--output",
        ])
        .arg(&project_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("Generated `test-template`"));

    let readme =
        fs::read_to_string(project_dir.join("README.md")).expect("README should be generated");

    assert!(
        readme.contains("my cool tool"),
        "README should contain rendered variable value",
    );

    assert!(
        project_dir.join("dir4/my_cool_tool.md").exists(),
        "rendered snake-case path should exist",
    );

    assert!(
        !project_dir.join("optional.md").exists(),
        "conditional file should be skipped by default",
    );
}

#[test]
fn new_dry_run_does_not_write_files() {
    let _ = TempDir::new().expect("output tempdir should be created");
    let output = TempDir::new().expect("output tempdir should be created");
    let project_dir = output.path().join("my-tool");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "new",
            "--template",
            "test-template",
            "--templates-dir",
            "tests/fixtures/templates",
            "--name",
            "my-tool",
            "--var",
            "test_var=my cool tool",
            "--output",
        ])
        .arg(&project_dir)
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicates::str::contains("Dry-run generation plan"));

    assert!(
        !project_dir.exists(),
        "dry-run must not create the output directory",
    );
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

#[test]
fn validate_generates_and_checks_template() {
    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "validate",
            "--template",
            "test-template",
            "--templates-dir",
            "tests/fixtures/templates",
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "Template `test-template` validated successfully.",
        ));
}

#[test]
fn validate_fails_without_validation_config() {
    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "validate",
            "--template",
            "template-without-validation",
            "--templates-dir",
            "tests/fixtures/templates",
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "does not define validation config",
        ));
}

#[test]
fn validate_keep_temp_prints_temp_dir() {
    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "validate",
            "--template",
            "test-template",
            "--templates-dir",
            "tests/fixtures/templates",
            "--keep-temp",
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("Validation temp dir kept at"));
}

#[test]
fn init_creates_template_skeleton() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("rust-cli");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args(["init", "--output"])
        .arg(&output_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("Initialized template"));

    assert!(output_dir.join("zappy.toml").exists());
    assert!(output_dir.join("template/README.md").exists());
}

#[test]
fn create_empty_creates_template_skeleton() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("rust-cli");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args(["create", "--empty", "--output"])
        .arg(&output_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("Initialized template"));

    assert!(output_dir.join("zappy.toml").exists());
    assert!(output_dir.join("template/README.md").exists());
}

#[test]
fn template_rust_cli_generates() {
    let output = tempfile::TempDir::new().expect("output tempdir should be created");
    let project_dir = output.path().join("my-tool");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "new",
            "--template",
            "rust-cli",
            "--templates-dir",
            "templates",
            "--name",
            "my_tool",
            "--var",
            "description=\"My generated tool\"",
            "--output",
        ])
        .arg(&project_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Generated `rust-cli`"));

    assert!(project_dir.join("Cargo.toml").exists());
    assert!(project_dir.join("src/main.rs").exists());
}

#[test]
fn template_cpp_cmake_app_generates() {
    let output = tempfile::TempDir::new().expect("output tempdir should be created");
    let project_dir = output.path().join("my-tool");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "new",
            "--template",
            "cpp-cmake-app",
            "--templates-dir",
            "templates",
            "--name",
            "my_tool",
            "--var",
            "description=\"My generated tool\"",
            "--output",
        ])
        .arg(&project_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Generated `cpp-cmake-app`"));

    assert!(project_dir.join("CMakeLists.txt").exists());
    assert!(project_dir.join("src/main.cpp").exists());
}
