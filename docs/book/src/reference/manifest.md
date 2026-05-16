# Manifest Reference

This reference describes the implemented `zappy.toml` schema. The manifest is
parsed by `zappy-core` and loaded from files named `zappy.toml`.

## Top-Level Tables

| Table | Required? | Type | Purpose |
| --- | --- | --- | --- |
| `[template]` | Yes | Table | Template metadata and source-root configuration. |
| `[variables]` | No | Table of variable specs | Declared template variables. |
| `[paths]` | No | Table | Excludes and binary file classification. |
| `[[conditionals]]` | No | Array of tables | Conditional source paths. |
| `[hooks]` | No | Table | Generation hooks. |
| `[validation]` | No | Table | Template validation configuration. |

## `[template]`

| Field | Required? | Default | Validation |
| --- | --- | --- | --- |
| `id` | Yes | None | Must start with ASCII letter or `_`; may contain ASCII letters, digits, `_`, or `-`. |
| `name` | Yes | None | Must not be empty. |
| `description` | No | None | Free-form string. |
| `language` | No | None | Free-form string used by `zappy list --language`. |
| `version` | No | None | Free-form string. |
| `authors` | No | `[]` | String array. |

Example:

```toml
[template]
id = "rust-cli"
name = "Rust CLI app"
description = "Rust-based CLI application using clap."
language = "rust"
version = "0.1.0"
authors = ["Example Team"]
```

## `[template.source]`

| Field | Required? | Default | Validation |
| --- | --- | --- | --- |
| `root` | No | `template` | Must be relative, non-empty, and must not contain `..`. |

The source root is resolved relative to the template directory containing
`zappy.toml`.

## `[variables]`

`[variables]` is a map from variable name to variable specification.

Variable names must start with an ASCII letter or `_`, contain only ASCII
letters, digits, and `_`, and must not be one of the reserved built-in names:
`project_name`, `user`, `email`, `date`, `day`, `month`, or `year`.

| Field | Required? | Default | Validation / Behavior |
| --- | --- | --- | --- |
| `prompt` | No | None | Must not be empty when provided. Parsed as metadata; interactive prompting is not implemented yet. |
| `default` | No | None | String, boolean, or integer. |
| `required` | No | `false` | Missing value fails resolution. If no default is provided, a prompt is required in the manifest. |
| `required_when` | No | None | Name of another declared variable. Cannot reference itself. Required only when that variable is boolean `true`. |
| `conflicts_with` | No | `[]` | Array of other declared variable names. Cannot reference itself. Fails when both variables are boolean `true`. |
| `choices` | No | `[]` | Allowed string, boolean, or integer values. Empty means unrestricted. |
| `validation_regex` | No | None | Parsed but not currently enforced. |
| `transforms` | No | `[]` | Array of transform names. |
| `placeholders` | No | `{}` | Map from transform name to non-empty literal placeholder string. |

Supported transform names are `raw`, `kebab`, `snake`, `pascal`, `camel`,
`screaming_snake`, `upper`, and `lower`.

Example:

```toml
[variables]
description = {
    required = true,
    default = "TODO: add description",
    placeholders = { raw = "__DESCRIPTION__" }
}

use_gitlab_ci = { default = false }

gitlab_image_registry = {
    prompt = "Gitlab image registry",
    required_when = "use_gitlab_ci",
    placeholders = { raw = "__GITLAB_IMAGE_REGISTRY__" }
}
```

## `[paths]`

| Field | Required? | Default | Behavior |
| --- | --- | --- | --- |
| `exclude` | No | `[]` | Relative paths or path components to skip. Empty string entries are invalid. |
| `binary_extensions` | No | `[]` | Extensions to copy as binary. Entries may include or omit a leading `.`. Empty string entries are invalid. |
| `binary_files` | No | `[]` | File names or path suffixes to copy as binary. Empty string entries are invalid. |

`exclude` matches when a source-relative path is exactly the exclude value,
starts with `<exclude>/`, or contains a path component equal to the exclude
value.

`binary_files` matches either a file name or a path suffix. `binary_extensions`
matches by extension after stripping an optional leading `.` from the configured
entry.

## `[[conditionals]]`

| Field | Required? | Default | Validation / Behavior |
| --- | --- | --- | --- |
| `path` | Yes | None | Relative source-root path; must not be empty, absolute, or contain `..`. |
| `when` | Yes | None | Name of a variable to evaluate; must not be empty. |

A conditional path is included only when `when` resolves to boolean `true`.
Missing variables and non-boolean values evaluate as false.

Example:

```toml
[[conditionals]]
path = ".github/workflows/ci.yml"
when = "use_github_actions"
```

## Hook Specs

Hook specs are used by `hooks.pre_generate`, `hooks.post_generate`,
`validation.setup`, `validation.steps`, and `validation.teardown`.

| Field | Required? | Default | Validation / Behavior |
| --- | --- | --- | --- |
| `name` | No | Command string | Human-readable name used in diagnostics. |
| `command` | Yes | None | Must not be empty. Rendered before execution. |
| `args` | No | `[]` | Rendered before execution. |
| `working_dir` | No | Output directory | Relative to output directory; must not be empty, absolute, or contain `..`. Rendered before execution. |
| `env` | No | `{}` | Environment values are rendered before execution. |
| `when` | No | Always run | Hook runs only when this variable resolves to boolean `true`. |
| `optional` | No | `false` | Optional hook failures become warnings. Required hook failures fail the command. |
| `shell` | No | `false` | Parsed, but `true` is rejected by the hook runner. |

## `[hooks]`

| Field | Required? | Default | Behavior |
| --- | --- | --- | --- |
| `pre_generate` | No | `[]` | Hooks run after creating the output directory and before writing files. |
| `post_generate` | No | `[]` | Hooks run after files are written. |

Example:

```toml
[[hooks.post_generate]]
name = "Format codebase"
command = "cargo"
args = ["fmt"]
optional = true
```

`zappy new --no-hooks` skips generation hooks.

## `[validation]`

| Field | Required? | Default | Validation / Behavior |
| --- | --- | --- | --- |
| `output_dir_name` | No | `zappy-validation-output` in the CLI | Must not be empty when provided. |
| `variables` | No | `{}` | Keys must reference declared variables. Values are used as explicit validation values. |
| `setup` | No | `[]` | Hook specs run before validation steps. |
| `steps` | No | `[]` | Hook specs run after setup succeeds. |
| `teardown` | No | `[]` | Hook specs run after setup/steps, even when earlier validation failed. |

Example:

```toml
[validation]
output_dir_name = "validated-rust-cli"
variables = {
    description = "Generated validation project",
}

[[validation.steps]]
name = "check README exists"
command = "test"
args = ["-f", "README.md"]
```

`zappy validate --no-hooks` skips generation hooks only. Validation setup,
steps, and teardown still run.
