# Template Discovery

Template discovery is implemented by `zappy-fs`. Every CLI command that needs a
template builds a discovery configuration, asks `zappy-fs` to discover template
directories, and then selects templates by manifest ID.

## Template Directory Detection

A search path can be either:

- A single template directory that directly contains `zappy.toml`.
- A directory containing child directories, where each child that contains
  `zappy.toml` is treated as a template directory.

Discovery does not recursively search arbitrary nested directories. It checks the
search path itself first, then its immediate child directories.

## Explicit Directory Mode

When `--templates-dir <PATH>` is passed, discovery uses only that path.

```bash
zappy list --templates-dir ./templates
zappy info --templates-dir ./templates --template rust-cli
zappy new --templates-dir ./templates --template rust-cli --name my-tool
```

In explicit mode:

- Bundled templates are disabled.
- Environment/config/default paths are ignored.
- A missing explicit path is an error.
- A path that exists but is not a directory is an error.

This mode is useful for tests, local template development, and CI jobs where you
want deterministic template input.

## Default Search Order

Without `--templates-dir`, Zappy searches these paths in order:

| Order | Source | Required? |
| --- | --- | --- |
| 1 | `ZAPPY_TEMPLATES_DIR` | Yes, when the variable is set. |
| 2 | `$ZAPPY_CONFIG/templates` | No. |
| 3 | The platform config directory for app `zappy`, plus `templates` | No. |
| 4 | `templates` next to the current `zappy` executable | No. |
| 5 | `templates` under the current working directory | No. |
| 6 | Bundled templates extracted by `zappy-templates` | No. |

Missing optional paths are skipped. `ZAPPY_TEMPLATES_DIR` is different: if it is
set, the path is considered required and a missing path causes discovery to
fail.

`ZAPPY_CONFIG` is interpreted as a base directory; Zappy appends `templates` to
it. For example, `ZAPPY_CONFIG=/opt/zappy` makes Zappy check
`/opt/zappy/templates`.

## Bundled Template Fallback

When no explicit templates directory is provided, the CLI asks
`zappy-templates` to prepare the bundled starter templates as a real filesystem
directory. That directory is appended as the final search path.

If preparing bundled templates fails, the CLI prints a warning and continues
without the bundled path. Discovery can still succeed if another search path has
templates.

## Duplicate Template IDs

Template IDs are first-hit wins.

If multiple search paths contain templates with the same `template.id`, the
first discovered template remains active and later templates are recorded as
shadowed duplicates. The public CLI commands use the active template list; they
do not currently print the shadowed list.

Active and shadowed templates are sorted by template ID after discovery. The
first-hit decision still follows search path order.

## Discovery Errors

Discovery can fail when:

- A required search path is missing.
- A required search path is not a directory.
- A search path cannot be read.
- A directory entry cannot be inspected.
- A `zappy.toml` file cannot be read, parsed, or validated.

Implicit optional paths that are missing or not directories are skipped instead
of failing discovery.
