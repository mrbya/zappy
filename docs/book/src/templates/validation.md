# Validation

Template validation lets a template prove that it can generate a working project.
Validation is driven by the `[validation]` block in `zappy.toml` and executed by
`zappy validate`.

## Basic Validation Block

```toml
[validation]
output_dir_name = "validated-my-template"
variables = {
    description = "Generated validation project",
}

[[validation.steps]]
name = "check README exists"
command = "test"
args = ["-f", "README.md"]
```

Run it with:

```bash
zappy validate --templates-dir . --template my-template
```

## Validation Flow

`zappy validate` performs these steps:

1. Discovers and loads the selected template.
2. Fails if the template does not define `[validation]`.
3. Creates a temporary root directory.
4. Chooses the validation output directory under that temp root.
5. Resolves variables using `validation.variables` plus built-ins.
6. Builds a generation plan with overwrite behavior enabled.
7. Generates the project into the validation output directory.
8. Runs `validation.setup` hooks.
9. Runs `validation.steps` hooks if setup succeeded.
10. Runs `validation.teardown` hooks.
11. Prints success when generation and validation hooks succeed.

Validation teardown runs even when setup or steps fail. The command returns a
failure if setup, steps, or teardown fail.

## Output Directory

`validation.output_dir_name` is optional. When omitted, Zappy uses
`zappy-validation-output`.

The field must not be an empty string when provided.

During validation, `project_name` built-in values come from the final validation
output directory name. For example, if `output_dir_name` is
`validated-my-template`, then `__ZAPPY_PROJECT_NAME__` renders as
`validated-my-template` during validation.

## Validation Variables

`validation.variables` supplies explicit values for declared template variables:

```toml
[validation]
variables = {
    description = "Generated validation project",
    use_ci = false,
}
```

Every key in `validation.variables` must reference a variable declared in
`[variables]`. Unknown validation variables make the manifest invalid.

## Setup, Steps, And Teardown

Validation hooks use the same fields as generation hooks:

```toml
[[validation.setup]]
name = "create build dir"
command = "mkdir"
args = ["build"]

[[validation.steps]]
name = "configure project"
working_dir = "build"
command = "cmake"
args = ["..", "-GNinja"]

[[validation.teardown]]
name = "cleanup marker"
command = "rm"
args = ["-f", "marker"]
optional = true
```

All hook commands run relative to the generated validation output directory by
default. `working_dir` is also relative to that output directory.

## Keeping The Temp Directory

By default, the temporary validation directory is removed when validation
finishes. Keep it for inspection with:

```bash
zappy validate --template my-template --keep-temp
```

When kept, Zappy prints the temporary directory path.

## `--no-hooks` During Validation

`zappy validate --no-hooks` skips template generation hooks such as
`hooks.pre_generate` and `hooks.post_generate`.

It does not skip validation hooks. `validation.setup`, `validation.steps`, and
`validation.teardown` still run.

## Common Failures

Validation fails when:

- The template has no `[validation]` block.
- Validation variables do not satisfy required template variables.
- A validation variable name is not declared in `[variables]`.
- Generation fails.
- A required validation hook cannot be spawned or exits unsuccessfully.
- A hook sets `shell = true`, which is unsupported.
