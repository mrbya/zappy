use std::fs;
use std::path::{Path, PathBuf};

use crate::{FsError, FsResult};

/// Source filesystem entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceEntry {
    /// Absolute/source-root-relative source path.
    pub path: PathBuf,

    /// Relative path from template source root.
    pub relative_path: PathBuf,

    /// Entry kind.
    pub kind: SourceEntryKind,
}

/// Source entry kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceEntryKind {
    /// Directory.
    Directory,

    /// Regular file.
    File,

    /// Symlink.
    Symlink,
}

/// Walks a template source root.
///
/// # Arguments
/// - `source_root`: template source root to start walk in.
///
/// # Returns
/// Ok(`Vec<SourceEntry>`) vector of found and inspected file entries.
///
/// # Errors
/// - [`FsError::ReadSearchPath`] if fails to read a path,
/// - [`FsError::ReadDirectoryEntry`] if fails to read a dir entry,
/// - [`FsError::FileType`] if fails to inspect a file type.
pub fn walk_source_root(source_root: &Path) -> FsResult<Vec<SourceEntry>> {
    tracing::debug!(root = %source_root.display(), "walking template source root");
    let mut entries = Vec::new();
    walk_dir(source_root, source_root, &mut entries)?;
    entries.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    tracing::debug!(root = %source_root.display(), entry_count = entries.len(), "completed source root walk");
    Ok(entries)
}

/// Recursive directory walk.
///
/// # Arguments
/// - `source_root`: root path to start walk from,
/// - `current`: current directory to walk,
/// - `entries`: mutable reference to a vector of entries already walked.
///
/// # Returns
/// Ok(()) on successful walk.
///
/// # Errors
/// Returns following errors:
/// - [`FsError::ReadSearchPath`] if fails to read a path,
/// - [`FsError::ReadDirectoryEntry`] if fails to read a dir entry,
/// - [`FsError::FileType`] if fails to inspect a file type.
fn walk_dir(source_root: &Path, current: &Path, entries: &mut Vec<SourceEntry>) -> FsResult<()> {
    tracing::trace!(current = %current.display(), "walking directory level");
    let mut children = Vec::new();

    let read_dir = fs::read_dir(current).map_err(|source| {
        Box::new(FsError::ReadSearchPath {
            path: current.to_path_buf(),
            source,
        })
    })?;

    for child in read_dir {
        let child = child.map_err(|source| {
            Box::new(FsError::ReadDirectoryEntry {
                path: current.to_path_buf(),
                source,
            })
        })?;

        children.push(child.path());
    }

    children.sort();

    for path in children {
        let metadata = fs::symlink_metadata(&path).map_err(|source| {
            Box::new(FsError::FileType {
                path: path.clone(),
                source,
            })
        })?;

        let relative_path = path
            .strip_prefix(source_root)
            .map_or_else(|_| path.clone(), Path::to_path_buf);

        let kind = if metadata.file_type().is_symlink() {
            SourceEntryKind::Symlink
        } else if metadata.is_dir() {
            SourceEntryKind::Directory
        } else {
            SourceEntryKind::File
        };

        tracing::trace!(path = %path.display(), relative = %relative_path.display(), kind = ?kind, "discovered source entry");

        entries.push(SourceEntry {
            path: path.clone(),
            relative_path,
            kind,
        });

        if kind == SourceEntryKind::Directory {
            walk_dir(source_root, &path, entries)?;
        }
    }

    Ok(())
}
