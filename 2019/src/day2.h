#pragma once

#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

#include "intcode.h"

namespace aoc {

class day2 {
public:
  static void solve(const std::string &input_path) {
    auto program = parse(input_path);

    auto program_output = part_one(program);

    std::cout << "Part one: " << program_output[0] << std::endl;

    int sum = part_two(program);

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

  static aoc::intcode::program_t part_one(aoc::intcode::program_t program) {
    program[1] = 12;
    program[2] = 2;
    aoc::intcode::run_program(program);

    return program;
  }


  static int part_two(aoc::intcode::program_t program) {
    int sought_output = 19690720;
    for (int noun = 0; noun < 99; noun++) {
      for (int verb = 0; verb < 99; verb++) {
        aoc::intcode::program_t new_program(program.begin(), program.end());
        new_program[1] = noun;
        new_program[2] = verb;
        aoc::intcode::run_program(new_program);

        if (new_program[0] == sought_output) {
          return 100 * noun + verb;
        }
      }
    }
    return -1;
  }

};
} // namespace aoc
