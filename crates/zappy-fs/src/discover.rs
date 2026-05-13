use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

use zappy_core::Manifest;

use crate::error::{FsError, FsResult};

/// Configuration used for template discovery.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DiscoveryConfig {
    /// Explicit tempalte directory override.
    ///
    /// When set, discovery uses only this directory.
    pub templates_dir: Option<PathBuf>,

    /// Directory containing bundled starter templates.
    pub bundled_templates_dir: Option<PathBuf>,
}

/// Template search path source/type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateSearchPathKind {
    /// Explicit `--templates-dir` CLI arg.
    Explicit,

    /// `ZAPPY_TEMPLATES_DIR` env variable.
    EnvironmentTemplatesDir,

    /// `$ZAPPY_CONFIG/templates`.
    EnvironmentConfigTemplates,

    /// Platform-specific config directory.
    PlatformConfig,

    /// Directory next to the `zappy` executable.
    ExecutableRelative,

    /// `templates/` under current working directory.
    CurrentWorkingDirectory,

    /// Bundled `templates/` extracted from `zappy-tempaltes`.
    Bundled,
}

/// A candidate template search path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateSearchPath {
    /// Search path kind.
    pub kind: TemplateSearchPathKind,

    /// Search path.
    pub path: PathBuf,

    /// Missing/invalid path should be reported as an error?
    pub required: bool,
}

/// A discoverred template directory with its parsed manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredTemplate {
    /// Search path this template came from.
    pub search_path: TemplateSearchPath,

    /// Template directory containing `zappy.toml`.
    pub template_dir: PathBuf,

    /// Manifest path.
    pub manifest_path: PathBuf,

    /// Parsed manifest.
    pub manifest: Manifest,
}

/// Result of template discovery.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TemplateCatalogue {
    /// Active templates.
    templates: Vec<DiscoveredTemplate>,

    /// Templates shadowed bu an earlier search path with the same template id.
    shadowed: Vec<DiscoveredTemplate>,

    /// Search paths used during discovery.
    search_paths: Vec<TemplateSearchPath>,
}

impl TemplateCatalogue {
    /// Returns active templates.
    #[must_use]
    pub fn templates(&self) -> &[DiscoveredTemplate] {
        &self.templates
    }

    /// Returns shadoved duplicate templates.
    #[must_use]
    pub fn shadowed(&self) -> &[DiscoveredTemplate] {
        &self.shadowed
    }

    /// Returns search paths used for discovery.
    #[must_use]
    pub fn search_paths(&self) -> &[TemplateSearchPath] {
        &self.search_paths
    }

    /// Finds a tempalte by id.
    #[must_use]
    pub fn find_by_id(&self, id: &str) -> Option<&DiscoveredTemplate> {
        self.templates
            .iter()
            .find(|template| template.manifest.template.id.as_str() == id)
    }
}

/// Discovers templates using the standard Zappy search-path model.
///
/// # Arguments
/// - `config`: template search path discovery config.
///
/// # Returns
/// A complete [`TemplateCatalogue`] on success.
///
/// # Errors
/// Returns following errors:
/// - [`FsError::ResolveSearchPaths`] on search path resolution failure,
/// - Other [`FsError`] on template discovery failure (see `discover_templates_from_search_paths`).
pub fn discover_templates(config: &DiscoveryConfig) -> FsResult<TemplateCatalogue> {
    let search_paths = resolve_template_search_paths(config)?;
    discover_templates_from_search_paths(search_paths)
}

/// Resolves templat esearch paths.
///
/// # Arguments
/// - `config`: template path discovery config.
///
/// # Returns
/// Vector of resolved [`TemplateSearchPath`] paths.
///
/// # Errors
/// Returns [`FsError::ResolveSearchPaths`] if no search path is resolved.
pub fn resolve_template_search_paths(
    config: &DiscoveryConfig,
) -> FsResult<Vec<TemplateSearchPath>> {
    if let Some(templates_dir) = config.templates_dir.as_ref() {
        return Ok(vec![TemplateSearchPath {
            kind: TemplateSearchPathKind::Explicit,
            path: templates_dir.clone(),
            required: true,
        }]);
    }

    let mut paths = Vec::new();

    if let Some(path) = env::var_os("ZAPPY_TEMPLATES_DIR") {
        paths.push(TemplateSearchPath {
            kind: TemplateSearchPathKind::EnvironmentTemplatesDir,
            path: PathBuf::from(path),
            required: true,
        });
    }

    if let Some(path) = env::var_os("ZAPPY_CONFIG") {
        paths.push(TemplateSearchPath {
            kind: TemplateSearchPathKind::EnvironmentConfigTemplates,
            path: PathBuf::from(path).join("templates"),
            required: false,
        });
    }

    if let Some(project_dirs) = directories::ProjectDirs::from("", "", "zappy") {
        paths.push(TemplateSearchPath {
            kind: TemplateSearchPathKind::PlatformConfig,
            path: project_dirs.config_dir().join("templates"),
            required: false,
        });
    }

    if let Ok(current_exe) = env::current_exe() {
        if let Some(exe_dir) = current_exe.parent() {
            paths.push(TemplateSearchPath {
                kind: TemplateSearchPathKind::ExecutableRelative,
                path: exe_dir.join("templates"),
                required: false,
            });
        }
    }

    if let Ok(current_dir) = env::current_dir() {
        paths.push(TemplateSearchPath {
            kind: TemplateSearchPathKind::CurrentWorkingDirectory,
            path: current_dir.join("templates"),
            required: false,
        });
    }

    if let Some(bundled_templates_dir) = config.bundled_templates_dir.as_ref() {
        paths.push(TemplateSearchPath {
            kind: TemplateSearchPathKind::Bundled,
            path: bundled_templates_dir.clone(),
            required: false,
        });
    }

    if paths.is_empty() {
        return Err(Box::new(FsError::ResolveSearchPaths));
    }

    Ok(paths)
}

