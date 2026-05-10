# Zappy

> Electrifying project templating/scaffolding engine.

`Zappy` is a project templating/scaffolding engine written in Rust to generate projects from a multitude of pre-defined templates. It also allows the user to create their own templates or generate ones from an existing project.

> **Note:** This project is a re-implementation of the [zappy](https://gitlab.com/byarocks/zappy) luarock and takes heavy inspiration from the [spawn_point](https://github.com/normano/spawnpoint/tree/main) crate.

<!-- toc -->

- [Why?](#why)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Templates](#templates)
  * [Built-in templates](#built-in-templates)
  * [Template structure](#template-structure)
  * [Placeholders and variables](#placeholders-and-variables)
  * [Hooks](#hooks)
  * [Template registries](#template-registries)
  * [User templates](#user-templates)
- [Config file](#config-file)
- [Development](#development)
  * [Prequisites](#prequisites)
  * [Getting started](#getting-started)
- [Documentation](#documentation)
  * [Style](#style)
- [Similar projects](#similar-projects)
- [License](#license)
- [Repository maturity note](#repository-maturity-note)

<!-- tocstop -->

## Why?

TBD

## Features

TBD

---

## Installation

TBD

---

## Usage

TBD

---

## Templates

TBD

### Built-in templates

TBD

### Template structure

TBD

### Placeholders and variables

TBD

### Hooks

TBD

### Template registries

TBD

### User templates

TBD

---

## Config file

TBD

---

## Development

### Prequisites

- Rust stable toolchain with `rustfmt` and `clippy` (`rust-toolchain.toml`)
- Rust `1.85.0` or newer for workspace builds
- [`just`](https://crates.io/crates/just)

For a first time setup, run:
```bash
cargo install just
just init
```

This installs just and its `init` bootstrap recipe installs all extra tooling used by this repository, including coverage, lint/audit tools, README indexing, pre-commit hooks and more.

### Getting started

Run zappy:
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

## Similar projects
- [Zync](https://gitlab.com/byarocks/zync)
- [Spawnpoint](https://github.com/normano/spawnpoint)

> 🔔 **Note**: Both projects are present in the workspace under `docs/similar-projects`. They are, however, not part of the project.

---

## License

Dual licensed under:

- Apache License 2.0 (`LICENSE-APACHE`)
- MIT (`LICENSE-MIT`)

---

## Repository maturity note

This is an actively evolving implementation. The architecture is intentionally ahead of the current feature set so the project can grow without needing a rewrite.
