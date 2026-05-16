# Quick Start

This guide uses implemented commands only. It assumes `zappy` is on your `PATH`.
If you are working from a source checkout, replace `zappy` with
`cargo run --` in the examples.

## 1. Check The CLI

Print the top-level help:

```bash
zappy --help
```

Zappy should show the available commands: `list`, `info`, `new`, `validate`,
`init`, and `create`.

## 2. List Templates

List templates from the default discovery paths, including bundled templates:

```bash
zappy list
```

Typical bundled template IDs include:

- `rust-cli`
- `cpp-cmake-app`
- `cpp-cmake-lib`
- `lua-cli`
- `nvim-plugin`

Filter by language when you only want templates for one ecosystem:

```bash
zappy list --language rust
```

When you want to use only a specific templates directory, pass
`--templates-dir`:

```bash
zappy list --templates-dir crates/zappy-templates/templates
```

## 3. Inspect A Template

Show metadata for the bundled Rust CLI template:

```bash
zappy info --template rust-cli
```

The output includes the template ID, name, description, language, version,
source root, template directory, and manifest path when those fields are present.

## 4. Preview Generation

Use `--dry-run` before writing files. This builds and prints the generation plan
without creating the output directory:

```bash
zappy new \
  --template rust-cli \
  --name my-tool \
  --var description="My generated tool" \
  --var repo="https://example.com/me/my-tool.git" \
  --output ./my-tool \
  --dry-run
```

Dry-run output includes operations such as `CREATE DIR`, `RENDER`, `COPY`, and
`SKIP`, plus warnings for situations such as existing destinations.

## 5. Generate The Project

Run the same command without `--dry-run` when the plan looks correct:

```bash
zappy new \
  --template rust-cli \
  --name my-tool \
  --var description="My generated tool" \
  --var repo="https://example.com/me/my-tool.git" \
  --output ./my-tool
```

By default, Zappy writes into the directory named by `--output`. If `--output` is
omitted, the project name is used as the output directory.

If the template defines generation hooks and you want to skip them, add
`--no-hooks`:

```bash
zappy new \
  --template rust-cli \
  --name my-tool \
  --var description="My generated tool" \
  --var repo="https://example.com/me/my-tool.git" \
  --output ./my-tool \
  --no-hooks
```

Use `--force` only when you want Zappy to overwrite conflicting destination
files.

## 6. Validate A Template

Templates can define a `[validation]` block. Validate a template with:

```bash
zappy validate --template rust-cli
```

Validation generates the template into a temporary directory, runs validation
setup hooks, runs validation steps, and then runs teardown hooks. Use
`--keep-temp` if you want to inspect the generated validation directory after a
successful run:

```bash
zappy validate --template rust-cli --keep-temp
```

`--no-hooks` skips generation hooks during validation, but it does not skip the
validation setup, step, or teardown hooks.

## 7. Start A Custom Template

Create an empty template skeleton with either command:

```bash
zappy init --output ./my-template --template my-template --name "My Template"
```

or:

```bash
zappy create --empty --output ./my-template --template my-template --name "My Template"
```

The non-empty `zappy create --from <project>` workflow is not implemented yet.
Use `init` or `create --empty` for template skeletons today.
