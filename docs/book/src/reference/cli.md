# CLI Reference

This page is a lookup reference for the implemented `zappy` command-line
interface. It is based on the Clap definitions in `zappy-cli`.

## Top-Level Syntax

```text
zappy [OPTIONS] <COMMAND>
```

## Global Options

| Option | Type | Behavior |
| --- | --- | --- |
| `-v`, `--verbose...` | Count | Accepted by the parser. Repeat for higher verbosity. Current command output does not use it extensively yet. |
| `-c`, `--clear` | Flag | Clears the bundled-template cache before executing the subcommand. |
| `-h`, `--help` | Flag | Prints help. |
| `-V`, `--version` | Flag | Prints version. |

Global options must appear before the subcommand when they affect top-level
dispatch, for example `zappy --clear list`.

## Commands And Aliases

| Command | Alias | Description |
| --- | --- | --- |
| `list` | `ls` | List available templates. |
| `info` | `i` | Display detailed info about a template. |
| `new` | `n` | Generate a new project from a template. |
| `validate` | `val` | Validate that a template generates a working project. |
| `init` | `it` | Initialize an empty template skeleton. |
| `create` | `c` | Create an empty template skeleton with `--empty`; non-empty behavior is a stub. |

## `list`

```text
zappy list [OPTIONS]
```

| Option | Type | Behavior |
| --- | --- | --- |
| `-i`, `--templates-dir <TEMPLATES_DIR>` | Path | Use only this template directory. Bundled templates are disabled. |
| `-l`, `--language <LANGUAGE>` | String | Show only templates whose manifest `template.language` equals this value. |

Output is a table with ID, language, and name. If no templates match, the
command prints `No templates found.` and exits successfully.

## `info`

```text
zappy info [OPTIONS] --template <TEMPLATE>
```

| Option | Type | Behavior |
| --- | --- | --- |
| `-t`, `--template <TEMPLATE>` | String | Template ID to inspect. Required. |
| `-i`, `--templates-dir <TEMPLATES_DIR>` | Path | Use only this template directory. Bundled templates are disabled. |

The command fails if the template ID is not found. Output includes manifest
metadata and filesystem paths.

## `new`

```text
zappy new [OPTIONS] --template <TEMPLATE> --name <PROJECT_NAME>
```

| Option | Type | Behavior |
| --- | --- | --- |
| `-t`, `--template <TEMPLATE>` | String | Template ID to generate from. Required. |
| `-n`, `--name <PROJECT_NAME>` | String | Project name. Required. Supplies the `project_name` built-in. |
| `-o`, `--output <OUTPUT>` | Path | Output directory. Defaults to the project name. |
| `-a`, `--var <VARS>` | `key=value` | Template variable override. May be repeated. |
| `-i`, `--templates-dir <TEMPLATES_DIR>` | Path | Use only this template directory. Bundled templates are disabled. |
| `-d`, `--dry-run` | Flag | Print the generation plan without writing files. |
| `-x`, `--non-interactive` | Flag | Parsed, but prompting is not implemented. Missing values still fail unless defaults or overrides exist. |
| `-f`, `--force` | Flag | Overwrite conflicting files. |
| `-s`, `--no-hooks` | Flag | Skip generation hooks. |

CLI variable parsing requires `key=value`. Values parse as booleans for
`true`, `yes`, `y`, `Y`, `1`, `false`, `no`, `n`, `N`, or `0`; integer-looking
values parse as integers; everything else remains a string.

## `validate`

```text
zappy validate [OPTIONS] --template <TEMPLATE>
```

| Option | Type | Behavior |
| --- | --- | --- |
| `-t`, `--template <TEMPLATE>` | String | Template ID to validate. Required. |
| `-i`, `--templates-dir <TEMPLATES_DIR>` | Path | Use only this template directory. Bundled templates are disabled. |
| `-k`, `--keep-temp` | Flag | Keep the temporary validation directory and print its path. |
| `-s`, `--no-hooks` | Flag | Skip generation hooks only; validation setup, steps, and teardown still run. |

The command fails when the selected template has no `[validation]` block.
Validation generation uses overwrite behavior internally.

## `init`

```text
zappy init [OPTIONS] --output <OUTPUT>
```

| Option | Type | Behavior |
| --- | --- | --- |
| `-o`, `--output <OUTPUT>` | Path | Directory where the template skeleton should be created. Required. |
| `-t`, `--template <TEMPLATE>` | String | Optional template ID for the generated manifest. |
| `-n`, `--name <NAME>` | String | Optional template name. |
| `-d`, `--description <DESCRIPTION>` | String | Optional template description. |
| `-f`, `--force` | Flag | Overwrite conflicting skeleton files. |

When metadata is omitted, the skeleton derives a template ID from the output
directory name, uses that ID as the name, and uses a TODO description.

## `create`

```text
zappy create [OPTIONS] --output <OUTPUT>
```

| Option | Type | Behavior |
| --- | --- | --- |
| `-i`, `--from <FROM>` | Path | Existing project path intended for future conversion behavior. |
| `-o`, `--output <OUTPUT>` | Path | Output directory. Required. |
| `-e`, `--empty` | Flag | Generate an empty template skeleton. |
| `-t`, `--template <TEMPLATE>` | String | Optional template ID for the generated manifest. |
| `-n`, `--name <NAME>` | String | Optional template name. |
| `-s`, `--description <DESCRIPTION>` | String | Optional template description. |
| `-a`, `--var <VARS>` | `key=value` | Parsed for future non-empty creation behavior. |
| `-f`, `--force` | Flag | Overwrite conflicting skeleton files when `--empty` is used. |

Only `create --empty` is implemented today. Without `--empty`, the command
prints a stub message and the parsed arguments, then exits successfully.

## Discovery And Environment

Commands that discover templates accept `--templates-dir`. When this option is
set, discovery uses only that path and does not use bundled templates.

Without `--templates-dir`, discovery checks `ZAPPY_TEMPLATES_DIR`,
`$ZAPPY_CONFIG/templates`, platform config templates, executable-relative
templates, current-working-directory templates, and bundled templates in that
order.

`ZAPPY_TEMPLATES_DIR` is required when set. Missing implicit paths are skipped.

## Exit Behavior

Commands return success on normal completion and failure on errors such as:

- Missing or invalid required arguments.
- Invalid variable override syntax.
- Unknown template IDs.
- Manifest parse or validation errors.
- Missing required variables.
- File conflicts without `--force`.
- Required hook failures.
- Validation failures.
