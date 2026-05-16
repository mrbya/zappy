//! Hook and validation command execution for Zappy.
//!
//! This crate will eventually own pre/post generation hooks, validation setup,
//! validation steps, teardown commands, working directories, environments, and
//! stdout/stderr capture.

/// Hook errors.
pub mod error;
/// Generation hook execution.
pub mod hooks;

// Re-exports.
pub use error::{HooksError, HooksResult};
pub use hooks::{ExecuteHooksInput, HookExecutionSummary, HookPhase, execute_hooks};

// Tests
#[cfg(test)]
mod tests;
