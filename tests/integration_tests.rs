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
