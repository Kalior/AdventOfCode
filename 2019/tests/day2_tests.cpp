#include "../src/day2.h"
#include <gtest/gtest.h>

TEST(DAY2, Programs) {
  EXPECT_EQ(aoc::day2::run_program({1,0,0,0,99}), std::vector<int>({2,0,0,0,99}));
  EXPECT_EQ(aoc::day2::run_program({2,3,0,3,99}), std::vector<int>({2,3,0,6,99}));
  EXPECT_EQ(aoc::day2::run_program({2,4,4,5,99,0}), std::vector<int>({2,4,4,5,99,9801}));
  EXPECT_EQ(aoc::day2::run_program({1,1,1,4,99,5,6,0,99}), std::vector<int>({30,1,1,4,2,5,6,0,99}));
}
