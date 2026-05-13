# __ZAPPY_PROJECT_NAME__

> __DESCRIPTION__

<!-- toc -->

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Development](#development)
  * [Prequisites](#prequisites)
  * [Getting started](#getting-started)
  * [Getting started](#getting-started-1)
- [Documentation](#documentation)
  * [Style](#style)
- [License](#license)

<!-- tocstop -->

## Features

TBD

---

## Installation

Install using cargo:
```bash
cargo install __ZAPPY_PROJECT_NAME_KEBAB__
```

## Usage

```bash
__ZAPPY_PROJECT_NAME_KEBAB__ <COMMAND>

Commands:
  greet        Print a greeting
  new-command  New command stub,
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```
---

## Development

### Prequisites

- Rust stable toolchain with `rustfmt` and `clippy` (`rust-toolchain.toml`)
- Rust `1.85.0` or newer for workspace builds
- [`just`](https://crates.io/crates/just)

### Getting started

For a first time setup, run:
```bash
cargo install just
just init
```

This installs just and its `init` bootstrap recipe installs all extra tooling used by this repository, including coverage, lint/audit tools, README indexing, pre-commit hooks and more.

### Getting started

Run __ZAPPY_PROJECT_NAME_KEBAB__:
```bash
just run -- <pass in args>

```

Run tests:
```bash
just test
```

Before committing work:
```bash
just pre-commit

```

To see all available recipes:
```bash
just list

# or

just help
```

---

## Documentation

TBD

### Style

Codebase documented using a consistent rustdoc style described in [rustdoc style guide](docs/rustdoc_style.md).

---

## License

Dual licensed under:

- Apache License 2.0 (`LICENSE-APACHE`)
- MIT (`LICENSE-MIT`)
