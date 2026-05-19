use std::fs;
use std::path::{Path, PathBuf};

use include_dir::{Dir, DirEntry, include_dir};

use crate::error::{TemplatesError, TemplatesResult};

/// Embedded bundled templates directory.
static BUNDLED_TEMPLATES: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/templates");

/// Cache marker file name.
pub const CACHE_MARKER: &str = ".zappy-templates-cache";

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

    tracing::debug!(path = %templates_dir.display(), "ensuring bundled templates are available");

    if cache_is_ready(&templates_dir) {
        tracing::debug!(path = %templates_dir.display(), "bundled template cache already ready");
        return Ok(templates_dir);
    }

    tracing::info!(path = %templates_dir.display(), "installing bundled templates into cache");
    install_bundled_templates(&templates_dir)?;

    Ok(templates_dir)
}

/// Clears bundled templates cache.
///
/// # Errors
/// Returns [`TemplatesError::ClearCache`] if cache clear fails.
pub fn clear_cache_dir() -> TemplatesResult<()> {
    let templates_dir = resolve_cache_dir()?;

    tracing::info!(path = %templates_dir.display(), "clearing bundled template cache directory");

    if templates_dir.exists() {
        fs::remove_dir_all(&templates_dir).map_err(|source| {
            Box::new(TemplatesError::ClearCache {
                path: templates_dir,
                source,
            })
        })?;
    } else {
        tracing::debug!(path = %templates_dir.display(), "bundled template cache directory already absent");
    }

    Ok(())
}

/// Installs bundled templates into template cache.
fn install_bundled_templates(templates_dir: &Path) -> TemplatesResult<()> {
    tracing::debug!(path = %templates_dir.display(), "creating bundled template cache directories");
    fs::create_dir_all(templates_dir).map_err(|source| {
        Box::new(TemplatesError::CreateDirectory {
            path: templates_dir.to_path_buf(),
            source,
        })
    })?;

    extract_dir(&BUNDLED_TEMPLATES, templates_dir)?;

    tracing::trace!(path = %templates_dir.display(), "writing bundled template cache marker");

    fs::write(templates_dir.join(CACHE_MARKER), expected_marker()).map_err(|source| {
        Box::new(TemplatesError::WriteFile {
            path: templates_dir.join(CACHE_MARKER),
            source,
        })
    })?;

    Ok(())
}

/// Checks whether the current cache is up to date and ready.
fn cache_is_ready(templates_dir: &Path) -> bool {
    let marker_path = templates_dir.join(CACHE_MARKER);

    let Ok(marker) = fs::read_to_string(marker_path) else {
        tracing::trace!(path = %templates_dir.display(), "bundled template cache marker is missing or unreadable");
        return false;
    };

    let is_ready = marker == expected_marker();
    tracing::trace!(path = %templates_dir.display(), cache_ready = is_ready, "checked bundled template cache marker");

    is_ready
}

/// Generates expected cache marker contents.
fn expected_marker() -> String {
    format!(
        "cache_format=1\npackage=zappy-templates\npackage_version={}\ntemplates_hash={}\n",
        env!("CARGO_PKG_VERSION"),
        env!("ZAPPY_TEMPLATES_HASH"),
    )
}

/// Resolves the bundled template cache directory.
///
/// # Errors
/// Returns [`TemplatesError::ResolveCacheDirectory`] if fails to resolve cache dir.
fn resolve_cache_dir() -> TemplatesResult<PathBuf> {
    let Some(project_dirs) = directories::ProjectDirs::from("", "", "zappy") else {
        return Err(Box::new(TemplatesError::ResolveCacheDirectory));
    };

    let cache_dir = project_dirs
        .cache_dir()
        .join("bundled-templates")
        .join(env!("CARGO_PKG_VERSION"))
        .join("templates");

    tracing::trace!(path = %cache_dir.display(), "resolved bundled template cache directory");

    Ok(cache_dir)
}

/// Recursively extracts en embedded directory.
fn extract_dir(dir: &Dir<'_>, destination_root: &Path) -> TemplatesResult<()> {
    for entry in dir.entries() {
        match *entry {
            DirEntry::Dir(ref child_dir) => {
                let destination = destination_root.join(child_dir.path());

                tracing::trace!(path = %destination.display(), "extracting bundled template directory");

                fs::create_dir_all(&destination).map_err(|source| {
                    Box::new(TemplatesError::CreateDirectory {
                        path: destination.clone(),
                        source,
                    })
                })?;

                extract_dir(child_dir, destination_root)?;
            }

            DirEntry::File(ref file) => {
                let destination = destination_root.join(file.path());

                tracing::trace!(path = %destination.display(), bytes = file.contents().len(), "extracting bundled template file");

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
