---
id: TASK-1.6
title: Write Lua Zappy migration guide
status: To Do
assignee: []
created_date: '2026-05-16 16:19'
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
- [ ] #1 The guide states the migration audience and summarizes major conceptual differences between Lua Zappy and the Rust implementation.
- [ ] #2 The guide maps common old workflows to current supported commands or explains when there is no supported equivalent yet.
- [ ] #3 Manifest, variable, hook, and validation differences are documented at a practical level with links to the detailed mdBook pages.
- [ ] #4 Unsupported or incomplete Rust behavior is called out clearly so migrators do not assume feature parity.
- [ ] #5 The page builds cleanly and does not depend on stale or unverifiable claims.
<!-- AC:END -->
