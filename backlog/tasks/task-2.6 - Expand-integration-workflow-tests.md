---
id: TASK-2.6
title: Expand integration workflow tests
status: Done
assignee:
  - OpenCode
created_date: '2026-05-17 10:45'
updated_date: '2026-05-17 11:07'
labels:
  - testing
  - coverage
  - integration
milestone: m-0
dependencies:
  - TASK-2.1
references:
  - tests/integration_tests.rs
  - tests/fixtures/templates/test_template
  - crates/zappy-templates/templates
documentation:
  - AGENTS.md
  - docs/book/src/quick-start.md
  - docs/book/src/commands.md
  - docs/book/src/templates/discovery.md
parent_task_id: TASK-2
priority: high
ordinal: 2600
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add repository-level integration tests under `tests/` for behavior that crosses crates or requires invoking the installed binary-style CLI. Focus on realistic user workflows with fixture templates and temporary output directories, while leaving pure crate behavior in crate-level tests.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Integration tests cover `list`, `info`, `new`, `validate`, `init`, and `create --empty` happy paths using realistic fixtures.
- [x] #2 Integration tests cover important negative paths such as missing templates directory, unknown template ID, output conflicts, required variable failures, validation failures, and unsupported non-empty `create`.
- [x] #3 Integration tests verify `--templates-dir` disables bundled discovery and explicit missing directories fail clearly.
- [x] #4 Integration tests verify generated project files, rendered paths/content, skipped hooks with `--no-hooks`, and dry-run behavior where applicable.
- [x] #5 All new end-to-end tests live under `tests/` and reuse or extend fixtures without moving crate-only tests there.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Review existing `tests/integration_tests.rs` to avoid duplicating already-covered CLI workflows.
2. Add end-to-end tests under `tests/` for missing explicit template directories, unknown template IDs for generation/validation, output conflicts, required variable failures, `--no-hooks`, validation hook failures, explicit `--templates-dir` behavior, and unsupported non-empty `create`.
3. Reuse existing fixtures where possible and create temporary templates only inside tests for workflow-specific behavior.
4. Run the repository integration test target and update this task with results.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Integration coverage now includes the key negative paths identified in TASK-2.1 while preserving crate-only tests inside crate modules.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Expanded repository-level CLI workflow coverage under `tests/`.

Added end-to-end coverage for:
- Explicit empty `--templates-dir` disabling bundled discovery.
- Missing explicit template directories failing clearly.
- Unknown template IDs for `new` and `validate`.
- Required variable failures during generation.
- Output conflict failures without `--force`.
- `new --no-hooks` skipping a failing generation hook while still writing output files.
- Validation step hook failures.
- Current non-empty `create` stub behavior.
- `init` failing when the target template already exists without force.

Verification:
- `cargo fmt --all` completed with existing stable rustfmt warnings for nightly-only options.
- `cargo test --test integration_tests` passed: 29 tests.
<!-- SECTION:FINAL_SUMMARY:END -->
