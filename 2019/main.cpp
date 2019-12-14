#include <iostream>
#include <chrono>

#include "src/day1.h"
#include "src/day2.h"
#include "src/day3.h"
#include "src/day4.h"
#include "src/day5.h"
#include "src/day6.h"
#include "src/day7.h"
#include "src/day8.h"
#include "src/day9.h"

int main(int argc, char** argv) {
  auto start = std::chrono::high_resolution_clock::now();

//  aoc::day7::solve("inputs/day7");

  aoc::day9::solve("inputs/day9");

  auto end = std::chrono::high_resolution_clock::now();
  auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end - start);
  std::cout << "Duration: " << duration.count() << "ms" << std::endl;

  return EXIT_SUCCESS;
}
