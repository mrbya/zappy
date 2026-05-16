---
id: TASK-1
title: Document the Zappy mdBook
status: To Do
assignee: []
created_date: '2026-05-16 16:18'
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
- [ ] #1 All pages listed in `docs/book/src/SUMMARY.md` are populated with useful project documentation instead of heading-only stubs.
- [ ] #2 Documentation describes implemented behavior and clearly calls out currently unsupported or stubbed behavior where relevant.
- [ ] #3 The mdBook builds successfully after the documentation work is complete.
- [ ] #4 The completed book has coherent navigation between user guide, template authoring, reference, migration, and development sections.
<!-- AC:END -->
