//! Filesystem support for Zappy.
//!
//! This crate owns template discovery, deterministic traversal, path safety,
//! text/binary classification, dry-run planning inputs, and safe materialization
//! of generated projects.

#![allow(clippy::module_name_repetitions)]
// clippy WARN level lints
#![warn(
    missing_docs,
    //clippy::cargo,
    clippy::pedantic,
    clippy::nursery,
    clippy::dbg_macro,
    clippy::unwrap_used,
    clippy::integer_division,
    clippy::large_include_file,
    clippy::map_err_ignore,
    clippy::missing_docs_in_private_items,
    clippy::panic,
    clippy::todo,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unreachable
)]
// clippy WARN level lints, that can be upgraded to DENY if preferred
#![warn(
    clippy::float_arithmetic,
    clippy::arithmetic_side_effects,
    clippy::modulo_arithmetic,
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::default_union_representation,
    clippy::deref_by_slicing,
    clippy::empty_drop,
    clippy::empty_structs_with_brackets,
    clippy::exit,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::pattern_type_mismatch,
    clippy::string_slice,
    clippy::try_err
)]
// clippy DENY level lints, they always have a quick fix that should be preferred
#![deny(
    clippy::wildcard_imports,
    clippy::multiple_inherent_impl,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::self_named_module_files,
    clippy::shadow_unrelated,
    clippy::str_to_string,
    clippy::string_add,
    clippy::implicit_clone,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::verbose_file_reads
)]

/// Text/binary classification.
pub mod classify;
/// Template discovery.
pub mod discover;
/// Filesystem error types.
pub mod error;
/// Generation plan materialization.
pub mod materialize;
/// Generation plan build.
pub mod plan;
/// Deterministic source traversal.
pub mod walk;

// Re-exports.
pub use classify::{classify_file_by_path, FileKind};
pub use discover::{
    discover_templates, resolve_template_search_paths, DiscoveredTemplate, DiscoveryConfig,
    TemplateCatalogue, TemplateSearchPath, TemplateSearchPathKind,
};
pub use error::{FsError, FsResult};
pub use materialize::{
    create_directory, materialize_generation_plan, MaterializationOptions, MaterializationSummary,
};
pub use plan::{build_generation_plan, BuildPlanInput};

// Tests.
#[allow(clippy::panic)]
#[cfg(test)]
mod tests;
