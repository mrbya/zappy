#####################################################################################
# Test setup
#####################################################################################

macro(setup_test testTarget)
    set(oneValueArgs TARGET_NAME TEST_DIR)
    set(multiValueArgs INCLUDE_DIRS SRC_DIRS APP_SRCS APP_INCLUDES APP_LIBS)

    cmake_parse_arguments(
        arg_setup_test
        "" "${oneValueArgs}" "${multiValueArgs}"
        ${ARGN}
    )

    find_package(GTest)

    if(DEFINED arg_setup_test_TARGET_NAME)
        set(testTargetName ${arg_setup_test_TARGET_NAME})
    else()
        set(testTargetName test)
    endif()

    if(DEFINED arg_setup_test_TEST_DIR)
        set(testDir ${arg_setup_test_TEST_DIR})
    else()
        set(testDir test)
    endif()

    if(DEFINED arg_setup_test_INCLUDE_DIRS)
        set(testIncludesDir ${arg_setup_test_INCLUDE_DIRS})
    else()
        set(testIncludesDir ${testDir}/include)
    endif()

    if(DEFINED arg_setup_test_SRC_DIRS)
        set(testSrcDirs ${arg_setup_test_SRC_DIRS})
    else()
        set(testSrcDirs ${testDir}/src)
    endif()

    if(DEFINED arg_setup_test_APP_SRCS)
        set(appSrcs ${arg_setup_test_APP_SRCS})
    endif()

    if(DEFINED arg_setup_test_APP_INCLUDES)
        set(appIncludes ${arg_setup_test_APP_INCLUDES})
    endif()

    if(DEFINED arg_setup_test_APP_LIBS)
        set(appLibs ${arg_setup_test_APP_LIBS})
    endif()

    set(testIncludes ${testIncludesDir})

    set(testSrcs)
    foreach(dir IN LISTS testSrcDirs)
        file(GLOB srcsInDir ${dir}/*.cpp)
        list(APPEND testSrcs ${srcsInDir})
    endforeach()

    add_executable(testbin EXCLUDE_FROM_ALL ${testSrcs} ${appSrcs})

    target_include_directories(testbin PUBLIC ${GTEST_INCLUDE_DIRS} ${testIncludes})
    if(DEFINED appIncludes)
        target_include_directories(testbin PUBLIC ${appIncludes})
    endif()

    target_link_libraries(testbin PUBLIC GTest::GTest GTest::Main Threads::Threads)
    if(DEFINED appLibs)
        target_link_libraries(testbin PUBLIC ${appLibs})
    endif()

    add_custom_target(
        ${testTargetName}
        DEPENDS
            testbin
        COMMAND
            ${CMAKE_BINARY_DIR}/testbin
        USES_TERMINAL
    )

    add_dependencies(${testTargetName} ${testTargetName})
endmacro()

#####################################################################################
# Test coverage setup
#####################################################################################
macro(setup_coverage)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fprofile-arcs -ftest-coverage")
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -fprofile-arcs -ftest-coverage")
    set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -fprofile-arcs -ftest-coverage")
    set(CMAKE_CXX_OUTPUT_EXTENSION_REPLACE ON)

    set(objectDirs)
    foreach(dir IN LISTS testSrcDirs)
        set(suffix)
        string(REPLACE ${CMAKE_SOURCE_DIR}/${testDir} "" suffix ${dir})
        list(APPEND objectDirs ${CMAKE_BINARY_DIR}/CMakeFiles/testbin.dir/${testDir}/${suffix})
    endforeach()
    set(OBJECT_DIR ${CMAKE_BINARY_DIR}/CMakeFiles/testbin.dir/${testDir}/src)

    add_custom_target(
        covprep
        DEPENDS
            ${testTargetName}
        COMMAND
            mkdir -p coverage
    )

    add_custom_target(
        gcov
        DEPENDS
            covprep
        COMMAND
            gcov -b ${testSrcs} -o ${OBJECT_DIR}
        WORKING_DIRECTORY
            ${CMAKE_BINARY_DIR}/coverage
    )

    add_custom_target(
        cov
        DEPENDS
            gcov
        COMMAND lcov -c -d . -o coverage.info --ignore-errors inconsistent
        COMMAND lcov -r coverage.info '/usr/*' '*/${testTargetName}/*' -o coverage.info
        COMMAND genhtml coverage.info -o coverage
    )

    add_custom_target(
        report
        DEPENDS
            cov
        COMMAND
            xdg-open ${CMAKE_BINARY_DIR}/coverage/index.html
    )

    set_property(DIRECTORY APPEND PROPERTY ADDITIONAL_MAKE_CLEAN_FILES coverage)
endmacro()
