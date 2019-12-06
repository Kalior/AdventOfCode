#include "../src/day6.h"
#include <gtest/gtest.h>

TEST(DAY6, Example0) {
  auto orbits = aoc::day6::parse("tests/day6_test_input");
  auto two = aoc::day6::part_two(orbits);
  EXPECT_EQ(two, 4);
}
