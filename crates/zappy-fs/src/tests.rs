use std::fs;
use std::path::{Path, PathBuf};

use tempfile::TempDir;
use zappy_core::{GenerationPlan, Manifest, PlanOperation, TemplateId};

use super::*;
use crate::discover::discover_templates_from_search_paths;
use crate::{MaterializationOptions, materialize_generation_plan};

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

#[test]
fn materializes_text_file() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("out");

    let plan = GenerationPlan {
        template_id: TemplateId::new("test-template").expect("template id should be valid"),
        output_dir: output_dir.clone(),
        warnings: Vec::new(),
        operations: vec![
            PlanOperation::CreateDirectory {
                source: None,
                destination: output_dir.join("src"),
            },
            PlanOperation::RenderTextFile {
                source: PathBuf::from("template/src/main.rs"),
                destination: output_dir.join("src/main.rs"),
                content: String::from("fn main() {}\n"),
            },
        ],
    };

    let summary = materialize_generation_plan(&plan, MaterializationOptions::no_force())
        .expect("plan should materialize");

    assert_eq!(summary.directories_created, 1);
    assert_eq!(summary.text_files_written, 1);

    let content = fs::read_to_string(output_dir.join("src/main.rs"))
        .expect("generated text file should be readable");

    assert_eq!(content, "fn main() {}\n");
}

#[test]
fn materializes_binary_file() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let source = temp_dir.path().join("source.bin");
    let output_dir = temp_dir.path().join("out");
    let destination = output_dir.join("source.bin");

    fs::write(&source, [0_u8, 159, 146, 150]).expect("binary source should be written");

    let plan = GenerationPlan {
        template_id: TemplateId::new("test-template").expect("template id should be valid"),
        output_dir,
        warnings: Vec::new(),
        operations: vec![PlanOperation::CopyBinaryFile {
            source,
            destination: destination.clone(),
        }],
    };

    let summary = materialize_generation_plan(&plan, MaterializationOptions::no_force())
        .expect("plan should materialize");

    assert_eq!(summary.binary_files_copied, 1);

    let copied = fs::read(destination).expect("binary output should be readable");

    assert_eq!(copied, [0_u8, 159, 146, 150]);
}

#[test]
fn refuses_to_overwrite_existing_file_without_force() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("out");
    let destination = output_dir.join("README.md");

    fs::create_dir_all(&output_dir).expect("output dir should be created");
    fs::write(&destination, "existing").expect("existing file should be written");

    let plan = GenerationPlan {
        template_id: TemplateId::new("test-template").expect("template id should be valid"),
        output_dir,
        warnings: Vec::new(),
        operations: vec![PlanOperation::RenderTextFile {
            source: PathBuf::from("template/README.md"),
            destination,
            content: String::from("new"),
        }],
    };

    let err = materialize_generation_plan(&plan, MaterializationOptions::no_force())
        .expect_err("existing file should fail without force");

    assert!(err.to_string().contains("already exists"));
}

#[test]
fn overwrites_existing_file_with_force() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("out");
    let destination = output_dir.join("README.md");

    fs::create_dir_all(&output_dir).expect("output dir should be created");
    fs::write(&destination, "existing").expect("existing file should be written");

    let plan = GenerationPlan {
        template_id: TemplateId::new("test-template").expect("template id should be valid"),
        output_dir,
        warnings: Vec::new(),
        operations: vec![PlanOperation::RenderTextFile {
            source: PathBuf::from("template/README.md"),
            destination: destination.clone(),
            content: String::from("new"),
        }],
    };

    materialize_generation_plan(&plan, MaterializationOptions::force())
        .expect("existing file should be overwritten with force");

    let content = fs::read_to_string(destination).expect("overwritten file should be readable");

    assert_eq!(content, "new");
}

#[test]
fn initializes_template_skeleton() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("rust-cli");

    let input = InitTemplateInput {
        output_dir: output_dir.clone(),
        template_id: Some(String::from("rust-cli")),
        name: None,
        description: None,
        force: false,
    };

    crate::init_template_skeleton(&input).expect("template skeleton should be initialized");

    assert!(output_dir.join("zappy.toml").exists());
    assert!(output_dir.join("template/README.md").exists());
}

#[test]
fn init_skeleton_refuses_existing_without_force() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("rust-cli");

    let input = InitTemplateInput {
        output_dir,
        template_id: Some(String::from("rust-cli")),
        name: None,
        description: None,
        force: false,
    };

    crate::init_template_skeleton(&input).expect("template skeleton should be initialized");

    let err = crate::init_template_skeleton(&input);
    assert!(err.is_err_and(|e| e.to_string().contains("already exists")));
}

#[test]
fn init_skeleton_overwrites_existing_with_force() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("rust-cli");

    let mut input = InitTemplateInput {
        output_dir: output_dir.clone(),
        template_id: Some(String::from("rust-cli")),
        name: None,
        description: None,
        force: true,
    };

    crate::init_template_skeleton(&input).expect("template skeleton should be initialized");

    input.name = Some(String::from("another name"));

    assert!(crate::init_template_skeleton(&input).is_ok());
    let readme_path = output_dir.join("template").join("README.md");
    let readme = fs::read_to_string(readme_path).expect("should be able to read README");
    assert!(readme.contains("another name"));
}

#[test]
fn init_skeleton_manifest_parses() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("rust-cli");

    let input = InitTemplateInput {
        output_dir: output_dir.clone(),
        template_id: Some(String::from("rust-cli")),
        name: None,
        description: None,
        force: false,
    };

    crate::init_template_skeleton(&input).expect("template skeleton should be initialized");

    let read_result = Manifest::load_from_path(output_dir.join("zappy.toml"));
    assert!(read_result.is_ok());
    assert_eq!(
        read_result.expect("should be ok").template.id.as_str(),
        "rust-cli"
    );
}
