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

        if let Some(validation) = validation.as_ref() {
            validate_validation_variables(&variables, validation)?;
        }

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
/// Returns [`CoreError::InvalidManifest`] if the list contains an empty string item.
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

/// Validates validation variable names.
///
/// Validates whether validation config variables
/// reference variables defined in template manifest.
///
/// # Arguments
/// - `variables`: template manifest variable map to validate against,
/// - `validation`: validation config which variables to validate.
///
/// # Returns
/// Ok(()) on successful validation.
///
/// # Errors
/// Returns [`CoreError::InvalidManifest`] if validation config contains
/// a variable that is not defined in the template manifest.
fn validate_validation_variables(
    variables: &VariableMap,
    validation: &ValidationConfig,
) -> CoreResult<()> {
    for name in validation.variables.keys() {
        if !variables.contains_key(name) {
            return Err(CoreError::invalid_manifest(format!(
                "`validation.variables.{name}` does not reference a declared template variable"
            )));
        }
    }

    Ok(())
}
