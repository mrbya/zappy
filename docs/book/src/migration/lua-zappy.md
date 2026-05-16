# Migrating from Lua Zappy

This guide is for users and template authors moving from the older Lua/LuaRocks
implementation of Zappy to the current Rust implementation.

The old implementation in `.idea/zappy` was a Lua CLI with `create`, `gen`, and
`ls` commands. Templates were Lua files that returned tables. The Rust
implementation is a Cargo-installed CLI with a TOML manifest and filesystem
source tree for each template.

## Migration Summary

The migration is not a file-format rename. Expect to rewrite templates.

| Area | Lua Zappy | Rust Zappy |
| --- | --- | --- |
| Install model | LuaRocks package named `zappy`. | Rust/Cargo package and binary named `zappy`. |
| Main generation command | `zappy gen` | `zappy new` |
| List and info command | `zappy ls`, with `-a` and `-t`. | `zappy list` and `zappy info`. |
| Empty template command | `zappy create -e` | `zappy init` or `zappy create --empty`. |
| Create from existing project | Implemented by scanning a project into a Lua template table. | Not implemented yet; non-empty `zappy create` is currently a stub. |
| Template format | Lua file returning a table. | Directory containing `zappy.toml` and a source root. |
| Template source | Nested Lua `structure` table. | Real files and directories under `template.source.root`. |
| Placeholders | `#{NAME}` interpreted from a Lua config table. | Literal placeholders declared in `zappy.toml`, plus built-in `__ZAPPY_*__` placeholders. |
| Hooks | Lua callback functions in `hooks.pre` and `hooks.post`. | External command specs in `hooks.pre_generate` and `hooks.post_generate`. |
| Interactive prompts | Used by `gen` when values were missing. | Not implemented yet; values must come from `--var`, defaults, validation variables, or built-ins. |
| Validation | No first-class validation block in the template table. | Optional `[validation]` block with setup, steps, and teardown hooks. |

## Command Mapping

### List Templates

Lua:

```bash
zappy ls
zappy ls -a
zappy ls -t cpp_cm
```

Rust:

```bash
zappy list
zappy info --template cpp-cmake-app
```

`list` and `info` are split in Rust. `list` prints a table of discovered
templates. `info` prints details for one template ID.

### Generate A Project

Lua:

```bash
zappy gen -n zappyProject -t cpp_cm -a BOARD=esp32s2_lolin_mini
```

Rust:

```bash
zappy new \
  --template cpp-cmake-app \
  --name zappyProject \
  --var description="Generated C++ app"
```

The Rust command requires `--template` and `--name`. It does not prompt for a
template or missing variables yet. Use `--var key=value` for every value that is
not supplied by a template default or built-in variable.

Use `--dry-run` to preview the generation plan:

```bash
zappy new \
  --template cpp-cmake-app \
  --name zappyProject \
  --var description="Generated C++ app" \
  --dry-run
```

There was no direct Lua equivalent to the Rust dry-run generation plan.

### Initialize A Git Repository

Lua `gen` had `-g` to initialize the generated project as a Git repository.

Rust Zappy does not have an equivalent `new` flag today. Initialize Git after
generation if you need it:

```bash
git -C zappyProject init
git -C zappyProject add .
git -C zappyProject commit -m "Initial commit"
```

### Create An Empty Template

Lua:

```bash
zappy create -e -f my_template
```

Rust:

```bash
zappy init --output ./my-template --template my-template --name "My Template"
```

or:

```bash
zappy create --empty --output ./my-template --template my-template --name "My Template"
```

Rust writes a directory containing `zappy.toml` and a `template/` source root.
Lua wrote a Lua template file into the configured templates directory.

### Create A Template From An Existing Project

Lua `create` could scan an existing project, respect basic `.gitignore`
patterns, and serialize the result into a Lua template file:

```bash
zappy create -p ./existing-project -f my_template
```

Rust Zappy does not implement this workflow yet. `zappy create` without
`--empty` currently prints a stub message and parsed arguments.

For now, create a Rust template manually:

