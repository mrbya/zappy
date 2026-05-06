# Zappy RS Implementation Plan

> Detailed implementation plan for a Rust re-implementation of the original Lua-based `zappy` project template generator.
>
> Intended audience: Viktor + coding agents working incrementally on the project.

---

## 1. Project Goal

`zappy-rs` is a Rust-based project templating and scaffolding engine for generating new projects consistently across multiple languages, ecosystems, and personal workflows.

The tool should preserve the useful parts of the old Lua `zappy` implementation:

- generate projects from predefined and user-defined templates;
- list available templates;
- create templates from existing projects;
- create empty template skeletons;
- support template variables;
- support variable substitution in both file contents and file/directory names;
- support pre-generation and post-generation hooks;
- support user templates and user-level default configuration.

At the same time, the Rust version should improve the original model by using:

- a declarative manifest instead of executable Lua template files;
- safer file generation and overwrite behavior;
- structured validation to prevent template rot;
- explicit binary/text file handling;
- interactive and non-interactive workflows;
- first-class dry-run support;
- good diagnostics suitable for both humans and coding agents;
- a modular Rust codebase that can grow without becoming a pile of ad-hoc special cases.

---

## 2. Design Inputs

### 2.1 Old Lua Zappy

Important concepts to preserve:

- CLI commands:
  - `zappy ls`
  - `zappy gen`
  - `zappy create`
- template variables provided through `-a key=value` arguments;
- generated project name via `-n`;
- generated project path via `-p`;
- template selection via `-t`;
- optional git initialization via `-g`;
- user configuration directory, historically `~/.config/zappy` or `$ZAPPY_CONFIG`;
- user templates under the config directory;
- global/default template variables from user config;
- built-in variables such as project name, user name, date, day, month, and year;
- variable substitution in both file contents and path names;
- hooks with access to the generation path and resolved config;
- ability to create a template from an existing project;
- ability to create an empty template skeleton.

### 2.2 Spawn Point Inspiration

Useful ideas to adapt:

- Rust CLI implementation;
- template listing;
- interactive and non-interactive generation;
- manifest-defined variables;
- variable transformations such as `PascalCase`, `kebab-case`, `snake_case`;
- filename/directory substitution;
- conditional files/directories;
- excludes for generated or irrelevant directories such as `target`, `node_modules`, `.git`, `.DS_Store`;
- pre/post generation hooks;
- validation command that generates a template into a temporary directory and runs build/test/lint steps;
- template authoring workflow centered around a manifest file.

---

## 3. Guiding Principles

### 3.1 Boring Core, Powerful Edges

The core engine should be deterministic and boring:

1. Load template metadata.
2. Resolve variables.
3. Build a render plan.
4. Validate the render plan.
5. Execute filesystem writes.
6. Run hooks.

More advanced behavior should be expressed through manifests, hooks, recipes, or future plugins instead of being hard-coded into the core.

### 3.2 Safe by Default

Default behavior should avoid surprising destructive actions.

Required defaults:

- do not overwrite existing files unless explicitly requested;
- reject output paths that escape the target directory;
- treat symlinks carefully;
- skip excluded files/directories deterministically;
- detect binary files and do not try to render them as text;
- show a dry-run plan before destructive or ambiguous actions when requested;
- fail with useful diagnostics instead of silently skipping important work.

### 3.3 Templates Should Be Mostly Normal Projects

A template should look as close as possible to the final generated project.

Prefer literal placeholder replacement over a complex embedded template language. This keeps template files readable and often valid for their target ecosystem.

Example placeholder style:

```text
__ZAPPY_PROJECT_NAME__
__ZAPPY_PROJECT_NAME_SNAKE__
__ZAPPY_PROJECT_NAME_PASCAL__
```

The old Lua `#{VAR}` syntax may be supported as a legacy compatibility mode later, but should not be the primary syntax for the new engine.

### 3.4 Separate Planning from Writing

The engine should first build a `GenerationPlan` that describes everything it intends to do.

This enables:

- dry-run output;
- testing without touching the filesystem;
- clear diagnostics;
- conflict detection;
- future UI/frontends;
- easier debugging by coding agents.

### 3.5 Prefer Data Manifests Over Executable Templates

The old Lua implementation used executable Lua files as templates. That was flexible, but difficult to validate, sandbox, and reason about.

The Rust version should use `zappy.toml` as the default manifest format.

Possible future support:

- YAML manifest as optional feature;
- migration/import from old Lua templates;
- recipe steps for ecosystem generators;
- optional plugin system.

---

## 4. Proposed User Experience

### 4.1 Primary CLI Shape

Recommended primary commands:

```bash
zappy list
zappy info <template>
zappy new <template> <project-name>
zappy validate <template>
zappy create --from ./existing-project --name my-template
zappy init-template ./my-template
```

### 4.2 Compatibility Aliases

To keep the spirit of old Zappy and make migration pleasant, provide aliases:

```bash
zappy ls       # alias for list
zappy gen      # alias for new/generate
zappy create   # preserved command name
```

Possible mapping:

```bash
zappy gen -t rust-cli -n my-tool -p ./out -a license=MIT
```

Equivalent modern form:

```bash
zappy new rust-cli my-tool --output ./out --var license=MIT
```

### 4.3 Common Workflows

List templates:

```bash
zappy list
zappy list --language rust
zappy list --templates-dir ./templates
```

Show template details:

```bash
zappy info rust-cli
```

Generate interactively:

```bash
zappy new rust-cli my-tool
```

Generate non-interactively:

```bash
zappy new rust-cli my-tool \
  --output ./playground \
  --var description="My new CLI" \
  --var license=MIT \
  --non-interactive
```

Preview generation:

```bash
zappy new rust-cli my-tool --dry-run
```

Validate a template:

```bash
zappy validate rust-cli
```

Create a template from an existing project:

```bash
zappy create --from ./existing-project --name cpp-cmake-lib
```

Create an empty template skeleton:

```bash
zappy init-template ./templates/rust-cli
```

---

## 5. Repository Layout

The repository is already bootstrapped as a multi-crate workspace. The implementation plan should use this split from the beginning instead of starting with only `zappy-core` and `zappy-cli`.

Current workspace shape:

```text
.
├── crates
│   ├── zappy-adapters
│   ├── zappy-cli
│   ├── zappy-core
│   ├── zappy-fs
│   └── zappy-hooks
├── docs
│   ├── similar-projects
│   ├── implementation_plan.md
│   └── rustdoc_style.md
├── src
│   ├── lib.rs
│   └── main.rs
├── Cargo.lock
├── Cargo.toml
├── justfile
├── LICENSE-APACHE
├── LICENSE-MIT
├── README.md
├── rustfmt.toml
├── rust-toolchain.toml
└── tree.md
```

The root package is the public `zappy` crate and owns the shipped `zappy` binary. The crates under `crates/` are internal implementation crates that keep the codebase modular.

### 5.1 Crate Responsibilities

#### Root crate: `zappy`

Purpose: public package, binary entry point, and optionally small public facade.

Recommended contents:

```text
src/
├── lib.rs      # optional facade/re-exports, intentionally small
└── main.rs     # calls zappy_cli::run()
```

