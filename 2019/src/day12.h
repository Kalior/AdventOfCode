#pragma once

#include <fstream>
#include <iostream>
#include <numeric>
#include <regex>
#include <string>
#include <vector>

#include "position.h"

namespace aoc {

using input_t = std::vector<Vector3D>;

class day12 {
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

    for (std::string line; std::getline(inputFile, line, '\n');) {
      input.push_back(parse_vector(line));
    }
    return input;
  }

  static Vector3D parse_vector(const std::string &line) {
    std::regex int_regex{"-?[0-9]+"};

    auto ints_begin = std::sregex_iterator(line.begin(), line.end(), int_regex);
    auto ints_end = std::sregex_iterator();

    std::vector<int> ints{};

    for (std::sregex_iterator i = ints_begin; i != ints_end; ++i) {
      std::smatch match = *i;
      std::string match_str = match.str();
      ints.push_back(std::stoi(match_str));
    }
    return Vector3D{ints[0], ints[1], ints[2]};
  }

  static int part_one(input_t moons) {
    input_t velocities(moons.size(), {0, 0, 0});

    int update_steps = 1000;
    for (int i = 0; i < update_steps; i++) {
      auto new_gravity = update_gravity(moons);
      for (int m = 0; m < moons.size(); m++) {
        velocities[m] = velocities[m] + new_gravity[m];
      }

      for (int m = 0; m < moons.size(); m++) {
        moons[m] = moons[m] + velocities[m];
      }

    }

    int total_energy = 0;
    for (int m = 0; m < moons.size(); m++) {
      total_energy += energy(moons[m]) * energy(velocities[m]);
    }

    return total_energy;
  }

  static input_t update_gravity(input_t moons) {
    input_t new_velocities(moons.size(), {0, 0, 0});
    for (int i = 0; i < moons.size(); i++) {
      for (int j = i + 1; j < moons.size() && i != j; j++) {
        auto [i_gravity, j_gravity] = get_gravity(moons[i], moons[j]);
        new_velocities[i] = new_velocities[i] + i_gravity;
        new_velocities[j] = new_velocities[j] + j_gravity;
      }
    }

    return new_velocities;
  }

  static std::tuple<Vector3D, Vector3D> get_gravity(Vector3D lhs,
                                                    Vector3D rhs) {

    auto [lhs_x, lhs_y, lhs_z] = lhs;
    auto [rhs_x, rhs_y, rhs_z] = rhs;

    auto [lhs_x_pull, rhs_x_pull] = gravity_pull(lhs_x, rhs_x);
    auto [lhs_y_pull, rhs_y_pull] = gravity_pull(lhs_y, rhs_y);
    auto [lhs_z_pull, rhs_z_pull] = gravity_pull(lhs_z, rhs_z);

    return {Vector3D{lhs_x_pull, lhs_y_pull, lhs_z_pull},
            Vector3D{rhs_x_pull, rhs_y_pull, rhs_z_pull}};
  }

  static std::tuple<int, int> gravity_pull(int lhs, int rhs) {
    if (lhs < rhs) {
      return {1, -1};
    } else if (lhs > rhs) {
      return {-1, 1};
    } else {
      return {0, 0};
    }
  }

  static int energy(Vector3D moon) {
    auto &[x, y, z] = moon;

    return std::abs(x) + std::abs(y) + std::abs(z);
  }

  static void debug_print(const input_t &moons, const input_t &velocities) {
    for (int m = 0; m < moons.size(); m++) {
      std::cout << "Pos = " << std::get<0>(moons[m]) << " "
                << std::get<1>(moons[m]) << " " << std::get<2>(moons[m])
                << " Vel = " << std::get<0>(velocities[m]) << " "
                << std::get<1>(velocities[m]) << " "
                << std::get<2>(velocities[m]) << std::endl;
    }
    std::cout << std::endl;
  }

  static int part_two(input_t input) { return -1; }
};
} // namespace aoc
