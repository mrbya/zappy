---
id: TASK-2.3
title: Expand zappy-fs behavior tests
status: Done
assignee:
  - OpenCode
created_date: '2026-05-17 10:45'
updated_date: '2026-05-17 11:00'
labels:
  - testing
  - coverage
  - zappy-fs
milestone: m-0
dependencies:
  - TASK-2.1
references:
  - crates/zappy-fs/src/tests.rs
  - crates/zappy-fs/src/discover.rs
  - crates/zappy-fs/src/walk.rs
  - crates/zappy-fs/src/plan.rs
  - crates/zappy-fs/src/materialize.rs
  - crates/zappy-fs/src/init.rs
  - tests/fixtures/templates/test_template
documentation:
  - AGENTS.md
  - docs/book/src/templates/discovery.md
  - docs/book/src/reference/cache.md
parent_task_id: TASK-2
priority: high
ordinal: 2300
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add meaningful crate-level tests for `zappy-fs`, focusing on template discovery, deterministic traversal, file classification, plan building, skeleton initialization, and materialization behavior. Tests should exercise realistic temporary filesystem layouts and stay within the crate's dedicated test modules.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Tests cover discovery search order and explicit directory behavior, including missing explicit directories and single-template directories.
- [x] #2 Tests cover duplicate template IDs and shadowed template handling.
- [x] #3 Tests cover deterministic traversal, excludes, conditional paths, binary/text classification, symlink handling, and conflict warnings.
- [x] #4 Tests cover materialization behavior for dry-run-equivalent planning, forced overwrites, rendered paths/content, and binary copies.
- [x] #5 Tests cover template skeleton initialization for valid output and existing-path error cases.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Review existing `zappy-fs` tests and source modules to target uncovered meaningful behavior without duplicating current assertions.
2. Add focused crate tests in `crates/zappy-fs/src/tests.rs` for discovery search-path composition, optional/missing search paths, invalid manifest loading, deterministic walk ordering, symlink and nested entry classification, plan-building skips/conditionals/binary classification/conflicts, materialization skip/copy failure/write parent creation, and init edge behavior where not already covered.
3. Keep all tests in `crates/zappy-fs` using temporary filesystem layouts.
4. Run focused `zappy-fs` tests and package coverage summary, then update task notes and acceptance criteria.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Focused `zappy-fs` package line coverage improved to 84.80%. Remaining low-value gaps are mostly hard-to-force filesystem error branches in discovery/walk and private helper fallback paths.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Expanded `zappy-fs` behavior coverage with realistic temporary filesystem tests.

Added coverage for:
- Explicit search paths overriding bundled discovery, implicit path composition, bundled fallback discovery, optional missing/file paths, required file-path errors, invalid manifest discovery, and duplicate/shadowed handling.
- Deterministic source traversal, nested directories, file classification, Unix symlink classification, and missing source-root errors.
- Generation planning for rendered paths/content, excludes, conditionals, symlinks, binary extension rules, non-UTF-8 copy fallback, and destination conflict warnings.
- Materialization summaries, skip operations, parent directory creation, binary copy failures, existing binary conflicts, directory write failures, forced overwrites, and binary copies.
- Template skeleton initialization with derived IDs and custom metadata in addition to existing valid/existing-path coverage.

Verification:
- `cargo fmt --all` completed with existing stable rustfmt warnings for nightly-only options.
- `cargo nextest run -p zappy-fs --all-features` passed: 28 tests.
- `cargo llvm-cov nextest -p zappy-fs --all-features --summary-only` passed with total line coverage at 84.80%.
<!-- SECTION:FINAL_SUMMARY:END -->
