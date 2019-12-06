#pragma once

#include <array>
#include <fstream>
#include <iostream>
#include <map>
#include <numeric>
#include <string>
#include <tuple>
#include <vector>

namespace aoc {

class day6 {
public:
  static void solve(const std::string &input_path) {
    auto direct_orbits = parse(input_path);

    auto valid = part_one(direct_orbits);

    std::cout << "Part one: " << valid << std::endl;

    auto sum = part_two(direct_orbits);

    std::cout << "Part two: " << sum << std::endl;
  }

  static std::vector<std::tuple<std::string, std::string>>
  parse(const std::string &input_path) {
    std::vector<std::tuple<std::string, std::string>> direct_orbits{};
    std::ifstream inputFile{input_path};

    for (std::string line; std::getline(inputFile, line, '\n');) {
      std::string object(line.begin(), line.begin() + 3);
      std::string orbiting(line.begin() + 4, line.end());
      direct_orbits.emplace_back(object, orbiting);
    }
    return direct_orbits;
  }

  static int part_one(
      const std::vector<std::tuple<std::string, std::string>> &direct_orbits) {

    std::map<std::string, std::string> orbits{};

    for (auto &[object, orbiting] : direct_orbits) {
      orbits[orbiting] = object;
    }

    int total_number_of_orbits = 0;
    for (auto &[object_, orbiting]: direct_orbits) {
      auto object {object_};
      total_number_of_orbits++;
      while (object != "COM") {
        total_number_of_orbits++;
        object = orbits[object];
      }
    }

    return total_number_of_orbits;
  }

  static int part_two(
      const std::vector<std::tuple<std::string, std::string>> &direct_orbits) {


    std::map<std::string, std::string> orbits{};

    for (auto &[object, orbiting] : direct_orbits) {
      orbits[orbiting] = object;
    }


    std::map<std::string, int> transfers{};
    auto object {orbits["YOU"]};
    int me_transfers = 0;
    while (object != "COM") {
      transfers[object] = me_transfers;
      me_transfers++;
      object = orbits[object];
    }

    object = orbits["SAN"];
    int san_transfers = 0;
    while (object != "COM") {
      if (transfers.contains(object)) {
        return transfers[object] + san_transfers;
      }
      san_transfers++;
      object = orbits[object];
    }

    throw std::logic_error("No shared orbits found");
  }

}; // namespace aoc
} // namespace aoc
