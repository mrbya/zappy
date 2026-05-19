---
id: TASK-3.3
title: Instrument hooks and bundled-template cache flow
status: Done
assignee:
  - OpenCode
created_date: '2026-05-19 21:33'
updated_date: '2026-05-19 21:49'
labels:
  - tracing
  - zappy-hooks
  - zappy-templates
milestone: m-1
dependencies:
  - TASK-3.1
references:
  - crates/zappy-hooks/src/hooks.rs
  - crates/zappy-templates/src/cache.rs
documentation:
  - AGENTS.md
  - docs/book/src/templates/hooks.md
  - docs/book/src/reference/cache.md
parent_task_id: TASK-3
priority: medium
ordinal: 3300
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add tracing to hook execution and bundled-template cache operations so hook phase transitions, skip decisions, optional failures, cache extraction, cache reuse, and cache clearing are observable. The resulting traces should help explain execution flow and outcomes without echoing full error-display text or turning expected noisy operations into overly prominent logs.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Hook execution emits tracing around phase boundaries command decisions skips optional failures and success summaries with appropriate levels.
- [x] #2 Bundled-template cache operations emit tracing around cache resolution extraction reuse and clearing with appropriate levels.
- [x] #3 Trace messages avoid repeating error payload text already available from hook and template error types.
- [x] #4 Tests for hooks and template cache behavior continue to pass and are updated if runtime tracing changes observable output expectations.
- [x] #5 The resulting logs help distinguish normal expected activity from warnings and actionable failures.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inspect existing `zappy-hooks` and `zappy-templates` tracing to identify missing lifecycle coverage.
2. Add tracing around hook phase entry, conditional skips, optional failures, command execution outcomes, cache resolution, extraction, reuse, and clear operations.
3. Use info for operator-relevant milestones, debug for summaries and branch decisions, and trace for noisier per-entry work.
4. Avoid repeating error-display text by tracing context fields rather than reconstructing the same error messages already carried by error types.
5. Run focused crate tests and then broader verification before finalizing the task.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
No integration expectation updates were needed because the added tracing is internal to hook and cache flow and does not alter the current user-facing diagnostics output.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Instrumented hook execution and bundled-template cache flow with structured tracing.

Scope completed:
- Added hook-phase tracing around execution start, conditional skips, optional failures, successful execution, non-zero exits, and summary completion.
- Added rendered command-input and working-directory trace points in `zappy-hooks`.
- Added cache lifecycle tracing in `zappy-templates` for path resolution, readiness checks, reuse, installation, clearing, marker writes, and recursive extraction.
- Used `debug` and `trace` for normal lifecycle detail, and `warn` only for degraded situations such as optional hook failures or non-zero command exits.
- Avoided duplicating hook and cache error-display text by tracing context fields and state transitions rather than restating full error strings.

Verification:
- `cargo nextest run -p zappy-hooks -p zappy-templates --all-features` passed.
- `cargo test --test integration_tests` passed.
<!-- SECTION:FINAL_SUMMARY:END -->
