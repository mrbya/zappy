cmake_minimum_required(VERSION 3.20)

#####################################################################################
# Project setup
#####################################################################################

project(__ZAPPY_PROJECT_NAME_SNAKE__)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

#####################################################################################
# app sources & includes setup
#####################################################################################

set(prjIncludes include)
file(GLOB prjSrcs ${CMAKE_SOURCE_DIR}/src/*.cpp)

#####################################################################################
# Executable setup
#####################################################################################

add_executable(__ZAPPY_PROJECT_NAME_SNAKE__ ${prjSrcs})
target_include_directories(__ZAPPY_PROJECT_NAME_SNAKE__ PUBLIC ${prjIncludes})

#####################################################################################
# Test setup
#####################################################################################

list(APPEND CMAKE_MODULE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/test/cmake/gtestsetup)
include(gtestSetup)
setup_test(__ZAPPY_PROJECT_NAME_SNAKE__)
setup_coverage()

#####################################################################################
# Misc targets
#####################################################################################

add_custom_target(
    run
    DEPENDS
        __ZAPPY_PROJECT_NAME_SNAKE__
    USES_TERMINAL
    COMMAND
        ${CMAKE_CURRENT_BINARY_DIR}/__ZAPPY_PROJECT_NAME_SNAKE__
)

add_custom_target(
    docs
    COMMAND sphinx-build -b html ${CMAKE_SOURCE_DIR}/docs ${CMAKE_SOURCE_DIR}/public
    WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
)
