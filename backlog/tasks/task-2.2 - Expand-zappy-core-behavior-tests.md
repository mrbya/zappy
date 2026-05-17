---
id: TASK-2.2
title: Expand zappy-core behavior tests
status: Done
assignee:
  - OpenCode
created_date: '2026-05-17 10:45'
updated_date: '2026-05-17 10:55'
labels:
  - testing
  - coverage
  - zappy-core
milestone: m-0
dependencies:
  - TASK-2.1
references:
  - crates/zappy-core/src/tests.rs
  - crates/zappy-core/src/manifest.rs
  - crates/zappy-core/src/resolution.rs
  - crates/zappy-core/src/render.rs
  - crates/zappy-core/src/condition.rs
  - crates/zappy-core/src/transform.rs
documentation:
  - AGENTS.md
  - docs/book/src/reference/manifest.md
  - docs/book/src/reference/built-ins.md
parent_task_id: TASK-2
priority: high
ordinal: 2200
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add meaningful crate-level tests for `zappy-core`, focusing on manifest parsing, variable resolution, transforms, rendering, condition evaluation, built-ins, validation models, and generation-plan model behavior. Tests should live in the crate's dedicated test module structure rather than repository-level integration tests unless they require full CLI/filesystem behavior.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Tests cover successful and failing manifest parsing paths, including invalid or unsupported manifest shapes.
- [x] #2 Tests cover variable precedence, required/default behavior, built-ins, and transform-specific placeholder resolution.
- [x] #3 Tests cover rendering behavior for normal placeholders, transformed placeholders, missing values, and non-rendered content boundaries.
- [x] #4 Tests cover condition evaluation truthy/falsey behavior and invalid condition references where implemented.
- [x] #5 Tests are placed in `crates/zappy-core` test modules and do not require filesystem writes or process execution.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Review existing `zappy-core` tests and source to avoid duplicating already-covered behavior.
2. Add focused tests in `crates/zappy-core/src/tests.rs` for meaningful gaps from TASK-2.1: built-in placeholder/replacement behavior, invalid rendered relative paths, template ID/source-root validation edge cases, condition behavior, validation setup/teardown parsing, hook/env parsing, and variable resolution edge cases.
3. Keep tests pure core tests with no filesystem writes or process execution.
4. Run focused `zappy-core` tests, then update this task with results and checked acceptance criteria.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
`zappy-core` package coverage now exceeds 90% line coverage in focused coverage run. Remaining uncovered lines are mostly lower-value manifest file-read and validation edge branches that can be revisited if the final workspace coverage remains below target.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Expanded `zappy-core` behavior coverage with meaningful crate-level tests.

Added coverage for:
- Manifest parsing failures for invalid template IDs, invalid source roots, empty path-list entries, and invalid hook working directories.
- Validation config parsing for setup and teardown hooks.
- Hook manifest fields including env, when, optional, shell, args, and working_dir.
- Built-in helper behavior including filtering unknown names and generating transformed placeholders.
- Built-in injection and unknown built-in resolution errors.
- Condition evaluation for true booleans versus false, string, integer, and missing values.
- Rendering behavior for ordered replacements, preserved missing placeholders, empty rendered paths, absolute rendered paths, and parent components.

Verification:
- `cargo fmt --all` completed with existing stable rustfmt warnings for nightly-only options.
- `cargo nextest run -p zappy-core --all-features` passed: 53 tests.
- `cargo llvm-cov nextest -p zappy-core --all-features --summary-only` passed with total line coverage at 91.02%.
<!-- SECTION:FINAL_SUMMARY:END -->
