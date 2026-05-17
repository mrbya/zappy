use std::fs;
use std::path::{Path, PathBuf};

use tempfile::TempDir;
use zappy_core::{GenerationPlan, Manifest, PlanOperation, TemplateId};

use super::*;
use crate::discover::discover_templates_from_search_paths;
use crate::{MaterializationOptions, materialize_generation_plan};

static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

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
        bundled_templates_dir: None,
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
        bundled_templates_dir: None,
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
        bundled_templates_dir: None,
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

#[test]
fn explicit_search_path_overrides_other_discovery_paths() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let explicit_root = temp_dir.path().join("explicit");
    let bundled_root = temp_dir.path().join("bundled");

    fs::create_dir_all(&explicit_root).expect("explicit root should be created");
    fs::create_dir_all(&bundled_root).expect("bundled root should be created");
    write_template(
        &explicit_root,
        "explicit-template",
        "explicit",
        "Explicit",
        "rust",
    );
    write_template(
        &bundled_root,
        "bundled-template",
        "bundled",
        "Bundled",
        "rust",
    );

    let paths = resolve_template_search_paths(&DiscoveryConfig {
        templates_dir: Some(explicit_root.clone()),
        bundled_templates_dir: Some(bundled_root),
    })
    .expect("search paths should resolve");

    assert_eq!(paths.len(), 1);
    assert_eq!(
        paths.first().expect("path should exist").kind,
        TemplateSearchPathKind::Explicit,
    );
    assert_eq!(
        paths.first().expect("path should exist").path,
        explicit_root
    );
}

#[test]
fn implicit_search_paths_include_standard_locations_and_bundled_last() {
    let _guard = ENV_LOCK.lock().expect("env lock should be acquired");
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let bundled_root = temp_dir.path().join("bundled");

    let paths = resolve_template_search_paths(&DiscoveryConfig {
        templates_dir: None,
        bundled_templates_dir: Some(bundled_root.clone()),
    })
    .expect("implicit search paths should resolve");

    assert!(
        paths
            .iter()
            .any(|path| path.kind == TemplateSearchPathKind::PlatformConfig)
    );
    assert!(
        paths
            .iter()
            .any(|path| path.kind == TemplateSearchPathKind::ExecutableRelative)
    );
    assert!(
        paths
            .iter()
            .any(|path| path.kind == TemplateSearchPathKind::CurrentWorkingDirectory)
    );

    let bundled = paths.last().expect("bundled path should be last");

    assert_eq!(bundled.kind, TemplateSearchPathKind::Bundled);
    assert_eq!(bundled.path, bundled_root);
    assert!(!bundled.required);
}

#[test]
fn implicit_search_paths_include_environment_overrides_first() {
    let _guard = ENV_LOCK.lock().expect("env lock should be acquired");
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let templates_env = temp_dir.path().join("env-templates");
    let config_env = temp_dir.path().join("config-root");

    // SAFETY: this test serializes environment access with ENV_LOCK and restores
    // the variables before returning.
    unsafe {
        std::env::set_var("ZAPPY_TEMPLATES_DIR", &templates_env);
        std::env::set_var("ZAPPY_CONFIG", &config_env);
    }

    let paths = resolve_template_search_paths(&DiscoveryConfig {
        templates_dir: None,
        bundled_templates_dir: None,
    })
    .expect("implicit search paths should resolve");

    // SAFETY: see the set_var safety note above.
    unsafe {
        std::env::remove_var("ZAPPY_TEMPLATES_DIR");
        std::env::remove_var("ZAPPY_CONFIG");
    }

    let first = paths.first().expect("env templates path should be first");
    let second = paths.get(1).expect("env config path should be second");

    assert_eq!(first.kind, TemplateSearchPathKind::EnvironmentTemplatesDir);
    assert_eq!(first.path, templates_env);
    assert!(first.required);
    assert_eq!(
        second.kind,
        TemplateSearchPathKind::EnvironmentConfigTemplates
    );
    assert_eq!(second.path, config_env.join("templates"));
    assert!(!second.required);
}

