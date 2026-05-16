# Introduction

Zappy is a project templating and scaffolding tool. It generates a new project
directory from a template manifest and a template source tree, replacing
placeholders, applying path rules, and optionally running generation hooks.

The Rust implementation is organized as a command-line application named
`zappy`. The installable root crate is intentionally small; it delegates command
parsing and dispatch to `zappy-cli`, while the workspace crates handle manifest
parsing, template discovery, generation planning, filesystem materialization,
hooks, and bundled templates.

## Who Zappy Is For

Zappy is useful if you:

- Start similar projects often and want a repeatable scaffold instead of copying
  boilerplate by hand.
- Maintain opinionated starter templates for a team or ecosystem.
- Want templates that can validate themselves after generation.
- Need local custom templates as well as bundled starter templates.

Template users normally interact with `zappy list`, `zappy info`, and
`zappy new`. Template authors use `zappy init`, `zappy create --empty`, and
`zappy validate` while building and checking template manifests.

## Current Scope

The current Rust CLI implements these workflows:

- Discover templates from explicit, configured, local, and bundled template
  directories.
- List available templates, optionally filtered by language.
- Show metadata for a selected template.
- Generate a project from a template with variable overrides, dry-run previews,
  force overwrites, and optional hook skipping.
- Validate templates that define a `[validation]` block.
- Initialize an empty template skeleton.
- Create an empty template skeleton with `zappy create --empty`.

Some behavior is intentionally not complete yet:

- `zappy create` without `--empty` currently prints a stub instead of converting
  an existing project into a template.
- `zappy new --non-interactive` is parsed, but there is no prompt flow yet.
  Missing required values must come from `--var`, template defaults, or built-in
  variables.
- Shell hooks declared with `shell = true` are not supported by the hook runner.

## How Templates Work

Every template is a directory containing a `zappy.toml` manifest and a template
source directory. The manifest describes template metadata, variables,
placeholder mappings, path rules, generation hooks, and optional validation
steps. During generation, Zappy resolves variables, builds a generation plan,
and materializes that plan into the target output directory.

The bundled templates are embedded in the `zappy-templates` crate and extracted
to a cache directory for normal discovery. You can also point Zappy at your own
templates with `--templates-dir`.

## Next Steps

- Read [Installation](./installation.md) to install or build the CLI.
- Follow [Quick Start](./quick-start.md) to list templates and generate a small
  project.
- Use [Commands](./commands.md) as a user-facing command overview.
- See the template chapters for authoring, discovery, variables, hooks, and
  validation details.
