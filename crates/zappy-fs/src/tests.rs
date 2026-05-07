use std::fs;
use std::path::Path;

use tempfile::TempDir;

use super::*;
use crate::discover::discover_templates_from_search_paths;

fn write_template(templates_root: &Path, dir_name: &str, id: &str, name: &str, language: &str) {
    let template_dir = templates_root.join(dir_name);
    fs::create_dir_all(&template_dir).expect("template dir should be created");

    let manifest = format!(
        r#"
[template]
id = "{id}"
name = "{name}"
language = "{language}"
"#
    );

    fs::write(template_dir.join("zappy.toml"), manifest).expect("manifest should be written");
}

#[test]
fn discovers_templates_from_explicit_directory() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let templates_root = temp_dir.path();

    write_template(templates_root, "rust-cli", "rust-cli", "Rust CLI", "rust");

    let catalogue = discover_templates(&DiscoveryConfig {
        templates_dir: Some(templates_root.to_path_buf()),
    })
    .expect("templates should be discovered");

    assert_eq!(catalogue.templates().len(), 1);

    let template = catalogue
        .find_by_id("rust-cli")
        .expect("rust-cli template should be discovered");

    assert_eq!(template.manifest.template.name, "Rust CLI");
    assert_eq!(template.manifest.template.language.as_deref(), Some("rust"));
}

#[test]
fn supports_search_path_that_is_itself_a_template() {
    let temp_dir = TempDir::new().expect("tempdir should be created");

    fs::write(
        temp_dir.path().join("zappy.toml"),
        r#"
[template]
id = "single-template"
name = "Single Template"
language = "rust"
"#,
    )
    .expect("manifest should be written");

    let catalogue = discover_templates(&DiscoveryConfig {
        templates_dir: Some(temp_dir.path().to_path_buf()),
    })
    .expect("template should be discovered");

    assert!(catalogue.find_by_id("single-template").is_some());
}

#[test]
fn missing_explicit_directory_is_an_error() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let missing = temp_dir.path().join("missing");

    let err = discover_templates(&DiscoveryConfig {
        templates_dir: Some(missing),
    })
    .expect_err("missing explicit directory should fail");

    assert!(matches!(*err, FsError::SearchPathMissing { .. }));
}

#[test]
fn duplicate_template_ids_are_shadowed() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let first_root = temp_dir.path().join("first");
    let second_root = temp_dir.path().join("second");

    fs::create_dir_all(&first_root).expect("first root should be created");
    fs::create_dir_all(&second_root).expect("second root should be created");

    write_template(&first_root, "template-a", "same-id", "First", "rust");
    write_template(&second_root, "template-b", "same-id", "Second", "rust");

    let catalogue = discover_templates_from_search_paths(vec![
        TemplateSearchPath {
            kind: TemplateSearchPathKind::Explicit,
            path: first_root,
            required: true,
        },
        TemplateSearchPath {
            kind: TemplateSearchPathKind::CurrentWorkingDirectory,
            path: second_root,
            required: true,
        },
    ])
    .expect("templates should be discovered");

    assert_eq!(catalogue.templates().len(), 1);
    assert_eq!(catalogue.shadowed().len(), 1);

    let active = catalogue
        .find_by_id("same-id")
        .expect("active template should exist");

    assert_eq!(active.manifest.template.name, "First");
    assert_eq!(
        catalogue
            .shadowed()
            .first()
            .expect("should be populated")
            .manifest
            .template
            .name,
        "Second"
    );
}
