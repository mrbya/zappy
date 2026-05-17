---
id: TASK-2.5
title: Expand hooks and bundled-template tests
status: Done
assignee:
  - OpenCode
created_date: '2026-05-17 10:45'
updated_date: '2026-05-17 11:05'
labels:
  - testing
  - coverage
  - zappy-hooks
  - zappy-templates
milestone: m-0
dependencies:
  - TASK-2.1
references:
  - crates/zappy-hooks/src/tests.rs
  - crates/zappy-hooks/src/hooks.rs
  - crates/zappy-templates/src/tests.rs
  - crates/zappy-templates/src/cache.rs
  - crates/zappy-templates/templates
documentation:
  - AGENTS.md
  - docs/book/src/templates/builtins.md
  - docs/book/src/templates/hooks.md
  - docs/book/src/reference/cache.md
parent_task_id: TASK-2
priority: medium
ordinal: 2500
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add meaningful crate-level tests for `zappy-hooks` and `zappy-templates`, focusing on hook execution semantics, unsupported shell hooks, failure propagation, bundled template extraction, cache clearing, and embedded template availability. Keep tests inside the relevant crate test modules.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `zappy-hooks` tests cover successful hook execution, non-zero exit handling, missing commands where practical, environment/working-directory behavior, and explicit rejection of `shell = true`.
- [x] #2 `zappy-templates` tests cover bundled template extraction into cache directories and reuse behavior.
- [x] #3 `zappy-templates` tests cover cache clearing behavior and error cases for invalid or unavailable cache paths where practical.
- [x] #4 Tests verify bundled starter templates remain discoverable and have expected manifest IDs.
- [x] #5 Tests are placed in the relevant crate test modules and avoid duplicating full CLI workflows.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Review existing `zappy-hooks` and `zappy-templates` tests and source to identify meaningful untested behavior.
2. Add `zappy-hooks` tests for optional failures, shell rejection, non-zero command failures, rendered command args/env/working_dir, and true conditional execution.
3. Add `zappy-templates` tests for bundled extraction/reuse/clear behavior and expected embedded template IDs where practical without mutating global state unsafely.
4. Run focused tests and coverage summaries for both crates, then finalize this task.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added `camino.workspace = true` as a zappy-hooks dev-dependency so tests can construct hook working directories using the same type as `HookSpec`.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Expanded `zappy-hooks` and `zappy-templates` behavior tests.

Added coverage for:
- Required hook success and true/false conditional execution.
- Missing command failures and optional hook warning behavior.
- Non-zero hook exit failures with captured output.
- Explicit rejection of `shell = true` hooks before spawning.
- Rendered command, args, env values, and working directory handling.
- Invalid rendered working directory failures.
- Bundled template cache extraction, reuse, marker-triggered reinstall, clearing, and idempotent missing-cache clearing.
- Expected bundled starter template manifest IDs: `cpp-cmake-app`, `cpp-cmake-lib`, `lua-cli`, `nvim-plugin`, and `rust-cli`.

Verification:
- `cargo fmt --all` completed with existing stable rustfmt warnings for nightly-only options.
- `cargo nextest run -p zappy-hooks -p zappy-templates --all-features` passed: 10 tests.
- `cargo llvm-cov nextest -p zappy-hooks -p zappy-templates --all-features --summary-only` passed.
<!-- SECTION:FINAL_SUMMARY:END -->
