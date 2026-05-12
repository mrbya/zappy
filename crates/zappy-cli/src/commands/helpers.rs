use std::path::Path;
use std::process::ExitCode;

use zappy_core::builtins::PROJECT_NAME;
use zappy_core::hooks::HookSpec;
use zappy_core::{GenerationPlan, ResolvedVariables, VariableValue, VariableValueMap};
use zappy_fs::{
    DiscoveredTemplate, MaterializationOptions, create_directory, materialize_generation_plan,
};
use zappy_hooks::{ExecuteHooksInput, HookPhase, execute_hooks};

/// Runs filesystem an hook execution paths for zappy commands.
///
/// # Returns
/// [`ExitCode::SUCCESS`] on success, [`ExitCode::FAILURE`] on failure.
pub(super) fn run_generation(
    no_hooks: bool,
    force: bool,
    template: &DiscoveredTemplate,
    resolved: &ResolvedVariables,
    plan: &GenerationPlan,
) -> ExitCode {
    if let Err(error) = create_directory(&plan.output_dir) {
        eprintln!("Error: {error}");
        return ExitCode::FAILURE;
    }

    if !no_hooks
        && run_hooks(
            "pre-generate",
            HookPhase::PreGenerate,
            &template.manifest.hooks.pre_generate,
            &plan.output_dir,
            resolved,
        )
    {
        return ExitCode::FAILURE;
    }

    let options = MaterializationOptions { force };

    let summary = match materialize_generation_plan(plan, options) {
        Ok(summary) => summary,
        Err(error) => {
            eprintln!("Error: {error}");
            return ExitCode::FAILURE;
        }
    };

    if !no_hooks
        && run_hooks(
            "post-generate",
            HookPhase::PostGenerate,
            &template.manifest.hooks.post_generate,
            &plan.output_dir,
            resolved,
        )
    {
        return ExitCode::FAILURE;
    }

    println!(
        "Generated `{}` in {}",
        &template.manifest.template.id.as_str(),
        plan.output_dir.display()
    );
    println!(
        "Created {} directories, wrote {} text files, copied {} binary files, skipped {} paths.",
        summary.directories_created,
        summary.text_files_written,
        summary.binary_files_copied,
        summary.skipped,
    );

    ExitCode::SUCCESS
}

/// Constructs command built-in variables.
pub(super) fn command_builtins(project_name: String) -> VariableValueMap {
    let mut builtins = VariableValueMap::new();

    builtins.insert(
        String::from(PROJECT_NAME),
        VariableValue::String(project_name),
    );

    builtins
}

/// Execute hooks for a single command phase.
///
/// # Returns
/// `true` if hook execution fails, `false` otherwise.
pub(super) fn run_hooks(
    phase_name: &str,
    phase: HookPhase,
    hooks: &[HookSpec],
    output_dir: &Path,
    resolved: &ResolvedVariables,
) -> bool {
    let input = ExecuteHooksInput {
        phase,
        hooks,
        output_dir,
        variables: resolved,
    };

    let summary = match execute_hooks(&input) {
        Ok(summary) => summary,
        Err(error) => {
            eprintln!("Error: {error}");
            return true;
        }
    };

    print_hook_summary(phase_name, &summary);
    false
}

/// Prints hooks execution summary.
pub(super) fn print_hook_summary(phase: &str, summary: &zappy_hooks::HookExecutionSummary) {
    if summary.executed == 0 && summary.skipped == 0 && summary.optional_failed == 0 {
        return;
    }

    println!(
        "Hooks ({phase}): executed {}, skipped {}, optional failures {}.",
        summary.executed, summary.skipped, summary.optional_failed,
    );

    for warning in &summary.warnings {
        eprintln!("Warning: {warning}");
    }
}
