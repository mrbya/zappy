---
id: TASK-3.1
title: Migrate zappy-cli runtime reporting to tracing
status: Done
assignee:
  - OpenCode
created_date: '2026-05-19 21:33'
updated_date: '2026-05-19 21:43'
labels:
  - tracing
  - zappy-cli
milestone: m-1
dependencies: []
references:
  - crates/zappy-cli/src/cli.rs
  - crates/zappy-cli/src/commands/helpers.rs
  - crates/zappy-cli/src/commands
  - crates/zappy-cli/src/diagnostics.rs
  - crates/zappy-cli/src/tracing.rs
documentation:
  - AGENTS.md
  - tests/integration_tests.rs
parent_task_id: TASK-3
priority: high
ordinal: 3100
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Update the `zappy-cli` runtime path so command entrypoints, command helpers, and command dispatch emit tracing events instead of using the diagnostics module for runtime reporting. The outcome should make command progress, decisions, and warnings visible through tracing while keeping user-facing error details sourced from the underlying error types rather than duplicated into ad hoc strings.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `zappy-cli` runtime flow emits tracing across command progress warnings and failures while preserving the current diagnostics-module user-facing output.
- [x] #2 Tracing levels in `zappy-cli` distinguish high-level command lifecycle events from lower-level decision and data details.
- [x] #3 Messages emitted from `zappy-cli` do not restate information already present in the underlying error types.
- [x] #4 Tracing covers clear cache handling template selection generation validation dry-run and stubbed create behavior where relevant.
- [x] #5 CLI and integration tests are updated to match any observable tracing-sensitive behavior and continue to pass.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inspect `zappy-cli` entrypoints, helpers, and command handlers to identify runtime paths that still lack tracing coverage.
2. Add tracing around command lifecycle events, decisions, warnings, and failures while preserving the current diagnostics-module user-facing output.
3. Choose levels based on significance: warn for actionable degraded situations, info for command milestones, debug for decisions/summaries, and trace for noisy helper-level events.
4. Avoid duplicating information already contained in error types by tracing surrounding context rather than repeating full error text in the tracing message.
5. Run focused CLI and integration verification, then finalize the task record.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Acceptance criteria were updated during execution to reflect the clarified requirement to preserve existing diagnostics output while adding tracing only where missing.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added missing `zappy-cli` tracing while preserving the existing diagnostics output.

Scope completed:
- Added command-level tracing for list, info, init, create, new, and validate.
- Added helper-level tracing around bundled template preparation, discovery, selection, generation, hook execution, and skeleton creation.
- Added warn-level failure context without restating full error text already carried by the error types and diagnostics output.
- Kept existing diagnostics-module user-facing behavior intact per updated instruction.

Verification:
- `cargo nextest run -p zappy-cli --all-features` passed.
- `cargo test --test integration_tests` passed.
<!-- SECTION:FINAL_SUMMARY:END -->
