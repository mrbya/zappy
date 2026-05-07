use camino::Utf8PathBuf;
use indexmap::IndexMap;
use serde::Deserialize;

use crate::{
    error::{CoreError, CoreResult},
    template::validate_path,
};

/// Hooks attached to normal generation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HookConfig {
    /// Pre-generation hooks.
    pub pre_generate: Vec<HookSpec>,

    /// Post-generation hooks.
    pub post_generate: Vec<HookSpec>,
}

/// Command hook specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HookSpec {
    /// Hook name.
    pub name: Option<String>,

    /// Hook command.
    pub command: String,

    /// Hook command args.
    pub args: Vec<String>,

    /// Hook command working directory.
    pub working_dir: Option<Utf8PathBuf>,

    /// Hook command env setup.
    pub env: IndexMap<String, String>,

    /// Hook trigger.
    pub when: Option<String>,

    /// Is the hook optional?
    pub optional: bool,

    /// Hook uses shell?
    pub shell: bool,
}

/// Raw hook config from template manifest.
#[derive(Debug, Default, Deserialize)]
pub(crate) struct RawHookConfig {
    /// Raw pre-generation hooks.
    #[serde(default)]
    pub pre_generate: Vec<RawHookSpec>,

    /// Raw post-generation hooks.
    #[serde(default)]
    pub post_generate: Vec<RawHookSpec>,
}

/// Raw hook specification from template manifest.
#[derive(Debug, Deserialize)]
pub(crate) struct RawHookSpec {
    /// Hook name slug
    #[serde(default)]
    pub name: Option<String>,

    /// Hook command slug
    pub command: String,

    /// Hook command arg slugs
    #[serde(default)]
    pub args: Vec<String>,

    /// Hook working dir slug
    #[serde(default)]
    pub working_dir: Option<Utf8PathBuf>,

    /// Hook env setup slugs
    #[serde(default)]
    pub env: IndexMap<String, String>,

    /// Hook trigger slug,
    #[serde(default)]
    pub when: Option<String>,

    /// Hook optional?
    #[serde(default)]
    pub optional: bool,

    /// Hook uses shell?
    #[serde(default)]
    pub shell: bool,
}

impl TryFrom<RawHookConfig> for HookConfig {
    type Error = CoreError;

    fn try_from(raw: RawHookConfig) -> CoreResult<Self> {
        Ok(Self {
            pre_generate: validate_hooks("hooks.pre_generate", raw.pre_generate)?,
            post_generate: validate_hooks("hooks.post_generate", raw.post_generate)?,
        })
    }
}

impl TryFrom<RawHookSpec> for HookSpec {
    type Error = CoreError;

    fn try_from(raw: RawHookSpec) -> CoreResult<Self> {
        if raw.command.trim().is_empty() {
            return Err(CoreError::invalid_manifest(
                "hook command must not be empty",
            ));
        }

        if let Some(working_dir) = raw.working_dir.as_ref() {
            validate_path("hook working dir", working_dir)?;
        }

        Ok(Self {
            name: raw.name,
            command: raw.command,
            args: raw.args,
            working_dir: raw.working_dir,
            env: raw.env,
            when: raw.when,
            optional: raw.optional,
            shell: raw.shell,
        })
    }
}

/// Validates manifest generation hooks.
///
/// # Arguments
/// - `section`: manifest section where the hooks are defined,
/// - `raw_hooks`: raw hooks parsed from template manifest.
///
/// # Returns
/// Vector of validated [`HookSpec`]s.
///
/// # Errors
/// Returns [`CoreError`] if:
/// - hook command is empty,
/// - provided hook command working dir is invalid.
pub(crate) fn validate_hooks(
    section: &str,
    raw_hooks: Vec<RawHookSpec>,
) -> CoreResult<Vec<HookSpec>> {
    raw_hooks
        .into_iter()
        .map(HookSpec::try_from)
        .collect::<CoreResult<Vec<_>>>()
        .map_err(|error| CoreError::invalid_manifest(format!("{section}: {error}")))
}
