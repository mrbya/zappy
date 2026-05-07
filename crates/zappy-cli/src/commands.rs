use std::process::ExitCode;

use zappy_fs::{DiscoveredTemplate, DiscoveryConfig, discover_templates};

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

/// New command stub.
pub fn new(args: &NewArgs) -> ExitCode {
    println!("zappy new: stub");
    println!("{args:#?}");
    ExitCode::SUCCESS
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
    println!("{:-<24} {:-<16} {:-<1}", "", "", "");

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
