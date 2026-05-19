use std::fs;
use std::path::Path;

use zappy_core::{GenerationPlan, PlanOperation};

use crate::{FsError, FsResult};

/// Options for executing a generation plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterializationOptions {
    /// Overwrite conflicting destination files?
    pub force: bool,
}

impl MaterializationOptions {
    /// Returns options with no force overwrite.
    #[must_use]
    pub const fn no_force() -> Self {
        Self { force: false }
    }

    /// Returns options with force overwrite.
    #[must_use]
    pub const fn force() -> Self {
        Self { force: true }
    }
}

/// Summary of executed materialization operations.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MaterializationSummary {
    /// Number of created directories.
    pub directories_created: usize,

    /// Number of rendered text files.
    pub text_files_written: usize,

    /// Number of binary files copied.
    pub binary_files_copied: usize,

    /// Number of skipped operations.
    pub skipped: usize,
}

/// Materializes (executes) a generation plan.
///
/// # Arguments
/// - `plan`: built generation plan,
/// - `options`: materialization options.
///
/// # Returns
/// Ok(MaterializationSummary) summary on successful execution.
///
/// # Errors
/// Returns [`FsError`] if:
/// - a destination already exists and `--force` was not provided,
/// - any filesystem operation fails.
pub fn materialize_generation_plan(
    plan: &GenerationPlan,
    options: MaterializationOptions,
) -> FsResult<MaterializationSummary> {
    tracing::info!(
        template = plan.template_id.as_str(),
        output = %plan.output_dir.display(),
        operations = plan.operations.len(),
        force = options.force,
        "materializing generation plan"
    );
    let mut summary = MaterializationSummary::default();

    for operation in plan.operations.iter().cloned() {
        match operation {
            PlanOperation::CreateDirectory { destination, .. } => {
                tracing::trace!(destination = %destination.display(), "creating output directory");
                create_directory(&destination)?;
                summary.directories_created = summary
                    .directories_created
                    .checked_add(1)
                    .unwrap_or(summary.directories_created);
            }

            PlanOperation::RenderTextFile {
                destination,
                content,
                ..
            } => {
                tracing::trace!(destination = %destination.display(), bytes = content.len(), "writing rendered text file");
                write_text_file(&destination, &content, options.force)?;
                summary.text_files_written = summary
                    .text_files_written
                    .checked_add(1)
                    .unwrap_or(summary.text_files_written);
            }

            PlanOperation::CopyBinaryFile {
                source,
                destination,
            } => {
                tracing::trace!(source = %source.display(), destination = %destination.display(), "copying binary file");
                copy_binary_file(&source, &destination, options.force)?;
                summary.binary_files_copied = summary
                    .binary_files_copied
                    .checked_add(1)
                    .unwrap_or(summary.binary_files_copied);
            }

            PlanOperation::Skip { .. } => {
                summary.skipped = summary.skipped.checked_add(1).unwrap_or(summary.skipped);
            }
        }
    }

    tracing::debug!(
        directories = summary.directories_created,
        text_files = summary.text_files_written,
        binary_files = summary.binary_files_copied,
        skipped = summary.skipped,
        "generation plan materialization completed"
    );

    Ok(summary)
}

/// Creates a directory and all its missing parents.
///
/// # Arguments
/// - `path`: directory path.
///
/// # Returns
/// Ok(()) on success.
///
/// # Errors
/// Returns [`FsError::CreateDirectory`] on directory creation failure.
pub fn create_directory(path: &Path) -> FsResult<()> {
    tracing::trace!(path = %path.display(), "creating directory path");
    fs::create_dir_all(path).map_err(|source| {
        Box::new(FsError::CreateDirectory {
            path: path.to_path_buf(),
            source,
        })
    })
}

// TODO: Preserve Unix executable bits for copied/rendered files.

/// Writes a rendered UTF-8 file.
pub(crate) fn write_text_file(path: &Path, content: &str, force: bool) -> FsResult<()> {
    tracing::trace!(path = %path.display(), bytes = content.len(), force, "preparing text file write");
    ensure_can_write_file(path, force)?;

    if let Some(parent) = path.parent() {
        create_parent_directory(parent)?;
    }

    fs::write(path, content).map_err(|source| {
        Box::new(FsError::WriteTextFile {
            path: path.to_path_buf(),
            source,
        })
    })
}

/// Copies a binary file.
fn copy_binary_file(source: &Path, destination: &Path, force: bool) -> FsResult<()> {
    tracing::trace!(source = %source.display(), destination = %destination.display(), force, "preparing binary copy");
    ensure_can_write_file(destination, force)?;

    if let Some(parent) = destination.parent() {
        create_parent_directory(parent)?;
    }

    fs::copy(source, destination)
        .map(|_| ())
        .map_err(|source_error| {
            Box::new(crate::FsError::CopyBinaryFile {
                source_path: source.to_path_buf(),
                destination: destination.to_path_buf(),
                source: source_error,
            })
        })
}

/// Ensures a file destination can be written.
fn ensure_can_write_file(path: &Path, force: bool) -> FsResult<()> {
    if path.exists() && path.is_file() && !force {
        tracing::debug!(path = %path.display(), "refusing to overwrite existing file without force");
        return Err(Box::new(crate::FsError::DestinationExists {
            path: path.to_path_buf(),
        }));
    }

    Ok(())
}

/// Creates a parent directory.
fn create_parent_directory(path: &Path) -> FsResult<()> {
    tracing::trace!(path = %path.display(), "creating parent directory");
    fs::create_dir_all(path).map_err(|source| {
        Box::new(crate::FsError::CreateParentDirectory {
            path: path.to_path_buf(),
            source,
        })
    })
}
