use std::path::PathBuf;

use thiserror::Error;

/// Result type alias used by `zappy-core`.
pub type CoreResult<T> = std::result::Result<T, CoreError>;

/// Core-domain errors.
#[derive(Debug, Error)]
pub enum CoreError {
    /// Failed to read a manifest file.
    #[error("failed to read manifest `{path}`")]
    ReadManifest {
        /// Manifest path.
        path: PathBuf,

        /// Underlying IO error.
        #[source]
        source: std::io::Error,
    },

    /// Failed to parse TOML manifest.
    #[error("failed to parse manifest `{path}`")]
    ParseManifest {
        /// Manifest path.
        path: PathBuf,

        /// Underlying toml deserialization error.
        #[source]
        source: toml::de::Error,
    },

    /// Manifest content is structurally valid, but has invalid config.
    #[error("invalid manifest: {message}")]
    InvalidManifest {
        /// Human-readable validation message.
        message: String,
    },

    /// Invalid CLI-style variable override.
    #[error("invalid variable override `{value}`, expected `key=value`")]
    InvalidVariableOverride {
        /// Invalid override string,
        value: String,
    },

    /// Unknown variable provided by an input source.
    #[error("unknown variable `{name}` in {vsource}")]
    UnknownVariable {
        /// Variable name.
        name: String,

        /// Source, where the variable was provided.
        vsource: &'static str,
    },

    /// A required variable was not resovled/provided.
    #[error("missing required variable `{name}`")]
    MissingRequiredVariable {
        /// Variable name.
        name: String,
    },

    /// Variable value is not one of the manifest-defined choices.
    #[error("invalid value `{value}` for variable `{name}`, expected one of: [{choices}]")]
    InvalidVariableChoice {
        /// Variable name.
        name: String,

        /// Invalid value.
        value: String,

        /// Allowed choices.
        choices: String,
    },

    /// Required when variable not found in the manifest.
    #[error("chosen `required_when` variable for `{name}`: `{when}` was not found in manifest")]
    InvalidRequiredWhen {
        /// Variable name.
        name: String,

        /// Required when choice.
        when: String,
    },

    /// Variable requires self throug `required_when`.
    #[error("variable `{name}` cannot define itself in `required_when`")]
    RequireSelf {
        /// Variable name.
        name: String,
    },

    /// Conditonally required variabe not provided.
    #[error("variable `{name}` is required when `{when}` is enabled")]
    ConditionallyRequiredVariable {
        /// Variable name.
        name: String,

        /// Required condition.
        when: String,
    },

    /// `conflicts_with` variable not found in the manifest.
    #[error("chosen conflicting variable for `{name}`: `{conflict}` not found in manifest")]
    InvalidConflictsWith {
        /// Variable name.
        name: String,

        /// Conflicting variable choice.
        conflict: String,
    },

    /// Variable defines self in `conflicts_with`.
    #[error("variable `{name}` cannot define itself in `conflicts_with`")]
    ConflictsWithSelf {
        /// Variable name.
        name: String,
    },

    /// Conflicting variables.
    #[error("variables `{left}` and `{right}` cannot be both enabled")]
    ConflictingVariables {
        /// Variable 1 name.
        left: String,

        /// Variable 2 name.
        right: String,
    },

    /// Variable name reserved by a built-in variable.
    #[error("[variable.{name}] is invalid because `{name}` is a reserved built-in variable")]
    ReservedVariableName {
        /// Variable name.
        name: String,
    },

    /// Rendered path is invalid.
    #[error("invalid rendered path `{path}`: {reason}")]
    InvalidRenderedPath {
        /// Rendered path.
        path: String,

        /// Validation failure reason.
        reason: String,
    },

    /// Unknown built-in variable was provided by the caller.
    #[error("unknown built-in variable `{name}`")]
    UnknownBuiltinVariable {
        /// Built-in variable name.
        name: String,
    },
}

impl CoreError {
    /// Constructs `InvalidManifest` error.
    ///
    /// # Arguments
    /// - `message`: human-readable validation message.
    ///
    /// # Returns
    /// Constructed [`CoreError::InvalidManifest`] error.
    pub(crate) fn invalid_manifest(message: impl Into<String>) -> Self {
        Self::InvalidManifest {
            message: message.into(),
        }
    }
}
