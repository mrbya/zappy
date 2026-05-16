# Architecture

Zappy is a Rust workspace split into small crates with clear ownership
boundaries. The root crate produces the installable `zappy` binary, while the
internal crates own CLI dispatch, core template semantics, filesystem work,
hook execution, bundled templates, and reserved ecosystem-specific adapters.

## Workspace Layout

```text
zappy/
  Cargo.toml
  src/
    main.rs
    lib.rs
  crates/
    zappy-cli/
    zappy-core/
    zappy-fs/
    zappy-hooks/
    zappy-templates/
    zappy-adapters/
  tests/
    integration_tests.rs
  docs/book/
```

The root crate is deliberately thin:

- `src/main.rs` calls `zappy::run()`.
- `src/lib.rs` re-exports `zappy_cli::cli::run`.

Most behavior lives in the workspace crates.

## Crate Boundaries

| Crate | Responsibility |
| --- | --- |
| `zappy` | Installable facade crate and binary entry point. |
| `zappy-cli` | Clap parsing, command dispatch, built-in variable injection, and terminal output. |
| `zappy-core` | Manifest parsing, validated domain models, variable resolution, placeholder rendering, transforms, and generation plan model types. |
| `zappy-fs` | Template discovery, deterministic source traversal, template skeleton init, generation plan building, and materialization. |
| `zappy-hooks` | External process execution for generation and validation hooks. |
| `zappy-templates` | Bundled starter template embedding and cache extraction. |
| `zappy-adapters` | Reserved for future ecosystem-specific behavior. |

Generic filesystem behavior belongs in `zappy-fs`. Generic hook behavior belongs
in `zappy-hooks`. Generic rendering and manifest semantics belong in
`zappy-core`. Do not put those concerns into `zappy-adapters`.

## Command Flow

The CLI flow for template commands is:

1. `zappy` starts in `src/main.rs` and delegates to `zappy_cli::cli::run`.
2. `zappy-cli` parses arguments with Clap in `crates/zappy-cli/src/cli.rs`.
3. `zappy-cli` dispatches to command handlers under
   `crates/zappy-cli/src/commands/`.
4. Commands build discovery configuration and resolve templates through
   `zappy-fs`.
5. Commands resolve variables through `zappy-core`.
6. Generation commands build plans through `zappy-fs`.
7. Dry-run commands print the plan and stop.
8. Real generation materializes the plan through `zappy-fs` and runs hooks
   through `zappy-hooks`.

`zappy-templates` participates when bundled templates are needed. The CLI asks
it to extract bundled templates to a cache directory, then passes that directory
to normal `zappy-fs` discovery.

## Generation Data Flow

For `zappy new`, the current flow is:

1. Parse CLI variable overrides with `zappy_core::parse_variable_overrides`.
2. Discover and select a template by `template.id`.
3. Construct CLI built-ins: `project_name`, `user`, `email`, `date`, `day`,
   `month`, and `year`.
4. Resolve manifest variables with `zappy_core::resolve_variables`.
5. Choose the output directory from `--output` or the project name.
6. Build a `zappy_core::GenerationPlan` with `zappy_fs::build_generation_plan`.
7. Print the plan for `--dry-run`, or execute generation for normal runs.

The plan builder walks the template source root deterministically, skips
excluded paths, skips false conditionals, skips symlinks, renders destination
paths, classifies binary files, renders UTF-8 text files, and records warnings.

## Validation Data Flow

For `zappy validate`, the current flow is:

1. Discover and select a template.
2. Require a `[validation]` block.
3. Create a temporary directory.
4. Resolve variables from `validation.variables` and built-ins.
5. Build a generation plan with overwrite behavior enabled.
6. Generate the project into the validation output directory.
7. Run `validation.setup` hooks.
8. Run `validation.steps` hooks when setup succeeds.
9. Run `validation.teardown` hooks even when setup or steps fail.

`--no-hooks` skips generation hooks during validation, but it does not skip
validation setup, steps, or teardown.

## Discovery Data Flow

Template discovery is owned by `zappy-fs`:

1. Resolve search paths from explicit CLI configuration, environment variables,
   platform config, executable-relative paths, current working directory, and
   optional bundled templates.
2. Treat a search path as either one template directory or a directory of child
   template directories.
3. Load each `zappy.toml` with `zappy-core`.
4. Keep the first discovered template for each ID and store later duplicates as
   shadowed.

The CLI disables bundled template discovery when `--templates-dir` is provided.

## Core Design Constraints

`zappy-core` intentionally has no filesystem writes or process execution. It
owns deterministic, testable domain behavior:

- Template metadata and manifest validation.
- Variable definitions and resolution.
- Built-in placeholder construction.
- Placeholder replacement in text and relative paths.
- Transform behavior.
- Hook and validation model types.
- Generation plan model types.

This separation keeps the core logic usable from tests and future frontends
without invoking the terminal, filesystem materialization, or external commands.

## Current Implementation Limits

The development docs should reflect these current limits:

- `list`, `info`, `new`, `validate`, `init`, and `create --empty` are
  implemented.
- `create` without `--empty` currently prints a stub.
- `new --non-interactive` is parsed, but interactive prompting is not
  implemented yet.
- `shell = true` hooks parse but are rejected by `zappy-hooks`.
- `validation_regex` is parsed in variable specs but not enforced.
