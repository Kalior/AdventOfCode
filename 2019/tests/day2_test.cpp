#include "../src/day2.h"
#include <gtest/gtest.h>

TEST(DAY2, Programs) {
  std::vector<int> program0{1, 0, 0, 0, 99};
  aoc::day2::run_program(program0);
  EXPECT_EQ(program0, std::vector<int>({2, 0, 0, 0, 99}));

  std::vector<int> program1{2, 3, 0, 3, 99};
  aoc::day2::run_program(program1);
  EXPECT_EQ(program1, std::vector<int>({2, 3, 0, 6, 99}));

  std::vector<int> program2{2, 4, 4, 5, 99, 0};
  aoc::day2::run_program(program2);
  EXPECT_EQ(program2, std::vector<int>({2, 4, 4, 5, 99, 9801}));

  std::vector<int> program3{1, 1, 1, 4, 99, 5, 6, 0, 99};
  aoc::day2::run_program(program3);
  EXPECT_EQ(program3, std::vector<int>({30, 1, 1, 4, 2, 5, 6, 0, 99}));
}
