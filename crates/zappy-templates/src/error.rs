use std::path::PathBuf;

use thiserror::Error;

/// Result alias used by `zappy-templates`.
pub type TemplatesResult<T> = std::result::Result<T, Box<TemplatesError>>;

/// Errors produced while managing bundled template cache files.
#[derive(Debug, Error)]
pub enum TemplatesError {
    /// Bundled templates cache directory could not have been resolved.
    #[error("failed to resolve bundled template cache directory")]
    ResolveCacheDirectory,

    /// Failed to clear old bundled template cache.
    #[error("failed to clear bundled template cache: `{path}`")]
    ClearCache {
        /// Cache path.
        path: PathBuf,

        /// Underlying IO error.
        #[source]
        source: std::io::Error,
    },

    /// Failed to create a directory.
    #[error("failed to create bundled template directory `{path}`")]
    CreateDirectory {
        /// Directory path.
        path: PathBuf,

        /// Underlying IO error.
        #[source]
        source: std::io::Error,
    },

    /// Failed to write a bundled template file.
    #[error("failed to write bundled template file `{path}`")]
    WriteFile {
        /// File path.
        path: PathBuf,

        /// Underlying IO error.
        #[source]
        source: std::io::Error,
    },
}
