#include "../src/day12.h"
#include <gtest/gtest.h>

#include "../src/position.h"

TEST(DAY7, ParseVector) {
  auto vector = aoc::day12::parse_vector("50 50 50");
  EXPECT_EQ(vector, std::make_tuple(50, 50, 50));

  auto vector_2 = aoc::day12::parse_vector("<x=-11, y=1, z=8>");
  EXPECT_EQ(vector_2, std::make_tuple(-11, 1, 8));
}


TEST(DAY7, GetGravity) {
  aoc::Vector3D moon1{1,5,10};
  aoc::Vector3D moon2{10,5,1};
  auto [one_pull, two_pull] = aoc::day12::get_gravity(moon1, moon2);
  EXPECT_EQ(one_pull, std::make_tuple(1, 0, -1));
  EXPECT_EQ(two_pull, std::make_tuple(-1, 0, 1));
}


TEST(DAY7, UpdateGravity) {
  aoc::Vector3D moon1{1,5,10};
  aoc::Vector3D moon2{10,5,1};
  std::vector<aoc::Vector3D> moons{moon1, moon2};
  auto new_gravities = aoc::day12::update_gravity(moons);

  EXPECT_EQ(new_gravities[0], std::make_tuple(1, 0, -1));
  EXPECT_EQ(new_gravities[1], std::make_tuple(-1, 0, 1));
}
