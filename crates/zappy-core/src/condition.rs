use camino::Utf8PathBuf;
use serde::Deserialize;

use crate::error::{CoreError, CoreResult};
use crate::template::validate_path;
use crate::{VariableValue, VariableValueMap};

/// A path included only when a variable condition evaluates to true.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalPath {
    /// File path.
    pub path: Utf8PathBuf,
    /// Path include condition.
    pub when: String,
}

/// Raw conditional path from template manifest.
#[derive(Debug, Deserialize)]
pub(crate) struct RawConditionalPath {
    /// Raw file path.
    pub path: Utf8PathBuf,
    /// Raw include condition slug.
    pub when: String,
}

impl TryFrom<RawConditionalPath> for ConditionalPath {
    type Error = CoreError;

    fn try_from(raw: RawConditionalPath) -> CoreResult<Self> {
        validate_path("conditional path", &raw.path)?;

        if raw.when.trim().is_empty() {
            return Err(CoreError::invalid_manifest(
                "conditional path `when` field must not be empty",
            ));
        }

        Ok(Self {
            path: raw.path,
            when: raw.when,
        })
    }
}

/// Evaluates a boolean condition variable.
///
/// Missing or non-boolean values evaluate to false for now.
#[must_use]
pub fn evaluate_condition(variable_name: &str, values: &VariableValueMap) -> bool {
    matches!(values.get(variable_name), Some(VariableValue::Bool(true)))
}
