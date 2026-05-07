use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::condition::{ConditionalPath, RawConditionalPath};
use crate::error::{CoreError, CoreResult};
use crate::hooks::{HookConfig, RawHookConfig};
use crate::template::{RawTemplateMetadata, TemplateMetadata};
use crate::validation::{RawValidationConfig, ValidationConfig};
use crate::variables::{RawVariableMap, RawVariableSpec, VariableMap};

/// Fully validated Zappy template manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    /// Template metadata.
    pub template: TemplateMetadata,

    /// Template variables.
    pub variables: VariableMap,

    /// Path-related config.
    pub paths: PathConfig,

    /// Path conditionals.
    pub conditionals: Vec<ConditionalPath>,

    /// Command hooks config.
    pub hooks: HookConfig,

    /// Template validation config.
    pub validation: Option<ValidationConfig>,
}

/// Path-related template configuration.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PathConfig {
    /// Exclude files/dirs.
    pub exclude: Vec<String>,

    /// Binary file extensions.
    pub binary_extensions: Vec<String>,

    /// Binary files.
    pub binary_files: Vec<String>,
}

/// Raw template manifest.
#[derive(Debug, Deserialize)]
struct RawManifest {
    /// Template metadata.
    pub template: RawTemplateMetadata,

    /// Template variables.
    #[serde(default)]
    pub variables: RawVariableMap,

    /// Path-related config.
    #[serde(default)]
    pub paths: RawPathConfig,

    /// Path conditionals.
    #[serde(default)]
    pub conditionals: Vec<RawConditionalPath>,

    /// Command hooks config.
    #[serde(default)]
    pub hooks: RawHookConfig,

    /// Template validation config.
    #[serde(default)]
    pub validation: Option<RawValidationConfig>,
}

/// Raw path-related template config from template manifest.
#[derive(Debug, Default, Deserialize)]
struct RawPathConfig {
    /// Exclude files/dirs.
    #[serde(default)]
    pub exclude: Vec<String>,

    /// Binary file extensions.
    #[serde(default)]
    pub binary_extensions: Vec<String>,

    /// Binary files.
    #[serde(default)]
    pub binary_files: Vec<String>,
}

impl Manifest {
    /// Loads and parses template manifest from a file.
    ///
    /// # Arguments
    /// - `path`: manifest file path.
    ///
    /// # Returns
    /// Parsed and validated template [`Manifest`] os success.
    ///
    /// # Errors
    /// Returns following errors:
    /// - [`CoreError::ReadManifest`] if fails to read manifest file,
    /// - [`CoreError::ParseManifest`] if fails to parse manifest file,
    /// - [`CoreError::InvalidManifest`] if validation fails.
    pub fn load_from_path(path: impl AsRef<Path>) -> CoreResult<Self> {
        let path = path.as_ref();
        let contents = fs::read_to_string(path).map_err(|source| CoreError::ReadManifest {
            path: path.to_path_buf(),
            source,
        })?;

        Self::from_toml_str(&contents, path)
    }

    /// Parses template manifest from file contents.
    ///
    /// # Arguments
    /// - `contents`: file contents to parse,
    /// - `path`: file path the contents belong to.
    ///
    /// # Returns
    /// Parsed and validated template [`Manifest`] on success.
    ///
    /// # Errors
    /// Returns following errors:
    /// - [`CoreError::ParseManifest`] if fails to parse file contents,
    /// - [`CoreError::InvalidManifest`] if validation fails.
    pub fn from_toml_str(contents: &str, path: impl AsRef<Path>) -> CoreResult<Self> {
        let path = path.as_ref();

        let raw =
            toml::from_str::<RawManifest>(contents).map_err(|source| CoreError::ParseManifest {
                path: path.to_path_buf(),
                source,
            })?;

        Self::try_from(raw)
    }
}

impl TryFrom<RawManifest> for Manifest {
    type Error = CoreError;

    fn try_from(raw: RawManifest) -> CoreResult<Self> {
        let template = TemplateMetadata::try_from(raw.template)?;
        let variables = RawVariableSpec::validate_map(raw.variables)?;
        let paths = PathConfig::try_from(raw.paths)?;

        let conditionals = raw
            .conditionals
            .into_iter()
            .map(ConditionalPath::try_from)
            .collect::<CoreResult<Vec<_>>>()?;

        let hooks = HookConfig::try_from(raw.hooks)?;

        let validation = raw.validation.map(ValidationConfig::try_from).transpose()?;

        Ok(Self {
            template,
            variables,
            paths,
            conditionals,
            hooks,
            validation,
        })
    }
}

impl TryFrom<RawPathConfig> for PathConfig {
    type Error = CoreError;

    fn try_from(raw: RawPathConfig) -> CoreResult<Self> {
        validate_non_empty_list_items("paths.exclude", &raw.exclude)?;
        validate_non_empty_list_items("paths.binary_extensions", &raw.binary_extensions)?;
        validate_non_empty_list_items("paths.binary_files", &raw.binary_files)?;

        Ok(Self {
            exclude: raw.exclude,
            binary_extensions: raw.binary_extensions,
            binary_files: raw.binary_files,
        })
    }
}

/// Validates non-empty items of a list.
///
/// # Arguments
/// - `section`: manifest section being inspected,
/// - `values`: list to inspect.
///
/// # Returns
/// Ok(()) on successful validation.
///
/// # Errors
/// Returns [`CoreError`] if the list contains an empty string item.
fn validate_non_empty_list_items(section: &str, values: &[String]) -> CoreResult<()> {
    for value in values {
        if value.trim().is_empty() {
            return Err(CoreError::invalid_manifest(format!(
                "`{section}` must not contain empty string entries"
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
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

        let err = Manifest::from_toml_str(source, "zappy.toml")
            .expect_err("empty template id should fail");

        assert!(err.to_string().contains("template.id"));
    }

    #[test]
    fn rejects_empty_template_name() {
        let source = r#"
[template]
id = "broken"
name = ""
"#;

        let err = Manifest::from_toml_str(source, "zappy.toml")
            .expect_err("empty template name should fail");

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

        let manifest =
            Manifest::from_toml_str(source, "zappy.toml").expect("manifest should parse");

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

[validation.variables]
project_name = "zappy-test-cli"

[[validation.steps]]
name = "Test"
command = "cargo"
args = ["test"]
"#;

        let manifest =
            Manifest::from_toml_str(source, "zappy.toml").expect("manifest should parse");

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

        let err = Manifest::from_toml_str(source, "zappy.toml")
            .expect_err("empty hook command should fail");

        assert!(err.to_string().contains("hook command"));
    }
}
