#pragma once

#include <fstream>
#include <iostream>
#include <map>
#include <numeric>
#include <sstream>
#include <string>
#include <vector>

namespace aoc {

struct Move {
  char direction;
  int steps;
};

enum Element : int { EMPTY, WIRE_ONE, WIRE_TWO, COLLISION };

using Position = std::tuple<int, int>;

int manhattan_distance(Position pos) {
  auto [x, y] = pos;
  return std::abs(x) + std::abs(y);
}

using wires_t = std::vector<std::vector<Move>>;

class day3 {
public:
  static void solve(const std::string &input_path) {
    auto wires = parse(input_path);

    auto distance = part_one(wires);

    std::cout << "Part one: " << distance << std::endl;

    int sum = part_two(wires);

    std::cout << "Part two: " << sum << std::endl;
  }

  static wires_t parse(const std::string &input_path) {
    wires_t wires{};
    std::ifstream inputFile{input_path};

    for (std::string wire; std::getline(inputFile, wire);) {
      auto parsed_wire = parse_wire(wire);
      wires.push_back(parsed_wire);
    }
    return wires;
  }

  static std::vector<Move> parse_wire(const std::string &wire) {
    std::vector<Move> parsed_wire{};

    std::stringstream wire_stream{wire};
    for (std::string move; std::getline(wire_stream, move, ',');) {
      auto int_part = std::string(move.begin() + 1, move.end());
      Move parsed_move{move[0], std::stoi(int_part)};
      parsed_wire.push_back(parsed_move);
    }
    return parsed_wire;
  }

  static int part_one(const wires_t &wires) {

    std::map<Position, Element> grid{};
    add_wire(grid, wires[0], Element::WIRE_ONE);
    auto collisions = add_wire(grid, wires[1], Element::WIRE_TWO);

    std::vector<int> distances{};
    std::transform(collisions.begin(), collisions.end(),
                   std::back_insert_iterator(distances),
                   [](std::tuple<Position, int> tuple) -> int {
                     return manhattan_distance(std::get<0>(tuple));
                   });

    int distance = *std::min_element(distances.begin(), distances.end());
    return distance;
  }

  static std::vector<std::tuple<Position, int>>
  add_wire(std::map<Position, Element> &grid, const std::vector<Move> &wire,
           Element wire_type) {
    std::vector<std::tuple<Position, int>> collisions{};

    Position pos{0, 0};
    int steps_taken = 0;
    for (Move move : wire) {
      Position new_pos{pos};
      auto &[new_x, new_y] = new_pos;
      if (move.direction == 'R') {
        new_x += move.steps;
      } else if (move.direction == 'U') {
        new_y += move.steps;
      } else if (move.direction == 'L') {
        new_x -= move.steps;
      } else if (move.direction == 'D') {
        new_y -= move.steps;
      } else {
        throw std::invalid_argument("Invalid direction");
      }
      fill_x(pos, new_pos, grid, collisions, wire_type, steps_taken);
      fill_y(pos, new_pos, grid, collisions, wire_type, steps_taken);
      pos = new_pos;
    }

    return collisions;
  }

  static void fill_x(Position start, Position end,
                     std::map<Position, Element> &grid,
                     std::vector<std::tuple<Position, int>> &collisions,
                     Element wire_type, int &steps_taken) {
    auto [start_x, start_y] = start;
    auto [end_x, end_y] = end;

    if (start_x < end_x) {
      for (int x = start_x; x < end_x; x++) {
        Position fill_pos{x, start_y};
        fill_square(fill_pos, grid, collisions, wire_type, steps_taken);
        steps_taken++;
      }
    } else {
      for (int x = start_x; x > end_x; x--) {
        Position fill_pos{x, start_y};
        fill_square(fill_pos, grid, collisions, wire_type, steps_taken);
        steps_taken++;
      }
    }
  }

  static void fill_y(Position start, Position end,
                     std::map<Position, Element> &grid,
                     std::vector<std::tuple<Position, int>> &collisions,
                     Element wire_type, int &steps_taken) {
    auto [start_x, start_y] = start;
    auto [end_x, end_y] = end;

    if (start_y < end_y) {
      for (int y = start_y; y < end_y; y++) {
        Position fill_pos{start_x, y};
        fill_square(fill_pos, grid, collisions, wire_type, steps_taken);
        steps_taken++;
      }
    } else {
      for (int y = start_y ; y > end_y; y--) {
        Position fill_pos{start_x, y};
        fill_square(fill_pos, grid, collisions, wire_type, steps_taken);
        steps_taken++;
      }
    }
  }

  static void fill_square(Position fill_pos, std::map<Position, Element> &grid,
                          std::vector<std::tuple<Position, int>> &collisions,
                          Element wire_type, int &steps_taken) {
    auto [x, y] = fill_pos;
    if (grid[fill_pos] == Element::EMPTY) {
      grid[fill_pos] = wire_type;
    } else if (grid[fill_pos] != wire_type) {
      grid[fill_pos] = Element::COLLISION;
      if (x != 0 || y != 0) {
        collisions.emplace_back(fill_pos, steps_taken);
      }
    }
  }

  static int part_two(wires_t wires) {
    std::map<Position, Element> grid{};
    add_wire(grid, wires[0], Element::WIRE_ONE);
    auto intersections_two = add_wire(grid, wires[1], Element::WIRE_TWO);
    auto intersections_one = add_wire(grid, wires[0], Element::WIRE_ONE);

    std::map<Position, std::tuple<int, int>> intersections{};
    for (auto &[pos, steps] : intersections_one) {
      auto [one, two] = intersections[pos];
      if (one == 0) {
        intersections[pos] = std::make_tuple(steps, two);
      }
    }
    for (auto &[pos, steps] : intersections_two) {
      auto [one, two] = intersections[pos];
      if (two == 0) {
        intersections[pos] = std::make_tuple(one, steps);
      }
    }

    std::vector<int> steps{};
    std::transform(
        intersections.begin(), intersections.end(),
        std::back_insert_iterator(steps),
        [](std::pair<Position, std::tuple<int, int>> element) -> int {
          return std::abs(std::get<0>(element.second)) +
                 std::abs(std::get<1>(element.second));
        });

    int distance = *std::min_element(steps.begin(), steps.end());
    return distance;
  }

  static void print_grid(std::map<Position, Element> &grid) {
    std::vector<int> xs{};
    std::transform(grid.begin(), grid.end(), std::back_inserter(xs),
                   [](std::pair<Position, Element> element) -> int {
                     return std::get<0>(element.first);
                   });
    std::vector<int> ys{};
    std::transform(grid.begin(), grid.end(), std::back_inserter(ys),
                   [](std::pair<Position, Element> element) -> int {
                     return std::get<1>(element.first);
                   });

    if (ys.empty() || xs.empty()) {
      return;
    }

    int min_x = *std::min_element(xs.begin(), xs.end());
    int max_x = *std::max_element(xs.begin(), xs.end());
    int min_y = *std::min_element(ys.begin(), ys.end());
    int max_y = *std::max_element(ys.begin(), ys.end());

    std::cout << min_x << " " << min_y << std::endl;
    std::cout << max_x << " " << max_y << std::endl;

    for (int x = min_x; x <= max_x; x++) {
      for (int y = min_y; y <= max_y; y++) {
        Position pos{x, y};
        if (grid[pos] == Element::COLLISION) {
          std::cout << "X";
        } else if (grid[pos] == Element::EMPTY) {
          std::cout << " ";
        } else {
          std::cout << "-";
        }
      }
      std::cout << std::endl;
    }
  }
};
} // namespace aoc
