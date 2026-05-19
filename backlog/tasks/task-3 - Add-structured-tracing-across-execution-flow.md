---
id: TASK-3
title: Add structured tracing across execution flow
status: Done
assignee:
  - OpenCode
created_date: '2026-05-19 21:33'
updated_date: '2026-05-19 21:51'
labels:
  - tracing
  - observability
milestone: m-1
dependencies: []
references:
  - Cargo.toml
  - crates/zappy-cli/src/tracing.rs
  - crates/zappy-cli/src/cli.rs
documentation:
  - AGENTS.md
priority: high
ordinal: 3000
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add tracing across Zappy's runtime execution flow so operators and developers can understand command dispatch, discovery, resolution, planning, generation, validation, hooks, and cache activity without stepping through the debugger. Tracing should use appropriate levels for each message, avoid repeating details already present in error types, and use tracing instead of `zappy-cli`'s diagnostics module for runtime reporting.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Runtime execution flow is instrumented with tracing across the relevant crates and command paths.
- [x] #2 Tracing messages use appropriate levels such as warn info debug and trace for the significance of the event.
- [x] #3 Tracing messages add useful execution context without duplicating information already contained in error types.
- [x] #4 Existing `zappy-cli` diagnostics output is preserved where already in use while missing tracing is added around it.
- [x] #5 The test suite passes after tracing changes and relevant tests cover the updated runtime behavior.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
Tracing initiative completed through four subtasks: `zappy-cli` tracing augmentation, `zappy-core` and `zappy-fs` instrumentation, `zappy-hooks` and `zappy-templates` instrumentation, and final verification. The final verification command was `just test`, which passed with 179 tests.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Parent acceptance criteria were adjusted to reflect the clarified requirement to preserve existing zappy-cli diagnostics output while adding missing tracing.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added structured tracing across the Zappy execution flow.

Completed scope:
- Added command, helper, and lifecycle tracing in `zappy-cli` without overwriting the current diagnostics-module user-facing output where it was already in use.
- Added structured tracing in `zappy-core` for manifest parsing, validation, variable resolution, and rendering transitions.
- Added structured tracing in `zappy-fs` for discovery, walking, planning, materialization, and overwrite/fallback decisions.
- Added structured tracing in `zappy-hooks` for phase execution, skip decisions, optional failures, command execution, and working-directory resolution.
- Added structured tracing in `zappy-templates` for cache resolution, readiness checks, reuse, installation, clearing, and extraction.

Final verification:
- `just test` passed with 179 tests.
- Tracing levels were reviewed and aligned with the intended severity: operator-relevant milestones at `info`, degraded/actionable situations at `warn`, decision/summaries at `debug`, and high-volume step details at `trace`.
- Tracing messages were written to add execution context without restating the full text already present in error types.

Residual observability gaps:
- Existing `zappy-cli` diagnostics-module output remains in place where already used, per clarified instruction, so runtime reporting is tracing-enhanced rather than tracing-only at the CLI boundary.
<!-- SECTION:FINAL_SUMMARY:END -->