1. Run `zappy init --output ./my-template --template my-template`.
2. Copy project files into `./my-template/template`.
3. Add `[paths]` excludes for generated/build/cache directories.
4. Add variables and placeholders in `zappy.toml`.
5. Test with `zappy new --templates-dir . --template my-template --name demo --dry-run`.

## Template Format Migration

### Lua Template Shape

Lua templates were modules that returned a table:

```lua
return {
    name = "Example template",
    desc = "Template description",
    args = {
        example = "Example app",
    },
    hooks = {
        pre = {},
        post = {
            function(path, config)
                os.execute('echo generated > ' .. path .. '/zappy.txt')
            end,
        },
    },
    structure = {
        ["README.md"] = "# #{example}",
        ["src"] = {
            ["main.cpp"] = "// Hello from #{PROJECT}",
        },
    },
}
```

### Rust Template Shape

Rust templates are directories:

```text
my-template/
  zappy.toml
  template/
    README.md
    src/
      main.cpp
```

The manifest holds metadata and behavior:

```toml
[template]
id = "my-template"
name = "My Template"
description = "Template description"
language = "cpp"

[template.source]
root = "template"

[variables]
example = {
    default = "Example app",
    placeholders = { raw = "__EXAMPLE__" }
}
```

The source files hold normal file contents:

```text
# __EXAMPLE__

Generated project: __ZAPPY_PROJECT_NAME__
```

See [Manifest Basics](../templates/manifest.md) and
[Manifest Reference](../reference/manifest.md) for the implemented manifest
schema.

## Variables And Placeholders

Lua Zappy used the `args` table for template variables and interpolated keys with
`#{...}` syntax. Defaults and user config values lived in a shared Lua config
table.

Rust Zappy declares variables in `[variables]` and replaces literal placeholders
configured per variable:

```toml
[variables]
board = {
    default = "nucleo_l432kc",
    placeholders = { raw = "__BOARD__" }
}
```

Then write `__BOARD__` in files or paths.

Rust also supports transform-specific placeholders:

```toml
[variables]
project_label = {
    default = "my cool tool",
    placeholders = {
        raw = "__PROJECT_LABEL__",
        kebab = "__PROJECT_LABEL_KEBAB__",
        snake = "__PROJECT_LABEL_SNAKE__",
        pascal = "__PROJECT_LABEL_PASCAL__"
    }
}
```

The current Rust CLI does not read a `zconfig.lua`-style user defaults file.
Values must come from `--var`, manifest defaults, validation variables, or
built-ins.

See [Variables and Placeholders](../templates/variables.md) for details.

## Built-In Variable Changes

Lua built-ins were available as plain config keys and used uppercase names:

| Lua Built-in | Meaning |
| --- | --- |
| `PROJECT` | Generated project name. |
| `USER` | Git user name. |
| `DATE` | OS date in `DD-MM-YYYY` format. |
| `DAY` | Day. |
| `MONTH` | Month. |
| `YEAR` | Year. |

Rust built-ins use reserved lower-case variable names and engine-defined
placeholders:

| Rust Built-in | Raw Placeholder |
| --- | --- |
| `project_name` | `__ZAPPY_PROJECT_NAME__` |
| `user` | `__ZAPPY_USER__` |
| `email` | `__ZAPPY_EMAIL__` |
| `date` | `__ZAPPY_DATE__` |
| `day` | `__ZAPPY_DAY__` |
| `month` | `__ZAPPY_MONTH__` |
| `year` | `__ZAPPY_YEAR__` |

Rust `date` currently renders as `YYYY-MM-DD`, not Lua's `DD-MM-YYYY`.

Do not declare Rust built-ins under `[variables]`; they are reserved names. See
[Built-in Variables](../reference/builtins.md) for transform suffixes and
fallback behavior.

## Hook Migration

Lua hooks were in-process Lua callback functions:

```lua
hooks = {
    pre = {},
    post = {
        function(path, config)
            os.execute('echo generated > ' .. path .. '/zappy.txt')
        end,
    },
}
```

Rust hooks are external command specs:

```toml
[[hooks.post_generate]]
name = "Write generation marker"
command = "sh"
args = ["-c", "printf generated > zappy.txt"]
optional = true
```

Important differences:

