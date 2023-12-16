from pathlib import Path

import numpy as np
from tqdm import tqdm


def parse_input() -> np.ndarray:
    p = Path("inputs") / "day14"
    with p.open("r") as f:
        input = f.read()
    return np.array([np.array(list(line)) for line in input.splitlines()])


def roll_north(platform: np.ndarray) -> np.ndarray:
    square_rocks = np.where(platform == "#")
    closest_square_rock = np.zeros(platform.shape, dtype=int)
    for i, j in zip(*square_rocks):
        closest_square_rock[np.arange(i, platform.shape[0]), j] = i + 1

    rolling_rocks = np.where(platform == "O")
    for i, j in zip(*rolling_rocks):
        closest_rock_i = closest_square_rock[i, j]
        n_rolling_in_row = sum(platform[closest_rock_i:i, j] == "O")
        platform[i, j] = "."
        platform[closest_rock_i + n_rolling_in_row, j] = "O"

    return platform


def roll_south(platform: np.ndarray) -> np.ndarray:
    return np.flipud(roll_north(np.flipud(platform)))


def roll_west(platform: np.ndarray) -> np.ndarray:
    return roll_north(platform.T).T


def roll_east(platform: np.ndarray) -> np.ndarray:
    return roll_south(platform.T).T


def solve(platform: np.ndarray):
    rolled = roll_north(platform)
    print(rolled)

    return sum(
        [
            rolled.shape[0] - i
            for i in range(rolled.shape[0])
            for j in range(rolled.shape[1])
            if rolled[i, j] == "O"
        ]
    )


def _has_cycle(platform, history):
    for h_i, platform_copy in enumerate(history):
        if np.array_equal(platform, platform_copy):
            return h_i

    return None


def solve_2(platform: np.ndarray):
    history = []

    i = 0
    with tqdm(total=1000000000) as pbar:
        while i < 1000000000:
            platform = roll_north(platform)
            platform = roll_west(platform)
            platform = roll_south(platform)
            platform = roll_east(platform)
            found_cycle = False

            cycle = _has_cycle(platform, history)
            if not found_cycle and cycle is not None:
                period = i - cycle

                skip = (1000000000 - i) % period
                i = 1000000000 - skip
                pbar.update(1000000000 - skip)
                found_cycle = True

            history.append(platform.copy())

            pbar.update(1)
            i += 1

    return sum(
        [
            platform.shape[0] - i
            for i in range(platform.shape[0])
            for j in range(platform.shape[1])
            if platform[i, j] == "O"
        ]
    )


if __name__ == "__main__":
    platform = parse_input()
    print(f"Part one: {solve(platform)}")
    print(f"Part two: {solve_2(platform)}")
