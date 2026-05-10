use std::fs;
use std::path::{Path, PathBuf};

use zappy_core::condition::evaluate_condition;
use zappy_core::render::{render_relative_path, render_text};
use zappy_core::{
    GenerationPlan, Manifest, PlanOperation, PlanWarning, ResolvedVariables, SkipReason,
};

use crate::classify::{FileKind, classify_file_by_path};
use crate::walk::{SourceEntryKind, walk_source_root};
use crate::{FsError, FsResult};

/// Input used to build generation plan.
#[derive(Debug, Clone)]
pub struct BuildPlanInput<'a> {
    /// Template directory containing `zappy.toml`.
    pub template_dir: &'a Path,

    /// Parsed template manifest.
    pub manifest: &'a Manifest,

    /// Resolved variables.
    pub variables: &'a ResolvedVariables,

    /// Output directory.
    pub output_dir: PathBuf,

    /// Overwrite conflicting files?
    pub force: bool,
}

/// Builds a generation plan without writing files.
///
/// # Arguments
/// - `input`: plan generation input data.
///
/// # Returns
/// Ok(GenerationPlan) built generation plan on success.
///
/// # Errors
/// Returns [`FsError`] if:
/// - template source dir cannot be read,
/// - template source dir walk fails,
/// - underlying core rendering fails.
pub fn build_generation_plan(input: &BuildPlanInput<'_>) -> FsResult<GenerationPlan> {
    let source_root = input
        .template_dir
        .join(&input.manifest.template.source.root);
    let entries = walk_source_root(&source_root)?;

    let mut operations = Vec::new();
    let mut warnings = Vec::new();

    for entry in entries {
        if is_excluded(&entry.relative_path, &input.manifest.paths.exclude) {
            operations.push(PlanOperation::Skip {
                source: entry.path,
                reason: SkipReason::Excluded,
            });
            continue;
        }

        if let Some(variable) =
            false_conditional_for_path(&entry.relative_path, input.manifest, input.variables)
        {
            operations.push(PlanOperation::Skip {
                source: entry.path,
                reason: SkipReason::ConditionalFalse { variable },
            });
            continue;
        }

        if entry.kind == SourceEntryKind::Symlink {
            operations.push(PlanOperation::Skip {
                source: entry.path,
                reason: SkipReason::Symlink,
            });
            continue;
        }

        let rendered_relative = render_relative_path(
            &path_to_posix_string(&entry.relative_path),
            &input.variables.replacements,
        )
        .map_err(|source| Box::new(FsError::RenderPath { source }))?;

        let destination = input.output_dir.join(rendered_relative);

        match entry.kind {
            SourceEntryKind::Directory => {
                operations.push(PlanOperation::CreateDirectory {
                    source: Some(entry.path),
                    destination,
                });
            }

            SourceEntryKind::File => {
                if destination.exists() && !input.force {
                    warnings.push(PlanWarning::DestinationExists {
                        destination: destination.clone(),
                    });
                }

                match classify_file_by_path(&entry.path, &input.manifest.paths) {
                    Some(FileKind::Binary) => {
                        operations.push(PlanOperation::CopyBinaryFile {
                            source: entry.path,
                            destination,
                        });
                    }
                    Some(FileKind::Text) | None => {
                        plan_text_or_binary_file(
                            &entry.path,
                            destination,
                            input,
                            &mut operations,
                            &mut warnings,
                        )?;
                    }
                }
            }

            SourceEntryKind::Symlink => {
                operations.push(PlanOperation::Skip {
                    source: entry.path,
                    reason: SkipReason::Symlink,
                });
            }
        }
    }

    Ok(GenerationPlan {
        template_id: input.manifest.template.id.clone(),
        output_dir: input.output_dir.clone(),
        operations,
        warnings,
    })
}

/// Plans a file by probing UTF-8 file render. Non-UTF-8 files become binary copies.
fn plan_text_or_binary_file(
    source: &Path,
    destination: PathBuf,
    input: &BuildPlanInput<'_>,
    operations: &mut Vec<PlanOperation>,
    warnings: &mut Vec<PlanWarning>,
) -> FsResult<()> {
    match fs::read_to_string(source) {
        Ok(content) => {
            let content = render_text(&content, &input.variables.replacements);

            operations.push(PlanOperation::RenderTextFile {
                source: source.to_path_buf(),
                destination,
                content,
            });

            Ok(())
        }

        Err(error) if error.kind() == std::io::ErrorKind::InvalidData => {
            warnings.push(PlanWarning::NonUtf8FileCopiedAsBinary {
                source: source.to_path_buf(),
            });

            operations.push(PlanOperation::CopyBinaryFile {
                source: source.to_path_buf(),
                destination,
            });

            Ok(())
        }

        Err(source_error) => Err(Box::new(FsError::ReadTemplateFile {
            path: source.to_path_buf(),
            source: source_error,
        })),
    }
}

/// Returns true if relative path matches excludes.
fn is_excluded(relative_path: &Path, excludes: &[String]) -> bool {
    let relative_path = path_to_posix_string(relative_path);

    excludes.iter().any(|exclude| {
        let exclude = exclude.trim();

        exclude == relative_path
            || relative_path.starts_with(&format!("{exclude}/"))
            || relative_path
                .split('/')
                .any(|component| component == exclude)
    })
}

/// Returns the first false conditional variable affecting given path.
fn false_conditional_for_path(
    relative_path: &Path,
    manifest: &Manifest,
    variables: &ResolvedVariables,
) -> Option<String> {
    for conditional in &manifest.conditionals {
        let conditional_path = PathBuf::from(conditional.path.as_str());

        if (relative_path == conditional_path || relative_path.starts_with(conditional_path))
            && !evaluate_condition(&conditional.when, &variables.values)
        {
            return Some(conditional.when.clone());
        }
    }

    None
}

/// Convert a path to posix for placeholder rendering.
fn path_to_posix_string(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