Responsibilities:

- define the installable `zappy` binary;
- keep `main.rs` tiny;
- convert the top-level CLI result into a process exit status;
- optionally expose a minimal public API later if publishing as a library is useful;
- avoid implementing real scaffolding logic directly in the root crate.

Suggested `main.rs` shape:

```rust
fn main() -> std::process::ExitCode {
    zappy_cli::run()
}
```

The exact return type can change, but the root binary should remain a thin handoff layer.

#### `zappy-core`

Purpose: pure domain model and deterministic engine logic.

Responsibilities:

- manifest data model;
- validated template metadata;
- variable definitions and resolved variable maps;
- built-in variable names;
- variable precedence model;
- transforms such as `snake_case`, `kebab-case`, `PascalCase`, etc.;
- placeholder rendering for text and paths;
- condition evaluation;
- render-plan data structures;
- validation-plan data structures;
- shared error and diagnostic types where they are domain-level rather than CLI-level.

`zappy-core` should not know how to prompt users, print tables, discover platform config directories, execute commands, or write files. Keeping it mostly pure makes it easy to unit-test and safe for coding agents to modify.

#### `zappy-fs`

Purpose: filesystem access and safe project materialization.

Responsibilities:

- path normalization and path traversal protection;
- deterministic directory walking;
- source template file enumeration;
- exclude matching at the filesystem layer;
- text/binary classification helpers;
- safe directory creation;
- safe file writes;
- copy operations for binary files;
- overwrite/force behavior;
- executable-bit preservation where supported;
- temporary-directory helpers for validation.

`zappy-fs` may depend on `zappy-core` for plan types, but `zappy-core` should not depend on `zappy-fs`.

#### `zappy-hooks`

Purpose: command execution for hooks and validation steps.

Responsibilities:

- pre-generation hook execution;
- post-generation hook execution;
- validation setup/step/teardown execution;
- command environment construction;
- working-directory handling;
- stdout/stderr capture;
- optional hook failure behavior;
- `--no-hooks` support at the execution layer;
- cross-platform command invocation rules.

`zappy-hooks` may depend on `zappy-core` for hook and validation step configuration types. It should not depend on `zappy-cli`.

#### `zappy-adapters`

Purpose: ecosystem-specific orchestration and future recipe support.

This crate should start small. It is a place for behavior that knows about external ecosystems but does not belong in the generic engine.

Potential future responsibilities:

- recipe steps such as `cargo init`, `uv init`, `npm create`, `dotnet new`, etc.;
- Git initialization adapter;
- Rust/Cargo adapter;
- Python/uv adapter;
- CMake adapter;
- Zephyr project adapter;
- Node/Tauri adapter;
- adapter-level availability checks;
- adapter-specific default validation commands.

For the MVP, `zappy-adapters` can remain mostly empty or only contain a small Git adapter if `zappy new --git` is implemented early. Avoid moving generic hook execution or generic filesystem behavior into this crate.

#### `zappy-cli`

Purpose: user-facing command-line interface and command orchestration.

Responsibilities:

- `clap` argument model;
- compatibility aliases such as `ls` and `gen`;
- interactive prompting;
- non-interactive mode behavior;
- terminal output, tables, and diagnostics;
- command dispatch;
- conversion from CLI arguments into core generation requests;
- high-level orchestration across `zappy-core`, `zappy-fs`, `zappy-hooks`, and `zappy-adapters`.

`zappy-cli` should be the only crate that knows about terminal UX. It can depend on all internal crates.

### 5.2 Recommended Dependency Direction

Keep the dependency graph acyclic and boring:

```text
zappy
└── zappy-cli
    ├── zappy-core
    ├── zappy-fs
    │   └── zappy-core
    ├── zappy-hooks
    │   └── zappy-core
    └── zappy-adapters
        ├── zappy-core
        ├── zappy-fs
        └── zappy-hooks
```

Important rules:

- `zappy-core` should not depend on any other `zappy-*` crate.
- `zappy-cli` may depend on all internal crates.
- root `zappy` should depend primarily on `zappy-cli`.
- no crate should depend on the root `zappy` crate.
- avoid making `zappy-adapters` a dumping ground; only ecosystem-specific behavior goes there.

### 5.3 Suggested Module Layout Per Crate

Recommended early module layout:

```text
crates/zappy-core/src/
├── lib.rs
├── error.rs
├── manifest.rs
├── template.rs
├── variables.rs
├── transform.rs
├── render.rs
├── condition.rs
├── plan.rs
└── validation.rs

crates/zappy-fs/src/
├── lib.rs
├── error.rs
├── discover.rs
├── paths.rs
├── walk.rs
├── classify.rs
├── materialize.rs
└── temp.rs

crates/zappy-hooks/src/
├── lib.rs
├── error.rs
├── command.rs
├── env.rs
├── hooks.rs
└── validation.rs

crates/zappy-adapters/src/
├── lib.rs
├── error.rs
├── git.rs          # optional early adapter
└── recipe.rs       # placeholder for later orchestrator mode

crates/zappy-cli/src/
├── lib.rs
├── args.rs
├── commands/
│   ├── create.rs
│   ├── info.rs
│   ├── init_template.rs
│   ├── list.rs
│   ├── new.rs
│   └── validate.rs
├── output.rs
├── prompt.rs
└── diagnostics.rs
```

Do not feel forced to create every module immediately. This structure is a target shape for the coding agent and can be filled in phase by phase.

### 5.4 Current Workspace Notes

Your current root `Cargo.toml` already has the correct high-level shape:

- root package name: `zappy`;
- root binary name: `zappy`;
- workspace members include the root crate and all internal crates;
- resolver is set to `2`;
- edition is `2024`;
- Rust version is `1.85.0`;
- workspace dependencies already define all internal path crates.

Two practical recommendations:

1. Add the internal crates to `[dependencies]` only where they are actually used. The root package probably only needs `zappy-cli` initially.
2. Put shared third-party dependencies under `[workspace.dependencies]` once two or more crates need them, but do not prematurely centralize every dependency.

---

## 6. Initial Dependency Suggestions

The dependency plan should follow the crate boundaries above.

### 6.1 Root `zappy` Dependencies

Initially, the root crate should be tiny:

```toml
[dependencies]
zappy-cli.workspace = true
```

The root crate should not need `clap`, `serde`, `walkdir`, or command execution dependencies directly.

### 6.2 `zappy-core` Dependencies

Likely useful crates:

```toml
[dependencies]
camino = "1"
heck = "0.5"
indexmap = { version = "2", features = ["serde"] }
serde = { version = "1", features = ["derive"] }
thiserror = "2"
toml = "0.8"
tracing = "0.1"
```

Optional / later:

```toml
regex = "1"         # variable validation, optional initially
serde_yaml = "0.9"  # optional YAML manifest support later
```

### 6.3 `zappy-fs` Dependencies

Likely useful crates:

```toml
[dependencies]
zappy-core.workspace = true
camino = "1"
directories = "6"
tempfile = "3"
thiserror = "2"
tracing = "0.1"
walkdir = "2"
```

Optional / later:

```toml
ignore = "0.4"      # gitignore-style traversal, useful later
```

