use std::path::{Path, PathBuf};

use crate::materialize::write_text_file;
use crate::{FsError, FsResult, create_directory};

/// Input metadata for tempalte skeleton generation.
#[derive(Debug, Clone)]
pub struct InitTemplateInput {
    /// Template skeleton output dir.
    pub output_dir: PathBuf,

    /// Template skeleton template id.
    pub template_id: Option<String>,

    /// Template skeleton template name.
    pub name: Option<String>,

    /// Template skeleton description.
    pub description: Option<String>,

    /// Force overwriting existing files?
    pub force: bool,
}

/// Generates a template skeleton for the given input.
///
/// # Arguments
/// - `input`: template skeleton config input.
///
/// # Returns
/// Ok(()) on success.
///
/// # Errors
/// Returns [`FsError`] if:
/// - destination already exists and `--force` was not provided,
/// - underlying file writes faile.
pub fn init_template_skeleton(input: &InitTemplateInput) -> FsResult<()> {
    ensure_output_can_be_written(&input.output_dir, input.force)?;

    let template_dir = input.output_dir.join("template");

    create_directory(&template_dir)?;

    let template_id = input
        .template_id
        .clone()
        .unwrap_or_else(|| template_id_from_output_path(&input.output_dir));
    let name = input.name.clone().unwrap_or_else(|| template_id.clone());
    let description = input
        .description
        .clone()
        .unwrap_or_else(|| String::from("TODO: describe this template"));

    let manifest_path = input.output_dir.join("zappy.toml");
    let manifest = render_manifest_skeleton(&template_id, &name, &description);
    write_text_file(&manifest_path, &manifest, input.force)?;

    let readme_path = template_dir.join("README.md");
    let readme = render_readme_skeleton(&name, &description);
    write_text_file(&readme_path, &readme, input.force)?;

    Ok(())
}

/// Renders template skeleton manifest file.
fn render_manifest_skeleton(id: &str, name: &str, description: &str) -> String {
    format!(
        r#"# Template metadata.
[template]
id = "{id}"
name = "{name}"
description = "{description}"
language = "other"
version = "0.0.1"

# Template source directory.
[template.source]
root = "template"

# Template variable definitions.
[variables]
test_var = {{
    required = true,
    prompt = "Test variable",
    transforms = ["raw", "kebab", "snake", "pascal", "camel", "screaming_snake", "upper", "lower"],
    placeholders = {{
        raw             = "__TEST_VAR__",
        kebab           = "__TEST_VAR_KEBAB__",
        snake           = "__TEST_VAR_SNAKE__",
        pascal          = "__TEST_VAR_PASCAL__",
        camel           = "__TEST_VAR_CAMEL__",
        screaming_snake = "__TEST_VAR_SCREAMING__",
        upper           = "__TEST_VAR_UPPER__",
        lower           = "__TEST_VAR_LOWER__",
   }}
}}

include_optional = {{ default = false }}

# Template file path config.
[paths]
exclude = [".git", ".DS_Store", "target", "node_modules"]
binary_extensions = ["png", "jpg", "jpeg", "gif", "ico", "pdf", "zip"]
binary_files = []

# Conditional include paths.
[[conditionals]]
path = "optional_file.txt"
when = "include_optional"

# Template generation hooks
[hooks]

# Template validation config.
# [validation]
# output_dir_name = "zappy-validation-{id}"
# variables = {{
#     test_var = "validation test var value",
#     include_optional = true,
# }}

# Validation setup hooks.
# [[validation.setup]]

# Validation process steps.
# [[validation.steps]]

# Validation teardown hooks.
# [[validation.teardown]]

"#
    )
}

/// Renders template skeleton readme file.
fn render_readme_skeleton(name: &str, description: &str) -> String {
    format!(
        r"# {name}

{description}
"
    )
}

/// Checks whether a destination can be written.
///
/// # Errors
/// Returns [`FsError::DestinationExists`] if destination already exists
/// and `--force` was not provided.
fn ensure_output_can_be_written(path: &Path, force: bool) -> FsResult<()> {
    if path.exists() && !force {
        return Err(Box::new(FsError::DestinationExists {
            path: path.to_path_buf(),
        }));
    }

    Ok(())
}

/// Generates template-id from a path.
fn template_id_from_output_path(path: &Path) -> String {
    path.file_name().and_then(|name| name.to_str()).map_or_else(
        || String::from("new-template"),
        heck::ToKebabCase::to_kebab_case,
    )
}
