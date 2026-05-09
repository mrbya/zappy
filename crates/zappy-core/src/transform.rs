use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutySnakeCase, ToSnakeCase};

use crate::{TransformKind, VariableValue};

/// Applies string transformation to a variable value.
///
/// # Examples
/// ```
/// use zappy_core::{TransformKind, VariableValue};
/// use zappy_core::transform::apply_transform;
///
/// let value = VariableValue::String(String::from("my cool tool"));
///
/// assert_eq!(apply_transform(&value, TransformKind::Kebab), "my-cool-tool");
/// ````
#[must_use]
pub fn apply_transform(value: &VariableValue, transform: TransformKind) -> String {
    let raw = value.render();

    match transform {
        TransformKind::Raw => raw,
        TransformKind::Kebab => raw.to_kebab_case(),
        TransformKind::Snake => raw.to_snake_case(),
        TransformKind::Pascal => raw.to_pascal_case(),
        TransformKind::Camel => raw.to_lower_camel_case(),
        TransformKind::ScreamingSnake => raw.to_shouty_snake_case(),
        TransformKind::Upper => raw.to_uppercase(),
        TransformKind::Lower => raw.to_lowercase(),
    }
}
