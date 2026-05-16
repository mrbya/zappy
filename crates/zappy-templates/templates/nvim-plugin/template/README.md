# __ZAPPY_PROJECT_NAME__

> __DESCRIPTION__

<!-- toc -->

- [Features](#features)
- [Installation](#installation)
- [Dependencies](#dependencies)
- [Configuration](#configuration)
  * [Default config](#default-config)
- [Usage](#usage)
  * [Kaymaps](#kaymaps)
  * [Commands](#commands)
- [Development](#development)
  * [Prequisites](#prequisites)
    + [Lua](#lua)
    + [Other](#other)
  * [Getting started](#getting-started)
- [Documentation](#documentation)
- [License](#license)

<!-- tocstop -->

## Features

TBD

---

## Installation

So far tested only with [lazy.nvim](https://github.com/folke/lazy.nvim)

```lua
{
    'mrbya/__ZAPPY_PROJECT_NAME_SNAKE__',
    event = 'VeryLazy',
    opts = {},
}
```
`opts` table required to load plugin (even if empty).

## Dependencies

TBD

## Configuration

### Default config
```lua
{
    filetypes = { '*' },

    keymaps = {

    },
}
```

---

## Usage

### Kaymaps

| Keymap | Command |
| -------------- | --------------- |
| `` |  |

### Commands

| Command | Action |
| -------------- | --------------- |
| `` |  |

---

## Development

### Prequisites

#### Lua
- Lua > 5.1
- [luacheck](https://luarocks.org/modules/lunarmodules/luacheck)

#### Other
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

Add __ZAPPY_PROJECT_NAME__ repo as a local plugin:

```lua
--- example plugins.init.lua:
return {
    {
        '__ZAPPY_PROJECT_NAME_SNAKE__',
        dir = 'path/to/__ZAPPY_PROJECT_NAME_SNAKE__',
        event = 'VeryLazy',
        opts = {}
    },
}
```

Fire up neovim and test the plugin.

---

## Documentation

TBD

---

## License

Licensed under [MIT](LICENSE) license.
