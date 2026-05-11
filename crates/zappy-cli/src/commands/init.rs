use std::process::ExitCode;

use crate::cli::InitArgs;

/// Init command stub.
pub fn init_template(args: &InitArgs) -> ExitCode {
    println!("zappy init-template: stub");
    println!("{args:#?}");
    ExitCode::SUCCESS
}
