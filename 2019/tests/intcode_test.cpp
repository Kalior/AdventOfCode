#include "../src/intcode.h"
#include <deque>
#include <gtest/gtest.h>
#include <vector>

TEST(IntCode, Add) {
  aoc::intcode::program_t program{1, 3, 5, 4, -1};
  size_t program_pointer = 0;
  aoc::intcode::add(program, program_pointer, 0, 0, 1, 1);

  EXPECT_EQ(program_pointer, 4);
  EXPECT_EQ(program[4], 8);
}

TEST(IntCode, Add_Indirect) {
  aoc::intcode::program_t program{1, 5, 6, 4, -1, 3, 5};
  size_t program_pointer = 0;
  aoc::intcode::add(program, program_pointer, 0, 0, 0, 0);

  EXPECT_EQ(program_pointer, 4);
  EXPECT_EQ(program[4], 8);
}

TEST(IntCode, Mul) {
  aoc::intcode::program_t program{2, 3, 5, 4, -1};
  size_t program_pointer = 0;
  aoc::intcode::mul(program, program_pointer, 0, 0, 1, 1);

  EXPECT_EQ(program_pointer, 4);
  EXPECT_EQ(program[4], 15);
}

TEST(IntCode, Input) {
  aoc::intcode::program_t program{3, 2, -1};
  aoc::intcode::channel input{1};
  size_t program_pointer = 0;
  aoc::intcode::input(program, program_pointer, 0, input, 0);

  EXPECT_EQ(program_pointer, 2);
  EXPECT_EQ(program[2], 1);
}

TEST(IntCode, Output) {
  aoc::intcode::program_t program{4, 3};
  aoc::intcode::channel output{};
  size_t program_pointer = 0;
  aoc::intcode::output(program, program_pointer, 0, output, 1);

  EXPECT_EQ(program_pointer, 2);
  EXPECT_EQ(output[0], 3);
}

TEST(IntCode, JumpNEQ_NEQ) {
  aoc::intcode::program_t program{5, 1, 5};
  size_t program_pointer = 0;
  aoc::intcode::jump_neq(program, program_pointer, 0, 1, 1);

  EXPECT_EQ(program_pointer, 5);
  EXPECT_EQ(program, (aoc::intcode::program_t{5, 1, 5}));
}

TEST(IntCode, JumpNEQ_EQ) {
  aoc::intcode::program_t program{5, 0, 5};
  size_t program_pointer = 0;
  aoc::intcode::jump_neq(program, program_pointer, 0, 1, 1);

  EXPECT_EQ(program_pointer, 3);
  EXPECT_EQ(program, (aoc::intcode::program_t{5, 0, 5}));
}

TEST(IntCode, JumpEQ_EQ) {
  aoc::intcode::program_t program{6, 0, 5};
  size_t program_pointer = 0;
  aoc::intcode::jump_eq(program, program_pointer, 0, 1, 1);

  EXPECT_EQ(program_pointer, 5);
  EXPECT_EQ(program, (aoc::intcode::program_t{6, 0, 5}));
}

TEST(IntCode, JumpEQ_NEQ) {
  aoc::intcode::program_t program{6, 1, 5};
  size_t program_pointer = 0;
  aoc::intcode::jump_eq(program, program_pointer, 0, 1, 1);

  EXPECT_EQ(program_pointer, 3);
  EXPECT_EQ(program, (aoc::intcode::program_t{6, 1, 5}));
}

TEST(IntCode, less_than) {
  aoc::intcode::program_t program{7, 3, 5, 4, -1};
  size_t program_pointer = 0;
  aoc::intcode::less_than(program, program_pointer, 0, 0, 1, 1);

  EXPECT_EQ(program_pointer, 4);
  EXPECT_EQ(program[4], 1);
}

TEST(IntCode, less_than_greater) {
  aoc::intcode::program_t program{7, 3, 1, 4, -1};
  size_t program_pointer = 0;
  aoc::intcode::less_than(program, program_pointer, 0, 0, 1, 1);

  EXPECT_EQ(program_pointer, 4);
  EXPECT_EQ(program[4], 0);
}

TEST(IntCode, equals) {
  aoc::intcode::program_t program{8, 3, 3, 4, -1};
  size_t program_pointer = 0;
  aoc::intcode::equals(program, program_pointer, 0, 0, 1, 1);

  EXPECT_EQ(program_pointer, 4);
  EXPECT_EQ(program[4], 1);
}

TEST(IntCode, not_equals) {
  aoc::intcode::program_t program{8, 4, 3, 4, -1};
  size_t program_pointer = 0;
  aoc::intcode::equals(program, program_pointer, 0, 0, 1, 1);

  EXPECT_EQ(program_pointer, 4);
  EXPECT_EQ(program[4], 0);
}

TEST(IntCode, AdjustRelativeBase) {
  aoc::intcode::program_t program{9, 2, 3};
  size_t program_pointer = 0;
  size_t relative_base = 0;
  aoc::intcode::adjust_relative_base(program, program_pointer, relative_base,
                                     0);

  EXPECT_EQ(program_pointer, 2);
  EXPECT_EQ(relative_base, 3);
}

TEST(IntCode, parse_parameters_position) {
  auto [mode_a, mode_b, mode_c, op_code] = aoc::intcode::get_parameters(0001);
  EXPECT_EQ(mode_a, 0);
  EXPECT_EQ(mode_b, 0);
  EXPECT_EQ(mode_c, 0);
  EXPECT_EQ(op_code, 1);
}

TEST(IntCode, parse_parameters_immediate) {
  auto [mode_a, mode_b, mode_c, op_code] = aoc::intcode::get_parameters(21101);
  EXPECT_EQ(mode_a, 2);
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

TEST(IntCode, BigOutput) {
  aoc::intcode::program_t program{104, 1125899906842624, 99};
  aoc::intcode::channel output{};
  aoc::intcode::channel input{};
  aoc::intcode::run_program(program, input, output);

  EXPECT_EQ(output[0], 1125899906842624);
}

TEST(IntCode, OtherBigOutput) {
  aoc::intcode::program_t program{1102, 34915192, 34915192, 7, 4, 7, 99, 0};
  aoc::intcode::channel output{};
  aoc::intcode::channel input{};
  aoc::intcode::run_program(program, input, output);

  EXPECT_EQ(output[0], 1219070632396864);
}

TEST(IntCode, Quine) {
  aoc::intcode::program_t program{109,  1,   204, -1,  1001, 100, 1, 100,
                                  1008, 100, 16,  101, 1006, 101, 0, 99};
  aoc::intcode::channel output{};
  aoc::intcode::channel input{};
  aoc::intcode::run_program(program, input, output);

  EXPECT_EQ(output,
            (aoc::intcode::channel{109, 1, 204, -1, 1001, 100, 1, 100, 1008,
                                   100, 16, 101, 1006, 101, 0, 99}));
}