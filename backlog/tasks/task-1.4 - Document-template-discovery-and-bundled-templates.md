---
id: TASK-1.4
title: Document template discovery and bundled templates
status: Done
assignee:
  - OpenCode
created_date: '2026-05-16 16:19'
updated_date: '2026-05-16 16:38'
labels:
  - documentation
  - mdbook
  - templates
milestone: Documentation
dependencies: []
references:
  - docs/book/src/templates/overview.md
  - docs/book/src/templates/discovery.md
  - docs/book/src/templates/builtins.md
documentation:
  - AGENTS.md
  - crates/zappy-fs/src
  - crates/zappy-templates/src
  - crates/zappy-templates/templates
parent_task_id: TASK-1
priority: high
ordinal: 1200
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Populate the template overview, discovery, and built-in template pages so users understand where Zappy looks for templates, how duplicate IDs and bundled cache extraction work, and what bundled templates are available. Ground the docs in `zappy-fs` discovery behavior and `zappy-templates` bundled extraction rather than assumptions.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `templates/overview.md` explains the template model and links readers to discovery, authoring, variables, hooks, and validation pages.
- [x] #2 `templates/discovery.md` documents the actual search order, explicit `--templates-dir` behavior, environment/config paths, single-template directories, duplicate ID handling, and bundled fallback behavior.
- [x] #3 `templates/builtins.md` lists bundled templates or explains how to inspect them, with content verified against `crates/zappy-templates/templates`.
- [x] #4 The pages distinguish implemented discovery behavior from any planned or unsupported behavior.
- [x] #5 The updated pages build cleanly in the mdBook.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inspect `zappy-fs` discovery code, discovery tests, `zappy-templates` cache extraction code, and bundled template manifests.
2. Populate `docs/book/src/templates/overview.md` with the template model and links into discovery, authoring, manifest, variables, hooks, validation, and reference pages.
3. Populate `docs/book/src/templates/discovery.md` with explicit `--templates-dir` behavior, default discovery order, environment/config paths, single-template directory support, duplicate/shadowed handling, and bundled fallback/cache behavior.
4. Populate `docs/book/src/templates/builtins.md` with the current bundled template IDs, languages, descriptions, required/common variables, optional conditionals, and validation notes verified against `crates/zappy-templates/templates`.
5. Keep planned/unsupported behavior clearly separated from implemented behavior.
6. Run `just book-check` after edits and update acceptance criteria/final summary.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Completed TASK-1.4. Discovery docs document the implemented first-hit-wins duplicate behavior, explicit-directory bundled-template disabling, required `ZAPPY_TEMPLATES_DIR`, optional implicit paths, and bundled cache fallback. `just book-check` passes.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Populated template discovery and bundled-template documentation for TASK-1.4.

Changes:
- Added `templates/overview.md` with the template directory model, generation flow, rendering model, links to related template topics, and current implementation limits.
- Added `templates/discovery.md` with explicit-directory behavior, default search order, required versus optional paths, bundled fallback, single-template directory support, duplicate/shadowed ID behavior, and discovery error cases.
- Added `templates/builtins.md` with the current bundled template list, common variables, conditional files, hooks, validation notes, and inspection guidance.

Verification:
- Verified discovery details against `zappy-fs/src/discover.rs` and discovery tests.
- Verified cache/bundled-template details against `zappy-templates/src/cache.rs` and bundled manifests under `crates/zappy-templates/templates`.
- Ran `just book-check` successfully after the documentation changes.
<!-- SECTION:FINAL_SUMMARY:END -->
