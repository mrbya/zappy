# Hooks

Hooks run external commands as part of generation or validation. They are useful
for formatting generated files, installing dependencies, or checking generated
output.

## Generation Hooks

Generation hooks are declared in two arrays:

```toml
[[hooks.pre_generate]]
name = "Prepare output"
command = "sh"
args = ["-c", "printf preparing"]

[[hooks.post_generate]]
name = "Format Rust code"
command = "cargo"
args = ["fmt"]
optional = true
```

`pre_generate` hooks run after Zappy creates the output directory and before it
writes the planned files. `post_generate` hooks run after files are written.

Pass `--no-hooks` to `zappy new` to skip generation hooks:

```bash
zappy new --template rust-cli --name my-tool --no-hooks
```

## Hook Fields

| Field | Required? | Default | Description |
| --- | --- | --- | --- |
| `name` | No | Command string | Human-readable name used in errors. |
| `command` | Yes | None | Executable to spawn. Must not be empty. |
| `args` | No | `[]` | Command arguments. |
| `working_dir` | No | Generated output directory | Relative path under the output directory. |
| `env` | No | `{}` | Extra environment variables for the command. |
| `when` | No | Always run | Boolean variable that controls whether the hook runs. |
| `optional` | No | `false` | If true, failures become warnings instead of hard failures. |
| `shell` | No | `false` | Parsed, but `shell = true` is rejected at execution time. |

`working_dir` must be relative and must not contain `..` path components.

## Conditions

Use `when` to run a hook only when a boolean variable is true:

```toml
[variables]
run_formatter = { default = true }

[[hooks.post_generate]]
name = "Format"
command = "cargo"
args = ["fmt"]
when = "run_formatter"
optional = true
```

Missing variables and non-boolean values evaluate as false.

## Rendering In Hooks

Zappy renders placeholders in:

- `command`
- `args`
- `working_dir`
- `env` values

Example:

```toml
[[hooks.post_generate]]
name = "Write marker"
command = "sh"
args = ["-c", "printf '__ZAPPY_PROJECT_NAME__' > project-name.txt"]
```

Hook commands run with the generated project output directory as the current
working directory unless `working_dir` is set.

## Required And Optional Hooks

Required hooks are the default. If a required hook cannot be spawned or exits
with a non-zero status, generation or validation fails.

Optional hooks still run, but failures are counted as optional failures and
reported as warnings. Generation or validation continues after an optional hook
failure.

## Shell Hooks

The manifest can parse `shell = true`, but the current hook runner rejects shell
hooks with an unsupported-shell error. Use an explicit shell command instead
when you need shell behavior:

```toml
[[hooks.post_generate]]
name = "Write marker"
command = "sh"
args = ["-c", "printf done > marker.txt"]
```

This explicit form is supported because Zappy spawns `sh` directly with the
provided arguments.

## Validation Hooks

Validation uses the same hook shape under:

- `[[validation.setup]]`
- `[[validation.steps]]`
- `[[validation.teardown]]`

`zappy validate --no-hooks` skips generation hooks, but validation setup, steps,
and teardown still run.
