use indexmap::IndexMap;
use serde::Deserialize;

use crate::{
    error::{CoreError, CoreResult},
    hooks::{validate_hooks, HookSpec, RawHookSpec},
    variables::VariableValue,
};

/// Template validation configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationConfig {
    /// Validation output dir.
    pub output_dir_name: Option<String>,

    /// Validation variables.
    pub variables: IndexMap<String, VariableValue>,

    /// Validation setup command hooks config.
    pub setup: Vec<HookSpec>,

    /// Validation steps command hooks config.
    pub steps: Vec<HookSpec>,

    /// Validation teardown command hooks config.
    pub teardown: Vec<HookSpec>,
}

/// Raw template validation config from template manifest.
#[derive(Debug, Deserialize)]
pub(crate) struct RawValidationConfig {
    /// Validation output dir slug
    #[serde(default)]
    pub output_dir_name: Option<String>,

    /// Validation variables.
    #[serde(default)]
    pub variables: IndexMap<String, VariableValue>,

    /// Validation setup command hooks config.
    #[serde(default)]
    pub setup: Vec<RawHookSpec>,

    /// Validation steps command hooks config.
    #[serde(default)]
    pub steps: Vec<RawHookSpec>,

    /// Validation teardown command hooks config.
    #[serde(default)]
    pub teardown: Vec<RawHookSpec>,
}

impl TryFrom<RawValidationConfig> for ValidationConfig {
    type Error = CoreError;

    fn try_from(raw: RawValidationConfig) -> CoreResult<Self> {
        if let Some(output_dir_name) = raw.output_dir_name.as_ref() {
            if output_dir_name.trim().is_empty() {
                return Err(CoreError::invalid_manifest(
                    "`validation.output_dir_name` must not be empty",
                ));
            }
        }

        Ok(Self {
            output_dir_name: raw.output_dir_name,
            variables: raw.variables,
            setup: validate_hooks("validation.setup", raw.setup)?,
            steps: validate_hooks("validation.steps", raw.steps)?,
            teardown: validate_hooks("validation.teardown", raw.teardown)?,
        })
    }
}
