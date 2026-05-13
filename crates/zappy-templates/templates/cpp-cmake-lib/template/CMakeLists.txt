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

add_library(__ZAPPY_PROJECT_NAME_SNAKE__ INTERFACE)
target_include_directories(__ZAPPY_PROJECT_NAME_SNAKE__ INTERFACE ${prjIncludes})
target_sources(__ZAPPY_PROJECT_NAME_SNAKE__ INTERFACE ${prjSrcs})

#####################################################################################
# Test setup
#####################################################################################

if (CMAKE_PROJECT_NAME STREQUAL __ZAPPY_PROJECT_NAME_SNAKE__)
    list(APPEND CMAKE_MODULE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/test/cmake/gtestsetup)
    include(gtestSetup)
    setup_test(__ZAPPY_PROJECT_NAME_SNAKE__ APP_LIBS __ZAPPY_PROJECT_NAME_SNAKE__)
    setup_coverage()
endif()

#####################################################################################
# Misc targets
#####################################################################################

if (CMAKE_PROJECT_NAME STREQUAL __ZAPPY_PROJECT_NAME_SNAKE__)
    add_custom_target(
        docs
        COMMAND sphinx-build -b html ${CMAKE_SOURCE_DIR}/docs ${CMAKE_SOURCE_DIR}/public
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
    )
endif()
