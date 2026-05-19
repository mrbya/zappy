---
id: TASK-3.4
title: Verify tracing behavior and close the tracing initiative
status: Done
assignee:
  - OpenCode
created_date: '2026-05-19 21:34'
updated_date: '2026-05-19 21:51'
labels:
  - tracing
  - verification
milestone: m-1
dependencies:
  - TASK-3.2
  - TASK-3.3
references:
  - justfile
  - Cargo.toml
documentation:
  - AGENTS.md
parent_task_id: TASK-3
priority: high
ordinal: 3400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
After tracing instrumentation is added across the relevant crates, run the full verification gates and review the resulting behavior to ensure the logs are useful, correctly leveled, and free from duplicated error information. Close the tracing initiative only after the final sweep confirms the new runtime reporting approach is consistent.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The full test suite passes after tracing changes.
- [x] #2 A final review confirms tracing levels are appropriate across the instrumented execution flow.
- [x] #3 A final review confirms tracing messages do not duplicate information already provided by error types.
- [x] #4 Parent task `TASK-3` is updated with commands run final status and any residual observability gaps.
- [x] #5 All tracing subtasks are complete before the parent is closed.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Run the full workspace test suite to verify the tracing changes did not regress behavior.
2. Review the instrumented tracing sites across `zappy-cli`, `zappy-core`, `zappy-fs`, `zappy-hooks`, and `zappy-templates` to confirm level selection and context quality.
3. Confirm that tracing messages add surrounding execution context without restating full error text already available from error types.
4. Update the parent task to reflect the final clarified scope and final verification results, then close both this task and the parent task.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Parent task needs acceptance criteria adjusted to the clarified scope before closure: tracing added broadly while existing zappy-cli diagnostics output remains in place where already used.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Verified and closed the tracing initiative.

Verification:
- `just test` passed with 179 tests.
- Final review of tracing sites confirmed appropriate use of `info`, `warn`, `debug`, and `trace` across command flow, core generation, filesystem planning/materialization, hook execution, and bundled-template cache handling.
- Final review confirmed the tracing messages add execution context and branch information without duplicating the full text already present in error types.

Clarified final scope:
- `zappy-cli` preserved its current diagnostics-module user-facing output where already in use.
- Missing tracing was added around that behavior instead of replacing it.
- Internal crates gained structured tracing for their execution flow without changing user-facing output.
<!-- SECTION:FINAL_SUMMARY:END -->
