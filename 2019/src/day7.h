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

  static std::vector<int> parse(const std::string &input_path) {
    std::vector<int> program{};
    std::ifstream inputFile{input_path};

    for (std::string line; std::getline(inputFile, line, ',');) {
      program.push_back(std::stoi(line));
    }
    return program;
  }

  static int part_one(const std::vector<int> &program) {

    int max_output = 0;
    std::array<int, 5> phase_settings{0, 1, 2, 3, 4};
    do {
      int new_output = run_amplifiers(program, phase_settings);
      max_output = std::max(max_output, new_output);
    } while (
        std::next_permutation(phase_settings.begin(), phase_settings.end()));

    return max_output;
  }

  static int run_amplifiers(const std::vector<int>& program,
                            std::array<int, 5> phase_settings) {
    std::vector<std::deque<int>> inputs{};
    for (int i = 0; i < 5; i++) {
      inputs.push_back(std::deque<int>{phase_settings[i]});
    }
    inputs[0].push_back(0);

    std::vector<std::thread> threads{};
    for (int i = 0; i < 5; i++) {
      int output_i = i == 4 ? 0 : i + 1;
      std::thread thread(run_program, program, std::ref(inputs[i]),
                         std::ref(inputs[output_i]));
      threads.push_back(std::move(thread));
    }

    for (int i = 0; i < 5; i++) {
      threads.at(i).join();
    }

    int output = inputs.at(0).at(0);
    return output;
  }

  static void run_program(std::vector<int> program, std::deque<int> &inputs,
                          std::deque<int> &outputs) {
    int program_pointer = 0;

    while (program[program_pointer] != 99) {
      program_pointer =
          intcode::run_instruction(program, program_pointer, inputs, outputs);
    }
  }

  static int part_two(const std::vector<int> &program) {
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