### 6.4 `zappy-hooks` Dependencies

Likely useful crates:

```toml
[dependencies]
zappy-core.workspace = true
camino = "1"
thiserror = "2"
tracing = "0.1"
```

Most hook execution can use `std::process::Command` initially. Avoid pulling in a heavy process-management dependency unless the standard library becomes painful.

### 6.5 `zappy-adapters` Dependencies

Initially:

```toml
[dependencies]
zappy-core.workspace = true
zappy-fs.workspace = true
zappy-hooks.workspace = true
thiserror = "2"
tracing = "0.1"
```

Add ecosystem-specific dependencies only when an adapter actually needs them. Prefer shelling out to ecosystem tools in recipe mode rather than linking large ecosystem libraries early.

### 6.6 `zappy-cli` Dependencies

Likely useful crates:

```toml
[dependencies]
zappy-adapters.workspace = true
zappy-core.workspace = true
zappy-fs.workspace = true
zappy-hooks.workspace = true
clap = { version = "4", features = ["derive"] }
color-eyre = "0.6"
inquire = "0.7"
miette = { version = "7", features = ["fancy"] }
tabled = "0.17"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

For integration tests, put these either in the root crate or in `zappy-cli`, depending on where CLI tests live:

```toml
[dev-dependencies]
assert_cmd = "2"
predicates = "3"
tempfile = "3"
pretty_assertions = "1"
```

---

## 7. Template Directory Model

A template is a directory containing a `zappy.toml` manifest and a template payload directory.

Recommended structure:

```text
rust-cli/
├── zappy.toml
└── template/
    ├── Cargo.toml
    ├── README.md
    └── src/
        └── main.rs
```

This keeps metadata separate from generated files.

Alternative supported later:

```text
rust-cli/
├── zappy.toml
├── Cargo.toml
├── README.md
└── src/
    └── main.rs
```

The explicit `template/` subdirectory is safer and clearer. Prefer it for new templates.

---

## 8. Template Discovery

### 8.1 Search Order

Resolve template directories in this order:

1. `--templates-dir <PATH>` CLI option;
2. `ZAPPY_TEMPLATES_DIR` environment variable;
3. `$ZAPPY_CONFIG/templates`, if `ZAPPY_CONFIG` is set;
4. platform config directory, usually `~/.config/zappy/templates` on Linux;
5. executable-relative `templates/` directory;
6. current-working-directory `templates/` directory.

This combines the old Zappy config model with the ergonomic discovery model from newer scaffolders.

### 8.2 Template Identity

Each template should have a stable ID.

Example:

```toml
[template]
id = "rust-cli"
name = "Rust CLI Application"
description = "A small Rust CLI using clap, tracing, and anyhow-style error handling."
language = "rust"
version = "0.1.0"
```

Use `id` for CLI lookup.
Use `name` for human-facing output.

### 8.3 Duplicate Handling

If multiple template directories contain the same template ID:

- higher-priority search locations win;
- emit a warning at verbose log level;
- `zappy list --all-locations` may show shadowed templates later.

---

## 9. Manifest Format

Use `zappy.toml` as the primary manifest.

### 9.1 Minimal Manifest

```toml
[template]
id = "rust-cli"
name = "Rust CLI Application"
description = "Small Rust CLI application."
language = "rust"
version = "0.1.0"

[variables.project_name]
prompt = "Project name"
default = "my-cli"
required = true

[variables.description]
prompt = "Project description"
default = "A small Rust CLI application."
```

### 9.2 Full Example Manifest

```toml
[template]
id = "rust-cli"
name = "Rust CLI Application"
description = "Rust CLI app using clap, tracing, and a clean module layout."
language = "rust"
version = "0.1.0"
authors = ["Viktor Toth"]

[template.source]
root = "template"

[variables.project_name]
prompt = "Project name"
default = "my-cli"
required = true
validation_regex = "^[a-zA-Z][a-zA-Z0-9_-]*$"
transforms = ["raw", "kebab", "snake", "pascal", "upper"]

[variables.project_name.placeholders]
raw = "__ZAPPY_PROJECT_NAME__"
kebab = "__ZAPPY_PROJECT_NAME_KEBAB__"
snake = "__ZAPPY_PROJECT_NAME_SNAKE__"
pascal = "__ZAPPY_PROJECT_NAME_PASCAL__"
upper = "__ZAPPY_PROJECT_NAME_UPPER__"

[variables.description]
prompt = "Description"
default = "A small Rust CLI application."

[variables.description.placeholders]
raw = "__ZAPPY_DESCRIPTION__"

[variables.license]
prompt = "License"
default = "MIT OR Apache-2.0"
choices = ["MIT", "Apache-2.0", "MIT OR Apache-2.0", "Proprietary"]

[variables.use_github_actions]
prompt = "Add GitHub Actions CI?"
default = true

[paths]
exclude = [
  ".git",
  ".DS_Store",
  "target",
  "node_modules",
  "dist",
]
binary_extensions = ["png", "jpg", "jpeg", "gif", "ico", "pdf"]
binary_files = ["Cargo.lock"]

[[conditionals]]
path = ".github/workflows/ci.yml"
when = "use_github_actions"

[[hooks.post_generate]]
name = "Format Rust code"
command = "cargo"
args = ["fmt"]
optional = true

[[hooks.post_generate]]
name = "Initialize git repository"
command = "git"
args = ["init"]
when = "init_git"
optional = true

[validation]
output_dir_name = "zappy-validation-rust-cli"

[validation.variables]
project_name = "zappy-test-cli"
description = "Generated test CLI"
license = "MIT OR Apache-2.0"
use_github_actions = true
init_git = false

[[validation.steps]]
name = "cargo fmt"
command = "cargo"
args = ["fmt", "--check"]

[[validation.steps]]
name = "cargo clippy"
command = "cargo"
args = ["clippy", "--", "-D", "warnings"]

[[validation.steps]]
name = "cargo test"
command = "cargo"
args = ["test"]
```

### 9.3 Manifest Sections

Required:

- `[template]`
- `[variables]`, although it may be empty

Optional:

- `[template.source]`
- `[paths]`
- `[[conditionals]]`
- `[hooks]`
- `[validation]`
- `[recipe]`, future feature
- `[legacy]`, future compatibility feature

---

## 10. Manifest Data Model

### 10.1 Core Rust Types

Suggested starting point:

```rust
pub struct Manifest {
    pub template: TemplateMetadata,
    pub variables: IndexMap<String, VariableSpec>,
    pub paths: PathConfig,
    pub conditionals: Vec<ConditionalPath>,
    pub hooks: HookConfig,
    pub validation: Option<ValidationConfig>,
}

pub struct TemplateMetadata {
    pub id: TemplateId,
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub version: Option<String>,
    pub authors: Vec<String>,
    pub source: SourceConfig,
}

pub struct VariableSpec {
    pub prompt: Option<String>,
    pub default: Option<VariableValue>,
    pub required: bool,
    pub choices: Vec<VariableValue>,
    pub validation_regex: Option<String>,
    pub transforms: Vec<TransformKind>,
    pub placeholders: IndexMap<TransformKind, String>,
}

