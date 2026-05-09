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
    #[error("invalid value `{value}` for variable `{name}, expected one of: {choices}`")]
    InvalidVariableChoice {
        /// Variable name.
        name: String,

        /// Invalid value.
        value: String,

        /// Allowed choices.
        choices: String,
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
