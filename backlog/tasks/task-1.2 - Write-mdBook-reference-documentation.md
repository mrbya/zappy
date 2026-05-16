---
id: TASK-1.2
title: Write mdBook reference documentation
status: Done
assignee:
  - OpenCode
created_date: '2026-05-16 16:18'
updated_date: '2026-05-16 16:38'
labels:
  - documentation
  - mdbook
  - reference
milestone: Documentation
dependencies: []
references:
  - docs/book/src/reference/manifest.md
  - docs/book/src/reference/cli.md
  - docs/book/src/reference/builtins.md
  - docs/book/src/reference/cache.md
documentation:
  - AGENTS.md
  - crates/zappy-core/src
  - crates/zappy-cli/src
  - crates/zappy-templates/src
  - tests
parent_task_id: TASK-1
priority: medium
ordinal: 1400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Populate the reference section with precise, lookup-oriented documentation for manifest fields, CLI syntax, built-in variables, and the template cache. This should be more exhaustive than the narrative user-guide pages and must be verified against source code and tests.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `reference/manifest.md` documents manifest tables and fields, required versus optional values, defaults, constraints, and validation-relevant behavior.
- [x] #2 `reference/cli.md` documents current command syntax, important options, environment behavior, and exit-relevant limitations based on Clap definitions and command dispatch.
- [x] #3 `reference/builtins.md` documents all CLI-injected built-in variables and transform suffixes, including fallback values for user and email.
- [x] #4 `reference/cache.md` documents bundled template cache behavior, cache clearing via `zappy --clear`, and where cache behavior is implemented.
- [x] #5 Reference pages avoid tutorial flow and are structured for quick lookup with tables or concise subsections where helpful.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inspect manifest model, validation model, built-in constants, CLI Clap definitions, bundled cache code, and integration tests.
2. Populate `docs/book/src/reference/manifest.md` as a lookup reference for manifest tables and fields, including defaults, required fields, constraints, validation behavior, and unsupported hook options.
3. Populate `docs/book/src/reference/cli.md` with command syntax, aliases, global options, command-specific options, discovery-related behavior, and current limitations.
4. Populate `docs/book/src/reference/builtins.md` with CLI-injected built-ins and placeholder transform suffixes verified against `zappy-core` and `zappy-cli`.
5. Populate `docs/book/src/reference/cache.md` with bundled template cache behavior and `zappy --clear` behavior verified against `zappy-templates` and CLI dispatch.
6. Run `just book-check` after edits and update acceptance criteria/final summary.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Completed TASK-1.2. Reference pages are intentionally lookup-oriented and call out implemented limitations, including parsed-but-unenforced `validation_regex`, parsed-but-unsupported `shell = true`, and non-empty `create` being a stub. `just book-check` passes.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Populated the mdBook reference section for TASK-1.2.

Changes:
- Added `reference/manifest.md` with implemented manifest tables, required and optional fields, defaults, constraints, variable semantics, path rules, hook specs, and validation behavior.
- Added `reference/cli.md` with top-level syntax, global options, command aliases, subcommand option tables, discovery/environment behavior, and current exit/failure cases.
- Added `reference/builtins.md` with all CLI-injected built-ins, fallback value sources, reserved names, placeholder suffixes, and transform output examples.
- Added `reference/cache.md` with bundled template cache location model, marker behavior, cache preparation, `zappy --clear`, and explicit-directory behavior.

Verification:
- Verified content against `zappy-core`, `zappy-cli`, `zappy-templates`, `zappy-fs`, and integration tests.
- Ran `just book-check` successfully after the documentation changes.
<!-- SECTION:FINAL_SUMMARY:END -->
