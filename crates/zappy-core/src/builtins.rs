/// Reserved variable names owned by Zappy.
/// Project name variable name.
pub const PROJECT_NAME: &str = "project_name";
/// Project user variable name.
pub const USER: &str = "user";
/// Generation date variable name.
pub const DATE: &str = "date";
/// Generation date day variable name.
pub const DAY: &str = "day";
/// Generation date month variable name.
pub const MONTH: &str = "month";
/// Generation date year variable name.
pub const YEAR: &str = "year";

/// Returns true if the name is reserved for a built-in variable.
#[must_use]
pub fn is_builtin_name(name: &str) -> bool {
    matches!(name, PROJECT_NAME | USER | DATE | DAY | MONTH | YEAR)
}
