#include "../src/day7.h"
#include <gtest/gtest.h>

TEST(DAY7, Example0) {
  auto program = aoc::day7::parse("tests/day7/example0");
  int output = aoc::day7::run_amplifiers(program, {4,3,2,1,0});
  EXPECT_EQ(output, 43210);
}

TEST(DAY7, Example1) {
  auto program = aoc::day7::parse("tests/day7/example1");
  int output = aoc::day7::run_amplifiers(program, {0,1,2,3,4});
  EXPECT_EQ(output, 54321);
}

TEST(DAY7, Example2) {
  auto program = aoc::day7::parse("tests/day7/example2");
  int output = aoc::day7::run_amplifiers(program, {1,0,4,3,2});
  EXPECT_EQ(output, 65210);
}