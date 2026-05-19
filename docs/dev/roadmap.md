# Zappy Release Roadmap

This document describes the planned post-`v0.1.0` roadmap for Zappy.

`v0.1.0` established the MVP foundation: a Rust workspace, modular crates, bundled templates, generation plans, template validation, documentation, CI, docs publishing, and release automation.

The next releases should remain focused. Each release should have one clear theme and should avoid pulling in unrelated ideas that would dilute the release goal.

```text
v0.2.0 — Usability release
v0.3.0 — Template authoring release
v0.4.0 — Configuration and ecosystem release
```

---

## 1. Current Baseline: `v0.1.0`

### 1.1 Implemented User-Facing Commands

`v0.1.0` provides the following command surface:

```text
zappy list
zappy info
zappy new
zappy validate
zappy init
zappy create
```

The `create` command exists, but deeper project-to-template conversion is still planned for a later release.

### 1.2 Implemented Core Concepts

The MVP includes:

- template discovery;
- bundled templates;
- template manifests via `zappy.toml`;
- manifest variables;
- typed variable values;
- variable transforms;
- built-in variables;
- conditional file inclusion;
- render plans;
- dry-run generation;
- generation materialization;
- pre/post generation hooks;
- validation hooks;
- template skeleton initialization;
- bundled-template cache extraction;
- mdBook-based user documentation;
- GitLab CI for tests, docs, validation, and release automation.

### 1.3 MVP Crate Responsibilities

The existing crate boundaries should continue to guide the roadmap.

```text
zappy-core
  Pure domain model and deterministic rendering semantics.

zappy-fs
  Filesystem discovery, traversal, classification, path safety, materialization,
  and template skeleton/project copy operations.

zappy-hooks
  Command execution for generation and validation hooks.

zappy-templates
  Embedded bundled templates and cache preparation.

zappy-cli
  Clap parser, command orchestration, terminal UX, prompts, logging, diagnostics.

zappy-adapters
  Future ecosystem-specific adapter layer.

zappy
  Thin root binary/facade crate.
```

---

## 2. Roadmap Principles

### 2.1 Keep Releases Thematic

Each release should have a single primary promise:

```text
v0.2.0:
  Zappy is pleasant to use.

v0.3.0:
  Zappy is pleasant to author templates for.

v0.4.0:
  Zappy is pleasant to customize as a personal tool.
```

### 2.2 Preserve Crate Boundaries

Feature work should respect existing crate responsibilities.

Examples:

- interactive prompts belong in `zappy-cli`, not `zappy-core`;
- regex validation semantics belong in `zappy-core`;
- project copying belongs in `zappy-fs`;
- hook shell execution belongs in `zappy-hooks`;
- bundled-template cache details belong in `zappy-templates`;
- user-facing diagnostics and formatting belong in `zappy-cli`.

### 2.3 Prefer Explicit Behavior Over Magic

Zappy should remain predictable.

Avoid early implementation of:

- remote template registries;
- plugin systems;
- full expression languages;
- Jinja/Handlebars-style template languages;
- complex automatic variable inference;
- old Lua template execution;
- hidden project-specific heuristics.

These can be revisited later once the core model is stable.

### 2.4 Keep Validation and Linting Separate

Template validation and template linting should not mean the same thing.

```text
zappy validate
  Generate a project and run validation hooks.

zappy lint
  Analyze the template and manifest without generating/building the project.
```

This distinction should become explicit in `v0.3.0`.

---

## 3. Release: `v0.2.0` — Usability Release

### 3.1 Goal

Make existing Zappy workflows pleasant, clear, and predictable.

This release should improve the commands that already exist, especially:

- `zappy new`;
- `zappy validate`;
- `zappy list`;
- `zappy info`.

### 3.2 Summary

`v0.2.0` should focus on terminal UX rather than expanding Zappy's conceptual model.

Core areas:

1. polished diagnostics and error messages;
2. real verbosity/logging;
3. string variable regex validation;
4. interactive generation;
5. better dry-run output.

### 3.3 Crate Focus

