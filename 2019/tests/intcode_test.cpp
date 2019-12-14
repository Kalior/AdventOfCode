#include "../src/intcode.h"
#include <gtest/gtest.h>
#include <vector>
#include <deque>

TEST(IntCode, Add) {
  std::vector<int> program{1, 3, 5, 4, -1};
  int new_pointer = aoc::intcode::add(program, 0, 1, 1);

  EXPECT_EQ(new_pointer, 4);
  EXPECT_EQ(program[4], 8);
}

TEST(IntCode, Add_Indirect) {
  std::vector<int> program{1, 5, 6, 4, -1, 3, 5};
  int new_pointer = aoc::intcode::add(program, 0, 0, 0);

  EXPECT_EQ(new_pointer, 4);
  EXPECT_EQ(program[4], 8);
}

TEST(IntCode, Mul) {
  std::vector<int> program{2, 3, 5, 4, -1};
  int new_pointer = aoc::intcode::mul(program, 0, 1, 1);

  EXPECT_EQ(new_pointer, 4);
  EXPECT_EQ(program[4], 15);
}

TEST(IntCode, Input) {
  std::vector<int> program{3, 2, -1};
  std::deque<int> input{1};
  int new_pointer = aoc::intcode::input(program, 0, input);

  EXPECT_EQ(new_pointer, 2);
  EXPECT_EQ(program[2], 1);
}

TEST(IntCode, Output) {
  std::vector<int> program{4, 3};
  std::deque<int> output{};
  int new_pointer = aoc::intcode::output(program, 0, output, 1);

  EXPECT_EQ(new_pointer, 2);
  EXPECT_EQ(output[0], 3);
}

TEST(IntCode, JumpNEQ_NEQ) {
  std::vector<int> program{5, 1, 5};
  int new_pointer = aoc::intcode::jump_neq(program, 0, 1, 1);

  EXPECT_EQ(new_pointer, 5);
  EXPECT_EQ(program, (std::vector<int>{5, 1, 5}));
}

TEST(IntCode, JumpNEQ_EQ) {
  std::vector<int> program{5, 0, 5};
  int new_pointer = aoc::intcode::jump_neq(program, 0, 1, 1);

  EXPECT_EQ(new_pointer, 3);
  EXPECT_EQ(program, (std::vector<int>{5, 0, 5}));
}

TEST(IntCode, JumpEQ_EQ) {
  std::vector<int> program{6, 0, 5};
  int new_pointer = aoc::intcode::jump_eq(program, 0, 1, 1);

  EXPECT_EQ(new_pointer, 5);
  EXPECT_EQ(program, (std::vector<int>{6, 0, 5}));
}

TEST(IntCode, JumpEQ_NEQ) {
  std::vector<int> program{6, 1, 5};
  int new_pointer = aoc::intcode::jump_eq(program, 0, 1, 1);

  EXPECT_EQ(new_pointer, 3);
  EXPECT_EQ(program, (std::vector<int>{6, 1, 5}));
}

TEST(IntCode, less_than) {
  std::vector<int> program{7, 3, 5, 4, -1};
  int new_pointer = aoc::intcode::less_than(program, 0, 1, 1);

  EXPECT_EQ(new_pointer, 4);
  EXPECT_EQ(program[4], 1);
}

TEST(IntCode, less_than_greater) {
  std::vector<int> program{7, 3, 1, 4, -1};
  int new_pointer = aoc::intcode::less_than(program, 0, 1, 1);

  EXPECT_EQ(new_pointer, 4);
  EXPECT_EQ(program[4], 0);
}

TEST(IntCode, equals) {
  std::vector<int> program{8, 3, 3, 4, -1};
  int new_pointer = aoc::intcode::equals(program, 0, 1, 1);

  EXPECT_EQ(new_pointer, 4);
  EXPECT_EQ(program[4], 1);
}

TEST(IntCode, not_equals) {
  std::vector<int> program{8, 4, 3, 4, -1};
  int new_pointer = aoc::intcode::equals(program, 0, 1, 1);

  EXPECT_EQ(new_pointer, 4);
  EXPECT_EQ(program[4], 0);
}

TEST(IntCode, parse_parameters_position) {
  auto [mode_a, mode_b, mode_c, op_code] = aoc::intcode::get_parameters(0001);
  EXPECT_EQ(mode_a, 0);
  EXPECT_EQ(mode_b, 0);
  EXPECT_EQ(mode_c, 0);
  EXPECT_EQ(op_code, 1);
}

TEST(IntCode, parse_parameters_immediate) {
  auto [mode_a, mode_b, mode_c, op_code] = aoc::intcode::get_parameters(11101);
  EXPECT_EQ(mode_a, 1);
  EXPECT_EQ(mode_b, 1);
  EXPECT_EQ(mode_c, 1);
  EXPECT_EQ(op_code, 1);
}

TEST(IntCode, parse_parameters_position_3) {
  auto [mode_a, mode_b, mode_c, op_code] = aoc::intcode::get_parameters(3);
  EXPECT_EQ(mode_a, 0);
  EXPECT_EQ(mode_b, 0);
  EXPECT_EQ(mode_c, 0);
  EXPECT_EQ(op_code, 3);
}
