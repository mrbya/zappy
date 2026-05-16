# Contributing

This page documents the current contributor workflow for the Rust workspace.
Prefer the `justfile`, crate code, and tests as the source of truth when this
page and the implementation disagree.

## Requirements

For normal development, install:

- Rust stable with `rustfmt` and `clippy`.
- Rust 1.85.0 or newer.
- `just`.

The workspace is pinned to stable in `rust-toolchain.toml`, but some checks use
nightly tooling. Formatting is run with `cargo +nightly fmt`, and unused
dependency checks use `cargo +nightly udeps`.

## First-Time Setup

Install `just` if needed:

```bash
cargo install just
```

Then run the bootstrap recipe from the repository root:

```bash
just init
```

The bootstrap installs or checks the tools used by the project gates:

- nightly Rust
- `cargo-binstall`
- `cargo-nextest`
- `cargo-llvm-cov`
- `cargo-udeps`
- `cargo-audit`
- `mdbook`
- global `markdown-toc`
- `pre-commit` hooks

The `justfile` has `set dotenv-load := true`, so recipes automatically load a
local `.env` file when one exists.

## Common Commands

Run these from the repository root.

| Command | Purpose |
| --- | --- |
| `just list` | Show available recipes. |
| `just run -- --help` | Build and run the CLI through Cargo. |
| `just fmt` | Apply Rust formatting with nightly rustfmt. |
| `just fmt -- --check` | Check Rust formatting without modifying files. |
| `just check -- -D warnings` | Run Clippy over tests, examples, all targets, all features, and the whole workspace. |
| `just test` | Run the main test suite with `cargo nextest run --all-features --workspace`. |
| `just doctest` | Run Rust doctests with `cargo test --workspace --doc`. |
| `just book-check` | Build the mdBook with `mdbook build docs/book`. |
| `just ci` | Run the non-mutating CI gate. |
| `just pre-commit` | Run the mutating local pre-commit gate. |

Use `just` recipes first when they exist. They encode the project defaults and
tool flags.

## Focused Checks

There is no dedicated `just` recipe for one crate. Use Cargo directly for
focused runs:

```bash
cargo nextest run -p zappy-core --all-features
cargo nextest run -p zappy-cli --all-features new_dry_run
cargo test -p zappy --test integration_tests list_filters_by_language
```

For bundled template validation:

```bash
just test-template rust-cli
just test-templates
```

`just test-template <id>` runs:

```bash
just run -- validate -i crates/zappy-templates/templates -t <id>
```

## CI Gate

`just ci` runs:

1. `just fmt --check`
2. `just thorough-check`
3. `just doctest`
4. `just test-cov-ci`

`just thorough-check` runs:

1. `just fmt --check`
2. `just check -- -D warnings`
3. `just unused`
4. `just audit`

`cargo audit` warnings are treated as failures through `.cargo/audit.toml`.

## Pre-Commit Hooks

Install hooks with:

```bash
just install-hooks
```

or:

```bash
just pre-commit-install
```

The configured hooks run these project-specific checks:

| Changed Files | Hook Behavior |
| --- | --- |
| `*.rs`, `*.toml`, or `justfile` outside bundled templates | Runs `just ci`. |
| Files under `crates/zappy-templates/templates/` | Runs `just test-templates`. |
| `README.md` | Runs `just index`, which rewrites the README table of contents. |
| Commit messages | Checks Conventional Commits format. |

The hook config also includes standard whitespace, end-of-file, YAML, and large
file checks.

## Commit Messages

Commit messages are checked by the `conventional-commit-check` hook. Use
Conventional Commits-style messages, such as:

```text
docs: document template validation
fix: reject invalid template source roots
test: cover bundled template discovery
```

## Documentation Contributions

Build the book before submitting mdBook changes:

```bash
just book-check
```

Build Rust API docs with:

```bash
just docs
```

Build both Rust docs and the book with:

```bash
just docs-all
```

When adding or rewriting Rustdoc, follow `docs/dev/rustdoc_style.md`. The root
crate, `zappy-cli`, `zappy-core`, `zappy-fs`, and `zappy-templates` enable
strict missing-doc lints. `zappy-hooks` and `zappy-adapters` currently do not
mirror all of those crate-level lint settings.

## Scope And Source Of Truth

The current docs under `docs/book` should describe implemented behavior. Avoid
copying from older placeholder documents without checking the code first:

- `docs/manifest.md` is empty.
- `docs/templates.md` is empty.
- `docs/dev/implementation_plan.md` is aspirational.

For behavior details, prefer current source files and tests.