```text
zappy-core
  Regex validation semantics for string variables.
  Additional error variants where needed.

zappy-cli
  Diagnostic formatting, logging setup, verbosity handling, prompts,
  interactive resolution, improved dry-run output.

zappy-fs
  Expose enough discovery/search-path details for diagnostics.

zappy-hooks
  Mostly unchanged in this release.

zappy-templates
  Mostly unchanged in this release.
```

---

### 3.4 Task Group A: Diagnostics Layer

#### Goal

Make errors actionable.

Instead of terse errors, Zappy should explain:

- what failed;
- where it happened;
- what paths were searched;
- what the user can try next.

#### Example

```text
Error: template `foo` was not found

Searched:
  explicit templates dir: ./templates
  bundled templates: ~/.cache/zappy/bundled-templates/0.1.0/templates
  config templates: ~/.config/zappy/templates
  current directory: ./templates

Hint:
  run `zappy list` to see available templates
```

#### Suggested Implementation

Keep crate-level errors as domain errors. Add a CLI-facing reporting layer in `zappy-cli`.

Possible module layout:

```text
crates/zappy-cli/src/diagnostics.rs
crates/zappy-cli/src/logging.rs
```

Potential internal types:

```rust
pub(crate) struct DiagnosticReport {
    pub title: String,
    pub details: Vec<String>,
    pub hints: Vec<String>,
}
```

This does not need a full diagnostic framework immediately. A lightweight formatter is acceptable for `v0.2.0`.

#### Acceptance Criteria

- missing template errors show searched paths;
- invalid variable errors show variable name and value;
- hook failures show command, exit status, stdout/stderr summary;
- validation failures clearly distinguish generation hooks from validation hooks;
- diagnostics are printed to stderr.

---

### 3.5 Task Group B: Verbosity and Logging

#### Goal

Make `-v`, `-vv`, and `-vvv` meaningful.

#### Suggested Verbosity Mapping

```text
default:
  user-facing output and warnings

-v:
  info-level operational messages

-vv:
  debug-level diagnostic messages

-vvv:
  trace-level diagnostic messages
```

#### Suggested Logging Stack

Use:

- `tracing`;
- `tracing-subscriber`.

Default command output should remain clean. Logs should go to stderr.

#### Examples

Default:

```text
Generated `rust-cli` in ./my-tool
Created 4 directories, wrote 12 text files, copied 1 binary file, skipped 2 paths.
```

Verbose:

```text
info: discovered 5 templates
info: selected template `rust-cli` from bundled templates
info: resolved 8 variables
info: built generation plan with 19 operations
```

Debug:

```text
debug: search path bundled templates: ~/.cache/zappy/...
debug: skipping `.gitlab-ci.yml`: condition `use_gitlab_ci` is false
debug: rendering `Cargo.toml` -> `Cargo.toml`
```

#### Acceptance Criteria

- verbosity flag is wired to logging;
- logs go to stderr;
- normal command output remains stable and script-friendly;
- tests cover logging setup only where practical;
- docs explain verbosity levels.

---

### 3.6 Task Group C: String Variable Regex Validation

#### Goal

Implement the existing `validation_regex` manifest field.

#### Semantics

Regex validation should apply to string values.

Suggested behavior:

```text
VariableValue::String(value)
  validate using regex.

VariableValue::Bool(_)
VariableValue::Integer(_)
  either skip regex validation or fail with a clear type error.
```

Recommended MVP behavior: fail if regex validation is configured for a non-string value. This keeps template mistakes visible.

#### Example Manifest

```toml
[variables.package_name]
required = true
validation_regex = "^[a-z][a-z0-9_-]*$"
```

#### Resolution Timing

Validation should run after source resolution and before render data is built.

Suggested order:

```text
1. validate input sources
2. resolve declared variable values
3. validate choices
4. validate regex
5. validate required_when
6. validate conflicts_with
7. build render data
8. inject built-ins
```

#### Crate Placement

```text
zappy-core
  regex validation logic and errors.

zappy-cli
  interactive re-prompting when regex validation fails.
```

#### Tests

Add tests for:

- valid string passes;
- invalid string fails;
- missing optional string with regex passes;
- required string with regex fails when missing;
- non-string value with regex fails with a clear error;
- interactive mode re-prompts after invalid string input.

