use std::path::PathBuf;

use thiserror::Error;

/// Result alias used by `zappy-fs`.
pub type FsResult<T> = std::result::Result<T, Box<FsError>>;

/// Filesystem/discovery errors.
#[derive(Debug, Error)]
pub enum FsError {
    /// Failed to resolve search paths (none were discovered).
    #[error("failed to resolve search paths")]
    ResolveSearchPaths,

    /// Required template search path does not exist.
    #[error("template search path `{path}` does not exist")]
    SearchPathMissing {
        /// Missing search path.
        path: PathBuf,
    },

    /// Required template search path is not a directory.
    #[error("template search path `{path}` is not a directory")]
    SearchPathNotDirectory {
        /// Invalid search path.
        path: PathBuf,
    },

    /// Failed to read a template search directory.
    #[error("failed to read template search path `{path}`")]
    ReadSearchPath {
        /// Search path being read.
        path: PathBuf,

        /// Underlying IO error.
        #[source]
        source: std::io::Error,
    },

    /// Failed to read a directory entry.
    #[error("failed to read directory entry under `{path}`")]
    ReadDirectoryEntry {
        /// Parent directory path.
        path: PathBuf,

        /// Underlying IO error.
        #[source]
        source: std::io::Error,
    },

    /// Failed to inspect a directory entry file type.
    #[error("failed to inspect file type for `{path}`")]
    FileType {
        /// File path.
        path: PathBuf,

        /// Underlying IO error.
        #[source]
        source: std::io::Error,
    },

    /// Failed to load template manifes.
    #[error("failed to load template manifest `{path}`")]
    LoadManifest {
        /// Manifest path.
        path: PathBuf,

        /// Underlying core error.
        #[source]
        source: zappy_core::CoreError,
    },

    /// Failed to render a template file path.
    #[error("failed to render template path")]
    RenderPath {
        /// Underlying core error.
        #[source]
        source: zappy_core::CoreError,
    },

    /// Failed to read template file.
    #[error("failed to read template file `{path}`")]
    ReadTemplateFile {
        /// Template file path.
        path: PathBuf,

        /// Underlying IO error.
        #[source]
        source: std::io::Error,
    },

    /// Destination already exists and --force was not enabled.
    #[error("destination `{path}` already exists")]
    DestinationExists {
        /// Existing destination path.
        path: PathBuf,
    },

    /// Dailed to create a directory.
    #[error("failed to create directory `{path}`")]
    CreateDirectory {
        /// Directory path.
        path: PathBuf,

        /// Underlying IO error.
        #[source]
        source: std::io::Error,
    },

    /// Failed to write a rendered text file.
    #[error("failed to write text file `{path}`")]
    WriteTextFile {
        /// Destination file path.
        path: PathBuf,

        /// Underlying IO error.
        #[source]
        source: std::io::Error,
    },

    /// Failed to copy a binary file.
    #[error("failed to copy binary files `{source}` to `{destination}`")]
    CopyBinaryFile {
        /// Source file path.
        source_path: PathBuf,

        /// Destination file path.
        destination: PathBuf,

        /// Underlying IO error.
        #[source]
        source: std::io::Error,
    },

    /// Failed to create a parent directory for a destination file.
    #[error("failed to create parent directory `{path}`")]
    CreateParentDirectory {
        /// Parent directory path.
        path: PathBuf,

        /// Underlying IO error.
        #[source]
        source: std::io::Error,
    },
}
