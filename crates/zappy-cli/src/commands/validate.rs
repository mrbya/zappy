use std::path::{Path, PathBuf};
use std::process::ExitCode;

use tempfile::TempDir;
use zappy_core::validation::ValidationConfig;
use zappy_core::{VariableResolutionInput, VariableValueMap, resolve_variables};
use zappy_fs::{BuildPlanInput, build_generation_plan};
use zappy_hooks::HookPhase;

use crate::cli::ValidateArgs;
use crate::commands::helpers::{command_builtins, resolve_template, run_generation, run_hooks};

/// Validate command stub.
pub fn validate(args: &ValidateArgs) -> ExitCode {
    let Some(template) = resolve_template(args.templates_dir.clone(), &args.template) else {
        return ExitCode::FAILURE;
    };

    let Some(validation) = template.manifest.validation.as_ref() else {
        eprintln!(
            "Error: template `{}` does not define validation config",
            args.template
        );
        return ExitCode::FAILURE;
    };

    let temp_dir = match TempDir::new() {
        Ok(temp_dir) => temp_dir,
        Err(error) => {
            eprintln!("Error: failed to create validation temp dir: {error}");
            return ExitCode::FAILURE;
        }
    };

    let output_dir = validation_output_dir(temp_dir.path(), validation);

    let project_name = output_dir
        .file_name()
        .and_then(|name| name.to_str())
        .map_or_else(|| String::from("zappy-validation-output"), String::from);
    let builtins = command_builtins(project_name);
    let input = VariableResolutionInput {
        explicit: validation.variables.clone(),
        interactive: VariableValueMap::new(),
        user_defaults: VariableValueMap::new(),
        builtins,
    };

    let resolved = match resolve_variables(&template.manifest.variables, &input) {
        Ok(resolved) => resolved,
        Err(error) => {
            eprintln!("Error: {error}");
            return ExitCode::FAILURE;
        }
    };

    let plan_input = BuildPlanInput {
        template_dir: &template.template_dir,
        manifest: &template.manifest,
        variables: &resolved,
        output_dir: output_dir.clone(),
        force: true,
    };

    let plan = match build_generation_plan(&plan_input) {
        Ok(plan) => plan,
        Err(error) => {
            eprintln!("Error: {error}");
            return ExitCode::FAILURE;
        }
    };

    if run_generation(args.no_hooks, true, &template, &resolved, &plan) {
        return ExitCode::FAILURE;
    }

    let setup_result = run_hooks(
        "validation setup",
        HookPhase::ValidationSetup,
        &validation.setup,
        &output_dir,
        &resolved,
    );

    let steps_result = if setup_result {
        true
    } else {
        run_hooks(
            "validation steps",
            HookPhase::ValidationStep,
            &validation.steps,
            &output_dir,
            &resolved,
        )
    };

    if steps_result
        | run_hooks(
            "validation teardown",
            HookPhase::ValidationTeardown,
            &validation.teardown,
            &output_dir,
            &resolved,
        )
    {
        return ExitCode::FAILURE;
    }

    if args.keep_temp {
        let temp_path = temp_dir.keep();
        if !temp_path.exists() {
            eprintln!(
                "Error: failed to keep validation temp dir `{}`",
                temp_path.display()
            );
            return ExitCode::SUCCESS;
        }

        println!("Validation temp dir kept at {}", temp_path.display());
    }

    println!("Template `{}` validated successfully.", args.template);
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
