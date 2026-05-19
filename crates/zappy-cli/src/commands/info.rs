use std::process::ExitCode;

use zappy_fs::DiscoveredTemplate;

use crate::cli::InfoArgs;
use crate::commands::helpers::resolve_template;

/// Info command stub.
pub fn info(args: &InfoArgs) -> ExitCode {
    let Some(template) = resolve_template(args.templates_dir.clone(), &args.template) else {
        return ExitCode::FAILURE;
    };

    print_template_info(&template);

    ExitCode::SUCCESS
}

/// Prints detailed information about a template.
fn print_template_info(template: &DiscoveredTemplate) {
    let metadata = &template.manifest.template;

    println!("ID:           {}", metadata.id.as_str());
    println!("Name:         {}", metadata.name);

    if let Some(description) = metadata.description.as_deref() {
        println!("Description:  {description}");
    }

    if let Some(language) = metadata.language.as_deref() {
        println!("Language:     {language}");
    }

    if let Some(version) = metadata.version.as_deref() {
        println!("Version:      {version}");
    }

    if !metadata.authors.is_empty() {
        println!("Authors:      {}", metadata.authors.join(", "));
    }

    println!("Source root:  {}", metadata.source.root);
    println!("Template dir: {}", template.template_dir.display());
    println!("Manifest:     {}", template.manifest_path.display());
}
