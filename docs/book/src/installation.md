# Installation

Zappy is a Rust command-line application. You need a working Rust toolchain to
build it from source or install it from a Git checkout.

## Requirements

For normal use, install:

- Rust 1.85.0 or newer.
- Cargo, which is installed with Rust.

For repository development, also install `just` and run the project bootstrap
recipe described below.

## Install From crates.io

If the current release is available on crates.io, install it with Cargo:

```bash
cargo install zappy
```

Confirm the binary is available:

```bash
zappy --help
```

## Install From A Git Checkout

From a local clone of this repository, install the workspace binary with:

```bash
cargo install --path .
```

You can also run the CLI without installing it:

```bash
cargo run -- --help
```

The extra `--` separates Cargo's arguments from Zappy's arguments. For example:

```bash
cargo run -- list
```

## Developer Setup

The repository uses a `justfile` for common development commands. Install
`just` first if it is not already available:

```bash
cargo install just
```

Then run the bootstrap recipe from the repository root:

```bash
just init
```

The bootstrap recipe installs the tooling used by the project checks, including
nightly Rust for formatting and unused-dependency checks, `cargo-nextest`,
`cargo-llvm-cov`, `cargo-udeps`, `cargo-audit`, `mdbook`, `markdown-toc`, and
pre-commit hooks.

## Useful Local Commands

Use these commands from the repository root while developing Zappy:

```bash
just run -- --help
just run -- list
just test
just book-check
```

The `just run` recipe forwards arguments to `cargo run`. The book check runs
`mdbook build docs/book`.

## Build A Release Binary

To build optimized workspace binaries without installing them:

```bash
cargo build --workspace --release
```

The Zappy binary is written to `target/release/zappy`.
