#pragma once

#include <array>
#include <deque>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <thread>
#include <vector>

#include "intcode.h"

namespace aoc {

class day7 {
public:
  static void solve(const std::string &input_path) {
    auto program = parse(input_path);

    auto one = part_one(program);

    std::cout << "Part one: " << one << std::endl;

    auto two = part_two(program);

    std::cout << "Part two: " << two << std::endl;
  }

  static aoc::intcode::program_t parse(const std::string &input_path) {
    aoc::intcode::program_t program{};
    std::ifstream inputFile{input_path};

    for (std::string line; std::getline(inputFile, line, ',');) {
      program.push_back(std::stoi(line));
    }
    return program;
  }

  static int part_one(const aoc::intcode::program_t &program) {

    int max_output = 0;
    std::array<int, 5> phase_settings{0, 1, 2, 3, 4};
    do {
      int new_output = run_amplifiers(program, phase_settings);
      max_output = std::max(max_output, new_output);
    } while (
        std::next_permutation(phase_settings.begin(), phase_settings.end()));

    return max_output;
  }

  static int run_amplifiers(const aoc::intcode::program_t &program,
                            std::array<int, 5> phase_settings) {
    std::vector<aoc::intcode::channel> inputs{};
    for (int i = 0; i < 5; i++) {
      inputs.push_back(aoc::intcode::channel{phase_settings[i]});
    }
    inputs[0].push_back(0);

    std::vector<std::thread> threads{};
    for (int i = 0; i < 5; i++) {
      int output_i = i == 4 ? 0 : i + 1;
      std::thread thread(aoc::intcode::run_program_thread, program,
                         std::ref(inputs[i]), std::ref(inputs[output_i]));
      threads.push_back(std::move(thread));
    }

    for (int i = 0; i < 5; i++) {
      threads.at(i).join();
    }

    int output = inputs.at(0).at(0);
    return output;
  }

  static int part_two(const aoc::intcode::program_t &program) {
    int max_output = 0;
    std::array<int, 5> phase_settings{5, 6, 7, 8, 9};
    do {
      int new_output = run_amplifiers(program, phase_settings);
      max_output = std::max(max_output, new_output);
    } while (
        std::next_permutation(phase_settings.begin(), phase_settings.end()));

    return max_output;
  }

}; // namespace aoc
} // namespace aoc
