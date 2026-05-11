use std::process::ExitCode;

use zappy_core::{resolve_variables, VariableResolutionInput, VariableValueMap};
use zappy_fs::{create_directory, discover_templates, DiscoveredTemplate, DiscoveryConfig};

use crate::cli::{CreateArgs, InfoArgs, InitArgs, ListArgs, NewArgs, ValidateArgs};

/// List command stub.
pub fn list(args: &ListArgs) -> ExitCode {
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

    let templates = catalogue
        .templates()
        .iter()
        .filter(|template| match_language_filter(template, args.language.as_deref()))
        .collect::<Vec<_>>();

    if templates.is_empty() {
        println!("No templates found.");
        return ExitCode::SUCCESS;
    }

    print_template_list(&templates);

    ExitCode::SUCCESS
}

/// Info command stub.
pub fn info(args: &InfoArgs) -> ExitCode {
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

    print_template_info(template);

    ExitCode::SUCCESS
}

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
                output_dir: output_dir.clone(),
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

            if let Err(error) = create_directory(&output_dir) {
                eprintln!("Error: {error}");
                return ExitCode::FAILURE;
            }

            if !args.no_hooks {
                let pre_input = zappy_hooks::ExecuteHooksInput {
                    phase: zappy_hooks::HookPhase::PreGenerate,
                    hooks: &template.manifest.hooks.pre_generate,
                    output_dir: &plan.output_dir,
                    variables: &resolved,
                };

                let pre_summary = match zappy_hooks::execute_hooks(&pre_input) {
                    Ok(pre_summary) => pre_summary,
                    Err(error) => {
                        eprint!("Error {error}");
                        return ExitCode::FAILURE;
                    }
                };

                print_hook_summary("pre-generate", &pre_summary);
            }

            let options = if args.force {
                zappy_fs::MaterializationOptions::force()
            } else {
                zappy_fs::MaterializationOptions::no_force()
            };

            let summary = match zappy_fs::materialize_generation_plan(&plan, options) {
                Ok(summary) => summary,
                Err(error) => {
                    eprint!("Error: {error}");
                    return ExitCode::FAILURE;
                }
            };

            println!(
                "Generated `{}` in {}",
                args.template,
                plan.output_dir.display()
            );
            println!(
                "Created {} directories, wrote {} text files, copied {} binary files, skipped {} \
                 paths.",
                summary.directories_created,
                summary.text_files_written,
                summary.binary_files_copied,
                summary.skipped,
            );

            if !args.no_hooks {
                let post_input = zappy_hooks::ExecuteHooksInput {
                    phase: zappy_hooks::HookPhase::PostGenerate,
                    hooks: &template.manifest.hooks.post_generate,
                    output_dir: &plan.output_dir,
                    variables: &resolved,
                };

                let post_summary = match zappy_hooks::execute_hooks(&post_input) {
                    Ok(post_summary) => post_summary,
                    Err(error) => {
                        eprint!("Error {error}");
                        return ExitCode::FAILURE;
                    }
                };

                print_hook_summary("post-generate", &post_summary);
            }

            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

/// Validate command stub.
pub fn validate(args: &ValidateArgs) -> ExitCode {
    println!("zappy validate: stub");
    println!("{args:#?}");
    ExitCode::SUCCESS
}

/// Init command stub.
pub fn init_template(args: &InitArgs) -> ExitCode {
    println!("zappy init-template: stub");
    println!("{args:#?}");
    ExitCode::SUCCESS
}

/// Create command stub.
pub fn create(args: &CreateArgs) -> ExitCode {
    println!("zappy create: stub");
    println!("{args:#?}");
    ExitCode::SUCCESS
}

/// Predicate used to filter templates based on a specific language if provided.
fn match_language_filter(template: &DiscoveredTemplate, language: Option<&str>) -> bool {
    let Some(language) = language else {
        return true;
    };

    template.manifest.template.language.as_deref() == Some(language)
}

/// Prints a compact template list.
fn print_template_list(templates: &[&DiscoveredTemplate]) {
    println!("{:<24} {:<16} Name", "ID", "Language");
    println!("{:-<24} {:-<16} {:-<24}", "", "", "");

    for template in templates {
        let metadata = &template.manifest.template;
        let language = metadata.language.as_deref().unwrap_or("-");
        println!(
            "{:<24} {:<16} {}",
            metadata.id.as_str(),
            language,
            metadata.name,
        );
    }
}

/// Prints detailed information about a template.
fn print_template_info(template: &DiscoveredTemplate) {
    let metadata = &template.manifest.template;

    println!("ID:          {}", metadata.id.as_str());
    println!("Name:        {}", metadata.name);

    if let Some(description) = metadata.description.as_deref() {
        println!("Description: {description}");
    }

    if let Some(language) = metadata.language.as_deref() {
        println!("Language:    {language}");
    }

    if let Some(version) = metadata.version.as_deref() {
        println!("Version:     {version}");
    }

    if !metadata.authors.is_empty() {
        println!("Authors:     {}", metadata.authors.join(", "));
    }

    println!("Source root: {}", metadata.source.root);
    println!("Template dir: {}", template.template_dir.display());
    println!("Manifest:    {}", template.manifest_path.display());
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
