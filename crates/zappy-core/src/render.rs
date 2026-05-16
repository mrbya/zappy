use indexmap::IndexMap;

use crate::{CoreError, CoreResult};

/// Renders text by applying literal placeholder replacements.
///
/// Replacements are applied in insertion order.
///
/// # Arguments
/// - `input`: input text to render,
/// - `replacements`: index map of placeholder-value replacement pairs.
///
/// # Returns
/// Rendered text.
#[must_use]
pub fn render_text(input: &str, replacements: &IndexMap<String, String>) -> String {
    let mut rendered = String::from(input);

    for (placeholder, value) in replacements {
        rendered = rendered.replace(placeholder, value);
    }

    rendered
}

/// Renders a relative path represented as a UTF-8 text.
///
/// # Arguments
/// - `input`: input path text,
/// - `replacements`: index map of placeholder-value replacement pairs.
///
/// # Returns
/// Ok(String) containing rendered path text on success.
///
/// # Errors
/// Returns [`CoreError::InvalidRenderedPath`] on validation failure.
pub fn render_relative_path(
    input: &str,
    replacements: &IndexMap<String, String>,
) -> CoreResult<String> {
    let rendered = render_text(input, replacements);
    validate_rendered_relative_path(&rendered)?;

    Ok(rendered)
}

/// Validates rendered relative path text.
///
/// This is string-level validation. Filesystem-level checks
/// still belong in zappy-fs.
///
/// # Arguments
/// - `path`: rendered path text.
///
/// # Returns
/// Ok(()) on successful validation.
///
/// # Errors
/// Returns [`CoreError::InvalidRenderedPath`] if:
/// - rendered path is empty,
/// - rendered path is absolute,
/// - rendered path contains `..` path components.
fn validate_rendered_relative_path(path: &str) -> CoreResult<()> {
    if path.trim().is_empty() {
        return Err(CoreError::InvalidRenderedPath {
            path: String::from(path),
            reason: String::from("rendered path must not be empty"),
        });
    }

    if path.starts_with('/') {
        return Err(CoreError::InvalidRenderedPath {
            path: String::from(path),
            reason: String::from("rendered path must be relative"),
        });
    }

    for component in path.split('/') {
        if component == ".." {
            return Err(CoreError::InvalidRenderedPath {
                path: String::from(path),
                reason: String::from("rendered path must not contain `..` path components"),
            });
        }
    }

    Ok(())
}
