#include <array>
#include <vector>

namespace aoc::intcode {

static std::array<int, 4> get_parameters(int instruction) {
  int opcode = instruction % 100;
  int mode_c = int(instruction / 100) % 10;
  int mode_b = int(instruction / 1000) % 10;
  int mode_a = int(instruction / 10000) % 10;
  return {mode_a, mode_b, mode_c, opcode};
}


static int run_instruction(std::vector<int> &program, int program_pointer) {
  auto [mode_a, mode_b, mode_c, op_code] =
  get_parameters(program[program_pointer]);
  int value_one = program[program_pointer + 1];
  if (mode_c == 0) {
    value_one = program[value_one];
  }

  int value_two = program[program_pointer + 2];
  if (mode_b == 0) {
    value_two = program[value_two];
  }

  int output = program[program_pointer + 3];

  if (op_code == 1) {
    program[output] = value_one + value_two;
    program_pointer += 4;
  } else if (op_code == 2) {
    program[output] = value_one * value_two;
    program_pointer += 4;
  } else if (op_code == 3) {
    std::cout << "Give input: " << std::endl;
    int val;
    std::cin >> val;
    program[program[program_pointer + 1]] = val;
    program_pointer += 2;
  } else if (op_code == 4) {
    std::cout << value_one << std::endl;
    program_pointer += 2;
  } else if (op_code == 5) {
    if (value_one != 0) {
      program_pointer = value_two;
    } else {
      program_pointer += 3;
    }
  } else if (op_code == 6) {
    if (value_one == 0) {
      program_pointer = value_two;
    } else {
      program_pointer += 3;
    }
  } else if (op_code == 7) {
    if (value_one < value_two) {
      program[output] = 1;
    } else {
      program[output] = 0;
    }
    program_pointer += 4;
  } else if (op_code == 8) {
    if (value_one == value_two) {
      program[output] = 1;
    } else {
      program[output] = 0;
    }
    program_pointer += 4;
  } else {
    throw std::invalid_argument("No such opcode " + std::to_string(op_code));
  }

  return program_pointer;
}
}