use std::process::ExitCode;

use zappy_fs::DiscoveredTemplate;

use crate::cli::ListArgs;
use crate::commands::helpers::build_templates_catalogue;
use crate::diagnostics::print_warning;

/// List command stub.
pub fn list(args: &ListArgs) -> ExitCode {
    let Some(catalogue) = build_templates_catalogue(args.templates_dir.clone()) else {
        return ExitCode::FAILURE;
    };

    let templates = catalogue
        .templates()
        .iter()
        .filter(|template| match_language_filter(template, args.language.as_deref()))
        .collect::<Vec<_>>();

    if templates.is_empty() {
        print_warning("no templates found");
        return ExitCode::SUCCESS;
    }

    print_template_list(&templates);

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
