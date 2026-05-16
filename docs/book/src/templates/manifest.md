# Manifest Basics

Every template manifest is named `zappy.toml`. This page explains the sections
template authors usually need first. For exhaustive field details, see the
[Manifest Reference](../reference/manifest.md).

## Minimal Manifest

The smallest useful manifest has template metadata:

```toml
[template]
id = "my-template"
name = "My Template"
```

With only this metadata, Zappy uses the default source root `template` and has no
custom variables, path rules, hooks, or validation.

## Template Metadata

`[template]` identifies the template for discovery and CLI lookup:

```toml
[template]
id = "rust-cli"
name = "Rust CLI app"
description = "Rust-based CLI application using clap."
language = "rust"
version = "0.1.0"
authors = ["Example Team"]

[template.source]
root = "template"
```

Important rules:

- `id` and `name` are required.
- `id` must start with an ASCII letter or `_`, and may contain ASCII letters,
  digits, `_`, or `-`.
- `template.source.root` defaults to `template`.
- Source roots must be relative and must not contain `..` path components.

## Variables

Variables are declared under `[variables]` using inline tables or standard TOML
subtables:

```toml
[variables]
description = {
    required = true,
    default = "TODO: add description",
    placeholders = { raw = "__DESCRIPTION__" }
}

use_ci = { default = false }
```

Variables can provide defaults, choices, conditional requirements, conflict
rules, transforms, and placeholder mappings. Zappy resolves variables before it
builds the generation plan.

## Paths

`[paths]` controls what the source walker does with files:

```toml
[paths]
exclude = [".git", "target", "node_modules"]
binary_extensions = ["png", "jpg", "zip"]
binary_files = ["Cargo.lock"]
```

- Excluded paths are skipped.
- Binary files are copied without placeholder rendering.
- Non-binary files are read as UTF-8 and rendered. If UTF-8 reading fails, the
  file is copied as binary and the plan receives a warning.

## Conditionals

Use `[[conditionals]]` to include paths only when a boolean variable is true:

```toml
[[conditionals]]
path = ".github/workflows/ci.yml"
when = "use_ci"
```

Missing variables and non-boolean variables evaluate as false for conditionals.
The condition path is relative to the source root.

## Hooks

Generation hooks live under `[[hooks.pre_generate]]` and
`[[hooks.post_generate]]`:

```toml
[[hooks.post_generate]]
name = "Format codebase"
command = "cargo"
args = ["fmt"]
optional = true
```

Hooks run in order. They default to the generated project output directory and
can be conditional with `when = "some_boolean_variable"`.

## Validation

Templates can define checks under `[validation]`:

```toml
[validation]
output_dir_name = "validated-my-template"
variables = { description = "Generated validation project" }

[[validation.steps]]
name = "check README exists"
command = "test"
args = ["-f", "README.md"]
```

`zappy validate` generates into a temporary directory, runs validation setup
hooks, validation steps, and validation teardown hooks, then reports success or
failure.
