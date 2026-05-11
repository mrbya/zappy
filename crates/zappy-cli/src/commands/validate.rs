use std::process::ExitCode;

use crate::cli::ValidateArgs;

/// Validate command stub.
pub fn validate(args: &ValidateArgs) -> ExitCode {
    println!("zappy validate: stub");
    println!("{args:#?}");
    ExitCode::SUCCESS
}
