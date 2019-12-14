cmake_minimum_required(VERSION 3.16)
project(aoc-2019)

set(CMAKE_CXX_STANDARD 20)

set(THREADS_PREFER_PTHREAD_FLAG ON)
find_package(Threads REQUIRED)

add_subdirectory(tests)

add_executable(2019 main.cpp)
target_link_libraries(2019 Threads::Threads)

