# Built-in Templates

Zappy ships with starter templates embedded in the `zappy-templates` crate. At
runtime, bundled templates are extracted to a cache directory and discovered
through the same `zappy-fs` discovery path as custom templates.

List the bundled templates with:

```bash
zappy list
```

For deterministic local inspection from a source checkout, point directly at the
bundled template source directory:

```bash
zappy list --templates-dir crates/zappy-templates/templates
```

## Bundled Template Summary

| ID | Name | Language | Description |
| --- | --- | --- | --- |
| `rust-cli` | Rust CLI app | `rust` | Rust-based CLI application using clap. |
| `cpp-cmake-app` | C++ CMake app | `cpp` | C++ app managed by CMake build system. |
| `cpp-cmake-lib` | C++ CMake library | `cpp` | C++ static library managed by CMake build system. |
| `lua-cli` | Lua CLI app | `lua` | Luarock-bundled Lua-based CLI application using argparse. |
| `nvim-plugin` | Neovim plugin | `lua` | Neovim editor plugin for its lazy.nvim package manager. |

## Common Variables

All bundled templates use a `description` variable. Some also use repository or
CI-related variables.

| Template | Variables |
| --- | --- |
| `rust-cli` | `description`, `repo`, `use_gitlab_ci`, `gitlab_image_registry`, `use_github_actions` |
| `cpp-cmake-app` | `description`, `use_gitlab_ci`, `gitlab_image_registry` |
| `cpp-cmake-lib` | `description`, `use_gitlab_ci`, `gitlab_image_registry` |
| `lua-cli` | `description`, `repo`, `use_gitlab_ci`, `gitlab_image_registry` |
| `nvim-plugin` | `description`, `use_gitlab_ci`, `gitlab_image_registry` |

The `gitlab_image_registry` variable is conditionally required when
`use_gitlab_ci` is enabled.

## Conditional Files

Bundled templates use conditionals for optional CI and Docker files.

| Variable | Typical Paths |
| --- | --- |
| `use_gitlab_ci` | `.gitlab-ci.yml`, `Dockerfile` |
| `use_github_actions` | `.github/workflows/ci.yml` in `rust-cli` |

Pass boolean values with `--var`, for example:

```bash
zappy new \
  --template rust-cli \
  --name my-tool \
  --var description="My generated tool" \
  --var repo="https://example.com/me/my-tool.git" \
  --var use_github_actions=true
```

CLI variable parsing accepts `true`, `yes`, `y`, `Y`, or `1` as `true`, and
`false`, `no`, `n`, `N`, or `0` as `false`.

## Hooks And Validation

Some bundled templates define optional formatting hooks:

- `rust-cli` runs `cargo fmt` after generation when hooks are not skipped.
- `lua-cli` and `nvim-plugin` run `stylua .` after generation when hooks are not
  skipped.

These hooks are marked optional, so hook failure is reported as a warning rather
than a hard generation failure.

Bundled templates also define validation blocks. Validation may require external
tooling for the generated ecosystem, such as `just`, Rust tooling, CMake, Ninja,
Lua tooling, or StyLua. The exact validation commands are stored in each bundled
template's `zappy.toml`.

## Inspecting A Bundled Template

Use `info` to inspect metadata:

```bash
zappy info --template rust-cli
```

From a source checkout, read the manifest directly:

```text
crates/zappy-templates/templates/rust-cli/zappy.toml
```

The source checkout is the best reference when changing bundled templates. The
runtime cache is an extracted copy.
