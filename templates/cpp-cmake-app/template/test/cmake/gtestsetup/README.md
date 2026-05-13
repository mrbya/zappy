# gtestSetup

A simple CMake module to setup cmake targets for testing C++ code using [GoogleTest](https://github.com/google/googletest) and test coverage analysis using [Gcov](https://gcc.gnu.org/onlinedocs/gcc/Gcov.html) and [lcov](https://github.com/linux-test-project/lcov).

## Requirements

1. [CMake](https://cmake.org/)
2. [GNU GCC](https://gcc.gnu.org/)
3. [GoogleTest](https://github.com/google/googletest)
4. [Gcov](https://gcc.gnu.org/onlinedocs/gcc/Gcov.html)
5. [lcov](https://github.com/linux-test-project/lcov)

## Installation

### System-wide:

```bash
mkdir build
cd build
cmake ..
make
sudo make install
```

### Local to project

Clone the repo to your project.

## Usage

1. Include the module in your projects' `CMakeLists.txt`

```cmake
include(gtestSetup)
```

In case of local installation you have to add the module path prefix to `CMAKE_MODULE_PATH`.

2. Set up test targets using `setup_test`

```cmake
setup_test(testTarget [TARGET_NAME <target name>]
            [TEST_DIR <test dir>]
            [INCLUDE_DIRS <includes dir>]
            [SRC_DIRS <srcs dir>]
            [APP_SRCS <app srcs...>]
            [APP_INCLUDES <app includes...>]
            [APP_LIBS <app libs...>])
```

| Arg | Default | Description |
| --------------- | --------------- | --------------- |
| `testTarget` |  | Project target the test is built for |
| `TARGET_NAME` | `test` | Test target name used to build and run tests |
| `TEST_DIR` | `test` | Tests root relative to `CMakeLists.txt` |
| `INCLUDE_DIRS` | `${TEST_DIR}/include` | Test include dirs relative to `CMakeLists.txt` |
| `SRC_DIRS` | `${TEST_DIR}/src` | Test source dirs relative to `CMakeLists.txt` |
| `APP_INCLUDES` |  | Project target include dirs to be included in test target |
| `APP_SRCS` |  | Project target sources to be linked to test target |
| `APP_LIBS` |  | Libraries to be linked to test target |

Eg.:
```cmake
setup_test(
    superDuperApp
    TEST_DIR
        myTests
    INCLUDE_DIRS
        myTests/inc
    SRC_DIRS
        myTests/src
    APP_INCLUDES
        inc
    APP_SRCS
        src/main.cpp src/superDuperUtils.cpp
)
```

3. Set up coverage targets using `setup_coverage`

```
setup_coverage()
```

4. Write GTest tests

Recommended test dir structure:

```bash
prj root
└── test
    ├── include
    │   └── //test includes
    └── src
        └── //test sources
```

5. Profit

### Targets added:

| Target   | Description    |
|--------------- | --------------- |
| `test` / `${TEST_NAME}`   | Builds test binary and runs tests |
| `testbin`   | Builds test binary (without running) |
| `cov`   | Generates coverage report |
| `report`   | Opens coverage report |
