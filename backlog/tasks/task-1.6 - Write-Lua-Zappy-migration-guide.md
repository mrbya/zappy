---
id: TASK-1.6
title: Write Lua Zappy migration guide
status: Done
assignee:
  - OpenCode
created_date: '2026-05-16 16:19'
updated_date: '2026-05-16 17:28'
labels:
  - documentation
  - mdbook
  - migration
milestone: Documentation
dependencies: []
references:
  - docs/book/src/migration/lua-zappy.md
documentation:
  - AGENTS.md
  - README.md
  - crates/zappy-cli/src
  - crates/zappy-core/src
  - .idea/zappy
parent_task_id: TASK-1
priority: medium
ordinal: 1500
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Populate `docs/book/src/migration/lua-zappy.md` for users moving from the older Lua Zappy workflow to the current Rust implementation. The guide should focus on practical differences, equivalent commands or manifest concepts where they exist, and explicit gaps where parity is not yet implemented.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The guide states the migration audience and summarizes major conceptual differences between Lua Zappy and the Rust implementation.
- [x] #2 The guide maps common old workflows to current supported commands or explains when there is no supported equivalent yet.
- [x] #3 Manifest, variable, hook, and validation differences are documented at a practical level with links to the detailed mdBook pages.
- [x] #4 Unsupported or incomplete Rust behavior is called out clearly so migrators do not assume feature parity.
- [x] #5 The page builds cleanly and does not depend on stale or unverifiable claims.
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inspect the old Lua implementation under `.idea/zappy`, focusing on README, CLI entry points, Lua modules, specs, and any template/config examples.
2. Compare old Lua workflows and template conventions with the current Rust implementation documented in the user guide, template pages, and source.
3. Populate `docs/book/src/migration/lua-zappy.md` with audience, conceptual differences, workflow mapping, manifest/template migration notes, variable/hook/validation differences, and explicit unsupported/incomplete Rust behavior.
4. Keep claims grounded in `.idea/zappy` and current Rust source/docs; avoid unverifiable feature-parity statements.
5. Run `just book-check` and then update TASK-1.6 acceptance criteria, implementation notes, and final summary.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Completed TASK-1.6. The guide uses `.idea/zappy` as the legacy source and avoids claiming automatic compatibility. It frames migration as a rewrite from Lua table templates to Rust `zappy.toml` plus source-tree templates. `just book-check` passes.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Populated the Lua Zappy migration guide for TASK-1.6.

Changes:
- Added `docs/book/src/migration/lua-zappy.md` with migration audience, high-level conceptual differences, and workflow mapping from Lua `ls`, `gen`, and `create` commands to Rust `list`, `info`, `new`, `init`, `create --empty`, and `validate` workflows.
- Documented how to migrate Lua table templates into Rust template directories with `zappy.toml` manifests and filesystem source roots.
- Documented placeholder, built-in variable, hook, discovery, user template, validation, and bundled template ID differences.
- Called out parity gaps explicitly: no interactive prompt flow yet, no `gen -g` equivalent, non-empty `create` is a stub, no `zconfig.lua` defaults, no in-process Lua hook callbacks, no Lua `structure` table source format, and no bundled Make/Zephyr templates in the Rust implementation.

Verification:
- Grounded Lua-era behavior in `.idea/zappy` README, command modules, template loader, project template generator, creator module, utilities, and specs.
- Cross-checked Rust behavior against existing mdBook pages and current CLI/core implementation.
- Ran `just book-check` successfully.
<!-- SECTION:FINAL_SUMMARY:END -->
