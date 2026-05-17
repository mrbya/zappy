---
id: TASK-2
title: Raise meaningful test coverage above 90%
status: Done
assignee:
  - OpenCode
created_date: '2026-05-17 10:44'
updated_date: '2026-05-17 11:12'
labels:
  - testing
  - coverage
milestone: m-0
dependencies: []
references:
  - justfile
  - Cargo.toml
  - tests/integration_tests.rs
documentation:
  - AGENTS.md
priority: high
ordinal: 2000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Increase Zappy's automated test coverage above 90% with behavior-focused tests rather than shallow line hits. Keep unit and module tests in each crate's dedicated test modules, and keep end-to-end/integration coverage under the repository-level `tests/` directory. This is an umbrella task for crate-specific and integration-test subtasks that can be worked independently while preserving the same coverage goal.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Coverage report shows more than 90% coverage for the workspace using the project's coverage tooling.
- [x] #2 Added tests exercise meaningful behavior, edge cases, and error paths instead of only executing lines.
- [x] #3 Crate-level tests remain in dedicated modules inside the relevant crates.
- [x] #4 Integration and CLI workflow tests remain under `tests/`.
- [x] #5 The test suite and coverage command complete successfully after all subtasks are done.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Coverage initiative completed through seven subtasks: baseline/gap map, zappy-core tests, zappy-fs tests, zappy-cli tests, hooks/templates tests, integration workflow tests, and final verification. Final verification used `just test` and `just test-cov --summary-only`; both passed, and workspace line coverage reached 90.51%.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Closed parent coverage initiative after all seven subtasks reached Done and final line coverage exceeded 90%.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Raised Zappy's meaningful test coverage above 90%.

Final result:
- Workspace line coverage: 90.51%.
- Workspace region coverage: 91.79%.
- `just test` passed with 174 tests.
- `just test-cov --summary-only` passed with 174 tests.

Scope completed:
- Added behavior-focused tests inside crate test modules for `zappy-core`, `zappy-fs`, `zappy-cli`, `zappy-hooks`, and `zappy-templates`.
- Added end-to-end CLI workflow tests under `tests/`.
- Preserved the requested test organization: crate behavior stayed in crate modules and integration workflows stayed under `tests/`.
- Prioritized meaningful behaviors: manifest errors, variable resolution, rendering, discovery, planning, materialization, hooks, cache behavior, command parsing, CLI workflow failures, and validation workflows.

Residual risks:
- Remaining uncovered areas are mainly portable-hostile OS/global-state failure paths, such as cache-dir resolution failures, filesystem permission failures, directory-entry read errors, git config fallbacks, and tempdir failure branches.
<!-- SECTION:FINAL_SUMMARY:END -->
