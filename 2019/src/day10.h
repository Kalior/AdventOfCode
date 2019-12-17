#pragma once

#include <cmath>
#include <fstream>
#include <iostream>
#include <map>
#include <numeric>
#include <set>
#include <string>
#include <tuple>
#include <vector>

#include "intcode.h"

namespace aoc {

enum Element : int { EMPTY, ASTEROID };

using Position = std::tuple<int, int>;

using input_t = std::map<Position, Element>;

int manhattan_distance(Position pos1, Position pos2) {
  auto [x1, y1] = pos1;
  auto [x2, y2] = pos2;
  return std::abs(x1 - x2) + std::abs(y1 - y2);
}

double degree(Position pos1, Position pos2) {
  auto [x1, y1] = pos1;
  auto [x2, y2] = pos2;
  auto dot = x1 * x2 + y1 * y2;
  auto magnitude_1 = std::sqrt(std::pow(x1, 2) + std::pow(y1, 2));
  auto magnitude_2 = std::sqrt(std::pow(x2, 2) + std::pow(y2, 2));

  auto angle = std::acos(dot / (magnitude_1 * magnitude_2));
  if (x1 < x2) {
    return -angle;
  } else {
    return angle;

  }
}

Position sign(Position pos) {
  auto [x, y] = pos;
  int new_x = 0;
  if (x > 0)
    new_x = 1;
  if (x < 0)
    new_x = -1;

  int new_y = 0;
  if (y > 0)
    new_y = 1;
  if (y < 0)
    new_y = -1;

  return Position{new_x, new_y};
}

Position gcd(Position pos) {
  auto [x, y] = pos;
  int gcd = std::gcd(x, y);
  return Position{x / gcd, y / gcd};
}

constexpr Position operator-(const Position &lhs, const Position &rhs) {
  auto [lhs_x, lhs_y] = lhs;
  auto [rhs_x, rhs_y] = rhs;
  return Position{lhs_x - rhs_x, lhs_y - rhs_y};
}

constexpr Position operator+(const Position &lhs, const Position &rhs) {
  auto [lhs_x, lhs_y] = lhs;
  auto [rhs_x, rhs_y] = rhs;
  return Position{lhs_x + rhs_x, lhs_y + rhs_y};
}

constexpr Position operator*(const Position &lhs, int v) {
  auto [lhs_x, lhs_y] = lhs;
  return Position{lhs_x * v, lhs_y * v};
}

constexpr Position operator*(const Position &lhs, const Position &rhs) {
  auto [lhs_x, lhs_y] = lhs;
  auto [rhs_x, rhs_y] = rhs;
  return Position{lhs_x * rhs_x, lhs_y * rhs_y};
}

class day10 {
public:
  static void solve(const std::string &input_path) {
    auto input = parse(input_path);

    auto valid = part_one(input);

    std::cout << "Part one: " << valid << std::endl;

    auto sum = part_two(input);

    std::cout << "Part two: " << sum << std::endl;
  }

  static input_t parse(const std::string &input_path) {
    input_t input{};
    std::ifstream inputFile{input_path};

    int y = 0;
    for (std::string line; std::getline(inputFile, line, '\n');) {
      int x = 0;
      for (char c : line) {
        Position pos{x, y};
        if (c == '#')
          input[pos] = Element::ASTEROID;
        else if (c == '.')
          input[pos] = Element::EMPTY;
        x++;
      }
      y++;
    }
    return input;
  }

  static int part_one(const input_t &input) {
    std::vector<Position> asteroids{};

    for (auto &[pos, el] : input) {
      if (el == Element::ASTEROID) {
        asteroids.push_back(pos);
      }
      auto &[x, y] = pos;
    }

    int max_in_sight = 0;
    for (auto asteroid : asteroids) {
      int visible = visible_asteroids(asteroids, asteroid).size();
      max_in_sight = std::max(visible, max_in_sight);
    }

    return max_in_sight;
  }

  static std::vector<Position>
  visible_asteroids(std::vector<Position> asteroids, Position pos) {
    std::vector<Position> visible{};

    std::sort(asteroids.begin(), asteroids.end(), [&](Position a, Position b) {
      return manhattan_distance(a, pos) < manhattan_distance(b, pos);
    });
    asteroids.erase(asteroids.begin());

    std::set<Position> hidden{};

    int max_index = 26;
    int min_index = 0;

    for (auto asteroid : asteroids) {
      if (!hidden.contains(asteroid)) {
        visible.push_back(asteroid);
      }

      auto diff = asteroid - pos;
      diff = gcd(diff);

      if (std::get<0>(diff) == 0 || std::get<1>(diff) == 0) {
        diff = sign(diff);
      }
      auto remove{asteroid + diff};
      do {
        hidden.insert(remove);
        remove = remove + diff;
      } while (std::get<0>(remove) <= max_index &&
               std::get<0>(remove) >= min_index &&
               std::get<1>(remove) <= max_index &&
               std::get<1>(remove) >= min_index);
    }

    return visible;
  }

  static int part_two(const input_t &input) {
    std::vector<Position> asteroids{};

    int width = 0;
    for (auto &[pos, el] : input) {
      if (el == Element::ASTEROID) {
        asteroids.push_back(pos);
      }
      auto &[x, y] = pos;
      width = std::max(width, x);
    }

    int max_in_sight = 0;
    Position optimal_position{0, 0};
    for (auto asteroid : asteroids) {
      int visible = visible_asteroids(asteroids, asteroid).size();
      if (visible > max_in_sight) {
        max_in_sight = visible;
        optimal_position = asteroid;
      }
    }

    std::set<Position> asteroid_set(asteroids.begin(), asteroids.end());
    asteroid_set.erase(optimal_position);
    int removed = 0;
    int output = 0;
    while (!asteroid_set.empty()) {
      std::vector<Position> new_asteroids(asteroid_set.begin(),
                                          asteroid_set.end());
      new_asteroids.push_back(optimal_position);
      auto visible = visible_asteroids(new_asteroids, optimal_position);

      std::sort(visible.begin(), visible.end(), [&](Position a, Position b) {
        auto v_a = a - optimal_position;
        auto v_b = b - optimal_position;
        double angle_a = degree(v_a, {0, 1});
        double angle_b = degree(v_b, {0, 1});

        return angle_a > angle_b;
      });

      for (auto &a : visible) {
        asteroid_set.erase(a);
        removed++;
        std::cout << removed << ": " << std::get<0>(a) << ", " << std::get<1>(a)
                  << std::endl;
        if (removed == 200) {
          output = std::get<0>(a) * 100 + std::get<1>(a);
        }
      }
    }

    return output;
  }
}; // namespace aoc
} // namespace aoc
