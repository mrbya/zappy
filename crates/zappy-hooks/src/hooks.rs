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
    tracing::debug!(phase = ?input.phase, hook_count = input.hooks.len(), output_dir = %input.output_dir.display(), "executing hook phase");
    let mut summary = HookExecutionSummary::default();

    for hook in input.hooks {
        if !should_run_hook(hook, input.variables) {
            tracing::trace!(phase = ?input.phase, hook = hook_name(hook), "skipping hook because condition evaluated to false");
            summary.skipped = summary.skipped.checked_add(1).unwrap_or(summary.skipped);
            continue;
        }

        match execute_hook(hook, input.output_dir, input.variables) {
            Ok(()) => {
                tracing::trace!(phase = ?input.phase, hook = hook_name(hook), "hook executed successfully");
                summary.executed = summary.executed.checked_add(1).unwrap_or(summary.executed);
            }
            Err(error) if hook.optional => {
                tracing::warn!(phase = ?input.phase, hook = hook_name(hook), "optional hook failed");
                summary.optional_failed = summary
                    .optional_failed
                    .checked_add(1)
                    .unwrap_or(summary.optional_failed);

                summary.warnings.push(error.to_string());
            }
            Err(error) => return Err(error),
        }
    }

    tracing::debug!(
        phase = ?input.phase,
        executed = summary.executed,
        skipped = summary.skipped,
        optional_failed = summary.optional_failed,
        warning_count = summary.warnings.len(),
        "hook phase completed"
    );

    Ok(summary)
}

/// Returns true if a hook should run.
fn should_run_hook(hook: &HookSpec, variables: &ResolvedVariables) -> bool {
    let should_run = hook
        .when
        .as_deref()
        .is_none_or(|name| evaluate_condition(name, &variables.values));

    tracing::trace!(hook = hook_name(hook), condition = ?hook.when, should_run, "evaluated hook run condition");

    should_run
}

/// Executes a hook.
fn execute_hook(
    hook: &HookSpec,
    output_dir: &Path,
    variables: &ResolvedVariables,
) -> HooksResult<()> {
    let name = hook_name(hook);

    tracing::trace!(
        hook = name,
        optional = hook.optional,
        shell = hook.shell,
        arg_count = hook.args.len(),
        env_count = hook.env.len(),
        "preparing hook execution"
    );

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

    tracing::trace!(hook = name, command, working_dir = %working_dir.display(), "resolved hook command inputs");

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
        tracing::trace!(hook = name, status = %output.status, "hook command finished successfully");
        return Ok(());
    }

    tracing::warn!(hook = name, status = %output.status, "hook command returned non-zero status");

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
        tracing::trace!(hook = hook_name(hook), output_dir = %output_dir.display(), "using output directory as hook working directory");
        return Ok(output_dir.to_path_buf());
    };

    let rendered =
        render_relative_path(working_dir.as_str(), &variables.replacements).map_err(|source| {
            Box::new(HooksError::RenderWorkingDir {
                name: hook_name(hook),
                source,
            })
        })?;

    tracing::trace!(
        hook = hook_name(hook),
        rendered_working_dir = rendered.as_str(),
        "rendered hook working directory"
    );

    Ok(output_dir.join(rendered))
}

/// Returns a human-readable hook name.
fn hook_name(hook: &HookSpec) -> String {
    hook.name.clone().unwrap_or_else(|| hook.command.clone())
}
