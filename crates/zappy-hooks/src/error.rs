use std::path::PathBuf;

use thiserror::Error;
use zappy_core::CoreError;

/// Result alias used by `zappy-hook`.
pub type HooksResult<T> = std::result::Result<T, Box<HooksError>>;

/// Hook execution errors.
#[derive(Debug, Error)]
pub enum HooksError {
    /// Shell hooks not supported yet.
    #[error("hook `{name}` requested shell execution, which is not supported yet")]
    ShellUnsupported {
        /// Hook display name.
        name: String,
    },

    /// Failed to spawn hook command.
    #[error("failed to spawn hook `{name}` command: {command}")]
    Spawn {
        /// Hook display name.
        name: String,

        /// Hook command.
        command: String,

        /// Working directory.
        working_dir: PathBuf,

        /// Underlying IO error.
        #[source]
        source: std::io::Error,
    },

    /// Hook command failed.
    #[error("hook `{name}` failed with {status}:\n{stdout}\n\n{stderr}")]
    Failed {
        /// Hook display name.
        name: String,

        /// Exit status as text.
        status: String,

        /// Captured stdout.
        stdout: String,

        /// Captured stderr.
        stderr: String,
    },

    /// Failed to render hook working directory.
    #[error("failed to render hook working directory for `{name}`")]
    RenderWorkingDir {
        /// Hook display name.
        name: String,

        /// Underlying zappy core error.
        #[source]
        source: CoreError,
    },
}
