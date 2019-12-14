#include <iostream>
#include <chrono>

#include "src/day7.h"

int main() {
  auto start = std::chrono::high_resolution_clock::now();
  aoc::day7::solve("inputs/day7");
  auto end = std::chrono::high_resolution_clock::now();
  auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
  std::cout << "Duration: " << duration.count() << "ms" << std::endl;

  return EXIT_SUCCESS;
}
