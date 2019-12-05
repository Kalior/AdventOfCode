#pragma once

#include <array>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

#include "intcode.h"

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
      program_pointer = intcode::run_instruction(program, program_pointer);
    }
  }

}; // namespace aoc
} // namespace aoc
