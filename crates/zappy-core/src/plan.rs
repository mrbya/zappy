use std::path::PathBuf;

use crate::TemplateId;

/// Dry-run/project generation plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationPlan {
    /// Template id.
    pub template_id: TemplateId,

    /// Output directory.
    pub output_dir: PathBuf,

    /// Planned operations.
    pub operations: Vec<PlanOperation>,

    /// Plan warnings.
    pub warnings: Vec<PlanWarning>,
}

/// Single generation plan operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanOperation {
    /// Create a directory.
    CreateDirectory {
        /// Source directory, if this came from a template directory.
        source: Option<PathBuf>,

        /// Destination directory.
        destination: PathBuf,
    },

    /// Render a UTF-8 text file.
    RenderTextFile {
        /// Source file path.
        source: PathBuf,

        /// Destination file path.
        destination: PathBuf,

        /// Rendered text content.
        content: String,
    },

    /// Copy a binary file without rendering.
    CopyBinaryFile {
        /// Source file path.
        source: PathBuf,

        /// Destination file path.
        destination: PathBuf,
    },

    /// Skip a source path.
    Skip {
        /// Source path.
        source: PathBuf,

        /// Skip reason.
        reason: SkipReason,
    },
}

/// Reason a path was skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkipReason {
    /// Path matched manifest excludes.
    Excluded,

    /// Path matched a false conditional.
    ConditionalFalse {
        /// Conditional variable name.
        variable: String,
    },

    /// Path is a symlink and symlink handling is not supported yet.
    Symlink,
}

/// Plan warning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanWarning {
    /// Destination path already exists.
    DestinationExists {
        /// Destination path.
        destination: PathBuf,
    },

    /// File was copied as a binary because UTF-8 decoding failed.
    NonUtf8FileCopiedAsBinary {
        /// File source path.
        source: PathBuf,
    },
}
