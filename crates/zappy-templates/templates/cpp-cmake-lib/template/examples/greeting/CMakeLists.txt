cmake_minimum_required(VERSION 3.12)

#####################################################################################
# Project setup
#####################################################################################

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
project(tb_with_sc_ref_model)

#####################################################################################
# App includes & sources setup
#####################################################################################

file(GLOB prjSrcs ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)

#####################################################################################
# Executable setup
#####################################################################################

add_executable(greeting ${prjSrcs})
target_link_libraries(greeting __ZAPPY_PROJECT_NAME_SNAKE__)

#####################################################################################
# Misc targets
#####################################################################################

add_custom_target(
    run_greeting
    DEPENDS
        greeting
    USES_TERMINAL
    COMMAND
        ${CMAKE_CURRENT_BINARY_DIR}/greeting
)
