#pragma once

#include <array>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

namespace aoc {

class day5 {
public:
  static void solve(const std::string &input_path) {
    auto program = parse(input_path);

    std::cout << "Part one: input 1.  Part two: input 5. "  << std::endl;
    part_one(program);
  }

  static std::vector<int> parse(const std::string &input_path) {
    std::vector<int> program{};
    std::ifstream inputFile{input_path};

    for (std::string line; std::getline(inputFile, line, ',');) {
      program.push_back(std::stoi(line));
    }
    return program;
  }

  static std::vector<int> part_one(std::vector<int> program) {
    run_program(program);

    return program;
  }

  static void run_program(std::vector<int> &program) {
    int program_pointer = 0;

    while (program[program_pointer] != 99) {
      program_pointer = run_instruction(program, program_pointer);
    }
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

  static std::array<int, 4> get_parameters(int instruction) {
    int opcode = instruction % 100;
    int mode_c = int(instruction / 100) % 10;
    int mode_b = int(instruction / 1000) % 10;
    int mode_a = int(instruction / 10000) % 10;
    return {mode_a, mode_b, mode_c, opcode};
  }

}; // namespace aoc
} // namespace aoc