#### Acceptance Criteria

- `validation_regex` is functional;
- invalid regex patterns fail manifest parsing or manifest validation clearly;
- invalid values fail variable resolution clearly;
- docs describe regex syntax and timing.

---

### 3.7 Task Group D: Interactive Generation

#### Goal

Allow Zappy to ask for missing required values.

This should be a terminal prompt flow, not a full TUI.

#### Prompt Conditions

Prompt when:

```text
required = true
  and variable is unresolved

required_when = "some_bool"
  and condition evaluates true
  and variable is unresolved or empty
```

#### Prompt Types

```text
string variable:
  text prompt

boolean variable:
  confirm prompt

integer variable:
  text prompt + parse integer

choices:
  selection prompt

regex validated string:
  text prompt + regex validation + retry
```

#### Suggested Dependency

Consider one of:

- `inquire`;
- `dialoguer`.

Choose the dependency based on simplicity, testability, and maintenance status.

#### Non-Interactive Mode

`--non-interactive` should continue to fail if required values are missing.

Interactive mode should be the default only if stdin is a terminal. If stdin is not interactive, default to non-interactive behavior or produce a clear error.

#### Suggested Internal Flow

```text
1. Resolve variables without prompts.
2. Detect missing required/conditionally-required variables.
3. Prompt for those values.
4. Resolve variables again with interactive values included.
5. Continue generation.
```

This avoids embedding prompt logic into `zappy-core`.

#### Acceptance Criteria

- missing required variables are prompted interactively;
- missing conditionally required variables are prompted only when their condition is true;
- choices are presented as choices;
- booleans are prompted as confirmations;
- invalid regex input is rejected and re-prompted;
- `--non-interactive` remains deterministic;
- prompt logic lives in `zappy-cli`.

---

### 3.8 Task Group E: Better Dry-Run Output

#### Goal

Make dry-run output useful as a generation preview.

#### Example Output

```text
Generation plan for `rust-cli`
Output directory: ./my-tool

Would create:
  dir  src/
  file Cargo.toml
  file src/main.rs
  file README.md

Would copy:
  file Cargo.lock

Would skip:
  .gitlab-ci.yml        condition use_gitlab_ci=false
  .github/workflows/ci.yml condition use_github_actions=false

Warnings:
  output exists: README.md
```

#### CLI Behavior

Dry-run should never write files or run hooks.

If hooks are currently run during dry-run, that should be treated as a bug.

#### Acceptance Criteria

- dry-run groups operations by operation type;
- skipped paths include skip reasons;
- warnings are visible;
- dry-run does not run hooks;
- dry-run does not create output directories or files.

---

### 3.9 v0.2.0 Documentation Tasks

Update the Zappy Book with:

- interactive generation;
- `--non-interactive` behavior;
- regex validation;
- verbosity levels;
- improved dry-run examples;
- better error examples.

### 3.10 v0.2.0 Suggested Agent Prompt

```text
Implement the v0.2.0 usability release for Zappy. Add a CLI diagnostics layer,
wire verbosity to tracing/logging, implement string variable regex validation in
zappy-core, add interactive prompting for missing required and conditionally
required variables in zappy-cli, and improve dry-run output. Keep prompt logic
out of zappy-core and preserve existing non-interactive behavior. Add tests and
update the Zappy Book.
```

### 3.11 v0.2.0 Release Checklist

```text
cargo fmt --check
cargo clippy --tests --examples --all-targets --all-features --workspace -- -D warnings
cargo test --workspace
cargo test --workspace --doc
just test-templates
just book-check
manual smoke test: zappy list
manual smoke test: zappy info -t rust-cli
manual smoke test: zappy new -t rust-cli -n demo
manual smoke test: zappy new -t rust-cli -n demo --dry-run
manual smoke test: missing required variable interactive prompt
```

---

## 4. Release: `v0.3.0` — Template Authoring Release

### 4.1 Goal

Make creating, maintaining, validating, and debugging templates robust.

This release should focus on template authors rather than end users.

### 4.2 Summary

Core areas:

