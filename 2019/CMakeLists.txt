cmake_minimum_required(VERSION 3.16)
project(aoc-2019)

set(CMAKE_CXX_STANDARD 20)


add_subdirectory(tests)

add_executable(2019 main.cpp src/day1.h src/day2.h src/day3.h)

