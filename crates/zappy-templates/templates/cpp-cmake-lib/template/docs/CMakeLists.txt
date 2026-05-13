cmake_minimum_required(VERSION 3.20.0)

project(__ZAPPY_PROJECT_NAME_SNAKE___docs)

add_library(__ZAPPY_PROJECT_NAME_SNAKE___docs INTERFACE)

add_custom_target(
    docs
    COMMAND sphinx-build -b html ${CMAKE_SOURCE_DIR} ${CMAKE_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)
