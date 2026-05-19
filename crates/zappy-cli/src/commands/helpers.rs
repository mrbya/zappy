use std::path::{Path, PathBuf};
use std::process::Command as OsCommand;

use zappy_core::builtins::{DATE, DAY, EMAIL, MONTH, PROJECT_NAME, USER, YEAR};
use zappy_core::hooks::HookSpec;
use zappy_core::{GenerationPlan, ResolvedVariables, VariableValue, VariableValueMap};
use zappy_fs::{
    DiscoveredTemplate, DiscoveryConfig, InitTemplateInput, MaterializationOptions,
    TemplateCatalogue, create_directory, discover_templates, init_template_skeleton,
    materialize_generation_plan,
};
use zappy_hooks::{ExecuteHooksInput, HookPhase, execute_hooks};
use zappy_templates::ensure_bundled_templates_available;

use crate::diagnostics::{
    print_error_with_source, print_info, print_info_with_details, print_template_not_found,
    print_warning, print_warning_with_source,
};

/// Constructs discovery config.
pub fn discovery_config(templates_dir: Option<PathBuf>) -> DiscoveryConfig {
    tracing::debug!(
        explicit_templates_dir = ?templates_dir,
        "building template discovery config"
    );
    let bundled_templates_dir = if templates_dir.is_some() {
        None
    } else {
        match ensure_bundled_templates_available() {
            Ok(path) => {
                tracing::debug!(path = %path.display(), "prepared bundled templates");
                Some(path)
            }
            Err(error) => {
                tracing::warn!(explicit_templates_dir = ?templates_dir, %error, "bundled templates are unavailable");
                print_warning_with_source("failed to prepare bundled templates", error);
                None
            }
        }
    };

    DiscoveryConfig {
        templates_dir,
        bundled_templates_dir,
    }
}

/// Resolve command template.
///
/// # Returns
/// Some(`DiscoveredTemplate`) discovered template on succes, None on failure.
pub(super) fn resolve_template(
    templates_dir: Option<PathBuf>,
    id: &str,
) -> Option<DiscoveredTemplate> {
    let catalogue = build_templates_catalogue(templates_dir)?;

    tracing::debug!(
        discovered = catalogue.templates().len(),
        shadowed = catalogue.shadowed().len(),
        search_path = catalogue.search_paths().len(),
        "template discovery completed"
    );

    let Some(template) = catalogue.find_by_id(id) else {
        tracing::warn!(
            template = id,
            discovered = catalogue.templates().len(),
            search_paths = catalogue.search_paths().len(),
            "requested template was not found"
        );
        print_template_not_found(id, catalogue.search_paths(), catalogue.templates());
        return None;
    };

    tracing::info!(
        template = id,
        path = %template.template_dir.display(),
        "selected template"
    );

    Some(template.clone())
}

/// Builds template catalogue.
pub(super) fn build_templates_catalogue(
    templates_dir: Option<PathBuf>,
) -> Option<TemplateCatalogue> {
    let config = discovery_config(templates_dir);

    match discover_templates(&config) {
        Ok(catalogue) => Some(catalogue),
        Err(error) => {
            tracing::warn!(%error, "template discovery failed");
            print_error_with_source("failed to discover templates", error);
            None
        }
    }
}

/// Runs filesystem an hook execution paths for zappy commands.
///
/// # Returns
/// `Err(())` if plan materialization or hook execution fails, `Ok(())` otherwise.
pub(super) fn run_generation(
    no_hooks: bool,
    force: bool,
    template: &DiscoveredTemplate,
    resolved: &ResolvedVariables,
    plan: &GenerationPlan,
) -> Result<(), ()> {
    tracing::info!(
        output = %plan.output_dir.display(),
        operations = plan.operations.len(),
        "materializing generation plan"
    );

    if let Err(error) = create_directory(&plan.output_dir) {
        tracing::warn!(output = %plan.output_dir.display(), %error, "failed to create project output directory");
        print_error_with_source("failed to create project dir", error);
        return Err(());
    }

    if no_hooks {
        tracing::debug!("generation hooks disabled");
    }

    if !no_hooks
        && run_hooks(
            "pre-generate",
            HookPhase::PreGenerate,
            &template.manifest.hooks.pre_generate,
            &plan.output_dir,
            resolved,
        )
        .is_err()
    {
        return Err(());
    }

    let options = MaterializationOptions { force };

    let summary = match materialize_generation_plan(plan, options) {
        Ok(summary) => summary,
        Err(error) => {
            tracing::warn!(output = %plan.output_dir.display(), %error, "generation plan materialization failed");
            print_error_with_source("failed to generate project", error);
            return Err(());
        }
    };

    tracing::debug!(
        directories = summary.directories_created,
        text_files = summary.text_files_written,
        binary_files = summary.binary_files_copied,
        skipped = summary.skipped,
        "generation plan materialized"
    );

    if !no_hooks
        && run_hooks(
            "post-generate",
            HookPhase::PostGenerate,
            &template.manifest.hooks.post_generate,
            &plan.output_dir,
            resolved,
        )
        .is_err()
    {
        return Err(());
    }

    print_info_with_details(
        format!(
            "Generated `{}` in {}",
            &template.manifest.template.id.as_str(),
            plan.output_dir.display()
        ),
        format!(
            "Created {} directories, wrote {} text files, copied {} binary files, skipped {} \
             paths.",
            summary.directories_created,
            summary.text_files_written,
            summary.binary_files_copied,
            summary.skipped,
        ),
    );

    Ok(())
}

