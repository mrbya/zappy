# Crate Map

Use this page to decide where a change belongs. It describes current crate
ownership and key files in the workspace.

## Root Crate: `zappy`

Path: `./`

Purpose:

- Owns the installable `zappy` package and binary.
- Delegates runtime behavior to `zappy-cli`.

Key files:

| File | Purpose |
| --- | --- |
| `src/main.rs` | Binary entry point; calls `zappy::run()`. |
| `src/lib.rs` | Public facade; re-exports `zappy_cli::cli::run`. |
| `Cargo.toml` | Workspace membership, shared package metadata, shared dependencies, root package config. |

Change here when packaging, root binary entry, or workspace-level metadata needs
to change.

## `zappy-cli`

Path: `crates/zappy-cli`

Purpose:

- Owns Clap definitions and subcommand dispatch.
- Owns terminal-facing command output.
- Injects CLI built-in variables.
- Coordinates `zappy-core`, `zappy-fs`, `zappy-hooks`, and `zappy-templates`.

Key files:

| File | Purpose |
| --- | --- |
| `src/cli.rs` | Top-level CLI parser, global options, subcommands, aliases, and dispatch. |
| `src/commands/list.rs` | `zappy list`. |
| `src/commands/info.rs` | `zappy info`. |
| `src/commands/new.rs` | `zappy new`, dry-run output, and generation orchestration. |
| `src/commands/validate.rs` | `zappy validate` orchestration. |
| `src/commands/init.rs` | `zappy init`. |
| `src/commands/create.rs` | `zappy create`; only `--empty` is implemented today. |
| `src/commands/helpers.rs` | Shared discovery, generation, hook, skeleton, and built-in helpers. |

Change here for CLI syntax, help text, command UX, command status handling,
terminal output, and built-in value collection.

## `zappy-core`

Path: `crates/zappy-core`

Purpose:

- Owns manifest parsing and validation.
- Owns domain models for templates, variables, hooks, validation, and generation
  plans.
- Owns variable resolution, transforms, built-in placeholders, and rendering.
- Avoids filesystem writes and process execution.

Key files:

| File | Purpose |
| --- | --- |
| `src/manifest.rs` | Top-level manifest loading, parsing, validation, and path config. |
| `src/template.rs` | Template ID, metadata, source root, and path validation. |
| `src/variables.rs` | Variable specs, CLI override parsing, value types, transform names. |
| `src/resolution.rs` | Variable resolution, required/conditional/conflict checks, replacement construction. |
| `src/builtins.rs` | Built-in names, transforms, and placeholder construction. |
| `src/transform.rs` | String transform behavior. |
| `src/render.rs` | Literal placeholder replacement in text and relative paths. |
| `src/condition.rs` | Conditional path model and boolean condition evaluation. |
| `src/hooks.rs` | Manifest hook model. |
| `src/validation.rs` | Manifest validation model. |
| `src/plan.rs` | Generation plan, operations, warnings, and skip reasons. |
| `src/error.rs` | Core error types. |
| `src/tests.rs` | Core unit tests for manifest parsing, variables, transforms, and rendering. |

Change here for manifest schema, variable semantics, transform behavior,
placeholder rendering rules, and generation plan model changes.

## `zappy-fs`

Path: `crates/zappy-fs`

Purpose:

- Owns template discovery and search path resolution.
- Owns deterministic source tree traversal.
- Owns template skeleton initialization.
- Owns generation plan building and materialization.
- Owns text/binary file classification.

Key files:

| File | Purpose |
| --- | --- |
| `src/discover.rs` | Discovery config, search paths, catalogue, duplicate shadowing, and manifest loading. |
| `src/walk.rs` | Deterministic source root traversal. |
| `src/classify.rs` | Binary file and extension classification. |
| `src/plan.rs` | Build generation plans from source entries and resolved variables. |
| `src/materialize.rs` | Execute generation plans with safe writes and binary copies. |
| `src/init.rs` | Create empty template skeletons. |
| `src/error.rs` | Filesystem error types. |
| `src/tests.rs` | Discovery and materialization tests. |

Change here for discovery behavior, search path rules, dry-run planning,
filesystem writes, skeleton layout, excludes, conditionals, symlink handling, or
binary classification.

## `zappy-hooks`

Path: `crates/zappy-hooks`

Purpose:

- Owns external command execution for hooks.
- Applies hook conditions.
- Renders hook commands, arguments, working directories, and environment values.
- Captures hook execution summaries and errors.

Key files:

| File | Purpose |
| --- | --- |
| `src/hooks.rs` | Hook phases, execution input, execution loop, command spawning, optional failure handling. |
| `src/error.rs` | Hook execution error types. |
| `src/tests.rs` | Hook execution tests. |

Change here for process execution, hook failure semantics, hook environment
behavior, hook working directory behavior, and future shell support. `shell =
true` is currently rejected as unsupported.

## `zappy-templates`

Path: `crates/zappy-templates`

Purpose:

- Embeds bundled starter templates at compile time.
- Extracts bundled templates to a cache directory.
- Clears the bundled template cache for `zappy --clear`.

Key files:

| File | Purpose |
| --- | --- |
| `src/cache.rs` | Bundled template extraction, cache marker, cache clearing, cache path resolution. |
| `src/error.rs` | Bundled template cache error types. |
| `src/tests.rs` | Cache extraction and clearing tests. |
| `build.rs` | Computes the bundled template hash used in the cache marker. |
| `templates/` | Bundled template source manifests and source trees. |

Change here for bundled template cache behavior or bundled template contents.
Changes under `templates/` should be validated with `just test-templates`.

## `zappy-adapters`

Path: `crates/zappy-adapters`

Purpose:

- Reserved for future ecosystem-specific adapters.
- Intended for behavior that knows about specific ecosystems such as Git, Cargo,
  uv, CMake, Zephyr, npm, Tauri, or future recipe mode.

Current state:

- Exposes only a `VERSION` constant.
- Does not own generic filesystem, hook, rendering, or manifest behavior.

Change here only for ecosystem-specific behavior that does not belong in the
generic crates.

## Tests And Fixtures

| Path | Purpose |
| --- | --- |
| `tests/integration_tests.rs` | End-to-end CLI tests using `assert_cmd`. |
| `tests/fixtures/templates/test_template` | Fixture template used by integration tests. |
| `crates/*/src/tests.rs` | Crate-local unit tests. |
| `crates/zappy-templates/templates` | Bundled templates used by `just test-templates`. |

Use integration tests for CLI-visible behavior. Use crate-local tests for core
domain logic, discovery/materialization behavior, hook execution, and cache
behavior.
