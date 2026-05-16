cmake_minimum_required(VERSION 3.20)

project(testModule)

set(CMAKE_INSTALL_PREFIX /usr/share)
install(
    FILES
        gtestSetup.cmake
    DESTINATION
        cmake/Modules
)
