#pragma once

#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

#include "position.h"

namespace aoc {

using input_t = std::vector<int>;

class day0 {
public:
  static void solve(const std::string &input_path) {
    auto program = parse(input_path);

    auto one = part_one(program);

    std::cout << "Part one: " << one << std::endl;

    auto two = part_two(program);

    std::cout << "Part two: " << two << std::endl;
  }

  static input_t parse(const std::string &input_path) {
    input_t input{};
    std::ifstream inputFile{input_path};

    for (std::string line; std::getline(inputFile, line, ',');) {
      input.push_back(std::stoi(line));
    }
    return input;
  }

  static int part_one(input_t input) {


    return -1;
  }


  static int part_two(input_t input) {

    return -1;
  }

};
} // namespace aoc
