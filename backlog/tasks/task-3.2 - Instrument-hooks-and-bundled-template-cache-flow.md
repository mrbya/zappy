---
id: TASK-3.2
title: Instrument hooks and bundled-template cache flow
status: To Do
assignee: []
created_date: '2026-05-19 21:33'
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
- [ ] #1 Hook execution emits tracing around phase boundaries command decisions skips optional failures and success summaries with appropriate levels.
- [ ] #2 Bundled-template cache operations emit tracing around cache resolution extraction reuse and clearing with appropriate levels.
- [ ] #3 Trace messages avoid repeating error payload text already available from hook and template error types.
- [ ] #4 Tests for hooks and template cache behavior continue to pass and are updated if runtime tracing changes observable output expectations.
- [ ] #5 The resulting logs help distinguish normal expected activity from warnings and actionable failures.
<!-- AC:END -->
