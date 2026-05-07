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
