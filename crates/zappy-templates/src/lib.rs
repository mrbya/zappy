//! Bundled starter templates for Zappy.
//!
//! This crate embeds Zappy's starter templates and exposes them as a real
//! filesystem directory so the normal `zappy-fs` discovery pipeline can use
//! them unchanged.

use std::fs;
use std::path::{Path, PathBuf};

use include_dir::{Dir, DirEntry, include_dir};

use crate::error::{TemplatesError, TemplatesResult};

/// Bundled template errors.
pub mod error;

/// Embedded bundled templates directory.
static BUNDLED_TEMPLATES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");

/// Returns a real filesystem directory containing bundled templates.
///
/// The directory is recreated on each call to avoid stale bundled templates
/// in local development builds.
///
/// # Errors
/// Returns [`TemplatesError`] if:
/// - the cache directory cannot be resolved
/// - template files cannot be written
pub fn ensure_bundled_templates_available() -> TemplatesResult<PathBuf> {
    let templates_dir = resolve_cache_dir()?;

    if templates_dir.exists() {
        fs::remove_dir_all(&templates_dir).map_err(|source| {
            Box::new(TemplatesError::ClearCache {
                path: templates_dir.clone(),
                source,
            })
        })?;
    }

    fs::create_dir_all(&templates_dir).map_err(|source| {
        Box::new(TemplatesError::CreateDirectory {
            path: templates_dir.clone(),
            source,
        })
    })?;

    extract_dir(&BUNDLED_TEMPLATES, &templates_dir)?;

    Ok(templates_dir)
}

/// Resolves the bundled template cache directory.
fn resolve_cache_dir() -> TemplatesResult<PathBuf> {
    let Some(project_dirs) = directories::ProjectDirs::from("", "", "zappy") else {
        return Err(Box::new(TemplatesError::ResolveCacheDirectory));
    };

    Ok(project_dirs
        .cache_dir()
        .join("bundled-templates")
        .join(env!("CARGO_PKG_VERSION"))
        .join("templates"))
}

/// Recursively extracts en embedded directory.
fn extract_dir(dir: &Dir<'_>, destination_root: &Path) -> TemplatesResult<()> {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(child_dir) => {
                let destination = destination_root.join(child_dir.path());

                fs::create_dir_all(&destination).map_err(|source| {
                    Box::new(TemplatesError::CreateDirectory {
                        path: destination.clone(),
                        source,
                    })
                })?;

                extract_dir(child_dir, destination_root)?;
            }

            DirEntry::File(file) => {
                let destination = destination_root.join(file.path());

                if let Some(parent) = destination.parent() {
                    fs::create_dir_all(parent).map_err(|source| {
                        Box::new(TemplatesError::CreateDirectory {
                            path: parent.to_path_buf(),
                            source,
                        })
                    })?;
                }

                fs::write(&destination, file.contents()).map_err(|source| {
                    Box::new(TemplatesError::WriteFile {
                        path: destination,
                        source,
                    })
                })?;
            }
        }
    }

    Ok(())
}

// Tests.
#[cfg(test)]
mod tests;
