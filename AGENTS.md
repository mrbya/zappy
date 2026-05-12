# Zappy Agent Notes

## Workspace Shape
- `README.md` is still mostly `TBD`; trust `justfile`, crate code, and tests over top-level prose when they disagree.
- The root crate is only the installable facade/binary: `src/main.rs` calls `zappy::run()`, and `src/lib.rs` re-exports `zappy_cli::cli::run`.
- Real boundaries are by crate:
  - `crates/zappy-cli`: Clap parsing and command dispatch.
  - `crates/zappy-core`: manifest parsing, variable resolution, rendering, and generation plan types; no filesystem or process execution.
  - `crates/zappy-fs`: template discovery, traversal, plan building, and materialization.
  - `crates/zappy-hooks`: hook execution.
  - `crates/zappy-adapters`: present in the workspace but currently empty.
- Current implementation status matters: `list`, `info`, and `new` are implemented; `validate`, `init`, and `create` in `crates/zappy-cli/src/commands/` are still stubs that print args and return success.

## Commands That Matter
- Use `just` recipes first; the `justfile` has `set dotenv-load := true`, so recipes automatically load `.env`.
- First-time setup is `just init`. It installs nightly plus the extra tools this repo expects: `cargo-nextest`, `cargo-llvm-cov`, `cargo-udeps`, `cargo-audit`, global `markdown-toc`, and `pre-commit`.
- Formatting is `just fmt` and check-only formatting is `just fmt -- --check`. This uses `cargo +nightly fmt`, so formatting needs nightly even though `rust-toolchain.toml` pins the workspace to stable.
- Linting is `just check -- -D warnings`, which runs `cargo clippy --tests --examples --all-targets --all-features --workspace`.
- Tests are split:
  - `just test`: `cargo nextest run --all-features --workspace`
  - `just doctest`: `cargo test --workspace --doc`
- There is no focused `just` recipe for one crate; use `cargo nextest run -p <crate> --all-features` when you only need package-level verification.
- The full non-mutating gate is `just ci`, in this order: `fmt --check -> clippy -D warnings -> udeps -> audit -> doctest -> llvm-cov nextest`.
- `just pre-commit` is the mutating local gate: it runs `just fmt` first, then the same checks plus full coverage.

## Hooks And Commit Expectations
- `.pre-commit-config.yaml` runs `just ci` for `*.rs`, `*.toml`, and `justfile` changes.
- README edits trigger `just index`, which rewrites the README TOC via `markdown-toc -i README.md`.
- Commit messages are checked against Conventional Commits by the pre-commit `commit-msg` hook.
- `cargo audit` warnings are treated as failures via `.cargo/audit.toml`.

## Template Discovery And Fixtures
- Template manifests are always named `zappy.toml`.
- `template.source.root` defaults to `template` and must stay relative without `..`.
- If `--templates-dir` is provided, discovery uses only that path, and a missing explicit directory is an error.
- Without `--templates-dir`, search order is:
  1. `ZAPPY_TEMPLATES_DIR`
  2. `$ZAPPY_CONFIG/templates`
  3. the platform config dir for app `zappy`
  4. `templates/` next to the current executable
  5. `./templates`
- Duplicate template IDs are first-hit wins; later matches are kept as `shadowed`.
- A search path may be either a directory of templates or a single template directory that directly contains `zappy.toml`.
- Integration tests rely on `tests/fixtures/templates/test_template`; many `new` tests pass `--templates-dir tests/fixtures/templates` explicitly.

## Rustdoc And Lints
- Crate roots enable `missing_docs` and `clippy::missing_docs_in_private_items`, so undocumented private items will fail normal lint flow.
- Do not add new `#[allow(...)]` attributes to silence lint warnings; fix the underlying issue instead.
- When adding or rewriting docs, match `docs/rustdoc_style.md` rather than ad-libbing a new style.

## Easy To Guess Wrong
- `new` only injects the built-in `project_name` variable today (`crates/zappy-cli/src/commands.rs`); do not assume the other reserved built-ins in `zappy-core/src/builtins.rs` are wired into CLI generation yet.
