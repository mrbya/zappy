use std::process::ExitCode;

use zappy_core::{VariableResolutionInput, VariableValueMap, resolve_variables};
use zappy_fs::{DiscoveryConfig, create_directory, discover_templates};

use crate::cli::NewArgs;

/// Zappy command: new.
pub fn new(args: &NewArgs) -> ExitCode {
    match zappy_core::parse_variable_overrides(args.vars.iter()) {
        Ok(overrides) => {
            println!("Zappy new command:");
            println!("template: {}", args.template);
            println!("project: {}", args.project_name);

            let config = DiscoveryConfig {
                templates_dir: args.templates_dir.clone(),
            };

            let catalogue = match discover_templates(&config) {
                Ok(catalogue) => catalogue,
                Err(error) => {
                    eprintln!("Error: {error}");
                    return ExitCode::FAILURE;
                }
            };

            let Some(template) = catalogue.find_by_id(&args.template) else {
                eprintln!("Error: template `{}` was not found", args.template);
                return ExitCode::FAILURE;
            };

            let input = VariableResolutionInput {
                explicit: overrides,
                interactive: VariableValueMap::new(),
                user_defaults: VariableValueMap::new(),
                builtins: new_command_builtins(args),
            };

            let resolved = match resolve_variables(&template.manifest.variables, &input) {
                Ok(resolved) => resolved,
                Err(error) => {
                    eprintln!("Error: {error}");
                    return ExitCode::FAILURE;
                }
            };

            let output_dir = args
                .output
                .clone()
                .unwrap_or_else(|| std::path::PathBuf::from(&args.project_name));

            let plan_input = zappy_fs::BuildPlanInput {
                template_dir: &template.template_dir,
                manifest: &template.manifest,
                variables: &resolved,
                output_dir,
                force: args.force,
            };

            let plan = match zappy_fs::build_generation_plan(&plan_input) {
                Ok(plan) => plan,
                Err(error) => {
                    eprintln!("Error: {error}");
                    return ExitCode::FAILURE;
                }
            };

            if args.dry_run {
                println!();
                print_generation_plan(&plan);
                return ExitCode::SUCCESS;
            }

            run_generation(args, template, &resolved, &plan)
        }
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}

/// Runs the filesystem and hook execution path for `zappy new`.
fn run_generation(
    args: &NewArgs,
    template: &zappy_fs::DiscoveredTemplate,
    resolved: &zappy_core::ResolvedVariables,
    plan: &zappy_core::GenerationPlan,
) -> ExitCode {
    if let Err(error) = create_directory(&plan.output_dir) {
        eprintln!("Error: {error}");
        return ExitCode::FAILURE;
    }

    if !args.no_hooks
        && !run_generation_hooks(
            "pre-generate",
            zappy_hooks::HookPhase::PreGenerate,
            &template.manifest.hooks.pre_generate,
            &plan.output_dir,
            resolved,
        )
    {
        return ExitCode::FAILURE;
    }

    let options = if args.force {
        zappy_fs::MaterializationOptions::force()
    } else {
        zappy_fs::MaterializationOptions::no_force()
    };

    let summary = match zappy_fs::materialize_generation_plan(plan, options) {
        Ok(summary) => summary,
        Err(error) => {
            eprintln!("Error: {error}");
            return ExitCode::FAILURE;
        }
    };

    if !args.no_hooks
        && !run_generation_hooks(
            "post-generate",
            zappy_hooks::HookPhase::PostGenerate,
            &template.manifest.hooks.post_generate,
            &plan.output_dir,
            resolved,
        )
    {
        return ExitCode::FAILURE;
    }

    println!(
        "Generated `{}` in {}",
        args.template,
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

/// Executes generation hooks for one phase and prints the phase summary.
fn run_generation_hooks(
    phase_name: &str,
    phase: zappy_hooks::HookPhase,
    hooks: &[zappy_core::hooks::HookSpec],
    output_dir: &std::path::Path,
    resolved: &zappy_core::ResolvedVariables,
) -> bool {
    let input = zappy_hooks::ExecuteHooksInput {
        phase,
        hooks,
        output_dir,
        variables: resolved,
    };

    let summary = match zappy_hooks::execute_hooks(&input) {
        Ok(summary) => summary,
        Err(error) => {
            eprintln!("Error: {error}");
            return false;
        }
    };

    print_hook_summary(phase_name, &summary);
    true
}

/// Prints generation plan for the `new` command dry-run.
fn print_generation_plan(plan: &zappy_core::GenerationPlan) {
    println!(
        "Dry-run generation plan for `{}`",
        plan.template_id.as_str(),
    );
    println!("Output: {}", plan.output_dir.display());

    if !plan.warnings.is_empty() {
        println!();
        println!("Warnings:");

        for warning in plan.warnings.iter().cloned() {
            match warning {
                zappy_core::PlanWarning::DestinationExists { destination } => {
                    println!("WARN         {} already exists", destination.display());
                }
                zappy_core::PlanWarning::NonUtf8FileCopiedAsBinary { source } => {
                    println!(
                        "WARN         {} is not UTF-8; copying as binary",
                        source.display()
                    );
                }
            }
        }
    }

    println!();

    for operation in plan.operations.clone() {
        match operation {
            zappy_core::PlanOperation::CreateDirectory { destination, .. } => {
                println!("CREATE DIR   {}", destination.display());
            }
            zappy_core::PlanOperation::RenderTextFile {
                source,
                destination,
                ..
            } => {
                println!(
                    "RENDER       {} -> {}",
                    source.display(),
                    destination.display()
                );
            }
            zappy_core::PlanOperation::CopyBinaryFile {
                source,
                destination,
            } => {
                println!(
                    "COPY         {} -> {}",
                    source.display(),
                    destination.display()
                );
            }
            zappy_core::PlanOperation::Skip { source, reason } => {
                println!("SKIP         {} [{reason:?}]", source.display());
            }
        }
    }
}

/// Builds builtin variable map from `new` command arguments.
fn new_command_builtins(args: &NewArgs) -> VariableValueMap {
    let mut builtins = VariableValueMap::new();

    builtins.insert(
        String::from(zappy_core::builtins::PROJECT_NAME),
        zappy_core::VariableValue::String(args.project_name.clone()),
    );

    builtins
}

/// Prints hooks execution summary.
fn print_hook_summary(phase: &str, summary: &zappy_hooks::HookExecutionSummary) {
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
