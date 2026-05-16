//! Core domain model and deterministic template engine logic.
//!
//! `zappy-core` owns manifest parsing, validated template metadata,
//! variable definitions, condition models, hook models, and validation models.
//! It intentionally does not perform CLI prompting, filesystem discovery,
//! filesystem writes, or process execution.

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
    clippy::separated_literal_suffix,
    clippy::shadow_unrelated,
    clippy::str_to_string,
    clippy::string_add,
    clippy::implicit_clone,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::verbose_file_reads
)]

/// Builtin variables.
pub mod builtins;
/// Condition models.
pub mod condition;
/// Zappy core error types.
pub mod error;
/// Hook models.
pub mod hooks;
/// Template manifest parsing.
pub mod manifest;
/// Generation plan model.
pub mod plan;
/// Placeholder replacement in strings/paths.
pub mod render;
/// Variable resolution.
pub mod resolution;
/// Template metadata parsing.
pub mod template;
/// Variable transforms.
pub mod transform;
/// Validation model.
pub mod validation;
/// Template variable definitions.
pub mod variables;

// Re-exports.
pub use error::{CoreError, CoreResult};
pub use manifest::Manifest;
pub use plan::{GenerationPlan, PlanOperation, PlanWarning, SkipReason};
pub use resolution::{
    ResolvedVariables, VariableResolutionInput, VariableResolutionSource, resolve_variables,
};
pub use template::{SourceConfig, TemplateId, TemplateMetadata};
pub use transform::apply_transform;
pub use variables::{
    TransformKind, VariableSpec, VariableValue, VariableValueMap, parse_variable_overrides,
};

// Tests.
#[cfg(test)]
mod tests;