pub enum VariableValue {
    String(String),
    Bool(bool),
    Integer(i64),
}

pub enum TransformKind {
    Raw,
    Kebab,
    Snake,
    Pascal,
    Camel,
    ScreamingSnake,
    Upper,
    Lower,
}
```

### 10.2 Important Modeling Notes

- Use newtypes for IDs and paths where helpful.
- Keep deserialization structs close to the manifest shape.
- Convert deserialized manifest into a validated internal representation.
- Do not let arbitrary strings float through the core after validation.
- Use `camino::Utf8PathBuf` for UTF-8 paths if acceptable; otherwise use `PathBuf` internally and convert only for display.

---

## 11. Variable Resolution

### 11.1 Sources of Values

Resolve variables from these sources, highest priority first:

1. CLI `--var key=value`;
2. explicit command options such as project name;
3. interactive prompt answers;
4. template variable defaults;
5. user config defaults;
6. built-in variables.

There is an important design choice here:

- old Zappy config variables behaved like an additional source of template variables;
- template defaults should probably override broad user defaults when the template author intentionally chooses a value;
- CLI values should always win.

Recommended precedence:

```text
CLI explicit values
> interactive answers
> template defaults
> user config defaults
> built-ins
```

For built-ins like `PROJECT`, the project name passed to `zappy new <template> <project-name>` should override everything else.

### 11.2 Built-In Variables

Preserve old Zappy built-ins, but expose modern names too.

Old-compatible names:

- `PROJECT`
- `USER`
- `DATE`
- `DAY`
- `MONTH`
- `YEAR`

Modern names:

- `project_name`
- `user`
- `date`
- `day`
- `month`
- `year`

Optional additional built-ins later:

- `crate_name`
- `package_name`
- `module_name`
- `git_user_name`
- `git_user_email`

### 11.3 Transform Expansion

For a single input:

```text
project_name = "my cool tool"
```

Generate transformed values:

```text
raw             = "my cool tool"
kebab           = "my-cool-tool"
snake           = "my_cool_tool"
pascal          = "MyCoolTool"
camel           = "myCoolTool"
screaming_snake = "MY_COOL_TOOL"
upper           = "MY COOL TOOL"
lower           = "my cool tool"
```

Use the `heck` crate for common transforms.

### 11.4 Boolean Variables

Boolean variables are needed for conditionals.

Example:

```toml
[variables.use_docker]
prompt = "Add Dockerfile?"
default = false
```

Boolean values should parse from:

- `true` / `false`;
- `yes` / `no`;
- `1` / `0`;
- possibly `y` / `n` in interactive mode.

---

## 12. Rendering Model

### 12.1 Render Plan

Before writing anything, build a render plan.

Suggested model:

```rust
pub struct GenerationPlan {
    pub template_id: TemplateId,
    pub output_dir: PathBuf,
    pub operations: Vec<PlanOperation>,
    pub warnings: Vec<PlanWarning>,
}

pub enum PlanOperation {
    CreateDirectory { path: PathBuf },
    RenderTextFile { source: PathBuf, destination: PathBuf, content: String },
    CopyBinaryFile { source: PathBuf, destination: PathBuf },
    Skip { source: PathBuf, reason: SkipReason },
    RunHook { phase: HookPhase, hook: HookSpec },
}
```

The plan should support display as:

```text
CREATE DIR   ./my-tool/src
RENDER       template/Cargo.toml -> ./my-tool/Cargo.toml
COPY         template/assets/logo.png -> ./my-tool/assets/logo.png
SKIP         template/target [excluded]
HOOK         post_generate: cargo fmt
```

### 12.2 Text Rendering

For each text file:

1. Read as UTF-8.
2. Replace placeholders with resolved values.
3. Preserve line endings where reasonable.
4. Write rendered content to destination.

Initial implementation can use simple literal replacement.

Avoid introducing a full templating language early.

### 12.3 Path Rendering

Path names should support placeholder replacement too.

Example template file:

```text
template/src/__ZAPPY_PROJECT_NAME_SNAKE__.rs
```

Generated output:

```text
src/my_cool_tool.rs
```

### 12.4 Binary Handling

Binary files should be copied without content rendering.

Detection strategy:

1. If file path matches `binary_files`, copy as binary.
2. Else if extension matches `binary_extensions`, copy as binary.
3. Else attempt UTF-8 read.
4. If UTF-8 read fails, copy as binary and emit verbose diagnostic.

### 12.5 Excludes

Exclude matching should support simple file/directory names initially:

```toml
[paths]
exclude = [".git", "target", "node_modules", ".DS_Store"]
```

Later upgrade path:

- glob patterns;
- `.gitignore`-style rules;
- include overrides.

### 12.6 Conditionals

Initial conditionals can be path-based and boolean-variable-based.

```toml
[[conditionals]]
path = "Dockerfile"
when = "use_docker"

[[conditionals]]
path = ".github/workflows"
when = "use_github_actions"
```

Evaluation rule:

- if `when` resolves to `true`, include path;
- otherwise skip path and all descendants.

Later support:

- `unless = "var"`;
- simple expressions;
- choices/enums;
- multiple variables.

---

## 13. Filesystem Safety

### 13.1 Output Directory Rules

Default:

- create output directory if missing;
- fail if destination files already exist;
- allow existing empty directories;
- do not overwrite unless `--force` is passed.

Useful flags:

```bash
--dry-run
--force
--allow-dirty-output-dir
--no-hooks
--no-git
```

### 13.2 Path Traversal Protection

When rendering destination paths:

- reject absolute paths from template paths;
- reject path components containing `..`;
- canonicalize cautiously because output paths may not exist yet;
- ensure final destination remains under output root.

### 13.3 Symlink Handling

MVP recommendation:

- do not follow symlinks by default;
- copy symlinks only behind an explicit future option;
- emit warning when symlink is skipped.

This avoids accidental leakage from outside the template directory.

### 13.4 Deterministic Traversal

Sort filesystem traversal results before planning operations.

This improves:

- stable dry-run output;
- reproducible tests;
- coding-agent debugging;
- snapshot tests.

---

## 14. Hooks

### 14.1 Hook Model

Use command hooks, not executable Rust/Lua/Python snippets.

```toml
[[hooks.pre_generate]]
name = "Check cargo availability"
command = "cargo"
args = ["--version"]
optional = false

[[hooks.post_generate]]
name = "Format generated Rust project"
command = "cargo"
args = ["fmt"]
working_dir = "{{ output_dir }}"
optional = true
```

### 14.2 Hook Fields

Suggested hook fields:

```rust
pub struct HookSpec {
    pub name: Option<String>,
    pub command: String,
    pub args: Vec<String>,
    pub working_dir: Option<PathBuf>,
    pub env: IndexMap<String, String>,
    pub when: Option<String>,
    pub optional: bool,
    pub shell: bool,
}
```

### 14.3 Security Stance

Default to `shell = false`.

That means:

- command and args are passed directly to `std::process::Command`;
- no shell expansion;
- no accidental command injection from variables.

Allow `shell = true` only as an explicit opt-in later.

### 14.4 Hook Phases

Initial phases:

- `pre_generate`: before filesystem writes;
- `post_generate`: after successful writes.

Validation-specific phases:

- `validation.setup`;
- `validation.steps`;
- `validation.teardown`.

Future phases:

- `pre_plan`;
- `post_plan`;
- `pre_render_file`;
- `post_render_file`.

Do not add file-level hooks until truly needed.

---

## 15. Validation

Validation is the main feature that prevents template rot.

### 15.1 Validation Flow

`zappy validate <template>` should:

1. discover and load the selected template;
2. read `validation.variables` from the manifest;
3. create a temporary directory;
4. generate the template into the temporary directory in non-interactive mode;
5. run setup hooks, if any;
6. run validation steps in order;
7. run teardown hooks, if any;
8. report success/failure;
9. clean up the temporary directory unless `--keep-temp` is passed.

### 15.2 Validation Config Example

```toml
[validation]
output_dir_name = "zappy-validation-rust-cli"

