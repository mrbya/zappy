---
id: TASK-1.1
title: Write mdBook user guide pages
status: Done
assignee:
  - OpenCode
created_date: '2026-05-16 16:18'
updated_date: '2026-05-16 16:25'
labels:
  - documentation
  - mdbook
  - user-guide
milestone: Documentation
dependencies: []
references:
  - docs/book/src/introduction.md
  - docs/book/src/quick-start.md
  - docs/book/src/installation.md
  - docs/book/src/commands.md
documentation:
  - AGENTS.md
  - README.md
  - justfile
  - crates/zappy-cli/src
parent_task_id: TASK-1
priority: high
ordinal: 1100
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Populate the user-facing pages in `docs/book/src` for readers who want to install Zappy, understand what it does, and run the implemented CLI commands. Ground the content in the current Rust implementation: the CLI facade is in `crates/zappy-cli`, the root crate only delegates to it, and non-empty `create` is currently a stub while `create --empty`, `init`, `new`, `list`, `info`, and `validate` are implemented.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `introduction.md` explains Zappy's purpose, primary audiences, and current implemented scope without overstating unsupported behavior.
- [x] #2 `installation.md` documents practical installation or local build paths and required tooling based on the repository setup.
- [x] #3 `quick-start.md` includes at least one end-to-end flow that can be run with implemented commands and bundled or fixture templates.
- [x] #4 `commands.md` summarizes all current CLI commands and important flags, including the limitation that non-empty `create` is still a stub.
- [x] #5 Examples and command descriptions are checked against the actual CLI help, source, or tests before completion.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Verify the current CLI shape from `crates/zappy-cli/src/cli.rs`, command implementations, integration tests, and selected `cargo run -- --help` subcommand output.
2. Populate `docs/book/src/introduction.md` with Zappy's purpose, audience, current implemented scope, and clear limitations.
3. Populate `docs/book/src/installation.md` with practical Cargo/local build paths and contributor tool setup from `Cargo.toml` and `justfile`.
4. Populate `docs/book/src/quick-start.md` with runnable flows using implemented commands, including bundled-template discovery and a dry-run/generation path.
5. Populate `docs/book/src/commands.md` with a user-oriented command summary, global options, aliases, key flags, examples, and the current non-empty `create` stub limitation.
6. Run `just book-check` to verify the book builds.
7. Update TASK-1.1 acceptance criteria, implementation notes, and final summary based on the completed work.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Completed documentation edits for the four TASK-1.1 pages. Verified CLI help for top-level command and each subcommand, ran `just book-check`, and spot-checked documented list/info/dry-run examples. The dry-run example produced a very large plan because the bundled `rust-cli` template tree currently contains excluded `target` paths, but the command succeeded and did not write output.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Populated the user-facing mdBook pages for TASK-1.1.

Changes:
- Expanded `docs/book/src/introduction.md` with Zappy's purpose, audience, implemented Rust CLI scope, template model overview, and current limitations.
- Expanded `docs/book/src/installation.md` with Rust/Cargo requirements, install-from-crates.io guidance, local checkout installation, source-run examples, developer setup, and release build notes.
- Expanded `docs/book/src/quick-start.md` with a runnable user flow covering help, list, info, dry-run generation, real generation, validation, and template skeleton creation.
- Expanded `docs/book/src/commands.md` with global options, aliases, command summaries, options tables, examples, and explicit notes for parsed-but-not-prompting `--non-interactive` and the non-empty `create` stub.

Verification:
- Checked command descriptions and flags against `crates/zappy-cli/src/cli.rs`, command implementations, integration tests, and `cargo run -- <command> --help` output.
- Ran `just book-check` successfully.
- Spot-checked documented `list`, `info`, and `new --dry-run` examples with `cargo run -- ...`.
<!-- SECTION:FINAL_SUMMARY:END -->