/// Discovers templates from already-resolved search paths.
///
/// # Arguments
/// - `search_paths`: search paths to discover templates in.
///
/// # Returns
/// A complete [`TemplateCatalogue`] on success.
///
/// # Errors
/// Returns following errors:
/// - [`FsError::LoadManifest`] if manifest load/parse fails,
/// - Other [`FsError`] if template dir discovery fails.
pub(crate) fn discover_templates_from_search_paths(
    search_paths: Vec<TemplateSearchPath>,
) -> FsResult<TemplateCatalogue> {
    let mut templates = Vec::new();
    let mut shadowed = Vec::new();
    let mut seen_ids = HashSet::new();

    for search_path in &search_paths {
        let template_dirs = discover_template_dirs(search_path)?;

        for template_dir in template_dirs {
            let manifest_path = template_dir.join("zappy.toml");
            let manifest = Manifest::load_from_path(&manifest_path).map_err(|source| {
                FsError::LoadManifest {
                    path: manifest_path.clone(),
                    source,
                }
            })?;

            let id = manifest.template.id.as_str().to_owned();

            let discovered = DiscoveredTemplate {
                search_path: search_path.clone(),
                template_dir,
                manifest_path,
                manifest,
            };

            if seen_ids.insert(id) {
                templates.push(discovered);
            } else {
                shadowed.push(discovered);
            }
        }
    }

    templates.sort_by(|left, right| {
        left.manifest
            .template
            .id
            .as_str()
            .cmp(right.manifest.template.id.as_str())
    });

    shadowed.sort_by(|left, right| {
        left.manifest
            .template
            .id
            .as_str()
            .cmp(right.manifest.template.id.as_str())
    });

    Ok(TemplateCatalogue {
        templates,
        shadowed,
        search_paths,
    })
}

/// Discovers template directories inside a search path.
///
/// # Arguments
/// - `search_path`: Template search path to start discovery.
///
/// A search path may either be:
/// - a directory containing `zappy.toml`, meaning a template directory,
/// - a directory containing child template directories.
///
/// # Returns
/// Vector of found template directories.
///
/// # Errors
/// Returns following errors:
/// - [`FsError::SearchPathMissing`] if the search path is missing,
/// - [`FsError::SearchPathNotDirectory`] if the search path is not a directory,
/// - [`FsError::ReadSearchPath`] if reading the search path fails,
/// - [`FsError::ReadDirectoryEntry`] if reading a search path entry fails,
/// - [`FsError::FileType`] if search path entry file type discovery fails.
fn discover_template_dirs(search_path: &TemplateSearchPath) -> FsResult<Vec<PathBuf>> {
    let path = &search_path.path;

    if !path.exists() {
        if search_path.required {
            return Err(Box::new(FsError::SearchPathMissing { path: path.clone() }));
        }

        return Ok(Vec::new());
    }

    if !path.is_dir() {
        if search_path.required {
            return Err(Box::new(FsError::SearchPathNotDirectory {
                path: path.clone(),
            }));
        }

        return Ok(Vec::new());
    }

    if path.join("zappy.toml").is_file() {
        return Ok(vec![path.clone()]);
    }

    let mut template_dirs = Vec::new();

    let entries = std::fs::read_dir(path).map_err(|source| FsError::ReadSearchPath {
        path: path.clone(),
        source,
    })?;

    for entry in entries {
        let entry = entry.map_err(|source| FsError::ReadDirectoryEntry {
            path: path.clone(),
            source,
        })?;

        let entry_path = entry.path();
        let file_type = entry.file_type().map_err(|source| FsError::FileType {
            path: entry_path.clone(),
            source,
        })?;

        if file_type.is_dir() && entry_path.join("zappy.toml").is_file() {
            template_dirs.push(entry_path);
        }
    }

    template_dirs.sort();

    Ok(template_dirs)
}