[validation.variables]
project_name = "zappy-test-cli"
description = "Generated test CLI"
use_github_actions = false

[[validation.setup]]
name = "Show Rust version"
command = "rustc"
args = ["--version"]
optional = true

[[validation.steps]]
name = "Format"
command = "cargo"
args = ["fmt", "--check"]

[[validation.steps]]
name = "Clippy"
command = "cargo"
args = ["clippy", "--", "-D", "warnings"]

[[validation.steps]]
name = "Test"
command = "cargo"
args = ["test"]
```

### 15.3 CLI Options

```bash
zappy validate rust-cli
zappy validate rust-cli --keep-temp
zappy validate rust-cli --step test
zappy validate rust-cli --verbose
zappy validate --all
zappy validate --language rust
```

MVP only needs:

```bash
zappy validate <template>
zappy validate <template> --keep-temp
```

---

## 16. Template Creation

The old Zappy implementation had `create`, which could generate a template from an existing project or create an empty template.

Preserve this, but split into clearer workflows:

```bash
zappy create --from ./existing-project --name my-template
zappy init-template ./templates/my-template
```

Keep `zappy create -e` as a compatibility alias for `init-template`.

### 16.1 `init-template`

Creates:

```text
my-template/
├── zappy.toml
└── template/
    └── README.md
```

Generated manifest should contain TODOs and examples.

### 16.2 `create --from`

MVP behavior:

1. copy an existing project into `template/`;
2. exclude common junk directories by default;
3. create a minimal `zappy.toml`;
4. optionally add variables from `--var` definitions;
5. optionally replace literal values in copied files with placeholders.

Example:

```bash
zappy create \
  --from ./my-existing-cli \
  --name rust-cli \
  --var project_name=my-existing-cli \
  --var description="My existing CLI"
```

This could generate placeholders such as:

```text
__ZAPPY_PROJECT_NAME__
__ZAPPY_DESCRIPTION__
```

and replace matching values inside copied text files.

### 16.3 Avoid Over-Automating Template Creation Initially

Do not try to infer everything.

A good initial `create --from` is a helper, not magic.

It should produce a reasonable starting point that the user can edit.

---

## 17. User Configuration

### 17.1 Config Directory

Support old environment variable:

```bash
ZAPPY_CONFIG=/path/to/config
```

Default platform config directory:

```text
Linux:   ~/.config/zappy
macOS:   ~/Library/Application Support/zappy
Windows: %APPDATA%\zappy
```

### 17.2 Config Structure

```text
zappy/
├── config.toml
└── templates/
    └── ...
```

### 17.3 Config File

Example:

```toml
[defaults]
license = "MIT OR Apache-2.0"
author = "Viktor Toth"
use_github_actions = true

[defaults.zephyr]
board = "nucleo_f767zi"
runner = "openocd"

[paths]
templates = [
  "~/projects/templates",
]
```

### 17.4 Config Precedence

Recommended:

```text
CLI flags
> interactive values
> template manifest defaults
> user config defaults
> built-ins
```

User config defaults should fill gaps, not unexpectedly override template-specific defaults.

---

## 18. Error Handling and Diagnostics

Each crate should own the errors for the layer it implements. Avoid forcing every crate-specific error into one huge enum too early.

### 18.1 Crate-Level Error Types

Recommended starting point:

```text
zappy-core::Error      # manifest, variables, transforms, render semantics
zappy-fs::Error        # discovery, path safety, traversal, writes, tempdirs
zappy-hooks::Error     # command execution, stdout/stderr capture, failures
zappy-adapters::Error  # ecosystem-specific adapter failures
zappy-cli::Error       # CLI argument/prompt/orchestration diagnostics
```

Use `thiserror` for library crates.

Example `zappy-core` categories:

```rust
pub enum Error {
    Manifest(ManifestError),
    Variable(VariableError),
    Transform(TransformError),
    Render(RenderError),
    ValidationModel(ValidationModelError),
}
```

Example `zappy-fs` categories:

```rust
pub enum Error {
    Discovery(DiscoveryError),
    Path(PathError),
    Walk(WalkError),
    Materialize(MaterializeError),
    TempDir(TempDirError),
}
```

The CLI can convert lower-level errors into user-facing `miette` reports.

### 18.2 CLI Diagnostics

Use `miette` or `color-eyre` for human-friendly output.

Goals:

- show which manifest failed;
- show which variable is missing;
- show which output file conflicts;
- show which hook failed;
- show command stdout/stderr for validation failures;
- suggest next actions.

Example:

```text
error: destination file already exists

  output: ./my-tool/Cargo.toml