1. template generation from existing projects;
2. template linting;
3. placeholder consistency checks;
4. proper shell hook support;
5. cache management command.

### 4.3 Crate Focus

```text
zappy-core
  Lint data structures for manifest-level checks if needed.

zappy-fs
  Copy existing project into template skeleton.
  Walk template sources for linting and placeholder checks.

zappy-hooks
  Implement shell=true execution semantics.

zappy-templates
  Expose cache status, path, clear, and rebuild operations.

zappy-cli
  Add lint and cache commands.
  Improve create --from orchestration.
```

---

### 4.4 Task Group A: Real `create --from`

#### Goal

Turn an existing project into a starter Zappy template.

#### Command

```bash
zappy create --from ./my-project -o ./templates/my-template
```

#### Initial Behavior

The initial implementation should be conservative and predictable.

It should:

- create a template skeleton;
- copy the source project into the skeleton's `template/` directory;
- generate a starter `zappy.toml`;
- exclude common generated directories and files;
- avoid aggressive variable inference.

#### Suggested Excludes

```text
.git
.hg
.svn
target
build
cmake-build-*
node_modules
.venv
venv
__pycache__
.DS_Store
*.pyc
```

The exact exclude model should be deterministic and documented.

#### Future Extensions

Defer to later releases:

- infer project name placeholders automatically;
- infer package metadata;
- infer language/ecosystem;
- detect toolchains;
- apply ecosystem-specific adapters.

#### Acceptance Criteria

- `create --from` creates a valid template directory;
- copied files appear under `template/`;
- generated `zappy.toml` parses;
- common generated directories are skipped;
- the created template appears in `zappy list -i <templates-dir>`;
- the created template can be generated with `zappy new`.

---

### 4.5 Task Group B: `zappy lint`

#### Goal

Add a lightweight static template analysis command.

`zappy lint` should not generate the project and should not run hooks.

#### Commands

```bash
zappy lint -t rust-cli
zappy lint -i ./templates -t my-template
```

#### Initial Checks

Manifest-level checks:

- manifest parses;
- variable names are valid;
- conditionals reference known variables;
- `required_when` references known variables;
- `conflicts_with` references known variables;
- validation variables reference known variables;
- hooks have non-empty commands;
- hook working directories are safe relative paths.

Template-source checks:

- source root exists;
- conditional paths exist;
- `binary_files` entries exist;
- optional warning when exclude patterns match nothing;
- optional warning when binary extension entries look suspicious.

#### Output Levels

Lint should distinguish errors and warnings.

```text
error:
  invalid manifest
  missing conditional path
  unknown variable reference

warning:
  placeholder defined but unused
  exclude pattern matched nothing
  binary file entry matched nothing
```

#### Suggested Data Model

```rust
pub struct LintReport {
    pub errors: Vec<LintDiagnostic>,
    pub warnings: Vec<LintDiagnostic>,
}

pub struct LintDiagnostic {
    pub code: &'static str,
    pub message: String,
    pub path: Option<PathBuf>,
}
```

This could live in `zappy-core` if purely semantic, or in `zappy-fs` if it depends heavily on filesystem traversal. Keep user-facing formatting in `zappy-cli`.

#### Acceptance Criteria

- `zappy lint` exits successfully with no errors;
- lint errors produce non-zero exit code;
- warnings are shown but do not fail by default;
- lint does not run hooks;
- lint does not create output files;
- docs explain lint versus validate.

---

### 4.6 Task Group C: Placeholder Consistency Checks

#### Goal

Catch common template authoring mistakes around placeholders.

#### Checks

Detect:

- placeholder defined in manifest but never used;
- placeholder used in template files but not provided by any variable or built-in;
- suspicious unresolved `__ZAPPY_...__` placeholders;
- malformed placeholder conventions if applicable.

#### Design Note

Zappy currently uses literal replacement rather than a full template language. Placeholder checks should preserve that simplicity.

Initial placeholder detection can be conservative:

```text
scan text files for tokens that look like __ZAPPY_...__
compare against resolved replacement keys known from variables and built-ins
```

Avoid parsing arbitrary placeholder syntaxes in this release.

#### Acceptance Criteria

