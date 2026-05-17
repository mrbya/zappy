---
id: TASK-1
title: Document the Zappy mdBook
status: Done
assignee:
  - OpenCode
created_date: '2026-05-16 16:18'
updated_date: '2026-05-17 10:41'
labels:
  - documentation
  - mdbook
milestone: Documentation
dependencies: []
references:
  - docs/book/book.toml
  - docs/book/src/SUMMARY.md
documentation:
  - AGENTS.md
  - README.md
  - justfile
priority: high
ordinal: 1000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Complete the empty mdBook documentation under `docs/book` so users, template authors, migrators, and contributors can understand and use the current Rust implementation of Zappy. This is an umbrella task for the focused documentation subtasks; each subtask should verify content against the implemented crates and tests rather than stale prose in older docs.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 All pages listed in `docs/book/src/SUMMARY.md` are populated with useful project documentation instead of heading-only stubs.
- [x] #2 Documentation describes implemented behavior and clearly calls out currently unsupported or stubbed behavior where relevant.
- [x] #3 The mdBook builds successfully after the documentation work is complete.
- [x] #4 The completed book has coherent navigation between user guide, template authoring, reference, migration, and development sections.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Documentation was completed as six focused subtasks: user guide, reference docs, developer docs, template discovery/bundled templates, template authoring concepts, and Lua Zappy migration. Each subtask has its own plan, acceptance criteria, notes, and final summary in Backlog. Parent completion is verified by all subtasks being Done and `just book-check` passing.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Closed parent documentation task after confirming all six subtasks are Done and `just book-check` passes.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed the Zappy mdBook documentation initiative.

Completed subtasks:
- TASK-1.1: User guide pages for introduction, installation, quick start, and commands.
- TASK-1.2: Reference pages for manifest, CLI, built-ins, and cache behavior.
- TASK-1.3: Developer pages for contributing, architecture, and crate map.
- TASK-1.4: Template overview, discovery, and bundled-template documentation.
- TASK-1.5: Template authoring pages for manifest basics, variables, hooks, and validation.
- TASK-1.6: Migration guide from the old Lua implementation.

Verification:
- Each subtask was completed against current source, tests, and relevant legacy Lua implementation context where applicable.
- Ran `just book-check` successfully after completion.
<!-- SECTION:FINAL_SUMMARY:END -->
