#include "../src/day1.h"
#include <gtest/gtest.h>

TEST(DAY1, FuelRequirement) {
  EXPECT_EQ(aoc::day1::fuel_requirement(12), 2);
  EXPECT_EQ(aoc::day1::fuel_requirement(14), 2);
  EXPECT_EQ(aoc::day1::fuel_requirement(1969), 654);
  EXPECT_EQ(aoc::day1::fuel_requirement(100756), 33583);
}

TEST(DAY1, ExtraFuel) {
  EXPECT_EQ(aoc::day1::extra_fuel_requirements(2), 0);
  EXPECT_EQ(aoc::day1::extra_fuel_requirements(654), 216 + 70 + 21 + 5);
  EXPECT_EQ(aoc::day1::extra_fuel_requirements(33583), 50346 - 33583);
  EXPECT_EQ(aoc::day1::extra_fuel_requirements(3327415), 4991081 - 3327415);
}

TEST(DAY1, PartTwo) {
  EXPECT_EQ(aoc::day1::part_two({14}), 2);
  EXPECT_EQ(aoc::day1::part_two({1969}), 966);
  EXPECT_EQ(aoc::day1::part_two({100756}), 50346);
}