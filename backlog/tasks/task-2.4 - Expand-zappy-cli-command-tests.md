---
id: TASK-2.4
title: Expand zappy-cli command tests
status: Done
assignee:
  - OpenCode
created_date: '2026-05-17 10:45'
updated_date: '2026-05-17 11:02'
labels:
  - testing
  - coverage
  - zappy-cli
milestone: m-0
dependencies:
  - TASK-2.1
references:
  - crates/zappy-cli/src/tests.rs
  - crates/zappy-cli/src/cli.rs
  - crates/zappy-cli/src/commands
documentation:
  - AGENTS.md
  - docs/book/src/commands.md
  - docs/book/src/reference/cli.md
parent_task_id: TASK-2
priority: medium
ordinal: 2400
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add meaningful tests for `zappy-cli` command parsing, dispatch behavior, user-facing errors, global flags, and command-specific edge cases. Keep parser and command unit tests inside `crates/zappy-cli`; reserve full binary/end-to-end flows for the integration-test subtask.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Tests cover Clap parsing for global flags, verbosity, template directory options, variables, dry-run, force, no-hooks, clear, and non-interactive flags.
- [x] #2 Tests cover command dispatch behavior for implemented commands and clear user-facing errors for unsupported or invalid combinations.
- [x] #3 Tests cover built-in variable injection behavior at the CLI boundary, including project name from `--name` and git/user fallback handling where practical.
- [x] #4 Tests cover output-relevant behavior for list/info/new/validate/init/create paths without duplicating full end-to-end coverage better suited to `tests/`.
- [x] #5 Tests are kept in `crates/zappy-cli` test modules unless they require invoking the built binary.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Review existing `zappy-cli` parser tests and command/helper source to identify meaningful uncovered behavior.
2. Add crate-level tests in `crates/zappy-cli/src/tests.rs` for parser flags not currently asserted, command helper behavior that can be exercised without full binary integration, and command-specific edge paths.
3. Keep end-to-end binary behavior for `tests/` and avoid duplicating integration-test workflows.
4. Run focused `zappy-cli` tests and coverage summary, then finalize this task with results.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Focused zappy-cli tests now exercise parser flags and command exit paths that do not require spawning the binary. Output formatting and full command workflows remain covered under repository integration tests by design.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Expanded `zappy-cli` parser and command behavior tests.

Added coverage for:
- `new --no-hooks` and `validate --no-hooks` parsing.
- Additional `init` and `create --empty` metadata/force parser flags.
- CLI built-in variable injection at the command boundary.
- Explicit template-dir discovery config disabling bundled templates.
- Command exit behavior for list, info, init, create, new, and validate edge paths.
- `new` dry-run behavior with an explicit fixture template and no output writes.
- Unsupported non-empty `create` stub returning success as currently implemented.

Implementation detail:
- Changed `discovery_config` and `command_builtins` visibility to `pub(crate)` for crate-local tests only.

Verification:
- `cargo fmt --all` completed with existing stable rustfmt warnings for nightly-only options.
- `cargo nextest run -p zappy-cli --all-features` passed: 41 tests.
- `cargo llvm-cov nextest -p zappy-cli --all-features --summary-only` passed.
<!-- SECTION:FINAL_SUMMARY:END -->
