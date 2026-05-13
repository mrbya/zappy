//! `__ZAPPY_PROJECT_NAME__` CLI commands.
//!
//! Implements `__ZAPPY_PROJECT_NAME_KEBAB__`'s command handlers and re-exports them for the shell
//! crate.

/// Prints a greeting.
pub mod greet;

// Command re-exports for cli parser,
pub use greet::greet;
