#pragma once

#include <fstream>
#include <iostream>
#include <map>
#include <numeric>
#include <sstream>
#include <string>
#include <vector>

namespace aoc {

class day4 {
public:
  static void solve(const std::string &input_path) {
    auto [lower_bound, upper_bound] = parse(input_path);

    auto valid = part_one(lower_bound, upper_bound);

    std::cout << "Part one: " << valid << std::endl;

    int sum = part_two(lower_bound, upper_bound);

    std::cout << "Part two: " << sum << std::endl;
  }

  static std::tuple<int, int> parse(const std::string &input_path) {

    return std::make_tuple(357253, 892942);
  }

  static int part_one(const int lower_bound, const int upper_bound) {
    int valid = 0;
    for (int i = lower_bound; i < upper_bound; i++) {
      if (is_valid(i)) {
        valid++;
      }
    }
    return valid;
  }

  static bool is_valid(const int i) {
    return has_adjacent_digits(i) && is_increasing(i);
  }

  static bool has_adjacent_digits(const int n) {
    std::string number{std::to_string(n)};
    for (int i = 0; i < number.size() - 1; i++) {
      if (number[i] == number[i + 1]) {
        return true;
      }
    }
    return false;
  }

  static bool is_increasing(const int n) {
    std::string number{std::to_string(n)};
    for (int i = 0; i < number.size() - 1; i++) {
      if (number[i] > number[i + 1]) {
        return false;
      }
    }
    return true;
  }

  static int part_two(const int lower_bound, const int upper_bound) {
    int valid = 0;
    for (int i = lower_bound; i < upper_bound; i++) {
      if (is_valid_2(i)) {
        valid++;
      }
    }
    return valid;
  }

  static bool is_valid_2(const int i) {
    return has_adjacent_digits_2(i) && is_increasing(i);
  }

  static bool has_adjacent_digits_2(const int n) {
    std::string number{std::to_string(n)};
    for (int i = 0; i < number.size() - 1; i++) {
      if (i >= 0 && i < number.size() - 2) {
        if ((number[i] == number[i + 1]) && (number[i] != number[i - 1]) &&
            (number[i + 1] != number[i + 2])) {
          return true;
        }
      } else if (i >= 0) {
        if ((number[i] == number[i + 1]) && (number[i] != number[i - 1])) {
          return true;
        }
      } else if (i < number.size() - 2) {
        if ((number[i] == number[i + 1]) && (number[i + 1] != number[i + 2])) {
          return true;
        }
      } else {
        if (number[i] == number[i + 1]) {
          return true;
        }
      }
    }
    return false;
  }
};
} // namespace aoc