help: rerun with --force to overwrite existing files, or choose another output directory
```

### 18.3 Agent-Friendly Output

Provide structured output later:

```bash
zappy new rust-cli my-tool --dry-run --format json
zappy validate rust-cli --format json
```

Not needed for MVP, but design internal data structures to make it easy.

---

## 19. Testing Strategy

Testing should follow crate boundaries.

### 19.1 Unit Tests by Crate

`zappy-core` unit tests:

- manifest deserialization;
- manifest validation;
- variable precedence;
- variable type parsing;
- transform expansion;
- placeholder replacement;
- path rendering;
- condition evaluation;
- render-plan domain invariants;
- validation config modeling.

`zappy-fs` unit tests:

- template discovery;
- search path precedence;
- deterministic directory traversal;
- excludes;
- binary/text classification;
- path traversal rejection;
- output conflict detection;
- safe materialization behavior;
- tempdir helpers.

`zappy-hooks` unit tests:

- command construction;
- argument passing;
- working directory handling;
- environment handling;
- optional command failure behavior;
- stdout/stderr capture.

`zappy-adapters` unit tests:

- adapter availability checks;
- recipe-step modeling;
- Git adapter behavior if implemented early.

`zappy-cli` unit tests:

- argument parsing;
- alias behavior;
- `--var key=value` parsing;
- non-interactive error selection;
- output formatting helpers.

### 19.2 Integration Tests

CLI integration tests using `assert_cmd` should usually live at the root package level because the root package owns the shipped `zappy` binary.

Test cases:

- `zappy list` finds fixture templates;
- `zappy ls` behaves like `zappy list`;
- `zappy info` prints expected metadata;
- `zappy new` generates expected files;
- `zappy gen` behaves like `zappy new`;
- `zappy new --dry-run` writes nothing;
- `zappy new --non-interactive` fails on missing required variables;
- `zappy new --var key=value` resolves variables correctly;
- `zappy validate` succeeds on good fixture;
- `zappy validate` fails on intentionally broken fixture.

### 19.3 Fixture Templates

Create fixture templates:

```text
tests/fixtures/templates/
├── minimal/
├── with-transforms/
├── with-conditionals/
├── with-binary-file/
├── with-hooks/
├── invalid-manifest/
└── failing-validation/
```

### 19.4 Snapshot Testing

Consider `insta` later for:

- dry-run plans;
- list output;
- info output;
- diagnostics.

Do not introduce snapshots before the CLI output stabilizes.

---

## 20. Phased Implementation Plan

The phases below assume the current workspace already contains these crates:

- root `zappy` package;
- `crates/zappy-core`;
- `crates/zappy-fs`;
- `crates/zappy-hooks`;
- `crates/zappy-adapters`;
- `crates/zappy-cli`.

Each phase should keep crate boundaries clean. A coding agent should be asked to work inside the intended crate instead of placing everything into `zappy-cli` or `zappy-core` by default.

## Phase 0: Workspace Wiring and CLI Shell

Goal: make the existing workspace compile with a thin root binary and empty-but-documented internal crates.

Tasks:

1. Ensure every workspace member has a valid `Cargo.toml`.
2. Ensure every internal crate has a `src/lib.rs`.
3. Add crate-level docs describing each crate's responsibility.
4. Add `zappy-cli` as a dependency of the root `zappy` crate.
5. Implement root `src/main.rs` as a tiny call into `zappy_cli::run()`.
6. Implement an initial `zappy-cli` command shell using `clap`.
7. Add top-level commands as stubs:
   - `list` / `ls`;
   - `info`;
   - `new` / `gen`;
   - `validate`;
   - `init-template`;
   - `create`.
8. Add baseline `just` recipes if desired:
   - `just fmt`;
   - `just clippy`;
   - `just test`;
   - `just check`.
9. Keep `zappy-adapters`, `zappy-fs`, and `zappy-hooks` compiling even if they only contain placeholder modules initially.

Acceptance criteria:

- `cargo build --workspace` succeeds;
- `cargo test --workspace` succeeds;
- `cargo run -- --help` prints root CLI help;
- `cargo run -- list` reaches the stub implementation;
- `cargo run -- ls` reaches the same stub implementation;
- the root crate contains no real scaffolding logic.

Suggested agent prompt:

```text
Implement Phase 0 of the Zappy plan using the existing workspace layout. Keep the root zappy crate as a thin binary facade over zappy-cli. Add stub commands in zappy-cli for list/ls, info, new/gen, validate, init-template, and create. Ensure cargo build/test --workspace pass. Do not implement template parsing or generation yet.
```

---

## Phase 1: Manifest Parsing and Validation in `zappy-core`

Goal: parse `zappy.toml` into a validated internal manifest model.

Crate focus: `zappy-core`.

Tasks:

1. Define raw deserialization structs for `zappy.toml`.
2. Define validated internal structs.
3. Implement `Manifest::load_from_path` or equivalent.
4. Validate required fields:
   - `template.id`;
   - `template.name`;
   - source root defaults to `template/`;
   - variable names are valid;
   - placeholders are not empty;
   - validation steps have commands.
5. Model manifest sections for:
   - template metadata;
   - variables;
   - transforms;
   - conditionals;
   - hooks;
   - validation;
   - excludes;
   - binary handling.
6. Add unit tests for valid and invalid manifests.

Acceptance criteria:

- a minimal fixture manifest parses;
- invalid manifests produce useful errors;
- `zappy-core` exposes a stable manifest-loading API;
- no CLI behavior depends on filesystem traversal yet.

Suggested agent prompt:

```text
Implement Phase 1 in zappy-core: zappy.toml manifest parsing and validation. Add raw serde structs, validated domain structs, errors, and unit tests. Do not implement template discovery, filesystem walking, or CLI output beyond what already exists.
```

---

## Phase 2: Template Discovery and Listing via `zappy-fs` + `zappy-cli`

Goal: discover available templates and list them via CLI.

Crate focus:

- `zappy-fs`: filesystem search paths and discovery;
- `zappy-core`: template identity/domain structs if needed;
- `zappy-cli`: command output.

Tasks:

1. Implement template search path resolution in `zappy-fs`.
2. Support `--templates-dir` in `zappy-cli`.
3. Support `ZAPPY_TEMPLATES_DIR`.
4. Support old-style `ZAPPY_CONFIG/templates`.
5. Support platform config directory.
6. Support executable-relative and CWD `templates/` fallback.
7. Implement duplicate ID shadowing rules.
8. Implement `zappy list`.
9. Add alias `zappy ls`.
10. Implement `zappy info <template>`.
11. Add fixture templates under an integration-test fixture directory.

Acceptance criteria:

- `zappy list --templates-dir tests/fixtures/templates` shows fixture templates;
- `zappy info minimal` prints manifest details;
- duplicate templates are handled deterministically;
- CLI integration tests pass;
- discovery logic does not live in the root crate.

Suggested agent prompt:

```text
Implement Phase 2: template discovery in zappy-fs and list/info commands in zappy-cli. Use the manifest loader from zappy-core. Support --templates-dir, ZAPPY_TEMPLATES_DIR, ZAPPY_CONFIG/templates, platform config dirs, executable-relative templates, and CWD templates. Add integration tests with fixture templates.
```

---

## Phase 3: Variables and Transforms in `zappy-core`

Goal: resolve generation variables from CLI, defaults, built-ins, and interactive prompts.

Crate focus:

- `zappy-core`: variable model, precedence, transforms;
- `zappy-cli`: parsing `--var`, prompting, non-interactive errors.

Tasks:

1. Implement `VariableValue` parsing in `zappy-core`.
2. Implement variable source/precedence model in `zappy-core`.
3. Implement CLI `--var key=value` collection in `zappy-cli`.
4. Implement project-name injection from positional arg.
5. Implement built-ins:
   - project name;
   - user;
   - date/day/month/year.
6. Implement transforms using `heck`.
7. Implement validation regex if regex dependency is accepted now; otherwise leave TODO.
8. Implement non-interactive missing-variable errors.
9. Implement interactive prompting in `zappy-cli`.

Acceptance criteria:

- variables resolve with correct precedence;
- transforms are generated correctly;
- missing required variables fail in non-interactive mode;
- interactive mode prompts for missing required variables;
- unit tests cover precedence and transforms;
- prompt code is not placed in `zappy-core`.

Suggested agent prompt:

```text
Implement Phase 3: variable resolution and transforms in zappy-core, and CLI --var/project-name/prompt integration in zappy-cli. Add tests for precedence and transformation behavior. Keep filesystem generation stubbed.
```

---

## Phase 4: Render Plan and Dry Run Across `zappy-core` + `zappy-fs`

Goal: build a dry-run render plan without writing files.

Crate focus:

- `zappy-core`: render-plan types, placeholder rendering, path rendering, condition evaluation;
- `zappy-fs`: deterministic traversal, binary/text classification, output conflict detection;
- `zappy-cli`: dry-run output.

Tasks:

1. Traverse template source directory deterministically in `zappy-fs`.
2. Apply excludes.
3. Apply conditionals.
4. Render destination paths using `zappy-core`.
5. Detect binary vs text files in `zappy-fs`.
6. Render text file contents into memory.
7. Detect output conflicts.
8. Represent all operations in `GenerationPlan`.
9. Implement `zappy new <template> <project-name> --dry-run`.

Acceptance criteria:

- dry-run prints planned operations;
- no files are written in dry-run mode;
- path placeholders are replaced;
- text placeholders are replaced;
- conditionals skip expected files;
- binary files are copied in plan, not rendered;
- output conflicts are reported before writing.

Suggested agent prompt:

```text
Implement Phase 4: GenerationPlan and dry-run rendering using zappy-core for render semantics and zappy-fs for traversal/classification/conflict checks. Add zappy new --dry-run output in zappy-cli. Do not write files yet.
```

---

## Phase 5: Filesystem Generation in `zappy-fs`

Goal: execute the render plan and generate projects.

Crate focus:

- `zappy-fs`: materialization of `GenerationPlan`;
- `zappy-cli`: command flow and success output.

Tasks:

1. Create directories.
2. Write rendered text files.
3. Copy binary files.
4. Preserve executable bits on Unix if feasible.
5. Respect overwrite behavior.
6. Implement `--force`.
7. Implement clear success output.
8. Add integration tests that inspect generated output.

Acceptance criteria:

- `zappy new minimal my-app` generates a complete project;
- existing files are not overwritten by default;
- `--force` overwrites files;
- binary files round-trip unchanged;
- generated content matches expected output;
- file-writing code is not implemented in `zappy-cli`.

Suggested agent prompt:

```text
Implement Phase 5: execute GenerationPlan in zappy-fs to write directories, rendered text files, and binary files. Wire --force and success output through zappy-cli. Add integration tests that compare generated fixture output.
```

---

## Phase 6: Hooks in `zappy-hooks`

Goal: run manifest-defined pre/post generation hooks safely.

Crate focus:

- `zappy-hooks`: command execution;
- `zappy-core`: hook config model if missing;
- `zappy-cli`: `--no-hooks` flag and diagnostics.

Tasks:

1. Implement hook data model if not already complete.
2. Implement direct command execution with `std::process::Command`.
3. Support args.
4. Support working directory.
5. Support environment variables.
6. Support `optional = true`.
7. Support `when = "bool_var"`.
8. Implement `--no-hooks`.
9. Add tests with harmless commands.

Acceptance criteria:

- pre-generation hook can run before writes;
- post-generation hook can run after successful generation;
- failing required hook fails generation;
- failing optional hook warns but does not fail;
- hooks are skipped with `--no-hooks`;
- command execution code is not implemented in `zappy-cli`.

Suggested agent prompt:

```text
Implement Phase 6: pre/post generation hooks in zappy-hooks using direct command execution. Add optional hooks, conditional hooks, --no-hooks wiring in zappy-cli, and tests with harmless fixture commands.
```

---

## Phase 7: Validation Command via `zappy-hooks`

Goal: validate templates by generating into a temporary directory and running manifest-defined checks.

Crate focus:

- `zappy-core`: validation config model;
- `zappy-fs`: tempdir helpers and generation materialization;
- `zappy-hooks`: validation command execution;
- `zappy-cli`: `validate` command output.

Tasks:

1. Implement validation config model.
2. Generate into a tempdir using validation variables.
3. Run setup commands.
4. Run validation steps.
5. Run teardown commands.
6. Capture stdout/stderr.
7. Implement `--keep-temp`.
8. Implement useful failure reporting.
9. Add passing and failing fixture templates.

Acceptance criteria:

- `zappy validate minimal` succeeds;
- `zappy validate failing-validation` fails clearly;
- validation tempdir is cleaned up by default;
- `--keep-temp` preserves tempdir and prints path;
- validation uses non-interactive generation;
- validation command execution lives in `zappy-hooks`.

Suggested agent prompt:

```text
Implement Phase 7: zappy validate. Use zappy-fs to generate into a temporary directory and zappy-hooks to run setup/steps/teardown from the manifest. Report failures with stdout/stderr and support --keep-temp.
```

---

## Phase 8: Template Creation Helpers Across `zappy-fs` + `zappy-cli`

Goal: reintroduce old Zappy's template creation features.

Crate focus:

- `zappy-fs`: copying existing projects into template payload directories;
- `zappy-core`: starter manifest model/serialization if supported;
- `zappy-cli`: user-facing `init-template` and `create` commands.

Tasks:

1. Implement `zappy init-template <path>`.
2. Add compatibility alias `zappy create -e`.
3. Implement `zappy create --from <project>`.
4. Copy project into `template/` while excluding junk directories.
5. Generate starter `zappy.toml`.
6. Optionally replace provided variable values with placeholders.
7. Add docs explaining manual cleanup/editing.

Acceptance criteria:

- empty template skeleton can be created;
- existing project can be copied into a new template skeleton;
- common generated directories are excluded;
- starter manifest is valid;
- created template appears in `zappy list`.

Suggested agent prompt:

```text
Implement Phase 8: zappy init-template and zappy create --from. Put project-copy/template-skeleton filesystem logic in zappy-fs, manifest helpers in zappy-core, and command UX in zappy-cli. Keep template inference conservative.
```

---

## Phase 9: Bundled Starter Templates

Goal: add practical templates that match actual recurring workflows.

Initial templates:

1. `rust-cli`
   - Clap-based CLI;
   - tracing/logging;
   - strict clippy;
   - README;
   - CI config optional.

2. `cpp-cmake-app`
   - modern CMake;
   - `src/`, `include/`, `tests/`;
   - optional clang-format;
   - optional GitLab CI.

3. `cpp-cmake-lib`
   - installable CMake library;
   - exported target;
   - tests;
   - package config.

4. `zephyr-app`
   - `CMakeLists.txt`;
   - `prj.conf`;
   - board overlay placeholder;
   - optional sysbuild.

5. `zephyr-cpp-app`
   - C++ oriented Zephyr app;
   - module-friendly layout;
   - app sources under `src/`;
   - C++ standard config.

6. `zephyr-module`
   - out-of-tree Zephyr module layout;
   - `zephyr/module.yml`;
   - `CMakeLists.txt`;
   - `Kconfig`;
   - include/src layout.

7. `nvim-plugin-lua`
   - Lua plugin skeleton;
   - `lua/<plugin>/init.lua`;
   - README;
   - stylua config.

8. `python-uv-cli`
   - `uv`-based Python project;
   - `pyproject.toml`;
   - src layout;
   - ruff/pyright optional.

Acceptance criteria:

- every bundled template validates;
- every bundled template has realistic validation variables;
- every bundled template has a README or generated README section.

Suggested agent prompt:

```text
Implement Phase 9 one template at a time. Start with rust-cli, add validation, and make zappy validate rust-cli pass. Do not add all templates in one large change.
```

---

## Phase 10: Documentation and Release Polish

Goal: make the project usable by future-you and coding agents.

Docs to write:

- `README.md`;
- `docs/manifest.md`;
- `docs/template-authoring.md`;
- `docs/validation.md`;
- `docs/migration-from-lua-zappy.md`;
- `docs/agent-guide.md`;
- `CHANGELOG.md`.

CLI polish:

- nicer tables for `list`;
- helpful `info` output;
- verbose logging with `-v`, `-vv`, `-vvv`;
- `--format json` later;
- shell completions later.

Release polish:

- GitLab CI or GitHub Actions;
- release binaries;
- `cargo install` path;
- version metadata;
- template packaging strategy.

Acceptance criteria:

- new user can install/build locally and generate first project from docs;
- template author can create and validate a template from docs;
- coding agent can follow `docs/agent-guide.md` without needing conversation context.

---

## 21. Recipe / Orchestrator Mode

This should be a later feature, not MVP.

The idea: some ecosystems already have generators. Zappy should be able to orchestrate them, then apply overlays.

Example future manifest:

```toml
[template]
id = "python-fastapi-uv"
name = "Python FastAPI project with uv"
language = "python"

