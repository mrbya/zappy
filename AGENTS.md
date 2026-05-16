# Zappy Agent Notes

## Workspace Shape
- Trust `justfile`, crate code, and tests over prose: `docs/manifest.md` and `docs/templates.md` are empty, and `docs/dev/implementation_plan.md` is aspirational.
- The root crate is only the installable facade: `src/main.rs` calls `zappy::run()`, and `src/lib.rs` re-exports `zappy_cli::cli::run`.
- `crates/zappy-cli` owns Clap parsing, command dispatch, built-in variable injection, and terminal output.
- `crates/zappy-core` owns manifest parsing, variable resolution, rendering, and generation plan model types; it intentionally has no filesystem writes or process execution.
- `crates/zappy-fs` owns template discovery, deterministic traversal, template skeleton init, plan building, and materialization.
- `crates/zappy-hooks` owns hook process execution; `shell = true` hooks are currently rejected as unsupported.
- `crates/zappy-templates` embeds bundled starter templates and extracts them to a cache dir for normal `zappy-fs` discovery.
- `crates/zappy-adapters` is only a reserved ecosystem-adapter crate today; keep generic filesystem, hook, and rendering behavior out of it.
- Current CLI status: `list`, `info`, `new`, `validate`, `init`, and `create --empty` are implemented; non-empty `create` still just prints a stub.

## Commands That Matter
- Use `just` recipes first; the `justfile` has `set dotenv-load := true`, so recipes automatically load `.env`.
- First-time setup is `cargo install just` then `just init`. The init recipe installs nightly, `cargo-binstall`, `cargo-nextest`, `cargo-llvm-cov`, `cargo-udeps`, `cargo-audit`, global `markdown-toc`, and `pre-commit`.
- Formatting is `just fmt`; check-only formatting is `just fmt -- --check`. This uses `cargo +nightly fmt`, so formatting needs nightly even though `rust-toolchain.toml` pins workspace builds to stable.
- Linting is `just check -- -D warnings`, which runs `cargo clippy --tests --examples --all-targets --all-features --workspace`.
- Main tests are `just test` (`cargo nextest run --all-features --workspace`) and `just doctest` (`cargo test --workspace --doc`).
- There is no focused `just` recipe for one crate; use `cargo nextest run -p <crate> --all-features <filter>` or `cargo test -p zappy --test integration_tests <test>` for focused verification.
- `just test-template <id>` runs `just run -- validate -i crates/zappy-templates/templates -t <id>`; `just test-templates` validates all bundled templates.
- The CI gate is `just ci`: format check, `thorough-check` (`fmt --check`, clippy `-D warnings`, nightly `udeps`, audit), doctests, and CI coverage.
- `just pre-commit` is the mutating local gate: it runs `just fmt`, then the same checks plus local coverage.

## Hooks And Commit Expectations
- `.pre-commit-config.yaml` runs `just ci` for `*.rs`, `*.toml`, and `justfile` changes outside `crates/zappy-templates/templates/`.
- Changes under `crates/zappy-templates/templates/` trigger `just test-templates` instead of the main Rust gate.
- README edits trigger `just index`, which rewrites the README TOC via `markdown-toc -i README.md`.
- Commit messages are checked against Conventional Commits by the pre-commit `commit-msg` hook.
- `cargo audit` warnings are treated as failures via `.cargo/audit.toml`.

## Template Discovery And Fixtures
- Template manifests are always named `zappy.toml`.
- `template.source.root` defaults to `template` and must stay relative without `..`.
- If `--templates-dir` is provided, discovery uses only that path, disables bundled template discovery, and treats a missing explicit directory as an error.
- Without `--templates-dir`, search order is:
  1. `ZAPPY_TEMPLATES_DIR`
  2. `$ZAPPY_CONFIG/templates`
  3. the platform config dir for app `zappy`
  4. `templates/` next to the current executable
  5. `./templates`
  6. bundled templates extracted from `crates/zappy-templates`
- `ZAPPY_TEMPLATES_DIR` is a required path when set; `$ZAPPY_CONFIG/templates` and other implicit paths may be missing.
- `zappy --clear ...` clears the bundled-template cache before command execution.
- Duplicate template IDs are first-hit wins; later matches are kept as `shadowed`.
- A search path may be either a directory of templates or a single template directory that directly contains `zappy.toml`.
- Integration tests rely on `tests/fixtures/templates/test_template`; bundled template tests use `crates/zappy-templates/templates` explicitly.

## CLI And Template Gotchas
- `new` does not prompt yet: `--non-interactive` is parsed but unused, and missing required vars must come from `--var`, template defaults, or built-ins.
- CLI built-ins are `project_name`, `user`, `email`, `date`, `day`, `month`, and `year`; `project_name` comes from `--name`, while `user`/`email` prefer git config and fall back to TODO strings.
- Built-in placeholders are engine-defined as `__ZAPPY_<NAME>__` plus transform suffixes such as `_SNAKE`, `_KEBAB`, and `_PASCAL`; do not declare them as normal manifest vars unless you want override validation behavior.
- `validate` requires a `[validation]` block, generates into a temp dir with `force = true`, and `--no-hooks` only skips generation hooks, not validation setup/steps/teardown.
- Non-empty `create` is not implemented; use `init` or `create --empty` when testing skeleton generation.

## Rustdoc And Lints
- The root crate plus `zappy-cli`, `zappy-core`, `zappy-fs`, and `zappy-templates` enable strict `missing_docs` and `clippy::missing_docs_in_private_items`; `zappy-hooks` and `zappy-adapters` currently do not mirror those crate-level lints.
- Do not add new `#[allow(...)]` attributes just to silence clippy; fix the warning unless an existing local test pattern clearly applies.
- When adding or rewriting docs, match `docs/dev/rustdoc_style.md` rather than the stale README link to `dev/docs/rustdoc_style.md`.

<!-- BACKLOG.MD MCP GUIDELINES START -->

<CRITICAL_INSTRUCTION>

## BACKLOG WORKFLOW INSTRUCTIONS

This project uses Backlog.md MCP for all task and project management activities.

**CRITICAL GUIDANCE**

- If your client supports MCP resources, read `backlog://workflow/overview` to understand when and how to use Backlog for this project.
- If your client only supports tools or the above request fails, call `backlog.get_workflow_overview()` tool to load the tool-oriented overview (it lists the matching guide tools).

- **First time working here?** Read the overview resource IMMEDIATELY to learn the workflow
- **Already familiar?** You should have the overview cached ("## Backlog.md Overview (MCP)")
- **When to read it**: BEFORE creating tasks, or when you're unsure whether to track work

These guides cover:
- Decision framework for when to create tasks
- Search-first workflow to avoid duplicates
- Links to detailed guides for task creation, execution, and finalization
- MCP tools reference

You MUST read the overview resource to understand the complete workflow. The information is NOT summarized here.

</CRITICAL_INSTRUCTION>

<!-- BACKLOG.MD MCP GUIDELINES END -->
