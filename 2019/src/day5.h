#pragma once

#include <array>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>
#include <deque>

#include "intcode.h"

namespace aoc {

class day5 {
public:
  static void solve(const std::string &input_path) {
    auto program = parse(input_path);

    auto valid = part_one(program);

    std::cout << "Part one: " << valid << std::endl;

    auto sum = part_two(program);

    std::cout << "Part two: " << sum << std::endl;
  }

  static std::vector<int> parse(const std::string &input_path) {
    std::vector<int> program{};
    std::ifstream inputFile{input_path};

    for (std::string line; std::getline(inputFile, line, ',');) {
      program.push_back(std::stoi(line));
    }
    return program;
  }

  static int part_one(std::vector<int> program) {
    std::deque<int> inputs{1};
    std::deque<int> outputs{};
    run_program(program, inputs, outputs);

    return outputs.back();
  }

  static void run_program(std::vector<int> &program, std::deque<int> &inputs, std::deque<int> &outputs) {
    int program_pointer = 0;

    while (program[program_pointer] != 99) {
      program_pointer = intcode::run_instruction(program, program_pointer, inputs, outputs);
    }
  }


  static int part_two(std::vector<int> program) {
    std::deque<int> inputs{5};
    std::deque<int> outputs{};
    run_program(program, inputs, outputs);

    return outputs[0];
  }

}; // namespace aoc
} // namespace aoc
