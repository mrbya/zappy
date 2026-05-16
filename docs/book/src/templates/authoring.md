# Authoring Templates

Template authoring starts with a normal directory containing `zappy.toml` and a
source tree. Zappy's current authoring workflow is file-based: write the
manifest, put source files under the source root, then use `zappy new --dry-run`
and `zappy validate` to check behavior.

## Start From A Skeleton

Create an empty template skeleton with `init`:

```bash
zappy init --output ./my-template --template my-template --name "My Template"
```

or with `create --empty`:

```bash
zappy create --empty --output ./my-template --template my-template --name "My Template"
```

Both commands create a directory with:

```text
my-template/
  zappy.toml
  template/
    README.md
```

If `--template` is omitted, the skeleton ID is derived from the output directory
name in kebab case. `--name` and `--description` fill the manifest metadata.
Without them, Zappy uses the template ID as the name and a TODO description.

Use `--force` only when you want to overwrite conflicting skeleton files.

## Add Template Files

Place files to generate under the source root, which defaults to `template`:

```text
my-template/
  zappy.toml
  template/
    README.md
    src/
      main.rs
```

Zappy renders UTF-8 text files and copies binary files. Output paths are rendered
too, so placeholders can appear in file names and directory names.

## Add Variables And Placeholders

Declare variables under `[variables]` and map transforms to literal
placeholders:

```toml
[variables]
description = {
    required = true,
    default = "TODO: add project description",
    placeholders = { raw = "__DESCRIPTION__" }
}
```

Then use the placeholder in a source file:

```text
# __ZAPPY_PROJECT_NAME__

__DESCRIPTION__
```

`__ZAPPY_PROJECT_NAME__` is a built-in placeholder supplied by Zappy. Custom
placeholders are the literal strings you configure in `zappy.toml`.

## Control Paths

Use `[paths]` for files that should not be generated or should be copied as
binary:

```toml
[paths]
exclude = [".git", "target", "node_modules"]
binary_extensions = ["png", "jpg", "zip"]
binary_files = ["Cargo.lock"]
```

Use `[[conditionals]]` for optional paths controlled by boolean variables:

```toml
[variables]
use_ci = { default = false }

[[conditionals]]
path = ".github/workflows/ci.yml"
when = "use_ci"
```

A conditional path is included only when the named variable resolves to boolean
`true`.

## Preview Generation

Use `--dry-run` frequently while authoring:

```bash
zappy new \
  --templates-dir . \
  --template my-template \
  --name demo-project \
  --var description="Demo project" \
  --dry-run
```

The dry-run output shows planned directory creation, text rendering, binary
copies, skipped paths, and warnings. It does not write files.

## Validate The Template

Add a `[validation]` block when the template can check itself:

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

Run validation with:

```bash
zappy validate --templates-dir . --template my-template
```

Validation generates the template into a temporary directory, then runs setup,
step, and teardown hooks from the validation block.

## Current Authoring Limits

- There is no interactive prompt flow yet. Missing required variables must have
  CLI overrides, defaults, or built-in values.
- `zappy create --from <project>` is not implemented yet. Start custom templates
  with `init` or `create --empty`.
- `shell = true` hooks are rejected at execution time.
- `validation_regex` is parsed but not currently enforced.