- Rust hooks are not Lua functions and do not receive a Lua `config` table.
- Rust renders placeholders in `command`, `args`, `working_dir`, and environment
  values before spawning the process.
- Hooks run from the generated output directory unless `working_dir` is set.
- Required hook failures fail generation; optional hook failures become warnings.
- `shell = true` is parsed but rejected. Use `command = "sh"` with `args` when
  shell behavior is needed.

See [Hooks](../templates/hooks.md) for the supported hook fields.

## Template Discovery And User Templates

Lua Zappy loaded built-in templates from `lua/zappy/lib/templates` and user
templates from `$ZAPPY_CONFIG/templates`, defaulting to
`~/.config/zappy/templates`. User templates were Lua files loaded through
`package.path`.

Rust Zappy discovers template directories instead of Lua files. A custom Rust
template must be a directory containing `zappy.toml`. A search path can be either
one template directory or a directory containing multiple template directories.

Without `--templates-dir`, Rust discovery checks several paths, including
`ZAPPY_TEMPLATES_DIR`, `$ZAPPY_CONFIG/templates`, platform config templates,
executable-relative templates, `./templates`, and bundled templates. With
`--templates-dir`, Rust uses only that path and disables bundled templates.

See [Template Discovery](../templates/discovery.md) for the exact search order.

## Validation

Lua Zappy templates did not have a first-class validation block. Tests existed
for the Lua implementation, but template validation was not part of the template
format described by the Lua README.

Rust templates can define `[validation]`:

```toml
[validation]
output_dir_name = "validated-my-template"
variables = {
    example = "Validation value",
}

[[validation.steps]]
name = "check README exists"
command = "test"
args = ["-f", "README.md"]
```

Run validation with:

```bash
zappy validate --template my-template
```

Validation generates into a temporary directory and runs setup, step, and
teardown hooks. See [Validation](../templates/validation.md).

## Built-In Template ID Changes

The bundled template sets do not match one-to-one.

| Lua Template | Closest Rust Template | Notes |
| --- | --- | --- |
| `cpp_cm` | `cpp-cmake-app` | C++ CMake app template. |
| `cpp_lib_cm` | `cpp-cmake-lib` | C++ CMake library template. |
| `nvim_plug` | `nvim-plugin` | Neovim plugin template. |
| `cpp_m` | No bundled equivalent | Make-based C++ app is not bundled in Rust Zappy. |
| `zos`, `zos_cpp`, `zos_module`, `zos_module_cpp` | No bundled equivalent | Zephyr templates are not bundled in Rust Zappy. |
| No Lua equivalent | `rust-cli`, `lua-cli` | New bundled Rust and Lua CLI templates. |

Use `zappy list` to see the templates available in your installed Rust version.

## Practical Migration Checklist

For each Lua template:

1. Create a Rust template skeleton with `zappy init`.
2. Move files from the Lua `structure` table into real files under `template/`.
3. Convert `name` and `desc` into `[template]` metadata.
4. Convert `args` into `[variables]` entries.
5. Replace `#{VAR}` placeholders with explicit placeholders declared in
   `zappy.toml`, or with Rust built-in placeholders such as
   `__ZAPPY_PROJECT_NAME__`.
6. Convert Lua callback hooks into external command specs, or remove hooks that
   need in-process Lua access to `config`.
7. Add `[paths]` excludes for generated directories and binary files.
8. Add `[validation]` steps if the template can check itself.
9. Test with `zappy new --templates-dir <parent> --template <id> --name demo --dry-run`.
10. Generate into a temporary output directory and inspect the result.

## Known Parity Gaps

These Lua-era behaviors do not currently have direct Rust equivalents:

- Interactive prompt flow for choosing templates and filling missing variables.
- `gen -g` Git initialization.
- `create` from an existing project.
- `zconfig.lua` as a user default-variable source.
- In-process Lua hook callbacks with access to the full config/template table.
- Lua `structure` tables as template source.
- Lua built-in placeholder syntax such as `#{PROJECT}`.
- Bundled Make and Zephyr templates from the old Lua distribution.

These are intentional migration concerns, not guaranteed compatibility targets.
Use the current Rust command and template reference pages as the source of truth
for supported behavior.