- unused placeholders are reported as warnings;
- unknown `__ZAPPY_...__` tokens are reported as warnings or errors;
- binary files are not scanned as text;
- docs explain placeholder linting limitations.

---

### 4.7 Task Group D: Proper Shell Hook Support

#### Goal

Implement `shell = true` for hooks.

#### Manifest Example

```toml
[[hooks.post_generate]]
name = "Format project"
command = "cargo fmt && cargo clippy"
shell = true
```

#### Semantics

Suggested MVP behavior:

```text
Unix:
  sh -c "<rendered command>"

Windows:
  cmd /C "<rendered command>"
```

Direct command execution should remain the default.

#### Safety Notes

Shell hooks are inherently less deterministic and less safe than direct command hooks. Documentation should clearly explain this.

#### Future Extensions

Defer shell selection:

```toml
shell = "bash"
shell = "powershell"
shell = "cmd"
```

#### Acceptance Criteria

- `shell = true` no longer returns unsupported-shell errors;
- shell hooks render variables before execution;
- shell hooks respect hook working directory;
- shell hooks receive rendered environment variables;
- direct command hooks continue to work unchanged;
- tests cover command construction without requiring platform-specific shells where possible.

---

### 4.8 Task Group E: Cache Management Command

#### Goal

Expose bundled-template cache management to users.

#### Commands

```bash
zappy cache path
zappy cache status
zappy cache clear
zappy cache rebuild
```

#### Command Behavior

```text
zappy cache path
  Print the cache path.

zappy cache status
  Print cache path, marker status, expected hash, installed hash if available.

zappy cache clear
  Remove bundled-template cache.

zappy cache rebuild
  Clear and re-extract bundled templates.
```

#### Crate Responsibilities

```text
zappy-templates
  cache path, status, clear, rebuild APIs.

zappy-cli
  command parser and user-facing output.
```

#### Acceptance Criteria

- cache path is printed reliably;
- status explains whether cache is ready/stale/missing;
- clear removes cache;
- rebuild recreates cache;
- normal commands remain idempotent and avoid deleting live caches unnecessarily.

---

### 4.9 v0.3.0 Documentation Tasks

Update the Zappy Book with:

- `create --from` workflow;
- template linting;
- lint versus validate;
- placeholder consistency checks;
- shell hook behavior;
- bundled template cache commands.

### 4.10 v0.3.0 Suggested Agent Prompt

```text
Implement the v0.3.0 template authoring release for Zappy. Add real
create --from support by copying existing projects into template skeletons,
add a zappy lint command for static manifest/template checks, add placeholder
consistency checks, implement shell=true hook execution in zappy-hooks, and add
cache path/status/clear/rebuild commands backed by zappy-templates. Keep linting
separate from validation and update the Zappy Book.
```

### 4.11 v0.3.0 Release Checklist

```text
cargo fmt --check
cargo clippy --tests --examples --all-targets --all-features --workspace -- -D warnings
cargo test --workspace
cargo test --workspace --doc
just test-templates
just book-check
manual smoke test: zappy create --from ./some-project -o ./my-template
manual smoke test: zappy lint -i ./templates -t my-template
manual smoke test: zappy cache status
manual smoke test: shell=true hook template
```

---

## 5. Release: `v0.4.0` — Configuration and Ecosystem Release

### 5.1 Goal

Make Zappy customizable as a daily personal scaffolding tool.

This release should make Zappy feel less like a standalone binary and more like a user-configurable workflow tool.

### 5.2 Summary

Core areas:

1. user config file;
2. user default variables;
3. configured template search paths;
4. better discovery diagnostics;
5. shell completions;
6. optional machine-readable output.

### 5.3 Crate Focus

```text
zappy-core
  Config-domain data types if kept pure and filesystem-independent.

zappy-fs
  Config path resolution and configured template search path expansion.

zappy-cli
  Config loading orchestration, diagnostics, completions, output formats.

zappy-templates
  Mostly unchanged.

zappy-hooks
  Mostly unchanged.
```

---

### 5.4 Task Group A: User Config File

#### Goal

Load user-level Zappy configuration.

#### Default Location

