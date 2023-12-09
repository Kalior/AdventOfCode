from pathlib import Path
import numpy as np


def parse_input() -> list[tuple[int, int]]:
    p = Path("inputs") / "day6"
    with p.open("r") as f:
        input = f.read()

    lines = input.split("\n")
    times = [int(time) for time in lines[0].replace("Time:", "").strip().split()]
    distances = [
        int(time) for time in lines[1].replace("Distance:", "").strip().split()
    ]

    return list(zip(times, distances))


def winning_times(time: int, distance: int) -> list[int]:
    velocities = np.arange(0, time)
    time_travelled = np.arange(time, 0, -1)

    distance_travelled = velocities * time_travelled

    return velocities[distance_travelled > distance]


def all_math_like(time: int, distance: int) -> int:
    # x^2 - time * x + distance
    x = - (time / 2) + np.sqrt(np.square(time / 2) - distance)
    x_far = - (time / 2) - np.sqrt(np.square(time / 2) - distance)

    return np.floor(x_far) - np.floor(x)


def solve(record_times: list[tuple[int, int]]):
    print(np.prod([all_math_like(time, distance) for time, distance in record_times]))

    return np.prod(
        [len(winning_times(time, distance)) for time, distance in record_times]
    )


def solve_2(record_times: list[tuple[int, int]]):
    time = int("".join([str(t) for t, _ in record_times]))
    distance = int("".join(str(d) for _, d in record_times))

    return len(winning_times(time, distance))


if __name__ == "__main__":
    games = parse_input()
    print(f"Part one: {solve(games)}")
    print(f"Part two: {solve_2(games)}")
