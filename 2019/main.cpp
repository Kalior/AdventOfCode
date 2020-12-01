#include <iostream>
#include <chrono>

#include "src/day12.h"


int main(int argc, char** argv) {
  auto start = std::chrono::high_resolution_clock::now();

  aoc::day12::solve("inputs/day12");

  auto end = std::chrono::high_resolution_clock::now();
  auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
  std::cout << "Duration: " << duration.count() << "ms" << std::endl;

  return EXIT_SUCCESS;
}
