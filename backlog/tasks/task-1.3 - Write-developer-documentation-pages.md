---
id: TASK-1.3
title: Write developer documentation pages
status: Done
assignee:
  - OpenCode
created_date: '2026-05-16 16:18'
updated_date: '2026-05-16 16:48'
labels:
  - documentation
  - mdbook
  - development
milestone: Documentation
dependencies: []
references:
  - docs/book/src/development/contributing.md
  - docs/book/src/development/architecture.md
  - docs/book/src/development/crate-map.md
documentation:
  - AGENTS.md
  - justfile
  - Cargo.toml
  - crates
  - tests
parent_task_id: TASK-1
priority: medium
ordinal: 1600
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Populate the development section for contributors who need to understand the repository layout, crate responsibilities, local workflows, checks, and contribution expectations. Use `AGENTS.md`, crate-level code, tests, and `justfile` as the source of truth.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `development/contributing.md` documents local setup, formatting, linting, tests, template validation, pre-commit expectations, and commit-message expectations.
- [x] #2 `development/architecture.md` explains the high-level architecture, core data flow, and separation between CLI, core, filesystem, hooks, templates, and reserved adapter crates.
- [x] #3 `development/crate-map.md` concisely documents each crate's current responsibility and where key behavior lives.
- [x] #4 The development docs clearly identify source-of-truth commands such as `just fmt`, `just check -- -D warnings`, `just test`, `just doctest`, and `just ci`.
- [x] #5 The pages avoid duplicating stale aspirational docs without validating them against current code.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Verify repository layout and crate responsibilities against `AGENTS.md`, `Cargo.toml`, crate `src/lib.rs` files, CLI command modules, and tests.
2. Populate `docs/book/src/development/contributing.md` with setup, `just` workflows, formatting, linting, tests, template validation, mdBook checks, pre-commit expectations, and commit-message expectations.
3. Populate `docs/book/src/development/architecture.md` with current high-level architecture, data flow from CLI to core/fs/hooks/templates, and known implementation boundaries/limits.
4. Populate `docs/book/src/development/crate-map.md` with concise crate-by-crate ownership, key modules, and where to make common changes.
5. Run `just book-check` to verify the mdBook builds.
6. Update TASK-1.3 acceptance criteria, implementation notes, and final summary.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Completed TASK-1.3. Development docs are based on current repo files and explicitly avoid stale docs such as empty `docs/manifest.md`, empty `docs/templates.md`, and aspirational `docs/dev/implementation_plan.md`. `just book-check` passes.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Populated the mdBook development section for TASK-1.3.

Changes:
- Added `development/contributing.md` with requirements, first-time setup, common `just` commands, focused test commands, CI/pre-commit gates, commit-message expectations, and documentation workflow notes.
- Added `development/architecture.md` with workspace layout, crate boundaries, command flow, generation flow, validation flow, discovery flow, core design constraints, and current implementation limits.
- Added `development/crate-map.md` with crate-by-crate ownership, key files, where common changes belong, and test/fixture locations.

Verification:
- Verified content against `AGENTS.md`, `justfile`, `Cargo.toml`, `.pre-commit-config.yaml`, `rust-toolchain.toml`, crate `src/lib.rs` files, command modules, and integration tests.
- Ran `just book-check` successfully.
<!-- SECTION:FINAL_SUMMARY:END -->