```text
Linux:   ~/.config/zappy/config.toml
macOS:   ~/Library/Application Support/zappy/config.toml
Windows: %APPDATA%\zappy\config.toml
```

Optionally retain support for:

```text
ZAPPY_CONFIG=/path/to/zappy
```

#### Example Config

```toml
[defaults]
license = "MIT OR Apache-2.0"
use_gitlab_ci = true
use_github_actions = false

[defaults.rust]
edition = "2024"

[paths]
templates = [
  "~/projects/templates",
  "~/work/templates",
]
```

#### Design Notes

The config file should be optional. Missing config should not be an error.

Invalid config should produce a clear diagnostic that includes the config path.

#### Acceptance Criteria

- config file is loaded from platform config location;
- missing config is fine;
- invalid config produces a clear error;
- config loading has tests with temporary directories or injected paths.

---

### 5.5 Task Group B: User Default Variables

#### Goal

Allow users to define default variable values once.

#### Example

```toml
[defaults]
license = "MIT OR Apache-2.0"
author = "Viktor Toth"
use_gitlab_ci = true
```

#### Recommended Precedence

```text
built-ins
> explicit CLI --var values
> interactive values
> template defaults
> user config defaults
```

The main rule: user defaults should fill gaps, not unexpectedly override template-specific defaults.

#### Template-Specific Defaults

Support for grouped defaults can be considered:

```toml
[defaults.rust]
edition = "2024"

[defaults.lua]
use_luarocks = true
```

Do not overcomplicate this in the first config release. Global defaults are the priority.

#### Acceptance Criteria

- global defaults are passed into variable resolution;
- template defaults still override user defaults;
- CLI `--var` overrides both;
- docs explain precedence clearly.

---

### 5.6 Task Group C: Configured Template Search Paths

#### Goal

Allow users to register custom template directories.

#### Example

```toml
[paths]
templates = [
  "~/dev/zappy-templates",
  "~/work/company-templates",
]
```

#### Search Order

Recommended order:

```text
1. explicit --templates-dir
2. environment template directory
3. config-defined template paths
4. platform config templates directory
5. executable-relative templates directory
6. current-working-directory templates directory
7. bundled templates
```

If `--templates-dir` is provided, it may continue to act as an override that disables normal discovery.

#### Acceptance Criteria

- configured template paths participate in discovery;
- invalid configured paths are reported as warnings or errors depending on strictness;
- `zappy list` shows templates from configured paths;
- shadowing behavior remains deterministic.

---

### 5.7 Task Group D: Discovery Diagnostics

#### Goal

Make template discovery transparent.

#### Useful Commands

```bash
zappy list -vv
zappy info -t foo -vv
```

#### Useful Details

- all searched paths;
- whether each path exists;
- whether each path is required or optional;
- templates discovered per path;
- shadowed templates;
- final selected template source.

#### Acceptance Criteria

- missing-template diagnostics include searched paths;
- verbose list/info output explains discovery sources;
- shadowed templates can be diagnosed;
- docs describe discovery precedence.

---

### 5.8 Task Group E: Shell Completions

#### Goal

Generate shell completions for common shells.

#### Command

```bash
zappy completions bash
zappy completions zsh
zappy completions fish
```

#### Suggested Dependency

Use `clap_complete`.

#### Acceptance Criteria

- completions can be generated for bash, zsh, and fish;
- command writes to stdout by default;
- docs show installation snippets.

---

### 5.9 Task Group F: Machine-Readable Output

#### Goal

Expose selected command output as JSON for tooling.

This is optional for `v0.4.0`; move it to `v0.5.0` if the release becomes too large.

#### Candidate Commands

```bash
zappy list --format json
zappy info -t rust-cli --format json
zappy new -t rust-cli -n my-tool --dry-run --format json
```

#### Suggested Design

Add an output format enum in `zappy-cli`:

```text
text
json
```

Keep JSON structures stable enough for tools, but do not promise long-term API stability before `v1.0.0`.

#### Acceptance Criteria

- `list --format json` produces valid JSON;
- `info --format json` produces valid JSON;
- dry-run JSON includes planned operations and warnings;
- text output remains the default.

---

