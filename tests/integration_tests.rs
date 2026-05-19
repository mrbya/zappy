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
            "Info: template `test-template` validated successfully",
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
        .stdout(predicates::str::contains("Info: validation temp"))
        .stdout(predicates::str::contains("kept @"));
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
            "crates/zappy-templates/templates",
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
            "crates/zappy-templates/templates",
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

#[test]
fn template_cpp_cmake_lib_generates() {
    let output = tempfile::TempDir::new().expect("output tempdir should be created");
    let project_dir = output.path().join("my-tool");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "new",
            "--template",
            "cpp-cmake-lib",
            "--templates-dir",
            "crates/zappy-templates/templates",
            "--name",
            "my-tool",
            "--var",
            "description=\"My generated tool\"",
            "--output",
        ])
        .arg(&project_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("Generated `cpp-cmake-lib`"));

    assert!(project_dir.join("CMakeLists.txt").exists());
}

#[test]
fn template_lua_cli_generates() {
    let output = tempfile::TempDir::new().expect("output tempdir should be created");
    let project_dir = output.path().join("my-tool");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "new",
            "--template",
            "lua-cli",
            "--templates-dir",
            "crates/zappy-templates/templates",
            "--name",
            "my-tool",
            "--var",
            "description=\"My generated tool\"",
            "--output",
        ])
        .arg(&project_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("Generated `lua-cli`"));

    assert!(project_dir.join("justfile").exists());
    assert!(project_dir.join(".luacov").exists());
    assert!(project_dir.join(".stylua.toml").exists());
}

#[test]
fn template_nvim_plugin_generates() {
    let output = tempfile::TempDir::new().expect("output tempdir should be created");
    let project_dir = output.path().join("my-tool");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "new",
            "--template",
            "nvim-plugin",
            "--templates-dir",
            "crates/zappy-templates/templates",
            "--name",
            "my-tool",
            "--var",
            "description=\"My generated tool\"",
            "--output",
        ])
        .arg(&project_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("Generated `nvim-plugin`"));

    assert!(project_dir.join("justfile").exists());
    assert!(project_dir.join(".stylua.toml").exists());
}

#[test]
fn zappy_should_see_builtin_templates() {
    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .arg("list")
        .assert()
        .success()
        .stdout(predicates::str::contains("rust-cli"));
}

#[test]
fn explicit_empty_templates_dir_disables_bundled_discovery() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "list",
            "--templates-dir",
            temp_dir.path().to_str().expect("path should be utf-8"),
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("rust-cli").not())
        .stderr(predicates::str::contains("Warning: no templates found"));
}

#[test]
fn missing_explicit_templates_dir_fails_clearly() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let missing = temp_dir.path().join("missing");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "list",
            "--templates-dir",
            missing.to_str().expect("path should be utf-8"),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("template search path"))
        .stderr(predicates::str::contains("does not exist"));
}

#[test]
fn new_fails_for_unknown_template_id() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "new",
            "--template",
            "missing",
            "--name",
            "my-tool",
            "--templates-dir",
            temp_dir.path().to_str().expect("path should be utf-8"),
            "--dry-run",
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "template `missing` was not found",
        ));
}

#[test]
fn validate_fails_for_unknown_template_id() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "validate",
            "--template",
            "missing",
            "--templates-dir",
            temp_dir.path().to_str().expect("path should be utf-8"),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains(
            "template `missing` was not found",
        ));
}

#[test]
fn new_fails_when_required_variable_is_missing() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let template_dir = temp_dir.path().join("required-template");

    fs::create_dir_all(template_dir.join("template")).expect("template dir should be created");
    fs::write(
        template_dir.join("zappy.toml"),
        r#"
[template]
id = "required-template"
name = "Required Template"

[variables.required_value]
required = true
prompt = "Required value"
"#,
    )
    .expect("manifest should be written");
    fs::write(template_dir.join("template/README.md"), "# README\n")
        .expect("template file should be written");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "new",
            "--template",
            "required-template",
            "--name",
            "my-tool",
            "--templates-dir",
            temp_dir.path().to_str().expect("path should be utf-8"),
            "--dry-run",
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("required_value"));
}

#[test]
fn new_fails_on_output_conflict_without_force() {
    let output = tempfile::TempDir::new().expect("output tempdir should be created");
    let project_dir = output.path().join("my-tool");

    fs::create_dir_all(&project_dir).expect("project dir should be created");
    fs::write(project_dir.join("README.md"), "existing").expect("conflict should be written");

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
        .failure()
        .stderr(predicates::str::contains("already exists"));
}

