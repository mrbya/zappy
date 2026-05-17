---
id: TASK-2.1
title: Establish coverage baseline and test gap map
status: Done
assignee:
  - OpenCode
created_date: '2026-05-17 10:45'
updated_date: '2026-05-17 10:52'
labels:
  - testing
  - coverage
  - analysis
milestone: m-0
dependencies: []
references:
  - justfile
  - Cargo.toml
documentation:
  - AGENTS.md
parent_task_id: TASK-2
priority: high
ordinal: 2100
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Run the workspace coverage tooling and inspect the current test suite to identify the highest-value gaps before adding tests. The outcome should guide the other coverage subtasks toward meaningful behavior coverage and away from shallow line execution.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 A current workspace coverage percentage is recorded in the task notes or final summary.
- [x] #2 Coverage gaps are grouped by crate and by behavior area.
- [x] #3 The gap map identifies meaningful edge cases and error paths worth testing, not just uncovered line ranges.
- [x] #4 Recommended test locations follow the project rule: crate tests inside crates and integration tests under `tests/`.
- [x] #5 The coverage command used for measurement is documented.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inspect `justfile` and workspace metadata to identify the canonical coverage command and test commands.
2. Run the project's coverage tooling to capture the current workspace baseline percentage and any per-file/per-crate report details available from the tool.
3. Inspect existing test organization in crate test modules and repository-level integration tests to understand current coverage shape.
4. Summarize meaningful coverage gaps by crate and behavior area, with recommended test locations that follow the project rule: crate tests inside crates and integration tests under `tests/`.
5. Record the coverage command, baseline, gap map, and recommendations in this task, then mark acceptance criteria complete.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Ran coverage baseline and inspected existing test/source organization. No code changes were made for this baseline task.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Established the workspace coverage baseline and mapped meaningful test gaps for the >90% coverage initiative.

Baseline:
- Command: `just test-cov --summary-only`.
- Result: 102 tests passed.
- Total line coverage: 83.01%.
- Total region coverage: 85.56%.
- Function execution coverage: 80.11%.

Gap map highlights:
- `zappy-cli`: command helpers, create/validate/new/list edge paths, dispatch failures, and built-in injection boundaries.
- `zappy-core`: invalid rendered paths, template field validation, built-in placeholder variants, conditions, and validation setup/teardown parsing.
- `zappy-fs`: discovery search-path behavior, walking, materialization failure/skip summaries, plan building, and filesystem edge cases.
- `zappy-hooks`: optional failures, shell rejection, non-zero exits, rendered args/env/working dirs, and conditional execution.
- `zappy-templates`: cache reuse/clearing/reinstall behavior and bundled manifest availability.
- `tests/`: end-to-end workflow gaps for missing templates, unknown IDs, output conflicts, required vars, `--no-hooks`, validation failures, and unsupported non-empty `create`.

Recommended locations follow the project rule: crate behavior in crate test modules, binary workflows under `tests/`.
<!-- SECTION:FINAL_SUMMARY:END -->