#[test]
fn discover_templates_uses_bundled_directory_when_no_explicit_dir() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let bundled_root = temp_dir.path().join("bundled");

    fs::create_dir_all(&bundled_root).expect("bundled root should be created");
    write_template(
        &bundled_root,
        "bundled-template",
        "bundled",
        "Bundled",
        "rust",
    );

    let catalogue = discover_templates(&DiscoveryConfig {
        templates_dir: None,
        bundled_templates_dir: Some(bundled_root),
    })
    .expect("bundled templates should be discovered");

    assert!(catalogue.find_by_id("bundled").is_some());
    assert!(
        catalogue
            .search_paths()
            .iter()
            .any(|path| path.kind == TemplateSearchPathKind::Bundled)
    );
}

#[test]
fn optional_missing_and_file_search_paths_are_ignored() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let existing_root = temp_dir.path().join("templates");
    let file_path = temp_dir.path().join("not-a-directory");

    fs::create_dir_all(&existing_root).expect("templates root should be created");
    fs::write(&file_path, "not a directory").expect("file path should be written");
    write_template(&existing_root, "rust-cli", "rust-cli", "Rust CLI", "rust");

    let catalogue = discover_templates_from_search_paths(vec![
        TemplateSearchPath {
            kind: TemplateSearchPathKind::EnvironmentConfigTemplates,
            path: temp_dir.path().join("missing"),
            required: false,
        },
        TemplateSearchPath {
            kind: TemplateSearchPathKind::PlatformConfig,
            path: file_path,
            required: false,
        },
        TemplateSearchPath {
            kind: TemplateSearchPathKind::CurrentWorkingDirectory,
            path: existing_root,
            required: false,
        },
    ])
    .expect("optional missing and file paths should be ignored");

    assert_eq!(catalogue.templates().len(), 1);
    assert!(catalogue.find_by_id("rust-cli").is_some());
    assert_eq!(catalogue.search_paths().len(), 3);
}

#[test]
fn required_search_path_that_is_file_is_error() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let file_path = temp_dir.path().join("not-a-directory");

    fs::write(&file_path, "not a directory").expect("file path should be written");

    let err = discover_templates_from_search_paths(vec![TemplateSearchPath {
        kind: TemplateSearchPathKind::Explicit,
        path: file_path,
        required: true,
    }])
    .expect_err("required file search path should fail");

    assert!(matches!(*err, FsError::SearchPathNotDirectory { .. }));
}

#[test]
fn invalid_manifest_in_search_path_reports_manifest_path() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let template_dir = temp_dir.path().join("broken-template");

    fs::create_dir_all(&template_dir).expect("template dir should be created");
    fs::write(template_dir.join("zappy.toml"), "not toml = [")
        .expect("broken manifest should be written");

    let err = discover_templates_from_search_paths(vec![TemplateSearchPath {
        kind: TemplateSearchPathKind::Explicit,
        path: temp_dir.path().to_path_buf(),
        required: true,
    }])
    .expect_err("invalid manifest should fail discovery");

    assert!(err.to_string().contains("zappy.toml"));
}

#[test]
fn walk_source_root_is_deterministic_and_classifies_entries() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let root = temp_dir.path();

    fs::create_dir_all(root.join("b/nested")).expect("nested dir should be created");
    fs::create_dir_all(root.join("a")).expect("a dir should be created");
    fs::write(root.join("b/file.txt"), "b").expect("b file should be written");
    fs::write(root.join("a/file.txt"), "a").expect("a file should be written");

    #[cfg(unix)]
    std::os::unix::fs::symlink(root.join("a/file.txt"), root.join("link.txt"))
        .expect("symlink should be created");

    let entries = crate::walk::walk_source_root(root).expect("source root should walk");
    let paths = entries
        .iter()
        .map(|entry| entry.relative_path.to_string_lossy().into_owned())
        .collect::<Vec<_>>();

    assert_eq!(
        paths,
        if cfg!(unix) {
            vec!["a", "a/file.txt", "b", "b/file.txt", "b/nested", "link.txt"]
        } else {
            vec!["a", "a/file.txt", "b", "b/file.txt", "b/nested"]
        }
    );

    let a_dir = entries
        .iter()
        .find(|entry| entry.relative_path == Path::new("a"))
        .expect("a dir entry should exist");
    assert_eq!(a_dir.kind, crate::walk::SourceEntryKind::Directory);

    let a_file = entries
        .iter()
        .find(|entry| entry.relative_path == Path::new("a/file.txt"))
        .expect("a file entry should exist");
    assert_eq!(a_file.kind, crate::walk::SourceEntryKind::File);

    #[cfg(unix)]
    {
        let symlink = entries
            .iter()
            .find(|entry| entry.relative_path == Path::new("link.txt"))
            .expect("symlink entry should exist");
        assert_eq!(symlink.kind, crate::walk::SourceEntryKind::Symlink);
    }
}

