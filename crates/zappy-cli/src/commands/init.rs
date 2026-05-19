use std::process::ExitCode;

use zappy_fs::init::InitTemplateInput;

use crate::cli::InitArgs;
use crate::commands::helpers::create_template_skeleton;

/// Init command stub.
pub fn init_template(args: &InitArgs) -> ExitCode {
    tracing::info!(
        output = %args.output.display(),
        template = ?args.template,
        force = args.force,
        "initializing template command"
    );

    let input = InitTemplateInput {
        output_dir: args.output.clone(),
        template_id: args.template.clone(),
        name: args.name.clone(),
        description: args.description.clone(),
        force: args.force,
    };

    if create_template_skeleton(&input).is_err() {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
