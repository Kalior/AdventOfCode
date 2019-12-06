macro(dependency name)
    # Download and unpack googletest at configure time
    configure_file(CMakeLists.txt.${name} ${name}-download/CMakeLists.txt)
    execute_process(COMMAND ${CMAKE_COMMAND} -G "${CMAKE_GENERATOR}" .
            RESULT_VARIABLE result
            WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/${name}-download )
    if(result)
        message(FATAL_ERROR "CMake step for ${name} failed: ${result}")
    endif()
    execute_process(COMMAND ${CMAKE_COMMAND} --build .
            RESULT_VARIABLE result
            WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/${name}-download )
    if(result)
        message(FATAL_ERROR "Build step for ${name} failed: ${result}")
    endif()

    # Add googletest directly to our build. This defines
    # the gtest and gtest_main targets.
    add_subdirectory(${CMAKE_CURRENT_BINARY_DIR}/${name}-src
            ${CMAKE_CURRENT_BINARY_DIR}/${name}-build
            EXCLUDE_FROM_ALL)
endmacro(dependency)

# Prevent overriding the parent project's compiler/linker
# settings on Windows
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)

dependency(googletest)


macro(test_case unit_test)
    get_filename_component(target "${unit_test}" NAME_WE)

    add_executable(${target} ${unit_test})
    target_link_libraries(${target} gtest_main)

    add_test(NAME ${target} COMMAND ${target})

    unset (unit_test)
    unset (target)
endmacro(test_case)

test_case(day1_test.cpp)
test_case(day2_test.cpp)
test_case(day3_test.cpp)
test_case(day4_test.cpp)
test_case(day5_test.cpp)
test_case(day6_test.cpp)