#[test]
fn walk_missing_source_root_reports_read_error() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let missing_root = temp_dir.path().join("missing");

    let err =
        crate::walk::walk_source_root(&missing_root).expect_err("missing source root should fail");

    assert!(matches!(*err, FsError::ReadSearchPath { .. }));
}

#[test]
fn classifies_binary_files_by_extension_and_file_name() {
    let config = zappy_core::manifest::PathConfig {
        exclude: Vec::new(),
        binary_extensions: vec![String::from(".png"), String::from("zip")],
        binary_files: vec![String::from("Cargo.lock"), String::from("assets/blob")],
    };

    assert_eq!(
        classify_file_by_path(Path::new("images/logo.png"), &config),
        Some(FileKind::Binary),
    );
    assert_eq!(
        classify_file_by_path(Path::new("archives/project.zip"), &config),
        Some(FileKind::Binary),
    );
    assert_eq!(
        classify_file_by_path(Path::new("nested/Cargo.lock"), &config),
        Some(FileKind::Binary),
    );
    assert_eq!(
        classify_file_by_path(Path::new("assets/blob"), &config),
        Some(FileKind::Binary),
    );
    assert_eq!(classify_file_by_path(Path::new("README"), &config), None);
    assert_eq!(
        classify_file_by_path(Path::new("src/main.rs"), &config),
        None
    );
}

fn write_plan_fixture(template_dir: &Path) -> Manifest {
    let source_root = template_dir.join("template");

    fs::create_dir_all(source_root.join("src")).expect("src dir should be created");
    fs::create_dir_all(source_root.join("excluded")).expect("excluded dir should be created");
    fs::create_dir_all(source_root.join("optional")).expect("optional dir should be created");
    fs::write(source_root.join("src/__NAME__.txt"), "hello __NAME__")
        .expect("text template should be written");
    fs::write(source_root.join("asset.bin"), [0_u8, 1, 2])
        .expect("binary template should be written");
    fs::write(source_root.join("raw.dat"), [0xff_u8, 0xfe, 0xfd])
        .expect("non-utf8 template should be written");
    fs::write(source_root.join("excluded/ignored.txt"), "ignored")
        .expect("excluded template should be written");
    fs::write(source_root.join("optional/feature.txt"), "optional")
        .expect("optional template should be written");

    #[cfg(unix)]
    std::os::unix::fs::symlink(
        source_root.join("src/__NAME__.txt"),
        source_root.join("link.txt"),
    )
    .expect("symlink should be created");

    Manifest::from_toml_str(
        r#"
[template]
id = "plan-fixture"
name = "Plan Fixture"

[variables.name]
default = "project-name"
transforms = ["raw"]

[variables.name.placeholders]
raw = "__NAME__"

[variables.include_optional]
default = false

[paths]
exclude = ["excluded"]
binary_extensions = ["bin"]

[[conditionals]]
path = "optional"
when = "include_optional"
"#,
        "zappy.toml",
    )
    .expect("fixture manifest should parse")
}

