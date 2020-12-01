#pragma once

#include <tuple>
#include <cmath>

namespace aoc {

using Position = std::tuple<int, int>;

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

using Vector3D = std::tuple<int, int, int>;

constexpr Vector3D operator+(const Vector3D &lhs, const Vector3D &rhs) {
  auto [lhs_x, lhs_y, lhs_z] = lhs;
  auto [rhs_x, rhs_y, rhs_z] = rhs;
  return Vector3D{lhs_x + rhs_x, lhs_y + rhs_y, lhs_z + rhs_z};
}

}