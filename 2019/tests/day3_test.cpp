#include "../src/day3.h"
#include <gtest/gtest.h>

TEST(DAY3, Example0) {
  auto wires = aoc::day3::parse("tests/day3/example0");
  auto one = aoc::day3::part_one(wires);
  auto two = aoc::day3::part_two(wires);
  EXPECT_EQ(one, 6);
  EXPECT_EQ(two, 30);
}

TEST(DAY3, Example1) {
  auto wires = aoc::day3::parse("tests/day3/example1");
  auto one = aoc::day3::part_one(wires);
  auto two = aoc::day3::part_two(wires);
  EXPECT_EQ(one, 159);
  EXPECT_EQ(two, 610);

}
TEST(DAY3, Example2) {
  auto wires = aoc::day3::parse("tests/day3/example2");
  auto one = aoc::day3::part_one(wires);
  auto two = aoc::day3::part_two(wires);
  EXPECT_EQ(one, 135);
  EXPECT_EQ(two, 410);
}