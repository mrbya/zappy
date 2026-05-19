use std::process::ExitCode;

use zappy_core::{VariableResolutionInput, VariableValueMap, resolve_variables};

use crate::cli::NewArgs;
use crate::commands::helpers::{command_builtins, resolve_template, run_generation};
use crate::diagnostics::{DiagnosticReport, print_error_with_source, print_info_with_details};

/// Zappy command: new.
pub fn new(args: &NewArgs) -> ExitCode {
    match zappy_core::parse_variable_overrides(args.vars.iter()) {
        Ok(overrides) => {
            DiagnosticReport::info("Zappy `new` command")
                .detail("Generating:")
                .detail(format!("template {}", args.template))
                .detail(format!("project {}", args.project_name))
                .print();

            let Some(template) = resolve_template(args.templates_dir.clone(), &args.template)
            else {
                return ExitCode::FAILURE;
            };

            let input = VariableResolutionInput {
                explicit: overrides,
                interactive: VariableValueMap::new(),
                user_defaults: VariableValueMap::new(),
                builtins: command_builtins(args.project_name.clone()),
            };

            let resolved = match resolve_variables(&template.manifest.variables, &input) {
                Ok(resolved) => resolved,
                Err(error) => {
                    print_error_with_source("failed to resolve variables", error);
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
                    print_error_with_source("failed to build generation plan", error);
                    return ExitCode::FAILURE;
                }
            };

            if args.dry_run {
                println!();
                print_generation_plan(&plan);
                return ExitCode::SUCCESS;
            }

            //run_generation(args, template, &resolved, &plan)
            if run_generation(args.no_hooks, args.force, &template, &resolved, &plan).is_err() {
                return ExitCode::FAILURE;
            }

            ExitCode::SUCCESS
        }
        Err(error) => {
            print_error_with_source("failed to parse variable overrides", error);
            ExitCode::FAILURE
        }
    }
}

/// Prints generation plan for the `new` command dry-run.
fn print_generation_plan(plan: &zappy_core::GenerationPlan) {
    print_info_with_details(
        format!("Dry-run generation plan for {}", plan.template_id.as_str()),
        String::new(),
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
