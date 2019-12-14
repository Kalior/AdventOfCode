#include "../src/day2.h"
#include "../src/intcode.h"
#include <gtest/gtest.h>

TEST(DAY2, Programs) {
  aoc::intcode::program_t program0{1, 0, 0, 0, 99};
  aoc::intcode::run_program(program0);
  EXPECT_EQ(program0, aoc::intcode::program_t({2, 0, 0, 0, 99}));

  aoc::intcode::program_t program1{2, 3, 0, 3, 99};
  aoc::intcode::run_program(program1);
  EXPECT_EQ(program1, aoc::intcode::program_t({2, 3, 0, 6, 99}));

  aoc::intcode::program_t program2{2, 4, 4, 5, 99, 0};
  aoc::intcode::run_program(program2);
  EXPECT_EQ(program2, aoc::intcode::program_t({2, 4, 4, 5, 99, 9801}));

  aoc::intcode::program_t program3{1, 1, 1, 4, 99, 5, 6, 0, 99};
  aoc::intcode::run_program(program3);
  EXPECT_EQ(program3, aoc::intcode::program_t({30, 1, 1, 4, 2, 5, 6, 0, 99}));
}