[recipe]
mode = "steps"

[[recipe.steps]]
name = "Initialize uv project"
command = "uv"
args = ["init", "--package", "__ZAPPY_PROJECT_NAME_KEBAB__"]

[[recipe.steps]]
name = "Apply overlay"
copy_overlay = "overlay"
```

Do not implement this until static templates and validation are stable.

---

## 22. Old Zappy Migration Plan

### 22.1 Preserve Concepts, Not Lua Execution

Do not execute old Lua templates in the Rust engine.

Instead:

- document how old concepts map to new manifest sections;
- optionally create a migration helper later;
- manually port important templates first.

### 22.2 Concept Mapping

| Old Lua Zappy | Zappy RS |
|---|---|
| Lua template file | template directory + `zappy.toml` |
| `name` | `[template].name` |
| `desc` | `[template].description` |
| `args` | `[variables.*]` |
| `structure` table | files under `template/` |
| `#{VAR}` substitution | literal placeholders such as `__ZAPPY_VAR__` |
| `hooks.pre` | `[[hooks.pre_generate]]` |
| `hooks.post` | `[[hooks.post_generate]]` |
| `zappy gen` | `zappy new` / `zappy gen` alias |
| `zappy ls` | `zappy list` / `zappy ls` alias |
| `zappy create -e` | `zappy init-template` / compatibility alias |
| `$ZAPPY_CONFIG/templates` | supported template search path |
| `zconfig.lua` | `config.toml` |

