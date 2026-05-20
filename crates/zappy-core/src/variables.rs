use indexmap::IndexMap;
use serde::Deserialize;

use crate::builtins::is_builtin_name;
use crate::error::{CoreError, CoreResult};
use crate::template::validate_field;

/// Type alias for an `IndexMap` of template variables.
pub type VariableMap = IndexMap<String, VariableSpec>;
/// Type alias for an `IndexMap` of raw template variables.
pub(crate) type RawVariableMap = IndexMap<String, RawVariableSpec>;
/// Type alias for resolved variable values.
pub type VariableValueMap = IndexMap<String, VariableValue>;

/// Template variable specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableSpec {
    /// Variable prompt.
    pub prompt: Option<String>,

    /// Default value.
    pub default: Option<VariableValue>,

    /// Is the variable required?
    pub required: bool,

    /// Variable required when other boolean variable provided.
    pub required_when: Option<String>,

    /// Other variable name this var conflicts with.
    ///
    /// e.g. `use_pnpm` and `use_yarn` in a js/ts project template
    pub conflicts_with: Vec<String>,

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

impl VariableValue {
    /// Parses value from a CLI override.
    ///
    /// Boolean and integer values are parsed into typed values.
    /// Everything else remains a string.
    #[must_use]
    pub fn parse_cli_value(value: &str) -> Self {
        match value {
            "true" | "yes" | "y" | "Y" | "1" => Self::Bool(true),
            "false" | "no" | "n" | "N" | "0" => Self::Bool(false),
            _ => value
                .parse::<i64>()
                .map_or_else(|_| Self::String(String::from(value)), Self::Integer),
        }
    }

    /// Converts variable value into a rederable string.
    #[must_use]
    pub fn render(&self) -> String {
        match self.to_owned() {
            Self::String(value) => value,
            Self::Bool(value) => {
                if value {
                    String::from("true")
                } else {
                    String::from("false")
                }
            }
            Self::Integer(value) => format!("{value}"),
        }
    }
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

    /// Raw variable `required_when` slug.
    #[serde(default)]
    pub required_when: Option<String>,

    /// Raw `conflicts_with` variable slugs.
    #[serde(default)]
    pub conflicts_with: Vec<String>,

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

        for (name, spec) in &validated {
            if let Some(var) = spec.required_when.as_ref() {
                validate_variable_name(var)?;
                validate_regex_syntax(name, spec.validation_regex.as_deref())?;

                if name == var {
                    return Err(CoreError::RequireSelf {
                        name: name.to_owned(),
                    });
                }

                if !validated.contains_key(var) {
                    return Err(CoreError::InvalidRequiredWhen {
                        name: name.to_owned(),
                        when: var.to_owned(),
                    });
                }
            }

            for var in &spec.conflicts_with {
                validate_variable_name(var)?;

                if name == var {
                    return Err(CoreError::ConflictsWithSelf {
                        name: name.to_owned(),
                    });
                }

                if !validated.contains_key(var) {
                    return Err(CoreError::InvalidConflictsWith {
                        name: name.to_owned(),
                        conflict: var.to_owned(),
                    });
                }
            }
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

        if raw
            .prompt
            .as_ref()
            .is_some_and(|prompt| prompt.trim().is_empty())
        {
            return Err(CoreError::invalid_manifest(
                "variable prompt must not be empty when provided",
            ));
        }

        if (raw.required || raw.required_when.is_some())
            && raw.default.is_none()
            && raw.prompt.is_none()
        {
            return Err(CoreError::invalid_manifest(
                "required variable without a default value should define a prompt",
            ));
        }

        Ok(Self {
            prompt: raw.prompt,
            default: raw.default,
            required: raw.required,
            required_when: raw.required_when,
            conflicts_with: raw.conflicts_with,
            choices: raw.choices,
            validation_regex: raw.validation_regex,
            transforms: raw.transforms,
            placeholders: raw.placeholders,
        })
    }
}

/// Parses CLI-style variable overrides in `key=value` form.
///
/// # Arguments
/// - `overrides`: vector/list of passed in vvariable overrides.
///
/// # Returns
/// [`VariableValueMap`] of parsed variable values.
///
/// # Errors
/// Returns [`CoreError::InvalidVariableOverride`] on invalid passed in overrides.
pub fn parse_variable_overrides<I, S>(overrides: I) -> CoreResult<VariableValueMap>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut values = VariableValueMap::new();

    for override_value in overrides {
        let override_value = override_value.as_ref();

        let Some((name, value)) = override_value.split_once('=') else {
            return Err(CoreError::InvalidVariableOverride {
                value: String::from(override_value),
            });
        };

        if name.trim().is_empty() {
            return Err(CoreError::InvalidVariableOverride {
                value: String::from(override_value),
            });
        }

        validate_variable_name(name)?;

        values.insert(String::from(name), VariableValue::parse_cli_value(value));
    }

    Ok(values)
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
/// Returns [`CoreError::InvalidManifest`] if:
/// - name empty,
/// - name contains characters outside ASCII letters and `_`.
pub(crate) fn validate_variable_name(name: &str) -> CoreResult<()> {
    validate_field("variable.name", name)?;

    if name.contains('-') {
        return Err(CoreError::invalid_manifest(format!(
            "variable name `{name}` must not contain `-`, use `_` instead"
        )));
    }

    if is_builtin_name(name) {
        return Err(CoreError::ReservedVariableName {
            name: String::from(name),
        });
    }

    Ok(())
}

/// Validates variable regex syntax.
fn validate_regex_syntax(name: &str, regex: Option<&str>) -> CoreResult<()> {
    let Some(regex) = regex else {
        return Ok(());
    };

    if regex.trim().is_empty() {
        return Err(CoreError::InvalidManifest {
            message: format!("validation_regex for variable `{name}` must not be empty"),
        });
    }

    regex::Regex::new(regex).map_err(|source| CoreError::InvalidVariableRegex {
        name: String::from(name),
        regex: String::from(regex),
        source,
    })?;

    Ok(())
}
