# Built-in Variables

Built-in variables are supplied by Zappy during generation and validation. They
are reserved names and cannot be declared under `[variables]` in a template
manifest.

## Names And Values

| Name | Value Source |
| --- | --- |
| `project_name` | `zappy new --name <PROJECT_NAME>`, or the validation output directory name during `zappy validate`. |
| `user` | `git config user.name`, then `USER`, then `USERNAME`, then `{TODO: add username}`. |
| `email` | `git config user.email`, then `{TODO: add user email}`. |
| `date` | Current local date as `YYYY-MM-DD`, falling back to UTC when local offset is unavailable. |
| `day` | Current day as two digits. |
| `month` | Current month as two digits. |
| `year` | Current year as four digits. |

Built-ins are injected after declared template variables resolve. They are
available for file contents, rendered output paths, hook commands, hook
arguments, hook working directories, and hook environment values.

## Placeholder Forms

Each built-in receives all supported transforms. The placeholder base is the
upper-case variable name prefixed with `__ZAPPY_`.

For `project_name`, the placeholders are:

| Transform | Placeholder |
| --- | --- |
| `raw` | `__ZAPPY_PROJECT_NAME__` |
| `kebab` | `__ZAPPY_PROJECT_NAME_KEBAB__` |
| `snake` | `__ZAPPY_PROJECT_NAME_SNAKE__` |
| `pascal` | `__ZAPPY_PROJECT_NAME_PASCAL__` |
| `camel` | `__ZAPPY_PROJECT_NAME_CAMEL__` |
| `screaming_snake` | `__ZAPPY_PROJECT_NAME_SCREAMING__` |
| `upper` | `__ZAPPY_PROJECT_NAME_UPPER__` |
| `lower` | `__ZAPPY_PROJECT_NAME_LOWER__` |

The same suffix pattern applies to every built-in variable. For example:

- `__ZAPPY_USER__`
- `__ZAPPY_EMAIL__`
- `__ZAPPY_DATE__`
- `__ZAPPY_YEAR__`
- `__ZAPPY_PROJECT_NAME_KEBAB__`
- `__ZAPPY_USER_SNAKE__`

## Transform Output

For the value `my cool tool`, transforms produce:

| Transform | Output |
| --- | --- |
| `raw` | `my cool tool` |
| `kebab` | `my-cool-tool` |
| `snake` | `my_cool_tool` |
| `pascal` | `MyCoolTool` |
| `camel` | `myCoolTool` |
| `screaming_snake` | `MY_COOL_TOOL` |
| `upper` | `MY COOL TOOL` |
| `lower` | `my cool tool` |

## Reserved Names

These names are reserved and invalid in `[variables]`:

- `project_name`
- `user`
- `email`
- `date`
- `day`
- `month`
- `year`

Use built-in placeholders directly instead of declaring these variables.