### 22.3 Port Order

Port templates in this order:

1. `cpp_cm` -> `cpp-cmake-app`
2. `cpp_lib_cm` -> `cpp-cmake-lib`
3. `zos_cpp` -> `zephyr-cpp-app`
4. `zos_module_cpp` -> `zephyr-cpp-module`
5. `nvim_plug` -> `nvim-plugin-lua`
6. `cpp_m`, `zos`, `zos_module` if still useful

---

## 23. Coding Agent Guidelines

### 23.1 General Rules

A coding agent should:

- implement one phase at a time;
- keep changes small;
- add tests with every behavior change;
- avoid adding features outside the current phase;
- prefer explicit data structures over clever abstractions;
- keep CLI behavior thin and core behavior testable;
- never write directly to output paths without going through the render plan;
- keep filesystem safety checks centralized;
- preserve deterministic traversal and output ordering.

### 23.2 Useful Agent Loop

For each phase:

1. Read this plan.
2. Read relevant existing modules.
3. Implement the smallest vertical slice.
4. Run `cargo fmt`.
5. Run `cargo clippy --all-targets --all-features -- -D warnings`.
6. Run `cargo test`.
7. Add or update docs if public behavior changed.
8. Summarize what changed and what remains.

### 23.3 Do Not Let the Agent Do This Too Early

Avoid early implementation of:

- plugin system;
- expression language for conditionals;
- full Jinja/Handlebars-style templating;
- YAML support;
- remote template registries;
- recipe/orchestrator mode;
- shell completions;
- JSON output;
- old Lua template execution;
- auto-inference of complex template variables.

These are all reasonable later, but they will slow down the MVP.

---

## 24. MVP Definition

The MVP is complete when the following works:

```bash
zappy list --templates-dir ./templates
zappy info rust-cli --templates-dir ./templates
zappy new rust-cli my-tool --templates-dir ./templates
zappy new rust-cli my-tool --templates-dir ./templates --dry-run
zappy validate rust-cli --templates-dir ./templates
```

Required MVP features:

- manifest parsing;
- template discovery;
- list/info commands;
- variable defaults;
- CLI `--var` overrides;
- project name positional arg;
- basic built-ins;
- string transforms;
- path/content placeholder replacement;
- excludes;
- binary copying;
- conflict detection;
- dry-run;
- safe filesystem writes;
- validation steps;
- at least one bundled template with passing validation.

Not required for MVP:

- `create --from`;
- interactive prompts;
- conditionals;
- hooks;
- recipe mode;
- config file;
- shell completions;
- JSON output;
- remote templates.

However, the architecture should leave room for these features.

---

## 25. Recommended First Commit Sequence

1. `chore: bootstrap rust workspace`
2. `feat(core): parse zappy template manifests`
3. `feat(core): discover templates from explicit directory`
4. `feat(cli): add list and info commands`
5. `feat(core): resolve variables and transforms`
6. `feat(core): build generation dry-run plan`
7. `feat(cli): add new --dry-run`
8. `feat(core): execute generation plan`
9. `feat(cli): generate projects from templates`
10. `feat(core): add template validation steps`
11. `feat(cli): add validate command`
12. `docs: add template authoring guide`
13. `templates: add rust-cli starter template`

---

## 26. Open Design Decisions

These can be decided during implementation:

1. Should the primary command be `new` or `gen`?
   - Recommendation: primary `new`, compatibility alias `gen`.

2. Should the manifest be TOML-only initially?
   - Recommendation: yes.

3. Should the old `#{VAR}` syntax be supported in MVP?
   - Recommendation: no, but reserve a future compatibility mode.

4. Should hooks run by default?
   - Recommendation: yes for post-generation, but provide `--no-hooks` once hooks exist.

5. Should `git init` be a built-in flag or a hook?
   - Recommendation: both eventually. MVP can skip. Later, `--git` maps to a built-in post-generation action, while templates can still define their own hook.

6. Should validation run hooks?
   - Recommendation: validation should run generation hooks by default only if they are part of normal generation, but provide a way to disable or mark hooks as `validation = false` later.

7. Should templates be allowed to write outside the output directory?
   - Recommendation: never by default.

---

## 27. Summary

The Rust reimplementation should become a safer, more maintainable, more extensible Zappy rather than a direct Lua rewrite.

The key architectural idea is:

```text
manifest + variables + render plan + safe filesystem executor + validation
```

The key MVP path is:

```text
parse manifest -> discover templates -> resolve vars -> dry-run plan -> generate -> validate
```

Once that is solid, the project can grow naturally into:

- template creation helpers;
- more bundled templates;
- recipe/orchestration mode;
- richer conditionals;
- remote template catalogs;
- better machine-readable output for agents and CI.

