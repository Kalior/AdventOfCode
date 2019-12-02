#pragma once

#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

namespace aoc {

class day1 {
public:
  static void solve(const std::string &input_path) {
    auto modules = parse(input_path);
    int full_fuel_requirements = part_one(modules);

    std::cout << "Part one: " << full_fuel_requirements << std::endl;

    int fuel_fuel_requirements = part_two(modules);

    std::cout << "Part two: " << fuel_fuel_requirements << std::endl;
  }

  static std::vector<int> parse(const std::string &input_path) {
    std::vector<int> modules{};
    std::ifstream inputFile{input_path};

    for (std::string line; std::getline(inputFile, line);) {
      modules.push_back(std::stoi(line));
    }
    return modules;
  }

  static int part_one(const std::vector<int> &modules) {
    return std::accumulate(
        modules.begin(), modules.end(), 0,
        [](int sum, int mass) { return sum + fuel_requirement(mass); });
  }

  static int fuel_requirement(int mass) { return int(mass / 3) - 2; }

  static int part_two(const std::vector<int> &modules) {
    return std::accumulate(modules.begin(), modules.end(), 0,
                           [](int sum, int mass) {
                             int fuel = fuel_requirement(mass);
                             return sum + fuel + extra_fuel_requirements(fuel);
                           });
  }

  static int extra_fuel_requirements(int module_requirement) {
    int extra {0};
    int new_fuel_requirement = fuel_requirement(module_requirement);
    while (new_fuel_requirement > 0) {
      extra += new_fuel_requirement;
      new_fuel_requirement = fuel_requirement(new_fuel_requirement);
    }

    return extra;
  }
};
} // namespace aoc