/// Creates template skeleton based on provided input.
///
/// # Returns
/// `Ok(())` on success `Err(())` otherwise.
pub(super) fn create_template_skeleton(input: &InitTemplateInput) -> Result<(), ()> {
    tracing::info!(output = %input.output_dir.display(), force = input.force, "initializing template skeleton");

    match init_template_skeleton(input) {
        Ok(()) => {
            tracing::debug!(output = %input.output_dir.display(), "template skeleton initialized");
            print_info(format!(
                "Initialized template skeleton at {}",
                input.output_dir.display()
            ));
            Ok(())
        }
        Err(error) => {
            tracing::warn!(output = %input.output_dir.display(), %error, "template skeleton initialization failed");
            print_error_with_source("failed to create template skeleton", error);
            Err(())
        }
    }
}

/// Date helper struct.
#[derive(Debug, Clone)]
struct DateParts {
    /// Full date.
    date: String,

    /// Day slice of the date.
    day: String,

    /// Month slice of the date.
    month: String,

    /// Year slice of the date.
    year: String,
}

impl DateParts {
    /// Constructs date.
    #[must_use]
    pub fn new() -> Self {
        let now =
            time::OffsetDateTime::now_local().unwrap_or_else(|_| time::OffsetDateTime::now_utc());

        let year = now.year();
        let month = now.month();
        let day = now.day();

        Self {
            date: format!("{year:04}-{month:02}-{day:02}"),
            day: format!("{day:02}"),
            month: format!("{month:02}"),
            year: format!("{year:04}"),
        }
    }
}

/// Constructs command built-in variables.
pub fn command_builtins(project_name: String) -> VariableValueMap {
    tracing::trace!(%project_name, "building CLI built-in variables");
    let mut builtins = VariableValueMap::new();

    let date = DateParts::new();
    let user = user_name();
    let email = user_email();

    builtins.insert(
        String::from(PROJECT_NAME),
        VariableValue::String(project_name),
    );
    builtins.insert(String::from(USER), VariableValue::String(user));
    builtins.insert(String::from(EMAIL), VariableValue::String(email));
    builtins.insert(String::from(DATE), VariableValue::String(date.date));
    builtins.insert(String::from(DAY), VariableValue::String(date.day));
    builtins.insert(String::from(MONTH), VariableValue::String(date.month));
    builtins.insert(String::from(YEAR), VariableValue::String(date.year));

    builtins
}

/// Retrieves host username.
fn user_name() -> String {
    if let Some(user) = git_user_name() {
        tracing::trace!("using git-configured user name");
        return user;
    }

    if let Some(user) = env_user() {
        tracing::trace!("using environment user name");
        return user;
    }

    tracing::debug!("falling back to placeholder user name");
    String::from("{TODO: add username}")
}

/// Retrieves git username.
fn git_user_name() -> Option<String> {
    let output = OsCommand::new("git")
        .args(["config", "user.name"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let value = String::from_utf8_lossy(&output.stdout).trim().to_owned();

    if value.is_empty() {
        return None;
    }

    Some(value)
}

/// Retrieves env username.
fn env_user() -> Option<String> {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .ok()
        .filter(|value| !value.trim().is_empty())
}

/// Retrieves user email.
fn user_email() -> String {
    if let Some(email) = git_user_email() {
        tracing::trace!("using git-configured user email");
        return email;
    }

    tracing::debug!("falling back to placeholder user email");
    String::from("{TODO: add user email}")
}

/// Retrieves user git email.
fn git_user_email() -> Option<String> {
    let output = OsCommand::new("git")
        .args(["config", "user.email"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let value = String::from_utf8_lossy(&output.stdout).trim().to_owned();

    if value.is_empty() {
        return None;
    }

    Some(value)
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
) -> Result<(), ()> {
    let input = ExecuteHooksInput {
        phase,
        hooks,
        output_dir,
        variables: resolved,
    };

    tracing::debug!(
        phase = phase_name,
        hook_count = hooks.len(),
        "running hooks"
    );

    let summary = match execute_hooks(&input) {
        Ok(summary) => summary,
        Err(error) => {
            tracing::warn!(phase = phase_name, %error, "hook execution failed");
            print_error_with_source("hook execution failed", error);
            return Err(());
        }
    };

    print_hook_summary(phase_name, &summary);
    Ok(())
}

/// Prints hooks execution summary.
pub(super) fn print_hook_summary(phase: &str, summary: &zappy_hooks::HookExecutionSummary) {
    if summary.executed == 0 && summary.skipped == 0 && summary.optional_failed == 0 {
        tracing::trace!(phase, "hook summary empty");
        return;
    }

    tracing::debug!(
        phase,
        executed = summary.executed,
        skipped = summary.skipped,
        optional_failed = summary.optional_failed,
        warning_count = summary.warnings.len(),
        "hook phase completed"
    );

    print_info_with_details(
        format!("hooks ({phase}) done"),
        format!(
            "Hooks - executed {}, skipped {}, optional failures {}.",
            summary.executed, summary.skipped, summary.optional_failed
        ),
    );

    for warning in &summary.warnings {
        tracing::warn!(phase, warning, "optional hook reported warning");
        print_warning(warning);
    }
}
