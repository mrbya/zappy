use indexmap::IndexMap;

use crate::transform::apply_transform;
use crate::variables::{VariableMap, VariableValueMap};
use crate::{CoreError, CoreResult, TransformKind, VariableSpec, VariableValue};

/// A namend variable input source.
///
/// Source order matters. Higher-priority sources should appear earlier.
/// See [`resolve_variables`] for precedence info.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableResolutionSource {
    /// Human-readable source name used in diagnostics.
    pub name: &'static str,

    /// Values provided by this source.
    pub values: VariableValueMap,
}

/// Variable resolution input.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VariableResolutionInput {
    /// Built-in values supplied by zappy.
    pub builtins: VariableValueMap,

    /// Highest-priority explicit values passed in via CLI `--var` args.
    pub explicit: VariableValueMap,

    /// Values obtained interactively from the user.
    pub interactive: VariableValueMap,

    /// User-level defaults.
    pub user_defaults: VariableValueMap,
}

/// Fully resolved variables prepared for rendering.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResolvedVariables {
    /// Resolved base variable values.
    pub values: VariableValueMap,

    /// Resolved transformed values.
    pub transformations: IndexMap<String, IndexMap<TransformKind, String>>,

    /// Placeholder-to-rendered-value replacements.
    pub replacements: IndexMap<String, String>,
}

/// Resolves template variables using Zappy's precedence rules.
///
/// Precedence:
/// ```text
/// built-ins
/// > explicit values
/// > interactive values
/// > template defaults
/// > user defaults
/// ```
///
/// # Arguments
/// - `variables`: variable map defined in template manifest,
/// - `input`: variable values captured from input sources.
///
/// # Errors
/// Returns [`CoreError`] if an input source contains unknown variables, a
/// required variable is missing, or a selected value violates choices.
pub fn resolve_variables(
    variables: &VariableMap,
    input: &VariableResolutionInput,
) -> CoreResult<ResolvedVariables> {
    validate_builtin_variables(&input.builtins)?;
    validate_known_variables(variables, "explicit values", &input.explicit)?;
    validate_known_variables(variables, "interactive values", &input.interactive)?;
    validate_known_variables(variables, "user defaults", &input.user_defaults)?;

    let mut resolved_values = VariableValueMap::new();
    let mut transformed_values = IndexMap::new();
    let mut replacements = IndexMap::new();

    for (name, spec) in variables {
        let value = resolve_single_variable(name, spec, input)?;

        let Some(value) = value else {
            continue;
        };

        validate_choices(name, &value, &spec.choices)?;

        let transforms = required_transforms(spec);

        let mut transformed_for_variable = IndexMap::new();

        for transform in transforms {
            let rendered = apply_transform(&value, transform);
            transformed_for_variable.insert(transform, rendered);
        }

        for (transform, placeholder) in &spec.placeholders {
            if let Some(rendered) = transformed_for_variable.get(transform) {
                replacements.insert(placeholder.clone(), rendered.clone());
            }
        }

        resolved_values.insert(name.clone(), value);
        transformed_values.insert(name.clone(), transformed_for_variable);
    }

    inject_builtins(
        &input.builtins,
        &mut resolved_values,
        &mut transformed_values,
        &mut replacements,
    );

    Ok(ResolvedVariables {
        values: resolved_values,
        transformations: transformed_values,
        replacements,
    })
}

/// Resolves a single variable according to source precedence.
fn resolve_single_variable(
    name: &str,
    spec: &VariableSpec,
    input: &VariableResolutionInput,
) -> CoreResult<Option<VariableValue>> {
    if let Some(value) = input.builtins.get(name) {
        return Ok(Some(value.clone()));
    }

    if let Some(value) = input.explicit.get(name) {
        return Ok(Some(value.clone()));
    }

    if let Some(value) = input.interactive.get(name) {
        return Ok(Some(value.clone()));
    }

    if let Some(value) = spec.default.as_ref() {
        return Ok(Some(value.clone()));
    }

    if let Some(value) = input.user_defaults.get(name) {
        return Ok(Some(value.clone()));
    }

    if spec.required {
        return Err(CoreError::MissingRequiredVariable {
            name: String::from(name),
        });
    }

    Ok(None)
}

/// Validates builtin variables.
fn validate_builtin_variables(values: &VariableValueMap) -> CoreResult<()> {
    for name in values.keys() {
        if !crate::builtins::is_builtin_name(name) {
            return Err(CoreError::UnknownBuiltinVariable { name: name.clone() });
        }
    }

    Ok(())
}

/// Injects built-in variables into a variable value map.
fn inject_builtins(
    builtins: &VariableValueMap,
    values: &mut VariableValueMap,
    transformations: &mut IndexMap<String, IndexMap<TransformKind, String>>,
    replacements: &mut IndexMap<String, String>,
) {
    let builtin_transformations = crate::builtins::builtin_transformations(builtins);
    let builtin_replacements = crate::builtins::builtin_replacements(builtins);

    for (name, value) in builtins {
        values.insert(name.clone(), value.clone());
    }

    for (name, transformed) in builtin_transformations {
        transformations.insert(name, transformed);
    }

    for (placeholder, value) in builtin_replacements {
        replacements.insert(placeholder, value);
    }
}

/// Validates that every source key references a variable declared in tempalte manifest.
fn validate_known_variables(
    variables: &VariableMap,
    source: &'static str,
    values: &VariableValueMap,
) -> CoreResult<()> {
    for name in values.keys() {
        if !variables.contains_key(name) {
            return Err(CoreError::UnknownVariable {
                name: name.clone(),
                vsource: source,
            });
        }
    }

    Ok(())
}

/// Validates variable value from manifest choices.
fn validate_choices(
    name: &str,
    value: &VariableValue,
    choices: &[VariableValue],
) -> CoreResult<()> {
    if choices.is_empty() || choices.contains(value) {
        return Ok(());
    }

    let value = value.render();
    let choices = choices
        .iter()
        .map(VariableValue::render)
        .collect::<Vec<_>>()
        .join(", ");

    Err(CoreError::InvalidVariableChoice {
        name: String::from(name),
        value,
        choices,
    })
}

/// Computes transfors that needs to be available for a variable.
fn required_transforms(spec: &VariableSpec) -> Vec<TransformKind> {
    let mut transforms = Vec::new();

    for transform in &spec.transforms {
        if !transforms.contains(transform) {
            transforms.push(*transform);
        }
    }

    for transform in spec.placeholders.keys() {
        if !transforms.contains(transform) {
            transforms.push(*transform);
        }
    }

    if transforms.is_empty() {
        transforms.push(TransformKind::Raw);
    }

    transforms
}