### 5.10 v0.4.0 Documentation Tasks

Update the Zappy Book with:

- config file location;
- config schema;
- default variable precedence;
- configured template paths;
- discovery order;
- shell completions;
- JSON output if implemented.

### 5.11 v0.4.0 Suggested Agent Prompt

```text
Implement the v0.4.0 configuration and ecosystem release for Zappy. Add optional
user config loading, user default variables, configured template search paths,
improved discovery diagnostics, and shell completion generation. Keep config
loading optional and preserve existing CLI overrides and template defaults.
Document config locations, precedence, and discovery order in the Zappy Book.
```

### 5.12 v0.4.0 Release Checklist

```text
cargo fmt --check
cargo clippy --tests --examples --all-targets --all-features --workspace -- -D warnings
cargo test --workspace
cargo test --workspace --doc
just test-templates
just book-check
manual smoke test: config defaults affect generation
manual smoke test: configured template path is discovered
manual smoke test: zappy completions bash
manual smoke test: missing template shows discovery diagnostics
```

---

## 6. Deferred Post-`v0.4.0` Ideas

These ideas are useful, but should not be pulled into the `v0.2.0`–`v0.4.0` roadmap unless they directly support a release theme.

### 6.1 Remote Template Registries

Potential future commands:

```bash
zappy registry add <url>
zappy registry list
zappy install-template <id>
```

This requires trust, versioning, update, and security design. Defer.

### 6.2 Recipe / Orchestrator Mode

Zappy could eventually orchestrate ecosystem-native generators and then apply overlays.

Example future idea:

```text
run `cargo new`
then apply Zappy overlay
then run hooks
```

This should remain post-MVP.

### 6.3 Expression Language for Conditions

Current conditions are intentionally simple boolean variable checks.

Future examples:

```text
when = "use_ci && ci_provider == 'gitlab'"
```

This requires parser/design work. Defer until simple conditions become limiting.

### 6.4 Full Template Language

Zappy currently uses literal placeholders. A full template language would add power but also complexity.

Candidates:

- Tera;
- Handlebars;
- MiniJinja.

Defer until placeholder replacement becomes clearly insufficient.

### 6.5 TUI / GUI Frontend

Interactive prompting in `v0.2.0` should be simple. A richer TUI can come later if needed.

---

## 7. Suggested Issue Groups

### 7.1 `v0.2.0`

```text
v0.2: add CLI diagnostics/reporting layer
v0.2: wire verbosity to tracing/logging
v0.2: implement regex validation for string variables
v0.2: add interactive variable prompting
v0.2: improve dry-run output
v0.2: update book for interactive generation and validation
```

### 7.2 `v0.3.0`

```text
v0.3: implement create --from existing project
v0.3: add zappy lint command
v0.3: add placeholder consistency checks
v0.3: implement shell=true hook support
v0.3: add zappy cache command
v0.3: expand template authoring docs
```

### 7.3 `v0.4.0`

```text
v0.4: add config file loading
v0.4: add user default variables
v0.4: add configured template search paths
v0.4: improve template discovery diagnostics
v0.4: add shell completions
v0.4: add optional JSON output for list/info/dry-run
v0.4: document config and precedence
```

---

## 8. Maintenance Notes

### 8.1 Versioning

Before `v1.0.0`, Zappy can still evolve its manifest format, CLI output, and JSON output if added. Breaking changes should still be documented in the changelog.

### 8.2 Docs

Every release should update:

- `README.md` if the quick-start changes;
- the Zappy Book for user-facing behavior;
- crate rustdocs for public library APIs;
- `CHANGELOG.md`.

### 8.3 Testing

Each release should keep these checks green:

```text
formatting
clippy
unit tests
integration tests
doc tests
bundled template validation
book build
```

Template validation may remain CI-rules-based if it is too expensive to run on every pipeline.

### 8.4 Agent Usage

This roadmap is intentionally structured so coding agents can implement one task group at a time.

For agent-driven work:

1. pick one task group;
2. read relevant crate modules;
3. implement the smallest vertical slice;
4. add tests;
5. update docs;
6. run the release checklist subset relevant to the task;
7. summarize what changed and what remains.
