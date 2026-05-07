use std::process::ExitCode;

use crate::cli::{CreateArgs, InfoArgs, InitArgs, ListArgs, NewArgs, ValidateArgs};

/// List command stub.
pub fn list(args: &ListArgs) -> ExitCode {
    println!("zappy list: stub");
    println!("{args:#?}");
    ExitCode::SUCCESS
}

/// Info command stub.
pub fn info(args: &InfoArgs) -> ExitCode {
    println!("zappy info: stub");
    println!("{args:#?}");
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
