from pathlib import Path
from itertools import combinations

import numpy as np


def parse_input() -> np.ndarray:
    p = Path("inputs") / "day11"
    with p.open("r") as f:
        input = f.read()
    return np.array([np.array([v for v in row]) for row in input.splitlines()])


def _expand(universe: np.ndarray, *, factor: int) -> np.ndarray:
    expanded_universe = np.ones(universe.shape)

    for i, row in enumerate(universe):
        if all(v == "." for v in row):
            expanded_universe[i, :] = factor

    for j, column in enumerate(universe.T):
        if all(column == "."):
            expanded_universe[:, j] = factor

    return expanded_universe


def solve(universe: np.ndarray) -> int:
    expanded_universe = _expand(universe, factor=2)

    galaxy_idxs = np.where(universe == "#")

    manhattan_dists = [
        sum(expanded_universe[i, y] for i in range(min(x, x_), max(x, x_)))
        + sum(expanded_universe[x, i] for i in range(min(y, y_), max(y, y_)))
        for (x, y), (x_, y_) in combinations(zip(*galaxy_idxs), 2)
    ]
    return sum(manhattan_dists)


def solve_2(universe: np.ndarray) -> int:
    expanded_universe = _expand(universe, factor=1_000_000)

    galaxy_idxs = np.where(universe == "#")

    manhattan_dists = [
        sum(expanded_universe[i, y] for i in range(min(x, x_), max(x, x_)))
        + sum(expanded_universe[x, i] for i in range(min(y, y_), max(y, y_)))
        for (x, y), (x_, y_) in combinations(zip(*galaxy_idxs), 2)
    ]
    return sum(manhattan_dists)


if __name__ == "__main__":
    universe = parse_input()
    print(f"Part one: {solve(universe)}")
    print(f"Part two: {solve_2(universe)}")
