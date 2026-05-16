---
id: TASK-1.5
title: Document template authoring concepts
status: Done
assignee:
  - OpenCode
created_date: '2026-05-16 16:19'
updated_date: '2026-05-16 16:39'
labels:
  - documentation
  - mdbook
  - templates
milestone: Documentation
dependencies: []
references:
  - docs/book/src/templates/authoring.md
  - docs/book/src/templates/manifest.md
  - docs/book/src/templates/variables.md
  - docs/book/src/templates/hooks.md
  - docs/book/src/templates/validation.md
documentation:
  - AGENTS.md
  - crates/zappy-core/src
  - crates/zappy-fs/src
  - crates/zappy-hooks/src
  - tests/fixtures/templates/test_template
parent_task_id: TASK-1
priority: high
ordinal: 1300
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Populate the authoring-oriented template pages for people creating `zappy.toml` manifests and template files. Cover manifest basics, variables and placeholder transforms, hooks, and validation using behavior implemented in `zappy-core`, `zappy-fs`, `zappy-hooks`, and CLI validation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `templates/authoring.md` walks through creating or organizing a template in terms that match the current skeleton and manifest behavior.
- [x] #2 `templates/manifest.md` explains the core manifest sections needed by template authors without duplicating the exhaustive reference page.
- [x] #3 `templates/variables.md` documents required variables, defaults, built-ins, placeholder syntax, and transform behavior such as snake, kebab, and pascal variants.
- [x] #4 `templates/hooks.md` describes supported hook execution and explicitly notes unsupported shell hooks if still rejected by the implementation.
- [x] #5 `templates/validation.md` explains the validation block, generation into a temporary directory, validation steps, and `--no-hooks` behavior accurately.
- [x] #6 Examples are either verified against fixtures/source or clearly marked as illustrative when they are not executable.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inspect template skeleton initialization, manifest parsing/types, variable resolution and transforms, rendering, path planning, hook execution, validation command behavior, bundled manifests, and fixture manifests.
2. Populate `docs/book/src/templates/authoring.md` with an end-to-end authoring workflow from skeleton creation to generation and validation.
3. Populate `docs/book/src/templates/manifest.md` with practical manifest basics for authors, keeping exhaustive details in `reference/manifest.md`.
4. Populate `docs/book/src/templates/variables.md` with variable definitions, defaults, required/required_when behavior, built-ins, placeholder syntax, and transform suffixes.
5. Populate `docs/book/src/templates/hooks.md` with generation hook behavior, hook fields, variable rendering inside command args, optional hooks, conditions, working directories, and unsupported `shell = true` hooks.
6. Populate `docs/book/src/templates/validation.md` with validation config behavior, temp output generation, validation variables, setup/steps/teardown, `--keep-temp`, and `--no-hooks` scope.
7. Run `just book-check` after edits and update acceptance criteria/final summary.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Completed TASK-1.5. Authoring docs cover implemented skeleton creation, variable transforms, placeholder rendering, path rules, hook execution, validation flow, and limitations such as no interactive prompting, non-empty `create` stub behavior, unsupported shell hooks, and parsed-but-unenforced `validation_regex`. `just book-check` passes.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Populated template authoring documentation for TASK-1.5.

Changes:
- Added `templates/authoring.md` with a skeleton-first authoring workflow, template source layout, variables/placeholders, path controls, dry-run preview, validation setup, and current authoring limits.
- Added `templates/manifest.md` with practical manifest basics for metadata, variables, paths, conditionals, hooks, and validation, deferring exhaustive details to the reference page.
- Added `templates/variables.md` with variable naming rules, CLI value parsing, variable fields, resolution behavior, placeholders, transforms, and built-in placeholder links.
- Added `templates/hooks.md` with generation hook arrays, hook fields, conditions, rendering behavior, required versus optional hook behavior, unsupported shell hooks, and validation hook placement.
- Added `templates/validation.md` with validation block structure, validation flow, output directory behavior, validation variables, setup/steps/teardown hooks, `--keep-temp`, and `--no-hooks` scope.

Verification:
- Verified authoring behavior against `zappy-core`, `zappy-fs`, `zappy-hooks`, `zappy-cli` validation behavior, and the fixture/bundled manifests.
- Ran `just book-check` successfully after the documentation changes.
<!-- SECTION:FINAL_SUMMARY:END -->
