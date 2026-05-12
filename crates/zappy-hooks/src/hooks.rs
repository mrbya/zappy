use std::path::{Path, PathBuf};
use std::process::Command;

use zappy_core::ResolvedVariables;
use zappy_core::condition::evaluate_condition;
use zappy_core::hooks::HookSpec;
use zappy_core::render::{render_relative_path, render_text};

use crate::error::{HooksError, HooksResult};

/// Hook execution phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookPhase {
    /// Before generation files are materialized.
    PreGenerate,

    /// After generation files are materialized.
    PostGenerate,

    /// Setup hooks before validation.
    ValidationSetup,

    /// Validation step hook.
    ValidationStep,

    /// Teardown hooks after validation.
    ValidationTeardown,
}

/// Input for executing hooks.
#[derive(Debug, Clone)]
pub struct ExecuteHooksInput<'a> {
    /// Hook phase,
    pub phase: HookPhase,

    /// Hooks to execute.
    pub hooks: &'a [HookSpec],

    /// Output directory of the generated project.
    pub output_dir: &'a Path,

    /// Resolved variables.
    pub variables: &'a ResolvedVariables,
}

/// Hook execution summary.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct HookExecutionSummary {
    /// Succesfully executed hooks.
    pub executed: usize,

    /// Hooks skipped because their condition evaluated to false.
    pub skipped: usize,

    /// Optional hooks that failed.
    pub optional_failed: usize,

    /// Optional hook failure messages.
    pub warnings: Vec<String>,
}

/// Executes hooks in order.
///
/// # Arguments
/// - `input`: hooks execution input config.
///
/// # Returns
/// Ok(`HookExecutionSummary`) summary on success.
///
/// # Errors
/// Returns [`HooksError`] if hook execution fails.
pub fn execute_hooks(input: &ExecuteHooksInput<'_>) -> HooksResult<HookExecutionSummary> {
    let mut summary = HookExecutionSummary::default();

    for hook in input.hooks {
        if !should_run_hook(hook, input.variables) {
            summary.skipped = summary.skipped.checked_add(1).unwrap_or(summary.skipped);
            continue;
        }

        match execute_hook(hook, input.output_dir, input.variables) {
            Ok(()) => {
                summary.executed = summary.executed.checked_add(1).unwrap_or(summary.executed);
            }
            Err(error) if hook.optional => {
                summary.optional_failed = summary
                    .optional_failed
                    .checked_add(1)
                    .unwrap_or(summary.optional_failed);

                summary.warnings.push(error.to_string());
            }
            Err(error) => return Err(error),
        }
    }

    Ok(summary)
}

/// Returns true if a hook should run.
fn should_run_hook(hook: &HookSpec, variables: &ResolvedVariables) -> bool {
    hook.when
        .as_deref()
        .is_none_or(|name| evaluate_condition(name, &variables.values))
}

/// Executes a hook.
fn execute_hook(
    hook: &HookSpec,
    output_dir: &Path,
    variables: &ResolvedVariables,
) -> HooksResult<()> {
    let name = hook_name(hook);

    if hook.shell {
        return Err(Box::new(HooksError::ShellUnsupported { name }));
    }

    let command = render_text(&hook.command, &variables.replacements);
    let args = hook
        .args
        .iter()
        .map(|arg| render_text(arg, &variables.replacements))
        .collect::<Vec<_>>();

    let working_dir = resolve_working_dir(hook, output_dir, variables)?;

    let mut command_builder = Command::new(&command);
    command_builder.args(args);
    command_builder.current_dir(&working_dir);

    for (key, value) in &hook.env {
        command_builder.env(key, render_text(value, &variables.replacements));
    }

    let output = command_builder.output().map_err(|source| {
        Box::new(HooksError::Spawn {
            name: name.clone(),
            command: command.clone(),
            working_dir: working_dir.clone(),
            source,
        })
    })?;

    if output.status.success() {
        return Ok(());
    }

    Err(Box::new(HooksError::Failed {
        name,
        status: output.status.to_string(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    }))
}

/// Resolves hook working directory.
///
/// If unset, hooks run in the generated project output directory.
/// If set, the path is rendered and interpreted relative to output dir.
fn resolve_working_dir(
    hook: &HookSpec,
    output_dir: &Path,
    variables: &ResolvedVariables,
) -> HooksResult<PathBuf> {
    let Some(working_dir) = hook.working_dir.as_ref() else {
        return Ok(output_dir.to_path_buf());
    };

    let rendered =
        render_relative_path(working_dir.as_str(), &variables.replacements).map_err(|source| {
            Box::new(HooksError::RenderWorkingDir {
                name: hook_name(hook),
                source,
            })
        })?;

    Ok(output_dir.join(rendered))
}

/// Returns a human-readable hook name.
fn hook_name(hook: &HookSpec) -> String {
    hook.name.clone().unwrap_or_else(|| hook.command.clone())
}
