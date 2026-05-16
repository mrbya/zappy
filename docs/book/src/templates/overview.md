# Overview

A Zappy template is a directory with two parts:

- `zappy.toml`, the template manifest.
- A source tree, usually named `template`, containing the files to generate.

The manifest gives Zappy enough information to discover the template, resolve
variables, decide which paths to include, render placeholders, run hooks, and
optionally validate the generated project.

## Template Directory Shape

The default shape is:

```text
my-template/
  zappy.toml
  template/
    README.md
    src/
      main.rs
```

The source directory name is controlled by `template.source.root` in the
manifest. If that field is omitted, Zappy uses `template`.

## Generation Flow

When `zappy new` runs, Zappy:

1. Discovers templates and selects one by `template.id`.
2. Parses and validates the selected `zappy.toml`.
3. Resolves variables from CLI overrides, defaults, and built-ins.
4. Walks the template source tree in deterministic path order.
5. Applies excludes, conditionals, symlink skips, placeholder rendering, and
   binary file rules to build a generation plan.
6. In dry-run mode, prints the plan and stops.
7. In normal mode, creates the output directory, runs pre-generation hooks,
   writes files, runs post-generation hooks, and prints a summary.

## Main Concepts

| Concept | Purpose | See Also |
| --- | --- | --- |
| Template discovery | Finds template directories and chooses the first template for each ID. | [Template Discovery](./discovery.md) |
| Bundled templates | Starter templates embedded in the `zappy-templates` crate and extracted to a cache. | [Built-in Templates](./builtins.md) |
| Manifest metadata | Defines the template ID, name, language, version, authors, and source root. | [Manifest Basics](./manifest.md) |
| Variables | Provide values for placeholders and conditions. | [Variables and Placeholders](./variables.md) |
| Path rules | Exclude paths, mark binary files, and conditionally include files. | [Manifest Reference](../reference/manifest.md) |
| Hooks | Run external commands before or after generation, or during validation. | [Hooks](./hooks.md) |
| Validation | Generate a template into a temporary directory and run checks. | [Validation](./validation.md) |

## What Zappy Renders

Zappy performs literal placeholder replacement in UTF-8 text files and in output
paths. It does not use a general-purpose template language. A placeholder is any
literal string configured in the manifest, such as `__DESCRIPTION__`, plus the
engine-defined built-in placeholders such as `__ZAPPY_PROJECT_NAME__`.

Files classified as binary are copied without text rendering. Files that are not
explicitly classified as binary are read as UTF-8 and rendered; if UTF-8 reading
fails, Zappy copies the file as binary and adds a warning to the generation plan.

## Implemented Limits

The current implementation intentionally keeps several behaviors narrow:

- Interactive prompting is not implemented yet, even though `zappy new` accepts
  `--non-interactive`.
- `zappy create` only creates an empty skeleton when `--empty` is passed;
  converting an existing project is still a stub.
- Hook specs may parse `shell = true`, but hook execution rejects shell hooks as
  unsupported.
- `validation_regex` is parsed in variable specs, but it is not enforced during
  variable resolution yet.