#[test]
fn new_no_hooks_skips_failing_generation_hook() {
    let templates = tempfile::TempDir::new().expect("templates tempdir should be created");
    let output = tempfile::TempDir::new().expect("output tempdir should be created");
    let template_dir = templates.path().join("hook-template");
    let project_dir = output.path().join("my-tool");

    fs::create_dir_all(template_dir.join("template")).expect("template dir should be created");
    fs::write(template_dir.join("template/README.md"), "# Hook Template\n")
        .expect("template file should be written");
    fs::write(
        template_dir.join("zappy.toml"),
        r#"
[template]
id = "hook-template"
name = "Hook Template"

[[hooks.post_generate]]
name = "Failing hook"
command = "zappy-definitely-missing-command"
"#,
    )
    .expect("manifest should be written");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "new",
            "--template",
            "hook-template",
            "--name",
            "my-tool",
            "--templates-dir",
            templates.path().to_str().expect("path should be utf-8"),
            "--output",
        ])
        .arg(&project_dir)
        .arg("--no-hooks")
        .assert()
        .success()
        .stdout(predicates::str::contains("Generated `hook-template`"));

    assert!(project_dir.join("README.md").exists());
}

#[test]
fn validate_fails_when_validation_step_fails() {
    let templates = tempfile::TempDir::new().expect("templates tempdir should be created");
    let template_dir = templates.path().join("bad-validation");

    fs::create_dir_all(template_dir.join("template")).expect("template dir should be created");
    fs::write(
        template_dir.join("template/README.md"),
        "# Bad Validation\n",
    )
    .expect("template file should be written");
    fs::write(
        template_dir.join("zappy.toml"),
        r#"
[template]
id = "bad-validation"
name = "Bad Validation"

[validation]

[[validation.steps]]
name = "Fail"
command = "rustc"
args = ["--definitely-not-a-rustc-flag"]
"#,
    )
    .expect("manifest should be written");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "validate",
            "--template",
            "bad-validation",
            "--templates-dir",
            templates.path().to_str().expect("path should be utf-8"),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Fail"));
}

#[test]
fn validate_no_hooks_skips_failing_generation_hook() {
    let templates = tempfile::TempDir::new().expect("templates tempdir should be created");
    let template_dir = templates.path().join("validate-no-hooks");

    fs::create_dir_all(template_dir.join("template")).expect("template dir should be created");
    fs::write(
        template_dir.join("template/README.md"),
        "# Validate No Hooks\n",
    )
    .expect("template file should be written");
    fs::write(
        template_dir.join("zappy.toml"),
        r#"
[template]
id = "validate-no-hooks"
name = "Validate No Hooks"

[[hooks.post_generate]]
name = "Failing generation hook"
command = "zappy-definitely-missing-command"

[validation]

[[validation.setup]]
name = "Setup"
command = "rustc"
args = ["--version"]

[[validation.steps]]
name = "Step"
command = "rustc"
args = ["--version"]

[[validation.teardown]]
name = "Teardown"
command = "rustc"
args = ["--version"]
"#,
    )
    .expect("manifest should be written");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args([
            "validate",
            "--template",
            "validate-no-hooks",
            "--templates-dir",
            templates.path().to_str().expect("path should be utf-8"),
            "--no-hooks",
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains(
            "Info: template `validate-no-hooks` validated successfully",
        ));
}

#[test]
fn create_non_empty_reports_current_stub() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let source_dir = temp_dir.path().join("project");
    let output_dir = temp_dir.path().join("template");

    fs::create_dir_all(&source_dir).expect("source project should be created");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args(["create", "--from"])
        .arg(&source_dir)
        .args(["--template", "stub-template", "--output"])
        .arg(&output_dir)
        .assert()
        .success()
        .stdout(predicates::str::contains("zappy create: stub"));
}

#[test]
fn init_fails_for_existing_template_without_force() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("rust-cli");

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args(["init", "--output"])
        .arg(&output_dir)
        .assert()
        .success();

    assert_cmd::Command::cargo_bin("zappy")
        .expect("zappy binary should exist")
        .args(["init", "--output"])
        .arg(&output_dir)
        .assert()
        .failure()
        .stderr(predicates::str::contains("already exists"));
}
