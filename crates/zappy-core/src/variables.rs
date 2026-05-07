use indexmap::IndexMap;
use serde::Deserialize;

use crate::error::{CoreError, CoreResult};
use crate::template::validate_field;

/// Type alias for an `IndexMap` of template variables.
pub type VariableMap = IndexMap<String, VariableSpec>;
/// Type alias for an `IndexMap` of raw template variables.
pub(crate) type RawVariableMap = IndexMap<String, RawVariableSpec>;

/// Template variable specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableSpec {
    /// Variable prompt.
    pub prompt: Option<String>,

    /// Default value.
    pub default: Option<VariableValue>,

    /// Is the variable required?
    pub required: bool,

    /// List of choiches for the variable.
    pub choices: Vec<VariableValue>,

    /// Regex used to validate variable.
    pub validation_regex: Option<String>,

    /// Transformations supported for the variable.
    pub transforms: Vec<TransformKind>,

    /// Transformed placeholders.
    pub placeholders: IndexMap<TransformKind, String>,
}

/// Manifest-level variable value.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum VariableValue {
    /// String variable.
    String(String),

    /// Boolean variable.
    Bool(bool),

    /// Integer variable.
    Integer(i64),
}

/// Supported variable transformations.
///
/// So far only parsing and validation supported. Actual
/// transformations will be implemented in Phase 3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransformKind {
    /// No transform.
    Raw,

    /// Kebab-case transform.
    Kebab,

    /// Snake-case transform.
    Snake,

    /// Pascal-case transform.
    Pascal,

    /// Camel-case transform.
    Camel,

    /// Screaming snake case transform
    ScreamingSnake,

    /// Upper-case transform.
    Upper,

    /// Lower-case transform.
    Lower,
}

/// Raw variable specification from template manifest.
#[derive(Debug, Deserialize)]
pub(crate) struct RawVariableSpec {
    /// Variable spec prompt slug.
    #[serde(default)]
    pub prompt: Option<String>,

    /// Variable default value slug.
    #[serde(default)]
    pub default: Option<VariableValue>,

    /// Raw variable required value.
    #[serde(default)]
    pub required: bool,

    /// Variable choice slugs.
    #[serde(default)]
    pub choices: Vec<VariableValue>,

    /// Variable validation regex slug.
    #[serde(default)]
    pub validation_regex: Option<String>,

    /// Variable transform slugs.
    #[serde(default)]
    pub transforms: Vec<TransformKind>,

    /// Variable transform placeholder slugs.
    #[serde(default)]
    pub placeholders: IndexMap<TransformKind, String>,
}

impl RawVariableSpec {
    /// Validates raw variable map.
    ///
    /// # Arguments
    /// - `variables`: index map of raw parsed template variables from manifest.
    ///
    /// # Returns
    /// Validated [`VariableMap`].
    ///
    /// # Errors
    /// Returns [`CoreError`] if:
    /// - variable name is invalid,
    /// - required variable is missing a default value and prompt,
    /// - variable transform config is invalid.
    pub(crate) fn validate_map(variables: RawVariableMap) -> CoreResult<VariableMap> {
        let mut validated = IndexMap::new();

        for (name, raw) in variables {
            validate_variable_name(&name)?;

            let spec = VariableSpec::try_from(raw)?;

            validated.insert(name, spec);
        }

        Ok(validated)
    }
}

impl TryFrom<RawVariableSpec> for VariableSpec {
    type Error = CoreError;

    fn try_from(raw: RawVariableSpec) -> CoreResult<Self> {
        for (transform, placeholder) in &raw.placeholders {
            if placeholder.trim().is_empty() {
                return Err(CoreError::invalid_manifest(format!(
                    "placeholder for transform `{transform:?}` must not be empty"
                )));
            }
        }

        if raw.required && raw.default.is_none() && raw.prompt.is_none() {
            return Err(CoreError::invalid_manifest(
                "required variable without a default value should define a prompt",
            ));
        }

        Ok(Self {
            prompt: raw.prompt,
            default: raw.default,
            required: raw.required,
            choices: raw.choices,
            validation_regex: raw.validation_regex,
            transforms: raw.transforms,
            placeholders: raw.placeholders,
        })
    }
}

/// Validates variable name.
///
/// # Arguments
/// - `name`: Variable name to validate.
///
/// # Returns
/// Ok(()) on successful validation.
///
/// # Errors
/// Returns [`CoreError`] if:
/// - name empty,
/// - name contains characters outside ASCII letters and `_`.
fn validate_variable_name(name: &str) -> CoreResult<()> {
    validate_field("variable.name", name)?;

    if name.contains('-') {
        return Err(CoreError::invalid_manifest(format!(
            "variable name `{name}` must not contain `-`, use `_` instead"
        )));
    }

    Ok(())
}
