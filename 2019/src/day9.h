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

class day9 {
public:
  static void solve(const std::string &input_path) {
    auto program = parse(input_path);

    auto valid = part_one(program);

    std::cout << "Part one: " << valid << std::endl;

    auto sum = part_two(program);

    std::cout << "Part two: " << sum << std::endl;
  }

  static aoc::intcode::program_t parse(const std::string &input_path) {
    aoc::intcode::program_t program{};
    std::ifstream inputFile{input_path};

    for (std::string line; std::getline(inputFile, line, ',');) {
      program.push_back(std::stoi(line));
    }
    return program;
  }

  static long long int part_one(aoc::intcode::program_t program) {
    aoc::intcode::channel inputs{1};
    aoc::intcode::channel outputs{};
    aoc::intcode::run_program(program, inputs, outputs);

    return outputs.back();
  }


  static int part_two(aoc::intcode::program_t program) {
    aoc::intcode::channel inputs{2};
    aoc::intcode::channel outputs{};
    aoc::intcode::run_program(program, inputs, outputs);

    return outputs.back();
  }

}; // namespace aoc
} // namespace aoc
