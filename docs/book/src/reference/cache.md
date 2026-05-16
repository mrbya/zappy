# Template Cache

Zappy's bundled templates are embedded in the `zappy-templates` crate at build
time. The discovery pipeline works with real filesystem directories, so the CLI
extracts bundled templates into a cache directory before discovery.

## Cache Location

The cache lives under the platform cache directory resolved for the app name
`zappy`:

```text
<zappy-cache-dir>/bundled-templates/<zappy-templates-version>/templates
```

The exact base directory depends on the operating system and the `directories`
crate's `ProjectDirs` resolution.

## Cache Contents

The cache directory contains extracted bundled template directories such as:

```text
rust-cli/
  zappy.toml
lua-cli/
  zappy.toml
.zappy-templates-cache
```

The marker file is named `.zappy-templates-cache`. It stores:

- cache format version
- package name
- package version
- bundled templates hash

## When The Cache Is Prepared

The CLI prepares bundled templates when a command needs template discovery and
`--templates-dir` was not provided.

If the cache marker exists and matches the expected marker, the cache is reused.
If the marker is missing or stale, Zappy extracts the embedded bundled templates
again and writes a fresh marker.

If cache preparation fails, the CLI prints a warning and continues without the
bundled search path. Other discovery paths may still provide templates.

## Clearing The Cache

Pass the top-level `--clear` flag before the subcommand:

```bash
zappy --clear list
zappy --clear info --template rust-cli
zappy --clear new --template rust-cli --name my-tool
```

`--clear` removes the bundled template cache before command execution. If the
command then needs bundled templates, the cache is prepared again during normal
discovery.

## Explicit Template Directories Disable Bundled Cache Use

When `--templates-dir` is set, the CLI does not prepare or use bundled templates
for discovery:

```bash
zappy list --templates-dir ./templates
```

This keeps explicit-directory runs deterministic and avoids mixing local
templates with bundled templates.
