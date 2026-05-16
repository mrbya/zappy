use std::path::Path;

use zappy_core::manifest::PathConfig;

/// File classification result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    /// UTF-8 readable text.
    Text,

    /// Binary/copy-only files.
    Binary,
}

/// Classifies file from manifest path rules and optional UTF-8 probe.
#[must_use]
pub fn classify_file_by_path(path: &Path, config: &PathConfig) -> Option<FileKind> {
    if matches_binary_file(path, &config.binary_files) {
        return Some(FileKind::Binary);
    }

    if matches_binary_extension(path, &config.binary_extensions) {
        return Some(FileKind::Binary);
    }

    None
}

/// Returns true if a path matches a manifest binary file entry.
fn matches_binary_file(path: &Path, binary_files: &[String]) -> bool {
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };

    binary_files.iter().any(|entry| {
        let entry = entry.trim();

        entry == file_name || path.ends_with(entry)
    })
}

/// Returns true if a path extension matches manifest binary extensions.
fn matches_binary_extension(path: &Path, binary_extensions: &[String]) -> bool {
    let Some(extension) = path.extension().and_then(|extension| extension.to_str()) else {
        return false;
    };

    binary_extensions.iter().any(|entry| {
        let normalized = entry.strip_prefix('.').unwrap_or(entry);
        normalized == extension
    })
}
