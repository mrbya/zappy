---
id: TASK-3.2
title: Instrument zappy-core and zappy-fs execution flow
status: Done
assignee:
  - OpenCode
created_date: '2026-05-19 21:33'
updated_date: '2026-05-19 21:47'
labels:
  - tracing
  - zappy-core
  - zappy-fs
milestone: m-1
dependencies:
  - TASK-3.1
references:
  - crates/zappy-core/src
  - crates/zappy-fs/src/discover.rs
  - crates/zappy-fs/src/plan.rs
  - crates/zappy-fs/src/materialize.rs
  - crates/zappy-fs/src/walk.rs
documentation:
  - AGENTS.md
  - docs/book/src/reference/manifest.md
  - docs/book/src/templates/discovery.md
parent_task_id: TASK-3
priority: high
ordinal: 3200
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add structured tracing to the core and filesystem crates so manifest parsing, variable resolution, path evaluation, discovery, traversal, plan building, and materialization expose meaningful execution state. The goal is to make the generation pipeline observable without flooding logs or repeating error-display text that the error types already provide.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `zappy-core` traces significant parsing resolution rendering and validation transitions with useful context fields.
- [x] #2 `zappy-fs` traces search-path resolution discovery traversal planning and materialization transitions with useful context fields.
- [x] #3 Low-level events that may be numerous use debug or trace rather than info unless they are operator-relevant milestones.
- [x] #4 Error-path tracing adds surrounding context but does not duplicate the textual content already provided by the error types.
- [x] #5 Crate tests cover the affected tracing-sensitive behavior where appropriate and the workspace tests continue to pass.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inspect `zappy-core` and `zappy-fs` modules to find execution-flow gaps where tracing is missing or too coarse.
2. Add tracing to significant lifecycle transitions in manifest parsing, variable resolution, render/path validation, discovery, traversal, plan building, and materialization.
3. Choose levels carefully: info for operator-relevant milestones, debug for decisions and summaries, and trace for high-volume step-level work.
4. Avoid duplicating error-display text by tracing surrounding context fields rather than reconstructing the same error messages already present in error types.
5. Run focused crate tests and then broader verification before finalizing the task.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
No user-facing output changes were required for this task because the added tracing is internal to core and filesystem execution flow.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Instrumented the core generation pipeline and filesystem execution flow with structured tracing.

Scope completed:
- Added manifest parsing and validation tracing in `zappy-core`.
- Added variable resolution, source precedence, and render-data summary tracing in `zappy-core`.
- Added text and relative-path rendering trace points in `zappy-core`.
- Added directory walking, source-entry discovery, plan-building, skip reasoning, conflict detection, non-UTF8 fallback, and materialization tracing in `zappy-fs`.
- Kept high-volume entry-level events at `trace`, summaries/decisions at `debug`, and operator-relevant lifecycle milestones at `info`.
- Avoided duplicating error-display text by tracing context fields and state transitions rather than restating the same error strings.

Verification:
- `cargo nextest run -p zappy-core -p zappy-fs --all-features` passed.
- `cargo test --test integration_tests` passed.
<!-- SECTION:FINAL_SUMMARY:END -->
