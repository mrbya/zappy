---
id: TASK-2.7
title: Close coverage gap to above 90%
status: Done
assignee:
  - OpenCode
created_date: '2026-05-17 10:45'
updated_date: '2026-05-17 11:11'
labels:
  - testing
  - coverage
  - verification
milestone: m-0
dependencies:
  - TASK-2.2
  - TASK-2.3
  - TASK-2.4
  - TASK-2.5
  - TASK-2.6
references:
  - justfile
  - Cargo.toml
documentation:
  - AGENTS.md
parent_task_id: TASK-2
priority: high
ordinal: 2700
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
After the crate and integration coverage subtasks are implemented, run the full test and coverage gates, address any remaining meaningful gaps, and close the parent coverage initiative only when the workspace exceeds 90% coverage.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The workspace coverage report is above 90%.
- [x] #2 Remaining uncovered areas are reviewed and either covered with meaningful tests or documented as intentionally low-value or impractical to test.
- [x] #3 `just test` passes after coverage additions.
- [x] #4 The project coverage command passes or produces a successful report consistent with the >90% target.
- [x] #5 Parent task `TASK-2` is updated with final coverage percentage, commands run, residual risks, and checked acceptance criteria.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Run the full workspace test suite with `just test`.
2. Run the workspace coverage command with `just test-cov --summary-only` to measure the final coverage percentage.
3. If coverage remains below 90%, inspect the largest remaining meaningful gaps and add targeted tests in the correct crate or `tests/` location before rerunning verification.
4. If coverage exceeds 90%, record the final percentage and commands run, check acceptance criteria, close this task, and update parent `TASK-2`.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
First final coverage run reached 90.52% region coverage but only 88.85% line coverage. Added targeted meaningful tests and reran coverage to reach 90.51% line coverage.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Closed the coverage gap above the requested >90% target.

Final verification:
- `just test` passed with 174 tests.
- `just test-cov --summary-only` passed with 174 tests.
- Workspace line coverage: 90.51%.
- Workspace region coverage: 91.79%.
- Workspace function execution coverage: 86.36%.

Additional targeted coverage added in this final pass:
- Core manifest load/read failures, conditional validation, source-root validation, template ID validation, validation hook validation, built-in name helpers, CLI override parsing aliases, and invalid override names.
- Filesystem environment-derived discovery paths, create-directory failure, and text/binary parent-directory materialization failures.
- Integration validation workflow where `validate --no-hooks` skips a failing generation hook while still running validation setup/steps/teardown.

Residual uncovered areas are mostly OS/global-state failure paths that are impractical to test portably without brittle permission or environment manipulation.
<!-- SECTION:FINAL_SUMMARY:END -->
