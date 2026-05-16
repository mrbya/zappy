# Changelog

All notable changes to Zappy are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

---

## [v.0.1.0] - 2026-05-16

### Added

- Initial Rust implementation of the `zappy` project templating and scaffolding CLI.
- Workspace split into focused crates for CLI dispatch, core template logic, filesystem operations, hook execution, bundled templates, and reserved ecosystem adapters.
- Template discovery from explicit `--templates-dir`, environment/config paths, executable-relative templates, current-directory templates, and bundled templates.
- Bundled starter templates for Rust CLI apps, C++ CMake apps and libraries, Lua CLI apps, and Neovim plugins.
- `zappy list` and `zappy info` commands for discovering and inspecting templates.
- `zappy new` command for generating projects from templates with variable overrides, dry-run previews, force overwrites, and optional hook skipping.
- Template manifest support through `zappy.toml`, including metadata, source roots, variables, transforms, placeholders, path rules, conditionals, hooks, and validation blocks.
- Built-in variables for project name, user, email, date, day, month, and year, with transform-specific placeholders.
- Generation planning and materialization with deterministic traversal, text rendering, binary file copying, excluded paths, conditional paths, symlink skips, and conflict warnings.
- `zappy validate` command for generating templates into temporary directories and running validation setup, step, and teardown hooks.
- `zappy init` and `zappy create --empty` commands for creating empty template skeletons.
- Bundled-template cache extraction and `zappy --clear` cache clearing.
- mdBook documentation covering user workflows, template authoring, template discovery, reference material, migration from Lua Zappy, and contributor architecture notes.

### Known Limitations

- `zappy create` without `--empty` is still a stub and does not convert existing projects into templates yet.
- `zappy new --non-interactive` is parsed but interactive prompting is not implemented yet.
- Hook specs with `shell = true` are parsed but rejected during execution.
- Variable `validation_regex` is parsed but not currently enforced.
