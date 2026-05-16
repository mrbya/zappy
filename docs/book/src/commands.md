# Commands

Zappy exposes one top-level binary, `zappy`, with subcommands for discovering,
inspecting, generating, validating, and creating templates.

```bash
zappy [OPTIONS] <COMMAND>
```

## Global Options

| Option | Description |
| --- | --- |
| `-v`, `--verbose...` | Accepted verbosity flag. Repeat as `-v`, `-vv`, or `-vvv` for increasing verbosity. |
| `-c`, `--clear` | Clear the bundled-template cache before running the command. |
| `-h`, `--help` | Print help. |
| `-V`, `--version` | Print the Zappy version. |

The `--clear` flag is useful when you want Zappy to refresh its extracted
bundled templates before discovery.

## Command Summary

| Command | Alias | Purpose |
| --- | --- | --- |
| `list` | `ls` | List discovered templates. |
| `info` | `i` | Show details for one template. |
| `new` | `n` | Generate a project from a template. |
| `validate` | `val` | Generate and validate a template that defines validation steps. |
| `init` | `it` | Initialize an empty template skeleton. |
| `create` | `c` | Create an empty template skeleton with `--empty`; non-empty creation is currently a stub. |

## `list`

List available templates:

```bash
zappy list
```

Usage:

```text
zappy list [OPTIONS]
```

Options:

| Option | Description |
| --- | --- |
| `-i`, `--templates-dir <TEMPLATES_DIR>` | Use only this template directory instead of the default discovery paths and bundled templates. |
| `-l`, `--language <LANGUAGE>` | Show only templates whose manifest language exactly matches the provided value. |

Examples:

```bash
zappy list --language rust
zappy list --templates-dir ./templates
```

When no templates match, Zappy prints `No templates found.` and exits
successfully.

## `info`

Show details for a template:

```bash
zappy info --template rust-cli
```

Usage:

```text
zappy info [OPTIONS] --template <TEMPLATE>
```

Options:

| Option | Description |
| --- | --- |
| `-t`, `--template <TEMPLATE>` | Template ID to inspect. Required. |
| `-i`, `--templates-dir <TEMPLATES_DIR>` | Use only this template directory for discovery. |

The output includes available metadata such as ID, name, description, language,
version, authors, source root, template directory, and manifest path. If the
template cannot be found, the command fails with an error.

## `new`

Generate a project from a template:

```bash
zappy new --template rust-cli --name my-tool
```

Usage:

```text
zappy new [OPTIONS] --template <TEMPLATE> --name <PROJECT_NAME>
```

Options:

| Option | Description |
| --- | --- |
| `-t`, `--template <TEMPLATE>` | Template ID to generate from. Required. |
| `-n`, `--name <PROJECT_NAME>` | Project name. Required. Also supplies the `project_name` built-in variable. |
| `-o`, `--output <OUTPUT>` | Output directory. Defaults to the project name. |
| `-a`, `--var <VARS>` | Variable override in `key=value` form. May be repeated. |
| `-i`, `--templates-dir <TEMPLATES_DIR>` | Use only this template directory for discovery. |
| `-d`, `--dry-run` | Preview the generation plan without writing files. |
| `-x`, `--non-interactive` | Parsed by the CLI, but there is no prompt flow yet. Missing values still need overrides, defaults, or built-ins. |
| `-f`, `--force` | Overwrite conflicting files. |
| `-s`, `--no-hooks` | Skip template generation hooks. |

Examples:

```bash
zappy new \
  --template rust-cli \
  --name my-tool \
  --var description="My generated tool" \
  --var repo="https://example.com/me/my-tool.git" \
  --dry-run
```

```bash
zappy new \
  --template rust-cli \
  --name my-tool \
  --var description="My generated tool" \
  --var repo="https://example.com/me/my-tool.git" \
  --output ./my-tool \
  --no-hooks
```

During a real generation run, Zappy creates the output directory, runs
pre-generation hooks unless skipped, writes planned files, runs post-generation
hooks unless skipped, and prints a summary.

## `validate`

Validate that a template can generate a working project:

```bash
zappy validate --template rust-cli
```

Usage:

```text
zappy validate [OPTIONS] --template <TEMPLATE>
```

Options:

| Option | Description |
| --- | --- |
| `-t`, `--template <TEMPLATE>` | Template ID to validate. Required. |
| `-i`, `--templates-dir <TEMPLATES_DIR>` | Use only this template directory for discovery. |
| `-k`, `--keep-temp` | Keep the temporary validation directory and print its path. |
| `-s`, `--no-hooks` | Skip generation hooks only. Validation setup, step, and teardown hooks still run. |

The selected template must define a `[validation]` block. Validation resolves
variables from that block, generates into a temporary directory with overwrite
behavior enabled, and runs validation hooks and steps.

## `init`

Initialize an empty template skeleton:

```bash
zappy init --output ./my-template
```

Usage:

```text
zappy init [OPTIONS] --output <OUTPUT>
```

Options:

| Option | Description |
| --- | --- |
| `-o`, `--output <OUTPUT>` | Directory where the template skeleton should be created. Required. |
| `-t`, `--template <TEMPLATE>` | Optional template ID to place in the generated manifest. |
| `-n`, `--name <NAME>` | Optional template name. |
| `-d`, `--description <DESCRIPTION>` | Optional template description. |
| `-f`, `--force` | Overwrite conflicting skeleton files. |

The skeleton includes a `zappy.toml` manifest and a `template/README.md` source
file.

## `create`

Create a template skeleton or, in the future, create a template from an existing
project.

Usage:

```text
zappy create [OPTIONS] --output <OUTPUT>
```

Options:

| Option | Description |
| --- | --- |
| `-i`, `--from <FROM>` | Existing project path intended for future conversion workflows. |
| `-o`, `--output <OUTPUT>` | Directory where the template or skeleton should be created. Required. |
| `-e`, `--empty` | Generate an empty template skeleton. |
| `-t`, `--template <TEMPLATE>` | Optional template ID to place in the generated manifest. |
| `-n`, `--name <NAME>` | Optional template name. |
| `-s`, `--description <DESCRIPTION>` | Optional template description. |
| `-a`, `--var <VARS>` | Variable in `key=value` form, reserved for future non-empty creation behavior. |
| `-f`, `--force` | Overwrite conflicting skeleton files when `--empty` is used. |

Today, only the empty skeleton path is implemented:

```bash
zappy create --empty --output ./my-template
```

Without `--empty`, `zappy create` currently prints a stub message and the parsed
arguments. It does not convert an existing project into a template yet.
