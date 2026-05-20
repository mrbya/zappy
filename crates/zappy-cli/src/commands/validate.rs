use std::path::{Path, PathBuf};
use std::process::ExitCode;

use tempfile::TempDir;
use zappy_core::validation::ValidationConfig;
use zappy_core::{VariableResolutionInput, VariableValueMap, resolve_variables};
use zappy_fs::{BuildPlanInput, build_generation_plan};
use zappy_hooks::HookPhase;

use crate::cli::ValidateArgs;
use crate::commands::helpers::{command_builtins, resolve_template, run_generation, run_hooks};
use crate::diagnostics::{print_error, print_error_with_source, print_info};

/// Validate command stub.
pub fn validate(args: &ValidateArgs) -> ExitCode {
    tracing::info!(template = %args.template, explicit_templates_dir = ?args.templates_dir, keep_temp = args.keep_temp, no_hooks = args.no_hooks, "validating template");

    let Some(template) = resolve_template(args.templates_dir.clone(), &args.template) else {
        return ExitCode::FAILURE;
    };

    let Some(validation) = template.manifest.validation.as_ref() else {
        tracing::warn!(template = %args.template, "validation requested for template without validation config");
        print_error(format!(
            "template `{}` does not define validation config",
            args.template
        ));
        return ExitCode::FAILURE;
    };

    let temp_dir = match TempDir::new() {
        Ok(temp_dir) => temp_dir,
        Err(error) => {
            tracing::warn!(template = %args.template, %error, "failed to create validation temp dir");
            print_error_with_source("failed to create validation temp dir", error);
            return ExitCode::FAILURE;
        }
    };

    let output_dir = validation_output_dir(temp_dir.path(), validation);
    tracing::debug!(output = %output_dir.display(), "prepared validation output directory");

    let Some((resolved, plan)) = resolve_validation_plan(
        args,
        &template.manifest,
        &template.template_dir,
        validation,
        &output_dir,
    ) else {
        return ExitCode::FAILURE;
    };

    if run_generation(args.no_hooks, true, &template, &resolved, &plan).is_err() {
        return ExitCode::FAILURE;
    }

    if run_validation_hooks(args, validation, &output_dir, &resolved) {
        tracing::warn!(template = %args.template, "validation hooks reported failure");
        return ExitCode::FAILURE;
    }

    maybe_keep_validation_temp_dir(args.keep_temp, temp_dir);

    tracing::info!(template = %args.template, "template validation completed successfully");
    print_info(format!(
        "template `{}` validated successfully",
        args.template
    ));
    ExitCode::SUCCESS
}

/// Returns validation output directory.
fn validation_output_dir(temp_root: &Path, validation: &ValidationConfig) -> PathBuf {
    let output_dir_name = validation
        .output_dir_name
        .as_deref()
        .unwrap_or("zappy-validation-output");

    temp_root.join(output_dir_name)
}

/// Generates validation project name.
fn generate_project_name(output_dir: &Path) -> String {
    output_dir
        .file_name()
        .and_then(|name| name.to_str())
        .map_or_else(|| String::from("zappy-validation-output"), String::from)
}

/// Resolves variables and builds the validation generation plan.
fn resolve_validation_plan(
    args: &ValidateArgs,
    manifest: &zappy_core::Manifest,
    template_dir: &Path,
    validation: &ValidationConfig,
    output_dir: &Path,
) -> Option<(zappy_core::ResolvedVariables, zappy_core::GenerationPlan)> {
    let project_name = generate_project_name(output_dir);
    let builtins = command_builtins(project_name);
    let input = VariableResolutionInput {
        explicit: validation.variables.clone(),
        interactive: VariableValueMap::new(),
        user_defaults: VariableValueMap::new(),
        builtins,
    };

    let resolved = match resolve_variables(&manifest.variables, &input) {
        Ok(resolved) => resolved,
        Err(error) => {
            tracing::warn!(template = %args.template, %error, "failed to resolve validation variables");
            print_error_with_source("failed to resolve variables", error);
            return None;
        }
    };

    let plan_input = BuildPlanInput {
        template_dir,
        manifest,
        variables: &resolved,
        output_dir: output_dir.to_path_buf(),
        force: true,
    };

    let plan = match build_generation_plan(&plan_input) {
        Ok(plan) => plan,
        Err(error) => {
            tracing::warn!(template = %args.template, output = %output_dir.display(), %error, "failed to build validation generation plan");
            print_error_with_source("failed to build generation plan", error);
            return None;
        }
    };

    Some((resolved, plan))
}

/// Runs validation setup steps and teardown hooks.
fn run_validation_hooks(
    args: &ValidateArgs,
    validation: &ValidationConfig,
    output_dir: &Path,
    resolved: &zappy_core::ResolvedVariables,
) -> bool {
    let setup_result = run_hooks(
        "validation setup",
        HookPhase::ValidationSetup,
        &validation.setup,
        output_dir,
        resolved,
    );

    let steps_failed = if setup_result.is_err() {
        tracing::debug!(template = %args.template, "skipping validation steps because setup failed");
        true
    } else {
        run_hooks(
            "validation steps",
            HookPhase::ValidationStep,
            &validation.steps,
            output_dir,
            resolved,
        )
        .is_err()
    };

    steps_failed
        | run_hooks(
            "validation teardown",
            HookPhase::ValidationTeardown,
            &validation.teardown,
            output_dir,
            resolved,
        )
        .is_err()
}

/// Keeps the validation temp directory if requested.
fn maybe_keep_validation_temp_dir(keep_temp: bool, temp_dir: TempDir) {
    if !keep_temp {
        return;
    }

    let temp_path = temp_dir.keep();
    if !temp_path.exists() {
        tracing::warn!(path = %temp_path.display(), "validation temp dir was not preserved after keep request");
        print_error(format!(
            "failed to keep validation temp dir @ `{}`",
            temp_path.display()
        ));
        return;
    }

    tracing::info!(path = %temp_path.display(), "validation temp dir kept");
    print_info(format!(
        "validation temp tir kept @ {}",
        temp_path.display()
    ));
}
