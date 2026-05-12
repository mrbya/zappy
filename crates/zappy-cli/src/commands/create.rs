use std::process::ExitCode;

use zappy_fs::InitTemplateInput;

use crate::cli::CreateArgs;
use crate::commands::helpers::create_template_skeleton;

/// Create command stub.
pub fn create(args: &CreateArgs) -> ExitCode {
    if args.empty {
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

        return ExitCode::SUCCESS;
    }

    println!("zappy create: stub");
    println!("{args:#?}");
    ExitCode::SUCCESS
}
