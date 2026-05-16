# __ZAPPY_PROJECT_NAME__ greeting example

Simple exampel showcasing __ZAPPY_PROJECT_NAME__.

## Run example

1. Configure and build library with examples.
```bash
# from lib root
mkdir build && cd build
cmake .. -GNinja -D__ZAPPY_PROJECT_NAME_SCREAMING___BUILD_EXAMPLES=ON
ninja
```

2. Run greeting example:
```bash
# inside build dir
ninja run_greeting
```
