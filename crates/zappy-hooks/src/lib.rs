//! Hook and validation command execution for Zappy.
//!
//! This crate will eventually own pre/post generation hooks, validation setup,
//! validation steps, teardown commands, working directories, environments, and
//! stdout/stderr capture.

/// Current crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
