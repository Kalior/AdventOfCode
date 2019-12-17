#pragma once

#include <chrono>
#include <fstream>
#include <iostream>
#include <map>
#include <numeric>
#include <string>
#include <thread>
#include <vector>

#include "intcode.h"
#include "position.h"

namespace aoc {

enum Color : int { BLACK = 0, WHITE = 1 };

enum Direction : int { UP, RIGHT, DOWN, LEFT };

class day11 {
public:
  static void solve(const std::string &input_path) {
    auto program = parse(input_path);

    auto one = part_one(program);

    std::cout << "Part one: " << one << std::endl;

    std::cout << "Part two: " << std::endl;
    part_two(program);

  }

  static aoc::intcode::program_t parse(const std::string &input_path) {
    aoc::intcode::program_t program{};
    std::ifstream inputFile{input_path};

    for (std::string line; std::getline(inputFile, line, ',');) {
      program.push_back(std::stol(line));
    }
    return program;
  }

  static int part_one(const aoc::intcode::program_t &program) {
    aoc::intcode::channel inputs{};
    aoc::intcode::channel outputs{};

    Position pos{0, 0};
    Direction direction{UP};

    std::map<Position, Color> map{};

    std::thread thread(aoc::intcode::run_program_thread, program,
                       std::ref(inputs), std::ref(outputs));

    while (thread.joinable()) {
      int current_color = 0;
      if (map.contains(pos)) {
        current_color = int(map[pos]);
      }
      inputs.push_back(current_color);

      while (outputs.empty()) {
        using namespace std::chrono_literals;
        std::this_thread::sleep_for(10us);
      }
      if (outputs.front() == -1) {
        break;
      }
      auto new_color = Color(outputs.front());
      outputs.pop_front();

      while (outputs.empty()) {
        using namespace std::chrono_literals;
        std::this_thread::sleep_for(10us);
      }
      auto turn_direction = outputs.front();
      outputs.pop_front();

      map[pos] = new_color;

      if (turn_direction == 1) {
        direction = turn_left(direction);
      } else {
        direction = turn_right(direction);
      }

      pos = pos + from_dir(direction);
    }
    std::cout << "Out of the loop" << std::endl;
    thread.join();

    return map.size();
  }

  static Direction turn_left(Direction dir) {
    switch (dir) {
    case Direction::UP:
      return Direction::LEFT;
    case Direction::RIGHT:
      return Direction::UP;
    case Direction::DOWN:
      return Direction::RIGHT;
    case Direction::LEFT:
      return Direction::DOWN;
    }
  }

  static Direction turn_right(Direction dir) {
    switch (dir) {
    case Direction::UP:
      return Direction::RIGHT;
    case Direction::RIGHT:
      return Direction::DOWN;
    case Direction::DOWN:
      return Direction::LEFT;
    case Direction::LEFT:
      return Direction::UP;
    }
  }

  static Position from_dir(Direction dir) {
    switch (dir) {
    case Direction::UP:
      return {0, -1};
    case Direction::RIGHT:
      return {1, 0};
    case Direction::DOWN:
      return {0, 1};
    case Direction::LEFT:
      return {-1, 0};
    }
  }

  static int part_two(const aoc::intcode::program_t &program) {
    aoc::intcode::channel inputs{};
    aoc::intcode::channel outputs{};

    Position pos{0, 0};
    Direction direction{UP};

    std::map<Position, Color> map{};
    map[pos] = Color(1);

    std::thread thread(aoc::intcode::run_program_thread, program,
                       std::ref(inputs), std::ref(outputs));

    int min_x = 0;
    int max_x = 0;
    int min_y = 0;
    int max_y = 0;
    while (thread.joinable()) {
      int current_color = 0;
      if (map.contains(pos)) {
        current_color = int(map[pos]);
      }
      inputs.push_back(current_color);

      while (outputs.empty()) {
        using namespace std::chrono_literals;
        std::this_thread::sleep_for(10us);
      }
      if (outputs.front() == -1) {
        break;
      }
      auto new_color = Color(outputs.front());
      outputs.pop_front();

      while (outputs.empty()) {
        using namespace std::chrono_literals;
        std::this_thread::sleep_for(10us);
      }
      auto turn_direction = outputs.front();
      outputs.pop_front();

      map[pos] = new_color;

      if (turn_direction == 1) {
        direction = turn_left(direction);
      } else {
        direction = turn_right(direction);
      }

      pos = pos + from_dir(direction);
      max_x = std::max(max_x, std::get<0>(pos));
      min_x = std::min(min_x, std::get<0>(pos));
      max_y = std::max(max_y, std::get<1>(pos));
      min_y = std::min(min_y, std::get<1>(pos));
    }

    thread.join();

    for (int j = min_y; j <= max_y; j++) {
      for (int i = max_x + 1; i >= min_x; i--) {
        Position p{i, j};
        if (!map.contains(p)) {
          printf(" ");
        } else if (map.at(p) == Color::WHITE) {
          printf("O");
        } else {
          printf(" ");
        }
      }
      std::cout << std::endl;
    }

    return 0;
  }
};
} // namespace aoc
