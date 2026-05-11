//! Zappy CLI commands.
//!
//! Implements zappy's CLI command handlers and re-exports them for the cli runner.

/// Creates a template from an existing project.
pub mod create;
/// Displays detailed info about a template.
pub mod info;
/// Initializes an empty template skeleton.
pub mod init;
/// Lists available templates.
pub mod list;
/// Generates a new project from a template.
pub mod new;
/// Validates that a template generates a working project.
pub mod validate;

// Command re-exports for cli runner.
pub use create::create;
pub use info::info;
pub use init::init_template;
pub use list::list;
pub use new::new;
pub use validate::validate;
