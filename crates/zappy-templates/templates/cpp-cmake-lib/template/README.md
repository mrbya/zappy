# __ZAPPY_PROJECT_NAME__

> __DESCRIPTION__

<!-- toc -->

- [Features](#features)
- [Requirements](#requirements)
- [Usage](#usage)
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
