#pragma once

#include <array>
#include <deque>
#include <fstream>
#include <iostream>
#include <numeric>
#include <string>
#include <vector>

#include "intcode.h"

namespace aoc {

class day8 {
public:
  static void solve(const std::string &input_path) {
    auto input = parse(input_path);

    auto valid = part_one(input);

    std::cout << "Part one: " << valid << std::endl;

    auto sum = part_two(input);

    std::cout << "Part two: " << sum << std::endl;
  }

  static std::vector<int> parse(const std::string &input_path) {
    std::vector<int> input{};
    std::ifstream inputFile{input_path};

    for (std::string line; std::getline(inputFile, line, '\n');) {
      for (char c : line) {
        input.push_back(int(c) - 48);
      }
    }
    return input;
  }

  static int part_one(std::vector<int> input) {

    int width = 25;
    int height = 6;

    std::vector<std::vector<int>> layers = map_layers(input, width, height);

    int n_zeros = width;
    int score = 0;

    for (const auto &layer : layers) {
      int layer_zeros = 0;
      int layer_ones = 0;
      int layer_twos = 0;
      for (int i : layer) {
        if (i == 0) {
          layer_zeros++;
        } else if (i == 1) {
          layer_ones++;
        } else if (i == 2) {
          layer_twos++;
        }
      }

      if (layer_zeros <= n_zeros) {
        n_zeros = layer_zeros;
        score = layer_ones * layer_twos;
      }
    }

    return score;
  }

  static int part_two(std::vector<int> input) {
    int width = 25;
    int height = 6;

    std::vector<std::vector<int>> layers = map_layers(input, width, height);

    std::vector<int> image(width * height, 2);
    for (auto layer : layers) {
      for (int i = 0; i < width * height; i++) {
        if (image.at(i) == 2) {
          image.at(i) = layer.at(i);
        }
      }
    }

    for (int i = 0; i < height; i++) {
      for (int j = 0; j < width; j++) {
        int index = i * width + j;
        if (image.at(index) == 1) {
          printf("1");
        } else {
          printf(" ");
        }
      }
      std::cout << std::endl;
    }

    return 0;
  }

  static std::vector<std::vector<int>> map_layers(std::vector<int> &input, int width, int height) {
    std::vector<std::vector<int>> layers{};

    for (int j = 0; (j + 1) * width * height <= input.size(); j++) {
      int layer_start = j * width * height;
      int layer_end = (j + 1) * width * height;
      std::vector<int> layer(input.begin() + layer_start,
                             input.begin() + layer_end);
      layers.push_back(layer);
    }
    return layers;
  }

}; // namespace aoc
} // namespace aoc
