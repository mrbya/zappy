use std::process::ExitCode;

use crate::cli::CreateArgs;

/// Create command stub.
pub fn create(args: &CreateArgs) -> ExitCode {
    println!("zappy create: stub");
    println!("{args:#?}");
    ExitCode::SUCCESS
}