#[test]
fn build_plan_handles_rendering_skips_binary_classification_and_warnings() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let template_dir = temp_dir.path().join("fixture");
    let output_dir = temp_dir.path().join("out");
    let manifest = write_plan_fixture(&template_dir);
    let variables = zappy_core::resolve_variables(
        &manifest.variables,
        &zappy_core::VariableResolutionInput::default(),
    )
    .expect("variables should resolve");

    fs::create_dir_all(output_dir.join("src")).expect("output src should be created");
    fs::write(output_dir.join("src/project-name.txt"), "existing")
        .expect("conflicting output should be written");

    let plan = build_generation_plan(&BuildPlanInput {
        template_dir: &template_dir,
        manifest: &manifest,
        variables: &variables,
        output_dir: output_dir.clone(),
        force: false,
    })
    .expect("generation plan should build");

    assert!(
        plan.warnings
            .iter()
            .any(|warning| matches!(warning, zappy_core::PlanWarning::DestinationExists { destination } if destination == &output_dir.join("src/project-name.txt")))
    );
    assert!(plan.warnings.iter().any(|warning| {
        matches!(warning, zappy_core::PlanWarning::NonUtf8FileCopiedAsBinary { source } if source.ends_with("raw.dat"))
    }));
    assert!(plan.operations.iter().any(|operation| {
        matches!(operation, PlanOperation::RenderTextFile { destination, content, .. } if destination == &output_dir.join("src/project-name.txt") && content == "hello project-name")
    }));
    assert!(plan.operations.iter().any(|operation| {
        matches!(operation, PlanOperation::CopyBinaryFile { destination, .. } if destination == &output_dir.join("asset.bin"))
    }));
    assert!(plan.operations.iter().any(|operation| {
        matches!(operation, PlanOperation::CopyBinaryFile { destination, .. } if destination == &output_dir.join("raw.dat"))
    }));
    assert!(plan.operations.iter().any(|operation| {
        matches!(operation, PlanOperation::Skip { reason: zappy_core::SkipReason::Excluded, source } if source.ends_with("excluded"))
    }));
    assert!(plan.operations.iter().any(|operation| {
        matches!(operation, PlanOperation::Skip { reason: zappy_core::SkipReason::ConditionalFalse { variable }, source } if variable == "include_optional" && source.ends_with("optional"))
    }));

    #[cfg(unix)]
    assert!(plan.operations.iter().any(|operation| {
        matches!(operation, PlanOperation::Skip { reason: zappy_core::SkipReason::Symlink, source } if source.ends_with("link.txt"))
    }));
}

#[test]
fn build_plan_includes_conditional_path_when_true() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let template_dir = temp_dir.path().join("fixture");
    let output_dir = temp_dir.path().join("out");
    let manifest = write_plan_fixture(&template_dir);
    let mut explicit = zappy_core::VariableValueMap::new();

    explicit.insert(
        String::from("include_optional"),
        zappy_core::VariableValue::Bool(true),
    );

    let variables = zappy_core::resolve_variables(
        &manifest.variables,
        &zappy_core::VariableResolutionInput {
            explicit,
            ..zappy_core::VariableResolutionInput::default()
        },
    )
    .expect("variables should resolve");

    let plan = build_generation_plan(&BuildPlanInput {
        template_dir: &template_dir,
        manifest: &manifest,
        variables: &variables,
        output_dir: output_dir.clone(),
        force: false,
    })
    .expect("generation plan should build");

    assert!(plan.operations.iter().any(|operation| {
        matches!(operation, PlanOperation::RenderTextFile { destination, content, .. } if destination == &output_dir.join("optional/feature.txt") && content == "optional")
    }));
}

#[test]
fn materialization_counts_skips_and_creates_parent_dirs() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("out");

    let plan = GenerationPlan {
        template_id: TemplateId::new("test-template").expect("template id should be valid"),
        output_dir: output_dir.clone(),
        warnings: Vec::new(),
        operations: vec![
            PlanOperation::RenderTextFile {
                source: PathBuf::from("template/nested/README.md"),
                destination: output_dir.join("nested/README.md"),
                content: String::from("nested"),
            },
            PlanOperation::Skip {
                source: PathBuf::from("template/skipped"),
                reason: zappy_core::SkipReason::Excluded,
            },
        ],
    };

    let summary = materialize_generation_plan(&plan, MaterializationOptions::no_force())
        .expect("plan should materialize");

    assert_eq!(summary.text_files_written, 1);
    assert_eq!(summary.skipped, 1);
    assert_eq!(
        fs::read_to_string(output_dir.join("nested/README.md"))
            .expect("nested readme should be readable"),
        "nested",
    );
}

#[test]
fn materialization_reports_missing_binary_source() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("out");

    let plan = GenerationPlan {
        template_id: TemplateId::new("test-template").expect("template id should be valid"),
        output_dir: output_dir.clone(),
        warnings: Vec::new(),
        operations: vec![PlanOperation::CopyBinaryFile {
            source: temp_dir.path().join("missing.bin"),
            destination: output_dir.join("missing.bin"),
        }],
    };

    let err = materialize_generation_plan(&plan, MaterializationOptions::no_force())
        .expect_err("missing binary source should fail");

    assert!(err.to_string().contains("failed to copy binary file"));
}

