use std::process::ExitCode;

use crate::cli::GreetArgs;

/// `__ZAPPY_PROJECT_NAME__` `greet` command handler.
pub fn greet(args: GreetArgs) -> ExitCode {
    println!("Hello from __ZAPPY_PROJECT_NAME__!");
    if let Some(name) = args.name {
        println!("Hi, {name}");
    }

    ExitCode::SUCCESS
}
