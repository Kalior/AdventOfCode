#include "../src/day4.h"
#include <gtest/gtest.h>

TEST(DAY4, CheckAdjacentDigits) {
  EXPECT_TRUE(aoc::day4::has_adjacent_digits(1112));
  EXPECT_TRUE(aoc::day4::has_adjacent_digits(99));
  EXPECT_FALSE(aoc::day4::has_adjacent_digits(1919));
}

TEST(DAY4, CheckIsIncreasing) {
  EXPECT_TRUE(aoc::day4::is_increasing(1112));
  EXPECT_TRUE(aoc::day4::is_increasing(99));
  EXPECT_FALSE(aoc::day4::is_increasing(1919));
}

TEST(DAY4, Examples) {
  EXPECT_TRUE(aoc::day4::is_valid(111111));
  EXPECT_FALSE(aoc::day4::is_valid(223450));
  EXPECT_FALSE(aoc::day4::is_valid(123789));
}

TEST(DAY4, CheckAdjacentDigits2) {
  EXPECT_FALSE(aoc::day4::has_adjacent_digits_2(1112));
  EXPECT_TRUE(aoc::day4::has_adjacent_digits_2(99));
  EXPECT_FALSE(aoc::day4::has_adjacent_digits_2(1919));
  EXPECT_TRUE(aoc::day4::has_adjacent_digits_2(11122));
  EXPECT_TRUE(aoc::day4::has_adjacent_digits_2(112233));
  EXPECT_FALSE(aoc::day4::has_adjacent_digits_2(123444));
}
