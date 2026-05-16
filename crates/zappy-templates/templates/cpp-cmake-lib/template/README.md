# __ZAPPY_PROJECT_NAME__

> __DESCRIPTION__

<!-- toc -->

- [Features](#features)
- [Requirements](#requirements)
- [Usage](#usage)
  * [Examples](#examples)
- [Development](#development)
  * [Prequisites](#prequisites)
  * [Getting started](#getting-started)
- [Documentation](#documentation)
- [License](#license)

<!-- tocstop -->

## Features

TBD

---

## Requirements

TBD

___

## Usage

TBD

### Examples

1. Configure and build library with examples.
```bash
# from lib root
mkdir build && cd build
cmake .. -GNinja -D__ZAPPY_PROJECT_NAME_SCREAMING___BUILD_EXAMPLES=ON
ninja
```

2. Run examples:
```bash
# inside build dir
ninja run_examples
```
---

## Development

### Prequisites

TBD

### Getting started

1. Configure using cmake:
```bash
mkdir build && cd build
cmake .. -GNinja
```

2. Build:
```bash
# inside build dir
ninja
```

3. Run tests:
```bash
# inside build dir
ninja test
```

4. Generate coverage report:
```bash
# inside build dir
ninja cov
```

## Documentation

Generate documentation:
```bash
# inside configured build dir
ninja docs
```

Documentation entrypoint: `docs/index.rst`
Rest of docs live in: `docs/rst/`

> **Note:** doc generation requires [Doxygen](https://www.doxygen.nl/index.html) + [Sphinx](https://github.com/sphinx-doc/sphinx) + [Breathe](https://github.com/breathe-doc/breathe)

## License

Licensed under [MIT license](LICENSE)
