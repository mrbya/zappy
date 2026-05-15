# __ZAPPY_PROJECT_NAME__

> __DESCRIPTION__

<!-- toc -->

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Development](#development)
  * [Prequisites](#prequisites)
  * [Getting started](#getting-started)
- [Documentation](#documentation)
- [License](#license)

<!-- tocstop -->

## Features

TBD

---

## Installation

Install using luarocks:
```bash
luarocks install __ZAPPY_PROJECT_NAME_KEBAB__
```

## Usage

```bash
Usage: __ZAPPY_PROJECT_NAME_SNAKE__ [-h] [-V] [<command>] ...

__DESCRIPTION__

Options:
   -h, --help            Show this help message and exit.
   -V, --version         Display my_tool version.

Commands:
   greet, g              Prints a greeting.

```
---

## Development

### Prequisites

### Lua
- Lua > 5.1
- [argparse](https://luarocks.org/modules/argparse/argparse)
- [busted](https://luarocks.org/modules/lunarmodules/busted)
- [luacov](https://luarocks.org/modules/lunarmodules/luacov)
- [luassert](https://luarocks.org/modules/lunarmodules/luassert)
- [luafilesystem](https://luarocks.org/modules/hisham/luafilesystem)
- [inspect](https://luarocks.org/modules/kikito/inspect)
- [luacheck](https://luarocks.org/modules/lunarmodules/luacheck)

### Other
- rust + [just](https://crates.io/crates/just) + [stylua](https://crates.io/crates/stylua)
- npm + [markdown-toc](https://www.npmjs.com/package/markdown-toc)
- python + [pre-commit](https://pre-commit.com/)

For a first time setup, run:
```bash
cargo install just
just init
```

This installs just and its `init` bootstrap recipe installs all extra tooling used by this repository, including coverage, README indexing, pre-commit hooks and more.

### Getting started

Run __ZAPPY_PROJECT_NAME_SNAKE__ in dev:
```bash
just run <args>

```

Run tests:
```bash
just test
```

Or run tests with coverage report:
```bash
just test-cov
```

---

## Documentation

TBD

---

## License

Licensed under [MIT](LICENSE) license.
