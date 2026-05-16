# Variables and Placeholders

Variables provide values for placeholders, conditions, hook arguments, hook
environment values, and rendered output paths.

## Variable Names

Variable names must:

- Start with an ASCII letter or `_`.
- Contain only ASCII letters, digits, and `_`.
- Not contain `-`.
- Not use a reserved built-in name such as `project_name`, `user`, or `date`.

## Values

Variable values can be strings, booleans, or integers. CLI overrides use
`key=value` form:

```bash
zappy new --template rust-cli --name my-tool --var description="My tool"
```

CLI parsing converts these values to booleans:

| True Values | False Values |
| --- | --- |
| `true`, `yes`, `y`, `Y`, `1` | `false`, `no`, `n`, `N`, `0` |

Other values remain strings unless they parse as integers.

## Variable Fields

Example:

```toml
[variables]
license = {
    default = "MIT",
    choices = ["MIT", "Apache-2.0"],
    placeholders = { raw = "__LICENSE__" }
}
```

Common fields:

| Field | Purpose |
| --- | --- |
| `prompt` | Prompt text metadata. Prompts are parsed, but interactive prompting is not implemented yet. |
| `default` | Template-provided default value. |
| `required` | Fails resolution when no value is available. |
| `required_when` | Makes the variable required when another boolean variable is true. |
| `conflicts_with` | Fails resolution when both boolean variables are true. |
| `choices` | Restricts resolved values to the listed values. |
| `validation_regex` | Parsed but not currently enforced. |
| `transforms` | Transform values Zappy should compute. |
| `placeholders` | Map transform names to literal placeholder strings. |

If a variable is `required` or uses `required_when` and has no default, the
manifest must define a non-empty `prompt`. Because prompting is not implemented
yet, users still need to provide such values with `--var` during generation.

## Resolution

For normal CLI use, declared template variables resolve from:

1. Explicit `--var key=value` overrides.
2. Template defaults from `zappy.toml`.

The core resolver also has internal slots for interactive values and user
defaults, but the current CLI does not populate them. Built-in variables are
reserved and injected separately after declared variables resolve.

Unknown CLI variables fail resolution. A CLI override must reference a variable
declared in `[variables]`; built-ins are not overridden with `--var`.

## Placeholders

Placeholders are literal strings. Zappy replaces each placeholder with the
rendered variable value for the mapped transform:

```toml
[variables]
project_label = {
    default = "my cool tool",
    placeholders = {
        raw = "__PROJECT_LABEL__",
        kebab = "__PROJECT_LABEL_KEBAB__",
        snake = "__PROJECT_LABEL_SNAKE__",
        pascal = "__PROJECT_LABEL_PASCAL__"
    }
}
```

With `project_label = "my cool tool"`, these replacements are produced:

| Transform | Output |
| --- | --- |
| `raw` | `my cool tool` |
| `kebab` | `my-cool-tool` |
| `snake` | `my_cool_tool` |
| `pascal` | `MyCoolTool` |

Placeholders are applied to UTF-8 file contents and output paths.

## Transforms

Supported transform names are:

| Transform | Example For `my cool tool` |
| --- | --- |
| `raw` | `my cool tool` |
| `kebab` | `my-cool-tool` |
| `snake` | `my_cool_tool` |
| `pascal` | `MyCoolTool` |
| `camel` | `myCoolTool` |
| `screaming_snake` | `MY_COOL_TOOL` |
| `upper` | `MY COOL TOOL` |
| `lower` | `my cool tool` |

If no transform is requested and no placeholders are configured, Zappy computes
`raw` by default. Any transform referenced by a placeholder is computed even if
it is not also listed in `transforms`.

## Built-in Placeholders

Zappy injects built-in placeholders for project name, user, email, and date
parts. Use the reference page for the complete list:

- [Built-in Variables](../reference/builtins.md)