#[test]
fn materialization_refuses_to_overwrite_existing_binary_without_force() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let source = temp_dir.path().join("source.bin");
    let output_dir = temp_dir.path().join("out");
    let destination = output_dir.join("source.bin");

    fs::create_dir_all(&output_dir).expect("output dir should be created");
    fs::write(&source, [1_u8, 2, 3]).expect("binary source should be written");
    fs::write(&destination, [4_u8, 5, 6]).expect("existing binary should be written");

    let plan = GenerationPlan {
        template_id: TemplateId::new("test-template").expect("template id should be valid"),
        output_dir,
        warnings: Vec::new(),
        operations: vec![PlanOperation::CopyBinaryFile {
            source,
            destination,
        }],
    };

    let err = materialize_generation_plan(&plan, MaterializationOptions::no_force())
        .expect_err("existing binary should fail without force");

    assert!(err.to_string().contains("already exists"));
}

#[test]
fn materialization_reports_text_destination_that_is_directory() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("out");
    let destination = output_dir.join("README.md");

    fs::create_dir_all(&destination).expect("directory destination should be created");

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

    let err = materialize_generation_plan(&plan, MaterializationOptions::force())
        .expect_err("writing text to a directory should fail");

    assert!(err.to_string().contains("failed to write text file"));
}

#[test]
fn create_directory_reports_path_that_is_file() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let file_path = temp_dir.path().join("not-a-directory");

    fs::write(&file_path, "file").expect("file should be written");

    let err = create_directory(&file_path).expect_err("file path cannot be created as directory");

    assert!(matches!(*err, FsError::CreateDirectory { .. }));
}

#[test]
fn materialization_reports_text_parent_that_is_file() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("out");
    let parent_file = output_dir.join("parent");

    fs::create_dir_all(&output_dir).expect("output dir should be created");
    fs::write(&parent_file, "file").expect("parent file should be written");

    let plan = GenerationPlan {
        template_id: TemplateId::new("test-template").expect("template id should be valid"),
        output_dir,
        warnings: Vec::new(),
        operations: vec![PlanOperation::RenderTextFile {
            source: PathBuf::from("template/README.md"),
            destination: parent_file.join("README.md"),
            content: String::from("new"),
        }],
    };

    let err = materialize_generation_plan(&plan, MaterializationOptions::force())
        .expect_err("parent file should prevent text write");

    assert!(matches!(*err, FsError::CreateParentDirectory { .. }));
}

#[test]
fn materialization_reports_binary_parent_that_is_file() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let source = temp_dir.path().join("source.bin");
    let output_dir = temp_dir.path().join("out");
    let parent_file = output_dir.join("parent");

    fs::create_dir_all(&output_dir).expect("output dir should be created");
    fs::write(&source, [1_u8, 2, 3]).expect("binary source should be written");
    fs::write(&parent_file, "file").expect("parent file should be written");

    let plan = GenerationPlan {
        template_id: TemplateId::new("test-template").expect("template id should be valid"),
        output_dir,
        warnings: Vec::new(),
        operations: vec![PlanOperation::CopyBinaryFile {
            source,
            destination: parent_file.join("source.bin"),
        }],
    };

    let err = materialize_generation_plan(&plan, MaterializationOptions::force())
        .expect_err("parent file should prevent binary copy");

    assert!(matches!(*err, FsError::CreateParentDirectory { .. }));
}

#[test]
fn init_skeleton_derives_id_and_uses_custom_metadata() {
    let temp_dir = TempDir::new().expect("tempdir should be created");
    let output_dir = temp_dir.path().join("My Cool Template");

    let input = InitTemplateInput {
        output_dir: output_dir.clone(),
        template_id: None,
        name: Some(String::from("Custom Name")),
        description: Some(String::from("Custom description")),
        force: false,
    };

    init_template_skeleton(&input).expect("template skeleton should be initialized");

    let manifest = Manifest::load_from_path(output_dir.join("zappy.toml"))
        .expect("generated manifest should parse");
    let readme = fs::read_to_string(output_dir.join("template/README.md"))
        .expect("generated README should be readable");

    assert_eq!(manifest.template.id.as_str(), "my-cool-template");
    assert_eq!(manifest.template.name, "Custom Name");
    assert_eq!(
        manifest.template.description.as_deref(),
        Some("Custom description")
    );
    assert!(readme.contains("# Custom Name"));
    assert!(readme.contains("Custom description"));
}
