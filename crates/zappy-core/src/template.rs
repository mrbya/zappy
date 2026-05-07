use camino::Utf8PathBuf;
use serde::Deserialize;

use crate::error::{CoreError, CoreResult};

/// Stable template identifier used for CLI lookup.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateId(String);

impl TemplateId {
    /// Creates a validated template id.
    ///
    /// # Arguments
    /// - `value`: template id slug.
    ///
    /// # Returns
    /// Validated [`TemplateId`].
    ///
    /// # Errors
    /// Returns [`CoreError`] if:
    /// - template id empty,
    /// - contains invalid characters.
    pub fn new(value: impl Into<String>) -> CoreResult<Self> {
        let value = value.into();
        validate_field("template.id", &value)?;
        Ok(Self(value))
    }

    /// Returns raw template id.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Human-facing and lookup template metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateMetadata {
    /// Template ID.
    pub id: TemplateId,
    /// Template name.
    pub name: String,
    /// Template description.
    pub description: Option<String>,
    /// Template language/ecosystem.
    pub language: Option<String>,
    /// Template version.
    pub version: Option<String>,
    /// Template authors.
    pub authors: Vec<String>,
    /// Template source config.
    pub source: SourceConfig,
}

/// Source payload config.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceConfig {
    /// Source root.
    pub root: Utf8PathBuf,
}

impl Default for SourceConfig {
    fn default() -> Self {
        Self {
            root: Utf8PathBuf::from("template"),
        }
    }
}

/// Raw template metadata from template manifest.
#[derive(Debug, Deserialize)]
pub(crate) struct RawTemplateMetadata {
    /// Template id slug.
    pub id: String,

    /// Template name slug.
    pub name: String,

    /// Template description slug.
    #[serde(default)]
    pub description: Option<String>,

    /// Template language slug.
    #[serde(default)]
    pub language: Option<String>,

    /// Template version slug.
    #[serde(default)]
    pub version: Option<String>,

    /// Template authors slugs.
    #[serde(default)]
    pub authors: Vec<String>,

    /// Template source slug.
    #[serde(default)]
    pub source: RawSourceConfig,
}

/// Raw source payload config.
#[derive(Debug, Default, Deserialize)]
pub(crate) struct RawSourceConfig {
    /// Source root slug.
    #[serde(default)]
    pub root: Option<Utf8PathBuf>,
}

impl TryFrom<RawTemplateMetadata> for TemplateMetadata {
    type Error = CoreError;

    fn try_from(raw: RawTemplateMetadata) -> CoreResult<Self> {
        let id = TemplateId::new(raw.id)?;

        if raw.name.trim().is_empty() {
            return Err(CoreError::invalid_manifest(
                "`template.name` must not be empty",
            ));
        }

        let source = SourceConfig {
            root: raw
                .source
                .root
                .unwrap_or_else(|| Utf8PathBuf::from("template")),
        };

        validate_path("`template.source.root`", &source.root)?;

        Ok(Self {
            id,
            name: raw.name,
            description: raw.description,
            language: raw.language,
            version: raw.version,
            authors: raw.authors,
            source,
        })
    }
}

/// Validates manifest field.
///
/// # Arguments
/// - `field`: validated field name,
/// - `value`: validated field value.
///
/// # Returns
/// Ok(()) on successful validation.
///
/// # Errors
/// Returns [`CoreError`] if:
/// - field value empty,
/// - field contains characters outside ASCII letters, `-` and `_`.
pub(crate) fn validate_field(field: &str, value: &str) -> CoreResult<()> {
    if value.trim().is_empty() {
        return Err(CoreError::invalid_manifest(format!(
            "`{field}` must not be empty"
        )));
    }

    let mut chars = value.chars();

    let Some(first) = chars.next() else {
        return Err(CoreError::invalid_manifest(format!(
            "`{field}` must not be empty"
        )));
    };

    if !(first.is_ascii_alphabetic() || first == '_') {
        return Err(CoreError::invalid_manifest(format!(
            "`{field}` must start with an ASCII letter or `_`"
        )));
    }

    if !chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '-') {
        return Err(CoreError::invalid_manifest(format!(
            "`{field}` may only contain ASCII letters, digits, `_`, or `-`"
        )));
    }

    Ok(())
}

/// Validates manifest path field.
///
/// # Arguments
/// - `name`: validated path field name,
/// - `path`: validated path field value.
///
/// # Returns
/// Ok(()) on successful validation.
///
/// # Errors
/// Returns [`CoreError`] if:
/// - path is empty,
/// - path path is absolute,
/// - path contains `..` path components.
pub(crate) fn validate_path(name: &str, path: &Utf8PathBuf) -> CoreResult<()> {
    if path.as_str().trim().is_empty() {
        return Err(CoreError::invalid_manifest(format!(
            "`{name}` must not be empty"
        )));
    }

    if path.is_absolute() {
        return Err(CoreError::invalid_manifest(format!(
            "`{name}` must be relative"
        )));
    }

    if path
        .components()
        .any(|component| component.as_str() == "..")
    {
        return Err(CoreError::invalid_manifest(format!(
            "`{name}` must not contain `..` path components"
        )));
    }

    Ok(())
}
